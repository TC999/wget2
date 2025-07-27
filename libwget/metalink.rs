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
    pub type wget_vector_st;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemcpy_a(
        s: *mut libc::c_char,
        ssize: size_t,
        m: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add_memdup(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_setcmpfunc(
        v: *mut wget_vector,
        cmp: Option::<wget_vector_compare_fn>,
    );
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_vector_sort(v: *mut wget_vector);
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_supported(iri: *const wget_iri) -> bool;
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_xml_parse_buffer(
        buf: *const libc::c_char,
        callback: Option::<wget_xml_callback>,
        user_ctx: *mut libc::c_void,
        hints: libc::c_int,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __off_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub type wget_xml_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    libc::c_int,
    *const libc::c_char,
    *const libc::c_char,
    *const libc::c_char,
    size_t,
    size_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink_mirror {
    pub iri: *const wget_iri,
    pub priority: libc::c_int,
    pub location: [libc::c_char; 8],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct metalink_context {
    pub metalink: *mut wget_metalink,
    pub priority: libc::c_int,
    pub hash: [libc::c_char; 128],
    pub hash_type: [libc::c_char; 16],
    pub location: [libc::c_char; 8],
    pub length: libc::c_longlong,
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
unsafe extern "C" fn mirror_free(mut mirror: *mut libc::c_void) {
    let mut m: *mut wget_metalink_mirror = mirror as *mut wget_metalink_mirror;
    if !m.is_null() {
        wget_iri_free(&mut (*m).iri as *mut *const wget_iri as *mut *mut wget_iri);
        if !m.is_null() {
            wget_free.expect("non-null function pointer")(m as *mut libc::c_void);
            m = 0 as *mut wget_metalink_mirror;
        }
    }
}
unsafe extern "C" fn add_piece(
    mut ctx: *mut metalink_context,
    mut value: *const libc::c_char,
) {
    let mut metalink: *mut wget_metalink = (*ctx).metalink;
    sscanf(
        value,
        b"%127s\0" as *const u8 as *const libc::c_char,
        ((*ctx).hash).as_mut_ptr(),
    );
    if (*ctx).length != 0 && *((*ctx).hash_type).as_mut_ptr() as libc::c_int != 0
        && *((*ctx).hash).as_mut_ptr() as libc::c_int != 0
    {
        let mut piece: wget_metalink_piece = wget_metalink_piece {
            hash: wget_metalink_hash {
                type_0: [0; 16],
                hash_hex: [0; 129],
            },
            position: 0,
            length: 0,
        };
        let mut piecep: *mut wget_metalink_piece = 0 as *mut wget_metalink_piece;
        if ((*metalink).pieces).is_null() {
            (*metalink).pieces = wget_vector_create(32 as libc::c_int, None);
        }
        piece.length = (*ctx).length as off_t;
        wget_strscpy(
            (piece.hash.type_0).as_mut_ptr(),
            ((*ctx).hash_type).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        );
        wget_strscpy(
            (piece.hash.hash_hex).as_mut_ptr(),
            ((*ctx).hash).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 129]>() as libc::c_ulong,
        );
        piecep = wget_vector_get(
            (*metalink).pieces,
            wget_vector_size((*metalink).pieces) - 1 as libc::c_int,
        ) as *mut wget_metalink_piece;
        if !piecep.is_null() && (*piecep).length > 0 as libc::c_int as off_t {
            if (*piecep).position
                <= 9223372036854775807 as libc::c_long - (*piecep).length
            {
                piece.position = (*piecep).position + (*piecep).length;
            } else {
                piece.position = 0 as libc::c_int as off_t;
            }
        } else {
            piece.position = 0 as libc::c_int as off_t;
        }
        wget_vector_add_memdup(
            (*metalink).pieces,
            &mut piece as *mut wget_metalink_piece as *const libc::c_void,
            ::core::mem::size_of::<wget_metalink_piece>() as libc::c_ulong,
        );
    }
    *((*ctx).hash).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn add_file_hash(
    mut ctx: *mut metalink_context,
    mut value: *const libc::c_char,
) {
    let mut metalink: *mut wget_metalink = (*ctx).metalink;
    sscanf(
        value,
        b"%127s\0" as *const u8 as *const libc::c_char,
        ((*ctx).hash).as_mut_ptr(),
    );
    if *((*ctx).hash_type).as_mut_ptr() as libc::c_int != 0
        && *((*ctx).hash).as_mut_ptr() as libc::c_int != 0
    {
        let mut hash: wget_metalink_hash = {
            let mut init = wget_metalink_hash {
                type_0: [
                    0 as libc::c_int as libc::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                hash_hex: [0; 129],
            };
            init
        };
        wget_strscpy(
            (hash.type_0).as_mut_ptr(),
            ((*ctx).hash_type).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        );
        wget_strscpy(
            (hash.hash_hex).as_mut_ptr(),
            ((*ctx).hash).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 129]>() as libc::c_ulong,
        );
        if ((*metalink).hashes).is_null() {
            (*metalink).hashes = wget_vector_create(4 as libc::c_int, None);
        }
        wget_vector_add_memdup(
            (*metalink).hashes,
            &mut hash as *mut wget_metalink_hash as *const libc::c_void,
            ::core::mem::size_of::<wget_metalink_hash>() as libc::c_ulong,
        );
    }
    let ref mut fresh0 = *((*ctx).hash).as_mut_ptr();
    *fresh0 = 0 as libc::c_int as libc::c_char;
    *((*ctx).hash_type).as_mut_ptr() = *fresh0;
}
unsafe extern "C" fn add_mirror(
    mut ctx: *mut metalink_context,
    mut value: *const libc::c_char,
) {
    let mut iri: *mut wget_iri = wget_iri_parse(value, 0 as *const libc::c_char);
    if iri.is_null() {
        return;
    }
    if !wget_iri_supported(iri) {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Mirror scheme not supported: '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            value,
        );
        wget_iri_free(&mut iri);
        return;
    }
    let mut metalink: *mut wget_metalink = (*ctx).metalink;
    let mut mirror: *mut wget_metalink_mirror = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_metalink_mirror>() as libc::c_ulong,
    ) as *mut wget_metalink_mirror;
    if !mirror.is_null() {
        wget_strscpy(
            ((*mirror).location).as_mut_ptr(),
            ((*ctx).location).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
        (*mirror).priority = (*ctx).priority;
        (*mirror).iri = iri;
        if ((*metalink).mirrors).is_null() {
            (*metalink).mirrors = wget_vector_create(4 as libc::c_int, None);
            wget_vector_set_destructor(
                (*metalink).mirrors,
                Some(mirror_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
        }
        wget_vector_add((*metalink).mirrors, mirror as *const libc::c_void);
    }
    *((*ctx).location).as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    (*ctx).priority = 999999 as libc::c_int;
}
unsafe extern "C" fn metalink_parse(
    mut context: *mut libc::c_void,
    mut flags: libc::c_int,
    mut dir: *const libc::c_char,
    mut attr: *const libc::c_char,
    mut val: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut metalink_context = context as *mut metalink_context;
    let mut valuebuf: [libc::c_char; 1024] = [0; 1024];
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    if flags
        & ((1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) == 0
    {
        return;
    }
    if wget_strncasecmp_ascii(
        dir,
        b"/metalink/file\0" as *const u8 as *const libc::c_char,
        14 as libc::c_int as size_t,
    ) != 0
    {
        return;
    }
    dir = dir.offset(14 as libc::c_int as isize);
    value = wget_strmemcpy_a(
        valuebuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        (if !val.is_null() { val } else { b"\0" as *const u8 as *const libc::c_char })
            as *const libc::c_void,
        len,
    ) as *const libc::c_char;
    if value.is_null() {
        return;
    }
    if wget_strncasecmp_ascii(
        dir,
        b"s/file\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    ) == 0
    {
        dir = dir.offset(6 as libc::c_int as isize);
        if !attr.is_null() {
            if *dir as libc::c_int == 0 as libc::c_int {
                if ((*(*ctx).metalink).name).is_null()
                    && wget_strcasecmp_ascii(
                        attr,
                        b"name\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    (*(*ctx).metalink).name = wget_strdup(value);
                }
            } else if wget_strcasecmp_ascii(
                dir,
                b"/verification/pieces\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if wget_strcasecmp_ascii(
                    attr,
                    b"type\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sscanf(
                        value,
                        b"%15s\0" as *const u8 as *const libc::c_char,
                        ((*ctx).hash_type).as_mut_ptr(),
                    );
                } else if wget_strcasecmp_ascii(
                    attr,
                    b"length\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    (*ctx).length = atoll(value);
                }
            } else if wget_strcasecmp_ascii(
                dir,
                b"/verification/hash\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if wget_strcasecmp_ascii(
                    attr,
                    b"type\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sscanf(
                        value,
                        b"%15s\0" as *const u8 as *const libc::c_char,
                        ((*ctx).hash_type).as_mut_ptr(),
                    );
                }
            } else if wget_strcasecmp_ascii(
                dir,
                b"/resources/url\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if wget_strcasecmp_ascii(
                    attr,
                    b"location\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sscanf(
                        value,
                        b" %2[a-zA-Z]\0" as *const u8 as *const libc::c_char,
                        ((*ctx).location).as_mut_ptr(),
                    );
                } else if wget_strcasecmp_ascii(
                    attr,
                    b"preference\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    sscanf(
                        value,
                        b" %6d\0" as *const u8 as *const libc::c_char,
                        &mut (*ctx).priority as *mut libc::c_int,
                    );
                    if (*ctx).priority < 1 as libc::c_int
                        || (*ctx).priority > 999999 as libc::c_int
                    {
                        (*ctx).priority = 999999 as libc::c_int;
                    }
                }
            }
        } else if wget_strcasecmp_ascii(
            dir,
            b"/verification/pieces/hash\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            add_piece(ctx, value);
        } else if wget_strcasecmp_ascii(
            dir,
            b"/verification/hash\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            add_file_hash(ctx, value);
        } else if wget_strcasecmp_ascii(
            dir,
            b"/size\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*(*ctx).metalink).size = atoll(value) as off_t;
        } else if wget_strcasecmp_ascii(
            dir,
            b"/resources/url\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            add_mirror(ctx, value);
        }
    } else if !attr.is_null() {
        if *dir as libc::c_int == 0 as libc::c_int {
            if ((*(*ctx).metalink).name).is_null()
                && wget_strcasecmp_ascii(
                    attr,
                    b"name\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                (*(*ctx).metalink).name = wget_strdup(value);
            }
        } else if wget_strcasecmp_ascii(
            dir,
            b"/pieces\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if wget_strcasecmp_ascii(attr, b"type\0" as *const u8 as *const libc::c_char)
                == 0
            {
                sscanf(
                    value,
                    b"%15s\0" as *const u8 as *const libc::c_char,
                    ((*ctx).hash_type).as_mut_ptr(),
                );
            } else if wget_strcasecmp_ascii(
                attr,
                b"length\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*ctx).length = atoll(value);
            }
        } else if wget_strcasecmp_ascii(
            dir,
            b"/hash\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if wget_strcasecmp_ascii(attr, b"type\0" as *const u8 as *const libc::c_char)
                == 0
            {
                sscanf(
                    value,
                    b"%15s\0" as *const u8 as *const libc::c_char,
                    ((*ctx).hash_type).as_mut_ptr(),
                );
            }
        } else if wget_strcasecmp_ascii(
            dir,
            b"/url\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if wget_strcasecmp_ascii(
                attr,
                b"location\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                sscanf(
                    value,
                    b" %2[a-zA-Z]\0" as *const u8 as *const libc::c_char,
                    ((*ctx).location).as_mut_ptr(),
                );
            } else if wget_strcasecmp_ascii(
                attr,
                b"priority\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    attr,
                    b"preference\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                sscanf(
                    value,
                    b" %6d\0" as *const u8 as *const libc::c_char,
                    &mut (*ctx).priority as *mut libc::c_int,
                );
                if (*ctx).priority < 1 as libc::c_int
                    || (*ctx).priority > 999999 as libc::c_int
                {
                    (*ctx).priority = 999999 as libc::c_int;
                }
            }
        }
    } else if wget_strcasecmp_ascii(
        dir,
        b"/pieces/hash\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        add_piece(ctx, value);
    } else if wget_strcasecmp_ascii(dir, b"/hash\0" as *const u8 as *const libc::c_char)
        == 0
    {
        add_file_hash(ctx, value);
    } else if wget_strcasecmp_ascii(dir, b"/size\0" as *const u8 as *const libc::c_char)
        == 0
    {
        (*(*ctx).metalink).size = atoll(value) as off_t;
    } else if wget_strcasecmp_ascii(dir, b"/url\0" as *const u8 as *const libc::c_char)
        == 0
    {
        add_mirror(ctx, value);
    }
    if value != valuebuf.as_mut_ptr() as *const libc::c_char {
        if !value.is_null() {
            wget_free.expect("non-null function pointer")(value as *mut libc::c_void);
            value = 0 as *const libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_metalink_parse(
    mut xml: *const libc::c_char,
) -> *mut wget_metalink {
    if xml.is_null() {
        return 0 as *mut wget_metalink;
    }
    let mut metalink: *mut wget_metalink = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_metalink>() as libc::c_ulong,
    ) as *mut wget_metalink;
    let mut ctx: metalink_context = {
        let mut init = metalink_context {
            metalink: metalink,
            priority: 999999 as libc::c_int,
            hash: [0; 128],
            hash_type: [0; 16],
            location: *::core::mem::transmute::<
                &[u8; 8],
                &mut [libc::c_char; 8],
            >(b"-\0\0\0\0\0\0\0"),
            length: 0,
        };
        init
    };
    if wget_xml_parse_buffer(
        xml,
        Some(
            metalink_parse
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        &mut ctx as *mut metalink_context as *mut libc::c_void,
        0 as libc::c_int,
    ) != WGET_E_SUCCESS as libc::c_int
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Error in parsing XML\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        wget_metalink_free(&mut metalink);
    }
    return metalink;
}
#[no_mangle]
pub unsafe extern "C" fn wget_metalink_free(mut metalink: *mut *mut wget_metalink) {
    if !metalink.is_null() && !(*metalink).is_null() {
        if !((**metalink).name).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((**metalink).name as *mut libc::c_void);
            (**metalink).name = 0 as *const libc::c_char;
        }
        wget_vector_free(&mut (**metalink).mirrors);
        wget_vector_free(&mut (**metalink).hashes);
        wget_vector_free(&mut (**metalink).pieces);
        if !(*metalink).is_null() {
            wget_free
                .expect("non-null function pointer")(*metalink as *mut libc::c_void);
            *metalink = 0 as *mut wget_metalink;
        }
    }
}
unsafe extern "C" fn compare_mirror(
    mut m1: *mut wget_metalink_mirror,
    mut m2: *mut wget_metalink_mirror,
) -> libc::c_int {
    return (*m1).priority - (*m2).priority;
}
#[no_mangle]
pub unsafe extern "C" fn wget_metalink_sort_mirrors(mut metalink: *mut wget_metalink) {
    if !metalink.is_null() {
        wget_vector_setcmpfunc(
            (*metalink).mirrors,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut wget_metalink_mirror,
                        *mut wget_metalink_mirror,
                    ) -> libc::c_int,
                >,
                Option::<wget_vector_compare_fn>,
            >(
                Some(
                    compare_mirror
                        as unsafe extern "C" fn(
                            *mut wget_metalink_mirror,
                            *mut wget_metalink_mirror,
                        ) -> libc::c_int,
                ),
            ),
        );
        wget_vector_sort((*metalink).mirrors);
    }
}
