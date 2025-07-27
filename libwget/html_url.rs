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
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_strmemcpy_a(
        s: *mut libc::c_char,
        ssize: size_t,
        m: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_find(v: *const wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_add_memdup(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_css_parse_buffer(
        buf: *const libc::c_char,
        len: size_t,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
    fn wget_html_parse_buffer(
        buf: *const libc::c_char,
        callback: Option::<wget_xml_callback>,
        user_ctx: *mut libc::c_void,
        hints: libc::c_int,
    );
    fn wget_http_parse_content_type(
        s: *const libc::c_char,
        content_type: *mut *const libc::c_char,
        charset: *mut *const libc::c_char,
    ) -> *const libc::c_char;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
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
pub struct wget_string {
    pub p: *const libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_html_parsed_url {
    pub url: wget_string,
    pub download: wget_string,
    pub attr: [libc::c_char; 16],
    pub tag: [libc::c_char; 16],
    #[bitfield(name = "link_inline", ty = "bool", bits = "0..=0")]
    pub link_inline: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_html_parsed_result {
    pub uris: *mut wget_vector,
    pub encoding: *const libc::c_char,
    pub base: wget_string,
    #[bitfield(name = "follow", ty = "bool", bits = "0..=0")]
    pub follow: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_html_tag {
    pub name: *const libc::c_char,
    pub attribute: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct html_context {
    pub result: wget_html_parsed_result,
    pub additional_tags: *mut wget_vector,
    pub ignore_tags: *mut wget_vector,
    pub download: wget_string,
    pub uri_index: libc::c_int,
    pub css_start_offset: size_t,
    pub found_robots: libc::c_char,
    pub found_content_type: libc::c_char,
    pub link_inline: libc::c_char,
    pub html: *const libc::c_char,
    pub css_attr: *const libc::c_char,
    pub css_dir: *const libc::c_char,
}
pub type wget_xml_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    libc::c_int,
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
    size_t,
    size_t,
) -> ();
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u) / 2 as libc::c_int as size_t;
        __p = (__base as *const libc::c_char).offset((__idx * __size) as isize)
            as *const libc::c_void;
        __comparison = (Some(__compar.expect("non-null function pointer")))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as size_t);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
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
static mut maybe: [libc::c_char; 256] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0,
    1 as libc::c_int as libc::c_char,
    0,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0,
    0,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0,
    0,
    1 as libc::c_int as libc::c_char,
    0,
    0,
    1 as libc::c_int as libc::c_char,
    0,
    1 as libc::c_int as libc::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut attrs: [[libc::c_char; 12]; 19] = unsafe {
    [
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"action\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"archive\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"background\0\0"),
        *::core::mem::transmute::<
            &[u8; 12],
            &[libc::c_char; 12],
        >(b"code\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"codebase\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 12],
            &[libc::c_char; 12],
        >(b"cite\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"classid\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 12],
            &[libc::c_char; 12],
        >(b"data\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"formaction\0\0"),
        *::core::mem::transmute::<
            &[u8; 12],
            &[libc::c_char; 12],
        >(b"href\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 12],
            &[libc::c_char; 12],
        >(b"icon\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"lowsrc\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"longdesc\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"manifest\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"profile\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"poster\0\0\0\0\0\0"),
        *::core::mem::transmute::<
            &[u8; 12],
            &[libc::c_char; 12],
        >(b"src\0\0\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"srcset\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"usemap\0\0\0\0\0\0"),
    ]
};
unsafe extern "C" fn css_parse_uri(
    mut context: *mut libc::c_void,
    mut url: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut html_context = context as *mut html_context;
    let mut res: *mut wget_html_parsed_result = &mut (*ctx).result;
    let mut parsed_url: *mut wget_html_parsed_url = 0 as *mut wget_html_parsed_url;
    parsed_url = wget_malloc(
        ::core::mem::size_of::<wget_html_parsed_url>() as libc::c_ulong,
    ) as *mut wget_html_parsed_url;
    if parsed_url.is_null() {
        return;
    }
    (*parsed_url).set_link_inline(1 as libc::c_int != 0);
    wget_strscpy(
        ((*parsed_url).attr).as_mut_ptr(),
        (*ctx).css_attr,
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    wget_strscpy(
        ((*parsed_url).tag).as_mut_ptr(),
        (*ctx).css_dir,
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    (*parsed_url)
        .url
        .p = ((*ctx).html).offset((*ctx).css_start_offset as isize).offset(pos as isize);
    (*parsed_url).url.len = len;
    (*parsed_url).download.p = 0 as *const libc::c_char;
    (*parsed_url).download.len = 0 as libc::c_int as size_t;
    if ((*res).uris).is_null() {
        (*res).uris = wget_vector_create(32 as libc::c_int, None);
    }
    wget_vector_add((*res).uris, parsed_url as *const libc::c_void);
}
unsafe extern "C" fn html_get_url(
    mut context: *mut libc::c_void,
    mut flags: libc::c_int,
    mut tag: *const libc::c_char,
    mut attr: *const libc::c_char,
    mut val: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut html_context = context as *mut html_context;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if *tag as libc::c_int | 0x20 as libc::c_int == 'a' as i32
            && (*tag.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                || wget_strcasecmp_ascii(
                    tag,
                    b"area\0" as *const u8 as *const libc::c_char,
                ) == 0)
        {
            (*ctx).uri_index = -(1 as libc::c_int);
            (*ctx).download.p = 0 as *const libc::c_char;
            (*ctx).download.len = 0 as libc::c_int as size_t;
        } else if *tag as libc::c_int | 0x20 as libc::c_int == 'm' as i32
            && wget_strcasecmp_ascii(tag, b"meta\0" as *const u8 as *const libc::c_char)
                == 0
        {
            (*ctx).found_content_type = 0 as libc::c_int as libc::c_char;
            (*ctx).found_robots = (*ctx).found_content_type;
        } else if *tag as libc::c_int | 0x20 as libc::c_int == 'l' as i32
            && wget_strcasecmp_ascii(tag, b"link\0" as *const u8 as *const libc::c_char)
                == 0
        {
            (*ctx).link_inline = 0 as libc::c_int as libc::c_char;
            (*ctx).uri_index = -(1 as libc::c_int);
        }
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 && !val.is_null() {
        let mut res: *mut wget_html_parsed_result = &mut (*ctx).result;
        if *tag as libc::c_int | 0x20 as libc::c_int == 'm' as i32
            && wget_strcasecmp_ascii(tag, b"meta\0" as *const u8 as *const libc::c_char)
                == 0
        {
            if (*ctx).found_robots == 0 {
                if wget_strcasecmp_ascii(
                    attr,
                    b"name\0" as *const u8 as *const libc::c_char,
                ) == 0
                    && wget_strncasecmp_ascii(
                        val,
                        b"robots\0" as *const u8 as *const libc::c_char,
                        len,
                    ) == 0
                {
                    (*ctx).found_robots = 1 as libc::c_int as libc::c_char;
                    return;
                }
            } else if (*ctx).found_robots as libc::c_int != 0
                && wget_strcasecmp_ascii(
                    attr,
                    b"content\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                let mut valbuf: [libc::c_char; 256] = [0; 256];
                let mut valp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut value: *const libc::c_char = 0 as *const libc::c_char;
                valp = wget_strmemcpy_a(
                    valbuf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    val as *const libc::c_void,
                    len,
                ) as *mut libc::c_char;
                value = valp;
                if value.is_null() {
                    return;
                }
                while *value != 0 {
                    let mut p: *const libc::c_char = 0 as *const libc::c_char;
                    while c_isspace(*value as libc::c_int) {
                        value = value.offset(1);
                        value;
                    }
                    if *value as libc::c_int == ',' as i32 {
                        value = value.offset(1);
                        value;
                    } else {
                        p = value;
                        while *p as libc::c_int != 0 && !c_isspace(*p as libc::c_int)
                            && *p as libc::c_int != ',' as i32
                        {
                            p = p.offset(1);
                            p;
                        }
                        if p == value {
                            break;
                        }
                        if wget_strncasecmp_ascii(
                            value,
                            b"all\0" as *const u8 as *const libc::c_char,
                            p.offset_from(value) as libc::c_long as size_t,
                        ) == 0
                            || wget_strncasecmp_ascii(
                                value,
                                b"follow\0" as *const u8 as *const libc::c_char,
                                p.offset_from(value) as libc::c_long as size_t,
                            ) == 0
                        {
                            (*res).set_follow(1 as libc::c_int != 0);
                        } else if wget_strncasecmp_ascii(
                            value,
                            b"nofollow\0" as *const u8 as *const libc::c_char,
                            p.offset_from(value) as libc::c_long as size_t,
                        ) == 0
                            || wget_strncasecmp_ascii(
                                value,
                                b"none\0" as *const u8 as *const libc::c_char,
                                p.offset_from(value) as libc::c_long as size_t,
                            ) == 0
                        {
                            (*res).set_follow(0 as libc::c_int != 0);
                        }
                        value = if *p as libc::c_int != 0 {
                            p.offset(1 as libc::c_int as isize)
                        } else {
                            p
                        };
                    }
                }
                if valp != valbuf.as_mut_ptr() {
                    if !valp.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(valp as *mut libc::c_void);
                        valp = 0 as *mut libc::c_char;
                    }
                }
                return;
            }
            if (*ctx).found_content_type as libc::c_int != 0
                && ((*res).encoding).is_null()
            {
                if wget_strcasecmp_ascii(
                    attr,
                    b"content\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    let mut valbuf_0: [libc::c_char; 256] = [0; 256];
                    let mut value_0: *const libc::c_char = 0 as *const libc::c_char;
                    value_0 = wget_strmemcpy_a(
                        valbuf_0.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                        val as *const libc::c_void,
                        len,
                    ) as *const libc::c_char;
                    if value_0.is_null() {
                        return;
                    }
                    wget_http_parse_content_type(
                        value_0,
                        0 as *mut *const libc::c_char,
                        &mut (*res).encoding,
                    );
                    if value_0 != valbuf_0.as_mut_ptr() as *const libc::c_char {
                        if !value_0.is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )(value_0 as *mut libc::c_void);
                            value_0 = 0 as *const libc::c_char;
                        }
                    }
                }
            } else if (*ctx).found_content_type == 0 && ((*res).encoding).is_null() {
                if wget_strcasecmp_ascii(
                    attr,
                    b"http-equiv\0" as *const u8 as *const libc::c_char,
                ) == 0
                    && wget_strncasecmp_ascii(
                        val,
                        b"Content-Type\0" as *const u8 as *const libc::c_char,
                        len,
                    ) == 0
                {
                    (*ctx).found_content_type = 1 as libc::c_int as libc::c_char;
                } else if wget_strcasecmp_ascii(
                    attr,
                    b"charset\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*res).encoding = wget_strmemdup(val as *const libc::c_void, len);
                }
            }
            return;
        }
        if !((*ctx).ignore_tags).is_null() {
            if wget_vector_find(
                (*ctx).ignore_tags,
                &mut {
                    let mut init = wget_html_tag {
                        name: tag,
                        attribute: 0 as *const libc::c_char,
                    };
                    init
                } as *mut wget_html_tag as *const libc::c_void,
            ) != -(1 as libc::c_int)
                || wget_vector_find(
                    (*ctx).ignore_tags,
                    &mut {
                        let mut init = wget_html_tag {
                            name: tag,
                            attribute: attr,
                        };
                        init
                    } as *mut wget_html_tag as *const libc::c_void,
                ) != -(1 as libc::c_int)
            {
                return;
            }
        }
        if *attr as libc::c_int | 0x20 as libc::c_int == 's' as i32
            && wget_strcasecmp_ascii(
                attr,
                b"style\0" as *const u8 as *const libc::c_char,
            ) == 0 && len != 0
        {
            (*ctx).css_dir = tag;
            (*ctx).css_attr = b"style\0" as *const u8 as *const libc::c_char;
            (*ctx)
                .css_start_offset = val.offset_from((*ctx).html) as libc::c_long
                as size_t;
            wget_css_parse_buffer(
                val,
                len,
                Some(
                    css_parse_uri
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const libc::c_char,
                            size_t,
                            size_t,
                        ) -> (),
                ),
                None,
                context,
            );
            return;
        }
        if *tag as libc::c_int | 0x20 as libc::c_int == 'l' as i32
            && wget_strcasecmp_ascii(tag, b"link\0" as *const u8 as *const libc::c_char)
                == 0
        {
            if wget_strcasecmp_ascii(attr, b"rel\0" as *const u8 as *const libc::c_char)
                == 0
            {
                (*ctx).link_inline = 0 as libc::c_int as libc::c_char;
                while len != 0 {
                    let mut p_0: *const libc::c_char = 0 as *const libc::c_char;
                    p_0 = val;
                    while len != 0 && !c_isspace(*val as libc::c_int) {
                        val = val.offset(1);
                        val;
                        len = len.wrapping_sub(1);
                        len;
                    }
                    if p_0 == val {
                        val = val.offset(1);
                        val;
                        len = len.wrapping_sub(1);
                        len;
                    } else {
                        if !(wget_strncasecmp_ascii(
                            p_0,
                            b"icon\0" as *const u8 as *const libc::c_char,
                            val.offset_from(p_0) as libc::c_long as size_t,
                        ) == 0
                            || wget_strncasecmp_ascii(
                                p_0,
                                b"manifest\0" as *const u8 as *const libc::c_char,
                                val.offset_from(p_0) as libc::c_long as size_t,
                            ) == 0
                            || wget_strncasecmp_ascii(
                                p_0,
                                b"modulepreload\0" as *const u8 as *const libc::c_char,
                                val.offset_from(p_0) as libc::c_long as size_t,
                            ) == 0
                            || wget_strncasecmp_ascii(
                                p_0,
                                b"stylesheet\0" as *const u8 as *const libc::c_char,
                                val.offset_from(p_0) as libc::c_long as size_t,
                            ) == 0
                            || wget_strncasecmp_ascii(
                                p_0,
                                b"prefetch\0" as *const u8 as *const libc::c_char,
                                val.offset_from(p_0) as libc::c_long as size_t,
                            ) == 0
                            || wget_strncasecmp_ascii(
                                p_0,
                                b"preload\0" as *const u8 as *const libc::c_char,
                                val.offset_from(p_0) as libc::c_long as size_t,
                            ) == 0)
                        {
                            continue;
                        }
                        (*ctx).link_inline = 1 as libc::c_int as libc::c_char;
                        break;
                    }
                }
                if (*ctx).uri_index >= 0 as libc::c_int {
                    let mut url: *mut wget_html_parsed_url = wget_vector_get(
                        (*res).uris,
                        (*ctx).uri_index,
                    ) as *mut wget_html_parsed_url;
                    if !url.is_null() {
                        (*url).set_link_inline((*ctx).link_inline != 0);
                    }
                }
                return;
            }
        }
        if *tag as libc::c_int | 0x20 as libc::c_int == 'a' as i32
            && (*tag.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int
                || wget_strcasecmp_ascii(
                    tag,
                    b"area\0" as *const u8 as *const libc::c_char,
                ) == 0)
            && wget_strcasecmp_ascii(
                attr,
                b"download\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if val.is_null() {
                return;
            }
            while len != 0 && c_isspace(*val as libc::c_int) as libc::c_int != 0 {
                val = val.offset(1);
                val;
                len = len.wrapping_sub(1);
                len;
            }
            while len != 0
                && c_isspace(
                    *val.offset(len.wrapping_sub(1 as libc::c_int as size_t) as isize)
                        as libc::c_int,
                ) as libc::c_int != 0
            {
                len = len.wrapping_sub(1);
                len;
            }
            if len == 0 {
                return;
            }
            (*ctx).download.p = val;
            (*ctx).download.len = len;
            if (*ctx).uri_index >= 0 as libc::c_int {
                let mut url_0: *mut wget_html_parsed_url = wget_vector_get(
                    (*res).uris,
                    (*ctx).uri_index,
                ) as *mut wget_html_parsed_url;
                (*url_0).download.p = val;
                (*url_0).download.len = len;
            }
            return;
        }
        let mut found: libc::c_int = 0 as libc::c_int;
        if maybe[(*attr as libc::c_uchar as libc::c_int | 0x20 as libc::c_int) as usize]
            as libc::c_int != 0
            && *attr.offset(1 as libc::c_int as isize) as libc::c_int != 0
            && *attr.offset(2 as libc::c_int as isize) as libc::c_int != 0
        {
            found = (bsearch(
                attr as *const libc::c_void,
                attrs.as_ptr() as *const libc::c_void,
                (::core::mem::size_of::<[[libc::c_char; 12]; 19]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                    ),
                ::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                ::core::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_char,
                            *const libc::c_char,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                    >,
                >(
                    Some(
                        wget_strcasecmp_ascii
                            as unsafe extern "C" fn(
                                *const libc::c_char,
                                *const libc::c_char,
                            ) -> libc::c_int,
                    ),
                ),
            ) != 0 as *mut libc::c_void) as libc::c_int;
        }
        if found == 0 && !((*ctx).additional_tags).is_null() {
            if wget_vector_find(
                (*ctx).additional_tags,
                &mut {
                    let mut init = wget_html_tag {
                        name: tag,
                        attribute: 0 as *const libc::c_char,
                    };
                    init
                } as *mut wget_html_tag as *const libc::c_void,
            ) != -(1 as libc::c_int)
                || wget_vector_find(
                    (*ctx).additional_tags,
                    &mut {
                        let mut init = wget_html_tag {
                            name: tag,
                            attribute: attr,
                        };
                        init
                    } as *mut wget_html_tag as *const libc::c_void,
                ) != -(1 as libc::c_int)
            {
                found = 1 as libc::c_int;
            }
        }
        if found != 0 {
            while len != 0 && c_isspace(*val as libc::c_int) as libc::c_int != 0 {
                val = val.offset(1);
                val;
                len = len.wrapping_sub(1);
                len;
            }
            while len != 0
                && c_isspace(
                    *val.offset(len.wrapping_sub(1 as libc::c_int as size_t) as isize)
                        as libc::c_int,
                ) as libc::c_int != 0
            {
                len = len.wrapping_sub(1);
                len;
            }
            if *tag as libc::c_int | 0x20 as libc::c_int == 'b' as i32
                && wget_strcasecmp_ascii(
                    tag,
                    b"base\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                (*res).base.p = val;
                (*res).base.len = len;
                return;
            }
            if ((*res).uris).is_null() {
                (*res).uris = wget_vector_create(32 as libc::c_int, None);
            }
            let mut url_1: wget_html_parsed_url = wget_html_parsed_url {
                url: wget_string {
                    p: 0 as *const libc::c_char,
                    len: 0,
                },
                download: wget_string {
                    p: 0 as *const libc::c_char,
                    len: 0,
                },
                attr: [0; 16],
                tag: [0; 16],
                link_inline: [0; 1],
                c2rust_padding: [0; 7],
            };
            if wget_strcasecmp_ascii(
                attr,
                b"srcset\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                while len != 0 {
                    let mut p_1: *const libc::c_char = 0 as *const libc::c_char;
                    while len != 0 && c_isspace(*val as libc::c_int) as libc::c_int != 0
                    {
                        val = val.offset(1);
                        val;
                        len = len.wrapping_sub(1);
                        len;
                    }
                    p_1 = val;
                    while len != 0 && !c_isspace(*val as libc::c_int)
                        && *val as libc::c_int != ',' as i32
                    {
                        val = val.offset(1);
                        val;
                        len = len.wrapping_sub(1);
                        len;
                    }
                    if p_1 != val {
                        if len != 0 && *val as libc::c_int == ',' as i32
                            && wget_strncasecmp_ascii(
                                p_1,
                                b"data:\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int as size_t,
                            ) == 0
                        {
                            val = val.offset(1);
                            val;
                            len = len.wrapping_sub(1);
                            len;
                            while len != 0 && !c_isspace(*val as libc::c_int)
                                && *val as libc::c_int != ',' as i32
                            {
                                val = val.offset(1);
                                val;
                                len = len.wrapping_sub(1);
                                len;
                            }
                        }
                        url_1.download.p = 0 as *const libc::c_char;
                        url_1.download.len = 0 as libc::c_int as size_t;
                        url_1.set_link_inline((*ctx).link_inline != 0);
                        wget_strscpy(
                            (url_1.attr).as_mut_ptr(),
                            attr,
                            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                        );
                        wget_strscpy(
                            (url_1.tag).as_mut_ptr(),
                            tag,
                            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                        );
                        url_1.url.p = p_1;
                        url_1.url.len = val.offset_from(p_1) as libc::c_long as size_t;
                        wget_vector_add_memdup(
                            (*res).uris,
                            &mut url_1 as *mut wget_html_parsed_url
                                as *const libc::c_void,
                            ::core::mem::size_of::<wget_html_parsed_url>()
                                as libc::c_ulong,
                        );
                    }
                    while len != 0 && *val as libc::c_int != ',' as i32 {
                        val = val.offset(1);
                        val;
                        len = len.wrapping_sub(1);
                        len;
                    }
                    if len != 0 && *val as libc::c_int == ',' as i32 {
                        val = val.offset(1);
                        val;
                        len = len.wrapping_sub(1);
                        len;
                    }
                }
            } else {
                url_1.download.p = (*ctx).download.p;
                url_1.download.len = (*ctx).download.len;
                url_1.set_link_inline((*ctx).link_inline != 0);
                wget_strscpy(
                    (url_1.attr).as_mut_ptr(),
                    attr,
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                );
                wget_strscpy(
                    (url_1.tag).as_mut_ptr(),
                    tag,
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                );
                url_1.url.p = val;
                url_1.url.len = len;
                (*ctx)
                    .uri_index = wget_vector_add_memdup(
                    (*res).uris,
                    &mut url_1 as *mut wget_html_parsed_url as *const libc::c_void,
                    ::core::mem::size_of::<wget_html_parsed_url>() as libc::c_ulong,
                );
            }
        }
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 && !val.is_null() && len != 0
        && wget_strcasecmp_ascii(tag, b"style\0" as *const u8 as *const libc::c_char)
            == 0
    {
        (*ctx).css_dir = b"style\0" as *const u8 as *const libc::c_char;
        (*ctx).css_attr = b"\0" as *const u8 as *const libc::c_char;
        (*ctx).css_start_offset = val.offset_from((*ctx).html) as libc::c_long as size_t;
        wget_css_parse_buffer(
            val,
            len,
            Some(
                css_parse_uri
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        size_t,
                    ) -> (),
            ),
            None,
            context,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_html_free_urls_inline(
    mut res: *mut *mut wget_html_parsed_result,
) {
    if !res.is_null() && !(*res).is_null() {
        if !((**res).encoding).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**res).encoding as *mut libc::c_void);
            (**res).encoding = 0 as *const libc::c_char;
        }
        wget_vector_free(&mut (**res).uris);
        if !(*res).is_null() {
            wget_free.expect("non-null function pointer")(*res as *mut libc::c_void);
            *res = 0 as *mut wget_html_parsed_result;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_html_get_urls_inline(
    mut html: *const libc::c_char,
    mut additional_tags: *mut wget_vector,
    mut ignore_tags: *mut wget_vector,
) -> *mut wget_html_parsed_result {
    let mut context: html_context = {
        let mut init = html_context {
            result: {
                let mut init = wget_html_parsed_result {
                    follow: [0; 1],
                    c2rust_padding: [0; 7],
                    uris: 0 as *mut wget_vector,
                    encoding: 0 as *const libc::c_char,
                    base: wget_string {
                        p: 0 as *const libc::c_char,
                        len: 0,
                    },
                };
                init.set_follow(1 as libc::c_int != 0);
                init
            },
            additional_tags: additional_tags,
            ignore_tags: ignore_tags,
            download: wget_string {
                p: 0 as *const libc::c_char,
                len: 0,
            },
            uri_index: 0,
            css_start_offset: 0,
            found_robots: 0,
            found_content_type: 0,
            link_inline: 0,
            html: html,
            css_attr: 0 as *const libc::c_char,
            css_dir: 0 as *const libc::c_char,
        };
        init
    };
    wget_html_parse_buffer(
        html,
        Some(
            html_get_url
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        &mut context as *mut html_context as *mut libc::c_void,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    return wget_memdup(
        &mut context.result as *mut wget_html_parsed_result as *const libc::c_void,
        ::core::mem::size_of::<wget_html_parsed_result>() as libc::c_ulong,
    ) as *mut wget_html_parsed_result;
}
