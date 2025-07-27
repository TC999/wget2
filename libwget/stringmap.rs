#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type wget_hashmap_st;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_hashmap_create(
        max: libc::c_int,
        hash: Option::<wget_hashmap_hash_fn>,
        cmp: Option::<wget_hashmap_compare_fn>,
    ) -> *mut wget_hashmap;
}
pub type __int32_t = libc::c_int;
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_hash_fn = unsafe extern "C" fn(
    *const libc::c_void,
) -> libc::c_uint;
pub type wget_stringmap = wget_hashmap;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn hash_string(mut key: *const libc::c_void) -> libc::c_uint {
    let mut k: *const libc::c_char = key as *const libc::c_char;
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *k != 0 {
        let fresh0 = k;
        k = k.offset(1);
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*fresh0 as libc::c_uchar as libc::c_uint);
    }
    return hash;
}
unsafe extern "C" fn hash_string_nocase(mut key: *const libc::c_void) -> libc::c_uint {
    let mut k: *const libc::c_char = key as *const libc::c_char;
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *k != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(
                ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let fresh1 = k;
                            k = k.offset(1);
                            let mut __c: libc::c_int = *fresh1 as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            let fresh2 = k;
                            k = k.offset(1);
                            __res = tolower(*fresh2 as libc::c_int);
                        }
                    } else {
                        let fresh3 = k;
                        k = k.offset(1);
                        __res = *(*__ctype_tolower_loc())
                            .offset(*fresh3 as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_uchar as libc::c_uint,
            );
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn wget_stringmap_create(
    mut max: libc::c_int,
) -> *mut wget_stringmap {
    return wget_hashmap_create(
        max,
        Some(hash_string as wget_hashmap_hash_fn),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                wget_strcmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_stringmap_create_nocase(
    mut max: libc::c_int,
) -> *mut wget_stringmap {
    return wget_hashmap_create(
        max,
        Some(hash_string_nocase as wget_hashmap_hash_fn),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                wget_strcasecmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
    );
}
