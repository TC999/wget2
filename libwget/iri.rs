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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn wget_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strtolower(s: *mut libc::c_char) -> *mut libc::c_char;
    fn wget_str_needs_encoding(s: *const libc::c_char) -> bool;
    fn wget_str_to_utf8(
        src: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn wget_utf8_to_str(
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
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_alloc(size: size_t) -> *mut wget_buffer;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_free(buf: *mut *mut wget_buffer);
    fn wget_buffer_reset(buf: *mut wget_buffer);
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
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_ip_is_family(host: *const libc::c_char, family: libc::c_int) -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iri_scheme {
    pub port: uint16_t,
    pub name: [libc::c_char; 6],
}
#[inline]
unsafe extern "C" fn c_isalnum(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72
        | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88
        | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isalpha(mut c: libc::c_int) -> bool {
    match c {
        97 | 98 | 99 | 100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110
        | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 65 | 66
        | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn c_isxdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 65 | 66 | 67 | 68 | 69 | 70 => return 1 as libc::c_int != 0,
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
static mut default_page: *const libc::c_char = b"index.html\0" as *const u8
    as *const libc::c_char;
static mut default_page_length: size_t = 10 as libc::c_int as size_t;
static mut schemes: [iri_scheme; 2] = unsafe {
    [
        {
            let mut init = iri_scheme {
                port: 80 as libc::c_int as uint16_t,
                name: *::core::mem::transmute::<
                    &[u8; 6],
                    &[libc::c_char; 6],
                >(b"http\0\0"),
            };
            init
        },
        {
            let mut init = iri_scheme {
                port: 443 as libc::c_int as uint16_t,
                name: *::core::mem::transmute::<&[u8; 6], &[libc::c_char; 6]>(b"https\0"),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn wget_iri_scheme_get_name(
    mut scheme: wget_iri_scheme,
) -> *const libc::c_char {
    if (scheme as libc::c_uint as libc::c_ulong)
        < (::core::mem::size_of::<[iri_scheme; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<iri_scheme>() as libc::c_ulong)
    {
        return (schemes[scheme as usize].name).as_ptr();
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_supported(mut iri: *const wget_iri) -> bool {
    return ((*iri).scheme as libc::c_uint as libc::c_ulong)
        < (::core::mem::size_of::<[iri_scheme; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<iri_scheme>() as libc::c_ulong);
}
static mut iri_ctype: [libc::c_uchar; 256] = [
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
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    0,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
    0,
    0,
    0,
    ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar,
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
#[no_mangle]
pub unsafe extern "C" fn wget_iri_isgendelim(mut c: libc::c_char) -> bool {
    return iri_ctype[c as libc::c_uchar as usize] as libc::c_int
        & (1 as libc::c_int) << 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_issubdelim(mut c: libc::c_char) -> bool {
    return iri_ctype[c as libc::c_uchar as usize] as libc::c_int
        & (1 as libc::c_int) << 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_isreserved(mut c: libc::c_char) -> bool {
    return wget_iri_isgendelim(c) as libc::c_int != 0
        || wget_iri_issubdelim(c) as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_isunreserved(mut c: libc::c_char) -> bool {
    return iri_ctype[c as libc::c_uchar as usize] as libc::c_int
        & (1 as libc::c_int) << 2 as libc::c_int != 0;
}
unsafe extern "C" fn unhex(mut c: libc::c_uchar) -> libc::c_uchar {
    return (if c as libc::c_int <= '9' as i32 {
        c as libc::c_int - '0' as i32
    } else if c as libc::c_int <= 'F' as i32 {
        c as libc::c_int - 'A' as i32 + 10 as libc::c_int
    } else {
        c as libc::c_int - 'a' as i32 + 10 as libc::c_int
    }) as libc::c_uchar;
}
unsafe extern "C" fn iri_unescape_inline(
    mut src: *mut libc::c_char,
    mut ctype: libc::c_int,
) -> *mut libc::c_char {
    let mut ret: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_uchar = src as *mut libc::c_uchar;
    let mut d: *mut libc::c_uchar = s;
    while *s != 0 {
        if *s as libc::c_int == '%' as i32 {
            if c_isxdigit(*s.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
                && c_isxdigit(*s.offset(2 as libc::c_int as isize) as libc::c_int)
                    as libc::c_int != 0
            {
                let mut c: libc::c_uchar = (((unhex(*s.offset(1 as libc::c_int as isize))
                    as libc::c_int) << 4 as libc::c_int) as libc::c_uchar as libc::c_int
                    | unhex(*s.offset(2 as libc::c_int as isize)) as libc::c_int)
                    as libc::c_uchar;
                if ctype == 0
                    || iri_ctype[c as usize] as libc::c_int & ctype == 0
                        && c as libc::c_int != '%' as i32
                {
                    let fresh0 = d;
                    d = d.offset(1);
                    *fresh0 = c;
                    s = s.offset(3 as libc::c_int as isize);
                    ret = src;
                    continue;
                }
            }
        } else if *s as libc::c_int == '#' as i32 {
            let mut value: uint32_t = 0 as libc::c_int as uint32_t;
            if *s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32 {
                let mut p: *mut libc::c_uchar = s.offset(2 as libc::c_int as isize);
                while c_isxdigit(*p as libc::c_int) {
                    value = (value & 0xfffffff as libc::c_int as uint32_t)
                        << 4 as libc::c_int | unhex(*p) as uint32_t;
                    p = p.offset(1);
                    p;
                }
                if *p as libc::c_int == ';' as i32 {
                    if value > 0 as libc::c_int as uint32_t
                        && value < 128 as libc::c_int as uint32_t
                    {
                        let fresh1 = d;
                        d = d.offset(1);
                        *fresh1 = value as libc::c_uchar;
                        s = p.offset(1 as libc::c_int as isize);
                        continue;
                    }
                }
            } else {
                let mut p_0: *mut libc::c_uchar = s.offset(1 as libc::c_int as isize);
                while c_isdigit(*p_0 as libc::c_int) as libc::c_int != 0
                    && value <= 0x10ffff as libc::c_int as uint32_t
                {
                    value = (value * 10 as libc::c_int as uint32_t)
                        .wrapping_add((*p_0 as libc::c_int - '0' as i32) as uint32_t);
                    p_0 = p_0.offset(1);
                    p_0;
                }
                if *p_0 as libc::c_int == ';' as i32 {
                    if value > 0 as libc::c_int as uint32_t
                        && value < 128 as libc::c_int as uint32_t
                    {
                        let fresh2 = d;
                        d = d.offset(1);
                        *fresh2 = value as libc::c_uchar;
                        s = p_0.offset(1 as libc::c_int as isize);
                        continue;
                    }
                }
            }
        } else if *s as libc::c_int == '\r' as i32 || *s as libc::c_int == '\n' as i32 {
            s = s.offset(1);
            s;
            continue;
        }
        let fresh3 = s;
        s = s.offset(1);
        let fresh4 = d;
        d = d.offset(1);
        *fresh4 = *fresh3;
    }
    *d = 0 as libc::c_int as libc::c_uchar;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_unescape_inline(
    mut src: *mut libc::c_char,
) -> *mut libc::c_char {
    return iri_unescape_inline(src, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_unescape_url_inline(
    mut src: *mut libc::c_char,
) -> *mut libc::c_char {
    return iri_unescape_inline(src, (1 as libc::c_int) << 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_free_content(mut iri: *mut wget_iri) {
    if !iri.is_null() {
        if !((*iri).userinfo).is_null() {
            if !((*iri).safe_uri).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*iri).safe_uri as *mut libc::c_void);
                (*iri).safe_uri = 0 as *const libc::c_char;
            }
        } else {
            (*iri).safe_uri = 0 as *const libc::c_char;
        }
        if (*iri).host_allocated() {
            if !((*iri).host).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*iri).host as *mut libc::c_void);
                (*iri).host = 0 as *const libc::c_char;
            }
        }
        if (*iri).path_allocated() {
            if !((*iri).path).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*iri).path as *mut libc::c_void);
                (*iri).path = 0 as *const libc::c_char;
            }
        }
        if (*iri).query_allocated() {
            if !((*iri).query).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*iri).query as *mut libc::c_void);
                (*iri).query = 0 as *const libc::c_char;
            }
        }
        if (*iri).fragment_allocated() {
            if !((*iri).fragment).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*iri).fragment as *mut libc::c_void);
                (*iri).fragment = 0 as *const libc::c_char;
            }
        }
        if !((*iri).connection_part).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*iri).connection_part as *mut libc::c_void);
            (*iri).connection_part = 0 as *const libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_free(mut iri: *mut *mut wget_iri) {
    if !iri.is_null() && !(*iri).is_null() {
        wget_iri_free_content(*iri);
        if !(*iri).is_null() {
            wget_free.expect("non-null function pointer")(*iri as *mut libc::c_void);
            *iri = 0 as *mut wget_iri;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_parse(
    mut url: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> *mut wget_iri {
    let mut iri: *mut wget_iri = 0 as *mut wget_iri;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut authority: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut slen: size_t = 0;
    let mut extra: size_t = 0;
    let mut have_scheme: libc::c_int = 0;
    if url.is_null() {
        return 0 as *mut wget_iri;
    }
    while c_isspace(*url as libc::c_int) {
        url = url.offset(1);
        url;
    }
    if *url == 0 {
        return 0 as *mut wget_iri;
    }
    if c_isalpha(*url as libc::c_int) {
        let mut x: *const libc::c_char = 0 as *const libc::c_char;
        have_scheme = 1 as libc::c_int;
        x = url;
        while *x as libc::c_int != 0
            && (c_isalnum(*x as libc::c_int) as libc::c_int != 0
                || *x as libc::c_int == '+' as i32 || *x as libc::c_int == '-' as i32
                || *x as libc::c_int == '.' as i32)
        {
            x = x.offset(1);
            x;
        }
        if *x as libc::c_int != ':' as i32
            || c_isdigit(*x.offset(1 as libc::c_int as isize) as libc::c_int)
                as libc::c_int != 0
        {
            have_scheme = 0 as libc::c_int;
        }
    } else {
        have_scheme = 0 as libc::c_int;
    }
    slen = strlen(url);
    extra = if have_scheme != 0 {
        0 as libc::c_int as libc::c_ulong
    } else {
        (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    };
    iri = wget_calloc(
        1 as libc::c_int as size_t,
        (::core::mem::size_of::<wget_iri>() as libc::c_ulong)
            .wrapping_add(
                slen.wrapping_add(extra).wrapping_add(1 as libc::c_int as size_t)
                    * 2 as libc::c_int as size_t,
            ),
    ) as *mut wget_iri;
    if iri.is_null() {
        return 0 as *mut wget_iri;
    }
    (*iri).msize = slen.wrapping_add(extra).wrapping_add(1 as libc::c_int as size_t);
    if have_scheme != 0 {
        (*iri)
            .uri = memcpy(
            iri.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            url as *const libc::c_void,
            slen.wrapping_add(1 as libc::c_int as size_t),
        ) as *const libc::c_char;
        s = memcpy(
            ((*iri).uri as *mut libc::c_char).offset((*iri).msize as isize)
                as *mut libc::c_void,
            url as *const libc::c_void,
            slen.wrapping_add(1 as libc::c_int as size_t),
        ) as *mut libc::c_char;
        p = s;
        s = strchr(s, ':' as i32);
        let fresh5 = s;
        s = s.offset(1);
        *fresh5 = 0 as libc::c_int as libc::c_char;
        wget_iri_unescape_inline(p);
        wget_strtolower(p);
        let mut found: bool = 0 as libc::c_int != 0;
        let mut it: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (it as libc::c_ulong)
            < (::core::mem::size_of::<[iri_scheme; 2]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<iri_scheme>() as libc::c_ulong)
        {
            if strcmp((schemes[it as usize].name).as_ptr(), p) == 0 {
                (*iri).scheme = it as wget_iri_scheme;
                (*iri).port = schemes[it as usize].port;
                found = 1 as libc::c_int != 0;
                break;
            } else {
                it = it.wrapping_add(1);
                it;
            }
        }
        if !found {
            wget_debug_printf(
                b"Unsupported scheme in '%s'\n\0" as *const u8 as *const libc::c_char,
                url,
            );
            wget_iri_free(&mut iri);
            return 0 as *mut wget_iri;
        }
    } else {
        (*iri)
            .uri = memcpy(
            iri.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            b"http://\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as *const libc::c_char;
        memcpy(
            ((*iri).uri as *mut libc::c_char)
                .offset(
                    (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as *mut libc::c_void,
            url as *const libc::c_void,
            slen.wrapping_add(1 as libc::c_int as size_t),
        );
        s = memcpy(
            ((*iri).uri as *mut libc::c_char).offset((*iri).msize as isize)
                as *mut libc::c_void,
            (*iri).uri as *const libc::c_void,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(slen)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        *s
            .offset(
                (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(3 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        s = s
            .offset(
                (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        (*iri).scheme = WGET_IRI_SCHEME_HTTP;
        (*iri).port = schemes[WGET_IRI_SCHEME_HTTP as libc::c_int as usize].port;
    }
    while *s as libc::c_int == '/' as i32 {
        s = s.offset(1);
        s;
    }
    authority = s;
    while *s as libc::c_int != 0 && *s as libc::c_int != '/' as i32
        && *s as libc::c_int != '?' as i32 && *s as libc::c_int != '#' as i32
    {
        s = s.offset(1);
        s;
    }
    c = *s;
    if c != 0 {
        let fresh6 = s;
        s = s.offset(1);
        *fresh6 = 0 as libc::c_int as libc::c_char;
    }
    wget_iri_unescape_inline(authority);
    if c as libc::c_int == '/' as i32 {
        (*iri).path = s;
        while *s as libc::c_int != 0 && *s as libc::c_int != '?' as i32
            && *s as libc::c_int != '#' as i32
        {
            s = s.offset(1);
            s;
        }
        c = *s;
        if c != 0 {
            let fresh7 = s;
            s = s.offset(1);
            *fresh7 = 0 as libc::c_int as libc::c_char;
        }
        wget_iri_unescape_inline((*iri).path as *mut libc::c_char);
        normalize_path((*iri).path as *mut libc::c_char);
    }
    if c as libc::c_int == '?' as i32 {
        (*iri).query = s;
        while *s as libc::c_int != 0 && *s as libc::c_int != '#' as i32 {
            if *s as libc::c_int == '+' as i32 {
                *s = ' ' as i32 as libc::c_char;
            }
            s = s.offset(1);
            s;
        }
        c = *s;
        if c != 0 {
            let fresh8 = s;
            s = s.offset(1);
            *fresh8 = 0 as libc::c_int as libc::c_char;
        }
    }
    if c as libc::c_int == '#' as i32 {
        (*iri).fragment = s;
        s = s.offset(strlen(s) as isize);
        wget_iri_unescape_inline((*iri).fragment as *mut libc::c_char);
    }
    if *s != 0 {
        wget_debug_printf(
            b"unparsed rest '%s'\n\0" as *const u8 as *const libc::c_char,
            s,
        );
    }
    if *authority != 0 {
        s = authority;
        p = strchr(authority, '@' as i32);
        if !p.is_null() {
            (*iri).userinfo = s;
            *p = 0 as libc::c_int as libc::c_char;
            s = strchr(s, ':' as i32);
            if !s.is_null() {
                *s = 0 as libc::c_int as libc::c_char;
                (*iri).password = s.offset(1 as libc::c_int as isize);
            }
            s = p.offset(1 as libc::c_int as isize);
        }
        if *s as libc::c_int == '[' as i32 {
            p = strrchr(s, ']' as i32);
            if !p.is_null() {
                (*iri).host = s.offset(1 as libc::c_int as isize);
                *p = 0 as libc::c_int as libc::c_char;
                s = p.offset(1 as libc::c_int as isize);
            } else {
                (*iri).host = s.offset(1 as libc::c_int as isize);
                s = s.offset(strlen(s) as isize);
            }
        } else {
            (*iri).host = s;
            while *s as libc::c_int != 0 && *s as libc::c_int != ':' as i32 {
                s = s.offset(1);
                s;
            }
        }
        if *s as libc::c_int == ':' as i32 {
            if c_isdigit(*s.offset(1 as libc::c_int as isize) as libc::c_int) {
                let mut port: libc::c_ulong = strtoul(
                    s.offset(1 as libc::c_int as isize),
                    0 as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                if port == 0 as libc::c_int as libc::c_ulong
                    || port > 65535 as libc::c_int as libc::c_ulong
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Port number must be in the range 1..65535\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    wget_iri_free(&mut iri);
                    return 0 as *mut wget_iri;
                }
                (*iri).port = port as uint16_t;
                (*iri).set_port_given(1 as libc::c_int != 0);
            }
        }
        *s = 0 as libc::c_int as libc::c_char;
    }
    if !((*iri).host).is_null() {
        wget_strtolower((*iri).host as *mut libc::c_char);
        if wget_str_needs_encoding((*iri).host) {
            s = wget_str_to_utf8((*iri).host, encoding);
            if !s.is_null() {
                (*iri).host = s;
                (*iri).set_host_allocated(1 as libc::c_int != 0);
            }
        }
        p = wget_str_to_ascii((*iri).host) as *mut libc::c_char;
        if p != (*iri).host as *mut libc::c_char {
            if (*iri).host_allocated() {
                if !((*iri).host).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )((*iri).host as *mut libc::c_void);
                    (*iri).host = 0 as *const libc::c_char;
                }
            }
            (*iri).host = p;
            (*iri).set_host_allocated(1 as libc::c_int != 0);
        }
        if wget_ip_is_family((*iri).host, 1 as libc::c_int) as libc::c_int != 0
            || wget_ip_is_family((*iri).host, 2 as libc::c_int) as libc::c_int != 0
        {
            (*iri).set_is_ip_address(1 as libc::c_int != 0);
        }
    }
    if ((*iri).host).is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Missing host/domain in URI '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*iri).uri,
        );
        wget_iri_free(&mut iri);
        return 0 as *mut wget_iri;
    }
    if !((*iri).path).is_null()
        && wget_str_needs_encoding((*iri).path) as libc::c_int != 0
    {
        s = wget_str_to_utf8((*iri).path, encoding);
        if !s.is_null() {
            (*iri).path = s;
            (*iri).set_path_allocated(1 as libc::c_int != 0);
        }
    }
    if !((*iri).query).is_null()
        && wget_str_needs_encoding((*iri).query) as libc::c_int != 0
    {
        s = wget_str_to_utf8((*iri).query, encoding);
        if !s.is_null() {
            (*iri).query = s;
            (*iri).set_query_allocated(1 as libc::c_int != 0);
        }
    }
    if !((*iri).fragment).is_null()
        && wget_str_needs_encoding((*iri).fragment) as libc::c_int != 0
    {
        s = wget_str_to_utf8((*iri).fragment, encoding);
        if !s.is_null() {
            (*iri).fragment = s;
            (*iri).set_fragment_allocated(1 as libc::c_int != 0);
        }
    }
    if !((*iri).userinfo).is_null() {
        (*iri).safe_uri = create_safe_uri(iri);
    } else {
        (*iri).safe_uri = (*iri).uri;
    }
    return iri;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_clone(mut iri: *const wget_iri) -> *mut wget_iri {
    if iri.is_null() || ((*iri).uri).is_null() {
        return 0 as *mut wget_iri;
    }
    let mut clone: *mut wget_iri = wget_memdup(
        iri as *const libc::c_void,
        (::core::mem::size_of::<wget_iri>() as libc::c_ulong)
            .wrapping_add((*iri).msize * 2 as libc::c_int as size_t),
    ) as *mut wget_iri;
    if clone.is_null() {
        return 0 as *mut wget_iri;
    }
    (*clone).uri = clone.offset(1 as libc::c_int as isize) as *mut libc::c_char;
    if !((*iri).userinfo).is_null() {
        (*clone).safe_uri = wget_strdup((*iri).safe_uri);
    } else {
        (*clone).safe_uri = (*clone).uri;
    }
    (*clone).connection_part = wget_strdup((*iri).connection_part);
    if (*iri).host_allocated() {
        (*clone).host = wget_strdup((*iri).host);
    } else {
        (*clone)
            .host = if !((*iri).host).is_null() {
            (clone as *mut libc::c_char)
                .offset(
                    ((*iri).host).offset_from(iri as *const libc::c_char) as libc::c_long
                        as size_t as isize,
                )
        } else {
            0 as *mut libc::c_char
        };
    }
    (*clone)
        .display = if !((*iri).display).is_null() {
        (clone as *mut libc::c_char)
            .offset(
                ((*iri).display).offset_from(iri as *const libc::c_char) as libc::c_long
                    as size_t as isize,
            )
    } else {
        0 as *mut libc::c_char
    };
    (*clone)
        .userinfo = if !((*iri).userinfo).is_null() {
        (clone as *mut libc::c_char)
            .offset(
                ((*iri).userinfo).offset_from(iri as *const libc::c_char) as libc::c_long
                    as size_t as isize,
            )
    } else {
        0 as *mut libc::c_char
    };
    (*clone)
        .password = if !((*iri).password).is_null() {
        (clone as *mut libc::c_char)
            .offset(
                ((*iri).password).offset_from(iri as *const libc::c_char) as libc::c_long
                    as size_t as isize,
            )
    } else {
        0 as *mut libc::c_char
    };
    if (*iri).path_allocated() {
        (*clone).path = wget_strdup((*iri).path);
    } else {
        (*clone)
            .path = if !((*iri).path).is_null() {
            (clone as *mut libc::c_char)
                .offset(
                    ((*iri).path).offset_from(iri as *const libc::c_char) as libc::c_long
                        as size_t as isize,
                )
        } else {
            0 as *mut libc::c_char
        };
    }
    if (*iri).query_allocated() {
        (*clone).query = wget_strdup((*iri).query);
    } else {
        (*clone)
            .query = if !((*iri).query).is_null() {
            (clone as *mut libc::c_char)
                .offset(
                    ((*iri).query).offset_from(iri as *const libc::c_char)
                        as libc::c_long as size_t as isize,
                )
        } else {
            0 as *mut libc::c_char
        };
    }
    if (*iri).fragment_allocated() {
        (*clone).fragment = wget_strdup((*iri).fragment);
    } else {
        (*clone)
            .fragment = if !((*iri).fragment).is_null() {
            (clone as *mut libc::c_char)
                .offset(
                    ((*iri).fragment).offset_from(iri as *const libc::c_char)
                        as libc::c_long as size_t as isize,
                )
        } else {
            0 as *mut libc::c_char
        };
    }
    return clone;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_get_connection_part(
    mut iri: *const wget_iri,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    if !iri.is_null() {
        if wget_ip_is_family((*iri).host, 2 as libc::c_int) {
            wget_buffer_printf_append(
                buf,
                b"%s://[%s]\0" as *const u8 as *const libc::c_char,
                (schemes[(*iri).scheme as usize].name).as_ptr(),
                (*iri).host,
            );
        } else {
            wget_buffer_printf_append(
                buf,
                b"%s://%s\0" as *const u8 as *const libc::c_char,
                (schemes[(*iri).scheme as usize].name).as_ptr(),
                (*iri).host,
            );
        }
        if (*iri).port_given() {
            wget_buffer_printf_append(
                buf,
                b":%hu\0" as *const u8 as *const libc::c_char,
                (*iri).port as libc::c_int,
            );
        }
    }
    return (*buf).data;
}
unsafe extern "C" fn normalize_path(mut path: *mut libc::c_char) -> size_t {
    let mut p1: *mut libc::c_char = path;
    let mut p2: *mut libc::c_char = path;
    wget_debug_printf(b"path %s ->\n\0" as *const u8 as *const libc::c_char, path);
    loop {
        if *p2 as libc::c_int == '/' as i32 {
            p2 = p2.offset(1);
            p2;
        } else {
            if !(*p2 as libc::c_int == '.' as i32) {
                break;
            }
            if *p2.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                p2 = p2.offset(2 as libc::c_int as isize);
            } else if *p2.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
            {
                if *p2.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32 {
                    p2 = p2.offset(3 as libc::c_int as isize);
                } else {
                    if !(*p2.offset(2 as libc::c_int as isize) == 0) {
                        break;
                    }
                    p2 = p2.offset(2 as libc::c_int as isize);
                }
            } else {
                if !(*p2.offset(1 as libc::c_int as isize) == 0) {
                    break;
                }
                p2 = p2.offset(1);
                p2;
            }
        }
    }
    while *p2 as libc::c_int != 0 && *p2 as libc::c_int != '?' as i32
        && *p2 as libc::c_int != '#' as i32
    {
        if *p2 as libc::c_int == '/' as i32 {
            if *p2.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32 {
                if strncmp(
                    p2,
                    b"/../\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    p2 = p2.offset(3 as libc::c_int as isize);
                    while p1 > path
                        && {
                            p1 = p1.offset(-1);
                            *p1 as libc::c_int != '/' as i32
                        }
                    {}
                } else if strcmp(p2, b"/..\0" as *const u8 as *const libc::c_char) == 0 {
                    p2 = p2.offset(3 as libc::c_int as isize);
                    while p1 > path
                        && {
                            p1 = p1.offset(-1);
                            *p1 as libc::c_int != '/' as i32
                        }
                    {}
                    if p1 > path {
                        let fresh9 = p1;
                        p1 = p1.offset(1);
                        *fresh9 = '/' as i32 as libc::c_char;
                    }
                } else if strncmp(
                    p2,
                    b"/./\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    p2 = p2.offset(2 as libc::c_int as isize);
                } else if strcmp(p2, b"/.\0" as *const u8 as *const libc::c_char) == 0 {
                    p2 = p2.offset(2 as libc::c_int as isize);
                    if p1 > path {
                        let fresh10 = p1;
                        p1 = p1.offset(1);
                        *fresh10 = '/' as i32 as libc::c_char;
                    }
                } else {
                    let fresh11 = p2;
                    p2 = p2.offset(1);
                    let fresh12 = p1;
                    p1 = p1.offset(1);
                    *fresh12 = *fresh11;
                }
            } else if p1 == path {
                p2 = p2.offset(1);
                p2;
            } else if *p2.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                p2 = p2.offset(1);
                p2;
            } else {
                let fresh13 = p2;
                p2 = p2.offset(1);
                let fresh14 = p1;
                p1 = p1.offset(1);
                *fresh14 = *fresh13;
            }
        } else {
            let fresh15 = p2;
            p2 = p2.offset(1);
            let fresh16 = p1;
            p1 = p1.offset(1);
            *fresh16 = *fresh15;
        }
    }
    if p1 != p2 {
        while *p2 != 0 {
            let fresh17 = p2;
            p2 = p2.offset(1);
            let fresh18 = p1;
            p1 = p1.offset(1);
            *fresh18 = *fresh17;
        }
        *p1 = 0 as libc::c_int as libc::c_char;
    } else {
        p1 = p1.offset(strlen(p1) as isize);
    }
    wget_debug_printf(b"     %s\n\0" as *const u8 as *const libc::c_char, path);
    return p1.offset_from(path) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_relative_to_abs(
    mut base: *const wget_iri,
    mut val: *const libc::c_char,
    mut len: size_t,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    if len == -(1 as libc::c_int) as size_t {
        len = strlen(val);
    }
    if *val as libc::c_int == '/' as i32 {
        if !base.is_null() {
            let mut tmp: [libc::c_char; 4096] = [0; 4096];
            let mut path: *mut libc::c_char = tmp.as_mut_ptr();
            if len >= ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong {
                path = wget_malloc(len.wrapping_add(1 as libc::c_int as size_t))
                    as *mut libc::c_char;
                if path.is_null() {
                    return 0 as *const libc::c_char;
                }
            }
            wget_strscpy(path, val, len.wrapping_add(1 as libc::c_int as size_t));
            if len >= 2 as libc::c_int as size_t
                && *val.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                p = strchr(path.offset(2 as libc::c_int as isize), '/' as i32);
                if !p.is_null() {
                    normalize_path(p.offset(1 as libc::c_int as isize));
                }
                wget_buffer_strcpy(
                    buf,
                    (schemes[(*base).scheme as usize].name).as_ptr(),
                );
                wget_buffer_strcat(buf, b":\0" as *const u8 as *const libc::c_char);
                wget_buffer_strcat(buf, path);
            } else {
                normalize_path(path);
                wget_buffer_reset(buf);
                wget_iri_get_connection_part(base, buf);
                wget_buffer_strcat(buf, b"/\0" as *const u8 as *const libc::c_char);
                wget_buffer_strcat(buf, path);
            }
            if path != tmp.as_mut_ptr() {
                if !path.is_null() {
                    wget_free
                        .expect("non-null function pointer")(path as *mut libc::c_void);
                    path = 0 as *mut libc::c_char;
                }
            }
        } else {
            return 0 as *const libc::c_char
        }
    } else if !(memchr(val as *const libc::c_void, ':' as i32, len)).is_null() {
        if !buf.is_null() {
            wget_buffer_memcpy(buf, val as *const libc::c_void, len);
        } else {
            return val
        }
    } else if !base.is_null() {
        let mut lastsep: *const libc::c_char = if !((*base).path).is_null() {
            strrchr((*base).path, '/' as i32)
        } else {
            0 as *mut libc::c_char
        };
        wget_buffer_reset(buf);
        wget_iri_get_connection_part(base, buf);
        wget_buffer_strcat(buf, b"/\0" as *const u8 as *const libc::c_char);
        let mut tmp_len: size_t = (*buf).length;
        if !lastsep.is_null() {
            wget_buffer_memcat(
                buf,
                (*base).path as *const libc::c_void,
                (lastsep.offset_from((*base).path) as libc::c_long
                    + 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
        if len != 0 {
            wget_buffer_memcat(buf, val as *const libc::c_void, len);
        }
        (*buf)
            .length = (normalize_path(((*buf).data).offset(tmp_len as isize)))
            .wrapping_add(tmp_len);
    } else if *val.offset(len as isize) as libc::c_int == 0 as libc::c_int {
        return val
    } else {
        return 0 as *const libc::c_char
    }
    return if !buf.is_null() as libc::c_int as libc::c_long != 0 {
        (*buf).data
    } else {
        0 as *mut libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_parse_base(
    mut base: *const wget_iri,
    mut url: *const libc::c_char,
    mut encoding: *const libc::c_char,
) -> *mut wget_iri {
    let mut iri: *mut wget_iri = 0 as *mut wget_iri;
    if !base.is_null() {
        let mut buf: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        let mut sbuf: [libc::c_char; 256] = [0; 256];
        wget_buffer_init(
            &mut buf,
            sbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        );
        iri = wget_iri_parse(
            wget_iri_relative_to_abs(base, url, -(1 as libc::c_int) as size_t, &mut buf),
            encoding,
        );
        wget_buffer_deinit(&mut buf);
    } else {
        iri = wget_iri_parse(
            wget_iri_relative_to_abs(
                0 as *const wget_iri,
                url,
                -(1 as libc::c_int) as size_t,
                0 as *mut wget_buffer,
            ),
            encoding,
        );
    }
    return iri;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_compare(
    mut iri1: *const wget_iri,
    mut iri2: *const wget_iri,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if iri1.is_null() {
        if iri2.is_null() { return 0 as libc::c_int } else { return -(1 as libc::c_int) }
    } else if iri2.is_null() {
        return 1 as libc::c_int
    }
    n = wget_strcasecmp((*iri1).path, (*iri2).path);
    if n != 0 {
        return n;
    }
    n = wget_strcasecmp((*iri1).query, (*iri2).query);
    if n != 0 {
        return n;
    }
    if (*iri1).scheme as libc::c_uint != (*iri2).scheme as libc::c_uint {
        return if ((*iri1).scheme as libc::c_uint) < (*iri2).scheme as libc::c_uint {
            -(1 as libc::c_int)
        } else {
            1 as libc::c_int
        };
    }
    n = (*iri1).port as libc::c_int - (*iri2).port as libc::c_int;
    if n != 0 {
        return n;
    }
    n = strcmp((*iri1).host, (*iri2).host);
    if n != 0 {
        return n;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_escape(
    mut src: *const libc::c_char,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    if src.is_null() {
        return (*buf).data;
    }
    begin = src;
    while *src != 0 {
        if iri_ctype[*src as libc::c_uchar as usize] as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int == 0
        {
            if begin != src {
                wget_buffer_memcat(
                    buf,
                    begin as *const libc::c_void,
                    src.offset_from(begin) as libc::c_long as size_t,
                );
            }
            begin = src.offset(1 as libc::c_int as isize);
            wget_buffer_printf_append(
                buf,
                b"%%%02X\0" as *const u8 as *const libc::c_char,
                *src as libc::c_uchar as libc::c_int,
            );
        }
        src = src.offset(1);
        src;
    }
    if begin != src {
        wget_buffer_memcat(
            buf,
            begin as *const libc::c_void,
            src.offset_from(begin) as libc::c_long as size_t,
        );
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_escape_path(
    mut src: *const libc::c_char,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    begin = src;
    while *src != 0 {
        if !(iri_ctype[*src as libc::c_uchar as usize] as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0
            || iri_ctype[*src as libc::c_uchar as usize] as libc::c_int
                & (1 as libc::c_int) << 1 as libc::c_int != 0
            || *src as libc::c_int == '/' as i32 || *src as libc::c_int == ':' as i32
            || *src as libc::c_int == '@' as i32)
        {
            if begin != src {
                wget_buffer_memcat(
                    buf,
                    begin as *const libc::c_void,
                    src.offset_from(begin) as libc::c_long as size_t,
                );
            }
            begin = src.offset(1 as libc::c_int as isize);
            wget_buffer_printf_append(
                buf,
                b"%%%02X\0" as *const u8 as *const libc::c_char,
                *src as libc::c_uchar as libc::c_int,
            );
        }
        src = src.offset(1);
        src;
    }
    if begin != src {
        wget_buffer_memcat(
            buf,
            begin as *const libc::c_void,
            src.offset_from(begin) as libc::c_long as size_t,
        );
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_escape_query(
    mut src: *const libc::c_char,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    begin = src;
    while *src != 0 {
        if iri_ctype[*src as libc::c_uchar as usize] as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int == 0
            && *src as libc::c_int != '=' as i32 && *src as libc::c_int != '&' as i32
        {
            if begin != src {
                wget_buffer_memcat(
                    buf,
                    begin as *const libc::c_void,
                    src.offset_from(begin) as libc::c_long as size_t,
                );
            }
            begin = src.offset(1 as libc::c_int as isize);
            if *src as libc::c_int == ' ' as i32 {
                wget_buffer_memcat(
                    buf,
                    b"+\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            } else {
                wget_buffer_printf_append(
                    buf,
                    b"%%%02X\0" as *const u8 as *const libc::c_char,
                    *src as libc::c_uchar as libc::c_int,
                );
            }
        }
        src = src.offset(1);
        src;
    }
    if begin != src {
        wget_buffer_memcat(
            buf,
            begin as *const libc::c_void,
            src.offset_from(begin) as libc::c_long as size_t,
        );
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_get_escaped_host(
    mut iri: *const wget_iri,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    return wget_iri_escape((*iri).host, buf);
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_get_escaped_resource(
    mut iri: *const wget_iri,
    mut buf: *mut wget_buffer,
) -> *const libc::c_char {
    if !((*iri).path).is_null() {
        wget_iri_escape_path((*iri).path, buf);
    }
    if !((*iri).query).is_null() {
        wget_buffer_memcat(
            buf,
            b"?\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        let mut p: *const libc::c_char = (*iri).query;
        while *p != 0 {
            wget_buffer_memcat(
                buf,
                (if *p as libc::c_int == ' ' as i32 {
                    b"+\0" as *const u8 as *const libc::c_char
                } else {
                    p
                }) as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            p = p.offset(1);
            p;
        }
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_get_path(
    mut iri: *const wget_iri,
    mut buf: *mut wget_buffer,
    mut encoding: *const libc::c_char,
) -> *mut libc::c_char {
    if (*buf).length != 0 as libc::c_int as size_t
        && *((*buf).data)
            .offset(((*buf).length).wrapping_sub(1 as libc::c_int as size_t) as isize)
            as libc::c_int != '/' as i32
    {
        wget_buffer_memcat(
            buf,
            b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    if !((*iri).path).is_null() {
        if wget_strcasecmp_ascii(
            encoding,
            b"utf-8\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
            fname = wget_utf8_to_str((*iri).path, encoding);
            if !fname.is_null() {
                wget_buffer_strcat(buf, fname);
                if !fname.is_null() {
                    wget_free
                        .expect("non-null function pointer")(fname as *mut libc::c_void);
                    fname = 0 as *mut libc::c_char;
                }
            } else {
                wget_buffer_strcat(buf, (*iri).path);
            }
        } else {
            wget_buffer_strcat(buf, (*iri).path);
        }
    }
    if ((*buf).length == 0 as libc::c_int as size_t
        || *((*buf).data)
            .offset(((*buf).length).wrapping_sub(1 as libc::c_int as size_t) as isize)
            as libc::c_int == '/' as i32) && !default_page.is_null()
    {
        wget_buffer_memcat(
            buf,
            default_page as *const libc::c_void,
            default_page_length,
        );
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_get_query_as_filename(
    mut iri: *const wget_iri,
    mut buf: *mut wget_buffer,
    mut encoding: *const libc::c_char,
) -> *mut libc::c_char {
    if !((*iri).query).is_null() {
        let mut query: *const libc::c_char = 0 as *const libc::c_char;
        let mut allocated: libc::c_int = 0 as libc::c_int;
        wget_buffer_memcat(
            buf,
            b"?\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        if wget_strcasecmp_ascii(
            encoding,
            b"utf-8\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            query = wget_utf8_to_str((*iri).query, encoding);
            if !query.is_null() {
                allocated = 1 as libc::c_int;
            } else {
                query = (*iri).query;
            }
        } else {
            query = (*iri).query;
        }
        let mut slashes: libc::c_int = 0 as libc::c_int;
        let mut src: *const libc::c_char = query;
        loop {
            src = strchr(src, '/' as i32);
            if src.is_null() {
                break;
            }
            slashes += 1;
            slashes;
            src = src.offset(1);
            src;
        }
        if slashes != 0 {
            let mut begin: *const libc::c_char = 0 as *const libc::c_char;
            begin = query;
            src = begin;
            while *src != 0 {
                if *src as libc::c_int == '/' as i32 {
                    if begin != src {
                        wget_buffer_memcat(
                            buf,
                            begin as *const libc::c_void,
                            src.offset_from(begin) as libc::c_long as size_t,
                        );
                    }
                    begin = src.offset(1 as libc::c_int as isize);
                    wget_buffer_memcat(
                        buf,
                        b"%2F\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        3 as libc::c_int as size_t,
                    );
                }
                src = src.offset(1);
                src;
            }
            if begin != src {
                wget_buffer_memcat(
                    buf,
                    begin as *const libc::c_void,
                    src.offset_from(begin) as libc::c_long as size_t,
                );
            }
        } else {
            wget_buffer_strcat(buf, query);
        }
        if allocated != 0 {
            if !query.is_null() {
                wget_free
                    .expect("non-null function pointer")(query as *mut libc::c_void);
                query = 0 as *const libc::c_char;
            }
        }
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_get_basename(
    mut iri: *const wget_iri,
    mut buf: *mut wget_buffer,
    mut encoding: *const libc::c_char,
    mut flags: libc::c_int,
) -> *mut libc::c_char {
    if !((*iri).path).is_null() {
        let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
        if wget_strcasecmp_ascii(
            encoding,
            b"utf-8\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
            p = strrchr((*iri).path, '/' as i32);
            if !p.is_null() {
                fname = wget_utf8_to_str(p.offset(1 as libc::c_int as isize), encoding);
                if fname.is_null() {
                    wget_buffer_strcat(buf, p.offset(1 as libc::c_int as isize));
                }
            } else {
                fname = wget_utf8_to_str((*iri).path, encoding);
                if fname.is_null() {
                    wget_buffer_strcat(buf, (*iri).path);
                }
            }
            if !fname.is_null() {
                wget_buffer_strcat(buf, fname);
                if !fname.is_null() {
                    wget_free
                        .expect("non-null function pointer")(fname as *mut libc::c_void);
                    fname = 0 as *mut libc::c_char;
                }
            }
        } else {
            fname = strrchr((*iri).path, '/' as i32);
            if !fname.is_null() {
                wget_buffer_strcat(buf, fname.offset(1 as libc::c_int as isize));
            } else {
                wget_buffer_strcat(buf, (*iri).path);
            }
        }
    }
    if ((*buf).length == 0 as libc::c_int as size_t
        || *((*buf).data)
            .offset(((*buf).length).wrapping_sub(1 as libc::c_int as size_t) as isize)
            as libc::c_int == '/' as i32) && !default_page.is_null()
    {
        wget_buffer_memcat(
            buf,
            default_page as *const libc::c_void,
            default_page_length,
        );
    }
    if flags & 1 as libc::c_int != 0 {
        return wget_iri_get_query_as_filename(iri, buf, encoding);
    }
    return (*buf).data;
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_set_defaultpage(mut page: *const libc::c_char) {
    default_page = page;
    default_page_length = if !default_page.is_null() {
        strlen(default_page)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_set_defaultport(
    mut scheme: wget_iri_scheme,
    mut port: uint16_t,
) -> libc::c_int {
    if (scheme as libc::c_uint as libc::c_ulong)
        < (::core::mem::size_of::<[iri_scheme; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<iri_scheme>() as libc::c_ulong)
    {
        schemes[scheme as usize].port = port;
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_iri_set_scheme(
    mut iri: *mut wget_iri,
    mut scheme: wget_iri_scheme,
) -> wget_iri_scheme {
    let mut old_scheme: wget_iri_scheme = (*iri).scheme;
    if (scheme as libc::c_uint as libc::c_ulong)
        < (::core::mem::size_of::<[iri_scheme; 2]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<iri_scheme>() as libc::c_ulong)
        && (*iri).scheme as libc::c_uint != scheme as libc::c_uint
    {
        (*iri).scheme = scheme;
        if (*iri).port as libc::c_int == schemes[old_scheme as usize].port as libc::c_int
        {
            (*iri).port = schemes[scheme as usize].port;
        }
        let mut old_scheme_len: size_t = strlen(
            (schemes[old_scheme as usize].name).as_ptr(),
        );
        let mut scheme_len: size_t = strlen((schemes[scheme as usize].name).as_ptr());
        if strncmp(
            (*iri).uri,
            (schemes[old_scheme as usize].name).as_ptr(),
            old_scheme_len,
        ) == 0 as libc::c_int
            && *((*iri).uri).offset(old_scheme_len as isize) as libc::c_int == ':' as i32
        {
            memmove(
                ((*iri).uri as *mut libc::c_char).offset(scheme_len as isize)
                    as *mut libc::c_void,
                ((*iri).uri).offset(old_scheme_len as isize) as *const libc::c_void,
                ((*iri).msize).wrapping_sub(old_scheme_len),
            );
            memcpy(
                (*iri).uri as *mut libc::c_char as *mut libc::c_void,
                (schemes[scheme as usize].name).as_ptr() as *const libc::c_void,
                scheme_len,
            );
        }
        if !((*iri).userinfo).is_null() {
            if !((*iri).safe_uri).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*iri).safe_uri as *mut libc::c_void);
                (*iri).safe_uri = 0 as *const libc::c_char;
            }
            (*iri).safe_uri = create_safe_uri(iri);
        } else {
            (*iri).safe_uri = (*iri).uri;
        }
    }
    return old_scheme;
}
unsafe extern "C" fn create_safe_uri(mut iri: *mut wget_iri) -> *mut libc::c_char {
    if iri.is_null() || ((*iri).uri).is_null() {
        return 0 as *mut libc::c_char;
    }
    let mut buf: *mut wget_buffer = wget_buffer_alloc(strlen((*iri).uri));
    if buf.is_null() {
        return 0 as *mut libc::c_char;
    }
    wget_buffer_printf(
        buf,
        b"%s://%s\0" as *const u8 as *const libc::c_char,
        (schemes[(*iri).scheme as usize].name).as_ptr(),
        (*iri).host,
    );
    if !((*iri).path).is_null() {
        wget_buffer_strcat(buf, b"/\0" as *const u8 as *const libc::c_char);
        wget_buffer_strcat(buf, (*iri).path);
    }
    if !((*iri).query).is_null() {
        wget_buffer_strcat(buf, b"?\0" as *const u8 as *const libc::c_char);
        wget_buffer_strcat(buf, (*iri).query);
    }
    if !((*iri).fragment).is_null() {
        wget_buffer_strcat(buf, b"#\0" as *const u8 as *const libc::c_char);
        wget_buffer_strcat(buf, (*iri).fragment);
    }
    let mut safe_uri: *mut libc::c_char = (*buf).data;
    (*buf).data = 0 as *mut libc::c_char;
    wget_buffer_free(&mut buf);
    return safe_uri;
}
