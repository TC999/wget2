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
    fn wget_memtohex(
        src: *const libc::c_uchar,
        src_len: size_t,
        dst: *mut libc::c_char,
        dst_size: size_t,
    );
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_vasprintf(
        strp: *mut *mut libc::c_char,
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_hash_fast(
        algorithm: wget_digest_algorithm,
        text: *const libc::c_void,
        textlen: size_t,
        digest: *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_hash_get_len(algorithm: wget_digest_algorithm) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type wget_digest_algorithm = libc::c_uint;
pub const WGET_DIGTYPE_MAX: wget_digest_algorithm = 9;
pub const WGET_DIGTYPE_SHA224: wget_digest_algorithm = 8;
pub const WGET_DIGTYPE_SHA512: wget_digest_algorithm = 7;
pub const WGET_DIGTYPE_SHA384: wget_digest_algorithm = 6;
pub const WGET_DIGTYPE_SHA256: wget_digest_algorithm = 5;
pub const WGET_DIGTYPE_MD2: wget_digest_algorithm = 4;
pub const WGET_DIGTYPE_RMD160: wget_digest_algorithm = 3;
pub const WGET_DIGTYPE_SHA1: wget_digest_algorithm = 2;
pub const WGET_DIGTYPE_MD5: wget_digest_algorithm = 1;
pub const WGET_DIGTYPE_UNKNOWN: wget_digest_algorithm = 0;
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_printf_hex(
    mut algorithm: wget_digest_algorithm,
    mut out: *mut libc::c_char,
    mut outsize: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut plaintext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut len: size_t = 0;
    args_0 = args.clone();
    len = wget_vasprintf(&mut plaintext, fmt, args_0.as_va_list());
    if plaintext.is_null() {
        return;
    }
    let mut tmp: [libc::c_uchar; 256] = [0; 256];
    let mut digest: *mut libc::c_uchar = tmp.as_mut_ptr();
    let mut digestlen: size_t = wget_hash_get_len(algorithm) as size_t;
    if digestlen > ::core::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong {
        digest = wget_malloc(digestlen) as *mut libc::c_uchar;
    }
    if !digest.is_null() {
        let mut rc: libc::c_int = 0;
        rc = wget_hash_fast(
            algorithm,
            plaintext as *const libc::c_void,
            len,
            digest as *mut libc::c_void,
        );
        if rc == 0 as libc::c_int {
            wget_memtohex(digest, digestlen, out, outsize);
        } else {
            *out = 0 as libc::c_int as libc::c_char;
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to hash (%d)\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                rc,
            );
        }
        if digest != tmp.as_mut_ptr() {
            if !digest.is_null() {
                wget_free
                    .expect("non-null function pointer")(digest as *mut libc::c_void);
                digest = 0 as *mut libc::c_uchar;
            }
        }
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to malloc %zu bytes\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*::core::mem::transmute::<
                &[u8; 21],
                &[libc::c_char; 21],
            >(b"wget_hash_printf_hex\0"))
                .as_ptr(),
            digestlen,
        );
    }
    if !plaintext.is_null() {
        wget_free.expect("non-null function pointer")(plaintext as *mut libc::c_void);
        plaintext = 0 as *mut libc::c_char;
    }
}
