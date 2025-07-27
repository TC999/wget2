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
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    pub type wget_http_connection_st;
    pub type wget_robots_st;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_get_timemillis() -> libc::c_longlong;
    fn wget_list_append(
        list: *mut *mut wget_list,
        data: *const libc::c_void,
        size: size_t,
    ) -> *mut libc::c_void;
    fn wget_list_getfirst(list: *const wget_list) -> *mut libc::c_void;
    fn wget_list_getnext(elem: *const libc::c_void) -> *mut libc::c_void;
    fn wget_list_remove(list: *mut *mut wget_list, elem: *mut libc::c_void);
    fn wget_list_free(list: *mut *mut wget_list);
    fn wget_list_browse(
        list: *const wget_list,
        browse: Option::<wget_list_browse_fn>,
        context: *mut libc::c_void,
    ) -> libc::c_int;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
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
    fn wget_hashmap_contains(
        h: *const wget_hashmap,
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
    fn wget_thread_self() -> wget_thread_id;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_parse_base(
        base: *const wget_iri,
        url: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_robots_free(robots: *mut *mut wget_robots);
    fn wget_robots_get_path_count(robots: *mut wget_robots) -> libc::c_int;
    fn wget_robots_get_path(
        robots: *mut wget_robots,
        index: libc::c_int,
    ) -> *mut wget_string;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn job_init(
        job: *mut JOB,
        blacklistp: *mut blacklist_entry,
        http_fallback: bool,
    ) -> *mut JOB;
    fn job_free(job: *mut JOB);
    fn blacklist_add(iri: *const wget_iri) -> *mut blacklist_entry;
    static mut config: config;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
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
pub type uint16_t = __uint16_t;
pub type wget_list = wget_list_st;
pub type wget_list_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
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
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_netrc_db = wget_netrc_db_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_string {
    pub p: *const libc::c_char,
    pub len: size_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blacklist_entry {
    pub iri: *const wget_iri,
    pub local_filename: *mut libc::c_char,
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
pub struct _find_free_job_context {
    pub job: *mut JOB,
    pub now: libc::c_longlong,
    pub pause: libc::c_longlong,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_args {
    pub filename: *const libc::c_char,
    pub fp: *mut FILE,
    pub format: wget_stats_format,
}
static mut hosts: *mut wget_hashmap = 0 as *const wget_hashmap as *mut wget_hashmap;
static mut hosts_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut qsize: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn host_init() {
    wget_thread_mutex_init(&mut hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn host_exit() {
    wget_thread_mutex_destroy(&mut hosts_mutex);
}
unsafe extern "C" fn _host_compare(
    mut host1: *const HOST,
    mut host2: *const HOST,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if (*host1).scheme as libc::c_uint != (*host2).scheme as libc::c_uint {
        return if ((*host1).scheme as libc::c_uint) < (*host2).scheme as libc::c_uint {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    }
    n = wget_strcmp((*host1).host, (*host2).host);
    if n != 0 {
        return n;
    }
    return if ((*host1).port as libc::c_int) < (*host2).port as libc::c_int {
        -(1 as libc::c_int)
    } else if (*host1).port as libc::c_int > (*host2).port as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn _host_hash(mut host: *const HOST) -> libc::c_uint {
    let mut hash: libc::c_uint = (*host).port as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    hash = hash
        .wrapping_mul(101 as libc::c_int as libc::c_uint)
        .wrapping_add((*host).scheme as libc::c_uint);
    p = (*host).host as *mut libc::c_uchar;
    while !p.is_null() && *p as libc::c_int != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn _free_host_entry(mut host: *mut HOST) {
    if !host.is_null() {
        host_queue_free(host);
        wget_robots_free(&mut (*host).robots);
        if !host.is_null() {
            wget_free.expect("non-null function pointer")(host as *mut libc::c_void);
            host = 0 as *mut HOST;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn host_add(mut iri: *const wget_iri) -> *mut HOST {
    wget_thread_mutex_lock(hosts_mutex);
    if hosts.is_null() {
        hosts = wget_hashmap_create(
            16 as libc::c_int,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*const HOST) -> libc::c_uint>,
                Option::<wget_hashmap_hash_fn>,
            >(Some(_host_hash as unsafe extern "C" fn(*const HOST) -> libc::c_uint)),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*const HOST, *const HOST) -> libc::c_int>,
                Option::<wget_hashmap_compare_fn>,
            >(
                Some(
                    _host_compare
                        as unsafe extern "C" fn(*const HOST, *const HOST) -> libc::c_int,
                ),
            ),
        );
        wget_hashmap_set_key_destructor(
            hosts,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut HOST) -> ()>,
                Option::<wget_hashmap_key_destructor>,
            >(Some(_free_host_entry as unsafe extern "C" fn(*mut HOST) -> ())),
        );
    }
    let mut hostp: *mut HOST = 0 as *mut HOST;
    let mut host: HOST = {
        let mut init = HOST {
            blocked: [0; 1],
            c2rust_padding: [0; 1],
            host: (*iri).host,
            robot_job: 0 as *mut JOB,
            robots: 0 as *mut wget_robots,
            queue: 0 as *mut wget_list,
            retry_ts: 0,
            qsize: 0,
            failures: 0,
            scheme: (*iri).scheme,
            port: (*iri).port,
        };
        init.set_blocked(false);
        init
    };
    if wget_hashmap_contains(hosts, &mut host as *mut HOST as *const libc::c_void) == 0 {
        hostp = wget_memdup(
            &mut host as *mut HOST as *const libc::c_void,
            ::core::mem::size_of::<HOST>() as libc::c_ulong,
        ) as *mut HOST;
        wget_hashmap_put(
            hosts,
            hostp as *const libc::c_void,
            hostp as *const libc::c_void,
        );
    }
    wget_thread_mutex_unlock(hosts_mutex);
    return hostp;
}
#[no_mangle]
pub unsafe extern "C" fn host_get(mut iri: *const wget_iri) -> *mut HOST {
    let mut hostp: *mut HOST = 0 as *mut HOST;
    let mut host: HOST = {
        let mut init = HOST {
            blocked: [0; 1],
            c2rust_padding: [0; 1],
            host: (*iri).host,
            robot_job: 0 as *mut JOB,
            robots: 0 as *mut wget_robots,
            queue: 0 as *mut wget_list,
            retry_ts: 0,
            qsize: 0,
            failures: 0,
            scheme: (*iri).scheme,
            port: (*iri).port,
        };
        init.set_blocked(false);
        init
    };
    wget_thread_mutex_lock(hosts_mutex);
    if hosts.is_null()
        || wget_hashmap_get(
            hosts,
            &mut host as *mut HOST as *const libc::c_void,
            &mut hostp as *mut *mut HOST as *mut *mut libc::c_void,
        ) == 0
    {
        hostp = 0 as *mut HOST;
    }
    wget_thread_mutex_unlock(hosts_mutex);
    return hostp;
}
unsafe extern "C" fn _search_queue_for_free_job(
    mut ctx: *mut _find_free_job_context,
    mut job: *mut JOB,
) -> libc::c_int {
    if !((*job).parts).is_null() {
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < wget_vector_size((*job).parts) {
            let mut part: *mut PART = wget_vector_get((*job).parts, it) as *mut PART;
            if !(*part).inuse() {
                (*part).set_inuse(1 as libc::c_int != 0);
                (*part).used_by = wget_thread_self();
                (*job).part = part;
                (*ctx).job = job;
                wget_debug_printf(
                    b"dequeue chunk %d/%d %s\n\0" as *const u8 as *const libc::c_char,
                    it + 1 as libc::c_int,
                    wget_vector_size((*job).parts),
                    (*(*job).metalink).name,
                );
                return 1 as libc::c_int;
            }
            it += 1;
            it;
        }
    } else if !(*job).inuse() {
        let mut pause: libc::c_longlong = (*job).retry_ts - (*ctx).now;
        if pause > 0 as libc::c_int as libc::c_longlong {
            if (*ctx).pause == 0 || (*ctx).pause < pause {
                (*ctx).pause = pause;
            }
            return 0 as libc::c_int;
        }
        (*job).set_done(1 as libc::c_int != 0);
        (*job).set_inuse((*job).done());
        (*job).used_by = wget_thread_self();
        (*job).part = 0 as *mut PART;
        (*ctx).job = job;
        wget_debug_printf(
            b"dequeue job %s\n\0" as *const u8 as *const libc::c_char,
            (*(*job).iri).safe_uri,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _search_host_for_free_job(
    mut _ctx: *mut libc::c_void,
    mut _host: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    let mut ctx: *mut _find_free_job_context = _ctx as *mut _find_free_job_context;
    let mut host: *const HOST = _host as *const HOST;
    if (*host).blocked() {
        wget_debug_printf(
            b"host %s is blocked (qsize=%d)\n\0" as *const u8 as *const libc::c_char,
            (*host).host,
            (*host).qsize,
        );
        return 0 as libc::c_int;
    }
    let mut pause: libc::c_longlong = (*host).retry_ts - (*ctx).now;
    if pause > 0 as libc::c_int as libc::c_longlong {
        wget_debug_printf(
            b"host %s is paused %lldms\n\0" as *const u8 as *const libc::c_char,
            (*host).host,
            pause,
        );
        if (*ctx).pause == 0 || (*ctx).pause < pause {
            (*ctx).pause = pause;
        }
        return 0 as libc::c_int;
    }
    if !((*host).robot_job).is_null() {
        if !(*(*host).robot_job).inuse() {
            (*(*host).robot_job).set_done(1 as libc::c_int != 0);
            (*(*host).robot_job).set_inuse((*(*host).robot_job).done());
            (*(*host).robot_job).used_by = wget_thread_self();
            (*ctx).job = (*host).robot_job;
            wget_debug_printf(
                b"host %s dequeue robot job\n\0" as *const u8 as *const libc::c_char,
                (*host).host,
            );
            return 1 as libc::c_int;
        }
        wget_debug_printf(
            b"robot job still in progress\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    wget_list_browse(
        (*host).queue,
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut _find_free_job_context,
                    *mut JOB,
                ) -> libc::c_int,
            >,
            Option::<wget_list_browse_fn>,
        >(
            Some(
                _search_queue_for_free_job
                    as unsafe extern "C" fn(
                        *mut _find_free_job_context,
                        *mut JOB,
                    ) -> libc::c_int,
            ),
        ),
        ctx as *mut libc::c_void,
    );
    return ((*ctx).job != 0 as *mut JOB) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn host_get_job(
    mut host: *mut HOST,
    mut pause: *mut libc::c_longlong,
) -> *mut JOB {
    let mut ctx: _find_free_job_context = {
        let mut init = _find_free_job_context {
            job: 0 as *mut JOB,
            now: wget_get_timemillis(),
            pause: 0,
        };
        init
    };
    if !host.is_null() {
        _search_host_for_free_job(
            &mut ctx as *mut _find_free_job_context as *mut libc::c_void,
            host as *const libc::c_void,
            0 as *mut libc::c_void,
        );
    } else {
        wget_thread_mutex_lock(hosts_mutex);
        wget_hashmap_browse(
            hosts,
            Some(
                _search_host_for_free_job
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut ctx as *mut _find_free_job_context as *mut libc::c_void,
        );
        wget_thread_mutex_unlock(hosts_mutex);
    }
    if !pause.is_null() {
        *pause = ctx.pause;
    }
    return ctx.job;
}
unsafe extern "C" fn _release_job(
    mut ctx: *mut wget_thread_id,
    mut job: *mut JOB,
) -> libc::c_int {
    let mut self_0: wget_thread_id = *ctx;
    if !((*job).parts).is_null() {
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < wget_vector_size((*job).parts) {
            let mut part: *mut PART = wget_vector_get((*job).parts, it) as *mut PART;
            if !(*part).done() && (*part).inuse() as libc::c_int != 0
                && (*part).used_by == self_0
            {
                (*part).set_inuse(0 as libc::c_int != 0);
                (*part).used_by = 0 as libc::c_int as wget_thread_id;
                wget_debug_printf(
                    b"released chunk %d/%d %s\n\0" as *const u8 as *const libc::c_char,
                    it + 1 as libc::c_int,
                    wget_vector_size((*job).parts),
                    (*(*job).blacklist_entry).local_filename,
                );
            }
            it += 1;
            it;
        }
    } else if (*job).inuse() as libc::c_int != 0 && (*job).used_by == self_0 {
        (*job).set_done(0 as libc::c_int != 0);
        (*job).set_inuse((*job).done());
        (*job).used_by = 0 as libc::c_int as wget_thread_id;
        wget_debug_printf(
            b"released job %s\n\0" as *const u8 as *const libc::c_char,
            (*(*job).iri).safe_uri,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn host_release_jobs(mut host: *mut HOST) {
    if host.is_null() {
        return;
    }
    let mut self_0: wget_thread_id = wget_thread_self();
    wget_thread_mutex_lock(hosts_mutex);
    if !((*host).robot_job).is_null() {
        if (*(*host).robot_job).inuse() as libc::c_int != 0
            && (*(*host).robot_job).used_by == self_0
        {
            (*(*host).robot_job).set_done(0 as libc::c_int != 0);
            (*(*host).robot_job).set_inuse((*(*host).robot_job).done());
            (*(*host).robot_job).used_by = 0 as libc::c_int as wget_thread_id;
            wget_debug_printf(
                b"released robots.txt job\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    wget_list_browse(
        (*host).queue,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_thread_id, *mut JOB) -> libc::c_int>,
            Option::<wget_list_browse_fn>,
        >(
            Some(
                _release_job
                    as unsafe extern "C" fn(*mut wget_thread_id, *mut JOB) -> libc::c_int,
            ),
        ),
        &mut self_0 as *mut wget_thread_id as *mut libc::c_void,
    );
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn host_add_job(mut host: *mut HOST, mut job: *const JOB) {
    let mut jobp: *mut JOB = 0 as *mut JOB;
    if !((*job).blacklist_entry).is_null() {
        wget_debug_printf(
            b"%s: job fname %s\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"host_add_job\0"))
                .as_ptr(),
            (*(*job).blacklist_entry).local_filename,
        );
    }
    wget_thread_mutex_lock(hosts_mutex);
    jobp = wget_list_append(
        &mut (*host).queue,
        job as *const libc::c_void,
        ::core::mem::size_of::<JOB>() as libc::c_ulong,
    ) as *mut JOB;
    (*host).qsize += 1;
    (*host).qsize;
    if !(*host).blocked() {
        qsize += 1;
        qsize;
    }
    (*jobp).host = host;
    if !((*jobp).iri).is_null() {
        wget_debug_printf(
            b"%s: %p %s\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"host_add_job\0"))
                .as_ptr(),
            jobp as *mut libc::c_void,
            (*(*jobp).iri).safe_uri,
        );
    } else if !((*jobp).metalink).is_null() {
        wget_debug_printf(
            b"%s: %p %s\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 13],
                &[libc::c_char; 13],
            >(b"host_add_job\0"))
                .as_ptr(),
            jobp as *mut libc::c_void,
            (*(*jobp).metalink).name,
        );
    }
    wget_debug_printf(
        b"%s: qsize %d host-qsize=%d\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"host_add_job\0"))
            .as_ptr(),
        qsize,
        (*host).qsize,
    );
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn host_add_robotstxt_job(
    mut host: *mut HOST,
    mut base: *const wget_iri,
    mut encoding: *const libc::c_char,
    mut http_fallback: bool,
) {
    let mut job: *mut JOB = 0 as *mut JOB;
    let mut blacklist_robots: *mut blacklist_entry = 0 as *mut blacklist_entry;
    let mut robot_iri: *mut wget_iri = wget_iri_parse_base(
        base,
        b"/robots.txt\0" as *const u8 as *const libc::c_char,
        encoding,
    );
    if robot_iri.is_null()
        || {
            blacklist_robots = blacklist_add(robot_iri);
            blacklist_robots.is_null()
        }
    {
        wget_iri_free(&mut robot_iri);
        return;
    }
    job = job_init(0 as *mut JOB, blacklist_robots, http_fallback);
    (*job).host = host;
    (*job).set_robotstxt(1 as libc::c_int != 0);
    wget_thread_mutex_lock(hosts_mutex);
    (*host).robot_job = job;
    (*host).qsize += 1;
    (*host).qsize;
    if !(*host).blocked() {
        qsize += 1;
        qsize;
    }
    wget_debug_printf(
        b"%s: %p %s\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"host_add_robotstxt_job\0"))
            .as_ptr(),
        job as *mut libc::c_void,
        (*(*job).iri).safe_uri,
    );
    wget_debug_printf(
        b"%s: qsize %d host-qsize=%d\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 23],
            &[libc::c_char; 23],
        >(b"host_add_robotstxt_job\0"))
            .as_ptr(),
        qsize,
        (*host).qsize,
    );
    wget_thread_mutex_unlock(hosts_mutex);
}
unsafe extern "C" fn _host_remove_job(mut host: *mut HOST, mut job: *mut JOB) {
    wget_debug_printf(
        b"%s: %p\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"_host_remove_job\0"))
            .as_ptr(),
        job as *mut libc::c_void,
    );
    if job == (*host).robot_job {
        if !((*host).robots).is_null() {
            let mut next: *mut JOB = 0 as *mut JOB;
            let mut thejob: *mut JOB = wget_list_getfirst((*host).queue) as *mut JOB;
            let mut max: libc::c_int = (*host).qsize - 1 as libc::c_int;
            while max > 0 as libc::c_int {
                next = wget_list_getnext(thejob as *const libc::c_void) as *mut JOB;
                if !(*thejob).requested_by_user() {
                    if !(*thejob).sitemap() {
                        let mut it: libc::c_int = 0 as libc::c_int;
                        let mut n: libc::c_int = wget_robots_get_path_count(
                            (*host).robots,
                        );
                        while it < n {
                            let mut path: *mut wget_string = wget_robots_get_path(
                                (*host).robots,
                                it,
                            );
                            if (*path).len != 0
                                && strncmp(
                                    ((*path).p).offset(1 as libc::c_int as isize),
                                    (if !((*(*thejob).iri).path).is_null() {
                                        (*(*thejob).iri).path
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                    }),
                                    ((*path).len).wrapping_sub(1 as libc::c_int as size_t),
                                ) == 0
                            {
                                wget_info_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"URL '%s' not followed (disallowed by robots.txt)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*(*thejob).iri).safe_uri,
                                );
                                _host_remove_job(host, thejob);
                                break;
                            } else {
                                it += 1;
                                it;
                            }
                        }
                    }
                }
                max -= 1;
                max;
                thejob = next;
            }
        }
        job_free(job);
        if !((*host).robot_job).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*host).robot_job as *mut libc::c_void);
            (*host).robot_job = 0 as *mut JOB;
        }
    } else {
        job_free(job);
        wget_list_remove(&mut (*host).queue, job as *mut libc::c_void);
    }
    (*host).qsize -= 1;
    (*host).qsize;
    if !(*host).blocked() {
        qsize -= 1;
        qsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn host_remove_job(mut host: *mut HOST, mut job: *mut JOB) {
    wget_thread_mutex_lock(hosts_mutex);
    _host_remove_job(host, job);
    wget_debug_printf(
        b"%s: qsize=%d host->qsize=%d\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"host_remove_job\0"))
            .as_ptr(),
        qsize,
        (*host).qsize,
    );
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn hosts_free() {
    wget_hashmap_free(&mut hosts);
}
#[no_mangle]
pub unsafe extern "C" fn host_increase_failure(mut host: *mut HOST) {
    wget_thread_mutex_lock(hosts_mutex);
    (*host).failures += 1;
    (*host).failures;
    (*host)
        .retry_ts = wget_get_timemillis()
        + ((*host).failures * 1000 as libc::c_int) as libc::c_longlong;
    wget_debug_printf(
        b"%s: %s failures=%d\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<
            &[u8; 22],
            &[libc::c_char; 22],
        >(b"host_increase_failure\0"))
            .as_ptr(),
        (*host).host,
        (*host).failures,
    );
    if config.tries != 0 && (*host).failures >= config.tries {
        if !(*host).blocked() {
            (*host).set_blocked(1 as libc::c_int != 0);
            qsize -= (*host).qsize;
            wget_debug_printf(
                b"%s: qsize=%d\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"host_increase_failure\0"))
                    .as_ptr(),
                qsize,
            );
        }
    }
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn host_final_failure(mut host: *mut HOST) {
    wget_thread_mutex_lock(hosts_mutex);
    if !(*host).blocked() {
        (*host).set_blocked(1 as libc::c_int != 0);
        qsize -= (*host).qsize;
        wget_debug_printf(
            b"%s: qsize=%d\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"host_final_failure\0"))
                .as_ptr(),
            qsize,
        );
    }
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn host_reset_failure(mut host: *mut HOST) {
    wget_thread_mutex_lock(hosts_mutex);
    (*host).failures = 0 as libc::c_int;
    (*host).retry_ts = 0 as libc::c_int as libc::c_longlong;
    if (*host).blocked() {
        (*host).set_blocked(0 as libc::c_int != 0);
        qsize += (*host).qsize;
        wget_debug_printf(
            b"%s: qsize=%d\n\0" as *const u8 as *const libc::c_char,
            (*::core::mem::transmute::<
                &[u8; 19],
                &[libc::c_char; 19],
            >(b"host_reset_failure\0"))
                .as_ptr(),
            qsize,
        );
    }
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn queue_empty() -> libc::c_int {
    return (qsize == 0) as libc::c_int;
}
unsafe extern "C" fn _queue_free_func(
    mut context: *mut libc::c_void,
    mut job: *mut JOB,
) -> libc::c_int {
    job_free(job);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn host_queue_free(mut host: *mut HOST) {
    wget_thread_mutex_lock(hosts_mutex);
    wget_list_browse(
        (*host).queue,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut JOB) -> libc::c_int>,
            Option::<wget_list_browse_fn>,
        >(
            Some(
                _queue_free_func
                    as unsafe extern "C" fn(*mut libc::c_void, *mut JOB) -> libc::c_int,
            ),
        ),
        0 as *mut libc::c_void,
    );
    wget_list_free(&mut (*host).queue);
    if !((*host).robot_job).is_null() {
        job_free((*host).robot_job);
        if !((*host).robot_job).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*host).robot_job as *mut libc::c_void);
            (*host).robot_job = 0 as *mut JOB;
        }
    }
    if !(*host).blocked() {
        qsize -= (*host).qsize;
    }
    (*host).qsize = 0 as libc::c_int;
    wget_thread_mutex_unlock(hosts_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn queue_size() -> libc::c_int {
    wget_debug_printf(
        b"%s: qsize=%d\n\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"queue_size\0"))
            .as_ptr(),
        qsize,
    );
    return qsize;
}
