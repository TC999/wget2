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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
    static mut wget_free: Option::<wget_free_function>;
}
pub type size_t = libc::c_ulong;
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
pub type wget_realloc_function = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
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
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[inline]
unsafe extern "C" fn wget_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_realloc_fn.expect("non-null function pointer")(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_init(
    mut buf: *mut wget_buffer,
    mut data: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    if !data.is_null() && (size != 0) as libc::c_int as libc::c_long != 0 {
        (*buf).size = size.wrapping_sub(1 as libc::c_int as size_t);
        (*buf).data = data;
        *(*buf).data = 0 as libc::c_int as libc::c_char;
        (*buf).set_release_data(0 as libc::c_int != 0);
    } else {
        if size == 0 {
            size = 127 as libc::c_int as size_t;
        }
        (*buf).size = size;
        (*buf)
            .data = wget_malloc(size.wrapping_add(1 as libc::c_int as size_t))
            as *mut libc::c_char;
        if ((*buf).data).is_null() {
            (*buf).set_error(1 as libc::c_int != 0);
            return WGET_E_MEMORY as libc::c_int;
        }
        *(*buf).data = 0 as libc::c_int as libc::c_char;
        (*buf).set_release_data(1 as libc::c_int != 0);
    }
    (*buf).set_error(0 as libc::c_int != 0);
    (*buf).set_release_buf(0 as libc::c_int != 0);
    (*buf).length = 0 as libc::c_int as size_t;
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_alloc(mut size: size_t) -> *mut wget_buffer {
    let mut buf: *mut wget_buffer = 0 as *mut wget_buffer;
    buf = wget_malloc(::core::mem::size_of::<wget_buffer>() as libc::c_ulong)
        as *mut wget_buffer;
    if buf.is_null() {
        return 0 as *mut wget_buffer;
    }
    if wget_buffer_init(buf, 0 as *mut libc::c_char, size) < 0 as libc::c_int {
        if !buf.is_null() {
            wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
            buf = 0 as *mut wget_buffer;
        }
        return 0 as *mut wget_buffer;
    }
    (*buf).set_release_buf(1 as libc::c_int != 0);
    return buf;
}
unsafe extern "C" fn buffer_realloc(
    mut buf: *mut wget_buffer,
    mut size: size_t,
) -> libc::c_int {
    let mut old_data: *mut libc::c_char = (*buf).data;
    if (*buf).release_data() {
        (*buf)
            .data = wget_realloc(
            (*buf).data as *mut libc::c_void,
            size.wrapping_add(1 as libc::c_int as size_t),
        ) as *mut libc::c_char;
    } else {
        (*buf)
            .data = wget_malloc(size.wrapping_add(1 as libc::c_int as size_t))
            as *mut libc::c_char;
    }
    if ((*buf).data).is_null() {
        (*buf).data = old_data;
        (*buf).set_error(1 as libc::c_int != 0);
        return WGET_E_MEMORY as libc::c_int;
    }
    if !(*buf).release_data() {
        if !old_data.is_null() as libc::c_int as libc::c_long != 0 && (*buf).length != 0
        {
            memcpy(
                (*buf).data as *mut libc::c_void,
                old_data as *const libc::c_void,
                ((*buf).length).wrapping_add(1 as libc::c_int as size_t),
            );
        } else {
            *(*buf).data = 0 as libc::c_int as libc::c_char;
        }
        (*buf).set_release_data(1 as libc::c_int != 0);
    }
    (*buf).size = size;
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_ensure_capacity(
    mut buf: *mut wget_buffer,
    mut size: size_t,
) -> libc::c_int {
    if !buf.is_null() as libc::c_int as libc::c_long != 0 {
        if (*buf).size < size {
            return buffer_realloc(buf, size);
        }
    }
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_deinit(mut buf: *mut wget_buffer) {
    if (*buf).release_data() {
        if !((*buf).data).is_null() {
            wget_free
                .expect("non-null function pointer")((*buf).data as *mut libc::c_void);
            (*buf).data = 0 as *mut libc::c_char;
        }
        (*buf).set_release_data(0 as libc::c_int != 0);
    }
    if (*buf).release_buf() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_free(mut buf: *mut *mut wget_buffer) {
    if (!buf.is_null() && !(*buf).is_null()) as libc::c_int as libc::c_long != 0 {
        wget_buffer_deinit(*buf);
        *buf = 0 as *mut wget_buffer;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_free_data(mut buf: *mut wget_buffer) {
    if !buf.is_null() as libc::c_int as libc::c_long != 0 {
        if (*buf).release_data() {
            if !((*buf).data).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*buf).data as *mut libc::c_void);
                (*buf).data = 0 as *mut libc::c_char;
            }
            (*buf).set_release_data(0 as libc::c_int != 0);
            (*buf).size = 0 as libc::c_int as size_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_reset(mut buf: *mut wget_buffer) {
    if !buf.is_null() as libc::c_int as libc::c_long != 0 {
        (*buf).length = 0 as libc::c_int as size_t;
        *(*buf).data = 0 as libc::c_int as libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_memcpy(
    mut buf: *mut wget_buffer,
    mut data: *const libc::c_void,
    mut length: size_t,
) -> size_t {
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as size_t;
    }
    (*buf).length = 0 as libc::c_int as size_t;
    return wget_buffer_memcat(buf, data, length);
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_memcat(
    mut buf: *mut wget_buffer,
    mut data: *const libc::c_void,
    mut length: size_t,
) -> size_t {
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as size_t;
    }
    if (length != 0) as libc::c_int as libc::c_long != 0 {
        if (*buf).size < ((*buf).length).wrapping_add(length) {
            if buffer_realloc(
                buf,
                ((*buf).size * 2 as libc::c_int as size_t).wrapping_add(length),
            ) != WGET_E_SUCCESS as libc::c_int
            {
                return (*buf).length;
            }
        }
        if !data.is_null() as libc::c_int as libc::c_long != 0 {
            memcpy(
                ((*buf).data).offset((*buf).length as isize) as *mut libc::c_void,
                data,
                length,
            );
        } else {
            memset(
                ((*buf).data).offset((*buf).length as isize) as *mut libc::c_void,
                0 as libc::c_int,
                length,
            );
        }
        (*buf).length = ((*buf).length).wrapping_add(length);
    }
    *((*buf).data).offset((*buf).length as isize) = 0 as libc::c_int as libc::c_char;
    return (*buf).length;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_strcpy(
    mut buf: *mut wget_buffer,
    mut s: *const libc::c_char,
) -> size_t {
    if !buf.is_null() as libc::c_int as libc::c_long != 0 {
        (*buf).length = 0 as libc::c_int as size_t;
    }
    return wget_buffer_memcat(
        buf,
        s as *const libc::c_void,
        if !s.is_null() as libc::c_int as libc::c_long != 0 {
            strlen(s)
        } else {
            0 as libc::c_int as libc::c_ulong
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_strcat(
    mut buf: *mut wget_buffer,
    mut s: *const libc::c_char,
) -> size_t {
    return wget_buffer_memcat(
        buf,
        s as *const libc::c_void,
        if !s.is_null() as libc::c_int as libc::c_long != 0 {
            strlen(s)
        } else {
            0 as libc::c_int as libc::c_ulong
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_bufcpy(
    mut buf: *mut wget_buffer,
    mut src: *mut wget_buffer,
) -> size_t {
    if !src.is_null() as libc::c_int as libc::c_long != 0 {
        return wget_buffer_memcpy(buf, (*src).data as *const libc::c_void, (*src).length)
    } else {
        return wget_buffer_memcpy(
            buf,
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_bufcat(
    mut buf: *mut wget_buffer,
    mut src: *mut wget_buffer,
) -> size_t {
    if !src.is_null() as libc::c_int as libc::c_long != 0 {
        return wget_buffer_memcat(buf, (*src).data as *const libc::c_void, (*src).length)
    } else {
        return wget_buffer_memcat(
            buf,
            0 as *const libc::c_void,
            0 as libc::c_int as size_t,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_memset(
    mut buf: *mut wget_buffer,
    mut c: libc::c_char,
    mut length: size_t,
) -> size_t {
    if !buf.is_null() as libc::c_int as libc::c_long != 0 {
        (*buf).length = 0 as libc::c_int as size_t;
    }
    return wget_buffer_memset_append(buf, c, length);
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_memset_append(
    mut buf: *mut wget_buffer,
    mut c: libc::c_char,
    mut length: size_t,
) -> size_t {
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int as size_t;
    }
    if (length != 0) as libc::c_int as libc::c_long != 0 {
        if ((*buf).size < ((*buf).length).wrapping_add(length)) as libc::c_int
            as libc::c_long != 0
        {
            if buffer_realloc(
                buf,
                ((*buf).size * 2 as libc::c_int as size_t).wrapping_add(length),
            ) != WGET_E_SUCCESS as libc::c_int
            {
                return (*buf).length;
            }
        }
        memset(
            ((*buf).data).offset((*buf).length as isize) as *mut libc::c_void,
            c as libc::c_int,
            length,
        );
        (*buf).length = ((*buf).length).wrapping_add(length);
    }
    *((*buf).data).offset((*buf).length as isize) = 0 as libc::c_int as libc::c_char;
    return (*buf).length;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_trim(
    mut buf: *mut wget_buffer,
) -> *mut libc::c_char {
    if buf.is_null() as libc::c_int as libc::c_long != 0 {
        return 0 as *mut libc::c_char;
    }
    while (*buf).length > 0 as libc::c_int as size_t
        && *(*__ctype_b_loc())
            .offset(
                *((*buf).data)
                    .offset(
                        ((*buf).length).wrapping_sub(1 as libc::c_int as size_t) as isize,
                    ) as libc::c_int as isize,
            ) as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*buf).length = ((*buf).length).wrapping_sub(1);
        (*buf).length;
    }
    *((*buf).data).offset((*buf).length as isize) = 0 as libc::c_int as libc::c_char;
    let mut spaces: size_t = 0 as libc::c_int as size_t;
    while (*buf).length > 0 as libc::c_int as size_t
        && *(*__ctype_b_loc())
            .offset(*((*buf).data).offset(spaces as isize) as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        spaces = spaces.wrapping_add(1);
        spaces;
    }
    if spaces != 0 {
        (*buf).length = ((*buf).length).wrapping_sub(spaces);
        memmove(
            (*buf).data as *mut libc::c_void,
            ((*buf).data).offset(spaces as isize) as *const libc::c_void,
            ((*buf).length).wrapping_add(1 as libc::c_int as size_t),
        );
    }
    return (*buf).data;
}
