#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_vasprintf(
        strp: *mut *mut libc::c_char,
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
static mut base64_2_bin: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn isbase64(mut c: libc::c_char) -> bool {
    return base64_2_bin[c as libc::c_uchar as usize] as libc::c_int != 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_is_string(mut src: *const libc::c_char) -> bool {
    if !src.is_null() {
        while isbase64(*src) {
            src = src.offset(1);
            src;
        }
        if *src == 0
            || *src as libc::c_int == '=' as i32
                && *src.offset(1 as libc::c_int as isize) as libc::c_int != 0
            || *src as libc::c_int == '=' as i32
                && *src.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
                && *src.offset(2 as libc::c_int as isize) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_decode(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) -> size_t {
    let mut usrc: *const libc::c_uchar = src as *const libc::c_uchar;
    let mut old: *mut libc::c_char = dst;
    let mut extra: libc::c_int = 0;
    while n > 0 as libc::c_int as size_t
        && !isbase64(*src.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize))
    {
        n = n.wrapping_sub(1);
        n;
    }
    extra = (n & 3 as libc::c_int as size_t) as libc::c_int;
    n = n / 4 as libc::c_int as size_t;
    while n > 0 as libc::c_int as size_t {
        let fresh0 = dst;
        dst = dst.offset(1);
        *fresh0 = ((base64_2_bin[*usrc.offset(0 as libc::c_int as isize) as usize]
            as libc::c_int) << 2 as libc::c_int
            | base64_2_bin[*usrc.offset(1 as libc::c_int as isize) as usize]
                as libc::c_int >> 4 as libc::c_int) as libc::c_char;
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = ((base64_2_bin[*usrc.offset(1 as libc::c_int as isize) as usize]
            as libc::c_int & 0xf as libc::c_int) << 4 as libc::c_int
            | base64_2_bin[*usrc.offset(2 as libc::c_int as isize) as usize]
                as libc::c_int >> 2 as libc::c_int) as libc::c_char;
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = ((base64_2_bin[*usrc.offset(2 as libc::c_int as isize) as usize]
            as libc::c_int & 0x3 as libc::c_int) << 6 as libc::c_int
            | base64_2_bin[*usrc.offset(3 as libc::c_int as isize) as usize]
                as libc::c_int) as libc::c_char;
        n = n.wrapping_sub(1);
        n;
        usrc = usrc.offset(4 as libc::c_int as isize);
    }
    match extra {
        1 => {
            let fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = ((base64_2_bin[*usrc.offset(0 as libc::c_int as isize) as usize]
                as libc::c_int) << 2 as libc::c_int) as libc::c_char;
        }
        2 => {
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = ((base64_2_bin[*usrc.offset(0 as libc::c_int as isize) as usize]
                as libc::c_int) << 2 as libc::c_int
                | base64_2_bin[*usrc.offset(1 as libc::c_int as isize) as usize]
                    as libc::c_int >> 4 as libc::c_int) as libc::c_char;
            *dst = ((base64_2_bin[*usrc.offset(1 as libc::c_int as isize) as usize]
                as libc::c_int & 0xf as libc::c_int) << 4 as libc::c_int)
                as libc::c_char;
            if *dst != 0 {
                dst = dst.offset(1);
                dst;
            }
        }
        3 => {
            let fresh5 = dst;
            dst = dst.offset(1);
            *fresh5 = ((base64_2_bin[*usrc.offset(0 as libc::c_int as isize) as usize]
                as libc::c_int) << 2 as libc::c_int
                | base64_2_bin[*usrc.offset(1 as libc::c_int as isize) as usize]
                    as libc::c_int >> 4 as libc::c_int) as libc::c_char;
            let fresh6 = dst;
            dst = dst.offset(1);
            *fresh6 = ((base64_2_bin[*usrc.offset(1 as libc::c_int as isize) as usize]
                as libc::c_int & 0xf as libc::c_int) << 4 as libc::c_int
                | base64_2_bin[*usrc.offset(2 as libc::c_int as isize) as usize]
                    as libc::c_int >> 2 as libc::c_int) as libc::c_char;
            *dst = ((base64_2_bin[*usrc.offset(2 as libc::c_int as isize) as usize]
                as libc::c_int & 0x3 as libc::c_int) << 6 as libc::c_int)
                as libc::c_char;
            if *dst != 0 {
                dst = dst.offset(1);
                dst;
            }
        }
        _ => {}
    }
    *dst = 0 as libc::c_int as libc::c_char;
    return dst.offset_from(old) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_decode_alloc(
    mut src: *const libc::c_char,
    mut n: size_t,
    mut outlen: *mut size_t,
) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = wget_malloc(wget_base64_get_decoded_length(n))
        as *mut libc::c_char;
    if dst.is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut _outlen: size_t = wget_base64_decode(dst, src, n);
    if !outlen.is_null() {
        *outlen = _outlen;
    }
    return dst;
}
unsafe extern "C" fn base64_encode(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
    mut flags: libc::c_int,
) -> size_t {
    static mut base64unsafe: [libc::c_char; 64] = unsafe {
        *::core::mem::transmute::<
            &[u8; 64],
            &[libc::c_char; 64],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
    };
    static mut base64urlsafe: [libc::c_char; 64] = unsafe {
        *::core::mem::transmute::<
            &[u8; 64],
            &[libc::c_char; 64],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_")
    };
    let mut base64: *const libc::c_char = if flags & 1 as libc::c_int != 0 {
        base64urlsafe.as_ptr()
    } else {
        base64unsafe.as_ptr()
    };
    let mut usrc: *const libc::c_uchar = src as *const libc::c_uchar;
    let mut start: *mut libc::c_char = dst;
    let mut extra: libc::c_int = (n % 3 as libc::c_int as size_t) as libc::c_int;
    n = n / 3 as libc::c_int as size_t;
    while n > 0 as libc::c_int as size_t {
        let fresh7 = dst;
        dst = dst.offset(1);
        *fresh7 = *base64
            .offset(
                (*usrc.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as isize,
            );
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = *base64
            .offset(
                ((*usrc.offset(0 as libc::c_int as isize) as libc::c_int
                    & 3 as libc::c_int) << 4 as libc::c_int
                    | *usrc.offset(1 as libc::c_int as isize) as libc::c_int
                        >> 4 as libc::c_int) as isize,
            );
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = *base64
            .offset(
                ((*usrc.offset(1 as libc::c_int as isize) as libc::c_int
                    & 15 as libc::c_int) << 2 as libc::c_int
                    | *usrc.offset(2 as libc::c_int as isize) as libc::c_int
                        >> 6 as libc::c_int) as isize,
            );
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = *base64
            .offset(
                (*usrc.offset(2 as libc::c_int as isize) as libc::c_int
                    & 0x3f as libc::c_int) as isize,
            );
        n = n.wrapping_sub(1);
        n;
        usrc = usrc.offset(3 as libc::c_int as isize);
    }
    if extra == 1 as libc::c_int {
        let fresh11 = dst;
        dst = dst.offset(1);
        *fresh11 = *base64
            .offset(
                (*usrc.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as isize,
            );
        let fresh12 = dst;
        dst = dst.offset(1);
        *fresh12 = *base64
            .offset(
                ((*usrc.offset(0 as libc::c_int as isize) as libc::c_int
                    & 3 as libc::c_int) << 4 as libc::c_int) as isize,
            );
        let fresh13 = dst;
        dst = dst.offset(1);
        *fresh13 = '=' as i32 as libc::c_char;
        let fresh14 = dst;
        dst = dst.offset(1);
        *fresh14 = '=' as i32 as libc::c_char;
    } else if extra == 2 as libc::c_int {
        let fresh15 = dst;
        dst = dst.offset(1);
        *fresh15 = *base64
            .offset(
                (*usrc.offset(0 as libc::c_int as isize) as libc::c_int
                    >> 2 as libc::c_int) as isize,
            );
        let fresh16 = dst;
        dst = dst.offset(1);
        *fresh16 = *base64
            .offset(
                ((*usrc.offset(0 as libc::c_int as isize) as libc::c_int
                    & 3 as libc::c_int) << 4 as libc::c_int
                    | *usrc.offset(1 as libc::c_int as isize) as libc::c_int
                        >> 4 as libc::c_int) as isize,
            );
        let fresh17 = dst;
        dst = dst.offset(1);
        *fresh17 = *base64
            .offset(
                ((*usrc.offset(1 as libc::c_int as isize) as libc::c_int
                    & 15 as libc::c_int) << 2 as libc::c_int) as isize,
            );
        let fresh18 = dst;
        dst = dst.offset(1);
        *fresh18 = '=' as i32 as libc::c_char;
    }
    *dst = 0 as libc::c_int as libc::c_char;
    return dst.offset_from(start) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_encode(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) -> size_t {
    return base64_encode(dst, src, n, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_urlencode(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut n: size_t,
) -> size_t {
    return base64_encode(dst, src, n, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_encode_alloc(
    mut src: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = wget_malloc(wget_base64_get_encoded_length(n))
        as *mut libc::c_char;
    if !dst.is_null() {
        wget_base64_encode(dst, src, n);
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_encode_vprintf_alloc(
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    n = wget_vasprintf(&mut data, fmt, args.as_va_list());
    if !data.is_null() {
        let mut dst: *mut libc::c_char = wget_base64_encode_alloc(data, n);
        if !data.is_null() {
            wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
            data = 0 as *mut libc::c_char;
        }
        return dst;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_encode_printf_alloc(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    dst = wget_base64_encode_vprintf_alloc(fmt, args_0.as_va_list());
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_get_decoded_length(mut len: size_t) -> size_t {
    return (len.wrapping_add(3 as libc::c_int as size_t) / 4 as libc::c_int as size_t
        * 3 as libc::c_int as size_t)
        .wrapping_add(1 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn wget_base64_get_encoded_length(mut len: size_t) -> size_t {
    return (len.wrapping_add(2 as libc::c_int as size_t) / 3 as libc::c_int as size_t
        * 4 as libc::c_int as size_t)
        .wrapping_add(1 as libc::c_int as size_t);
}
