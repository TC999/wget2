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
    pub type wget_logger_st;
    pub type wget_thread_mutex_st;
    pub type wget_dns_cache_st;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
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
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn wget_millisleep(ms: libc::c_int);
    fn wget_get_timemillis() -> libc::c_longlong;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_logger_is_active(logger: *mut wget_logger) -> bool;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_get_logger(id: libc::c_int) -> *mut wget_logger;
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn wget_dns_cache_get(
        cache: *mut wget_dns_cache,
        host: *const libc::c_char,
        port: uint16_t,
    ) -> *mut addrinfo;
    fn wget_dns_cache_add(
        cache: *mut wget_dns_cache,
        host: *const libc::c_char,
        port: uint16_t,
        addrinfo: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn wget_ip_is_family(host: *const libc::c_char, family: libc::c_int) -> bool;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint16_t = libc::c_ushort;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
pub type uint16_t = __uint16_t;
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
pub type C2RustUnnamed = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed = -12;
pub const WGET_E_IO: C2RustUnnamed = -11;
pub const WGET_E_OPEN: C2RustUnnamed = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed = -6;
pub const WGET_E_CONNECT: C2RustUnnamed = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed = -4;
pub const WGET_E_INVALID: C2RustUnnamed = -3;
pub const WGET_E_MEMORY: C2RustUnnamed = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed = 0;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_logger = wget_logger_st;
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_dns_cache = wget_dns_cache_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_dns_st {
    pub cache: *mut wget_dns_cache,
    pub mutex: wget_thread_mutex,
    pub stats_callback: Option::<wget_dns_stats_callback>,
    pub stats_ctx: *mut libc::c_void,
    pub stats: wget_dns_stats_data,
    pub timeout: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_dns_stats_data {
    pub hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub port: uint16_t,
    pub dns_secs: libc::c_longlong,
}
pub type wget_dns_stats_callback = unsafe extern "C" fn(
    *mut wget_dns,
    *mut wget_dns_stats_data,
    *mut libc::c_void,
) -> ();
pub type wget_dns = wget_dns_st;
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
static mut default_dns: wget_dns = {
    let mut init = wget_dns_st {
        cache: 0 as *const wget_dns_cache as *mut wget_dns_cache,
        mutex: 0 as *const wget_thread_mutex_st as *mut wget_thread_mutex_st,
        stats_callback: None,
        stats_ctx: 0 as *const libc::c_void as *mut libc::c_void,
        stats: wget_dns_stats_data {
            hostname: 0 as *const libc::c_char,
            ip: 0 as *const libc::c_char,
            port: 0,
            dns_secs: 0,
        },
        timeout: -(1 as libc::c_int),
    };
    init
};
static mut initialized: bool = false;
unsafe extern "C" fn dns_exit() {
    if initialized {
        wget_thread_mutex_destroy(&mut default_dns.mutex);
        initialized = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn dns_init() {
    if !initialized {
        wget_thread_mutex_init(&mut default_dns.mutex);
        initialized = 1 as libc::c_int != 0;
        atexit(Some(dns_exit as unsafe extern "C" fn() -> ()));
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_init(mut dns: *mut *mut wget_dns) -> libc::c_int {
    dns_init();
    if dns.is_null() {
        return WGET_E_SUCCESS as libc::c_int;
    }
    let mut _dns: *mut wget_dns = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_dns>() as libc::c_ulong,
    ) as *mut wget_dns;
    if _dns.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    if wget_thread_mutex_init(&mut (*_dns).mutex) != 0 {
        if !_dns.is_null() {
            wget_free.expect("non-null function pointer")(_dns as *mut libc::c_void);
            _dns = 0 as *mut wget_dns;
        }
        return WGET_E_INVALID as libc::c_int;
    }
    (*_dns).timeout = -(1 as libc::c_int);
    *dns = _dns;
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_free(mut dns: *mut *mut wget_dns) {
    if dns.is_null() {
        dns_exit();
        return;
    }
    if !(*dns).is_null() {
        wget_thread_mutex_destroy(&mut (**dns).mutex);
        if !(*dns).is_null() {
            wget_free.expect("non-null function pointer")(*dns as *mut libc::c_void);
            *dns = 0 as *mut wget_dns;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_set_timeout(
    mut dns: *mut wget_dns,
    mut timeout: libc::c_int,
) {
    (*if !dns.is_null() { dns } else { &mut default_dns }).timeout = timeout;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_set_cache(
    mut dns: *mut wget_dns,
    mut cache: *mut wget_dns_cache,
) {
    let ref mut fresh0 = (*if !dns.is_null() { dns } else { &mut default_dns }).cache;
    *fresh0 = cache;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_get_cache(
    mut dns: *mut wget_dns,
) -> *mut wget_dns_cache {
    return (*if !dns.is_null() { dns } else { &mut default_dns }).cache;
}
unsafe extern "C" fn sort_preferred(
    mut addrinfo: *mut addrinfo,
    mut preferred_family: libc::c_int,
) -> *mut addrinfo {
    let mut preferred: *mut addrinfo = 0 as *mut addrinfo;
    let mut preferred_tail: *mut addrinfo = 0 as *mut addrinfo;
    let mut unpreferred: *mut addrinfo = 0 as *mut addrinfo;
    let mut unpreferred_tail: *mut addrinfo = 0 as *mut addrinfo;
    let mut ai: *mut addrinfo = addrinfo;
    while !ai.is_null() {
        if (*ai).ai_family == preferred_family {
            if !preferred_tail.is_null() {
                (*preferred_tail).ai_next = ai;
            } else {
                preferred = ai;
            }
            preferred_tail = ai;
            ai = (*ai).ai_next;
            (*preferred_tail).ai_next = 0 as *mut addrinfo;
        } else {
            if !unpreferred_tail.is_null() {
                (*unpreferred_tail).ai_next = ai;
            } else {
                unpreferred = ai;
            }
            unpreferred_tail = ai;
            ai = (*ai).ai_next;
            (*unpreferred_tail).ai_next = 0 as *mut addrinfo;
        }
    }
    if !preferred.is_null() {
        (*preferred_tail).ai_next = unpreferred;
        return preferred;
    } else {
        return unpreferred
    };
}
unsafe extern "C" fn getaddrinfo_merging(
    mut host: *const libc::c_char,
    mut s_port: *const libc::c_char,
    mut hints: *mut addrinfo,
    mut out_addr: *mut *mut addrinfo,
) -> libc::c_int {
    if (*out_addr).is_null() {
        return getaddrinfo(host, s_port, hints, out_addr);
    }
    let mut ai_tail: *mut addrinfo = *out_addr;
    while !((*ai_tail).ai_next).is_null() {
        ai_tail = (*ai_tail).ai_next;
    }
    return getaddrinfo(host, s_port, hints, &mut (*ai_tail).ai_next);
}
unsafe extern "C" fn resolve(
    mut family: libc::c_int,
    mut flags: libc::c_int,
    mut host: *const libc::c_char,
    mut port: uint16_t,
    mut out_addr: *mut *mut addrinfo,
) -> libc::c_int {
    let mut hints: addrinfo = {
        let mut init = addrinfo {
            ai_flags: 0x20 as libc::c_int | flags,
            ai_family: family,
            ai_socktype: 0 as libc::c_int,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: 0 as *mut sockaddr,
            ai_canonname: 0 as *mut libc::c_char,
            ai_next: 0 as *mut addrinfo,
        };
        init
    };
    let mut s_port: [libc::c_char; 32] = [0; 32];
    *out_addr = 0 as *mut addrinfo;
    if port != 0 {
        hints.ai_flags |= 0x400 as libc::c_int;
        wget_snprintf(
            s_port.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%hu\0" as *const u8 as *const libc::c_char,
            port as libc::c_int,
        );
        if !host.is_null() {
            if family == 10 as libc::c_int {
                wget_debug_printf(
                    b"resolving [%s]:%s...\n\0" as *const u8 as *const libc::c_char,
                    host,
                    s_port.as_mut_ptr(),
                );
            } else {
                wget_debug_printf(
                    b"resolving %s:%s...\n\0" as *const u8 as *const libc::c_char,
                    host,
                    s_port.as_mut_ptr(),
                );
            }
        } else {
            wget_debug_printf(
                b"resolving :%s...\n\0" as *const u8 as *const libc::c_char,
                s_port.as_mut_ptr(),
            );
        }
    } else {
        wget_debug_printf(
            b"resolving %s...\n\0" as *const u8 as *const libc::c_char,
            host,
        );
    }
    let mut ret: libc::c_int = 0;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    ret = getaddrinfo_merging(
        host,
        (if port as libc::c_int != 0 {
            s_port.as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        }),
        &mut hints,
        out_addr,
    );
    if ret != 0 as libc::c_int {
        return ret;
    }
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    ret = getaddrinfo_merging(
        host,
        (if port as libc::c_int != 0 {
            s_port.as_mut_ptr()
        } else {
            0 as *mut libc::c_char
        }),
        &mut hints,
        out_addr,
    );
    if ret != 0 as libc::c_int {
        if !(*out_addr).is_null() {
            freeaddrinfo(*out_addr);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_cache_ip(
    mut dns: *mut wget_dns,
    mut ip: *const libc::c_char,
    mut name: *const libc::c_char,
    mut port: uint16_t,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut family: libc::c_int = 0;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    if dns.is_null() || ((*dns).cache).is_null() || name.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if wget_ip_is_family(ip, 1 as libc::c_int) {
        family = 2 as libc::c_int;
    } else if wget_ip_is_family(ip, 2 as libc::c_int) {
        family = 10 as libc::c_int;
    } else {
        return WGET_E_INVALID as libc::c_int
    }
    rc = resolve(family, 0x4 as libc::c_int, ip, port, &mut ai);
    if rc != 0 as libc::c_int {
        if family == 10 as libc::c_int {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to resolve '[%s]:%d': %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ip,
                port as libc::c_int,
                gai_strerror(rc),
            );
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to resolve '%s:%d': %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ip,
                port as libc::c_int,
                gai_strerror(rc),
            );
        }
        return WGET_E_UNKNOWN as libc::c_int;
    }
    rc = wget_dns_cache_add((*dns).cache, name, port, &mut ai);
    if rc < 0 as libc::c_int {
        freeaddrinfo(ai);
        return rc;
    }
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_resolve(
    mut dns: *mut wget_dns,
    mut host: *const libc::c_char,
    mut port: uint16_t,
    mut family: libc::c_int,
    mut preferred_family: libc::c_int,
) -> *mut addrinfo {
    let mut addrinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut adr: [libc::c_char; 1025] = [0; 1025];
    let mut sport: [libc::c_char; 32] = [0; 32];
    let mut before_millisecs: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut stats: wget_dns_stats_data = wget_dns_stats_data {
        hostname: 0 as *const libc::c_char,
        ip: 0 as *const libc::c_char,
        port: 0,
        dns_secs: 0,
    };
    if dns.is_null() {
        dns = &mut default_dns;
    }
    if ((*dns).stats_callback).is_some() {
        before_millisecs = wget_get_timemillis();
    }
    let mut tries: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = 3 as libc::c_int;
    while tries < max {
        if !((*dns).cache).is_null() {
            addrinfo = wget_dns_cache_get((*dns).cache, host, port);
            if !addrinfo.is_null() {
                return addrinfo;
            }
            wget_thread_mutex_lock((*dns).mutex);
            addrinfo = wget_dns_cache_get((*dns).cache, host, port);
            if !addrinfo.is_null() {
                wget_thread_mutex_unlock((*dns).mutex);
                return addrinfo;
            }
        }
        addrinfo = 0 as *mut addrinfo;
        rc = resolve(family, 0 as libc::c_int, host, port, &mut addrinfo);
        if rc == 0 as libc::c_int || rc != -(3 as libc::c_int) {
            break;
        }
        if tries < max - 1 as libc::c_int {
            if !((*dns).cache).is_null() {
                wget_thread_mutex_unlock((*dns).mutex);
            }
            wget_millisleep(100 as libc::c_int);
        }
        tries += 1;
        tries;
    }
    if ((*dns).stats_callback).is_some() {
        let mut after_millisecs: libc::c_longlong = wget_get_timemillis();
        stats.dns_secs = after_millisecs - before_millisecs;
        stats.hostname = host;
        stats.port = port;
    }
    if rc != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to resolve '%s' (%s)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            if !host.is_null() {
                host
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            gai_strerror(rc),
        );
        if !((*dns).cache).is_null() {
            wget_thread_mutex_unlock((*dns).mutex);
        }
        if ((*dns).stats_callback).is_some() {
            stats.ip = 0 as *const libc::c_char;
            ((*dns).stats_callback)
                .expect("non-null function pointer")(dns, &mut stats, (*dns).stats_ctx);
        }
        return 0 as *mut addrinfo;
    }
    if family == 0 as libc::c_int && preferred_family != 0 as libc::c_int {
        addrinfo = sort_preferred(addrinfo, preferred_family);
    }
    if ((*dns).stats_callback).is_some() {
        if getnameinfo(
            (*addrinfo).ai_addr,
            (*addrinfo).ai_addrlen,
            adr.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong as socklen_t,
            sport.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as socklen_t,
            1 as libc::c_int | 2 as libc::c_int,
        ) == 0 as libc::c_int
        {
            stats.ip = adr.as_mut_ptr();
        } else {
            stats.ip = b"???\0" as *const u8 as *const libc::c_char;
        }
        ((*dns).stats_callback)
            .expect("non-null function pointer")(dns, &mut stats, (*dns).stats_ctx);
    }
    if wget_logger_is_active(wget_get_logger(3 as libc::c_int)) {
        let mut ai: *mut addrinfo = addrinfo;
        while !ai.is_null() {
            rc = getnameinfo(
                (*ai).ai_addr,
                (*ai).ai_addrlen,
                adr.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 1025]>() as libc::c_ulong
                    as socklen_t,
                sport.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                    as socklen_t,
                1 as libc::c_int | 2 as libc::c_int,
            );
            if rc == 0 as libc::c_int {
                if (*ai).ai_family == 10 as libc::c_int {
                    wget_debug_printf(
                        b"has [%s]:%s\n\0" as *const u8 as *const libc::c_char,
                        adr.as_mut_ptr(),
                        sport.as_mut_ptr(),
                    );
                } else {
                    wget_debug_printf(
                        b"has %s:%s\n\0" as *const u8 as *const libc::c_char,
                        adr.as_mut_ptr(),
                        sport.as_mut_ptr(),
                    );
                }
            } else {
                wget_debug_printf(
                    b"has ??? (%s)\n\0" as *const u8 as *const libc::c_char,
                    gai_strerror(rc),
                );
            }
            ai = (*ai).ai_next;
        }
    }
    if !((*dns).cache).is_null() {
        rc = wget_dns_cache_add((*dns).cache, host, port, &mut addrinfo);
        wget_thread_mutex_unlock((*dns).mutex);
        if rc < 0 as libc::c_int {
            freeaddrinfo(addrinfo);
            return 0 as *mut addrinfo;
        }
    }
    return addrinfo;
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_freeaddrinfo(
    mut dns: *mut wget_dns,
    mut addrinfo: *mut *mut addrinfo,
) {
    if !addrinfo.is_null() && !(*addrinfo).is_null() {
        if dns.is_null() {
            dns = &mut default_dns;
        }
        if ((*dns).cache).is_null() {
            freeaddrinfo(*addrinfo);
            *addrinfo = 0 as *mut addrinfo;
        } else {
            *addrinfo = 0 as *mut addrinfo;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_dns_set_stats_callback(
    mut dns: *mut wget_dns,
    mut fn_0: Option::<wget_dns_stats_callback>,
    mut ctx: *mut libc::c_void,
) {
    if dns.is_null() {
        dns = &mut default_dns;
    }
    (*dns).stats_callback = fn_0;
    (*dns).stats_ctx = ctx;
}
