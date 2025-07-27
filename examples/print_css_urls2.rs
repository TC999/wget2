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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_local_charset_encoding() -> *const libc::c_char;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_logger_set_stream(logger: *mut wget_logger, fp: *mut FILE);
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
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
    fn wget_css_parse_file(
        fname: *const libc::c_char,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
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
pub type wget_logger = wget_logger_st;
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
    pub base: *mut wget_iri,
    pub encoding: *const libc::c_char,
    pub uri_buf: wget_buffer,
    pub encoding_allocated: libc::c_char,
}
unsafe extern "C" fn usage(mut myname: *const libc::c_char) -> ! {
    wget_error_printf_exit(
        b"\nUsage: %s [options] file...\n  --base <URI>          Default base for relative URIs, default: http://www.example.com\n  --encoding <Encoding> Default file character encoding, default: iso-8859-1\n\n  Examples:\n    %s --base http://www.mydomain.com x.css\n    cat x.css | %s --base http://www.mydomain.com -\n\n  Print URIs as found (without a base):\n    %s --base \"\" x.css\n\n\0"
            as *const u8 as *const libc::c_char,
        myname,
        myname,
        myname,
        myname,
    );
}
unsafe extern "C" fn css_parse_encoding(
    mut context: *mut libc::c_void,
    mut encoding: *const libc::c_char,
    mut len: size_t,
) {
    let mut ctx: *mut css_context = context as *mut css_context;
    if (*ctx).encoding_allocated == 0
        && wget_strncasecmp_ascii((*ctx).encoding, encoding, len) != 0
    {
        if !((*ctx).encoding).is_null() {
            wget_info_printf(
                b"Encoding changed from '%s' to '%.*s'\n\0" as *const u8
                    as *const libc::c_char,
                (*ctx).encoding,
                len as libc::c_int,
                encoding,
            );
        } else {
            wget_info_printf(
                b"Encoding set to '%.*s'\n\0" as *const u8 as *const libc::c_char,
                len as libc::c_int,
                encoding,
            );
        }
        (*ctx).encoding = wget_strmemdup(encoding as *const libc::c_void, len);
        (*ctx).encoding_allocated = 1 as libc::c_int as libc::c_char;
    }
}
unsafe extern "C" fn css_parse_uri(
    mut context: *mut libc::c_void,
    mut url: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut css_context = context as *mut css_context;
    if ((*ctx).base).is_null() {
        wget_info_printf(
            b"  %.*s\n\0" as *const u8 as *const libc::c_char,
            len as libc::c_int,
            url,
        );
    } else if !(wget_iri_relative_to_abs((*ctx).base, url, len, &mut (*ctx).uri_buf))
        .is_null()
    {
        wget_info_printf(
            b"  %.*s -> %s\n\0" as *const u8 as *const libc::c_char,
            len as libc::c_int,
            url,
            (*ctx).uri_buf.data,
        );
    } else {
        wget_error_printf(
            b"Cannot resolve relative URI %.*s\n\0" as *const u8 as *const libc::c_char,
            len as libc::c_int,
            url,
        );
    };
}
unsafe extern "C" fn css_parse_localfile(
    mut fname: *const libc::c_char,
    mut base: *mut wget_iri,
    mut encoding: *const libc::c_char,
) {
    let mut context: css_context = {
        let mut init = css_context {
            base: base,
            encoding: encoding,
            uri_buf: wget_buffer {
                data: 0 as *mut libc::c_char,
                length: 0,
                size: 0,
                release_data_release_buf_error: [0; 1],
                c2rust_padding: [0; 7],
            },
            encoding_allocated: 0,
        };
        init
    };
    wget_buffer_init(
        &mut context.uri_buf,
        0 as *mut libc::c_char,
        128 as libc::c_int as size_t,
    );
    wget_css_parse_file(
        fname,
        Some(
            css_parse_uri
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        Some(
            css_parse_encoding
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                ) -> (),
        ),
        &mut context as *mut css_context as *mut libc::c_void,
    );
    if context.encoding_allocated != 0 {
        if !(context.encoding).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(context.encoding as *mut libc::c_void);
            context.encoding = 0 as *const libc::c_char;
        }
    }
    wget_buffer_deinit(&mut context.uri_buf);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    let mut base: *const libc::c_char = b"http://www.example.com\0" as *const u8
        as *const libc::c_char;
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
                b"Base URL encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
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
    base_uri = wget_iri_parse(base, local_encoding);
    while argpos < argc {
        css_parse_localfile(*argv.offset(argpos as isize), base_uri, css_encoding);
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
                args.as_mut_ptr() as *const *const libc::c_char,
            ) as i32,
        )
    }
}
