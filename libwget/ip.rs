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
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[no_mangle]
pub unsafe extern "C" fn wget_ip_is_family(
    mut host: *const libc::c_char,
    mut family: libc::c_int,
) -> bool {
    let mut dst: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    if host.is_null() {
        return 0 as libc::c_int != 0;
    }
    match family {
        1 => {
            return inet_pton(
                2 as libc::c_int,
                host,
                &mut dst as *mut sockaddr_storage as *mut in_addr as *mut libc::c_void,
            ) == 1 as libc::c_int;
        }
        2 => {
            return inet_pton(
                10 as libc::c_int,
                host,
                &mut dst as *mut sockaddr_storage as *mut in6_addr as *mut libc::c_void,
            ) == 1 as libc::c_int;
        }
        _ => return 0 as libc::c_int != 0,
    };
}
