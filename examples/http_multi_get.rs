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
    pub type wget_hpkp_st;
    pub type wget_ocsp_db_st;
    pub type wget_http_connection_st;
    static mut stderr: *mut FILE;
    fn wget_global_init(key: libc::c_int, _: ...);
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_ocsp_db_init(
        _: *mut wget_ocsp_db,
        _: *const libc::c_char,
    ) -> *mut wget_ocsp_db;
    fn wget_ocsp_db_free(_: *mut *mut wget_ocsp_db);
    fn wget_ocsp_db_save(_: *mut wget_ocsp_db) -> libc::c_int;
    fn wget_ocsp_db_load(_: *mut wget_ocsp_db) -> libc::c_int;
    fn wget_ssl_set_config_string(key: libc::c_int, value: *const libc::c_char);
    fn wget_ssl_set_config_int(key: libc::c_int, value: libc::c_int);
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
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_ocsp_db_init_fn = unsafe extern "C" fn(
    *mut wget_ocsp_db,
    *const libc::c_char,
) -> *mut wget_ocsp_db;
pub type wget_ocsp_db_free_fn = unsafe extern "C" fn(*mut *mut wget_ocsp_db) -> ();
pub type wget_ocsp_db_save_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> libc::c_int;
pub type wget_ocsp_db_load_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> libc::c_int;
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
    let mut current_block: u64;
    let mut urls: [*const libc::c_char; 3] = [
        b"https://www.google.de/1.html\0" as *const u8 as *const libc::c_char,
        b"https://www.google.de/2.html\0" as *const u8 as *const libc::c_char,
        b"https://www.google.de/4.html\0" as *const u8 as *const libc::c_char,
    ];
    let mut iris: [*mut wget_iri; 3] = [
        0 as *mut wget_iri,
        0 as *mut wget_iri,
        0 as *mut wget_iri,
    ];
    let mut reqs: [*mut wget_http_request; 3] = [
        0 as *mut wget_http_request,
        0 as *mut wget_http_request,
        0 as *mut wget_http_request,
    ];
    let mut conn: *mut wget_http_connection = 0 as *mut wget_http_connection;
    wget_global_init(
        1000 as libc::c_int,
        stderr,
        1003 as libc::c_int,
        stderr,
        1006 as libc::c_int,
        stderr,
        1009 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    let mut ocsp_db: *mut wget_ocsp_db = wget_ocsp_db_init(
        0 as *mut wget_ocsp_db,
        b".wget-ocsp\0" as *const u8 as *const libc::c_char,
    );
    wget_ocsp_db_load(ocsp_db);
    wget_ssl_set_config_string(17 as libc::c_int, ocsp_db as *const libc::c_char);
    wget_ssl_set_config_int(11 as libc::c_int, 1 as libc::c_int);
    let mut it: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (it as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        iris[it as usize] = wget_iri_parse(urls[it as usize], 0 as *const libc::c_char);
        reqs[it
            as usize] = wget_http_create_request(
            iris[it as usize],
            b"GET\0" as *const u8 as *const libc::c_char,
        );
        wget_http_add_header(
            reqs[it as usize],
            b"User-Agent\0" as *const u8 as *const libc::c_char,
            b"TheUserAgent/0.5\0" as *const u8 as *const libc::c_char,
        );
        wget_http_add_header(
            reqs[it as usize],
            b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
            b"gzip, deflate\0" as *const u8 as *const libc::c_char,
        );
        wget_http_add_header(
            reqs[it as usize],
            b"Accept\0" as *const u8 as *const libc::c_char,
            b"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\0"
                as *const u8 as *const libc::c_char,
        );
        wget_http_add_header(
            reqs[it as usize],
            b"Accept-Language\0" as *const u8 as *const libc::c_char,
            b"en-us,en;q=0.5\0" as *const u8 as *const libc::c_char,
        );
        wget_http_request_set_int(
            reqs[it as usize],
            2009 as libc::c_int,
            1 as libc::c_int,
        );
        it = it.wrapping_add(1);
        it;
    }
    wget_http_open(&mut conn, iris[0 as libc::c_int as usize]);
    if !conn.is_null() {
        let mut it_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        loop {
            if !((it_0 as libc::c_ulong)
                < (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ))
            {
                current_block = 15652330335145281839;
                break;
            }
            if wget_http_send_request(conn, reqs[it_0 as usize]) != 0 {
                current_block = 6580153298360140781;
                break;
            }
            it_0 = it_0.wrapping_add(1);
            it_0;
        }
        match current_block {
            6580153298360140781 => {}
            _ => {
                loop {
                    let mut resp: *mut wget_http_response = wget_http_get_response(conn);
                    if resp.is_null() {
                        break;
                    }
                    if !((*resp).header).is_null() {
                        wget_info_printf(
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            (*(*resp).header).data,
                        );
                    }
                    if !((*resp).body).is_null() {
                        wget_info_printf(
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            (*(*resp).body).data,
                        );
                    }
                    wget_http_free_response(&mut resp);
                }
            }
        }
    }
    wget_http_close(&mut conn);
    let mut it_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (it_1 as libc::c_ulong)
        < (::core::mem::size_of::<[*const libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        wget_http_free_request(&mut *reqs.as_mut_ptr().offset(it_1 as isize));
        wget_iri_free(&mut *iris.as_mut_ptr().offset(it_1 as isize));
        it_1 = it_1.wrapping_add(1);
        it_1;
    }
    wget_ocsp_db_save(ocsp_db);
    wget_ocsp_db_free(&mut ocsp_db);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
