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
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type wget_error = libc::c_int;
pub const WGET_E_UNSUPPORTED: wget_error = -12;
pub const WGET_E_IO: wget_error = -11;
pub const WGET_E_OPEN: wget_error = -10;
pub const WGET_E_XML_PARSE_ERR: wget_error = -9;
pub const WGET_E_TLS_DISABLED: wget_error = -8;
pub const WGET_E_CERTIFICATE: wget_error = -7;
pub const WGET_E_HANDSHAKE: wget_error = -6;
pub const WGET_E_CONNECT: wget_error = -5;
pub const WGET_E_TIMEOUT: wget_error = -4;
pub const WGET_E_INVALID: wget_error = -3;
pub const WGET_E_MEMORY: wget_error = -2;
pub const WGET_E_UNKNOWN: wget_error = -1;
pub const WGET_E_SUCCESS: wget_error = 0;
#[no_mangle]
pub unsafe extern "C" fn wget_strerror(mut err: wget_error) -> *const libc::c_char {
    match err as libc::c_int {
        0 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Success\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -1 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"General error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -2 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"No memory\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -3 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Invalid value\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -4 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Timeout\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -5 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Connect error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -6 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Handshake error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -7 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Certificate error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -8 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"libwget has been built without TLS support\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -9 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Failed to parse XML\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -10 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Failed to open file\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -11 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"I/O error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        -12 => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Unsupported function\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
        _ => {
            return dcgettext(
                0 as *const libc::c_char,
                b"Unknown error\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        }
    };
}
