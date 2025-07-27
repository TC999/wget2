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
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
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
    fn wget_ip_is_family(host: *const libc::c_char, family: libc::c_int) -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
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
pub type uint16_t = __uint16_t;
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
pub struct wget_hsts_db_st {
    pub fname: *const libc::c_char,
    pub entries: *mut wget_hashmap,
    pub mutex: wget_thread_mutex,
    pub load_time: int64_t,
}
pub type wget_hsts_db = wget_hsts_db_st;
pub type wget_hsts_host_match_fn = unsafe extern "C" fn(
    *const wget_hsts_db,
    *const libc::c_char,
    uint16_t,
) -> libc::c_int;
pub type wget_hsts_db_init_fn = unsafe extern "C" fn(
    *mut wget_hsts_db,
    *const libc::c_char,
) -> *mut wget_hsts_db;
pub type wget_hsts_db_deinit_fn = unsafe extern "C" fn(*mut wget_hsts_db) -> ();
pub type wget_hsts_db_free_fn = unsafe extern "C" fn(*mut *mut wget_hsts_db) -> ();
pub type wget_hsts_db_add_fn = unsafe extern "C" fn(
    *mut wget_hsts_db,
    *const libc::c_char,
    uint16_t,
    int64_t,
    bool,
) -> ();
pub type wget_hsts_db_save_fn = unsafe extern "C" fn(*mut wget_hsts_db) -> libc::c_int;
pub type wget_hsts_db_load_fn = unsafe extern "C" fn(*mut wget_hsts_db) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hsts_db_vtable {
    pub host_match: Option::<wget_hsts_host_match_fn>,
    pub init: Option::<wget_hsts_db_init_fn>,
    pub deinit: Option::<wget_hsts_db_deinit_fn>,
    pub free: Option::<wget_hsts_db_free_fn>,
    pub add: Option::<wget_hsts_db_add_fn>,
    pub load: Option::<wget_hsts_db_load_fn>,
    pub save: Option::<wget_hsts_db_save_fn>,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct hsts_entry {
    pub host: *const libc::c_char,
    pub expires: int64_t,
    pub created: int64_t,
    pub maxage: int64_t,
    pub port: uint16_t,
    #[bitfield(name = "include_subdomains", ty = "bool", bits = "0..=0")]
    pub include_subdomains: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
static mut plugin_vtable: *const wget_hsts_db_vtable = 0 as *const wget_hsts_db_vtable;
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_set_plugin(mut vtable: *const wget_hsts_db_vtable) {
    plugin_vtable = vtable;
}
unsafe extern "C" fn hash_hsts(mut hsts: *const hsts_entry) -> libc::c_uint {
    let mut hash: libc::c_uint = (*hsts).port as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    p = (*hsts).host as *mut libc::c_uchar;
    while *p != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn compare_hsts(
    mut h1: *const hsts_entry,
    mut h2: *const hsts_entry,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = strcmp((*h1).host, (*h2).host);
    if n != 0 {
        return n;
    }
    return if ((*h1).port as libc::c_int) < (*h2).port as libc::c_int {
        -(1 as libc::c_int)
    } else if (*h1).port as libc::c_int > (*h2).port as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn init_hsts(mut hsts: *mut hsts_entry) -> *mut hsts_entry {
    if hsts.is_null() {
        hsts = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<hsts_entry>() as libc::c_ulong,
        ) as *mut hsts_entry;
        if hsts.is_null() {
            return 0 as *mut hsts_entry;
        }
    } else {
        memset(
            hsts as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<hsts_entry>() as libc::c_ulong,
        );
    }
    (*hsts).created = time(0 as *mut time_t);
    return hsts;
}
unsafe extern "C" fn deinit_hsts(mut hsts: *mut hsts_entry) {
    if !hsts.is_null() {
        if !((*hsts).host).is_null() {
            wget_free
                .expect("non-null function pointer")((*hsts).host as *mut libc::c_void);
            (*hsts).host = 0 as *const libc::c_char;
        }
    }
}
unsafe extern "C" fn free_hsts(mut hsts: *mut hsts_entry) {
    if !hsts.is_null() {
        deinit_hsts(hsts);
        if !hsts.is_null() {
            wget_free.expect("non-null function pointer")(hsts as *mut libc::c_void);
            hsts = 0 as *mut hsts_entry;
        }
    }
}
unsafe extern "C" fn new_hsts(
    mut host: *const libc::c_char,
    mut port: uint16_t,
    mut maxage: int64_t,
    mut include_subdomains: bool,
) -> *mut hsts_entry {
    let mut hsts: *mut hsts_entry = init_hsts(0 as *mut hsts_entry);
    if hsts.is_null() {
        return 0 as *mut hsts_entry;
    }
    (*hsts).host = wget_strdup(host);
    (*hsts)
        .port = (if port as libc::c_int != 0 {
        port as libc::c_int
    } else {
        443 as libc::c_int
    }) as uint16_t;
    (*hsts).set_include_subdomains(include_subdomains);
    if maxage <= 0 as libc::c_int as int64_t
        || maxage
            >= 9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long
        || (*hsts).created < 0 as libc::c_int as int64_t
        || (*hsts).created
            >= 9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long
    {
        (*hsts).maxage = 0 as libc::c_int as int64_t;
        (*hsts).expires = 0 as libc::c_int as int64_t;
    } else {
        (*hsts).maxage = maxage;
        (*hsts).expires = (*hsts).created + maxage;
    }
    return hsts;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_host_match(
    mut hsts_db: *const wget_hsts_db,
    mut host: *const libc::c_char,
    mut port: uint16_t,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).host_match)
            .expect("non-null function pointer")(hsts_db, host, port);
    }
    if hsts_db.is_null() {
        return 0 as libc::c_int;
    }
    let mut hsts: hsts_entry = hsts_entry {
        host: 0 as *const libc::c_char,
        expires: 0,
        created: 0,
        maxage: 0,
        port: 0,
        include_subdomains: [0; 1],
        c2rust_padding: [0; 5],
    };
    let mut hstsp: *mut hsts_entry = 0 as *mut hsts_entry;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut now: int64_t = time(0 as *mut time_t);
    hsts
        .port = (if port as libc::c_int == 80 as libc::c_int {
        443 as libc::c_int
    } else {
        port as libc::c_int
    }) as uint16_t;
    hsts.host = host;
    if wget_hashmap_get(
        (*hsts_db).entries,
        &mut hsts as *mut hsts_entry as *const libc::c_void,
        &mut hstsp as *mut *mut hsts_entry as *mut *mut libc::c_void,
    ) != 0 && (*hstsp).expires >= now
    {
        return 1 as libc::c_int;
    }
    p = host;
    loop {
        p = strchr(p, '.' as i32);
        if p.is_null() {
            break;
        }
        p = p.offset(1);
        hsts.host = p;
        if wget_hashmap_get(
            (*hsts_db).entries,
            &mut hsts as *mut hsts_entry as *const libc::c_void,
            &mut hstsp as *mut *mut hsts_entry as *mut *mut libc::c_void,
        ) != 0 && (*hstsp).include_subdomains() as libc::c_int != 0
            && (*hstsp).expires >= now
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_db_deinit(mut hsts_db: *mut wget_hsts_db) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).deinit).expect("non-null function pointer")(hsts_db);
        return;
    }
    if !hsts_db.is_null() {
        if !((*hsts_db).fname).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*hsts_db).fname as *mut libc::c_void);
            (*hsts_db).fname = 0 as *const libc::c_char;
        }
        wget_thread_mutex_lock((*hsts_db).mutex);
        wget_hashmap_free(&mut (*hsts_db).entries);
        wget_thread_mutex_unlock((*hsts_db).mutex);
        wget_thread_mutex_destroy(&mut (*hsts_db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_db_free(mut hsts_db: *mut *mut wget_hsts_db) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).free).expect("non-null function pointer")(hsts_db);
        return;
    }
    if !hsts_db.is_null() && !(*hsts_db).is_null() {
        wget_hsts_db_deinit(*hsts_db);
        if !(*hsts_db).is_null() {
            wget_free.expect("non-null function pointer")(*hsts_db as *mut libc::c_void);
            *hsts_db = 0 as *mut wget_hsts_db;
        }
    }
}
unsafe extern "C" fn hsts_db_add_entry(
    mut hsts_db: *mut wget_hsts_db,
    mut hsts: *mut hsts_entry,
) {
    if hsts.is_null() {
        return;
    }
    wget_thread_mutex_lock((*hsts_db).mutex);
    if (*hsts).maxage == 0 as libc::c_int as int64_t {
        if wget_hashmap_remove((*hsts_db).entries, hsts as *const libc::c_void) != 0 {
            if wget_ip_is_family((*hsts).host, 2 as libc::c_int) {
                wget_debug_printf(
                    b"removed HSTS [%s]:%hu\n\0" as *const u8 as *const libc::c_char,
                    (*hsts).host,
                    (*hsts).port as libc::c_int,
                );
            } else {
                wget_debug_printf(
                    b"removed HSTS %s:%hu\n\0" as *const u8 as *const libc::c_char,
                    (*hsts).host,
                    (*hsts).port as libc::c_int,
                );
            }
        }
        free_hsts(hsts);
        hsts = 0 as *mut hsts_entry;
    } else {
        let mut old: *mut hsts_entry = 0 as *mut hsts_entry;
        if wget_hashmap_get(
            (*hsts_db).entries,
            hsts as *const libc::c_void,
            &mut old as *mut *mut hsts_entry as *mut *mut libc::c_void,
        ) != 0
        {
            if (*old).created < (*hsts).created || (*old).maxage != (*hsts).maxage
                || (*old).include_subdomains() as libc::c_int
                    != (*hsts).include_subdomains() as libc::c_int
            {
                (*old).created = (*hsts).created;
                (*old).expires = (*hsts).expires;
                (*old).maxage = (*hsts).maxage;
                (*old).set_include_subdomains((*hsts).include_subdomains());
                if wget_ip_is_family((*old).host, 2 as libc::c_int) {
                    wget_debug_printf(
                        b"update HSTS [%s]:%hu (maxage=%lld, includeSubDomains=%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        (*old).host,
                        (*old).port as libc::c_int,
                        (*old).maxage as libc::c_longlong,
                        (*old).include_subdomains() as libc::c_int,
                    );
                } else {
                    wget_debug_printf(
                        b"update HSTS %s:%hu (maxage=%lld, includeSubDomains=%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        (*old).host,
                        (*old).port as libc::c_int,
                        (*old).maxage as libc::c_longlong,
                        (*old).include_subdomains() as libc::c_int,
                    );
                }
            }
            free_hsts(hsts);
            hsts = 0 as *mut hsts_entry;
        } else {
            wget_hashmap_put(
                (*hsts_db).entries,
                hsts as *const libc::c_void,
                hsts as *const libc::c_void,
            );
        }
    }
    wget_thread_mutex_unlock((*hsts_db).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_db_add(
    mut hsts_db: *mut wget_hsts_db,
    mut host: *const libc::c_char,
    mut port: uint16_t,
    mut maxage: int64_t,
    mut include_subdomains: bool,
) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).add)
            .expect(
                "non-null function pointer",
            )(hsts_db, host, port, maxage, include_subdomains);
        return;
    }
    if !hsts_db.is_null() {
        let mut hsts: *mut hsts_entry = new_hsts(host, port, maxage, include_subdomains);
        hsts_db_add_entry(hsts_db, hsts);
    }
}
unsafe extern "C" fn hsts_db_load(
    mut hsts_db: *mut wget_hsts_db,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut hsts: hsts_entry = hsts_entry {
        host: 0 as *const libc::c_char,
        expires: 0,
        created: 0,
        maxage: 0,
        port: 0,
        include_subdomains: [0; 1],
        c2rust_padding: [0; 5],
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
    let mut ok: libc::c_int = 0;
    if fstat(fileno(fp), &mut st) == 0 as libc::c_int {
        if st.st_mtim.tv_sec != (*hsts_db).load_time {
            (*hsts_db).load_time = st.st_mtim.tv_sec;
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
        init_hsts(&mut hsts);
        ok = 0 as libc::c_int;
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
            hsts
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
            hsts.port = atoi(p) as uint16_t;
            if hsts.port as libc::c_int == 0 as libc::c_int {
                hsts.port = 443 as libc::c_int as uint16_t;
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
            hsts.set_include_subdomains(
                if atoi(p) != 0 { 1 as libc::c_int } else { 0 as libc::c_int } != 0,
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
            hsts.created = atoll(p) as int64_t;
            if hsts.created < 0 as libc::c_int as int64_t
                || hsts.created
                    >= 9223372036854775807 as libc::c_long
                        / 2 as libc::c_int as libc::c_long
            {
                hsts.created = 0 as libc::c_int as int64_t;
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
            hsts.maxage = atoll(p) as int64_t;
            if hsts.maxage < 0 as libc::c_int as int64_t
                || hsts.maxage
                    >= 9223372036854775807 as libc::c_long
                        / 2 as libc::c_int as libc::c_long
            {
                hsts.maxage = 0 as libc::c_int as int64_t;
            }
            hsts
                .expires = if hsts.maxage != 0 {
                hsts.created + hsts.maxage
            } else {
                0 as libc::c_int as int64_t
            };
            if hsts.expires < now {
                deinit_hsts(&mut hsts);
                continue;
            } else {
                ok = 1 as libc::c_int;
            }
        }
        if ok != 0 {
            hsts_db_add_entry(
                hsts_db,
                wget_memdup(
                    &mut hsts as *mut hsts_entry as *const libc::c_void,
                    ::core::mem::size_of::<hsts_entry>() as libc::c_ulong,
                ) as *mut hsts_entry,
            );
        } else {
            deinit_hsts(&mut hsts);
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
        (*hsts_db).load_time = 0 as libc::c_int as int64_t;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_db_load(
    mut hsts_db: *mut wget_hsts_db,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).load).expect("non-null function pointer")(hsts_db);
    }
    if hsts_db.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*hsts_db).fname).is_null() || *(*hsts_db).fname == 0 {
        return 0 as libc::c_int;
    }
    if wget_update_file(
        (*hsts_db).fname,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_hsts_db, *mut FILE) -> libc::c_int>,
            Option::<wget_update_load_fn>,
        >(
            Some(
                hsts_db_load
                    as unsafe extern "C" fn(*mut wget_hsts_db, *mut FILE) -> libc::c_int,
            ),
        ),
        None,
        hsts_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read HSTS data\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    } else {
        wget_debug_printf(
            b"Fetched HSTS data from '%s'\n\0" as *const u8 as *const libc::c_char,
            (*hsts_db).fname,
        );
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn hsts_save(
    mut _fp: *mut libc::c_void,
    mut _hsts: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut hsts: *const hsts_entry = _hsts as *const hsts_entry;
    wget_fprintf(
        fp,
        b"%s %hu %d %lld %lld\n\0" as *const u8 as *const libc::c_char,
        (*hsts).host,
        (*hsts).port as libc::c_int,
        (*hsts).include_subdomains() as libc::c_int,
        (*hsts).created as libc::c_longlong,
        (*hsts).maxage as libc::c_longlong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn hsts_db_save(
    mut hsts_db: *mut libc::c_void,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut entries: *mut wget_hashmap = (*(hsts_db as *mut wget_hsts_db)).entries;
    if wget_hashmap_size(entries) > 0 as libc::c_int {
        fputs(b"#HSTS 1.0 file\n\0" as *const u8 as *const libc::c_char, fp);
        fputs(
            b"#Generated by libwget 2.2.0. Edit at your own risk.\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        fputs(
            b"# <hostname> <port> <incl. subdomains> <created> <max-age>\n\0"
                as *const u8 as *const libc::c_char,
            fp,
        );
        wget_hashmap_browse(
            entries,
            Some(
                hsts_save
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
pub unsafe extern "C" fn wget_hsts_db_save(
    mut hsts_db: *mut wget_hsts_db,
) -> libc::c_int {
    let mut size: libc::c_int = 0;
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).save).expect("non-null function pointer")(hsts_db);
    }
    if hsts_db.is_null() {
        return -(1 as libc::c_int);
    }
    if ((*hsts_db).fname).is_null() || *(*hsts_db).fname == 0 {
        return -(1 as libc::c_int);
    }
    if wget_update_file(
        (*hsts_db).fname,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_hsts_db, *mut FILE) -> libc::c_int>,
            Option::<wget_update_load_fn>,
        >(
            Some(
                hsts_db_load
                    as unsafe extern "C" fn(*mut wget_hsts_db, *mut FILE) -> libc::c_int,
            ),
        ),
        Some(
            hsts_db_save
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        hsts_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write HSTS file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*hsts_db).fname,
        );
        return -(1 as libc::c_int);
    }
    size = wget_hashmap_size((*hsts_db).entries);
    if size != 0 {
        wget_debug_printf(
            b"Saved %d HSTS entr%s into '%s'\n\0" as *const u8 as *const libc::c_char,
            size,
            if size != 1 as libc::c_int {
                b"ies\0" as *const u8 as *const libc::c_char
            } else {
                b"y\0" as *const u8 as *const libc::c_char
            },
            (*hsts_db).fname,
        );
    } else {
        wget_debug_printf(
            b"No HSTS entries to save. Table is empty.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_db_init(
    mut hsts_db: *mut wget_hsts_db,
    mut fname: *const libc::c_char,
) -> *mut wget_hsts_db {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).init)
            .expect("non-null function pointer")(hsts_db, fname);
    }
    if !fname.is_null() {
        fname = wget_strdup(fname);
        if fname.is_null() {
            return 0 as *mut wget_hsts_db;
        }
    }
    let mut entries: *mut wget_hashmap = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const hsts_entry) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_hsts as unsafe extern "C" fn(*const hsts_entry) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const hsts_entry, *const hsts_entry) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_hsts
                    as unsafe extern "C" fn(
                        *const hsts_entry,
                        *const hsts_entry,
                    ) -> libc::c_int,
            ),
        ),
    );
    if entries.is_null() {
        if !fname.is_null() {
            wget_free.expect("non-null function pointer")(fname as *mut libc::c_void);
            fname = 0 as *const libc::c_char;
        }
        return 0 as *mut wget_hsts_db;
    }
    if hsts_db.is_null() {
        hsts_db = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_hsts_db_st>() as libc::c_ulong,
        ) as *mut wget_hsts_db;
        if hsts_db.is_null() {
            wget_hashmap_free(&mut entries);
            if !fname.is_null() {
                wget_free
                    .expect("non-null function pointer")(fname as *mut libc::c_void);
                fname = 0 as *const libc::c_char;
            }
            return 0 as *mut wget_hsts_db;
        }
    } else {
        memset(
            hsts_db as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_hsts_db>() as libc::c_ulong,
        );
    }
    (*hsts_db).fname = fname;
    (*hsts_db).entries = entries;
    wget_hashmap_set_key_destructor(
        (*hsts_db).entries,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut hsts_entry) -> ()>,
            Option::<wget_hashmap_key_destructor>,
        >(Some(free_hsts as unsafe extern "C" fn(*mut hsts_entry) -> ())),
    );
    wget_hashmap_set_value_destructor(
        (*hsts_db).entries,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut hsts_entry) -> ()>,
            Option::<wget_hashmap_value_destructor>,
        >(Some(free_hsts as unsafe extern "C" fn(*mut hsts_entry) -> ())),
    );
    wget_thread_mutex_init(&mut (*hsts_db).mutex);
    return hsts_db;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hsts_db_set_fname(
    mut hsts_db: *mut wget_hsts_db,
    mut fname: *const libc::c_char,
) {
    if !((*hsts_db).fname).is_null() {
        wget_free
            .expect("non-null function pointer")((*hsts_db).fname as *mut libc::c_void);
        (*hsts_db).fname = 0 as *const libc::c_char;
    }
    (*hsts_db).fname = wget_strdup(fname);
}
