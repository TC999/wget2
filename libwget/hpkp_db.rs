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
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_thread_mutex_st;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
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
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_find(v: *const wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_browse(
        v: *const wget_vector,
        browse: Option::<wget_vector_browse_fn>,
        ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
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
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn wget_hpkp_new() -> *mut wget_hpkp;
    fn wget_hpkp_free(hpkp: *mut wget_hpkp);
    fn wget_hpkp_pin_add(
        hpkp: *mut wget_hpkp,
        pin_type: *const libc::c_char,
        pin_b64: *const libc::c_char,
    );
    fn wget_hash_fast(
        algorithm: wget_digest_algorithm,
        text: *const libc::c_void,
        textlen: size_t,
        digest: *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_hash_get_len(algorithm: wget_digest_algorithm) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn time(__timer: *mut time_t) -> time_t;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type wget_vector = wget_vector_st;
pub type wget_vector_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
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
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hpkp_db_st {
    pub fname: *mut libc::c_char,
    pub entries: *mut wget_hashmap,
    pub mutex: wget_thread_mutex,
    pub load_time: int64_t,
}
pub type wget_hpkp_db = wget_hpkp_db_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_hpkp_st {
    pub host: *const libc::c_char,
    pub expires: int64_t,
    pub created: int64_t,
    pub maxage: int64_t,
    pub pins: *mut wget_vector,
    #[bitfield(name = "include_subdomains", ty = "bool", bits = "0..=0")]
    pub include_subdomains: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type wget_hpkp = wget_hpkp_st;
pub type wget_hpkp_db_init_fn = unsafe extern "C" fn(
    *mut wget_hpkp_db,
    *const libc::c_char,
) -> *mut wget_hpkp_db;
pub type wget_hpkp_db_deinit_fn = unsafe extern "C" fn(*mut wget_hpkp_db) -> ();
pub type wget_hpkp_db_free_fn = unsafe extern "C" fn(*mut *mut wget_hpkp_db) -> ();
pub type wget_hpkp_db_check_pubkey_fn = unsafe extern "C" fn(
    *mut wget_hpkp_db,
    *const libc::c_char,
    *const libc::c_void,
    size_t,
) -> libc::c_int;
pub type wget_hpkp_db_add_fn = unsafe extern "C" fn(
    *mut wget_hpkp_db,
    *mut *mut wget_hpkp,
) -> ();
pub type wget_hpkp_db_load_fn = unsafe extern "C" fn(*mut wget_hpkp_db) -> libc::c_int;
pub type wget_hpkp_db_save_fn = unsafe extern "C" fn(*mut wget_hpkp_db) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hpkp_db_vtable {
    pub init: Option::<wget_hpkp_db_init_fn>,
    pub deinit: Option::<wget_hpkp_db_deinit_fn>,
    pub free: Option::<wget_hpkp_db_free_fn>,
    pub check_pubkey: Option::<wget_hpkp_db_check_pubkey_fn>,
    pub add: Option::<wget_hpkp_db_add_fn>,
    pub load: Option::<wget_hpkp_db_load_fn>,
    pub save: Option::<wget_hpkp_db_save_fn>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hpkp_pin {
    pub pin_b64: *const libc::c_char,
    pub pin: *const libc::c_void,
    pub hash_type: *const libc::c_char,
    pub pinsize: size_t,
}
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
pub const _ISspace: C2RustUnnamed = 8192;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
static mut plugin_vtable: *const wget_hpkp_db_vtable = 0 as *const wget_hpkp_db_vtable;
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_set_plugin(mut vtable: *const wget_hpkp_db_vtable) {
    plugin_vtable = vtable;
}
unsafe extern "C" fn hash_hpkp(mut hpkp: *const wget_hpkp) -> libc::c_uint {
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    p = (*hpkp).host as *mut libc::c_uchar;
    while *p != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn compare_hpkp(
    mut h1: *const wget_hpkp,
    mut h2: *const wget_hpkp,
) -> libc::c_int {
    return strcmp((*h1).host, (*h2).host);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_deinit(mut hpkp_db: *mut wget_hpkp_db) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).deinit).expect("non-null function pointer")(hpkp_db);
        return;
    }
    if !hpkp_db.is_null() {
        if !((*hpkp_db).fname).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*hpkp_db).fname as *mut libc::c_void);
            (*hpkp_db).fname = 0 as *mut libc::c_char;
        }
        wget_thread_mutex_lock((*hpkp_db).mutex);
        wget_hashmap_free(&mut (*hpkp_db).entries);
        wget_thread_mutex_unlock((*hpkp_db).mutex);
        wget_thread_mutex_destroy(&mut (*hpkp_db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_free(mut hpkp_db: *mut *mut wget_hpkp_db) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).free).expect("non-null function pointer")(hpkp_db);
        return;
    }
    if !hpkp_db.is_null() && !(*hpkp_db).is_null() {
        wget_hpkp_db_deinit(*hpkp_db);
        if !(*hpkp_db).is_null() {
            wget_free.expect("non-null function pointer")(*hpkp_db as *mut libc::c_void);
            *hpkp_db = 0 as *mut wget_hpkp_db;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_check_pubkey(
    mut hpkp_db: *mut wget_hpkp_db,
    mut host: *const libc::c_char,
    mut pubkey: *const libc::c_void,
    mut pubkeysize: size_t,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).check_pubkey)
            .expect("non-null function pointer")(hpkp_db, host, pubkey, pubkeysize);
    }
    let mut hpkp: *mut wget_hpkp = 0 as *mut wget_hpkp;
    let mut subdomain: libc::c_int = 0 as libc::c_int;
    let mut digest: [libc::c_char; 32] = [0; 32];
    let mut digestlen: size_t = wget_hash_get_len(WGET_DIGTYPE_SHA256) as size_t;
    if digestlen > ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Unexpected hash len %zu > %zu\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"wget_hpkp_db_check_pubkey\0"))
                .as_ptr(),
            digestlen,
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
        return -(1 as libc::c_int);
    }
    let mut domain: *const libc::c_char = host;
    while *domain as libc::c_int != 0 && hpkp.is_null() {
        while *domain as libc::c_int == '.' as i32 {
            domain = domain.offset(1);
            domain;
        }
        let mut key: wget_hpkp = {
            let mut init = wget_hpkp_st {
                include_subdomains: [0; 1],
                c2rust_padding: [0; 7],
                host: domain,
                expires: 0,
                created: 0,
                maxage: 0,
                pins: 0 as *mut wget_vector,
            };
            init.set_include_subdomains(false);
            init
        };
        if wget_hashmap_get(
            (*hpkp_db).entries,
            &mut key as *mut wget_hpkp as *const libc::c_void,
            &mut hpkp as *mut *mut wget_hpkp as *mut *mut libc::c_void,
        ) == 0
        {
            subdomain = 1 as libc::c_int;
        }
        domain = strchrnul(domain, '.' as i32);
    }
    if hpkp.is_null() {
        return 0 as libc::c_int;
    }
    if subdomain != 0 && !(*hpkp).include_subdomains() {
        return 0 as libc::c_int;
    }
    if wget_hash_fast(
        WGET_DIGTYPE_SHA256,
        pubkey,
        pubkeysize,
        digest.as_mut_ptr() as *mut libc::c_void,
    ) != 0
    {
        return -(1 as libc::c_int);
    }
    let mut pinkey: wget_hpkp_pin = {
        let mut init = wget_hpkp_pin {
            pin_b64: 0 as *const libc::c_char,
            pin: digest.as_mut_ptr() as *const libc::c_void,
            hash_type: b"sha256\0" as *const u8 as *const libc::c_char,
            pinsize: digestlen,
        };
        init
    };
    if wget_vector_find(
        (*hpkp).pins,
        &mut pinkey as *mut wget_hpkp_pin as *const libc::c_void,
    ) != -(1 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    return -(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_add(
    mut hpkp_db: *mut wget_hpkp_db,
    mut _hpkp: *mut *mut wget_hpkp,
) {
    if !plugin_vtable.is_null() {
        ((*plugin_vtable).add).expect("non-null function pointer")(hpkp_db, _hpkp);
        *_hpkp = 0 as *mut wget_hpkp;
        return;
    }
    if _hpkp.is_null() || (*_hpkp).is_null() {
        return;
    }
    let mut hpkp: *mut wget_hpkp = *_hpkp;
    wget_thread_mutex_lock((*hpkp_db).mutex);
    if (*hpkp).maxage == 0 as libc::c_int as int64_t
        || wget_vector_size((*hpkp).pins) == 0 as libc::c_int
    {
        if wget_hashmap_remove((*hpkp_db).entries, hpkp as *const libc::c_void) != 0 {
            wget_debug_printf(
                b"removed HPKP %s\n\0" as *const u8 as *const libc::c_char,
                (*hpkp).host,
            );
        }
        wget_hpkp_free(hpkp);
    } else {
        let mut old: *mut wget_hpkp = 0 as *mut wget_hpkp;
        if wget_hashmap_get(
            (*hpkp_db).entries,
            hpkp as *const libc::c_void,
            &mut old as *mut *mut wget_hpkp as *mut *mut libc::c_void,
        ) != 0
        {
            (*old).created = (*hpkp).created;
            (*old).maxage = (*hpkp).maxage;
            (*old).expires = (*hpkp).expires;
            (*old).set_include_subdomains((*hpkp).include_subdomains());
            wget_vector_free(&mut (*old).pins);
            (*old).pins = (*hpkp).pins;
            (*hpkp).pins = 0 as *mut wget_vector;
            wget_debug_printf(
                b"update HPKP %s (maxage=%lld, includeSubDomains=%d)\n\0" as *const u8
                    as *const libc::c_char,
                (*old).host,
                (*old).maxage as libc::c_longlong,
                (*old).include_subdomains() as libc::c_int,
            );
            wget_hpkp_free(hpkp);
        } else {
            wget_hashmap_put(
                (*hpkp_db).entries,
                hpkp as *const libc::c_void,
                hpkp as *const libc::c_void,
            );
        }
    }
    wget_thread_mutex_unlock((*hpkp_db).mutex);
    *_hpkp = 0 as *mut wget_hpkp;
}
unsafe extern "C" fn hpkp_db_load(
    mut hpkp_db: *mut wget_hpkp_db,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut created: int64_t = 0;
    let mut max_age: int64_t = 0;
    let mut _created: libc::c_longlong = 0;
    let mut _max_age: libc::c_longlong = 0;
    let mut include_subdomains: libc::c_int = 0;
    let mut hpkp: *mut wget_hpkp = 0 as *mut wget_hpkp;
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
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut buflen: ssize_t = 0;
    let mut hash_type: [libc::c_char; 32] = [0; 32];
    let mut host: [libc::c_char; 256] = [0; 256];
    let mut pin_b64: [libc::c_char; 256] = [0; 256];
    let mut now: int64_t = time(0 as *mut time_t);
    if fstat(fileno(fp), &mut st) == 0 as libc::c_int {
        if st.st_mtim.tv_sec != (*hpkp_db).load_time {
            (*hpkp_db).load_time = st.st_mtim.tv_sec;
        } else {
            return 0 as libc::c_int
        }
    }
    loop {
        buflen = wget_getline(&mut buf, &mut bufsize, fp);
        if !(buflen >= 0 as libc::c_int as ssize_t) {
            break;
        }
        let mut linep: *mut libc::c_char = buf;
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
        if *linep as libc::c_int != '*' as i32 {
            wget_hpkp_db_add(hpkp_db, &mut hpkp);
            if sscanf(
                linep,
                b"%255s %d %lld %lld\0" as *const u8 as *const libc::c_char,
                host.as_mut_ptr(),
                &mut include_subdomains as *mut libc::c_int,
                &mut _created as *mut libc::c_longlong,
                &mut _max_age as *mut libc::c_longlong,
            ) == 4 as libc::c_int
            {
                created = _created as int64_t;
                max_age = _max_age as int64_t;
                if created < 0 as libc::c_int as int64_t
                    || max_age < 0 as libc::c_int as int64_t
                    || created
                        >= 9223372036854775807 as libc::c_long
                            / 2 as libc::c_int as libc::c_long
                    || max_age
                        >= 9223372036854775807 as libc::c_long
                            / 2 as libc::c_int as libc::c_long
                {
                    max_age = 0 as libc::c_int as int64_t;
                }
                let mut expires: int64_t = created + max_age;
                if max_age != 0 && expires >= now {
                    hpkp = wget_hpkp_new();
                    if !hpkp.is_null() {
                        (*hpkp).host = wget_strdup(host.as_mut_ptr());
                        if ((*hpkp).host).is_null() {
                            if !hpkp.is_null() {
                                wget_free
                                    .expect(
                                        "non-null function pointer",
                                    )(hpkp as *mut libc::c_void);
                                hpkp = 0 as *mut wget_hpkp;
                            }
                        } else {
                            (*hpkp).maxage = max_age;
                            (*hpkp).created = created;
                            (*hpkp).expires = expires;
                            (*hpkp)
                                .set_include_subdomains(
                                    include_subdomains != 0 as libc::c_int,
                                );
                        }
                    }
                } else {
                    wget_debug_printf(
                        b"HPKP: entry '%s' is expired\n\0" as *const u8
                            as *const libc::c_char,
                        host.as_mut_ptr(),
                    );
                }
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"HPKP: could not parse host line '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    buf,
                );
            }
        } else if !hpkp.is_null() {
            if sscanf(
                linep,
                b"*%31s %255s\0" as *const u8 as *const libc::c_char,
                hash_type.as_mut_ptr(),
                pin_b64.as_mut_ptr(),
            ) == 2 as libc::c_int
            {
                wget_hpkp_pin_add(hpkp, hash_type.as_mut_ptr(), pin_b64.as_mut_ptr());
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"HPKP: could not parse pin line '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    buf,
                );
            }
        } else {
            wget_debug_printf(
                b"HPKP: skipping PIN entry: '%s'\n\0" as *const u8
                    as *const libc::c_char,
                buf,
            );
        }
    }
    wget_hpkp_db_add(hpkp_db, &mut hpkp);
    if !buf.is_null() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    if ferror(fp) != 0 {
        (*hpkp_db).load_time = 0 as libc::c_int as int64_t;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_load(
    mut hpkp_db: *mut wget_hpkp_db,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).load).expect("non-null function pointer")(hpkp_db);
    }
    if hpkp_db.is_null() {
        return 0 as libc::c_int;
    }
    if ((*hpkp_db).fname).is_null() || *(*hpkp_db).fname == 0 {
        return 0 as libc::c_int;
    }
    if wget_update_file(
        (*hpkp_db).fname,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_hpkp_db, *mut FILE) -> libc::c_int>,
            Option::<wget_update_load_fn>,
        >(
            Some(
                hpkp_db_load
                    as unsafe extern "C" fn(*mut wget_hpkp_db, *mut FILE) -> libc::c_int,
            ),
        ),
        None,
        hpkp_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read HPKP data\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    } else {
        wget_debug_printf(
            b"Fetched HPKP data from '%s'\n\0" as *const u8 as *const libc::c_char,
            (*hpkp_db).fname,
        );
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn hpkp_save_pin(
    mut _fp: *mut libc::c_void,
    mut _pin: *mut libc::c_void,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut pin: *mut wget_hpkp_pin = _pin as *mut wget_hpkp_pin;
    wget_fprintf(
        fp,
        b"*%s %s\n\0" as *const u8 as *const libc::c_char,
        (*pin).hash_type,
        (*pin).pin_b64,
    );
    if ferror(fp) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hpkp_save(
    mut _fp: *mut libc::c_void,
    mut _hpkp: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut fp: *mut FILE = _fp as *mut FILE;
    let mut hpkp: *const wget_hpkp = _hpkp as *const wget_hpkp;
    if wget_vector_size((*hpkp).pins) == 0 as libc::c_int {
        wget_debug_printf(
            b"HPKP: drop '%s', no PIN entries\n\0" as *const u8 as *const libc::c_char,
            (*hpkp).host,
        );
    } else if (*hpkp).expires < time(0 as *mut time_t) {
        wget_debug_printf(
            b"HPKP: drop '%s', expired\n\0" as *const u8 as *const libc::c_char,
            (*hpkp).host,
        );
    } else {
        wget_fprintf(
            fp,
            b"%s %d %lld %lld\n\0" as *const u8 as *const libc::c_char,
            (*hpkp).host,
            (*hpkp).include_subdomains() as libc::c_int,
            (*hpkp).created as libc::c_longlong,
            (*hpkp).maxage as libc::c_longlong,
        );
        if ferror(fp) != 0 {
            return -(1 as libc::c_int);
        }
        return wget_vector_browse(
            (*hpkp).pins,
            Some(
                hpkp_save_pin
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            fp as *mut libc::c_void,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hpkp_db_save(
    mut hpkp_db: *mut wget_hpkp_db,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut entries: *mut wget_hashmap = (*hpkp_db).entries;
    if wget_hashmap_size(entries) > 0 as libc::c_int {
        fputs(b"# HPKP 1.0 file\n\0" as *const u8 as *const libc::c_char, fp);
        fputs(
            b"#Generated by libwget 2.2.0. Edit at your own risk.\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        fputs(
            b"#<hostname> <incl. subdomains> <created> <max-age>\n\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        if ferror(fp) != 0 {
            return -(1 as libc::c_int);
        }
        return wget_hashmap_browse(
            entries,
            Some(
                hpkp_save
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            fp as *mut libc::c_void,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_save(
    mut hpkp_db: *mut wget_hpkp_db,
) -> libc::c_int {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).save).expect("non-null function pointer")(hpkp_db);
    }
    if hpkp_db.is_null() {
        return -(1 as libc::c_int);
    }
    let mut size: libc::c_int = 0;
    if ((*hpkp_db).fname).is_null() || *(*hpkp_db).fname == 0 {
        return -(1 as libc::c_int);
    }
    if wget_update_file(
        (*hpkp_db).fname,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_hpkp_db, *mut FILE) -> libc::c_int>,
            Option::<wget_update_load_fn>,
        >(
            Some(
                hpkp_db_load
                    as unsafe extern "C" fn(*mut wget_hpkp_db, *mut FILE) -> libc::c_int,
            ),
        ),
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_hpkp_db, *mut FILE) -> libc::c_int>,
            Option::<wget_update_load_fn>,
        >(
            Some(
                hpkp_db_save
                    as unsafe extern "C" fn(*mut wget_hpkp_db, *mut FILE) -> libc::c_int,
            ),
        ),
        hpkp_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write HPKP file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*hpkp_db).fname,
        );
        return -(1 as libc::c_int);
    }
    size = wget_hashmap_size((*hpkp_db).entries);
    if size != 0 {
        wget_debug_printf(
            b"Saved %d HPKP entr%s into '%s'\n\0" as *const u8 as *const libc::c_char,
            size,
            if size != 1 as libc::c_int {
                b"ies\0" as *const u8 as *const libc::c_char
            } else {
                b"y\0" as *const u8 as *const libc::c_char
            },
            (*hpkp_db).fname,
        );
    } else {
        wget_debug_printf(
            b"No HPKP entries to save. Table is empty.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_init(
    mut hpkp_db: *mut wget_hpkp_db,
    mut fname: *const libc::c_char,
) -> *mut wget_hpkp_db {
    if !plugin_vtable.is_null() {
        return ((*plugin_vtable).init)
            .expect("non-null function pointer")(hpkp_db, fname);
    }
    if hpkp_db.is_null() {
        hpkp_db = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_hpkp_db_st>() as libc::c_ulong,
        ) as *mut wget_hpkp_db;
        if hpkp_db.is_null() {
            return 0 as *mut wget_hpkp_db;
        }
    } else {
        memset(
            hpkp_db as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_hpkp_db>() as libc::c_ulong,
        );
    }
    if !fname.is_null() {
        (*hpkp_db).fname = wget_strdup(fname);
    }
    (*hpkp_db)
        .entries = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const wget_hpkp) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_hpkp as unsafe extern "C" fn(*const wget_hpkp) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const wget_hpkp, *const wget_hpkp) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_hpkp
                    as unsafe extern "C" fn(
                        *const wget_hpkp,
                        *const wget_hpkp,
                    ) -> libc::c_int,
            ),
        ),
    );
    wget_hashmap_set_key_destructor(
        (*hpkp_db).entries,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_hpkp) -> ()>,
            Option::<wget_hashmap_key_destructor>,
        >(Some(wget_hpkp_free as unsafe extern "C" fn(*mut wget_hpkp) -> ())),
    );
    wget_thread_mutex_init(&mut (*hpkp_db).mutex);
    return hpkp_db;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_db_set_fname(
    mut hpkp_db: *mut wget_hpkp_db,
    mut fname: *const libc::c_char,
) {
    if !((*hpkp_db).fname).is_null() {
        wget_free
            .expect("non-null function pointer")((*hpkp_db).fname as *mut libc::c_void);
        (*hpkp_db).fname = 0 as *mut libc::c_char;
    }
    (*hpkp_db).fname = wget_strdup(fname);
}
