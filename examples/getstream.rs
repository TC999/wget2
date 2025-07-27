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
    pub type wget_vector_st;
    pub type wget_hpkp_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn wget_global_init(key: libc::c_int, _: ...);
    fn wget_global_deinit();
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
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_get(first_key: libc::c_int, _: ...) -> *mut wget_http_response;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
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
pub type wget_vector = wget_vector_st;
pub type wget_iri_scheme = libc::c_uint;
pub const WGET_IRI_SCHEME_HTTPS: wget_iri_scheme = 1;
pub const WGET_IRI_SCHEME_HTTP: wget_iri_scheme = 0;
pub type wget_hpkp = wget_hpkp_st;
pub type wget_transfer_encoding = libc::c_uint;
pub const wget_transfer_encoding_chunked: wget_transfer_encoding = 1;
pub const wget_transfer_encoding_identity: wget_transfer_encoding = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_response_st {
    pub req: *mut wget_http_request,
    pub links: *mut wget_vector,
    pub digests: *mut wget_vector,
    pub cookies: *mut wget_vector,
    pub challenges: *mut wget_vector,
    pub hpkp: *mut wget_hpkp,
    pub content_type: *const libc::c_char,
    pub content_type_encoding: *const libc::c_char,
    pub content_filename: *const libc::c_char,
    pub location: *const libc::c_char,
    pub etag: *const libc::c_char,
    pub header: *mut wget_buffer,
    pub body: *mut wget_buffer,
    pub response_end: libc::c_longlong,
    pub content_length: size_t,
    pub cur_downloaded: size_t,
    pub accounted_for: size_t,
    pub last_modified: int64_t,
    pub hsts_maxage: int64_t,
    pub reason: [libc::c_char; 32],
    pub icy_metaint: libc::c_int,
    pub major: libc::c_short,
    pub minor: libc::c_short,
    pub code: libc::c_short,
    pub transfer_encoding: wget_transfer_encoding,
    pub content_encoding: libc::c_char,
    pub hsts_include_subdomains: bool,
    pub keep_alive: bool,
    #[bitfield(name = "content_length_valid", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "length_inconsistent", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "hsts", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "csp", ty = "bool", bits = "3..=3")]
    pub content_length_valid_length_inconsistent_hsts_csp: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_request {
    pub headers: *mut wget_vector,
    pub body: *const libc::c_char,
    pub header_callback: Option::<wget_http_header_callback>,
    pub body_callback: Option::<wget_http_body_callback>,
    pub user_data: *mut libc::c_void,
    pub header_user_data: *mut libc::c_void,
    pub body_user_data: *mut libc::c_void,
    pub esc_resource: wget_buffer,
    pub esc_host: wget_buffer,
    pub body_length: size_t,
    pub stream_id: int32_t,
    pub scheme: wget_iri_scheme,
    pub esc_resource_buf: [libc::c_char; 256],
    pub esc_host_buf: [libc::c_char; 64],
    pub method: [libc::c_char; 8],
    #[bitfield(name = "response_keepheader", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "response_ignorelength", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "debug_skip_body", ty = "bool", bits = "2..=2")]
    pub response_keepheader_response_ignorelength_debug_skip_body: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub request_start: libc::c_longlong,
    pub first_response_start: libc::c_longlong,
}
pub type wget_http_body_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
pub type wget_http_response = wget_http_response_st;
pub type wget_http_header_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
) -> libc::c_int;
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
static mut stream_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut streamdata: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut metadata: [libc::c_char; 4080] = [0; 4080];
static mut metaint: libc::c_int = 0;
static mut streamdatalen: libc::c_int = 0;
static mut metadatalen: libc::c_int = 0;
unsafe extern "C" fn header_callback(
    mut resp: *mut wget_http_response,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    if !((*resp).header).is_null() {
        let mut key: [libc::c_char; 64] = [0; 64];
        let mut value: [libc::c_char; 128] = *::core::mem::transmute::<
            &[u8; 128],
            &mut [libc::c_char; 128],
        >(
            b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
        let mut p: *mut libc::c_char = strchr((*(*resp).header).data, '\n' as i32);
        while !p.is_null()
            && sscanf(
                p.offset(1 as libc::c_int as isize),
                b" %63[a-zA-z-] : %127[^\r\n]\0" as *const u8 as *const libc::c_char,
                key.as_mut_ptr(),
                value.as_mut_ptr(),
            ) >= 1 as libc::c_int
        {
            if wget_strcasecmp_ascii(
                key.as_mut_ptr(),
                b"icy-name\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                stream_name = wget_strdup(value.as_mut_ptr());
                break;
            } else {
                *value.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
                p = strchr(p.offset(1 as libc::c_int as isize), '\n' as i32);
            }
        }
    }
    metaint = (*resp).icy_metaint;
    if metaint != 0 {
        streamdata = wget_malloc(metaint as size_t) as *mut libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stream_callback(
    mut resp: *mut wget_http_response,
    mut context: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if metaint != 0 {
        static mut collect_metadata: libc::c_int = 0;
        static mut metadatasize: size_t = 0;
        while len != 0 {
            if collect_metadata != 0 {
                while len != 0 && metadatasize != 0 {
                    let fresh0 = data;
                    data = data.offset(1);
                    let fresh1 = metadatalen;
                    metadatalen = metadatalen + 1;
                    metadata[fresh1 as usize] = *fresh0;
                    metadatasize = metadatasize.wrapping_sub(1);
                    metadatasize;
                    len = len.wrapping_sub(1);
                    len;
                }
                if metadatasize == 0 as libc::c_int as size_t {
                    collect_metadata = 0 as libc::c_int;
                    wget_info_printf(
                        b"%.*s\n\0" as *const u8 as *const libc::c_char,
                        metadatalen,
                        metadata.as_mut_ptr(),
                    );
                }
            } else {
                while len != 0 && streamdatalen < metaint {
                    let fresh2 = data;
                    data = data.offset(1);
                    let fresh3 = streamdatalen;
                    streamdatalen = streamdatalen + 1;
                    *streamdata.offset(fresh3 as isize) = *fresh2;
                    len = len.wrapping_sub(1);
                    len;
                }
                if len != 0 {
                    let fresh4 = data;
                    data = data.offset(1);
                    metadatasize = (*fresh4 as libc::c_uchar as libc::c_int
                        * 16 as libc::c_int) as size_t;
                    if metadatasize > 0 as libc::c_int as size_t {
                        collect_metadata = 1 as libc::c_int;
                    }
                    len = len.wrapping_sub(1);
                    len;
                    fwrite(
                        streamdata as *const libc::c_void,
                        1 as libc::c_int as libc::c_ulong,
                        streamdatalen as libc::c_ulong,
                        stdout,
                    );
                    metadatalen = 0 as libc::c_int;
                    streamdatalen = 0 as libc::c_int;
                }
            }
        }
    } else {
        fwrite(
            data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len,
            stdout,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn strcasestr_ascii(
    mut haystack: *const libc::c_char,
    mut needle: *const libc::c_char,
) -> *mut libc::c_char {
    let mut needle_len: size_t = strlen(needle);
    while *haystack != 0 {
        if wget_strncasecmp_ascii(haystack, needle, needle_len) == 0 {
            return haystack as *mut libc::c_char;
        }
        haystack = haystack.offset(1);
        haystack;
    }
    return 0 as *mut libc::c_char;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut stream_url: *mut libc::c_char = 0 as *mut libc::c_char;
    wget_global_init(
        1003 as libc::c_int,
        stderr,
        1006 as libc::c_int,
        stderr,
        0 as libc::c_int,
    );
    if argc != 2 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: %s <Playlist URL>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    resp = wget_http_get(
        2000 as libc::c_int,
        *argv.offset(1 as libc::c_int as isize),
        0 as libc::c_int,
    );
    if resp.is_null() {
        fprintf(
            stderr,
            b"Failed to get response from %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(1 as libc::c_int as isize),
        );
        return 1 as libc::c_int;
    }
    if (*resp).code as libc::c_int != 200 as libc::c_int {
        fprintf(
            stderr,
            b"Got response code %d\n\0" as *const u8 as *const libc::c_char,
            (*resp).code as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    if wget_strcasecmp_ascii(
        (*resp).content_type,
        b"audio/x-mpegurl\0" as *const u8 as *const libc::c_char,
    ) == 0
        || wget_strcasecmp_ascii(
            (*resp).content_type,
            b"audio/x-pn-realaudio\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        e = (*(*resp).body).data;
        loop {
            s = e;
            while *(*__ctype_b_loc()).offset(*s as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                s = s.offset(1);
                s;
            }
            if *s == 0 {
                break;
            }
            e = s;
            while *e as libc::c_int != 0 && *e as libc::c_int != '\r' as i32
                && *e as libc::c_int != '\n' as i32
            {
                e = e.offset(1);
                e;
            }
            if *s as libc::c_int != '#' as i32 && s < e {
                stream_url = wget_strmemdup(
                    s as *const libc::c_void,
                    e.offset_from(s) as libc::c_long as size_t,
                );
                break;
            } else if !(*e != 0) {
                break;
            }
        }
    } else if wget_strcasecmp_ascii(
        (*resp).content_type,
        b"audio/x-ms-wax\0" as *const u8 as *const libc::c_char,
    ) == 0
        || wget_strcasecmp_ascii(
            (*resp).content_type,
            b"video/x-ms-asf\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut url: [libc::c_char; 128] = [0; 128];
        p = strcasestr_ascii(
            (*(*resp).body).data,
            b" HREF=\"\0" as *const u8 as *const libc::c_char,
        );
        if !p.is_null()
            && sscanf(
                p.offset(7 as libc::c_int as isize),
                b"%127[^\"]\0" as *const u8 as *const libc::c_char,
                url.as_mut_ptr(),
            ) == 1 as libc::c_int
        {
            stream_url = wget_strdup(url.as_mut_ptr());
        } else {
            fprintf(
                stderr,
                b"Failed to parse playlist URL\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if wget_strcasecmp_ascii(
        (*resp).content_type,
        b"application/pls+xml\0" as *const u8 as *const libc::c_char,
    ) == 0
        || wget_strcasecmp_ascii(
            (*resp).content_type,
            b"audio/x-scpls\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut p_0: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut url_0: [libc::c_char; 128] = [0; 128];
        p_0 = strcasestr_ascii(
            (*(*resp).body).data,
            b"File1=\0" as *const u8 as *const libc::c_char,
        );
        if !p_0.is_null()
            && sscanf(
                p_0.offset(6 as libc::c_int as isize),
                b"%127[^\r\n]\0" as *const u8 as *const libc::c_char,
                url_0.as_mut_ptr(),
            ) == 1 as libc::c_int
        {
            stream_url = wget_strdup(url_0.as_mut_ptr());
        } else {
            fprintf(
                stderr,
                b"Failed to parse playlist URL\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if wget_strcasecmp_ascii(
        (*resp).content_type,
        b"application/xspf+xml\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut p_1: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut url_1: [libc::c_char; 128] = [0; 128];
        p_1 = strcasestr_ascii(
            (*(*resp).body).data,
            b"<location>\0" as *const u8 as *const libc::c_char,
        );
        if !p_1.is_null()
            && sscanf(
                p_1.offset(10 as libc::c_int as isize),
                b" %127[^< \t\r\n]\0" as *const u8 as *const libc::c_char,
                url_1.as_mut_ptr(),
            ) == 1 as libc::c_int
        {
            stream_url = wget_strdup(url_1.as_mut_ptr());
        } else {
            fprintf(
                stderr,
                b"Failed to parse playlist URL\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        fprintf(
            stderr,
            b"Unsupported type of stream: '%s'\n\0" as *const u8 as *const libc::c_char,
            (*resp).content_type,
        );
        return 1 as libc::c_int;
    }
    wget_http_free_response(&mut resp);
    if stream_url.is_null() {
        return 1 as libc::c_int;
    }
    resp = wget_http_get(
        2000 as libc::c_int,
        stream_url,
        2004 as libc::c_int,
        b"Icy-Metadata\0" as *const u8 as *const libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char,
        2015 as libc::c_int,
        Some(header_callback as wget_http_header_callback),
        0 as *mut libc::c_void,
        2014 as libc::c_int,
        Some(stream_callback as wget_http_body_callback),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    );
    wget_http_free_response(&mut resp);
    wget_global_deinit();
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
