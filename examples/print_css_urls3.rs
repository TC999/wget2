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
    pub type wget_logger_st;
    pub type wget_vector_st;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn wget_local_charset_encoding() -> *const libc::c_char;
    fn wget_logger_set_stream(logger: *mut wget_logger, fp: *mut FILE);
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_css_get_urls_from_localfile(
        fname: *const libc::c_char,
        base: *mut wget_iri,
        encoding: *mut *const libc::c_char,
    ) -> *mut wget_vector;
}
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type uint16_t = __uint16_t;
pub type wget_logger = wget_logger_st;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_css_parsed_url_st {
    pub len: size_t,
    pub pos: size_t,
    pub url: *const libc::c_char,
    pub abs_url: *const libc::c_char,
}
pub type wget_css_parsed_url = wget_css_parsed_url_st;
unsafe extern "C" fn usage(mut myname: *const libc::c_char) -> ! {
    wget_error_printf_exit(
        b"\nUsage: %s [options] file...\n  --base <URI>          Default base for relative URIs, default: http://www.example.com\n  --encoding <Encoding> Default file character encoding, default: iso-8859-1\n\n  Examples:\n    %s --base http://www.mydomain.com x.css\n    cat x.css | %s --base http://www.mydomain.com -\n    %s http://www.example.com\n\n  Print URIs as found (without a base):\n    %s --base \"\" x.css\n\n\0"
            as *const u8 as *const libc::c_char,
        myname,
        myname,
        myname,
        myname,
        myname,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut base: *const libc::c_char = 0 as *const libc::c_char;
    let mut local_encoding: *const libc::c_char = wget_local_charset_encoding();
    let mut base_uri: *mut wget_iri = 0 as *mut wget_iri;
    let mut css_encoding: *const libc::c_char = 0 as *const libc::c_char;
    let mut argpos: libc::c_int = 0;
    wget_logger_set_stream(wget_get_logger(2 as libc::c_int), stderr);
    wget_logger_set_stream(wget_get_logger(1 as libc::c_int), stdout);
    argpos = 1 as libc::c_int;
    while argpos < argc {
        if strcmp(
            *argv.offset(argpos as isize),
            b"--base\0" as *const u8 as *const libc::c_char,
        ) == 0 && argc - argpos > 1 as libc::c_int
        {
            argpos += 1;
            base = *argv.offset(argpos as isize);
            wget_info_printf(
                b"Local URI encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
                local_encoding,
            );
        } else if strcmp(
            *argv.offset(argpos as isize),
            b"--encoding\0" as *const u8 as *const libc::c_char,
        ) == 0 && argc - argpos > 1 as libc::c_int
        {
            argpos += 1;
            css_encoding = *argv.offset(argpos as isize);
        } else if strcmp(
            *argv.offset(argpos as isize),
            b"--\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            argpos += 1;
            argpos;
            break;
        } else {
            if !(*(*argv.offset(argpos as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32)
            {
                break;
            }
            usage(*argv.offset(0 as libc::c_int as isize));
        }
        argpos += 1;
        argpos;
    }
    base_uri = if !base.is_null() {
        wget_iri_parse(base, local_encoding)
    } else {
        0 as *mut wget_iri
    };
    while argpos < argc {
        let mut css_urls: *mut wget_vector = wget_css_get_urls_from_localfile(
            *argv.offset(argpos as isize),
            base_uri,
            &mut css_encoding,
        );
        if wget_vector_size(css_urls) > 0 as libc::c_int {
            wget_info_printf(
                b"URL encoding for %s is '%s':\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(argpos as isize),
                if !css_encoding.is_null() {
                    css_encoding
                } else {
                    b"UTF-8\0" as *const u8 as *const libc::c_char
                },
            );
            let mut it: libc::c_int = 0 as libc::c_int;
            while it < wget_vector_size(css_urls) {
                let mut css_url: *mut wget_css_parsed_url = wget_vector_get(css_urls, it)
                    as *mut wget_css_parsed_url;
                if !((*css_url).abs_url).is_null() {
                    wget_info_printf(
                        b"  %s -> %s\n\0" as *const u8 as *const libc::c_char,
                        (*css_url).url,
                        (*css_url).abs_url,
                    );
                } else {
                    wget_info_printf(
                        b"  %s\n\0" as *const u8 as *const libc::c_char,
                        (*css_url).url,
                    );
                }
                it += 1;
                it;
            }
            wget_info_printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        wget_vector_free(&mut css_urls);
        argpos += 1;
        argpos;
    }
    wget_iri_free(&mut base_uri);
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
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
