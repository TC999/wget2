#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_default_cert_dir() -> *const libc::c_char {
    return b"/etc/ssl/certs\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_default_ca_bundle_path() -> *const libc::c_char {
    return 0 as *const libc::c_char;
}
