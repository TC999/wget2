#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn free(_: *mut libc::c_void);
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_free: Option::<wget_free_function>;
}
pub type size_t = libc::c_ulong;
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
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hashmap_st {
    pub hash: Option::<wget_hashmap_hash_fn>,
    pub cmp: Option::<wget_hashmap_compare_fn>,
    pub key_destructor: Option::<wget_hashmap_key_destructor>,
    pub value_destructor: Option::<wget_hashmap_value_destructor>,
    pub entry: *mut *mut entry_t,
    pub max: libc::c_int,
    pub cur: libc::c_int,
    pub threshold: libc::c_int,
    pub resize_factor: libc::c_float,
    pub load_factor: libc::c_float,
}
pub type entry_t = entry_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entry_st {
    pub key: *mut libc::c_void,
    pub value: *mut libc::c_void,
    pub next: *mut entry_t,
    pub hash: libc::c_uint,
}
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_hash_fn = unsafe extern "C" fn(
    *const libc::c_void,
) -> libc::c_uint;
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_hashmap_iterator_st {
    pub h: *mut wget_hashmap_st,
    pub entry: *mut entry_t,
    pub pos: libc::c_int,
}
pub type wget_hashmap_iterator = wget_hashmap_iterator_st;
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
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_iterator_alloc(
    mut h: *mut wget_hashmap,
) -> *mut wget_hashmap_iterator {
    let mut iter: *mut wget_hashmap_iterator_st = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<wget_hashmap_iterator_st>() as libc::c_ulong,
    ) as *mut wget_hashmap_iterator_st;
    if !iter.is_null() {
        (*iter).h = h;
    }
    return iter as *mut wget_hashmap_iterator;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_iterator_free(
    mut iter: *mut *mut wget_hashmap_iterator,
) {
    if !iter.is_null() {
        if !(*iter).is_null() {
            wget_free.expect("non-null function pointer")(*iter as *mut libc::c_void);
            *iter = 0 as *mut wget_hashmap_iterator;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_iterator_next(
    mut iter: *mut wget_hashmap_iterator,
    mut value: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut _iter: *mut wget_hashmap_iterator_st = iter as *mut wget_hashmap_iterator_st;
    let mut h: *mut wget_hashmap_st = (*_iter).h;
    if !((*_iter).entry).is_null() {
        (*_iter).entry = (*(*_iter).entry).next;
        if !((*_iter).entry).is_null() {
            current_block = 16188577710263447426;
        } else {
            (*_iter).pos += 1;
            (*_iter).pos;
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            if ((*_iter).entry).is_null() && !h.is_null() {
                loop {
                    if !((*_iter).pos < (*h).max) {
                        current_block = 2968425633554183086;
                        break;
                    }
                    if !(*((*h).entry).offset((*_iter).pos as isize)).is_null() {
                        (*_iter).entry = *((*h).entry).offset((*_iter).pos as isize);
                        current_block = 16188577710263447426;
                        break;
                    } else {
                        (*_iter).pos += 1;
                        (*_iter).pos;
                    }
                }
            } else {
                current_block = 2968425633554183086;
            }
            match current_block {
                16188577710263447426 => {}
                _ => return 0 as *mut libc::c_void,
            }
        }
        _ => {}
    }
    if !value.is_null() {
        *value = (*(*_iter).entry).value;
    }
    return (*(*_iter).entry).key;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_create(
    mut max: libc::c_int,
    mut hash: Option::<wget_hashmap_hash_fn>,
    mut cmp: Option::<wget_hashmap_compare_fn>,
) -> *mut wget_hashmap {
    let mut h: *mut wget_hashmap = wget_malloc(
        ::core::mem::size_of::<wget_hashmap>() as libc::c_ulong,
    ) as *mut wget_hashmap;
    if h.is_null() {
        return 0 as *mut wget_hashmap;
    }
    (*h)
        .entry = wget_calloc(
        max as size_t,
        ::core::mem::size_of::<*mut entry_t>() as libc::c_ulong,
    ) as *mut *mut entry_t;
    if ((*h).entry).is_null() {
        if !h.is_null() {
            wget_free.expect("non-null function pointer")(h as *mut libc::c_void);
            h = 0 as *mut wget_hashmap;
        }
        return 0 as *mut wget_hashmap;
    }
    (*h).max = max;
    (*h).cur = 0 as libc::c_int;
    (*h).resize_factor = 2 as libc::c_int as libc::c_float;
    (*h).hash = hash;
    (*h).cmp = cmp;
    (*h).key_destructor = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*h).value_destructor = Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*h).load_factor = 0.75f64 as libc::c_float;
    (*h).threshold = (max as libc::c_float * (*h).load_factor) as libc::c_int;
    return h;
}
unsafe extern "C" fn hashmap_find_entry(
    mut h: *const wget_hashmap,
    mut key: *const libc::c_char,
    mut hash: libc::c_uint,
) -> *mut entry_t {
    let mut e: *mut entry_t = *((*h).entry)
        .offset(hash.wrapping_rem((*h).max as libc::c_uint) as isize);
    while !e.is_null() {
        if hash == (*e).hash
            && (key == (*e).key as *const libc::c_char
                || ((*h).cmp)
                    .expect(
                        "non-null function pointer",
                    )(key as *const libc::c_void, (*e).key) == 0)
        {
            return e;
        }
        e = (*e).next;
    }
    return 0 as *mut entry_t;
}
unsafe extern "C" fn hashmap_rehash(
    mut h: *mut wget_hashmap,
    mut new_entry: *mut *mut entry_t,
    mut newmax: libc::c_int,
    mut recalc_hash: libc::c_int,
) {
    let mut entry: *mut entry_t = 0 as *mut entry_t;
    let mut next: *mut entry_t = 0 as *mut entry_t;
    let mut cur: libc::c_int = (*h).cur;
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < (*h).max && cur != 0 {
        entry = *((*h).entry).offset(it as isize);
        while !entry.is_null() {
            next = (*entry).next;
            if recalc_hash != 0 {
                (*entry)
                    .hash = ((*h).hash)
                    .expect("non-null function pointer")((*entry).key);
            }
            let mut pos: libc::c_int = ((*entry).hash)
                .wrapping_rem(newmax as libc::c_uint) as libc::c_int;
            (*entry).next = *new_entry.offset(pos as isize);
            let ref mut fresh0 = *new_entry.offset(pos as isize);
            *fresh0 = entry;
            cur -= 1;
            cur;
            entry = next;
        }
        it += 1;
        it;
    }
    if !((*h).entry).is_null() {
        wget_free.expect("non-null function pointer")((*h).entry as *mut libc::c_void);
        (*h).entry = 0 as *mut *mut entry_t;
    }
    (*h).entry = new_entry;
    (*h).max = newmax;
    (*h).threshold = (newmax as libc::c_float * (*h).load_factor) as libc::c_int;
}
unsafe extern "C" fn hashmap_new_entry(
    mut h: *mut wget_hashmap,
    mut hash: libc::c_uint,
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut entry: *mut entry_t = 0 as *mut entry_t;
    entry = wget_malloc(::core::mem::size_of::<entry_t>() as libc::c_ulong)
        as *mut entry_t;
    if entry.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    let mut pos: libc::c_int = hash.wrapping_rem((*h).max as libc::c_uint)
        as libc::c_int;
    (*entry).key = key as *mut libc::c_void;
    (*entry).value = value as *mut libc::c_void;
    (*entry).hash = hash;
    (*entry).next = *((*h).entry).offset(pos as isize);
    let ref mut fresh1 = *((*h).entry).offset(pos as isize);
    *fresh1 = entry;
    (*h).cur += 1;
    if (*h).cur >= (*h).threshold {
        let mut newsize: libc::c_int = ((*h).max as libc::c_float * (*h).resize_factor)
            as libc::c_int;
        if newsize > 0 as libc::c_int {
            let mut new_entry: *mut *mut entry_t = 0 as *mut *mut entry_t;
            new_entry = wget_calloc(
                newsize as size_t,
                ::core::mem::size_of::<*mut entry_t>() as libc::c_ulong,
            ) as *mut *mut entry_t;
            if new_entry.is_null() {
                (*h).cur -= 1;
                (*h).cur;
                if !(*((*h).entry).offset(pos as isize)).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )(*((*h).entry).offset(pos as isize) as *mut libc::c_void);
                    let ref mut fresh2 = *((*h).entry).offset(pos as isize);
                    *fresh2 = 0 as *mut entry_t;
                }
                return WGET_E_MEMORY as libc::c_int;
            }
            hashmap_rehash(h, new_entry, newsize, 0 as libc::c_int);
        }
    }
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_put(
    mut h: *mut wget_hashmap,
    mut key: *const libc::c_void,
    mut value: *const libc::c_void,
) -> libc::c_int {
    if !h.is_null() && !key.is_null() {
        let mut entry: *mut entry_t = 0 as *mut entry_t;
        let mut hash: libc::c_uint = ((*h).hash)
            .expect("non-null function pointer")(key);
        let mut rc: libc::c_int = 0;
        entry = hashmap_find_entry(h, key as *const libc::c_char, hash);
        if !entry.is_null() {
            if (*entry).key != key as *mut libc::c_void
                && (*entry).key != value as *mut libc::c_void
            {
                if ((*h).key_destructor).is_some() {
                    ((*h).key_destructor)
                        .expect("non-null function pointer")((*entry).key);
                }
                if (*entry).key == (*entry).value {
                    (*entry).value = 0 as *mut libc::c_void;
                }
            }
            if (*entry).value != value as *mut libc::c_void
                && (*entry).value != key as *mut libc::c_void
            {
                if ((*h).value_destructor).is_some() {
                    ((*h).value_destructor)
                        .expect("non-null function pointer")((*entry).value);
                }
            }
            (*entry).key = key as *mut libc::c_void;
            (*entry).value = value as *mut libc::c_void;
            return 1 as libc::c_int;
        }
        rc = hashmap_new_entry(
            h,
            hash,
            key as *const libc::c_char,
            value as *const libc::c_char,
        );
        if rc < 0 as libc::c_int {
            return rc;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_contains(
    mut h: *const wget_hashmap,
    mut key: *const libc::c_void,
) -> libc::c_int {
    return wget_hashmap_get(h, key, 0 as *mut libc::c_void as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_get(
    mut h: *const wget_hashmap,
    mut key: *const libc::c_void,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    if !h.is_null() && !key.is_null() {
        let mut entry: *mut entry_t = 0 as *mut entry_t;
        entry = hashmap_find_entry(
            h,
            key as *const libc::c_char,
            ((*h).hash).expect("non-null function pointer")(key),
        );
        if !entry.is_null() {
            if !value.is_null() {
                *value = (*entry).value;
            }
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn hashmap_remove_entry(
    mut h: *mut wget_hashmap,
    mut key: *const libc::c_char,
    mut free_kv: libc::c_int,
) -> libc::c_int {
    let mut entry: *mut entry_t = 0 as *mut entry_t;
    let mut next: *mut entry_t = 0 as *mut entry_t;
    let mut prev: *mut entry_t = 0 as *mut entry_t;
    let mut hash: libc::c_uint = ((*h).hash)
        .expect("non-null function pointer")(key as *const libc::c_void);
    let mut pos: libc::c_int = hash.wrapping_rem((*h).max as libc::c_uint)
        as libc::c_int;
    entry = *((*h).entry).offset(pos as isize);
    while !entry.is_null() {
        next = (*entry).next;
        if hash == (*entry).hash
            && (key == (*entry).key as *const libc::c_char
                || ((*h).cmp)
                    .expect(
                        "non-null function pointer",
                    )(key as *const libc::c_void, (*entry).key) == 0)
        {
            if !prev.is_null() {
                (*prev).next = next;
            } else {
                let ref mut fresh3 = *((*h).entry).offset(pos as isize);
                *fresh3 = next;
            }
            if free_kv != 0 {
                if ((*h).key_destructor).is_some() {
                    ((*h).key_destructor)
                        .expect("non-null function pointer")((*entry).key);
                }
                if (*entry).value != (*entry).key {
                    if ((*h).value_destructor).is_some() {
                        ((*h).value_destructor)
                            .expect("non-null function pointer")((*entry).value);
                    }
                }
                (*entry).key = 0 as *mut libc::c_void;
                (*entry).value = 0 as *mut libc::c_void;
            }
            if !entry.is_null() {
                wget_free
                    .expect("non-null function pointer")(entry as *mut libc::c_void);
                entry = 0 as *mut entry_t;
            }
            (*h).cur -= 1;
            (*h).cur;
            return 1 as libc::c_int;
        }
        prev = entry;
        entry = next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_remove(
    mut h: *mut wget_hashmap,
    mut key: *const libc::c_void,
) -> libc::c_int {
    if !h.is_null() && !key.is_null() {
        return hashmap_remove_entry(h, key as *const libc::c_char, 1 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_remove_nofree(
    mut h: *mut wget_hashmap,
    mut key: *const libc::c_void,
) -> libc::c_int {
    if !h.is_null() && !key.is_null() {
        return hashmap_remove_entry(h, key as *const libc::c_char, 0 as libc::c_int)
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_free(mut h: *mut *mut wget_hashmap) {
    if !h.is_null() && !(*h).is_null() {
        wget_hashmap_clear(*h);
        if !((**h).entry).is_null() {
            wget_free
                .expect("non-null function pointer")((**h).entry as *mut libc::c_void);
            (**h).entry = 0 as *mut *mut entry_t;
        }
        if !(*h).is_null() {
            wget_free.expect("non-null function pointer")(*h as *mut libc::c_void);
            *h = 0 as *mut wget_hashmap;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_clear(mut h: *mut wget_hashmap) {
    if !h.is_null() {
        let mut entry: *mut entry_t = 0 as *mut entry_t;
        let mut next: *mut entry_t = 0 as *mut entry_t;
        let mut it: libc::c_int = 0;
        let mut cur: libc::c_int = (*h).cur;
        it = 0 as libc::c_int;
        while it < (*h).max && cur != 0 {
            entry = *((*h).entry).offset(it as isize);
            while !entry.is_null() {
                next = (*entry).next;
                if ((*h).key_destructor).is_some() {
                    ((*h).key_destructor)
                        .expect("non-null function pointer")((*entry).key);
                }
                if ((*h).value_destructor).is_some() {
                    if (*entry).value != (*entry).key
                        || (*entry).value == (*entry).key
                            && ((*h).key_destructor).is_none()
                    {
                        ((*h).value_destructor)
                            .expect("non-null function pointer")((*entry).value);
                    }
                }
                (*entry).key = 0 as *mut libc::c_void;
                (*entry).value = 0 as *mut libc::c_void;
                if !entry.is_null() {
                    wget_free
                        .expect("non-null function pointer")(entry as *mut libc::c_void);
                    entry = 0 as *mut entry_t;
                }
                cur -= 1;
                cur;
                entry = next;
            }
            let ref mut fresh4 = *((*h).entry).offset(it as isize);
            *fresh4 = 0 as *mut entry_t;
            it += 1;
            it;
        }
        (*h).cur = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_size(mut h: *const wget_hashmap) -> libc::c_int {
    return if !h.is_null() { (*h).cur } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_browse(
    mut h: *const wget_hashmap,
    mut browse: Option::<wget_hashmap_browse_fn>,
    mut ctx: *mut libc::c_void,
) -> libc::c_int {
    if !h.is_null() && browse.is_some() {
        let mut entry: *mut entry_t = 0 as *mut entry_t;
        let mut it: libc::c_int = 0;
        let mut ret: libc::c_int = 0;
        let mut cur: libc::c_int = (*h).cur;
        it = 0 as libc::c_int;
        while it < (*h).max && cur != 0 {
            entry = *((*h).entry).offset(it as isize);
            while !entry.is_null() {
                ret = browse
                    .expect(
                        "non-null function pointer",
                    )(ctx, (*entry).key, (*entry).value);
                if ret != 0 as libc::c_int {
                    return ret;
                }
                cur -= 1;
                cur;
                entry = (*entry).next;
            }
            it += 1;
            it;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_setcmpfunc(
    mut h: *mut wget_hashmap,
    mut cmp: Option::<wget_hashmap_compare_fn>,
) {
    if !h.is_null() {
        (*h).cmp = cmp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_sethashfunc(
    mut h: *mut wget_hashmap,
    mut hash: Option::<wget_hashmap_hash_fn>,
) -> libc::c_int {
    if h.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    if (*h).cur == 0 {
        return WGET_E_SUCCESS as libc::c_int;
    }
    let mut new_entry: *mut *mut entry_t = wget_calloc(
        (*h).max as size_t,
        ::core::mem::size_of::<*mut entry_t>() as libc::c_ulong,
    ) as *mut *mut entry_t;
    if new_entry.is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    (*h).hash = hash;
    hashmap_rehash(h, new_entry, (*h).max, 1 as libc::c_int);
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_set_key_destructor(
    mut h: *mut wget_hashmap,
    mut destructor: Option::<wget_hashmap_key_destructor>,
) {
    if !h.is_null() {
        (*h).key_destructor = destructor;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_set_value_destructor(
    mut h: *mut wget_hashmap,
    mut destructor: Option::<wget_hashmap_value_destructor>,
) {
    if !h.is_null() {
        (*h).value_destructor = destructor;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_set_load_factor(
    mut h: *mut wget_hashmap,
    mut factor: libc::c_float,
) {
    if !h.is_null() {
        (*h).load_factor = factor;
        (*h).threshold = ((*h).max as libc::c_float * (*h).load_factor) as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_hashmap_set_resize_factor(
    mut h: *mut wget_hashmap,
    mut factor: libc::c_float,
) {
    if !h.is_null() {
        (*h).resize_factor = factor;
    }
}
