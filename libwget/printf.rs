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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
    static mut wget_free: Option::<wget_free_function>;
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type va_list = __builtin_va_list;
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
unsafe extern "C" fn wget_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_realloc_fn.expect("non-null function pointer")(ptr, size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vasprintf(
    mut strp: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> size_t {
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    wget_buffer_init(&mut buf, 0 as *mut libc::c_char, 128 as libc::c_int as size_t);
    let mut len: size_t = wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    if buf.error() as libc::c_int as libc::c_long != 0 {
        if !(buf.data).is_null() {
            wget_free.expect("non-null function pointer")(buf.data as *mut libc::c_void);
            buf.data = 0 as *mut libc::c_char;
        }
        return -(1 as libc::c_int) as size_t;
    }
    if !strp.is_null() {
        *strp = wget_realloc(
            buf.data as *mut libc::c_void,
            len.wrapping_add(1 as libc::c_int as size_t),
        ) as *mut libc::c_char;
    } else if !(buf.data).is_null() {
        wget_free.expect("non-null function pointer")(buf.data as *mut libc::c_void);
        buf.data = 0 as *mut libc::c_char;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn wget_asprintf(
    mut strp: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut len: size_t = wget_vasprintf(strp, fmt, args_0.as_va_list());
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vaprintf(
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    wget_vasprintf(&mut s, fmt, args.as_va_list());
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_aprintf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    args_0 = args.clone();
    wget_vasprintf(&mut s, fmt, args_0.as_va_list());
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vfprintf(
    mut fp: *mut FILE,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> size_t {
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    let mut rc: size_t = 0;
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    let mut len: size_t = wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    if buf.error() as libc::c_int as libc::c_long != 0 {
        wget_buffer_deinit(&mut buf);
        return -(1 as libc::c_int) as size_t;
    }
    if len > 0 as libc::c_int as size_t {
        rc = fwrite(
            buf.data as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            len,
            fp,
        );
    } else {
        rc = 0 as libc::c_int as size_t;
    }
    wget_buffer_deinit(&mut buf);
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_fprintf(
    mut fp: *mut FILE,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut rc: size_t = wget_vfprintf(fp, fmt, args_0.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_printf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut rc: size_t = wget_vfprintf(stdout, fmt, args_0.as_va_list());
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vsnprintf(
    mut str: *mut libc::c_char,
    mut size: size_t,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> size_t {
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    wget_buffer_init(&mut buf, str, size);
    let mut len: size_t = wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    if buf.error() as libc::c_int as libc::c_long != 0 {
        wget_buffer_deinit(&mut buf);
        return -(1 as libc::c_int) as size_t;
    }
    if !str.is_null() {
        if buf.data == str {
            buf.data = 0 as *mut libc::c_char;
        } else if len < size {
            memcpy(
                str as *mut libc::c_void,
                buf.data as *const libc::c_void,
                len.wrapping_add(1 as libc::c_int as size_t),
            );
        } else {
            memcpy(
                str as *mut libc::c_void,
                buf.data as *const libc::c_void,
                size.wrapping_sub(1 as libc::c_int as size_t),
            );
            *str
                .offset(
                    size.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
    }
    wget_buffer_deinit(&mut buf);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn wget_snprintf(
    mut str: *mut libc::c_char,
    mut size: size_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut len: size_t = wget_vsnprintf(str, size, fmt, args_0.as_va_list());
    return len;
}
