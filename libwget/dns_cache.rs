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
    pub type wget_hashmap_st;
    pub type wget_thread_mutex_st;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn wget_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
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
    fn wget_hashmap_free(h: *mut *mut wget_hashmap);
    fn wget_hashmap_get(
        h: *const wget_hashmap,
        key: *const libc::c_void,
        value: *mut *mut libc::c_void,
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
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __socklen_t = libc::c_uint;
pub type uint16_t = __uint16_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
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
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_dns_cache_st {
    pub cache: *mut wget_hashmap,
    pub mutex: wget_thread_mutex,
}
pub type wget_dns_cache = wget_dns_cache_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cache_entry {
    pub host: *const libc::c_char,
    pub addrinfo: *mut addrinfo,
    pub port: uint16_t,
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
unsafe extern "C" fn hash_dns(mut entry: *const cache_entry) -> libc::c_uint {
    let mut hash: libc::c_uint = (*entry).port as libc::c_uint;
    let mut p: *const libc::c_uchar = (*entry).host as *mut libc::c_uchar;
    while *p != 0 {
        let fresh0 = p;
        p = p.offset(1);
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*fresh0 as libc::c_uint);
    }
    return hash;
}
unsafe extern "C" fn compare_dns(
    mut a1: *const cache_entry,
    mut a2: *const cache_entry,
) -> libc::c_int {
    if ((*a1).port as libc::c_int) < (*a2).port as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*a1).port as libc::c_int > (*a2).port as libc::c_int {
        return 1 as libc::c_int;
    }
    return wget_strcasecmp((*a1).host, (*a2).host);
}
unsafe extern "C" fn free_dns(mut entry: *mut cache_entry) {
    freeaddrinfo((*entry).addrinfo);
    if !entry.is_null() {
        wget_free.expect("non-null function pointer")(entry as *mut libc::c_void);
        entry = 0 as *mut cache_entry;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_cache_init(
    mut cache: *mut *mut wget_dns_cache,
) -> libc::c_int {
    let mut _cache: *mut wget_dns_cache = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_dns_cache>() as libc::c_ulong,
    ) as *mut wget_dns_cache;
    if _cache.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    if wget_thread_mutex_init(&mut (*_cache).mutex) != 0 {
        if !_cache.is_null() {
            wget_free.expect("non-null function pointer")(_cache as *mut libc::c_void);
            _cache = 0 as *mut wget_dns_cache;
        }
        return WGET_E_INVALID as libc::c_int;
    }
    (*_cache)
        .cache = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cache_entry) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_dns as unsafe extern "C" fn(*const cache_entry) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cache_entry,
                    *const cache_entry,
                ) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_dns
                    as unsafe extern "C" fn(
                        *const cache_entry,
                        *const cache_entry,
                    ) -> libc::c_int,
            ),
        ),
    );
    if ((*_cache).cache).is_null() {
        wget_dns_cache_free(&mut _cache);
        return WGET_E_MEMORY as libc::c_int;
    }
    wget_hashmap_set_key_destructor(
        (*_cache).cache,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cache_entry) -> ()>,
            Option::<wget_hashmap_key_destructor>,
        >(Some(free_dns as unsafe extern "C" fn(*mut cache_entry) -> ())),
    );
    wget_hashmap_set_value_destructor(
        (*_cache).cache,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cache_entry) -> ()>,
            Option::<wget_hashmap_value_destructor>,
        >(Some(free_dns as unsafe extern "C" fn(*mut cache_entry) -> ())),
    );
    *cache = _cache;
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_cache_free(mut cache: *mut *mut wget_dns_cache) {
    if !cache.is_null() && !(*cache).is_null() {
        wget_thread_mutex_lock((**cache).mutex);
        wget_hashmap_free(&mut (**cache).cache);
        wget_thread_mutex_unlock((**cache).mutex);
        wget_thread_mutex_destroy(&mut (**cache).mutex);
        if !(*cache).is_null() {
            wget_free.expect("non-null function pointer")(*cache as *mut libc::c_void);
            *cache = 0 as *mut wget_dns_cache;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_cache_get(
    mut cache: *mut wget_dns_cache,
    mut host: *const libc::c_char,
    mut port: uint16_t,
) -> *mut addrinfo {
    if !cache.is_null() {
        let mut entryp: *mut cache_entry = 0 as *mut cache_entry;
        let mut entry: cache_entry = {
            let mut init = cache_entry {
                host: host,
                addrinfo: 0 as *mut addrinfo,
                port: port,
            };
            init
        };
        wget_thread_mutex_lock((*cache).mutex);
        if wget_hashmap_get(
            (*cache).cache,
            &mut entry as *mut cache_entry as *const libc::c_void,
            &mut entryp as *mut *mut cache_entry as *mut *mut libc::c_void,
        ) == 0
        {
            entryp = 0 as *mut cache_entry;
        }
        wget_thread_mutex_unlock((*cache).mutex);
        if !entryp.is_null() {
            if wget_ip_is_family((*entryp).host, 2 as libc::c_int) {
                wget_debug_printf(
                    b"Found dns cache entry [%s]:%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*entryp).host,
                    (*entryp).port as libc::c_int,
                );
            } else {
                wget_debug_printf(
                    b"Found dns cache entry %s:%d\n\0" as *const u8
                        as *const libc::c_char,
                    (*entryp).host,
                    (*entryp).port as libc::c_int,
                );
            }
            return (*entryp).addrinfo;
        }
    }
    return 0 as *mut addrinfo;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_cache_add(
    mut cache: *mut wget_dns_cache,
    mut host: *const libc::c_char,
    mut port: uint16_t,
    mut addrinfo: *mut *mut addrinfo,
) -> libc::c_int {
    if cache.is_null() || host.is_null() || addrinfo.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    let mut entry: cache_entry = {
        let mut init = cache_entry {
            host: host,
            addrinfo: 0 as *mut addrinfo,
            port: port,
        };
        init
    };
    let mut entryp: *mut cache_entry = 0 as *mut cache_entry;
    wget_thread_mutex_lock((*cache).mutex);
    if wget_hashmap_get(
        (*cache).cache,
        &mut entry as *mut cache_entry as *const libc::c_void,
        &mut entryp as *mut *mut cache_entry as *mut *mut libc::c_void,
    ) != 0
    {
        wget_thread_mutex_unlock((*cache).mutex);
        if *addrinfo != (*entryp).addrinfo {
            freeaddrinfo(*addrinfo);
        }
        *addrinfo = (*entryp).addrinfo;
        return WGET_E_SUCCESS as libc::c_int;
    }
    let mut hostlen: size_t = (strlen(host))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    entryp = wget_malloc(
        (::core::mem::size_of::<cache_entry>() as libc::c_ulong).wrapping_add(hostlen),
    ) as *mut cache_entry;
    if entryp.is_null() {
        wget_thread_mutex_unlock((*cache).mutex);
        return WGET_E_MEMORY as libc::c_int;
    }
    (*entryp).port = port;
    (*entryp).host = entryp.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    memcpy(
        (*entryp).host as *mut libc::c_char as *mut libc::c_void,
        host as *const libc::c_void,
        hostlen,
    );
    (*entryp).addrinfo = *addrinfo;
    wget_hashmap_put(
        (*cache).cache,
        entryp as *const libc::c_void,
        entryp as *const libc::c_void,
    );
    wget_thread_mutex_unlock((*cache).mutex);
    return WGET_E_SUCCESS as libc::c_int;
}
