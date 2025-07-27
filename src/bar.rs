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
    pub type wget_logger_st;
    pub type wget_vector_st;
    pub type wget_thread_st;
    pub type wget_cookie_db_st;
    pub type wget_hsts_db_st;
    pub type wget_hpkp_db_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    pub type wget_bar_st;
    fn wget_millisleep(ms: libc::c_int);
    fn wget_logger_set_func(logger: *mut wget_logger, func: Option::<wget_logger_func>);
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_thread_start(
        thread: *mut wget_thread,
        start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        arg: *mut libc::c_void,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn wget_thread_join(thread: *mut wget_thread) -> libc::c_int;
    fn wget_thread_support() -> bool;
    fn wget_bar_init(bar_0: *mut wget_bar, nslots: libc::c_int) -> *mut wget_bar;
    fn wget_bar_free(bar_0: *mut *mut wget_bar);
    fn wget_bar_print(bar_0: *mut wget_bar, slot: libc::c_int, s: *const libc::c_char);
    fn wget_bar_vprintf(
        bar_0: *mut wget_bar,
        slot: libc::c_int,
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    );
    fn wget_bar_slot_begin(
        bar_0: *mut wget_bar,
        slot: libc::c_int,
        filename: *const libc::c_char,
        new_file: libc::c_int,
        filesize: ssize_t,
    );
    fn wget_bar_slot_downloaded(bar_0: *mut wget_bar, slot: libc::c_int, nbytes: size_t);
    fn wget_bar_slot_deregister(bar_0: *mut wget_bar, slot: libc::c_int);
    fn wget_bar_update(bar_0: *mut wget_bar);
    fn wget_bar_set_slots(bar_0: *mut wget_bar, nslots: libc::c_int);
    fn wget_bar_write_line(bar_0: *mut wget_bar, buf: *const libc::c_char, len: size_t);
    fn wget_bar_write_line_ext(
        bar_0: *mut wget_bar,
        buf: *const libc::c_char,
        len: size_t,
        pre: *const libc::c_char,
        post: *const libc::c_char,
    );
    fn wget_bar_set_speed_type(type_0: wget_report_speed);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut config: config;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type va_list = __gnuc_va_list;
pub type ssize_t = __ssize_t;
pub type uint16_t = __uint16_t;
pub type wget_logger = wget_logger_st;
pub type wget_logger_func = unsafe extern "C" fn(*const libc::c_char, size_t) -> ();
pub type wget_vector = wget_vector_st;
pub type wget_thread = *mut wget_thread_st;
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
pub type wget_hpkp_db = wget_hpkp_db_st;
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_netrc_db = wget_netrc_db_st;
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
pub type wget_bar = wget_bar_st;
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
pub const BAR_THREAD_SLEEP_DURATION: C2RustUnnamed = 1000;
pub type C2RustUnnamed = libc::c_uint;
pub const BAR_THREAD_WINDOWS_CONSOLE_SIZE_CHECK_INTERVAL: C2RustUnnamed = 1000;
static mut bar: *mut wget_bar = 0 as *const wget_bar as *mut wget_bar;
static mut progress_thread: wget_thread = 0 as *const wget_thread_st
    as *mut wget_thread_st;
static mut terminate_thread: bool = false;
unsafe extern "C" fn bar_update_thread(mut p: *mut libc::c_void) -> *mut libc::c_void {
    while !terminate_thread {
        wget_bar_update(bar);
        wget_millisleep(BAR_THREAD_SLEEP_DURATION as libc::c_int);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn error_write(mut buf: *const libc::c_char, mut len: size_t) {
    wget_bar_write_line_ext(
        bar,
        buf,
        len,
        b"\x1B[31m\0" as *const u8 as *const libc::c_char,
        b"\x1B[m\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn info_write(mut buf: *const libc::c_char, mut len: size_t) {
    wget_bar_write_line(bar, buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn bar_init() -> bool {
    if wget_thread_support() {
        bar = wget_bar_init(0 as *mut wget_bar, 1 as libc::c_int);
        if !bar.is_null() {
            wget_bar_set_speed_type(config.report_speed);
            wget_logger_set_func(
                wget_get_logger(2 as libc::c_int),
                Some(
                    error_write
                        as unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                ),
            );
            wget_logger_set_func(
                wget_get_logger(1 as libc::c_int),
                Some(
                    info_write as unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                ),
            );
            ::core::ptr::write_volatile(
                &mut terminate_thread as *mut bool,
                0 as libc::c_int != 0,
            );
            if wget_thread_start(
                &mut progress_thread,
                Some(
                    bar_update_thread
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                0 as *mut libc::c_void,
                0 as libc::c_int,
            ) != 0
            {
                wget_bar_free(&mut bar);
            } else {
                return 1 as libc::c_int != 0
            }
        }
    }
    wget_error_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Cannot create progress bar thread. Disabling progress bar.\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    config.progress = 0 as libc::c_int as libc::c_char;
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn bar_deinit() {
    if !bar.is_null() {
        ::core::ptr::write_volatile(
            &mut terminate_thread as *mut bool,
            1 as libc::c_int != 0,
        );
        wget_thread_join(&mut progress_thread);
        wget_bar_free(&mut bar);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bar_print(mut slot: libc::c_int, mut s: *const libc::c_char) {
    wget_bar_print(bar, slot, s);
}
#[no_mangle]
pub unsafe extern "C" fn bar_vprintf(
    mut slot: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    wget_bar_vprintf(bar, slot, fmt, args.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn bar_printf(
    mut slot: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    bar_vprintf(slot, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn bar_slot_begin(
    mut slot: libc::c_int,
    mut filename: *const libc::c_char,
    mut new_file: libc::c_int,
    mut filesize: ssize_t,
) {
    wget_bar_slot_begin(bar, slot, filename, new_file, filesize);
}
#[no_mangle]
pub unsafe extern "C" fn bar_set_downloaded(mut slot: libc::c_int, mut nbytes: size_t) {
    wget_bar_slot_downloaded(bar, slot, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn bar_slot_deregister(mut slot: libc::c_int) {
    wget_bar_slot_deregister(bar, slot);
}
#[no_mangle]
pub unsafe extern "C" fn bar_update_slots(mut nslots: libc::c_int) {
    wget_bar_set_slots(bar, nslots);
}
