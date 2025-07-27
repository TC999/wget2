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
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn wget_css_parse_file(
        fname: *const libc::c_char,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
}
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn _css_parse_encoding(
    mut context: *mut libc::c_void,
    mut encoding: *const libc::c_char,
    mut len: size_t,
) {
    printf(
        b"URI encoding '%.*s'\n\0" as *const u8 as *const libc::c_char,
        len as libc::c_int,
        encoding,
    );
}
unsafe extern "C" fn _css_parse_uri(
    mut context: *mut libc::c_void,
    mut url: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    printf(b"  %.*s\n\0" as *const u8 as *const libc::c_char, len as libc::c_int, url);
}
unsafe extern "C" fn css_parse_localfile(mut fname: *const libc::c_char) {
    wget_css_parse_file(
        fname,
        Some(
            _css_parse_uri
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        Some(
            _css_parse_encoding
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    if isatty(0 as libc::c_int) == 0 {
        css_parse_localfile(b"-\0" as *const u8 as *const libc::c_char);
    } else {
        let mut argpos: libc::c_int = 0;
        argpos = 1 as libc::c_int;
        while argpos < argc {
            printf(
                b"%s:\n\0" as *const u8 as *const libc::c_char,
                *argv.offset(argpos as isize),
            );
            css_parse_localfile(*argv.offset(argpos as isize));
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
