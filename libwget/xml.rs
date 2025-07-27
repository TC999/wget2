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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strlcpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> size_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_memcat(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_buffer {
    pub data: *mut libc::c_char,
    pub length: size_t,
    pub size: size_t,
    #[bitfield(name = "release_data", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "release_buf", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "error", ty = "bool", bits = "2..=2")]
    pub release_data_release_buf_error: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type wget_xml_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    libc::c_int,
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
    size_t,
    size_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xml_context {
    pub buf: *const libc::c_char,
    pub p: *const libc::c_char,
    pub token: *const libc::c_char,
    pub hints: libc::c_int,
    pub token_size: size_t,
    pub token_len: size_t,
    pub user_ctx: *mut libc::c_void,
    pub callback: Option::<wget_xml_callback>,
}
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
unsafe extern "C" fn getToken(mut context: *mut xml_context) -> *const libc::c_char {
    let mut c: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        c = *(*context).p as libc::c_int;
        if !(c != 0
            && (c == ' ' as i32 || c >= 9 as libc::c_int && c <= 13 as libc::c_int))
        {
            break;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
    }
    if c == 0 {
        return 0 as *const libc::c_char;
    }
    let fresh0 = (*context).p;
    (*context).p = ((*context).p).offset(1);
    (*context).token = fresh0;
    if c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32
        || c == '_' as i32
    {
        loop {
            c = *(*context).p as libc::c_int;
            if !(c != 0
                && !(c == ' ' as i32 || c >= 9 as libc::c_int && c <= 13 as libc::c_int)
                && c != '>' as i32 && c != '=' as i32)
            {
                break;
            }
            (*context).p = ((*context).p).offset(1);
            (*context).p;
        }
        if c == 0 {
            return 0 as *const libc::c_char;
        }
        (*context)
            .token_len = ((*context).p).offset_from((*context).token) as libc::c_long
            as size_t;
        return (*context).token;
    }
    if c == '/' as i32 {
        c = *(*context).p as libc::c_int;
        if c == 0 {
            return 0 as *const libc::c_char;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
        if c == '>' as i32 {
            (*context).token_len = 2 as libc::c_int as size_t;
            return (*context).token;
        } else {
            return 0 as *const libc::c_char
        }
    }
    if c == '"' as i32 || c == '\'' as i32 {
        let mut quote: libc::c_int = c;
        (*context).token = (*context).p;
        p = strchr((*context).p, quote);
        if p.is_null() {
            return 0 as *const libc::c_char;
        }
        (*context).p = p.offset(1 as libc::c_int as isize);
        (*context)
            .token_len = (((*context).p).offset_from((*context).token) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        return (*context).token;
    }
    if c == '<' as i32 {
        c = *(*context).p as libc::c_int;
        if c == 0 {
            return 0 as *const libc::c_char;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
        if c == '?' as i32 || c == '/' as i32 {
            (*context).token_len = 2 as libc::c_int as size_t;
            return (*context).token;
        }
        if c == '!' as i32 {
            c = *(*context).p as libc::c_int;
            if c == 0 {
                return 0 as *const libc::c_char;
            }
            if c == '-' as i32 {
                (*context).p = ((*context).p).offset(1);
                (*context).p;
                c = *(*context).p as libc::c_int;
                if c == 0 {
                    return 0 as *const libc::c_char;
                }
                (*context).p = ((*context).p).offset(1);
                (*context).p;
                if c == '-' as i32 {
                    (*context).token_len = 4 as libc::c_int as size_t;
                    return (*context).token;
                } else {
                    (*context).p = ((*context).p).offset(-(2 as libc::c_int as isize));
                    (*context).token_len = 2 as libc::c_int as size_t;
                    return (*context).token;
                }
            } else {
                (*context).token_len = 2 as libc::c_int as size_t;
                return (*context).token;
            }
        } else {
            (*context).p = ((*context).p).offset(-1);
            (*context).p;
            (*context).token_len = 1 as libc::c_int as size_t;
            return (*context).token;
        }
    }
    if c == '>' as i32 || c == '=' as i32 {
        (*context).token_len = 1 as libc::c_int as size_t;
        return (*context).token;
    }
    if c == '-' as i32 {
        c = *(*context).p as libc::c_int;
        if c == 0 {
            return 0 as *const libc::c_char;
        }
        if c != '-' as i32 {
            c = '-' as i32;
        } else {
            (*context).p = ((*context).p).offset(1);
            (*context).p;
            c = *(*context).p as libc::c_int;
            if c == 0 {
                return 0 as *const libc::c_char;
            }
            (*context).p = ((*context).p).offset(1);
            (*context).p;
            if c != '>' as i32 {
                (*context).p = ((*context).p).offset(-(2 as libc::c_int as isize));
                c = '-' as i32;
            } else {
                (*context).token_len = 3 as libc::c_int as size_t;
                return (*context).token;
            }
        }
    }
    if c == '?' as i32 {
        c = *(*context).p as libc::c_int;
        if c == 0 {
            return 0 as *const libc::c_char;
        }
        if c != '>' as i32 {} else {
            (*context).p = ((*context).p).offset(1);
            (*context).p;
            (*context).token_len = 2 as libc::c_int as size_t;
            return (*context).token;
        }
    }
    loop {
        c = *(*context).p as libc::c_int;
        if !(c != 0
            && !(c == ' ' as i32 || c >= 9 as libc::c_int && c <= 13 as libc::c_int))
        {
            break;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
    }
    if c != 0 {
        wget_debug_printf(
            b"getToken =%.*s\n\0" as *const u8 as *const libc::c_char,
            ((*context).p).offset_from((*context).token) as libc::c_long as libc::c_int,
            (*context).token,
        );
        (*context)
            .token_len = ((*context).p).offset_from((*context).token) as libc::c_long
            as size_t;
        return (*context).token;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn getHTMLValue(mut context: *mut xml_context) -> *const libc::c_char {
    let mut c: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    loop {
        c = *(*context).p as libc::c_int;
        if !(c != 0
            && (c == ' ' as i32 || c >= 9 as libc::c_int && c <= 13 as libc::c_int))
        {
            break;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
    }
    if c == 0 {
        return 0 as *const libc::c_char;
    }
    let fresh1 = (*context).p;
    (*context).p = ((*context).p).offset(1);
    (*context).token = fresh1;
    if c == '"' as i32 || c == '\'' as i32 || c == '`' as i32 {
        let mut quote: libc::c_int = c;
        (*context).token = (*context).p;
        p = strchr((*context).p, quote);
        if p.is_null() {
            return 0 as *const libc::c_char;
        }
        (*context).p = p.offset(1 as libc::c_int as isize);
        (*context)
            .token_len = (((*context).p).offset_from((*context).token) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as size_t;
        return (*context).token;
    }
    loop {
        c = *(*context).p as libc::c_int;
        if !(c != 0
            && !(c == ' ' as i32 || c >= 9 as libc::c_int && c <= 13 as libc::c_int)
            && c != '<' as i32 && c != '>' as i32
            && !(c == '/' as i32 && *(*context).p as libc::c_int == '>' as i32))
        {
            break;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
    }
    if c != 0 {
        wget_debug_printf(
            b"getHTMLValue =%.*s\n\0" as *const u8 as *const libc::c_char,
            ((*context).p).offset_from((*context).token) as libc::c_long as libc::c_int,
            (*context).token,
        );
        (*context)
            .token_len = ((*context).p).offset_from((*context).token) as libc::c_long
            as size_t;
        return (*context).token;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn getValue(mut context: *mut xml_context) -> libc::c_int {
    let mut c: libc::c_int = 0;
    (*context).token_len = 0 as libc::c_int as size_t;
    (*context).token = (*context).p;
    loop {
        c = *(*context).p as libc::c_int;
        if !(c != 0
            && (c == ' ' as i32 || c >= 9 as libc::c_int && c <= 13 as libc::c_int))
        {
            break;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
    }
    if c == 0 {
        return -(1 as libc::c_int);
    }
    if c == '=' as i32 {
        (*context).p = ((*context).p).offset(1);
        (*context).p;
        if (*context).hints & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            if (getHTMLValue(context)).is_null() {
                return -(1 as libc::c_int)
            } else {
                return 1 as libc::c_int
            }
        }
        if (getToken(context)).is_null() {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    }
    (*context).token = (*context).p;
    return 1 as libc::c_int;
}
unsafe extern "C" fn getScriptContent(
    mut context: *mut xml_context,
) -> *const libc::c_char {
    let mut comment: libc::c_int = 0 as libc::c_int;
    let mut length_valid: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    (*context).token = (*context).p;
    p = (*context).token;
    while *p != 0 {
        if comment != 0 {
            if *p as libc::c_int == '-' as i32
                && strncmp(
                    p,
                    b"-->\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0
            {
                p = p.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
                comment = 0 as libc::c_int;
            }
        } else if *p as libc::c_int == '<' as i32
            && strncmp(
                p,
                b"<!--\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            p = p.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            comment = 1 as libc::c_int;
        } else if *p as libc::c_int == '<' as i32
            && wget_strncasecmp_ascii(
                p,
                b"</script\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as size_t,
            ) == 0
        {
            (*context)
                .token_len = p.offset_from((*context).token) as libc::c_long as size_t;
            length_valid = 1 as libc::c_int;
            p = p.offset(8 as libc::c_int as isize);
            while *p as libc::c_int == ' ' as i32
                || *p as libc::c_int >= 9 as libc::c_int
                    && *p as libc::c_int <= 13 as libc::c_int
            {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '>' as i32 {
                p = p.offset(1);
                p;
                break;
            } else if *p == 0 {
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    (*context).p = p;
    if length_valid == 0 {
        (*context).token_len = p.offset_from((*context).token) as libc::c_long as size_t;
    }
    if *p == 0 && (*context).token_len == 0 {
        return 0 as *const libc::c_char;
    }
    if ((*context).callback).is_some() {
        ((*context).callback)
            .expect(
                "non-null function pointer",
            )(
            (*context).user_ctx,
            (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
            b"script\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            (*context).token,
            (*context).token_len,
            ((*context).token).offset_from((*context).buf) as libc::c_long as size_t,
        );
    }
    return (*context).token;
}
unsafe extern "C" fn getStyleContent(
    mut context: *mut xml_context,
) -> *const libc::c_char {
    let mut comment: libc::c_int = 0 as libc::c_int;
    let mut length_valid: libc::c_int = 0 as libc::c_int;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    (*context).token = (*context).p;
    p = (*context).token;
    while *p != 0 {
        if comment != 0 {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                p = p.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
                comment = 0 as libc::c_int;
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
        {
            p = p.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            comment = 1 as libc::c_int;
        } else if *p as libc::c_int == '<' as i32
            && wget_strncasecmp_ascii(
                p,
                b"</style\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as size_t,
            ) == 0
        {
            (*context)
                .token_len = p.offset_from((*context).token) as libc::c_long as size_t;
            length_valid = 1 as libc::c_int;
            p = p.offset(7 as libc::c_int as isize);
            while *p as libc::c_int == ' ' as i32
                || *p as libc::c_int >= 9 as libc::c_int
                    && *p as libc::c_int <= 13 as libc::c_int
            {
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '>' as i32 {
                p = p.offset(1);
                p;
                break;
            } else if *p == 0 {
                break;
            }
        }
        p = p.offset(1);
        p;
    }
    (*context).p = p;
    if length_valid == 0 {
        (*context).token_len = p.offset_from((*context).token) as libc::c_long as size_t;
    }
    if *p == 0 && (*context).token_len == 0 {
        return 0 as *const libc::c_char;
    }
    if ((*context).callback).is_some() {
        ((*context).callback)
            .expect(
                "non-null function pointer",
            )(
            (*context).user_ctx,
            (1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
            b"style\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
            (*context).token,
            (*context).token_len,
            ((*context).token).offset_from((*context).buf) as libc::c_long as size_t,
        );
    }
    return (*context).token;
}
unsafe extern "C" fn getUnparsed(
    mut context: *mut xml_context,
    mut flags: libc::c_int,
    mut end: *const libc::c_char,
    mut len: size_t,
    mut directory: *const libc::c_char,
) -> *const libc::c_char {
    let mut c: libc::c_int = 0;
    if len == 1 as libc::c_int as size_t {
        (*context).token = (*context).p;
        loop {
            c = *(*context).p as libc::c_int;
            if !(c != 0 && c != *end as libc::c_int) {
                break;
            }
            (*context).p = ((*context).p).offset(1);
            (*context).p;
        }
    } else {
        (*context).token = (*context).p;
        loop {
            c = *(*context).p as libc::c_int;
            if !(c != 0) {
                break;
            }
            if c == *end as libc::c_int
                && *((*context).p).offset(1 as libc::c_int as isize) as libc::c_int
                    == *end.offset(1 as libc::c_int as isize) as libc::c_int
                && (len == 2 as libc::c_int as size_t
                    || *((*context).p).offset(2 as libc::c_int as isize) as libc::c_int
                        == *end.offset(2 as libc::c_int as isize) as libc::c_int)
            {
                break;
            }
            (*context).p = ((*context).p).offset(1);
            (*context).p;
        }
    }
    (*context)
        .token_len = ((*context).p).offset_from((*context).token) as libc::c_long
        as size_t;
    if c != 0 {
        (*context).p = ((*context).p).offset(len as isize);
    }
    if c == 0 && (*context).token_len == 0 {
        return 0 as *const libc::c_char;
    }
    if ((*context).callback).is_some() {
        ((*context).callback)
            .expect(
                "non-null function pointer",
            )(
            (*context).user_ctx,
            flags,
            directory,
            0 as *const libc::c_char,
            (*context).token,
            (*context).token_len,
            ((*context).token).offset_from((*context).buf) as libc::c_long as size_t,
        );
    }
    return (*context).token;
}
unsafe extern "C" fn getComment(mut context: *mut xml_context) -> *const libc::c_char {
    return getUnparsed(
        context,
        (1 as libc::c_int) << 5 as libc::c_int,
        b"-->\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn getProcessing(
    mut context: *mut xml_context,
) -> *const libc::c_char {
    return getUnparsed(
        context,
        (1 as libc::c_int) << 7 as libc::c_int,
        b"?>\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn getSpecial(mut context: *mut xml_context) -> *const libc::c_char {
    return getUnparsed(
        context,
        (1 as libc::c_int) << 8 as libc::c_int,
        b">\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn getContent(
    mut context: *mut xml_context,
    mut directory: *const libc::c_char,
) -> *const libc::c_char {
    let mut c: libc::c_int = 0;
    (*context).token = (*context).p;
    loop {
        c = *(*context).p as libc::c_int;
        if !(c != 0 && c != '<' as i32) {
            break;
        }
        (*context).p = ((*context).p).offset(1);
        (*context).p;
    }
    (*context)
        .token_len = ((*context).p).offset_from((*context).token) as libc::c_long
        as size_t;
    if c == 0 && (*context).token_len == 0 {
        return 0 as *const libc::c_char;
    }
    if ((*context).callback).is_some() && (*context).token_len != 0 {
        ((*context).callback)
            .expect(
                "non-null function pointer",
            )(
            (*context).user_ctx,
            (1 as libc::c_int) << 4 as libc::c_int,
            directory,
            0 as *const libc::c_char,
            (*context).token,
            (*context).token_len,
            ((*context).token).offset_from((*context).buf) as libc::c_long as size_t,
        );
    }
    return (*context).token;
}
unsafe extern "C" fn parseXML(
    mut dir: *const libc::c_char,
    mut context: *mut xml_context,
) -> libc::c_int {
    let mut tok: *const libc::c_char = 0 as *const libc::c_char;
    let mut directory: [libc::c_char; 256] = *::core::mem::transmute::<
        &[u8; 256],
        &mut [libc::c_char; 256],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut pos: size_t = 0 as libc::c_int as size_t;
    if (*context).hints & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        pos = wget_strlcpy(
            directory.as_mut_ptr(),
            dir,
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        if pos >= ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
            pos = (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
        }
    }
    loop {
        getContent(context, directory.as_mut_ptr());
        if (*context).token_len != 0 {
            wget_debug_printf(
                b"%s='%.*s'\n\0" as *const u8 as *const libc::c_char,
                directory.as_mut_ptr(),
                (*context).token_len as libc::c_int,
                (*context).token,
            );
        }
        tok = getToken(context);
        if tok.is_null() {
            return WGET_E_SUCCESS as libc::c_int;
        }
        if (*context).token_len == 1 as libc::c_int as size_t
            && *tok as libc::c_int == '<' as i32
        {
            let mut flags: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int;
            tok = getToken(context);
            if tok.is_null() {
                return WGET_E_XML_PARSE_ERR as libc::c_int;
            }
            if (*context).hints & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                if pos == 0
                    || directory[pos.wrapping_sub(1 as libc::c_int as size_t) as usize]
                        as libc::c_int != '/' as i32
                {
                    wget_snprintf(
                        &mut *directory.as_mut_ptr().offset(pos as isize)
                            as *mut libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(pos),
                        b"/%.*s\0" as *const u8 as *const libc::c_char,
                        (*context).token_len as libc::c_int,
                        tok,
                    );
                } else {
                    wget_snprintf(
                        &mut *directory.as_mut_ptr().offset(pos as isize)
                            as *mut libc::c_char,
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(pos),
                        b"%.*s\0" as *const u8 as *const libc::c_char,
                        (*context).token_len as libc::c_int,
                        tok,
                    );
                }
            } else {
                let mut dirlen: size_t = if (*context).token_len
                    >= ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                {
                    (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                } else {
                    (*context).token_len
                };
                memcpy(
                    directory.as_mut_ptr() as *mut libc::c_void,
                    tok as *const libc::c_void,
                    dirlen,
                );
                directory[dirlen as usize] = 0 as libc::c_int as libc::c_char;
            }
            loop {
                tok = getToken(context);
                if tok.is_null() {
                    break;
                }
                if (*context).token_len == 2 as libc::c_int as size_t
                    && strncmp(
                        tok,
                        b"/>\0" as *const u8 as *const libc::c_char,
                        2 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    if ((*context).callback).is_some() {
                        ((*context).callback)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*context).user_ctx,
                            flags | (1 as libc::c_int) << 2 as libc::c_int,
                            directory.as_mut_ptr(),
                            0 as *const libc::c_char,
                            0 as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            0 as libc::c_int as size_t,
                        );
                    }
                    break;
                } else if (*context).token_len == 1 as libc::c_int as size_t
                    && *tok as libc::c_int == '>' as i32
                {
                    if ((*context).callback).is_some() {
                        ((*context).callback)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*context).user_ctx,
                            flags | (1 as libc::c_int) << 1 as libc::c_int,
                            directory.as_mut_ptr(),
                            0 as *const libc::c_char,
                            0 as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            0 as libc::c_int as size_t,
                        );
                    }
                    if (*context).hints & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                        if wget_strcasecmp_ascii(
                            directory.as_mut_ptr(),
                            b"script\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            wget_debug_printf(
                                b"*** need special <script> handling\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            getScriptContent(context);
                            if (*context).token_len != 0 {
                                wget_debug_printf(
                                    b"%s=%.*s\n\0" as *const u8 as *const libc::c_char,
                                    directory.as_mut_ptr(),
                                    (*context).token_len as libc::c_int,
                                    (*context).token,
                                );
                            }
                        } else if wget_strcasecmp_ascii(
                            directory.as_mut_ptr(),
                            b"style\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            getStyleContent(context);
                            if (*context).token_len != 0 {
                                wget_debug_printf(
                                    b"%s=%.*s\n\0" as *const u8 as *const libc::c_char,
                                    directory.as_mut_ptr(),
                                    (*context).token_len as libc::c_int,
                                    (*context).token,
                                );
                            }
                        }
                    } else {
                        parseXML(directory.as_mut_ptr(), context);
                    }
                    break;
                } else {
                    let mut attribute: [libc::c_char; 256] = [0; 256];
                    let mut attrlen: size_t = if (*context).token_len
                        >= ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    {
                        (::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    } else {
                        (*context).token_len
                    };
                    memcpy(
                        attribute.as_mut_ptr() as *mut libc::c_void,
                        tok as *const libc::c_void,
                        attrlen,
                    );
                    attribute[attrlen as usize] = 0 as libc::c_int as libc::c_char;
                    if getValue(context) == -(1 as libc::c_int) {
                        return WGET_E_XML_PARSE_ERR as libc::c_int;
                    }
                    if (*context).token_len != 0 {
                        wget_debug_printf(
                            b"%s/@%s=%.*s\n\0" as *const u8 as *const libc::c_char,
                            directory.as_mut_ptr(),
                            attribute.as_mut_ptr(),
                            (*context).token_len as libc::c_int,
                            (*context).token,
                        );
                        if ((*context).callback).is_some() {
                            ((*context).callback)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*context).user_ctx,
                                flags | (1 as libc::c_int) << 3 as libc::c_int,
                                directory.as_mut_ptr(),
                                attribute.as_mut_ptr(),
                                (*context).token,
                                (*context).token_len,
                                ((*context).token).offset_from((*context).buf)
                                    as libc::c_long as size_t,
                            );
                        }
                    } else {
                        wget_debug_printf(
                            b"%s/@%s\n\0" as *const u8 as *const libc::c_char,
                            directory.as_mut_ptr(),
                            attribute.as_mut_ptr(),
                        );
                        if ((*context).callback).is_some() {
                            ((*context).callback)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*context).user_ctx,
                                flags | (1 as libc::c_int) << 3 as libc::c_int,
                                directory.as_mut_ptr(),
                                attribute.as_mut_ptr(),
                                0 as *const libc::c_char,
                                0 as libc::c_int as size_t,
                                0 as libc::c_int as size_t,
                            );
                        }
                    }
                    flags = 0 as libc::c_int;
                }
            }
            directory[pos as usize] = 0 as libc::c_int as libc::c_char;
        } else if (*context).token_len == 2 as libc::c_int as size_t {
            if strncmp(
                tok,
                b"</\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                tok = getToken(context);
                if tok.is_null() {
                    return WGET_E_XML_PARSE_ERR as libc::c_int;
                }
                if ((*context).callback).is_some() {
                    if (*context).hints & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                        ((*context).callback)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*context).user_ctx,
                            (1 as libc::c_int) << 2 as libc::c_int,
                            directory.as_mut_ptr(),
                            0 as *const libc::c_char,
                            0 as *const libc::c_char,
                            0 as libc::c_int as size_t,
                            0 as libc::c_int as size_t,
                        );
                    } else {
                        let mut tmp: [libc::c_char; 128] = [0; 128];
                        let mut tag: *mut libc::c_char = tmp.as_mut_ptr();
                        if (*context).token_len
                            >= ::core::mem::size_of::<[libc::c_char; 128]>()
                                as libc::c_ulong
                        {
                            tag = wget_malloc(
                                ((*context).token_len)
                                    .wrapping_add(1 as libc::c_int as size_t),
                            ) as *mut libc::c_char;
                        }
                        if !tag.is_null() {
                            memcpy(
                                tag as *mut libc::c_void,
                                tok as *const libc::c_void,
                                (*context).token_len,
                            );
                            *tag
                                .offset(
                                    (*context).token_len as isize,
                                ) = 0 as libc::c_int as libc::c_char;
                            ((*context).callback)
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*context).user_ctx,
                                (1 as libc::c_int) << 2 as libc::c_int,
                                tag,
                                0 as *const libc::c_char,
                                0 as *const libc::c_char,
                                0 as libc::c_int as size_t,
                                0 as libc::c_int as size_t,
                            );
                            if tag != tmp.as_mut_ptr() {
                                if !tag.is_null() {
                                    wget_free
                                        .expect(
                                            "non-null function pointer",
                                        )(tag as *mut libc::c_void);
                                    tag = 0 as *mut libc::c_char;
                                }
                            }
                        }
                    }
                }
                tok = getToken(context);
                if tok.is_null() {
                    return WGET_E_XML_PARSE_ERR as libc::c_int;
                }
                if (*context).hints & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                    return WGET_E_SUCCESS as libc::c_int;
                }
            } else if strncmp(
                tok,
                b"<?\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                getProcessing(context);
                wget_debug_printf(
                    b"%s=<?%.*s?>\n\0" as *const u8 as *const libc::c_char,
                    directory.as_mut_ptr(),
                    (*context).token_len as libc::c_int,
                    (*context).token,
                );
            } else if strncmp(
                tok,
                b"<!\0" as *const u8 as *const libc::c_char,
                2 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                getSpecial(context);
                wget_debug_printf(
                    b"%s=<!%.*s>\n\0" as *const u8 as *const libc::c_char,
                    directory.as_mut_ptr(),
                    (*context).token_len as libc::c_int,
                    (*context).token,
                );
            }
        } else if (*context).token_len == 4 as libc::c_int as size_t
            && strncmp(
                tok,
                b"<!--\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            getComment(context);
            wget_debug_printf(
                b"%s=<!--%.*s-->\n\0" as *const u8 as *const libc::c_char,
                directory.as_mut_ptr(),
                (*context).token_len as libc::c_int,
                (*context).token,
            );
        }
        if tok.is_null() {
            break;
        }
    }
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_xml_parse_buffer(
    mut buf: *const libc::c_char,
    mut callback: Option::<wget_xml_callback>,
    mut user_ctx: *mut libc::c_void,
    mut hints: libc::c_int,
) -> libc::c_int {
    let mut context: xml_context = xml_context {
        buf: 0 as *const libc::c_char,
        p: 0 as *const libc::c_char,
        token: 0 as *const libc::c_char,
        hints: 0,
        token_size: 0,
        token_len: 0,
        user_ctx: 0 as *mut libc::c_void,
        callback: None,
    };
    context.token = 0 as *const libc::c_char;
    context.token_size = 0 as libc::c_int as size_t;
    context.token_len = 0 as libc::c_int as size_t;
    context.buf = buf;
    context.p = buf;
    context.user_ctx = user_ctx;
    context.callback = callback;
    context.hints = hints;
    return parseXML(b"/\0" as *const u8 as *const libc::c_char, &mut context);
}
#[no_mangle]
pub unsafe extern "C" fn wget_html_parse_buffer(
    mut buf: *const libc::c_char,
    mut callback: Option::<wget_xml_callback>,
    mut user_ctx: *mut libc::c_void,
    mut hints: libc::c_int,
) {
    wget_xml_parse_buffer(
        buf,
        callback,
        user_ctx,
        hints | (1 as libc::c_int) << 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_xml_parse_file(
    mut fname: *const libc::c_char,
    mut callback: Option::<wget_xml_callback>,
    mut user_ctx: *mut libc::c_void,
    mut hints: libc::c_int,
) {
    if strcmp(fname, b"-\0" as *const u8 as *const libc::c_char) != 0 {
        let mut fd: libc::c_int = 0;
        fd = open(fname, 0 as libc::c_int | 0 as libc::c_int);
        if fd != -(1 as libc::c_int) {
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
            if fstat(fd, &mut st) == 0 as libc::c_int {
                let mut nread: size_t = st.st_size as size_t;
                let mut buf: *mut libc::c_char = mmap(
                    0 as *mut libc::c_void,
                    nread.wrapping_add(1 as libc::c_int as size_t),
                    0x1 as libc::c_int | 0x2 as libc::c_int,
                    0x2 as libc::c_int,
                    fd,
                    0 as libc::c_int as __off_t,
                ) as *mut libc::c_char;
                if nread > 0 as libc::c_int as size_t {
                    *buf.offset(nread as isize) = 0 as libc::c_int as libc::c_char;
                    wget_xml_parse_buffer(buf, callback, user_ctx, hints);
                }
                munmap(buf as *mut libc::c_void, nread);
            }
            close(fd);
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
        }
    } else {
        let mut tmp: [libc::c_char; 4096] = [0; 4096];
        let mut nbytes: ssize_t = 0;
        let mut buf_0: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        wget_buffer_init(
            &mut buf_0,
            0 as *mut libc::c_char,
            4096 as libc::c_int as size_t,
        );
        loop {
            nbytes = read(
                0 as libc::c_int,
                tmp.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            );
            if !(nbytes > 0 as libc::c_int as ssize_t) {
                break;
            }
            wget_buffer_memcat(
                &mut buf_0,
                tmp.as_mut_ptr() as *const libc::c_void,
                nbytes as size_t,
            );
        }
        if buf_0.length != 0 {
            wget_xml_parse_buffer(buf_0.data, callback, user_ctx, hints);
        }
        wget_buffer_deinit(&mut buf_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_html_parse_file(
    mut fname: *const libc::c_char,
    mut callback: Option::<wget_xml_callback>,
    mut user_ctx: *mut libc::c_void,
    mut hints: libc::c_int,
) {
    wget_xml_parse_file(
        fname,
        callback,
        user_ctx,
        hints | (1 as libc::c_int) << 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_xml_decode_entities_inline(
    mut src: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_uchar = src as *mut libc::c_uchar;
    let mut d: *mut libc::c_uchar = s;
    while *s != 0 {
        if *s as libc::c_int == '&' as i32 {
            if *s.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32 {
                if *s.offset(2 as libc::c_int as isize) as libc::c_int == 'x' as i32 {
                    *d = strtol(
                        (s as *mut libc::c_char).offset(3 as libc::c_int as isize),
                        &mut s as *mut *mut libc::c_uchar as *mut *mut libc::c_char,
                        16 as libc::c_int,
                    ) as libc::c_uchar;
                } else {
                    *d = strtol(
                        (s as *mut libc::c_char).offset(2 as libc::c_int as isize),
                        &mut s as *mut *mut libc::c_uchar as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as libc::c_uchar;
                }
                if *d as libc::c_int == ' ' as i32 {
                    *d = '+' as i32 as libc::c_uchar;
                }
                d = d.offset(1);
                d;
                if *s as libc::c_int == ';' as i32 {
                    s = s.offset(1);
                    s;
                }
                ret = src;
                continue;
            } else if strncmp(
                (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
                b"amp;\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                let fresh2 = d;
                d = d.offset(1);
                *fresh2 = '&' as i32 as libc::c_uchar;
                s = s.offset(5 as libc::c_int as isize);
                ret = src;
                continue;
            } else if strncmp(
                (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
                b"gt;\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                let fresh3 = d;
                d = d.offset(1);
                *fresh3 = '>' as i32 as libc::c_uchar;
                s = s.offset(4 as libc::c_int as isize);
                ret = src;
                continue;
            } else if strncmp(
                (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
                b"lt;\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                let fresh4 = d;
                d = d.offset(1);
                *fresh4 = '<' as i32 as libc::c_uchar;
                s = s.offset(4 as libc::c_int as isize);
                ret = src;
                continue;
            } else if strncmp(
                (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
                b"quot;\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                let fresh5 = d;
                d = d.offset(1);
                *fresh5 = '"' as i32 as libc::c_uchar;
                s = s.offset(6 as libc::c_int as isize);
                ret = src;
                continue;
            } else if strncmp(
                (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
                b"apos;\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                let fresh6 = d;
                d = d.offset(1);
                *fresh6 = '\'' as i32 as libc::c_uchar;
                s = s.offset(6 as libc::c_int as isize);
                ret = src;
                continue;
            }
        }
        let fresh7 = s;
        s = s.offset(1);
        let fresh8 = d;
        d = d.offset(1);
        *fresh8 = *fresh7;
    }
    *d = 0 as libc::c_int as libc::c_uchar;
    return ret;
}
