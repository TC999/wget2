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
    pub type wget_hashmap_st;
    pub type wget_thread_mutex_st;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn time(__timer: *mut time_t) -> time_t;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn wget_getline(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        fp: *mut FILE,
    ) -> ssize_t;
    fn wget_update_file(
        fname: *const libc::c_char,
        load_func: Option::<wget_update_load_fn>,
        save_func: Option::<wget_update_save_fn>,
        context: *mut libc::c_void,
    ) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_base64_get_decoded_length(len: size_t) -> size_t;
    fn wget_base64_get_encoded_length(len: size_t) -> size_t;
    fn wget_base64_decode(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        n: size_t,
    ) -> size_t;
    fn wget_base64_encode(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        n: size_t,
    ) -> size_t;
    fn wget_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_hashmap_create(
        max: libc::c_int,
        hash: Option::<wget_hashmap_hash_fn>,
        cmp: Option::<wget_hashmap_compare_fn>,
    ) -> *mut wget_hashmap;
    fn wget_hashmap_put(
        h: *mut wget_hashmap,
        key: *const libc::c_void,
        value: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_size(h: *const wget_hashmap) -> libc::c_int;
    fn wget_hashmap_browse(
        h: *const wget_hashmap,
        browse: Option::<wget_hashmap_browse_fn>,
        ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_free(h: *mut *mut wget_hashmap);
    fn wget_hashmap_get(
        h: *const wget_hashmap,
        key: *const libc::c_void,
        value: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_remove(
        h: *mut wget_hashmap,
        key: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_set_key_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_key_destructor>,
    );
    fn wget_hashmap_set_value_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_value_destructor>,
    );
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type wget_update_load_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut FILE,
) -> libc::c_int;
pub type wget_update_save_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut FILE,
) -> libc::c_int;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_hash_fn = unsafe extern "C" fn(
    *const libc::c_void,
) -> libc::c_uint;
pub type wget_hashmap_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_tls_session_st {
    pub host: *const libc::c_char,
    pub expires: int64_t,
    pub created: int64_t,
    pub maxage: int64_t,
    pub data_size: size_t,
    pub data: *const libc::c_char,
}
pub type wget_tls_session = wget_tls_session_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_tls_session_db_st {
    pub entries: *mut wget_hashmap,
    pub mutex: wget_thread_mutex,
    pub load_time: int64_t,
    #[bitfield(name = "changed", ty = "bool", bits = "0..=0")]
    pub changed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type wget_tls_session_db = wget_tls_session_db_st;
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
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
unsafe extern "C" fn hash_tls_session(
    mut tls_session: *const wget_tls_session,
) -> libc::c_uint {
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    p = (*tls_session).host as *mut libc::c_uchar;
    while *p != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn compare_tls_session(
    mut s1: *const wget_tls_session,
    mut s2: *const wget_tls_session,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = strcmp((*s1).host, (*s2).host);
    if n != 0 {
        return n;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_init(
    mut tls_session: *mut wget_tls_session,
) -> *mut wget_tls_session {
    if tls_session.is_null() {
        tls_session = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_tls_session>() as libc::c_ulong,
        ) as *mut wget_tls_session;
        if tls_session.is_null() {
            return 0 as *mut wget_tls_session;
        }
    } else {
        memset(
            tls_session as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_tls_session>() as libc::c_ulong,
        );
    }
    (*tls_session).created = time(0 as *mut time_t);
    return tls_session;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_deinit(
    mut tls_session: *mut wget_tls_session,
) {
    if !tls_session.is_null() {
        if !((*tls_session).host).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*tls_session).host as *mut libc::c_void);
            (*tls_session).host = 0 as *const libc::c_char;
        }
        if !((*tls_session).data).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*tls_session).data as *mut libc::c_void);
            (*tls_session).data = 0 as *const libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_free(mut tls_session: *mut wget_tls_session) {
    if !tls_session.is_null() {
        wget_tls_session_deinit(tls_session);
        if !tls_session.is_null() {
            wget_free
                .expect("non-null function pointer")(tls_session as *mut libc::c_void);
            tls_session = 0 as *mut wget_tls_session;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_new(
    mut host: *const libc::c_char,
    mut maxage: int64_t,
    mut data: *const libc::c_void,
    mut data_size: size_t,
) -> *mut wget_tls_session {
    let mut tls_session: *mut wget_tls_session = wget_tls_session_init(
        0 as *mut wget_tls_session,
    );
    if !tls_session.is_null() {
        (*tls_session).host = wget_strdup(host);
        (*tls_session).data = wget_memdup(data, data_size) as *const libc::c_char;
        (*tls_session).data_size = data_size;
        if maxage <= 0 as libc::c_int as int64_t
            || maxage
                >= 9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long
            || (*tls_session).created < 0 as libc::c_int as int64_t
            || (*tls_session).created
                >= 9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long
        {
            (*tls_session).maxage = 0 as libc::c_int as int64_t;
            (*tls_session).expires = 0 as libc::c_int as int64_t;
        } else {
            (*tls_session).maxage = maxage;
            (*tls_session).expires = (*tls_session).created + maxage;
        }
    }
    return tls_session;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_get(
    mut tls_session_db: *const wget_tls_session_db,
    mut host: *const libc::c_char,
    mut data: *mut *mut libc::c_void,
    mut size: *mut size_t,
) -> libc::c_int {
    if !tls_session_db.is_null() {
        let mut tls_session: wget_tls_session = wget_tls_session_st {
            host: 0 as *const libc::c_char,
            expires: 0,
            created: 0,
            maxage: 0,
            data_size: 0,
            data: 0 as *const libc::c_char,
        };
        let mut tls_sessionp: *mut wget_tls_session = 0 as *mut wget_tls_session;
        let mut now: int64_t = time(0 as *mut time_t);
        tls_session.host = host;
        if wget_hashmap_get(
            (*tls_session_db).entries,
            &mut tls_session as *mut wget_tls_session as *const libc::c_void,
            &mut tls_sessionp as *mut *mut wget_tls_session as *mut *mut libc::c_void,
        ) != 0 && (*tls_sessionp).expires >= now
        {
            if !data.is_null() {
                *data = wget_memdup(
                    (*tls_sessionp).data as *const libc::c_void,
                    (*tls_sessionp).data_size,
                );
            }
            if !size.is_null() {
                *size = (*tls_sessionp).data_size;
            }
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_init(
    mut tls_session_db: *mut wget_tls_session_db,
) -> *mut wget_tls_session_db {
    let mut entries: *mut wget_hashmap = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const wget_tls_session) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(
            Some(
                hash_tls_session
                    as unsafe extern "C" fn(*const wget_tls_session) -> libc::c_uint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const wget_tls_session,
                    *const wget_tls_session,
                ) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_tls_session
                    as unsafe extern "C" fn(
                        *const wget_tls_session,
                        *const wget_tls_session,
                    ) -> libc::c_int,
            ),
        ),
    );
    if entries.is_null() {
        return 0 as *mut wget_tls_session_db;
    }
    if tls_session_db.is_null() {
        tls_session_db = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_tls_session_db>() as libc::c_ulong,
        ) as *mut wget_tls_session_db;
        if tls_session_db.is_null() {
            wget_hashmap_free(&mut entries);
            return 0 as *mut wget_tls_session_db;
        }
    } else {
        memset(
            tls_session_db as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_tls_session_db>() as libc::c_ulong,
        );
    }
    wget_hashmap_set_key_destructor(
        entries,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_tls_session) -> ()>,
            Option::<wget_hashmap_key_destructor>,
        >(
            Some(
                wget_tls_session_free
                    as unsafe extern "C" fn(*mut wget_tls_session) -> (),
            ),
        ),
    );
    wget_hashmap_set_value_destructor(
        entries,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_tls_session) -> ()>,
            Option::<wget_hashmap_value_destructor>,
        >(
            Some(
                wget_tls_session_free
                    as unsafe extern "C" fn(*mut wget_tls_session) -> (),
            ),
        ),
    );
    (*tls_session_db).entries = entries;
    wget_thread_mutex_init(&mut (*tls_session_db).mutex);
    return tls_session_db;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_deinit(
    mut tls_session_db: *mut wget_tls_session_db,
) {
    if !tls_session_db.is_null() {
        wget_thread_mutex_lock((*tls_session_db).mutex);
        wget_hashmap_free(&mut (*tls_session_db).entries);
        wget_thread_mutex_unlock((*tls_session_db).mutex);
        wget_thread_mutex_destroy(&mut (*tls_session_db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_free(
    mut tls_session_db: *mut *mut wget_tls_session_db,
) {
    if !tls_session_db.is_null() {
        wget_tls_session_db_deinit(*tls_session_db);
        if !(*tls_session_db).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(*tls_session_db as *mut libc::c_void);
            *tls_session_db = 0 as *mut wget_tls_session_db;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_add(
    mut tls_session_db: *mut wget_tls_session_db,
    mut tls_session: *mut wget_tls_session,
) {
    if tls_session_db.is_null() || tls_session.is_null() {
        return;
    }
    wget_thread_mutex_lock((*tls_session_db).mutex);
    if (*tls_session).maxage == 0 as libc::c_int as int64_t {
        if wget_hashmap_remove(
            (*tls_session_db).entries,
            tls_session as *const libc::c_void,
        ) != 0
        {
            (*tls_session_db).set_changed(1 as libc::c_int != 0);
            wget_debug_printf(
                b"removed TLS session data for %s\n\0" as *const u8
                    as *const libc::c_char,
                (*tls_session).host,
            );
        }
        wget_tls_session_free(tls_session);
        tls_session = 0 as *mut wget_tls_session;
    } else {
        let mut old: *mut wget_tls_session = 0 as *mut wget_tls_session;
        if wget_hashmap_get(
            (*tls_session_db).entries,
            tls_session as *const libc::c_void,
            &mut old as *mut *mut wget_tls_session as *mut *mut libc::c_void,
        ) != 0
        {
            wget_debug_printf(
                b"found TLS session data for %s\n\0" as *const u8 as *const libc::c_char,
                (*old).host,
            );
            if wget_hashmap_remove((*tls_session_db).entries, old as *const libc::c_void)
                != 0
            {
                wget_debug_printf(
                    b"removed TLS session data for %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*tls_session).host,
                );
            }
        }
        wget_debug_printf(
            b"add TLS session data for %s (maxage=%lld, size=%zu)\n\0" as *const u8
                as *const libc::c_char,
            (*tls_session).host,
            (*tls_session).maxage as libc::c_longlong,
            (*tls_session).data_size,
        );
        wget_hashmap_put(
            (*tls_session_db).entries,
            tls_session as *const libc::c_void,
            tls_session as *const libc::c_void,
        );
        (*tls_session_db).set_changed(1 as libc::c_int != 0);
    }
    wget_thread_mutex_unlock((*tls_session_db).mutex);
}
unsafe extern "C" fn tls_session_db_load(
    mut tls_session_db: *mut wget_tls_session_db,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut tls_session: wget_tls_session = wget_tls_session_st {
        host: 0 as *const libc::c_char,
        expires: 0,
        created: 0,
        maxage: 0,
        data_size: 0,
        data: 0 as *const libc::c_char,
    };
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
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut buflen: ssize_t = 0;
    let mut now: int64_t = time(0 as *mut time_t);
    let mut ok: bool = false;
    if fstat(fileno(fp), &mut st) == 0 as libc::c_int {
        if st.st_mtim.tv_sec != (*tls_session_db).load_time {
            (*tls_session_db).load_time = st.st_mtim.tv_sec;
        } else {
            return 0 as libc::c_int
        }
    }
    loop {
        buflen = wget_getline(&mut buf, &mut bufsize, fp);
        if !(buflen >= 0 as libc::c_int as ssize_t) {
            break;
        }
        linep = buf;
        while *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            linep = linep.offset(1);
            linep;
        }
        if *linep == 0 {
            continue;
        }
        if *linep as libc::c_int == '#' as i32 {
            continue;
        }
        while buflen > 0 as libc::c_int as ssize_t
            && (*buf.offset(buflen as isize) as libc::c_int == '\n' as i32
                || *buf.offset(buflen as isize) as libc::c_int == '\r' as i32)
        {
            buflen -= 1;
            *buf.offset(buflen as isize) = 0 as libc::c_int as libc::c_char;
        }
        wget_tls_session_init(&mut tls_session);
        ok = 0 as libc::c_int != 0;
        if *linep != 0 {
            p = linep;
            while *linep as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                linep = linep.offset(1);
                linep;
            }
            tls_session
                .host = wget_strmemdup(
                p as *const libc::c_void,
                linep.offset_from(p) as libc::c_long as size_t,
            );
        }
        if *linep != 0 {
            linep = linep.offset(1);
            p = linep;
            while *linep as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                linep = linep.offset(1);
                linep;
            }
            tls_session.created = atoll(p) as int64_t;
            if tls_session.created < 0 as libc::c_int as int64_t
                || tls_session.created
                    >= 9223372036854775807 as libc::c_long
                        / 2 as libc::c_int as libc::c_long
            {
                tls_session.created = 0 as libc::c_int as int64_t;
            }
        }
        if *linep != 0 {
            linep = linep.offset(1);
            p = linep;
            while *linep as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                linep = linep.offset(1);
                linep;
            }
            tls_session.maxage = atoll(p) as int64_t;
            if tls_session.maxage < 0 as libc::c_int as int64_t
                || tls_session.maxage
                    >= 9223372036854775807 as libc::c_long
                        / 2 as libc::c_int as libc::c_long
            {
                tls_session.maxage = 0 as libc::c_int as int64_t;
            }
            tls_session
                .expires = if tls_session.maxage != 0 {
                tls_session.created + tls_session.maxage
            } else {
                0 as libc::c_int as int64_t
            };
            if tls_session.expires < now {
                wget_tls_session_deinit(&mut tls_session);
                continue;
            }
        }
        if *linep != 0 {
            linep = linep.offset(1);
            p = linep;
            while *linep as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                linep = linep.offset(1);
                linep;
            }
            let mut len: size_t = linep.offset_from(p) as libc::c_long as size_t;
            let mut data: *mut libc::c_char = wget_malloc(
                wget_base64_get_decoded_length(len),
            ) as *mut libc::c_char;
            if !data.is_null() {
                tls_session.data_size = wget_base64_decode(data, p, len);
                tls_session.data = data;
                ok = 1 as libc::c_int != 0;
            }
        }
        if ok {
            let mut no_change: bool = wget_hashmap_size((*tls_session_db).entries)
                == 0 as libc::c_int;
            wget_tls_session_db_add(
                tls_session_db,
                wget_memdup(
                    &mut tls_session as *mut wget_tls_session as *const libc::c_void,
                    ::core::mem::size_of::<wget_tls_session>() as libc::c_ulong,
                ) as *mut wget_tls_session,
            );
            if no_change {
                (*tls_session_db).set_changed(0 as libc::c_int != 0);
            }
        } else {
            wget_tls_session_deinit(&mut tls_session);
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to parse HSTS line: '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                buf,
            );
        }
    }
    if !buf.is_null() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    if ferror(fp) != 0 {
        (*tls_session_db).load_time = 0 as libc::c_int as int64_t;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_load(
    mut tls_session_db: *mut wget_tls_session_db,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    if tls_session_db.is_null() || fname.is_null() || *fname == 0 {
        return 0 as libc::c_int;
    }
    if wget_update_file(
        fname,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut wget_tls_session_db, *mut FILE) -> libc::c_int,
            >,
            Option::<wget_update_load_fn>,
        >(
            Some(
                tls_session_db_load
                    as unsafe extern "C" fn(
                        *mut wget_tls_session_db,
                        *mut FILE,
                    ) -> libc::c_int,
            ),
        ),
        None,
        tls_session_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read TLS session data\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    } else {
        wget_debug_printf(
            b"Fetched TLS session data from '%s'\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn tls_session_save(
    mut _fp: *mut libc::c_void,
    mut _tls_session: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut tls_session: *const wget_tls_session = _tls_session
        as *const wget_tls_session;
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut session_b64: *mut libc::c_char = tmp.as_mut_ptr();
    let mut b64_len: size_t = wget_base64_get_encoded_length((*tls_session).data_size);
    if b64_len > ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        session_b64 = wget_malloc(b64_len) as *mut libc::c_char;
        if session_b64.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to allocate %zu bytes\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b64_len,
            );
            return 0 as libc::c_int;
        }
    }
    wget_base64_encode(session_b64, (*tls_session).data, (*tls_session).data_size);
    wget_fprintf(
        fp,
        b"%s %lld %lld %s\n\0" as *const u8 as *const libc::c_char,
        (*tls_session).host,
        (*tls_session).created as libc::c_longlong,
        (*tls_session).maxage as libc::c_longlong,
        session_b64,
    );
    if session_b64 != tmp.as_mut_ptr() {
        if !session_b64.is_null() {
            wget_free
                .expect("non-null function pointer")(session_b64 as *mut libc::c_void);
            session_b64 = 0 as *mut libc::c_char;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn tls_session_db_save(
    mut tls_session_db: *mut libc::c_void,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut entries: *mut wget_hashmap = (*(tls_session_db as *mut wget_tls_session_db))
        .entries;
    if wget_hashmap_size(entries) > 0 as libc::c_int {
        fputs(b"#TLSSession 1.0 file\n\0" as *const u8 as *const libc::c_char, fp);
        fputs(
            b"#Generated by libwget 2.2.0. Edit at your own risk.\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        fputs(
            b"#<hostname>  <created> <max-age> <session data>\n\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        wget_hashmap_browse(
            entries,
            Some(
                tls_session_save
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            fp as *mut libc::c_void,
        );
        if ferror(fp) != 0 {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_save(
    mut tls_session_db: *mut wget_tls_session_db,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut size: libc::c_int = 0;
    if tls_session_db.is_null() || fname.is_null() || *fname == 0 {
        return -(1 as libc::c_int);
    }
    if wget_update_file(
        fname,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut wget_tls_session_db, *mut FILE) -> libc::c_int,
            >,
            Option::<wget_update_load_fn>,
        >(
            Some(
                tls_session_db_load
                    as unsafe extern "C" fn(
                        *mut wget_tls_session_db,
                        *mut FILE,
                    ) -> libc::c_int,
            ),
        ),
        Some(
            tls_session_db_save
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        tls_session_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write TLS session file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        return -(1 as libc::c_int);
    }
    size = wget_hashmap_size((*tls_session_db).entries);
    if size != 0 {
        wget_debug_printf(
            b"Saved %d TLS session entr%s into '%s'\n\0" as *const u8
                as *const libc::c_char,
            size,
            if size != 1 as libc::c_int {
                b"ies\0" as *const u8 as *const libc::c_char
            } else {
                b"y\0" as *const u8 as *const libc::c_char
            },
            fname,
        );
    } else {
        wget_debug_printf(
            b"No TLS session entries to save. Table is empty.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*tls_session_db).set_changed(0 as libc::c_int != 0);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tls_session_db_changed(
    mut tls_session_db: *mut wget_tls_session_db,
) -> libc::c_int {
    return if !tls_session_db.is_null() {
        (*tls_session_db).changed() as libc::c_int
    } else {
        0 as libc::c_int
    };
}
