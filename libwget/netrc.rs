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
    pub type wget_hashmap_st;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_getline(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        fp: *mut FILE,
    ) -> ssize_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_hashmap_create(
        max: libc::c_int,
        hash: Option::<wget_hashmap_hash_fn>,
        cmp: Option::<wget_hashmap_compare_fn>,
    ) -> *mut wget_hashmap;
    fn wget_hashmap_put(
        h: *mut wget_hashmap,
        key: *const libc::c_void,
        value: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_size(h: *const wget_hashmap) -> libc::c_int;
    fn wget_hashmap_free(h: *mut *mut wget_hashmap);
    fn wget_hashmap_get(
        h: *const wget_hashmap,
        key: *const libc::c_void,
        value: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_set_key_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_key_destructor>,
    );
    fn wget_hashmap_set_value_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_value_destructor>,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type uint16_t = __uint16_t;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed_0 = -12;
pub const WGET_E_IO: C2RustUnnamed_0 = -11;
pub const WGET_E_OPEN: C2RustUnnamed_0 = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed_0 = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed_0 = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed_0 = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed_0 = -6;
pub const WGET_E_CONNECT: C2RustUnnamed_0 = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed_0 = -4;
pub const WGET_E_INVALID: C2RustUnnamed_0 = -3;
pub const WGET_E_MEMORY: C2RustUnnamed_0 = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed_0 = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed_0 = 0;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_hash_fn = unsafe extern "C" fn(
    *const libc::c_void,
) -> libc::c_uint;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_netrc_db_st {
    pub machines: *mut wget_hashmap,
}
pub type wget_netrc_db = wget_netrc_db_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_netrc_st {
    pub host: *const libc::c_char,
    pub login: *const libc::c_char,
    pub password: *const libc::c_char,
    pub port: uint16_t,
    #[bitfield(name = "force", ty = "bool", bits = "0..=0")]
    pub force: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type wget_netrc = wget_netrc_st;
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
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
unsafe extern "C" fn hash_netrc(mut netrc: *const wget_netrc) -> libc::c_uint {
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *const libc::c_uchar = 0 as *const libc::c_uchar;
    p = (*netrc).host as *mut libc::c_uchar;
    while *p != 0 {
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*p as libc::c_uint);
        p = p.offset(1);
        p;
    }
    return hash;
}
unsafe extern "C" fn compare_netrc(
    mut h1: *const wget_netrc,
    mut h2: *const wget_netrc,
) -> libc::c_int {
    return wget_strcmp((*h1).host, (*h2).host);
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_init(mut netrc: *mut wget_netrc) -> *mut wget_netrc {
    if netrc.is_null() {
        netrc = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_netrc>() as libc::c_ulong,
        ) as *mut wget_netrc;
        if netrc.is_null() {
            return 0 as *mut wget_netrc;
        }
    } else {
        memset(
            netrc as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_netrc>() as libc::c_ulong,
        );
    }
    return netrc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_deinit(mut netrc: *mut wget_netrc) {
    if !netrc.is_null() {
        if !((*netrc).host).is_null() {
            wget_free
                .expect("non-null function pointer")((*netrc).host as *mut libc::c_void);
            (*netrc).host = 0 as *const libc::c_char;
        }
        if !((*netrc).login).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*netrc).login as *mut libc::c_void);
            (*netrc).login = 0 as *const libc::c_char;
        }
        if !((*netrc).password).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*netrc).password as *mut libc::c_void);
            (*netrc).password = 0 as *const libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_free(mut netrc: *mut wget_netrc) {
    if !netrc.is_null() {
        wget_netrc_deinit(netrc);
        if !netrc.is_null() {
            wget_free.expect("non-null function pointer")(netrc as *mut libc::c_void);
            netrc = 0 as *mut wget_netrc;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_new(
    mut machine: *const libc::c_char,
    mut login: *const libc::c_char,
    mut password: *const libc::c_char,
) -> *mut wget_netrc {
    let mut netrc: *mut wget_netrc = wget_netrc_init(0 as *mut wget_netrc);
    if !netrc.is_null() {
        (*netrc).host = wget_strdup(machine);
        (*netrc).login = wget_strdup(login);
        (*netrc).password = wget_strdup(password);
    }
    return netrc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_get(
    mut netrc_db: *const wget_netrc_db,
    mut host: *const libc::c_char,
) -> *mut wget_netrc {
    if !netrc_db.is_null() {
        let mut netrc: wget_netrc = wget_netrc_st {
            host: 0 as *const libc::c_char,
            login: 0 as *const libc::c_char,
            password: 0 as *const libc::c_char,
            port: 0,
            force: [0; 1],
            c2rust_padding: [0; 5],
        };
        let mut netrcp: *mut wget_netrc = 0 as *mut wget_netrc;
        netrc.host = host;
        if wget_hashmap_get(
            (*netrc_db).machines,
            &mut netrc as *mut wget_netrc as *const libc::c_void,
            &mut netrcp as *mut *mut wget_netrc as *mut *mut libc::c_void,
        ) != 0
        {
            return netrcp;
        }
    }
    return 0 as *mut wget_netrc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_db_init(
    mut netrc_db: *mut wget_netrc_db,
) -> *mut wget_netrc_db {
    let mut machines: *mut wget_hashmap = wget_hashmap_create(
        16 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const wget_netrc) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_netrc as unsafe extern "C" fn(*const wget_netrc) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const wget_netrc, *const wget_netrc) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                compare_netrc
                    as unsafe extern "C" fn(
                        *const wget_netrc,
                        *const wget_netrc,
                    ) -> libc::c_int,
            ),
        ),
    );
    if machines.is_null() {
        return 0 as *mut wget_netrc_db;
    }
    if netrc_db.is_null() {
        netrc_db = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_netrc_db>() as libc::c_ulong,
        ) as *mut wget_netrc_db;
        if netrc_db.is_null() {
            wget_hashmap_free(&mut machines);
            return 0 as *mut wget_netrc_db;
        }
    } else {
        memset(
            netrc_db as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_netrc_db>() as libc::c_ulong,
        );
    }
    wget_hashmap_set_key_destructor(
        machines,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_netrc) -> ()>,
            Option::<wget_hashmap_key_destructor>,
        >(Some(wget_netrc_free as unsafe extern "C" fn(*mut wget_netrc) -> ())),
    );
    wget_hashmap_set_value_destructor(
        machines,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut wget_netrc) -> ()>,
            Option::<wget_hashmap_value_destructor>,
        >(Some(wget_netrc_free as unsafe extern "C" fn(*mut wget_netrc) -> ())),
    );
    (*netrc_db).machines = machines;
    return netrc_db;
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_db_deinit(mut netrc_db: *mut wget_netrc_db) {
    if !netrc_db.is_null() {
        wget_hashmap_free(&mut (*netrc_db).machines);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_db_free(mut netrc_db: *mut *mut wget_netrc_db) {
    if !netrc_db.is_null() {
        wget_netrc_db_deinit(*netrc_db);
        if !(*netrc_db).is_null() {
            wget_free
                .expect("non-null function pointer")(*netrc_db as *mut libc::c_void);
            *netrc_db = 0 as *mut wget_netrc_db;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_db_add(
    mut netrc_db: *mut wget_netrc_db,
    mut netrc: *mut wget_netrc,
) {
    if netrc.is_null() {
        return;
    }
    if netrc_db.is_null() {
        wget_netrc_free(netrc);
        return;
    }
    wget_debug_printf(
        b"add .netrc %s (login=%s, password=*)\n\0" as *const u8 as *const libc::c_char,
        (*netrc).host,
        (*netrc).login,
    );
    wget_hashmap_put(
        (*netrc_db).machines,
        netrc as *const libc::c_void,
        netrc as *const libc::c_void,
    );
}
unsafe extern "C" fn unescape_password(
    mut p: *const libc::c_char,
    mut n: size_t,
) -> *const libc::c_char {
    let mut dst: *mut libc::c_char = wget_malloc(
        n.wrapping_add(1 as libc::c_int as size_t),
    ) as *mut libc::c_char;
    let mut bufp: *mut libc::c_char = dst;
    if dst.is_null() {
        return dst;
    }
    while n != 0 {
        if *p as libc::c_int == '\\' as i32 {
            p = p.offset(1);
            p;
        }
        let fresh0 = p;
        p = p.offset(1);
        let fresh1 = bufp;
        bufp = bufp.offset(1);
        *fresh1 = *fresh0;
        n = n.wrapping_sub(1);
        n;
    }
    *bufp = 0 as libc::c_int as libc::c_char;
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn wget_netrc_db_load(
    mut netrc_db: *mut wget_netrc_db,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    if netrc_db.is_null() || fname.is_null() || *fname == 0 {
        return WGET_E_INVALID as libc::c_int;
    }
    fp = rpl_fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return WGET_E_OPEN as libc::c_int;
    }
    let mut netrc: wget_netrc = wget_netrc_st {
        host: 0 as *const libc::c_char,
        login: 0 as *const libc::c_char,
        password: 0 as *const libc::c_char,
        port: 0,
        force: [0; 1],
        c2rust_padding: [0; 5],
    };
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut buflen: ssize_t = 0;
    let mut nentries: libc::c_int = 0 as libc::c_int;
    let mut in_macdef: bool = 0 as libc::c_int != 0;
    let mut in_machine: bool = 0 as libc::c_int != 0;
    let mut quoted: bool = 0 as libc::c_int != 0;
    loop {
        buflen = wget_getline(&mut buf, &mut bufsize, fp);
        if !(buflen >= 0 as libc::c_int as ssize_t) {
            break;
        }
        linep = buf;
        while *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            linep = linep.offset(1);
            linep;
        }
        if *linep as libc::c_int == '#' as i32 {
            continue;
        }
        while buflen > 0 as libc::c_int as ssize_t
            && (*buf.offset(buflen as isize) as libc::c_int == '\n' as i32
                || *buf.offset(buflen as isize) as libc::c_int == '\r' as i32)
        {
            buflen -= 1;
            *buf.offset(buflen as isize) = 0 as libc::c_int as libc::c_char;
        }
        if *linep == 0 {
            in_macdef = 0 as libc::c_int != 0;
        } else {
            if in_macdef {
                continue;
            }
            let mut current_block_75: u64;
            loop {
                if !key.is_null() {
                    wget_free
                        .expect("non-null function pointer")(key as *mut libc::c_void);
                    key = 0 as *mut libc::c_char;
                }
                while *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                    as libc::c_int
                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    linep = linep.offset(1);
                    linep;
                }
                p = linep;
                while *linep as libc::c_int != 0
                    && *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                        as libc::c_int
                        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    linep = linep.offset(1);
                    linep;
                }
                key = wget_strmemdup(
                    p as *const libc::c_void,
                    linep.offset_from(p) as libc::c_long as size_t,
                );
                if key.is_null() {
                    if !buf.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(buf as *mut libc::c_void);
                        buf = 0 as *mut libc::c_char;
                    }
                    rpl_fclose(fp);
                    return WGET_E_MEMORY as libc::c_int;
                }
                if strcmp(key, b"machine\0" as *const u8 as *const libc::c_char) == 0
                    || strcmp(key, b"default\0" as *const u8 as *const libc::c_char) == 0
                {
                    if in_machine {
                        wget_netrc_db_add(
                            netrc_db,
                            wget_memdup(
                                &mut netrc as *mut wget_netrc as *const libc::c_void,
                                ::core::mem::size_of::<wget_netrc>() as libc::c_ulong,
                            ) as *mut wget_netrc,
                        );
                    }
                    wget_netrc_init(&mut netrc);
                    in_machine = 1 as libc::c_int != 0;
                    if strcmp(key, b"default\0" as *const u8 as *const libc::c_char) == 0
                    {
                        netrc
                            .host = wget_strdup(
                            b"default\0" as *const u8 as *const libc::c_char,
                        );
                        current_block_75 = 17860125682698302841;
                    } else {
                        current_block_75 = 15090052786889560393;
                    }
                } else if !in_machine {
                    current_block_75 = 17860125682698302841;
                } else {
                    current_block_75 = 15090052786889560393;
                }
                match current_block_75 {
                    15090052786889560393 => {
                        while *(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                            as libc::c_int
                            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                            != 0
                        {
                            linep = linep.offset(1);
                            linep;
                        }
                        if *linep as libc::c_int == '"' as i32 {
                            quoted = 1 as libc::c_int != 0;
                            linep = linep.offset(1);
                            linep;
                        }
                        let mut escaped: libc::c_int = 0 as libc::c_int;
                        p = linep;
                        while *linep as libc::c_int != 0
                            && (if quoted as libc::c_int != 0 {
                                (*linep as libc::c_int != '"' as i32) as libc::c_int
                            } else {
                                (*(*__ctype_b_loc()).offset(*linep as libc::c_int as isize)
                                    as libc::c_int
                                    & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0) as libc::c_int
                            }) != 0
                        {
                            if *linep as libc::c_int == '\\' as i32 {
                                escaped += 1;
                                escaped;
                                linep = linep.offset(1);
                                linep;
                            }
                            linep = linep.offset(1);
                            linep;
                        }
                        if strcmp(key, b"machine\0" as *const u8 as *const libc::c_char)
                            == 0
                        {
                            if (netrc.host).is_null() {
                                netrc
                                    .host = wget_strmemdup(
                                    p as *const libc::c_void,
                                    linep.offset_from(p) as libc::c_long as size_t,
                                );
                            }
                        } else if strcmp(
                            key,
                            b"login\0" as *const u8 as *const libc::c_char,
                        ) == 0
                            || strcmp(key, b"user\0" as *const u8 as *const libc::c_char)
                                == 0
                        {
                            if (netrc.login).is_null() {
                                netrc
                                    .login = wget_strmemdup(
                                    p as *const libc::c_void,
                                    linep.offset_from(p) as libc::c_long as size_t,
                                );
                            }
                        } else if strcmp(
                            key,
                            b"password\0" as *const u8 as *const libc::c_char,
                        ) == 0
                            || strcmp(
                                key,
                                b"passwd\0" as *const u8 as *const libc::c_char,
                            ) == 0
                        {
                            if (netrc.password).is_null() {
                                if escaped == 0 {
                                    netrc
                                        .password = wget_strmemdup(
                                        p as *const libc::c_void,
                                        linep.offset_from(p) as libc::c_long as size_t,
                                    );
                                } else {
                                    netrc
                                        .password = unescape_password(
                                        p,
                                        (linep.offset_from(p) as libc::c_long
                                            - escaped as libc::c_long) as size_t,
                                    );
                                }
                            }
                        } else if strcmp(
                            key,
                            b"port\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            netrc.port = atoi(p) as uint16_t;
                        } else if strcmp(
                            key,
                            b"force\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            netrc
                                .set_force(
                                    wget_strncasecmp_ascii(
                                        b"yes\0" as *const u8 as *const libc::c_char,
                                        p,
                                        3 as libc::c_int as size_t,
                                    ) == 0,
                                );
                        } else if strcmp(
                            key,
                            b"macdef\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            in_macdef = 1 as libc::c_int != 0;
                        }
                        if quoted as libc::c_int != 0
                            && *linep as libc::c_int == '"' as i32
                        {
                            linep = linep.offset(1);
                            linep;
                        }
                    }
                    _ => {}
                }
                if !(*linep != 0) {
                    break;
                }
            }
            if !key.is_null() {
                wget_free.expect("non-null function pointer")(key as *mut libc::c_void);
                key = 0 as *mut libc::c_char;
            }
        }
    }
    if in_machine {
        wget_netrc_db_add(
            netrc_db,
            wget_memdup(
                &mut netrc as *mut wget_netrc as *const libc::c_void,
                ::core::mem::size_of::<wget_netrc>() as libc::c_ulong,
            ) as *mut wget_netrc,
        );
    }
    if !buf.is_null() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    rpl_fclose(fp);
    nentries = wget_hashmap_size((*netrc_db).machines);
    wget_debug_printf(
        b"loaded %d .netrc %s\n\0" as *const u8 as *const libc::c_char,
        nentries,
        if nentries != 1 as libc::c_int {
            b"entries\0" as *const u8 as *const libc::c_char
        } else {
            b"entry\0" as *const u8 as *const libc::c_char
        },
    );
    return nentries;
}
