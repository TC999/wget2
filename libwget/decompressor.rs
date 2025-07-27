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
    pub type internal_state;
    pub type ZSTD_DCtx_s;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit_(
        strm: z_streamp,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn ZSTD_isError(code: size_t) -> libc::c_uint;
    fn ZSTD_getErrorName(code: size_t) -> *const libc::c_char;
    fn ZSTD_createDStream() -> *mut ZSTD_DStream;
    fn ZSTD_freeDStream(zds: *mut ZSTD_DStream) -> size_t;
    fn ZSTD_initDStream(zds: *mut ZSTD_DStream) -> size_t;
    fn ZSTD_decompressStream(
        zds: *mut ZSTD_DStream,
        output: *mut ZSTD_outBuffer,
        input: *mut ZSTD_inBuffer,
    ) -> size_t;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type ZSTD_DCtx = ZSTD_DCtx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_inBuffer_s {
    pub src: *const libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_inBuffer = ZSTD_inBuffer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ZSTD_outBuffer_s {
    pub dst: *mut libc::c_void,
    pub size: size_t,
    pub pos: size_t,
}
pub type ZSTD_outBuffer = ZSTD_outBuffer_s;
pub type ZSTD_DStream = ZSTD_DCtx;
pub type uint8_t = __uint8_t;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_decompressor_st {
    pub z_strm: z_stream,
    pub zstd_strm: *mut ZSTD_DStream,
    pub sink: Option::<wget_decompressor_sink_fn>,
    pub error_handler: Option::<wget_decompressor_error_handler>,
    pub decompress: Option::<wget_decompressor_decompress_fn>,
    pub exit: Option::<wget_decompressor_exit_fn>,
    pub context: *mut libc::c_void,
    pub encoding: wget_content_encoding,
    pub inflating: bool,
}
pub type wget_content_encoding = libc::c_int;
pub const wget_content_encoding_max: wget_content_encoding = 9;
pub const wget_content_encoding_lzip: wget_content_encoding = 8;
pub const wget_content_encoding_zstd: wget_content_encoding = 7;
pub const wget_content_encoding_brotli: wget_content_encoding = 6;
pub const wget_content_encoding_bzip2: wget_content_encoding = 5;
pub const wget_content_encoding_lzma: wget_content_encoding = 4;
pub const wget_content_encoding_xz: wget_content_encoding = 3;
pub const wget_content_encoding_deflate: wget_content_encoding = 2;
pub const wget_content_encoding_gzip: wget_content_encoding = 1;
pub const wget_content_encoding_identity: wget_content_encoding = 0;
pub const wget_content_encoding_unknown: wget_content_encoding = -1;
pub type wget_decompressor_exit_fn = unsafe extern "C" fn(*mut wget_decompressor) -> ();
pub type wget_decompressor = wget_decompressor_st;
pub type wget_decompressor_decompress_fn = unsafe extern "C" fn(
    *mut wget_decompressor,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
pub type wget_decompressor_error_handler = unsafe extern "C" fn(
    *mut wget_decompressor,
    libc::c_int,
) -> libc::c_int;
pub type wget_decompressor_sink_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
unsafe extern "C" fn gzip_init(mut strm: *mut z_stream) -> libc::c_int {
    memset(
        strm as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<z_stream>() as libc::c_ulong,
    );
    if inflateInit2_(
        strm,
        15 as libc::c_int + 32 as libc::c_int,
        b"1.3\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    ) != 0 as libc::c_int
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to init gzip decompression\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gzip_decompress(
    mut dc: *mut wget_decompressor,
    mut src: *const libc::c_char,
    mut srclen: size_t,
) -> libc::c_int {
    let mut strm: *mut z_stream = 0 as *mut z_stream;
    let mut dst: [libc::c_char; 10240] = [0; 10240];
    let mut status: libc::c_int = 0;
    if srclen == 0 {
        if ((*dc).sink).is_some() {
            ((*dc).sink)
                .expect(
                    "non-null function pointer",
                )(
                (*dc).context,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        return 0 as libc::c_int;
    }
    strm = &mut (*dc).z_strm;
    '_restart: loop {
        (*strm).next_in = src as *const libc::c_uchar;
        (*strm).avail_in = srclen as libc::c_uint;
        loop {
            (*strm).next_out = dst.as_mut_ptr() as *mut libc::c_uchar;
            (*strm)
                .avail_out = ::core::mem::size_of::<[libc::c_char; 10240]>()
                as libc::c_ulong as uInt;
            status = inflate(strm, 2 as libc::c_int);
            if status == -(3 as libc::c_int) && !(*dc).inflating {
                inflateEnd(strm);
                if inflateInit2_(
                    strm,
                    -(15 as libc::c_int),
                    b"1.3\0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
                ) != 0 as libc::c_int
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to re-init deflate/gzip decompression\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    return -(1 as libc::c_int);
                }
                (*dc).inflating = 1 as libc::c_int != 0;
                break;
            } else {
                (*dc).inflating = 1 as libc::c_int != 0;
                if (status == 0 as libc::c_int || status == 1 as libc::c_int)
                    && ((*strm).avail_out as libc::c_ulong)
                        < ::core::mem::size_of::<[libc::c_char; 10240]>()
                            as libc::c_ulong
                {
                    if ((*dc).sink).is_some() {
                        ((*dc).sink)
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dc).context,
                            dst.as_mut_ptr(),
                            (::core::mem::size_of::<[libc::c_char; 10240]>()
                                as libc::c_ulong)
                                .wrapping_sub((*strm).avail_out as libc::c_ulong),
                        );
                    }
                }
                if !(status == 0 as libc::c_int && (*strm).avail_out == 0) {
                    break '_restart;
                }
            }
        }
    }
    if status == 0 as libc::c_int || status == -(5 as libc::c_int)
        || status == 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    wget_error_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"Failed to uncompress gzip stream (%d)\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        status,
    );
    return -(1 as libc::c_int);
}
unsafe extern "C" fn gzip_exit(mut dc: *mut wget_decompressor) {
    let mut status: libc::c_int = 0;
    status = inflateEnd(&mut (*dc).z_strm);
    if status != 0 as libc::c_int {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to close gzip stream (%d)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            status,
        );
    }
}
unsafe extern "C" fn deflate_init(mut strm: *mut z_stream) -> libc::c_int {
    memset(
        strm as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<z_stream>() as libc::c_ulong,
    );
    if inflateInit_(
        strm,
        b"1.3\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    ) != 0 as libc::c_int
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to init deflate decompression\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zstd_init(mut strm: *mut *mut ZSTD_DStream) -> libc::c_int {
    *strm = ZSTD_createDStream();
    if (*strm).is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to create Zstandard decompression\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    let mut rc: size_t = ZSTD_initDStream(*strm);
    if ZSTD_isError(rc) != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to init Zstandard decompression: %s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            ZSTD_getErrorName(rc),
        );
        ZSTD_freeDStream(*strm);
        *strm = 0 as *mut ZSTD_DStream;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zstd_decompress(
    mut dc: *mut wget_decompressor,
    mut src: *const libc::c_char,
    mut srclen: size_t,
) -> libc::c_int {
    let mut strm: *mut ZSTD_DStream = 0 as *mut ZSTD_DStream;
    let mut dst: [uint8_t; 10240] = [0; 10240];
    if srclen == 0 {
        if ((*dc).sink).is_some() {
            ((*dc).sink)
                .expect(
                    "non-null function pointer",
                )(
                (*dc).context,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
        }
        return 0 as libc::c_int;
    }
    strm = (*dc).zstd_strm;
    let mut input: ZSTD_inBuffer = {
        let mut init = ZSTD_inBuffer_s {
            src: src as *const libc::c_void,
            size: srclen,
            pos: 0 as libc::c_int as size_t,
        };
        init
    };
    while input.pos < input.size {
        let mut output: ZSTD_outBuffer = {
            let mut init = ZSTD_outBuffer_s {
                dst: dst.as_mut_ptr() as *mut libc::c_void,
                size: ::core::mem::size_of::<[uint8_t; 10240]>() as libc::c_ulong,
                pos: 0 as libc::c_int as size_t,
            };
            init
        };
        let mut rc: size_t = ZSTD_decompressStream(strm, &mut output, &mut input);
        if ZSTD_isError(rc) != 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to uncompress Zstandard stream: %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                ZSTD_getErrorName(rc),
            );
            return -(1 as libc::c_int);
        }
        if output.pos != 0 && ((*dc).sink).is_some() {
            ((*dc).sink)
                .expect(
                    "non-null function pointer",
                )((*dc).context, dst.as_mut_ptr() as *mut libc::c_char, output.pos);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zstd_exit(mut dc: *mut wget_decompressor) {
    ZSTD_freeDStream((*dc).zstd_strm);
}
unsafe extern "C" fn identity(
    mut dc: *mut wget_decompressor,
    mut src: *const libc::c_char,
    mut srclen: size_t,
) -> libc::c_int {
    if ((*dc).sink).is_some() {
        ((*dc).sink).expect("non-null function pointer")((*dc).context, src, srclen);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_decompress_open(
    mut encoding: wget_content_encoding,
    mut sink: Option::<wget_decompressor_sink_fn>,
    mut context: *mut libc::c_void,
) -> *mut wget_decompressor {
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut dc: *mut wget_decompressor = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_decompressor>() as libc::c_ulong,
    ) as *mut wget_decompressor;
    if dc.is_null() {
        return 0 as *mut wget_decompressor;
    }
    if encoding as libc::c_int == wget_content_encoding_gzip as libc::c_int {
        rc = gzip_init(&mut (*dc).z_strm);
        if rc == 0 as libc::c_int {
            (*dc)
                .decompress = Some(
                gzip_decompress
                    as unsafe extern "C" fn(
                        *mut wget_decompressor,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            );
            (*dc)
                .exit = Some(
                gzip_exit as unsafe extern "C" fn(*mut wget_decompressor) -> (),
            );
        }
    } else if encoding as libc::c_int == wget_content_encoding_deflate as libc::c_int {
        rc = deflate_init(&mut (*dc).z_strm);
        if rc == 0 as libc::c_int {
            (*dc)
                .decompress = Some(
                gzip_decompress
                    as unsafe extern "C" fn(
                        *mut wget_decompressor,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            );
            (*dc)
                .exit = Some(
                gzip_exit as unsafe extern "C" fn(*mut wget_decompressor) -> (),
            );
        }
    } else if !(encoding as libc::c_int == wget_content_encoding_bzip2 as libc::c_int) {
        if !(encoding as libc::c_int == wget_content_encoding_lzma as libc::c_int
            || encoding as libc::c_int == wget_content_encoding_xz as libc::c_int)
        {
            if !(encoding as libc::c_int == wget_content_encoding_brotli as libc::c_int)
            {
                if encoding as libc::c_int == wget_content_encoding_zstd as libc::c_int {
                    rc = zstd_init(&mut (*dc).zstd_strm);
                    if rc == 0 as libc::c_int {
                        (*dc)
                            .decompress = Some(
                            zstd_decompress
                                as unsafe extern "C" fn(
                                    *mut wget_decompressor,
                                    *const libc::c_char,
                                    size_t,
                                ) -> libc::c_int,
                        );
                        (*dc)
                            .exit = Some(
                            zstd_exit
                                as unsafe extern "C" fn(*mut wget_decompressor) -> (),
                        );
                    }
                } else {
                    encoding as libc::c_int == wget_content_encoding_lzip as libc::c_int;
                }
            }
        }
    }
    if ((*dc).decompress).is_none() {
        if encoding as libc::c_int != wget_content_encoding_identity as libc::c_int {
            wget_debug_printf(
                b"Falling back to Content-Encoding 'identity'\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*dc)
            .decompress = Some(
            identity
                as unsafe extern "C" fn(
                    *mut wget_decompressor,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
        );
    }
    if rc != 0 {
        if !dc.is_null() {
            wget_free.expect("non-null function pointer")(dc as *mut libc::c_void);
            dc = 0 as *mut wget_decompressor;
        }
        return 0 as *mut wget_decompressor;
    }
    (*dc).encoding = encoding;
    (*dc).sink = sink;
    (*dc).context = context;
    return dc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_decompress_close(mut dc: *mut wget_decompressor) {
    if !dc.is_null() {
        if ((*dc).exit).is_some() {
            ((*dc).exit).expect("non-null function pointer")(dc);
        }
        if !dc.is_null() {
            wget_free.expect("non-null function pointer")(dc as *mut libc::c_void);
            dc = 0 as *mut wget_decompressor;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_decompress(
    mut dc: *mut wget_decompressor,
    mut src: *const libc::c_char,
    mut srclen: size_t,
) -> libc::c_int {
    if !dc.is_null() {
        let mut rc: libc::c_int = ((*dc).decompress)
            .expect("non-null function pointer")(dc, src, srclen);
        if rc != 0 && ((*dc).error_handler).is_some() {
            ((*dc).error_handler).expect("non-null function pointer")(dc, rc);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_decompress_set_error_handler(
    mut dc: *mut wget_decompressor,
    mut error_handler: Option::<wget_decompressor_error_handler>,
) {
    if !dc.is_null() {
        (*dc).error_handler = error_handler;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_decompress_get_context(
    mut dc: *mut wget_decompressor,
) -> *mut libc::c_void {
    return if !dc.is_null() { (*dc).context } else { 0 as *mut libc::c_void };
}
static mut _encoding_names: [[libc::c_char; 9]; 9] = unsafe {
    [
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"identity\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"gzip\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"deflate\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"xz\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"lzma\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"bzip2\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"br\0\0\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"zstd\0\0\0\0\0"),
        *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"lzip\0\0\0\0\0"),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn wget_content_encoding_by_name(
    mut name: *const libc::c_char,
) -> wget_content_encoding {
    if !name.is_null() {
        let mut it: wget_content_encoding = wget_content_encoding_identity;
        while (it as libc::c_int) < wget_content_encoding_max as libc::c_int {
            if strcmp((_encoding_names[it as usize]).as_mut_ptr(), name) == 0 {
                return it;
            }
            it += 1;
            it;
        }
        if strcmp(b"none\0" as *const u8 as *const libc::c_char, name) == 0 {
            return wget_content_encoding_identity;
        }
    }
    return wget_content_encoding_unknown;
}
#[no_mangle]
pub unsafe extern "C" fn wget_content_encoding_to_name(
    mut type_0: wget_content_encoding,
) -> *const libc::c_char {
    if type_0 as libc::c_int >= 0 as libc::c_int
        && (type_0 as libc::c_int) < wget_content_encoding_max as libc::c_int
    {
        return (_encoding_names[type_0 as usize]).as_mut_ptr();
    }
    return 0 as *const libc::c_char;
}
