#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type wget_logger_st;
    pub type wget_dns_st;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn wget_ready_2_read(fd: libc::c_int, timeout: libc::c_int) -> libc::c_int;
    fn wget_ready_2_write(fd: libc::c_int, timeout: libc::c_int) -> libc::c_int;
    fn wget_ready_2_transfer(
        fd: libc::c_int,
        timeout: libc::c_int,
        mode: libc::c_int,
    ) -> libc::c_int;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_vprintf(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> size_t;
    fn wget_logger_is_active(logger: *mut wget_logger) -> bool;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_write(buf: *const libc::c_char, len: size_t);
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_dns_resolve(
        dns: *mut wget_dns,
        host: *const libc::c_char,
        port: uint16_t,
        family: libc::c_int,
        preferred_family: libc::c_int,
    ) -> *mut addrinfo;
    fn wget_dns_freeaddrinfo(dns: *mut wget_dns, addrinfo: *mut *mut addrinfo);
    fn wget_ssl_open(tcp: *mut wget_tcp) -> libc::c_int;
    fn wget_ssl_close(session: *mut *mut libc::c_void);
    fn wget_ssl_read_timeout(
        session: *mut libc::c_void,
        buf: *mut libc::c_char,
        count: size_t,
        timeout: libc::c_int,
    ) -> ssize_t;
    fn wget_ssl_write_timeout(
        session: *mut libc::c_void,
        buf: *const libc::c_char,
        count: size_t,
        timeout: libc::c_int,
    ) -> ssize_t;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn gl_sockets_startup(version: libc::c_int) -> libc::c_int;
    fn gl_sockets_cleanup() -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_0 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_0 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_0 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type C2RustUnnamed_1 = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed_1 = -12;
pub const WGET_E_IO: C2RustUnnamed_1 = -11;
pub const WGET_E_OPEN: C2RustUnnamed_1 = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed_1 = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed_1 = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed_1 = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed_1 = -6;
pub const WGET_E_CONNECT: C2RustUnnamed_1 = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed_1 = -4;
pub const WGET_E_INVALID: C2RustUnnamed_1 = -3;
pub const WGET_E_MEMORY: C2RustUnnamed_1 = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed_1 = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed_1 = 0;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
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
pub type wget_dns = wget_dns_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_tcp_st {
    pub ssl_session: *mut libc::c_void,
    pub addrinfo: *mut addrinfo,
    pub bind_addrinfo: *mut addrinfo,
    pub connect_addrinfo: *mut addrinfo,
    pub host: *const libc::c_char,
    pub ssl_hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub bind_interface: *const libc::c_char,
    pub dns: *mut wget_dns,
    pub sockfd: libc::c_int,
    pub dns_timeout: libc::c_int,
    pub connect_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub family: libc::c_int,
    pub preferred_family: libc::c_int,
    pub protocol: libc::c_int,
    pub hpkp: wget_hpkp_stats_result,
    pub remote_port: uint16_t,
    #[bitfield(name = "ssl", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "tls_false_start", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "tcp_fastopen", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "first_send", ty = "bool", bits = "3..=3")]
    pub ssl_tls_false_start_tcp_fastopen_first_send: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type wget_hpkp_stats_result = libc::c_uint;
pub const WGET_STATS_HPKP_ERROR: wget_hpkp_stats_result = 3;
pub const WGET_STATS_HPKP_NOMATCH: wget_hpkp_stats_result = 2;
pub const WGET_STATS_HPKP_MATCH: wget_hpkp_stats_result = 1;
pub const WGET_STATS_HPKP_NO: wget_hpkp_stats_result = 0;
pub type wget_tcp = wget_tcp_st;
#[inline]
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
static mut global_tcp: wget_tcp_st = wget_tcp_st {
    ssl_session: 0 as *const libc::c_void as *mut libc::c_void,
    addrinfo: 0 as *const addrinfo as *mut addrinfo,
    bind_addrinfo: 0 as *const addrinfo as *mut addrinfo,
    connect_addrinfo: 0 as *const addrinfo as *mut addrinfo,
    host: 0 as *const libc::c_char,
    ssl_hostname: 0 as *const libc::c_char,
    ip: 0 as *const libc::c_char,
    bind_interface: 0 as *const libc::c_char,
    dns: 0 as *const wget_dns as *mut wget_dns,
    sockfd: 0,
    dns_timeout: 0,
    connect_timeout: 0,
    timeout: 0,
    family: 0,
    preferred_family: 0,
    protocol: 0,
    hpkp: WGET_STATS_HPKP_NO,
    remote_port: 0,
    ssl_tls_false_start_tcp_fastopen_first_send: [0; 1],
    c2rust_padding: [0; 5],
};
#[inline]
unsafe extern "C" fn print_error_host(
    mut msg: *const libc::c_char,
    mut host: *const libc::c_char,
) {
    wget_error_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"%s (hostname='%s', errno=%d)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        msg,
        host,
        *__errno_location(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_net_init() -> libc::c_int {
    let mut rc: libc::c_int = gl_sockets_startup(0x202 as libc::c_int);
    return if rc != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn wget_net_deinit() -> libc::c_int {
    let mut rc: libc::c_int = gl_sockets_cleanup();
    return if rc != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
unsafe extern "C" fn value_to_family(mut value: libc::c_int) -> libc::c_int {
    match value {
        1 => return 2 as libc::c_int,
        2 => return 10 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
unsafe extern "C" fn family_to_value(mut family: libc::c_int) -> libc::c_int {
    match family {
        2 => return 1 as libc::c_int,
        10 => return 2 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_dns(
    mut tcp: *mut wget_tcp,
    mut dns: *mut wget_dns,
) {
    let ref mut fresh0 = (*if !tcp.is_null() { tcp } else { &mut global_tcp }).dns;
    *fresh0 = dns;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_tcp_fastopen(
    mut tcp: *mut wget_tcp,
    mut tcp_fastopen: bool,
) {
    let ref mut fresh1 = *if !tcp.is_null() { tcp } else { &mut global_tcp };
    (*fresh1).set_tcp_fastopen(tcp_fastopen);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_tcp_fastopen(mut tcp: *mut wget_tcp) -> bool {
    return (*if !tcp.is_null() { tcp } else { &mut global_tcp }).tcp_fastopen();
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_tls_false_start(
    mut tcp: *mut wget_tcp,
    mut false_start: bool,
) {
    let ref mut fresh2 = *if !tcp.is_null() { tcp } else { &mut global_tcp };
    (*fresh2).set_tls_false_start(false_start);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_tls_false_start(mut tcp: *mut wget_tcp) -> bool {
    return (*if !tcp.is_null() { tcp } else { &mut global_tcp }).tls_false_start();
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_protocol(
    mut tcp: *mut wget_tcp,
    mut protocol: libc::c_int,
) {
    (*if !tcp.is_null() { tcp } else { &mut global_tcp }).protocol = protocol;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_protocol(mut tcp: *mut wget_tcp) -> libc::c_int {
    return (*if !tcp.is_null() { tcp } else { &mut global_tcp }).protocol;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_preferred_family(
    mut tcp: *mut wget_tcp,
    mut family: libc::c_int,
) {
    (*if !tcp.is_null() { tcp } else { &mut global_tcp })
        .preferred_family = value_to_family(family);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_preferred_family(
    mut tcp: *mut wget_tcp,
) -> libc::c_int {
    return family_to_value(
        (*if !tcp.is_null() { tcp } else { &mut global_tcp }).preferred_family,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_family(
    mut tcp: *mut wget_tcp,
    mut family: libc::c_int,
) {
    (*if !tcp.is_null() { tcp } else { &mut global_tcp })
        .family = value_to_family(family);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_family(mut tcp: *mut wget_tcp) -> libc::c_int {
    return family_to_value((*if !tcp.is_null() { tcp } else { &mut global_tcp }).family);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_local_port(mut tcp: *mut wget_tcp) -> libc::c_int {
    if tcp.is_null() as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int;
    }
    let mut addr_store: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut addr: *mut sockaddr = &mut addr_store as *mut sockaddr_storage
        as *mut sockaddr;
    let mut addr_len: socklen_t = ::core::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    if getsockname(
        (*tcp).sockfd,
        __SOCKADDR_ARG {
            __sockaddr__: addr,
        },
        &mut addr_len,
    ) == 0 as libc::c_int
    {
        let mut s_port: [libc::c_char; 32] = [0; 32];
        if getnameinfo(
            addr,
            addr_len,
            0 as *mut libc::c_char,
            0 as libc::c_int as socklen_t,
            s_port.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as socklen_t,
            2 as libc::c_int,
        ) == 0 as libc::c_int
        {
            return atoi(s_port.as_mut_ptr());
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_connect_timeout(
    mut tcp: *mut wget_tcp,
    mut timeout: libc::c_int,
) {
    (*if !tcp.is_null() { tcp } else { &mut global_tcp }).connect_timeout = timeout;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_timeout(
    mut tcp: *mut wget_tcp,
    mut timeout: libc::c_int,
) {
    (*if !tcp.is_null() { tcp } else { &mut global_tcp }).timeout = timeout;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_timeout(mut tcp: *mut wget_tcp) -> libc::c_int {
    return (*if !tcp.is_null() { tcp } else { &mut global_tcp }).timeout;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_bind_address(
    mut tcp: *mut wget_tcp,
    mut bind_address: *const libc::c_char,
) {
    if tcp.is_null() {
        tcp = &mut global_tcp;
    }
    wget_dns_freeaddrinfo((*tcp).dns, &mut (*tcp).bind_addrinfo);
    if !bind_address.is_null() {
        let mut host: *const libc::c_char = 0 as *const libc::c_char;
        let mut s: *const libc::c_char = bind_address;
        if *s as libc::c_int == '[' as i32 {
            let mut p: *mut libc::c_char = strrchr(s, ']' as i32);
            if !p.is_null() {
                host = s.offset(1 as libc::c_int as isize);
                s = p.offset(1 as libc::c_int as isize);
            } else {
                host = s.offset(1 as libc::c_int as isize);
                while *s != 0 {
                    s = s.offset(1);
                    s;
                }
            }
        } else {
            host = s;
            while *s as libc::c_int != 0 && *s as libc::c_int != ':' as i32 {
                s = s.offset(1);
                s;
            }
        }
        if *s as libc::c_int == ':' as i32 {
            let mut port: [libc::c_char; 6] = [0; 6];
            wget_strscpy(
                port.as_mut_ptr(),
                s.offset(1 as libc::c_int as isize),
                ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
            );
            if c_isdigit(*port.as_mut_ptr() as libc::c_int) {
                (*tcp)
                    .bind_addrinfo = wget_dns_resolve(
                    (*tcp).dns,
                    host,
                    atoi(port.as_mut_ptr()) as uint16_t,
                    (*tcp).family,
                    (*tcp).preferred_family,
                );
            }
        } else {
            (*tcp)
                .bind_addrinfo = wget_dns_resolve(
                (*tcp).dns,
                host,
                0 as libc::c_int as uint16_t,
                (*tcp).family,
                (*tcp).preferred_family,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_bind_interface(
    mut tcp: *mut wget_tcp,
    mut bind_interface: *const libc::c_char,
) {
    if tcp.is_null() {
        tcp = &mut global_tcp;
    }
    (*tcp).bind_interface = bind_interface;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_ssl(mut tcp: *mut wget_tcp, mut ssl: bool) {
    let ref mut fresh3 = *if !tcp.is_null() { tcp } else { &mut global_tcp };
    (*fresh3).set_ssl(ssl);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_ssl(mut tcp: *mut wget_tcp) -> bool {
    return (*if !tcp.is_null() { tcp } else { &mut global_tcp }).ssl();
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_ip(mut tcp: *mut wget_tcp) -> *const libc::c_char {
    return if !tcp.is_null() { (*tcp).ip } else { 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_set_ssl_hostname(
    mut tcp: *mut wget_tcp,
    mut hostname: *const libc::c_char,
) {
    if tcp.is_null() {
        tcp = &mut global_tcp;
    }
    if !((*tcp).ssl_hostname).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*tcp).ssl_hostname as *mut libc::c_void);
        (*tcp).ssl_hostname = 0 as *const libc::c_char;
    }
    (*tcp).ssl_hostname = wget_strdup(hostname);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_get_ssl_hostname(
    mut tcp: *mut wget_tcp,
) -> *const libc::c_char {
    return (*if !tcp.is_null() { tcp } else { &mut global_tcp }).ssl_hostname;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_init() -> *mut wget_tcp {
    let mut tcp: *mut wget_tcp = wget_malloc(
        ::core::mem::size_of::<wget_tcp>() as libc::c_ulong,
    ) as *mut wget_tcp;
    if !tcp.is_null() {
        *tcp = global_tcp;
        (*tcp).ssl_hostname = wget_strdup(global_tcp.ssl_hostname);
    }
    return tcp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_deinit(mut _tcp: *mut *mut wget_tcp) {
    let mut tcp: *mut wget_tcp = 0 as *mut wget_tcp;
    if _tcp.is_null() {
        if !(global_tcp.ssl_hostname).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(global_tcp.ssl_hostname as *mut libc::c_void);
            global_tcp.ssl_hostname = 0 as *const libc::c_char;
        }
        return;
    }
    tcp = *_tcp;
    if !tcp.is_null() {
        wget_tcp_close(tcp);
        wget_dns_freeaddrinfo((*tcp).dns, &mut (*tcp).bind_addrinfo);
        if !((*tcp).ssl_hostname).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*tcp).ssl_hostname as *mut libc::c_void);
            (*tcp).ssl_hostname = 0 as *const libc::c_char;
        }
        if !((*tcp).ip).is_null() {
            wget_free
                .expect("non-null function pointer")((*tcp).ip as *mut libc::c_void);
            (*tcp).ip = 0 as *const libc::c_char;
        }
        if !tcp.is_null() {
            wget_free.expect("non-null function pointer")(tcp as *mut libc::c_void);
            tcp = 0 as *mut wget_tcp;
        }
        *_tcp = 0 as *mut wget_tcp;
    }
}
unsafe extern "C" fn _set_async(mut fd: libc::c_int) {
    let mut flags: libc::c_int = 0;
    flags = rpl_fcntl(fd, 3 as libc::c_int);
    if flags < 0 as libc::c_int {
        wget_error_printf_exit(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to get socket flags\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if rpl_fcntl(fd, 4 as libc::c_int, flags | 0o4000 as libc::c_int) < 0 as libc::c_int
    {
        wget_error_printf_exit(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set socket to non-blocking\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
unsafe extern "C" fn set_socket_options(mut tcp: *const wget_tcp, mut fd: libc::c_int) {
    let mut on: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set socket option REUSEADDR\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    on = 1 as libc::c_int;
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set socket option NODELAY\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if !((*tcp).bind_interface).is_null() {
        if setsockopt(
            fd,
            1 as libc::c_int,
            25 as libc::c_int,
            (*tcp).bind_interface as *const libc::c_void,
            strlen((*tcp).bind_interface) as socklen_t,
        ) == -(1 as libc::c_int)
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to set socket option BINDTODEVICE\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    if (*tcp).tcp_fastopen() {
        on = 1 as libc::c_int;
        if setsockopt(
            fd,
            IPPROTO_TCP as libc::c_int,
            30 as libc::c_int,
            &mut on as *mut libc::c_int as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        ) == -(1 as libc::c_int)
        {
            wget_debug_printf(
                b"Failed to set socket option TCP_FASTOPEN_CONNECT\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if (*tcp).connect_timeout > 0 as libc::c_int {
        let mut tv: timeval = {
            let mut init = timeval {
                tv_sec: ((*tcp).connect_timeout / 1000 as libc::c_int) as __time_t,
                tv_usec: ((*tcp).connect_timeout % 1000 as libc::c_int
                    * 1000 as libc::c_int) as __suseconds_t,
            };
            init
        };
        if setsockopt(
            fd,
            1 as libc::c_int,
            21 as libc::c_int,
            &mut tv as *mut timeval as *const libc::c_void,
            ::core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
        ) == -(1 as libc::c_int)
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to set socket option SO_SNDTIMEO\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_ready_2_transfer(
    mut tcp: *mut wget_tcp,
    mut flags: libc::c_int,
) -> libc::c_int {
    if !tcp.is_null() as libc::c_int as libc::c_long != 0 {
        return wget_ready_2_transfer((*tcp).sockfd, (*tcp).timeout, flags)
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn debug_addr(
    mut caption: *const libc::c_char,
    mut ai_addr: *const sockaddr,
    mut ai_addrlen: socklen_t,
) {
    let mut rc: libc::c_int = 0;
    let mut adr: [libc::c_char; 1025] = [0; 1025];
    let mut s_port: [libc::c_char; 32] = [0; 32];
    rc = getnameinfo(
        ai_addr,
        ai_addrlen,
        adr.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong as socklen_t,
        s_port.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as socklen_t,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if rc == 0 as libc::c_int {
        if (*ai_addr).sa_family as libc::c_int == 10 as libc::c_int {
            wget_debug_printf(
                b"%s [%s]:%s...\n\0" as *const u8 as *const libc::c_char,
                caption,
                adr.as_mut_ptr(),
                s_port.as_mut_ptr(),
            );
        } else {
            wget_debug_printf(
                b"%s %s:%s...\n\0" as *const u8 as *const libc::c_char,
                caption,
                adr.as_mut_ptr(),
                s_port.as_mut_ptr(),
            );
        }
    } else {
        wget_debug_printf(
            b"%s ???:%s (%s)...\n\0" as *const u8 as *const libc::c_char,
            caption,
            s_port.as_mut_ptr(),
            gai_strerror(rc),
        );
    };
}
unsafe extern "C" fn tcp_connect(
    mut tcp: *mut wget_tcp,
    mut ai: *mut addrinfo,
    mut sockfd: libc::c_int,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    if (*tcp).tcp_fastopen() {
        (*tcp).connect_addrinfo = ai;
        rc = connect(
            sockfd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: (*ai).ai_addr,
            },
            (*ai).ai_addrlen,
        );
        (*tcp).set_first_send(0 as libc::c_int != 0);
    } else {
        rc = connect(
            sockfd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: (*ai).ai_addr,
            },
            (*ai).ai_addrlen,
        );
        (*tcp).set_first_send(0 as libc::c_int != 0);
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_connect(
    mut tcp: *mut wget_tcp,
    mut host: *const libc::c_char,
    mut port: uint16_t,
) -> libc::c_int {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut rc: libc::c_int = 0;
    let mut ret: libc::c_int = WGET_E_UNKNOWN as libc::c_int;
    let mut adr: [libc::c_char; 1025] = [0; 1025];
    let mut s_port: [libc::c_char; 32] = [0; 32];
    let mut debug: bool = wget_logger_is_active(wget_get_logger(3 as libc::c_int));
    if tcp.is_null() as libc::c_int as libc::c_long != 0 {
        return WGET_E_INVALID as libc::c_int;
    }
    wget_dns_freeaddrinfo((*tcp).dns, &mut (*tcp).addrinfo);
    if !((*tcp).host).is_null() {
        wget_free.expect("non-null function pointer")((*tcp).host as *mut libc::c_void);
        (*tcp).host = 0 as *const libc::c_char;
    }
    (*tcp)
        .addrinfo = wget_dns_resolve(
        (*tcp).dns,
        host,
        port,
        (*tcp).family,
        (*tcp).preferred_family,
    );
    (*tcp).remote_port = port;
    if ((*tcp).addrinfo).is_null() {
        return WGET_E_CONNECT as libc::c_int;
    }
    let mut current_block_44: u64;
    ai = (*tcp).addrinfo;
    while !ai.is_null() {
        if !((*ai).ai_socktype != SOCK_STREAM as libc::c_int) {
            if debug {
                debug_addr(
                    b"trying\0" as *const u8 as *const libc::c_char,
                    (*ai).ai_addr,
                    (*ai).ai_addrlen,
                );
            }
            let mut sockfd: libc::c_int = 0;
            sockfd = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
            if sockfd == -(1 as libc::c_int) {
                print_error_host(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to create socket\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    host,
                );
                ret = WGET_E_UNKNOWN as libc::c_int;
            } else {
                set_socket_options(tcp, sockfd);
                if !((*tcp).bind_addrinfo).is_null() {
                    if debug {
                        debug_addr(
                            b"binding to\0" as *const u8 as *const libc::c_char,
                            (*(*tcp).bind_addrinfo).ai_addr,
                            (*(*tcp).bind_addrinfo).ai_addrlen,
                        );
                    }
                    if bind(
                        sockfd,
                        __CONST_SOCKADDR_ARG {
                            __sockaddr__: (*(*tcp).bind_addrinfo).ai_addr,
                        },
                        (*(*tcp).bind_addrinfo).ai_addrlen,
                    ) != 0 as libc::c_int
                    {
                        print_error_host(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to bind\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            host,
                        );
                        close(sockfd);
                        return WGET_E_UNKNOWN as libc::c_int;
                    }
                }
                rc = tcp_connect(tcp, ai, sockfd);
                if rc < 0 as libc::c_int && *__errno_location() != 11 as libc::c_int
                    && *__errno_location() != 115 as libc::c_int
                {
                    print_error_host(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to connect\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        host,
                    );
                    ret = WGET_E_CONNECT as libc::c_int;
                    close(sockfd);
                } else {
                    (*tcp).sockfd = sockfd;
                    if (*tcp).ssl() {
                        ret = wget_ssl_open(tcp);
                        if ret != 0 {
                            if ret == WGET_E_CERTIFICATE as libc::c_int {
                                wget_tcp_close(tcp);
                                break;
                            } else {
                                let mut ai_tmp: *mut addrinfo = (*tcp).addrinfo;
                                (*tcp).addrinfo = 0 as *mut addrinfo;
                                wget_tcp_close(tcp);
                                (*tcp).addrinfo = ai_tmp;
                            }
                            current_block_44 = 1841672684692190573;
                        } else {
                            current_block_44 = 14832935472441733737;
                        }
                    } else {
                        current_block_44 = 14832935472441733737;
                    }
                    match current_block_44 {
                        1841672684692190573 => {}
                        _ => {
                            if getnameinfo(
                                (*ai).ai_addr,
                                (*ai).ai_addrlen,
                                adr.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 1025]>()
                                    as libc::c_ulong as socklen_t,
                                s_port.as_mut_ptr(),
                                ::core::mem::size_of::<[libc::c_char; 32]>()
                                    as libc::c_ulong as socklen_t,
                                1 as libc::c_int | 2 as libc::c_int,
                            ) == 0 as libc::c_int
                            {
                                (*tcp).ip = wget_strdup(adr.as_mut_ptr());
                            } else {
                                (*tcp).ip = 0 as *const libc::c_char;
                            }
                            (*tcp).host = wget_strdup(host);
                            _set_async(sockfd);
                            return WGET_E_SUCCESS as libc::c_int;
                        }
                    }
                }
            }
        }
        ai = (*ai).ai_next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_tls_start(mut tcp: *mut wget_tcp) -> libc::c_int {
    return wget_ssl_open(tcp);
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_tls_stop(mut tcp: *mut wget_tcp) {
    if !tcp.is_null() {
        wget_ssl_close(&mut (*tcp).ssl_session);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_read(
    mut tcp: *mut wget_tcp,
    mut buf: *mut libc::c_char,
    mut count: size_t,
) -> ssize_t {
    let mut rc: ssize_t = 0;
    if (tcp.is_null() || buf.is_null()) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as ssize_t;
    }
    if !((*tcp).ssl_session).is_null() {
        rc = wget_ssl_read_timeout((*tcp).ssl_session, buf, count, (*tcp).timeout);
    } else {
        if (*tcp).timeout != 0 {
            rc = wget_ready_2_read((*tcp).sockfd, (*tcp).timeout) as ssize_t;
            if rc <= 0 as libc::c_int as ssize_t {
                return rc;
            }
        }
        rc = recvfrom(
            (*tcp).sockfd,
            buf as *mut libc::c_void,
            count,
            0 as libc::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
            },
            0 as *mut socklen_t,
        );
    }
    if rc < 0 as libc::c_int as ssize_t {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to read %zu bytes (hostname='%s', ip=%s, errno=%d)\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            count,
            (*tcp).host,
            (*tcp).ip,
            *__errno_location(),
        );
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_write(
    mut tcp: *mut wget_tcp,
    mut buf: *const libc::c_char,
    mut count: size_t,
) -> ssize_t {
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    if (tcp.is_null() || buf.is_null()) as libc::c_int as libc::c_long != 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    if !((*tcp).ssl_session).is_null() {
        return wget_ssl_write_timeout((*tcp).ssl_session, buf, count, (*tcp).timeout);
    }
    while count != 0 {
        let mut n: ssize_t = 0;
        n = send((*tcp).sockfd, buf as *const libc::c_void, count, 0 as libc::c_int);
        if n >= 0 as libc::c_int as ssize_t {
            nwritten += n;
            if n as size_t >= count {
                return nwritten;
            }
            count = count.wrapping_sub(n as size_t);
            buf = buf.offset(n as isize);
        } else {
            if *__errno_location() != 11 as libc::c_int
                && *__errno_location() != 107 as libc::c_int
                && *__errno_location() != 115 as libc::c_int
            {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to send %zu bytes (hostname='%s', ip=%s, errno=%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    count,
                    (*tcp).host,
                    (*tcp).ip,
                    *__errno_location(),
                );
                return -(1 as libc::c_int) as ssize_t;
            }
            if (*tcp).timeout != 0 {
                let mut rc: libc::c_int = wget_ready_2_write(
                    (*tcp).sockfd,
                    (*tcp).timeout,
                );
                if rc <= 0 as libc::c_int {
                    return rc as ssize_t;
                }
            }
        }
    }
    return 0 as libc::c_int as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_vprintf(
    mut tcp: *mut wget_tcp,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> ssize_t {
    let mut sbuf: [libc::c_char; 4096] = [0; 4096];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut len2: ssize_t = 0;
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    len2 = wget_tcp_write(tcp, buf.data, buf.length);
    wget_buffer_deinit(&mut buf);
    if len2 > 0 as libc::c_int as ssize_t {
        wget_debug_write(buf.data, len2 as size_t);
    }
    if len2 > 0 as libc::c_int as ssize_t && buf.length as ssize_t != len2 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: internal error: length mismatch %zu != %zd\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*::core::mem::transmute::<
                &[u8; 17],
                &[libc::c_char; 17],
            >(b"wget_tcp_vprintf\0"))
                .as_ptr(),
            buf.length,
            len2,
        );
    }
    return len2;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_printf(
    mut tcp: *mut wget_tcp,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ssize_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut len: ssize_t = wget_tcp_vprintf(tcp, fmt, args_0.as_va_list());
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn wget_tcp_close(mut tcp: *mut wget_tcp) {
    if !tcp.is_null() as libc::c_int as libc::c_long != 0 {
        wget_tcp_tls_stop(tcp);
        if (*tcp).sockfd != -(1 as libc::c_int) {
            close((*tcp).sockfd);
            (*tcp).sockfd = -(1 as libc::c_int);
        }
        wget_dns_freeaddrinfo((*tcp).dns, &mut (*tcp).addrinfo);
        if !((*tcp).host).is_null() {
            wget_free
                .expect("non-null function pointer")((*tcp).host as *mut libc::c_void);
            (*tcp).host = 0 as *const libc::c_char;
        }
    }
}
unsafe extern "C" fn run_static_initializers() {
    global_tcp = {
        let mut init = wget_tcp_st {
            ssl_tls_false_start_tcp_fastopen_first_send: [0; 1],
            c2rust_padding: [0; 5],
            ssl_session: 0 as *mut libc::c_void,
            addrinfo: 0 as *mut addrinfo,
            bind_addrinfo: 0 as *mut addrinfo,
            connect_addrinfo: 0 as *mut addrinfo,
            host: 0 as *const libc::c_char,
            ssl_hostname: 0 as *const libc::c_char,
            ip: 0 as *const libc::c_char,
            bind_interface: 0 as *const libc::c_char,
            dns: 0 as *mut wget_dns,
            sockfd: -(1 as libc::c_int),
            dns_timeout: -(1 as libc::c_int),
            connect_timeout: -(1 as libc::c_int),
            timeout: -(1 as libc::c_int),
            family: 0 as libc::c_int,
            preferred_family: 0,
            protocol: 0,
            hpkp: WGET_STATS_HPKP_NO,
            remote_port: 0,
        };
        init.set_ssl(false);
        init.set_tls_false_start(false);
        init.set_tcp_fastopen(1 as libc::c_int != 0);
        init.set_first_send(false);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
