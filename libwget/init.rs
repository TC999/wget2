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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wget_thread_mutex_st;
    pub type wget_cookie_db_st;
    pub type wget_tcp_st;
    pub type wget_dns_cache_st;
    pub type wget_dns_st;
    pub type wget_logger_st;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn wget_logger_set_func(logger: *mut wget_logger, func: Option::<wget_logger_func>);
    fn wget_logger_set_stream(logger: *mut wget_logger, fp: *mut FILE);
    fn wget_logger_set_file(logger: *mut wget_logger, fname: *const libc::c_char);
    fn wget_logger_get_func(logger: *mut wget_logger) -> Option::<wget_logger_func>;
    fn wget_logger_get_stream(logger: *mut wget_logger) -> *mut FILE;
    fn wget_logger_get_file(logger: *mut wget_logger) -> *const libc::c_char;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn wget_cookie_db_init(cookie_db: *mut wget_cookie_db) -> *mut wget_cookie_db;
    fn wget_cookie_db_free(cookie_db: *mut *mut wget_cookie_db);
    fn wget_cookie_set_keep_session_cookies(cookie_db: *mut wget_cookie_db, keep: bool);
    fn wget_cookie_db_save(
        cookie_db: *mut wget_cookie_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_cookie_db_load(
        cookie_db: *mut wget_cookie_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_cookie_db_load_psl(
        cookie_db: *mut wget_cookie_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_dns_cache_init(cache: *mut *mut wget_dns_cache) -> libc::c_int;
    fn wget_dns_cache_free(cache: *mut *mut wget_dns_cache);
    fn wget_dns_set_cache(dns: *mut wget_dns, cache: *mut wget_dns_cache);
    fn wget_net_init() -> libc::c_int;
    fn wget_net_deinit() -> libc::c_int;
    fn wget_tcp_set_tcp_fastopen(tcp: *mut wget_tcp, tcp_fastopen: bool);
    fn wget_tcp_get_family(tcp: *mut wget_tcp) -> libc::c_int;
    fn wget_tcp_get_preferred_family(tcp: *mut wget_tcp) -> libc::c_int;
    fn wget_tcp_set_family(tcp: *mut wget_tcp, family: libc::c_int);
    fn wget_tcp_set_preferred_family(tcp: *mut wget_tcp, family: libc::c_int);
    fn wget_tcp_set_bind_address(tcp: *mut wget_tcp, bind_address: *const libc::c_char);
    fn wget_tcp_set_bind_interface(
        tcp: *mut wget_tcp,
        bind_interface: *const libc::c_char,
    );
    fn wget_ssl_deinit();
    fn wget_http_set_http_proxy(
        proxy: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_http_set_https_proxy(
        proxy: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_http_set_no_proxy(
        no_proxy: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_http_init();
    fn wget_random_init();
    fn wget_console_init() -> libc::c_int;
    fn wget_console_deinit() -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type wget_global_func = unsafe extern "C" fn(*const libc::c_char, size_t) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_cookie_db = wget_cookie_db_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub cookie_file: *mut libc::c_char,
    pub cookie_db: *mut wget_cookie_db,
    pub cookies_enabled: bool,
    pub keep_session_cookies: bool,
}
pub type wget_tcp = wget_tcp_st;
pub type wget_dns_cache = wget_dns_cache_st;
pub type wget_dns = wget_dns_st;
pub type wget_logger = wget_logger_st;
pub type wget_logger_func = unsafe extern "C" fn(*const libc::c_char, size_t) -> ();
static mut config: config = {
    let mut init = config {
        cookie_file: 0 as *const libc::c_char as *mut libc::c_char,
        cookie_db: 0 as *const wget_cookie_db as *mut wget_cookie_db,
        cookies_enabled: 0 as libc::c_int != 0,
        keep_session_cookies: false,
    };
    init
};
static mut dns_cache: *mut wget_dns_cache = 0 as *const wget_dns_cache
    as *mut wget_dns_cache;
static mut global_initialized: libc::c_int = 0;
static mut _mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut initialized: bool = false;
unsafe extern "C" fn global_exit() {
    if initialized {
        wget_thread_mutex_destroy(&mut _mutex);
        initialized = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn global_init() {
    if !initialized {
        wget_thread_mutex_init(&mut _mutex);
        initialized = 1 as libc::c_int != 0;
        atexit(Some(global_exit as unsafe extern "C" fn() -> ()));
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_global_init(mut first_key: libc::c_int, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut key: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut caching: libc::c_int = 0;
    let mut psl_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut func: Option::<wget_logger_func> = None;
    global_init();
    wget_thread_mutex_lock(_mutex);
    let fresh0 = global_initialized;
    global_initialized = global_initialized + 1;
    if fresh0 != 0 {
        wget_thread_mutex_unlock(_mutex);
        return;
    }
    wget_console_init();
    wget_random_init();
    wget_http_init();
    args_0 = args.clone();
    key = first_key;
    while key != 0 {
        match key {
            1000 => {
                wget_logger_set_stream(
                    wget_get_logger(3 as libc::c_int),
                    args_0.arg::<*mut FILE>(),
                );
            }
            1001 => {
                func = ::core::mem::transmute(
                    args_0
                        .arg::<
                            *mut unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                        >(),
                );
                wget_logger_set_func(wget_get_logger(3 as libc::c_int), func);
            }
            1002 => {
                wget_logger_set_file(
                    wget_get_logger(3 as libc::c_int),
                    args_0.arg::<*const libc::c_char>(),
                );
            }
            1003 => {
                wget_logger_set_stream(
                    wget_get_logger(2 as libc::c_int),
                    args_0.arg::<*mut FILE>(),
                );
            }
            1004 => {
                func = ::core::mem::transmute(
                    args_0
                        .arg::<
                            *mut unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                        >(),
                );
                wget_logger_set_func(wget_get_logger(2 as libc::c_int), func);
            }
            1005 => {
                wget_logger_set_file(
                    wget_get_logger(2 as libc::c_int),
                    args_0.arg::<*const libc::c_char>(),
                );
            }
            1006 => {
                wget_logger_set_stream(
                    wget_get_logger(1 as libc::c_int),
                    args_0.arg::<*mut FILE>(),
                );
            }
            1007 => {
                func = ::core::mem::transmute(
                    args_0
                        .arg::<
                            *mut unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                        >(),
                );
                wget_logger_set_func(wget_get_logger(1 as libc::c_int), func);
            }
            1008 => {
                wget_logger_set_file(
                    wget_get_logger(1 as libc::c_int),
                    args_0.arg::<*const libc::c_char>(),
                );
            }
            1009 => {
                caching = args_0.arg::<libc::c_int>();
                if caching != 0 {
                    rc = wget_dns_cache_init(&mut dns_cache);
                    if rc == WGET_E_SUCCESS as libc::c_int {
                        wget_dns_set_cache(0 as *mut wget_dns, dns_cache);
                    } else {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to init DNS cache (%d)\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            rc,
                        );
                    }
                }
            }
            1018 => {
                wget_tcp_set_tcp_fastopen(
                    0 as *mut wget_tcp,
                    args_0.arg::<libc::c_int>() != 0 as libc::c_int,
                );
            }
            1010 => {
                psl_file = args_0.arg::<*const libc::c_char>();
                config.cookies_enabled = 1 as libc::c_int != 0;
            }
            1011 => {
                config.cookies_enabled = args_0.arg::<libc::c_int>() != 0 as libc::c_int;
            }
            1012 => {
                config.cookies_enabled = 1 as libc::c_int != 0;
                config.cookie_file = args_0.arg::<*mut libc::c_char>();
            }
            1014 => {
                config
                    .keep_session_cookies = args_0.arg::<libc::c_int>()
                    != 0 as libc::c_int;
            }
            1015 => {
                wget_tcp_set_bind_address(
                    0 as *mut wget_tcp,
                    args_0.arg::<*const libc::c_char>(),
                );
            }
            1016 => {
                wget_tcp_set_family(0 as *mut wget_tcp, args_0.arg::<libc::c_int>());
            }
            1017 => {
                wget_tcp_set_preferred_family(
                    0 as *mut wget_tcp,
                    args_0.arg::<libc::c_int>(),
                );
            }
            1019 => {
                wget_tcp_set_bind_interface(
                    0 as *mut wget_tcp,
                    args_0.arg::<*const libc::c_char>(),
                );
            }
            _ => {
                wget_thread_mutex_unlock(_mutex);
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Unknown option %d\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*::core::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"wget_global_init\0"))
                        .as_ptr(),
                    key,
                );
                return;
            }
        }
        key = args_0.arg::<libc::c_int>();
    }
    if config.cookies_enabled as libc::c_int != 0 && !(config.cookie_file).is_null() {
        config.cookie_db = wget_cookie_db_init(0 as *mut wget_cookie_db);
        wget_cookie_set_keep_session_cookies(
            config.cookie_db,
            config.keep_session_cookies,
        );
        wget_cookie_db_load(config.cookie_db, config.cookie_file);
        wget_cookie_db_load_psl(config.cookie_db, psl_file);
    }
    rc = wget_net_init();
    wget_thread_mutex_unlock(_mutex);
    if rc != 0 {
        wget_error_printf_exit(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to init networking (%d)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"wget_global_init\0"))
                .as_ptr(),
            rc,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_global_deinit() {
    let mut rc: libc::c_int = 0 as libc::c_int;
    if global_initialized == 1 as libc::c_int {
        if !(config.cookie_db).is_null() && config.cookies_enabled as libc::c_int != 0
            && !(config.cookie_file).is_null()
        {
            wget_cookie_db_save(config.cookie_db, config.cookie_file);
            wget_cookie_db_free(&mut config.cookie_db);
        }
        wget_tcp_set_bind_address(0 as *mut wget_tcp, 0 as *const libc::c_char);
        wget_dns_cache_free(&mut dns_cache);
        rc = wget_net_deinit();
        wget_ssl_deinit();
        wget_http_set_http_proxy(0 as *const libc::c_char, 0 as *const libc::c_char);
        wget_http_set_https_proxy(0 as *const libc::c_char, 0 as *const libc::c_char);
        wget_http_set_no_proxy(0 as *const libc::c_char, 0 as *const libc::c_char);
    }
    if global_initialized > 0 as libc::c_int {
        global_initialized -= 1;
        global_initialized;
    }
    global_exit();
    if rc != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: Failed to deinit networking (%d)\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"wget_global_deinit\0"))
                .as_ptr(),
            rc,
        );
    }
    wget_console_deinit();
}
#[no_mangle]
pub unsafe extern "C" fn wget_global_get_int(mut key: libc::c_int) -> libc::c_int {
    match key {
        1011 => return config.cookies_enabled as libc::c_int,
        1014 => return config.keep_session_cookies as libc::c_int,
        1016 => return wget_tcp_get_family(0 as *mut wget_tcp),
        1017 => return wget_tcp_get_preferred_family(0 as *mut wget_tcp),
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown option %d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"wget_global_get_int\0"))
                    .as_ptr(),
                key,
            );
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_global_get_ptr(
    mut key: libc::c_int,
) -> *const libc::c_void {
    match key {
        1000 => {
            return wget_logger_get_stream(wget_get_logger(3 as libc::c_int))
                as *const libc::c_void;
        }
        1002 => {
            return wget_logger_get_file(wget_get_logger(3 as libc::c_int))
                as *const libc::c_void;
        }
        1003 => {
            return wget_logger_get_stream(wget_get_logger(2 as libc::c_int))
                as *const libc::c_void;
        }
        1005 => {
            return wget_logger_get_file(wget_get_logger(2 as libc::c_int))
                as *const libc::c_void;
        }
        1006 => {
            return wget_logger_get_stream(wget_get_logger(1 as libc::c_int))
                as *const libc::c_void;
        }
        1008 => {
            return wget_logger_get_file(wget_get_logger(1 as libc::c_int))
                as *const libc::c_void;
        }
        1012 => return config.cookie_file as *const libc::c_void,
        1013 => return config.cookie_db as *const libc::c_void,
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown option %d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_char; 20],
                >(b"wget_global_get_ptr\0"))
                    .as_ptr(),
                key,
            );
            return 0 as *const libc::c_void;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_global_get_func(
    mut key: libc::c_int,
) -> Option::<wget_global_func> {
    match key {
        1001 => return wget_logger_get_func(wget_get_logger(3 as libc::c_int)),
        1004 => return wget_logger_get_func(wget_get_logger(2 as libc::c_int)),
        1007 => return wget_logger_get_func(wget_get_logger(1 as libc::c_int)),
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown option %d\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 21],
                    &[libc::c_char; 21],
                >(b"wget_global_get_func\0"))
                    .as_ptr(),
                key,
            );
            return None;
        }
    };
}
