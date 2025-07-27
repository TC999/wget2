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
    pub type wget_vector_st;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_base64_decode_alloc(
        src: *const libc::c_char,
        n: size_t,
        outlen: *mut size_t,
    ) -> *mut libc::c_char;
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __int64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int64_t = __int64_t;
pub type wget_calloc_function = unsafe extern "C" fn(
    size_t,
    size_t,
) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_hpkp_st {
    pub host: *const libc::c_char,
    pub expires: int64_t,
    pub created: int64_t,
    pub maxage: int64_t,
    pub pins: *mut wget_vector,
    #[bitfield(name = "include_subdomains", ty = "bool", bits = "0..=0")]
    pub include_subdomains: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type wget_hpkp = wget_hpkp_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hpkp_pin {
    pub pin_b64: *const libc::c_char,
    pub pin: *const libc::c_void,
    pub hash_type: *const libc::c_char,
    pub pinsize: size_t,
}
#[inline]
unsafe extern "C" fn wget_calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return wget_calloc_fn.expect("non-null function pointer")(nmemb, size);
}
unsafe extern "C" fn compare_pin(
    mut p1: *mut wget_hpkp_pin,
    mut p2: *mut wget_hpkp_pin,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = strcmp((*p1).hash_type, (*p2).hash_type);
    if n != 0 {
        return n;
    }
    if (*p1).pinsize < (*p2).pinsize {
        return -(1 as libc::c_int);
    }
    if (*p1).pinsize > (*p2).pinsize {
        return 1 as libc::c_int;
    }
    return memcmp((*p1).pin, (*p2).pin, (*p1).pinsize);
}
unsafe extern "C" fn hpkp_pin_free(mut pin: *mut libc::c_void) {
    let mut p: *mut wget_hpkp_pin = pin as *mut wget_hpkp_pin;
    if !p.is_null() {
        if !((*p).hash_type).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*p).hash_type as *mut libc::c_void);
            (*p).hash_type = 0 as *const libc::c_char;
        }
        if !((*p).pin).is_null() {
            wget_free.expect("non-null function pointer")((*p).pin as *mut libc::c_void);
            (*p).pin = 0 as *const libc::c_void;
        }
        if !((*p).pin_b64).is_null() {
            wget_free
                .expect("non-null function pointer")((*p).pin_b64 as *mut libc::c_void);
            (*p).pin_b64 = 0 as *const libc::c_char;
        }
        if !p.is_null() {
            wget_free.expect("non-null function pointer")(p as *mut libc::c_void);
            p = 0 as *mut wget_hpkp_pin;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_pin_add(
    mut hpkp: *mut wget_hpkp,
    mut pin_type: *const libc::c_char,
    mut pin_b64: *const libc::c_char,
) {
    let mut pin: *mut wget_hpkp_pin = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_hpkp_pin>() as libc::c_ulong,
    ) as *mut wget_hpkp_pin;
    if pin.is_null() {
        return;
    }
    let mut len_b64: size_t = strlen(pin_b64);
    (*pin).hash_type = wget_strdup(pin_type);
    (*pin).pin_b64 = wget_strdup(pin_b64);
    (*pin)
        .pin = wget_base64_decode_alloc(pin_b64, len_b64, &mut (*pin).pinsize)
        as *mut libc::c_uchar as *const libc::c_void;
    if ((*hpkp).pins).is_null() {
        (*hpkp)
            .pins = wget_vector_create(
            5 as libc::c_int,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut wget_hpkp_pin,
                        *mut wget_hpkp_pin,
                    ) -> libc::c_int,
                >,
                Option::<wget_vector_compare_fn>,
            >(
                Some(
                    compare_pin
                        as unsafe extern "C" fn(
                            *mut wget_hpkp_pin,
                            *mut wget_hpkp_pin,
                        ) -> libc::c_int,
                ),
            ),
        );
        wget_vector_set_destructor(
            (*hpkp).pins,
            Some(hpkp_pin_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    wget_vector_add((*hpkp).pins, pin as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_free(mut hpkp: *mut wget_hpkp) {
    if !hpkp.is_null() {
        if !((*hpkp).host).is_null() {
            wget_free
                .expect("non-null function pointer")((*hpkp).host as *mut libc::c_void);
            (*hpkp).host = 0 as *const libc::c_char;
        }
        wget_vector_free(&mut (*hpkp).pins);
        if !hpkp.is_null() {
            wget_free.expect("non-null function pointer")(hpkp as *mut libc::c_void);
            hpkp = 0 as *mut wget_hpkp;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_new() -> *mut wget_hpkp {
    let mut hpkp: *mut wget_hpkp = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_hpkp>() as libc::c_ulong,
    ) as *mut wget_hpkp;
    if !hpkp.is_null() {
        (*hpkp).created = time(0 as *mut time_t);
    }
    return hpkp;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_set_host(
    mut hpkp: *mut wget_hpkp,
    mut host: *const libc::c_char,
) {
    if !((*hpkp).host).is_null() {
        wget_free.expect("non-null function pointer")((*hpkp).host as *mut libc::c_void);
        (*hpkp).host = 0 as *const libc::c_char;
    }
    (*hpkp).host = wget_strdup(host);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_set_maxage(
    mut hpkp: *mut wget_hpkp,
    mut maxage: int64_t,
) {
    let mut now: int64_t = 0;
    if maxage <= 0 as libc::c_int as int64_t
        || maxage
            >= 9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long
        || {
            now = time(0 as *mut time_t);
            now < 0 as libc::c_int as int64_t
        }
        || now >= 9223372036854775807 as libc::c_long / 2 as libc::c_int as libc::c_long
    {
        (*hpkp).maxage = 0 as libc::c_int as int64_t;
        (*hpkp).expires = 0 as libc::c_int as int64_t;
    } else {
        (*hpkp).maxage = maxage;
        (*hpkp).expires = now + maxage;
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_set_include_subdomains(
    mut hpkp: *mut wget_hpkp,
    mut include_subdomains: bool,
) {
    (*hpkp).set_include_subdomains(include_subdomains);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_get_n_pins(mut hpkp: *mut wget_hpkp) -> libc::c_int {
    return wget_vector_size((*hpkp).pins);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_get_pins_b64(
    mut hpkp: *mut wget_hpkp,
    mut pin_types: *mut *const libc::c_char,
    mut pins_b64: *mut *const libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut n_pins: libc::c_int = 0;
    n_pins = wget_vector_size((*hpkp).pins);
    i = 0 as libc::c_int;
    while i < n_pins {
        let mut pin: *mut wget_hpkp_pin = wget_vector_get((*hpkp).pins, i)
            as *mut wget_hpkp_pin;
        if !pin.is_null() {
            let ref mut fresh0 = *pin_types.offset(i as isize);
            *fresh0 = (*pin).hash_type;
            let ref mut fresh1 = *pins_b64.offset(i as isize);
            *fresh1 = (*pin).pin_b64;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_get_pins(
    mut hpkp: *mut wget_hpkp,
    mut pin_types: *mut *const libc::c_char,
    mut sizes: *mut size_t,
    mut pins: *mut *const libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut n_pins: libc::c_int = 0;
    n_pins = wget_vector_size((*hpkp).pins);
    i = 0 as libc::c_int;
    while i < n_pins {
        let mut pin: *mut wget_hpkp_pin = wget_vector_get((*hpkp).pins, i)
            as *mut wget_hpkp_pin;
        if !pin.is_null() {
            let ref mut fresh2 = *pin_types.offset(i as isize);
            *fresh2 = (*pin).hash_type;
            *sizes.offset(i as isize) = (*pin).pinsize;
            let ref mut fresh3 = *pins.offset(i as isize);
            *fresh3 = (*pin).pin;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_get_host(
    mut hpkp: *mut wget_hpkp,
) -> *const libc::c_char {
    return (*hpkp).host;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_get_maxage(mut hpkp: *mut wget_hpkp) -> int64_t {
    return (*hpkp).maxage;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hpkp_get_include_subdomains(
    mut hpkp: *mut wget_hpkp,
) -> bool {
    return (*hpkp).include_subdomains();
}
