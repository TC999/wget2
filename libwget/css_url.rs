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
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
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
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_iri_relative_to_abs(
        base: *const wget_iri,
        val: *const libc::c_char,
        len: size_t,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_css_parse_buffer(
        buf: *const libc::c_char,
        len: size_t,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
    fn wget_css_parse_file(
        fname: *const libc::c_char,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint16_t = libc::c_ushort;
pub type size_t = libc::c_ulong;
pub type uint16_t = __uint16_t;
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
pub struct wget_css_parsed_url_st {
    pub len: size_t,
    pub pos: size_t,
    pub url: *const libc::c_char,
    pub abs_url: *const libc::c_char,
}
pub type wget_css_parsed_url = wget_css_parsed_url_st;
pub type wget_css_parse_uri_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
    size_t,
) -> ();
pub type wget_css_parse_encoding_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct css_context {
    pub encoding: *mut *const libc::c_char,
    pub uris: *mut wget_vector,
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
unsafe extern "C" fn url_free(mut url: *mut libc::c_void) {
    let mut u: *mut wget_css_parsed_url = url as *mut wget_css_parsed_url;
    if !((*u).url).is_null() {
        wget_free.expect("non-null function pointer")((*u).url as *mut libc::c_void);
        (*u).url = 0 as *const libc::c_char;
    }
    if !((*u).abs_url).is_null() {
        wget_free.expect("non-null function pointer")((*u).abs_url as *mut libc::c_void);
        (*u).abs_url = 0 as *const libc::c_char;
    }
    if !u.is_null() {
        wget_free.expect("non-null function pointer")(u as *mut libc::c_void);
        u = 0 as *mut wget_css_parsed_url;
    }
}
unsafe extern "C" fn get_encoding(
    mut context: *mut libc::c_void,
    mut encoding: *const libc::c_char,
    mut len: size_t,
) {
    let mut ctx: *mut css_context = context as *mut css_context;
    if (*(*ctx).encoding).is_null() {
        *(*ctx).encoding = wget_strmemdup(encoding as *const libc::c_void, len);
        wget_debug_printf(
            b"URI content encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
            *(*ctx).encoding,
        );
    }
}
unsafe extern "C" fn get_url(
    mut context: *mut libc::c_void,
    mut url: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut css_context = context as *mut css_context;
    let mut parsed_url: *mut wget_css_parsed_url = 0 as *mut wget_css_parsed_url;
    parsed_url = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_css_parsed_url>() as libc::c_ulong,
    ) as *mut wget_css_parsed_url;
    if parsed_url.is_null() {
        return;
    }
    (*parsed_url).url = wget_strmemdup(url as *const libc::c_void, len);
    if ((*parsed_url).url).is_null() {
        if !parsed_url.is_null() {
            wget_free
                .expect("non-null function pointer")(parsed_url as *mut libc::c_void);
            parsed_url = 0 as *mut wget_css_parsed_url;
        }
        return;
    }
    (*parsed_url).len = len;
    (*parsed_url).pos = pos;
    if ((*ctx).uris).is_null() {
        (*ctx).uris = wget_vector_create(16 as libc::c_int, None);
        wget_vector_set_destructor(
            (*ctx).uris,
            Some(url_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    wget_vector_add((*ctx).uris, parsed_url as *const libc::c_void);
}
unsafe extern "C" fn urls_to_absolute(
    mut urls: *mut wget_vector,
    mut base: *mut wget_iri,
) {
    if !base.is_null() && !urls.is_null() {
        let mut buf: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        wget_buffer_init(
            &mut buf,
            0 as *mut libc::c_char,
            1024 as libc::c_int as size_t,
        );
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < wget_vector_size(urls) {
            let mut url: *mut wget_css_parsed_url = wget_vector_get(urls, it)
                as *mut wget_css_parsed_url;
            if !(wget_iri_relative_to_abs(base, (*url).url, (*url).len, &mut buf))
                .is_null()
            {
                (*url)
                    .abs_url = wget_strmemdup(
                    buf.data as *const libc::c_void,
                    buf.length,
                );
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot resolve relative URI '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*url).url,
                );
            }
            it += 1;
            it;
        }
        wget_buffer_deinit(&mut buf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_css_get_urls(
    mut css: *const libc::c_char,
    mut len: size_t,
    mut base: *mut wget_iri,
    mut encoding: *mut *const libc::c_char,
) -> *mut wget_vector {
    let mut context: css_context = {
        let mut init = css_context {
            encoding: encoding,
            uris: 0 as *mut wget_vector,
        };
        init
    };
    wget_css_parse_buffer(
        css,
        len,
        Some(
            get_url
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        if !encoding.is_null() {
            Some(
                get_encoding
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            )
        } else {
            None
        },
        &mut context as *mut css_context as *mut libc::c_void,
    );
    urls_to_absolute(context.uris, base);
    return context.uris;
}
#[no_mangle]
pub unsafe extern "C" fn wget_css_get_urls_from_localfile(
    mut fname: *const libc::c_char,
    mut base: *mut wget_iri,
    mut encoding: *mut *const libc::c_char,
) -> *mut wget_vector {
    let mut context: css_context = {
        let mut init = css_context {
            encoding: encoding,
            uris: 0 as *mut wget_vector,
        };
        init
    };
    wget_css_parse_file(
        fname,
        Some(
            get_url
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        if !encoding.is_null() {
            Some(
                get_encoding
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            )
        } else {
            None
        },
        &mut context as *mut css_context as *mut libc::c_void,
    );
    urls_to_absolute(context.uris, base);
    return context.uris;
}
