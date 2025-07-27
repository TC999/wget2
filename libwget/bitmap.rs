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
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed = -12;
pub const WGET_E_IO: C2RustUnnamed = -11;
pub const WGET_E_OPEN: C2RustUnnamed = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed = -6;
pub const WGET_E_CONNECT: C2RustUnnamed = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed = -4;
pub const WGET_E_INVALID: C2RustUnnamed = -3;
pub const WGET_E_MEMORY: C2RustUnnamed = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed = 0;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_bitmap_st {
    pub bits: uint64_t,
    pub map: [uint64_t; 0],
}
pub type wget_bitmap = wget_bitmap_st;
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bitmap_set(mut b: *mut wget_bitmap, mut n: libc::c_uint) {
    if !b.is_null() && (n as uint64_t) < (*b).bits {
        *((*b).map).as_mut_ptr().offset((n >> 6 as libc::c_int) as isize)
            |= (1 as libc::c_int as uint64_t)
                << (n as libc::c_ulong
                    & (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong));
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_bitmap_clear(
    mut b: *mut wget_bitmap,
    mut n: libc::c_uint,
) {
    if !b.is_null() && (n as uint64_t) < (*b).bits {
        *((*b).map).as_mut_ptr().offset((n >> 6 as libc::c_int) as isize)
            &= !((1 as libc::c_int as uint64_t)
                << (n as libc::c_ulong
                    & (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_bitmap_get(
    mut b: *const wget_bitmap,
    mut n: libc::c_uint,
) -> bool {
    if !b.is_null() && (n as uint64_t) < (*(b as *mut wget_bitmap)).bits {
        return *((*(b as *mut wget_bitmap)).map)
            .as_mut_ptr()
            .offset((n >> 6 as libc::c_int) as isize)
            & (1 as libc::c_int as uint64_t)
                << (n as libc::c_ulong
                    & (::core::mem::size_of::<uint64_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
            != 0 as libc::c_int as uint64_t;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_bitmap_init(
    mut b: *mut *mut wget_bitmap,
    mut bits: libc::c_uint,
) -> libc::c_int {
    if b.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    let mut _b: *mut wget_bitmap = wget_calloc(
        (bits as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<uint64_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<uint64_t>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) as *mut wget_bitmap;
    if _b.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    (*_b).bits = bits as uint64_t;
    *b = _b;
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_bitmap_free(mut b: *mut *mut wget_bitmap) {
    if !b.is_null() {
        if !(*b).is_null() {
            wget_free.expect("non-null function pointer")(*b as *mut libc::c_void);
            *b = 0 as *mut wget_bitmap;
        }
    }
}
