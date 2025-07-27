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
    pub type wget_cookie_db_st;
    pub type wget_hsts_db_st;
    pub type wget_hpkp_db_st;
    pub type wget_hpkp_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    pub type wget_dns_st;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_fprintf(fp_0: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
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
    fn wget_hashmap_contains(
        h: *const wget_hashmap,
        key: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_set_key_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_key_destructor>,
    );
    fn wget_thread_mutex_init(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex_0: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex_0: wget_thread_mutex);
    fn wget_iri_scheme_get_name(scheme: wget_iri_scheme) -> *const libc::c_char;
    fn wget_tcp_get_ip(tcp: *mut wget_tcp) -> *const libc::c_char;
    fn wget_http_get_host(conn: *const wget_http_connection) -> *const libc::c_char;
    fn wget_server_set_stats_callback(fn_0: Option::<wget_server_stats_callback>);
    static mut config: config;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type socklen_t = __socklen_t;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
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
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_hash_fn = unsafe extern "C" fn(
    *const libc::c_void,
) -> libc::c_uint;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
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
pub type wget_hpkp = wget_hpkp_st;
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_netrc_db = wget_netrc_db_st;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub type wget_dns = wget_dns_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_tcp_st {
    pub ssl_session: *mut libc::c_void,
    pub addrinfo: *mut addrinfo,
    pub bind_addrinfo: *mut addrinfo,
    pub connect_addrinfo: *mut addrinfo,
    pub host: *const libc::c_char,
    pub ssl_hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub bind_interface: *const libc::c_char,
    pub dns: *mut wget_dns,
    pub sockfd: libc::c_int,
    pub dns_timeout: libc::c_int,
    pub connect_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub family: libc::c_int,
    pub preferred_family: libc::c_int,
    pub protocol: libc::c_int,
    pub hpkp: wget_hpkp_stats_result,
    pub remote_port: uint16_t,
    #[bitfield(name = "ssl", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "tls_false_start", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "tcp_fastopen", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "first_send", ty = "bool", bits = "3..=3")]
    pub ssl_tls_false_start_tcp_fastopen_first_send: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type wget_hpkp_stats_result = libc::c_uint;
pub const WGET_STATS_HPKP_ERROR: wget_hpkp_stats_result = 3;
pub const WGET_STATS_HPKP_NOMATCH: wget_hpkp_stats_result = 2;
pub const WGET_STATS_HPKP_MATCH: wget_hpkp_stats_result = 1;
pub const WGET_STATS_HPKP_NO: wget_hpkp_stats_result = 0;
pub type wget_tcp = wget_tcp_st;
pub type wget_transfer_encoding = libc::c_uint;
pub const wget_transfer_encoding_chunked: wget_transfer_encoding = 1;
pub const wget_transfer_encoding_identity: wget_transfer_encoding = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_response_st {
    pub req: *mut wget_http_request,
    pub links: *mut wget_vector,
    pub digests: *mut wget_vector,
    pub cookies: *mut wget_vector,
    pub challenges: *mut wget_vector,
    pub hpkp: *mut wget_hpkp,
    pub content_type: *const libc::c_char,
    pub content_type_encoding: *const libc::c_char,
    pub content_filename: *const libc::c_char,
    pub location: *const libc::c_char,
    pub etag: *const libc::c_char,
    pub header: *mut wget_buffer,
    pub body: *mut wget_buffer,
    pub response_end: libc::c_longlong,
    pub content_length: size_t,
    pub cur_downloaded: size_t,
    pub accounted_for: size_t,
    pub last_modified: int64_t,
    pub hsts_maxage: int64_t,
    pub reason: [libc::c_char; 32],
    pub icy_metaint: libc::c_int,
    pub major: libc::c_short,
    pub minor: libc::c_short,
    pub code: libc::c_short,
    pub transfer_encoding: wget_transfer_encoding,
    pub content_encoding: libc::c_char,
    pub hsts_include_subdomains: bool,
    pub keep_alive: bool,
    #[bitfield(name = "content_length_valid", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "length_inconsistent", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "hsts", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "csp", ty = "bool", bits = "3..=3")]
    pub content_length_valid_length_inconsistent_hsts_csp: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_request {
    pub headers: *mut wget_vector,
    pub body: *const libc::c_char,
    pub header_callback: Option::<wget_http_header_callback>,
    pub body_callback: Option::<wget_http_body_callback>,
    pub user_data: *mut libc::c_void,
    pub header_user_data: *mut libc::c_void,
    pub body_user_data: *mut libc::c_void,
    pub esc_resource: wget_buffer,
    pub esc_host: wget_buffer,
    pub body_length: size_t,
    pub stream_id: int32_t,
    pub scheme: wget_iri_scheme,
    pub esc_resource_buf: [libc::c_char; 256],
    pub esc_host_buf: [libc::c_char; 64],
    pub method: [libc::c_char; 8],
    #[bitfield(name = "response_keepheader", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "response_ignorelength", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "debug_skip_body", ty = "bool", bits = "2..=2")]
    pub response_keepheader_response_ignorelength_debug_skip_body: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub request_start: libc::c_longlong,
    pub first_response_start: libc::c_longlong,
}
pub type wget_http_body_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
pub type wget_http_response = wget_http_response_st;
pub type wget_http_header_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_connection_st {
    pub tcp: *mut wget_tcp,
    pub esc_host: *const libc::c_char,
    pub buf: *mut wget_buffer,
    pub pending_requests: *mut wget_vector,
    pub received_http2_responses: *mut wget_vector,
    pub pending_http2_requests: libc::c_int,
    pub scheme: wget_iri_scheme,
    pub port: uint16_t,
    pub protocol: libc::c_char,
    #[bitfield(name = "print_response_headers", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "abort_indicator", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "proxied", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "goaway", ty = "bool", bits = "3..=3")]
    pub print_response_headers_abort_indicator_proxied_goaway: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type wget_http_connection = wget_http_connection_st;
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
pub type wget_server_stats_callback = unsafe extern "C" fn(
    *mut wget_http_connection,
    *mut wget_http_response,
) -> ();
pub type wget_stats_format = libc::c_uint;
pub const WGET_STATS_FORMAT_CSV: wget_stats_format = 1;
pub const WGET_STATS_FORMAT_HUMAN: wget_stats_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_stats_host {
    pub hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub scheme: wget_iri_scheme,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_stats_data {
    pub hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub hpkp: wget_hpkp_stats_result,
    pub scheme: wget_iri_scheme,
    pub hsts: libc::c_char,
    pub csp: libc::c_char,
    pub hpkp_new: libc::c_char,
}
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
pub type gpg_verify_mode = libc::c_uint;
pub const WGET_GPG_VERIFY_SIG_NO_FAIL: gpg_verify_mode = 2;
pub const GPG_VERIFY_SIG_FAIL: gpg_verify_mode = 1;
pub const GPG_VERIFY_DISABLED: gpg_verify_mode = 0;
pub type https_enforce_mode = libc::c_uint;
pub const HTTPS_ENFORCE_HARD: https_enforce_mode = 2;
pub const HTTPS_ENFORCE_SOFT: https_enforce_mode = 1;
pub const HTTPS_ENFORCE_NONE: https_enforce_mode = 0;
pub type check_certificate_mode = libc::c_uint;
pub const CHECK_CERTIFICATE_LOG_DISABLED: check_certificate_mode = 2;
pub const CHECK_CERTIFICATE_DISABLED: check_certificate_mode = 1;
pub const CHECK_CERTIFICATE_ENABLED: check_certificate_mode = 0;
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
static mut hosts: *mut wget_hashmap = 0 as *const wget_hashmap as *mut wget_hashmap;
static mut mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn host_compare(
    mut host1: *const server_stats_host,
    mut host2: *const server_stats_host,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = wget_strcmp((*host1).hostname, (*host2).hostname);
    if n != 0 {
        return n;
    }
    n = wget_strcmp((*host1).ip, (*host2).ip);
    if n != 0 {
        return n;
    }
    return ((*host1).scheme as libc::c_uint)
        .wrapping_sub((*host2).scheme as libc::c_uint) as libc::c_int;
}
unsafe extern "C" fn host_hash(mut host: *const server_stats_host) -> libc::c_uint {
    let mut hash: libc::c_uint = (*host).scheme as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    p = (*host).hostname as *mut libc::c_uchar;
    while !p.is_null() && *p as libc::c_int != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    p = (*host).ip as *mut libc::c_uchar;
    while !p.is_null() && *p as libc::c_int != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn free_host_entry(mut host: *mut server_stats_host) {
    if !host.is_null() {
        if !((*host).hostname).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*host).hostname as *mut libc::c_void);
            (*host).hostname = 0 as *const libc::c_char;
        }
        if !((*host).ip).is_null() {
            wget_free
                .expect("non-null function pointer")((*host).ip as *mut libc::c_void);
            (*host).ip = 0 as *const libc::c_char;
        }
        if !host.is_null() {
            wget_free.expect("non-null function pointer")(host as *mut libc::c_void);
            host = 0 as *mut server_stats_host;
        }
    }
}
unsafe extern "C" fn hpkp_string(
    mut hpkp: wget_hpkp_stats_result,
) -> *const libc::c_char {
    match hpkp as libc::c_uint {
        0 => return b"HPKP_NO\0" as *const u8 as *const libc::c_char,
        1 => return b"HPKP_MATCH\0" as *const u8 as *const libc::c_char,
        2 => return b"HPKP_NOMATCH\0" as *const u8 as *const libc::c_char,
        3 => return b"HPKP_ERROR\0" as *const u8 as *const libc::c_char,
        _ => return b"?\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn server_stats_print(mut stats: *mut server_stats_data) {
    if (*config.stats_server_args).format as libc::c_uint
        == WGET_STATS_FORMAT_HUMAN as libc::c_int as libc::c_uint
    {
        wget_fprintf(
            fp,
            b"  %s:\n\0" as *const u8 as *const libc::c_char,
            if !((*stats).hostname).is_null() {
                (*stats).hostname
            } else {
                b"-\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    IP             : %s\n\0" as *const u8 as *const libc::c_char,
            if !((*stats).ip).is_null() {
                (*stats).ip
            } else {
                b"-\0" as *const u8 as *const libc::c_char
            },
        );
        wget_fprintf(
            fp,
            b"    Scheme         : %s\n\0" as *const u8 as *const libc::c_char,
            wget_iri_scheme_get_name((*stats).scheme),
        );
        wget_fprintf(
            fp,
            b"    HPKP           : %s\n\0" as *const u8 as *const libc::c_char,
            hpkp_string((*stats).hpkp),
        );
        wget_fprintf(
            fp,
            b"    HPKP New Entry : %s\n\0" as *const u8 as *const libc::c_char,
            if (*stats).hpkp_new as libc::c_int != 0 {
                if (*stats).hpkp_new as libc::c_int == 1 as libc::c_int {
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
            b"    HSTS           : %s\n\0" as *const u8 as *const libc::c_char,
            if (*stats).hsts as libc::c_int != 0 {
                if (*stats).hsts as libc::c_int == 1 as libc::c_int {
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
            b"    CSP            : %s\n\n\0" as *const u8 as *const libc::c_char,
            if (*stats).csp as libc::c_int != 0 {
                if (*stats).csp as libc::c_int == 1 as libc::c_int {
                    b"On\0" as *const u8 as *const libc::c_char
                } else {
                    b"-\0" as *const u8 as *const libc::c_char
                }
            } else {
                b"Off\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        wget_fprintf(
            fp,
            b"%s,%s,%s,%d,%d,%d,%d\n\0" as *const u8 as *const libc::c_char,
            if !((*stats).hostname).is_null() {
                (*stats).hostname
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !((*stats).ip).is_null() {
                (*stats).ip
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            wget_iri_scheme_get_name((*stats).scheme),
            (*stats).hpkp as libc::c_int,
            (*stats).hpkp_new as libc::c_int,
            (*stats).hsts as libc::c_int,
            (*stats).csp as libc::c_int,
        );
    };
}
unsafe extern "C" fn server_stats_add(
    mut conn: *mut wget_http_connection,
    mut resp: *mut wget_http_response,
) {
    let mut hostp: *mut server_stats_host = wget_malloc(
        ::core::mem::size_of::<server_stats_host>() as libc::c_ulong,
    ) as *mut server_stats_host;
    (*hostp).hostname = wget_strdup(wget_http_get_host(conn));
    (*hostp).ip = wget_strdup(wget_tcp_get_ip((*conn).tcp));
    (*hostp).scheme = (*conn).scheme;
    wget_thread_mutex_lock(mutex);
    if wget_hashmap_contains(hosts, hostp as *const libc::c_void) == 0 {
        let mut stats: server_stats_data = server_stats_data {
            hostname: 0 as *const libc::c_char,
            ip: 0 as *const libc::c_char,
            hpkp: WGET_STATS_HPKP_NO,
            scheme: WGET_IRI_SCHEME_HTTP,
            hsts: 0,
            csp: 0,
            hpkp_new: 0,
        };
        stats.hostname = (*hostp).hostname;
        stats.ip = (*hostp).ip;
        stats.scheme = (*hostp).scheme;
        stats.hpkp = (*(*conn).tcp).hpkp;
        stats
            .hpkp_new = (if !resp.is_null() {
            if !((*resp).hpkp).is_null() { 1 as libc::c_int } else { 0 as libc::c_int }
        } else {
            -(1 as libc::c_int)
        }) as libc::c_char;
        stats
            .hsts = (if !resp.is_null() {
            if (*resp).hsts() as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }
        } else {
            -(1 as libc::c_int)
        }) as libc::c_char;
        stats
            .csp = (if !resp.is_null() {
            if (*resp).csp() as libc::c_int != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }
        } else {
            -(1 as libc::c_int)
        }) as libc::c_char;
        server_stats_print(&mut stats);
        wget_hashmap_put(
            hosts,
            hostp as *const libc::c_void,
            hostp as *const libc::c_void,
        );
    } else {
        free_host_entry(hostp);
    }
    wget_thread_mutex_unlock(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn server_stats_init(mut fpout: *mut FILE) {
    wget_thread_mutex_init(&mut mutex);
    hosts = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const server_stats_host) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(
            Some(
                host_hash
                    as unsafe extern "C" fn(*const server_stats_host) -> libc::c_uint,
            ),
        ),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const server_stats_host,
                    *const server_stats_host,
                ) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                host_compare
                    as unsafe extern "C" fn(
                        *const server_stats_host,
                        *const server_stats_host,
                    ) -> libc::c_int,
            ),
        ),
    );
    wget_hashmap_set_key_destructor(
        hosts,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut server_stats_host) -> ()>,
            Option::<wget_hashmap_key_destructor>,
        >(Some(free_host_entry as unsafe extern "C" fn(*mut server_stats_host) -> ())),
    );
    fp = fpout;
    wget_server_set_stats_callback(
        Some(
            server_stats_add
                as unsafe extern "C" fn(
                    *mut wget_http_connection,
                    *mut wget_http_response,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn server_stats_exit() {
    wget_hashmap_free(&mut hosts);
    wget_thread_mutex_destroy(&mut mutex);
}
