#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn wget_buffer_alloc(size: size_t) -> *mut wget_buffer;
    fn wget_buffer_free(buf: *mut *mut wget_buffer);
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
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
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
unsafe fn main_0() -> libc::c_int {
    let mut base_url: *const libc::c_char = b"https://example.com/subdir1/\0"
        as *const u8 as *const libc::c_char;
    let mut relative_url: *const libc::c_char = 0 as *const libc::c_char;
    let mut absolute_url: *const libc::c_char = 0 as *const libc::c_char;
    let mut base: *mut wget_iri = wget_iri_parse(base_url, 0 as *const libc::c_char);
    let mut buf: *mut wget_buffer = wget_buffer_alloc(128 as libc::c_int as size_t);
    relative_url = b"x.png\0" as *const u8 as *const libc::c_char;
    absolute_url = wget_iri_relative_to_abs(
        base,
        relative_url,
        -(1 as libc::c_int) as size_t,
        buf,
    );
    printf(
        b"%s + %s -> %s\n\0" as *const u8 as *const libc::c_char,
        base_url,
        relative_url,
        absolute_url,
    );
    relative_url = b"../x.png\0" as *const u8 as *const libc::c_char;
    absolute_url = wget_iri_relative_to_abs(
        base,
        relative_url,
        -(1 as libc::c_int) as size_t,
        buf,
    );
    printf(
        b"%s + %s -> %s\n\0" as *const u8 as *const libc::c_char,
        base_url,
        relative_url,
        absolute_url,
    );
    relative_url = b"../../x.png\0" as *const u8 as *const libc::c_char;
    absolute_url = wget_iri_relative_to_abs(
        base,
        relative_url,
        -(1 as libc::c_int) as size_t,
        buf,
    );
    printf(
        b"%s + %s -> %s\n\0" as *const u8 as *const libc::c_char,
        base_url,
        relative_url,
        absolute_url,
    );
    relative_url = b"subdir2/x.png\0" as *const u8 as *const libc::c_char;
    absolute_url = wget_iri_relative_to_abs(
        base,
        relative_url,
        -(1 as libc::c_int) as size_t,
        buf,
    );
    printf(
        b"%s + %s -> %s\n\0" as *const u8 as *const libc::c_char,
        base_url,
        relative_url,
        absolute_url,
    );
    relative_url = b"/x.png\0" as *const u8 as *const libc::c_char;
    absolute_url = wget_iri_relative_to_abs(
        base,
        relative_url,
        -(1 as libc::c_int) as size_t,
        buf,
    );
    printf(
        b"%s + %s -> %s\n\0" as *const u8 as *const libc::c_char,
        base_url,
        relative_url,
        absolute_url,
    );
    wget_buffer_free(&mut buf);
    wget_iri_free(&mut base);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
