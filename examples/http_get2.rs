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
    pub type wget_hpkp_st;
    pub type wget_http_connection_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_logger_set_stream(logger: *mut wget_logger, fp: *mut FILE);
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_cookie_normalize_cookies(iri: *const wget_iri, cookies: *const wget_vector);
    fn wget_cookie_store_cookies(
        cookie_db: *mut wget_cookie_db,
        cookies: *mut wget_vector,
    );
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
    fn wget_cookie_create_request_header(
        cookie_db: *mut wget_cookie_db,
        iri: *const wget_iri,
    ) -> *mut libc::c_char;
    fn wget_net_init() -> libc::c_int;
    fn wget_net_deinit() -> libc::c_int;
    fn wget_http_add_header(
        req: *mut wget_http_request,
        name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_http_free_request(req: *mut *mut wget_http_request);
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_get_response(
        conn: *mut wget_http_connection,
    ) -> *mut wget_http_response;
    fn wget_http_open(
        _conn: *mut *mut wget_http_connection,
        iri: *const wget_iri,
    ) -> libc::c_int;
    fn wget_http_create_request(
        iri: *const wget_iri,
        method: *const libc::c_char,
    ) -> *mut wget_http_request;
    fn wget_http_close(conn: *mut *mut wget_http_connection);
    fn wget_http_request_set_int(
        req: *mut wget_http_request,
        key: libc::c_int,
        value: libc::c_int,
    );
    fn wget_http_send_request(
        conn: *mut wget_http_connection,
        req: *mut wget_http_request,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type wget_logger = wget_logger_st;
pub type wget_vector = wget_vector_st;
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
pub type wget_hpkp = wget_hpkp_st;
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
unsafe fn main_0() -> libc::c_int {
    let mut uri: *mut wget_iri = 0 as *mut wget_iri;
    let mut conn: *mut wget_http_connection = 0 as *mut wget_http_connection;
    let mut req: *mut wget_http_request = 0 as *mut wget_http_request;
    let mut cookies: *mut wget_cookie_db = 0 as *mut wget_cookie_db;
    wget_logger_set_stream(wget_get_logger(3 as libc::c_int), stderr);
    wget_logger_set_stream(wget_get_logger(2 as libc::c_int), stderr);
    wget_logger_set_stream(wget_get_logger(1 as libc::c_int), stdout);
    wget_net_init();
    uri = wget_iri_parse(
        b"http://www.example.org\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    );
    req = wget_http_create_request(uri, b"GET\0" as *const u8 as *const libc::c_char);
    wget_http_add_header(
        req,
        b"User-Agent\0" as *const u8 as *const libc::c_char,
        b"TheUserAgent/0.5\0" as *const u8 as *const libc::c_char,
    );
    wget_http_add_header(
        req,
        b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
        b"gzip, deflate\0" as *const u8 as *const libc::c_char,
    );
    wget_http_add_header(
        req,
        b"Accept\0" as *const u8 as *const libc::c_char,
        b"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\0" as *const u8
            as *const libc::c_char,
    );
    wget_http_add_header(
        req,
        b"Accept-Language\0" as *const u8 as *const libc::c_char,
        b"en-us,en;q=0.5\0" as *const u8 as *const libc::c_char,
    );
    wget_http_request_set_int(req, 2009 as libc::c_int, 1 as libc::c_int);
    let mut cookie_string: *const libc::c_char = 0 as *const libc::c_char;
    cookies = wget_cookie_db_init(0 as *mut wget_cookie_db);
    wget_cookie_set_keep_session_cookies(cookies, 1 as libc::c_int != 0);
    wget_cookie_db_load_psl(
        cookies,
        b"public_suffixes.txt\0" as *const u8 as *const libc::c_char,
    );
    wget_cookie_db_load(cookies, b"cookies.txt\0" as *const u8 as *const libc::c_char);
    cookie_string = wget_cookie_create_request_header(cookies, uri);
    if !cookie_string.is_null() {
        wget_http_add_header(
            req,
            b"Cookie\0" as *const u8 as *const libc::c_char,
            cookie_string,
        );
        if !cookie_string.is_null() {
            wget_free
                .expect("non-null function pointer")(cookie_string as *mut libc::c_void);
            cookie_string = 0 as *const libc::c_char;
        }
    }
    wget_http_open(&mut conn, uri);
    if !conn.is_null() {
        let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
        if wget_http_send_request(conn, req) == 0 as libc::c_int {
            resp = wget_http_get_response(conn);
            if !resp.is_null() {
                if !(*resp).keep_alive {
                    wget_http_close(&mut conn);
                }
                wget_cookie_normalize_cookies(uri, (*resp).cookies);
                wget_cookie_store_cookies(cookies, (*resp).cookies);
                wget_cookie_db_save(
                    cookies,
                    b"cookies.txt\0" as *const u8 as *const libc::c_char,
                );
                wget_info_printf(
                    b"%s%s\n\0" as *const u8 as *const libc::c_char,
                    (*(*resp).header).data,
                    (*(*resp).body).data,
                );
                wget_http_free_response(&mut resp);
            }
        }
    }
    wget_cookie_db_free(&mut cookies);
    wget_http_close(&mut conn);
    wget_http_free_request(&mut req);
    wget_iri_free(&mut uri);
    wget_net_deinit();
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
