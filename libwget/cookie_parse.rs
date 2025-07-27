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
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strtolower(s: *mut libc::c_char) -> *mut libc::c_char;
    fn wget_strncmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_http_parse_token(
        s: *const libc::c_char,
        token: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn wget_http_parse_full_date(s: *const libc::c_char) -> int64_t;
    fn wget_http_print_date(
        t: int64_t,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_cookie_st {
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
    pub domain: *const libc::c_char,
    pub path: *const libc::c_char,
    pub expires: int64_t,
    pub maxage: int64_t,
    pub last_access: int64_t,
    pub creation: int64_t,
    pub sort_age: libc::c_uint,
    #[bitfield(name = "domain_dot", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "normalized", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "persistent", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "host_only", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "secure_only", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "http_only", ty = "bool", bits = "5..=5")]
    pub domain_dot_normalized_persistent_host_only_secure_only_http_only: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type wget_cookie = wget_cookie_st;
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn c_isspace(mut c: libc::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
#[no_mangle]
pub unsafe extern "C" fn cookie_domain_match(
    mut domain: *const libc::c_char,
    mut host: *const libc::c_char,
) -> bool {
    let mut domain_length: size_t = 0;
    let mut host_length: size_t = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    wget_debug_printf(
        b"domain_match(%s,%s)\0" as *const u8 as *const libc::c_char,
        domain,
        host,
    );
    if strcmp(domain, host) == 0 {
        return 1 as libc::c_int != 0;
    }
    domain_length = strlen(domain);
    host_length = strlen(host);
    if domain_length >= host_length {
        return 0 as libc::c_int != 0;
    }
    p = host.offset(host_length as isize).offset(-(domain_length as isize));
    if strcmp(p, domain) == 0
        && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '.' as i32
    {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn cookie_path_match(
    mut cookie_path: *const libc::c_char,
    mut request_path: *const libc::c_char,
) -> bool {
    let mut last_slash: *const libc::c_char = 0 as *const libc::c_char;
    let mut cookie_path_length: size_t = 0;
    let mut iri_path_length: size_t = 0;
    let mut cookie_path_slash: bool = 0 as libc::c_int != 0;
    if *cookie_path as libc::c_int == '/' as i32 {
        cookie_path = cookie_path.offset(1);
        cookie_path;
        cookie_path_slash = 1 as libc::c_int != 0;
    }
    if !request_path.is_null() && *request_path as libc::c_int == '/' as i32 {
        request_path = request_path.offset(1);
        request_path;
    }
    wget_debug_printf(
        b"path_match(/%s,/%s)\n\0" as *const u8 as *const libc::c_char,
        cookie_path,
        if !request_path.is_null() {
            request_path
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    if request_path.is_null()
        || {
            last_slash = strrchr(request_path, '/' as i32);
            last_slash.is_null()
        }
    {
        request_path = b"\0" as *const u8 as *const libc::c_char;
        iri_path_length = 0 as libc::c_int as size_t;
    } else {
        iri_path_length = last_slash.offset_from(request_path) as libc::c_long as size_t;
    }
    cookie_path_length = strlen(cookie_path);
    if iri_path_length < cookie_path_length {
        return 0 as libc::c_int != 0;
    }
    if iri_path_length == 0 as libc::c_int as size_t
        && cookie_path_length == 0 as libc::c_int as size_t
    {
        return 1 as libc::c_int != 0;
    }
    if strncmp(cookie_path, request_path, cookie_path_length) == 0 {
        if *request_path.offset(cookie_path_length as isize) == 0 {
            return 1 as libc::c_int != 0;
        }
        if cookie_path_length > 0 as libc::c_int as size_t
            && *cookie_path
                .offset(
                    cookie_path_length.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as libc::c_int == '/' as i32 || cookie_path_slash as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        if *request_path.offset(cookie_path_length as isize) as libc::c_int == '/' as i32
        {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_init(
    mut cookie: *mut wget_cookie,
) -> *mut wget_cookie {
    if cookie.is_null() {
        cookie = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_cookie>() as libc::c_ulong,
        ) as *mut wget_cookie;
        if cookie.is_null() {
            return 0 as *mut wget_cookie;
        }
    } else {
        memset(
            cookie as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_cookie>() as libc::c_ulong,
        );
    }
    (*cookie).creation = time(0 as *mut time_t);
    (*cookie).last_access = (*cookie).creation;
    return cookie;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_deinit(mut cookie: *mut wget_cookie) {
    if !cookie.is_null() {
        if !((*cookie).name).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*cookie).name as *mut libc::c_void);
            (*cookie).name = 0 as *const libc::c_char;
        }
        if !((*cookie).value).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*cookie).value as *mut libc::c_void);
            (*cookie).value = 0 as *const libc::c_char;
        }
        if !((*cookie).domain).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*cookie).domain as *mut libc::c_void);
            (*cookie).domain = 0 as *const libc::c_char;
        }
        if !((*cookie).path).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*cookie).path as *mut libc::c_void);
            (*cookie).path = 0 as *const libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_free(mut cookie: *mut *mut wget_cookie) {
    if !cookie.is_null() {
        wget_cookie_deinit(*cookie);
        if !(*cookie).is_null() {
            wget_free.expect("non-null function pointer")(*cookie as *mut libc::c_void);
            *cookie = 0 as *mut wget_cookie;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cookie_free(mut cookie: *mut libc::c_void) {
    if !cookie.is_null() {
        wget_cookie_deinit(cookie as *mut wget_cookie);
        if !cookie.is_null() {
            wget_free.expect("non-null function pointer")(cookie);
            cookie = 0 as *mut libc::c_void;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_to_setcookie(
    mut cookie: *mut wget_cookie,
) -> *mut libc::c_char {
    let mut expires: [libc::c_char; 32] = *::core::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    if cookie.is_null() {
        return wget_strdup(b"(null)\0" as *const u8 as *const libc::c_char);
    }
    if (*cookie).expires != 0 {
        wget_http_print_date(
            (*cookie).expires,
            expires.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        );
    }
    return wget_aprintf(
        b"%s=%s%s%s%s%s; domain=%s%s%s%s\0" as *const u8 as *const libc::c_char,
        (*cookie).name,
        (*cookie).value,
        if *expires.as_mut_ptr() as libc::c_int != 0 {
            b"; expires=\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if *expires.as_mut_ptr() as libc::c_int != 0 {
            expires.as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !((*cookie).path).is_null() {
            b"; path=\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if !((*cookie).path).is_null() {
            (*cookie).path
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if (*cookie).host_only() as libc::c_int != 0 {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b".\0" as *const u8 as *const libc::c_char
        },
        (*cookie).domain,
        if (*cookie).http_only() as libc::c_int != 0 {
            b"; HttpOnly\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        if (*cookie).secure_only() as libc::c_int != 0 {
            b"; Secure\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_parse_setcookie(
    mut s: *const libc::c_char,
    mut _cookie: *mut *mut wget_cookie,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut cookie: *mut wget_cookie = wget_cookie_init(0 as *mut wget_cookie);
    while c_isspace(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    p = s;
    while *s as libc::c_int >= 32 as libc::c_int
        && *s as libc::c_int <= 126 as libc::c_int && *s as libc::c_int != '=' as i32
        && *s as libc::c_int != ';' as i32 || (*s as libc::c_int) < 0 as libc::c_int
    {
        s = s.offset(1);
        s;
    }
    while s > p
        && c_isspace(*s.offset(-(1 as libc::c_int) as isize) as libc::c_int)
            as libc::c_int != 0
    {
        s = s.offset(-1);
        s;
    }
    (*cookie)
        .name = wget_strmemdup(
        p as *const libc::c_void,
        s.offset_from(p) as libc::c_long as size_t,
    );
    while c_isspace(*s as libc::c_int) {
        s = s.offset(1);
        s;
    }
    if !((*cookie).name).is_null() && *(*cookie).name as libc::c_int != 0
        && *s as libc::c_int == '=' as i32
    {
        s = s.offset(1);
        s;
        while c_isspace(*s as libc::c_int) {
            s = s.offset(1);
            s;
        }
        p = s;
        while *s as libc::c_int >= 32 as libc::c_int
            && *s as libc::c_int <= 126 as libc::c_int && *s as libc::c_int != ';' as i32
            || (*s as libc::c_int) < 0 as libc::c_int
        {
            s = s.offset(1);
            s;
        }
        while s > p
            && c_isspace(*s.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                as libc::c_int != 0
        {
            s = s.offset(-1);
            s;
        }
        (*cookie)
            .value = wget_strmemdup(
            p as *const libc::c_void,
            s.offset_from(p) as libc::c_long as size_t,
        );
        loop {
            while *s as libc::c_int != 0 && *s as libc::c_int != ';' as i32 {
                s = s.offset(1);
                s;
            }
            if *s == 0 {
                break;
            }
            s = s.offset(1);
            s;
            while c_isspace(*s as libc::c_int) {
                s = s.offset(1);
                s;
            }
            if *s == 0 {
                break;
            }
            s = wget_http_parse_token(s, &mut name);
            if !name.is_null() {
                while *s as libc::c_int != 0 && *s as libc::c_int != '=' as i32
                    && *s as libc::c_int != ';' as i32
                {
                    s = s.offset(1);
                    s;
                }
                if *s as libc::c_int == '=' as i32 {
                    s = s.offset(1);
                    s;
                    while c_isspace(*s as libc::c_int) {
                        s = s.offset(1);
                        s;
                    }
                    p = s;
                    while *s as libc::c_int >= 32 as libc::c_int
                        && *s as libc::c_int <= 126 as libc::c_int
                        && *s as libc::c_int != ';' as i32
                        || (*s as libc::c_int) < 0 as libc::c_int
                    {
                        s = s.offset(1);
                        s;
                    }
                    if wget_strcasecmp_ascii(
                        name,
                        b"expires\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*cookie).expires = wget_http_parse_full_date(p);
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"max-age\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        let mut offset: libc::c_long = atol(p);
                        if offset > 0 as libc::c_int as libc::c_long {
                            if offset > 2147483647 as libc::c_int as libc::c_long {
                                offset = 2147483647 as libc::c_int as libc::c_long;
                            }
                            (*cookie).maxage = time(0 as *mut time_t) + offset;
                        } else {
                            (*cookie).maxage = 0 as libc::c_int as int64_t;
                        }
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"domain\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if p != s {
                            if *p as libc::c_int == '.' as i32 {
                                loop {
                                    p = p.offset(1);
                                    p;
                                    if !(*p as libc::c_int == '.' as i32) {
                                        break;
                                    }
                                }
                                (*cookie).set_domain_dot(1 as libc::c_int != 0);
                            } else {
                                (*cookie).set_domain_dot(0 as libc::c_int != 0);
                            }
                            while s > p
                                && c_isspace(
                                    *s.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                                ) as libc::c_int != 0
                            {
                                s = s.offset(-1);
                                s;
                            }
                            if !((*cookie).domain).is_null() {
                                wget_free
                                    .expect(
                                        "non-null function pointer",
                                    )((*cookie).domain as *mut libc::c_void);
                                (*cookie).domain = 0 as *const libc::c_char;
                            }
                            (*cookie)
                                .domain = wget_strmemdup(
                                p as *const libc::c_void,
                                s.offset_from(p) as libc::c_long as size_t,
                            );
                        }
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"path\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        while s > p
                            && c_isspace(
                                *s.offset(-(1 as libc::c_int) as isize) as libc::c_int,
                            ) as libc::c_int != 0
                        {
                            s = s.offset(-1);
                            s;
                        }
                        if !((*cookie).path).is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )((*cookie).path as *mut libc::c_void);
                            (*cookie).path = 0 as *const libc::c_char;
                        }
                        (*cookie)
                            .path = wget_strmemdup(
                            p as *const libc::c_void,
                            s.offset_from(p) as libc::c_long as size_t,
                        );
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"secure\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*cookie).set_secure_only(1 as libc::c_int != 0);
                    } else if wget_strcasecmp_ascii(
                        name,
                        b"httponly\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*cookie).set_http_only(1 as libc::c_int != 0);
                    } else {
                        wget_debug_printf(
                            b"Unsupported cookie-av '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            name,
                        );
                    }
                } else if wget_strcasecmp_ascii(
                    name,
                    b"secure\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*cookie).set_secure_only(1 as libc::c_int != 0);
                } else if wget_strcasecmp_ascii(
                    name,
                    b"httponly\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*cookie).set_http_only(1 as libc::c_int != 0);
                } else {
                    wget_debug_printf(
                        b"Unsupported cookie-av '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        name,
                    );
                }
                if !name.is_null() {
                    wget_free
                        .expect("non-null function pointer")(name as *mut libc::c_void);
                    name = 0 as *const libc::c_char;
                }
            }
            if !(*s != 0) {
                break;
            }
        }
    } else {
        wget_cookie_free(&mut cookie);
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Cookie without name or assignment ignored\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !_cookie.is_null() {
        *_cookie = cookie;
    } else {
        wget_cookie_free(&mut cookie);
    }
    return s;
}
unsafe extern "C" fn cookie_normalize_cookie(
    mut iri: *const wget_iri,
    mut cookie: *mut wget_cookie,
) -> libc::c_int {
    if cookie.is_null() {
        return -(1 as libc::c_int);
    }
    (*cookie).set_normalized(0 as libc::c_int != 0);
    if (*cookie).maxage != 0 {
        (*cookie).expires = (*cookie).maxage;
    }
    (*cookie).set_persistent((*cookie).expires != 0 as libc::c_int as int64_t);
    wget_strtolower((*cookie).domain as *mut libc::c_char);
    if !iri.is_null() {
        if wget_strncmp(
            (*cookie).name,
            b"__Secure-\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        ) == 0
        {
            if !(*cookie).secure_only()
                || (*iri).scheme as libc::c_uint
                    != WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
            {
                wget_debug_printf(
                    b"Cookie prefix requires secure origin: %s %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).name,
                    (*iri).host,
                );
                return -(1 as libc::c_int);
            }
        } else if wget_strncmp(
            (*cookie).name,
            b"__Host-\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        ) == 0
        {
            if !(*cookie).secure_only()
                || (*iri).scheme as libc::c_uint
                    != WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
            {
                wget_debug_printf(
                    b"Cookie prefix requires secure origin: %s %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).name,
                    (*iri).host,
                );
                return -(1 as libc::c_int);
            }
            if !(*cookie).host_only() {
                wget_debug_printf(
                    b"Cookie prefix requires hostonly flag: %s %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).name,
                    (*iri).host,
                );
                return -(1 as libc::c_int);
            }
            if wget_strcmp((*cookie).path, b"/\0" as *const u8 as *const libc::c_char)
                != 0
            {
                wget_debug_printf(
                    b"Cookie prefix requires path \"/\": %s %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*cookie).name,
                    (*iri).host,
                );
                return -(1 as libc::c_int);
            }
        }
        if !((*cookie).domain).is_null() && *(*cookie).domain as libc::c_int != 0 {
            if strcmp((*cookie).domain, (*iri).host) == 0 {
                (*cookie).set_host_only(1 as libc::c_int != 0);
            } else if cookie_domain_match((*cookie).domain, (*iri).host) {
                (*cookie).set_host_only(0 as libc::c_int != 0);
            } else {
                wget_debug_printf(
                    b"Domain mismatch: %s %s\n\0" as *const u8 as *const libc::c_char,
                    (*cookie).domain,
                    (*iri).host,
                );
                return -(1 as libc::c_int);
            }
        } else {
            if !((*cookie).domain).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*cookie).domain as *mut libc::c_void);
                (*cookie).domain = 0 as *const libc::c_char;
            }
            (*cookie).domain = wget_strdup((*iri).host);
            (*cookie).set_host_only(1 as libc::c_int != 0);
        }
        if ((*cookie).path).is_null() || *(*cookie).path as libc::c_int != '/' as i32 {
            let mut p: *const libc::c_char = if !((*iri).path).is_null() {
                strrchr((*iri).path, '/' as i32)
            } else {
                0 as *mut libc::c_char
            };
            if !((*cookie).path).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*cookie).path as *mut libc::c_void);
                (*cookie).path = 0 as *const libc::c_char;
            }
            if !p.is_null() && p != (*iri).path {
                (*cookie)
                    .path = wget_strmemdup(
                    (*iri).path as *const libc::c_void,
                    p.offset_from((*iri).path) as libc::c_long as size_t,
                );
            } else {
                (*cookie).path = wget_strdup(b"/\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    (*cookie).set_normalized(1 as libc::c_int != 0);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_normalize(
    mut iri: *const wget_iri,
    mut cookie: *mut wget_cookie,
) -> libc::c_int {
    let mut ret: libc::c_int = cookie_normalize_cookie(iri, cookie);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_cookie_normalize_cookies(
    mut iri: *const wget_iri,
    mut cookies: *const wget_vector,
) {
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(cookies) {
        cookie_normalize_cookie(iri, wget_vector_get(cookies, it) as *mut wget_cookie);
        it += 1;
        it;
    }
}
