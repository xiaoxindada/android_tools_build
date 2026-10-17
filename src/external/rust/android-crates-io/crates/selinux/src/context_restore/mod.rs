#[cfg(test)]
mod tests;

use core::ffi::CStr;
use core::marker::PhantomData;
use core::{iter, ptr};
use std::io;
use std::os::raw::{c_int, c_uint};
use std::path::Path;

use crate::errors::{Error, Result};
use crate::label::Labeler;
use crate::utils::{
    OptionalNativeFunctions, c_str_ptr_to_path, c_str_ptr_to_str, os_str_to_c_string,
    ret_val_to_result,
};

bitflags! {
    /// Flags controlling relabeling operations.
    pub struct RestoreFlags: c_uint {
        /// Force the checking of labels even if the stored SHA1 digest matches
        /// the specfile entries SHA1 digest.
        ///
        /// The specfile entries digest will be written to the `security.sehash`
        /// extended attribute once relabeling has been completed successfully
        /// provided the [`NO_CHANGE`] flag has not been set.
        ///
        /// [`NO_CHANGE`]: Self::NO_CHANGE
        #[allow(clippy::as_conversions)]
        const IGNORE_DIGEST = selinux_sys::SELINUX_RESTORECON_IGNORE_DIGEST as c_uint;

        /// Don't change any file labels (passive check) or update the digest in
        /// the `security.sehash` extended attribute.
        #[allow(clippy::as_conversions)]
        const NO_CHANGE = selinux_sys::SELINUX_RESTORECON_NOCHANGE as c_uint;

        /// If set, reset the files label to match the default spec file context.
        /// If not set only reset the files "type" component of the context
        /// to match the default spec file context.
        #[allow(clippy::as_conversions)]
        const SET_SPEC_FILE_CTX =
            selinux_sys::SELINUX_RESTORECON_SET_SPECFILE_CTX as c_uint;

        /// Change file and directory labels recursively (descend directories)
        /// and if successful write an SHA1 digest of the spec file entries
        /// to an extended attribute.
        #[allow(clippy::as_conversions)]
        const RECURSE = selinux_sys::SELINUX_RESTORECON_RECURSE as c_uint;

        /// Log file label changes.
        ///
        /// Note that if [`VERBOSE`] and [`PROGRESS`] flags are set,
        /// then [`PROGRESS`] will take precedence.
        ///
        /// [`VERBOSE`]: Self::VERBOSE
        /// [`PROGRESS`]:  Self::PROGRESS
        #[allow(clippy::as_conversions)]
        const VERBOSE = selinux_sys::SELINUX_RESTORECON_VERBOSE as c_uint;

        /// Show progress by outputting the number of files in 1k blocks
        /// processed to stdout.
        ///
        /// If the [`MASS_RELABEL`] flag is also set then the approximate
        /// percentage complete will be shown.
        ///
        /// [`MASS_RELABEL`]: Self::MASS_RELABEL
        #[allow(clippy::as_conversions)]
        const PROGRESS = selinux_sys::SELINUX_RESTORECON_PROGRESS as c_uint;

        /// Convert passed-in path name to the canonical path name using
        /// `realpath()`.
        #[allow(clippy::as_conversions)]
        const REAL_PATH = selinux_sys::SELINUX_RESTORECON_REALPATH as c_uint;

        /// Prevent descending into directories that have a different device
        /// number than the path name entry from which the descent began.
        #[allow(clippy::as_conversions)]
        const XDEV = selinux_sys::SELINUX_RESTORECON_XDEV as c_uint;

        /// Attempt to add an association between an inode and a specification.
        /// If there is already an association for the inode and it conflicts
        /// with the specification, then use the last matching specification.
        #[allow(clippy::as_conversions)]
        const ADD_ASSOC = selinux_sys::SELINUX_RESTORECON_ADD_ASSOC as c_uint;

        /// Abort on errors during the file tree walk.
        #[allow(clippy::as_conversions)]
        const ABORT_ON_ERROR = selinux_sys::SELINUX_RESTORECON_ABORT_ON_ERROR as c_uint;

        /// Log any label changes to `syslog()`.
        #[allow(clippy::as_conversions)]
        const SYS_LOG_CHANGES = selinux_sys::SELINUX_RESTORECON_SYSLOG_CHANGES as c_uint;

        /// Log what spec file context matched each file.
        #[allow(clippy::as_conversions)]
        const LOG_MATCHES = selinux_sys::SELINUX_RESTORECON_LOG_MATCHES as c_uint;

        /// Ignore files that do not exist.
        #[allow(clippy::as_conversions)]
        const IGNORE_NO_ENTRY = selinux_sys::SELINUX_RESTORECON_IGNORE_NOENTRY as c_uint;

        /// Do not read `/proc/mounts` to obtain a list of non-seclabel mounts
        /// to be excluded from relabeling checks.
        ///
        /// Setting [`IGNORE_MOUNTS`] is useful where there is a non-seclabel fs
        /// mounted with a seclabel fs mounted on a directory below this.
        ///
        /// [`IGNORE_MOUNTS`]: Self::IGNORE_MOUNTS
        #[allow(clippy::as_conversions)]
        const IGNORE_MOUNTS = selinux_sys::SELINUX_RESTORECON_IGNORE_MOUNTS as c_uint;

        /// Generally set when relabeling the entire OS, that will then show
        /// the approximate percentage complete.
        ///
        /// The [`PROGRESS`] flag must also be set.
        ///
        /// [`PROGRESS`]: Self::PROGRESS
        #[allow(clippy::as_conversions)]
        const MASS_RELABEL = selinux_sys::SELINUX_RESTORECON_MASS_RELABEL as c_uint;

        // The rest of the constants were defined after version 2.8, so selinux_sys might not
        // export them. We therefore define them manually.

        /// Do not check or update any extended attribute security.sehash entries.
        ///
        /// This flag is supported only by `libselinux` version `3.0` or later.
        const SKIP_DIGEST = 0x8000;

        /// Treat conflicting specifications, such as where two hardlinks for
        /// the same inode have different contexts, as errors.
        ///
        /// This flag is supported only by `libselinux` version `3.1` or later.
        const CONFLICT_ERROR = 0x1_0000;

        /// Count, but otherwise ignore, errors during the file tree walk.
        ///
        /// This flag requires `libselinux` version `3.4` or later.
        const COUNT_ERRORS = 0x2_0000;

        /// In addition to the type component also change the user and
        /// role component of security contexts.
        ///
        /// This flag requires `libselinux` version `3.9` or later.
        const SET_USER_ROLE = 0x4_0000;

        /// Count the number of relabeled files (or would be relabeled if
        /// the [`NO_CHANGE`] flag was not set).
        ///
        /// This flag requires `libselinux` version `3.10` or later.
        ///
        /// [`NO_CHANGE`]: Self::NO_CHANGE
        const COUNT_RELABELED = 0x8_0000;

        /// Do not relabel non-directories that have more than one hard link.
        ///
        /// This flag requires `libselinux` version `3.11` or later.
        const SKIP_MULTILINK = 0x10_0000;
    }
}

bitflags! {
    /// Flags of [`ContextRestore::manage_security_sehash_xattr_entries`].
    pub struct XAttrFlags: c_uint {
        /// Recursively descend directories.
        #[allow(clippy::as_conversions)]
        const RECURSE = selinux_sys::SELINUX_RESTORECON_XATTR_RECURSE as c_uint;

        /// Delete non-matching digests from each directory in path name.
        #[allow(clippy::as_conversions)]
        const DELETE_NON_MATCH_DIGESTS = selinux_sys::SELINUX_RESTORECON_XATTR_DELETE_NONMATCH_DIGESTS as c_uint;

        /// Delete all digests from each directory in path name.
        #[allow(clippy::as_conversions)]
        const DELETE_ALL_DIGESTS = selinux_sys::SELINUX_RESTORECON_XATTR_DELETE_ALL_DIGESTS as c_uint;

        /// Don't read `/proc/mounts` to obtain a list of non-seclabel mounts
        /// to be excluded from the search.
        ///
        /// Setting [`IGNORE_MOUNTS`] is useful where there is a non-seclabel fs
        /// mounted with a seclabel fs mounted on a directory below this.
        ///
        /// [`IGNORE_MOUNTS`]: Self::IGNORE_MOUNTS
        #[allow(clippy::as_conversions)]
        const IGNORE_MOUNTS = selinux_sys::SELINUX_RESTORECON_XATTR_IGNORE_MOUNTS as c_uint;
    }
}

/// Restore file(s) default SELinux security contexts.
#[derive(Debug, Default)]
pub struct ContextRestore<'labeler, T: crate::label::back_end::BackEnd> {
    labeler: Option<&'labeler mut Labeler<T>>,
}

impl<'labeler, T> ContextRestore<'labeler, T>
where
    T: crate::label::back_end::BackEnd,
{
    /// Set a labeling handle for relabeling.
    ///
    /// See: `selinux_restorecon_set_sehandle()`.
    #[doc(alias = "selinux_restorecon_set_sehandle")]
    pub fn with_labeler(labeler: &'labeler mut Labeler<T>) -> Self {
        Self {
            labeler: Some(labeler),
        }
    }

    /// Get the labeling handle to be used for relabeling.
    #[must_use]
    pub fn labeler(&self) -> Option<&&'labeler mut Labeler<T>> {
        self.labeler.as_ref()
    }

    /// Set an alternate root path for relabeling.
    ///
    /// See: `selinux_restorecon_set_alt_rootpath()`.
    #[doc(alias = "selinux_restorecon_set_alt_rootpath")]
    pub fn set_alternative_root_path(&mut self, path: impl AsRef<Path>) -> Result<()> {
        let c_path = os_str_to_c_string(path.as_ref().as_os_str())?;
        let r = unsafe { selinux_sys::selinux_restorecon_set_alt_rootpath(c_path.as_ptr()) };
        ret_val_to_result("selinux_restorecon_set_alt_rootpath()", r)
    }

    /// Add to the list of directories to be excluded from relabeling.
    ///
    /// See: `selinux_restorecon_set_exclude_list()`.
    #[doc(alias = "selinux_restorecon_set_exclude_list")]
    pub fn add_exclude_list<P>(
        &mut self,
        exclusion_patterns: impl IntoIterator<Item = P>,
    ) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let c_list_storage = exclusion_patterns
            .into_iter()
            .map(|path| os_str_to_c_string(path.as_ref().as_os_str()))
            .collect::<Result<Vec<_>>>()?;

        if !c_list_storage.is_empty() {
            let mut c_ptr_list: Vec<_> = c_list_storage
                .iter()
                .map(AsRef::as_ref)
                .map(CStr::as_ptr)
                .chain(iter::once(ptr::null()))
                .collect();

            unsafe { selinux_sys::selinux_restorecon_set_exclude_list(c_ptr_list.as_mut_ptr()) };
        }
        Ok(())
    }

    /// Restore file(s) default SELinux security contexts.
    ///
    /// If `threads_count` is zero, then:
    /// - If `selinux_restorecon_parallel()` is supported by `libselinux` (version 3.4 or later),
    ///   then this operation will use as many threads as the number of online processor
    ///   cores present.
    /// - Otherwise, this operation will run in one thread.
    ///
    /// If this method succeeds, then it returns some statistics about the result of the operation.
    /// The information returned depends on the `libselinux` version, and on the specified `flags`.
    /// See the fields of [`FileSystemEntryContextRestoreResult`], in order to find out
    /// which [`RestoreFlags`] enable which statistics.
    ///
    /// See: `selinux_restorecon()`, `selinux_restorecon_parallel()`,
    /// `selinux_restorecon_get_skipped_errors()`, `selinux_restorecon_get_relabeled_files()`.
    #[doc(alias = "selinux_restorecon")]
    #[doc(alias = "selinux_restorecon_parallel")]
    #[allow(clippy::useless_conversion)]
    pub fn restore_context_of_file_system_entry(
        self,
        path: impl AsRef<Path>,
        threads_count: usize,
        flags: RestoreFlags,
    ) -> Result<FileSystemEntryContextRestoreResult> {
        if let Some(labeler) = self.labeler.map(Labeler::as_mut_ptr) {
            unsafe { selinux_sys::selinux_restorecon_set_sehandle(labeler) };
        }

        let c_path = os_str_to_c_string(path.as_ref().as_os_str())?;
        match threads_count {
            0 => {
                // Call `selinux_restorecon_parallel()` if possible.
                Error::clear_errno();
                let r = unsafe {
                    (OptionalNativeFunctions::get().selinux_restorecon_parallel)(
                        c_path.as_ptr(),
                        flags.bits(),
                        threads_count,
                    )
                };

                if r == -1_i32 {
                    if io::Error::last_os_error().raw_os_error() != Some(libc::ENOSYS) {
                        return Err(Error::last_io_error("selinux_restorecon_parallel()"));
                    }

                    // `selinux_restorecon_parallel()` is unsupported.
                    // Call `selinux_restorecon()` instead.
                    let r =
                        unsafe { selinux_sys::selinux_restorecon(c_path.as_ptr(), flags.bits()) };
                    ret_val_to_result("selinux_restorecon()", r)?;
                }
            }

            1 => {
                let r = unsafe { selinux_sys::selinux_restorecon(c_path.as_ptr(), flags.bits()) };
                ret_val_to_result("selinux_restorecon()", r)?;
            }

            _ => {
                let r = unsafe {
                    (OptionalNativeFunctions::get().selinux_restorecon_parallel)(
                        c_path.as_ptr(),
                        flags.bits(),
                        threads_count,
                    )
                };
                ret_val_to_result("selinux_restorecon_parallel()", r)?;
            }
        }

        let skipped_errors = flags.contains(RestoreFlags::COUNT_ERRORS).then(|| {
            let r =
                unsafe { (OptionalNativeFunctions::get().selinux_restorecon_get_skipped_errors)() };
            u64::from(r)
        });

        let relabeled_files = if flags.contains(RestoreFlags::COUNT_RELABELED) {
            Error::clear_errno();

            let r = unsafe {
                (OptionalNativeFunctions::get().selinux_restorecon_get_relabeled_files)()
            };

            (r != 0 || io::Error::last_os_error().raw_os_error() != Some(libc::ENOSYS))
                .then_some(u64::from(r))
        } else {
            None
        };

        Ok(FileSystemEntryContextRestoreResult {
            relabeled_files,
            skipped_errors,
        })
    }

    /// Manage default `security.sehash` extended attribute entries added by
    /// `selinux_restorecon()`, `setfiles()` or `restorecon()`.
    ///
    /// See: `selinux_restorecon_xattr()`.
    #[doc(alias = "selinux_restorecon_xattr")]
    pub fn manage_security_sehash_xattr_entries(
        dir_path: impl AsRef<Path>,
        flags: XAttrFlags,
    ) -> Result<DirectoryXAttributesIter> {
        let mut xattr_list_ptr: *mut *mut selinux_sys::dir_xattr = ptr::null_mut();
        let c_dir_path = os_str_to_c_string(dir_path.as_ref().as_os_str())?;
        let r: c_int = unsafe {
            selinux_sys::selinux_restorecon_xattr(
                c_dir_path.as_ptr(),
                flags.bits(),
                &raw mut xattr_list_ptr,
            )
        };

        if r == -1_i32 {
            return Err(Error::last_io_error("selinux_restorecon_xattr()"));
        }

        let Some(mut xattr_list_ptr) = ptr::NonNull::new(xattr_list_ptr) else {
            return Ok(DirectoryXAttributesIter(None)); // Empty list.
        };

        let xattr_list = unsafe { *xattr_list_ptr.as_ref() };

        // Detach the linked list from libselinux, because the returned DirectoryXAttributesIter
        // owns it from now on.
        unsafe { *xattr_list_ptr.as_mut() = ptr::null_mut() };

        Ok(DirectoryXAttributesIter(ptr::NonNull::new(xattr_list)))
    }
}

/// Statistics of a successful [`ContextRestore::restore_context_of_file_system_entry`] operation.
///
/// The information contained here depends on the `libselinux` version,
/// and on the specified `flags`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct FileSystemEntryContextRestoreResult {
    /// Number of files that were successfully relabeled.
    ///
    /// If `flags` included [`RestoreFlags::NO_CHANGE`], then this is the number of files
    /// that would be relabeled.
    ///
    /// This information will only be available when supported, and when `flags` included
    /// [`RestoreFlags::COUNT_RELABELED`].
    pub relabeled_files: Option<u64>,

    /// Number of errors ignored during the file tree walk.
    ///
    /// This information will only be available when supported, and when `flags` included
    /// [`RestoreFlags::COUNT_ERRORS`].
    pub skipped_errors: Option<u64>,
}

/// Status of a [`DirectoryXAttributes`].
#[derive(Debug)]
#[non_exhaustive]
pub enum DirectoryDigestResult {
    /// Match.
    Match {
        /// Matching digest deleted from the directory.
        deleted: bool,
    },
    /// No match.
    NoMatch {
        /// Non-matching digest deleted from the directory.
        deleted: bool,
    },
    /// Error.
    Error,
    /// Unknown status.
    Unknown(c_uint),
}

/// Result of [`ContextRestore::manage_security_sehash_xattr_entries`].
#[derive(Debug)]
pub struct DirectoryXAttributes {
    pointer: ptr::NonNull<selinux_sys::dir_xattr>,
    _phantom_data: PhantomData<selinux_sys::dir_xattr>,
}

impl DirectoryXAttributes {
    /// Return the managed raw pointer to [`selinux_sys::dir_xattr`].
    #[must_use]
    pub fn as_ptr(&self) -> *const selinux_sys::dir_xattr {
        self.pointer.as_ptr()
    }

    /// Return the managed raw pointer to [`selinux_sys::dir_xattr`].
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut selinux_sys::dir_xattr {
        self.pointer.as_ptr()
    }

    /// Directory path.
    #[must_use]
    pub fn directory_path(&self) -> &Path {
        c_str_ptr_to_path(unsafe { self.pointer.as_ref().directory })
    }

    /// A hex encoded string that can be printed.
    pub fn digest(&self) -> Result<&str> {
        c_str_ptr_to_str(unsafe { self.pointer.as_ref().digest })
    }

    /// Status of this entry.
    #[must_use]
    pub fn digest_result(&self) -> DirectoryDigestResult {
        match unsafe { self.pointer.as_ref().result } {
            selinux_sys::digest_result::MATCH => DirectoryDigestResult::Match { deleted: false },

            selinux_sys::digest_result::NOMATCH => {
                DirectoryDigestResult::NoMatch { deleted: false }
            }

            selinux_sys::digest_result::DELETED_MATCH => {
                DirectoryDigestResult::Match { deleted: true }
            }

            selinux_sys::digest_result::DELETED_NOMATCH => {
                DirectoryDigestResult::NoMatch { deleted: true }
            }

            selinux_sys::digest_result::ERROR => DirectoryDigestResult::Error,

            value => DirectoryDigestResult::Unknown(value),
        }
    }
}

impl Drop for DirectoryXAttributes {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.pointer.as_ref().directory.cast());
            libc::free(self.pointer.as_ref().digest.cast());
            libc::free(self.pointer.as_ptr().cast());
        }
    }
}

/// Iterator producing [`DirectoryXAttributes`] elements.
#[derive(Debug)]
pub struct DirectoryXAttributesIter(Option<ptr::NonNull<selinux_sys::dir_xattr>>);

impl Drop for DirectoryXAttributesIter {
    fn drop(&mut self) {
        self.for_each(drop);
    }
}

impl Iterator for DirectoryXAttributesIter {
    type Item = DirectoryXAttributes;

    fn next(&mut self) -> Option<DirectoryXAttributes> {
        self.0.map(|pointer| {
            // `pointer` is the head of the linked list that we currently own.
            // Detach the head and return it inside a `DirectoryXAttributes` which owns it.
            // We still own the remaining tail of the linked list.

            self.0 = ptr::NonNull::new(unsafe { pointer.as_ref() }.next);

            DirectoryXAttributes {
                pointer,
                _phantom_data: PhantomData,
            }
        })
    }
}
