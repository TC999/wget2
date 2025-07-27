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
    pub type wget_list_st;
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_thread_st;
    pub type wget_thread_mutex_st;
    pub type wget_thread_cond_st;
    pub type wget_cookie_db_st;
    pub type wget_hsts_db_st;
    pub type wget_hpkp_db_st;
    pub type wget_hpkp_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    pub type wget_http_connection_st;
    pub type wget_robots_st;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_fprintf(fp_0: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_browse(
        v: *const wget_vector,
        browse: Option::<wget_vector_browse_fn>,
        ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
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
    fn wget_stringmap_create(max: libc::c_int) -> *mut wget_stringmap;
    fn wget_thread_mutex_init(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex_0: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex_0: wget_thread_mutex);
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    static mut config: config;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
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
pub type off_t = __off_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type wget_list = wget_list_st;
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
pub type wget_vector_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_stringmap = wget_hashmap;
pub type wget_stringmap_key_destructor = unsafe extern "C" fn(*mut libc::c_char) -> ();
pub type wget_stringmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_id = libc::c_ulong;
pub type wget_thread = *mut wget_thread_st;
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_thread_cond = *mut wget_thread_cond_st;
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
pub type wget_http_connection = wget_http_connection_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink {
    pub name: *const libc::c_char,
    pub mirrors: *mut wget_vector,
    pub hashes: *mut wget_vector,
    pub pieces: *mut wget_vector,
    pub size: off_t,
}
pub type wget_robots = wget_robots_st;
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
pub type wget_stats_format = libc::c_uint;
pub const WGET_STATS_FORMAT_CSV: wget_stats_format = 1;
pub const WGET_STATS_FORMAT_HUMAN: wget_stats_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blacklist_entry {
    pub iri: *const wget_iri,
    pub local_filename: *mut libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct JOB {
    pub iri: *const wget_iri,
    pub original_url: *const wget_iri,
    pub referer: *const wget_iri,
    pub metalink: *mut wget_metalink,
    pub challenges: *mut wget_vector,
    pub proxy_challenges: *mut wget_vector,
    pub parts: *mut wget_vector,
    pub remaining_sig_ext: *mut wget_list,
    pub host: *mut HOST,
    pub blacklist_entry: *const blacklist_entry,
    pub sig_filename: *mut libc::c_char,
    pub sig_req: *mut libc::c_char,
    pub part: *mut PART,
    pub downloader: *mut DOWNLOADER,
    pub used_by: wget_thread_id,
    pub id: libc::c_ulonglong,
    pub parent_id: libc::c_ulonglong,
    pub retry_ts: libc::c_longlong,
    pub level: libc::c_int,
    pub redirection_level: libc::c_int,
    pub auth_failure_count: libc::c_int,
    pub mirror_pos: libc::c_int,
    pub piece_pos: libc::c_int,
    pub failures: libc::c_int,
    #[bitfield(name = "challenges_alloc", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "inuse", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "done", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "sitemap", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "robotstxt", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "head_first", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "requested_by_user", ty = "bool", bits = "6..=6")]
    #[bitfield(name = "ignore_patterns", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "http_fallback", ty = "bool", bits = "8..=8")]
    #[bitfield(name = "recursive_send_head", ty = "bool", bits = "9..=9")]
    #[bitfield(name = "redirect_get", ty = "bool", bits = "10..=10")]
    pub challenges_alloc_inuse_done_sitemap_robotstxt_head_first_requested_by_user_ignore_patterns_http_fallback_recursive_send_head_redirect_get: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct DOWNLOADER {
    pub thread: wget_thread,
    pub job: *mut JOB,
    pub conn: *mut wget_http_connection,
    pub buf: *mut libc::c_char,
    pub bufsize: size_t,
    pub id: libc::c_int,
    pub cond: wget_thread_cond,
    #[bitfield(name = "final_error", ty = "bool", bits = "0..=0")]
    pub final_error: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct PART {
    pub position: off_t,
    pub length: off_t,
    pub id: libc::c_int,
    pub used_by: wget_thread_id,
    #[bitfield(name = "inuse", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "done", ty = "bool", bits = "1..=1")]
    pub inuse_done: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct HOST {
    pub host: *const libc::c_char,
    pub robot_job: *mut JOB,
    pub robots: *mut wget_robots,
    pub queue: *mut wget_list,
    pub retry_ts: libc::c_longlong,
    pub qsize: libc::c_int,
    pub failures: libc::c_int,
    pub scheme: wget_iri_scheme,
    pub port: uint16_t,
    #[bitfield(name = "blocked", ty = "bool", bits = "0..=0")]
    pub blocked: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_gpg_info_t {
    pub bad_sigs: libc::c_int,
    pub missing_sigs: libc::c_int,
    pub invalid_sigs: libc::c_int,
    pub valid_sigs: libc::c_int,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct site_stats_data {
    pub iri: *const wget_iri,
    pub size_downloaded: libc::c_longlong,
    pub size_decompressed: libc::c_longlong,
    pub request_start: libc::c_longlong,
    pub response_end: libc::c_longlong,
    pub initial_response_duration: libc::c_longlong,
    pub id: libc::c_ulonglong,
    pub parent_id: libc::c_ulonglong,
    pub status: libc::c_int,
    pub signature_status: libc::c_int,
    pub encoding: libc::c_char,
    pub method: libc::c_char,
    pub mime_type: *const libc::c_char,
    #[bitfield(name = "redirect", ty = "bool", bits = "0..=0")]
    pub redirect: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub last_modified: int64_t,
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
pub const STATS_METHOD_POST: C2RustUnnamed = 3;
pub const STATS_METHOD_HEAD: C2RustUnnamed = 2;
pub const STATS_METHOD_GET: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
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
unsafe extern "C" fn wget_stringmap_get(
    mut h: *const wget_stringmap,
    mut key: *const libc::c_char,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    return wget_hashmap_get(h, key as *const libc::c_void, value);
}
#[inline]
unsafe extern "C" fn wget_stringmap_free(mut h: *mut *mut wget_stringmap) {
    wget_hashmap_free(h);
}
#[inline]
unsafe extern "C" fn wget_stringmap_set_key_destructor(
    mut h: *mut wget_hashmap,
    mut destructor: Option::<wget_stringmap_key_destructor>,
) {
    wget_hashmap_set_key_destructor(
        h,
        ::core::mem::transmute::<
            Option::<wget_stringmap_key_destructor>,
            Option::<wget_hashmap_key_destructor>,
        >(destructor),
    );
}
#[inline]
unsafe extern "C" fn wget_stringmap_set_value_destructor(
    mut h: *mut wget_hashmap,
    mut destructor: Option::<wget_stringmap_value_destructor>,
) {
    wget_hashmap_set_value_destructor(
        h,
        ::core::mem::transmute::<
            Option::<wget_stringmap_value_destructor>,
            Option::<wget_hashmap_value_destructor>,
        >(destructor),
    );
}
static mut data: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut docs: *mut wget_hashmap = 0 as *const wget_hashmap as *mut wget_hashmap;
static mut fp: *mut FILE = 0 as *const FILE as *mut FILE;
unsafe extern "C" fn free_stats(mut stats: *mut libc::c_void) {
    let mut s: *mut site_stats_data = stats as *mut site_stats_data;
    if !s.is_null() {
        if !((*s).mime_type).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*s).mime_type as *mut libc::c_void);
            (*s).mime_type = 0 as *const libc::c_char;
        }
        if !s.is_null() {
            wget_free.expect("non-null function pointer")(s as *mut libc::c_void);
            s = 0 as *mut site_stats_data;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn site_stats_init(mut fpout: *mut FILE) {
    wget_thread_mutex_init(&mut mutex);
    data = wget_vector_create(8 as libc::c_int, None);
    wget_vector_set_destructor(
        data,
        Some(free_stats as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    fp = fpout;
}
#[no_mangle]
pub unsafe extern "C" fn site_stats_exit() {
    wget_stringmap_free(&mut docs);
    wget_vector_free(&mut data);
    wget_thread_mutex_destroy(&mut mutex);
}
#[no_mangle]
pub unsafe extern "C" fn stats_site_add(
    mut resp: *mut wget_http_response,
    mut gpg_info: *mut wget_gpg_info_t,
) {
    let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
    let mut iri: *const wget_iri = (*job).iri;
    if !gpg_info.is_null() {
        wget_thread_mutex_lock(mutex);
        if docs.is_null() {
            docs = wget_stringmap_create(128 as libc::c_int);
            wget_stringmap_set_key_destructor(docs, None);
            wget_stringmap_set_value_destructor(docs, None);
            let mut it: libc::c_int = 0 as libc::c_int;
            while it < wget_vector_size(data) {
                let mut e: *mut site_stats_data = wget_vector_get(data, it)
                    as *mut site_stats_data;
                wget_stringmap_put(docs, (*(*e).iri).uri, e as *const libc::c_void);
                it += 1;
                it;
            }
        }
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut uri: *mut libc::c_char = wget_strdup((*iri).uri);
        p = strrchr(uri, '.' as i32);
        if !p.is_null() {
            *p = 0 as libc::c_int as libc::c_char;
        }
        let mut doc: *mut site_stats_data = 0 as *mut site_stats_data;
        let mut rc: libc::c_int = wget_stringmap_get(
            docs,
            uri,
            &mut doc as *mut *mut site_stats_data as *mut *mut libc::c_void,
        );
        if !uri.is_null() {
            wget_free.expect("non-null function pointer")(uri as *mut libc::c_void);
            uri = 0 as *mut libc::c_char;
        }
        if rc != 0 && !doc.is_null() {
            if (*gpg_info).valid_sigs != 0 {
                (*doc).signature_status = 1 as libc::c_int;
            } else if (*gpg_info).invalid_sigs != 0 {
                (*doc).signature_status = 2 as libc::c_int;
            } else if (*gpg_info).bad_sigs != 0 {
                (*doc).signature_status = 3 as libc::c_int;
            } else if (*gpg_info).missing_sigs != 0 {
                (*doc).signature_status = 4 as libc::c_int;
            }
            wget_thread_mutex_unlock(mutex);
            return;
        }
        wget_thread_mutex_unlock(mutex);
    }
    let mut doc_0: *mut site_stats_data = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<site_stats_data>() as libc::c_ulong,
    ) as *mut site_stats_data;
    (*doc_0).id = (*job).id;
    (*doc_0).parent_id = (*job).parent_id;
    (*doc_0).iri = iri;
    (*doc_0).status = (*resp).code as libc::c_int;
    (*doc_0).encoding = (*resp).content_encoding;
    (*doc_0).set_redirect((*job).redirection_level != 0 as libc::c_int);
    (*doc_0).mime_type = wget_strdup((*resp).content_type);
    (*doc_0).last_modified = (*resp).last_modified;
    (*doc_0).request_start = (*(*resp).req).request_start;
    (*doc_0).response_end = (*resp).response_end;
    (*doc_0)
        .initial_response_duration = (*(*resp).req).first_response_start
        - (*(*resp).req).request_start;
    (*doc_0).size_downloaded = (*resp).cur_downloaded as libc::c_longlong;
    (*doc_0).size_decompressed = (*(*resp).body).length as libc::c_longlong;
    if wget_strcasecmp_ascii(
        ((*(*resp).req).method).as_mut_ptr(),
        b"GET\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*doc_0).method = STATS_METHOD_GET as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(
        ((*(*resp).req).method).as_mut_ptr(),
        b"HEAD\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*doc_0).size_downloaded = (*resp).content_length as libc::c_longlong;
        (*doc_0).method = STATS_METHOD_HEAD as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(
        ((*(*resp).req).method).as_mut_ptr(),
        b"POST\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        (*doc_0).method = STATS_METHOD_POST as libc::c_int as libc::c_char;
    }
    wget_thread_mutex_lock(mutex);
    wget_vector_add(data, doc_0 as *const libc::c_void);
    if !docs.is_null() {
        wget_stringmap_put(docs, (*(*doc_0).iri).uri, doc_0 as *const libc::c_void);
    }
    wget_thread_mutex_unlock(mutex);
}
unsafe extern "C" fn print_human_entry(
    mut _fp: *mut FILE,
    mut doc: *mut site_stats_data,
) -> libc::c_int {
    let mut transfer_time: libc::c_longlong = (*doc).response_end - (*doc).request_start;
    wget_fprintf(
        _fp,
        b"  %6d %5lld %6lld %s\n\0" as *const u8 as *const libc::c_char,
        (*doc).status,
        transfer_time,
        (*doc).size_downloaded,
        (*(*doc).iri).safe_uri,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_csv_entry(
    mut _fp: *mut FILE,
    mut doc: *mut site_stats_data,
) -> libc::c_int {
    let mut transfer_time: libc::c_longlong = (*doc).response_end - (*doc).request_start;
    wget_fprintf(
        _fp,
        b"%llu,%llu,%s,%d,%d,%d,%lld,%lld,%lld,%lld,%d,%d,%lld,%s\n\0" as *const u8
            as *const libc::c_char,
        (*doc).id,
        (*doc).parent_id,
        (*(*doc).iri).uri,
        (*doc).status,
        !(*doc).redirect() as libc::c_int,
        (*doc).method as libc::c_int,
        (*doc).size_downloaded,
        (*doc).size_decompressed,
        transfer_time,
        (*doc).initial_response_duration,
        (*doc).encoding as libc::c_int,
        (*doc).signature_status,
        (*doc).last_modified as libc::c_longlong,
        (*doc).mime_type,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn print_human() {
    wget_fprintf(fp, b"\nSite Statistics:\n\0" as *const u8 as *const libc::c_char);
    wget_fprintf(
        fp,
        b"  %6s %5s %6s %s\n\0" as *const u8 as *const libc::c_char,
        b"Status\0" as *const u8 as *const libc::c_char,
        b"ms\0" as *const u8 as *const libc::c_char,
        b"Size\0" as *const u8 as *const libc::c_char,
        b"URL\0" as *const u8 as *const libc::c_char,
    );
    wget_vector_browse(
        data,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, *mut site_stats_data) -> libc::c_int,
            >,
            Option::<wget_vector_browse_fn>,
        >(
            Some(
                print_human_entry
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut site_stats_data,
                    ) -> libc::c_int,
            ),
        ),
        fp as *mut libc::c_void,
    );
}
unsafe extern "C" fn print_csv() {
    wget_fprintf(
        fp,
        b"ID,ParentID,URL,Status,Link,Method,Size,SizeDecompressed,TransferTime,ResponseTime,Encoding,Verification,Last-Modified,Content-Type\n\0"
            as *const u8 as *const libc::c_char,
    );
    wget_vector_browse(
        data,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, *mut site_stats_data) -> libc::c_int,
            >,
            Option::<wget_vector_browse_fn>,
        >(
            Some(
                print_csv_entry
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut site_stats_data,
                    ) -> libc::c_int,
            ),
        ),
        fp as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn site_stats_print() {
    if (*config.stats_site_args).format as libc::c_uint
        == WGET_STATS_FORMAT_HUMAN as libc::c_int as libc::c_uint
    {
        print_human();
    } else {
        print_csv();
    };
}
