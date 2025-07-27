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
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_read_file(
        fname: *const libc::c_char,
        size: *mut size_t,
    ) -> *mut libc::c_char;
    fn wget_memiconv(
        src_encoding: *const libc::c_char,
        src: *const libc::c_void,
        srclen: size_t,
        dst_encoding: *const libc::c_char,
        out: *mut *mut libc::c_char,
        outlen: *mut size_t,
    ) -> libc::c_int;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_html_get_urls_inline(
        html: *const libc::c_char,
        additional_tags: *mut wget_vector,
        ignore_tags: *mut wget_vector,
    ) -> *mut wget_html_parsed_result;
    fn wget_html_free_urls_inline(res: *mut *mut wget_html_parsed_result);
}
pub type size_t = libc::c_ulong;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
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
unsafe extern "C" fn html_parse_localfile(mut fname: *const libc::c_char) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_allocated: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    data = wget_read_file(fname, &mut len);
    data_allocated = data;
    if !data_allocated.is_null() {
        let mut encoding: *const libc::c_char = 0 as *const libc::c_char;
        if *data.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xfe as libc::c_int
            && *data.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xff as libc::c_int
        {
            encoding = b"UTF-16BE\0" as *const u8 as *const libc::c_char;
            data = data.offset(2 as libc::c_int as isize);
            len = len.wrapping_sub(2 as libc::c_int as size_t);
        } else if *data.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xff as libc::c_int
            && *data.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xfe as libc::c_int
        {
            encoding = b"UTF-16LE\0" as *const u8 as *const libc::c_char;
            data = data.offset(2 as libc::c_int as isize);
            len = len.wrapping_sub(2 as libc::c_int as size_t);
        } else if *data.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xef as libc::c_int
            && *data.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xbb as libc::c_int
            && *data.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int
                == 0xbf as libc::c_int
        {
            encoding = b"UTF-8\0" as *const u8 as *const libc::c_char;
            data = data.offset(3 as libc::c_int as isize);
            len = len.wrapping_sub(3 as libc::c_int as size_t);
        }
        if !encoding.is_null() {
            printf(
                b"URI encoding '%s' set by BOM\n\0" as *const u8 as *const libc::c_char,
                encoding,
            );
        }
        if wget_strncasecmp_ascii(
            encoding,
            b"UTF-16\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        ) == 0
        {
            let mut n: size_t = 0;
            let mut utf8: *mut libc::c_char = 0 as *mut libc::c_char;
            len = len.wrapping_sub(len & 1 as libc::c_int as size_t);
            if wget_memiconv(
                encoding,
                data as *const libc::c_void,
                len,
                b"UTF-8\0" as *const u8 as *const libc::c_char,
                &mut utf8,
                &mut n,
            ) == 0 as libc::c_int
            {
                printf(
                    b"Convert non-ASCII encoding '%s' to UTF-8\n\0" as *const u8
                        as *const libc::c_char,
                    encoding,
                );
                if !data_allocated.is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(data_allocated as *mut libc::c_void);
                    data_allocated = 0 as *mut libc::c_char;
                }
                data = utf8;
                data_allocated = data;
            } else {
                printf(
                    b"Failed to convert non-ASCII encoding '%s' to UTF-8, skip parsing\n\0"
                        as *const u8 as *const libc::c_char,
                    encoding,
                );
                return;
            }
        }
        let mut res: *mut wget_html_parsed_result = wget_html_get_urls_inline(
            data,
            0 as *mut wget_vector,
            0 as *mut wget_vector,
        );
        if !encoding.is_null() {
            if !((*res).encoding).is_null()
                && wget_strcasecmp_ascii(encoding, (*res).encoding) != 0
            {
                printf(
                    b"Encoding '%s' as stated in document has been ignored\n\0"
                        as *const u8 as *const libc::c_char,
                    encoding,
                );
            }
        }
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < wget_vector_size((*res).uris) {
            let mut html_url: *mut wget_html_parsed_url = wget_vector_get(
                (*res).uris,
                it,
            ) as *mut wget_html_parsed_url;
            let mut url: *mut wget_string = &mut (*html_url).url;
            printf(
                b"  %s.%s '%.*s'\0" as *const u8 as *const libc::c_char,
                ((*html_url).tag).as_mut_ptr(),
                ((*html_url).attr).as_mut_ptr(),
                (*url).len as libc::c_int,
                (*url).p,
            );
            if !((*html_url).download.p).is_null() {
                printf(
                    b" (save as '%.*s')\0" as *const u8 as *const libc::c_char,
                    (*html_url).download.len as libc::c_int,
                    (*html_url).download.p,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            it += 1;
            it;
        }
        if !data_allocated.is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(data_allocated as *mut libc::c_void);
            data_allocated = 0 as *mut libc::c_char;
        }
        wget_html_free_urls_inline(&mut res);
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    if isatty(0 as libc::c_int) == 0 {
        html_parse_localfile(b"-\0" as *const u8 as *const libc::c_char);
    } else {
        let mut argpos: libc::c_int = 0;
        argpos = 1 as libc::c_int;
        while argpos < argc {
            printf(
                b"%s:\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(argpos as isize),
            );
            html_parse_localfile(*argv.offset(argpos as isize));
            argpos += 1;
            argpos;
        }
    }
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
