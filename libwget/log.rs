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
    fn exit(_: libc::c_int) -> !;
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
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_logger_st {
    pub fp: *mut FILE,
    pub fname: *const libc::c_char,
    pub func: Option::<unsafe extern "C" fn(*const libc::c_char, size_t) -> ()>,
    pub vprintf: Option::<
        unsafe extern "C" fn(
            *const wget_logger,
            *const libc::c_char,
            ::core::ffi::VaList,
        ) -> (),
    >,
    pub write: Option::<
        unsafe extern "C" fn(*const wget_logger, *const libc::c_char, size_t) -> (),
    >,
}
pub type wget_logger = wget_logger_st;
static mut info_logger: wget_logger = wget_logger_st {
    fp: 0 as *const FILE as *mut FILE,
    fname: 0 as *const libc::c_char,
    func: None,
    vprintf: None,
    write: None,
};
static mut error_logger: wget_logger = wget_logger_st {
    fp: 0 as *const FILE as *mut FILE,
    fname: 0 as *const libc::c_char,
    func: None,
    vprintf: None,
    write: None,
};
static mut debug_logger: wget_logger = wget_logger_st {
    fp: 0 as *const FILE as *mut FILE,
    fname: 0 as *const libc::c_char,
    func: None,
    vprintf: None,
    write: None,
};
#[no_mangle]
pub unsafe extern "C" fn wget_info_vprintf(
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    if (info_logger.vprintf).is_some() {
        (info_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut info_logger, fmt, args.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_info_printf(mut fmt: *const libc::c_char, mut args: ...) {
    if (info_logger.vprintf).is_some() {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        (info_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut info_logger, fmt, args_0.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_error_vprintf(
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    if (error_logger.vprintf).is_some() {
        (error_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut error_logger, fmt, args.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_error_printf(mut fmt: *const libc::c_char, mut args: ...) {
    if (error_logger.vprintf).is_some() {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        (error_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut error_logger, fmt, args_0.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_error_printf_exit(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    if (error_logger.vprintf).is_some() {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        (error_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut error_logger, fmt, args_0.as_va_list());
    }
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_debug_vprintf(
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    if (debug_logger.vprintf).is_some() {
        (debug_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut debug_logger, fmt, args.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_debug_printf(mut fmt: *const libc::c_char, mut args: ...) {
    if (debug_logger.vprintf).is_some() {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        (debug_logger.vprintf)
            .expect(
                "non-null function pointer",
            )(&mut debug_logger, fmt, args_0.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_debug_write(
    mut buf: *const libc::c_char,
    mut len: size_t,
) {
    if (debug_logger.write).is_some() {
        (debug_logger.write)
            .expect("non-null function pointer")(&mut debug_logger, buf, len);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_get_logger(mut id: libc::c_int) -> *mut wget_logger {
    if id == 3 as libc::c_int {
        return &mut debug_logger
    } else if id == 2 as libc::c_int {
        return &mut error_logger
    } else if id == 1 as libc::c_int {
        return &mut info_logger
    } else {
        return 0 as *mut wget_logger
    };
}
