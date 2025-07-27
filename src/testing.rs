#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
static mut testing: bool = false;
#[no_mangle]
pub unsafe extern "C" fn enable_testing() {
    testing = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn is_testing() -> bool {
    return testing;
}
