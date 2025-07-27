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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_xml_parse_buffer(
        buf: *const libc::c_char,
        callback: Option::<wget_xml_callback>,
        user_ctx: *mut libc::c_void,
        hints: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_string {
    pub p: *const libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct atom_context {
    pub urls: *mut wget_vector,
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
unsafe extern "C" fn atom_get_url(
    mut context: *mut libc::c_void,
    mut flags: libc::c_int,
    mut dir: *const libc::c_char,
    mut attr: *const libc::c_char,
    mut val: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut atom_context = context as *mut atom_context;
    let mut url: *mut wget_string = 0 as *mut wget_string;
    if val.is_null() || len == 0 {
        return;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        if wget_strcasecmp_ascii(attr, b"href\0" as *const u8 as *const libc::c_char)
            == 0
            || wget_strcasecmp_ascii(attr, b"uri\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(attr, b"src\0" as *const u8 as *const libc::c_char)
                == 0
            || wget_strcasecmp_ascii(
                attr,
                b"scheme\0" as *const u8 as *const libc::c_char,
            ) == 0
            || wget_strcasecmp_ascii(
                attr,
                b"xmlns\0" as *const u8 as *const libc::c_char,
            ) == 0
            || wget_strncasecmp_ascii(
                attr,
                b"xmlns:\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
            ) == 0
        {
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
            url = wget_malloc(::core::mem::size_of::<wget_string>() as libc::c_ulong)
                as *mut wget_string;
            if url.is_null() {
                return;
            }
            (*url).p = val;
            (*url).len = len;
            if ((*ctx).urls).is_null() {
                (*ctx).urls = wget_vector_create(32 as libc::c_int, None);
            }
            wget_vector_add((*ctx).urls, url as *const libc::c_void);
        }
    } else if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        let mut elem: *const libc::c_char = strrchr(dir, '/' as i32);
        if !elem.is_null() {
            elem = elem.offset(1);
            elem;
            if wget_strcasecmp_ascii(elem, b"icon\0" as *const u8 as *const libc::c_char)
                == 0
                || wget_strcasecmp_ascii(
                    elem,
                    b"id\0" as *const u8 as *const libc::c_char,
                ) == 0
                || wget_strcasecmp_ascii(
                    elem,
                    b"logo\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                while len != 0 && c_isspace(*val as libc::c_int) as libc::c_int != 0 {
                    val = val.offset(1);
                    val;
                    len = len.wrapping_sub(1);
                    len;
                }
                while len != 0
                    && c_isspace(
                        *val
                            .offset(
                                len.wrapping_sub(1 as libc::c_int as size_t) as isize,
                            ) as libc::c_int,
                    ) as libc::c_int != 0
                {
                    len = len.wrapping_sub(1);
                    len;
                }
                url = wget_malloc(::core::mem::size_of::<wget_string>() as libc::c_ulong)
                    as *mut wget_string;
                if url.is_null() {
                    return;
                }
                (*url).p = val;
                (*url).len = len;
                if ((*ctx).urls).is_null() {
                    (*ctx).urls = wget_vector_create(32 as libc::c_int, None);
                }
                wget_vector_add((*ctx).urls, url as *const libc::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_atom_get_urls_inline(
    mut atom: *const libc::c_char,
    mut urls: *mut *mut wget_vector,
) {
    let mut context: atom_context = {
        let mut init = atom_context {
            urls: 0 as *mut wget_vector,
        };
        init
    };
    wget_xml_parse_buffer(
        atom,
        Some(
            atom_get_url
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
        &mut context as *mut atom_context as *mut libc::c_void,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    *urls = context.urls;
}
