#![cfg(any(target_os = "linux", target_os = "android"))]
#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/selinux-sys/0.7.0")]

#[cfg(test)]
mod tests;

include!(concat!(env!("OUT_DIR"), "/selinux-sys.rs"));

/// Unspecified SID.
pub const SECSID_WILD: security_id_t = std::ptr::null_mut();

/// Initialize an `avc_entry_ref` structure.
///
/// # Safety
/// `aeref` is assumed to be a valid pointer to a mutable `avc_entry_ref` structure.
pub unsafe fn avc_entry_ref_init(aeref: *mut avc_entry_ref) {
    if !aeref.is_null() {
        unsafe {
            aeref.write(avc_entry_ref {
                ae: std::ptr::null_mut(),
            });
        }
    }
}
