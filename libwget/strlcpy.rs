#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn wget_strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    if src.is_null() {
        return 0 as libc::c_int as size_t;
    }
    if dst.is_null() {
        return strlen(src);
    }
    return strlcpy(dst, src, size);
}
