#[cfg(test)]
mod tests;

use alloc::sync::{Arc, Weak};
use core::marker::PhantomData;
use core::ptr;
use std::io;
use std::os::raw::{c_char, c_int, c_uint, c_void};

//use reference_counted_singleton::{RCSRef, RefCountedSingleton};

use parking_lot::Mutex;

use crate::SecurityContext;
use crate::errors::{Error, Result};
use crate::utils::{ret_val_to_result, str_to_c_string};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
struct SELinuxOption(selinux_sys::selinux_opt);

// SAFETY: `selinux_sys::selinux_opt` is treated as an opaque byte sequence.
// In particular, the pointer inside `selinux_sys::selinux_opt` is never dereferenced.
unsafe impl Send for SELinuxOption {}
unsafe impl Sync for SELinuxOption {}

/// Access vector cache.
#[derive(Debug, PartialEq, Eq)]
pub struct AccessVectorCache {
    options: Box<[SELinuxOption]>,
}

impl AccessVectorCache {
    /// Initialize the user space access vector cache.
    ///
    /// The `options` parameter produces zero or more `(type, value)` tuples, where:
    /// - `type` is one of `selinux_sys::AVC_OPT_*` values,
    ///   *e.g.*, [`selinux_sys::AVC_OPT_SETENFORCE`].
    /// - `value` is a pointer whose semantics are specific to `type`.
    ///
    /// Attempting to initialize the access vector cache while it is still
    /// initialized succeeds only if the subsequent initialization uses the same
    /// set of options as the previous, still in scope, one.
    ///
    /// See: `avc_open()`.
    #[doc(alias = "avc_open")]
    pub fn initialize(options: &[(c_int, *const c_void)]) -> Result<Arc<Self>> {
        static AVC: Mutex<Weak<AccessVectorCache>> = Mutex::new(Weak::new());

        let mut options: Vec<SELinuxOption> = options
            .iter()
            .map(|&(type_, value)| {
                SELinuxOption(selinux_sys::selinux_opt {
                    type_,
                    value: value.cast(),
                })
            })
            .collect();
        options.sort_unstable();
        options.dedup();
        let mut options = options.into_boxed_slice();

        let count = c_uint::try_from(options.len())?;
        let options_ptr = if count == 0 {
            ptr::null_mut()
        } else {
            options.as_mut_ptr().cast()
        };

        let mut avc = AVC.lock();
        if let Some(existing_avc) = Weak::upgrade(&*avc) {
            if *existing_avc.options == *options {
                // Initializing, while still initialized, using the same set of options.
                Ok(existing_avc)
            } else {
                // Initializing, while still initialized, with a different set of options,
                // is an error.
                let err = io::ErrorKind::AlreadyExists.into();
                Err(Error::from_io("AccessVectorCache::initialize()", err))
            }
        } else {
            // First initialization, or reinitialization.
            if unsafe { selinux_sys::avc_open(options_ptr, count) } == -1 {
                Err(Error::last_io_error("avc_open()"))
            } else {
                let new_avc = Arc::new(AccessVectorCache { options });
                *avc = Arc::downgrade(&new_avc);
                Ok(new_avc)
            }
        }
    }

    /// Flush the user space access vector cache, causing it to forget any
    /// cached access decisions.
    ///
    /// See: `avc_reset()`.
    #[doc(alias = "avc_reset")]
    pub fn reset(&self) -> Result<()> {
        ret_val_to_result("avc_reset()", unsafe { selinux_sys::avc_reset() })
    }

    /// Attempt to free unused memory within the user space access vector
    /// cache, but do not flush any cached access decisions.
    ///
    /// See: `avc_cleanup()`.
    #[doc(alias = "avc_cleanup")]
    pub fn clean_up(&self) {
        unsafe { selinux_sys::avc_cleanup() }
    }

    /// Return a security identifier for the kernel initial security identifier
    /// specified by `security_identifier_name`.
    ///
    /// See: `avc_get_initial_sid()`.
    #[doc(alias = "avc_get_initial_sid")]
    pub fn kernel_initial_security_id<'context>(
        &'context self,
        security_id_name: &str,
        raw_format: bool,
    ) -> Result<SecurityID<'context>> {
        let c_name = str_to_c_string(security_id_name)?;
        let mut security_id: *mut selinux_sys::security_id = ptr::null_mut();
        if unsafe { selinux_sys::avc_get_initial_sid(c_name.as_ptr(), &raw mut security_id) }
            == -1_i32
        {
            Err(Error::last_io_error("avc_get_initial_sid()"))
        } else {
            Ok(SecurityID {
                security_id,
                is_raw: raw_format,
                _phantom_data: PhantomData,
            })
        }
    }

    /// Return a security context for the given security identifier.
    ///
    /// See: `avc_sid_to_context()`.
    #[doc(alias = "avc_sid_to_context")]
    pub fn security_context_from_security_id<'context>(
        &'context self,
        mut security_id: SecurityID,
    ) -> Result<SecurityContext<'context>> {
        let is_raw = security_id.is_raw_format();
        let (proc, proc_name): (unsafe extern "C" fn(_, _) -> _, _) = if is_raw {
            let proc_name = "avc_sid_to_context_raw()";
            (selinux_sys::avc_sid_to_context_raw, proc_name)
        } else {
            let proc_name = "avc_sid_to_context()";
            (selinux_sys::avc_sid_to_context, proc_name)
        };

        let mut context: *mut c_char = ptr::null_mut();
        let r = unsafe { proc(security_id.as_mut_ptr(), &raw mut context) };
        SecurityContext::from_result(proc_name, r, context, is_raw)
    }

    /// Return a security identifier for the given security context.
    ///
    /// See: `avc_context_to_sid()`.
    #[doc(alias = "avc_context_to_sid")]
    pub fn security_id_from_security_context<'context>(
        &'context self,
        context: SecurityContext,
    ) -> Result<SecurityID<'context>> {
        let is_raw = context.is_raw_format();
        let (proc, proc_name): (unsafe extern "C" fn(_, _) -> _, _) = if is_raw {
            let proc_name = "avc_context_to_sid_raw()";
            (selinux_sys::avc_context_to_sid_raw, proc_name)
        } else {
            let proc_name = "avc_context_to_sid()";
            (selinux_sys::avc_context_to_sid, proc_name)
        };

        let mut security_id: *mut selinux_sys::security_id = ptr::null_mut();
        if unsafe { proc(context.as_ptr(), &raw mut security_id) } == -1_i32 {
            Err(Error::last_io_error(proc_name))
        } else {
            Ok(SecurityID {
                security_id,
                is_raw,
                _phantom_data: PhantomData,
            })
        }
    }
}

impl Drop for AccessVectorCache {
    fn drop(&mut self) {
        unsafe { selinux_sys::avc_destroy() };
    }
}

/// SELinux security identifier.
#[derive(Debug)]
pub struct SecurityID<'id> {
    security_id: *mut selinux_sys::security_id,
    is_raw: bool,
    _phantom_data: PhantomData<&'id selinux_sys::security_id>,
}

impl SecurityID<'_> {
    /// Return `true` if the security identifier is unspecified.
    #[must_use]
    pub fn is_unspecified(&self) -> bool {
        self.security_id.is_null()
    }

    /// Return `false` if security context translation must be performed.
    #[must_use]
    pub fn is_raw_format(&self) -> bool {
        self.is_raw
    }

    /// Return the managed raw pointer to [`selinux_sys::security_id`].
    #[must_use]
    pub fn as_ptr(&self) -> *const selinux_sys::security_id {
        self.security_id.cast()
    }

    /// Return the managed raw pointer to [`selinux_sys::security_id`].
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut selinux_sys::security_id {
        self.security_id
    }
}

impl Default for SecurityID<'_> {
    /// Return an unspecified security identifier.
    fn default() -> Self {
        Self {
            security_id: ptr::null_mut(),
            is_raw: false,
            _phantom_data: PhantomData,
        }
    }
}
