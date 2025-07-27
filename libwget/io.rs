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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getuid() -> __uid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
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
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
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
pub type wget_update_load_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut FILE,
) -> libc::c_int;
pub type wget_update_save_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut FILE,
) -> libc::c_int;
pub type wget_realloc_function = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
#[inline]
unsafe extern "C" fn wget_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_realloc_fn.expect("non-null function pointer")(ptr, size);
}
unsafe extern "C" fn read_fp(
    mut f: *const libc::c_void,
    mut dst: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut fp: *mut FILE = f as *mut FILE;
    let mut ret: ssize_t = fread(
        dst as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len,
        fp,
    ) as ssize_t;
    if ferror(fp) != 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    return ret;
}
unsafe extern "C" fn read_fd(
    mut f: *const libc::c_void,
    mut dst: *mut libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut fd: *mut libc::c_int = f as *mut libc::c_int;
    return read(*fd, dst as *mut libc::c_void, len);
}
unsafe extern "C" fn getline_internal(
    mut buf: *mut *mut libc::c_char,
    mut bufsize: *mut size_t,
    mut f: *const libc::c_void,
    mut reader: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_char, size_t) -> ssize_t,
    >,
) -> ssize_t {
    let mut nbytes: ssize_t = 0 as libc::c_int as ssize_t;
    let mut sizep: *mut size_t = 0 as *mut size_t;
    let mut length: size_t = 0 as libc::c_int as size_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if buf.is_null() || bufsize.is_null() {
        return WGET_E_INVALID as libc::c_int as ssize_t;
    }
    if (*buf).is_null() || *bufsize == 0 {
        p = wget_calloc(10240 as libc::c_int as size_t, 1 as libc::c_int as size_t)
            as *mut libc::c_char;
        if p.is_null() {
            return WGET_E_MEMORY as libc::c_int as ssize_t;
        }
        *buf = p;
        *bufsize = 10240 as libc::c_int as size_t;
        sizep = (*buf)
            .offset(*bufsize as isize)
            .offset(
                -((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    as isize),
            ) as *mut libc::c_void as *mut size_t;
        let ref mut fresh0 = *sizep.offset(1 as libc::c_int as isize);
        *fresh0 = 0 as libc::c_int as size_t;
        *sizep.offset(0 as libc::c_int as isize) = *fresh0;
    } else {
        sizep = (*buf)
            .offset(*bufsize as isize)
            .offset(
                -((2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    as isize),
            ) as *mut libc::c_void as *mut size_t;
        if *sizep.offset(1 as libc::c_int as isize) != 0 {
            p = memchr(
                (*buf).offset(*sizep.offset(0 as libc::c_int as isize) as isize)
                    as *const libc::c_void,
                '\n' as i32,
                *sizep.offset(1 as libc::c_int as isize),
            ) as *mut libc::c_char;
            if !p.is_null() {
                let fresh1 = p;
                p = p.offset(1);
                *fresh1 = 0 as libc::c_int as libc::c_char;
                length = p
                    .offset_from(
                        (*buf).offset(*sizep.offset(0 as libc::c_int as isize) as isize),
                    ) as libc::c_long as size_t;
                if *sizep.offset(0 as libc::c_int as isize) != 0 {
                    memmove(
                        *buf as *mut libc::c_void,
                        (*buf).offset(*sizep.offset(0 as libc::c_int as isize) as isize)
                            as *const libc::c_void,
                        length,
                    );
                }
                let ref mut fresh2 = *sizep.offset(0 as libc::c_int as isize);
                *fresh2 = (*fresh2).wrapping_add(length);
                let ref mut fresh3 = *sizep.offset(1 as libc::c_int as isize);
                *fresh3 = (*fresh3).wrapping_sub(length);
                return length.wrapping_sub(1 as libc::c_int as size_t) as ssize_t;
            }
            length = *sizep.offset(1 as libc::c_int as isize);
            memmove(
                *buf as *mut libc::c_void,
                (*buf).offset(*sizep.offset(0 as libc::c_int as isize) as isize)
                    as *const libc::c_void,
                length.wrapping_add(1 as libc::c_int as size_t),
            );
            let ref mut fresh4 = *sizep.offset(1 as libc::c_int as isize);
            *fresh4 = 0 as libc::c_int as size_t;
            *sizep.offset(0 as libc::c_int as isize) = *fresh4;
        } else {
            **buf = 0 as libc::c_int as libc::c_char;
        }
    }
    loop {
        nbytes = reader
            .expect(
                "non-null function pointer",
            )(
            f,
            (*buf).offset(length as isize),
            (*bufsize)
                .wrapping_sub(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
                )
                .wrapping_sub(length)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        if !(nbytes > 0 as libc::c_int as ssize_t) {
            break;
        }
        length = length.wrapping_add(nbytes as size_t);
        p = memchr(
            (*buf).offset(length as isize).offset(-(nbytes as isize))
                as *const libc::c_void,
            '\n' as i32,
            nbytes as libc::c_ulong,
        ) as *mut libc::c_char;
        if !p.is_null() {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = 0 as libc::c_int as libc::c_char;
            *sizep
                .offset(
                    0 as libc::c_int as isize,
                ) = p.offset_from(*buf) as libc::c_long as size_t;
            *sizep
                .offset(
                    1 as libc::c_int as isize,
                ) = length.wrapping_sub(*sizep.offset(0 as libc::c_int as isize));
            return (*sizep.offset(0 as libc::c_int as isize))
                .wrapping_sub(1 as libc::c_int as size_t) as ssize_t;
        }
        if length
            >= (*bufsize)
                .wrapping_sub(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
                )
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let mut off: ptrdiff_t = (sizep as *mut libc::c_char).offset_from(*buf)
                as libc::c_long;
            let mut old: *mut size_t = 0 as *mut size_t;
            *bufsize = *bufsize * 2 as libc::c_int as size_t;
            p = wget_realloc(*buf as *mut libc::c_void, *bufsize) as *mut libc::c_char;
            if p.is_null() {
                return WGET_E_MEMORY as libc::c_int as ssize_t;
            }
            *buf = p;
            old = (*buf).offset(off as isize) as *mut libc::c_void as *mut size_t;
            sizep = (*buf)
                .offset(*bufsize as isize)
                .offset(
                    -((2 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong)
                        as isize),
                ) as *mut libc::c_void as *mut size_t;
            *sizep
                .offset(
                    0 as libc::c_int as isize,
                ) = *old.offset(0 as libc::c_int as isize);
            *sizep
                .offset(
                    1 as libc::c_int as isize,
                ) = *old.offset(1 as libc::c_int as isize);
        }
    }
    if nbytes == -(1 as libc::c_int) as ssize_t
        && *__errno_location() != 11 as libc::c_int
    {
        if *__errno_location() != 9 as libc::c_int {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to read, error %d\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"getline_internal\0"))
                    .as_ptr(),
                *__errno_location(),
            );
        }
    }
    if length != 0 {
        if *(*buf).offset(length.wrapping_sub(1 as libc::c_int as size_t) as isize)
            as libc::c_int == '\n' as i32
        {
            *(*buf)
                .offset(
                    length.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        } else {
            *(*buf).offset(length as isize) = 0 as libc::c_int as libc::c_char;
        }
        return length as ssize_t;
    } else {
        **buf = 0 as libc::c_int as libc::c_char;
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_fdgetline(
    mut buf: *mut *mut libc::c_char,
    mut bufsize: *mut size_t,
    mut fd: libc::c_int,
) -> ssize_t {
    return getline_internal(
        buf,
        bufsize,
        &mut fd as *mut libc::c_int as *mut libc::c_void,
        Some(
            read_fd
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_char,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_getline(
    mut buf: *mut *mut libc::c_char,
    mut bufsize: *mut size_t,
    mut fp: *mut FILE,
) -> ssize_t {
    return getline_internal(
        buf,
        bufsize,
        fp as *mut libc::c_void,
        Some(
            read_fp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_char,
                    size_t,
                ) -> ssize_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_ready_2_transfer(
    mut fd: libc::c_int,
    mut timeout: libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = -(1 as libc::c_int);
    let mut pollfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    pollfd.fd = fd;
    pollfd.events = 0 as libc::c_int as libc::c_short;
    pollfd.revents = 0 as libc::c_int as libc::c_short;
    if mode & 1 as libc::c_int != 0 {
        pollfd
            .events = (pollfd.events as libc::c_int | 0x1 as libc::c_int)
            as libc::c_short;
    }
    if mode & 2 as libc::c_int != 0 {
        pollfd
            .events = (pollfd.events as libc::c_int | 0x4 as libc::c_int)
            as libc::c_short;
    }
    rc = poll(&mut pollfd, 1 as libc::c_int as nfds_t, timeout);
    if rc > 0 as libc::c_int {
        rc = 0 as libc::c_int;
        if pollfd.revents as libc::c_int & 0x1 as libc::c_int != 0 {
            rc |= 1 as libc::c_int;
        }
        if pollfd.revents as libc::c_int & 0x4 as libc::c_int != 0 {
            rc |= 2 as libc::c_int;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ready_2_read(
    mut fd: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return (wget_ready_2_transfer(fd, timeout, 1 as libc::c_int) > 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ready_2_write(
    mut fd: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    return (wget_ready_2_transfer(fd, timeout, 2 as libc::c_int) > 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_read_file(
    mut fname: *const libc::c_char,
    mut size: *mut size_t,
) -> *mut libc::c_char {
    let mut nread: ssize_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    if fname.is_null() {
        return 0 as *mut libc::c_char;
    }
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
                let mut total: off_t = 0 as libc::c_int as off_t;
                buf = wget_malloc((st.st_size + 1 as libc::c_int as __off_t) as size_t)
                    as *mut libc::c_char;
                if buf.is_null() {
                    close(fd);
                    return 0 as *mut libc::c_char;
                }
                while total < st.st_size
                    && {
                        nread = read(
                            fd,
                            buf.offset(total as isize) as *mut libc::c_void,
                            (st.st_size - total) as size_t,
                        );
                        nread > 0 as libc::c_int as ssize_t
                    }
                {
                    total += nread;
                }
                *buf.offset(total as isize) = 0 as libc::c_int as libc::c_char;
                if !size.is_null() {
                    *size = total as size_t;
                }
                if total != st.st_size {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"WARNING: Size of %s changed from %lld to %lld while reading. This may lead to unwanted results !\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        fname,
                        st.st_size as libc::c_longlong,
                        total as libc::c_longlong,
                    );
                }
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to fstat %s\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname,
                );
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
        let mut buffer: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        wget_buffer_init(
            &mut buffer,
            0 as *mut libc::c_char,
            4096 as libc::c_int as size_t,
        );
        loop {
            nread = read(
                0 as libc::c_int,
                tmp.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            );
            if !(nread > 0 as libc::c_int as ssize_t) {
                break;
            }
            wget_buffer_memcat(
                &mut buffer,
                tmp.as_mut_ptr() as *const libc::c_void,
                nread as size_t,
            );
        }
        if !size.is_null() {
            *size = buffer.length;
        }
        buf = buffer.data;
        buffer.data = 0 as *mut libc::c_char;
        wget_buffer_deinit(&mut buffer);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn wget_update_file(
    mut fname: *const libc::c_char,
    mut load_func: Option::<wget_update_load_fn>,
    mut save_func: Option::<wget_update_save_fn>,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmpdir: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmpfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lockfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lockfd: libc::c_int = -(1 as libc::c_int);
    let mut rc: libc::c_int = WGET_E_SUCCESS as libc::c_int;
    tmpfile = wget_aprintf(b"%sXXXXXX\0" as *const u8 as *const libc::c_char, fname);
    if tmpfile.is_null() {
        rc = WGET_E_MEMORY as libc::c_int;
    } else {
        basename = base_name(fname);
        if basename.is_null() {
            rc = WGET_E_MEMORY as libc::c_int;
        } else {
            tmpdir = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
            if tmpdir.is_null()
                && {
                    tmpdir = getenv(b"TMP\0" as *const u8 as *const libc::c_char);
                    tmpdir.is_null()
                }
                && {
                    tmpdir = getenv(b"TEMP\0" as *const u8 as *const libc::c_char);
                    tmpdir.is_null()
                }
                && {
                    tmpdir = getenv(b"TEMPDIR\0" as *const u8 as *const libc::c_char);
                    tmpdir.is_null()
                }
            {
                tmpdir = b"/tmp\0" as *const u8 as *const libc::c_char;
            }
            if *tmpdir != 0 {
                lockfile = wget_aprintf(
                    b"%s/%s_lck_%u\0" as *const u8 as *const libc::c_char,
                    tmpdir,
                    basename,
                    getuid(),
                );
            } else {
                lockfile = wget_aprintf(
                    b"%s_lck_%u\0" as *const u8 as *const libc::c_char,
                    basename,
                    getuid(),
                );
            }
            if lockfile.is_null() {
                rc = WGET_E_MEMORY as libc::c_int;
            } else {
                lockfd = open(
                    lockfile,
                    0o1 as libc::c_int | 0o100 as libc::c_int,
                    0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int
                        | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
                );
                if lockfd == -(1 as libc::c_int) {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to create '%s' (%d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        lockfile,
                        *__errno_location(),
                    );
                    rc = WGET_E_OPEN as libc::c_int;
                } else if flock(lockfd, 2 as libc::c_int) == -(1 as libc::c_int) {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to lock '%s' (%d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        lockfile,
                        *__errno_location(),
                    );
                    rc = WGET_E_IO as libc::c_int;
                } else {
                    if load_func.is_some() {
                        fp = rpl_fopen(
                            fname,
                            b"r\0" as *const u8 as *const libc::c_char,
                        );
                        if fp.is_null() {
                            if *__errno_location() != 2 as libc::c_int {
                                wget_error_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Failed to read open '%s' (%d)\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    fname,
                                    *__errno_location(),
                                );
                                rc = WGET_E_OPEN as libc::c_int;
                                current_block = 6299914982191342269;
                            } else {
                                current_block = 15089075282327824602;
                            }
                        } else {
                            current_block = 15089075282327824602;
                        }
                        match current_block {
                            6299914982191342269 => {}
                            _ => {
                                if !fp.is_null() {
                                    if load_func
                                        .expect("non-null function pointer")(context, fp) != 0
                                    {
                                        rc = WGET_E_UNKNOWN as libc::c_int;
                                        current_block = 6299914982191342269;
                                    } else {
                                        rpl_fclose(fp);
                                        fp = 0 as *mut FILE;
                                        current_block = 15125582407903384992;
                                    }
                                } else {
                                    current_block = 15125582407903384992;
                                }
                            }
                        }
                    } else {
                        current_block = 15125582407903384992;
                    }
                    match current_block {
                        6299914982191342269 => {}
                        _ => {
                            if save_func.is_some() {
                                let mut fd: libc::c_int = 0;
                                fd = mkstemp(tmpfile);
                                if fd == -(1 as libc::c_int) {
                                    wget_error_printf(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Failed to open tmpfile '%s' (%d)\n\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        tmpfile,
                                        *__errno_location(),
                                    );
                                    rc = WGET_E_OPEN as libc::c_int;
                                } else {
                                    fp = fdopen(fd, b"w\0" as *const u8 as *const libc::c_char);
                                    if fp.is_null() {
                                        unlink(tmpfile);
                                        close(fd);
                                        wget_error_printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Failed to write open '%s' (%d)\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            tmpfile,
                                            *__errno_location(),
                                        );
                                        rc = WGET_E_OPEN as libc::c_int;
                                    } else if save_func
                                        .expect("non-null function pointer")(context, fp) != 0
                                    {
                                        unlink(tmpfile);
                                        rc = WGET_E_UNKNOWN as libc::c_int;
                                    } else if rpl_fclose(fp) != 0 {
                                        fp = 0 as *mut FILE;
                                        unlink(tmpfile);
                                        wget_error_printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Failed to write/close '%s' (%d)\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            tmpfile,
                                            *__errno_location(),
                                        );
                                        rc = WGET_E_IO as libc::c_int;
                                    } else {
                                        fp = 0 as *mut FILE;
                                        if rename(tmpfile, fname) == -(1 as libc::c_int) {
                                            wget_error_printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Failed to rename '%s' to '%s' (%d)\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                tmpfile,
                                                fname,
                                                *__errno_location(),
                                            );
                                            wget_error_printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Take manually care for '%s'\n\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                tmpfile,
                                            );
                                            rc = WGET_E_IO as libc::c_int;
                                        } else {
                                            wget_debug_printf(
                                                b"Successfully updated '%s'.\n\0" as *const u8
                                                    as *const libc::c_char,
                                                fname,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !fp.is_null() {
        rpl_fclose(fp);
    }
    if lockfd != -(1 as libc::c_int) {
        close(lockfd);
    }
    if !lockfile.is_null() {
        wget_free.expect("non-null function pointer")(lockfile as *mut libc::c_void);
        lockfile = 0 as *mut libc::c_char;
    }
    if !basename.is_null() {
        wget_free.expect("non-null function pointer")(basename as *mut libc::c_void);
        basename = 0 as *mut libc::c_char;
    }
    if !tmpfile.is_null() {
        wget_free.expect("non-null function pointer")(tmpfile as *mut libc::c_void);
        tmpfile = 0 as *mut libc::c_char;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_truncate(
    mut path: *const libc::c_char,
    mut length: off_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    if path.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    fd = open(path, 0o2 as libc::c_int | 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return WGET_E_OPEN as libc::c_int;
    }
    rc = ftruncate(fd, length);
    close(fd);
    return if rc != 0 {
        WGET_E_IO as libc::c_int
    } else {
        WGET_E_SUCCESS as libc::c_int
    };
}
