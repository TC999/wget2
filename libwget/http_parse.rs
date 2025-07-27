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
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_cookie_st;
    pub type wget_hpkp_st;
    pub type wget_tcp_st;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_percent_unescape(src: *mut libc::c_char) -> libc::c_int;
    fn wget_str_needs_encoding(s: *const libc::c_char) -> bool;
    fn wget_str_is_valid_utf8(utf8: *const libc::c_char) -> bool;
    fn wget_str_to_utf8(
        src: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut libc::c_char;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_strmemcpy_a(
        s: *mut libc::c_char,
        ssize: size_t,
        m: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_free(buf: *mut *mut wget_buffer);
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
    fn wget_vector_add_memdup(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
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
    fn wget_stringmap_create_nocase(max: libc::c_int) -> *mut wget_stringmap;
    fn wget_cookie_free(cookie: *mut *mut wget_cookie);
    fn wget_cookie_parse_setcookie(
        s: *const libc::c_char,
        cookie: *mut *mut wget_cookie,
    ) -> *const libc::c_char;
    fn wget_hpkp_new() -> *mut wget_hpkp;
    fn wget_hpkp_free(hpkp: *mut wget_hpkp);
    fn wget_hpkp_pin_add(
        hpkp: *mut wget_hpkp,
        pin_type: *const libc::c_char,
        pin_b64: *const libc::c_char,
    );
    fn wget_hpkp_set_maxage(hpkp: *mut wget_hpkp, maxage: int64_t);
    fn wget_hpkp_set_include_subdomains(hpkp: *mut wget_hpkp, include_subdomains: bool);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const wget_content_encoding_max: C2RustUnnamed_0 = 9;
pub const wget_content_encoding_lzip: C2RustUnnamed_0 = 8;
pub const wget_content_encoding_zstd: C2RustUnnamed_0 = 7;
pub const wget_content_encoding_brotli: C2RustUnnamed_0 = 6;
pub const wget_content_encoding_bzip2: C2RustUnnamed_0 = 5;
pub const wget_content_encoding_lzma: C2RustUnnamed_0 = 4;
pub const wget_content_encoding_xz: C2RustUnnamed_0 = 3;
pub const wget_content_encoding_deflate: C2RustUnnamed_0 = 2;
pub const wget_content_encoding_gzip: C2RustUnnamed_0 = 1;
pub const wget_content_encoding_identity: C2RustUnnamed_0 = 0;
pub const wget_content_encoding_unknown: C2RustUnnamed_0 = -1;
pub type wget_iri_scheme = libc::c_uint;
pub const WGET_IRI_SCHEME_HTTPS: wget_iri_scheme = 1;
pub const WGET_IRI_SCHEME_HTTP: wget_iri_scheme = 0;
pub type wget_cookie = wget_cookie_st;
pub type wget_hpkp = wget_hpkp_st;
pub type wget_tcp = wget_tcp_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_header_param {
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_link {
    pub uri: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub pri: libc::c_int,
    pub rel: C2RustUnnamed_1,
}
pub type C2RustUnnamed_1 = libc::c_uint;
pub const link_rel_duplicate: C2RustUnnamed_1 = 2;
pub const link_rel_describedby: C2RustUnnamed_1 = 1;
pub const link_rel_none: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_digest {
    pub algorithm: *const libc::c_char,
    pub encoded_digest: *const libc::c_char,
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn c_isblank(mut c: libc::c_int) -> bool {
    return c == ' ' as i32 || c == '\t' as i32;
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
unsafe extern "C" fn wget_stringmap_free(mut h: *mut *mut wget_stringmap) {
    wget_hashmap_free(h);
}
static mut http_ctype: [libc::c_uchar; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[inline]
unsafe extern "C" fn http_isseparator(mut c: libc::c_char) -> bool {
    return http_ctype[c as libc::c_uchar as usize] as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_host(
    mut conn: *const wget_http_connection,
) -> *const libc::c_char {
    return (*conn).esc_host;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_port(
    mut conn: *const wget_http_connection,
) -> uint16_t {
    return (*conn).port;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_scheme(
    mut conn: *const wget_http_connection,
) -> wget_iri_scheme {
    return (*conn).scheme;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get_protocol(
    mut conn: *const wget_http_connection,
) -> libc::c_int {
    return (*conn).protocol as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_isseparator(mut c: libc::c_char) -> bool {
    return http_isseparator(c);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_istoken(mut c: libc::c_char) -> bool {
    return c as libc::c_int > 32 as libc::c_int && c as libc::c_int <= 126 as libc::c_int
        && !http_isseparator(c);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_token(
    mut s: *const libc::c_char,
    mut token: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = s;
    while wget_http_istoken(*s) {
        s = s.offset(1);
        s;
    }
    *token = wget_strmemdup(
        p as *const libc::c_void,
        s.offset_from(p) as libc::c_long as size_t,
    );
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_quoted_string(
    mut s: *const libc::c_char,
    mut qstring: *mut *const libc::c_char,
) -> *const libc::c_char {
    if *s as libc::c_int == '"' as i32 {
        s = s.offset(1);
        let mut p: *const libc::c_char = s;
        while *s != 0 {
            if *s as libc::c_int == '"' as i32 {
                break;
            }
            if *s as libc::c_int == '\\' as i32
                && *s.offset(1 as libc::c_int as isize) as libc::c_int != 0
            {
                s = s.offset(2 as libc::c_int as isize);
            } else {
                s = s.offset(1);
                s;
            }
        }
        *qstring = wget_strmemdup(
            p as *const libc::c_void,
            s.offset_from(p) as libc::c_long as size_t,
        );
        if *s as libc::c_int == '"' as i32 {
            s = s.offset(1);
            s;
        }
    } else {
        *qstring = 0 as *const libc::c_char;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_param(
    mut s: *const libc::c_char,
    mut param: *mut *const libc::c_char,
    mut value: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    *value = 0 as *const libc::c_char;
    *param = *value;
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == ';' as i32 {
        s = s.offset(1);
        s;
        while c_isblank(*s as libc::c_int) {
            s = s.offset(1);
            s;
        }
    }
    if *s == 0 {
        return s;
    }
    p = s;
    while wget_http_istoken(*s) {
        s = s.offset(1);
        s;
    }
    *param = wget_strmemdup(
        p as *const libc::c_void,
        s.offset_from(p) as libc::c_long as size_t,
    );
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int != 0
        && {
            let fresh0 = s;
            s = s.offset(1);
            *fresh0 as libc::c_int == '=' as i32
        }
    {
        while c_isblank(*s as libc::c_int) {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == '"' as i32 {
            s = wget_http_parse_quoted_string(s, value);
        } else {
            s = wget_http_parse_token(s, value);
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_name(
    mut s: *const libc::c_char,
    mut name: *mut *const libc::c_char,
) -> *const libc::c_char {
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    s = wget_http_parse_token(s, name);
    while *s as libc::c_int != 0 && *s as libc::c_int != ':' as i32 {
        s = s.offset(1);
        s;
    }
    return if *s as libc::c_int == ':' as i32 {
        s.offset(1 as libc::c_int as isize)
    } else {
        s
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_parse_name_fixed(
    mut s: *const libc::c_char,
    mut name: *mut *const libc::c_char,
    mut namelen: *mut size_t,
) -> *const libc::c_char {
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    *name = s;
    while wget_http_istoken(*s) {
        s = s.offset(1);
        s;
    }
    *namelen = s.offset_from(*name) as libc::c_long as size_t;
    while *s as libc::c_int != 0 && *s as libc::c_int != ':' as i32 {
        s = s.offset(1);
        s;
    }
    return if *s as libc::c_int == ':' as i32 {
        s.offset(1 as libc::c_int as isize)
    } else {
        s
    };
}
unsafe extern "C" fn compare_param(
    mut p1: *mut wget_http_header_param,
    mut p2: *mut wget_http_header_param,
) -> libc::c_int {
    return wget_strcasecmp_ascii((*p1).name, (*p2).name);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_add_param(
    mut params: *mut *mut wget_vector,
    mut param: *mut wget_http_header_param,
) {
    if (*params).is_null() {
        *params = wget_vector_create(
            4 as libc::c_int,
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
                    compare_param
                        as unsafe extern "C" fn(
                            *mut wget_http_header_param,
                            *mut wget_http_header_param,
                        ) -> libc::c_int,
                ),
            ),
        );
    }
    wget_vector_add_memdup(
        *params,
        param as *const libc::c_void,
        ::core::mem::size_of::<wget_http_header_param>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_link(
    mut s: *const libc::c_char,
    mut link: *mut wget_http_link,
) -> *const libc::c_char {
    memset(
        link as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<wget_http_link>() as libc::c_ulong,
    );
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '<' as i32 {
        let mut p: *const libc::c_char = s.offset(1 as libc::c_int as isize);
        s = strchr(p, '>' as i32);
        if !s.is_null() {
            let mut name: *const libc::c_char = 0 as *const libc::c_char;
            let mut value: *const libc::c_char = 0 as *const libc::c_char;
            (*link)
                .uri = wget_strmemdup(
                p as *const libc::c_void,
                s.offset_from(p) as libc::c_long as size_t,
            );
            s = s.offset(1);
            s;
            while c_isblank(*s as libc::c_int) {
                s = s.offset(1);
                s;
            }
            while *s as libc::c_int == ';' as i32 {
                s = wget_http_parse_param(s, &mut name, &mut value);
                if !name.is_null() && !value.is_null() {
                    if wget_strcasecmp_ascii(
                        name,
                        b"rel\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if wget_strcasecmp_ascii(
                            value,
                            b"describedby\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*link).rel = link_rel_describedby;
                        } else if wget_strcasecmp_ascii(
                            value,
                            b"duplicate\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*link).rel = link_rel_duplicate;
                        }
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"pri\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*link).pri = atoi(value);
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"type\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if ((*link).type_0).is_null() {
                            (*link).type_0 = value;
                            value = 0 as *const libc::c_char;
                        }
                    }
                    while c_isblank(*s as libc::c_int) {
                        s = s.offset(1);
                        s;
                    }
                }
                if !name.is_null() {
                    wget_free
                        .expect("non-null function pointer")(name as *mut libc::c_void);
                    name = 0 as *const libc::c_char;
                }
                if !value.is_null() {
                    wget_free
                        .expect("non-null function pointer")(value as *mut libc::c_void);
                    value = 0 as *const libc::c_char;
                }
            }
            while *s as libc::c_int != 0 && !c_isblank(*s as libc::c_int) {
                s = s.offset(1);
                s;
            }
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_digest(
    mut s: *const libc::c_char,
    mut digest: *mut wget_http_digest,
) -> *const libc::c_char {
    memset(
        digest as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<wget_http_digest>() as libc::c_ulong,
    );
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    s = wget_http_parse_token(s, &mut (*digest).algorithm);
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '=' as i32 {
        s = s.offset(1);
        s;
        while c_isblank(*s as libc::c_int) {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == '"' as i32 {
            s = wget_http_parse_quoted_string(s, &mut (*digest).encoded_digest);
        } else {
            let mut p: *const libc::c_char = 0 as *const libc::c_char;
            p = s;
            while *s as libc::c_int != 0 && !c_isblank(*s as libc::c_int)
                && *s as libc::c_int != ',' as i32 && *s as libc::c_int != ';' as i32
            {
                s = s.offset(1);
                s;
            }
            (*digest)
                .encoded_digest = wget_strmemdup(
                p as *const libc::c_void,
                s.offset_from(p) as libc::c_long as size_t,
            );
        }
    }
    while *s as libc::c_int != 0 && !c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_challenge(
    mut s: *const libc::c_char,
    mut challenge: *mut wget_http_challenge,
) -> *const libc::c_char {
    memset(
        challenge as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<wget_http_challenge>() as libc::c_ulong,
    );
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    s = wget_http_parse_token(s, &mut (*challenge).auth_scheme);
    if *s as libc::c_int == ' ' as i32 {
        s = s.offset(1);
        s;
    } else {
        if !((*challenge).auth_scheme).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*challenge).auth_scheme as *mut libc::c_void);
            (*challenge).auth_scheme = 0 as *const libc::c_char;
        }
        return s;
    }
    let mut param: wget_http_header_param = wget_http_header_param {
        name: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
    };
    let mut current_block_36: u64;
    loop {
        let mut old: *const libc::c_char = s;
        s = wget_http_parse_param(s, &mut param.name, &mut param.value);
        if !(param.name).is_null() {
            if *param.name as libc::c_int != 0 && (param.value).is_null() {
                if !(param.name).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(param.name as *mut libc::c_void);
                    param.name = 0 as *const libc::c_char;
                }
                return old;
            }
            if (param.value).is_null() {
                if !(param.name).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(param.name as *mut libc::c_void);
                    param.name = 0 as *const libc::c_char;
                }
                current_block_36 = 7746791466490516765;
            } else {
                if ((*challenge).params).is_null() {
                    (*challenge).params = wget_stringmap_create_nocase(8 as libc::c_int);
                }
                wget_stringmap_put(
                    (*challenge).params,
                    param.name,
                    param.value as *const libc::c_void,
                );
                current_block_36 = 5689316957504528238;
            }
        } else {
            current_block_36 = 5689316957504528238;
        }
        match current_block_36 {
            5689316957504528238 => {
                while c_isblank(*s as libc::c_int) {
                    s = s.offset(1);
                    s;
                }
                if *s as libc::c_int != ',' as i32 {
                    break;
                }
                if *s != 0 {
                    s = s.offset(1);
                    s;
                }
            }
            _ => {}
        }
        if !(*s != 0) {
            break;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_challenges(
    mut s: *const libc::c_char,
    mut challenges: *mut wget_vector,
) -> *const libc::c_char {
    let mut challenge: wget_http_challenge = wget_http_challenge {
        auth_scheme: 0 as *const libc::c_char,
        params: 0 as *mut wget_stringmap,
    };
    while *s != 0 {
        s = wget_http_parse_challenge(s, &mut challenge);
        if !(challenge.auth_scheme).is_null() {
            wget_vector_add_memdup(
                challenges,
                &mut challenge as *mut wget_http_challenge as *const libc::c_void,
                ::core::mem::size_of::<wget_http_challenge>() as libc::c_ulong,
            );
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_location(
    mut s: *const libc::c_char,
    mut location: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    p = s;
    while *s as libc::c_int != 0 && *s as libc::c_int != '\r' as i32
        && *s as libc::c_int != '\n' as i32
    {
        s = s.offset(1);
        s;
    }
    while s > p
        && c_isblank(*s.offset(-(1 as libc::c_int as isize)) as libc::c_int)
            as libc::c_int != 0
    {
        s = s.offset(-1);
        s;
    }
    *location = wget_strmemdup(
        p as *const libc::c_void,
        s.offset_from(p) as libc::c_long as size_t,
    );
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_transfer_encoding(
    mut s: *const libc::c_char,
    mut transfer_encoding: *mut wget_transfer_encoding,
) -> *const libc::c_char {
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if wget_strcasecmp_ascii(s, b"identity\0" as *const u8 as *const libc::c_char) == 0 {
        *transfer_encoding = wget_transfer_encoding_identity;
    } else {
        *transfer_encoding = wget_transfer_encoding_chunked;
    }
    while wget_http_istoken(*s) {
        s = s.offset(1);
        s;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_content_type(
    mut s: *const libc::c_char,
    mut content_type: *mut *const libc::c_char,
    mut charset: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut param: wget_http_header_param = wget_http_header_param {
        name: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
    };
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    p = s;
    while *s as libc::c_int != 0
        && (wget_http_istoken(*s) as libc::c_int != 0 || *s as libc::c_int == '/' as i32)
    {
        s = s.offset(1);
        s;
    }
    if !content_type.is_null() {
        *content_type = wget_strmemdup(
            p as *const libc::c_void,
            s.offset_from(p) as libc::c_long as size_t,
        );
    }
    if !charset.is_null() {
        *charset = 0 as *const libc::c_char;
        while *s != 0 {
            s = wget_http_parse_param(s, &mut param.name, &mut param.value);
            if wget_strcasecmp_ascii(
                b"charset\0" as *const u8 as *const libc::c_char,
                param.name,
            ) == 0
            {
                if !(param.name).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(param.name as *mut libc::c_void);
                    param.name = 0 as *const libc::c_char;
                }
                *charset = param.value;
                break;
            } else {
                if !(param.name).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(param.name as *mut libc::c_void);
                    param.name = 0 as *const libc::c_char;
                }
                if !(param.value).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(param.value as *mut libc::c_void);
                    param.value = 0 as *const libc::c_char;
                }
            }
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_content_disposition(
    mut s: *const libc::c_char,
    mut filename: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut param: wget_http_header_param = wget_http_header_param {
        name: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if !filename.is_null() {
        *filename = 0 as *const libc::c_char;
        while *s as libc::c_int != 0 && (*filename).is_null() {
            s = wget_http_parse_param(s, &mut param.name, &mut param.value);
            if !(param.value).is_null()
                && wget_strcasecmp_ascii(
                    b"filename\0" as *const u8 as *const libc::c_char,
                    param.name,
                ) == 0
            {
                if (*filename).is_null() {
                    p = strpbrk(
                        param.value,
                        b"/\\\0" as *const u8 as *const libc::c_char,
                    );
                    if !p.is_null() {
                        p = wget_strdup(p.offset(1 as libc::c_int as isize));
                    } else {
                        p = param.value as *mut libc::c_char;
                        param.value = 0 as *const libc::c_char;
                    }
                    wget_percent_unescape(p);
                    if !wget_str_is_valid_utf8(p) {
                        *filename = wget_str_to_utf8(
                            p,
                            b"iso-8859-1\0" as *const u8 as *const libc::c_char,
                        );
                        if !p.is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )(p as *mut libc::c_void);
                            p = 0 as *mut libc::c_char;
                        }
                    } else {
                        *filename = p;
                        p = 0 as *mut libc::c_char;
                    }
                }
            } else if !(param.value).is_null()
                && wget_strcasecmp_ascii(
                    b"filename*\0" as *const u8 as *const libc::c_char,
                    param.name,
                ) == 0
            {
                p = strchr(param.value, '\'' as i32);
                if !p.is_null() {
                    let mut charset: *const libc::c_char = param.value;
                    let mut language: *const libc::c_char = p
                        .offset(1 as libc::c_int as isize);
                    *p = 0 as libc::c_int as libc::c_char;
                    p = strchr(language, '\'' as i32);
                    if !p.is_null() {
                        let fresh1 = p;
                        p = p.offset(1);
                        *fresh1 = 0 as libc::c_int as libc::c_char;
                        if *p != 0 {
                            wget_percent_unescape(p);
                            if wget_str_needs_encoding(p) {
                                *filename = wget_str_to_utf8(p, charset);
                            } else {
                                *filename = wget_strdup(p);
                            }
                            if !(*filename).is_null()
                                && {
                                    p = strpbrk(
                                        *filename,
                                        b"/\\\0" as *const u8 as *const libc::c_char,
                                    );
                                    !p.is_null()
                                }
                            {
                                p = wget_strdup(p.offset(1 as libc::c_int as isize));
                                if !(*filename).is_null() {
                                    wget_free
                                        .expect(
                                            "non-null function pointer",
                                        )(*filename as *mut libc::c_void);
                                    *filename = 0 as *const libc::c_char;
                                }
                                *filename = p;
                            }
                            if !(param.name).is_null() {
                                wget_free
                                    .expect(
                                        "non-null function pointer",
                                    )(param.name as *mut libc::c_void);
                                param.name = 0 as *const libc::c_char;
                            }
                            if !(param.value).is_null() {
                                wget_free
                                    .expect(
                                        "non-null function pointer",
                                    )(param.value as *mut libc::c_void);
                                param.value = 0 as *const libc::c_char;
                            }
                            break;
                        }
                    }
                }
            }
            if !(param.name).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(param.name as *mut libc::c_void);
                param.name = 0 as *const libc::c_char;
            }
            if !(param.value).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(param.value as *mut libc::c_void);
                param.value = 0 as *const libc::c_char;
            }
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_public_key_pins(
    mut s: *const libc::c_char,
    mut hpkp: *mut wget_hpkp,
) -> *const libc::c_char {
    let mut param: wget_http_header_param = wget_http_header_param {
        name: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
    };
    wget_hpkp_set_include_subdomains(hpkp, 0 as libc::c_int != 0);
    while *s != 0 {
        s = wget_http_parse_param(s, &mut param.name, &mut param.value);
        if !(param.value).is_null() {
            if wget_strcasecmp_ascii(
                param.name,
                b"max-age\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                wget_hpkp_set_maxage(hpkp, atoll(param.value) as int64_t);
            } else if wget_strncasecmp_ascii(
                param.name,
                b"pin-\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            ) == 0
            {
                wget_hpkp_pin_add(
                    hpkp,
                    (param.name).offset(4 as libc::c_int as isize),
                    param.value,
                );
            }
        } else if wget_strcasecmp_ascii(
            param.name,
            b"includeSubDomains\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            wget_hpkp_set_include_subdomains(hpkp, 1 as libc::c_int != 0);
        }
        if !(param.name).is_null() {
            wget_free
                .expect("non-null function pointer")(param.name as *mut libc::c_void);
            param.name = 0 as *const libc::c_char;
        }
        if !(param.value).is_null() {
            wget_free
                .expect("non-null function pointer")(param.value as *mut libc::c_void);
            param.value = 0 as *const libc::c_char;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_strict_transport_security(
    mut s: *const libc::c_char,
    mut maxage: *mut int64_t,
    mut include_subdomains: *mut bool,
) -> *const libc::c_char {
    let mut param: wget_http_header_param = wget_http_header_param {
        name: 0 as *const libc::c_char,
        value: 0 as *const libc::c_char,
    };
    *maxage = 0 as libc::c_int as int64_t;
    *include_subdomains = 0 as libc::c_int != 0;
    while *s != 0 {
        s = wget_http_parse_param(s, &mut param.name, &mut param.value);
        if !(param.value).is_null() {
            if wget_strcasecmp_ascii(
                param.name,
                b"max-age\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                *maxage = atoll(param.value) as int64_t;
            }
        } else if wget_strcasecmp_ascii(
            param.name,
            b"includeSubDomains\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            *include_subdomains = 1 as libc::c_int != 0;
        }
        if !(param.name).is_null() {
            wget_free
                .expect("non-null function pointer")(param.name as *mut libc::c_void);
            param.name = 0 as *const libc::c_char;
        }
        if !(param.value).is_null() {
            wget_free
                .expect("non-null function pointer")(param.value as *mut libc::c_void);
            param.value = 0 as *const libc::c_char;
        }
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_content_encoding(
    mut s: *const libc::c_char,
    mut content_encoding: *mut libc::c_char,
) -> *const libc::c_char {
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if wget_strcasecmp_ascii(s, b"gzip\0" as *const u8 as *const libc::c_char) == 0
        || wget_strcasecmp_ascii(s, b"x-gzip\0" as *const u8 as *const libc::c_char) == 0
    {
        *content_encoding = wget_content_encoding_gzip as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(s, b"deflate\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *content_encoding = wget_content_encoding_deflate as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(s, b"bzip2\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *content_encoding = wget_content_encoding_bzip2 as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(s, b"xz\0" as *const u8 as *const libc::c_char) == 0
        || wget_strcasecmp_ascii(s, b"lzma\0" as *const u8 as *const libc::c_char) == 0
        || wget_strcasecmp_ascii(s, b"x-lzma\0" as *const u8 as *const libc::c_char) == 0
    {
        *content_encoding = wget_content_encoding_lzma as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(s, b"br\0" as *const u8 as *const libc::c_char) == 0
    {
        *content_encoding = wget_content_encoding_brotli as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(s, b"zstd\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *content_encoding = wget_content_encoding_zstd as libc::c_int as libc::c_char;
    } else if wget_strcasecmp_ascii(s, b"lzip\0" as *const u8 as *const libc::c_char)
        == 0
    {
        *content_encoding = wget_content_encoding_lzip as libc::c_int as libc::c_char;
    } else {
        *content_encoding = wget_content_encoding_identity as libc::c_int
            as libc::c_char;
    }
    while wget_http_istoken(*s) {
        s = s.offset(1);
        s;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_connection(
    mut s: *const libc::c_char,
    mut keep_alive: *mut bool,
) -> *const libc::c_char {
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    *keep_alive = 0 as libc::c_int != 0;
    e = s;
    while *e != 0 {
        e = strchrnul(s, ',' as i32);
        if e != s {
            while c_isblank(*s as libc::c_int) {
                s = s.offset(1);
                s;
            }
            if wget_strncasecmp_ascii(
                s,
                b"keep-alive\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as size_t,
            ) == 0
            {
                *keep_alive = 1 as libc::c_int != 0;
            }
        }
        s = e.offset(1 as libc::c_int as isize);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_etag(
    mut s: *const libc::c_char,
    mut etag: *mut *const libc::c_char,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    while c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    p = s;
    while *s as libc::c_int != 0 && !c_isblank(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    *etag = wget_strmemdup(
        p as *const libc::c_void,
        s.offset_from(p) as libc::c_long as size_t,
    );
    return s;
}
unsafe extern "C" fn leap_days(mut y1: libc::c_int, mut y2: libc::c_int) -> libc::c_int {
    y1 -= 1;
    y1;
    y2 -= 1;
    y2;
    return y2 / 4 as libc::c_int - y1 / 4 as libc::c_int
        - (y2 / 100 as libc::c_int - y1 / 100 as libc::c_int)
        + (y2 / 400 as libc::c_int - y1 / 400 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_full_date(
    mut s: *const libc::c_char,
) -> int64_t {
    static mut mnames: [*const libc::c_char; 12] = [
        b"Jan\0" as *const u8 as *const libc::c_char,
        b"Feb\0" as *const u8 as *const libc::c_char,
        b"Mar\0" as *const u8 as *const libc::c_char,
        b"Apr\0" as *const u8 as *const libc::c_char,
        b"May\0" as *const u8 as *const libc::c_char,
        b"Jun\0" as *const u8 as *const libc::c_char,
        b"Jul\0" as *const u8 as *const libc::c_char,
        b"Aug\0" as *const u8 as *const libc::c_char,
        b"Sep\0" as *const u8 as *const libc::c_char,
        b"Oct\0" as *const u8 as *const libc::c_char,
        b"Nov\0" as *const u8 as *const libc::c_char,
        b"Dec\0" as *const u8 as *const libc::c_char,
    ];
    static mut days_per_month: [libc::c_int; 12] = [
        31 as libc::c_int,
        28 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
    ];
    static mut sum_of_days: [libc::c_int; 12] = [
        0 as libc::c_int,
        31 as libc::c_int,
        59 as libc::c_int,
        90 as libc::c_int,
        120 as libc::c_int,
        151 as libc::c_int,
        181 as libc::c_int,
        212 as libc::c_int,
        243 as libc::c_int,
        273 as libc::c_int,
        304 as libc::c_int,
        334 as libc::c_int,
    ];
    let mut day: libc::c_int = 0;
    let mut mon: libc::c_int = 0 as libc::c_int;
    let mut year: libc::c_int = 0;
    let mut hour: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    let mut leap_month: libc::c_int = 0;
    let mut leap_year: libc::c_int = 0;
    let mut days: libc::c_int = 0;
    let mut mname: [libc::c_char; 4] = *::core::mem::transmute::<
        &[u8; 4],
        &mut [libc::c_char; 4],
    >(b"\0\0\0\0");
    if !(sscanf(
        s,
        b" %*[a-zA-Z], %2d %3s %4d %2d:%2d:%2d\0" as *const u8 as *const libc::c_char,
        &mut day as *mut libc::c_int,
        mname.as_mut_ptr(),
        &mut year as *mut libc::c_int,
        &mut hour as *mut libc::c_int,
        &mut min as *mut libc::c_int,
        &mut sec as *mut libc::c_int,
    ) == 6 as libc::c_int)
    {
        if !(sscanf(
            s,
            b" %*[a-zA-Z], %2d-%3s-%4d %2d:%2d:%2d\0" as *const u8
                as *const libc::c_char,
            &mut day as *mut libc::c_int,
            mname.as_mut_ptr(),
            &mut year as *mut libc::c_int,
            &mut hour as *mut libc::c_int,
            &mut min as *mut libc::c_int,
            &mut sec as *mut libc::c_int,
        ) == 6 as libc::c_int)
        {
            if !(sscanf(
                s,
                b" %*[a-zA-Z] %3s %2d %2d:%2d:%2d %4d\0" as *const u8
                    as *const libc::c_char,
                mname.as_mut_ptr(),
                &mut day as *mut libc::c_int,
                &mut hour as *mut libc::c_int,
                &mut min as *mut libc::c_int,
                &mut sec as *mut libc::c_int,
                &mut year as *mut libc::c_int,
            ) == 6 as libc::c_int)
            {
                if !(sscanf(
                    s,
                    b" %d %3s %4d %2d:%2d:%2d\0" as *const u8 as *const libc::c_char,
                    &mut day as *mut libc::c_int,
                    mname.as_mut_ptr(),
                    &mut year as *mut libc::c_int,
                    &mut hour as *mut libc::c_int,
                    &mut min as *mut libc::c_int,
                    &mut sec as *mut libc::c_int,
                ) == 6 as libc::c_int)
                {
                    if sscanf(
                        s,
                        b" %*s %3s %2d %4d %2d:%2d:%2d\0" as *const u8
                            as *const libc::c_char,
                        mname.as_mut_ptr(),
                        &mut day as *mut libc::c_int,
                        &mut year as *mut libc::c_int,
                        &mut hour as *mut libc::c_int,
                        &mut min as *mut libc::c_int,
                        &mut sec as *mut libc::c_int,
                    ) == 6 as libc::c_int
                    {} else {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to parse date '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            s,
                        );
                        return 0 as libc::c_int as int64_t;
                    }
                }
            }
        }
    }
    if *mname.as_mut_ptr() != 0 {
        let mut it: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (it as libc::c_ulong)
            < (::core::mem::size_of::<[*const libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                )
        {
            if wget_strcasecmp_ascii(mname.as_mut_ptr(), mnames[it as usize]) == 0 {
                mon = it.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
                break;
            } else {
                it = it.wrapping_add(1);
                it;
            }
        }
    }
    if year < 70 as libc::c_int && year >= 0 as libc::c_int {
        year += 2000 as libc::c_int;
    } else if year >= 70 as libc::c_int && year <= 99 as libc::c_int {
        year += 1900 as libc::c_int;
    }
    if year < 1970 as libc::c_int {
        year = 1970 as libc::c_int;
    }
    leap_year = (year % 4 as libc::c_int == 0 as libc::c_int
        && (year % 100 as libc::c_int != 0 as libc::c_int
            || year % 400 as libc::c_int == 0 as libc::c_int)) as libc::c_int;
    leap_month = (mon == 2 as libc::c_int && leap_year != 0) as libc::c_int;
    if mon < 1 as libc::c_int || mon > 12 as libc::c_int || day < 1 as libc::c_int
        || day > days_per_month[(mon - 1 as libc::c_int) as usize] + leap_month
        || hour < 0 as libc::c_int || hour > 23 as libc::c_int || min < 0 as libc::c_int
        || min > 60 as libc::c_int || sec < 0 as libc::c_int || sec > 60 as libc::c_int
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to parse date '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            s,
        );
        return 0 as libc::c_int as int64_t;
    }
    days = 365 as libc::c_int * (year - 1970 as libc::c_int)
        + leap_days(1970 as libc::c_int, year);
    days
        += sum_of_days[(mon - 1 as libc::c_int) as usize]
            + (mon > 2 as libc::c_int && leap_year != 0) as libc::c_int;
    days += day - 1 as libc::c_int;
    return ((days as int64_t * 24 as libc::c_int as int64_t + hour as int64_t)
        * 60 as libc::c_int as int64_t + min as int64_t) * 60 as libc::c_int as int64_t
        + sec as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_print_date(
    mut t: int64_t,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> *mut libc::c_char {
    static mut dnames: [*const libc::c_char; 7] = [
        b"Sun\0" as *const u8 as *const libc::c_char,
        b"Mon\0" as *const u8 as *const libc::c_char,
        b"Tue\0" as *const u8 as *const libc::c_char,
        b"Wed\0" as *const u8 as *const libc::c_char,
        b"Thu\0" as *const u8 as *const libc::c_char,
        b"Fri\0" as *const u8 as *const libc::c_char,
        b"Sat\0" as *const u8 as *const libc::c_char,
    ];
    static mut mnames: [*const libc::c_char; 12] = [
        b"Jan\0" as *const u8 as *const libc::c_char,
        b"Feb\0" as *const u8 as *const libc::c_char,
        b"Mar\0" as *const u8 as *const libc::c_char,
        b"Apr\0" as *const u8 as *const libc::c_char,
        b"May\0" as *const u8 as *const libc::c_char,
        b"Jun\0" as *const u8 as *const libc::c_char,
        b"Jul\0" as *const u8 as *const libc::c_char,
        b"Aug\0" as *const u8 as *const libc::c_char,
        b"Sep\0" as *const u8 as *const libc::c_char,
        b"Oct\0" as *const u8 as *const libc::c_char,
        b"Nov\0" as *const u8 as *const libc::c_char,
        b"Dec\0" as *const u8 as *const libc::c_char,
    ];
    let mut tm: tm = tm {
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
    let mut tt: time_t = 0;
    if bufsize == 0 {
        return buf;
    }
    tt = t;
    if !(gmtime_r(&mut tt, &mut tm)).is_null() {
        wget_snprintf(
            buf,
            bufsize,
            b"%s, %02d %s %d %02d:%02d:%02d GMT\0" as *const u8 as *const libc::c_char,
            dnames[tm.tm_wday as usize],
            tm.tm_mday,
            mnames[tm.tm_mon as usize],
            tm.tm_year + 1900 as libc::c_int,
            tm.tm_hour,
            tm.tm_min,
            tm.tm_sec,
        );
    } else {
        *buf = 0 as libc::c_int as libc::c_char;
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_setcookie(
    mut s: *const libc::c_char,
    mut cookie: *mut *mut wget_cookie,
) -> *const libc::c_char {
    return wget_cookie_parse_setcookie(s, cookie);
}
unsafe extern "C" fn cookie_free(mut cookie: *mut libc::c_void) {
    if !cookie.is_null() {
        wget_cookie_free(&mut cookie as *mut *mut libc::c_void as *mut *mut wget_cookie);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_header_line(
    mut resp: *mut wget_http_response,
    mut name: *const libc::c_char,
    mut namelen: size_t,
    mut value: *const libc::c_char,
    mut valuelen: size_t,
) -> libc::c_int {
    if name.is_null() || value.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    let mut valuebuf: [libc::c_char; 256] = [0; 256];
    let mut value0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = WGET_E_SUCCESS as libc::c_int;
    value0 = wget_strmemcpy_a(
        valuebuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        value as *const libc::c_void,
        valuelen,
    ) as *mut libc::c_char;
    if value0.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    match *name as libc::c_int | 0x20 as libc::c_int {
        58 => {
            if memcmp(
                name as *const libc::c_void,
                b":status\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                namelen,
            ) == 0 && valuelen == 3 as libc::c_int as size_t
            {
                (*resp)
                    .code = (((*value.offset(0 as libc::c_int as isize) as libc::c_int
                    - '0' as i32) * 10 as libc::c_int
                    + (*value.offset(1 as libc::c_int as isize) as libc::c_int
                        - '0' as i32)) * 10 as libc::c_int
                    + (*value.offset(2 as libc::c_int as isize) as libc::c_int
                        - '0' as i32)) as libc::c_short;
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        99 => {
            if wget_strncasecmp_ascii(
                name,
                b"content-encoding\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                wget_http_parse_content_encoding(value0, &mut (*resp).content_encoding);
            } else if wget_strncasecmp_ascii(
                name,
                b"content-type\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                if ((*resp).content_type).is_null()
                    && ((*resp).content_type_encoding).is_null()
                {
                    wget_http_parse_content_type(
                        value0,
                        &mut (*resp).content_type,
                        &mut (*resp).content_type_encoding,
                    );
                }
            } else if wget_strncasecmp_ascii(
                name,
                b"content-length\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                (*resp).content_length = atoll(value0) as size_t;
                (*resp).set_content_length_valid(1 as libc::c_int != 0);
            } else if wget_strncasecmp_ascii(
                name,
                b"content-disposition\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                if ((*resp).content_filename).is_null() {
                    wget_http_parse_content_disposition(
                        value0,
                        &mut (*resp).content_filename,
                    );
                }
            } else if wget_strncasecmp_ascii(
                name,
                b"connection\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                wget_http_parse_connection(value0, &mut (*resp).keep_alive);
            } else if wget_strncasecmp_ascii(
                name,
                b"Content-Security-Policy\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                (*resp).set_csp(1 as libc::c_int != 0);
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        100 => {
            if wget_strncasecmp_ascii(
                name,
                b"digest\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                let mut digest: wget_http_digest = wget_http_digest {
                    algorithm: 0 as *const libc::c_char,
                    encoded_digest: 0 as *const libc::c_char,
                };
                wget_http_parse_digest(value0, &mut digest);
                if ((*resp).digests).is_null() {
                    (*resp).digests = wget_vector_create(4 as libc::c_int, None);
                    wget_vector_set_destructor(
                        (*resp).digests,
                        ::core::mem::transmute::<
                            Option::<unsafe extern "C" fn(*mut wget_http_digest) -> ()>,
                            Option::<wget_vector_destructor>,
                        >(
                            Some(
                                wget_http_free_digest
                                    as unsafe extern "C" fn(*mut wget_http_digest) -> (),
                            ),
                        ),
                    );
                }
                wget_vector_add_memdup(
                    (*resp).digests,
                    &mut digest as *mut wget_http_digest as *const libc::c_void,
                    ::core::mem::size_of::<wget_http_digest>() as libc::c_ulong,
                );
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        101 => {
            if wget_strncasecmp_ascii(
                name,
                b"etag\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                if ((*resp).etag).is_null() {
                    wget_http_parse_etag(value0, &mut (*resp).etag);
                }
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        105 => {
            if wget_strncasecmp_ascii(
                name,
                b"icy-metaint\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                (*resp).icy_metaint = atoi(value0);
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        108 => {
            if wget_strncasecmp_ascii(
                name,
                b"last-modified\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                (*resp).last_modified = wget_http_parse_full_date(value0);
            } else if (*resp).code as libc::c_int / 100 as libc::c_int
                == 3 as libc::c_int
                && wget_strncasecmp_ascii(
                    name,
                    b"location\0" as *const u8 as *const libc::c_char,
                    namelen,
                ) == 0
            {
                if ((*resp).location).is_null() {
                    wget_http_parse_location(value0, &mut (*resp).location);
                }
            } else if (*resp).code as libc::c_int / 100 as libc::c_int
                == 3 as libc::c_int
                && wget_strncasecmp_ascii(
                    name,
                    b"link\0" as *const u8 as *const libc::c_char,
                    namelen,
                ) == 0
            {
                let mut link: wget_http_link = wget_http_link {
                    uri: 0 as *const libc::c_char,
                    type_0: 0 as *const libc::c_char,
                    pri: 0,
                    rel: link_rel_none,
                };
                wget_http_parse_link(value0, &mut link);
                if ((*resp).links).is_null() {
                    (*resp).links = wget_vector_create(8 as libc::c_int, None);
                    wget_vector_set_destructor(
                        (*resp).links,
                        ::core::mem::transmute::<
                            Option::<unsafe extern "C" fn(*mut wget_http_link) -> ()>,
                            Option::<wget_vector_destructor>,
                        >(
                            Some(
                                wget_http_free_link
                                    as unsafe extern "C" fn(*mut wget_http_link) -> (),
                            ),
                        ),
                    );
                }
                wget_vector_add_memdup(
                    (*resp).links,
                    &mut link as *mut wget_http_link as *const libc::c_void,
                    ::core::mem::size_of::<wget_http_link>() as libc::c_ulong,
                );
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        112 => {
            if wget_strncasecmp_ascii(
                name,
                b"public-key-pins\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                if ((*resp).hpkp).is_null() {
                    (*resp).hpkp = wget_hpkp_new();
                    wget_http_parse_public_key_pins(value0, (*resp).hpkp);
                    wget_debug_printf(
                        b"new host pubkey pinnings added to hpkp db\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else if wget_strncasecmp_ascii(
                name,
                b"proxy-authenticate\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                let mut challenge: *mut wget_http_challenge = wget_malloc(
                    ::core::mem::size_of::<wget_http_challenge>() as libc::c_ulong,
                ) as *mut wget_http_challenge;
                if challenge.is_null() {
                    ret = WGET_E_MEMORY as libc::c_int;
                } else {
                    wget_http_parse_challenge(value0, challenge);
                    if ((*resp).challenges).is_null() {
                        (*resp).challenges = wget_vector_create(2 as libc::c_int, None);
                        wget_vector_set_destructor(
                            (*resp).challenges,
                            ::core::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(*mut wget_http_challenge) -> (),
                                >,
                                Option::<wget_vector_destructor>,
                            >(
                                Some(
                                    wget_http_free_challenge
                                        as unsafe extern "C" fn(*mut wget_http_challenge) -> (),
                                ),
                            ),
                        );
                    }
                    wget_vector_add(
                        (*resp).challenges,
                        challenge as *const libc::c_void,
                    );
                }
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        115 => {
            if wget_strncasecmp_ascii(
                name,
                b"set-cookie\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                let mut cookie: *mut wget_cookie = 0 as *mut wget_cookie;
                wget_http_parse_setcookie(value0, &mut cookie);
                if !cookie.is_null() {
                    if ((*resp).cookies).is_null() {
                        (*resp).cookies = wget_vector_create(4 as libc::c_int, None);
                        wget_vector_set_destructor(
                            (*resp).cookies,
                            Some(
                                cookie_free as unsafe extern "C" fn(*mut libc::c_void) -> (),
                            ),
                        );
                    }
                    wget_vector_add((*resp).cookies, cookie as *const libc::c_void);
                }
            } else if wget_strncasecmp_ascii(
                name,
                b"strict-transport-security\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                (*resp).set_hsts(1 as libc::c_int != 0);
                wget_http_parse_strict_transport_security(
                    value0,
                    &mut (*resp).hsts_maxage,
                    &mut (*resp).hsts_include_subdomains,
                );
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        116 => {
            if wget_strncasecmp_ascii(
                name,
                b"transfer-encoding\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                wget_http_parse_transfer_encoding(
                    value0,
                    &mut (*resp).transfer_encoding,
                );
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        119 => {
            if wget_strncasecmp_ascii(
                name,
                b"www-authenticate\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                let mut challenge_0: *mut wget_http_challenge = wget_malloc(
                    ::core::mem::size_of::<wget_http_challenge>() as libc::c_ulong,
                ) as *mut wget_http_challenge;
                if challenge_0.is_null() {
                    ret = WGET_E_MEMORY as libc::c_int;
                } else {
                    wget_http_parse_challenge(value0, challenge_0);
                    if ((*resp).challenges).is_null() {
                        (*resp).challenges = wget_vector_create(2 as libc::c_int, None);
                        wget_vector_set_destructor(
                            (*resp).challenges,
                            ::core::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(*mut wget_http_challenge) -> (),
                                >,
                                Option::<wget_vector_destructor>,
                            >(
                                Some(
                                    wget_http_free_challenge
                                        as unsafe extern "C" fn(*mut wget_http_challenge) -> (),
                                ),
                            ),
                        );
                    }
                    wget_vector_add(
                        (*resp).challenges,
                        challenge_0 as *const libc::c_void,
                    );
                }
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        120 => {
            if wget_strncasecmp_ascii(
                name,
                b"x-archive-orig-last-modified\0" as *const u8 as *const libc::c_char,
                namelen,
            ) == 0
            {
                (*resp).last_modified = wget_http_parse_full_date(value0);
            } else {
                ret = WGET_E_UNKNOWN as libc::c_int;
            }
        }
        _ => {
            ret = WGET_E_UNKNOWN as libc::c_int;
        }
    }
    if value0 != valuebuf.as_mut_ptr() {
        if !value0.is_null() {
            wget_free.expect("non-null function pointer")(value0 as *mut libc::c_void);
            value0 = 0 as *mut libc::c_char;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_parse_response_header(
    mut buf: *mut libc::c_char,
) -> *mut wget_http_response {
    let mut eol: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut resp: *mut wget_http_response = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_http_response>() as libc::c_ulong,
    ) as *mut wget_http_response;
    if resp.is_null() {
        return 0 as *mut wget_http_response;
    }
    if sscanf(
        buf,
        b" HTTP/%3hd.%3hd %3hd %31[^\r\n] \0" as *const u8 as *const libc::c_char,
        &mut (*resp).major as *mut libc::c_short,
        &mut (*resp).minor as *mut libc::c_short,
        &mut (*resp).code as *mut libc::c_short,
        ((*resp).reason).as_mut_ptr(),
    ) >= 3 as libc::c_int
    {
        eol = strchr(buf.offset(10 as libc::c_int as isize), '\n' as i32);
        if !eol.is_null() {} else { return resp }
    } else if sscanf(
        buf,
        b" ICY %3hd %31[^\r\n] \0" as *const u8 as *const libc::c_char,
        &mut (*resp).code as *mut libc::c_short,
        ((*resp).reason).as_mut_ptr(),
    ) >= 1 as libc::c_int
    {
        eol = strchr(buf.offset(4 as libc::c_int as isize), '\n' as i32);
        if !eol.is_null() {} else { return resp }
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"HTTP response header not found\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        if !resp.is_null() {
            wget_free.expect("non-null function pointer")(resp as *mut libc::c_void);
            resp = 0 as *mut wget_http_response;
        }
        return 0 as *mut wget_http_response;
    }
    if (*resp).major as libc::c_int == 1 as libc::c_int
        && (*resp).minor as libc::c_int >= 1 as libc::c_int
        || (*resp).major as libc::c_int > 1 as libc::c_int
    {
        (*resp).keep_alive = 1 as libc::c_int != 0;
    }
    let mut line: *mut libc::c_char = eol.offset(1 as libc::c_int as isize);
    while !eol.is_null() && *line as libc::c_int != 0
        && *line as libc::c_int != '\r' as i32 && *line as libc::c_int != '\n' as i32
    {
        eol = strchr(line, '\n' as i32);
        while !eol.is_null()
            && c_isblank(*eol.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            let ref mut fresh2 = *eol.offset(-(1 as libc::c_int) as isize);
            *fresh2 = ' ' as i32 as libc::c_char;
            *eol = *fresh2;
            eol = strchr(eol, '\n' as i32);
        }
        if !eol.is_null() {
            if *eol.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\r' as i32 {
                *eol
                    .offset(
                        -(1 as libc::c_int) as isize,
                    ) = 0 as libc::c_int as libc::c_char;
            } else {
                *eol = 0 as libc::c_int as libc::c_char;
            }
        }
        let mut namelen: size_t = 0;
        let mut valuelen: size_t = 0;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        let mut value: *const libc::c_char = wget_parse_name_fixed(
            line,
            &mut name,
            &mut namelen,
        );
        if !eol.is_null() {
            valuelen = (eol.offset_from(value) as libc::c_long
                - (*eol.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == 0 as libc::c_int) as libc::c_int as libc::c_long) as size_t;
        } else {
            valuelen = strlen(value);
        }
        wget_http_parse_header_line(resp, name, namelen, value, valuelen);
        line = if !eol.is_null() {
            eol.offset(1 as libc::c_int as isize)
        } else {
            0 as *mut libc::c_char
        };
    }
    return resp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_param(mut param: *mut wget_http_header_param) {
    if !((*param).name).is_null() {
        wget_free
            .expect("non-null function pointer")((*param).name as *mut libc::c_void);
        (*param).name = 0 as *const libc::c_char;
    }
    if !((*param).value).is_null() {
        wget_free
            .expect("non-null function pointer")((*param).value as *mut libc::c_void);
        (*param).value = 0 as *const libc::c_char;
    }
    if !param.is_null() {
        wget_free.expect("non-null function pointer")(param as *mut libc::c_void);
        param = 0 as *mut wget_http_header_param;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_link(mut link: *mut wget_http_link) {
    if !((*link).uri).is_null() {
        wget_free.expect("non-null function pointer")((*link).uri as *mut libc::c_void);
        (*link).uri = 0 as *const libc::c_char;
    }
    if !((*link).type_0).is_null() {
        wget_free
            .expect("non-null function pointer")((*link).type_0 as *mut libc::c_void);
        (*link).type_0 = 0 as *const libc::c_char;
    }
    if !link.is_null() {
        wget_free.expect("non-null function pointer")(link as *mut libc::c_void);
        link = 0 as *mut wget_http_link;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_links(mut links: *mut *mut wget_vector) {
    wget_vector_free(links);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_digest(mut digest: *mut wget_http_digest) {
    if !((*digest).algorithm).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*digest).algorithm as *mut libc::c_void);
        (*digest).algorithm = 0 as *const libc::c_char;
    }
    if !((*digest).encoded_digest).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*digest).encoded_digest as *mut libc::c_void);
        (*digest).encoded_digest = 0 as *const libc::c_char;
    }
    if !digest.is_null() {
        wget_free.expect("non-null function pointer")(digest as *mut libc::c_void);
        digest = 0 as *mut wget_http_digest;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_digests(mut digests: *mut *mut wget_vector) {
    wget_vector_free(digests);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_challenge(
    mut challenge: *mut wget_http_challenge,
) {
    if !((*challenge).auth_scheme).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*challenge).auth_scheme as *mut libc::c_void);
        (*challenge).auth_scheme = 0 as *const libc::c_char;
    }
    wget_stringmap_free(&mut (*challenge).params);
    if !challenge.is_null() {
        wget_free.expect("non-null function pointer")(challenge as *mut libc::c_void);
        challenge = 0 as *mut wget_http_challenge;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_challenges(
    mut challenges: *mut *mut wget_vector,
) {
    wget_vector_free(challenges);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_cookies(mut cookies: *mut *mut wget_vector) {
    wget_vector_free(cookies);
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_hpkp_entries(mut hpkp: *mut *mut wget_hpkp) {
    if !hpkp.is_null() {
        wget_hpkp_free(*hpkp);
        *hpkp = 0 as *mut wget_hpkp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_response(
    mut resp: *mut *mut wget_http_response,
) {
    if !resp.is_null() && !(*resp).is_null() {
        wget_http_free_links(&mut (**resp).links);
        wget_http_free_digests(&mut (**resp).digests);
        wget_http_free_challenges(&mut (**resp).challenges);
        wget_http_free_cookies(&mut (**resp).cookies);
        wget_http_free_hpkp_entries(&mut (**resp).hpkp);
        if !((**resp).content_type).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**resp).content_type as *mut libc::c_void);
            (**resp).content_type = 0 as *const libc::c_char;
        }
        if !((**resp).content_type_encoding).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**resp).content_type_encoding as *mut libc::c_void);
            (**resp).content_type_encoding = 0 as *const libc::c_char;
        }
        if !((**resp).content_filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**resp).content_filename as *mut libc::c_void);
            (**resp).content_filename = 0 as *const libc::c_char;
        }
        if !((**resp).location).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**resp).location as *mut libc::c_void);
            (**resp).location = 0 as *const libc::c_char;
        }
        if !((**resp).etag).is_null() {
            wget_free
                .expect("non-null function pointer")((**resp).etag as *mut libc::c_void);
            (**resp).etag = 0 as *const libc::c_char;
        }
        wget_buffer_free(&mut (**resp).header);
        wget_buffer_free(&mut (**resp).body);
        if !(*resp).is_null() {
            wget_free.expect("non-null function pointer")(*resp as *mut libc::c_void);
            *resp = 0 as *mut wget_http_response;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_free_request(mut req: *mut *mut wget_http_request) {
    if !req.is_null() && !(*req).is_null() {
        wget_buffer_deinit(&mut (**req).esc_resource);
        wget_buffer_deinit(&mut (**req).esc_host);
        wget_vector_free(&mut (**req).headers);
        if !((**req).body).is_null() {
            wget_free
                .expect("non-null function pointer")((**req).body as *mut libc::c_void);
            (**req).body = 0 as *const libc::c_char;
        }
        if !(*req).is_null() {
            wget_free.expect("non-null function pointer")(*req as *mut libc::c_void);
            *req = 0 as *mut wget_http_request;
        }
    }
}
