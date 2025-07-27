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
    pub type wget_logger_st;
    pub type wget_vector_st;
    pub type wget_cookie_db_st;
    pub type wget_hsts_db_st;
    pub type wget_hpkp_db_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gettime(_: *mut timespec);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
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
    fn wget_buffer_strcpy(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_strcat(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_printf_append(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_logger_set_func(logger: *mut wget_logger, func: Option::<wget_logger_func>);
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_console_init() -> libc::c_int;
    static mut config: config;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub type wget_logger = wget_logger_st;
pub type wget_logger_func = unsafe extern "C" fn(*const libc::c_char, size_t) -> ();
pub type wget_vector = wget_vector_st;
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
pub type wget_console_color = libc::c_uint;
pub const WGET_CONSOLE_COLOR_MAGENTA: wget_console_color = 5;
pub const WGET_CONSOLE_COLOR_RED: wget_console_color = 4;
pub const WGET_CONSOLE_COLOR_GREEN: wget_console_color = 3;
pub const WGET_CONSOLE_COLOR_BLUE: wget_console_color = 2;
pub const WGET_CONSOLE_COLOR_WHITE: wget_console_color = 1;
pub const WGET_CONSOLE_COLOR_RESET: wget_console_color = 0;
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
unsafe extern "C" fn write_out(
    mut default_fp: *mut FILE,
    mut data: *const libc::c_char,
    mut len: size_t,
    mut with_timestamp: libc::c_int,
    mut colorstring: *const libc::c_char,
    mut color_id: wget_console_color,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    if data.is_null() || len as ssize_t <= 0 as libc::c_int as ssize_t {
        return;
    }
    if (config.logfile).is_null() {
        fp = default_fp;
    } else if strcmp(config.logfile, b"-\0" as *const u8 as *const libc::c_char) == 0 {
        fp = stdout;
    } else {
        fp = 0 as *mut FILE;
        if !config.dont_write {
            fd = open(
                config.logfile,
                0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int,
                0o400 as libc::c_int | 0o200 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
            );
        }
        if fd == -(1 as libc::c_int) {
            fp = default_fp;
        }
    }
    let mut sbuf: [libc::c_char; 4096] = [0; 4096];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    let mut use_color: libc::c_int = 0 as libc::c_int;
    if !fp.is_null() && !colorstring.is_null() && isatty(fileno(fp)) != 0 {
        use_color = 1 as libc::c_int;
    }
    if use_color != 0 {
        wget_buffer_strcpy(&mut buf, colorstring);
    }
    if with_timestamp != 0 {
        let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        let mut tp: *mut tm = 0 as *mut tm;
        let mut tbuf: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        gettime(&mut ts);
        tp = localtime_r(&mut ts.tv_sec as *mut __time_t as *const time_t, &mut tbuf);
        wget_buffer_printf_append(
            &mut buf as *mut wget_buffer,
            b"%02d.%02d%02d%02d.%03d \0" as *const u8 as *const libc::c_char,
            (*tp).tm_mday,
            (*tp).tm_hour,
            (*tp).tm_min,
            (*tp).tm_sec,
            (ts.tv_nsec / 1000000 as libc::c_int as __syscall_slong_t) as libc::c_int,
        );
    }
    wget_buffer_memcat(&mut buf, data as *const libc::c_void, len);
    if *data.offset(len.wrapping_sub(1 as libc::c_int as size_t) as isize) as libc::c_int
        != '\n' as i32
    {
        wget_buffer_memcat(
            &mut buf,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    if use_color != 0 {
        wget_buffer_strcat(&mut buf, b"\x1B[m\0" as *const u8 as *const libc::c_char);
    }
    if !fp.is_null() {
        fwrite(
            buf.data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            buf.length,
            fp,
        );
    } else if fd != -(1 as libc::c_int) {
        if write(fd, buf.data as *const libc::c_void, buf.length)
            == -(1 as libc::c_int) as ssize_t
        {
            fwrite(
                buf.data as *const libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                buf.length,
                stderr,
            );
        }
        close(fd);
    }
    wget_buffer_deinit(&mut buf);
}
unsafe extern "C" fn write_debug(
    mut fp: *mut FILE,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    write_out(
        fp,
        data,
        len,
        1 as libc::c_int,
        b"\x1B[35m\0" as *const u8 as *const libc::c_char,
        WGET_CONSOLE_COLOR_MAGENTA,
    );
}
unsafe extern "C" fn write_error(
    mut fp: *mut FILE,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    write_out(
        fp,
        data,
        len,
        0 as libc::c_int,
        b"\x1B[31m\0" as *const u8 as *const libc::c_char,
        WGET_CONSOLE_COLOR_RED,
    );
}
unsafe extern "C" fn write_info(
    mut fp: *mut FILE,
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    if data.is_null() || len as ssize_t <= 0 as libc::c_int as ssize_t {
        return;
    }
    write_out(
        fp,
        data,
        len,
        0 as libc::c_int,
        0 as *const libc::c_char,
        WGET_CONSOLE_COLOR_WHITE,
    );
}
unsafe extern "C" fn write_debug_stderr(mut data: *const libc::c_char, mut len: size_t) {
    write_debug(stderr, data, len);
}
unsafe extern "C" fn write_error_stderr(mut data: *const libc::c_char, mut len: size_t) {
    write_error(stderr, data, len);
}
unsafe extern "C" fn write_error_stdout(mut data: *const libc::c_char, mut len: size_t) {
    write_error(stdout, data, len);
}
unsafe extern "C" fn write_info_stderr(mut data: *const libc::c_char, mut len: size_t) {
    write_info(stderr, data, len);
}
unsafe extern "C" fn write_info_stdout(mut data: *const libc::c_char, mut len: size_t) {
    write_info(stdout, data, len);
}
#[no_mangle]
pub unsafe extern "C" fn log_write_error_stdout(
    mut data: *const libc::c_char,
    mut len: size_t,
) {
    write_error_stdout(data, len);
}
#[no_mangle]
pub unsafe extern "C" fn log_init() {
    wget_console_init();
    wget_logger_set_func(
        wget_get_logger(3 as libc::c_int),
        if config.debug as libc::c_int != 0 {
            Some(
                write_debug_stderr
                    as unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
            )
        } else {
            None
        },
    );
    wget_logger_set_func(
        wget_get_logger(2 as libc::c_int),
        if config.quiet as libc::c_int != 0 {
            None
        } else {
            Some(
                write_error_stderr
                    as unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
            )
        },
    );
    wget_logger_set_func(
        wget_get_logger(1 as libc::c_int),
        if config.verbose as libc::c_int != 0 && !config.quiet {
            if fileno(stdout) == fileno(stderr)
                || wget_strcmp(
                    config.output_document,
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                Some(
                    write_info_stderr
                        as unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                )
            } else {
                Some(
                    write_info_stdout
                        as unsafe extern "C" fn(*const libc::c_char, size_t) -> (),
                )
            }
        } else {
            None
        },
    );
}
