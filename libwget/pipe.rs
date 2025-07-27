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
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn __errno_location() -> *mut libc::c_int;
    fn exit(_: libc::c_int) -> !;
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
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
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
pub type __pid_t = libc::c_int;
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
pub type pid_t = __pid_t;
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
#[no_mangle]
pub unsafe extern "C" fn wget_vpopenf(
    mut type_0: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    if type_0.is_null() || fmt.is_null() {
        return 0 as *mut FILE;
    }
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    wget_buffer_vprintf(&mut buf, fmt, args.as_va_list());
    fp = popen(buf.data, type_0);
    wget_buffer_deinit(&mut buf);
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_popenf(
    mut type_0: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut FILE {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    fp = wget_vpopenf(type_0, fmt, args_0.as_va_list());
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_fd_popen3(
    mut fdin: *mut libc::c_int,
    mut fdout: *mut libc::c_int,
    mut fderr: *mut libc::c_int,
    mut argv: *const *const libc::c_char,
) -> pid_t {
    let mut pipefd_in: [libc::c_int; 2] = [0; 2];
    let mut pipefd_out: [libc::c_int; 2] = [0; 2];
    let mut pipefd_err: [libc::c_int; 2] = [0; 2];
    let mut pid: pid_t = 0;
    if argv.is_null() {
        return -(1 as libc::c_int);
    }
    if !fdin.is_null() && pipe(pipefd_in.as_mut_ptr()) == -(1 as libc::c_int) {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to create pipe for STDIN on %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            *argv.offset(0 as libc::c_int as isize),
        );
        return -(1 as libc::c_int);
    }
    if !fdout.is_null() && pipe(pipefd_out.as_mut_ptr()) == -(1 as libc::c_int) {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to create pipe for STDOUT on %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            *argv.offset(0 as libc::c_int as isize),
        );
        if !fdin.is_null() {
            close(pipefd_in[0 as libc::c_int as usize]);
            close(pipefd_in[1 as libc::c_int as usize]);
        }
        return -(1 as libc::c_int);
    }
    if !fderr.is_null() && fderr != fdout
        && pipe(pipefd_err.as_mut_ptr()) == -(1 as libc::c_int)
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to create pipe for STDERR on %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            *argv.offset(0 as libc::c_int as isize),
        );
        if !fdin.is_null() {
            close(pipefd_in[0 as libc::c_int as usize]);
            close(pipefd_in[1 as libc::c_int as usize]);
        }
        if !fdout.is_null() {
            close(pipefd_out[0 as libc::c_int as usize]);
            close(pipefd_out[1 as libc::c_int as usize]);
        }
        return -(1 as libc::c_int);
    }
    pid = fork();
    if pid == 0 as libc::c_int {
        if !fdin.is_null() {
            close(pipefd_in[1 as libc::c_int as usize]);
            if dup2(pipefd_in[0 as libc::c_int as usize], 0 as libc::c_int)
                == -(1 as libc::c_int)
            {
                wget_error_printf_exit(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to dup2(%d,%d) (%d)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    pipefd_in[0 as libc::c_int as usize],
                    0 as libc::c_int,
                    *__errno_location(),
                );
            }
            close(pipefd_in[0 as libc::c_int as usize]);
        }
        if !fdout.is_null() {
            close(pipefd_out[0 as libc::c_int as usize]);
            if dup2(pipefd_out[1 as libc::c_int as usize], 1 as libc::c_int)
                == -(1 as libc::c_int)
            {
                wget_error_printf_exit(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to dup2(%d,%d) (%d)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    pipefd_out[1 as libc::c_int as usize],
                    1 as libc::c_int,
                    *__errno_location(),
                );
            }
            close(pipefd_out[1 as libc::c_int as usize]);
        }
        if !fderr.is_null() {
            if fderr != fdout {
                close(pipefd_err[0 as libc::c_int as usize]);
                if dup2(pipefd_err[1 as libc::c_int as usize], 2 as libc::c_int)
                    == -(1 as libc::c_int)
                {
                    wget_error_printf_exit(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to dup2(%d,%d) (%d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        pipefd_err[1 as libc::c_int as usize],
                        2 as libc::c_int,
                        *__errno_location(),
                    );
                }
                close(pipefd_err[1 as libc::c_int as usize]);
            } else if dup2(1 as libc::c_int, 2 as libc::c_int) == -(1 as libc::c_int) {
                exit(1 as libc::c_int);
            }
        }
        execvp(
            *argv.offset(0 as libc::c_int as isize),
            argv as *const *mut libc::c_char,
        );
        exit(1 as libc::c_int);
    } else if pid < 0 as libc::c_int {
        if !fdin.is_null() {
            close(pipefd_in[0 as libc::c_int as usize]);
            close(pipefd_in[1 as libc::c_int as usize]);
        }
        if !fdout.is_null() {
            close(pipefd_out[0 as libc::c_int as usize]);
            close(pipefd_out[1 as libc::c_int as usize]);
        }
        if !fderr.is_null() && fderr != fdout {
            close(pipefd_err[0 as libc::c_int as usize]);
            close(pipefd_err[1 as libc::c_int as usize]);
        }
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to fork '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *argv.offset(0 as libc::c_int as isize),
        );
        return pid;
    }
    if !fdin.is_null() {
        close(pipefd_in[0 as libc::c_int as usize]);
        *fdin = pipefd_in[1 as libc::c_int as usize];
    }
    if !fdout.is_null() {
        close(pipefd_out[1 as libc::c_int as usize]);
        *fdout = pipefd_out[0 as libc::c_int as usize];
    }
    if !fderr.is_null() && fderr != fdout {
        close(pipefd_err[1 as libc::c_int as usize]);
        *fderr = pipefd_err[0 as libc::c_int as usize];
    }
    return pid;
}
#[no_mangle]
pub unsafe extern "C" fn wget_popen3(
    mut fpin: *mut *mut FILE,
    mut fpout: *mut *mut FILE,
    mut fperr: *mut *mut FILE,
    mut argv: *const *const libc::c_char,
) -> pid_t {
    let mut fdin: libc::c_int = -(1 as libc::c_int);
    let mut fdout: libc::c_int = -(1 as libc::c_int);
    let mut fderr: libc::c_int = -(1 as libc::c_int);
    let mut pid: pid_t = 0;
    if !fpin.is_null() {
        *fpin = 0 as *mut FILE;
    }
    if !fpout.is_null() {
        *fpout = 0 as *mut FILE;
    }
    if !fperr.is_null() {
        *fperr = 0 as *mut FILE;
    }
    pid = wget_fd_popen3(
        (if !fpin.is_null() { &mut fdin } else { 0 as *mut libc::c_int }),
        (if !fpout.is_null() { &mut fdout } else { 0 as *mut libc::c_int }),
        (if !fperr.is_null() {
            (if fperr != fpout { &mut fderr } else { &mut fdout })
        } else {
            0 as *mut libc::c_int
        }),
        argv,
    );
    if pid > 0 as libc::c_int {
        if !fpin.is_null() {
            *fpin = fdopen(fdin, b"w\0" as *const u8 as *const libc::c_char);
        }
        if !fpout.is_null() {
            *fpout = fdopen(fdout, b"r\0" as *const u8 as *const libc::c_char);
        }
        if !fperr.is_null() && fperr != fpout {
            *fperr = fdopen(fderr, b"r\0" as *const u8 as *const libc::c_char);
        }
    }
    return pid;
}
