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
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn __errno_location() -> *mut libc::c_int;
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
}
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
pub type wget_logger_func = unsafe extern "C" fn(*const libc::c_char, size_t) -> ();
unsafe extern "C" fn logger_vprintf_func(
    mut logger: *const wget_logger,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    let mut sbuf: [libc::c_char; 4096] = [0; 4096];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut err: libc::c_int = *__errno_location();
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    ((*logger).func).expect("non-null function pointer")(buf.data, buf.length);
    wget_buffer_deinit(&mut buf);
    *__errno_location() = err;
}
unsafe extern "C" fn logger_write_func(
    mut logger: *const wget_logger,
    mut buf: *const libc::c_char,
    mut len: size_t,
) {
    ((*logger).func).expect("non-null function pointer")(buf, len);
}
unsafe extern "C" fn logger_vfprintf(
    mut fp: *mut FILE,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    let mut sbuf: [libc::c_char; 4096] = [0; 4096];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut err: libc::c_int = *__errno_location();
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
    );
    wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    fwrite(
        buf.data as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        buf.length,
        fp,
    );
    wget_buffer_deinit(&mut buf);
    *__errno_location() = err;
}
unsafe extern "C" fn logger_vprintf_file(
    mut logger: *const wget_logger,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    logger_vfprintf((*logger).fp, fmt, args.as_va_list());
}
unsafe extern "C" fn logger_write_file(
    mut logger: *const wget_logger,
    mut buf: *const libc::c_char,
    mut len: size_t,
) {
    fwrite(
        buf as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        len,
        (*logger).fp,
    );
}
unsafe extern "C" fn logger_vprintf_fname(
    mut logger: *const wget_logger,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    let mut fp: *mut FILE = rpl_fopen(
        (*logger).fname,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        logger_vfprintf(fp, fmt, args.as_va_list());
        rpl_fclose(fp);
    }
}
unsafe extern "C" fn logger_write_fname(
    mut logger: *const wget_logger,
    mut buf: *const libc::c_char,
    mut len: size_t,
) {
    let mut fp: *mut FILE = rpl_fopen(
        (*logger).fname,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        fwrite(buf as *const libc::c_void, 1 as libc::c_int as libc::c_ulong, len, fp);
        rpl_fclose(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_set_func(
    mut logger: *mut wget_logger,
    mut func: Option::<wget_logger_func>,
) {
    if !logger.is_null() {
        (*logger).func = func;
        (*logger)
            .vprintf = if func.is_some() {
            Some(
                logger_vprintf_func
                    as unsafe extern "C" fn(
                        *const wget_logger,
                        *const libc::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            )
        } else {
            None
        };
        (*logger)
            .write = if func.is_some() {
            Some(
                logger_write_func
                    as unsafe extern "C" fn(
                        *const wget_logger,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            )
        } else {
            None
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_get_func(
    mut logger: *mut wget_logger,
) -> Option::<wget_logger_func> {
    return if !logger.is_null() { (*logger).func } else { None };
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_set_stream(
    mut logger: *mut wget_logger,
    mut fp: *mut FILE,
) {
    if !logger.is_null() {
        (*logger).fp = fp;
        (*logger)
            .vprintf = if !fp.is_null() {
            Some(
                logger_vprintf_file
                    as unsafe extern "C" fn(
                        *const wget_logger,
                        *const libc::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            )
        } else {
            None
        };
        (*logger)
            .write = if !fp.is_null() {
            Some(
                logger_write_file
                    as unsafe extern "C" fn(
                        *const wget_logger,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            )
        } else {
            None
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_get_stream(
    mut logger: *mut wget_logger,
) -> *mut FILE {
    return if !logger.is_null() { (*logger).fp } else { 0 as *mut FILE };
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_set_file(
    mut logger: *mut wget_logger,
    mut fname: *const libc::c_char,
) {
    if !logger.is_null() {
        (*logger).fname = fname;
        (*logger)
            .vprintf = if !fname.is_null() {
            Some(
                logger_vprintf_fname
                    as unsafe extern "C" fn(
                        *const wget_logger,
                        *const libc::c_char,
                        ::core::ffi::VaList,
                    ) -> (),
            )
        } else {
            None
        };
        (*logger)
            .write = if !fname.is_null() {
            Some(
                logger_write_fname
                    as unsafe extern "C" fn(
                        *const wget_logger,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            )
        } else {
            None
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_get_file(
    mut logger: *mut wget_logger,
) -> *const libc::c_char {
    return if !logger.is_null() { (*logger).fname } else { 0 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn wget_logger_is_active(mut logger: *mut wget_logger) -> bool {
    return ((*logger).vprintf).is_some();
}
