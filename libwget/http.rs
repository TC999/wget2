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
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_thread_mutex_st;
    pub type wget_decompressor_st;
    pub type wget_hpkp_st;
    pub type wget_dns_st;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
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
    fn wget_get_timemillis() -> libc::c_longlong;
    fn wget_match_tail(s: *const libc::c_char, tail: *const libc::c_char) -> libc::c_int;
    fn wget_str_needs_encoding(s: *const libc::c_char) -> bool;
    fn wget_str_to_utf8(
        src: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wget_str_to_ascii(src: *const libc::c_char) -> *const libc::c_char;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_strmemcpy(
        s: *mut libc::c_char,
        ssize: size_t,
        m: *const libc::c_void,
        n: size_t,
    ) -> size_t;
    fn wget_base64_encode_printf_alloc(
        fmt: *const libc::c_char,
        _: ...
    ) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_alloc(size: size_t) -> *mut wget_buffer;
    fn wget_buffer_ensure_capacity(buf: *mut wget_buffer, size: size_t) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_free(buf: *mut *mut wget_buffer);
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
    fn wget_buffer_strcpy(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_strcat(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_bufcat(buf: *mut wget_buffer, src: *mut wget_buffer) -> size_t;
    fn wget_buffer_printf_append(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_buffer_printf(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_vaprintf(
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut libc::c_char;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_remove_nofree(v: *mut wget_vector, pos: libc::c_int) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_clear_nofree(v: *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_hashmap_get(
        h: *const wget_hashmap,
        key: *const libc::c_void,
        value: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn wget_decompress_open(
        encoding: wget_content_encoding,
        data_sink: Option::<wget_decompressor_sink_fn>,
        context: *mut libc::c_void,
    ) -> *mut wget_decompressor;
    fn wget_decompress_close(dc: *mut wget_decompressor);
    fn wget_decompress(
        dc: *mut wget_decompressor,
        src: *const libc::c_char,
        srclen: size_t,
    ) -> libc::c_int;
    fn wget_decompress_set_error_handler(
        dc: *mut wget_decompressor,
        error_handler: Option::<wget_decompressor_error_handler>,
    );
    fn wget_decompress_get_context(dc: *mut wget_decompressor) -> *mut libc::c_void;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_iri_get_escaped_host(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_iri_get_escaped_resource(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_iri_scheme_get_name(scheme: wget_iri_scheme) -> *const libc::c_char;
    fn wget_tcp_init() -> *mut wget_tcp;
    fn wget_tcp_deinit(tcp: *mut *mut wget_tcp);
    fn wget_tcp_set_ssl(tcp: *mut wget_tcp, ssl: bool);
    fn wget_tcp_set_ssl_hostname(tcp: *mut wget_tcp, hostname: *const libc::c_char);
    fn wget_tcp_connect(
        tcp: *mut wget_tcp,
        host: *const libc::c_char,
        port: uint16_t,
    ) -> libc::c_int;
    fn wget_tcp_tls_start(tcp: *mut wget_tcp) -> libc::c_int;
    fn wget_tcp_write(
        tcp: *mut wget_tcp,
        buf: *const libc::c_char,
        count: size_t,
    ) -> ssize_t;
    fn wget_tcp_read(
        tcp: *mut wget_tcp,
        buf: *mut libc::c_char,
        count: size_t,
    ) -> ssize_t;
    fn wget_ip_is_family(host: *const libc::c_char, family: libc::c_int) -> bool;
    fn wget_http_free_param(param: *mut wget_http_header_param);
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_parse_response_header(
        buf: *mut libc::c_char,
    ) -> *mut wget_http_response;
    fn wget_random() -> libc::c_int;
    fn wget_hash_get_len(algorithm: wget_digest_algorithm) -> libc::c_int;
    fn wget_hash_printf_hex(
        algorithm: wget_digest_algorithm,
        out: *mut libc::c_char,
        outsize: size_t,
        fmt: *const libc::c_char,
        _: ...
    );
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
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type va_list = __gnuc_va_list;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
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
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
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
pub type wget_hashmap = wget_hashmap_st;
pub type wget_stringmap = wget_hashmap;
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_decompressor = wget_decompressor_st;
pub type wget_decompressor_sink_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
pub type wget_decompressor_error_handler = unsafe extern "C" fn(
    *mut wget_decompressor,
    libc::c_int,
) -> libc::c_int;
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
pub type wget_hpkp = wget_hpkp_st;
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
pub type wget_digest_algorithm = libc::c_uint;
pub const WGET_DIGTYPE_MAX: wget_digest_algorithm = 9;
pub const WGET_DIGTYPE_SHA224: wget_digest_algorithm = 8;
pub const WGET_DIGTYPE_SHA512: wget_digest_algorithm = 7;
pub const WGET_DIGTYPE_SHA384: wget_digest_algorithm = 6;
pub const WGET_DIGTYPE_SHA256: wget_digest_algorithm = 5;
pub const WGET_DIGTYPE_MD2: wget_digest_algorithm = 4;
pub const WGET_DIGTYPE_RMD160: wget_digest_algorithm = 3;
pub const WGET_DIGTYPE_SHA1: wget_digest_algorithm = 2;
pub const WGET_DIGTYPE_MD5: wget_digest_algorithm = 1;
pub const WGET_DIGTYPE_UNKNOWN: wget_digest_algorithm = 0;
pub type wget_server_stats_callback = unsafe extern "C" fn(
    *mut wget_http_connection,
    *mut wget_http_response,
) -> ();
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
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
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
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
unsafe extern "C" fn wget_stringmap_get(
    mut h: *const wget_stringmap,
    mut key: *const libc::c_char,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    return wget_hashmap_get(h, key as *const libc::c_void, value);
}
static mut abort_indicator: libc::c_char = 0;
static mut http_proxies: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut https_proxies: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut no_proxies: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut proxy_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut hosts_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut initialized: bool = false;
unsafe extern "C" fn http_exit() {
    if initialized {
        wget_thread_mutex_destroy(&mut proxy_mutex);
        wget_thread_mutex_destroy(&mut hosts_mutex);
        initialized = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn http_init() {
    if !initialized {
        wget_thread_mutex_init(&mut proxy_mutex);
        wget_thread_mutex_init(&mut hosts_mutex);
        initialized = 1 as libc::c_int != 0;
        atexit(Some(http_exit as unsafe extern "C" fn() -> ()));
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_init() {
    http_init();
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_exit() {
    http_exit();
}
static mut server_stats_callback: Option::<wget_server_stats_callback> = None;
unsafe extern "C" fn body_callback(
    mut resp: *mut wget_http_response,
    mut user_data: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    if ((*resp).body).is_null() {
        (*resp).body = wget_buffer_alloc(102400 as libc::c_int as size_t);
    }
    wget_buffer_memcat((*resp).body, data as *const libc::c_void, length);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_create_request(
    mut iri: *const wget_iri,
    mut method: *const libc::c_char,
) -> *mut wget_http_request {
    let mut req: *mut wget_http_request = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_http_request>() as libc::c_ulong,
    ) as *mut wget_http_request;
    if req.is_null() {
        return 0 as *mut wget_http_request;
    }
    wget_buffer_init(
        &mut (*req).esc_resource,
        ((*req).esc_resource_buf).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    wget_buffer_init(
        &mut (*req).esc_host,
        ((*req).esc_host_buf).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    (*req).scheme = (*iri).scheme;
    wget_strscpy(
        ((*req).method).as_mut_ptr(),
        method,
        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
    );
    wget_iri_get_escaped_resource(iri, &mut (*req).esc_resource);
    if wget_ip_is_family((*iri).host, 2 as libc::c_int) {
        wget_buffer_printf(
            &mut (*req).esc_host as *mut wget_buffer,
            b"[%s]\0" as *const u8 as *const libc::c_char,
            (*iri).host,
        );
    } else {
        wget_iri_get_escaped_host(iri, &mut (*req).esc_host);
    }
    (*req).headers = wget_vector_create(8 as libc::c_int, None);
    wget_vector_set_destructor(
        (*req).headers,
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
    wget_http_add_header(
        req,
        b"Host\0" as *const u8 as *const libc::c_char,
        (*req).esc_host.data,
    );
    wget_http_request_set_body_cb(
        req,
        Some(body_callback as wget_http_body_callback),
        0 as *mut libc::c_void,
    );
    return req;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_set_header_cb(
    mut req: *mut wget_http_request,
    mut callback: Option::<wget_http_header_callback>,
    mut user_data: *mut libc::c_void,
) {
    (*req).header_callback = callback;
    (*req).header_user_data = user_data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_set_body_cb(
    mut req: *mut wget_http_request,
    mut callback: Option::<wget_http_body_callback>,
    mut user_data: *mut libc::c_void,
) {
    (*req).body_callback = callback;
    (*req).body_user_data = user_data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_set_int(
    mut req: *mut wget_http_request,
    mut key: libc::c_int,
    mut value: libc::c_int,
) {
    match key {
        2009 => {
            (*req).set_response_keepheader(value != 0 as libc::c_int);
        }
        2020 => {
            (*req).set_response_ignorelength(value != 0 as libc::c_int);
        }
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown key %d (or value must not be an integer)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"wget_http_request_set_int\0"))
                    .as_ptr(),
                key,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_get_int(
    mut req: *mut wget_http_request,
    mut key: libc::c_int,
) -> libc::c_int {
    match key {
        2009 => return (*req).response_keepheader() as libc::c_int,
        2020 => return (*req).response_ignorelength() as libc::c_int,
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown key %d (or value must not be an integer)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"wget_http_request_get_int\0"))
                    .as_ptr(),
                key,
            );
            return -(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_set_ptr(
    mut req: *mut wget_http_request,
    mut key: libc::c_int,
    mut value: *mut libc::c_void,
) {
    match key {
        2019 => {
            (*req).user_data = value;
        }
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown key %d (or value must not be an integer)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"wget_http_request_set_ptr\0"))
                    .as_ptr(),
                key,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_get_ptr(
    mut req: *mut wget_http_request,
    mut key: libc::c_int,
) -> *mut libc::c_void {
    match key {
        2019 => return (*req).user_data,
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s: Unknown key %d (or value must not be an integer)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"wget_http_request_get_ptr\0"))
                    .as_ptr(),
                key,
            );
            return 0 as *mut libc::c_void;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_set_body(
    mut req: *mut wget_http_request,
    mut mimetype: *const libc::c_char,
    mut body: *mut libc::c_char,
    mut length: size_t,
) {
    if !mimetype.is_null() {
        wget_http_add_header(
            req,
            b"Content-Type\0" as *const u8 as *const libc::c_char,
            mimetype,
        );
    }
    (*req).body = body;
    (*req).body_length = length;
}
unsafe extern "C" fn http_add_header(
    mut req: *mut wget_http_request,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut param: *mut wget_http_header_param = wget_malloc(
        ::core::mem::size_of::<wget_http_header_param>() as libc::c_ulong,
    ) as *mut wget_http_header_param;
    if !(param.is_null() || name.is_null() || value.is_null()) {
        (*param).name = name;
        (*param).value = value;
        if wget_vector_add((*req).headers, param as *const libc::c_void)
            >= 0 as libc::c_int
        {
            return WGET_E_SUCCESS as libc::c_int;
        }
        if !param.is_null() {
            wget_free.expect("non-null function pointer")(param as *mut libc::c_void);
            param = 0 as *mut wget_http_header_param;
        }
    }
    if !value.is_null() {
        wget_free.expect("non-null function pointer")(value as *mut libc::c_void);
        value = 0 as *const libc::c_char;
    }
    if !name.is_null() {
        wget_free.expect("non-null function pointer")(name as *mut libc::c_void);
        name = 0 as *const libc::c_char;
    }
    return WGET_E_MEMORY as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_add_header_vprintf(
    mut req: *mut wget_http_request,
    mut name: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    return http_add_header(
        req,
        wget_strdup(name),
        wget_vaprintf(fmt, args.as_va_list()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_add_header_printf(
    mut req: *mut wget_http_request,
    mut name: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut rc: libc::c_int = wget_http_add_header_vprintf(
        req,
        name,
        fmt,
        args_0.as_va_list(),
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_add_header(
    mut req: *mut wget_http_request,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    return http_add_header(req, wget_strdup(name), wget_strdup(value));
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_add_header_param(
    mut req: *mut wget_http_request,
    mut param: *mut wget_http_header_param,
) -> libc::c_int {
    return http_add_header(req, wget_strdup((*param).name), wget_strdup((*param).value));
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_add_credentials(
    mut req: *mut wget_http_request,
    mut challenge: *mut wget_http_challenge,
    mut username: *const libc::c_char,
    mut password: *const libc::c_char,
    mut proxied: libc::c_int,
) {
    if challenge.is_null() {
        return;
    }
    if username.is_null() {
        username = b"\0" as *const u8 as *const libc::c_char;
    }
    if password.is_null() {
        password = b"\0" as *const u8 as *const libc::c_char;
    }
    if wget_strcasecmp_ascii(
        (*challenge).auth_scheme,
        b"basic\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut encoded: *const libc::c_char = wget_base64_encode_printf_alloc(
            b"%s:%s\0" as *const u8 as *const libc::c_char,
            username,
            password,
        );
        if proxied != 0 {
            wget_http_add_header_printf(
                req,
                b"Proxy-Authorization\0" as *const u8 as *const libc::c_char,
                b"Basic %s\0" as *const u8 as *const libc::c_char,
                encoded,
            );
        } else {
            wget_http_add_header_printf(
                req,
                b"Authorization\0" as *const u8 as *const libc::c_char,
                b"Basic %s\0" as *const u8 as *const libc::c_char,
                encoded,
            );
        }
        if !encoded.is_null() {
            wget_free.expect("non-null function pointer")(encoded as *mut libc::c_void);
            encoded = 0 as *const libc::c_char;
        }
    } else if wget_strcasecmp_ascii(
        (*challenge).auth_scheme,
        b"digest\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut realm: *const libc::c_char = 0 as *const libc::c_char;
        let mut opaque: *const libc::c_char = 0 as *const libc::c_char;
        let mut nonce: *const libc::c_char = 0 as *const libc::c_char;
        let mut qop: *const libc::c_char = 0 as *const libc::c_char;
        let mut algorithm: *const libc::c_char = 0 as *const libc::c_char;
        let mut buf: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        let mut hashtype: libc::c_int = 0;
        let mut hashlen: libc::c_int = 0;
        if wget_stringmap_get(
            (*challenge).params,
            b"realm\0" as *const u8 as *const libc::c_char,
            &mut realm as *mut *const libc::c_char as *mut *mut libc::c_void,
        ) == 0
        {
            realm = 0 as *const libc::c_char;
        }
        if wget_stringmap_get(
            (*challenge).params,
            b"opaque\0" as *const u8 as *const libc::c_char,
            &mut opaque as *mut *const libc::c_char as *mut *mut libc::c_void,
        ) == 0
        {
            opaque = 0 as *const libc::c_char;
        }
        if wget_stringmap_get(
            (*challenge).params,
            b"nonce\0" as *const u8 as *const libc::c_char,
            &mut nonce as *mut *const libc::c_char as *mut *mut libc::c_void,
        ) == 0
        {
            nonce = 0 as *const libc::c_char;
        }
        if wget_stringmap_get(
            (*challenge).params,
            b"qop\0" as *const u8 as *const libc::c_char,
            &mut qop as *mut *const libc::c_char as *mut *mut libc::c_void,
        ) == 0
        {
            qop = 0 as *const libc::c_char;
        }
        if wget_stringmap_get(
            (*challenge).params,
            b"algorithm\0" as *const u8 as *const libc::c_char,
            &mut algorithm as *mut *const libc::c_char as *mut *mut libc::c_void,
        ) == 0
        {
            algorithm = 0 as *const libc::c_char;
        }
        if !qop.is_null()
            && (wget_strcasecmp_ascii(qop, b"auth\0" as *const u8 as *const libc::c_char)
                != 0
                && wget_strcasecmp_ascii(
                    qop,
                    b"auth-int\0" as *const u8 as *const libc::c_char,
                ) != 0)
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unsupported quality of protection '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                qop,
            );
            return;
        }
        if wget_strcasecmp_ascii(algorithm, b"MD5\0" as *const u8 as *const libc::c_char)
            == 0
            || wget_strcasecmp_ascii(
                algorithm,
                b"MD5-sess\0" as *const u8 as *const libc::c_char,
            ) == 0 || algorithm.is_null()
        {
            hashtype = WGET_DIGTYPE_MD5 as libc::c_int;
        } else if wget_strcasecmp_ascii(
            algorithm,
            b"SHA-256\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                algorithm,
                b"SHA-256-sess\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            hashtype = WGET_DIGTYPE_SHA256 as libc::c_int;
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unsupported algorithm '%s'.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                algorithm,
            );
            return;
        }
        if realm.is_null() || nonce.is_null() {
            return;
        }
        let mut a1buf: [libc::c_char; 65] = [0; 65];
        let mut a2buf: [libc::c_char; 65] = [0; 65];
        let mut response_digest: [libc::c_char; 65] = [0; 65];
        let mut cnonce: [libc::c_char; 16] = *::core::mem::transmute::<
            &[u8; 16],
            &mut [libc::c_char; 16],
        >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
        hashlen = wget_hash_get_len(hashtype as wget_digest_algorithm);
        let mut buflen: size_t = (hashlen * 2 as libc::c_int + 1 as libc::c_int)
            as size_t;
        if buflen > ::core::mem::size_of::<[libc::c_char; 65]>() as libc::c_ulong {
            return;
        }
        wget_hash_printf_hex(
            hashtype as wget_digest_algorithm,
            a1buf.as_mut_ptr(),
            buflen,
            b"%s:%s:%s\0" as *const u8 as *const libc::c_char,
            username,
            realm,
            password,
        );
        if wget_strcasecmp_ascii(
            algorithm,
            b"MD5-sess\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                algorithm,
                b"SHA-256-sess\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            wget_snprintf(
                cnonce.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                b"%08x\0" as *const u8 as *const libc::c_char,
                wget_random() as libc::c_uint,
            );
            wget_hash_printf_hex(
                hashtype as wget_digest_algorithm,
                a1buf.as_mut_ptr(),
                buflen,
                b"%s:%s:%s\0" as *const u8 as *const libc::c_char,
                a1buf.as_mut_ptr(),
                nonce,
                cnonce.as_mut_ptr(),
            );
        }
        wget_hash_printf_hex(
            hashtype as wget_digest_algorithm,
            a2buf.as_mut_ptr(),
            buflen,
            b"%s:/%s\0" as *const u8 as *const libc::c_char,
            ((*req).method).as_mut_ptr(),
            (*req).esc_resource.data,
        );
        if qop.is_null() {
            wget_hash_printf_hex(
                hashtype as wget_digest_algorithm,
                response_digest.as_mut_ptr(),
                buflen,
                b"%s:%s:%s\0" as *const u8 as *const libc::c_char,
                a1buf.as_mut_ptr(),
                nonce,
                a2buf.as_mut_ptr(),
            );
        } else {
            if *cnonce.as_mut_ptr() == 0 {
                wget_snprintf(
                    cnonce.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    b"%08x\0" as *const u8 as *const libc::c_char,
                    wget_random() as libc::c_uint,
                );
            }
            wget_hash_printf_hex(
                hashtype as wget_digest_algorithm,
                response_digest.as_mut_ptr(),
                buflen,
                b"%s:%s:00000001:%s:%s:%s\0" as *const u8 as *const libc::c_char,
                a1buf.as_mut_ptr(),
                nonce,
                cnonce.as_mut_ptr(),
                qop,
                a2buf.as_mut_ptr(),
            );
        }
        wget_buffer_init(&mut buf, 0 as *mut libc::c_char, 256 as libc::c_int as size_t);
        wget_buffer_printf(
            &mut buf as *mut wget_buffer,
            b"Digest username=\"%s\", realm=\"%s\", nonce=\"%s\", uri=\"/%s\", response=\"%s\"\0"
                as *const u8 as *const libc::c_char,
            username,
            realm,
            nonce,
            (*req).esc_resource.data,
            response_digest.as_mut_ptr(),
        );
        if wget_strcasecmp_ascii(qop, b"auth\0" as *const u8 as *const libc::c_char) == 0
        {
            wget_buffer_printf_append(
                &mut buf as *mut wget_buffer,
                b", qop=auth, nc=00000001, cnonce=\"%s\"\0" as *const u8
                    as *const libc::c_char,
                cnonce.as_mut_ptr(),
            );
        }
        if !opaque.is_null() {
            wget_buffer_printf_append(
                &mut buf as *mut wget_buffer,
                b", opaque=\"%s\"\0" as *const u8 as *const libc::c_char,
                opaque,
            );
        }
        if !algorithm.is_null() {
            wget_buffer_printf_append(
                &mut buf as *mut wget_buffer,
                b", algorithm=%s\0" as *const u8 as *const libc::c_char,
                algorithm,
            );
        }
        if proxied != 0 {
            wget_http_add_header(
                req,
                b"Proxy-Authorization\0" as *const u8 as *const libc::c_char,
                buf.data,
            );
        } else {
            wget_http_add_header(
                req,
                b"Authorization\0" as *const u8 as *const libc::c_char,
                buf.data,
            );
        }
        wget_buffer_deinit(&mut buf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn http_decompress_error_handler_cb(
    mut dc: *mut wget_decompressor,
    mut err: libc::c_int,
) -> libc::c_int {
    let mut resp: *mut wget_http_response = wget_decompress_get_context(dc)
        as *mut wget_http_response;
    if !resp.is_null() && !((*resp).req).is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Decompress failed [host: %s - resource: %s]\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*resp).req).esc_host.data,
            (*(*resp).req).esc_resource.data,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_get_body_cb(
    mut userdata: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut resp: *mut wget_http_response = userdata as *mut wget_http_response;
    return ((*(*resp).req).body_callback)
        .expect(
            "non-null function pointer",
        )(resp, (*(*resp).req).body_user_data, data, length);
}
#[no_mangle]
pub unsafe extern "C" fn http_fix_broken_server_encoding(
    mut resp: *mut wget_http_response,
) {
    if (*resp).content_encoding as libc::c_int
        == wget_content_encoding_gzip as libc::c_int
    {
        let mut ext: *const libc::c_char = 0 as *const libc::c_char;
        if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"application/x-gzip\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/gzip\0" as *const u8 as *const libc::c_char,
            ) == 0
            || wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/gunzip\0" as *const u8 as *const libc::c_char,
            ) == 0
            || {
                ext = strrchr((*(*resp).req).esc_resource.data, '.' as i32);
                !ext.is_null()
                    && (wget_strcasecmp_ascii(
                        ext,
                        b".gz\0" as *const u8 as *const libc::c_char,
                    ) == 0
                        || wget_strcasecmp_ascii(
                            ext,
                            b".tgz\0" as *const u8 as *const libc::c_char,
                        ) == 0)
            }
        {
            wget_debug_printf(
                b"Broken server configuration gzip workaround triggered\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*resp)
                .content_encoding = wget_content_encoding_identity as libc::c_int
                as libc::c_char;
        }
    }
}
unsafe extern "C" fn establish_proxy_connect(
    mut tcp: *mut wget_tcp,
    mut host: *const libc::c_char,
    mut port: uint16_t,
) -> libc::c_int {
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
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
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if wget_ip_is_family(host, 2 as libc::c_int) {
        wget_buffer_printf(
            &mut buf as *mut wget_buffer,
            b"CONNECT [%s]:%hu HTTP/1.1\r\nHost: [%s]:%hu\r\n\r\n\0" as *const u8
                as *const libc::c_char,
            host,
            port as libc::c_int,
            host,
            port as libc::c_int,
        );
    } else {
        wget_buffer_printf(
            &mut buf as *mut wget_buffer,
            b"CONNECT %s:%hu HTTP/1.1\r\nHost: %s:%hu\r\n\r\n\0" as *const u8
                as *const libc::c_char,
            host,
            port as libc::c_int,
            host,
            port as libc::c_int,
        );
    }
    if wget_tcp_write(tcp, buf.data, buf.length) != buf.length as ssize_t {
        wget_buffer_deinit(&mut buf);
        return WGET_E_CONNECT as libc::c_int;
    }
    wget_buffer_deinit(&mut buf);
    let mut nbytes: ssize_t = 0;
    nbytes = wget_tcp_read(
        tcp,
        sbuf.as_mut_ptr(),
        (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if nbytes < 0 as libc::c_int as ssize_t {
        return WGET_E_CONNECT as libc::c_int;
    }
    sbuf[nbytes as usize] = 0 as libc::c_int as libc::c_char;
    while nbytes > 0 as libc::c_int as ssize_t
        && {
            nbytes -= 1;
            c_isspace(sbuf[nbytes as usize] as libc::c_int) as libc::c_int != 0
        }
    {
        sbuf[nbytes as usize] = 0 as libc::c_int as libc::c_char;
    }
    if wget_strncasecmp_ascii(
        sbuf.as_mut_ptr(),
        b"HTTP/1.1 200\0" as *const u8 as *const libc::c_char,
        12 as libc::c_int as size_t,
    ) != 0
        && wget_strncasecmp_ascii(
            sbuf.as_mut_ptr(),
            b"HTTP/1.0 200\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        ) != 0
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Proxy connection failed with: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            sbuf.as_mut_ptr(),
        );
        return WGET_E_CONNECT as libc::c_int;
    }
    wget_debug_printf(
        b"Proxy connection established: %s\n\0" as *const u8 as *const libc::c_char,
        sbuf.as_mut_ptr(),
    );
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_open(
    mut _conn: *mut *mut wget_http_connection,
    mut iri: *const wget_iri,
) -> libc::c_int {
    static mut next_http_proxy: libc::c_int = -(1 as libc::c_int);
    static mut next_https_proxy: libc::c_int = -(1 as libc::c_int);
    let mut conn: *mut wget_http_connection = 0 as *mut wget_http_connection;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: uint16_t = 0;
    let mut rc: libc::c_int = 0;
    let mut ssl: libc::c_int = ((*iri).scheme as libc::c_uint
        == WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint) as libc::c_int;
    let mut need_connect: bool = 0 as libc::c_int != 0;
    if _conn.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    conn = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_http_connection>() as libc::c_ulong,
    ) as *mut wget_http_connection;
    if conn.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    *_conn = conn;
    host = (*iri).host;
    port = (*iri).port;
    (*conn).tcp = wget_tcp_init();
    if wget_http_match_no_proxy(no_proxies, (*iri).host) == 0 {
        if ssl == 0 && !http_proxies.is_null() {
            wget_thread_mutex_lock(proxy_mutex);
            next_http_proxy += 1;
            let mut proxy: *mut wget_iri = wget_vector_get(
                http_proxies,
                next_http_proxy % wget_vector_size(http_proxies),
            ) as *mut wget_iri;
            wget_thread_mutex_unlock(proxy_mutex);
            if proxy.is_null() {
                if !conn.is_null() {
                    wget_free
                        .expect("non-null function pointer")(conn as *mut libc::c_void);
                    conn = 0 as *mut wget_http_connection;
                }
                *_conn = 0 as *mut wget_http_connection;
                return WGET_E_UNKNOWN as libc::c_int;
            }
            host = (*proxy).host;
            port = (*proxy).port;
            ssl = ((*proxy).scheme as libc::c_uint
                == WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint) as libc::c_int;
            (*conn).set_proxied(1 as libc::c_int != 0);
        } else if ssl != 0 && !https_proxies.is_null() {
            wget_thread_mutex_lock(proxy_mutex);
            next_https_proxy += 1;
            let mut proxy_0: *mut wget_iri = wget_vector_get(
                https_proxies,
                next_https_proxy % wget_vector_size(https_proxies),
            ) as *mut wget_iri;
            wget_thread_mutex_unlock(proxy_mutex);
            if proxy_0.is_null() {
                if !conn.is_null() {
                    wget_free
                        .expect("non-null function pointer")(conn as *mut libc::c_void);
                    conn = 0 as *mut wget_http_connection;
                }
                *_conn = 0 as *mut wget_http_connection;
                return WGET_E_UNKNOWN as libc::c_int;
            }
            host = (*proxy_0).host;
            port = (*proxy_0).port;
            ssl = ((*proxy_0).scheme as libc::c_uint
                == WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint) as libc::c_int;
            need_connect = 1 as libc::c_int != 0;
        }
    }
    if ssl != 0 {
        wget_tcp_set_ssl((*conn).tcp, 1 as libc::c_int != 0);
        wget_tcp_set_ssl_hostname((*conn).tcp, host);
    }
    rc = wget_tcp_connect((*conn).tcp, host, port);
    if rc != WGET_E_SUCCESS as libc::c_int {
        if server_stats_callback.is_some() && rc == WGET_E_CERTIFICATE as libc::c_int {
            server_stats_callback
                .expect("non-null function pointer")(conn, 0 as *mut wget_http_response);
        }
        wget_http_close(_conn);
        return rc;
    }
    if need_connect {
        rc = establish_proxy_connect((*conn).tcp, (*iri).host, (*iri).port);
        if rc != WGET_E_SUCCESS as libc::c_int {
            wget_http_close(_conn);
            return rc;
        }
        if (*iri).scheme as libc::c_uint
            == WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
        {
            wget_tcp_set_ssl((*conn).tcp, 1 as libc::c_int != 0);
            wget_tcp_set_ssl_hostname((*conn).tcp, (*iri).host);
            wget_tcp_tls_start((*conn).tcp);
        }
    }
    (*conn)
        .esc_host = if !((*iri).host).is_null() {
        wget_strdup((*iri).host)
    } else {
        0 as *mut libc::c_char
    };
    (*conn).port = (*iri).port;
    (*conn).scheme = (*iri).scheme;
    (*conn).buf = wget_buffer_alloc(102400 as libc::c_int as size_t);
    (*conn).pending_requests = wget_vector_create(16 as libc::c_int, None);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_close(mut conn: *mut *mut wget_http_connection) {
    if !(*conn).is_null() {
        wget_debug_printf(b"closing connection\n\0" as *const u8 as *const libc::c_char);
        wget_tcp_deinit(&mut (**conn).tcp);
        if !((**conn).esc_host).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**conn).esc_host as *mut libc::c_void);
            (**conn).esc_host = 0 as *const libc::c_char;
        }
        wget_buffer_free(&mut (**conn).buf);
        wget_vector_clear_nofree((**conn).pending_requests);
        wget_vector_free(&mut (**conn).pending_requests);
        if !(*conn).is_null() {
            wget_free.expect("non-null function pointer")(*conn as *mut libc::c_void);
            *conn = 0 as *mut wget_http_connection;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_send_request(
    mut conn: *mut wget_http_connection,
    mut req: *mut wget_http_request,
) -> libc::c_int {
    let mut nbytes: ssize_t = 0;
    nbytes = wget_http_request_to_buffer(
        req,
        (*conn).buf,
        (*conn).proxied() as libc::c_int,
        (*conn).port as libc::c_int,
    );
    if nbytes < 0 as libc::c_int as ssize_t {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to create request buffer\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    (*req).request_start = wget_get_timemillis();
    if wget_tcp_write((*conn).tcp, (*(*conn).buf).data, nbytes as size_t) != nbytes {
        return -(1 as libc::c_int);
    }
    wget_vector_add((*conn).pending_requests, req as *const libc::c_void);
    if (*req).debug_skip_body() {
        wget_debug_printf(
            b"# sent %zd bytes:\n%.*s<body skipped>\0" as *const u8
                as *const libc::c_char,
            nbytes,
            ((*(*conn).buf).length).wrapping_sub((*req).body_length) as libc::c_int,
            (*(*conn).buf).data,
        );
    } else {
        wget_debug_printf(
            b"# sent %zd bytes:\n%.*s\0" as *const u8 as *const libc::c_char,
            nbytes,
            (*(*conn).buf).length as libc::c_int,
            (*(*conn).buf).data,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_request_to_buffer(
    mut req: *mut wget_http_request,
    mut buf: *mut wget_buffer,
    mut proxied: libc::c_int,
    mut port: libc::c_int,
) -> ssize_t {
    let mut have_content_length: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut check_content_length: libc::c_char = (!((*req).body).is_null()
        && (*req).body_length != 0) as libc::c_int as libc::c_char;
    wget_buffer_strcpy(buf, ((*req).method).as_mut_ptr());
    wget_buffer_memcat(
        buf,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    if proxied != 0 {
        wget_buffer_strcat(buf, wget_iri_scheme_get_name((*req).scheme));
        wget_buffer_memcat(
            buf,
            b"://\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
        wget_buffer_bufcat(buf, &mut (*req).esc_host);
        wget_buffer_printf_append(
            buf,
            b":%d\0" as *const u8 as *const libc::c_char,
            port,
        );
    }
    wget_buffer_memcat(
        buf,
        b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    wget_buffer_bufcat(buf, &mut (*req).esc_resource);
    wget_buffer_memcat(
        buf,
        b" HTTP/1.1\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        11 as libc::c_int as size_t,
    );
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size((*req).headers) {
        let mut param: *mut wget_http_header_param = wget_vector_get((*req).headers, it)
            as *mut wget_http_header_param;
        if !param.is_null() {
            wget_buffer_strcat(buf, (*param).name);
            wget_buffer_memcat(
                buf,
                b": \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
            wget_buffer_strcat(buf, (*param).value);
            if *((*buf).data)
                .offset(
                    ((*buf).length).wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as libc::c_int != '\n' as i32
            {
                wget_buffer_memcat(
                    buf,
                    b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            if check_content_length as libc::c_int != 0
                && wget_strcasecmp_ascii(
                    (*param).name,
                    b"Content-Length\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                have_content_length = 1 as libc::c_int as libc::c_char;
            }
        }
        it += 1;
        it;
    }
    if check_content_length as libc::c_int != 0 && have_content_length == 0 {
        wget_buffer_printf_append(
            buf,
            b"Content-Length: %zu\r\n\0" as *const u8 as *const libc::c_char,
            (*req).body_length,
        );
    }
    wget_buffer_memcat(
        buf,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    if !((*req).body).is_null() && (*req).body_length != 0 {
        wget_buffer_memcat(buf, (*req).body as *const libc::c_void, (*req).body_length);
    }
    return (*buf).length as ssize_t;
}
unsafe extern "C" fn get_page(mut req: *mut wget_http_request) -> *mut libc::c_char {
    return wget_aprintf(
        b"%s://%s/%s\0" as *const u8 as *const libc::c_char,
        wget_iri_scheme_get_name((*req).scheme),
        (*req).esc_host.data,
        (*req).esc_resource.data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_response_cb(
    mut conn: *mut wget_http_connection,
) -> *mut wget_http_response {
    let mut current_block: u64;
    let mut bufsize: size_t = 0;
    let mut body_len: size_t = 0 as libc::c_int as size_t;
    let mut body_size: size_t = 0 as libc::c_int as size_t;
    let mut nbytes: ssize_t = 0;
    let mut nread: ssize_t = 0 as libc::c_int as ssize_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut dc: *mut wget_decompressor = 0 as *mut wget_decompressor;
    let mut req: *mut wget_http_request = wget_vector_get(
        (*conn).pending_requests,
        0 as libc::c_int,
    ) as *mut wget_http_request;
    wget_debug_printf(
        b"### req %p pending requests = %d\n\0" as *const u8 as *const libc::c_char,
        req as *mut libc::c_void,
        wget_vector_size((*conn).pending_requests),
    );
    if !req.is_null() {
        wget_vector_remove_nofree((*conn).pending_requests, 0 as libc::c_int);
        buf = (*(*conn).buf).data;
        bufsize = (*(*conn).buf).size;
        's_30: loop {
            nbytes = wget_tcp_read(
                (*conn).tcp,
                buf.offset(nread as isize),
                bufsize.wrapping_sub(nread as size_t),
            );
            if !(nbytes > 0 as libc::c_int as ssize_t) {
                current_block = 7828949454673616476;
                break;
            }
            (*req).first_response_start = wget_get_timemillis();
            nread += nbytes;
            *buf.offset(nread as isize) = 0 as libc::c_int as libc::c_char;
            loop {
                if nread < 4 as libc::c_int as ssize_t {
                    continue 's_30;
                }
                if nread - nbytes <= 4 as libc::c_int as ssize_t {
                    p = buf;
                } else {
                    p = buf
                        .offset(nread as isize)
                        .offset(-(nbytes as isize))
                        .offset(-(3 as libc::c_int as isize));
                }
                p = strstr(p, b"\r\n\r\n\0" as *const u8 as *const libc::c_char);
                if !p.is_null() {
                    *p = 0 as libc::c_int as libc::c_char;
                    wget_debug_printf(
                        b"# got header %zd bytes:\n%s\n\n\0" as *const u8
                            as *const libc::c_char,
                        p.offset_from(buf) as libc::c_long,
                        buf,
                    );
                    resp = wget_http_parse_response_header(buf);
                    if resp.is_null() {
                        current_block = 7456705339585925631;
                        break 's_30;
                    }
                    if (*resp).code as libc::c_int >= 100 as libc::c_int
                        && ((*resp).code as libc::c_int) < 200 as libc::c_int
                    {
                        wget_http_free_response(&mut resp);
                        p = p.offset(4 as libc::c_int as isize);
                        nread -= p.offset_from(buf) as libc::c_long;
                        nbytes = nread;
                        memmove(
                            buf as *mut libc::c_void,
                            p as *const libc::c_void,
                            (nread + 1 as libc::c_int as ssize_t) as libc::c_ulong,
                        );
                    } else {
                        if (*req).response_keepheader() {
                            let mut header: *mut wget_buffer = wget_buffer_alloc(
                                (p.offset_from(buf) as libc::c_long
                                    + 4 as libc::c_int as libc::c_long) as size_t,
                            );
                            wget_buffer_memcpy(
                                header,
                                buf as *const libc::c_void,
                                p.offset_from(buf) as libc::c_long as size_t,
                            );
                            wget_buffer_memcat(
                                header,
                                b"\r\n\0" as *const u8 as *const libc::c_char
                                    as *const libc::c_void,
                                2 as libc::c_int as size_t,
                            );
                            (*resp).header = header;
                        }
                        (*resp).req = req;
                        if server_stats_callback.is_some() {
                            server_stats_callback
                                .expect("non-null function pointer")(conn, resp);
                        }
                        if ((*req).header_callback).is_some() {
                            ((*req).header_callback)
                                .expect(
                                    "non-null function pointer",
                                )(resp, (*req).header_user_data);
                        }
                        if wget_strcasecmp_ascii(
                            ((*req).method).as_mut_ptr(),
                            b"HEAD\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            current_block = 7456705339585925631;
                            break 's_30;
                        }
                        http_fix_broken_server_encoding(resp);
                        p = p.offset(4 as libc::c_int as isize);
                        current_block = 7828949454673616476;
                        break 's_30;
                    }
                } else {
                    if !((nread as size_t).wrapping_add(1024 as libc::c_int as size_t)
                        > bufsize)
                    {
                        continue 's_30;
                    }
                    if wget_buffer_ensure_capacity(
                        (*conn).buf,
                        bufsize.wrapping_add(1024 as libc::c_int as size_t),
                    ) != WGET_E_SUCCESS as libc::c_int
                    {
                        current_block = 17281240262373992796;
                        break;
                    } else {
                        current_block = 1538046216550696469;
                        break;
                    }
                }
            }
            match current_block {
                17281240262373992796 => {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to allocate %zu bytes\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        bufsize.wrapping_add(1024 as libc::c_int as size_t),
                    );
                    current_block = 7456705339585925631;
                    break;
                }
                _ => {
                    buf = (*(*conn).buf).data;
                    bufsize = (*(*conn).buf).size;
                }
            }
        }
        match current_block {
            7456705339585925631 => {}
            _ => {
                if !(nread == 0) {
                    if !p.is_null() {
                        if !(!resp.is_null()
                            && (*resp).code as libc::c_int == 416 as libc::c_int)
                        {
                            if !(resp.is_null()
                                || (*resp).code as libc::c_int == 204 as libc::c_int
                                || (*resp).code as libc::c_int == 304 as libc::c_int
                                || (*resp).transfer_encoding as libc::c_uint
                                    == wget_transfer_encoding_identity as libc::c_int
                                        as libc::c_uint
                                    && (*resp).content_length == 0 as libc::c_int as size_t
                                    && (*resp).content_length_valid() as libc::c_int != 0)
                            {
                                dc = wget_decompress_open(
                                    (*resp).content_encoding as wget_content_encoding,
                                    Some(
                                        http_get_body_cb
                                            as unsafe extern "C" fn(
                                                *mut libc::c_void,
                                                *const libc::c_char,
                                                size_t,
                                            ) -> libc::c_int,
                                    ),
                                    resp as *mut libc::c_void,
                                );
                                wget_decompress_set_error_handler(
                                    dc,
                                    Some(
                                        http_decompress_error_handler_cb
                                            as unsafe extern "C" fn(
                                                *mut wget_decompressor,
                                                libc::c_int,
                                            ) -> libc::c_int,
                                    ),
                                );
                                body_len = (nread - p.offset_from(buf) as libc::c_long)
                                    as size_t;
                                memmove(
                                    buf as *mut libc::c_void,
                                    p as *const libc::c_void,
                                    body_len,
                                );
                                *buf
                                    .offset(
                                        body_len as isize,
                                    ) = 0 as libc::c_int as libc::c_char;
                                (*resp).cur_downloaded = body_len;
                                if (*resp).transfer_encoding as libc::c_uint
                                    == wget_transfer_encoding_chunked as libc::c_int
                                        as libc::c_uint
                                {
                                    let mut chunk_size: size_t = 0 as libc::c_int as size_t;
                                    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
                                    wget_debug_printf(
                                        b"method 1 %zu %zu:\n\0" as *const u8
                                            as *const libc::c_char,
                                        body_len,
                                        body_size,
                                    );
                                    p = buf;
                                    's_242: loop {
                                        end = strchr(p, '\r' as i32);
                                        if end.is_null()
                                            || *end.offset(1 as libc::c_int as isize) as libc::c_int
                                                != '\n' as i32
                                        {
                                            if http_connection_is_aborted(conn) != 0 {
                                                break;
                                            }
                                            if body_len.wrapping_add(1024 as libc::c_int as size_t)
                                                > bufsize
                                            {
                                                if wget_buffer_ensure_capacity(
                                                    (*conn).buf,
                                                    bufsize.wrapping_add(1024 as libc::c_int as size_t),
                                                ) != WGET_E_SUCCESS as libc::c_int
                                                {
                                                    let mut page: *mut libc::c_char = get_page(req);
                                                    wget_error_printf(
                                                        dcgettext(
                                                            0 as *const libc::c_char,
                                                            b"Failed to allocate %zu bytes (%s)\n\0" as *const u8
                                                                as *const libc::c_char,
                                                            5 as libc::c_int,
                                                        ),
                                                        bufsize.wrapping_add(1024 as libc::c_int as size_t),
                                                        page,
                                                    );
                                                    if !page.is_null() {
                                                        wget_free
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(page as *mut libc::c_void);
                                                        page = 0 as *mut libc::c_char;
                                                    }
                                                    break;
                                                } else {
                                                    p = ((*(*conn).buf).data)
                                                        .offset(p.offset_from(buf) as libc::c_long as isize);
                                                    buf = (*(*conn).buf).data;
                                                    bufsize = (*(*conn).buf).size;
                                                }
                                            }
                                            nbytes = wget_tcp_read(
                                                (*conn).tcp,
                                                buf.offset(body_len as isize),
                                                bufsize.wrapping_sub(body_len),
                                            );
                                            if nbytes <= 0 as libc::c_int as ssize_t {
                                                break;
                                            }
                                            body_len = body_len.wrapping_add(nbytes as size_t);
                                            *buf
                                                .offset(
                                                    body_len as isize,
                                                ) = 0 as libc::c_int as libc::c_char;
                                        } else {
                                            end = end.offset(2 as libc::c_int as isize);
                                            *__errno_location() = 0 as libc::c_int;
                                            chunk_size = strtoll(
                                                p,
                                                0 as *mut *mut libc::c_char,
                                                16 as libc::c_int,
                                            ) as size_t;
                                            if *__errno_location() != 0 {
                                                let mut page_0: *mut libc::c_char = get_page(req);
                                                wget_error_printf(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Failed to convert chunk size '%.31s' (%s)\n\0"
                                                            as *const u8 as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    p,
                                                    page_0,
                                                );
                                                if !page_0.is_null() {
                                                    wget_free
                                                        .expect(
                                                            "non-null function pointer",
                                                        )(page_0 as *mut libc::c_void);
                                                    page_0 = 0 as *mut libc::c_char;
                                                }
                                                break;
                                            } else if chunk_size == 0 as libc::c_int as size_t {
                                                if *end as libc::c_int == '\r' as i32
                                                    && *end.offset(1 as libc::c_int as isize) as libc::c_int
                                                        == '\n' as i32
                                                {
                                                    break;
                                                }
                                                wget_debug_printf(
                                                    b"reading trailer\n\0" as *const u8 as *const libc::c_char,
                                                );
                                                while (strstr(
                                                    end,
                                                    b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
                                                ))
                                                    .is_null()
                                                {
                                                    if body_len > 3 as libc::c_int as size_t {
                                                        memmove(
                                                            buf as *mut libc::c_void,
                                                            buf
                                                                .offset(body_len as isize)
                                                                .offset(-(3 as libc::c_int as isize))
                                                                as *const libc::c_void,
                                                            4 as libc::c_int as libc::c_ulong,
                                                        );
                                                        body_len = 3 as libc::c_int as size_t;
                                                    }
                                                    if http_connection_is_aborted(conn) != 0 {
                                                        break 's_242;
                                                    }
                                                    nbytes = wget_tcp_read(
                                                        (*conn).tcp,
                                                        buf.offset(body_len as isize),
                                                        bufsize.wrapping_sub(body_len),
                                                    );
                                                    if nbytes <= 0 as libc::c_int as ssize_t {
                                                        break 's_242;
                                                    }
                                                    body_len = body_len.wrapping_add(nbytes as size_t);
                                                    *buf
                                                        .offset(
                                                            body_len as isize,
                                                        ) = 0 as libc::c_int as libc::c_char;
                                                    end = buf;
                                                }
                                                wget_debug_printf(
                                                    b"end of trailer\n\0" as *const u8 as *const libc::c_char,
                                                );
                                                break;
                                            } else if chunk_size
                                                > (18446744073709551615 as libc::c_ulong)
                                                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong)
                                                || end
                                                    >= end
                                                        .offset(chunk_size as isize)
                                                        .offset(2 as libc::c_int as isize)
                                            {
                                                let mut page_1: *mut libc::c_char = get_page(req);
                                                wget_error_printf(
                                                    dcgettext(
                                                        0 as *const libc::c_char,
                                                        b"Chunk size overflow: %zX (%s)\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        5 as libc::c_int,
                                                    ),
                                                    chunk_size,
                                                    page_1,
                                                );
                                                if !page_1.is_null() {
                                                    wget_free
                                                        .expect(
                                                            "non-null function pointer",
                                                        )(page_1 as *mut libc::c_void);
                                                    page_1 = 0 as *mut libc::c_char;
                                                }
                                                break;
                                            } else {
                                                p = end
                                                    .offset(chunk_size as isize)
                                                    .offset(2 as libc::c_int as isize);
                                                if p <= buf.offset(body_len as isize) {
                                                    (*resp)
                                                        .cur_downloaded = ((*resp).cur_downloaded)
                                                        .wrapping_add(chunk_size);
                                                    wget_decompress(dc, end, chunk_size);
                                                } else {
                                                    if buf.offset(body_len as isize).offset_from(end)
                                                        as libc::c_long as uintptr_t > chunk_size
                                                    {
                                                        (*resp)
                                                            .cur_downloaded = ((*resp).cur_downloaded)
                                                            .wrapping_add(chunk_size);
                                                        wget_decompress(dc, end, chunk_size);
                                                    } else {
                                                        (*resp)
                                                            .cur_downloaded = ((*resp).cur_downloaded)
                                                            .wrapping_add(
                                                                buf.offset(body_len as isize).offset_from(end)
                                                                    as libc::c_long as size_t,
                                                            );
                                                        wget_decompress(
                                                            dc,
                                                            end,
                                                            buf.offset(body_len as isize).offset_from(end)
                                                                as libc::c_long as size_t,
                                                        );
                                                    }
                                                    chunk_size = (p as uintptr_t)
                                                        .wrapping_sub(buf.offset(body_len as isize) as uintptr_t);
                                                    wget_debug_printf(
                                                        b"need at least %zu more bytes\n\0" as *const u8
                                                            as *const libc::c_char,
                                                        chunk_size,
                                                    );
                                                    while chunk_size > 0 as libc::c_int as size_t {
                                                        if http_connection_is_aborted(conn) != 0 {
                                                            break 's_242;
                                                        }
                                                        nbytes = wget_tcp_read((*conn).tcp, buf, bufsize);
                                                        if nbytes <= 0 as libc::c_int as ssize_t {
                                                            break 's_242;
                                                        }
                                                        if chunk_size <= nbytes as size_t {
                                                            if chunk_size == 1 as libc::c_int as size_t
                                                                || strncmp(
                                                                    buf
                                                                        .offset(chunk_size as isize)
                                                                        .offset(-(2 as libc::c_int as isize)),
                                                                    b"\r\n\0" as *const u8 as *const libc::c_char,
                                                                    2 as libc::c_int as libc::c_ulong,
                                                                ) == 0
                                                            {
                                                                wget_debug_printf(
                                                                    b"chunk completed\n\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if chunk_size > 2 as libc::c_int as size_t {
                                                                    (*resp)
                                                                        .cur_downloaded = ((*resp).cur_downloaded)
                                                                        .wrapping_add(
                                                                            chunk_size.wrapping_sub(2 as libc::c_int as size_t),
                                                                        );
                                                                    wget_decompress(
                                                                        dc,
                                                                        buf,
                                                                        chunk_size.wrapping_sub(2 as libc::c_int as size_t),
                                                                    );
                                                                }
                                                                body_len = (nbytes as size_t).wrapping_sub(chunk_size);
                                                                if body_len != 0 {
                                                                    memmove(
                                                                        buf as *mut libc::c_void,
                                                                        buf.offset(chunk_size as isize) as *const libc::c_void,
                                                                        body_len,
                                                                    );
                                                                }
                                                                *buf
                                                                    .offset(
                                                                        body_len as isize,
                                                                    ) = 0 as libc::c_int as libc::c_char;
                                                                p = buf;
                                                                break;
                                                            } else {
                                                                let mut page_2: *mut libc::c_char = get_page(req);
                                                                wget_error_printf(
                                                                    dcgettext(
                                                                        0 as *const libc::c_char,
                                                                        b"Expected end-of-chunk not found (%s)\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        5 as libc::c_int,
                                                                    ),
                                                                    page_2,
                                                                );
                                                                if !page_2.is_null() {
                                                                    wget_free
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(page_2 as *mut libc::c_void);
                                                                    page_2 = 0 as *mut libc::c_char;
                                                                }
                                                                break 's_242;
                                                            }
                                                        } else {
                                                            chunk_size = chunk_size.wrapping_sub(nbytes as size_t);
                                                            if chunk_size >= 2 as libc::c_int as size_t {
                                                                (*resp)
                                                                    .cur_downloaded = ((*resp).cur_downloaded)
                                                                    .wrapping_add(nbytes as size_t);
                                                                wget_decompress(dc, buf, nbytes as size_t);
                                                            } else {
                                                                (*resp)
                                                                    .cur_downloaded = ((*resp).cur_downloaded)
                                                                    .wrapping_add(
                                                                        (nbytes - 1 as libc::c_int as ssize_t) as size_t,
                                                                    );
                                                                wget_decompress(
                                                                    dc,
                                                                    buf,
                                                                    (nbytes - 1 as libc::c_int as ssize_t) as size_t,
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else if (*resp).content_length_valid() as libc::c_int != 0
                                    && !(*(*resp).req).response_ignorelength()
                                {
                                    wget_debug_printf(
                                        b"method 2\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    if body_len != 0 {
                                        wget_decompress(dc, buf, body_len);
                                    }
                                    while body_len < (*resp).content_length {
                                        if http_connection_is_aborted(conn) != 0 {
                                            break;
                                        }
                                        nbytes = wget_tcp_read((*conn).tcp, buf, bufsize);
                                        if nbytes <= 0 as libc::c_int as ssize_t {
                                            break;
                                        }
                                        body_len = body_len.wrapping_add(nbytes as size_t);
                                        (*resp)
                                            .cur_downloaded = ((*resp).cur_downloaded)
                                            .wrapping_add(nbytes as size_t);
                                        wget_decompress(dc, buf, nbytes as size_t);
                                    }
                                    if nbytes < 0 as libc::c_int as ssize_t {
                                        let mut page_3: *mut libc::c_char = get_page(req);
                                        wget_error_printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Failed to read %zd bytes (%d) (%s)\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            nbytes,
                                            *__errno_location(),
                                            page_3,
                                        );
                                        if !page_3.is_null() {
                                            wget_free
                                                .expect(
                                                    "non-null function pointer",
                                                )(page_3 as *mut libc::c_void);
                                            page_3 = 0 as *mut libc::c_char;
                                        }
                                    }
                                    if body_len < (*resp).content_length {
                                        (*resp).set_length_inconsistent(1 as libc::c_int != 0);
                                        let mut page_4: *mut libc::c_char = get_page(req);
                                        wget_error_printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Just got %zu of %zu bytes (%s)\n\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            body_len,
                                            (*resp).content_length,
                                            page_4,
                                        );
                                        if !page_4.is_null() {
                                            wget_free
                                                .expect(
                                                    "non-null function pointer",
                                                )(page_4 as *mut libc::c_void);
                                            page_4 = 0 as *mut libc::c_char;
                                        }
                                    } else if body_len > (*resp).content_length {
                                        (*resp).set_length_inconsistent(1 as libc::c_int != 0);
                                        let mut page_5: *mut libc::c_char = get_page(req);
                                        wget_error_printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Body too large: %zu instead of %zu bytes (%s)\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            body_len,
                                            (*resp).content_length,
                                            page_5,
                                        );
                                        if !page_5.is_null() {
                                            wget_free
                                                .expect(
                                                    "non-null function pointer",
                                                )(page_5 as *mut libc::c_void);
                                            page_5 = 0 as *mut libc::c_char;
                                        }
                                    }
                                    (*resp).content_length = body_len;
                                } else {
                                    wget_debug_printf(
                                        b"method 3\n\0" as *const u8 as *const libc::c_char,
                                    );
                                    if body_len != 0 {
                                        wget_decompress(dc, buf, body_len);
                                    }
                                    while http_connection_is_aborted(conn) == 0
                                        && {
                                            nbytes = wget_tcp_read((*conn).tcp, buf, bufsize);
                                            nbytes > 0 as libc::c_int as ssize_t
                                        }
                                    {
                                        body_len = body_len.wrapping_add(nbytes as size_t);
                                        (*resp)
                                            .cur_downloaded = ((*resp).cur_downloaded)
                                            .wrapping_add(nbytes as size_t);
                                        wget_decompress(dc, buf, nbytes as size_t);
                                    }
                                    (*resp).content_length = body_len;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !resp.is_null() {
        (*resp).response_end = wget_get_timemillis();
    }
    wget_decompress_close(dc);
    return resp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_response(
    mut conn: *mut wget_http_connection,
) -> *mut wget_http_response {
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    resp = wget_http_get_response_cb(conn);
    if !resp.is_null() {
        if wget_strcasecmp_ascii(
            ((*(*resp).req).method).as_mut_ptr(),
            b"GET\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !((*resp).body).is_null() {
                (*resp).content_length = (*(*resp).body).length;
            }
        }
    }
    return resp;
}
unsafe extern "C" fn iri_free(mut iri: *mut libc::c_void) {
    if !iri.is_null() {
        wget_iri_free(&mut iri as *mut *mut libc::c_void as *mut *mut wget_iri);
    }
}
unsafe extern "C" fn parse_proxies(
    mut proxy: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> *mut wget_vector {
    if proxy.is_null() {
        return 0 as *mut wget_vector;
    }
    let mut proxies: *mut wget_vector = 0 as *mut wget_vector;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = proxy;
    s = p;
    while *p != 0 {
        p = strchrnul(s, ',' as i32);
        if p != s
            && (p.offset_from(s) as libc::c_long) < 256 as libc::c_int as libc::c_long
        {
            let mut iri: *mut wget_iri = 0 as *mut wget_iri;
            let mut host: [libc::c_char; 256] = [0; 256];
            wget_strmemcpy(
                host.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                s as *const libc::c_void,
                p.offset_from(s) as libc::c_long as size_t,
            );
            iri = wget_iri_parse(host.as_mut_ptr(), encoding);
            if !iri.is_null() {
                if proxies.is_null() {
                    proxies = wget_vector_create(8 as libc::c_int, None);
                    wget_vector_set_destructor(
                        proxies,
                        Some(iri_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                    );
                }
                wget_vector_add(proxies, iri as *const libc::c_void);
            }
        }
        s = p.offset(1 as libc::c_int as isize);
    }
    return proxies;
}
unsafe extern "C" fn parse_no_proxies(
    mut no_proxy: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> *mut wget_vector {
    if no_proxy.is_null() {
        return 0 as *mut wget_vector;
    }
    let mut proxies: *mut wget_vector = 0 as *mut wget_vector;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    proxies = wget_vector_create(8 as libc::c_int, None);
    p = no_proxy;
    s = p;
    while *p != 0 {
        while c_isspace(*s as libc::c_int) as libc::c_int != 0 && s < p {
            s = s.offset(1);
            s;
        }
        p = strchrnul(s, ',' as i32);
        if p != s
            && (p.offset_from(s) as libc::c_long) < 256 as libc::c_int as libc::c_long
        {
            let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut hostp: *mut libc::c_char = 0 as *mut libc::c_char;
            while c_isspace(*s as libc::c_int) as libc::c_int != 0 && s < p {
                s = s.offset(1);
                s;
            }
            if !(s >= p
                || {
                    host = wget_strmemdup(
                        s as *const libc::c_void,
                        p.offset_from(s) as libc::c_long as size_t,
                    );
                    host.is_null()
                })
            {
                wget_strtolower(host);
                if wget_str_needs_encoding(host) {
                    hostp = wget_str_to_utf8(host, encoding);
                    if !hostp.is_null() {
                        if !host.is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )(host as *mut libc::c_void);
                            host = 0 as *mut libc::c_char;
                        }
                        host = hostp;
                    }
                }
                hostp = wget_str_to_ascii(host) as *mut libc::c_char;
                if hostp != host {
                    if !host.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(host as *mut libc::c_void);
                        host = 0 as *mut libc::c_char;
                    }
                    host = hostp;
                }
                wget_vector_add(proxies, host as *const libc::c_void);
            }
        }
        s = p.offset(1 as libc::c_int as isize);
    }
    return proxies;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_set_http_proxy(
    mut proxy: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> libc::c_int {
    if !http_proxies.is_null() {
        wget_vector_free(&mut http_proxies);
    }
    http_proxies = parse_proxies(proxy, encoding);
    return wget_vector_size(http_proxies);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_set_https_proxy(
    mut proxy: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> libc::c_int {
    if !https_proxies.is_null() {
        wget_vector_free(&mut https_proxies);
    }
    https_proxies = parse_proxies(proxy, encoding);
    return wget_vector_size(https_proxies);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_set_no_proxy(
    mut no_proxy: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> libc::c_int {
    if !no_proxies.is_null() {
        wget_vector_free(&mut no_proxies);
    }
    no_proxies = parse_no_proxies(no_proxy, encoding);
    if no_proxies.is_null() {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_no_proxy() -> *const wget_vector {
    return no_proxies;
}
unsafe extern "C" fn cidr_v4_match(
    mut cidr: *const libc::c_char,
    mut addr: *mut in_addr,
) -> bool {
    let mut slash_pos: *const libc::c_char = strchr(cidr, '/' as i32);
    if slash_pos.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut prefix_len: libc::c_int = atoi(slash_pos.offset(1 as libc::c_int as isize));
    if prefix_len < 0 as libc::c_int || prefix_len > 32 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut network_addr: in_addr = in_addr { s_addr: 0 };
    let mut prefix: *const libc::c_char = wget_strmemdup(
        cidr as *const libc::c_void,
        slash_pos.offset_from(cidr) as libc::c_long as size_t,
    );
    if inet_pton(
        2 as libc::c_int,
        prefix,
        &mut network_addr as *mut in_addr as *mut libc::c_void,
    ) != 1 as libc::c_int
    {
        if !prefix.is_null() {
            wget_free.expect("non-null function pointer")(prefix as *mut libc::c_void);
            prefix = 0 as *const libc::c_char;
        }
        return 0 as libc::c_int != 0;
    }
    if !prefix.is_null() {
        wget_free.expect("non-null function pointer")(prefix as *mut libc::c_void);
        prefix = 0 as *const libc::c_char;
    }
    let mut mask: uint32_t = !(0xffffffff as libc::c_ulonglong >> prefix_len)
        as uint32_t;
    let mut network: uint32_t = __bswap_32(network_addr.s_addr) & mask;
    let mut test_addr: uint32_t = __bswap_32((*addr).s_addr);
    return test_addr & mask == network;
}
unsafe extern "C" fn cidr_v6_match(
    mut cidr: *const libc::c_char,
    mut addr: *mut in6_addr,
) -> bool {
    let mut slash_pos: *const libc::c_char = strchr(cidr, '/' as i32);
    if slash_pos.is_null() {
        return 0 as libc::c_int != 0;
    }
    let mut prefix_len: libc::c_int = atoi(slash_pos.offset(1 as libc::c_int as isize));
    if prefix_len < 0 as libc::c_int || prefix_len > 128 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    let mut network_addr: in6_addr = in6_addr {
        __in6_u: C2RustUnnamed {
            __u6_addr8: [0; 16],
        },
    };
    let mut prefix: *const libc::c_char = wget_strmemdup(
        cidr as *const libc::c_void,
        slash_pos.offset_from(cidr) as libc::c_long as size_t,
    );
    if inet_pton(
        10 as libc::c_int,
        prefix,
        &mut network_addr as *mut in6_addr as *mut libc::c_void,
    ) != 1 as libc::c_int
    {
        if !prefix.is_null() {
            wget_free.expect("non-null function pointer")(prefix as *mut libc::c_void);
            prefix = 0 as *const libc::c_char;
        }
        return 0 as libc::c_int != 0;
    }
    if !prefix.is_null() {
        wget_free.expect("non-null function pointer")(prefix as *mut libc::c_void);
        prefix = 0 as *const libc::c_char;
    }
    let mut bytes: libc::c_int = prefix_len / 8 as libc::c_int;
    if bytes != 0
        && memcmp(
            (network_addr.__in6_u.__u6_addr8).as_mut_ptr() as *const libc::c_void,
            ((*addr).__in6_u.__u6_addr8).as_mut_ptr() as *const libc::c_void,
            bytes as libc::c_ulong,
        ) != 0
    {
        return 0 as libc::c_int != 0;
    }
    let mut bits: libc::c_int = prefix_len & 7 as libc::c_int;
    if bits == 0 {
        return 1 as libc::c_int != 0;
    }
    let mut mask: uint8_t = !(0xff as libc::c_int >> bits) as uint8_t;
    return (network_addr.__in6_u.__u6_addr8[bytes as usize] as libc::c_int
        ^ (*addr).__in6_u.__u6_addr8[bytes as usize] as libc::c_int)
        & mask as libc::c_int == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_match_no_proxy(
    mut no_proxies_vec: *const wget_vector,
    mut host: *const libc::c_char,
) -> libc::c_int {
    if wget_vector_size(no_proxies_vec) < 1 as libc::c_int || host.is_null() {
        return 0 as libc::c_int;
    }
    let mut addr: in_addr = in_addr { s_addr: 0 };
    let mut addr6: in6_addr = in6_addr {
        __in6_u: C2RustUnnamed {
            __u6_addr8: [0; 16],
        },
    };
    let mut ipv4: bool = 0 as libc::c_int != 0;
    let mut ipv6: bool = 0 as libc::c_int != 0;
    if inet_pton(2 as libc::c_int, host, &mut addr as *mut in_addr as *mut libc::c_void)
        == 1 as libc::c_int
    {
        ipv4 = 1 as libc::c_int != 0;
    } else if inet_pton(
        10 as libc::c_int,
        host,
        &mut addr6 as *mut in6_addr as *mut libc::c_void,
    ) == 1 as libc::c_int
    {
        ipv6 = 1 as libc::c_int != 0;
    }
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(no_proxies_vec) {
        let mut no_proxy: *const libc::c_char = wget_vector_get(no_proxies_vec, it)
            as *const libc::c_char;
        if !no_proxy.is_null() {
            if strcmp(no_proxy, host) == 0 {
                return 1 as libc::c_int;
            }
            if ipv4 {
                if cidr_v4_match(no_proxy, &mut addr) {
                    return 1 as libc::c_int;
                }
            } else if ipv6 {
                if cidr_v6_match(no_proxy, &mut addr6) {
                    return 1 as libc::c_int;
                }
            }
            if *no_proxy as libc::c_int == '.' as i32
                && wget_match_tail(host, no_proxy) != 0
            {
                return 1 as libc::c_int;
            }
        }
        it += 1;
        it;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_abort_connection(
    mut conn: *mut wget_http_connection,
) {
    if !conn.is_null() {
        (*conn).set_abort_indicator(1 as libc::c_int != 0);
    } else {
        abort_indicator = 1 as libc::c_int as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn http_connection_is_aborted(
    mut conn: *mut wget_http_connection,
) -> libc::c_int {
    return ((*conn).abort_indicator() as libc::c_int != 0
        || abort_indicator as libc::c_int != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_connection_receive_only(
    mut conn: *mut wget_http_connection,
) -> bool {
    return (*conn).goaway();
}
#[no_mangle]
pub unsafe extern "C" fn wget_server_set_stats_callback(
    mut fn_0: Option::<wget_server_stats_callback>,
) {
    server_stats_callback = fn_0;
}
