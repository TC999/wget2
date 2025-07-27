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
    pub type wget_list_st;
    pub type wget_vector_st;
    pub type wget_thread_st;
    pub type wget_thread_cond_st;
    pub type wget_http_connection_st;
    pub type wget_robots_st;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_truncate(path: *const libc::c_char, length: off_t) -> libc::c_int;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    fn wget_list_free(list: *mut *mut wget_list);
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add_memdup(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_clear(v: *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_http_free_challenges(challenges: *mut *mut wget_vector);
    fn wget_hash_file_fd(
        hashname: *const libc::c_char,
        fd: libc::c_int,
        digest_hex: *mut libc::c_char,
        digest_hex_size: size_t,
        offset: off_t,
        length: off_t,
    ) -> libc::c_int;
    fn wget_metalink_free(metalink: *mut *mut wget_metalink);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type uint16_t = __uint16_t;
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
pub type wget_list = wget_list_st;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_thread_id = libc::c_ulong;
pub type wget_thread = *mut wget_thread_st;
pub type wget_thread_cond = *mut wget_thread_cond_st;
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
pub type wget_http_connection = wget_http_connection_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink_hash {
    pub type_0: [libc::c_char; 16],
    pub hash_hex: [libc::c_char; 129],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink_piece {
    pub hash: wget_metalink_hash,
    pub position: off_t,
    pub length: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink {
    pub name: *const libc::c_char,
    pub mirrors: *mut wget_vector,
    pub hashes: *mut wget_vector,
    pub pieces: *mut wget_vector,
    pub size: off_t,
}
pub type wget_robots = wget_robots_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blacklist_entry {
    pub iri: *const wget_iri,
    pub local_filename: *mut libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct JOB {
    pub iri: *const wget_iri,
    pub original_url: *const wget_iri,
    pub referer: *const wget_iri,
    pub metalink: *mut wget_metalink,
    pub challenges: *mut wget_vector,
    pub proxy_challenges: *mut wget_vector,
    pub parts: *mut wget_vector,
    pub remaining_sig_ext: *mut wget_list,
    pub host: *mut HOST,
    pub blacklist_entry: *const blacklist_entry,
    pub sig_filename: *mut libc::c_char,
    pub sig_req: *mut libc::c_char,
    pub part: *mut PART,
    pub downloader: *mut DOWNLOADER,
    pub used_by: wget_thread_id,
    pub id: libc::c_ulonglong,
    pub parent_id: libc::c_ulonglong,
    pub retry_ts: libc::c_longlong,
    pub level: libc::c_int,
    pub redirection_level: libc::c_int,
    pub auth_failure_count: libc::c_int,
    pub mirror_pos: libc::c_int,
    pub piece_pos: libc::c_int,
    pub failures: libc::c_int,
    #[bitfield(name = "challenges_alloc", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "inuse", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "done", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "sitemap", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "robotstxt", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "head_first", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "requested_by_user", ty = "bool", bits = "6..=6")]
    #[bitfield(name = "ignore_patterns", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "http_fallback", ty = "bool", bits = "8..=8")]
    #[bitfield(name = "recursive_send_head", ty = "bool", bits = "9..=9")]
    #[bitfield(name = "redirect_get", ty = "bool", bits = "10..=10")]
    pub challenges_alloc_inuse_done_sitemap_robotstxt_head_first_requested_by_user_ignore_patterns_http_fallback_recursive_send_head_redirect_get: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct DOWNLOADER {
    pub thread: wget_thread,
    pub job: *mut JOB,
    pub conn: *mut wget_http_connection,
    pub buf: *mut libc::c_char,
    pub bufsize: size_t,
    pub id: libc::c_int,
    pub cond: wget_thread_cond,
    #[bitfield(name = "final_error", ty = "bool", bits = "0..=0")]
    pub final_error: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct PART {
    pub position: off_t,
    pub length: off_t,
    pub id: libc::c_int,
    pub used_by: wget_thread_id,
    #[bitfield(name = "inuse", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "done", ty = "bool", bits = "1..=1")]
    pub inuse_done: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct HOST {
    pub host: *const libc::c_char,
    pub robot_job: *mut JOB,
    pub robots: *mut wget_robots,
    pub queue: *mut wget_list,
    pub retry_ts: libc::c_longlong,
    pub qsize: libc::c_int,
    pub failures: libc::c_int,
    pub scheme: wget_iri_scheme,
    pub port: uint16_t,
    #[bitfield(name = "blocked", ty = "bool", bits = "0..=0")]
    pub blocked: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
#[no_mangle]
pub unsafe extern "C" fn job_free(mut job: *mut JOB) {
    if (*job).challenges_alloc() {
        wget_http_free_challenges(&mut (*job).challenges);
    }
    wget_http_free_challenges(&mut (*job).proxy_challenges);
    wget_metalink_free(&mut (*job).metalink);
    wget_vector_free(&mut (*job).parts);
    wget_list_free(&mut (*job).remaining_sig_ext);
    if !((*job).sig_req).is_null() {
        wget_free
            .expect("non-null function pointer")((*job).sig_req as *mut libc::c_void);
        (*job).sig_req = 0 as *mut libc::c_char;
    }
    if !((*job).sig_filename).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*job).sig_filename as *mut libc::c_void);
        (*job).sig_filename = 0 as *mut libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn job_create_parts(mut job: *mut JOB) {
    let mut part: PART = PART {
        position: 0,
        length: 0,
        id: 0,
        used_by: 0,
        inuse_done: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut metalink: *mut wget_metalink = 0 as *mut wget_metalink;
    let mut fsize: ssize_t = 0;
    metalink = (*job).metalink;
    if metalink.is_null() {
        return;
    }
    memset(
        &mut part as *mut PART as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<PART>() as libc::c_ulong,
    );
    if ((*job).parts).is_null() {
        (*job).parts = wget_vector_create(wget_vector_size((*metalink).pieces), None);
    } else {
        wget_vector_clear((*job).parts);
    }
    fsize = (*metalink).size;
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size((*metalink).pieces) {
        let mut piece: *mut wget_metalink_piece = wget_vector_get((*metalink).pieces, it)
            as *mut wget_metalink_piece;
        if fsize >= (*piece).length {
            part.length = (*piece).length;
        } else {
            part.length = fsize;
        }
        part.id = it + 1 as libc::c_int;
        wget_vector_add_memdup(
            (*job).parts,
            &mut part as *mut PART as *const libc::c_void,
            ::core::mem::size_of::<PART>() as libc::c_ulong,
        );
        part.position += part.length;
        fsize -= (*piece).length;
        it += 1;
        it;
    }
}
unsafe extern "C" fn check_piece_hash(
    mut hash: *mut wget_metalink_hash,
    mut fd: libc::c_int,
    mut offset: off_t,
    mut length: size_t,
) -> libc::c_int {
    let mut sum: [libc::c_char; 129] = [0; 129];
    if wget_hash_file_fd(
        ((*hash).type_0).as_mut_ptr(),
        fd,
        sum.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 129]>() as libc::c_ulong,
        offset,
        length as off_t,
    ) != -(1 as libc::c_int)
    {
        return (wget_strcasecmp_ascii(sum.as_mut_ptr(), ((*hash).hash_hex).as_mut_ptr())
            == 0) as libc::c_int;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn check_file_fd(
    mut hash: *mut wget_metalink_hash,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut sum: [libc::c_char; 129] = [0; 129];
    if wget_hash_file_fd(
        ((*hash).type_0).as_mut_ptr(),
        fd,
        sum.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 129]>() as libc::c_ulong,
        0 as libc::c_int as off_t,
        0 as libc::c_int as off_t,
    ) != -(1 as libc::c_int)
    {
        return (wget_strcasecmp_ascii(sum.as_mut_ptr(), ((*hash).hash_hex).as_mut_ptr())
            == 0) as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn job_validate_file(mut job: *mut JOB) -> libc::c_int {
    let mut part: PART = PART {
        position: 0,
        length: 0,
        id: 0,
        used_by: 0,
        inuse_done: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut metalink: *mut wget_metalink = 0 as *mut wget_metalink;
    let mut fsize: off_t = 0;
    let mut real_fsize: off_t = 0 as libc::c_int as off_t;
    let mut fd: libc::c_int = 0;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    metalink = (*job).metalink;
    if metalink.is_null() {
        return 0 as libc::c_int;
    }
    memset(
        &mut part as *mut PART as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<PART>() as libc::c_ulong,
    );
    if ((*metalink).pieces).is_null() {
        let mut piece: wget_metalink_piece = wget_metalink_piece {
            hash: wget_metalink_hash {
                type_0: [0; 16],
                hash_hex: [0; 129],
            },
            position: 0,
            length: 0,
        };
        let mut hash: *mut wget_metalink_hash = wget_vector_get(
            (*metalink).hashes,
            0 as libc::c_int,
        ) as *mut wget_metalink_hash;
        if hash.is_null() {
            return 1 as libc::c_int;
        }
        piece.length = (*metalink).size;
        piece.position = 0 as libc::c_int as off_t;
        wget_strscpy(
            (piece.hash.type_0).as_mut_ptr(),
            ((*hash).type_0).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        );
        wget_strscpy(
            (piece.hash.hash_hex).as_mut_ptr(),
            ((*hash).hash_hex).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 129]>() as libc::c_ulong,
        );
        (*metalink).pieces = wget_vector_create(1 as libc::c_int, None);
        wget_vector_add_memdup(
            (*metalink).pieces,
            &mut piece as *mut wget_metalink_piece as *const libc::c_void,
            ::core::mem::size_of::<wget_metalink_piece>() as libc::c_ulong,
        );
    }
    if ((*job).parts).is_null() {
        (*job).parts = wget_vector_create(wget_vector_size((*metalink).pieces), None);
    } else {
        wget_vector_clear((*job).parts);
    }
    fsize = (*metalink).size;
    if wget_vector_size((*metalink).hashes) == 0 as libc::c_int {
        if stat((*metalink).name, &mut st) == 0 as libc::c_int && st.st_size == fsize {
            return 1 as libc::c_int;
        }
    }
    if stat((*metalink).name, &mut st) == 0 as libc::c_int
        && {
            real_fsize = st.st_size;
            real_fsize > fsize
        }
    {
        if wget_truncate((*metalink).name, fsize) != WGET_E_SUCCESS as libc::c_int {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to truncate %s\n from %llu to %llu bytes\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*metalink).name,
                st.st_size as libc::c_ulonglong,
                fsize as libc::c_ulonglong,
            );
        } else {
            real_fsize = fsize;
        }
    }
    if wget_vector_size((*metalink).hashes) > 0 as libc::c_int
        && {
            fd = open((*metalink).name, 0 as libc::c_int | 0 as libc::c_int);
            fd != -(1 as libc::c_int)
        }
    {
        let mut rc: libc::c_int = -(1 as libc::c_int);
        let mut it: libc::c_int = 0 as libc::c_int;
        while *__errno_location() != 4 as libc::c_int
            && it < wget_vector_size((*metalink).hashes)
        {
            let mut hash_0: *mut wget_metalink_hash = wget_vector_get(
                (*metalink).hashes,
                it,
            ) as *mut wget_metalink_hash;
            rc = check_file_fd(hash_0, fd);
            if !(rc == -(1 as libc::c_int)) {
                break;
            }
            it += 1;
            it;
        }
        if rc == 1 as libc::c_int {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Checksum OK for '%s'\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*metalink).name,
            );
            close(fd);
            return 1 as libc::c_int;
        }
        if rc == -(1 as libc::c_int) {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to build checksum, assuming file to be OK\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            close(fd);
            return 1 as libc::c_int;
        }
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Bad checksum for '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*metalink).name,
        );
        let mut it_0: libc::c_int = 0 as libc::c_int;
        while *__errno_location() != 4 as libc::c_int
            && it_0 < wget_vector_size((*metalink).pieces)
        {
            let mut piece_0: *mut wget_metalink_piece = wget_vector_get(
                (*metalink).pieces,
                it_0,
            ) as *mut wget_metalink_piece;
            let mut hash_1: *mut wget_metalink_hash = &mut (*piece_0).hash;
            if fsize >= (*piece_0).length {
                part.length = (*piece_0).length;
            } else {
                part.length = fsize as size_t as off_t;
            }
            part.id = it_0 + 1 as libc::c_int;
            if check_piece_hash(hash_1, fd, part.position, part.length as size_t)
                != 1 as libc::c_int
            {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Piece %d/%d not OK - requeuing\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    it_0 + 1 as libc::c_int,
                    wget_vector_size((*metalink).pieces),
                );
                wget_vector_add_memdup(
                    (*job).parts,
                    &mut part as *mut PART as *const libc::c_void,
                    ::core::mem::size_of::<PART>() as libc::c_ulong,
                );
                wget_debug_printf(
                    b"  need to download %llu bytes from pos=%llu\n\0" as *const u8
                        as *const libc::c_char,
                    part.length as libc::c_ulonglong,
                    part.position as libc::c_ulonglong,
                );
            }
            part.position += part.length;
            fsize -= (*piece_0).length;
            it_0 += 1;
            it_0;
        }
        close(fd);
    } else {
        let mut it_1: libc::c_int = 0 as libc::c_int;
        while it_1 < wget_vector_size((*metalink).pieces) {
            let mut piece_1: *mut wget_metalink_piece = wget_vector_get(
                (*metalink).pieces,
                it_1,
            ) as *mut wget_metalink_piece;
            if fsize >= (*piece_1).length {
                part.length = (*piece_1).length;
            } else {
                part.length = fsize;
            }
            part.id = it_1 + 1 as libc::c_int;
            if real_fsize < part.position + part.length {
                let mut idx: libc::c_int = wget_vector_add_memdup(
                    (*job).parts,
                    &mut part as *mut PART as *const libc::c_void,
                    ::core::mem::size_of::<PART>() as libc::c_ulong,
                );
                if real_fsize > part.position {
                    let mut p: *mut PART = wget_vector_get((*job).parts, idx)
                        as *mut PART;
                    (*p).position = real_fsize;
                    (*p).length = part.position + part.length - real_fsize;
                }
            }
            part.position += part.length;
            fsize -= (*piece_1).length;
            it_1 += 1;
            it_1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn job_init(
    mut job: *mut JOB,
    mut blacklistp: *mut blacklist_entry,
    mut http_fallback: bool,
) -> *mut JOB {
    static mut jobid: libc::c_ulonglong = 0;
    if job.is_null() {
        job = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<JOB>() as libc::c_ulong,
        ) as *mut JOB;
    } else {
        memset(
            job as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<JOB>() as libc::c_ulong,
        );
    }
    (*job).blacklist_entry = blacklistp;
    (*job).iri = (*blacklistp).iri;
    (*job).set_http_fallback(http_fallback);
    jobid = jobid.wrapping_add(1);
    (*job).id = jobid;
    return job;
}
