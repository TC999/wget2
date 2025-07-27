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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn time(__timer: *mut time_t) -> time_t;
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
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub struct wget_ocsp_db_st {
    pub fname: *const libc::c_char,
    pub fingerprints: *mut wget_hashmap,
    pub hosts: *mut wget_hashmap,
    pub mutex: wget_thread_mutex,
}
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_ocsp_db_init_fn = unsafe extern "C" fn(
    *mut wget_ocsp_db,
    *const libc::c_char,
) -> *mut wget_ocsp_db;
pub type wget_ocsp_db_deinit_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> ();
pub type wget_ocsp_db_free_fn = unsafe extern "C" fn(*mut *mut wget_ocsp_db) -> ();
pub type wget_ocsp_fingerprint_in_cache_fn = unsafe extern "C" fn(
    *const wget_ocsp_db,
    *const libc::c_char,
    *mut libc::c_int,
) -> bool;
pub type wget_ocsp_hostname_is_valid_fn = unsafe extern "C" fn(
    *const wget_ocsp_db,
    *const libc::c_char,
) -> bool;
pub type wget_ocsp_db_add_fingerprint_fn = unsafe extern "C" fn(
    *mut wget_ocsp_db,
    *const libc::c_char,
    int64_t,
    bool,
) -> ();
pub type wget_ocsp_db_add_host_fn = unsafe extern "C" fn(
    *mut wget_ocsp_db,
    *const libc::c_char,
    int64_t,
) -> ();
pub type wget_ocsp_db_save_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> libc::c_int;
pub type wget_ocsp_db_load_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_ocsp_db_vtable {
    pub init: Option::<wget_ocsp_db_init_fn>,
    pub deinit: Option::<wget_ocsp_db_deinit_fn>,
    pub free: Option::<wget_ocsp_db_free_fn>,
    pub fingerprint_in_cache: Option::<wget_ocsp_fingerprint_in_cache_fn>,
    pub hostname_is_valid: Option::<wget_ocsp_hostname_is_valid_fn>,
    pub add_fingerprint: Option::<wget_ocsp_db_add_fingerprint_fn>,
    pub add_host: Option::<wget_ocsp_db_add_host_fn>,
    pub load: Option::<wget_ocsp_db_save_fn>,
    pub save: Option::<wget_ocsp_db_load_fn>,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct ocsp_entry {
    pub key: *const libc::c_char,
    pub maxage: int64_t,
    pub mtime: int64_t,
    #[bitfield(name = "valid", ty = "bool", bits = "0..=0")]
    pub valid: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
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
static mut plugin_vtable: *const wget_ocsp_db_vtable = 0 as *const wget_ocsp_db_vtable;
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_set_plugin(mut vtable: *const wget_ocsp_db_vtable) {
    plugin_vtable = vtable;
}
unsafe extern "C" fn hash_ocsp(mut ocsp: *const ocsp_entry) -> libc::c_uint {
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    p = (*ocsp).key as *mut libc::c_uchar;
    while *p != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn compare_ocsp(
    mut h1: *const ocsp_entry,
    mut h2: *const ocsp_entry,
) -> libc::c_int {
    return strcmp((*h1).key, (*h2).key);
}
unsafe extern "C" fn init_ocsp(mut ocsp: *mut ocsp_entry) -> *mut ocsp_entry {
    if ocsp.is_null() {
        ocsp = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<ocsp_entry>() as libc::c_ulong,
        ) as *mut ocsp_entry;
        if ocsp.is_null() {
            return 0 as *mut ocsp_entry;
        }
    } else {
        memset(
            ocsp as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<ocsp_entry>() as libc::c_ulong,
        );
    }
    (*ocsp).mtime = time(0 as *mut time_t);
    return ocsp;
}
unsafe extern "C" fn deinit_ocsp(mut ocsp: *mut ocsp_entry) {
    if !ocsp.is_null() {
        if !((*ocsp).key).is_null() {
            wget_free
                .expect("non-null function pointer")((*ocsp).key as *mut libc::c_void);
            (*ocsp).key = 0 as *const libc::c_char;
        }
    }
}
unsafe extern "C" fn free_ocsp(mut ocsp: *mut ocsp_entry) {
    if !ocsp.is_null() {
        deinit_ocsp(ocsp);
        if !ocsp.is_null() {
            wget_free.expect("non-null function pointer")(ocsp as *mut libc::c_void);
            ocsp = 0 as *mut ocsp_entry;
        }
    }
}
unsafe extern "C" fn new_ocsp(
    mut fingerprint: *const libc::c_char,
    mut maxage: int64_t,
    mut valid: bool,
) -> *mut ocsp_entry {
    if !fingerprint.is_null() {
        fingerprint = wget_strdup(fingerprint);
        if fingerprint.is_null() {
            return 0 as *mut ocsp_entry;
        }
    }
    let mut ocsp: *mut ocsp_entry = init_ocsp(0 as *mut ocsp_entry);
    if !ocsp.is_null() {
        (*ocsp).key = fingerprint;
        (*ocsp).maxage = maxage;
        (*ocsp).set_valid(valid);
    } else if !fingerprint.is_null() {
        wget_free.expect("non-null function pointer")(fingerprint as *mut libc::c_void);
        fingerprint = 0 as *const libc::c_char;
    }
    return ocsp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_fingerprint_in_cache(
    mut ocsp_db: *const wget_ocsp_db,
    mut fingerprint: *const libc::c_char,
    mut revoked: *mut libc::c_int,
) -> bool {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).fingerprint_in_cache)
            .expect("non-null function pointer")(ocsp_db, fingerprint, revoked);
    }
    if ocsp_db.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut ocsp: ocsp_entry = ocsp_entry {
        key: 0 as *const libc::c_char,
        maxage: 0,
        mtime: 0,
        valid: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut ocspp: *mut ocsp_entry = 0 as *mut ocsp_entry;
    ocsp.key = fingerprint;
    if wget_hashmap_get(
        (*ocsp_db).fingerprints,
        &mut ocsp as *mut ocsp_entry as *const libc::c_void,
        &mut ocspp as *mut *mut ocsp_entry as *mut *mut libc::c_void,
    ) != 0 && (*ocspp).maxage >= time(0 as *mut time_t)
    {
        if !revoked.is_null() {
            *revoked = !(*ocspp).valid() as libc::c_int;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_hostname_is_valid(
    mut ocsp_db: *const wget_ocsp_db,
    mut hostname: *const libc::c_char,
) -> bool {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).hostname_is_valid)
            .expect("non-null function pointer")(ocsp_db, hostname);
    }
    if ocsp_db.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut ocsp: ocsp_entry = ocsp_entry {
        key: 0 as *const libc::c_char,
        maxage: 0,
        mtime: 0,
        valid: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut ocspp: *mut ocsp_entry = 0 as *mut ocsp_entry;
    ocsp.key = hostname;
    if wget_hashmap_get(
        (*ocsp_db).hosts,
        &mut ocsp as *mut ocsp_entry as *const libc::c_void,
        &mut ocspp as *mut *mut ocsp_entry as *mut *mut libc::c_void,
    ) != 0 && (*ocspp).maxage >= time(0 as *mut time_t)
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_deinit(mut ocsp_db: *mut wget_ocsp_db) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).deinit).expect("non-null function pointer")(ocsp_db);
        return;
    }
    if !ocsp_db.is_null() {
        if !((*ocsp_db).fname).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*ocsp_db).fname as *mut libc::c_void);
            (*ocsp_db).fname = 0 as *const libc::c_char;
        }
        wget_thread_mutex_lock((*ocsp_db).mutex);
        wget_hashmap_free(&mut (*ocsp_db).fingerprints);
        wget_hashmap_free(&mut (*ocsp_db).hosts);
        wget_thread_mutex_unlock((*ocsp_db).mutex);
        wget_thread_mutex_destroy(&mut (*ocsp_db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_free(mut ocsp_db: *mut *mut wget_ocsp_db) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).free).expect("non-null function pointer")(ocsp_db);
        return;
    }
    if !ocsp_db.is_null() && !(*ocsp_db).is_null() {
        wget_ocsp_db_deinit(*ocsp_db);
        if !(*ocsp_db).is_null() {
            wget_free.expect("non-null function pointer")(*ocsp_db as *mut libc::c_void);
            *ocsp_db = 0 as *mut wget_ocsp_db;
        }
    }
}
unsafe extern "C" fn ocsp_db_add_fingerprint_entry(
    mut ocsp_db: *mut wget_ocsp_db,
    mut ocsp: *mut ocsp_entry,
) {
    if ocsp.is_null() {
        return;
    }
    if ocsp_db.is_null() {
        free_ocsp(ocsp);
        return;
    }
    wget_thread_mutex_lock((*ocsp_db).mutex);
    if (*ocsp).maxage == 0 as libc::c_int as int64_t {
        if wget_hashmap_remove((*ocsp_db).fingerprints, ocsp as *const libc::c_void) != 0
        {
            wget_debug_printf(
                b"removed OCSP cert %s\n\0" as *const u8 as *const libc::c_char,
                (*ocsp).key,
            );
        }
        free_ocsp(ocsp);
    } else {
        let mut old: *mut ocsp_entry = 0 as *mut ocsp_entry;
        if wget_hashmap_get(
            (*ocsp_db).fingerprints,
            ocsp as *const libc::c_void,
            &mut old as *mut *mut ocsp_entry as *mut *mut libc::c_void,
        ) != 0
        {
            if (*old).mtime < (*ocsp).mtime {
                (*old).mtime = (*ocsp).mtime;
                (*old).maxage = (*ocsp).maxage;
                (*old).set_valid((*ocsp).valid());
                wget_debug_printf(
                    b"update OCSP cert %s (maxage=%lld,valid=%d)\n\0" as *const u8
                        as *const libc::c_char,
                    (*old).key,
                    (*old).maxage as libc::c_longlong,
                    (*old).valid() as libc::c_int,
                );
            }
            free_ocsp(ocsp);
        } else {
            wget_debug_printf(
                b"add OCSP cert %s (maxage=%lld,valid=%d)\n\0" as *const u8
                    as *const libc::c_char,
                (*ocsp).key,
                (*ocsp).maxage as libc::c_longlong,
                (*ocsp).valid() as libc::c_int,
            );
            wget_hashmap_put(
                (*ocsp_db).fingerprints,
                ocsp as *const libc::c_void,
                ocsp as *const libc::c_void,
            );
        }
    }
    wget_thread_mutex_unlock((*ocsp_db).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_add_fingerprint(
    mut ocsp_db: *mut wget_ocsp_db,
    mut fingerprint: *const libc::c_char,
    mut maxage: int64_t,
    mut valid: bool,
) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).add_fingerprint)
            .expect("non-null function pointer")(ocsp_db, fingerprint, maxage, valid);
        return;
    }
    let mut ocsp: *mut ocsp_entry = new_ocsp(fingerprint, maxage, valid);
    ocsp_db_add_fingerprint_entry(ocsp_db, ocsp);
}
unsafe extern "C" fn ocsp_db_add_host_entry(
    mut ocsp_db: *mut wget_ocsp_db,
    mut ocsp: *mut ocsp_entry,
) {
    if ocsp.is_null() {
        return;
    }
    if ocsp_db.is_null() {
        free_ocsp(ocsp);
        return;
    }
    wget_thread_mutex_lock((*ocsp_db).mutex);
    if (*ocsp).maxage == 0 as libc::c_int as int64_t {
        if wget_hashmap_remove((*ocsp_db).hosts, ocsp as *const libc::c_void) != 0 {
            wget_debug_printf(
                b"removed OCSP host %s\n\0" as *const u8 as *const libc::c_char,
                (*ocsp).key,
            );
        }
        free_ocsp(ocsp);
    } else {
        let mut old: *mut ocsp_entry = 0 as *mut ocsp_entry;
        if wget_hashmap_get(
            (*ocsp_db).hosts,
            ocsp as *const libc::c_void,
            &mut old as *mut *mut ocsp_entry as *mut *mut libc::c_void,
        ) != 0
        {
            if (*old).mtime < (*ocsp).mtime {
                (*old).mtime = (*ocsp).mtime;
                (*old).maxage = (*ocsp).maxage;
                (*old).set_valid((*ocsp).valid());
                wget_debug_printf(
                    b"update OCSP host %s (maxage=%lld)\n\0" as *const u8
                        as *const libc::c_char,
                    (*old).key,
                    (*old).maxage as libc::c_longlong,
                );
            }
            free_ocsp(ocsp);
        } else {
            wget_hashmap_put(
                (*ocsp_db).hosts,
                ocsp as *const libc::c_void,
                ocsp as *const libc::c_void,
            );
            wget_debug_printf(
                b"add OCSP host %s (maxage=%lld)\n\0" as *const u8
                    as *const libc::c_char,
                (*ocsp).key,
                (*ocsp).maxage as libc::c_longlong,
            );
        }
    }
    wget_thread_mutex_unlock((*ocsp_db).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_add_host(
    mut ocsp_db: *mut wget_ocsp_db,
    mut host: *const libc::c_char,
    mut maxage: int64_t,
) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).add_host)
            .expect("non-null function pointer")(ocsp_db, host, maxage);
        return;
    }
    let mut ocsp: *mut ocsp_entry = new_ocsp(host, maxage, 0 as libc::c_int != 0);
    ocsp_db_add_host_entry(ocsp_db, ocsp);
}
unsafe extern "C" fn ocsp_db_load(
    mut ocsp_db: *mut wget_ocsp_db,
    mut fp: *mut FILE,
    mut load_hosts: bool,
) -> libc::c_int {
    let mut ocsp: ocsp_entry = ocsp_entry {
        key: 0 as *const libc::c_char,
        maxage: 0,
        mtime: 0,
        valid: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut buflen: ssize_t = 0;
    let mut now: int64_t = time(0 as *mut time_t);
    let mut ok: libc::c_int = 0;
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
        init_ocsp(&mut ocsp);
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
            ocsp
                .key = wget_strmemdup(
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
            ocsp.maxage = atoll(p) as int64_t;
            if ocsp.maxage < now {
                deinit_ocsp(&mut ocsp);
                continue;
            } else {
                ok = 1 as libc::c_int;
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
            ocsp.mtime = atoll(p) as int64_t;
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
            ocsp.set_valid(atoi(p) != 0 as libc::c_int);
        }
        if ok != 0 {
            if load_hosts {
                ocsp_db_add_host_entry(
                    ocsp_db,
                    wget_memdup(
                        &mut ocsp as *mut ocsp_entry as *const libc::c_void,
                        ::core::mem::size_of::<ocsp_entry>() as libc::c_ulong,
                    ) as *mut ocsp_entry,
                );
            } else {
                ocsp_db_add_fingerprint_entry(
                    ocsp_db,
                    wget_memdup(
                        &mut ocsp as *mut ocsp_entry as *const libc::c_void,
                        ::core::mem::size_of::<ocsp_entry>() as libc::c_ulong,
                    ) as *mut ocsp_entry,
                );
            }
        } else {
            deinit_ocsp(&mut ocsp);
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to parse OCSP line: '%s'\n\0" as *const u8
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
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ocsp_db_load_hosts(
    mut ocsp_db: *mut libc::c_void,
    mut fp: *mut FILE,
) -> libc::c_int {
    return ocsp_db_load(ocsp_db as *mut wget_ocsp_db, fp, 1 as libc::c_int != 0);
}
unsafe extern "C" fn ocsp_db_load_fingerprints(
    mut ocsp_db: *mut libc::c_void,
    mut fp: *mut FILE,
) -> libc::c_int {
    return ocsp_db_load(ocsp_db as *mut wget_ocsp_db, fp, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_load(
    mut ocsp_db: *mut wget_ocsp_db,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).load).expect("non-null function pointer")(ocsp_db);
    }
    let mut ret: libc::c_int = 0;
    if ((*ocsp_db).fname).is_null() || *(*ocsp_db).fname == 0 {
        return -(1 as libc::c_int);
    }
    let mut fname_hosts: *mut libc::c_char = wget_aprintf(
        b"%s_hosts\0" as *const u8 as *const libc::c_char,
        (*ocsp_db).fname,
    );
    ret = wget_update_file(
        fname_hosts,
        Some(
            ocsp_db_load_hosts
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        None,
        ocsp_db as *mut libc::c_void,
    );
    if ret != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read OCSP hosts\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        wget_debug_printf(
            b"Fetched OCSP hosts from '%s'\n\0" as *const u8 as *const libc::c_char,
            fname_hosts,
        );
    }
    if !fname_hosts.is_null() {
        wget_free.expect("non-null function pointer")(fname_hosts as *mut libc::c_void);
        fname_hosts = 0 as *mut libc::c_char;
    }
    if wget_update_file(
        (*ocsp_db).fname,
        Some(
            ocsp_db_load_fingerprints
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        None,
        ocsp_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read OCSP fingerprints\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        ret = -(1 as libc::c_int);
    } else {
        wget_debug_printf(
            b"Fetched OCSP fingerprints from '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*ocsp_db).fname,
        );
    }
    return ret;
}
unsafe extern "C" fn ocsp_save_fingerprint(
    mut _fp: *mut libc::c_void,
    mut _ocsp: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut ocsp: *const ocsp_entry = _ocsp as *const ocsp_entry;
    wget_fprintf(
        fp,
        b"%s %lld %lld %d\n\0" as *const u8 as *const libc::c_char,
        (*ocsp).key,
        (*ocsp).maxage as libc::c_longlong,
        (*ocsp).mtime as libc::c_longlong,
        (*ocsp).valid() as libc::c_int,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ocsp_save_host(
    mut _fp: *mut libc::c_void,
    mut _ocsp: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut ocsp: *const ocsp_entry = _ocsp as *const ocsp_entry;
    wget_fprintf(
        fp,
        b"%s %lld %lld\n\0" as *const u8 as *const libc::c_char,
        (*ocsp).key,
        (*ocsp).maxage as libc::c_longlong,
        (*ocsp).mtime as libc::c_longlong,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ocsp_db_save_hosts(
    mut ocsp_db: *mut libc::c_void,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut map: *mut wget_hashmap = (*(ocsp_db as *mut wget_ocsp_db)).hosts;
    if wget_hashmap_size(map) > 0 as libc::c_int {
        fputs(b"#OCSP 1.0 host file\n\0" as *const u8 as *const libc::c_char, fp);
        fputs(
            b"#Generated by libwget 2.2.0. Edit at your own risk.\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        fputs(
            b"<hostname> <time_t maxage> <time_t mtime>\n\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        wget_hashmap_browse(
            map,
            Some(
                ocsp_save_host
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
unsafe extern "C" fn ocsp_db_save_fingerprints(
    mut ocsp_db: *mut libc::c_void,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut map: *mut wget_hashmap = (*(ocsp_db as *mut wget_ocsp_db)).fingerprints;
    if wget_hashmap_size(map) > 0 as libc::c_int {
        fputs(b"#OCSP 1.0 fingerprint file\n\0" as *const u8 as *const libc::c_char, fp);
        fputs(
            b"#Generated by Wget 2.2.0. Edit at your own risk.\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        fputs(
            b"<sha256 fingerprint of cert> <time_t maxage> <time_t mtime> <valid>\n\n\0"
                as *const u8 as *const libc::c_char,
            fp,
        );
        wget_hashmap_browse(
            map,
            Some(
                ocsp_save_fingerprint
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
pub unsafe extern "C" fn wget_ocsp_db_save(
    mut ocsp_db: *mut wget_ocsp_db,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).save).expect("non-null function pointer")(ocsp_db);
    }
    let mut ret: libc::c_int = 0;
    if ocsp_db.is_null() || ((*ocsp_db).fname).is_null() || *(*ocsp_db).fname == 0 {
        return -(1 as libc::c_int);
    }
    let mut fname_hosts: *mut libc::c_char = wget_aprintf(
        b"%s_hosts\0" as *const u8 as *const libc::c_char,
        (*ocsp_db).fname,
    );
    ret = wget_update_file(
        fname_hosts,
        Some(
            ocsp_db_load_hosts
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        Some(
            ocsp_db_save_hosts
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        ocsp_db as *mut libc::c_void,
    );
    if ret != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write to OCSP hosts to '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname_hosts,
        );
    } else {
        wget_debug_printf(
            b"Saved OCSP hosts to '%s'\n\0" as *const u8 as *const libc::c_char,
            fname_hosts,
        );
    }
    if !fname_hosts.is_null() {
        wget_free.expect("non-null function pointer")(fname_hosts as *mut libc::c_void);
        fname_hosts = 0 as *mut libc::c_char;
    }
    if wget_update_file(
        (*ocsp_db).fname,
        Some(
            ocsp_db_load_fingerprints
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        Some(
            ocsp_db_save_fingerprints
                as unsafe extern "C" fn(*mut libc::c_void, *mut FILE) -> libc::c_int,
        ),
        ocsp_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write to OCSP fingerprints to '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*ocsp_db).fname,
        );
        ret = -(1 as libc::c_int);
    } else {
        wget_debug_printf(
            b"Saved OCSP fingerprints to '%s'\n\0" as *const u8 as *const libc::c_char,
            (*ocsp_db).fname,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_init(
    mut ocsp_db: *mut wget_ocsp_db,
    mut fname: *const libc::c_char,
) -> *mut wget_ocsp_db {
    let mut current_block: u64;
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).init)
            .expect("non-null function pointer")(ocsp_db, fname);
    }
    if !fname.is_null() {
        fname = wget_strdup(fname);
        if fname.is_null() {
            return 0 as *mut wget_ocsp_db;
        }
    }
    let mut fingerprints: *mut wget_hashmap = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const ocsp_entry) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_ocsp as unsafe extern "C" fn(*const ocsp_entry) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const ocsp_entry, *const ocsp_entry) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_ocsp
                    as unsafe extern "C" fn(
                        *const ocsp_entry,
                        *const ocsp_entry,
                    ) -> libc::c_int,
            ),
        ),
    );
    let mut hosts: *mut wget_hashmap = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const ocsp_entry) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_ocsp as unsafe extern "C" fn(*const ocsp_entry) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const ocsp_entry, *const ocsp_entry) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_ocsp
                    as unsafe extern "C" fn(
                        *const ocsp_entry,
                        *const ocsp_entry,
                    ) -> libc::c_int,
            ),
        ),
    );
    if !(fingerprints.is_null() || hosts.is_null()) {
        if ocsp_db.is_null() {
            ocsp_db = wget_calloc(
                1 as libc::c_int as size_t,
                ::core::mem::size_of::<wget_ocsp_db_st>() as libc::c_ulong,
            ) as *mut wget_ocsp_db;
            if ocsp_db.is_null() {
                current_block = 9362285161327210675;
            } else {
                current_block = 13056961889198038528;
            }
        } else {
            memset(
                ocsp_db as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<wget_ocsp_db>() as libc::c_ulong,
            );
            current_block = 13056961889198038528;
        }
        match current_block {
            9362285161327210675 => {}
            _ => {
                (*ocsp_db).fname = fname;
                wget_hashmap_set_key_destructor(
                    fingerprints,
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut ocsp_entry) -> ()>,
                        Option::<wget_hashmap_key_destructor>,
                    >(Some(free_ocsp as unsafe extern "C" fn(*mut ocsp_entry) -> ())),
                );
                wget_hashmap_set_value_destructor(
                    fingerprints,
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut ocsp_entry) -> ()>,
                        Option::<wget_hashmap_value_destructor>,
                    >(Some(free_ocsp as unsafe extern "C" fn(*mut ocsp_entry) -> ())),
                );
                (*ocsp_db).fingerprints = fingerprints;
                wget_hashmap_set_key_destructor(
                    hosts,
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut ocsp_entry) -> ()>,
                        Option::<wget_hashmap_key_destructor>,
                    >(Some(free_ocsp as unsafe extern "C" fn(*mut ocsp_entry) -> ())),
                );
                wget_hashmap_set_value_destructor(
                    hosts,
                    ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut ocsp_entry) -> ()>,
                        Option::<wget_hashmap_value_destructor>,
                    >(Some(free_ocsp as unsafe extern "C" fn(*mut ocsp_entry) -> ())),
                );
                (*ocsp_db).hosts = hosts;
                wget_thread_mutex_init(&mut (*ocsp_db).mutex);
                return ocsp_db;
            }
        }
    }
    wget_hashmap_free(&mut hosts);
    wget_hashmap_free(&mut fingerprints);
    if !fname.is_null() {
        wget_free.expect("non-null function pointer")(fname as *mut libc::c_void);
        fname = 0 as *const libc::c_char;
    }
    return 0 as *mut wget_ocsp_db;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ocsp_db_set_fname(
    mut ocsp_db: *mut wget_ocsp_db,
    mut fname: *const libc::c_char,
) {
    if !((*ocsp_db).fname).is_null() {
        wget_free
            .expect("non-null function pointer")((*ocsp_db).fname as *mut libc::c_void);
        (*ocsp_db).fname = 0 as *const libc::c_char;
    }
    (*ocsp_db).fname = wget_strdup(fname);
}
