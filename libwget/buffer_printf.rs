#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strnlen(__string: *const libc::c_char, __maxlen: size_t) -> size_t;
    fn wget_buffer_memcat(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_strcat(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_memset_append(
        buf: *mut wget_buffer,
        c: libc::c_char,
        length: size_t,
    ) -> size_t;
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
pub type __ssize_t = libc::c_long;
pub type va_list = __gnuc_va_list;
pub type ssize_t = __ssize_t;
pub type ptrdiff_t = libc::c_long;
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
unsafe extern "C" fn c_isdigit(mut c: libc::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return 1 as libc::c_int != 0,
        _ => return 0 as libc::c_int != 0,
    };
}
unsafe extern "C" fn copy_string(
    mut buf: *mut wget_buffer,
    mut flags: libc::c_uint,
    mut field_width: libc::c_int,
    mut precision: libc::c_int,
    mut arg: *const libc::c_char,
) {
    let mut length: size_t = 0;
    if arg.is_null() {
        wget_buffer_strcat(buf, b"(null)\0" as *const u8 as *const libc::c_char);
        return;
    }
    if precision >= 0 as libc::c_int {
        length = strnlen(arg, precision as size_t);
    } else {
        length = strlen(arg);
    }
    if field_width != 0 {
        if field_width as libc::c_uint as size_t > length {
            if flags & 2 as libc::c_uint != 0 {
                wget_buffer_memcat(buf, arg as *const libc::c_void, length);
                wget_buffer_memset_append(
                    buf,
                    ' ' as i32 as libc::c_char,
                    (field_width as size_t).wrapping_sub(length),
                );
            } else {
                wget_buffer_memset_append(
                    buf,
                    ' ' as i32 as libc::c_char,
                    (field_width as size_t).wrapping_sub(length),
                );
                wget_buffer_memcat(buf, arg as *const libc::c_void, length);
            }
        } else {
            wget_buffer_memcat(buf, arg as *const libc::c_void, length);
        }
    } else {
        wget_buffer_memcat(buf, arg as *const libc::c_void, length);
    };
}
unsafe extern "C" fn convert_dec_fast(mut buf: *mut wget_buffer, mut arg: libc::c_int) {
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut dst: *mut libc::c_char = str
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut minus: libc::c_int = 0;
    if arg < 0 as libc::c_int {
        minus = 1 as libc::c_int;
        arg = -arg;
    } else {
        minus = 0 as libc::c_int;
    }
    while arg >= 10 as libc::c_int {
        let fresh0 = dst;
        dst = dst.offset(-1);
        *fresh0 = (arg % 10 as libc::c_int + '0' as i32) as libc::c_char;
        arg /= 10 as libc::c_int;
    }
    let fresh1 = dst;
    dst = dst.offset(-1);
    *fresh1 = (arg % 10 as libc::c_int + '0' as i32) as libc::c_char;
    if minus != 0 {
        let fresh2 = dst;
        dst = dst.offset(-1);
        *fresh2 = '-' as i32 as libc::c_char;
    }
    wget_buffer_memcat(
        buf,
        dst.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(
                dst.offset_from(str.as_mut_ptr()) as libc::c_long as libc::c_ulong,
            )
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn convert_dec(
    mut buf: *mut wget_buffer,
    mut flags: libc::c_uint,
    mut field_width: libc::c_int,
    mut precision: libc::c_int,
    mut arg: libc::c_longlong,
) {
    let mut argu: libc::c_ulonglong = arg as libc::c_ulonglong;
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut minus: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut dst: *mut libc::c_char = str
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut c: libc::c_uchar = 0;
    let mut length: size_t = 0;
    if flags & 16 as libc::c_uint != 0 {
        if flags & 8 as libc::c_uint != 0 && arg < 0 as libc::c_int as libc::c_longlong {
            minus = 1 as libc::c_int as libc::c_char;
            argu = -arg as libc::c_ulonglong;
        }
        while argu != 0 {
            let fresh3 = dst;
            dst = dst.offset(-1);
            *fresh3 = argu
                .wrapping_rem(10 as libc::c_int as libc::c_ulonglong)
                .wrapping_add('0' as i32 as libc::c_ulonglong) as libc::c_char;
            argu = argu.wrapping_div(10 as libc::c_int as libc::c_ulonglong);
        }
    } else if flags & 64 as libc::c_uint != 0 {
        while argu != 0 {
            c = (argu & 0xf as libc::c_int as libc::c_ulonglong) as libc::c_uchar;
            let fresh4 = dst;
            dst = dst.offset(-1);
            *fresh4 = (if c as libc::c_int >= 10 as libc::c_int {
                c as libc::c_int + 'a' as i32 - 10 as libc::c_int
            } else {
                c as libc::c_int + '0' as i32
            }) as libc::c_char;
            argu >>= 4 as libc::c_int;
        }
    } else if flags & 128 as libc::c_uint != 0 {
        while argu != 0 {
            c = (argu & 0xf as libc::c_int as libc::c_ulonglong) as libc::c_uchar;
            let fresh5 = dst;
            dst = dst.offset(-1);
            *fresh5 = (if c as libc::c_int >= 10 as libc::c_int {
                c as libc::c_int + 'A' as i32 - 10 as libc::c_int
            } else {
                c as libc::c_int + '0' as i32
            }) as libc::c_char;
            argu >>= 4 as libc::c_int;
        }
    } else if flags & 32 as libc::c_uint != 0 {
        while argu != 0 {
            let fresh6 = dst;
            dst = dst.offset(-1);
            *fresh6 = (argu & 0x7 as libc::c_int as libc::c_ulonglong)
                .wrapping_add('0' as i32 as libc::c_ulonglong) as libc::c_char;
            argu >>= 3 as libc::c_int;
        }
    }
    dst = dst.offset(1);
    dst;
    length = (::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
        .wrapping_sub(
            dst.offset_from(str.as_mut_ptr()) as libc::c_long as libc::c_ulong,
        );
    if precision < 0 as libc::c_int {
        precision = 1 as libc::c_int;
    } else {
        flags &= !(1 as libc::c_uint);
    }
    if field_width != 0 {
        if field_width as libc::c_uint as size_t > length.wrapping_add(minus as size_t) {
            if flags & 2 as libc::c_uint != 0 {
                if minus != 0 {
                    wget_buffer_memset_append(
                        buf,
                        '-' as i32 as libc::c_char,
                        1 as libc::c_int as size_t,
                    );
                }
                if length < precision as libc::c_uint as size_t {
                    wget_buffer_memset_append(
                        buf,
                        '0' as i32 as libc::c_char,
                        (precision as size_t).wrapping_sub(length),
                    );
                    wget_buffer_memcat(buf, dst as *const libc::c_void, length);
                    if field_width > precision + minus as libc::c_int {
                        wget_buffer_memset_append(
                            buf,
                            ' ' as i32 as libc::c_char,
                            (field_width - precision - minus as libc::c_int) as size_t,
                        );
                    }
                } else {
                    wget_buffer_memcat(buf, dst as *const libc::c_void, length);
                    wget_buffer_memset_append(
                        buf,
                        ' ' as i32 as libc::c_char,
                        (field_width as size_t)
                            .wrapping_sub(length)
                            .wrapping_sub(minus as size_t),
                    );
                }
            } else {
                if length < precision as libc::c_uint as size_t {
                    if field_width > precision + minus as libc::c_int {
                        if flags & 1 as libc::c_uint != 0 {
                            if minus != 0 {
                                wget_buffer_memset_append(
                                    buf,
                                    '-' as i32 as libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                            }
                            wget_buffer_memset_append(
                                buf,
                                '0' as i32 as libc::c_char,
                                (field_width - precision - minus as libc::c_int) as size_t,
                            );
                        } else {
                            wget_buffer_memset_append(
                                buf,
                                ' ' as i32 as libc::c_char,
                                (field_width - precision - minus as libc::c_int) as size_t,
                            );
                            if minus != 0 {
                                wget_buffer_memset_append(
                                    buf,
                                    '-' as i32 as libc::c_char,
                                    1 as libc::c_int as size_t,
                                );
                            }
                        }
                    } else if minus != 0 {
                        wget_buffer_memset_append(
                            buf,
                            '-' as i32 as libc::c_char,
                            1 as libc::c_int as size_t,
                        );
                    }
                    wget_buffer_memset_append(
                        buf,
                        '0' as i32 as libc::c_char,
                        (precision as size_t).wrapping_sub(length),
                    );
                } else if flags & 1 as libc::c_uint != 0 {
                    if minus != 0 {
                        wget_buffer_memset_append(
                            buf,
                            '-' as i32 as libc::c_char,
                            1 as libc::c_int as size_t,
                        );
                    }
                    wget_buffer_memset_append(
                        buf,
                        '0' as i32 as libc::c_char,
                        (field_width as size_t)
                            .wrapping_sub(length)
                            .wrapping_sub(minus as size_t),
                    );
                } else {
                    wget_buffer_memset_append(
                        buf,
                        ' ' as i32 as libc::c_char,
                        (field_width as size_t)
                            .wrapping_sub(length)
                            .wrapping_sub(minus as size_t),
                    );
                    if minus != 0 {
                        wget_buffer_memset_append(
                            buf,
                            '-' as i32 as libc::c_char,
                            1 as libc::c_int as size_t,
                        );
                    }
                }
                wget_buffer_memcat(buf, dst as *const libc::c_void, length);
            }
        } else {
            if minus != 0 {
                wget_buffer_memset_append(
                    buf,
                    '-' as i32 as libc::c_char,
                    1 as libc::c_int as size_t,
                );
            }
            if length < precision as libc::c_uint as size_t {
                wget_buffer_memset_append(
                    buf,
                    '0' as i32 as libc::c_char,
                    (precision as size_t).wrapping_sub(length),
                );
            }
            wget_buffer_memcat(buf, dst as *const libc::c_void, length);
        }
    } else {
        if minus != 0 {
            wget_buffer_memset_append(
                buf,
                '-' as i32 as libc::c_char,
                1 as libc::c_int as size_t,
            );
        }
        if length < precision as libc::c_uint as size_t {
            wget_buffer_memset_append(
                buf,
                '0' as i32 as libc::c_char,
                (precision as size_t).wrapping_sub(length),
            );
        }
        wget_buffer_memcat(buf, dst as *const libc::c_void, length);
    };
}
unsafe extern "C" fn convert_pointer(
    mut buf: *mut wget_buffer,
    mut pointer: *mut libc::c_void,
) {
    static mut HEX: [libc::c_char; 16] = unsafe {
        *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"0123456789abcdef")
    };
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut dst: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: libc::c_int = 0;
    let mut arg: size_t = 0;
    if pointer.is_null() {
        wget_buffer_memcat(
            buf,
            b"0x0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
        return;
    } else {
        wget_buffer_memcat(
            buf,
            b"0x\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
    }
    arg = pointer as size_t;
    length = 0 as libc::c_int;
    dst = str
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as isize);
    dst = dst.offset(-1);
    *dst = 0 as libc::c_int as libc::c_char;
    loop {
        dst = dst.offset(-1);
        *dst = HEX[(arg & 0xf as libc::c_int as size_t) as usize];
        arg >>= 4 as libc::c_int;
        length += 1;
        length;
        if !(arg != 0) {
            break;
        }
    }
    wget_buffer_memcat(buf, dst as *const libc::c_void, length as size_t);
}
unsafe extern "C" fn read_precision(
    mut p: *const libc::c_char,
    mut out: *mut libc::c_int,
    mut precision_is_external: bool,
) -> *const libc::c_char {
    let mut precision: libc::c_int = 0;
    if precision_is_external {
        precision = *out;
        if precision < 0 as libc::c_int {
            precision = 0 as libc::c_int;
        }
        p = p.offset(1);
        p;
    } else if c_isdigit(*p as libc::c_int) {
        precision = 0 as libc::c_int;
        loop {
            precision = precision * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
            p = p.offset(1);
            if !c_isdigit(*p as libc::c_int) {
                break;
            }
        }
    } else {
        precision = -(1 as libc::c_int);
    }
    *out = precision;
    return p;
}
unsafe extern "C" fn read_flag_chars(
    mut p: *const libc::c_char,
    mut out: *mut libc::c_uint,
) -> *const libc::c_char {
    let mut flags: libc::c_uint = 0;
    flags = 0 as libc::c_int as libc::c_uint;
    while *p != 0 {
        if *p as libc::c_int == '0' as i32 {
            flags |= 1 as libc::c_uint;
        } else if *p as libc::c_int == '-' as i32 {
            flags |= 2 as libc::c_uint;
        } else {
            if !(*p as libc::c_int == '#' as i32) {
                break;
            }
            flags |= 4 as libc::c_uint;
        }
        p = p.offset(1);
        p;
    }
    *out = flags;
    return p;
}
unsafe extern "C" fn read_field_width(
    mut p: *const libc::c_char,
    mut out: *mut libc::c_int,
    mut flags: *mut libc::c_uint,
    mut width_is_external: bool,
) -> *const libc::c_char {
    let mut field_width: libc::c_int = 0;
    if width_is_external {
        field_width = *out;
        if field_width < 0 as libc::c_int {
            *flags |= 2 as libc::c_uint;
            field_width = -field_width;
        }
        p = p.offset(1);
        p;
    } else {
        field_width = 0 as libc::c_int;
        while c_isdigit(*p as libc::c_int) {
            field_width = field_width * 10 as libc::c_int
                + (*p as libc::c_int - '0' as i32);
            p = p.offset(1);
            p;
        }
    }
    *out = field_width;
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_vprintf_append(
    mut buf: *mut wget_buffer,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> size_t {
    let mut p: *const libc::c_char = fmt;
    let mut begin: *const libc::c_char = 0 as *const libc::c_char;
    let mut field_width: libc::c_int = 0;
    let mut precision: libc::c_int = 0;
    let mut flags: libc::c_uint = 0;
    let mut arg: libc::c_longlong = 0;
    let mut argu: libc::c_ulonglong = 0;
    if p.is_null() {
        return 0 as libc::c_int as size_t;
    }
    while *p != 0 {
        begin = p;
        while *p as libc::c_int != 0 && *p as libc::c_int != '%' as i32 {
            p = p.offset(1);
            p;
        }
        if p != begin {
            wget_buffer_memcat(
                buf,
                begin as *const libc::c_void,
                p.offset_from(begin) as libc::c_long as size_t,
            );
        }
        if *p == 0 {
            break;
        }
        p = p.offset(1);
        if *p as libc::c_int == 's' as i32 {
            let mut s: *const libc::c_char = args.arg::<*const libc::c_char>();
            wget_buffer_strcat(
                buf,
                if !s.is_null() {
                    s
                } else {
                    b"(null)\0" as *const u8 as *const libc::c_char
                },
            );
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == 'd' as i32 {
            convert_dec_fast(buf, args.arg::<libc::c_int>());
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == 'c' as i32 {
            let mut c: libc::c_char = args.arg::<libc::c_int>() as libc::c_char;
            wget_buffer_memcat(
                buf,
                &mut c as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == 'p' as i32 {
            convert_pointer(buf, args.arg::<*mut libc::c_void>());
            p = p.offset(1);
            p;
        } else if *p as libc::c_int == '%' as i32 {
            wget_buffer_memset_append(
                buf,
                '%' as i32 as libc::c_char,
                1 as libc::c_int as size_t,
            );
            p = p.offset(1);
            p;
        } else {
            p = read_flag_chars(p, &mut flags);
            if *p as libc::c_int == '*' as i32 {
                field_width = args.arg::<libc::c_int>();
                p = read_field_width(
                    p,
                    &mut field_width,
                    &mut flags,
                    1 as libc::c_int != 0,
                );
            } else {
                p = read_field_width(
                    p,
                    &mut field_width,
                    &mut flags,
                    0 as libc::c_int != 0,
                );
            }
            if *p as libc::c_int == '.' as i32 {
                p = p.offset(1);
                if *p as libc::c_int == '*' as i32 {
                    precision = args.arg::<libc::c_int>();
                    p = read_precision(p, &mut precision, 1 as libc::c_int != 0);
                } else {
                    p = read_precision(p, &mut precision, 0 as libc::c_int != 0);
                }
            } else {
                precision = -(1 as libc::c_int);
            }
            match *p as libc::c_int {
                122 => {
                    arg = args.arg::<ssize_t>() as libc::c_longlong;
                    argu = arg as size_t as libc::c_ulonglong;
                    p = p.offset(1);
                    p;
                }
                108 => {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'l' as i32
                    {
                        p = p.offset(2 as libc::c_int as isize);
                        arg = args.arg::<libc::c_longlong>();
                        argu = arg as libc::c_ulonglong;
                    } else {
                        p = p.offset(1);
                        p;
                        arg = args.arg::<libc::c_long>() as libc::c_longlong;
                        argu = arg as libc::c_ulong as libc::c_ulonglong;
                    }
                }
                76 => {
                    p = p.offset(1);
                    p;
                    arg = args.arg::<libc::c_longlong>();
                    argu = arg as libc::c_ulonglong;
                }
                104 => {
                    if *p.offset(1 as libc::c_int as isize) as libc::c_int == 'h' as i32
                    {
                        p = p.offset(2 as libc::c_int as isize);
                        arg = args.arg::<libc::c_int>() as libc::c_schar
                            as libc::c_longlong;
                        argu = arg as libc::c_uchar as libc::c_ulonglong;
                    } else {
                        p = p.offset(1);
                        p;
                        arg = args.arg::<libc::c_int>() as libc::c_short
                            as libc::c_longlong;
                        argu = arg as libc::c_ushort as libc::c_ulonglong;
                    }
                }
                115 => {
                    p = p.offset(1);
                    p;
                    copy_string(
                        buf,
                        flags,
                        field_width,
                        precision,
                        args.arg::<*const libc::c_char>(),
                    );
                    continue;
                }
                99 => {
                    let mut c_0: [libc::c_char; 2] = [
                        args.arg::<libc::c_int>() as libc::c_char,
                        0 as libc::c_int as libc::c_char,
                    ];
                    p = p.offset(1);
                    p;
                    copy_string(buf, flags, field_width, precision, c_0.as_mut_ptr());
                    continue;
                }
                112 => {
                    p = p.offset(1);
                    p;
                    convert_dec(
                        buf,
                        flags | 64 as libc::c_uint | 4 as libc::c_uint,
                        field_width,
                        precision,
                        args.arg::<*mut libc::c_void>() as ptrdiff_t as libc::c_longlong,
                    );
                    continue;
                }
                _ => {
                    arg = args.arg::<libc::c_int>() as libc::c_longlong;
                    argu = arg as libc::c_uint as libc::c_ulonglong;
                }
            }
            if *p as libc::c_int == 'd' as i32 || *p as libc::c_int == 'i' as i32 {
                convert_dec(
                    buf,
                    flags | 8 as libc::c_uint | 16 as libc::c_uint,
                    field_width,
                    precision,
                    arg,
                );
            } else if *p as libc::c_int == 'u' as i32 {
                convert_dec(
                    buf,
                    flags | 16 as libc::c_uint,
                    field_width,
                    precision,
                    argu as libc::c_longlong,
                );
            } else if *p as libc::c_int == 'x' as i32 {
                convert_dec(
                    buf,
                    flags | 64 as libc::c_uint,
                    field_width,
                    precision,
                    argu as libc::c_longlong,
                );
            } else if *p as libc::c_int == 'X' as i32 {
                convert_dec(
                    buf,
                    flags | 128 as libc::c_uint,
                    field_width,
                    precision,
                    argu as libc::c_longlong,
                );
            } else if *p as libc::c_int == 'o' as i32 {
                convert_dec(
                    buf,
                    flags | 32 as libc::c_uint,
                    field_width,
                    precision,
                    argu as libc::c_longlong,
                );
            } else {
                wget_buffer_memset_append(
                    buf,
                    '%' as i32 as libc::c_char,
                    1 as libc::c_int as size_t,
                );
                p = begin.offset(1 as libc::c_int as isize);
                continue;
            }
            p = p.offset(1);
            p;
        }
    }
    return (*buf).length;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_vprintf(
    mut buf: *mut wget_buffer,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> size_t {
    (*buf).length = 0 as libc::c_int as size_t;
    return wget_buffer_vprintf_append(buf, fmt, args.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_printf_append(
    mut buf: *mut wget_buffer,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    wget_buffer_vprintf_append(buf, fmt, args_0.as_va_list());
    return (*buf).length;
}
#[no_mangle]
pub unsafe extern "C" fn wget_buffer_printf(
    mut buf: *mut wget_buffer,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> size_t {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut len: size_t = wget_buffer_vprintf(buf, fmt, args_0.as_va_list());
    return len;
}
