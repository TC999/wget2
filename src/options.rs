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
    pub type dirent;
    pub type __spawn_action;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_cookie_db_st;
    pub type wget_hsts_db_st;
    pub type wget_hpkp_db_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    pub type wget_dns_cache_st;
    pub type wget_dns_st;
    pub type wget_tcp_st;
    pub type dl_file_st;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn getpass(__prompt: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn posix_spawnp(
        __pid: *mut pid_t,
        __file: *const libc::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_destroy(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_addclose(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
    ) -> libc::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: libc::c_int,
        __newfd: libc::c_int,
    ) -> libc::c_int;
    fn xgethostname() -> *mut libc::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn wget_global_deinit();
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strtolower(s: *mut libc::c_char) -> *mut libc::c_char;
    fn wget_percent_unescape(src: *mut libc::c_char) -> libc::c_int;
    fn wget_strnglob(
        str: *const libc::c_char,
        n: size_t,
        flags: libc::c_int,
    ) -> *mut libc::c_char;
    fn wget_getline(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        fp: *mut FILE,
    ) -> ssize_t;
    fn wget_local_charset_encoding() -> *const libc::c_char;
    fn wget_str_needs_encoding(s: *const libc::c_char) -> bool;
    fn wget_str_to_utf8(
        src: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wget_str_to_ascii(src: *const libc::c_char) -> *const libc::c_char;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_memcpy(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_memcat(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_strcat(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
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
    fn wget_vector_clear(v: *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_hashmap_put(
        h: *mut wget_hashmap,
        key: *const libc::c_void,
        value: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_clear(h: *mut wget_hashmap);
    fn wget_content_encoding_by_name(name: *const libc::c_char) -> wget_content_encoding;
    fn wget_content_encoding_to_name(
        type_0: wget_content_encoding,
    ) -> *const libc::c_char;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_set_defaultpage(page: *const libc::c_char);
    fn wget_iri_set_defaultport(
        scheme: wget_iri_scheme,
        port: libc::c_ushort,
    ) -> libc::c_int;
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_cookie_db_init(cookie_db: *mut wget_cookie_db) -> *mut wget_cookie_db;
    fn wget_cookie_db_free(cookie_db: *mut *mut wget_cookie_db);
    fn wget_cookie_set_keep_session_cookies(cookie_db: *mut wget_cookie_db, keep: bool);
    fn wget_cookie_db_load(
        cookie_db: *mut wget_cookie_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_cookie_db_load_psl(
        cookie_db: *mut wget_cookie_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_hsts_db_init(
        _: *mut wget_hsts_db,
        _: *const libc::c_char,
    ) -> *mut wget_hsts_db;
    fn wget_hsts_db_free(_: *mut *mut wget_hsts_db);
    fn wget_hsts_db_load(_: *mut wget_hsts_db) -> libc::c_int;
    fn wget_hpkp_db_init(
        _: *mut wget_hpkp_db,
        _: *const libc::c_char,
    ) -> *mut wget_hpkp_db;
    fn wget_hpkp_db_free(_: *mut *mut wget_hpkp_db);
    fn wget_hpkp_db_load(_: *mut wget_hpkp_db) -> libc::c_int;
    fn wget_tls_session_db_init(
        tls_session_db: *mut wget_tls_session_db,
    ) -> *mut wget_tls_session_db;
    fn wget_tls_session_db_free(tls_session_db: *mut *mut wget_tls_session_db);
    fn wget_tls_session_db_load(
        tls_session_db: *mut wget_tls_session_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_ocsp_db_init(
        _: *mut wget_ocsp_db,
        _: *const libc::c_char,
    ) -> *mut wget_ocsp_db;
    fn wget_ocsp_db_free(_: *mut *mut wget_ocsp_db);
    fn wget_ocsp_db_load(_: *mut wget_ocsp_db) -> libc::c_int;
    fn wget_netrc_db_free(netrc_db: *mut *mut wget_netrc_db);
    fn wget_dns_cache_init(cache: *mut *mut wget_dns_cache) -> libc::c_int;
    fn wget_dns_cache_free(cache: *mut *mut wget_dns_cache);
    fn wget_dns_init(dns_0: *mut *mut wget_dns) -> libc::c_int;
    fn wget_dns_free(dns_0: *mut *mut wget_dns);
    fn wget_dns_set_timeout(dns_0: *mut wget_dns, timeout: libc::c_int);
    fn wget_dns_set_cache(dns_0: *mut wget_dns, cache: *mut wget_dns_cache);
    fn wget_dns_cache_ip(
        dns_0: *mut wget_dns,
        ip: *const libc::c_char,
        name: *const libc::c_char,
        port: uint16_t,
    ) -> libc::c_int;
    fn wget_net_init() -> libc::c_int;
    fn wget_tcp_set_dns(tcp: *mut wget_tcp, dns_0: *mut wget_dns);
    fn wget_tcp_set_timeout(tcp: *mut wget_tcp, timeout: libc::c_int);
    fn wget_tcp_set_connect_timeout(tcp: *mut wget_tcp, timeout: libc::c_int);
    fn wget_tcp_set_tcp_fastopen(tcp: *mut wget_tcp, tcp_fastopen: bool);
    fn wget_tcp_set_tls_false_start(tcp: *mut wget_tcp, false_start: bool);
    fn wget_tcp_set_family(tcp: *mut wget_tcp, family: libc::c_int);
    fn wget_tcp_set_preferred_family(tcp: *mut wget_tcp, family: libc::c_int);
    fn wget_tcp_set_bind_address(tcp: *mut wget_tcp, bind_address: *const libc::c_char);
    fn wget_tcp_set_bind_interface(
        tcp: *mut wget_tcp,
        bind_interface: *const libc::c_char,
    );
    fn wget_ip_is_family(host: *const libc::c_char, family: libc::c_int) -> bool;
    fn wget_ssl_set_config_string(key: libc::c_int, value: *const libc::c_char);
    fn wget_ssl_set_config_object(key: libc::c_int, value: *mut libc::c_void);
    fn wget_ssl_set_config_int(key: libc::c_int, value: libc::c_int);
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
    fn wget_http_free_param(param: *mut wget_http_header_param);
    fn wget_http_free_challenge(challenge: *mut wget_http_challenge);
    fn wget_dns_set_stats_callback(
        dns_0: *mut wget_dns,
        fn_0: Option::<wget_dns_stats_callback>,
        ctx: *mut libc::c_void,
    );
    fn wget_ssl_set_stats_callback_ocsp(
        fn_0: Option::<wget_ocsp_stats_callback>,
        ctx: *mut libc::c_void,
    );
    fn wget_ssl_set_stats_callback_tls(
        fn_0: Option::<wget_tls_stats_callback>,
        ctx: *mut libc::c_void,
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn log_init();
    fn dl_error_set(e: *mut dl_error_t, msg: *const libc::c_char);
    fn plugin_db_add_search_paths(paths: *const libc::c_char, separator: libc::c_char);
    fn plugin_db_clear_search_paths();
    fn plugin_db_load_from_path(
        path: *const libc::c_char,
        e: *mut dl_error_t,
    ) -> *mut plugin_t;
    fn plugin_db_load_from_name(
        name: *const libc::c_char,
        e: *mut dl_error_t,
    ) -> *mut plugin_t;
    fn plugin_db_load_from_envvar() -> libc::c_int;
    fn plugin_db_list(names_out: *mut wget_vector);
    fn plugin_db_forward_option(
        plugin_option: *const libc::c_char,
        e: *mut dl_error_t,
    ) -> libc::c_int;
    fn plugin_db_help_forwarded() -> bool;
    fn plugin_db_show_help();
    fn site_stats_init(fp: *mut FILE);
    fn site_stats_exit();
    fn server_stats_init(fp: *mut FILE);
    fn server_stats_exit();
    fn is_testing() -> bool;
    fn mkdir_path(fname: *const libc::c_char, is_file: bool);
    fn shell_expand(fname: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type __size_t = libc::c_ulong;
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
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: libc::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: libc::c_int,
    pub __cgroup: libc::c_int,
    pub __pad: [libc::c_int; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: libc::c_int,
    pub __used: libc::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [libc::c_int; 16],
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_realloc_function = unsafe extern "C" fn(
    *mut libc::c_void,
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
pub type wget_hashmap = wget_hashmap_st;
pub type wget_stringmap = wget_hashmap;
pub type wget_content_encoding = libc::c_int;
pub const wget_content_encoding_max: wget_content_encoding = 9;
pub const wget_content_encoding_lzip: wget_content_encoding = 8;
pub const wget_content_encoding_zstd: wget_content_encoding = 7;
pub const wget_content_encoding_brotli: wget_content_encoding = 6;
pub const wget_content_encoding_bzip2: wget_content_encoding = 5;
pub const wget_content_encoding_lzma: wget_content_encoding = 4;
pub const wget_content_encoding_xz: wget_content_encoding = 3;
pub const wget_content_encoding_deflate: wget_content_encoding = 2;
pub const wget_content_encoding_gzip: wget_content_encoding = 1;
pub const wget_content_encoding_identity: wget_content_encoding = 0;
pub const wget_content_encoding_unknown: wget_content_encoding = -1;
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
pub type wget_cookie_db = wget_cookie_db_st;
pub type wget_hsts_db = wget_hsts_db_st;
pub type wget_hsts_db_init_fn = unsafe extern "C" fn(
    *mut wget_hsts_db,
    *const libc::c_char,
) -> *mut wget_hsts_db;
pub type wget_hsts_db_free_fn = unsafe extern "C" fn(*mut *mut wget_hsts_db) -> ();
pub type wget_hsts_db_load_fn = unsafe extern "C" fn(*mut wget_hsts_db) -> libc::c_int;
pub type wget_hpkp_db = wget_hpkp_db_st;
pub type wget_hpkp_db_init_fn = unsafe extern "C" fn(
    *mut wget_hpkp_db,
    *const libc::c_char,
) -> *mut wget_hpkp_db;
pub type wget_hpkp_db_free_fn = unsafe extern "C" fn(*mut *mut wget_hpkp_db) -> ();
pub type wget_hpkp_db_load_fn = unsafe extern "C" fn(*mut wget_hpkp_db) -> libc::c_int;
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_ocsp_db_init_fn = unsafe extern "C" fn(
    *mut wget_ocsp_db,
    *const libc::c_char,
) -> *mut wget_ocsp_db;
pub type wget_ocsp_db_free_fn = unsafe extern "C" fn(*mut *mut wget_ocsp_db) -> ();
pub type wget_ocsp_db_load_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> libc::c_int;
pub type wget_netrc_db = wget_netrc_db_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_html_tag {
    pub name: *const libc::c_char,
    pub attribute: *const libc::c_char,
}
pub type wget_dns_cache = wget_dns_cache_st;
pub type wget_dns = wget_dns_st;
pub type wget_tcp = wget_tcp_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_header_param {
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_challenge {
    pub auth_scheme: *const libc::c_char,
    pub params: *mut wget_stringmap,
}
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_plugin_vtable {
    pub get_name: Option::<
        unsafe extern "C" fn(*mut wget_plugin) -> *const libc::c_char,
    >,
    pub register_finalizer: Option::<
        unsafe extern "C" fn(*mut wget_plugin, Option::<wget_plugin_finalizer_fn>) -> (),
    >,
    pub register_argp: Option::<
        unsafe extern "C" fn(
            *mut wget_plugin,
            Option::<wget_plugin_option_callback>,
        ) -> (),
    >,
    pub action_reject: Option::<unsafe extern "C" fn(*mut wget_intercept_action) -> ()>,
    pub action_accept: Option::<unsafe extern "C" fn(*mut wget_intercept_action) -> ()>,
    pub action_set_alt_url: Option::<
        unsafe extern "C" fn(*mut wget_intercept_action, *const wget_iri) -> (),
    >,
    pub action_set_local_filename: Option::<
        unsafe extern "C" fn(*mut wget_intercept_action, *const libc::c_char) -> (),
    >,
    pub register_url_filter: Option::<
        unsafe extern "C" fn(
            *mut wget_plugin,
            Option::<wget_plugin_url_filter_callback>,
        ) -> (),
    >,
    pub file_get_source_url: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> *const wget_iri,
    >,
    pub file_get_local_filename: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> *const libc::c_char,
    >,
    pub file_get_size: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> uint64_t,
    >,
    pub file_get_contents: Option::<
        unsafe extern "C" fn(
            *mut wget_downloaded_file,
            *mut *const libc::c_void,
            *mut size_t,
        ) -> libc::c_int,
    >,
    pub file_open_stream: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> *mut FILE,
    >,
    pub file_get_recurse: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> bool,
    >,
    pub file_add_recurse_url: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file, *const wget_iri) -> (),
    >,
    pub register_post_processor: Option::<
        unsafe extern "C" fn(
            *mut wget_plugin,
            Option::<wget_plugin_post_processor>,
        ) -> (),
    >,
}
pub type wget_plugin_post_processor = unsafe extern "C" fn(
    *mut wget_plugin,
    *mut wget_downloaded_file,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_downloaded_file {
    pub vtable: *mut wget_plugin_vtable,
}
pub type wget_plugin = wget_plugin_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_plugin_st {
    pub plugin_data: *mut libc::c_void,
    pub vtable: *mut wget_plugin_vtable,
}
pub type wget_plugin_url_filter_callback = unsafe extern "C" fn(
    *mut wget_plugin,
    *const wget_iri,
    *mut wget_intercept_action,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_intercept_action {
    pub vtable: *mut wget_plugin_vtable,
}
pub type wget_plugin_option_callback = unsafe extern "C" fn(
    *mut wget_plugin,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
pub type wget_plugin_finalizer_fn = unsafe extern "C" fn(
    *mut wget_plugin,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_dns_stats_data {
    pub hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub port: uint16_t,
    pub dns_secs: libc::c_longlong,
}
pub type wget_dns_stats_callback = unsafe extern "C" fn(
    *mut wget_dns,
    *mut wget_dns_stats_data,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_ocsp_stats_data {
    pub hostname: *const libc::c_char,
    pub nvalid: libc::c_int,
    pub nrevoked: libc::c_int,
    pub nignored: libc::c_int,
    pub stapling: libc::c_int,
}
pub type wget_ocsp_stats_callback = unsafe extern "C" fn(
    *mut wget_ocsp_stats_data,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_tls_stats_data {
    pub hostname: *const libc::c_char,
    pub alpn_protocol: *const libc::c_char,
    pub tls_secs: libc::c_longlong,
    pub version: libc::c_int,
    pub cert_chain_size: libc::c_int,
    pub http_protocol: libc::c_char,
    pub false_start: bool,
    pub tfo: bool,
    pub tls_con: bool,
    pub resumed: bool,
}
pub type wget_tls_stats_callback = unsafe extern "C" fn(
    *mut wget_tls_stats_data,
    *mut libc::c_void,
) -> ();
pub type wget_stats_format = libc::c_uint;
pub const WGET_STATS_FORMAT_CSV: wget_stats_format = 1;
pub const WGET_STATS_FORMAT_HUMAN: wget_stats_format = 0;
pub type check_certificate_mode = libc::c_uint;
pub const CHECK_CERTIFICATE_LOG_DISABLED: check_certificate_mode = 2;
pub const CHECK_CERTIFICATE_DISABLED: check_certificate_mode = 1;
pub const CHECK_CERTIFICATE_ENABLED: check_certificate_mode = 0;
pub type https_enforce_mode = libc::c_uint;
pub const HTTPS_ENFORCE_HARD: https_enforce_mode = 2;
pub const HTTPS_ENFORCE_SOFT: https_enforce_mode = 1;
pub const HTTPS_ENFORCE_NONE: https_enforce_mode = 0;
pub type gpg_verify_mode = libc::c_uint;
pub const WGET_GPG_VERIFY_SIG_NO_FAIL: gpg_verify_mode = 2;
pub const GPG_VERIFY_SIG_FAIL: gpg_verify_mode = 1;
pub const GPG_VERIFY_DISABLED: gpg_verify_mode = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const DOWNLOAD_ATTR_USEPATH: C2RustUnnamed = 2;
pub const DOWNLOAD_ATTR_STRIPPATH: C2RustUnnamed = 1;
pub const DOWNLOAD_ATTR_NO: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_args {
    pub filename: *const libc::c_char,
    pub fp: *mut FILE,
    pub format: wget_stats_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub base: *mut wget_iri,
    pub post_file: *const libc::c_char,
    pub post_data: *const libc::c_char,
    pub body_file: *const libc::c_char,
    pub body_data: *const libc::c_char,
    pub http_username: *const libc::c_char,
    pub http_password: *const libc::c_char,
    pub http_proxy_username: *const libc::c_char,
    pub http_proxy_password: *const libc::c_char,
    pub input_encoding: *const libc::c_char,
    pub local_encoding: *const libc::c_char,
    pub remote_encoding: *const libc::c_char,
    pub bind_address: *const libc::c_char,
    pub bind_interface: *const libc::c_char,
    pub input_file: *const libc::c_char,
    pub base_url: *const libc::c_char,
    pub default_page: *const libc::c_char,
    pub referer: *const libc::c_char,
    pub directory_prefix: *const libc::c_char,
    pub http_proxy: *const libc::c_char,
    pub https_proxy: *const libc::c_char,
    pub no_proxy: *const libc::c_char,
    pub cookie_suffixes: *const libc::c_char,
    pub load_cookies: *const libc::c_char,
    pub save_cookies: *const libc::c_char,
    pub logfile: *const libc::c_char,
    pub logfile_append: *const libc::c_char,
    pub user_agent: *const libc::c_char,
    pub output_document: *const libc::c_char,
    pub ca_cert: *const libc::c_char,
    pub ca_directory: *const libc::c_char,
    pub cert_file: *const libc::c_char,
    pub crl_file: *const libc::c_char,
    pub egd_file: *const libc::c_char,
    pub private_key: *const libc::c_char,
    pub random_file: *const libc::c_char,
    pub secure_protocol: *const libc::c_char,
    pub accept_regex: *const libc::c_char,
    pub reject_regex: *const libc::c_char,
    pub gnupg_homedir: *const libc::c_char,
    pub stats_all: *const libc::c_char,
    pub system_config: *const libc::c_char,
    pub user_config: *const libc::c_char,
    pub hsts_file: *const libc::c_char,
    pub hsts_preload_file: *const libc::c_char,
    pub hpkp_file: *const libc::c_char,
    pub tls_session_file: *const libc::c_char,
    pub ocsp_server: *const libc::c_char,
    pub ocsp_file: *const libc::c_char,
    pub netrc_file: *const libc::c_char,
    pub use_askpass_bin: *const libc::c_char,
    pub hostname: *const libc::c_char,
    pub dns_cache_preload: *const libc::c_char,
    pub method: *const libc::c_char,
    pub compression: *mut wget_vector,
    pub domains: *mut wget_vector,
    pub exclude_directories: *mut wget_vector,
    pub exclude_domains: *mut wget_vector,
    pub accept_patterns: *mut wget_vector,
    pub reject_patterns: *mut wget_vector,
    pub follow_tags: *mut wget_vector,
    pub ignore_tags: *mut wget_vector,
    pub default_challenges: *mut wget_vector,
    pub headers: *mut wget_vector,
    pub mime_types: *mut wget_vector,
    pub retry_on_http_error: *mut wget_vector,
    pub save_content_on: *mut wget_vector,
    pub compression_methods: [wget_content_encoding; 10],
    pub hsts_db: *mut wget_hsts_db,
    pub hpkp_db: *mut wget_hpkp_db,
    pub tls_session_db: *mut wget_tls_session_db,
    pub ocsp_db: *mut wget_ocsp_db,
    pub netrc_db: *mut wget_netrc_db,
    pub cookie_db: *mut wget_cookie_db,
    pub stats_dns_args: *mut stats_args,
    pub stats_ocsp_args: *mut stats_args,
    pub stats_server_args: *mut stats_args,
    pub stats_site_args: *mut stats_args,
    pub stats_tls_args: *mut stats_args,
    pub password: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub chunk_size: libc::c_longlong,
    pub quota: libc::c_longlong,
    pub limit_rate: libc::c_longlong,
    pub start_pos: libc::c_longlong,
    pub http2_request_window: libc::c_int,
    pub backups: libc::c_int,
    pub tries: libc::c_int,
    pub wait: libc::c_int,
    pub waitretry: libc::c_int,
    pub restrict_file_names: libc::c_int,
    pub level: libc::c_int,
    pub preferred_family: libc::c_int,
    pub cut_directories: libc::c_int,
    pub connect_timeout: libc::c_int,
    pub dns_timeout: libc::c_int,
    pub read_timeout: libc::c_int,
    pub max_redirect: libc::c_int,
    pub max_threads: libc::c_int,
    pub default_http_port: uint16_t,
    pub default_https_port: uint16_t,
    pub report_speed: wget_report_speed,
    pub check_certificate: check_certificate_mode,
    pub https_enforce: https_enforce_mode,
    pub verify_sig: gpg_verify_mode,
    pub cert_type: libc::c_char,
    pub private_key_type: libc::c_char,
    pub progress: libc::c_char,
    pub regex_type: libc::c_char,
    pub download_attr: libc::c_char,
    pub tls_resume: bool,
    pub content_on_error: bool,
    pub fsync_policy: bool,
    pub netrc: bool,
    pub http2: bool,
    pub http2_only: bool,
    pub ocsp_stapling: bool,
    pub ocsp: bool,
    pub mirror: bool,
    pub backup_converted: bool,
    pub convert_file_only: bool,
    pub convert_links: bool,
    pub ignore_case: bool,
    pub ignore_length: bool,
    pub hsts: bool,
    pub hsts_preload: bool,
    pub hpkp: bool,
    pub random_wait: bool,
    pub trust_server_names: bool,
    pub robots: bool,
    pub parent: bool,
    pub https_only: bool,
    pub content_disposition: bool,
    pub page_requisites: bool,
    pub follow_sitemaps: bool,
    pub force_rss: bool,
    pub force_atom: bool,
    pub force_sitemap: bool,
    pub force_css: bool,
    pub force_html: bool,
    pub force_metalink: bool,
    pub adjust_extension: bool,
    pub save_headers: bool,
    pub clobber: bool,
    pub cache: bool,
    pub inet4_only: bool,
    pub inet6_only: bool,
    pub delete_after: bool,
    pub strict_comments: bool,
    pub protocol_directories: bool,
    pub host_directories: bool,
    pub force_directories: bool,
    pub directories: bool,
    pub timestamping: bool,
    pub use_server_timestamps: bool,
    pub continue_download: bool,
    pub server_response: bool,
    pub keep_alive: bool,
    pub keep_extension: bool,
    pub keep_session_cookies: bool,
    pub cookies: bool,
    pub spider: bool,
    pub dns_caching: bool,
    pub check_hostname: bool,
    pub span_hosts: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub debug: bool,
    pub hyperlink: bool,
    pub metalink: bool,
    pub cut_url_get_vars: bool,
    pub cut_file_get_vars: bool,
    pub proxy: bool,
    pub xattr: bool,
    pub force_progress: bool,
    pub local_db: bool,
    pub dont_write: bool,
    pub filter_urls: bool,
    pub askpass: bool,
    pub verify_save_failed: bool,
    pub retry_connrefused: bool,
    pub unlink: bool,
    pub background: bool,
    pub if_modified_since: bool,
    pub auth_no_challenge: bool,
    pub no_compression: bool,
    pub ocsp_date: bool,
    pub ocsp_nonce: bool,
    pub recursive: bool,
    pub tls_false_start: bool,
    pub tcp_fastopen: bool,
    pub dane: bool,
}
pub type exit_status_e = libc::c_uint;
pub const EXIT_STATUS_GPG_ERROR: exit_status_e = 9;
pub const EXIT_STATUS_REMOTE: exit_status_e = 8;
pub const EXIT_STATUS_PROTOCOL: exit_status_e = 7;
pub const EXIT_STATUS_AUTH: exit_status_e = 6;
pub const EXIT_STATUS_TLS: exit_status_e = 5;
pub const EXIT_STATUS_NETWORK: exit_status_e = 4;
pub const EXIT_STATUS_IO: exit_status_e = 3;
pub const EXIT_STATUS_PARSE_INIT: exit_status_e = 2;
pub const EXIT_STATUS_GENERIC: exit_status_e = 1;
pub const EXIT_STATUS_NO_ERROR: exit_status_e = 0;
pub type option_t = *const optionw;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct optionw {
    pub long_name: [libc::c_char; 22],
    pub var: *mut libc::c_void,
    pub parser: Option::<
        unsafe extern "C" fn(option_t, *const libc::c_char, libc::c_char) -> libc::c_int,
    >,
    pub args: libc::c_int,
    pub short_name: libc::c_char,
    pub section: help_section_t,
    pub help_str: [*const libc::c_char; 4],
}
pub type help_section_t = libc::c_uint;
pub const SECTION_END: help_section_t = 6;
pub const SECTION_PLUGIN: help_section_t = 5;
pub const SECTION_DIRECTORY: help_section_t = 4;
pub const SECTION_SSL: help_section_t = 3;
pub const SECTION_HTTP: help_section_t = 2;
pub const SECTION_DOWNLOAD: help_section_t = 1;
pub const SECTION_STARTUP: help_section_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_error_t {
    pub msg: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_t {
    pub parent: wget_plugin,
    pub name: *mut libc::c_char,
    pub dm: *mut dl_file_t,
}
pub type dl_file_t = dl_file_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub argv: [*const libc::c_char; 3],
    pub result: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub argv: [*const libc::c_char; 3],
    pub result: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub argv: [*const libc::c_char; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub argv: [*const libc::c_char; 5],
    pub result: [*const libc::c_char; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub argv: [*const libc::c_char; 3],
    pub result: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub argv: [*const libc::c_char; 3],
    pub result: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub argv: [*const libc::c_char; 3],
    pub result: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub argv: [*const libc::c_char; 3],
    pub result: bool,
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
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u) / 2 as libc::c_int as size_t;
        __p = (__base as *const libc::c_char).offset((__idx * __size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as size_t);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
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
#[inline]
unsafe extern "C" fn wget_stringmap_put(
    mut h: *mut wget_stringmap,
    mut key: *const libc::c_char,
    mut value: *const libc::c_void,
) -> libc::c_int {
    return wget_hashmap_put(h, key as *const libc::c_void, value);
}
#[inline]
unsafe extern "C" fn wget_stringmap_clear(mut h: *mut wget_stringmap) {
    wget_hashmap_clear(h);
}
#[inline]
unsafe extern "C" fn dl_error_init(mut e: *mut dl_error_t) {
    (*e).msg = 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn dl_error_get_msg(mut e: *mut dl_error_t) -> *const libc::c_char {
    return (*e).msg;
}
static mut exit_status: exit_status_e = EXIT_STATUS_NO_ERROR;
#[no_mangle]
pub unsafe extern "C" fn set_exit_status(mut status: exit_status_e) {
    if exit_status as u64 != 0 {
        if (status as libc::c_uint) < exit_status as libc::c_uint {
            wget_debug_printf(
                b"%s(%d)\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 16],
                    &[libc::c_char; 16],
                >(b"set_exit_status\0"))
                    .as_ptr(),
                status as libc::c_int,
            );
            exit_status = status;
        }
    } else {
        wget_debug_printf(
            b"%s(%d)\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 16],
                &[libc::c_char; 16],
            >(b"set_exit_status\0"))
                .as_ptr(),
            status as libc::c_int,
        );
        exit_status = status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_exit_status() -> exit_status_e {
    return exit_status;
}
static mut version_text: [libc::c_char; 359] = unsafe {
    *::core::mem::transmute::<
        &[u8; 359],
        &[libc::c_char; 359],
    >(
        b"\nCopyright (C) 2012-2015 Tim Ruehsen\nCopyright (C) 2015-2024 Free Software Foundation, Inc.\n\nLicense GPLv3+: GNU GPL version 3 or later\n<http://www.gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\nPlease send bug reports and questions to <bug-wget@gnu.org>.\0",
    )
};
unsafe extern "C" fn print_version(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    puts(
        b"GNU Wget2 2.2.0 - multithreaded metalink/file/website downloader\n\0"
            as *const u8 as *const libc::c_char,
    );
    let version_info: [libc::c_char; 147] = *::core::mem::transmute::<
        &[u8; 147],
        &[libc::c_char; 147],
    >(
        b"+digest +https +ssl/openssl +ipv6 +iri +large-file +nls -ntlm -opie -psl -hsts\n+iconv -idn +zlib -lzma -brotlidec +zstd -bzip2 -lzip -http2 -gpgme\0",
    );
    puts(version_info.as_ptr());
    puts(version_text.as_ptr());
    set_exit_status(EXIT_STATUS_NO_ERROR);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_integer(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    *((*opt).var
        as *mut libc::c_int) = if !val.is_null() { atoi(val) } else { 0 as libc::c_int };
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_uint16(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut port: libc::c_int = if !val.is_null() {
        atoi(val)
    } else {
        0 as libc::c_int
    };
    if port >= 0 as libc::c_int && port <= 65535 as libc::c_int {
        *((*opt).var as *mut uint16_t) = port as uint16_t;
        return 0 as libc::c_int;
    }
    wget_error_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Value out of range (0-65535): %s\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        val,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_double_modifier(
    mut in_0: *const libc::c_char,
    mut d: *mut libc::c_double,
    mut c: *mut libc::c_char,
) -> libc::c_int {
    let mut minus: bool = 0 as libc::c_int != 0;
    while c_isspace(*in_0 as libc::c_int) {
        in_0 = in_0.offset(1);
        in_0;
    }
    if *in_0 as libc::c_int == '+' as i32 {
        in_0 = in_0.offset(1);
        in_0;
    } else if *in_0 as libc::c_int == '-' as i32 {
        in_0 = in_0.offset(1);
        in_0;
        minus = 1 as libc::c_int != 0;
    }
    if !c_isdigit(*in_0 as libc::c_int) {
        return 0 as libc::c_int;
    }
    *d = 0 as libc::c_int as libc::c_double;
    while c_isdigit(*in_0 as libc::c_int) {
        *d = *d * 10 as libc::c_int as libc::c_double
            + (*in_0 as libc::c_int - '0' as i32) as libc::c_double;
        in_0 = in_0.offset(1);
        in_0;
    }
    if *in_0 as libc::c_int == '.' as i32 {
        in_0 = in_0.offset(1);
        in_0;
        let mut q: libc::c_double = 10 as libc::c_int as libc::c_double;
        while c_isdigit(*in_0 as libc::c_int) {
            *d += (*in_0 as libc::c_int - '0' as i32) as libc::c_double / q;
            q *= 10 as libc::c_int as libc::c_double;
            in_0 = in_0.offset(1);
            in_0;
        }
    }
    if minus {
        *d = -*d;
    }
    *c = *in_0;
    return if *c as libc::c_int != 0 { 2 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn parse_numbytes(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !val.is_null() {
        let mut modifier: libc::c_char = 0 as libc::c_int as libc::c_char;
        let mut error: libc::c_char = 0 as libc::c_int as libc::c_char;
        let mut num: libc::c_double = 0 as libc::c_int as libc::c_double;
        if wget_strcasecmp_ascii(val, b"INF\0" as *const u8 as *const libc::c_char) == 0
            || wget_strcasecmp_ascii(
                val,
                b"INFINITY\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            *((*opt).var
                as *mut libc::c_longlong) = 0 as libc::c_int as libc::c_longlong;
            return 0 as libc::c_int;
        }
        if parse_double_modifier(val, &mut num, &mut modifier) >= 1 as libc::c_int
            && num >= 0 as libc::c_int as libc::c_double
        {
            if modifier != 0 {
                match c_tolower(modifier as libc::c_int) {
                    107 => {
                        num *= 1024 as libc::c_int as libc::c_double;
                    }
                    109 => {
                        num
                            *= (1024 as libc::c_int * 1024 as libc::c_int)
                                as libc::c_double;
                    }
                    103 => {
                        num
                            *= (1024 as libc::c_int * 1024 as libc::c_int
                                * 1024 as libc::c_int) as libc::c_double;
                    }
                    116 => {
                        num
                            *= ((1024 as libc::c_int * 1024 as libc::c_int
                                * 1024 as libc::c_int) as libc::c_longlong
                                * 1024 as libc::c_longlong) as libc::c_double;
                    }
                    _ => {
                        error = 1 as libc::c_int as libc::c_char;
                    }
                }
            }
        } else {
            error = 1 as libc::c_int as libc::c_char;
        }
        if error != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid byte specifier: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            return -(1 as libc::c_int);
        }
        *((*opt).var
            as *mut libc::c_longlong) = if num
            > 9223372036854775807 as libc::c_longlong as libc::c_double
        {
            9223372036854775807 as libc::c_longlong
        } else {
            num as libc::c_longlong
        };
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_filename(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !(*((*opt).var as *mut *const libc::c_char)).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(*((*opt).var as *mut *const libc::c_char) as *mut libc::c_void);
        let ref mut fresh0 = *((*opt).var as *mut *const libc::c_char);
        *fresh0 = 0 as *const libc::c_char;
    }
    let ref mut fresh1 = *((*opt).var as *mut *const libc::c_char);
    *fresh1 = if !val.is_null() { shell_expand(val) } else { 0 as *mut libc::c_char };
    wget_debug_printf(
        b"Expanded value = %s\n\0" as *const u8 as *const libc::c_char,
        *((*opt).var as *mut *const libc::c_char),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_string(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !(*((*opt).var as *mut *const libc::c_char)).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(*((*opt).var as *mut *const libc::c_char) as *mut libc::c_void);
        let ref mut fresh2 = *((*opt).var as *mut *const libc::c_char);
        *fresh2 = 0 as *const libc::c_char;
    }
    let ref mut fresh3 = *((*opt).var as *mut *const libc::c_char);
    *fresh3 = wget_strdup(val);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_stringset(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut map: *mut wget_stringmap = *((*opt).var as *mut *mut wget_stringmap);
    if !val.is_null() {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        wget_stringmap_clear(map);
        p = val;
        s = p;
        while *p != 0 {
            p = strchrnul(s, ',' as i32);
            if p != s {
                wget_stringmap_put(
                    map,
                    wget_strmemdup(
                        s as *const libc::c_void,
                        p.offset_from(s) as libc::c_long as size_t,
                    ),
                    0 as *const libc::c_void,
                );
            }
            s = p.offset(1 as libc::c_int as isize);
        }
    } else {
        wget_stringmap_clear(map);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn compare_wget_http_param(
    mut a: *mut wget_http_header_param,
    mut b: *mut wget_http_header_param,
) -> libc::c_int {
    if wget_strcasecmp_ascii((*a).name, (*b).name) == 0 as libc::c_int {
        if wget_strcasecmp_ascii((*a).value, (*b).value) == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn parse_header(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut v: *mut wget_vector = *((*opt).var as *mut *mut wget_vector);
    if !val.is_null() && *val as libc::c_int != 0 {
        let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut delim_pos: *mut libc::c_char = 0 as *mut libc::c_char;
        if v.is_null() {
            let ref mut fresh4 = *((*opt).var as *mut *mut wget_vector);
            *fresh4 = wget_vector_create(
                8 as libc::c_int,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut wget_http_header_param,
                            *mut wget_http_header_param,
                        ) -> libc::c_int,
                    >,
                    Option::<wget_vector_compare_fn>,
                >(
                    Some(
                        compare_wget_http_param
                            as unsafe extern "C" fn(
                                *mut wget_http_header_param,
                                *mut wget_http_header_param,
                            ) -> libc::c_int,
                    ),
                ),
            );
            v = *fresh4;
            wget_vector_set_destructor(
                v,
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut wget_http_header_param) -> ()>,
                    Option::<wget_vector_destructor>,
                >(
                    Some(
                        wget_http_free_param
                            as unsafe extern "C" fn(*mut wget_http_header_param) -> (),
                    ),
                ),
            );
        }
        delim_pos = strchr(val, ':' as i32);
        if delim_pos.is_null() || delim_pos == val as *mut libc::c_char {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Ignoring invalid header: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            return 0 as libc::c_int;
        }
        value = delim_pos.offset(1 as libc::c_int as isize);
        while *value as libc::c_int == ' ' as i32 {
            value = value.offset(1);
            value;
        }
        if *value as libc::c_int == '\0' as i32 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No value in header (ignoring): %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            return 0 as libc::c_int;
        }
        let mut param: *mut wget_http_header_param = wget_malloc(
            ::core::mem::size_of::<wget_http_header_param>() as libc::c_ulong,
        ) as *mut wget_http_header_param;
        (*param)
            .name = wget_strmemdup(
            val as *const libc::c_void,
            delim_pos.offset_from(val) as libc::c_long as size_t,
        );
        (*param).value = wget_strdup(value);
        if wget_vector_find(v, param as *const libc::c_void) < 0 as libc::c_int {
            wget_vector_add(v, param as *const libc::c_void);
        } else {
            wget_http_free_param(param);
        }
    } else if !val.is_null() && *val as libc::c_int == '\0' as i32 {
        wget_vector_clear(v);
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn strchrnul_esc(
    mut s: *const libc::c_char,
    mut c: libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = s;
    while *p != 0 {
        if *p as libc::c_int == '\\' as i32
            && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32
                || *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == c as libc::c_int)
        {
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == c as libc::c_int {
            return p
        }
        p = p.offset(1);
        p;
    }
    return p;
}
unsafe extern "C" fn strmemdup_esc(
    mut s: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut newsize: size_t = 0 as libc::c_int as size_t;
    p = s;
    e = s.offset(size as isize);
    while p < e {
        if *p as libc::c_int == '\\' as i32 {
            if p < e.offset(-(1 as libc::c_int as isize)) {
                newsize = newsize.wrapping_add(1);
                newsize;
                p = p.offset(1);
                p;
            }
        } else {
            newsize = newsize.wrapping_add(1);
            newsize;
        }
        p = p.offset(1);
        p;
    }
    let mut ret: *mut libc::c_char = wget_malloc(
        newsize.wrapping_add(1 as libc::c_int as size_t),
    ) as *mut libc::c_char;
    let mut dst: *mut libc::c_char = ret;
    p = s;
    e = s.offset(size as isize);
    while p < e {
        if *p as libc::c_int == '\\' as i32 {
            if p < e.offset(-(1 as libc::c_int as isize)) {
                p = p.offset(1);
                let fresh5 = dst;
                dst = dst.offset(1);
                *fresh5 = *p;
            }
        } else {
            let fresh6 = dst;
            dst = dst.offset(1);
            *fresh6 = *p;
        }
        p = p.offset(1);
        p;
    }
    *dst = 0 as libc::c_int as libc::c_char;
    return ret;
}
unsafe extern "C" fn parse_stringlist_expand(
    mut opt: option_t,
    mut val: *const libc::c_char,
    mut expand: libc::c_int,
    mut max_entries: libc::c_int,
) -> libc::c_int {
    if !val.is_null() && *val as libc::c_int != 0 {
        let mut v: *mut wget_vector = *((*opt).var as *mut *mut wget_vector);
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        if v.is_null() {
            let ref mut fresh7 = *((*opt).var as *mut *mut wget_vector);
            *fresh7 = wget_vector_create(
                8 as libc::c_int,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<wget_vector_compare_fn>,
                >(
                    Some(
                        strcmp
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *const libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            );
            v = *fresh7;
        }
        p = val;
        s = p;
        while *p != 0 {
            p = strchrnul_esc(s, ',' as i32 as libc::c_char);
            if p != s {
                if wget_vector_size(v) >= max_entries {
                    wget_debug_printf(
                        b"%s: More than %d entries, ignoring overflow\n\0" as *const u8
                            as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 24],
                            &[libc::c_char; 24],
                        >(b"parse_stringlist_expand\0"))
                            .as_ptr(),
                        max_entries,
                    );
                    return -(1 as libc::c_int);
                }
                let mut fname: *const libc::c_char = strmemdup_esc(
                    s,
                    p.offset_from(s) as libc::c_long as size_t,
                );
                if expand != 0 && *s as libc::c_int == '~' as i32 {
                    wget_vector_add(v, shell_expand(fname) as *const libc::c_void);
                    if !fname.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(fname as *mut libc::c_void);
                        fname = 0 as *const libc::c_char;
                    }
                } else {
                    wget_vector_add(v, fname as *const libc::c_void);
                }
            }
            s = p.offset(1 as libc::c_int as isize);
        }
    } else {
        wget_vector_free((*opt).var as *mut *mut wget_vector);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_stringlist(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    return parse_stringlist_expand(opt, val, 0 as libc::c_int, 1024 as libc::c_int);
}
unsafe extern "C" fn set_char_prefix(
    mut val: *const libc::c_char,
    mut prefix: libc::c_char,
) -> *mut libc::c_char {
    if !val.is_null() && *val as libc::c_int != 0 {
        let mut prefixed_val: *mut libc::c_char = wget_malloc(
            (strlen(val))
                .wrapping_mul(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        let mut dst: *mut libc::c_char = prefixed_val;
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = prefix;
        let mut src: *const libc::c_char = val;
        while *src != 0 {
            if *src as libc::c_int == '\\' as i32 {
                let fresh9 = dst;
                dst = dst.offset(1);
                *fresh9 = *src;
                if *src.offset(1 as libc::c_int as isize) != 0 {
                    src = src.offset(1);
                    let fresh10 = dst;
                    dst = dst.offset(1);
                    *fresh10 = *src;
                }
            } else if *src as libc::c_int == ',' as i32 {
                while *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32
                {
                    dst = dst.offset(-1);
                    dst;
                }
                let fresh11 = dst;
                dst = dst.offset(1);
                *fresh11 = *src;
                let fresh12 = dst;
                dst = dst.offset(1);
                *fresh12 = prefix;
            } else {
                let fresh13 = dst;
                dst = dst.offset(1);
                *fresh13 = *src;
            }
            src = src.offset(1);
            src;
        }
        while *dst.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32 {
            dst = dst.offset(-1);
            dst;
        }
        *dst = 0 as libc::c_int as libc::c_char;
        return prefixed_val;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn parse_included_directories(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut prefixed_val: *mut libc::c_char = set_char_prefix(
        val,
        '+' as i32 as libc::c_char,
    );
    let mut ret: libc::c_int = parse_stringlist_expand(
        opt,
        prefixed_val,
        0 as libc::c_int,
        1024 as libc::c_int,
    );
    if !prefixed_val.is_null() {
        if !prefixed_val.is_null() {
            wget_free
                .expect("non-null function pointer")(prefixed_val as *mut libc::c_void);
            prefixed_val = 0 as *mut libc::c_char;
        }
    }
    return ret;
}
unsafe extern "C" fn parse_excluded_directories(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut prefixed_val: *mut libc::c_char = set_char_prefix(
        val,
        '-' as i32 as libc::c_char,
    );
    let mut ret: libc::c_int = parse_stringlist_expand(
        opt,
        prefixed_val,
        0 as libc::c_int,
        1024 as libc::c_int,
    );
    if !prefixed_val.is_null() {
        if !prefixed_val.is_null() {
            wget_free
                .expect("non-null function pointer")(prefixed_val as *mut libc::c_void);
            prefixed_val = 0 as *mut libc::c_char;
        }
    }
    return ret;
}
unsafe extern "C" fn parse_filenames(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    return parse_stringlist_expand(opt, val, 1 as libc::c_int, 32 as libc::c_int);
}
unsafe extern "C" fn tag_free(mut tag: *mut libc::c_void) {
    let mut t: *mut wget_html_tag = tag as *mut wget_html_tag;
    if !t.is_null() {
        if !((*t).attribute).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*t).attribute as *mut libc::c_void);
            (*t).attribute = 0 as *const libc::c_char;
        }
        if !((*t).name).is_null() {
            wget_free
                .expect("non-null function pointer")((*t).name as *mut libc::c_void);
            (*t).name = 0 as *const libc::c_char;
        }
        if !t.is_null() {
            wget_free.expect("non-null function pointer")(t as *mut libc::c_void);
            t = 0 as *mut wget_html_tag;
        }
    }
}
unsafe extern "C" fn add_tag(
    mut v: *mut wget_vector,
    mut begin: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut tag: *mut wget_html_tag = wget_malloc(
        ::core::mem::size_of::<wget_html_tag>() as libc::c_ulong,
    ) as *mut wget_html_tag;
    let mut attribute: *const libc::c_char = 0 as *const libc::c_char;
    attribute = memchr(
        begin as *const libc::c_void,
        '/' as i32,
        end.offset_from(begin) as libc::c_long as libc::c_ulong,
    ) as *const libc::c_char;
    if !attribute.is_null() {
        (*tag)
            .name = wget_strmemdup(
            begin as *const libc::c_void,
            attribute.offset_from(begin) as libc::c_long as size_t,
        );
        (*tag)
            .attribute = wget_strmemdup(
            attribute.offset(1 as libc::c_int as isize) as *const libc::c_void,
            (end.offset_from(begin) as libc::c_long
                - attribute.offset_from(begin) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as size_t,
        );
    } else {
        (*tag)
            .name = wget_strmemdup(
            begin as *const libc::c_void,
            end.offset_from(begin) as libc::c_long as size_t,
        );
        (*tag).attribute = 0 as *const libc::c_char;
    }
    if wget_vector_find(v, tag as *const libc::c_void) < 0 as libc::c_int {
        wget_vector_insert_sorted(v, tag as *const libc::c_void);
    } else {
        tag_free(tag as *mut libc::c_void);
    };
}
unsafe extern "C" fn compare_tag(
    mut t1: *const wget_html_tag,
    mut t2: *const wget_html_tag,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = wget_strcasecmp_ascii((*t1).name, (*t2).name);
    if n == 0 {
        if ((*t1).attribute).is_null() {
            if ((*t2).attribute).is_null() {
                n = 0 as libc::c_int;
            } else {
                n = -(1 as libc::c_int);
            }
        } else if ((*t2).attribute).is_null() {
            n = 1 as libc::c_int;
        } else {
            n = wget_strcasecmp_ascii((*t1).attribute, (*t2).attribute);
        }
    }
    return n;
}
unsafe extern "C" fn parse_taglist(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !val.is_null() && *val as libc::c_int != 0 {
        let mut v: *mut wget_vector = *((*opt).var as *mut *mut wget_vector);
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        if v.is_null() {
            let ref mut fresh14 = *((*opt).var as *mut *mut wget_vector);
            *fresh14 = wget_vector_create(
                8 as libc::c_int,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const wget_html_tag,
                            *const wget_html_tag,
                        ) -> libc::c_int,
                    >,
                    Option::<wget_vector_compare_fn>,
                >(
                    Some(
                        compare_tag
                            as unsafe extern "C" fn(
                                *const wget_html_tag,
                                *const wget_html_tag,
                            ) -> libc::c_int,
                    ),
                ),
            );
            v = *fresh14;
            wget_vector_set_destructor(
                v,
                Some(tag_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
        }
        p = val;
        s = p;
        while *p != 0 {
            p = strchrnul(s, ',' as i32);
            if p != s {
                add_tag(v, s, p);
            }
            s = p.offset(1 as libc::c_int as isize);
        }
    } else {
        wget_vector_free((*opt).var as *mut *mut wget_vector);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_check_certificate(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !((*opt).var).is_null() {
        if val.is_null() || strcmp(val, b"1\0" as *const u8 as *const libc::c_char) == 0
            || wget_strcasecmp_ascii(val, b"y\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"yes\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"on\0" as *const u8 as *const libc::c_char)
                == 0
        {
            *((*opt).var
                as *mut check_certificate_mode) = (if invert == 0 {
                CHECK_CERTIFICATE_ENABLED as libc::c_int
            } else {
                CHECK_CERTIFICATE_DISABLED as libc::c_int
            }) as check_certificate_mode;
        } else if *val == 0
            || strcmp(val, b"0\0" as *const u8 as *const libc::c_char) == 0
            || wget_strcasecmp_ascii(val, b"n\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"no\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"off\0" as *const u8 as *const libc::c_char)
                == 0
        {
            *((*opt).var
                as *mut check_certificate_mode) = (if invert as libc::c_int != 0 {
                CHECK_CERTIFICATE_ENABLED as libc::c_int
            } else {
                CHECK_CERTIFICATE_DISABLED as libc::c_int
            }) as check_certificate_mode;
        } else if strcmp(val, b"quiet\0" as *const u8 as *const libc::c_char) == 0 {
            *((*opt).var
                as *mut check_certificate_mode) = CHECK_CERTIFICATE_LOG_DISABLED;
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid value '%s'\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_bool(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !((*opt).var).is_null() {
        if val.is_null() || strcmp(val, b"1\0" as *const u8 as *const libc::c_char) == 0
            || wget_strcasecmp_ascii(val, b"y\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"yes\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"on\0" as *const u8 as *const libc::c_char)
                == 0
        {
            *((*opt).var
                as *mut libc::c_char) = (invert == 0) as libc::c_int as libc::c_char;
        } else if *val == 0
            || strcmp(val, b"0\0" as *const u8 as *const libc::c_char) == 0
            || wget_strcasecmp_ascii(val, b"n\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"no\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(val, b"off\0" as *const u8 as *const libc::c_char)
                == 0
        {
            *((*opt).var as *mut libc::c_char) = invert;
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid boolean value '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_mirror(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = parse_bool(opt, val, invert);
    if rc < 0 as libc::c_int {
        return rc;
    }
    if config.mirror {
        config.recursive = 1 as libc::c_int != 0;
        config.level = 0 as libc::c_int;
        config.timestamping = 1 as libc::c_int != 0;
    } else {
        config.recursive = 0 as libc::c_int != 0;
        config.level = 5 as libc::c_int;
        config.timestamping = 0 as libc::c_int != 0;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_timeout(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut fval: libc::c_double = -(1 as libc::c_int) as libc::c_double;
    if wget_strcasecmp_ascii(val, b"INF\0" as *const u8 as *const libc::c_char) != 0
        && wget_strcasecmp_ascii(val, b"INFINITY\0" as *const u8 as *const libc::c_char)
            != 0
    {
        let mut modifier: libc::c_char = 0 as libc::c_int as libc::c_char;
        if parse_double_modifier(val, &mut fval, &mut modifier) >= 1 as libc::c_int
            && fval >= 0 as libc::c_int as libc::c_double
        {
            if modifier != 0 {
                match c_tolower(modifier as libc::c_int) {
                    115 => {
                        fval *= 1000 as libc::c_int as libc::c_double;
                    }
                    109 => {
                        fval
                            *= (60 as libc::c_int * 1000 as libc::c_int)
                                as libc::c_double;
                    }
                    104 => {
                        fval
                            *= (60 as libc::c_int * 60 as libc::c_int
                                * 1000 as libc::c_int) as libc::c_double;
                    }
                    100 => {
                        fval
                            *= (60 as libc::c_int * 60 as libc::c_int * 24 as libc::c_int
                                * 1000 as libc::c_int) as libc::c_double;
                    }
                    _ => {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Invalid time specifier in '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            val,
                        );
                        return -(1 as libc::c_int);
                    }
                }
            } else {
                fval *= 1000 as libc::c_int as libc::c_double;
            }
        }
    }
    if fval <= 0 as libc::c_int as libc::c_double {
        fval = -(1 as libc::c_int) as libc::c_double;
    }
    if !((*opt).var).is_null() {
        *((*opt).var
            as *mut libc::c_int) = if fval > 2147483647 as libc::c_int as libc::c_double
        {
            2147483647 as libc::c_int
        } else {
            fval as libc::c_int
        };
    } else {
        config
            .read_timeout = if fval > 2147483647 as libc::c_int as libc::c_double {
            2147483647 as libc::c_int
        } else {
            fval as libc::c_int
        };
        config.dns_timeout = config.read_timeout;
        config.connect_timeout = config.dns_timeout;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_cert_type(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if val.is_null()
        || wget_strcasecmp_ascii(val, b"PEM\0" as *const u8 as *const libc::c_char) == 0
    {
        *((*opt).var as *mut libc::c_char) = 0 as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(val, b"DER\0" as *const u8 as *const libc::c_char)
        == 0
        || wget_strcasecmp_ascii(val, b"ASN1\0" as *const u8 as *const libc::c_char) == 0
    {
        *((*opt).var as *mut libc::c_char) = 1 as libc::c_int as libc::c_char;
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown cert type '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_regex_type(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if val.is_null()
        || wget_strcasecmp_ascii(val, b"posix\0" as *const u8 as *const libc::c_char)
            == 0
    {
        *((*opt).var as *mut libc::c_char) = 0 as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(val, b"pcre\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *((*opt).var as *mut libc::c_char) = 1 as libc::c_int as libc::c_char;
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unsupported regex type '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_progress_type(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if val.is_null() || *val == 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Empty progress type\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    if wget_strcasecmp_ascii(val, b"none\0" as *const u8 as *const libc::c_char) == 0 {
        *((*opt).var as *mut libc::c_char) = 0 as libc::c_int as libc::c_char;
    } else if wget_strncasecmp_ascii(
        val,
        b"bar\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) == 0
        && (*val.offset(3 as libc::c_int as isize) as libc::c_int == ':' as i32
            || *val.offset(3 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
    {
        *((*opt).var as *mut libc::c_char) = 1 as libc::c_int as libc::c_char;
        if wget_strncasecmp_ascii(
            val.offset(4 as libc::c_int as isize),
            b"force\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        ) == 0
            || wget_strncasecmp_ascii(
                val.offset(4 as libc::c_int as isize),
                b"noscroll:force\0" as *const u8 as *const libc::c_char,
                14 as libc::c_int as size_t,
            ) == 0
        {
            config.force_progress = 1 as libc::c_int != 0;
        }
    } else if wget_strncasecmp_ascii(
        val,
        b"dot\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    ) == 0
        && (*val.offset(3 as libc::c_int as isize) as libc::c_int == ':' as i32
            || *val.offset(3 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int)
    {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Progress type '%s' ignored. It is not implemented yet\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown progress type '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_restrict_names(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut current_block: u64;
    let mut flags: libc::c_int = 0 as libc::c_int;
    if val.is_null() || *val == 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Missing restrict-file-name type\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        if wget_strcasecmp_ascii(val, b"none\0" as *const u8 as *const libc::c_char) == 0
        {
            *((*opt).var as *mut libc::c_int) = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
        s = 0 as *const libc::c_char;
        p = 0 as *const libc::c_char;
        p = val;
        s = p;
        loop {
            if !(*p != 0) {
                current_block = 13797916685926291137;
                break;
            }
            p = strchrnul(s, ',' as i32);
            if !(p == s) {
                let mut len: size_t = p.offset_from(s) as libc::c_long as size_t;
                if wget_strncasecmp_ascii(
                    s,
                    b"unix\0" as *const u8 as *const libc::c_char,
                    len,
                ) == 0
                {
                    flags |= (1 as libc::c_int) << 0 as libc::c_int;
                } else if wget_strncasecmp_ascii(
                    s,
                    b"windows\0" as *const u8 as *const libc::c_char,
                    len,
                ) == 0
                {
                    flags |= (1 as libc::c_int) << 1 as libc::c_int;
                } else if wget_strncasecmp_ascii(
                    s,
                    b"nocontrol\0" as *const u8 as *const libc::c_char,
                    len,
                ) == 0
                {
                    flags |= (1 as libc::c_int) << 2 as libc::c_int;
                } else if wget_strncasecmp_ascii(
                    s,
                    b"ascii\0" as *const u8 as *const libc::c_char,
                    len,
                ) == 0
                {
                    flags |= (1 as libc::c_int) << 3 as libc::c_int;
                } else if wget_strncasecmp_ascii(
                    s,
                    b"uppercase\0" as *const u8 as *const libc::c_char,
                    len,
                ) == 0
                {
                    flags |= (1 as libc::c_int) << 4 as libc::c_int;
                } else if wget_strncasecmp_ascii(
                    s,
                    b"lowercase\0" as *const u8 as *const libc::c_char,
                    len,
                ) == 0
                {
                    flags |= (1 as libc::c_int) << 5 as libc::c_int;
                } else {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unknown restrict-file-name type '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        val,
                    );
                    current_block = 5659523682992071034;
                    break;
                }
            }
            s = p.offset(1 as libc::c_int as isize);
        }
        match current_block {
            5659523682992071034 => {}
            _ => {
                if flags & (1 as libc::c_int) << 0 as libc::c_int != 0
                    && flags & (1 as libc::c_int) << 1 as libc::c_int != 0
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Restrict file names to either 'unix' or 'windows'\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else if flags & (1 as libc::c_int) << 4 as libc::c_int != 0
                    && flags & (1 as libc::c_int) << 5 as libc::c_int != 0
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Restrict file names to either 'uppercase' or 'lowercase'\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    *((*opt).var as *mut libc::c_int) = flags;
                    return 0 as libc::c_int;
                }
            }
        }
    }
    wget_error_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"    use [none]|[unix|windows],[lowercase|uppercase],[nocontrol][,ascii].\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_n_option(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if !val.is_null() {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        p = val;
        while *p != 0 {
            match *p as libc::c_int {
                118 => {
                    config.verbose = 0 as libc::c_int != 0;
                }
                99 => {
                    config.clobber = 0 as libc::c_int != 0;
                }
                100 => {
                    config.directories = 0 as libc::c_int != 0;
                }
                72 => {
                    config.host_directories = 0 as libc::c_int != 0;
                }
                112 => {
                    config.parent = 0 as libc::c_int != 0;
                }
                _ => {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unknown option '-n%c'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *p as libc::c_int,
                    );
                    return -(1 as libc::c_int);
                }
            }
            wget_debug_printf(
                b"name=-n%c value=0\n\0" as *const u8 as *const libc::c_char,
                *p as libc::c_int,
            );
            p = p.offset(1);
            p;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_prefer_family(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if val.is_null()
        || wget_strcasecmp_ascii(val, b"none\0" as *const u8 as *const libc::c_char) == 0
    {
        *((*opt).var as *mut libc::c_int) = 0 as libc::c_int;
    } else if wget_strcasecmp_ascii(val, b"ipv4\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *((*opt).var as *mut libc::c_int) = 1 as libc::c_int;
    } else if wget_strcasecmp_ascii(val, b"ipv6\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *((*opt).var as *mut libc::c_int) = 2 as libc::c_int;
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown address family '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_stats(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut format: wget_stats_format = WGET_STATS_FORMAT_HUMAN;
    let mut stats: *mut *mut stats_args = (*opt).var as *mut *mut stats_args;
    if !(*stats).is_null() {
        if !((**stats).filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**stats).filename as *mut libc::c_void);
            (**stats).filename = 0 as *const libc::c_char;
        }
    }
    if invert != 0 {
        if !(*stats).is_null() {
            wget_free.expect("non-null function pointer")(*stats as *mut libc::c_void);
            *stats = 0 as *mut stats_args;
        }
        return 0 as libc::c_int;
    }
    if !val.is_null()
        && {
            p = strchr(val, ':' as i32);
            !p.is_null()
        }
    {
        if wget_strncasecmp_ascii(
            b"human\0" as *const u8 as *const libc::c_char,
            val,
            p.offset_from(val) as libc::c_long as size_t,
        ) == 0
            || wget_strncasecmp_ascii(
                b"h\0" as *const u8 as *const libc::c_char,
                val,
                p.offset_from(val) as libc::c_long as size_t,
            ) == 0
        {
            format = WGET_STATS_FORMAT_HUMAN;
        } else if wget_strncasecmp_ascii(
            b"csv\0" as *const u8 as *const libc::c_char,
            val,
            p.offset_from(val) as libc::c_long as size_t,
        ) == 0
        {
            format = WGET_STATS_FORMAT_CSV;
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown stats format '%s'\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                val,
            );
            return -(1 as libc::c_int);
        }
        val = p.offset(1 as libc::c_int as isize);
    } else {
        format = WGET_STATS_FORMAT_HUMAN;
    }
    if (*stats).is_null() {
        *stats = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<stats_args>() as libc::c_ulong,
        ) as *mut stats_args;
    }
    (**stats).filename = shell_expand(val);
    (**stats).format = format;
    return 0 as libc::c_int;
}
static mut plugin_loading_enabled: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn parse_plugin(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut e: [dl_error_t; 1] = [dl_error_t {
        msg: 0 as *const libc::c_char,
    }; 1];
    if plugin_loading_enabled == 0 {
        return 0 as libc::c_int;
    }
    dl_error_init(e.as_mut_ptr());
    if (plugin_db_load_from_name(val, e.as_mut_ptr())).is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Plugin '%s' failed to load: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
            dl_error_get_msg(e.as_mut_ptr()),
        );
        dl_error_set(e.as_mut_ptr(), 0 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_plugin_local(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut e: [dl_error_t; 1] = [dl_error_t {
        msg: 0 as *const libc::c_char,
    }; 1];
    if plugin_loading_enabled == 0 {
        return 0 as libc::c_int;
    }
    dl_error_init(e.as_mut_ptr());
    if (plugin_db_load_from_path(val, e.as_mut_ptr())).is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Plugin '%s' failed to load: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
            dl_error_get_msg(e.as_mut_ptr()),
        );
        dl_error_set(e.as_mut_ptr(), 0 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_plugin_dirs(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if plugin_loading_enabled == 0 {
        return 0 as libc::c_int;
    }
    plugin_db_clear_search_paths();
    plugin_db_add_search_paths(val, ',' as i32 as libc::c_char);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_plugin_option(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut e: [dl_error_t; 1] = [dl_error_t {
        msg: 0 as *const libc::c_char,
    }; 1];
    if plugin_loading_enabled == 0 {
        return 0 as libc::c_int;
    }
    dl_error_init(e.as_mut_ptr());
    if plugin_db_forward_option(val, e.as_mut_ptr()) < 0 as libc::c_int {
        wget_error_printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            dl_error_get_msg(e.as_mut_ptr()),
        );
        dl_error_set(e.as_mut_ptr(), 0 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_local_db(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    rc = parse_bool(opt, val, invert);
    if rc < 0 as libc::c_int {
        return rc;
    }
    config.tls_resume = config.local_db;
    config.ocsp = config.tls_resume;
    config.hpkp = config.ocsp;
    config.hsts = config.hpkp;
    config.cookies = config.hsts;
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_report_speed_type(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if wget_strcasecmp_ascii(val, b"bytes\0" as *const u8 as *const libc::c_char) == 0 {
        *((*opt).var as *mut wget_report_speed) = WGET_REPORT_SPEED_BYTES;
    } else if wget_strcasecmp_ascii(val, b"bits\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *((*opt).var as *mut wget_report_speed) = WGET_REPORT_SPEED_BITS;
    } else if *val.offset(0 as libc::c_int as isize) == 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Missing required type specifier\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid type specifier: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_https_enforce(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if wget_strcasecmp_ascii(val, b"hard\0" as *const u8 as *const libc::c_char) == 0 {
        *((*opt).var as *mut https_enforce_mode) = HTTPS_ENFORCE_HARD;
    } else if wget_strcasecmp_ascii(val, b"soft\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *((*opt).var as *mut https_enforce_mode) = HTTPS_ENFORCE_SOFT;
    } else if wget_strcasecmp_ascii(val, b"none\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *((*opt).var as *mut https_enforce_mode) = HTTPS_ENFORCE_NONE;
    } else if *val.offset(0 as libc::c_int as isize) == 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Missing required type specifier\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid type specifier: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_compression(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    let mut v: *mut wget_vector = *((*opt).var as *mut *mut wget_vector);
    if val.is_null() && invert as libc::c_int != 0 {
        if !v.is_null() {
            wget_vector_free((*opt).var as *mut *mut wget_vector);
            config
                .compression_methods[wget_content_encoding_max as libc::c_int
                as usize] = wget_content_encoding_identity;
        }
        config.no_compression = 1 as libc::c_int != 0;
        return 0 as libc::c_int;
    } else if !val.is_null() && invert == 0 {
        let mut rc: libc::c_int = 0;
        if !v.is_null() {
            wget_vector_free((*opt).var as *mut *mut wget_vector);
            config
                .compression_methods[wget_content_encoding_max as libc::c_int
                as usize] = wget_content_encoding_identity;
        }
        rc = parse_stringlist_expand(opt, val, 0 as libc::c_int, 16 as libc::c_int);
        if rc != 0 {
            return rc;
        }
        v = *((*opt).var as *mut *mut wget_vector);
        config.no_compression = 0 as libc::c_int != 0;
        let mut methods_bits: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < wget_vector_size(v) {
            let mut not_built: libc::c_int = 0 as libc::c_int;
            let mut name: *const libc::c_char = wget_vector_get(v, it)
                as *const libc::c_char;
            let mut type_0: wget_content_encoding = wget_content_encoding_by_name(name);
            if type_0 as libc::c_int == wget_content_encoding_unknown as libc::c_int {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Compression type '%s' not supported\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                return -(1 as libc::c_int);
            } else if methods_bits
                & ((1 as libc::c_int) << type_0 as libc::c_int) as libc::c_longlong != 0
            {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Duplicate type '%s'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                return -(1 as libc::c_int);
            }
            if type_0 as libc::c_int == wget_content_encoding_bzip2 as libc::c_int {
                not_built = 1 as libc::c_int;
            }
            if type_0 as libc::c_int == wget_content_encoding_xz as libc::c_int
                || type_0 as libc::c_int == wget_content_encoding_lzma as libc::c_int
            {
                not_built = 1 as libc::c_int;
            }
            if type_0 as libc::c_int == wget_content_encoding_brotli as libc::c_int {
                not_built = 1 as libc::c_int;
            }
            if type_0 as libc::c_int == wget_content_encoding_lzip as libc::c_int {
                not_built = 1 as libc::c_int;
            }
            if not_built != 0 {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Lib for type %s not built\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    wget_content_encoding_to_name(type_0),
                );
                return -(1 as libc::c_int);
            }
            methods_bits
                |= ((1 as libc::c_int) << type_0 as libc::c_int) as libc::c_longlong;
            let fresh15 = config
                .compression_methods[wget_content_encoding_max as libc::c_int as usize];
            config
                .compression_methods[wget_content_encoding_max as libc::c_int
                as usize] = config
                .compression_methods[wget_content_encoding_max as libc::c_int as usize]
                + 1;
            config.compression_methods[fresh15 as usize] = type_0;
            it += 1;
            it;
        }
        return 0 as libc::c_int;
    } else if val.is_null() && invert == 0 {
        config.no_compression = 0 as libc::c_int != 0;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn parse_download_attr(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if val.is_null() && invert as libc::c_int != 0 {
        config.download_attr = DOWNLOAD_ATTR_NO as libc::c_int as libc::c_char;
        return 0 as libc::c_int;
    }
    if val.is_null() {
        config.download_attr = DOWNLOAD_ATTR_STRIPPATH as libc::c_int as libc::c_char;
        return 0 as libc::c_int;
    }
    if invert != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Disallowed Value for --no-download-attr: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    if strcasecmp(val, b"strippath\0" as *const u8 as *const libc::c_char) == 0 {
        config.download_attr = DOWNLOAD_ATTR_STRIPPATH as libc::c_int as libc::c_char;
    } else if strcasecmp(val, b"usepath\0" as *const u8 as *const libc::c_char) == 0 {
        config.download_attr = DOWNLOAD_ATTR_USEPATH as libc::c_int as libc::c_char;
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Invalid value for --download-attr: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            val,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn list_plugins(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if plugin_loading_enabled == 0 {
        return 0 as libc::c_int;
    }
    let mut v: *mut wget_vector = wget_vector_create(16 as libc::c_int, None);
    plugin_db_list(v);
    let mut n_names: libc::c_int = wget_vector_size(v);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_names {
        let mut name: *const libc::c_char = wget_vector_get(v, i) as *const libc::c_char;
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, name);
        i += 1;
        i;
    }
    wget_vector_free(&mut v);
    set_exit_status(EXIT_STATUS_NO_ERROR);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn print_plugin_help(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if plugin_loading_enabled == 0 {
        return 0 as libc::c_int;
    }
    plugin_db_show_help();
    set_exit_status(EXIT_STATUS_NO_ERROR);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub static mut config: config = {
    let mut init = config {
        base: 0 as *const wget_iri as *mut wget_iri,
        post_file: 0 as *const libc::c_char,
        post_data: 0 as *const libc::c_char,
        body_file: 0 as *const libc::c_char,
        body_data: 0 as *const libc::c_char,
        http_username: 0 as *const libc::c_char,
        http_password: 0 as *const libc::c_char,
        http_proxy_username: 0 as *const libc::c_char,
        http_proxy_password: 0 as *const libc::c_char,
        input_encoding: 0 as *const libc::c_char,
        local_encoding: 0 as *const libc::c_char,
        remote_encoding: 0 as *const libc::c_char,
        bind_address: 0 as *const libc::c_char,
        bind_interface: 0 as *const libc::c_char,
        input_file: 0 as *const libc::c_char,
        base_url: 0 as *const libc::c_char,
        default_page: b"index.html\0" as *const u8 as *const libc::c_char,
        referer: 0 as *const libc::c_char,
        directory_prefix: 0 as *const libc::c_char,
        http_proxy: 0 as *const libc::c_char,
        https_proxy: 0 as *const libc::c_char,
        no_proxy: 0 as *const libc::c_char,
        cookie_suffixes: 0 as *const libc::c_char,
        load_cookies: 0 as *const libc::c_char,
        save_cookies: 0 as *const libc::c_char,
        logfile: 0 as *const libc::c_char,
        logfile_append: 0 as *const libc::c_char,
        user_agent: b"Wget/2.2.0\0" as *const u8 as *const libc::c_char,
        output_document: 0 as *const libc::c_char,
        ca_cert: b"system\0" as *const u8 as *const libc::c_char,
        ca_directory: b"system\0" as *const u8 as *const libc::c_char,
        cert_file: 0 as *const libc::c_char,
        crl_file: 0 as *const libc::c_char,
        egd_file: 0 as *const libc::c_char,
        private_key: 0 as *const libc::c_char,
        random_file: 0 as *const libc::c_char,
        secure_protocol: b"AUTO\0" as *const u8 as *const libc::c_char,
        accept_regex: 0 as *const libc::c_char,
        reject_regex: 0 as *const libc::c_char,
        gnupg_homedir: 0 as *const libc::c_char,
        stats_all: 0 as *const libc::c_char,
        system_config: b"/usr/local/etc/wget2rc\0" as *const u8 as *const libc::c_char,
        user_config: 0 as *const libc::c_char,
        hsts_file: 0 as *const libc::c_char,
        hsts_preload_file: 0 as *const libc::c_char,
        hpkp_file: 0 as *const libc::c_char,
        tls_session_file: 0 as *const libc::c_char,
        ocsp_server: 0 as *const libc::c_char,
        ocsp_file: 0 as *const libc::c_char,
        netrc_file: 0 as *const libc::c_char,
        use_askpass_bin: 0 as *const libc::c_char,
        hostname: 0 as *const libc::c_char,
        dns_cache_preload: 0 as *const libc::c_char,
        method: 0 as *const libc::c_char,
        compression: 0 as *const wget_vector as *mut wget_vector,
        domains: 0 as *const wget_vector as *mut wget_vector,
        exclude_directories: 0 as *const wget_vector as *mut wget_vector,
        exclude_domains: 0 as *const wget_vector as *mut wget_vector,
        accept_patterns: 0 as *const wget_vector as *mut wget_vector,
        reject_patterns: 0 as *const wget_vector as *mut wget_vector,
        follow_tags: 0 as *const wget_vector as *mut wget_vector,
        ignore_tags: 0 as *const wget_vector as *mut wget_vector,
        default_challenges: 0 as *const wget_vector as *mut wget_vector,
        headers: 0 as *const wget_vector as *mut wget_vector,
        mime_types: 0 as *const wget_vector as *mut wget_vector,
        retry_on_http_error: 0 as *const wget_vector as *mut wget_vector,
        save_content_on: 0 as *const wget_vector as *mut wget_vector,
        compression_methods: [wget_content_encoding_identity; 10],
        hsts_db: 0 as *const wget_hsts_db as *mut wget_hsts_db,
        hpkp_db: 0 as *const wget_hpkp_db as *mut wget_hpkp_db,
        tls_session_db: 0 as *const wget_tls_session_db as *mut wget_tls_session_db,
        ocsp_db: 0 as *const wget_ocsp_db as *mut wget_ocsp_db,
        netrc_db: 0 as *const wget_netrc_db as *mut wget_netrc_db,
        cookie_db: 0 as *const wget_cookie_db as *mut wget_cookie_db,
        stats_dns_args: 0 as *const stats_args as *mut stats_args,
        stats_ocsp_args: 0 as *const stats_args as *mut stats_args,
        stats_server_args: 0 as *const stats_args as *mut stats_args,
        stats_site_args: 0 as *const stats_args as *mut stats_args,
        stats_tls_args: 0 as *const stats_args as *mut stats_args,
        password: 0 as *const libc::c_char as *mut libc::c_char,
        username: 0 as *const libc::c_char as *mut libc::c_char,
        chunk_size: 0,
        quota: 0,
        limit_rate: 0,
        start_pos: 0,
        http2_request_window: 0,
        backups: 0,
        tries: 20 as libc::c_int,
        wait: 0,
        waitretry: 10 as libc::c_int * 1000 as libc::c_int,
        restrict_file_names: 0,
        level: 5 as libc::c_int,
        preferred_family: 0,
        cut_directories: 0,
        connect_timeout: -(1 as libc::c_int),
        dns_timeout: -(1 as libc::c_int),
        read_timeout: 900 as libc::c_int * 1000 as libc::c_int,
        max_redirect: 20 as libc::c_int,
        max_threads: 5 as libc::c_int,
        default_http_port: 80 as libc::c_int as uint16_t,
        default_https_port: 443 as libc::c_int as uint16_t,
        report_speed: WGET_REPORT_SPEED_BYTES,
        check_certificate: CHECK_CERTIFICATE_ENABLED,
        https_enforce: HTTPS_ENFORCE_NONE,
        verify_sig: GPG_VERIFY_DISABLED,
        cert_type: 0 as libc::c_int as libc::c_char,
        private_key_type: 0 as libc::c_int as libc::c_char,
        progress: 1 as libc::c_int as libc::c_char,
        regex_type: 0,
        download_attr: 0,
        tls_resume: false,
        content_on_error: false,
        fsync_policy: false,
        netrc: 1 as libc::c_int != 0,
        http2: false,
        http2_only: false,
        ocsp_stapling: 1 as libc::c_int != 0,
        ocsp: 0 as libc::c_int != 0,
        mirror: false,
        backup_converted: false,
        convert_file_only: false,
        convert_links: false,
        ignore_case: false,
        ignore_length: false,
        hsts: 1 as libc::c_int != 0,
        hsts_preload: 1 as libc::c_int != 0,
        hpkp: 1 as libc::c_int != 0,
        random_wait: false,
        trust_server_names: false,
        robots: 1 as libc::c_int != 0,
        parent: 1 as libc::c_int != 0,
        https_only: false,
        content_disposition: false,
        page_requisites: false,
        follow_sitemaps: 1 as libc::c_int != 0,
        force_rss: false,
        force_atom: false,
        force_sitemap: false,
        force_css: false,
        force_html: false,
        force_metalink: false,
        adjust_extension: false,
        save_headers: false,
        clobber: 1 as libc::c_int != 0,
        cache: 1 as libc::c_int != 0,
        inet4_only: false,
        inet6_only: false,
        delete_after: false,
        strict_comments: false,
        protocol_directories: false,
        host_directories: 1 as libc::c_int != 0,
        force_directories: false,
        directories: 1 as libc::c_int != 0,
        timestamping: false,
        use_server_timestamps: 1 as libc::c_int != 0,
        continue_download: false,
        server_response: false,
        keep_alive: 1 as libc::c_int != 0,
        keep_extension: false,
        keep_session_cookies: false,
        cookies: 1 as libc::c_int != 0,
        spider: false,
        dns_caching: 1 as libc::c_int != 0,
        check_hostname: 1 as libc::c_int != 0,
        span_hosts: false,
        verbose: 1 as libc::c_int != 0,
        quiet: false,
        debug: false,
        hyperlink: 0 as libc::c_int != 0,
        metalink: 1 as libc::c_int != 0,
        cut_url_get_vars: false,
        cut_file_get_vars: false,
        proxy: 1 as libc::c_int != 0,
        xattr: false,
        force_progress: false,
        local_db: 1 as libc::c_int != 0,
        dont_write: false,
        filter_urls: false,
        askpass: false,
        verify_save_failed: false,
        retry_connrefused: false,
        unlink: false,
        background: false,
        if_modified_since: 1 as libc::c_int != 0,
        auth_no_challenge: 0 as libc::c_int != 0,
        no_compression: false,
        ocsp_date: 1 as libc::c_int != 0,
        ocsp_nonce: 1 as libc::c_int != 0,
        recursive: false,
        tls_false_start: 1 as libc::c_int != 0,
        tcp_fastopen: false,
        dane: false,
    };
    init
};
static mut options: [optionw; 189] = [optionw {
    long_name: [0; 22],
    var: 0 as *mut libc::c_void,
    parser: None,
    args: 0,
    short_name: 0,
    section: SECTION_STARTUP,
    help_str: [0 as *const libc::c_char; 4],
}; 189];
#[inline]
unsafe extern "C" fn print_first(
    s: libc::c_char,
    mut l: *const libc::c_char,
    mut msg: *const libc::c_char,
) {
    if strlen(l) > 18 as libc::c_int as libc::c_ulong {
        printf(
            b"  %c%-2c --%s\n\0" as *const u8 as *const libc::c_char,
            if s as libc::c_int != 0 { '-' as i32 } else { ' ' as i32 },
            if s as libc::c_int != 0 { s as libc::c_int } else { ' ' as i32 },
            l,
        );
        printf(
            b"%28s%s\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            msg,
        );
    } else {
        printf(
            b"  %c%-2c --%-18.18s  %s\0" as *const u8 as *const libc::c_char,
            if s as libc::c_int != 0 { '-' as i32 } else { ' ' as i32 },
            if s as libc::c_int != 0 { s as libc::c_int } else { ' ' as i32 },
            l,
            msg,
        );
    };
}
#[inline]
unsafe extern "C" fn print_next(mut msg: *const libc::c_char) {
    printf(
        b"%30s%s\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        msg,
    );
}
unsafe extern "C" fn print_help(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    printf(
        b"GNU Wget2 V2.2.0 - multithreaded metalink/file/website downloader\n\nUsage: wget [options...] <url>...\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    let mut sect: help_section_t = SECTION_STARTUP;
    while (sect as libc::c_uint) < SECTION_END as libc::c_int as libc::c_uint {
        match sect as libc::c_uint {
            0 => {
                printf(b"Startup:\n\0" as *const u8 as *const libc::c_char);
            }
            1 => {
                printf(b"Download:\n\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                printf(b"HTTP related options:\n\0" as *const u8 as *const libc::c_char);
            }
            3 => {
                printf(
                    b"HTTPS (SSL/TLS) related options:\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            4 => {
                printf(b"Directory options:\n\0" as *const u8 as *const libc::c_char);
            }
            5 => {
                printf(b"Plugin options:\n\0" as *const u8 as *const libc::c_char);
            }
            6 | _ => {
                printf(
                    b"Unknown help section %d\n\0" as *const u8 as *const libc::c_char,
                    sect as libc::c_int,
                );
                exit(1 as libc::c_int);
            }
        }
        let mut it: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (it as libc::c_ulong)
            < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
        {
            if options[it as usize].section as libc::c_uint == sect as libc::c_uint {
                if !(options[it as usize].short_name as libc::c_int == 'n' as i32) {
                    print_first(
                        options[it as usize].short_name,
                        (options[it as usize].long_name).as_ptr(),
                        options[it as usize].help_str[0 as libc::c_int as usize],
                    );
                    let mut i: libc::c_uint = 1 as libc::c_int as libc::c_uint;
                    while (i as libc::c_ulong)
                        < (::core::mem::size_of::<[*const libc::c_char; 4]>()
                            as libc::c_ulong)
                            .wrapping_div(
                                ::core::mem::size_of::<*const libc::c_char>()
                                    as libc::c_ulong,
                            ) && !(options[it as usize].help_str[i as usize]).is_null()
                    {
                        print_next(options[it as usize].help_str[i as usize]);
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
            it = it.wrapping_add(1);
            it;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        sect += 1;
        sect;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Example boolean option:\n --quiet=no is the same as --no-quiet or --quiet=off or --quiet off\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Example string option:\n --user-agent=SpecialAgent/1.3.5 or --user-agent \"SpecialAgent/1.3.5\"\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"To reset string options use --[no-]option\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    set_exit_status(EXIT_STATUS_NO_ERROR);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn opt_compare(
    mut key: *const libc::c_void,
    mut option: *const libc::c_void,
) -> libc::c_int {
    return strcmp(
        key as *const libc::c_char,
        ((*(option as option_t)).long_name).as_ptr(),
    );
}
unsafe extern "C" fn opt_compare_config(
    mut key: *const libc::c_void,
    mut option: *const libc::c_void,
) -> libc::c_int {
    return wget_strcasecmp_ascii(
        key as *const libc::c_char,
        ((*(option as option_t)).long_name).as_ptr(),
    );
}
unsafe extern "C" fn opt_compare_config_linear(
    mut key: *const libc::c_char,
    mut command: *const libc::c_char,
) -> libc::c_int {
    let mut s1: *const libc::c_char = key;
    let mut s2: *const libc::c_char = command;
    while *s1 as libc::c_int != 0 && *s2 as libc::c_int != 0 {
        if *s2 as libc::c_int == '-' as i32 || *s2 as libc::c_int == '_' as i32 {
            if *s1 as libc::c_int == '-' as i32 || *s1 as libc::c_int == '_' as i32 {
                s1 = s1.offset(1);
                s1;
            }
            s2 = s2.offset(1);
            s2;
        }
        if *s1 == 0 || *s2 == 0 || c_tolower(*s1 as libc::c_int) != *s2 as libc::c_int {
            break;
        }
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    return (*s1 as libc::c_int != *s2 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn set_long_option(
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut parsing_config: libc::c_char,
) -> libc::c_int {
    let mut opt: option_t = 0 as *const optionw;
    let mut invert: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut value_present: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut case_insensitive: libc::c_char = 1 as libc::c_int as libc::c_char;
    let mut namebuf: [libc::c_char; 27] = [0; 27];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    p = strchr(name, '=' as i32);
    if !p.is_null() {
        if p.offset_from(name) as libc::c_long
            >= ::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong
                as libc::c_int as libc::c_long
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown option '%s'\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            return -(1 as libc::c_int);
        }
        memcpy(
            namebuf.as_mut_ptr() as *mut libc::c_void,
            name as *const libc::c_void,
            p.offset_from(name) as libc::c_long as libc::c_ulong,
        );
        namebuf[p.offset_from(name) as libc::c_long
            as usize] = 0 as libc::c_int as libc::c_char;
        name = namebuf.as_mut_ptr();
        value = p.offset(1 as libc::c_int as isize);
        value_present = 1 as libc::c_int as libc::c_char;
    }
    if strncmp(
        name,
        b"--\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        case_insensitive = 0 as libc::c_int as libc::c_char;
        name = name.offset(2 as libc::c_int as isize);
    }
    if strncmp(
        name,
        b"no-\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        invert = 1 as libc::c_int as libc::c_char;
        name = name.offset(3 as libc::c_int as isize);
    }
    if parsing_config as libc::c_int != 0 && case_insensitive as libc::c_int != 0 {
        opt = bsearch(
            name as *const libc::c_void,
            options.as_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
            ::core::mem::size_of::<optionw>() as libc::c_ulong,
            Some(
                opt_compare_config
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as option_t;
        if opt.is_null() {
            let mut it: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while (it as libc::c_ulong)
                < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
                && opt.is_null()
            {
                if opt_compare_config_linear(
                    name,
                    (options[it as usize].long_name).as_ptr(),
                ) == 0 as libc::c_int
                {
                    opt = &*options.as_ptr().offset(it as isize) as *const optionw;
                }
                it = it.wrapping_add(1);
                it;
            }
        }
    } else {
        opt = bsearch(
            name as *const libc::c_void,
            options.as_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
            ::core::mem::size_of::<optionw>() as libc::c_ulong,
            Some(
                opt_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as option_t;
    }
    if opt.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unknown option '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return -(1 as libc::c_int);
    }
    if value_present != 0 {
        if invert != 0 {
            if (*opt).args == 0
                || (*opt).parser
                    == Some(
                        parse_string
                            as unsafe extern "C" fn(
                                option_t,
                                *const libc::c_char,
                                libc::c_char,
                            ) -> libc::c_int,
                    )
                || (*opt).parser
                    == Some(
                        parse_stringset
                            as unsafe extern "C" fn(
                                option_t,
                                *const libc::c_char,
                                libc::c_char,
                            ) -> libc::c_int,
                    )
                || (*opt).parser
                    == Some(
                        parse_stringlist
                            as unsafe extern "C" fn(
                                option_t,
                                *const libc::c_char,
                                libc::c_char,
                            ) -> libc::c_int,
                    )
                || (*opt).parser
                    == Some(
                        parse_filename
                            as unsafe extern "C" fn(
                                option_t,
                                *const libc::c_char,
                                libc::c_char,
                            ) -> libc::c_int,
                    )
                || (*opt).parser
                    == Some(
                        parse_filenames
                            as unsafe extern "C" fn(
                                option_t,
                                *const libc::c_char,
                                libc::c_char,
                            ) -> libc::c_int,
                    )
                || (*opt).parser
                    == Some(
                        parse_stats
                            as unsafe extern "C" fn(
                                option_t,
                                *const libc::c_char,
                                libc::c_char,
                            ) -> libc::c_int,
                    )
            {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Option 'no-%s' doesn't allow an argument\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                return -(1 as libc::c_int);
            }
        } else if (*opt).args == 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Option '%s' doesn't allow an argument\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                name,
            );
            return -(1 as libc::c_int);
        }
    } else {
        match (*opt).args {
            0 => {
                value = 0 as *const libc::c_char;
            }
            1 => {
                if value.is_null() {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Missing argument for option '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                    return -(1 as libc::c_int);
                }
                if invert as libc::c_int != 0
                    && ((*opt).parser
                        == Some(
                            parse_string
                                as unsafe extern "C" fn(
                                    option_t,
                                    *const libc::c_char,
                                    libc::c_char,
                                ) -> libc::c_int,
                        )
                        || (*opt).parser
                            == Some(
                                parse_stringset
                                    as unsafe extern "C" fn(
                                        option_t,
                                        *const libc::c_char,
                                        libc::c_char,
                                    ) -> libc::c_int,
                            )
                        || (*opt).parser
                            == Some(
                                parse_stringlist
                                    as unsafe extern "C" fn(
                                        option_t,
                                        *const libc::c_char,
                                        libc::c_char,
                                    ) -> libc::c_int,
                            )
                        || (*opt).parser
                            == Some(
                                parse_filename
                                    as unsafe extern "C" fn(
                                        option_t,
                                        *const libc::c_char,
                                        libc::c_char,
                                    ) -> libc::c_int,
                            )
                        || (*opt).parser
                            == Some(
                                parse_filenames
                                    as unsafe extern "C" fn(
                                        option_t,
                                        *const libc::c_char,
                                        libc::c_char,
                                    ) -> libc::c_int,
                            )
                        || (*opt).parser
                            == Some(
                                parse_stats
                                    as unsafe extern "C" fn(
                                        option_t,
                                        *const libc::c_char,
                                        libc::c_char,
                                    ) -> libc::c_int,
                            ))
                {
                    value = 0 as *const libc::c_char;
                } else {
                    ret = (*opt).args;
                }
            }
            -1 => {
                if parsing_config == 0 {
                    value = 0 as *const libc::c_char;
                } else if !value.is_null() {
                    ret = 1 as libc::c_int;
                }
            }
            _ => {}
        }
    }
    rc = ((*opt).parser).expect("non-null function pointer")(opt, value, invert);
    if rc < 0 as libc::c_int {
        return rc;
    }
    return ret;
}
unsafe extern "C" fn parse_proxy(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    if parse_bool(opt, val, invert) < 0 as libc::c_int {
        if invert != 0 {
            if !(config.no_proxy).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(config.no_proxy as *mut libc::c_void);
                config.no_proxy = 0 as *const libc::c_char;
            }
            config
                .no_proxy = if !val.is_null() {
                wget_strdup(val)
            } else {
                0 as *mut libc::c_char
            };
        } else {
            opt = bsearch(
                b"http-proxy\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                options.as_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
                ::core::mem::size_of::<optionw>() as libc::c_ulong,
                Some(
                    opt_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            ) as option_t;
            if !opt.is_null() {
                return parse_string(opt, val, invert);
            }
            opt = bsearch(
                b"https-proxy\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                options.as_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
                ::core::mem::size_of::<optionw>() as libc::c_ulong,
                Some(
                    opt_compare
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            ) as option_t;
            if !opt.is_null() {
                return parse_string(opt, val, invert);
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parse_execute(
    mut opt: option_t,
    mut val: *const libc::c_char,
    invert: libc::c_char,
) -> libc::c_int {
    return set_long_option(
        val,
        0 as *const libc::c_char,
        1 as libc::c_int as libc::c_char,
    );
}
unsafe extern "C" fn parse_option(
    mut linep: *mut libc::c_char,
    mut name: *mut *mut libc::c_char,
    mut val: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut quote: libc::c_int = 0;
    while c_isspace(*linep as libc::c_int) {
        linep = linep.offset(1);
        linep;
    }
    *name = linep;
    while c_isalnum(*linep as libc::c_int) as libc::c_int != 0
        || *linep as libc::c_int == '-' as i32 || *linep as libc::c_int == '_' as i32
    {
        linep = linep.offset(1);
        linep;
    }
    if **name == 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to parse: '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            linep,
        );
        return 0 as libc::c_int;
    }
    if c_isspace(*linep as libc::c_int) {
        let fresh16 = linep;
        linep = linep.offset(1);
        *fresh16 = 0 as libc::c_int as libc::c_char;
        while c_isspace(*linep as libc::c_int) {
            linep = linep.offset(1);
            linep;
        }
    }
    if *linep as libc::c_int == '=' as i32 {
        let fresh17 = linep;
        linep = linep.offset(1);
        *fresh17 = 0 as libc::c_int as libc::c_char;
        while c_isspace(*linep as libc::c_int) {
            linep = linep.offset(1);
            linep;
        }
        *val = linep;
        quote = *linep as libc::c_int;
        if quote == '"' as i32 || quote == '\'' as i32 {
            let mut src: *mut libc::c_char = linep.offset(1 as libc::c_int as isize);
            let mut dst: *mut libc::c_char = linep;
            let mut c: libc::c_char = 0;
            loop {
                c = *src;
                if !(c as libc::c_int != quote && c as libc::c_int != 0) {
                    break;
                }
                if c as libc::c_int == '\\' as i32
                    && *src.offset(1 as libc::c_int as isize) as libc::c_int != 0
                {
                    let fresh18 = dst;
                    dst = dst.offset(1);
                    *fresh18 = *src.offset(1 as libc::c_int as isize);
                    src = src.offset(2 as libc::c_int as isize);
                } else {
                    let fresh19 = src;
                    src = src.offset(1);
                    let fresh20 = dst;
                    dst = dst.offset(1);
                    *fresh20 = *fresh19;
                }
            }
            *dst = 0 as libc::c_int as libc::c_char;
        }
        return 1 as libc::c_int;
    } else {
        if *linep != 0 {
            let fresh21 = linep;
            linep = linep.offset(1);
            *fresh21 = 0 as libc::c_int as libc::c_char;
        }
        while c_isspace(*linep as libc::c_int) {
            linep = linep.offset(1);
            linep;
        }
        *val = linep;
        return 2 as libc::c_int;
    };
}
unsafe extern "C" fn read_config_expand(
    mut cfgfile: *const libc::c_char,
    mut expand: libc::c_int,
) -> libc::c_int {
    static mut level: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut append: libc::c_int = 0 as libc::c_int;
    let mut found: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut rc: libc::c_int = 0;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut len: ssize_t = 0;
    let mut linebuf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    level += 1;
    if level > 20 as libc::c_int {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Config file recursion detected in %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            cfgfile,
        );
        level -= 1;
        level;
        return -(2 as libc::c_int);
    }
    if expand != 0 {
        let mut globbuf: glob_t = {
            let mut init = glob_t {
                gl_pathc: 0 as libc::c_int as __size_t,
                gl_pathv: 0 as *mut *mut libc::c_char,
                gl_offs: 0,
                gl_flags: 0,
                gl_closedir: None,
                gl_readdir: None,
                gl_opendir: None,
                gl_lstat: None,
                gl_stat: None,
            };
            init
        };
        if glob(
            cfgfile,
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 12 as libc::c_int,
            None,
            &mut globbuf,
        ) == 0 as libc::c_int
        {
            let mut it: size_t = 0;
            it = 0 as libc::c_int as size_t;
            while it < globbuf.gl_pathc && ret == 0 as libc::c_int {
                if *(*(globbuf.gl_pathv).offset(it as isize))
                    .offset(
                        (strlen(*(globbuf.gl_pathv).offset(it as isize)))
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                    ) as libc::c_int != '/' as i32
                {
                    ret = read_config_expand(
                        *(globbuf.gl_pathv).offset(it as isize),
                        0 as libc::c_int,
                    );
                }
                it = it.wrapping_add(1);
                it;
            }
            globfree(&mut globbuf);
        } else {
            ret = read_config_expand(cfgfile, 0 as libc::c_int);
        }
        level -= 1;
        level;
        return ret;
    }
    fp = rpl_fopen(cfgfile, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to open %s (%d): %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            cfgfile,
            *__errno_location(),
            strerror(*__errno_location()),
        );
        level -= 1;
        level;
        return -(1 as libc::c_int);
    }
    wget_debug_printf(b"Reading %s\n\0" as *const u8 as *const libc::c_char, cfgfile);
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    wget_buffer_init(
        &mut linebuf,
        tmp.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    while ret == 0 as libc::c_int
        && {
            len = wget_getline(&mut buf, &mut bufsize, fp);
            len >= 0 as libc::c_int as ssize_t
        }
    {
        if len == 0 as libc::c_int as ssize_t || *buf as libc::c_int == '\r' as i32
            || *buf as libc::c_int == '\n' as i32
        {
            continue;
        }
        linep = buf;
        if append == 0 {
            while c_isspace(*linep as libc::c_int) {
                linep = linep.offset(1);
                linep;
                len -= 1;
                len;
            }
        }
        if *linep as libc::c_int == '#' as i32 {
            continue;
        }
        while len > 0 as libc::c_int as ssize_t
            && c_isspace(
                *linep.offset((len - 1 as libc::c_int as ssize_t) as isize)
                    as libc::c_int,
            ) as libc::c_int != 0
        {
            len -= 1;
            len;
        }
        *linep.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        if len > 0 as libc::c_int as ssize_t
            && *linep.offset((len - 1 as libc::c_int as ssize_t) as isize) as libc::c_int
                == '\\' as i32
        {
            if append != 0 {
                wget_buffer_memcat(
                    &mut linebuf,
                    linep as *const libc::c_void,
                    (len - 1 as libc::c_int as ssize_t) as size_t,
                );
            } else {
                wget_buffer_memcpy(
                    &mut linebuf,
                    linep as *const libc::c_void,
                    (len - 1 as libc::c_int as ssize_t) as size_t,
                );
                append = 1 as libc::c_int;
            }
        } else {
            if append != 0 {
                wget_buffer_strcat(&mut linebuf, linep);
                append = 0 as libc::c_int;
                linep = linebuf.data;
            }
            found = parse_option(linep, &mut name, &mut val);
            if found == 1 as libc::c_int {
                rc = set_long_option(name, val, 1 as libc::c_int as libc::c_char);
                if rc < 0 as libc::c_int {
                    ret = rc;
                }
            } else if found == 2 as libc::c_int {
                if strcmp(name, b"include\0" as *const u8 as *const libc::c_char) == 0 {
                    ret = read_config_expand(val, 1 as libc::c_int);
                } else {
                    rc = set_long_option(
                        name,
                        0 as *const libc::c_char,
                        0 as libc::c_int as libc::c_char,
                    );
                    if rc < 0 as libc::c_int {
                        ret = rc;
                    }
                }
            }
        }
    }
    wget_buffer_deinit(&mut linebuf);
    if !buf.is_null() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    rpl_fclose(fp);
    if append != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to parse last line in '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            cfgfile,
        );
        ret = -(4 as libc::c_int);
    }
    level -= 1;
    level;
    return ret;
}
unsafe extern "C" fn read_config() -> bool {
    let mut ret: bool = 1 as libc::c_int != 0;
    if !(config.system_config).is_null() {
        ret = read_config_expand(config.system_config, 1 as libc::c_int) != 0;
    }
    if !(config.user_config).is_null() {
        ret = (ret as libc::c_int
            & read_config_expand(config.user_config, 1 as libc::c_int)) as bool;
    }
    return ret;
}
unsafe extern "C" fn parse_command_line(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    static mut shortcut_to_option: [libc::c_short; 128] = [0; 128];
    let mut first_arg: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    if shortcut_to_option[0 as libc::c_int as usize] == 0 {
        let mut it: libc::c_short = 0 as libc::c_int as libc::c_short;
        while (it as libc::c_int)
            < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
                as libc::c_short as libc::c_int
        {
            if options[it as usize].short_name as libc::c_int > 0 as libc::c_int {
                shortcut_to_option[options[it as usize].short_name as libc::c_uchar
                    as usize] = (it as libc::c_int + 1 as libc::c_int) as libc::c_short;
            }
            it += 1;
            it;
        }
    }
    n = 1 as libc::c_int;
    while n < argc && first_arg != *argv.offset(n as isize) {
        let mut argp: *const libc::c_char = *argv.offset(n as isize);
        if *argp.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
            let mut cur: *const libc::c_char = *argv.offset(n as isize);
            let mut it_0: libc::c_int = n;
            while it_0 < argc - 1 as libc::c_int {
                let ref mut fresh22 = *argv.offset(it_0 as isize);
                *fresh22 = *argv.offset((it_0 + 1 as libc::c_int) as isize);
                it_0 += 1;
                it_0;
            }
            let ref mut fresh23 = *argv.offset((argc - 1 as libc::c_int) as isize);
            *fresh23 = cur;
            if first_arg.is_null() {
                first_arg = cur;
            }
            n -= 1;
            n;
        } else if *argp.offset(1 as libc::c_int as isize) as libc::c_int == '-' as i32 {
            if *argp.offset(2 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
            {
                return n + 1 as libc::c_int;
            }
            rc = set_long_option(
                argp.offset(2 as libc::c_int as isize),
                (if n < argc - 1 as libc::c_int {
                    *argv.offset((n + 1 as libc::c_int) as isize)
                } else {
                    0 as *const libc::c_char
                }),
                0 as libc::c_int as libc::c_char,
            );
            if rc < 0 as libc::c_int {
                return rc;
            }
            n += rc;
        } else if *argp.offset(1 as libc::c_int as isize) != 0 {
            let mut pos: libc::c_int = 1 as libc::c_int;
            while *argp.offset(pos as isize) != 0 {
                let mut opt: option_t = 0 as *const optionw;
                let mut idx: libc::c_int = 0;
                if c_isalnum(*argp.offset(pos as isize) as libc::c_int) as libc::c_int
                    != 0
                    && {
                        idx = shortcut_to_option[*argp.offset(pos as isize)
                            as libc::c_uchar as usize] as libc::c_int;
                        idx != 0
                    }
                {
                    opt = &*options.as_ptr().offset((idx - 1 as libc::c_int) as isize)
                        as *const optionw;
                    if (*opt).args > 0 as libc::c_int {
                        let mut val: *const libc::c_char = 0 as *const libc::c_char;
                        if *argp.offset((pos + 1 as libc::c_int) as isize) == 0
                            && argc <= n + (*opt).args
                        {
                            wget_error_printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Missing argument(s) for option '-%c'\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                *argp.offset(pos as isize) as libc::c_int,
                            );
                            return -(1 as libc::c_int);
                        }
                        val = if *argp.offset((pos + 1 as libc::c_int) as isize)
                            as libc::c_int != 0
                        {
                            argp.offset(pos as isize).offset(1 as libc::c_int as isize)
                        } else {
                            n += 1;
                            *argv.offset(n as isize)
                        };
                        rc = ((*opt).parser)
                            .expect(
                                "non-null function pointer",
                            )(opt, val, 0 as libc::c_int as libc::c_char);
                        if rc < 0 as libc::c_int {
                            return rc;
                        }
                        n += rc;
                        break;
                    } else {
                        rc = ((*opt).parser)
                            .expect(
                                "non-null function pointer",
                            )(
                            opt,
                            0 as *const libc::c_char,
                            0 as libc::c_int as libc::c_char,
                        );
                        if rc < 0 as libc::c_int {
                            return rc;
                        }
                        pos += 1;
                        pos;
                    }
                } else {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Unknown option '-%c'\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argp.offset(pos as isize) as libc::c_int,
                    );
                    return -(1 as libc::c_int);
                }
            }
        }
        n += 1;
        n;
    }
    return n;
}
unsafe extern "C" fn get_home_dir(mut free_home: bool) -> *const libc::c_char {
    static mut home: *const libc::c_char = 0 as *const libc::c_char;
    if free_home {
        if !home.is_null() {
            wget_free.expect("non-null function pointer")(home as *mut libc::c_void);
            home = 0 as *const libc::c_char;
        }
        return 0 as *const libc::c_char;
    }
    if home.is_null() {
        home = wget_strnglob(
            b"~\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            (1 as libc::c_int) << 14 as libc::c_int,
        );
        if home.is_null() {
            home = wget_strdup(b".\0" as *const u8 as *const libc::c_char);
        }
    }
    return home;
}
unsafe extern "C" fn prompt_for_password() -> *mut libc::c_char {
    if !(config.username).is_null() {
        wget_fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Password for user \"%s\": \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            config.username,
        );
    } else {
        wget_fprintf(
            stderr,
            dcgettext(
                0 as *const libc::c_char,
                b"Password: \0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !is_testing() {
        return getpass(b"\0" as *const u8 as *const libc::c_char)
    } else {
        return wget_strdup(b"xxx\0" as *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn run_use_askpass(
    mut question: *const libc::c_char,
    mut answer: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tmp: [libc::c_char; 1024] = [0; 1024];
    let mut pid: pid_t = 0;
    let mut rc: libc::c_int = 0;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut com: [libc::c_int; 2] = [0; 2];
    let mut fa_valid: bool = 0 as libc::c_int != 0;
    let mut bytes: ssize_t = 0 as libc::c_int as ssize_t;
    let argv: [*const libc::c_char; 3] = [
        config.use_askpass_bin,
        question,
        0 as *const libc::c_char,
    ];
    let mut fa: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: 0 as *mut __spawn_action,
        __pad: [0; 16],
    };
    if pipe(com.as_mut_ptr()) == -(1 as libc::c_int) {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot create pipe\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    rc = posix_spawn_file_actions_init(&mut fa);
    if rc != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Error initializing spawn file actions for use-askpass: %d\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            rc,
        );
    } else {
        fa_valid = 1 as libc::c_int != 0;
        rc = posix_spawn_file_actions_addclose(&mut fa, com[0 as libc::c_int as usize]);
        if rc == 0 {
            rc = posix_spawn_file_actions_adddup2(
                &mut fa,
                com[1 as libc::c_int as usize],
                1 as libc::c_int,
            );
        }
        if rc == 0 {
            rc = posix_spawn_file_actions_addclose(
                &mut fa,
                com[1 as libc::c_int as usize],
            );
        }
        if rc != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error setting spawn file actions for use-askpass: %d\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                rc,
            );
        } else {
            if !is_testing() {
                rc = posix_spawnp(
                    &mut pid,
                    config.use_askpass_bin,
                    &mut fa,
                    0 as *const posix_spawnattr_t,
                    argv.as_ptr() as *const *mut libc::c_char,
                    environ as *const *mut libc::c_char,
                );
                if rc != 0 {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Error spawning %s: %d\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        config.use_askpass_bin,
                        rc,
                    );
                    current_block = 651278059919248869;
                } else {
                    current_block = 15904375183555213903;
                }
            } else {
                current_block = 15904375183555213903;
            }
            match current_block {
                651278059919248869 => {}
                _ => {
                    close(com[1 as libc::c_int as usize]);
                    com[1 as libc::c_int as usize] = -(1 as libc::c_int);
                    if !is_testing() {
                        bytes = read(
                            com[0 as libc::c_int as usize],
                            tmp.as_mut_ptr() as *mut libc::c_void,
                            (::core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                    } else {
                        bytes = (rand() as libc::c_ulong)
                            .wrapping_rem(
                                (::core::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            ) as ssize_t;
                        memset(
                            tmp.as_mut_ptr() as *mut libc::c_void,
                            'x' as i32,
                            bytes as libc::c_ulong,
                        );
                    }
                    if bytes <= 0 as libc::c_int as ssize_t {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Error reading response from command \"%s %s\": %s\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            config.use_askpass_bin,
                            question,
                            strerror(*__errno_location()),
                        );
                    } else {
                        let fresh24 = bytes;
                        bytes = bytes - 1;
                        tmp[fresh24 as usize] = '\0' as i32 as libc::c_char;
                        while bytes >= 0 as libc::c_int as ssize_t
                            && (tmp[bytes as usize] as libc::c_int == '\n' as i32
                                || tmp[bytes as usize] as libc::c_int == '\r' as i32)
                        {
                            let fresh25 = bytes;
                            bytes = bytes - 1;
                            tmp[fresh25 as usize] = '\0' as i32 as libc::c_char;
                        }
                        *answer = wget_strdup(tmp.as_mut_ptr());
                        ret = 0 as libc::c_int;
                    }
                }
            }
        }
    }
    if com[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(com[1 as libc::c_int as usize]);
    }
    if com[0 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(com[0 as libc::c_int as usize]);
    }
    if fa_valid {
        posix_spawn_file_actions_destroy(&mut fa);
    }
    return ret;
}
unsafe extern "C" fn use_askpass() -> libc::c_int {
    let mut question: [libc::c_char; 1024] = [0; 1024];
    if !(config.username).is_null() {
        wget_free
            .expect("non-null function pointer")(config.username as *mut libc::c_void);
        config.username = 0 as *mut libc::c_char;
    }
    if run_use_askpass(
        b"Type username:\0" as *const u8 as *const libc::c_char,
        &mut config.username,
    ) < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    wget_snprintf(
        question.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"Type password for '%s':\0" as *const u8 as *const libc::c_char,
        config.username,
    );
    if !(config.password).is_null() {
        wget_free
            .expect("non-null function pointer")(config.password as *mut libc::c_void);
        config.password = 0 as *mut libc::c_char;
    }
    if run_use_askpass(question.as_mut_ptr(), &mut config.password) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
static mut dns_cache: *mut wget_dns_cache = 0 as *const wget_dns_cache
    as *mut wget_dns_cache;
static mut dns: *mut wget_dns = 0 as *const wget_dns as *mut wget_dns;
unsafe extern "C" fn preload_dns_cache(mut fname: *const libc::c_char) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ip: [libc::c_char; 64] = [0; 64];
    let mut name: [libc::c_char; 256] = [0; 256];
    if strcmp(fname, b"-\0" as *const u8 as *const libc::c_char) == 0
        && !config.dont_write
    {
        fp = stdin;
    } else {
        fp = rpl_fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
        if fp.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
            return -(1 as libc::c_int);
        }
    }
    while !(fgets(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        if sscanf(
            buf.as_mut_ptr(),
            b"%63s %255s\0" as *const u8 as *const libc::c_char,
            ip.as_mut_ptr(),
            name.as_mut_ptr(),
        ) != 2 as libc::c_int
        {
            continue;
        }
        wget_strtolower(name.as_mut_ptr());
        wget_debug_printf(
            b"Adding DNS Mapping: %s -> %s\n\0" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            ip.as_mut_ptr(),
        );
        wget_dns_cache_ip(
            dns,
            ip.as_mut_ptr(),
            name.as_mut_ptr(),
            80 as libc::c_int as uint16_t,
        );
        wget_dns_cache_ip(
            dns,
            ip.as_mut_ptr(),
            name.as_mut_ptr(),
            443 as libc::c_int as uint16_t,
        );
    }
    if fp != stdin {
        rpl_fclose(fp);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn get_config_files(
    mut config_home: *const libc::c_char,
    mut user_home: *const libc::c_char,
) {
    let mut env: *const libc::c_char = 0 as *const libc::c_char;
    env = getenv(b"SYSTEM_WGET2RC\0" as *const u8 as *const libc::c_char);
    if !env.is_null() && *env as libc::c_int != 0 {
        config.system_config = wget_strdup(env);
    } else if !(config.system_config).is_null() {
        if access(config.system_config, 4 as libc::c_int) != 0 as libc::c_int {
            if !(config.system_config).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(config.system_config as *mut libc::c_void);
                config.system_config = 0 as *const libc::c_char;
            }
        }
    }
    env = getenv(b"WGET2RC\0" as *const u8 as *const libc::c_char);
    if !env.is_null() && *env as libc::c_int != 0 {
        config.user_config = wget_strdup(env);
    } else {
        let mut path: *const libc::c_char = wget_aprintf(
            b"%s/wget2rc\0" as *const u8 as *const libc::c_char,
            config_home,
        );
        if access(path, 4 as libc::c_int) == 0 as libc::c_int {
            config.user_config = path;
        } else if !path.is_null() {
            wget_free.expect("non-null function pointer")(path as *mut libc::c_void);
            path = 0 as *const libc::c_char;
        }
    }
    if (config.user_config).is_null() {
        let mut path_0: *const libc::c_char = wget_aprintf(
            b"%s/.wget2rc\0" as *const u8 as *const libc::c_char,
            user_home,
        );
        if access(path_0, 4 as libc::c_int) == 0 as libc::c_int {
            config.user_config = path_0;
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"~/.wget2rc is deprecated. Please move it to %s/wget2rc\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                config_home,
            );
        } else if !path_0.is_null() {
            wget_free.expect("non-null function pointer")(path_0 as *mut libc::c_void);
            path_0 = 0 as *const libc::c_char;
        }
    }
}
unsafe extern "C" fn get_xdg_data_home(
    mut user_home: *const libc::c_char,
) -> *const libc::c_char {
    static mut data_home: *const libc::c_char = 0 as *const libc::c_char;
    let mut env: *const libc::c_char = 0 as *const libc::c_char;
    if user_home.is_null() {
        if !data_home.is_null() {
            wget_free
                .expect("non-null function pointer")(data_home as *mut libc::c_void);
            data_home = 0 as *const libc::c_char;
        }
        return 0 as *const libc::c_char;
    }
    if !data_home.is_null() {
        return data_home;
    }
    env = getenv(b"XDG_DATA_HOME\0" as *const u8 as *const libc::c_char);
    if !env.is_null() && *env as libc::c_int != 0 {
        data_home = wget_aprintf(b"%s/wget\0" as *const u8 as *const libc::c_char, env);
    } else {
        data_home = wget_aprintf(
            b"%s/.local/share/wget\0" as *const u8 as *const libc::c_char,
            user_home,
        );
    }
    mkdir_path(data_home, 0 as libc::c_int != 0);
    return data_home;
}
unsafe extern "C" fn get_xdg_config_home(
    mut user_home: *const libc::c_char,
) -> *const libc::c_char {
    static mut home_dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut env: *const libc::c_char = 0 as *const libc::c_char;
    if user_home.is_null() {
        if !home_dir.is_null() {
            wget_free.expect("non-null function pointer")(home_dir as *mut libc::c_void);
            home_dir = 0 as *const libc::c_char;
        }
        return 0 as *const libc::c_char;
    }
    if !home_dir.is_null() {
        return home_dir;
    }
    env = getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char);
    if !env.is_null() && *env as libc::c_int != 0 {
        home_dir = wget_aprintf(b"%s/wget\0" as *const u8 as *const libc::c_char, env);
    } else {
        home_dir = wget_aprintf(
            b"%s/.config/wget\0" as *const u8 as *const libc::c_char,
            user_home,
        );
    }
    return home_dir;
}
unsafe extern "C" fn stats_callback_dns(
    mut _dns: *mut wget_dns,
    mut stats: *mut wget_dns_stats_data,
    mut ctx: *mut libc::c_void,
) {
    let mut fp: *mut FILE = ctx as *mut FILE;
    if (*config.stats_dns_args).format as libc::c_uint
        == WGET_STATS_FORMAT_HUMAN as libc::c_int as libc::c_uint
    {
        if wget_ip_is_family((*stats).hostname, 2 as libc::c_int) {
            wget_fprintf(
                fp,
                b"  %4lld [%s]:%hu (%s)\n\0" as *const u8 as *const libc::c_char,
                (*stats).dns_secs,
                if !((*stats).hostname).is_null() {
                    (*stats).hostname
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                },
                (*stats).port as libc::c_int,
                if !((*stats).ip).is_null() {
                    (*stats).ip
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                },
            );
        } else {
            wget_fprintf(
                fp,
                b"  %4lld %s:%hu (%s)\n\0" as *const u8 as *const libc::c_char,
                (*stats).dns_secs,
                if !((*stats).hostname).is_null() {
                    (*stats).hostname
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                },
                (*stats).port as libc::c_int,
                if !((*stats).ip).is_null() {
                    (*stats).ip
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                },
            );
        }
    } else {
        wget_fprintf(
            fp,
            b"%s,%s,%hu,%lld\n\0" as *const u8 as *const libc::c_char,
            (*stats).hostname,
            (*stats).ip,
            (*stats).port as libc::c_int,
            (*stats).dns_secs,
        );
    };
}
unsafe extern "C" fn stats_callback_ocsp(
    mut stats: *mut wget_ocsp_stats_data,
    mut ctx: *mut libc::c_void,
) {
    let mut fp: *mut FILE = ctx as *mut FILE;
    if (*config.stats_ocsp_args).format as libc::c_uint
        == WGET_STATS_FORMAT_HUMAN as libc::c_int as libc::c_uint
    {
        wget_fprintf(
            fp,
            b"  %s:\n\0" as *const u8 as *const libc::c_char,
            (*stats).hostname,
        );
        wget_fprintf(
            fp,
            b"    Stapling       : %d\n\0" as *const u8 as *const libc::c_char,
            (*stats).stapling,
        );
        wget_fprintf(
            fp,
            b"    Valid          : %d\n\0" as *const u8 as *const libc::c_char,
            (*stats).nvalid,
        );
        wget_fprintf(
            fp,
            b"    Revoked        : %d\n\0" as *const u8 as *const libc::c_char,
            (*stats).nrevoked,
        );
        wget_fprintf(
            fp,
            b"    Ignored        : %d\n\n\0" as *const u8 as *const libc::c_char,
            (*stats).nignored,
        );
    } else {
        wget_fprintf(
            fp,
            b"%s,%d,%d,%d,%d\n\0" as *const u8 as *const libc::c_char,
            (*stats).hostname,
            (*stats).stapling,
            (*stats).nvalid,
            (*stats).nrevoked,
            (*stats).nignored,
        );
    };
}
unsafe extern "C" fn tlsversion_string(mut v: libc::c_int) -> *const libc::c_char {
    match v {
        1 => return b"SSL3\0" as *const u8 as *const libc::c_char,
        2 => return b"TLS1.0\0" as *const u8 as *const libc::c_char,
        3 => return b"TLS1.1\0" as *const u8 as *const libc::c_char,
        4 => return b"TLS1.2\0" as *const u8 as *const libc::c_char,
        5 => return b"TLS1.3\0" as *const u8 as *const libc::c_char,
        _ => return b"?\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn stats_callback_tls(
    mut stats: *mut wget_tls_stats_data,
    mut ctx: *mut libc::c_void,
) {
    let mut fp: *mut FILE = ctx as *mut FILE;
    if (*config.stats_tls_args).format as libc::c_uint
        == WGET_STATS_FORMAT_HUMAN as libc::c_int as libc::c_uint
    {
        wget_fprintf(
            fp,
            b"  %s:\n\0" as *const u8 as *const libc::c_char,
            (*stats).hostname,
        );
        wget_fprintf(
            fp,
            b"    Version         : %s\n\0" as *const u8 as *const libc::c_char,
            tlsversion_string((*stats).version),
        );
        wget_fprintf(
            fp,
            b"    False Start     : %s\n\0" as *const u8 as *const libc::c_char,
            if (*stats).false_start as libc::c_int != 0 {
                if (*stats).false_start as libc::c_int == 1 as libc::c_int {
                    b"On\0" as *const u8 as *const libc::c_char
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                }
            } else {
                b"Off\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    TFO             : %s\n\0" as *const u8 as *const libc::c_char,
            if (*stats).tfo as libc::c_int != 0 {
                if (*stats).tfo as libc::c_int == 1 as libc::c_int {
                    b"On\0" as *const u8 as *const libc::c_char
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                }
            } else {
                b"Off\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    ALPN Protocol   : %s\n\0" as *const u8 as *const libc::c_char,
            if !((*stats).alpn_protocol).is_null() {
                (*stats).alpn_protocol
            } else {
                b"-\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    Resumed         : %s\n\0" as *const u8 as *const libc::c_char,
            if (*stats).resumed as libc::c_int != 0 {
                b"Yes\0" as *const u8 as *const libc::c_char
            } else {
                b"No\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    TCP Protocol    : %s\n\0" as *const u8 as *const libc::c_char,
            if (*stats).http_protocol as libc::c_int == 0 as libc::c_int {
                b"HTTP/1.1\0" as *const u8 as *const libc::c_char
            } else if (*stats).http_protocol as libc::c_int == 1 as libc::c_int {
                b"HTTP/2\0" as *const u8 as *const libc::c_char
            } else {
                b"-\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    Cert Chain Size : %d\n\0" as *const u8 as *const libc::c_char,
            (*stats).cert_chain_size,
        );
        wget_fprintf(fp, b"    TLS negotiation\n\0" as *const u8 as *const libc::c_char);
        wget_fprintf(
            fp,
            b"    duration (ms)   : %lld\n\n\0" as *const u8 as *const libc::c_char,
            (*stats).tls_secs,
        );
    } else {
        wget_fprintf(
            fp,
            b"%s,%d,%d,%d,%d,%s,%d,%d,%lld\n\0" as *const u8 as *const libc::c_char,
            (*stats).hostname,
            (*stats).version,
            (*stats).false_start as libc::c_int,
            (*stats).tfo as libc::c_int,
            (*stats).resumed as libc::c_int,
            if !((*stats).alpn_protocol).is_null() {
                (*stats).alpn_protocol
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            (*stats).http_protocol as libc::c_int,
            (*stats).cert_chain_size,
            (*stats).tls_secs,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    set_allocation_functions();
    if argc >= 2 as libc::c_int {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-d\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            config.debug = 1 as libc::c_int != 0;
        } else if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--debug\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            rc = set_long_option(
                (*argv.offset(1 as libc::c_int as isize))
                    .offset(2 as libc::c_int as isize),
                *argv.offset(2 as libc::c_int as isize),
                0 as libc::c_int as libc::c_char,
            );
            if rc < 0 as libc::c_int {
                return rc;
            }
        }
    }
    let mut home_dir: *const libc::c_char = get_home_dir(0 as libc::c_int != 0);
    let mut xdg_config_home: *const libc::c_char = get_xdg_config_home(home_dir);
    let mut xdg_data_home: *const libc::c_char = get_xdg_data_home(home_dir);
    config.user_agent = wget_strdup(config.user_agent);
    config.secure_protocol = wget_strdup(config.secure_protocol);
    config.ca_directory = wget_strdup(config.ca_directory);
    config.ca_cert = wget_strdup(config.ca_cert);
    config.default_page = wget_strdup(config.default_page);
    config.system_config = wget_strdup(config.system_config);
    log_init();
    get_config_files(xdg_config_home, home_dir);
    if parse_command_line(argc, argv) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if !(config.logfile_append).is_null() {
        if !(config.logfile).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.logfile as *mut libc::c_void);
            config.logfile = 0 as *const libc::c_char;
        }
        config.logfile = config.logfile_append;
        config.logfile_append = 0 as *const libc::c_char;
    } else if !(config.logfile).is_null()
        && strcmp(config.logfile, b"-\0" as *const u8 as *const libc::c_char) != 0
        && !config.dont_write
    {
        let mut fd: libc::c_int = open(
            config.logfile,
            0o1 as libc::c_int | 0o1000 as libc::c_int,
        );
        if fd != -(1 as libc::c_int) {
            close(fd);
        }
    }
    log_init();
    if config.hsts as libc::c_int != 0 && (config.hsts_file).is_null() {
        config
            .hsts_file = wget_aprintf(
            b"%s/.wget-hsts\0" as *const u8 as *const libc::c_char,
            xdg_data_home,
        );
    }
    if config.hpkp as libc::c_int != 0 && (config.hpkp_file).is_null() {
        config
            .hpkp_file = wget_aprintf(
            b"%s/.wget-hpkp\0" as *const u8 as *const libc::c_char,
            xdg_data_home,
        );
    }
    if config.tls_resume as libc::c_int != 0 && (config.tls_session_file).is_null() {
        config
            .tls_session_file = wget_aprintf(
            b"%s/.wget-session\0" as *const u8 as *const libc::c_char,
            xdg_data_home,
        );
    }
    if config.ocsp as libc::c_int != 0 && (config.ocsp_file).is_null() {
        config
            .ocsp_file = wget_aprintf(
            b"%s/.wget-ocsp\0" as *const u8 as *const libc::c_char,
            xdg_data_home,
        );
    }
    if config.netrc as libc::c_int != 0 && (config.netrc_file).is_null() {
        config
            .netrc_file = wget_aprintf(
            b"%s/.netrc\0" as *const u8 as *const libc::c_char,
            home_dir,
        );
    }
    wget_vector_free(&mut config.exclude_directories);
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    plugin_loading_enabled = 1 as libc::c_int;
    path = getenv(b"WGET2_PLUGIN_DIRS\0" as *const u8 as *const libc::c_char);
    if !path.is_null() {
        plugin_db_clear_search_paths();
        plugin_db_add_search_paths(path, ':' as i32 as libc::c_char);
    }
    if plugin_db_load_from_envvar() != 0 {
        set_exit_status(EXIT_STATUS_PARSE_INIT);
        return -(1 as libc::c_int);
    }
    read_config();
    n = parse_command_line(argc, argv);
    if n < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if plugin_db_help_forwarded() {
        set_exit_status(EXIT_STATUS_NO_ERROR);
        return -(1 as libc::c_int);
    }
    if !(config.logfile_append).is_null() {
        if !(config.logfile).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.logfile as *mut libc::c_void);
            config.logfile = 0 as *const libc::c_char;
        }
        config.logfile = config.logfile_append;
        config.logfile_append = 0 as *const libc::c_char;
    } else if !(config.logfile).is_null()
        && strcmp(config.logfile, b"-\0" as *const u8 as *const libc::c_char) != 0
        && !config.dont_write
    {
        let mut fd_0: libc::c_int = open(
            config.logfile,
            0o1 as libc::c_int | 0o1000 as libc::c_int,
        );
        if fd_0 != -(1 as libc::c_int) {
            close(fd_0);
        }
    }
    log_init();
    if !(config.logfile).is_null() {
        config.progress = 0 as libc::c_int as libc::c_char;
    }
    if config.https_only as libc::c_int != 0 && config.https_enforce as libc::c_uint != 0
    {
        config.https_enforce = HTTPS_ENFORCE_NONE;
    }
    wget_iri_set_defaultport(WGET_IRI_SCHEME_HTTP, config.default_http_port);
    wget_iri_set_defaultport(WGET_IRI_SCHEME_HTTPS, config.default_https_port);
    if config.max_threads < 1 as libc::c_int
        || config.max_threads > 1 as libc::c_int && config.chunk_size != 0
    {
        config.max_threads = 1 as libc::c_int;
    }
    if config.hyperlink {
        config.hostname = xgethostname();
    }
    if !(config.output_document).is_null()
        && strcmp(config.output_document, b"-\0" as *const u8 as *const libc::c_char)
            != 0 && !config.dont_write
    {
        if config.unlink {
            unlink(config.output_document);
        } else if !config.continue_download && config.clobber as libc::c_int != 0 {
            let mut fd_1: libc::c_int = open(
                config.output_document,
                0o1 as libc::c_int | 0o1000 as libc::c_int | 0 as libc::c_int,
            );
            if fd_1 != -(1 as libc::c_int) {
                close(fd_1);
            }
        }
    }
    if (config.local_encoding).is_null() {
        config.local_encoding = wget_local_charset_encoding();
    }
    if (config.input_encoding).is_null() {
        config.input_encoding = wget_strdup(config.local_encoding);
    }
    wget_debug_printf(
        b"Local URI encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
        config.local_encoding,
    );
    wget_debug_printf(
        b"Input URI encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
        config.input_encoding,
    );
    if config.proxy {
        if (config.http_proxy).is_null() {
            config
                .http_proxy = wget_strdup(
                getenv(b"http_proxy\0" as *const u8 as *const libc::c_char),
            );
        }
        if (config.https_proxy).is_null() {
            config
                .https_proxy = wget_strdup(
                getenv(b"https_proxy\0" as *const u8 as *const libc::c_char),
            );
        }
        if (config.no_proxy).is_null() {
            config
                .no_proxy = wget_strdup(
                getenv(b"no_proxy\0" as *const u8 as *const libc::c_char),
            );
        }
    }
    if !(config.http_proxy).is_null() && *config.http_proxy as libc::c_int != 0
        && wget_http_set_http_proxy(config.http_proxy, config.local_encoding) == 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set http proxies %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            config.http_proxy,
        );
        return -(1 as libc::c_int);
    }
    if !(config.https_proxy).is_null() && *config.https_proxy as libc::c_int != 0
        && wget_http_set_https_proxy(config.https_proxy, config.local_encoding) == 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set https proxies %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            config.https_proxy,
        );
        return -(1 as libc::c_int);
    }
    if !(config.no_proxy).is_null()
        && wget_http_set_no_proxy(config.no_proxy, config.local_encoding)
            < 0 as libc::c_int
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set proxy exceptions %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            config.no_proxy,
        );
        return -(1 as libc::c_int);
    }
    if !(config.http_proxy).is_null() {
        wget_free
            .expect("non-null function pointer")(config.http_proxy as *mut libc::c_void);
        config.http_proxy = 0 as *const libc::c_char;
    }
    if !(config.https_proxy).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.https_proxy as *mut libc::c_void);
        config.https_proxy = 0 as *const libc::c_char;
    }
    if !(config.no_proxy).is_null() {
        wget_free
            .expect("non-null function pointer")(config.no_proxy as *mut libc::c_void);
        config.no_proxy = 0 as *const libc::c_char;
    }
    if config.cookies {
        config.cookie_db = wget_cookie_db_init(0 as *mut wget_cookie_db);
        wget_cookie_set_keep_session_cookies(
            config.cookie_db,
            config.keep_session_cookies,
        );
        if !(config.cookie_suffixes).is_null() {
            wget_cookie_db_load_psl(config.cookie_db, config.cookie_suffixes);
        }
        if !(config.load_cookies).is_null() {
            wget_cookie_db_load(config.cookie_db, config.load_cookies);
        }
    }
    if config.hsts {
        if (config.hsts_db).is_null() {
            config.hsts_db = wget_hsts_db_init(0 as *mut wget_hsts_db, config.hsts_file);
        }
        wget_hsts_db_load(config.hsts_db);
    }
    if config.hpkp {
        if (config.hpkp_db).is_null() {
            config.hpkp_db = wget_hpkp_db_init(0 as *mut wget_hpkp_db, config.hpkp_file);
        }
        wget_hpkp_db_load(config.hpkp_db);
    }
    if config.tls_resume {
        config.tls_session_db = wget_tls_session_db_init(0 as *mut wget_tls_session_db);
        wget_tls_session_db_load(config.tls_session_db, config.tls_session_file);
    }
    if config.ocsp {
        if (config.ocsp_db).is_null() {
            config.ocsp_db = wget_ocsp_db_init(0 as *mut wget_ocsp_db, config.ocsp_file);
        }
        wget_ocsp_db_load(config.ocsp_db);
    }
    if !(config.base_url).is_null() {
        config.base = wget_iri_parse(config.base_url, config.local_encoding);
    }
    if config.askpass as libc::c_int != 0 && n < argc {
        if !(config.password).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.password as *mut libc::c_void);
            config.password = 0 as *mut libc::c_char;
        }
        config.password = prompt_for_password();
    }
    if !(config.use_askpass_bin).is_null() && n < argc
        && use_askpass() < 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if (config.http_username).is_null() {
        config.http_username = wget_strdup(config.username);
    }
    if (config.http_password).is_null() {
        config.http_password = wget_strdup(config.password);
    }
    if (config.http_proxy_username).is_null() {
        config.http_proxy_username = wget_strdup(config.username);
    }
    if (config.http_proxy_password).is_null() {
        config.http_proxy_password = wget_strdup(config.password);
    }
    if config.auth_no_challenge {
        config.default_challenges = wget_vector_create(1 as libc::c_int, None);
        let mut basic: *mut wget_http_challenge = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_http_challenge>() as libc::c_ulong,
        ) as *mut wget_http_challenge;
        (*basic)
            .auth_scheme = wget_strdup(b"basic\0" as *const u8 as *const libc::c_char);
        wget_vector_add(config.default_challenges, basic as *const libc::c_void);
        wget_vector_set_destructor(
            config.default_challenges,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut wget_http_challenge) -> ()>,
                Option::<wget_vector_destructor>,
            >(
                Some(
                    wget_http_free_challenge
                        as unsafe extern "C" fn(*mut wget_http_challenge) -> (),
                ),
            ),
        );
    }
    if config.page_requisites as libc::c_int != 0 && !config.recursive {
        config.recursive = 1 as libc::c_int != 0;
        config.level = 1 as libc::c_int;
    }
    if config.start_pos != 0 && config.continue_download as libc::c_int != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Specifying both --start-pos and --continue is not recommended; --continue will be disabled\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        config.continue_download = 0 as libc::c_int != 0;
    }
    if config.timestamping {
        if !(config.output_document).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: timestamping does nothing in combination with -O\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            config.timestamping = 0 as libc::c_int != 0;
        }
        if !config.clobber {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Can't timestamp and not clobber old files at the same time\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            config.timestamping = 0 as libc::c_int != 0;
        }
        if config.spider {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: timestamping does nothing in combination with spider\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            config.timestamping = 0 as libc::c_int != 0;
        }
        if config.chunk_size != 0 && config.if_modified_since as libc::c_int != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"WARNING: timestamping and chunk-size only work together without If-Modified-Since\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            config.if_modified_since = 0 as libc::c_int != 0;
        }
    }
    if config.mirror {
        config.metalink = 0 as libc::c_int != 0;
    }
    config.verify_sig as u64 != 0;
    rc = wget_net_init();
    if rc != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to init networking (%d)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            rc,
        );
        return -(1 as libc::c_int);
    }
    rc = wget_dns_init(&mut dns);
    if rc != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to init DNS (%d)\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            rc,
        );
        return -(1 as libc::c_int);
    }
    if config.dns_caching {
        rc = wget_dns_cache_init(&mut dns_cache);
        if rc != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to init DNS cache (%d)\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                rc,
            );
            return -(1 as libc::c_int);
        }
        wget_dns_set_cache(dns, dns_cache);
    }
    wget_dns_set_timeout(dns, config.dns_timeout);
    wget_tcp_set_dns(0 as *mut wget_tcp, dns);
    if !(config.stats_dns_args).is_null() {
        (*config.stats_dns_args)
            .fp = if !((*config.stats_dns_args).filename).is_null()
            && *(*config.stats_dns_args).filename as libc::c_int != 0
            && strcmp(
                (*config.stats_dns_args).filename,
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 && !config.dont_write
        {
            rpl_fopen(
                (*config.stats_dns_args).filename,
                b"w\0" as *const u8 as *const libc::c_char,
            )
        } else {
            stdout
        };
        if ((*config.stats_dns_args).fp).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open '%s' (%d)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*config.stats_dns_args).filename,
                rc,
            );
            return -(1 as libc::c_int);
        }
        wget_dns_set_stats_callback(
            dns,
            Some(
                stats_callback_dns
                    as unsafe extern "C" fn(
                        *mut wget_dns,
                        *mut wget_dns_stats_data,
                        *mut libc::c_void,
                    ) -> (),
            ),
            (*config.stats_dns_args).fp as *mut libc::c_void,
        );
    }
    if !(config.stats_ocsp_args).is_null() {
        (*config.stats_ocsp_args)
            .fp = if !((*config.stats_ocsp_args).filename).is_null()
            && *(*config.stats_ocsp_args).filename as libc::c_int != 0
            && strcmp(
                (*config.stats_ocsp_args).filename,
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 && !config.dont_write
        {
            rpl_fopen(
                (*config.stats_ocsp_args).filename,
                b"w\0" as *const u8 as *const libc::c_char,
            )
        } else {
            stdout
        };
        if ((*config.stats_ocsp_args).fp).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open '%s' (%d)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*config.stats_ocsp_args).filename,
                rc,
            );
            return -(1 as libc::c_int);
        }
        wget_ssl_set_stats_callback_ocsp(
            Some(
                stats_callback_ocsp
                    as unsafe extern "C" fn(
                        *mut wget_ocsp_stats_data,
                        *mut libc::c_void,
                    ) -> (),
            ),
            (*config.stats_ocsp_args).fp as *mut libc::c_void,
        );
    }
    if !(config.stats_tls_args).is_null() {
        (*config.stats_tls_args)
            .fp = if !((*config.stats_tls_args).filename).is_null()
            && *(*config.stats_tls_args).filename as libc::c_int != 0
            && strcmp(
                (*config.stats_tls_args).filename,
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 && !config.dont_write
        {
            rpl_fopen(
                (*config.stats_tls_args).filename,
                b"w\0" as *const u8 as *const libc::c_char,
            )
        } else {
            stdout
        };
        if ((*config.stats_tls_args).fp).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open '%s' (%d)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*config.stats_tls_args).filename,
                rc,
            );
            return -(1 as libc::c_int);
        }
        wget_ssl_set_stats_callback_tls(
            Some(
                stats_callback_tls
                    as unsafe extern "C" fn(
                        *mut wget_tls_stats_data,
                        *mut libc::c_void,
                    ) -> (),
            ),
            (*config.stats_tls_args).fp as *mut libc::c_void,
        );
    }
    if !(config.stats_server_args).is_null() {
        (*config.stats_server_args)
            .fp = if !((*config.stats_server_args).filename).is_null()
            && *(*config.stats_server_args).filename as libc::c_int != 0
            && strcmp(
                (*config.stats_server_args).filename,
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 && !config.dont_write
        {
            rpl_fopen(
                (*config.stats_server_args).filename,
                b"w\0" as *const u8 as *const libc::c_char,
            )
        } else {
            stdout
        };
        if ((*config.stats_server_args).fp).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open '%s' (%d)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*config.stats_server_args).filename,
                rc,
            );
            return -(1 as libc::c_int);
        }
        server_stats_init((*config.stats_server_args).fp);
    }
    if !(config.stats_site_args).is_null() {
        (*config.stats_site_args)
            .fp = if !((*config.stats_site_args).filename).is_null()
            && *(*config.stats_site_args).filename as libc::c_int != 0
            && strcmp(
                (*config.stats_site_args).filename,
                b"-\0" as *const u8 as *const libc::c_char,
            ) != 0 && !config.dont_write
        {
            rpl_fopen(
                (*config.stats_site_args).filename,
                b"w\0" as *const u8 as *const libc::c_char,
            )
        } else {
            stdout
        };
        if ((*config.stats_site_args).fp).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open '%s' (%d)\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*config.stats_site_args).filename,
                rc,
            );
            return -(1 as libc::c_int);
        }
        site_stats_init((*config.stats_site_args).fp);
    }
    wget_tcp_set_timeout(0 as *mut wget_tcp, config.read_timeout);
    wget_tcp_set_connect_timeout(0 as *mut wget_tcp, config.connect_timeout);
    wget_tcp_set_tcp_fastopen(0 as *mut wget_tcp, config.tcp_fastopen);
    wget_tcp_set_tls_false_start(0 as *mut wget_tcp, config.tls_false_start);
    if !config.dont_write {
        wget_tcp_set_bind_address(0 as *mut wget_tcp, config.bind_address);
    }
    if !(config.bind_interface).is_null() {
        wget_tcp_set_bind_interface(0 as *mut wget_tcp, config.bind_interface);
    }
    if config.inet4_only {
        wget_tcp_set_family(0 as *mut wget_tcp, 1 as libc::c_int);
    } else if config.inet6_only {
        wget_tcp_set_family(0 as *mut wget_tcp, 2 as libc::c_int);
    } else {
        wget_tcp_set_preferred_family(0 as *mut wget_tcp, config.preferred_family);
    }
    wget_iri_set_defaultpage(config.default_page);
    wget_ssl_set_config_int(
        9 as libc::c_int,
        (config.check_certificate as libc::c_uint
            == CHECK_CERTIFICATE_ENABLED as libc::c_int as libc::c_uint) as libc::c_int,
    );
    wget_ssl_set_config_int(
        23 as libc::c_int,
        (config.check_certificate as libc::c_uint
            != CHECK_CERTIFICATE_LOG_DISABLED as libc::c_int as libc::c_uint)
            as libc::c_int,
    );
    wget_ssl_set_config_int(10 as libc::c_int, config.check_hostname as libc::c_int);
    wget_ssl_set_config_int(7 as libc::c_int, config.cert_type as libc::c_int);
    wget_ssl_set_config_int(8 as libc::c_int, config.private_key_type as libc::c_int);
    wget_ssl_set_config_int(11 as libc::c_int, config.debug as libc::c_int);
    wget_ssl_set_config_int(16 as libc::c_int, config.ocsp as libc::c_int);
    wget_ssl_set_config_int(22 as libc::c_int, config.ocsp_date as libc::c_int);
    wget_ssl_set_config_int(21 as libc::c_int, config.ocsp_nonce as libc::c_int);
    wget_ssl_set_config_int(14 as libc::c_int, config.ocsp_stapling as libc::c_int);
    wget_ssl_set_config_string(15 as libc::c_int, config.ocsp_server);
    wget_ssl_set_config_string(1 as libc::c_int, config.secure_protocol);
    wget_ssl_set_config_string(2 as libc::c_int, config.ca_directory);
    wget_ssl_set_config_string(3 as libc::c_int, config.ca_cert);
    wget_ssl_set_config_string(4 as libc::c_int, config.cert_file);
    wget_ssl_set_config_string(5 as libc::c_int, config.private_key);
    wget_ssl_set_config_string(13 as libc::c_int, config.crl_file);
    wget_ssl_set_config_object(17 as libc::c_int, config.ocsp_db as *mut libc::c_void);
    wget_ssl_set_config_object(
        19 as libc::c_int,
        config.tls_session_db as *mut libc::c_void,
    );
    wget_ssl_set_config_object(20 as libc::c_int, config.hpkp_db as *mut libc::c_void);
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(config.domains) {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut hostname: *mut libc::c_char = wget_vector_get(config.domains, it)
            as *mut libc::c_char;
        wget_percent_unescape(hostname);
        if wget_str_needs_encoding(hostname) {
            s = wget_str_to_utf8(hostname, config.local_encoding);
            if !s.is_null() {
                wget_vector_replace(config.domains, s as *const libc::c_void, it);
                hostname = s;
            }
            s = wget_str_to_ascii(hostname) as *mut libc::c_char;
            if s != hostname {
                wget_vector_replace(config.domains, s as *const libc::c_void, it);
            }
        } else {
            wget_strtolower(hostname);
        }
        it += 1;
        it;
    }
    let mut it_0: libc::c_int = 0 as libc::c_int;
    while it_0 < wget_vector_size(config.exclude_domains) {
        let mut s_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut hostname_0: *mut libc::c_char = wget_vector_get(
            config.exclude_domains,
            it_0,
        ) as *mut libc::c_char;
        wget_percent_unescape(hostname_0);
        if wget_str_needs_encoding(hostname_0) {
            s_0 = wget_str_to_utf8(hostname_0, config.local_encoding);
            if !s_0.is_null() {
                wget_vector_replace(
                    config.exclude_domains,
                    s_0 as *const libc::c_void,
                    it_0,
                );
                hostname_0 = s_0;
            }
            s_0 = wget_str_to_ascii(hostname_0) as *mut libc::c_char;
            if s_0 != hostname_0 {
                wget_vector_replace(
                    config.exclude_domains,
                    s_0 as *const libc::c_void,
                    it_0,
                );
            }
        } else {
            wget_strtolower(hostname_0);
        }
        it_0 += 1;
        it_0;
    }
    if !(config.dns_cache_preload).is_null() {
        preload_dns_cache(config.dns_cache_preload);
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn deinit() {
    wget_global_deinit();
    get_home_dir(1 as libc::c_int != 0);
    get_xdg_config_home(0 as *const libc::c_char);
    get_xdg_data_home(0 as *const libc::c_char);
    wget_dns_free(&mut dns);
    wget_dns_cache_free(&mut dns_cache);
    wget_cookie_db_free(&mut config.cookie_db);
    wget_hsts_db_free(&mut config.hsts_db);
    wget_hpkp_db_free(&mut config.hpkp_db);
    wget_tls_session_db_free(&mut config.tls_session_db);
    wget_ocsp_db_free(&mut config.ocsp_db);
    wget_netrc_db_free(&mut config.netrc_db);
    if !(config.accept_regex).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.accept_regex as *mut libc::c_void);
        config.accept_regex = 0 as *const libc::c_char;
    }
    if !(config.base_url).is_null() {
        wget_free
            .expect("non-null function pointer")(config.base_url as *mut libc::c_void);
        config.base_url = 0 as *const libc::c_char;
    }
    if !(config.bind_address).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.bind_address as *mut libc::c_void);
        config.bind_address = 0 as *const libc::c_char;
    }
    if !(config.bind_interface).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.bind_interface as *mut libc::c_void);
        config.bind_interface = 0 as *const libc::c_char;
    }
    if !(config.body_data).is_null() {
        wget_free
            .expect("non-null function pointer")(config.body_data as *mut libc::c_void);
        config.body_data = 0 as *const libc::c_char;
    }
    if !(config.body_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.body_file as *mut libc::c_void);
        config.body_file = 0 as *const libc::c_char;
    }
    if !(config.ca_cert).is_null() {
        wget_free
            .expect("non-null function pointer")(config.ca_cert as *mut libc::c_void);
        config.ca_cert = 0 as *const libc::c_char;
    }
    if !(config.ca_directory).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.ca_directory as *mut libc::c_void);
        config.ca_directory = 0 as *const libc::c_char;
    }
    if !(config.cert_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.cert_file as *mut libc::c_void);
        config.cert_file = 0 as *const libc::c_char;
    }
    if !(config.cookie_suffixes).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.cookie_suffixes as *mut libc::c_void);
        config.cookie_suffixes = 0 as *const libc::c_char;
    }
    if !(config.crl_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.crl_file as *mut libc::c_void);
        config.crl_file = 0 as *const libc::c_char;
    }
    if !(config.default_page).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.default_page as *mut libc::c_void);
        config.default_page = 0 as *const libc::c_char;
    }
    if !(config.directory_prefix).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.directory_prefix as *mut libc::c_void);
        config.directory_prefix = 0 as *const libc::c_char;
    }
    if !(config.egd_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.egd_file as *mut libc::c_void);
        config.egd_file = 0 as *const libc::c_char;
    }
    if !(config.hsts_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.hsts_file as *mut libc::c_void);
        config.hsts_file = 0 as *const libc::c_char;
    }
    if !(config.hpkp_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.hpkp_file as *mut libc::c_void);
        config.hpkp_file = 0 as *const libc::c_char;
    }
    if !(config.http_password).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.http_password as *mut libc::c_void);
        config.http_password = 0 as *const libc::c_char;
    }
    if !(config.http_proxy).is_null() {
        wget_free
            .expect("non-null function pointer")(config.http_proxy as *mut libc::c_void);
        config.http_proxy = 0 as *const libc::c_char;
    }
    if !(config.http_proxy_password).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.http_proxy_password as *mut libc::c_void);
        config.http_proxy_password = 0 as *const libc::c_char;
    }
    if !(config.http_proxy_username).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.http_proxy_username as *mut libc::c_void);
        config.http_proxy_username = 0 as *const libc::c_char;
    }
    if !(config.http_username).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.http_username as *mut libc::c_void);
        config.http_username = 0 as *const libc::c_char;
    }
    if !(config.https_proxy).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.https_proxy as *mut libc::c_void);
        config.https_proxy = 0 as *const libc::c_char;
    }
    if !(config.hsts_preload_file).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.hsts_preload_file as *mut libc::c_void);
        config.hsts_preload_file = 0 as *const libc::c_char;
    }
    if !(config.input_encoding).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.input_encoding as *mut libc::c_void);
        config.input_encoding = 0 as *const libc::c_char;
    }
    if !(config.input_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.input_file as *mut libc::c_void);
        config.input_file = 0 as *const libc::c_char;
    }
    if !(config.load_cookies).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.load_cookies as *mut libc::c_void);
        config.load_cookies = 0 as *const libc::c_char;
    }
    if !(config.local_encoding).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.local_encoding as *mut libc::c_void);
        config.local_encoding = 0 as *const libc::c_char;
    }
    if !(config.logfile).is_null() {
        wget_free
            .expect("non-null function pointer")(config.logfile as *mut libc::c_void);
        config.logfile = 0 as *const libc::c_char;
    }
    if !(config.logfile_append).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.logfile_append as *mut libc::c_void);
        config.logfile_append = 0 as *const libc::c_char;
    }
    if !(config.method).is_null() {
        wget_free
            .expect("non-null function pointer")(config.method as *mut libc::c_void);
        config.method = 0 as *const libc::c_char;
    }
    if !(config.hostname).is_null() {
        wget_free
            .expect("non-null function pointer")(config.hostname as *mut libc::c_void);
        config.hostname = 0 as *const libc::c_char;
    }
    if !(config.netrc_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.netrc_file as *mut libc::c_void);
        config.netrc_file = 0 as *const libc::c_char;
    }
    if !(config.ocsp_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.ocsp_file as *mut libc::c_void);
        config.ocsp_file = 0 as *const libc::c_char;
    }
    if !(config.ocsp_server).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.ocsp_server as *mut libc::c_void);
        config.ocsp_server = 0 as *const libc::c_char;
    }
    if !(config.output_document).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.output_document as *mut libc::c_void);
        config.output_document = 0 as *const libc::c_char;
    }
    if !(config.password).is_null() {
        wget_free
            .expect("non-null function pointer")(config.password as *mut libc::c_void);
        config.password = 0 as *mut libc::c_char;
    }
    if !(config.post_data).is_null() {
        wget_free
            .expect("non-null function pointer")(config.post_data as *mut libc::c_void);
        config.post_data = 0 as *const libc::c_char;
    }
    if !(config.post_file).is_null() {
        wget_free
            .expect("non-null function pointer")(config.post_file as *mut libc::c_void);
        config.post_file = 0 as *const libc::c_char;
    }
    if !(config.private_key).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.private_key as *mut libc::c_void);
        config.private_key = 0 as *const libc::c_char;
    }
    if !(config.random_file).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.random_file as *mut libc::c_void);
        config.random_file = 0 as *const libc::c_char;
    }
    if !(config.referer).is_null() {
        wget_free
            .expect("non-null function pointer")(config.referer as *mut libc::c_void);
        config.referer = 0 as *const libc::c_char;
    }
    if !(config.reject_regex).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.reject_regex as *mut libc::c_void);
        config.reject_regex = 0 as *const libc::c_char;
    }
    if !(config.remote_encoding).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.remote_encoding as *mut libc::c_void);
        config.remote_encoding = 0 as *const libc::c_char;
    }
    if !(config.save_cookies).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.save_cookies as *mut libc::c_void);
        config.save_cookies = 0 as *const libc::c_char;
    }
    if !(config.secure_protocol).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.secure_protocol as *mut libc::c_void);
        config.secure_protocol = 0 as *const libc::c_char;
    }
    if !(config.tls_session_file).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.tls_session_file as *mut libc::c_void);
        config.tls_session_file = 0 as *const libc::c_char;
    }
    if !(config.user_agent).is_null() {
        wget_free
            .expect("non-null function pointer")(config.user_agent as *mut libc::c_void);
        config.user_agent = 0 as *const libc::c_char;
    }
    if !(config.use_askpass_bin).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.use_askpass_bin as *mut libc::c_void);
        config.use_askpass_bin = 0 as *const libc::c_char;
    }
    if !(config.username).is_null() {
        wget_free
            .expect("non-null function pointer")(config.username as *mut libc::c_void);
        config.username = 0 as *mut libc::c_char;
    }
    if !(config.gnupg_homedir).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.gnupg_homedir as *mut libc::c_void);
        config.gnupg_homedir = 0 as *const libc::c_char;
    }
    if !(config.stats_all).is_null() {
        wget_free
            .expect("non-null function pointer")(config.stats_all as *mut libc::c_void);
        config.stats_all = 0 as *const libc::c_char;
    }
    if !(config.user_config).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.user_config as *mut libc::c_void);
        config.user_config = 0 as *const libc::c_char;
    }
    if !(config.system_config).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )(config.system_config as *mut libc::c_void);
        config.system_config = 0 as *const libc::c_char;
    }
    if !(config.stats_dns_args).is_null() {
        if !((*config.stats_dns_args).fp).is_null()
            && (*config.stats_dns_args).fp != stdout
        {
            rpl_fclose((*config.stats_dns_args).fp);
        }
        if !((*config.stats_dns_args).filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*config.stats_dns_args).filename as *mut libc::c_void);
            (*config.stats_dns_args).filename = 0 as *const libc::c_char;
        }
        if !(config.stats_dns_args).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.stats_dns_args as *mut libc::c_void);
            config.stats_dns_args = 0 as *mut stats_args;
        }
    }
    if !(config.stats_ocsp_args).is_null() {
        if !((*config.stats_ocsp_args).fp).is_null()
            && (*config.stats_ocsp_args).fp != stdout
        {
            rpl_fclose((*config.stats_ocsp_args).fp);
        }
        if !((*config.stats_ocsp_args).filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*config.stats_ocsp_args).filename as *mut libc::c_void);
            (*config.stats_ocsp_args).filename = 0 as *const libc::c_char;
        }
        if !(config.stats_ocsp_args).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.stats_ocsp_args as *mut libc::c_void);
            config.stats_ocsp_args = 0 as *mut stats_args;
        }
    }
    if !(config.stats_server_args).is_null() {
        if !((*config.stats_server_args).fp).is_null() {
            server_stats_exit();
        }
        if !((*config.stats_server_args).fp).is_null()
            && (*config.stats_server_args).fp != stdout
        {
            rpl_fclose((*config.stats_server_args).fp);
        }
        if !((*config.stats_server_args).filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*config.stats_server_args).filename as *mut libc::c_void);
            (*config.stats_server_args).filename = 0 as *const libc::c_char;
        }
        if !(config.stats_server_args).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.stats_server_args as *mut libc::c_void);
            config.stats_server_args = 0 as *mut stats_args;
        }
    }
    if !(config.stats_site_args).is_null() {
        if !((*config.stats_site_args).fp).is_null() {
            site_stats_exit();
        }
        if !((*config.stats_site_args).fp).is_null()
            && (*config.stats_site_args).fp != stdout
        {
            rpl_fclose((*config.stats_site_args).fp);
        }
        if !((*config.stats_site_args).filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*config.stats_site_args).filename as *mut libc::c_void);
            (*config.stats_site_args).filename = 0 as *const libc::c_char;
        }
        if !(config.stats_site_args).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.stats_site_args as *mut libc::c_void);
            config.stats_site_args = 0 as *mut stats_args;
        }
    }
    if !(config.stats_tls_args).is_null() {
        if !((*config.stats_tls_args).fp).is_null()
            && (*config.stats_tls_args).fp != stdout
        {
            rpl_fclose((*config.stats_tls_args).fp);
        }
        if !((*config.stats_tls_args).filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*config.stats_tls_args).filename as *mut libc::c_void);
            (*config.stats_tls_args).filename = 0 as *const libc::c_char;
        }
        if !(config.stats_tls_args).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(config.stats_tls_args as *mut libc::c_void);
            config.stats_tls_args = 0 as *mut stats_args;
        }
    }
    wget_iri_free(&mut config.base);
    wget_vector_free(&mut config.exclude_directories);
    wget_vector_free(&mut config.save_content_on);
    wget_vector_free(&mut config.mime_types);
    wget_vector_free(&mut config.retry_on_http_error);
    wget_vector_free(&mut config.domains);
    wget_vector_free(&mut config.exclude_domains);
    wget_vector_free(&mut config.follow_tags);
    wget_vector_free(&mut config.ignore_tags);
    wget_vector_free(&mut config.accept_patterns);
    wget_vector_free(&mut config.reject_patterns);
    wget_vector_free(&mut config.headers);
    wget_vector_free(&mut config.default_challenges);
    wget_vector_free(&mut config.compression);
    wget_http_set_http_proxy(0 as *const libc::c_char, 0 as *const libc::c_char);
    wget_http_set_https_proxy(0 as *const libc::c_char, 0 as *const libc::c_char);
    wget_http_set_no_proxy(0 as *const libc::c_char, 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn selftest_options() -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut it: size_t = 0;
    it = 1 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
    {
        if opt_compare(
            (options[it.wrapping_sub(1 as libc::c_int as size_t) as usize].long_name)
                .as_ptr() as *const libc::c_void,
            &*options.as_ptr().offset(it as isize) as *const optionw
                as *const libc::c_void,
        ) > 0 as libc::c_int
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Option not in order '%s' after '%s' (using opt_compare())\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                (options[it as usize].long_name).as_ptr(),
                (options[it.wrapping_sub(1 as libc::c_int as size_t) as usize].long_name)
                    .as_ptr(),
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    it = 1 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
    {
        if opt_compare_config(
            (options[it.wrapping_sub(1 as libc::c_int as size_t) as usize].long_name)
                .as_ptr() as *const libc::c_void,
            &*options.as_ptr().offset(it as isize) as *const optionw
                as *const libc::c_void,
        ) > 0 as libc::c_int
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Option not in order '%s' after '%s' (using opt_compare_config())\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                (options[it as usize].long_name).as_ptr(),
                (options[it.wrapping_sub(1 as libc::c_int as size_t) as usize].long_name)
                    .as_ptr(),
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
    {
        let mut opt: option_t = bsearch(
            (options[it as usize].long_name).as_ptr() as *const libc::c_void,
            options.as_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
            ::core::mem::size_of::<optionw>() as libc::c_ulong,
            Some(
                opt_compare
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as option_t;
        if opt.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to find option '%s' (using opt_compare())\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                (options[it as usize].long_name).as_ptr(),
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
    {
        let mut opt_0: option_t = bsearch(
            (options[it as usize].long_name).as_ptr() as *const libc::c_void,
            options.as_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
            ::core::mem::size_of::<optionw>() as libc::c_ulong,
            Some(
                opt_compare_config
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as option_t;
        if opt_0.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to find option '%s' (using opt_compare_config())\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                (options[it as usize].long_name).as_ptr(),
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    static mut test_command: [*const libc::c_char; 6] = [
        b"httpproxy\0" as *const u8 as *const libc::c_char,
        b"http_proxy\0" as *const u8 as *const libc::c_char,
        b"http-proxy\0" as *const u8 as *const libc::c_char,
        b"Httpproxy\0" as *const u8 as *const libc::c_char,
        b"Http_proxy\0" as *const u8 as *const libc::c_char,
        b"Http-proxy\0" as *const u8 as *const libc::c_char,
    ];
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[*const libc::c_char; 6]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        let mut opt_1: option_t = bsearch(
            test_command[it as usize] as *const libc::c_void,
            options.as_ptr() as *const libc::c_void,
            (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong),
            ::core::mem::size_of::<optionw>() as libc::c_ulong,
            Some(
                opt_compare_config
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as option_t;
        if opt_1.is_null() {
            let mut it2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while (it2 as libc::c_ulong)
                < (::core::mem::size_of::<[optionw; 189]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<optionw>() as libc::c_ulong)
                && opt_1.is_null()
            {
                if opt_compare_config_linear(
                    test_command[it as usize],
                    (options[it2 as usize].long_name).as_ptr(),
                ) == 0 as libc::c_int
                {
                    opt_1 = &*options.as_ptr().offset(it2 as isize) as *const optionw;
                }
                it2 = it2.wrapping_add(1);
                it2;
            }
            if opt_1.is_null() {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Failed to find option '%s' (using opt_compare_config())\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*::core::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"selftest_options\0"))
                        .as_ptr(),
                    test_command[it as usize],
                );
                ret = 1 as libc::c_int;
            }
        }
        it = it.wrapping_add(1);
        it;
    }
    static mut test_bool_short: [C2RustUnnamed_7; 1] = [
        {
            let mut init = C2RustUnnamed_7 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-r\0" as *const u8 as *const libc::c_char,
                    b"-\0" as *const u8 as *const libc::c_char,
                ],
                result: 1 as libc::c_int != 0,
            };
            init
        },
    ];
    let mut recursive: bool = config.recursive;
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_7; 1]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_7>() as libc::c_ulong)
    {
        parse_command_line(
            3 as libc::c_int,
            (test_bool_short[it as usize].argv).as_mut_ptr(),
        );
        if config.recursive as libc::c_int
            != test_bool_short[it as usize].result as libc::c_int
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse bool short option #%zu (=%d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.recursive as libc::c_int,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    static mut test_bool: [C2RustUnnamed_6; 10] = [
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 1 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--no-recursive\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=y\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 1 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=n\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=1\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 1 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=0\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=yes\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 1 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=no\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 0 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=on\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 1 as libc::c_int as libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_6 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--recursive=off\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 0 as libc::c_int as libc::c_char,
            };
            init
        },
    ];
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_6; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_6>() as libc::c_ulong)
    {
        parse_command_line(2 as libc::c_int, (test_bool[it as usize].argv).as_mut_ptr());
        if config.recursive as libc::c_int
            != test_bool[it as usize].result as libc::c_int
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse bool long option #%zu (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.recursive as libc::c_int,
            );
            ret = 1 as libc::c_int;
        }
        parse_command_line(3 as libc::c_int, (test_bool[it as usize].argv).as_mut_ptr());
        if config.recursive as libc::c_int
            != test_bool[it as usize].result as libc::c_int
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse bool long option #%zu (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.recursive as libc::c_int,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    config.recursive = recursive;
    static mut test_timeout_short: [C2RustUnnamed_5; 14] = [
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"123\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"-1\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"inf\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"infinity\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"0\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"+123\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T\0" as *const u8 as *const libc::c_char,
                    b"60.2\0" as *const u8 as *const libc::c_char,
                ],
                result: 60200 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T123\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T-1\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-Tinf\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-Tinfinity\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T0\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T+123\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_5 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-T60.2\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 60200 as libc::c_int,
            };
            init
        },
    ];
    let mut dns_timeout: libc::c_int = config.dns_timeout;
    let mut connect_timeout: libc::c_int = config.connect_timeout;
    let mut read_timeout: libc::c_int = config.read_timeout;
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_5; 14]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong)
    {
        config.dns_timeout = 555 as libc::c_int;
        parse_command_line(
            3 as libc::c_int,
            (test_timeout_short[it as usize].argv).as_mut_ptr(),
        );
        if config.dns_timeout != test_timeout_short[it as usize].result {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse timeout short option #%zu (=%d)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.dns_timeout,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    static mut test_timeout: [C2RustUnnamed_4; 14] = [
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"123\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"-1\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"inf\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"infinity\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"0\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"+123\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout\0" as *const u8 as *const libc::c_char,
                    b"60.2\0" as *const u8 as *const libc::c_char,
                ],
                result: 60200 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=123\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=-1\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=inf\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=infinity\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=0\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: -(1 as libc::c_int),
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=+123\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 123000 as libc::c_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_4 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--timeout=60.2\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 60200 as libc::c_int,
            };
            init
        },
    ];
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_4; 14]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong)
    {
        config.dns_timeout = 555 as libc::c_int;
        parse_command_line(
            3 as libc::c_int,
            (test_timeout[it as usize].argv).as_mut_ptr(),
        );
        if config.dns_timeout != test_timeout[it as usize].result {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse timeout long option #%zu (%d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.dns_timeout,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    config.dns_timeout = dns_timeout;
    config.connect_timeout = connect_timeout;
    config.read_timeout = read_timeout;
    static mut test_header: [C2RustUnnamed_3; 3] = [
        {
            let mut init = C2RustUnnamed_3 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b"Hello: World\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: [
                    b"Hello\0" as *const u8 as *const libc::c_char,
                    b"World\0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header=Hello: World\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: [0 as *const libc::c_char, 0 as *const libc::c_char],
            };
            init
        },
        {
            let mut init = C2RustUnnamed_3 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header=Hello: World\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header=Test: Passed\0" as *const u8 as *const libc::c_char,
                ],
                result: [
                    b"Test\0" as *const u8 as *const libc::c_char,
                    b"Passed\0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
    ];
    wget_vector_clear(config.headers);
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_3; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_3>() as libc::c_ulong)
    {
        let mut res_name: *const libc::c_char = test_header[it as usize]
            .result[0 as libc::c_int as usize];
        let mut res_value: *const libc::c_char = test_header[it as usize]
            .result[1 as libc::c_int as usize];
        parse_command_line(
            5 as libc::c_int,
            (test_header[it as usize].argv).as_mut_ptr(),
        );
        let mut config_value: *mut wget_http_header_param = wget_vector_get(
            config.headers,
            0 as libc::c_int,
        ) as *mut wget_http_header_param;
        if res_name.is_null() {
            if wget_vector_size(config.headers) != 0 as libc::c_int {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Extra headers found in option #%zu\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*::core::mem::transmute::<
                        &[u8; 17],
                        &[libc::c_char; 17],
                    >(b"selftest_options\0"))
                        .as_ptr(),
                    it,
                );
                ret = 1 as libc::c_int;
            }
        } else if wget_strcmp((*config_value).name, res_name) != 0
            && wget_strcmp((*config_value).value, res_value) != 0
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse header option #%zu\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    static mut test_header_illegal: [C2RustUnnamed_2; 5] = [
        {
            let mut init = C2RustUnnamed_2 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b"Hello World\0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b"Hello:\0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b"Hello:  \0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b":World\0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = C2RustUnnamed_2 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--header\0" as *const u8 as *const libc::c_char,
                    b":\0" as *const u8 as *const libc::c_char,
                ],
            };
            init
        },
    ];
    wget_vector_clear(config.headers);
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_2; 5]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_2>() as libc::c_ulong)
    {
        parse_command_line(
            3 as libc::c_int,
            (test_header_illegal[it as usize].argv).as_mut_ptr(),
        );
        if wget_vector_size(config.headers) != 0 as libc::c_int {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Accepted illegal header option #%zu\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    wget_vector_clear(config.headers);
    static mut test_string_short: [C2RustUnnamed_1; 2] = [
        {
            let mut init = C2RustUnnamed_1 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-U\0" as *const u8 as *const libc::c_char,
                    b"hello1\0" as *const u8 as *const libc::c_char,
                ],
                result: b"hello1\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"-Uhello2\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: b"hello2\0" as *const u8 as *const libc::c_char,
            };
            init
        },
    ];
    let mut user_agent: *const libc::c_char = config.user_agent;
    config.user_agent = 0 as *const libc::c_char;
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_1; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        parse_command_line(
            3 as libc::c_int,
            (test_string_short[it as usize].argv).as_mut_ptr(),
        );
        if wget_strcmp(config.user_agent, test_string_short[it as usize].result) != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse string short option #%zu (=%s)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.user_agent,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    static mut test_string: [C2RustUnnamed_0; 3] = [
        {
            let mut init = C2RustUnnamed_0 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--user-agent\0" as *const u8 as *const libc::c_char,
                    b"hello3\0" as *const u8 as *const libc::c_char,
                ],
                result: b"hello3\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--user-agent=hello4\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: b"hello4\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_0 {
                argv: [
                    b"\0" as *const u8 as *const libc::c_char,
                    b"--no-user-agent\0" as *const u8 as *const libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                ],
                result: 0 as *const libc::c_char,
            };
            init
        },
    ];
    it = 0 as libc::c_int as size_t;
    while it
        < (::core::mem::size_of::<[C2RustUnnamed_0; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong)
    {
        parse_command_line(
            3 as libc::c_int,
            (test_string[it as usize].argv).as_mut_ptr(),
        );
        if wget_strcmp(config.user_agent, test_string[it as usize].result) != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Failed to parse string short option #%zu (=%s)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 17],
                    &[libc::c_char; 17],
                >(b"selftest_options\0"))
                    .as_ptr(),
                it,
                config.user_agent,
            );
            ret = 1 as libc::c_int;
        }
        it = it.wrapping_add(1);
        it;
    }
    if !(config.user_agent).is_null() {
        wget_free
            .expect("non-null function pointer")(config.user_agent as *mut libc::c_void);
        config.user_agent = 0 as *const libc::c_char;
    }
    config.user_agent = user_agent;
    return ret;
}
unsafe extern "C" fn no_memory() -> ! {
    fputs(b"No memory\n\0" as *const u8 as *const libc::c_char, stderr);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn my_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if !p.is_null() {
        return p;
    }
    no_memory();
}
unsafe extern "C" fn my_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = calloc(nmemb, size);
    if !p.is_null() {
        return p;
    }
    no_memory();
}
unsafe extern "C" fn my_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = realloc(ptr, size);
    if !p.is_null() || !ptr.is_null() && size == 0 as libc::c_int as size_t {
        return p;
    }
    no_memory();
}
unsafe extern "C" fn my_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
unsafe extern "C" fn set_allocation_functions() {
    wget_malloc_fn = Some(
        my_malloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
    );
    wget_calloc_fn = Some(
        my_calloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
    );
    wget_realloc_fn = Some(
        my_realloc
            as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    );
    wget_free = Some(my_free as unsafe extern "C" fn(*mut libc::c_void) -> ());
}
unsafe extern "C" fn run_static_initializers() {
    options = [
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"accept\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.accept_patterns as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'A' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Comma-separated list of file name suffixes or\n\0" as *const u8
                        as *const libc::c_char,
                    b"patterns.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"accept-regex\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.accept_regex as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Regex matching accepted URLs.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"adjust-extension\0\0\0\0\0\0"),
                var: &mut config.adjust_extension as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'E' as i32 as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Append extension to saved file (.html or .css).\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"append-output\0\0\0\0\0\0\0\0\0"),
                var: &mut config.logfile_append as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'a' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"File where messages are appended to, '-' for STDOUT\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ask-password\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.askpass as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Print prompt for password\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"auth-no-challenge\0\0\0\0\0"),
                var: &mut config.auth_no_challenge as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"send Basic HTTP Authentication before challenge\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"background\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.background as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'b' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Go to background immediately after startup. If no\n\0" as *const u8
                        as *const libc::c_char,
                    b"output file is specified via the -o, output is redirected to wget-log\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"backup-converted\0\0\0\0\0\0"),
                var: &mut config.backup_converted as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'K' as i32 as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"When converting, keep the original file with\n\0" as *const u8
                        as *const libc::c_char,
                    b"a .orig suffix. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"backups\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.backups as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Make backups instead of overwriting/increasing\n\0" as *const u8
                        as *const libc::c_char,
                    b"number. (default: 0)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"base\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.base_url as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'B' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Base for relative URLs read from input-file\n\0" as *const u8
                        as *const libc::c_char,
                    b"or from command line\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"bind-address\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.bind_address as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Bind to sockets to local address.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: automatic)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"bind-interface\0\0\0\0\0\0\0\0"),
                var: &mut config.bind_interface as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Bind sockets to the input Network Interface.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: automatic)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"body-data\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.body_data as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Data to be sent in a request.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"body-file\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.body_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"File with data to be sent in a request.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ca-certificate\0\0\0\0\0\0\0\0"),
                var: &mut config.ca_cert as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"File with bundle of PEM CA certificates.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ca-directory\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ca_directory as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Directory with PEM CA certificates.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.cache as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Enabled using of server cache. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"certificate\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.cert_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"File with client certificate.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"certificate-type\0\0\0\0\0\0"),
                var: &mut config.cert_type as *mut libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_cert_type
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Certificate type: PEM or DER (known as ASN1).\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: PEM)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"check-certificate\0\0\0\0\0"),
                var: &mut config.check_certificate as *mut check_certificate_mode
                    as *mut libc::c_void,
                parser: Some(
                    parse_check_certificate
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Check the server's certificate. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"check-hostname\0\0\0\0\0\0\0\0"),
                var: &mut config.check_hostname as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Check the server's certificate's hostname.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"chunk-size\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.chunk_size as *mut libc::c_longlong
                    as *mut libc::c_void,
                parser: Some(
                    parse_numbytes
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Download large files in multithreaded chunks.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: 0 (=off)) Example:\n\0" as *const u8
                        as *const libc::c_char,
                    b"wget --chunk-size=1M\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.clobber as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Enable file clobbering. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"compression\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.compression as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_compression
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Customize Accept-Encoding with\n\0" as *const u8
                        as *const libc::c_char,
                    b"identity, gzip, deflate, xz, lzma, br, bzip2, zstd, lzip\n\0"
                        as *const u8 as *const libc::c_char,
                    b"and any combination of it\n\0" as *const u8 as *const libc::c_char,
                    b"no-compression means no Accept-Encoding\n\0" as *const u8
                        as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.user_config as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Path to initialization file (default: ~/.config/wget/wget2rc)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"connect-timeout\0\0\0\0\0\0\0"),
                var: &mut config.connect_timeout as *mut libc::c_int
                    as *mut libc::c_void,
                parser: Some(
                    parse_timeout
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Connect timeout in seconds.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"content-disposition\0\0\0"),
                var: &mut config.content_disposition as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Take filename from Content-Disposition.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"content-on-error\0\0\0\0\0\0"),
                var: &mut config.content_on_error as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Save response body even on error status.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"continue\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.continue_download as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'c' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Continue download for given files. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"convert-file-only\0\0\0\0\0"),
                var: &mut config.convert_file_only as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Convert only filename part of embedded URLs.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"convert-links\0\0\0\0\0\0\0\0\0"),
                var: &mut config.convert_links as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'k' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Convert embedded URLs to local URLs.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"cookie-suffixes\0\0\0\0\0\0\0"),
                var: &mut config.cookie_suffixes as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Load public suffixes from file. \n\0" as *const u8
                        as *const libc::c_char,
                    b"They prevent 'supercookie' vulnerabilities.\n\0" as *const u8
                        as *const libc::c_char,
                    b"See https://publicsuffix.org/ for details.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.cookies as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Enable use of cookies. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"crl-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.crl_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"File with PEM CRL certificates.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"cut-dirs\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.cut_directories as *mut libc::c_int
                    as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DIRECTORY,
                help_str: [
                    b"Skip creating given number of directory\n\0" as *const u8
                        as *const libc::c_char,
                    b"components. (default: 0)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"cut-file-get-vars\0\0\0\0\0"),
                var: &mut config.cut_file_get_vars as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Cut HTTP GET vars from file names. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"cut-url-get-vars\0\0\0\0\0\0"),
                var: &mut config.cut_url_get_vars as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Cut HTTP GET vars from URLs. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"dane\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.dane as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Enable DANE certificate checking.(default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"debug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.debug as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'd' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print debugging messages.(default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"default-http-port\0\0\0\0\0"),
                var: &mut config.default_http_port as *mut uint16_t as *mut libc::c_void,
                parser: Some(
                    parse_uint16
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Set default port for HTTP. (default: 80)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"default-https-port\0\0\0\0"),
                var: &mut config.default_https_port as *mut uint16_t
                    as *mut libc::c_void,
                parser: Some(
                    parse_uint16
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Set default port for HTTPS. (default: 443)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"default-page\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.default_page as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Default file name. (default: index.html)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"delete-after\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.delete_after as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Don't save downloaded files. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"directories\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.directories as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DIRECTORY,
                help_str: [
                    b"Create hierarchy of directories when retrieving\n\0" as *const u8
                        as *const libc::c_char,
                    b"recursively. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"directory-prefix\0\0\0\0\0\0"),
                var: &mut config.directory_prefix as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'P' as i32 as libc::c_char,
                section: SECTION_DIRECTORY,
                help_str: [
                    b"Set directory prefix.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"dns-cache\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.dns_caching as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Caching of domain name lookups. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"dns-cache-preload\0\0\0\0\0"),
                var: &mut config.dns_cache_preload as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"File to be used to preload the DNS cache.\n\0" as *const u8
                        as *const libc::c_char,
                    b"Format is like /etc/hosts (IP<whitespace>hostname).\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"dns-timeout\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.dns_timeout as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_timeout
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"DNS lookup timeout in seconds.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"domains\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.domains as *mut *mut wget_vector as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'D' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Comma-separated list of domains to follow.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"download-attr\0\0\0\0\0\0\0\0\0"),
                var: &mut config.download_attr as *mut libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_download_attr
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Recognize HTML5 download attributes.\n\0" as *const u8
                        as *const libc::c_char,
                    b"'strippath' strips the path to be more secure.\n'usepath' uses the path as is (this can be extremely dangerous !).\n(default: strippath)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"egd-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.egd_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"File to be used as socket for random data from\n\0" as *const u8
                        as *const libc::c_char,
                    b"Entropy Gathering Daemon.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"exclude-directories\0\0\0"),
                var: &mut config.exclude_directories as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_excluded_directories
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'X' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Comma-separated list of directories NOT to download.\n\0"
                        as *const u8 as *const libc::c_char,
                    b"Wildcards are allowed.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"exclude-domains\0\0\0\0\0\0\0"),
                var: &mut config.exclude_domains as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Comma-separated list of domains NOT to follow.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"execute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_execute
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'e' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Wget compatibility option, not needed for Wget\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"filter-mime-type\0\0\0\0\0\0"),
                var: &mut config.mime_types as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Specify a list of mime types to be saved or ignored\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"filter-urls\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.filter_urls as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Apply the accept and reject filters on the URL\n\0" as *const u8
                        as *const libc::c_char,
                    b"before starting a download. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"follow-sitemaps\0\0\0\0\0\0\0"),
                var: &mut config.follow_sitemaps as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Scan sitemaps found in robots.txt. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"follow-tags\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.follow_tags as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_taglist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Scan additional tag/attributes for URLs,\n\0" as *const u8
                        as *const libc::c_char,
                    b"e.g. --follow-tags=\"img/data-500px,img/data-hires\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-atom\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.force_atom as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Treat input file as Atom Feed. (default: off) (NEW!)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-css\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.force_css as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Treat input file as CSS. (default: off) (NEW!)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-directories\0\0\0\0\0"),
                var: &mut config.force_directories as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'x' as i32 as libc::c_char,
                section: SECTION_DIRECTORY,
                help_str: [
                    b"Create hierarchy of directories when not\n\0" as *const u8
                        as *const libc::c_char,
                    b"retrieving recursively. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-html\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.force_html as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'F' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Treat input file as HTML. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-metalink\0\0\0\0\0\0\0\0"),
                var: &mut config.force_metalink as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Treat input file as Metalink. (default: off) (NEW!)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-progress\0\0\0\0\0\0\0\0"),
                var: &mut config.force_progress as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Force progress bar.\n\0" as *const u8 as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-rss\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.force_rss as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Treat input file as RSS Feed. (default: off) (NEW!)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"force-sitemap\0\0\0\0\0\0\0\0\0"),
                var: &mut config.force_sitemap as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Treat input file as Sitemap. (default: off) (NEW!)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"fsync-policy\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.fsync_policy as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Use fsync() to wait for data being written to\n\0" as *const u8
                        as *const libc::c_char,
                    b"the physical layer. (default: off) (NEW!)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.headers as *mut *mut wget_vector as *mut libc::c_void,
                parser: Some(
                    parse_header
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Insert input string as a HTTP header in\n\0" as *const u8
                        as *const libc::c_char,
                    b"all requests\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"help\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    print_help
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 0 as libc::c_int,
                short_name: 'h' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print this help.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"host-directories\0\0\0\0\0\0"),
                var: &mut config.host_directories as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DIRECTORY,
                help_str: [
                    b"Create host directories when retrieving\n\0" as *const u8
                        as *const libc::c_char,
                    b"recursively. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hpkp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.hpkp as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use HTTP Public Key Pinning (HPKP). (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hpkp-file\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.hpkp_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set file for storing HPKP data\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: $XDG_DATA_HOME/wget/.wget-hpkp)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hsts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.hsts as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use HTTP Strict Transport Security (HSTS).\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hsts-file\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.hsts_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set file for HSTS caching. (default: $XDG_DATA_HOME/wget/.wget-hsts)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hsts-preload\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.hsts_preload as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use HTTP Strict Transport Security (HSTS).\n\0" as *const u8
                        as *const libc::c_char,
                    b"[Not built with libhsts, so not functional]\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hsts-preload-file\0\0\0\0\0"),
                var: &mut config.hsts_preload_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set name for the HSTS Preload file (DAFSA format).\n\0"
                        as *const u8 as *const libc::c_char,
                    b"[Not built with libhsts, so not functional]\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"html-extension\0\0\0\0\0\0\0\0"),
                var: &mut config.adjust_extension as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Obsoleted by --adjust-extension\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http-keep-alive\0\0\0\0\0\0\0"),
                var: &mut config.keep_alive as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Keep connection open for further requests.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http-password\0\0\0\0\0\0\0\0\0"),
                var: &mut config.http_password as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Password for HTTP Authentication.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: empty password)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http-proxy\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.http_proxy as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Set HTTP proxy/proxies, overriding environment\n\0" as *const u8
                        as *const libc::c_char,
                    b"variables. Use comma to separate proxies.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http-proxy-password\0\0\0"),
                var: &mut config.http_proxy_password as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Password for HTTP Proxy Authentication.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: empty password)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http-proxy-user\0\0\0\0\0\0\0"),
                var: &mut config.http_proxy_username as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Username for HTTP Proxy Authentication.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: empty username)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http-user\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.http_username as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Username for HTTP Authentication.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: empty username)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.http2 as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use HTTP/2 protocol if possible. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http2-only\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.http2_only as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Only use HTTP/2 protocol, error if server doesn't offer it. (default: off)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"http2-request-window\0\0"),
                var: &mut config.http2_request_window as *mut libc::c_int
                    as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Max. number of parallel streams per HTTP/2\n\0" as *const u8
                        as *const libc::c_char,
                    b"connection. (default: 30)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"https-enforce\0\0\0\0\0\0\0\0\0"),
                var: &mut config.https_enforce as *mut https_enforce_mode
                    as *mut libc::c_void,
                parser: Some(
                    parse_https_enforce
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use secure HTTPS instead of HTTP. Legal types are\n\0" as *const u8
                        as *const libc::c_char,
                    b"'hard', 'soft' and 'none'.\n\0" as *const u8
                        as *const libc::c_char,
                    b"If --https-only is enabled,\n\0" as *const u8
                        as *const libc::c_char,
                    b"this option has no effect. (default: none)\n\0" as *const u8
                        as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"https-only\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.https_only as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Do not follow non-secure URLs. (default: off).\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"https-proxy\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.https_proxy as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set HTTPS proxy/proxies, overriding environment\n\0" as *const u8
                        as *const libc::c_char,
                    b"variables. Use comma to separate proxies.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"hyperlink\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.hyperlink as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Enable terminal hyperlink support\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"if-modified-since\0\0\0\0\0"),
                var: &mut config.if_modified_since as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Do not send If-Modified-Since header in -N mode.\nSend preliminary HEAD request instead. This has only\n\0"
                        as *const u8 as *const libc::c_char,
                    b"effect in -N mode.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ignore-case\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ignore_case as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Ignore case when matching files. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ignore-length\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ignore_length as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Ignore content-length header field\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ignore-tags\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ignore_tags as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_taglist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Ignore tag/attributes for URL scanning,\n\0" as *const u8
                        as *const libc::c_char,
                    b"e.g. --ignore-tags=\"img,a/href\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"include-directories\0\0\0"),
                var: &mut config.exclude_directories as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_included_directories
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'I' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Comma-separated list of directories TO download.\n\0" as *const u8
                        as *const libc::c_char,
                    b"Wildcards are allowed.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"inet4-only\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.inet4_only as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: '4' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Use IPv4 connections only. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"inet6-only\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.inet6_only as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: '6' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Use IPv6 connections only. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"input-encoding\0\0\0\0\0\0\0\0"),
                var: &mut config.input_encoding as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Character encoding of the file contents read with\n\0" as *const u8
                        as *const libc::c_char,
                    b"--input-file. (default: local encoding)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"input-file\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.input_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'i' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"File where URLs are read from, - for STDIN.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"iri\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Wget dummy option, you can't switch off\n\0" as *const u8
                        as *const libc::c_char,
                    b"international support\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"keep-extension\0\0\0\0\0\0\0\0"),
                var: &mut config.keep_extension as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"If file exists: Use pattern 'basename_N.ext'\n\0" as *const u8
                        as *const libc::c_char,
                    b"instead of 'filename.N'. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"keep-session-cookies\0\0"),
                var: &mut config.keep_session_cookies as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Also save session cookies. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"level\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.level as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'l' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Maximum recursion depth. (default: 5)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"limit-rate\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.limit_rate as *mut libc::c_longlong
                    as *mut libc::c_void,
                parser: Some(
                    parse_numbytes
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Limit rate of download per second, 0 = no limit. (default: 0)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"list-plugins\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    list_plugins
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 0 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_PLUGIN,
                help_str: [
                    b"Lists all the plugins in the plugin search paths.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"load-cookies\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.load_cookies as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Load cookies from file.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"local-db\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.local_db as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_local_db
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Read or load databases\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"local-encoding\0\0\0\0\0\0\0\0"),
                var: &mut config.local_encoding as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Character encoding of environment and filenames.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"local-plugin\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_plugin_local
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_PLUGIN,
                help_str: [
                    b"Loads a plugin with a given path.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"max-redirect\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.max_redirect as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Max. number of redirections to follow.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: 20)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"max-threads\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.max_threads as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Max. concurrent download threads.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: 5) (NEW!)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"metalink\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.metalink as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Follow a metalink file instead of storing it\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"method\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.method as *mut *const libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"HTTP method to use for request.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"mirror\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.mirror as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_mirror
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'm' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Turn on mirroring options -r -N -l inf\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_n_option
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'n' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Special compatibility option\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"netrc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.netrc as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Load credentials from ~/.netrc if not given.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"netrc-file\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.netrc_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Set file for login/password to use instead of\n\0" as *const u8
                        as *const libc::c_char,
                    b"~/.netrc. (default: ~/.netrc)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ocsp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ocsp as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use OCSP server access to verify server's\n\0" as *const u8
                        as *const libc::c_char,
                    b"certificate. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ocsp-date\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ocsp_date as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Check if OCSP response date is too old.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ocsp-file\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ocsp_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set file for OCSP chaching.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: $XDG_DATA_HOME/wget/.wget-ocsp)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ocsp-nonce\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ocsp_nonce as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Allow nonce checking when verifying OCSP\n\0" as *const u8
                        as *const libc::c_char,
                    b"response. (default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ocsp-server\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ocsp_server as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set OCSP server address.\n\0" as *const u8 as *const libc::c_char,
                    b"(default: OCSP server given in certificate)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"ocsp-stapling\0\0\0\0\0\0\0\0\0"),
                var: &mut config.ocsp_stapling as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Use OCSP stapling to verify the server's\n\0" as *const u8
                        as *const libc::c_char,
                    b"certificate. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"output-document\0\0\0\0\0\0\0"),
                var: &mut config.output_document as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'O' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"File where downloaded content is written to,\n\0" as *const u8
                        as *const libc::c_char,
                    b"'-O'  for STDOUT.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"output-file\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.logfile as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'o' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"File where messages are printed to,\n\0" as *const u8
                        as *const libc::c_char,
                    b"'-' for STDOUT.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"page-requisites\0\0\0\0\0\0\0"),
                var: &mut config.page_requisites as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'p' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Download all necessary files to display a\n\0" as *const u8
                        as *const libc::c_char,
                    b"HTML page\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.parent as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Ascend above parent directory. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"password\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.password as *mut *mut libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Password for Authentication.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: empty password)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"plugin\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_plugin
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_PLUGIN,
                help_str: [
                    b"Load a plugin with a given name.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"plugin-dirs\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_plugin_dirs
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_PLUGIN,
                help_str: [
                    b"Specify alternative directories to look\n\0" as *const u8
                        as *const libc::c_char,
                    b"for plugins, separated by ','\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"plugin-help\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    print_plugin_help
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 0 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_PLUGIN,
                help_str: [
                    b"Print help message for all loaded plugins\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"plugin-opt\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_plugin_option
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_PLUGIN,
                help_str: [
                    b"Forward an option to a loaded plugin.\n\0" as *const u8
                        as *const libc::c_char,
                    b"The option should be in format:\n\0" as *const u8
                        as *const libc::c_char,
                    b"<plugin_name>.<option>[=value]\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"post-data\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.post_data as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Data to be sent in a POST request.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"post-file\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.post_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"File with data to be sent in a POST request.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"prefer-family\0\0\0\0\0\0\0\0\0"),
                var: &mut config.preferred_family as *mut libc::c_int
                    as *mut libc::c_void,
                parser: Some(
                    parse_prefer_family
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Prefer IPv4 or IPv6. (default: none)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"private-key\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.private_key as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"File with private key.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"private-key-type\0\0\0\0\0\0"),
                var: &mut config.private_key_type as *mut libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_cert_type
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Type of the private key (PEM or DER).\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: PEM)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"progress\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.progress as *mut libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_progress_type
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Type of progress bar (bar, none).\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: none)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"protocol-directories\0\0"),
                var: &mut config.protocol_directories as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DIRECTORY,
                help_str: [
                    b"Force creating protocol directories.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.proxy as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_proxy
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Enable support for *_proxy environment variables.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"quiet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.quiet as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'q' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print no messages except debugging messages.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"quota\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.quota as *mut libc::c_longlong as *mut libc::c_void,
                parser: Some(
                    parse_numbytes
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'Q' as i32 as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Download quota, 0 = no quota. (default: 0)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"random-file\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.random_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"File to be used as source of random data.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"random-wait\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.random_wait as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Wait 0.5 up to 1.5*<--wait> seconds between\n\0" as *const u8
                        as *const libc::c_char,
                    b"downloads (per thread). (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"read-timeout\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.read_timeout as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_timeout
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Read and write timeout in seconds.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"recursive\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.recursive as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'r' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Recursive download. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"referer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.referer as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Include Referer: url in HTTP request.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"regex-type\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.regex_type as *mut libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_regex_type
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Regular expression type. Possible types are\n\0" as *const u8
                        as *const libc::c_char,
                    b"posix or pcre. (default: posix)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"reject\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.reject_patterns as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'R' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Comma-separated list of file name suffixes or\n\0" as *const u8
                        as *const libc::c_char,
                    b"patterns.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"reject-regex\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.reject_regex as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Regex matching rejected URLs.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"remote-encoding\0\0\0\0\0\0\0"),
                var: &mut config.remote_encoding as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Character encoding of remote files\n\0" as *const u8
                        as *const libc::c_char,
                    b"(if not specified in Content-Type HTTP header\n\0" as *const u8
                        as *const libc::c_char,
                    b"or in document itself)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"report-speed\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.report_speed as *mut wget_report_speed
                    as *mut libc::c_void,
                parser: Some(
                    parse_report_speed_type
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Output bandwidth as TYPE. TYPE can be bytes\n\0" as *const u8
                        as *const libc::c_char,
                    b"or bits. --progress MUST be used.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"restrict-file-names\0\0\0"),
                var: &mut config.restrict_file_names as *mut libc::c_int
                    as *mut libc::c_void,
                parser: Some(
                    parse_restrict_names
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"unix, windows, nocontrol, ascii, lowercase,\n\0" as *const u8
                        as *const libc::c_char,
                    b"uppercase, none\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"retry-connrefused\0\0\0\0\0"),
                var: &mut config.retry_connrefused as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Consider \"connection refused\" a transient error.\n\0"
                        as *const u8 as *const libc::c_char,
                    b" (default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"retry-on-http-error\0\0\0"),
                var: &mut config.retry_on_http_error as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Specify a list of http statuses in which the download will be retried\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"robots\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.robots as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Respect robots.txt standard for recursive\n\0" as *const u8
                        as *const libc::c_char,
                    b"downloads. (default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"save-content-on\0\0\0\0\0\0\0"),
                var: &mut config.save_content_on as *mut *mut wget_vector
                    as *mut libc::c_void,
                parser: Some(
                    parse_stringlist
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Specify a list of response codes that requires it's\n\0"
                        as *const u8 as *const libc::c_char,
                    b"response body to be saved on error status\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"save-cookies\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.save_cookies as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Save cookies to file.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"save-headers\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.save_headers as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"Save the response headers in front of the response\n\0"
                        as *const u8 as *const libc::c_char,
                    b"data. (default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"secure-protocol\0\0\0\0\0\0\0"),
                var: &mut config.secure_protocol as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set protocol to be used (auto, SSLv3, TLSv1, TLSv1_1, TLSv1_2, TLS1_3, PFS).\n\0"
                        as *const u8 as *const libc::c_char,
                    b"(default: auto). Or use GnuTLS priority\n\0" as *const u8
                        as *const libc::c_char,
                    b"strings, e.g. NORMAL:-VERS-SSL3.0:-RSA\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"server-response\0\0\0\0\0\0\0"),
                var: &mut config.server_response as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'S' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Print the server response headers. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"show-progress\0\0\0\0\0\0\0\0\0"),
                var: &mut config.force_progress as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Show Progress Bar (Deprecated alias for --force-progress)\n\0"
                        as *const u8 as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"span-hosts\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.span_hosts as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'H' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Span hosts that were not given on the\n\0" as *const u8
                        as *const libc::c_char,
                    b"command line. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"spider\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.spider as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Enable web spider mode. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"start-pos\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.start_pos as *mut libc::c_longlong as *mut libc::c_void,
                parser: Some(
                    parse_numbytes
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Start downloading at zero-based position, 0 = option disabled. (default: 0)\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"stats-dns\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.stats_dns_args as *mut *mut stats_args
                    as *mut libc::c_void,
                parser: Some(
                    parse_stats
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print DNS stats. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    b"Additional format supported:\n\0" as *const u8
                        as *const libc::c_char,
                    b"--stats-dns=[FORMAT:]FILE\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"stats-ocsp\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.stats_ocsp_args as *mut *mut stats_args
                    as *mut libc::c_void,
                parser: Some(
                    parse_stats
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print OCSP stats. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    b"Additional format supported:\n\0" as *const u8
                        as *const libc::c_char,
                    b"--stats-ocsp=[FORMAT:]FILE\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"stats-server\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.stats_server_args as *mut *mut stats_args
                    as *mut libc::c_void,
                parser: Some(
                    parse_stats
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print server stats. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    b"Additional format supported:\n\0" as *const u8
                        as *const libc::c_char,
                    b"--stats-server=[FORMAT:]FILE\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"stats-site\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.stats_site_args as *mut *mut stats_args
                    as *mut libc::c_void,
                parser: Some(
                    parse_stats
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print site stats. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    b"Additional format supported:\n\0" as *const u8
                        as *const libc::c_char,
                    b"--stats-site=[FORMAT:]FILE\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"stats-tls\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.stats_tls_args as *mut *mut stats_args
                    as *mut libc::c_void,
                parser: Some(
                    parse_stats
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print TLS stats. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    b"Additional format supported:\n\0" as *const u8
                        as *const libc::c_char,
                    b"--stats-tls=[FORMAT:]FILE\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"strict-comments\0\0\0\0\0\0\0"),
                var: &mut config.strict_comments as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"A dummy option. Parsing always works non-strict.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"tcp-fastopen\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.tcp_fastopen as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Enable TCP Fast Open (TFO). (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    parse_timeout
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'T' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"General network timeout in seconds.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"timestamping\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.timestamping as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'N' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Just retrieve younger files than the local ones.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"tls-false-start\0\0\0\0\0\0\0"),
                var: &mut config.tls_false_start as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Enable TLS False Start (needs GnuTLS 3.5+).\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"tls-resume\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.tls_resume as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Enable TLS Session Resumption. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"tls-session-file\0\0\0\0\0\0"),
                var: &mut config.tls_session_file as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_filename
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_SSL,
                help_str: [
                    b"Set file for TLS Session caching.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: $XDG_DATA_HOME/wget/.wget-session)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"tries\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.tries as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_integer
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 't' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Number of tries for each download. (default 20)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"trust-server-names\0\0\0\0"),
                var: &mut config.trust_server_names as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"On redirection use the server's filename.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: off)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"unlink\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.unlink as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Remove files before clobbering. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"use-askpass\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.use_askpass_bin as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Prompt for a user and password using\n\0" as *const u8
                        as *const libc::c_char,
                    b"the specified command.\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"use-server-timestamps\0"),
                var: &mut config.use_server_timestamps as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Set local file's timestamp to server's timestamp.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: on)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.username as *mut *mut libc::c_char as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Username for Authentication.\n\0" as *const u8
                        as *const libc::c_char,
                    b"(default: empty username)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"user-agent\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.user_agent as *mut *const libc::c_char
                    as *mut libc::c_void,
                parser: Some(
                    parse_string
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'U' as i32 as libc::c_char,
                section: SECTION_HTTP,
                help_str: [
                    b"HTTP User Agent string.\n\0" as *const u8 as *const libc::c_char,
                    b"(default: wget)\n\0" as *const u8 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"verbose\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.verbose as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 'v' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Print more messages. (default: on)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: 0 as *mut libc::c_void,
                parser: Some(
                    print_version
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 0 as libc::c_int,
                short_name: 'V' as i32 as libc::c_char,
                section: SECTION_STARTUP,
                help_str: [
                    b"Display the version of Wget and exit.\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.wait as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_timeout
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 'w' as i32 as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Wait number of seconds between downloads\n\0" as *const u8
                        as *const libc::c_char,
                    b"(per thread). (default: 0)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"waitretry\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.waitretry as *mut libc::c_int as *mut libc::c_void,
                parser: Some(
                    parse_timeout
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: 1 as libc::c_int,
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Wait up to number of seconds after error\n\0" as *const u8
                        as *const libc::c_char,
                    b"(per thread). (default: 10)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
        {
            let mut init = optionw {
                long_name: *::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"xattr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
                var: &mut config.xattr as *mut bool as *mut libc::c_void,
                parser: Some(
                    parse_bool
                        as unsafe extern "C" fn(
                            option_t,
                            *const libc::c_char,
                            libc::c_char,
                        ) -> libc::c_int,
                ),
                args: -(1 as libc::c_int),
                short_name: 0 as libc::c_int as libc::c_char,
                section: SECTION_DOWNLOAD,
                help_str: [
                    b"Save extended file attributes. (default: off)\n\0" as *const u8
                        as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                    0 as *const libc::c_char,
                ],
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
