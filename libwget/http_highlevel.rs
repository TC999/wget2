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
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_cookie_db_st;
    pub type wget_hpkp_st;
    pub type wget_tcp_st;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn __errno_location() -> *mut libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn wget_global_get_ptr(key: libc::c_int) -> *const libc::c_void;
    fn wget_global_get_int(key: libc::c_int) -> libc::c_int;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
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
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_iri_relative_to_abs(
        base: *const wget_iri,
        val: *const libc::c_char,
        len: size_t,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_cookie_normalize_cookies(iri: *const wget_iri, cookies: *const wget_vector);
    fn wget_cookie_store_cookies(
        cookie_db: *mut wget_cookie_db,
        cookies: *mut wget_vector,
    );
    fn wget_cookie_create_request_header(
        cookie_db: *mut wget_cookie_db,
        iri: *const wget_iri,
    ) -> *mut libc::c_char;
    fn wget_http_add_header(
        req: *mut wget_http_request,
        name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_http_add_header_param(
        req: *mut wget_http_request,
        param: *mut wget_http_header_param,
    ) -> libc::c_int;
    fn wget_http_add_credentials(
        req: *mut wget_http_request,
        challenge: *mut wget_http_challenge,
        username: *const libc::c_char,
        password: *const libc::c_char,
        proxied: libc::c_int,
    );
    fn wget_http_free_challenges(challenges: *mut *mut wget_vector);
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
    fn wget_http_request_set_header_cb(
        req: *mut wget_http_request,
        cb: Option::<wget_http_header_callback>,
        user_data: *mut libc::c_void,
    );
    fn wget_http_request_set_body_cb(
        req: *mut wget_http_request,
        cb: Option::<wget_http_body_callback>,
        user_data: *mut libc::c_void,
    );
    fn wget_http_request_set_int(
        req: *mut wget_http_request,
        key: libc::c_int,
        value: libc::c_int,
    );
    fn wget_http_request_set_body(
        req: *mut wget_http_request,
        mimetype: *const libc::c_char,
        body: *mut libc::c_char,
        length: size_t,
    );
    fn wget_http_send_request(
        conn: *mut wget_http_connection,
        req: *mut wget_http_request,
    ) -> libc::c_int;
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
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
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
pub type wget_hashmap = wget_hashmap_st;
pub type wget_stringmap = wget_hashmap;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "cookies_enabled", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "keep_header", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "free_uri", ty = "bool", bits = "2..=2")]
    pub cookies_enabled_keep_header_free_uri: [u8; 1],
}
unsafe extern "C" fn stream_callback(
    mut resp: *mut wget_http_response,
    mut user_data: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut stream: *mut FILE = user_data as *mut FILE;
    let mut nbytes: size_t = fwrite(
        data as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        length,
        stream,
    );
    if nbytes != length {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to fwrite %zu bytes of data (%d)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            length,
            *__errno_location(),
        );
        if feof(stream) != 0 {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn fd_callback(
    mut resp: *mut wget_http_response,
    mut user_data: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut fd: libc::c_int = *(user_data as *mut libc::c_int);
    let mut nbytes: ssize_t = write(fd, data as *const libc::c_void, length);
    if nbytes == -(1 as libc::c_int) as ssize_t || nbytes as size_t != length {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to write %zu bytes of data (%d)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            length,
            *__errno_location(),
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_http_get(
    mut first_key: libc::c_int,
    mut args: ...
) -> *mut wget_http_response {
    let mut current_block: u64;
    let mut headers: *mut wget_vector = 0 as *mut wget_vector;
    let mut uri: *mut wget_iri = 0 as *mut wget_iri;
    let mut conn: *mut wget_http_connection = 0 as *mut wget_http_connection;
    let mut connp: *mut *mut wget_http_connection = 0 as *mut *mut wget_http_connection;
    let mut req: *mut wget_http_request = 0 as *mut wget_http_request;
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut challenges: *mut wget_vector = 0 as *mut wget_vector;
    let mut cookie_db: *mut wget_cookie_db = 0 as *mut wget_cookie_db;
    let mut saveas_stream: *mut FILE = 0 as *mut FILE;
    let mut saveas_callback: Option::<wget_http_body_callback> = None;
    let mut saveas_fd: libc::c_int = -(1 as libc::c_int);
    let mut header_callback: Option::<wget_http_header_callback> = None;
    let mut args_0: ::core::ffi::VaListImpl;
    let mut url: *const libc::c_char = 0 as *const libc::c_char;
    let mut url_encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut scheme: *const libc::c_char = b"GET\0" as *const u8 as *const libc::c_char;
    let mut http_username: *const libc::c_char = 0 as *const libc::c_char;
    let mut http_password: *const libc::c_char = 0 as *const libc::c_char;
    let mut saveas_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: libc::c_int = 0;
    let mut it: libc::c_int = 0;
    let mut max_redirections: libc::c_int = 5 as libc::c_int;
    let mut redirection_level: libc::c_int = 0 as libc::c_int;
    let mut bodylen: size_t = 0 as libc::c_int as size_t;
    let mut body: *const libc::c_void = 0 as *const libc::c_void;
    let mut header_user_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut body_user_data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut debug_skip_body: bool = 0 as libc::c_int != 0;
    let mut bits: C2RustUnnamed_0 = {
        let mut init = C2RustUnnamed_0 {
            cookies_enabled_keep_header_free_uri: [0; 1],
        };
        init.set_cookies_enabled(
            wget_global_get_int(1011 as libc::c_int) != 0 as libc::c_int,
        );
        init.set_keep_header(false);
        init.set_free_uri(false);
        init
    };
    headers = wget_vector_create(8 as libc::c_int, None);
    if headers.is_null() {
        wget_debug_printf(b"no memory\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut wget_http_response;
    }
    args_0 = args.clone();
    key = first_key;
    loop {
        if !(key != 0) {
            current_block = 1847472278776910194;
            break;
        }
        match key {
            2000 => {
                url = args_0.arg::<*const libc::c_char>();
            }
            2002 => {
                uri = args_0.arg::<*mut wget_iri>();
            }
            2001 => {
                url_encoding = args_0.arg::<*const libc::c_char>();
            }
            2004 => {
                let mut param: wget_http_header_param = {
                    let mut init = wget_http_header_param {
                        name: args_0.arg::<*const libc::c_char>(),
                        value: args_0.arg::<*const libc::c_char>(),
                    };
                    init
                };
                if wget_vector_add_memdup(
                    headers,
                    &mut param as *mut wget_http_header_param as *const libc::c_void,
                    ::core::mem::size_of::<wget_http_header_param>() as libc::c_ulong,
                ) < 0 as libc::c_int
                {
                    current_block = 11456131856909903094;
                    break;
                }
            }
            2008 => {
                connp = args_0.arg::<*mut *mut wget_http_connection>();
                if !connp.is_null() {
                    conn = *connp;
                }
            }
            2009 => {
                bits.set_keep_header(args_0.arg::<libc::c_int>() != 0);
            }
            2010 => {
                max_redirections = args_0.arg::<libc::c_int>();
            }
            2018 => {
                saveas_name = args_0.arg::<*const libc::c_char>();
            }
            2011 => {
                saveas_stream = args_0.arg::<*mut FILE>();
            }
            2014 => {
                saveas_callback = ::core::mem::transmute(
                    args_0
                        .arg::<
                            *mut unsafe extern "C" fn(
                                *mut wget_http_response,
                                *mut libc::c_void,
                                *const libc::c_char,
                                size_t,
                            ) -> libc::c_int,
                        >(),
                );
                body_user_data = args_0.arg::<*mut libc::c_void>();
            }
            2013 => {
                saveas_fd = args_0.arg::<libc::c_int>();
            }
            2015 => {
                header_callback = ::core::mem::transmute(
                    args_0
                        .arg::<
                            *mut unsafe extern "C" fn(
                                *mut wget_http_response,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                        >(),
                );
                header_user_data = args_0.arg::<*mut libc::c_void>();
            }
            2016 => {
                scheme = args_0.arg::<*const libc::c_char>();
            }
            2017 => {
                body = args_0.arg::<*const libc::c_void>();
                bodylen = args_0.arg::<size_t>();
            }
            2021 => {
                debug_skip_body = 1 as libc::c_int != 0;
            }
            _ => {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unknown option %d\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    key,
                );
                current_block = 11456131856909903094;
                break;
            }
        }
        key = args_0.arg::<libc::c_int>();
    }
    match current_block {
        1847472278776910194 => {
            if !url.is_null() && uri.is_null() {
                uri = wget_iri_parse(url, url_encoding);
                if uri.is_null() {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Error parsing URL\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    current_block = 11456131856909903094;
                } else {
                    bits.set_free_uri(1 as libc::c_int != 0);
                    current_block = 1924505913685386279;
                }
            } else {
                current_block = 1924505913685386279;
            }
            match current_block {
                11456131856909903094 => {}
                _ => {
                    if uri.is_null() {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Missing URL/URI\n\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    } else {
                        if bits.cookies_enabled() {
                            cookie_db = wget_global_get_ptr(1013 as libc::c_int)
                                as *mut wget_cookie_db;
                        }
                        while !uri.is_null() && redirection_level <= max_redirections {
                            req = wget_http_create_request(uri, scheme);
                            if req.is_null() {
                                break;
                            }
                            it = 0 as libc::c_int;
                            while it < wget_vector_size(headers) {
                                wget_http_add_header_param(
                                    req,
                                    wget_vector_get(headers, it) as *mut wget_http_header_param,
                                );
                                it += 1;
                                it;
                            }
                            if !challenges.is_null() {
                                wget_http_add_credentials(
                                    req,
                                    wget_vector_get(challenges, 0 as libc::c_int)
                                        as *mut wget_http_challenge,
                                    http_username,
                                    http_password,
                                    0 as libc::c_int,
                                );
                                wget_http_free_challenges(&mut challenges);
                            }
                            if !cookie_db.is_null() {
                                let mut cookie_string: *const libc::c_char = 0
                                    as *const libc::c_char;
                                cookie_string = wget_cookie_create_request_header(
                                    cookie_db,
                                    uri,
                                );
                                if !cookie_string.is_null() {
                                    wget_http_add_header(
                                        req,
                                        b"Cookie\0" as *const u8 as *const libc::c_char,
                                        cookie_string,
                                    );
                                    if !cookie_string.is_null() {
                                        wget_free
                                            .expect(
                                                "non-null function pointer",
                                            )(cookie_string as *mut libc::c_void);
                                        cookie_string = 0 as *const libc::c_char;
                                    }
                                }
                            }
                            if !connp.is_null() {
                                wget_http_add_header(
                                    req,
                                    b"Connection\0" as *const u8 as *const libc::c_char,
                                    b"keepalive\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if !conn.is_null()
                                && wget_strcmp((*conn).esc_host, (*uri).host) == 0
                                && (*conn).scheme as libc::c_uint
                                    == (*uri).scheme as libc::c_uint
                                && (*conn).port as libc::c_int == (*uri).port as libc::c_int
                            {
                                wget_debug_printf(
                                    b"reuse connection %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    (*conn).esc_host,
                                );
                            } else {
                                if !conn.is_null() {
                                    wget_debug_printf(
                                        b"close connection %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*conn).esc_host,
                                    );
                                    wget_http_close(&mut conn);
                                }
                                if wget_http_open(&mut conn, uri)
                                    == WGET_E_SUCCESS as libc::c_int
                                {
                                    wget_debug_printf(
                                        b"opened connection %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        (*conn).esc_host,
                                    );
                                }
                            }
                            if !conn.is_null() {
                                let mut rc: libc::c_int = 0;
                                if !body.is_null() && bodylen != 0 {
                                    wget_http_request_set_body(
                                        req,
                                        0 as *const libc::c_char,
                                        wget_memdup(body, bodylen) as *mut libc::c_char,
                                        bodylen,
                                    );
                                }
                                (*req).set_debug_skip_body(debug_skip_body);
                                rc = wget_http_send_request(conn, req);
                                if rc == 0 as libc::c_int {
                                    wget_http_request_set_header_cb(
                                        req,
                                        header_callback,
                                        header_user_data,
                                    );
                                    wget_http_request_set_int(
                                        req,
                                        2009 as libc::c_int,
                                        1 as libc::c_int,
                                    );
                                    if !saveas_name.is_null() {
                                        let mut fp: *mut FILE = 0 as *mut FILE;
                                        fp = rpl_fopen(
                                            saveas_name,
                                            b"wb\0" as *const u8 as *const libc::c_char,
                                        );
                                        if !fp.is_null() {
                                            wget_http_request_set_body_cb(
                                                req,
                                                Some(
                                                    stream_callback
                                                        as unsafe extern "C" fn(
                                                            *mut wget_http_response,
                                                            *mut libc::c_void,
                                                            *const libc::c_char,
                                                            size_t,
                                                        ) -> libc::c_int,
                                                ),
                                                fp as *mut libc::c_void,
                                            );
                                            resp = wget_http_get_response(conn);
                                            rpl_fclose(fp);
                                        } else {
                                            wget_debug_printf(
                                                b"Failed to open '%s' for writing\n\0" as *const u8
                                                    as *const libc::c_char,
                                                saveas_name,
                                            );
                                        }
                                    } else if !saveas_stream.is_null() {
                                        wget_http_request_set_body_cb(
                                            req,
                                            Some(
                                                stream_callback
                                                    as unsafe extern "C" fn(
                                                        *mut wget_http_response,
                                                        *mut libc::c_void,
                                                        *const libc::c_char,
                                                        size_t,
                                                    ) -> libc::c_int,
                                            ),
                                            saveas_stream as *mut libc::c_void,
                                        );
                                        resp = wget_http_get_response(conn);
                                    } else if saveas_callback.is_some() {
                                        wget_http_request_set_body_cb(
                                            req,
                                            saveas_callback,
                                            body_user_data,
                                        );
                                        resp = wget_http_get_response(conn);
                                    } else if saveas_fd != -(1 as libc::c_int) {
                                        wget_http_request_set_body_cb(
                                            req,
                                            Some(
                                                fd_callback
                                                    as unsafe extern "C" fn(
                                                        *mut wget_http_response,
                                                        *mut libc::c_void,
                                                        *const libc::c_char,
                                                        size_t,
                                                    ) -> libc::c_int,
                                            ),
                                            &mut saveas_fd as *mut libc::c_int as *mut libc::c_void,
                                        );
                                        resp = wget_http_get_response(conn);
                                    } else {
                                        resp = wget_http_get_response(conn);
                                    }
                                }
                            }
                            wget_http_free_request(&mut req);
                            if resp.is_null() {
                                break;
                            }
                            if !(*resp).keep_alive {
                                wget_http_close(&mut conn);
                            }
                            if !cookie_db.is_null() {
                                wget_cookie_normalize_cookies(uri, (*resp).cookies);
                                wget_cookie_store_cookies(cookie_db, (*resp).cookies);
                            }
                            if (*resp).code as libc::c_int == 401 as libc::c_int
                                && challenges.is_null()
                            {
                                challenges = (*resp).challenges;
                                if challenges.is_null() {
                                    break;
                                }
                                (*resp).challenges = 0 as *mut wget_vector;
                                wget_http_free_response(&mut resp);
                                if !(redirection_level == 0 as libc::c_int
                                    && max_redirections != 0)
                                {
                                    break;
                                }
                                redirection_level = max_redirections;
                            } else {
                                if (*resp).code as libc::c_int / 100 as libc::c_int
                                    == 2 as libc::c_int
                                    || (*resp).code as libc::c_int / 100 as libc::c_int
                                        >= 4 as libc::c_int
                                    || (*resp).code as libc::c_int == 304 as libc::c_int
                                {
                                    break;
                                }
                                if ((*resp).location).is_null() {
                                    break;
                                }
                                let mut uri_sbuf: [libc::c_char; 1024] = [0; 1024];
                                let mut uri_buf: wget_buffer = wget_buffer {
                                    data: 0 as *mut libc::c_char,
                                    length: 0,
                                    size: 0,
                                    release_data_release_buf_error: [0; 1],
                                    c2rust_padding: [0; 7],
                                };
                                wget_buffer_init(
                                    &mut uri_buf,
                                    uri_sbuf.as_mut_ptr(),
                                    ::core::mem::size_of::<[libc::c_char; 1024]>()
                                        as libc::c_ulong,
                                );
                                wget_iri_relative_to_abs(
                                    uri,
                                    (*resp).location,
                                    -(1 as libc::c_int) as size_t,
                                    &mut uri_buf,
                                );
                                if bits.free_uri() {
                                    wget_iri_free(&mut uri);
                                }
                                uri = wget_iri_parse(
                                    uri_buf.data,
                                    0 as *const libc::c_char,
                                );
                                bits.set_free_uri(1 as libc::c_int != 0);
                                wget_buffer_deinit(&mut uri_buf);
                                redirection_level += 1;
                                redirection_level;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    if !connp.is_null() {
        *connp = conn;
    } else {
        wget_http_close(&mut conn);
    }
    wget_http_free_challenges(&mut challenges);
    wget_vector_free(&mut headers);
    if bits.free_uri() {
        wget_iri_free(&mut uri);
    }
    return resp;
}
