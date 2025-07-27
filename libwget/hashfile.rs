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
    pub type evp_md_ctx_st;
    pub type engine_st;
    pub type evp_md_st;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn wget_strerror(err: wget_error) -> *const libc::c_char;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_memtohex(
        src: *const libc::c_uchar,
        src_len: size_t,
        dst: *mut libc::c_char,
        dst_size: size_t,
    );
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn EVP_MD_get_size(md: *const EVP_MD) -> libc::c_int;
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
    fn EVP_DigestInit_ex(
        ctx: *mut EVP_MD_CTX,
        type_0: *const EVP_MD,
        impl_0: *mut ENGINE,
    ) -> libc::c_int;
    fn EVP_DigestUpdate(
        ctx: *mut EVP_MD_CTX,
        d: *const libc::c_void,
        cnt: size_t,
    ) -> libc::c_int;
    fn EVP_DigestFinal_ex(
        ctx: *mut EVP_MD_CTX,
        md: *mut libc::c_uchar,
        s: *mut libc::c_uint,
    ) -> libc::c_int;
    fn EVP_Digest(
        data: *const libc::c_void,
        count: size_t,
        md: *mut libc::c_uchar,
        size: *mut libc::c_uint,
        type_0: *const EVP_MD,
        impl_0: *mut ENGINE,
    ) -> libc::c_int;
    fn EVP_md5() -> *const EVP_MD;
    fn EVP_sha1() -> *const EVP_MD;
    fn EVP_sha224() -> *const EVP_MD;
    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_sha384() -> *const EVP_MD;
    fn EVP_sha512() -> *const EVP_MD;
    fn EVP_ripemd160() -> *const EVP_MD;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type wget_error = libc::c_int;
pub const WGET_E_UNSUPPORTED: wget_error = -12;
pub const WGET_E_IO: wget_error = -11;
pub const WGET_E_OPEN: wget_error = -10;
pub const WGET_E_XML_PARSE_ERR: wget_error = -9;
pub const WGET_E_TLS_DISABLED: wget_error = -8;
pub const WGET_E_CERTIFICATE: wget_error = -7;
pub const WGET_E_HANDSHAKE: wget_error = -6;
pub const WGET_E_CONNECT: wget_error = -5;
pub const WGET_E_TIMEOUT: wget_error = -4;
pub const WGET_E_INVALID: wget_error = -3;
pub const WGET_E_MEMORY: wget_error = -2;
pub const WGET_E_UNKNOWN: wget_error = -1;
pub const WGET_E_SUCCESS: wget_error = 0;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hash_hd_st {
    pub ctx: *mut EVP_MD_CTX,
}
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type wget_hash_hd = wget_hash_hd_st;
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
pub type ENGINE = engine_st;
pub type EVP_MD = evp_md_st;
pub type evp_md_func = unsafe extern "C" fn() -> *const EVP_MD;
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_get_algorithm(
    mut hashname: *const libc::c_char,
) -> wget_digest_algorithm {
    if !hashname.is_null() {
        if *hashname as libc::c_int == 's' as i32
            || *hashname as libc::c_int == 'S' as i32
        {
            if wget_strcasecmp_ascii(
                hashname,
                b"sha-1\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    hashname,
                    b"sha1\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                return WGET_DIGTYPE_SHA1
            } else if wget_strcasecmp_ascii(
                hashname,
                b"sha-256\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    hashname,
                    b"sha256\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                return WGET_DIGTYPE_SHA256
            } else if wget_strcasecmp_ascii(
                hashname,
                b"sha-512\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    hashname,
                    b"sha512\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                return WGET_DIGTYPE_SHA512
            } else if wget_strcasecmp_ascii(
                hashname,
                b"sha-224\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    hashname,
                    b"sha224\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                return WGET_DIGTYPE_SHA224
            } else if wget_strcasecmp_ascii(
                hashname,
                b"sha-384\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    hashname,
                    b"sha384\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                return WGET_DIGTYPE_SHA384
            }
        } else if wget_strcasecmp_ascii(
            hashname,
            b"md5\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            return WGET_DIGTYPE_MD5
        } else if wget_strcasecmp_ascii(
            hashname,
            b"md2\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            return WGET_DIGTYPE_MD2
        } else if wget_strcasecmp_ascii(
            hashname,
            b"rmd160\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            return WGET_DIGTYPE_RMD160
        }
    }
    wget_error_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Unknown hash type '%s'\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        hashname,
    );
    return WGET_DIGTYPE_UNKNOWN;
}
static mut algorithms: [Option::<evp_md_func>; 9] = unsafe {
    [
        None,
        Some(EVP_md5 as unsafe extern "C" fn() -> *const EVP_MD),
        Some(EVP_sha1 as unsafe extern "C" fn() -> *const EVP_MD),
        Some(EVP_ripemd160 as unsafe extern "C" fn() -> *const EVP_MD),
        None,
        Some(EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD),
        Some(EVP_sha384 as unsafe extern "C" fn() -> *const EVP_MD),
        Some(EVP_sha512 as unsafe extern "C" fn() -> *const EVP_MD),
        Some(EVP_sha224 as unsafe extern "C" fn() -> *const EVP_MD),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn wget_hash_fast(
    mut algorithm: wget_digest_algorithm,
    mut text: *const libc::c_void,
    mut textlen: size_t,
    mut digest: *mut libc::c_void,
) -> libc::c_int {
    if algorithm as libc::c_uint as libc::c_ulong
        >= (::core::mem::size_of::<[Option::<evp_md_func>; 9]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<Option::<evp_md_func>>() as libc::c_ulong,
            )
    {
        return WGET_E_INVALID as libc::c_int;
    }
    let mut evp: Option::<evp_md_func> = algorithms[algorithm as usize];
    if evp.is_none() {
        return WGET_E_UNSUPPORTED as libc::c_int;
    }
    if EVP_Digest(
        text,
        textlen,
        digest as *mut libc::c_uchar,
        0 as *mut libc::c_uint,
        evp.expect("non-null function pointer")(),
        0 as *mut ENGINE,
    ) == 0 as libc::c_int
    {
        return WGET_E_UNKNOWN as libc::c_int;
    }
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_get_len(
    mut algorithm: wget_digest_algorithm,
) -> libc::c_int {
    let mut evp: Option::<evp_md_func> = None;
    if algorithm as libc::c_uint as libc::c_ulong
        >= (::core::mem::size_of::<[Option::<evp_md_func>; 9]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<Option::<evp_md_func>>() as libc::c_ulong,
            )
        || {
            evp = algorithms[algorithm as usize];
            evp.is_none()
        }
    {
        return 0 as libc::c_int;
    }
    return EVP_MD_get_size(evp.expect("non-null function pointer")());
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_init(
    mut handle: *mut *mut wget_hash_hd,
    mut algorithm: wget_digest_algorithm,
) -> libc::c_int {
    let mut evp: Option::<evp_md_func> = None;
    if algorithm as libc::c_uint as libc::c_ulong
        >= (::core::mem::size_of::<[Option::<evp_md_func>; 9]>() as libc::c_ulong)
            .wrapping_div(
                ::core::mem::size_of::<Option::<evp_md_func>>() as libc::c_ulong,
            )
    {
        return WGET_E_UNSUPPORTED as libc::c_int;
    }
    evp = algorithms[algorithm as usize];
    if evp.is_none() {
        return WGET_E_UNSUPPORTED as libc::c_int;
    }
    *handle = wget_malloc(::core::mem::size_of::<wget_hash_hd_st>() as libc::c_ulong)
        as *mut wget_hash_hd;
    if (*handle).is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    (**handle).ctx = EVP_MD_CTX_new();
    if ((**handle).ctx).is_null() {
        if !(*handle).is_null() {
            wget_free.expect("non-null function pointer")(*handle as *mut libc::c_void);
            *handle = 0 as *mut wget_hash_hd;
        }
        return WGET_E_UNKNOWN as libc::c_int;
    }
    if EVP_DigestInit_ex(
        (**handle).ctx,
        evp.expect("non-null function pointer")(),
        0 as *mut ENGINE,
    ) != 0
    {
        return WGET_E_SUCCESS as libc::c_int;
    }
    EVP_MD_CTX_free((**handle).ctx);
    if !(*handle).is_null() {
        wget_free.expect("non-null function pointer")(*handle as *mut libc::c_void);
        *handle = 0 as *mut wget_hash_hd;
    }
    return WGET_E_UNKNOWN as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash(
    mut handle: *mut wget_hash_hd,
    mut text: *const libc::c_void,
    mut textlen: size_t,
) -> libc::c_int {
    if EVP_DigestUpdate((*handle).ctx, text, textlen) != 0 {
        return WGET_E_SUCCESS as libc::c_int;
    }
    return WGET_E_INVALID as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_deinit(
    mut handle: *mut *mut wget_hash_hd,
    mut digest: *mut libc::c_void,
) -> libc::c_int {
    EVP_DigestFinal_ex(
        (**handle).ctx,
        digest as *mut libc::c_uchar,
        0 as *mut libc::c_uint,
    );
    EVP_MD_CTX_free((**handle).ctx);
    if !(*handle).is_null() {
        wget_free.expect("non-null function pointer")(*handle as *mut libc::c_void);
        *handle = 0 as *mut wget_hash_hd;
    }
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_file_fd(
    mut hashname: *const libc::c_char,
    mut fd: libc::c_int,
    mut digest_hex: *mut libc::c_char,
    mut digest_hex_size: size_t,
    mut offset: off_t,
    mut length: off_t,
) -> libc::c_int {
    let mut algorithm: wget_digest_algorithm = WGET_DIGTYPE_UNKNOWN;
    let mut ret: libc::c_int = WGET_E_UNKNOWN as libc::c_int;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if digest_hex_size != 0 {
        *digest_hex = 0 as libc::c_int as libc::c_char;
    }
    if fd == -(1 as libc::c_int) || fstat(fd, &mut st) != 0 as libc::c_int {
        return WGET_E_IO as libc::c_int;
    }
    if length == 0 as libc::c_int as off_t {
        length = st.st_size;
    }
    if offset + length > st.st_size {
        return WGET_E_INVALID as libc::c_int;
    }
    wget_debug_printf(
        b"%s hashing pos %llu, length %llu...\n\0" as *const u8 as *const libc::c_char,
        hashname,
        offset as libc::c_ulonglong,
        length as libc::c_ulonglong,
    );
    algorithm = wget_hash_get_algorithm(hashname);
    if algorithm as libc::c_uint != WGET_DIGTYPE_UNKNOWN as libc::c_int as libc::c_uint {
        let mut digest: [libc::c_uchar; 256] = [0; 256];
        let mut digestlen: size_t = wget_hash_get_len(algorithm) as size_t;
        if digestlen > ::core::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unexpected hash len %zu > %zu\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 18],
                    &[libc::c_char; 18],
                >(b"wget_hash_file_fd\0"))
                    .as_ptr(),
                digestlen,
                ::core::mem::size_of::<[libc::c_uchar; 256]>() as libc::c_ulong,
            );
            return ret;
        }
        let mut buf: *mut libc::c_char = mmap(
            0 as *mut libc::c_void,
            length as size_t,
            0x1 as libc::c_int,
            0x2 as libc::c_int,
            fd,
            offset,
        ) as *mut libc::c_char;
        if buf != -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char {
            if wget_hash_fast(
                algorithm,
                buf as *const libc::c_void,
                length as size_t,
                digest.as_mut_ptr() as *mut libc::c_void,
            ) == 0 as libc::c_int
            {
                wget_memtohex(
                    digest.as_mut_ptr(),
                    digestlen,
                    digest_hex,
                    digest_hex_size,
                );
                wget_debug_printf(
                    b"  hash %s\0" as *const u8 as *const libc::c_char,
                    digest_hex,
                );
                ret = WGET_E_SUCCESS as libc::c_int;
            }
            munmap(buf as *mut libc::c_void, length as size_t);
        } else {
            let mut nbytes: ssize_t = 0 as libc::c_int as ssize_t;
            let mut dig: *mut wget_hash_hd = 0 as *mut wget_hash_hd;
            let mut tmp: [libc::c_char; 65536] = [0; 65536];
            ret = wget_hash_init(&mut dig, algorithm);
            if ret != 0 {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Hash init failed for type '%s': %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"wget_hash_file_fd\0"))
                        .as_ptr(),
                    hashname,
                    wget_strerror(ret as wget_error),
                );
                return ret;
            }
            while length > 0 as libc::c_int as off_t
                && {
                    nbytes = read(
                        fd,
                        tmp.as_mut_ptr() as *mut libc::c_void,
                        ::core::mem::size_of::<[libc::c_char; 65536]>() as libc::c_ulong,
                    );
                    nbytes > 0 as libc::c_int as ssize_t
                }
            {
                ret = wget_hash(
                    dig,
                    tmp.as_mut_ptr() as *const libc::c_void,
                    nbytes as size_t,
                );
                if ret != 0 {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: Hash update failed: %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*::core::mem::transmute::<
                            &[u8; 18],
                            &[libc::c_char; 18],
                        >(b"wget_hash_file_fd\0"))
                            .as_ptr(),
                        wget_strerror(ret as wget_error),
                    );
                    return ret;
                }
                if nbytes <= length {
                    length -= nbytes;
                } else {
                    length = 0 as libc::c_int as off_t;
                }
            }
            ret = wget_hash_deinit(&mut dig, digest.as_mut_ptr() as *mut libc::c_void);
            if ret != 0 {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Hash finalization failed: %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"wget_hash_file_fd\0"))
                        .as_ptr(),
                    wget_strerror(ret as wget_error),
                );
                return ret;
            }
            if nbytes < 0 as libc::c_int as ssize_t {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Failed to read %llu bytes\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*::core::mem::transmute::<
                        &[u8; 18],
                        &[libc::c_char; 18],
                    >(b"wget_hash_file_fd\0"))
                        .as_ptr(),
                    length as libc::c_ulonglong,
                );
                return WGET_E_IO as libc::c_int;
            }
            wget_memtohex(digest.as_mut_ptr(), digestlen, digest_hex, digest_hex_size);
            ret = WGET_E_SUCCESS as libc::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_file_offset(
    mut hashname: *const libc::c_char,
    mut fname: *const libc::c_char,
    mut digest_hex: *mut libc::c_char,
    mut digest_hex_size: size_t,
    mut offset: off_t,
    mut length: off_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    fd = open(fname, 0 as libc::c_int | 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        if digest_hex_size != 0 {
            *digest_hex = 0 as libc::c_int as libc::c_char;
        }
        return 0 as libc::c_int;
    }
    ret = wget_hash_file_fd(hashname, fd, digest_hex, digest_hex_size, offset, length);
    close(fd);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hash_file(
    mut hashname: *const libc::c_char,
    mut fname: *const libc::c_char,
    mut digest_hex: *mut libc::c_char,
    mut digest_hex_size: size_t,
) -> libc::c_int {
    return wget_hash_file_offset(
        hashname,
        fname,
        digest_hex,
        digest_hex_size,
        0 as libc::c_int as off_t,
        0 as libc::c_int as off_t,
    );
}
