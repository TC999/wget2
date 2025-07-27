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
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
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
pub type wget_console_color = libc::c_uint;
pub const WGET_CONSOLE_COLOR_MAGENTA: wget_console_color = 5;
pub const WGET_CONSOLE_COLOR_RED: wget_console_color = 4;
pub const WGET_CONSOLE_COLOR_GREEN: wget_console_color = 3;
pub const WGET_CONSOLE_COLOR_BLUE: wget_console_color = 2;
pub const WGET_CONSOLE_COLOR_WHITE: wget_console_color = 1;
pub const WGET_CONSOLE_COLOR_RESET: wget_console_color = 0;
unsafe extern "C" fn reset_color() {
    if isatty(fileno(stdout)) != 0 {
        fputs(b"\x1B[m\0" as *const u8 as *const libc::c_char, stdout);
    }
    rpl_fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn wget_console_set_fg_color(mut colorid: wget_console_color) {}
#[no_mangle]
pub unsafe extern "C" fn wget_console_reset_fg_color() {
    wget_console_set_fg_color(WGET_CONSOLE_COLOR_RESET);
}
#[no_mangle]
pub unsafe extern "C" fn wget_console_init() -> libc::c_int {
    atexit(Some(reset_color as unsafe extern "C" fn() -> ()));
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_console_deinit() -> libc::c_int {
    reset_color();
    return 0 as libc::c_int;
}
