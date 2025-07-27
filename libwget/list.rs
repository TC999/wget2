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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_list_st {
    pub next: *mut wget_list,
    pub prev: *mut wget_list,
}
pub type wget_list = wget_list_st;
pub type wget_list_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_append(
    mut list: *mut *mut wget_list,
    mut data: *const libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut node: *mut wget_list = wget_malloc(
        (::core::mem::size_of::<wget_list>() as libc::c_ulong).wrapping_add(size),
    ) as *mut wget_list;
    if node.is_null() {
        return 0 as *mut libc::c_void;
    }
    memcpy(node.offset(1 as libc::c_int as isize) as *mut libc::c_void, data, size);
    if (*list).is_null() {
        *list = node;
        (*node).prev = node;
        (*node).next = (*node).prev;
    } else {
        (*node).next = *list;
        (*node).prev = (**list).prev;
        (*(**list).prev).next = node;
        (**list).prev = node;
    }
    return node.offset(1 as libc::c_int as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_prepend(
    mut list: *mut *mut wget_list,
    mut data: *const libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    if (*list).is_null() {
        return wget_list_append(list, data, size)
    } else {
        return wget_list_append(&mut (**list).prev, data, size)
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_remove(
    mut list: *mut *mut wget_list,
    mut elem: *mut libc::c_void,
) {
    if (*list).is_null() {
        return;
    }
    let mut node: *mut wget_list = (elem as *mut wget_list)
        .offset(-(1 as libc::c_int as isize));
    if node == (*node).prev {
        *list = 0 as *mut wget_list;
    } else {
        (*(*node).prev).next = (*node).next;
        (*(*node).next).prev = (*node).prev;
        if node == *list {
            *list = (*node).next;
        }
    }
    if !node.is_null() {
        wget_free.expect("non-null function pointer")(node as *mut libc::c_void);
        node = 0 as *mut wget_list;
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_getfirst(
    mut list: *const wget_list,
) -> *mut libc::c_void {
    return (if !list.is_null() { list.offset(1 as libc::c_int as isize) } else { list })
        as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_getlast(
    mut list: *const wget_list,
) -> *mut libc::c_void {
    return (if !list.is_null() {
        ((*list).prev).offset(1 as libc::c_int as isize) as *const wget_list
    } else {
        list
    }) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_getnext(
    mut elem: *const libc::c_void,
) -> *mut libc::c_void {
    if !elem.is_null() {
        let mut node: *mut wget_list = (elem as *mut wget_list)
            .offset(-(1 as libc::c_int as isize));
        return ((*node).next).offset(1 as libc::c_int as isize) as *mut libc::c_void;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_browse(
    mut list: *const wget_list,
    mut browse: Option::<wget_list_browse_fn>,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    if list.is_null() {
        return -(1 as libc::c_int);
    }
    let mut ret: libc::c_int = 0;
    let mut end: *const wget_list = (*list).prev;
    let mut cur: *const wget_list = list;
    loop {
        ret = browse
            .expect(
                "non-null function pointer",
            )(context, cur.offset(1 as libc::c_int as isize) as *mut libc::c_void);
        if !(ret == 0 as libc::c_int && cur != end) {
            break;
        }
        cur = (*cur).next;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn wget_list_free(mut list: *mut *mut wget_list) {
    while !(*list).is_null() {
        wget_list_remove(
            list,
            (*list).offset(1 as libc::c_int as isize) as *mut libc::c_void,
        );
    }
}
