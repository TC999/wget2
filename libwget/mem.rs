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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
}
pub type size_t = libc::c_ulong;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_memdup(
    mut m: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    if m.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut d: *mut libc::c_void = wget_malloc(n);
    if d.is_null() {
        return 0 as *mut libc::c_void;
    }
    return memcpy(d, m, n);
}
#[no_mangle]
pub unsafe extern "C" fn wget_strdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    return (if !s.is_null() {
        wget_memdup(
            s as *const libc::c_void,
            (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wget_strmemdup(
    mut m: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_char {
    if m.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut d: *mut libc::c_void = wget_malloc(
        n.wrapping_add(1 as libc::c_int as size_t),
    );
    if d.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut ret: *mut libc::c_char = memcpy(d, m, n) as *mut libc::c_char;
    *ret.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_strmemcpy(
    mut s: *mut libc::c_char,
    mut ssize: size_t,
    mut m: *const libc::c_void,
    mut n: size_t,
) -> size_t {
    if s.is_null() || ssize == 0 {
        return 0 as libc::c_int as size_t;
    }
    if (n > 0 as libc::c_int as size_t) as libc::c_int as libc::c_long != 0 {
        if n >= ssize {
            n = ssize.wrapping_sub(1 as libc::c_int as size_t);
        }
        if !m.is_null() {
            memmove(s as *mut libc::c_void, m, n);
        } else {
            n = 0 as libc::c_int as size_t;
        }
    }
    *s.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn wget_strmemcpy_a(
    mut s: *mut libc::c_char,
    mut ssize: size_t,
    mut m: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    if n >= ssize {
        s = wget_malloc(n.wrapping_add(1 as libc::c_int as size_t)) as *mut libc::c_char;
        if s.is_null() {
            return 0 as *mut libc::c_void;
        }
    }
    memmove(s as *mut libc::c_void, m, n);
    *s.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    return s as *mut libc::c_void;
}
