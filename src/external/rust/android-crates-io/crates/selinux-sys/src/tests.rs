use std::os::raw::c_char;

#[test]
fn is_selinux_enabled() {
    let r = dbg!(unsafe { super::is_selinux_enabled() });
    assert!(r == 0 || r == 1);
}

#[test]
fn getcon() {
    let mut context: *mut c_char = core::ptr::null_mut();
    let r = unsafe { super::getcon(&raw mut context) };
    assert_ne!(r, -1);
    assert!(!context.is_null());

    unsafe { super::freecon(context) }
}
