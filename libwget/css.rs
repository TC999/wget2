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
    pub type yy_buffer_state;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
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
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn yyget_leng(yyscanner: yyscan_t) -> libc::c_int;
    fn yyget_text(yyscanner: yyscan_t) -> *mut libc::c_char;
    fn yylex_init(scanner: *mut yyscan_t) -> libc::c_int;
    fn yy_scan_bytes(
        yystr: *const libc::c_char,
        len: libc::c_int,
        yyscanner: yyscan_t,
    ) -> YY_BUFFER_STATE;
    fn yylex(yyscanner: yyscan_t) -> libc::c_int;
    fn yylex_destroy(yyscanner: yyscan_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_realloc_function = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
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
pub type wget_css_parse_uri_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
    size_t,
) -> ();
pub type wget_css_parse_encoding_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> ();
pub type yyscan_t = *mut libc::c_void;
pub const STRING: C2RustUnnamed = 6;
pub const S: C2RustUnnamed = 1;
pub const CHARSET_SYM: C2RustUnnamed = 13;
pub const URI: C2RustUnnamed = 24;
pub const IMPORT_SYM: C2RustUnnamed = 10;
pub const CSSEOF: C2RustUnnamed = 0;
pub type YY_BUFFER_STATE = *mut yy_buffer_state;
pub type C2RustUnnamed = libc::c_uint;
pub const COMMENT: C2RustUnnamed = 27;
pub const FUNCTION: C2RustUnnamed = 26;
pub const BAD_URI: C2RustUnnamed = 25;
pub const NUMBER: C2RustUnnamed = 23;
pub const PERCENTAGE: C2RustUnnamed = 22;
pub const DIMENSION: C2RustUnnamed = 21;
pub const FREQ: C2RustUnnamed = 20;
pub const TIME: C2RustUnnamed = 19;
pub const ANGLE: C2RustUnnamed = 18;
pub const LENGTH: C2RustUnnamed = 17;
pub const EXS: C2RustUnnamed = 16;
pub const EMS: C2RustUnnamed = 15;
pub const IMPORTANT_SYM: C2RustUnnamed = 14;
pub const MEDIA_SYM: C2RustUnnamed = 12;
pub const PAGE_SYM: C2RustUnnamed = 11;
pub const HASH: C2RustUnnamed = 9;
pub const IDENT: C2RustUnnamed = 8;
pub const BAD_STRING: C2RustUnnamed = 7;
pub const DASHMATCH: C2RustUnnamed = 5;
pub const INCLUDES: C2RustUnnamed = 4;
pub const CDC: C2RustUnnamed = 3;
pub const CDO: C2RustUnnamed = 2;
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[inline]
unsafe extern "C" fn wget_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_realloc_fn.expect("non-null function pointer")(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn yyalloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc(size);
}
#[no_mangle]
pub unsafe extern "C" fn yyrealloc(
    mut p: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_realloc(p, size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_css_parse_buffer(
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut callback_uri: Option::<wget_css_parse_uri_callback>,
    mut callback_encoding: Option::<wget_css_parse_encoding_callback>,
    mut user_ctx: *mut libc::c_void,
) {
    let mut token: libc::c_int = 0;
    let mut length: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut scanner: yyscan_t = 0 as *mut libc::c_void;
    yylex_init(&mut scanner);
    yy_scan_bytes(buf, len as libc::c_int, scanner);
    loop {
        token = yylex(scanner);
        if !(token != CSSEOF as libc::c_int) {
            break;
        }
        if token == IMPORT_SYM as libc::c_int {
            pos = pos.wrapping_add(yyget_leng(scanner) as size_t);
            loop {
                token = yylex(scanner);
                if !(token == S as libc::c_int) {
                    break;
                }
                pos = pos.wrapping_add(yyget_leng(scanner) as size_t);
            }
            if token == STRING as libc::c_int {
                token = URI as libc::c_int;
            }
        }
        if token == URI as libc::c_int && callback_uri.is_some() {
            text = yyget_text(scanner);
            length = yyget_leng(scanner) as size_t;
            if *text as libc::c_int == '\'' as i32 || *text as libc::c_int == '"' as i32
            {
                callback_uri
                    .expect(
                        "non-null function pointer",
                    )(
                    user_ctx,
                    text.offset(1 as libc::c_int as isize),
                    length.wrapping_sub(2 as libc::c_int as size_t),
                    pos.wrapping_add(1 as libc::c_int as size_t),
                );
            } else if wget_strncasecmp_ascii(
                text,
                b"url(\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            ) == 0
            {
                let mut otext: *mut libc::c_char = text;
                length = length.wrapping_sub(1);
                length;
                while c_isspace(
                    *text
                        .offset(length.wrapping_sub(1 as libc::c_int as size_t) as isize)
                        as libc::c_int,
                ) {
                    length = length.wrapping_sub(1);
                    length;
                }
                length = length.wrapping_sub(4 as libc::c_int as size_t);
                text = text.offset(4 as libc::c_int as isize);
                while length != 0 && c_isspace(*text as libc::c_int) as libc::c_int != 0
                {
                    text = text.offset(1);
                    text;
                    length = length.wrapping_sub(1);
                    length;
                }
                if length != 0
                    && (*text as libc::c_int == '\'' as i32
                        || *text as libc::c_int == '"' as i32)
                {
                    text = text.offset(1);
                    text;
                    length = length.wrapping_sub(1);
                    length;
                }
                if length != 0
                    && (*text
                        .offset(length.wrapping_sub(1 as libc::c_int as size_t) as isize)
                        as libc::c_int == '\'' as i32
                        || *text
                            .offset(
                                length.wrapping_sub(1 as libc::c_int as size_t) as isize,
                            ) as libc::c_int == '"' as i32)
                {
                    length = length.wrapping_sub(1);
                    length;
                }
                callback_uri
                    .expect(
                        "non-null function pointer",
                    )(
                    user_ctx,
                    text,
                    length,
                    pos.wrapping_add(text.offset_from(otext) as libc::c_long as size_t),
                );
            }
        } else if token == CHARSET_SYM as libc::c_int && callback_encoding.is_some() {
            pos = pos.wrapping_add(yyget_leng(scanner) as size_t);
            loop {
                token = yylex(scanner);
                if !(token == S as libc::c_int) {
                    break;
                }
                pos = pos.wrapping_add(yyget_leng(scanner) as size_t);
            }
            if token == STRING as libc::c_int {
                text = yyget_text(scanner);
                length = yyget_leng(scanner) as size_t;
                if *text as libc::c_int == '\'' as i32
                    || *text as libc::c_int == '"' as i32
                {
                    callback_encoding
                        .expect(
                            "non-null function pointer",
                        )(
                        user_ctx,
                        text.offset(1 as libc::c_int as isize),
                        length.wrapping_sub(2 as libc::c_int as size_t),
                    );
                } else {
                    callback_encoding
                        .expect("non-null function pointer")(user_ctx, text, length);
                }
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown token after @charset: %d\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    token,
                );
            }
        }
        pos = pos.wrapping_add(yyget_leng(scanner) as size_t);
    }
    yylex_destroy(scanner);
}
#[no_mangle]
pub unsafe extern "C" fn wget_css_parse_file(
    mut fname: *const libc::c_char,
    mut callback_uri: Option::<wget_css_parse_uri_callback>,
    mut callback_encoding: Option::<wget_css_parse_encoding_callback>,
    mut user_ctx: *mut libc::c_void,
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
                    wget_css_parse_buffer(
                        buf,
                        st.st_size as size_t,
                        callback_uri,
                        callback_encoding,
                        user_ctx,
                    );
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
            wget_css_parse_buffer(
                buf_0.data,
                buf_0.length,
                callback_uri,
                callback_encoding,
                user_ctx,
            );
        }
        wget_buffer_deinit(&mut buf_0);
    };
}
