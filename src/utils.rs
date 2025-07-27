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
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn wget_strnglob(
        str: *const libc::c_char,
        n: size_t,
        flags: libc::c_int,
    ) -> *mut libc::c_char;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemcpy_a(
        s: *mut libc::c_char,
        ssize: size_t,
        m: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
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
pub type __syscall_slong_t = libc::c_long;
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
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[no_mangle]
pub unsafe extern "C" fn mkdir_path(mut _fname: *const libc::c_char, mut is_file: bool) {
    let mut p1: *const libc::c_char = 0 as *const libc::c_char;
    let mut p2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    fname = wget_strmemcpy_a(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        _fname as *const libc::c_void,
        strlen(_fname),
    ) as *mut libc::c_char;
    p1 = fname.offset(1 as libc::c_int as isize);
    while *p1 as libc::c_int != 0
        && {
            p2 = strchr(p1, '/' as i32);
            !p2.is_null()
        }
    {
        let mut rc: libc::c_int = 0;
        *p2 = 0 as libc::c_int as libc::c_char;
        if *p1 as libc::c_int == '.' as i32
            && *p1.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        {
            wget_error_printf_exit(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Internal error: Unexpected relative path: '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
        }
        rc = mkdir(fname, 0o755 as libc::c_int as __mode_t);
        if rc < 0 as libc::c_int && *__errno_location() != 17 as libc::c_int {
            wget_debug_printf(
                b"mkdir(%s)=%d errno=%d\n\0" as *const u8 as *const libc::c_char,
                fname,
                rc,
                *__errno_location(),
            );
        }
        if rc != 0 {
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
            if *__errno_location() == 17 as libc::c_int
                && stat(fname, &mut st) == 0 as libc::c_int
                && st.st_mode & 0o170000 as libc::c_int as __mode_t
                    == 0o100000 as libc::c_int as __mode_t
            {
                let mut renamed: libc::c_int = 0 as libc::c_int;
                let mut fnum: libc::c_int = 1 as libc::c_int;
                while fnum <= 999 as libc::c_int && renamed == 0 {
                    let mut tmp: [libc::c_char; 1024] = [0; 1024];
                    let mut dst: *mut libc::c_char = tmp.as_mut_ptr();
                    if wget_snprintf(
                        tmp.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"%s.%d\0" as *const u8 as *const libc::c_char,
                        fname,
                        fnum,
                    ) >= ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    {
                        dst = wget_aprintf(
                            b"%s.%d\0" as *const u8 as *const libc::c_char,
                            fname,
                            fnum,
                        );
                    }
                    if access(dst, 0 as libc::c_int) != 0 as libc::c_int
                        && rename(fname, dst) == 0 as libc::c_int
                    {
                        renamed = 1 as libc::c_int;
                    }
                    if dst != tmp.as_mut_ptr() {
                        if !dst.is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )(dst as *mut libc::c_void);
                            dst = 0 as *mut libc::c_char;
                        }
                    }
                    fnum += 1;
                    fnum;
                }
                if renamed != 0 {
                    rc = mkdir(fname, 0o755 as libc::c_int as __mode_t);
                    if rc != 0 {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to make directory '%s' (errno=%d)\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            fname,
                            *__errno_location(),
                        );
                        *p2 = '/' as i32 as libc::c_char;
                        break;
                    }
                } else {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to rename '%s' (errno=%d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        fname,
                        *__errno_location(),
                    );
                }
            } else if *__errno_location() != 17 as libc::c_int {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to make directory '%s' (errno=%d)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname,
                    *__errno_location(),
                );
                *p2 = '/' as i32 as libc::c_char;
                break;
            }
        } else {
            wget_debug_printf(
                b"created dir %s\n\0" as *const u8 as *const libc::c_char,
                fname,
            );
        }
        *p2 = '/' as i32 as libc::c_char;
        p1 = p2.offset(1 as libc::c_int as isize);
    }
    if fname != buf.as_mut_ptr() {
        if !fname.is_null() {
            wget_free.expect("non-null function pointer")(fname as *mut libc::c_void);
            fname = 0 as *mut libc::c_char;
        }
    }
    if !is_file {
        let mut rc_0: libc::c_int = mkdir(_fname, 0o755 as libc::c_int as __mode_t);
        if rc_0 < 0 as libc::c_int && *__errno_location() != 17 as libc::c_int {
            wget_debug_printf(
                b"mkdir(%s)=%d errno=%d\n\0" as *const u8 as *const libc::c_char,
                _fname,
                rc_0,
                *__errno_location(),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn shell_expand(
    mut fname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut expanded_str: *mut libc::c_char = 0 as *mut libc::c_char;
    if *fname as libc::c_int == '~' as i32 {
        let mut slash: *mut libc::c_char = strchrnul(fname, '/' as i32);
        expanded_str = wget_strnglob(
            fname,
            slash.offset_from(fname) as libc::c_long as size_t,
            (1 as libc::c_int) << 12 as libc::c_int
                | (1 as libc::c_int) << 13 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
        );
    }
    return if !expanded_str.is_null() { expanded_str } else { wget_strdup(fname) };
}
#[no_mangle]
pub unsafe extern "C" fn wget_restrict_file_name(
    mut fname: *const libc::c_char,
    mut esc: *mut libc::c_char,
    mut mode: libc::c_int,
) -> *mut libc::c_char {
    if fname.is_null() || esc.is_null() || mode == 0 as libc::c_int {
        return fname as *mut libc::c_char;
    }
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut escaped: bool = 0 as libc::c_int != 0;
    let mut lowercase: bool = mode & (1 as libc::c_int) << 5 as libc::c_int != 0;
    let mut uppercase: bool = mode & (1 as libc::c_int) << 4 as libc::c_int != 0;
    let mut mode_unix: bool = mode & (1 as libc::c_int) << 0 as libc::c_int != 0;
    let mut windows: bool = mode & (1 as libc::c_int) << 1 as libc::c_int != 0;
    let mut nocontrol: bool = mode & (1 as libc::c_int) << 2 as libc::c_int != 0;
    let mut ascii: bool = mode & (1 as libc::c_int) << 3 as libc::c_int != 0;
    dst = esc;
    s = fname;
    while *s != 0 {
        let mut c: libc::c_schar = *s as libc::c_schar;
        if lowercase as libc::c_int != 0 && c as libc::c_int >= 'A' as i32
            && c as libc::c_int <= 'Z' as i32
        {
            c = (c as libc::c_int | 0x20 as libc::c_int) as libc::c_schar;
            escaped = 1 as libc::c_int != 0;
        } else if uppercase as libc::c_int != 0 && c as libc::c_int >= 'a' as i32
            && c as libc::c_int <= 'z' as i32
        {
            c = (c as libc::c_int & !(0x20 as libc::c_int)) as libc::c_schar;
            escaped = 1 as libc::c_int != 0;
        }
        if mode_unix as libc::c_int != 0
            && (c as libc::c_int >= 1 as libc::c_int
                && c as libc::c_int <= 31 as libc::c_int)
            || windows as libc::c_int != 0
                && ((c as libc::c_int) < 32 as libc::c_int
                    || !(strchr(
                        b"\\<>:\"|?*\0" as *const u8 as *const libc::c_char,
                        c as libc::c_int,
                    ))
                        .is_null())
            || !nocontrol
                && (c as libc::c_int >= 1 as libc::c_int
                    && c as libc::c_int <= 31 as libc::c_int
                    || c as libc::c_int == 127 as libc::c_int)
            || ascii as libc::c_int != 0 && c as libc::c_int <= 31 as libc::c_int
        {
            let mut nibble: libc::c_char = 0;
            let fresh0 = dst;
            dst = dst.offset(1);
            *fresh0 = '%' as i32 as libc::c_char;
            nibble = (c as libc::c_uchar as libc::c_int >> 4 as libc::c_int)
                as libc::c_char;
            let fresh1 = dst;
            dst = dst.offset(1);
            *fresh1 = (if nibble as libc::c_int >= 10 as libc::c_int {
                nibble as libc::c_int + 'A' as i32 - 10 as libc::c_int
            } else {
                nibble as libc::c_int + '0' as i32
            }) as libc::c_char;
            nibble = (c as libc::c_int & 0xf as libc::c_int) as libc::c_char;
            let fresh2 = dst;
            dst = dst.offset(1);
            *fresh2 = (if nibble as libc::c_int >= 10 as libc::c_int {
                nibble as libc::c_int + 'A' as i32 - 10 as libc::c_int
            } else {
                nibble as libc::c_int + '0' as i32
            }) as libc::c_char;
            escaped = 1 as libc::c_int != 0;
        } else {
            let fresh3 = dst;
            dst = dst.offset(1);
            *fresh3 = c as libc::c_char;
        }
        s = s.offset(1);
        s;
    }
    if escaped {
        *dst = 0 as libc::c_int as libc::c_char;
        return esc;
    }
    return fname as *mut libc::c_char;
}
