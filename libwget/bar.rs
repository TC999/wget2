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
    pub type wget_thread_mutex_st;
    static mut stdout: *mut FILE;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn rpl_fflush(gl_stream: *mut FILE) -> libc::c_int;
    fn mbtowc(__pwc: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> libc::c_int;
    fn memcpy(
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
    fn wcwidth(__c: wchar_t) -> libc::c_int;
    fn wget_get_timemillis() -> libc::c_longlong;
    fn wget_human_readable(
        buf: *mut libc::c_char,
        bufsize: size_t,
        n: uint64_t,
    ) -> *mut libc::c_char;
    fn wget_get_screen_size(
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    ) -> libc::c_int;
    fn wget_strlcpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> size_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_fprintf(fp: *mut FILE, fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_vsnprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
    ) -> size_t;
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
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
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
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
pub type ssize_t = __ssize_t;
pub type wchar_t = libc::c_int;
pub type uint64_t = __uint64_t;
pub type sig_atomic_t = __sig_atomic_t;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_realloc_function = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_bar_st {
    pub slots: *mut bar_slot,
    pub progress_mem_holder: *mut libc::c_char,
    pub unknown_size: *mut libc::c_char,
    pub known_size: *mut libc::c_char,
    pub spaces: *mut libc::c_char,
    pub nslots: libc::c_int,
    pub max_width: libc::c_int,
    pub mutex: wget_thread_mutex,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct bar_slot {
    pub progress: *mut libc::c_char,
    pub filename: *mut libc::c_char,
    pub speed_buf: [libc::c_char; 8],
    pub human_size: [libc::c_char; 8],
    pub time_ring: [uint64_t; 24],
    pub bytes_ring: [uint64_t; 24],
    pub file_size: uint64_t,
    pub bytes_downloaded: uint64_t,
    pub ring_pos: libc::c_int,
    pub ring_used: libc::c_int,
    pub tick: libc::c_int,
    pub status: bar_slot_status,
    #[bitfield(name = "redraw", ty = "bool", bits = "0..=0")]
    pub redraw: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type bar_slot_status = libc::c_uint;
pub const COMPLETE: bar_slot_status = 2;
pub const DOWNLOADING: bar_slot_status = 1;
pub const EMPTY: bar_slot_status = 0;
pub type wget_bar = wget_bar_st;
pub const BAR_SPEED_SIZE: BAR_SIZES = 8;
pub const BAR_DOWNBYTES_SIZE: BAR_SIZES = 8;
pub const BAR_RATIO_SIZE: BAR_SIZES = 3;
pub const BAR_FILENAME_SIZE: BAR_SIZES = 20;
pub const SPEED_RING_SIZE: BAR_SETTINGS = 24;
pub const BAR_DECOR_COST: BAR_DECOR_SIZE = 49;
pub const DEFAULT_SCREEN_WIDTH: SCREEN_WIDTH = 70;
pub const MINIMUM_SCREEN_WIDTH: SCREEN_WIDTH = 49;
pub type BAR_SIZES = libc::c_uint;
pub const BAR_METER_COST: BAR_SIZES = 2;
pub type BAR_DECOR_SIZE = libc::c_uint;
pub type SCREEN_WIDTH = libc::c_uint;
pub type BAR_SETTINGS = libc::c_uint;
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
#[inline]
unsafe extern "C" fn wget_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_realloc_fn.expect("non-null function pointer")(ptr, size);
}
static mut report_speed_type: wget_report_speed = WGET_REPORT_SPEED_BYTES;
static mut report_speed_type_char: libc::c_char = 'B' as i32 as libc::c_char;
static mut speed_modifier: libc::c_ushort = 1000 as libc::c_int as libc::c_ushort;
unsafe extern "C" fn bar_update_speed_stats(mut slotp: *mut bar_slot) {
    let mut ring_pos: libc::c_int = (*slotp).ring_pos;
    let mut ring_used: libc::c_int = (*slotp).ring_used;
    let mut next_pos: libc::c_int = 0;
    if (*slotp).bytes_downloaded == (*slotp).bytes_ring[ring_pos as usize] {
        return;
    }
    let mut curtime: uint64_t = wget_get_timemillis() as uint64_t;
    ring_pos += 1;
    if ring_pos == SPEED_RING_SIZE as libc::c_int {
        ring_pos = 0 as libc::c_int;
    }
    (*slotp).bytes_ring[ring_pos as usize] = (*slotp).bytes_downloaded;
    (*slotp).time_ring[ring_pos as usize] = curtime;
    if ring_used < SPEED_RING_SIZE as libc::c_int {
        ring_used += 1;
        ring_used;
        next_pos = 1 as libc::c_int;
    } else {
        next_pos = if ring_pos + 1 as libc::c_int == SPEED_RING_SIZE as libc::c_int {
            0 as libc::c_int
        } else {
            ring_pos + 1 as libc::c_int
        };
    }
    if ring_used < 2 as libc::c_int {
        wget_strlcpy(
            ((*slotp).speed_buf).as_mut_ptr(),
            b" --.-K\0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        );
    } else {
        let mut bytes: size_t = ((*slotp).bytes_ring[ring_pos as usize])
            .wrapping_sub((*slotp).bytes_ring[next_pos as usize]);
        let mut time: size_t = ((*slotp).time_ring[ring_pos as usize])
            .wrapping_sub((*slotp).time_ring[next_pos as usize]);
        let mut speed: size_t = bytes * speed_modifier as size_t
            / (if time != 0 { time } else { 1 as libc::c_int as size_t });
        wget_human_readable(
            ((*slotp).speed_buf).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            speed,
        );
    }
    (*slotp).ring_pos = ring_pos;
    (*slotp).ring_used = ring_used;
}
static mut winsize_changed: sig_atomic_t = 0;
#[inline(always)]
unsafe extern "C" fn restore_cursor_position() {
    fputs(b"\x1B8\0" as *const u8 as *const libc::c_char, stdout);
}
#[inline(always)]
unsafe extern "C" fn bar_print_slot(mut bar: *const wget_bar, mut slot: libc::c_int) {
    wget_fprintf(
        stdout,
        b"\x1B7\x1B[%dA\x1B[1G\0" as *const u8 as *const libc::c_char,
        (*bar).nslots - slot,
    );
}
#[inline(always)]
unsafe extern "C" fn bar_set_progress(mut bar: *const wget_bar, mut slot: libc::c_int) {
    let mut slotp: *mut bar_slot = &mut *((*bar).slots).offset(slot as isize)
        as *mut bar_slot;
    if (*slotp).file_size > 0 as libc::c_int as uint64_t {
        let mut bytes: size_t = (*slotp).bytes_downloaded;
        let mut cols: libc::c_int = (bytes as libc::c_double
            / (*slotp).file_size as libc::c_double * (*bar).max_width as libc::c_double)
            as libc::c_int;
        if cols > (*bar).max_width {
            cols = (*bar).max_width;
        } else if cols <= 0 as libc::c_int {
            cols = 1 as libc::c_int;
        }
        memcpy(
            (*slotp).progress as *mut libc::c_void,
            (*bar).known_size as *const libc::c_void,
            (cols - 1 as libc::c_int) as libc::c_ulong,
        );
        *((*slotp).progress)
            .offset((cols - 1 as libc::c_int) as isize) = '>' as i32 as libc::c_char;
        if cols < (*bar).max_width {
            memset(
                ((*slotp).progress).offset(cols as isize) as *mut libc::c_void,
                ' ' as i32,
                ((*bar).max_width - cols) as libc::c_ulong,
            );
        }
    } else if (*bar).max_width > 3 as libc::c_int {
        let mut ind: libc::c_int = (*slotp).tick
            % ((*bar).max_width * 2 as libc::c_int - 6 as libc::c_int);
        let mut pre_space: libc::c_int = 0;
        if ind <= (*bar).max_width - 3 as libc::c_int {
            pre_space = ind;
        } else {
            pre_space = (*bar).max_width - (ind - (*bar).max_width + 5 as libc::c_int);
        }
        memset(
            (*slotp).progress as *mut libc::c_void,
            ' ' as i32,
            (*bar).max_width as libc::c_ulong,
        );
        memcpy(
            ((*slotp).progress).offset(pre_space as isize) as *mut libc::c_void,
            b"<=>\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        );
    } else {
        memset(
            (*slotp).progress as *mut libc::c_void,
            ' ' as i32,
            (*bar).max_width as libc::c_ulong,
        );
    }
    *((*slotp).progress)
        .offset((*bar).max_width as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn bar_inspect_multibyte(
    mut s: *mut libc::c_char,
    mut available_space: size_t,
    mut inspectedp: *mut size_t,
    mut padp: *mut size_t,
) {
    let mut displayed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut inspected: libc::c_int = 0 as libc::c_int;
    let mut wide: wchar_t = 0;
    let mut mblen: libc::c_int = 0;
    let mut remaining: size_t = 0;
    if s.is_null() {
        *inspectedp = inspected as size_t;
        *padp = available_space;
        return;
    }
    remaining = strlen(s);
    loop {
        mblen = mbtowc(&mut wide, &mut *s.offset(inspected as isize), remaining);
        if !(mblen > 0 as libc::c_int) {
            break;
        }
        let mut wid: libc::c_int = wcwidth(wide);
        if wid == 0 as libc::c_int && displayed as size_t == available_space
            || displayed.wrapping_add(wid as libc::c_uint) as size_t > available_space
        {
            break;
        }
        inspected += mblen;
        remaining = remaining.wrapping_sub(mblen as size_t);
        displayed = displayed.wrapping_add(wid as libc::c_uint);
    }
    *inspectedp = inspected as size_t;
    *padp = available_space.wrapping_sub(displayed as size_t);
}
unsafe extern "C" fn bar_update_slot(mut bar: *const wget_bar, mut slot: libc::c_int) {
    let mut slotp: *mut bar_slot = &mut *((*bar).slots).offset(slot as isize)
        as *mut bar_slot;
    if (*slotp).status as libc::c_uint == DOWNLOADING as libc::c_int as libc::c_uint
        || (*slotp).status as libc::c_uint == COMPLETE as libc::c_int as libc::c_uint
    {
        let mut max: uint64_t = 0;
        let mut cur: uint64_t = 0;
        let mut ratio: libc::c_int = 0;
        let mut consumed: size_t = 0;
        let mut pad: size_t = 0;
        if (*slotp).file_size == 0 as libc::c_int as uint64_t
            && (*slotp).status as libc::c_uint == COMPLETE as libc::c_int as libc::c_uint
        {
            (*slotp).file_size = (*slotp).bytes_downloaded;
        }
        max = (*slotp).file_size;
        cur = (*slotp).bytes_downloaded;
        ratio = if max != 0 {
            (100 as libc::c_int as uint64_t * cur / max) as libc::c_int
        } else {
            0 as libc::c_int
        };
        if ratio > 100 as libc::c_int {
            ratio = 100 as libc::c_int;
        }
        wget_human_readable(
            ((*slotp).human_size).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
            cur,
        );
        bar_update_speed_stats(slotp);
        bar_set_progress(bar, slot);
        bar_print_slot(bar, slot);
        bar_inspect_multibyte(
            (*slotp).filename,
            BAR_FILENAME_SIZE as libc::c_int as size_t,
            &mut consumed,
            &mut pad,
        );
        wget_fprintf(
            stdout,
            b"%-*.*s %*d%% [%s] %*s %*s%c/s\0" as *const u8 as *const libc::c_char,
            consumed.wrapping_add(pad) as libc::c_int,
            consumed.wrapping_add(pad) as libc::c_int,
            (*slotp).filename,
            BAR_RATIO_SIZE as libc::c_int,
            ratio,
            (*slotp).progress,
            BAR_DOWNBYTES_SIZE as libc::c_int,
            ((*slotp).human_size).as_mut_ptr(),
            BAR_SPEED_SIZE as libc::c_int,
            ((*slotp).speed_buf).as_mut_ptr(),
            report_speed_type_char as libc::c_int,
        );
        restore_cursor_position();
        rpl_fflush(stdout);
        (*slotp).tick += 1;
        (*slotp).tick;
    }
}
unsafe extern "C" fn bar_get_width() -> libc::c_int {
    let mut width: libc::c_int = DEFAULT_SCREEN_WIDTH as libc::c_int;
    if wget_get_screen_size(&mut width, 0 as *mut libc::c_int) == 0 as libc::c_int {
        if width < MINIMUM_SCREEN_WIDTH as libc::c_int {
            width = MINIMUM_SCREEN_WIDTH as libc::c_int;
        } else {
            width -= 1;
            width;
        }
    }
    return width - BAR_DECOR_COST as libc::c_int;
}
unsafe extern "C" fn bar_update_winsize(
    mut bar: *mut wget_bar,
    mut slots_changed: bool,
) {
    if winsize_changed != 0 || slots_changed as libc::c_int != 0 {
        let mut progress_mem_holder: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut max_width: libc::c_int = bar_get_width();
        progress_mem_holder = wget_calloc(
            (*bar).nslots as size_t,
            (max_width + 1 as libc::c_int) as size_t,
        ) as *mut libc::c_char;
        if progress_mem_holder.is_null() {
            return;
        }
        if (*bar).max_width < max_width {
            let mut known_size: *mut libc::c_char = wget_malloc(max_width as size_t)
                as *mut libc::c_char;
            let mut unknown_size: *mut libc::c_char = wget_malloc(max_width as size_t)
                as *mut libc::c_char;
            let mut spaces: *mut libc::c_char = wget_malloc(max_width as size_t)
                as *mut libc::c_char;
            if known_size.is_null() || unknown_size.is_null() || spaces.is_null() {
                if !spaces.is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(spaces as *mut libc::c_void);
                    spaces = 0 as *mut libc::c_char;
                }
                if !unknown_size.is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(unknown_size as *mut libc::c_void);
                    unknown_size = 0 as *mut libc::c_char;
                }
                if !known_size.is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(known_size as *mut libc::c_void);
                    known_size = 0 as *mut libc::c_char;
                }
                if !progress_mem_holder.is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(progress_mem_holder as *mut libc::c_void);
                    progress_mem_holder = 0 as *mut libc::c_char;
                }
                return;
            }
            if !((*bar).known_size).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*bar).known_size as *mut libc::c_void);
                (*bar).known_size = 0 as *mut libc::c_char;
            }
            (*bar).known_size = known_size;
            memset(
                (*bar).known_size as *mut libc::c_void,
                '=' as i32,
                max_width as libc::c_ulong,
            );
            if !((*bar).unknown_size).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*bar).unknown_size as *mut libc::c_void);
                (*bar).unknown_size = 0 as *mut libc::c_char;
            }
            (*bar).unknown_size = unknown_size;
            memset(
                (*bar).unknown_size as *mut libc::c_void,
                '*' as i32,
                max_width as libc::c_ulong,
            );
            if !((*bar).spaces).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )((*bar).spaces as *mut libc::c_void);
                (*bar).spaces = 0 as *mut libc::c_char;
            }
            (*bar).spaces = spaces;
            memset(
                (*bar).spaces as *mut libc::c_void,
                ' ' as i32,
                max_width as libc::c_ulong,
            );
        }
        if !((*bar).progress_mem_holder).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*bar).progress_mem_holder as *mut libc::c_void);
            (*bar).progress_mem_holder = 0 as *mut libc::c_char;
        }
        (*bar).progress_mem_holder = progress_mem_holder;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*bar).nslots {
            let ref mut fresh0 = (*((*bar).slots).offset(i as isize)).progress;
            *fresh0 = ((*bar).progress_mem_holder).offset((i * max_width) as isize);
            i += 1;
            i;
        }
        (*bar).max_width = max_width;
    }
    ::core::ptr::write_volatile(
        &mut winsize_changed as *mut sig_atomic_t,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn bar_update(mut bar: *mut wget_bar) {
    let mut redraw: bool = winsize_changed != 0 as libc::c_int;
    bar_update_winsize(bar, 0 as libc::c_int != 0);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*bar).nslots {
        if (*((*bar).slots).offset(i as isize)).redraw() as libc::c_int != 0
            || redraw as libc::c_int != 0
        {
            bar_update_slot(bar, i);
            let ref mut fresh1 = *((*bar).slots).offset(i as isize);
            (*fresh1).set_redraw(0 as libc::c_int != 0);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_init(
    mut bar: *mut wget_bar,
    mut nslots: libc::c_int,
) -> *mut wget_bar {
    let mut max_width: libc::c_int = bar_get_width();
    if nslots < 1 as libc::c_int || max_width < 1 as libc::c_int {
        return 0 as *mut wget_bar;
    }
    if bar.is_null() {
        bar = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_bar>() as libc::c_ulong,
        ) as *mut wget_bar;
        if bar.is_null() {
            return 0 as *mut wget_bar;
        }
    } else {
        memset(
            bar as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<wget_bar>() as libc::c_ulong,
        );
    }
    wget_thread_mutex_init(&mut (*bar).mutex);
    wget_bar_set_slots(bar, nslots);
    return bar;
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_set_slots(
    mut bar: *mut wget_bar,
    mut nslots: libc::c_int,
) {
    wget_thread_mutex_lock((*bar).mutex);
    let mut more_slots: libc::c_int = nslots - (*bar).nslots;
    if more_slots > 0 as libc::c_int {
        let mut slots: *mut bar_slot = wget_realloc(
            (*bar).slots as *mut libc::c_void,
            (nslots as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<bar_slot>() as libc::c_ulong),
        ) as *mut bar_slot;
        if slots.is_null() {
            wget_thread_mutex_unlock((*bar).mutex);
            return;
        }
        (*bar).slots = slots;
        memset(
            ((*bar).slots).offset((*bar).nslots as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (more_slots as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<bar_slot>() as libc::c_ulong),
        );
        (*bar).nslots = nslots;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < more_slots {
            fputs(b"\n\0" as *const u8 as *const libc::c_char, stdout);
            i += 1;
            i;
        }
        bar_update_winsize(bar, 1 as libc::c_int != 0);
        bar_update(bar);
    }
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_slot_begin(
    mut bar: *mut wget_bar,
    mut slot: libc::c_int,
    mut filename: *const libc::c_char,
    mut new_file: libc::c_int,
    mut file_size: ssize_t,
) {
    wget_thread_mutex_lock((*bar).mutex);
    let mut slotp: *mut bar_slot = &mut *((*bar).slots).offset(slot as isize)
        as *mut bar_slot;
    if !((*slotp).filename).is_null() {
        wget_free
            .expect("non-null function pointer")((*slotp).filename as *mut libc::c_void);
        (*slotp).filename = 0 as *mut libc::c_char;
    }
    (*slotp).filename = wget_strdup(filename);
    memset(
        &mut (*slotp).time_ring as *mut [uint64_t; 24] as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint64_t; 24]>() as libc::c_ulong,
    );
    memset(
        &mut (*slotp).bytes_ring as *mut [uint64_t; 24] as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[uint64_t; 24]>() as libc::c_ulong,
    );
    (*slotp).file_size = file_size as uint64_t;
    (*slotp).bytes_downloaded = 0 as libc::c_int as uint64_t;
    (*slotp).ring_pos = 0 as libc::c_int;
    (*slotp).ring_used = 0 as libc::c_int;
    (*slotp).tick = 0 as libc::c_int;
    (*slotp).status = DOWNLOADING;
    (*slotp).set_redraw(1 as libc::c_int != 0);
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_slot_downloaded(
    mut bar: *mut wget_bar,
    mut slot: libc::c_int,
    mut nbytes: size_t,
) {
    wget_thread_mutex_lock((*bar).mutex);
    let ref mut fresh2 = (*((*bar).slots).offset(slot as isize)).bytes_downloaded;
    *fresh2 = (*fresh2 as libc::c_ulong).wrapping_add(nbytes) as uint64_t as uint64_t;
    let ref mut fresh3 = *((*bar).slots).offset(slot as isize);
    (*fresh3).set_redraw(1 as libc::c_int != 0);
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_slot_deregister(
    mut bar: *mut wget_bar,
    mut slot: libc::c_int,
) {
    wget_thread_mutex_lock((*bar).mutex);
    if slot >= 0 as libc::c_int && slot < (*bar).nslots {
        let mut slotp: *mut bar_slot = &mut *((*bar).slots).offset(slot as isize)
            as *mut bar_slot;
        (*slotp).status = COMPLETE;
        bar_update_slot(bar, slot);
    }
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_update(mut bar: *mut wget_bar) {
    wget_thread_mutex_lock((*bar).mutex);
    bar_update(bar);
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_deinit(mut bar: *mut wget_bar) {
    if !bar.is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*bar).nslots {
            if !((*((*bar).slots).offset(i as isize)).filename).is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(
                    (*((*bar).slots).offset(i as isize)).filename as *mut libc::c_void,
                );
                let ref mut fresh4 = (*((*bar).slots).offset(i as isize)).filename;
                *fresh4 = 0 as *mut libc::c_char;
            }
            i += 1;
            i;
        }
        if !((*bar).progress_mem_holder).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*bar).progress_mem_holder as *mut libc::c_void);
            (*bar).progress_mem_holder = 0 as *mut libc::c_char;
        }
        if !((*bar).spaces).is_null() {
            wget_free
                .expect("non-null function pointer")((*bar).spaces as *mut libc::c_void);
            (*bar).spaces = 0 as *mut libc::c_char;
        }
        if !((*bar).known_size).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*bar).known_size as *mut libc::c_void);
            (*bar).known_size = 0 as *mut libc::c_char;
        }
        if !((*bar).unknown_size).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*bar).unknown_size as *mut libc::c_void);
            (*bar).unknown_size = 0 as *mut libc::c_char;
        }
        if !((*bar).slots).is_null() {
            wget_free
                .expect("non-null function pointer")((*bar).slots as *mut libc::c_void);
            (*bar).slots = 0 as *mut bar_slot;
        }
        wget_thread_mutex_destroy(&mut (*bar).mutex);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_free(mut bar: *mut *mut wget_bar) {
    if !bar.is_null() {
        wget_bar_deinit(*bar);
        if !(*bar).is_null() {
            wget_free.expect("non-null function pointer")(*bar as *mut libc::c_void);
            *bar = 0 as *mut wget_bar;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_print(
    mut bar: *mut wget_bar,
    mut slot: libc::c_int,
    mut display: *const libc::c_char,
) {
    wget_thread_mutex_lock((*bar).mutex);
    bar_print_slot(bar, slot);
    wget_fprintf(
        stdout,
        b"\x1B[27G[%-*.*s]\0" as *const u8 as *const libc::c_char,
        (*bar).max_width,
        (*bar).max_width,
        display,
    );
    restore_cursor_position();
    rpl_fflush(stdout);
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_vprintf(
    mut bar: *mut wget_bar,
    mut slot: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) {
    let mut tmp: [libc::c_char; 256] = [0; 256];
    let mut text: *mut libc::c_char = tmp.as_mut_ptr();
    let mut textlen: size_t = ((*bar).max_width + 1 as libc::c_int) as size_t;
    if textlen > ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong {
        text = wget_malloc(textlen) as *mut libc::c_char;
        if text.is_null() {
            text = tmp.as_mut_ptr();
            textlen = ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong;
        }
    }
    wget_vsnprintf(text, textlen, fmt, args.as_va_list());
    wget_bar_print(bar, slot, text);
    if text != tmp.as_mut_ptr() {
        if !text.is_null() {
            wget_free.expect("non-null function pointer")(text as *mut libc::c_void);
            text = 0 as *mut libc::c_char;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_printf(
    mut bar: *mut wget_bar,
    mut slot: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    wget_bar_vprintf(bar, slot, fmt, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_screen_resized() {
    ::core::ptr::write_volatile(
        &mut winsize_changed as *mut sig_atomic_t,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_write_line(
    mut bar: *mut wget_bar,
    mut buf: *const libc::c_char,
    mut len: size_t,
) {
    wget_bar_write_line_ext(
        bar,
        buf,
        len,
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_write_line_ext(
    mut bar: *mut wget_bar,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut pre: *const libc::c_char,
    mut post: *const libc::c_char,
) {
    wget_thread_mutex_lock((*bar).mutex);
    wget_fprintf(
        stdout,
        b"\x1B7\x1B[1S\x1B[%dA\x1B[1G\x1B[0J%s\0" as *const u8 as *const libc::c_char,
        (*bar).nslots + 1 as libc::c_int,
        pre,
    );
    fwrite(buf as *const libc::c_void, 1 as libc::c_int as libc::c_ulong, len, stdout);
    fputs(post, stdout);
    restore_cursor_position();
    bar_update(bar);
    wget_thread_mutex_unlock((*bar).mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_bar_set_speed_type(mut type_0: wget_report_speed) {
    report_speed_type = type_0;
    if type_0 as libc::c_uint == WGET_REPORT_SPEED_BITS as libc::c_int as libc::c_uint {
        report_speed_type_char = 'b' as i32 as libc::c_char;
        speed_modifier = 8 as libc::c_int as libc::c_ushort;
    }
}
