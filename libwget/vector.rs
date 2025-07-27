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
    fn free(_: *mut libc::c_void);
    fn qsort_r(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_d_fn_t,
        __arg: *mut libc::c_void,
    );
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_vaprintf(
        fmt: *const libc::c_char,
        args: ::core::ffi::VaList,
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
pub type __compar_d_fn_t = Option::<
    unsafe extern "C" fn(
        *const libc::c_void,
        *const libc::c_void,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_vector_st {
    pub cmp: Option::<wget_vector_compare_fn>,
    pub destructor: Option::<wget_vector_destructor>,
    pub entry: *mut *mut libc::c_void,
    pub max: libc::c_int,
    pub cur: libc::c_int,
    #[bitfield(name = "sorted", ty = "bool", bits = "0..=0")]
    pub sorted: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub resize_factor: libc::c_float,
}
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_vector = wget_vector_st;
pub type wget_vector_find_fn = unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int;
pub type wget_vector_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn wget_vector_create(
    mut max: libc::c_int,
    mut cmp: Option::<wget_vector_compare_fn>,
) -> *mut wget_vector {
    let mut v: *mut wget_vector = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_vector>() as libc::c_ulong,
    ) as *mut wget_vector;
    if v.is_null() {
        return 0 as *mut wget_vector;
    }
    (*v)
        .entry = wget_malloc(
        (max as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    if ((*v).entry).is_null() {
        if !v.is_null() {
            wget_free.expect("non-null function pointer")(v as *mut libc::c_void);
            v = 0 as *mut wget_vector;
        }
        return 0 as *mut wget_vector;
    }
    (*v).max = max;
    (*v).resize_factor = 2 as libc::c_int as libc::c_float;
    (*v).cmp = cmp;
    (*v).destructor = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_set_resize_factor(
    mut v: *mut wget_vector,
    mut factor: libc::c_float,
) {
    if !v.is_null() {
        (*v).resize_factor = factor;
    }
}
unsafe extern "C" fn insert_element(
    mut v: *mut wget_vector,
    mut elem: *const libc::c_void,
    mut pos: libc::c_int,
    mut replace: libc::c_int,
) -> libc::c_int {
    if pos < 0 as libc::c_int || v.is_null() || pos > (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if replace == 0 {
        if (*v).max == (*v).cur {
            let mut newsize: libc::c_int = ((*v).max as libc::c_float
                * (*v).resize_factor) as libc::c_int;
            if newsize <= (*v).max {
                return WGET_E_INVALID as libc::c_int;
            }
            let mut tmp: *mut *mut libc::c_void = wget_realloc(
                (*v).entry as *mut libc::c_void,
                (newsize as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
            ) as *mut *mut libc::c_void;
            if tmp.is_null() {
                return WGET_E_MEMORY as libc::c_int;
            }
            (*v).entry = tmp;
            (*v).max = newsize;
        }
        memmove(
            &mut *((*v).entry).offset((pos + 1 as libc::c_int) as isize)
                as *mut *mut libc::c_void as *mut libc::c_void,
            &mut *((*v).entry).offset(pos as isize) as *mut *mut libc::c_void
                as *const libc::c_void,
            (((*v).cur - pos) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
        (*v).cur += 1;
        (*v).cur;
    }
    let ref mut fresh0 = *((*v).entry).offset(pos as isize);
    *fresh0 = elem as *mut libc::c_void;
    if ((*v).cmp).is_some() {
        if (*v).cur == 1 as libc::c_int {
            (*v).set_sorted(1 as libc::c_int != 0);
        } else if (*v).cur > 1 as libc::c_int && (*v).sorted() as libc::c_int != 0 {
            if pos == 0 as libc::c_int {
                if ((*v).cmp)
                    .expect(
                        "non-null function pointer",
                    )(elem, *((*v).entry).offset(1 as libc::c_int as isize))
                    > 0 as libc::c_int
                {
                    (*v).set_sorted(0 as libc::c_int != 0);
                }
            } else if pos == (*v).cur - 1 as libc::c_int {
                if ((*v).cmp)
                    .expect(
                        "non-null function pointer",
                    )(elem, *((*v).entry).offset(((*v).cur - 2 as libc::c_int) as isize))
                    < 0 as libc::c_int
                {
                    (*v).set_sorted(0 as libc::c_int != 0);
                }
            } else if ((*v).cmp)
                .expect(
                    "non-null function pointer",
                )(elem, *((*v).entry).offset((pos - 1 as libc::c_int) as isize))
                < 0 as libc::c_int
                || ((*v).cmp)
                    .expect(
                        "non-null function pointer",
                    )(elem, *((*v).entry).offset((pos + 1 as libc::c_int) as isize))
                    > 0 as libc::c_int
            {
                (*v).set_sorted(0 as libc::c_int != 0);
            }
        }
    }
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_insert(
    mut v: *mut wget_vector,
    mut elem: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_int {
    return insert_element(v, elem, pos, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_insert_sorted(
    mut v: *mut wget_vector,
    mut elem: *const libc::c_void,
) -> libc::c_int {
    if v.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if ((*v).cmp).is_none() {
        return insert_element(v, elem, (*v).cur, 0 as libc::c_int);
    }
    if !(*v).sorted() {
        wget_vector_sort(v);
    }
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = (*v).cur - 1 as libc::c_int;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    while l <= r {
        m = (l + r) / 2 as libc::c_int;
        res = ((*v).cmp)
            .expect("non-null function pointer")(elem, *((*v).entry).offset(m as isize));
        if res > 0 as libc::c_int {
            l = m + 1 as libc::c_int;
        } else if res < 0 as libc::c_int {
            r = m - 1 as libc::c_int;
        } else {
            return insert_element(v, elem, m, 0 as libc::c_int)
        }
    }
    if res > 0 as libc::c_int {
        m += 1;
        m;
    }
    return insert_element(v, elem, m, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_add_memdup(
    mut v: *mut wget_vector,
    mut elem: *const libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    let mut elemp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rc: libc::c_int = 0;
    if v.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    elemp = wget_memdup(elem, size);
    if elemp.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    rc = insert_element(v, elemp, (*v).cur, 0 as libc::c_int);
    if rc < 0 as libc::c_int {
        if !elemp.is_null() {
            wget_free.expect("non-null function pointer")(elemp);
            elemp = 0 as *mut libc::c_void;
        }
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_add(
    mut v: *mut wget_vector,
    mut elem: *const libc::c_void,
) -> libc::c_int {
    return if !v.is_null() {
        insert_element(v, elem, (*v).cur, 0 as libc::c_int)
    } else {
        WGET_E_INVALID as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_add_vprintf(
    mut v: *mut wget_vector,
    mut fmt: *const libc::c_char,
    mut args: ::core::ffi::VaList,
) -> libc::c_int {
    if v.is_null() || fmt.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    let mut p: *mut libc::c_char = wget_vaprintf(fmt, args.as_va_list());
    if p.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    return insert_element(v, p as *const libc::c_void, (*v).cur, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_add_printf(
    mut v: *mut wget_vector,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    if v.is_null() || fmt.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    let mut p: *mut libc::c_char = wget_vaprintf(fmt, args_0.as_va_list());
    if p.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    return insert_element(v, p as *const libc::c_void, (*v).cur, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_replace(
    mut v: *mut wget_vector,
    mut elem: *const libc::c_void,
    mut pos: libc::c_int,
) -> libc::c_int {
    if v.is_null() || pos < 0 as libc::c_int || pos >= (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if ((*v).destructor).is_some() {
        ((*v).destructor)
            .expect("non-null function pointer")(*((*v).entry).offset(pos as isize));
    }
    return insert_element(v, elem, pos, 1 as libc::c_int);
}
unsafe extern "C" fn remove_element(
    mut v: *mut wget_vector,
    mut pos: libc::c_int,
    mut free_entry: libc::c_int,
) -> libc::c_int {
    if pos < 0 as libc::c_int || v.is_null() || pos >= (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if free_entry != 0 {
        if ((*v).destructor).is_some() {
            ((*v).destructor)
                .expect("non-null function pointer")(*((*v).entry).offset(pos as isize));
        }
    }
    memmove(
        &mut *((*v).entry).offset(pos as isize) as *mut *mut libc::c_void
            as *mut libc::c_void,
        &mut *((*v).entry).offset((pos + 1 as libc::c_int) as isize)
            as *mut *mut libc::c_void as *const libc::c_void,
        (((*v).cur - pos - 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    );
    (*v).cur -= 1;
    (*v).cur;
    return pos;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_remove(
    mut v: *mut wget_vector,
    mut pos: libc::c_int,
) -> libc::c_int {
    return remove_element(v, pos, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_remove_nofree(
    mut v: *mut wget_vector,
    mut pos: libc::c_int,
) -> libc::c_int {
    return remove_element(v, pos, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_move(
    mut v: *mut wget_vector,
    mut old_pos: libc::c_int,
    mut new_pos: libc::c_int,
) -> libc::c_int {
    if v.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if old_pos < 0 as libc::c_int || old_pos >= (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if new_pos < 0 as libc::c_int || new_pos >= (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if old_pos == new_pos {
        return new_pos;
    }
    if (*v).sorted() as libc::c_int != 0 && ((*v).cmp).is_some()
        && ((*v).cmp)
            .expect(
                "non-null function pointer",
            )(
            *((*v).entry).offset(old_pos as isize),
            *((*v).entry).offset(new_pos as isize),
        ) != 0
    {
        (*v).set_sorted(0 as libc::c_int != 0);
    }
    let mut tmp: *mut libc::c_void = *((*v).entry).offset(old_pos as isize);
    if old_pos < new_pos {
        memmove(
            &mut *((*v).entry).offset(old_pos as isize) as *mut *mut libc::c_void
                as *mut libc::c_void,
            &mut *((*v).entry).offset((old_pos + 1 as libc::c_int) as isize)
                as *mut *mut libc::c_void as *const libc::c_void,
            ((new_pos - old_pos) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    } else {
        memmove(
            &mut *((*v).entry).offset((new_pos + 1 as libc::c_int) as isize)
                as *mut *mut libc::c_void as *mut libc::c_void,
            &mut *((*v).entry).offset(new_pos as isize) as *mut *mut libc::c_void
                as *const libc::c_void,
            ((old_pos - new_pos) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    }
    let ref mut fresh1 = *((*v).entry).offset(new_pos as isize);
    *fresh1 = tmp;
    return new_pos;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_swap(
    mut v: *mut wget_vector,
    mut pos1: libc::c_int,
    mut pos2: libc::c_int,
) -> libc::c_int {
    if v.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if pos1 < 0 as libc::c_int || pos1 >= (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if pos2 < 0 as libc::c_int || pos2 >= (*v).cur {
        return WGET_E_INVALID as libc::c_int;
    }
    if pos1 == pos2 {
        return pos2;
    }
    let mut tmp: *mut libc::c_void = *((*v).entry).offset(pos1 as isize);
    let ref mut fresh2 = *((*v).entry).offset(pos1 as isize);
    *fresh2 = *((*v).entry).offset(pos2 as isize);
    let ref mut fresh3 = *((*v).entry).offset(pos2 as isize);
    *fresh3 = tmp;
    if (*v).sorted() as libc::c_int != 0 && ((*v).cmp).is_some()
        && ((*v).cmp)
            .expect(
                "non-null function pointer",
            )(*((*v).entry).offset(pos1 as isize), *((*v).entry).offset(pos2 as isize))
            != 0
    {
        (*v).set_sorted(0 as libc::c_int != 0);
    }
    return pos2;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_free(mut v: *mut *mut wget_vector) {
    if !v.is_null() && !(*v).is_null() {
        wget_vector_clear(*v);
        if !((**v).entry).is_null() {
            wget_free
                .expect("non-null function pointer")((**v).entry as *mut libc::c_void);
            (**v).entry = 0 as *mut *mut libc::c_void;
        }
        if !(*v).is_null() {
            wget_free.expect("non-null function pointer")(*v as *mut libc::c_void);
            *v = 0 as *mut wget_vector;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_clear(mut v: *mut wget_vector) {
    if !v.is_null() {
        if ((*v).destructor).is_some() {
            let mut it: libc::c_int = 0 as libc::c_int;
            while it < (*v).cur {
                ((*v).destructor)
                    .expect(
                        "non-null function pointer",
                    )(*((*v).entry).offset(it as isize));
                let ref mut fresh4 = *((*v).entry).offset(it as isize);
                *fresh4 = 0 as *mut libc::c_void;
                it += 1;
                it;
            }
        }
        (*v).cur = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_clear_nofree(mut v: *mut wget_vector) {
    if !v.is_null() {
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < (*v).cur {
            let ref mut fresh5 = *((*v).entry).offset(it as isize);
            *fresh5 = 0 as *mut libc::c_void;
            it += 1;
            it;
        }
        (*v).cur = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_size(mut v: *const wget_vector) -> libc::c_int {
    return if !v.is_null() { (*v).cur } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_get(
    mut v: *const wget_vector,
    mut pos: libc::c_int,
) -> *mut libc::c_void {
    if pos < 0 as libc::c_int || v.is_null() || pos >= (*v).cur {
        return 0 as *mut libc::c_void;
    }
    return *((*v).entry).offset(pos as isize);
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_browse(
    mut v: *const wget_vector,
    mut browse: Option::<wget_vector_browse_fn>,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if !v.is_null() {
        let mut ret: libc::c_int = 0;
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < (*v).cur {
            ret = browse
                .expect(
                    "non-null function pointer",
                )(ctx, *((*v).entry).offset(it as isize));
            if ret != 0 as libc::c_int {
                return ret;
            }
            it += 1;
            it;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_setcmpfunc(
    mut v: *mut wget_vector,
    mut cmp: Option::<wget_vector_compare_fn>,
) {
    if !v.is_null() {
        (*v).cmp = cmp;
        if (*v).cur == 1 as libc::c_int {
            (*v).set_sorted(1 as libc::c_int != 0);
        } else {
            (*v).set_sorted(0 as libc::c_int != 0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_set_destructor(
    mut v: *mut wget_vector,
    mut destructor: Option::<wget_vector_destructor>,
) {
    if !v.is_null() {
        (*v).destructor = destructor;
    }
}
unsafe extern "C" fn compare_element(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
    mut v: *mut libc::c_void,
) -> libc::c_int {
    return ((*(v as *mut wget_vector)).cmp)
        .expect(
            "non-null function pointer",
        )(*(p1 as *mut *mut libc::c_void), *(p2 as *mut *mut libc::c_void));
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_sort(mut v: *mut wget_vector) {
    if !v.is_null() && ((*v).cmp).is_some() {
        qsort_r(
            (*v).entry as *mut libc::c_void,
            (*v).cur as size_t,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
            Some(
                compare_element
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            v as *mut libc::c_void,
        );
        (*v).set_sorted(1 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_find(
    mut v: *const wget_vector,
    mut elem: *const libc::c_void,
) -> libc::c_int {
    if v.is_null() || ((*v).cmp).is_none() {
        return WGET_E_INVALID as libc::c_int;
    }
    if (*v).cur == 1 as libc::c_int {
        if ((*v).cmp)
            .expect(
                "non-null function pointer",
            )(elem, *((*v).entry).offset(0 as libc::c_int as isize)) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    } else if (*v).sorted() {
        let mut l: libc::c_int = 0 as libc::c_int;
        let mut r: libc::c_int = (*v).cur - 1 as libc::c_int;
        while l <= r {
            let mut res: libc::c_int = 0;
            let mut m: libc::c_int = (l + r) / 2 as libc::c_int;
            res = ((*v).cmp)
                .expect(
                    "non-null function pointer",
                )(elem, *((*v).entry).offset(m as isize));
            if res > 0 as libc::c_int {
                l = m + 1 as libc::c_int;
            } else if res < 0 as libc::c_int {
                r = m - 1 as libc::c_int;
            } else {
                return m
            }
        }
    } else {
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < (*v).cur {
            if ((*v).cmp)
                .expect(
                    "non-null function pointer",
                )(elem, *((*v).entry).offset(it as isize)) == 0 as libc::c_int
            {
                return it;
            }
            it += 1;
            it;
        }
    }
    return WGET_E_UNKNOWN as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_contains(
    mut v: *const wget_vector,
    mut elem: *const libc::c_void,
) -> bool {
    return wget_vector_find(v, elem) >= 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_vector_findext(
    mut v: *const wget_vector,
    mut start: libc::c_int,
    mut direction: libc::c_int,
    mut find: Option::<wget_vector_find_fn>,
) -> libc::c_int {
    if v.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if direction != 0 {
        if start < (*v).cur {
            let mut it: libc::c_int = start;
            while it >= 0 as libc::c_int {
                if find
                    .expect(
                        "non-null function pointer",
                    )(*((*v).entry).offset(it as isize)) == 0 as libc::c_int
                {
                    return it;
                }
                it -= 1;
                it;
            }
        }
    } else if start >= 0 as libc::c_int {
        let mut it_0: libc::c_int = start;
        while it_0 < (*v).cur {
            if find
                .expect("non-null function pointer")(*((*v).entry).offset(it_0 as isize))
                == 0 as libc::c_int
            {
                return it_0;
            }
            it_0 += 1;
            it_0;
        }
    }
    return WGET_E_UNKNOWN as libc::c_int;
}
