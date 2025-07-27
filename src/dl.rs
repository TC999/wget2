#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type __dirstream;
    pub type wget_vector_st;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_vaprintf(
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type va_list = __builtin_va_list;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_error_t {
    pub msg: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_file_st {
    pub handle: *mut libc::c_void,
}
pub type dl_file_t = dl_file_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct object_pattern_t {
    pub prefix: *const libc::c_char,
    pub suffix: *const libc::c_char,
}
unsafe extern "C" fn dl_error_set_noalloc(
    mut e: *mut dl_error_t,
    mut msg: *const libc::c_char,
) {
    if !msg.is_null() && !((*e).msg).is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Piling up error '%s' over error '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            msg,
            (*e).msg,
        );
    }
    wget_free.expect("non-null function pointer")((*e).msg as *mut libc::c_void);
    (*e).msg = msg;
}
#[no_mangle]
pub unsafe extern "C" fn dl_error_set(
    mut e: *mut dl_error_t,
    mut msg: *const libc::c_char,
) {
    dl_error_set_noalloc(e, wget_strdup(msg));
}
#[no_mangle]
pub unsafe extern "C" fn dl_error_set_printf(
    mut e: *mut dl_error_t,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut arglist: ::core::ffi::VaListImpl;
    arglist = args.clone();
    dl_error_set_noalloc(e, wget_vaprintf(format, arglist.as_va_list()));
}
unsafe extern "C" fn convert_to_path_if_not(
    mut str: *const libc::c_char,
) -> *mut libc::c_char {
    if !str.is_null() && (strchr(str, '/' as i32)).is_null() {
        return wget_aprintf(b"./%s\0" as *const u8 as *const libc::c_char, str);
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn dl_supported() -> libc::c_int {
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dl_file_open(
    mut filename: *const libc::c_char,
    mut e: *mut dl_error_t,
) -> *mut dl_file_t {
    let mut dm: *mut dl_file_t = 0 as *mut dl_file_t;
    let mut dm_st: dl_file_t = dl_file_st {
        handle: 0 as *mut libc::c_void,
    };
    let mut buf: *mut libc::c_char = convert_to_path_if_not(filename);
    dm_st
        .handle = dlopen(
        if !buf.is_null() { buf as *const libc::c_char } else { filename },
        0x1 as libc::c_int | 0 as libc::c_int,
    );
    if !buf.is_null() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    if !(dm_st.handle).is_null() {
        dm = wget_memdup(
            &mut dm_st as *mut dl_file_t as *const libc::c_void,
            ::core::mem::size_of::<dl_file_t>() as libc::c_ulong,
        ) as *mut dl_file_t;
    } else {
        dl_error_set(e, dlerror());
    }
    return dm;
}
#[no_mangle]
pub unsafe extern "C" fn dl_file_lookup(
    mut dm: *mut dl_file_t,
    mut symbol: *const libc::c_char,
    mut e: *mut dl_error_t,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: *mut libc::c_char = 0 as *mut libc::c_char;
    res = dlsym((*dm).handle, symbol);
    error = dlerror();
    if !error.is_null() {
        dl_error_set(e, error);
        return 0 as *mut libc::c_void;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn dl_file_close(mut dm: *mut dl_file_t) {
    dlclose((*dm).handle);
    wget_free.expect("non-null function pointer")(dm as *mut libc::c_void);
}
static mut dl_patterns: [object_pattern_t; 2] = [
    {
        let mut init = object_pattern_t {
            prefix: b"lib\0" as *const u8 as *const libc::c_char,
            suffix: b".so\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = object_pattern_t {
            prefix: 0 as *const libc::c_char,
            suffix: 0 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn dl_match(
    mut path: *const libc::c_char,
    mut start_out: *mut size_t,
    mut len_out: *mut size_t,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut mark: size_t = 0;
    let mut start: size_t = 0;
    let mut len: size_t = 0;
    mark = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while *path.offset(i as isize) != 0 {
        if *path.offset(i as isize) as libc::c_int == '/' as i32 {
            mark = i.wrapping_add(1 as libc::c_int as size_t);
        }
        i = i.wrapping_add(1);
        i;
    }
    start = mark;
    len = i.wrapping_sub(start);
    i = 0 as libc::c_int as size_t;
    while !(dl_patterns[i as usize].prefix).is_null() {
        let mut p: *const libc::c_char = dl_patterns[i as usize].prefix;
        let mut s: *const libc::c_char = dl_patterns[i as usize].suffix;
        let mut pl: size_t = strlen(p);
        let mut sl: size_t = strlen(s);
        if !(pl.wrapping_add(sl) >= len) {
            if memcmp(
                path.offset(start as isize).offset(len as isize).offset(-(sl as isize))
                    as *const libc::c_void,
                s as *const libc::c_void,
                sl,
            ) == 0 as libc::c_int
                && memcmp(
                    path.offset(start as isize) as *const libc::c_void,
                    p as *const libc::c_void,
                    pl,
                ) == 0 as libc::c_int
            {
                start = start.wrapping_add(pl);
                len = len.wrapping_sub(pl.wrapping_add(sl));
                break;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    *start_out = start;
    *len_out = len;
    return if !(dl_patterns[i as usize].prefix).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn is_regular_file(mut filename: *const libc::c_char) -> libc::c_int {
    let mut statbuf: stat = stat {
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
    if stat(filename, &mut statbuf) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if statbuf.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o100000 as libc::c_int as __mode_t
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dl_get_name_from_path(
    mut path: *const libc::c_char,
    mut strict: libc::c_int,
) -> *mut libc::c_char {
    let mut start: size_t = 0;
    let mut len: size_t = 0;
    let mut match_0: libc::c_int = dl_match(path, &mut start, &mut len);
    if match_0 == 0 && strict != 0 {
        return 0 as *mut libc::c_char
    } else {
        return wget_strmemdup(path.offset(start as isize) as *const libc::c_void, len)
    };
}
#[no_mangle]
pub unsafe extern "C" fn dl_search(
    mut name: *const libc::c_char,
    mut dirs: *const wget_vector,
) -> *mut libc::c_char {
    let mut n_dirs: libc::c_int = wget_vector_size(dirs);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_dirs {
        let mut dir: *const libc::c_char = wget_vector_get(dirs, i)
            as *const libc::c_char;
        if !dir.is_null() && *dir as libc::c_int != 0 {
            let mut j: libc::c_int = 0 as libc::c_int;
            while !(dl_patterns[j as usize].prefix).is_null() {
                let mut filename: *mut libc::c_char = wget_aprintf(
                    b"%s/%s%s%s\0" as *const u8 as *const libc::c_char,
                    dir,
                    dl_patterns[j as usize].prefix,
                    name,
                    dl_patterns[j as usize].suffix,
                );
                if is_regular_file(filename) != 0 {
                    return filename;
                }
                wget_free
                    .expect("non-null function pointer")(filename as *mut libc::c_void);
                j += 1;
                j;
            }
        } else {
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while !(dl_patterns[j_0 as usize].prefix).is_null() {
                let mut filename_0: *mut libc::c_char = wget_aprintf(
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    dl_patterns[j_0 as usize].prefix,
                    name,
                    dl_patterns[j_0 as usize].suffix,
                );
                if is_regular_file(filename_0) != 0 {
                    return filename_0;
                }
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(filename_0 as *mut libc::c_void);
                j_0 += 1;
                j_0;
            }
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn dl_list(
    mut dirs: *const wget_vector,
    mut names_out: *mut wget_vector,
) {
    let mut n_dirs: libc::c_int = wget_vector_size(dirs);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_dirs {
        let mut dirp: *mut DIR = 0 as *mut DIR;
        let mut ent: *mut dirent = 0 as *mut dirent;
        let mut dir: *const libc::c_char = wget_vector_get(dirs, i)
            as *const libc::c_char;
        dirp = opendir(dir);
        if !dirp.is_null() {
            loop {
                ent = readdir(dirp);
                if ent.is_null() {
                    break;
                }
                let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                fname = ((*ent).d_name).as_mut_ptr();
                name = dl_get_name_from_path(fname, 1 as libc::c_int);
                if name.is_null() {
                    continue;
                }
                let mut sfname: *mut libc::c_char = wget_aprintf(
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    dir,
                    fname,
                );
                let mut x: libc::c_int = is_regular_file(sfname);
                wget_free
                    .expect("non-null function pointer")(sfname as *mut libc::c_void);
                if x == 0 {
                    wget_free
                        .expect("non-null function pointer")(name as *mut libc::c_void);
                } else {
                    wget_vector_add(names_out, name as *const libc::c_void);
                }
            }
            closedir(dirp);
        }
        i += 1;
        i;
    }
}
