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
    pub type wget_http_connection_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn wget_global_init(key: libc::c_int, _: ...);
    fn wget_global_deinit();
    fn wget_buffer_alloc(size: size_t) -> *mut wget_buffer;
    fn wget_buffer_free(buf: *mut *mut wget_buffer);
    fn wget_buffer_strcpy(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
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
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_iri_escape_query(
        src: *const libc::c_char,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_close(conn: *mut *mut wget_http_connection);
    fn wget_http_get(first_key: libc::c_int, _: ...) -> *mut wget_http_response;
}
pub type size_t = libc::c_ulong;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    let mut conn: *mut wget_http_connection = 0 as *mut wget_http_connection;
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    wget_global_init(
        1003 as libc::c_int,
        stderr,
        1006 as libc::c_int,
        stdout,
        0 as libc::c_int,
    );
    let mut text: *const libc::c_char = b"alice->bob: authentication request\nbob-->alice: response\0"
        as *const u8 as *const libc::c_char;
    let mut style: *const libc::c_char = b"qsd\0" as *const u8 as *const libc::c_char;
    let mut url: *mut wget_buffer = wget_buffer_alloc(128 as libc::c_int as size_t);
    let mut body: *mut wget_buffer = wget_buffer_alloc(128 as libc::c_int as size_t);
    wget_buffer_strcpy(body, b"message=\0" as *const u8 as *const libc::c_char);
    wget_iri_escape_query(text, body);
    wget_buffer_printf_append(
        body,
        b"&style=%s&apiVersion=1\0" as *const u8 as *const libc::c_char,
        style,
    );
    resp = wget_http_get(
        2000 as libc::c_int,
        b"https://www.websequencediagrams.com\0" as *const u8 as *const libc::c_char,
        2016 as libc::c_int,
        b"POST\0" as *const u8 as *const libc::c_char,
        2004 as libc::c_int,
        b"Content-Type\0" as *const u8 as *const libc::c_char,
        b"application/x-www-form-urlencoded\0" as *const u8 as *const libc::c_char,
        2017 as libc::c_int,
        (*body).data,
        (*body).length,
        2008 as libc::c_int,
        &mut conn as *mut *mut wget_http_connection,
        0 as libc::c_int,
    );
    if !resp.is_null() {
        let mut p: *const libc::c_char = 0 as *const libc::c_char;
        let mut e: *const libc::c_char = 0 as *const libc::c_char;
        p = strstr(
            (*(*resp).body).data,
            b"\"img\": \"\0" as *const u8 as *const libc::c_char,
        );
        if !p.is_null() {
            e = strchr(p.offset(8 as libc::c_int as isize), '"' as i32);
            if !e.is_null() {
                p = p.offset(8 as libc::c_int as isize);
                wget_buffer_printf(
                    url,
                    b"https://www.websequencediagrams.com/%.*s\0" as *const u8
                        as *const libc::c_char,
                    e.offset_from(p) as libc::c_long as libc::c_int,
                    p,
                );
                wget_http_free_response(&mut resp);
                resp = wget_http_get(
                    2000 as libc::c_int,
                    (*url).data,
                    2018 as libc::c_int,
                    b"out.png\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                if !resp.is_null() {
                    wget_info_printf(
                        b"Saved out.png\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    wget_http_free_response(&mut resp);
    wget_http_close(&mut conn);
    wget_buffer_free(&mut body);
    wget_buffer_free(&mut url);
    wget_global_deinit();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *const *const libc::c_char,
            ) as i32,
        )
    }
}
