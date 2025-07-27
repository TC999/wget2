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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
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
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type C2RustUnnamed_0 = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed_0 = -12;
pub const WGET_E_IO: C2RustUnnamed_0 = -11;
pub const WGET_E_OPEN: C2RustUnnamed_0 = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed_0 = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed_0 = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed_0 = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed_0 = -6;
pub const WGET_E_CONNECT: C2RustUnnamed_0 = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed_0 = -4;
pub const WGET_E_INVALID: C2RustUnnamed_0 = -3;
pub const WGET_E_MEMORY: C2RustUnnamed_0 = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed_0 = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed_0 = 0;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_string {
    pub p: *const libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_robots_st {
    pub paths: *mut wget_vector,
    pub sitemaps: *mut wget_vector,
}
pub type wget_robots = wget_robots_st;
pub type record = libc::c_uint;
pub const NO_MORE_RECORDS: record = 4;
pub const ADDED_DISALLOW: record = 3;
pub const IN_RECORD_STAR: record = 2;
pub const IN_RECORD_CLIENT: record = 1;
pub const NOT_IN_RECORD: record = 0;
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
unsafe extern "C" fn path_free(mut path: *mut libc::c_void) {
    let mut p: *mut wget_string = path as *mut wget_string;
    if !((*p).p).is_null() {
        wget_free.expect("non-null function pointer")((*p).p as *mut libc::c_void);
        (*p).p = 0 as *const libc::c_char;
    }
    if !p.is_null() {
        wget_free.expect("non-null function pointer")(p as *mut libc::c_void);
        p = 0 as *mut wget_string;
    }
}
#[inline]
unsafe extern "C" fn advance_ws(mut s: *mut *const libc::c_char) {
    while *(*__ctype_b_loc()).offset(**s as libc::c_int as isize) as libc::c_int
        & _ISblank as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        *s = (*s).offset(1);
        *s;
    }
}
unsafe extern "C" fn parse_record_field(
    mut data: *mut *const libc::c_char,
    mut field: *const libc::c_char,
    mut field_length: size_t,
) -> bool {
    advance_ws(data);
    if wget_strncasecmp_ascii(*data, field, field_length) != 0 {
        return 0 as libc::c_int != 0;
    }
    *data = (*data).offset(field_length as isize);
    advance_ws(data);
    if **data as libc::c_int != ':' as i32 {
        return 0 as libc::c_int != 0;
    }
    *data = (*data).offset(1 as libc::c_int as isize);
    advance_ws(data);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn wget_robots_parse(
    mut _robots: *mut *mut wget_robots,
    mut data: *const libc::c_char,
    mut client: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut robots: *mut wget_robots = 0 as *mut wget_robots;
    let mut path: wget_string = wget_string {
        p: 0 as *const libc::c_char,
        len: 0,
    };
    let mut client_length: size_t = if !client.is_null() {
        strlen(client)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut seek_record_client: bool = 0 as libc::c_int != 0;
    let mut state: record = NOT_IN_RECORD;
    if data.is_null() || *data == 0 || _robots.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    robots = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_robots>() as libc::c_ulong,
    ) as *mut wget_robots;
    if robots.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    state = NOT_IN_RECORD;
    loop {
        if state as libc::c_uint != NO_MORE_RECORDS as libc::c_int as libc::c_uint
            && state as libc::c_uint != IN_RECORD_CLIENT as libc::c_int as libc::c_uint
            && parse_record_field(
                &mut data,
                b"User-agent\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int != 0
        {
            if !client.is_null()
                && wget_strncasecmp_ascii(data, client, client_length) == 0
            {
                if !seek_record_client {
                    wget_vector_free(&mut (*robots).paths);
                }
                seek_record_client = 1 as libc::c_int != 0;
                state = IN_RECORD_CLIENT;
            } else if !seek_record_client && *data as libc::c_int == '*' as i32 {
                state = IN_RECORD_STAR;
            } else if state as libc::c_uint
                == ADDED_DISALLOW as libc::c_int as libc::c_uint
            {
                state = NOT_IN_RECORD;
            }
        } else if state as libc::c_uint != NO_MORE_RECORDS as libc::c_int as libc::c_uint
            && state as libc::c_uint != NOT_IN_RECORD as libc::c_int as libc::c_uint
            && parse_record_field(
                &mut data,
                b"Disallow\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int != 0
        {
            if *data == 0
                || *(*__ctype_b_loc()).offset(*data as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *data as libc::c_int == '#' as i32
            {
                wget_vector_free(&mut (*robots).paths);
                if seek_record_client {
                    state = NO_MORE_RECORDS;
                } else {
                    state = NOT_IN_RECORD;
                    seek_record_client = 1 as libc::c_int != 0;
                }
            } else {
                if ((*robots).paths).is_null() {
                    (*robots).paths = wget_vector_create(32 as libc::c_int, None);
                    if ((*robots).paths).is_null() {
                        current_block = 11509306451286645182;
                        break;
                    }
                    wget_vector_set_destructor(
                        (*robots).paths,
                        Some(path_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                    );
                }
                p = data;
                while *p as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                    && *p as libc::c_int != '#' as i32
                {
                    p = p.offset(1);
                    p;
                }
                path.len = p.offset_from(data) as libc::c_long as size_t;
                path.p = wget_strmemdup(data as *const libc::c_void, path.len);
                if (path.p).is_null() {
                    current_block = 11509306451286645182;
                    break;
                }
                if wget_vector_add_memdup(
                    (*robots).paths,
                    &mut path as *mut wget_string as *const libc::c_void,
                    ::core::mem::size_of::<wget_string>() as libc::c_ulong,
                ) < 0 as libc::c_int
                {
                    if !(path.p).is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(path.p as *mut libc::c_void);
                        path.p = 0 as *const libc::c_char;
                    }
                    current_block = 11509306451286645182;
                    break;
                } else {
                    state = ADDED_DISALLOW;
                }
            }
        } else if parse_record_field(
            &mut data,
            b"Sitemap\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) {
            p = data;
            while *p as libc::c_int != 0
                && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                && *p as libc::c_int != '#' as i32
            {
                p = p.offset(1);
                p;
            }
            if p > data {
                if ((*robots).sitemaps).is_null() {
                    (*robots).sitemaps = wget_vector_create(4 as libc::c_int, None);
                    if ((*robots).sitemaps).is_null() {
                        current_block = 11509306451286645182;
                        break;
                    }
                }
                let mut sitemap: *mut libc::c_char = wget_strmemdup(
                    data as *const libc::c_void,
                    p.offset_from(data) as libc::c_long as size_t,
                );
                if sitemap.is_null() {
                    current_block = 11509306451286645182;
                    break;
                }
                if wget_vector_add((*robots).sitemaps, sitemap as *const libc::c_void)
                    < 0 as libc::c_int
                {
                    current_block = 11509306451286645182;
                    break;
                }
            }
        }
        data = strchr(data, '\n' as i32);
        if !data.is_null() {
            data = data.offset(1);
            data;
        }
        if !(!data.is_null() && *data as libc::c_int != 0) {
            current_block = 7990025728955927862;
            break;
        }
    }
    match current_block {
        11509306451286645182 => {
            wget_robots_free(&mut robots);
            return WGET_E_MEMORY as libc::c_int;
        }
        _ => {
            *_robots = robots;
            return WGET_E_SUCCESS as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_robots_free(mut robots: *mut *mut wget_robots) {
    if !robots.is_null() && !(*robots).is_null() {
        wget_vector_free(&mut (**robots).paths);
        wget_vector_free(&mut (**robots).sitemaps);
        if !(*robots).is_null() {
            wget_free.expect("non-null function pointer")(*robots as *mut libc::c_void);
            *robots = 0 as *mut wget_robots;
        }
        *robots = 0 as *mut wget_robots;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_robots_get_path_count(
    mut robots: *mut wget_robots,
) -> libc::c_int {
    if !robots.is_null() {
        return wget_vector_size((*robots).paths);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_robots_get_path(
    mut robots: *mut wget_robots,
    mut index: libc::c_int,
) -> *mut wget_string {
    if !robots.is_null() && !((*robots).paths).is_null() {
        return wget_vector_get((*robots).paths, index) as *mut wget_string;
    }
    return 0 as *mut wget_string;
}
#[no_mangle]
pub unsafe extern "C" fn wget_robots_get_sitemap_count(
    mut robots: *mut wget_robots,
) -> libc::c_int {
    if !robots.is_null() {
        return wget_vector_size((*robots).sitemaps);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_robots_get_sitemap(
    mut robots: *mut wget_robots,
    mut index: libc::c_int,
) -> *const libc::c_char {
    if !robots.is_null() && !((*robots).sitemaps).is_null() {
        return wget_vector_get((*robots).sitemaps, index) as *const libc::c_char;
    }
    return 0 as *const libc::c_char;
}
