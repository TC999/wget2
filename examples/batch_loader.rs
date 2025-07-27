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
    pub type wget_thread_st;
    pub type wget_hpkp_st;
    pub type wget_tcp_st;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn wget_global_init(key: libc::c_int, _: ...);
    fn wget_global_deinit();
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_memiconv(
        src_encoding: *const libc::c_char,
        src: *const libc::c_void,
        srclen: size_t,
        dst_encoding: *const libc::c_char,
        out: *mut *mut libc::c_char,
        outlen: *mut size_t,
    ) -> libc::c_int;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_thread_start(
        thread: *mut wget_thread,
        start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        arg: *mut libc::c_void,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn wget_thread_join(thread: *mut wget_thread) -> libc::c_int;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_unescape_url_inline(src: *mut libc::c_char) -> *mut libc::c_char;
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
    fn wget_tcp_set_timeout(tcp: *mut wget_tcp, timeout: libc::c_int);
    fn wget_tcp_set_connect_timeout(tcp: *mut wget_tcp, timeout: libc::c_int);
    fn wget_ssl_set_config_int(key: libc::c_int, value: libc::c_int);
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_get(first_key: libc::c_int, _: ...) -> *mut wget_http_response;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type wget_vector = wget_vector_st;
pub type wget_thread = *mut wget_thread_st;
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
pub type wget_hpkp = wget_hpkp_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_string {
    pub p: *const libc::c_char,
    pub len: size_t,
}
pub type wget_tcp = wget_tcp_st;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_t {
    pub http_links: libc::c_int,
    pub https_links: libc::c_int,
    pub status: libc::c_int,
    pub redirs: libc::c_int,
    pub redir_insecure: libc::c_int,
    pub landed_on_https: libc::c_int,
    pub host: [libc::c_char; 256],
    pub content_type: [libc::c_char; 128],
}
unsafe extern "C" fn write_stats(mut stats: *const stats_t) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = fopen(
        b"out.csv\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        fprintf(
            fp,
            b"%s,%d,%d,%d,%d,%s\n\0" as *const u8 as *const libc::c_char,
            ((*stats).host).as_ptr(),
            (*stats).status,
            (*stats).redir_insecure,
            (*stats).redirs,
            (*stats).landed_on_https,
            ((*stats).content_type).as_ptr(),
        );
        fclose(fp);
    }
}
unsafe extern "C" fn _normalize_uri(
    mut base: *mut wget_iri,
    mut url: *mut wget_string,
    mut encoding: *const libc::c_char,
    mut buf: *mut wget_buffer,
) -> libc::c_int {
    let mut urlpart_encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut urlpart_encoded_length: size_t = 0;
    let mut rc: libc::c_int = 0;
    if (*url).len == 0 as libc::c_int as size_t
        || (*url).len >= 1 as libc::c_int as size_t
            && *(*url).p as libc::c_int == '#' as i32
    {
        return -(1 as libc::c_int);
    }
    let mut urlpart: *mut libc::c_char = wget_strmemdup(
        (*url).p as *const libc::c_void,
        (*url).len,
    );
    if urlpart.is_null() {
        return -(2 as libc::c_int);
    }
    wget_iri_unescape_url_inline(urlpart);
    rc = wget_memiconv(
        encoding,
        urlpart as *const libc::c_void,
        strlen(urlpart),
        b"utf-8\0" as *const u8 as *const libc::c_char,
        &mut urlpart_encoded,
        &mut urlpart_encoded_length,
    );
    if !urlpart.is_null() {
        wget_free.expect("non-null function pointer")(urlpart as *mut libc::c_void);
        urlpart = 0 as *mut libc::c_char;
    }
    if rc != 0 {
        return -(3 as libc::c_int);
    }
    rc = (wget_iri_relative_to_abs(base, urlpart_encoded, urlpart_encoded_length, buf))
        .is_null() as libc::c_int;
    if !urlpart_encoded.is_null() {
        wget_free
            .expect("non-null function pointer")(urlpart_encoded as *mut libc::c_void);
        urlpart_encoded = 0 as *mut libc::c_char;
    }
    if rc != 0 {
        return -(4 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _normalize_location(
    mut base: *const libc::c_char,
    mut url: *const libc::c_char,
) -> *mut libc::c_char {
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut url_s: wget_string = {
        let mut init = wget_string {
            p: url,
            len: strlen(url),
        };
        init
    };
    let mut base_iri: *mut wget_iri = wget_iri_parse(
        base,
        b"utf-8\0" as *const u8 as *const libc::c_char,
    );
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    let mut norm_url: *mut libc::c_char = 0 as *mut libc::c_char;
    if base_iri.is_null() {
        return 0 as *mut libc::c_char;
    }
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if _normalize_uri(
        base_iri,
        &mut url_s,
        b"utf-8\0" as *const u8 as *const libc::c_char,
        &mut buf,
    ) == 0 as libc::c_int
    {
        norm_url = wget_strmemdup(buf.data as *const libc::c_void, buf.length);
    }
    wget_buffer_deinit(&mut buf);
    wget_iri_free(&mut base_iri);
    return norm_url;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *const *const libc::c_char,
) -> libc::c_int {
    static mut downloaders: [wget_thread; 500] = [0 as *const wget_thread_st
        as *mut wget_thread_st; 500];
    wget_global_init(
        1003 as libc::c_int,
        stdout,
        1006 as libc::c_int,
        stdout,
        1009 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    let mut sig_action: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t),
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        init
    };
    sigaction(13 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    wget_tcp_set_timeout(0 as *mut wget_tcp, 3000 as libc::c_int);
    wget_tcp_set_connect_timeout(0 as *mut wget_tcp, 3000 as libc::c_int);
    wget_ssl_set_config_int(16 as libc::c_int, 0 as libc::c_int);
    wget_ssl_set_config_int(14 as libc::c_int, 0 as libc::c_int);
    wget_ssl_set_config_int(9 as libc::c_int, 0 as libc::c_int);
    wget_ssl_set_config_int(10 as libc::c_int, 0 as libc::c_int);
    let mut rc: libc::c_int = 0;
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < 500 as libc::c_int {
        rc = wget_thread_start(
            &mut *downloaders.as_mut_ptr().offset(it as isize),
            Some(
                downloader_thread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            0 as *mut libc::c_void,
            0 as libc::c_int,
        );
        if rc != 0 as libc::c_int {
            wget_error_printf(
                b"Failed to start thread, error %d\n\0" as *const u8
                    as *const libc::c_char,
                rc,
            );
        }
        it += 1;
        it;
    }
    let mut rc_0: libc::c_int = 0;
    let mut it_0: libc::c_int = 0 as libc::c_int;
    while it_0 < 500 as libc::c_int {
        rc_0 = wget_thread_join(&mut *downloaders.as_mut_ptr().offset(it_0 as isize));
        if rc_0 != 0 as libc::c_int {
            wget_error_printf(
                b"Failed to wait for downloader #%d (%d %d)\n\0" as *const u8
                    as *const libc::c_char,
                it_0,
                rc_0,
                *__errno_location(),
            );
        }
        it_0 += 1;
        it_0;
    }
    wget_global_deinit();
    return 0 as libc::c_int;
}
unsafe extern "C" fn downloader_thread(mut p: *mut libc::c_void) -> *mut libc::c_void {
    let mut stats: stats_t = stats_t {
        http_links: 0,
        https_links: 0,
        status: 0,
        redirs: 0,
        redir_insecure: 0,
        landed_on_https: 0,
        host: [0; 256],
        content_type: [0; 128],
    };
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    while fscanf(
        stdin,
        b"%255s\0" as *const u8 as *const libc::c_char,
        (stats.host).as_mut_ptr(),
    ) == 1 as libc::c_int
    {
        if !url.is_null() {
            wget_free.expect("non-null function pointer")(url as *mut libc::c_void);
            url = 0 as *mut libc::c_char;
        }
        if wget_strncasecmp_ascii(
            (stats.host).as_mut_ptr(),
            b"http://\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        ) == 0
        {
            url = wget_strdup((stats.host).as_mut_ptr());
        } else if wget_strncasecmp_ascii(
            (stats.host).as_mut_ptr(),
            b"https://\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        ) == 0
        {
            url = wget_strdup((stats.host).as_mut_ptr());
        } else {
            url = wget_aprintf(
                b"http://%s\0" as *const u8 as *const libc::c_char,
                (stats.host).as_mut_ptr(),
            );
        }
        stats.https_links = 0 as libc::c_int;
        stats.http_links = stats.https_links;
        stats.status = -(1 as libc::c_int);
        stats.landed_on_https = 0 as libc::c_int;
        stats.redir_insecure = stats.landed_on_https;
        stats.redirs = stats.redir_insecure;
        *(stats.content_type).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        let mut redirs: libc::c_int = 0 as libc::c_int;
        let mut max: libc::c_int = 5 as libc::c_int;
        while redirs < max {
            wget_http_free_response(&mut resp);
            wget_info_printf(
                b"%s%s\n\0" as *const u8 as *const libc::c_char,
                if redirs != 0 {
                    b"  -> \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                url,
            );
            resp = wget_http_get(
                2000 as libc::c_int,
                url,
                2004 as libc::c_int,
                b"User-Agent\0" as *const u8 as *const libc::c_char,
                b"Mozilla/5.0\0" as *const u8 as *const libc::c_char,
                2004 as libc::c_int,
                b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
                b"gzip, br\0" as *const u8 as *const libc::c_char,
                2004 as libc::c_int,
                b"Accept\0" as *const u8 as *const libc::c_char,
                b"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\0"
                    as *const u8 as *const libc::c_char,
                2004 as libc::c_int,
                b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
                b"gzip, br\0" as *const u8 as *const libc::c_char,
                2010 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
            if resp.is_null() {
                wget_info_printf(
                    b"  No connection / response\n\0" as *const u8 as *const libc::c_char,
                );
                break;
            } else {
                snprintf(
                    (stats.content_type).as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    (*resp).content_type,
                );
                stats.status = (*resp).code as libc::c_int;
                if (*resp).code as libc::c_int != 200 as libc::c_int {
                    if !((*resp).location).is_null() {
                        stats.redirs += 1;
                        stats.redirs;
                        wget_info_printf(
                            b"  Response code %hd, %s\n\0" as *const u8
                                as *const libc::c_char,
                            (*resp).code as libc::c_int,
                            (*resp).location,
                        );
                        let mut newurl: *mut libc::c_char = _normalize_location(
                            url,
                            (*resp).location,
                        );
                        if newurl.is_null() {
                            wget_info_printf(
                                b"  Failed to normalize '%s', '%s'\n\0" as *const u8
                                    as *const libc::c_char,
                                url,
                                (*resp).location,
                            );
                            break;
                        } else {
                            if !url.is_null() {
                                wget_free
                                    .expect(
                                        "non-null function pointer",
                                    )(url as *mut libc::c_void);
                                url = 0 as *mut libc::c_char;
                            }
                            url = newurl;
                            if wget_strncasecmp(
                                url,
                                b"https://\0" as *const u8 as *const libc::c_char,
                                8 as libc::c_int as size_t,
                            ) != 0
                            {
                                stats.redir_insecure += 1;
                                stats.redir_insecure;
                            }
                            redirs += 1;
                            redirs;
                        }
                    } else {
                        wget_info_printf(
                            b"  Response code %hd\n\0" as *const u8
                                as *const libc::c_char,
                            (*resp).code as libc::c_int,
                        );
                        break;
                    }
                } else {
                    if wget_strncasecmp(
                        url,
                        b"https://\0" as *const u8 as *const libc::c_char,
                        8 as libc::c_int as size_t,
                    ) != 0
                    {
                        break;
                    }
                    stats.landed_on_https = 1 as libc::c_int;
                    break;
                }
            }
        }
        wget_http_free_response(&mut resp);
        write_stats(&mut stats);
    }
    if !url.is_null() {
        wget_free.expect("non-null function pointer")(url as *mut libc::c_void);
        url = 0 as *mut libc::c_char;
    }
    return 0 as *mut libc::c_void;
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
