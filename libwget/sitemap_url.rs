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
pub struct sitemap_context {
    pub sitemap_urls: *mut wget_vector,
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
unsafe extern "C" fn sitemap_get_url(
    mut context: *mut libc::c_void,
    mut flags: libc::c_int,
    mut dir: *const libc::c_char,
    mut attr: *const libc::c_char,
    mut val: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut sitemap_context = context as *mut sitemap_context;
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 && len != 0 {
        let mut type_0: libc::c_int = 0;
        if wget_strcasecmp_ascii(
            dir,
            b"/sitemapindex/sitemap/loc\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            type_0 = 1 as libc::c_int;
        } else if wget_strcasecmp_ascii(
            dir,
            b"/urlset/url/loc\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            type_0 = 2 as libc::c_int;
        } else {
            type_0 = 0 as libc::c_int;
        }
        if type_0 != 0 {
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
            let mut url: *mut wget_string = 0 as *mut wget_string;
            url = wget_malloc(::core::mem::size_of::<wget_string>() as libc::c_ulong)
                as *mut wget_string;
            if url.is_null() {
                return;
            }
            (*url).p = val;
            (*url).len = len;
            if type_0 == 1 as libc::c_int {
                if ((*ctx).sitemap_urls).is_null() {
                    (*ctx).sitemap_urls = wget_vector_create(32 as libc::c_int, None);
                }
                wget_vector_add((*ctx).sitemap_urls, url as *const libc::c_void);
            } else {
                if ((*ctx).urls).is_null() {
                    (*ctx).urls = wget_vector_create(32 as libc::c_int, None);
                }
                wget_vector_add((*ctx).urls, url as *const libc::c_void);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_sitemap_get_urls_inline(
    mut sitemap: *const libc::c_char,
    mut urls: *mut *mut wget_vector,
    mut sitemap_urls: *mut *mut wget_vector,
) {
    let mut context: sitemap_context = {
        let mut init = sitemap_context {
            sitemap_urls: 0 as *mut wget_vector,
            urls: 0 as *mut wget_vector,
        };
        init
    };
    wget_xml_parse_buffer(
        sitemap,
        Some(
            sitemap_get_url
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
        &mut context as *mut sitemap_context as *mut libc::c_void,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    *urls = context.urls;
    *sitemap_urls = context.sitemap_urls;
}
