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
    pub type wget_thread_mutex_st;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn time(__timer: *mut time_t) -> time_t;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
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
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_printf_append(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_find(v: *const wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_insert_sorted(
        v: *mut wget_vector,
        elem: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_replace(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_clear_nofree(v: *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_vector_sort(v: *mut wget_vector);
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn wget_cookie_init(cookie: *mut wget_cookie) -> *mut wget_cookie;
    fn wget_cookie_deinit(cookie: *mut wget_cookie);
    fn wget_cookie_free(cookie: *mut *mut wget_cookie);
    fn wget_cookie_normalize(
        iri: *const wget_iri,
        cookie: *mut wget_cookie,
    ) -> libc::c_int;
    fn cookie_domain_match(
        domain: *const libc::c_char,
        host: *const libc::c_char,
    ) -> bool;
    fn cookie_path_match(
        cookie_path: *const libc::c_char,
        request_path: *const libc::c_char,
    ) -> bool;
    fn cookie_free(cookie: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed_0 = -12;
pub const WGET_E_IO: C2RustUnnamed_0 = -11;
pub const WGET_E_OPEN: C2RustUnnamed_0 = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed_0 = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed_0 = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed_0 = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed_0 = -6;
pub const WGET_E_CONNECT: C2RustUnnamed_0 = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed_0 = -4;
pub const WGET_E_INVALID: C2RustUnnamed_0 = -3;
pub const WGET_E_MEMORY: C2RustUnnamed_0 = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed_0 = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed_0 = 0;
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
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_iri_scheme = libc::c_uint;
pub const WGET_IRI_SCHEME_HTTPS: wget_iri_scheme = 1;
pub const WGET_IRI_SCHEME_HTTP: wget_iri_scheme = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_iri_st {
    pub uri: *const libc::c_char,
    pub safe_uri: *const libc::c_char,
    pub display: *const libc::c_char,
    pub userinfo: *const libc::c_char,
    pub password: *const libc::c_char,
    pub host: *const libc::c_char,
    pub path: *const libc::c_char,
    pub query: *const libc::c_char,
    pub fragment: *const libc::c_char,
    pub connection_part: *const libc::c_char,
    pub dirlen: size_t,
    pub msize: size_t,
    pub port: uint16_t,
    pub scheme: wget_iri_scheme,
    #[bitfield(name = "port_given", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "host_allocated", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "path_allocated", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "query_allocated", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "fragment_allocated", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "is_ip_address", ty = "bool", bits = "5..=5")]
    pub port_given_host_allocated_path_allocated_query_allocated_fragment_allocated_is_ip_address: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type wget_iri = wget_iri_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_cookie_db_st {
    pub cookies: *mut wget_vector,
    pub mutex: wget_thread_mutex,
    pub age: libc::c_uint,
    #[bitfield(name = "keep_session_cookies", ty = "bool", bits = "0..=0")]
    pub keep_session_cookies: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type wget_cookie_db = wget_cookie_db_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_cookie_st {
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
    pub domain: *const libc::c_char,
    pub path: *const libc::c_char,
    pub expires: int64_t,
    pub maxage: int64_t,
    pub last_access: int64_t,
    pub creation: int64_t,
    pub sort_age: libc::c_uint,
    #[bitfield(name = "domain_dot", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "normalized", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "persistent", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "host_only", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "secure_only", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "http_only", ty = "bool", bits = "5..=5")]
    pub domain_dot_normalized_persistent_host_only_secure_only_http_only: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type wget_cookie = wget_cookie_st;
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
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_db_load_psl(
    mut cookie_db: *mut wget_cookie_db,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
unsafe extern "C" fn compare_cookie(
    mut c1: *const wget_cookie,
    mut c2: *const wget_cookie,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = wget_strcmp((*c1).domain, (*c2).domain);
    if n == 0 {
        n = wget_strcmp((*c1).name, (*c2).name);
        if n == 0 {
            n = wget_strcmp((*c1).path, (*c2).path);
        }
    }
    return n;
}
unsafe extern "C" fn compare_cookie2(
    mut c1: *const wget_cookie,
    mut c2: *const wget_cookie,
) -> libc::c_int {
    let mut len1: size_t = strlen((*c1).path);
    let mut len2: size_t = strlen((*c2).path);
    if len1 < len2 {
        return 1 as libc::c_int;
    }
    if len1 > len2 {
        return -(1 as libc::c_int);
    }
    if (*c1).sort_age < (*c2).sort_age {
        return -(1 as libc::c_int);
    }
    if (*c1).sort_age > (*c2).sort_age {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_check_psl(
    mut cookie_db: *const wget_cookie_db,
    mut cookie: *const wget_cookie,
) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_store_cookie(
    mut cookie_db: *mut wget_cookie_db,
    mut cookie: *mut wget_cookie,
) -> libc::c_int {
    let mut old: *mut wget_cookie = 0 as *mut wget_cookie;
    let mut pos: libc::c_int = 0;
    if cookie.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if cookie_db.is_null() {
        wget_cookie_free(&mut cookie);
        return WGET_E_INVALID as libc::c_int;
    }
    wget_debug_printf(
        b"got cookie %s=%s\n\0" as *const u8 as *const libc::c_char,
        (*cookie).name,
        (*cookie).value,
    );
    if !(*cookie).normalized() {
        wget_debug_printf(
            b"cookie '%s' dropped, it wasn't normalized\n\0" as *const u8
                as *const libc::c_char,
            (*cookie).name,
        );
        wget_cookie_free(&mut cookie);
        return WGET_E_INVALID as libc::c_int;
    }
    if wget_cookie_check_psl(cookie_db, cookie) != 0 as libc::c_int {
        wget_debug_printf(
            b"cookie '%s' dropped, domain '%s' is a public suffix\n\0" as *const u8
                as *const libc::c_char,
            (*cookie).name,
            (*cookie).domain,
        );
        wget_cookie_free(&mut cookie);
        return WGET_E_INVALID as libc::c_int;
    }
    wget_thread_mutex_lock((*cookie_db).mutex);
    pos = wget_vector_find((*cookie_db).cookies, cookie as *const libc::c_void);
    old = wget_vector_get((*cookie_db).cookies, pos) as *mut wget_cookie;
    if !old.is_null() {
        wget_debug_printf(
            b"replace old cookie %s=%s\n\0" as *const u8 as *const libc::c_char,
            (*cookie).name,
            (*cookie).value,
        );
        (*cookie).creation = (*old).creation;
        (*cookie).sort_age = (*old).sort_age;
        wget_vector_replace((*cookie_db).cookies, cookie as *const libc::c_void, pos);
    } else {
        wget_debug_printf(
            b"store new cookie %s=%s\n\0" as *const u8 as *const libc::c_char,
            (*cookie).name,
            (*cookie).value,
        );
        (*cookie_db).age = ((*cookie_db).age).wrapping_add(1);
        (*cookie).sort_age = (*cookie_db).age;
        wget_vector_insert_sorted((*cookie_db).cookies, cookie as *const libc::c_void);
    }
    wget_thread_mutex_unlock((*cookie_db).mutex);
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_store_cookies(
    mut cookie_db: *mut wget_cookie_db,
    mut cookies: *mut wget_vector,
) {
    if !cookie_db.is_null() {
        let mut it: libc::c_int = 0;
        it = 0 as libc::c_int;
        while it < wget_vector_size(cookies) {
            let mut cookie: *mut wget_cookie = wget_vector_get(cookies, it)
                as *mut wget_cookie;
            wget_cookie_store_cookie(cookie_db, cookie);
            it += 1;
            it;
        }
        wget_vector_clear_nofree(cookies);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_create_request_header(
    mut cookie_db: *mut wget_cookie_db,
    mut iri: *const wget_iri,
) -> *mut libc::c_char {
    let mut it: libc::c_int = 0;
    let mut init: libc::c_int = 0 as libc::c_int;
    let mut now: int64_t = time(0 as *mut time_t);
    let mut cookies: *mut wget_vector = 0 as *mut wget_vector;
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    if cookie_db.is_null() || iri.is_null() {
        return 0 as *mut libc::c_char;
    }
    wget_debug_printf(
        b"cookie_create_request_header for host=%s path=%s\n\0" as *const u8
            as *const libc::c_char,
        (*iri).host,
        (*iri).path,
    );
    wget_thread_mutex_lock((*cookie_db).mutex);
    it = 0 as libc::c_int;
    while it < wget_vector_size((*cookie_db).cookies) {
        let mut cookie: *mut wget_cookie = wget_vector_get((*cookie_db).cookies, it)
            as *mut wget_cookie;
        if !cookie.is_null() {
            if (*cookie).host_only() as libc::c_int != 0
                && strcmp((*cookie).domain, (*iri).host) != 0
            {
                wget_debug_printf(
                    b"cookie host match failed (%s,%s)\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).domain,
                    (*iri).host,
                );
            } else if !(*cookie).host_only()
                && !cookie_domain_match((*cookie).domain, (*iri).host)
            {
                wget_debug_printf(
                    b"cookie domain match failed (%s,%s)\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).domain,
                    (*iri).host,
                );
            } else if (*cookie).expires != 0 && (*cookie).expires <= now {
                wget_debug_printf(
                    b"cookie expired (%lld <= %lld)\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).expires as libc::c_longlong,
                    now as libc::c_longlong,
                );
            } else if (*cookie).secure_only() as libc::c_int != 0
                && (*iri).scheme as libc::c_uint
                    != WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
            {
                wget_debug_printf(
                    b"cookie ignored, not secure\n\0" as *const u8 as *const libc::c_char,
                );
            } else if !cookie_path_match((*cookie).path, (*iri).path) {
                wget_debug_printf(
                    b"cookie path doesn't match (%s, %s)\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).path,
                    (*iri).path,
                );
            } else {
                wget_debug_printf(
                    b"found %s=%s\n\0" as *const u8 as *const libc::c_char,
                    (*cookie).name,
                    (*cookie).value,
                );
                if cookies.is_null() {
                    cookies = wget_vector_create(
                        16 as libc::c_int,
                        ::core::mem::transmute::<
                            Option::<
                                unsafe extern "C" fn(
                                    *const wget_cookie,
                                    *const wget_cookie,
                                ) -> libc::c_int,
                            >,
                            Option::<wget_vector_compare_fn>,
                        >(
                            Some(
                                compare_cookie2
                                    as unsafe extern "C" fn(
                                        *const wget_cookie,
                                        *const wget_cookie,
                                    ) -> libc::c_int,
                            ),
                        ),
                    );
                }
                wget_vector_add(cookies, cookie as *const libc::c_void);
            }
        }
        it += 1;
        it;
    }
    wget_vector_sort(cookies);
    it = 0 as libc::c_int;
    while it < wget_vector_size(cookies) {
        let mut cookie_0: *mut wget_cookie = wget_vector_get(cookies, it)
            as *mut wget_cookie;
        if !cookie_0.is_null() {
            if init == 0 {
                wget_buffer_init(
                    &mut buf,
                    0 as *mut libc::c_char,
                    128 as libc::c_int as size_t,
                );
                init = 1 as libc::c_int;
            }
            if buf.length != 0 {
                wget_buffer_printf_append(
                    &mut buf as *mut wget_buffer,
                    b"; %s=%s\0" as *const u8 as *const libc::c_char,
                    (*cookie_0).name,
                    (*cookie_0).value,
                );
            } else {
                wget_buffer_printf_append(
                    &mut buf as *mut wget_buffer,
                    b"%s=%s\0" as *const u8 as *const libc::c_char,
                    (*cookie_0).name,
                    (*cookie_0).value,
                );
            }
        }
        it += 1;
        it;
    }
    wget_vector_clear_nofree(cookies);
    wget_vector_free(&mut cookies);
    wget_thread_mutex_unlock((*cookie_db).mutex);
    return if init != 0 { buf.data } else { 0 as *mut libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_db_init(
    mut cookie_db: *mut wget_cookie_db,
) -> *mut wget_cookie_db {
    if cookie_db.is_null() {
        cookie_db = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_cookie_db>() as libc::c_ulong,
        ) as *mut wget_cookie_db;
        if cookie_db.is_null() {
            return 0 as *mut wget_cookie_db;
        }
    } else {
        memset(
            cookie_db as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_cookie_db>() as libc::c_ulong,
        );
    }
    (*cookie_db)
        .cookies = wget_vector_create(
        32 as libc::c_int,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const wget_cookie,
                    *const wget_cookie,
                ) -> libc::c_int,
            >,
            Option::<wget_vector_compare_fn>,
        >(
            Some(
                compare_cookie
                    as unsafe extern "C" fn(
                        *const wget_cookie,
                        *const wget_cookie,
                    ) -> libc::c_int,
            ),
        ),
    );
    wget_vector_set_destructor(
        (*cookie_db).cookies,
        Some(cookie_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    wget_thread_mutex_init(&mut (*cookie_db).mutex);
    return cookie_db;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_db_deinit(mut cookie_db: *mut wget_cookie_db) {
    if !cookie_db.is_null() {
        wget_thread_mutex_lock((*cookie_db).mutex);
        wget_vector_free(&mut (*cookie_db).cookies);
        wget_thread_mutex_unlock((*cookie_db).mutex);
        wget_thread_mutex_destroy(&mut (*cookie_db).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_db_free(mut cookie_db: *mut *mut wget_cookie_db) {
    if !cookie_db.is_null() {
        wget_cookie_db_deinit(*cookie_db);
        if !(*cookie_db).is_null() {
            wget_free
                .expect("non-null function pointer")(*cookie_db as *mut libc::c_void);
            *cookie_db = 0 as *mut wget_cookie_db;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_set_keep_session_cookies(
    mut cookie_db: *mut wget_cookie_db,
    mut keep: bool,
) {
    if !cookie_db.is_null() {
        (*cookie_db).set_keep_session_cookies(keep);
    }
}
unsafe extern "C" fn cookie_db_load(
    mut cookie_db: *mut wget_cookie_db,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut cookie: wget_cookie = wget_cookie_st {
        name: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
        domain: 0 as *const libc::c_char,
        path: 0 as *const libc::c_char,
        expires: 0,
        maxage: 0,
        last_access: 0,
        creation: 0,
        sort_age: 0,
        domain_dot_normalized_persistent_host_only_secure_only_http_only: [0; 1],
        c2rust_padding: [0; 3],
    };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut buflen: ssize_t = 0;
    let mut now: int64_t = time(0 as *mut time_t);
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
        wget_cookie_init(&mut cookie);
        if *linep as libc::c_int == '#' as i32 {
            if strncmp(
                linep,
                b"#HttpOnly_\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_ulong,
            ) != 0
            {
                continue;
            }
            linep = linep.offset(10 as libc::c_int as isize);
            cookie.set_http_only(1 as libc::c_int != 0);
        } else {
            cookie.set_http_only(0 as libc::c_int != 0);
        }
        while buflen > 0 as libc::c_int as ssize_t
            && (*buf.offset((buflen - 1 as libc::c_int as ssize_t) as isize)
                as libc::c_int == '\n' as i32
                || *buf.offset((buflen - 1 as libc::c_int as ssize_t) as isize)
                    as libc::c_int == '\r' as i32)
        {
            buflen -= 1;
            *buf.offset(buflen as isize) = 0 as libc::c_int as libc::c_char;
        }
        p = linep;
        while *linep as libc::c_int != 0 && *linep as libc::c_int != '\t' as i32 {
            linep = linep.offset(1);
            linep;
        }
        if *p as libc::c_int == '.' as i32 {
            p = p.offset(1);
            p;
            cookie.set_domain_dot(1 as libc::c_int != 0);
        }
        cookie
            .domain = wget_strmemdup(
            p as *const libc::c_void,
            linep.offset_from(p) as libc::c_long as size_t,
        );
        p = if *linep as libc::c_int != 0 {
            linep = linep.offset(1);
            linep
        } else {
            linep
        };
        while *linep as libc::c_int != 0 && *linep as libc::c_int != '\t' as i32 {
            linep = linep.offset(1);
            linep;
        }
        cookie
            .set_host_only(
                wget_strncasecmp_ascii(
                    p,
                    b"TRUE\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                ) != 0,
            );
        p = if *linep as libc::c_int != 0 {
            linep = linep.offset(1);
            linep
        } else {
            linep
        };
        while *linep as libc::c_int != 0 && *linep as libc::c_int != '\t' as i32 {
            linep = linep.offset(1);
            linep;
        }
        if p != linep {
            cookie
                .path = wget_strmemdup(
                p as *const libc::c_void,
                linep.offset_from(p) as libc::c_long as size_t,
            );
        } else {
            cookie
                .path = wget_strmemdup(
                b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        p = if *linep as libc::c_int != 0 {
            linep = linep.offset(1);
            linep
        } else {
            linep
        };
        while *linep as libc::c_int != 0 && *linep as libc::c_int != '\t' as i32 {
            linep = linep.offset(1);
            linep;
        }
        cookie
            .set_secure_only(
                wget_strncasecmp_ascii(
                    p,
                    b"TRUE\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                ) == 0,
            );
        p = if *linep as libc::c_int != 0 {
            linep = linep.offset(1);
            linep
        } else {
            linep
        };
        while *linep as libc::c_int != 0 && *linep as libc::c_int != '\t' as i32 {
            linep = linep.offset(1);
            linep;
        }
        cookie.expires = atoll(p) as int64_t;
        if cookie.expires != 0 && cookie.expires <= now {
            wget_cookie_deinit(&mut cookie);
        } else if cookie.expires == 0 && !(*cookie_db).keep_session_cookies() {
            wget_cookie_deinit(&mut cookie);
        } else {
            p = if *linep as libc::c_int != 0 {
                linep = linep.offset(1);
                linep
            } else {
                linep
            };
            while *linep as libc::c_int != 0 && *linep as libc::c_int != '\t' as i32 {
                linep = linep.offset(1);
                linep;
            }
            if linep == p {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Incomplete cookie entry: %s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    buf,
                );
                wget_cookie_deinit(&mut cookie);
            } else {
                cookie
                    .name = wget_strmemdup(
                    p as *const libc::c_void,
                    linep.offset_from(p) as libc::c_long as size_t,
                );
                p = if *linep as libc::c_int != 0 {
                    linep = linep.offset(1);
                    linep
                } else {
                    linep
                };
                while *linep != 0 {
                    linep = linep.offset(1);
                    linep;
                }
                cookie
                    .value = wget_strmemdup(
                    p as *const libc::c_void,
                    linep.offset_from(p) as libc::c_long as size_t,
                );
                if wget_cookie_normalize(0 as *const wget_iri, &mut cookie)
                    == 0 as libc::c_int
                    && wget_cookie_check_psl(cookie_db, &mut cookie) == 0 as libc::c_int
                {
                    wget_cookie_store_cookie(
                        cookie_db,
                        wget_memdup(
                            &mut cookie as *mut wget_cookie as *const libc::c_void,
                            ::core::mem::size_of::<wget_cookie>() as libc::c_ulong,
                        ) as *mut wget_cookie,
                    );
                } else {
                    wget_cookie_deinit(&mut cookie);
                }
            }
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
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_db_load(
    mut cookie_db: *mut wget_cookie_db,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    if cookie_db.is_null() || fname.is_null() || *fname == 0 {
        return 0 as libc::c_int;
    }
    if wget_update_file(
        fname,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut wget_cookie_db, *mut FILE) -> libc::c_int,
            >,
            Option::<wget_update_load_fn>,
        >(
            Some(
                cookie_db_load
                    as unsafe extern "C" fn(
                        *mut wget_cookie_db,
                        *mut FILE,
                    ) -> libc::c_int,
            ),
        ),
        None,
        cookie_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read cookies\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    } else {
        wget_debug_printf(
            b"Fetched cookies from '%s'\n\0" as *const u8 as *const libc::c_char,
            fname,
        );
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn cookie_db_save(
    mut cookie_db: *mut wget_cookie_db,
    mut fp: *mut FILE,
) -> libc::c_int {
    if wget_vector_size((*cookie_db).cookies) > 0 as libc::c_int {
        let mut it: libc::c_int = 0;
        let mut now: int64_t = time(0 as *mut time_t);
        fputs(b"# HTTP Cookie File\n\0" as *const u8 as *const libc::c_char, fp);
        fputs(
            b"#Generated by libwget 2.2.0. Edit at your own risk.\n\n\0" as *const u8
                as *const libc::c_char,
            fp,
        );
        let mut current_block_5: u64;
        it = 0 as libc::c_int;
        while it < wget_vector_size((*cookie_db).cookies) {
            let mut cookie: *mut wget_cookie = wget_vector_get((*cookie_db).cookies, it)
                as *mut wget_cookie;
            if !cookie.is_null() {
                if (*cookie).persistent() {
                    if (*cookie).expires <= now {
                        current_block_5 = 12675440807659640239;
                    } else {
                        current_block_5 = 3276175668257526147;
                    }
                } else if !(*cookie_db).keep_session_cookies() {
                    current_block_5 = 12675440807659640239;
                } else {
                    current_block_5 = 3276175668257526147;
                }
                match current_block_5 {
                    12675440807659640239 => {}
                    _ => {
                        wget_fprintf(
                            fp,
                            b"%s%s%s\t%s\t%s\t%s\t%lld\t%s\t%s\n\0" as *const u8
                                as *const libc::c_char,
                            if (*cookie).http_only() as libc::c_int != 0 {
                                b"#HttpOnly_\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            if (*cookie).domain_dot() as libc::c_int != 0 {
                                b".\0" as *const u8 as *const libc::c_char
                            } else {
                                b"\0" as *const u8 as *const libc::c_char
                            },
                            (*cookie).domain,
                            if (*cookie).host_only() as libc::c_int != 0 {
                                b"FALSE\0" as *const u8 as *const libc::c_char
                            } else {
                                b"TRUE\0" as *const u8 as *const libc::c_char
                            },
                            (*cookie).path,
                            if (*cookie).secure_only() as libc::c_int != 0 {
                                b"TRUE\0" as *const u8 as *const libc::c_char
                            } else {
                                b"FALSE\0" as *const u8 as *const libc::c_char
                            },
                            (*cookie).expires as libc::c_longlong,
                            (*cookie).name,
                            (*cookie).value,
                        );
                        if ferror(fp) != 0 {
                            return -(1 as libc::c_int);
                        }
                    }
                }
            }
            it += 1;
            it;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_db_save(
    mut cookie_db: *mut wget_cookie_db,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut size: libc::c_int = 0;
    if cookie_db.is_null() || fname.is_null() || *fname == 0 {
        return -(1 as libc::c_int);
    }
    if wget_update_file(
        fname,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut wget_cookie_db, *mut FILE) -> libc::c_int,
            >,
            Option::<wget_update_load_fn>,
        >(
            Some(
                cookie_db_load
                    as unsafe extern "C" fn(
                        *mut wget_cookie_db,
                        *mut FILE,
                    ) -> libc::c_int,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut wget_cookie_db, *mut FILE) -> libc::c_int,
            >,
            Option::<wget_update_save_fn>,
        >(
            Some(
                cookie_db_save
                    as unsafe extern "C" fn(
                        *mut wget_cookie_db,
                        *mut FILE,
                    ) -> libc::c_int,
            ),
        ),
        cookie_db as *mut libc::c_void,
    ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write cookie file '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            fname,
        );
        return -(1 as libc::c_int);
    }
    size = wget_vector_size((*cookie_db).cookies);
    if size != 0 {
        wget_debug_printf(
            b"Saved %d cookie%s into '%s'\n\0" as *const u8 as *const libc::c_char,
            size,
            if size != 1 as libc::c_int {
                b"s\0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            fname,
        );
    } else {
        wget_debug_printf(
            b"No cookies to save. Table is empty.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
