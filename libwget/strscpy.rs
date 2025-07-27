#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = libc::c_ulong;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[no_mangle]
pub unsafe extern "C" fn wget_strscpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> ssize_t {
    if dst.is_null() as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    if src.is_null() as libc::c_int as libc::c_long != 0 {
        if size != 0 {
            *dst = 0 as libc::c_int as libc::c_char;
            return 0 as libc::c_int as ssize_t;
        } else {
            return -(1 as libc::c_int) as ssize_t
        }
    }
    let mut old: *const libc::c_char = src;
    if (size != 0) as libc::c_int as libc::c_long != 0 {
        loop {
            size = size.wrapping_sub(1);
            if !(size != 0) {
                break;
            }
            let fresh0 = src;
            src = src.offset(1);
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = *fresh0;
            if *fresh1 == 0 {
                return src.offset_from(old) as libc::c_long
                    - 1 as libc::c_int as libc::c_long;
            }
        }
        *dst = 0 as libc::c_int as libc::c_char;
        return src.offset_from(old) as libc::c_long;
    }
    return -(1 as libc::c_int) as ssize_t;
}
