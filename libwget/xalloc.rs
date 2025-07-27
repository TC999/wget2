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
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_realloc_function = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[no_mangle]
pub static mut wget_malloc_fn: Option::<wget_malloc_function> = unsafe {
    Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void)
};
#[no_mangle]
pub static mut wget_calloc_fn: Option::<wget_calloc_function> = unsafe {
    Some(
        calloc as unsafe extern "C" fn(libc::c_ulong, libc::c_ulong) -> *mut libc::c_void,
    )
};
#[no_mangle]
pub static mut wget_realloc_fn: Option::<wget_realloc_function> = unsafe {
    Some(
        realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                libc::c_ulong,
            ) -> *mut libc::c_void,
    )
};
#[no_mangle]
pub static mut wget_free: Option::<wget_free_function> = unsafe {
    Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ())
};
