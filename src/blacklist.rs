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
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
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
    fn wget_hashmap_remove_nofree(
        h: *mut wget_hashmap,
        key: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_set_key_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_key_destructor>,
    );
    fn wget_hashmap_set_value_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_value_destructor>,
    );
    fn wget_thread_mutex_init(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex_0: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex_0: wget_thread_mutex);
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_supported(iri: *const wget_iri) -> bool;
    fn wget_iri_compare(iri1: *const wget_iri, iri2: *const wget_iri) -> libc::c_int;
    fn wget_iri_get_path(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
        encoding: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wget_iri_get_query_as_filename(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
        encoding: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wget_iri_get_basename(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
        encoding: *const libc::c_char,
        flags: libc::c_int,
    ) -> *mut libc::c_char;
    fn wget_iri_scheme_get_name(scheme: wget_iri_scheme) -> *const libc::c_char;
    static mut config: config;
    fn wget_restrict_file_name(
        fname: *const libc::c_char,
        esc: *mut libc::c_char,
        mode: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint16_t = __uint16_t;
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
pub type wget_hashmap_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_netrc_db = wget_netrc_db_st;
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blacklist_entry {
    pub iri: *const wget_iri,
    pub local_filename: *mut libc::c_char,
}
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
static mut blacklist: *mut wget_hashmap = 0 as *const wget_hashmap as *mut wget_hashmap;
static mut mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
unsafe extern "C" fn get_local_filename_real(
    mut iri: *const wget_iri,
) -> *mut libc::c_char {
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut directories: bool = false;
    directories = config.recursive;
    if config.directories as libc::c_int == 0 as libc::c_int {
        directories = 0 as libc::c_int != 0;
    }
    if config.force_directories as libc::c_int == 1 as libc::c_int {
        directories = 1 as libc::c_int != 0;
    }
    wget_buffer_init(&mut buf, 0 as *mut libc::c_char, 256 as libc::c_int as size_t);
    if !(config.directory_prefix).is_null()
        && *config.directory_prefix as libc::c_int != 0
    {
        wget_buffer_strcat(&mut buf, config.directory_prefix);
        wget_buffer_memcat(
            &mut buf,
            b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    if directories {
        if config.protocol_directories as libc::c_int != 0
            && wget_iri_supported(iri) as libc::c_int != 0
        {
            wget_buffer_strcat(&mut buf, wget_iri_scheme_get_name((*iri).scheme));
            wget_buffer_memcat(
                &mut buf,
                b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        if config.host_directories as libc::c_int != 0 && !((*iri).host).is_null()
            && *(*iri).host as libc::c_int != 0
        {
            wget_buffer_strcat(&mut buf, (*iri).host);
            wget_buffer_memcat(
                &mut buf,
                b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        if config.cut_directories != 0 {
            let mut path_buf: wget_buffer = wget_buffer {
                data: 0 as *mut libc::c_char,
                length: 0,
                size: 0,
                release_data_release_buf_error: [0; 1],
                c2rust_padding: [0; 7],
            };
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            let mut n: libc::c_int = 0;
            let mut sbuf: [libc::c_char; 256] = [0; 256];
            wget_buffer_init(
                &mut path_buf,
                sbuf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            );
            wget_iri_get_path(iri, &mut path_buf, config.local_encoding);
            n = 0 as libc::c_int;
            p = path_buf.data;
            while n < config.cut_directories && !p.is_null() {
                p = strchr(
                    if *p as libc::c_int == '/' as i32 {
                        p.offset(1 as libc::c_int as isize)
                    } else {
                        p
                    },
                    '/' as i32,
                );
                n += 1;
                n;
            }
            if p.is_null() && !(path_buf.data).is_null() {
                p = strrchr(path_buf.data, '/' as i32);
                if p.is_null() {
                    p = path_buf.data;
                }
            }
            if !p.is_null() {
                while *p as libc::c_int == '/' as i32 {
                    p = p.offset(1);
                    p;
                }
                wget_buffer_strcat(&mut buf, p);
            }
            wget_buffer_deinit(&mut path_buf);
        } else {
            wget_iri_get_path(iri, &mut buf, config.local_encoding);
        }
        if config.cut_file_get_vars {
            fname = buf.data;
        } else {
            fname = wget_iri_get_query_as_filename(iri, &mut buf, config.local_encoding);
        }
    } else if config.cut_file_get_vars {
        fname = wget_iri_get_basename(
            iri,
            &mut buf,
            config.local_encoding,
            0 as libc::c_int,
        );
    } else {
        fname = wget_iri_get_basename(
            iri,
            &mut buf,
            config.local_encoding,
            1 as libc::c_int,
        );
    }
    if config.restrict_file_names != 0 {
        let mut tmp: [libc::c_char; 1024] = [0; 1024];
        let mut fname_esc: *mut libc::c_char = (if (::core::mem::size_of::<
            [libc::c_char; 1024],
        >() as libc::c_ulong)
            < (buf.length * 3 as libc::c_int as size_t)
                .wrapping_add(1 as libc::c_int as size_t)
        {
            tmp.as_mut_ptr() as *mut libc::c_void
        } else {
            wget_malloc(
                (buf.length * 3 as libc::c_int as size_t)
                    .wrapping_add(1 as libc::c_int as size_t),
            )
        }) as *mut libc::c_char;
        if wget_restrict_file_name(fname, fname_esc, config.restrict_file_names) != fname
        {
            wget_buffer_strcpy(&mut buf, fname_esc);
            fname = buf.data;
        }
        if fname_esc != tmp.as_mut_ptr() {
            if !fname_esc.is_null() {
                wget_free
                    .expect("non-null function pointer")(fname_esc as *mut libc::c_void);
                fname_esc = 0 as *mut libc::c_char;
            }
        }
    }
    wget_debug_printf(
        b"local filename = '%s'\n\0" as *const u8 as *const libc::c_char,
        fname,
    );
    return fname;
}
#[no_mangle]
pub unsafe extern "C" fn get_local_filename(
    mut iri: *const wget_iri,
) -> *mut libc::c_char {
    if config.delete_after {
        return 0 as *mut libc::c_char;
    }
    if (config.spider as libc::c_int != 0 || !(config.output_document).is_null())
        && !config.continue_download
    {
        return 0 as *mut libc::c_char;
    }
    return get_local_filename_real(iri);
}
unsafe extern "C" fn hash_iri(mut key: *const libc::c_void) -> libc::c_uint {
    let mut iri: *const wget_iri = key as *mut wget_iri;
    let mut h: libc::c_uint = (*iri).port as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    h = h
        .wrapping_mul(101 as libc::c_int as libc::c_uint)
        .wrapping_add((*iri).scheme as libc::c_uint);
    p = (*iri).host as *mut libc::c_uchar;
    while !p.is_null() && *p as libc::c_int != 0 {
        h = h
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    p = (*iri).path as *mut libc::c_uchar;
    while !p.is_null() && *p as libc::c_int != 0 {
        h = h
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    p = (*iri).query as *mut libc::c_uchar;
    while !p.is_null() && *p as libc::c_int != 0 {
        h = h
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return h;
}
unsafe extern "C" fn blacklist_print_entry(
    mut ctx: *mut libc::c_void,
    mut key: *const libc::c_void,
    mut value: *mut libc::c_void,
) -> libc::c_int {
    let mut iri: *const wget_iri = key as *mut wget_iri;
    wget_debug_printf(
        b"blacklist %s\n\0" as *const u8 as *const libc::c_char,
        (*iri).safe_uri,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_value(mut value: *mut libc::c_void) {
    let mut blacklistp: *mut blacklist_entry = value as *mut blacklist_entry;
    if !((*blacklistp).local_filename).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*blacklistp).local_filename as *mut libc::c_void);
        (*blacklistp).local_filename = 0 as *mut libc::c_char;
    }
    wget_iri_free(&mut (*blacklistp).iri as *mut *const wget_iri as *mut *mut wget_iri);
    if !value.is_null() {
        wget_free.expect("non-null function pointer")(value);
        value = 0 as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_init() {
    wget_thread_mutex_init(&mut mutex);
    blacklist = wget_hashmap_create(
        128 as libc::c_int,
        Some(hash_iri as wget_hashmap_hash_fn),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const wget_iri, *const wget_iri) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                wget_iri_compare
                    as unsafe extern "C" fn(
                        *const wget_iri,
                        *const wget_iri,
                    ) -> libc::c_int,
            ),
        ),
    );
    wget_hashmap_set_key_destructor(blacklist, None);
    wget_hashmap_set_value_destructor(
        blacklist,
        Some(free_value as wget_hashmap_value_destructor),
    );
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_exit() {
    wget_thread_mutex_destroy(&mut mutex);
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_print() {
    wget_hashmap_browse(
        blacklist,
        Some(blacklist_print_entry as wget_hashmap_browse_fn),
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_add(
    mut iri: *const wget_iri,
) -> *mut blacklist_entry {
    let mut entryp: *mut blacklist_entry = 0 as *mut blacklist_entry;
    wget_thread_mutex_lock(mutex);
    if wget_hashmap_get(
        blacklist,
        iri as *const libc::c_void,
        &mut entryp as *mut *mut blacklist_entry as *mut *mut libc::c_void,
    ) == 0
    {
        entryp = wget_malloc(::core::mem::size_of::<blacklist_entry>() as libc::c_ulong)
            as *mut blacklist_entry;
        (*entryp).iri = iri;
        (*entryp).local_filename = get_local_filename(iri);
        wget_hashmap_put(
            blacklist,
            iri as *const libc::c_void,
            entryp as *const libc::c_void,
        );
        wget_thread_mutex_unlock(mutex);
        return entryp;
    }
    wget_thread_mutex_unlock(mutex);
    wget_debug_printf(
        b"not requesting '%s'. (Already Seen)\n\0" as *const u8 as *const libc::c_char,
        (*iri).safe_uri,
    );
    return 0 as *mut blacklist_entry;
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_set_filename(
    mut blacklistp: *mut blacklist_entry,
    mut fname: *const libc::c_char,
) {
    if wget_strcmp((*blacklistp).local_filename, fname) == 0 {
        return;
    }
    wget_debug_printf(
        b"blacklist set filename: %s -> %s\n\0" as *const u8 as *const libc::c_char,
        (*blacklistp).local_filename,
        fname,
    );
    wget_hashmap_remove_nofree(blacklist, (*blacklistp).iri as *const libc::c_void);
    if !((*blacklistp).local_filename).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*blacklistp).local_filename as *mut libc::c_void);
        (*blacklistp).local_filename = 0 as *mut libc::c_char;
    }
    (*blacklistp).local_filename = wget_strdup(fname);
    wget_hashmap_put(
        blacklist,
        (*blacklistp).iri as *const libc::c_void,
        blacklistp as *const libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_get(
    mut iri: *const wget_iri,
) -> *mut blacklist_entry {
    let mut entryp: *mut blacklist_entry = 0 as *mut blacklist_entry;
    if wget_hashmap_get(
        blacklist,
        iri as *const libc::c_void,
        &mut entryp as *mut *mut blacklist_entry as *mut *mut libc::c_void,
    ) != 0
    {
        return entryp;
    }
    return 0 as *mut blacklist_entry;
}
#[no_mangle]
pub unsafe extern "C" fn blacklist_free() {
    wget_hashmap_free(&mut blacklist);
}
