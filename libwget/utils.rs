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
    pub type stat;
    pub type dirent;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rpl_nanosleep(__rqtp: *const timespec, __rmtp: *mut timespec) -> libc::c_int;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn c_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn c_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn gettime(_: *mut timespec);
    fn rpl_ioctl(fd: libc::c_int, request: libc::c_int, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint64_t = __uint64_t;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[inline]
unsafe extern "C" fn c_isupper(mut c: libc::c_int) -> bool {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_strcmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    if s1.is_null() {
        if s2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if s2.is_null() {
        return 1 as libc::c_int
    } else {
        return strcmp(s1, s2)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_strcasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    if s1.is_null() {
        if s2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if s2.is_null() {
        return 1 as libc::c_int
    } else {
        return strcasecmp(s1, s2)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_strcasecmp_ascii(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    if s1.is_null() {
        if s2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if s2.is_null() {
        return 1 as libc::c_int
    } else {
        return c_strcasecmp(s1, s2)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_strncasecmp_ascii(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    if s1.is_null() {
        if s2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if s2.is_null() {
        return 1 as libc::c_int
    } else {
        return c_strncasecmp(s1, s2, n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_strtolower(mut s: *mut libc::c_char) -> *mut libc::c_char {
    if !s.is_null() {
        let mut d: *mut libc::c_char = s;
        while *d != 0 {
            if c_isupper(*d as libc::c_int) {
                *d = c_tolower(*d as libc::c_int) as libc::c_char;
            }
            d = d.offset(1);
            d;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_strncmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    if s1.is_null() {
        if s2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if s2.is_null() {
        return 1 as libc::c_int
    } else {
        return strncmp(s1, s2, n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_strncasecmp(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut n: size_t,
) -> libc::c_int {
    if s1.is_null() {
        if s2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if s2.is_null() {
        return 1 as libc::c_int
    } else {
        return strncasecmp(s1, s2, n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_memtohex(
    mut src: *const libc::c_uchar,
    mut src_len: size_t,
    mut dst: *mut libc::c_char,
    mut dst_size: size_t,
) {
    let mut it: size_t = 0;
    let mut adjust: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    if dst_size == 0 as libc::c_int as size_t || dst.is_null() || src.is_null() {
        return;
    }
    if src_len * 2 as libc::c_int as size_t >= dst_size {
        src_len = dst_size.wrapping_sub(1 as libc::c_int as size_t)
            / 2 as libc::c_int as size_t;
        adjust = 1 as libc::c_int;
    }
    it = 0 as libc::c_int as size_t;
    while it < src_len {
        c = *src as libc::c_int >> 4 as libc::c_int;
        let fresh0 = dst;
        dst = dst.offset(1);
        *fresh0 = (if c >= 10 as libc::c_int {
            c + 'a' as i32 - 10 as libc::c_int
        } else {
            c + '0' as i32
        }) as libc::c_char;
        c = *src as libc::c_int & 0xf as libc::c_int;
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = (if c >= 10 as libc::c_int {
            c + 'a' as i32 - 10 as libc::c_int
        } else {
            c + '0' as i32
        }) as libc::c_char;
        it = it.wrapping_add(1);
        it;
        src = src.offset(1);
        src;
    }
    if adjust != 0 && dst_size & 1 as libc::c_int as size_t == 0 as libc::c_int as size_t
    {
        c = *src as libc::c_int >> 4 as libc::c_int;
        let fresh2 = dst;
        dst = dst.offset(1);
        *fresh2 = (if c >= 10 as libc::c_int {
            c + 'a' as i32 - 10 as libc::c_int
        } else {
            c + '0' as i32
        }) as libc::c_char;
    }
    *dst = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wget_millisleep(mut ms: libc::c_int) {
    if ms <= 0 as libc::c_int {
        return;
    }
    rpl_nanosleep(
        &mut {
            let mut init = timespec {
                tv_sec: (ms / 1000 as libc::c_int) as __time_t,
                tv_nsec: (ms % 1000 as libc::c_int * 1000000 as libc::c_int)
                    as __syscall_slong_t,
            };
            init
        },
        0 as *mut timespec,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_get_timemillis() -> libc::c_longlong {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut ts);
    return ts.tv_sec as libc::c_longlong * 1000 as libc::c_longlong
        + (ts.tv_nsec / 1000000 as libc::c_int as __syscall_slong_t) as libc::c_longlong;
}
unsafe extern "C" fn unhex(mut c: libc::c_uchar) -> libc::c_uchar {
    return (if c as libc::c_int <= '9' as i32 {
        c as libc::c_int - '0' as i32
    } else if c as libc::c_int <= 'F' as i32 {
        c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    } else {
        c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn wget_percent_unescape(
    mut src: *mut libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut s: *mut libc::c_uchar = src as *mut libc::c_uchar;
    let mut d: *mut libc::c_uchar = s;
    while *s != 0 {
        if *s as libc::c_int == '%' as i32 {
            if c_isxdigit(*s.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
                && c_isxdigit(*s.offset(2 as libc::c_int as isize) as libc::c_int)
                    as libc::c_int != 0
            {
                let fresh3 = d;
                d = d.offset(1);
                *fresh3 = (((unhex(*s.offset(1 as libc::c_int as isize)) as libc::c_int)
                    << 4 as libc::c_int) as libc::c_uchar as libc::c_int
                    | unhex(*s.offset(2 as libc::c_int as isize)) as libc::c_int)
                    as libc::c_uchar;
                s = s.offset(3 as libc::c_int as isize);
                ret = 1 as libc::c_int;
                continue;
            }
        }
        let fresh4 = s;
        s = s.offset(1);
        let fresh5 = d;
        d = d.offset(1);
        *fresh5 = *fresh4;
    }
    *d = 0 as libc::c_int as libc::c_uchar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_match_tail(
    mut s: *const libc::c_char,
    mut tail: *const libc::c_char,
) -> libc::c_int {
    let mut s_len: size_t = 0;
    let mut tail_len: size_t = 0;
    s_len = strlen(s);
    tail_len = strlen(tail);
    if s_len < tail_len {
        return 0 as libc::c_int;
    }
    let mut p: *const libc::c_char = s.offset(s_len.wrapping_sub(tail_len) as isize);
    return (strcmp(p, tail) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_match_tail_nocase(
    mut s: *const libc::c_char,
    mut tail: *const libc::c_char,
) -> libc::c_int {
    let mut s_len: size_t = 0;
    let mut tail_len: size_t = 0;
    s_len = strlen(s);
    tail_len = strlen(tail);
    if s_len < tail_len {
        return 0 as libc::c_int;
    }
    let mut p: *const libc::c_char = s.offset(s_len.wrapping_sub(tail_len) as isize);
    return (wget_strcasecmp_ascii(p, tail) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_strnglob(
    mut str: *const libc::c_char,
    mut n: size_t,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    let mut pglob: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    let mut expanded_str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut globstr: *mut libc::c_char = wget_strmemdup(str as *const libc::c_void, n);
    if globstr.is_null() {
        return 0 as *mut libc::c_char;
    }
    if glob(globstr, flags, None, &mut pglob) == 0 as libc::c_int {
        if pglob.gl_pathc > 0 as libc::c_int as __size_t {
            expanded_str = wget_aprintf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                *(pglob.gl_pathv).offset(0 as libc::c_int as isize),
                str.offset(n as isize),
            );
        }
        globfree(&mut pglob);
    }
    if !globstr.is_null() {
        wget_free.expect("non-null function pointer")(globstr as *mut libc::c_void);
        globstr = 0 as *mut libc::c_char;
    }
    return expanded_str;
}
#[no_mangle]
pub unsafe extern "C" fn wget_human_readable(
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut n: uint64_t,
) -> *mut libc::c_char {
    static mut powers: [libc::c_char; 8] = [
        'K' as i32 as libc::c_char,
        'M' as i32 as libc::c_char,
        'G' as i32 as libc::c_char,
        'T' as i32 as libc::c_char,
        'P' as i32 as libc::c_char,
        'E' as i32 as libc::c_char,
        'Z' as i32 as libc::c_char,
        'Y' as i32 as libc::c_char,
    ];
    if n < 1024 as libc::c_int as uint64_t {
        wget_snprintf(
            buf,
            bufsize,
            b"%u \0" as *const u8 as *const libc::c_char,
            n as libc::c_uint,
        );
        return buf;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
    {
        if (n / 1024 as libc::c_int as uint64_t) < 1024 as libc::c_int as uint64_t
            || i as libc::c_ulong
                == (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let mut val: libc::c_double = n as libc::c_double / 1024.0f64;
            if val < 1000 as libc::c_int as libc::c_double {
                wget_snprintf(
                    buf,
                    bufsize,
                    b"%d.%02d%c\0" as *const u8 as *const libc::c_char,
                    val as libc::c_int,
                    (val * 100 as libc::c_int as libc::c_double) as libc::c_int
                        % 100 as libc::c_int,
                    powers[i as usize] as libc::c_int,
                );
            } else {
                wget_snprintf(
                    buf,
                    bufsize,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    (val + 0.5f64) as libc::c_int,
                    powers[i as usize] as libc::c_int,
                );
            }
            return buf;
        }
        n = n / 1024 as libc::c_int as uint64_t;
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wget_get_screen_size(
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) -> libc::c_int {
    let mut wsz: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut fd: libc::c_int = fileno(stderr);
    if rpl_ioctl(fd, 0x5413 as libc::c_int, &mut wsz as *mut winsize) >= 0 as libc::c_int
    {
        if !width.is_null() {
            *width = wsz.ws_col as libc::c_int;
        }
        if !height.is_null() {
            *height = wsz.ws_row as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
