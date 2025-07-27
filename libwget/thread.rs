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
    fn __errno_location() -> *mut libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn gettime(_: *mut timespec);
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type gl_thread_t = pthread_t;
pub type gl_lock_t = pthread_mutex_t;
pub type gl_cond_t = pthread_cond_t;
pub type C2RustUnnamed_0 = libc::c_int;
pub const WGET_E_UNSUPPORTED: C2RustUnnamed_0 = -12;
pub const WGET_E_IO: C2RustUnnamed_0 = -11;
pub const WGET_E_OPEN: C2RustUnnamed_0 = -10;
pub const WGET_E_XML_PARSE_ERR: C2RustUnnamed_0 = -9;
pub const WGET_E_TLS_DISABLED: C2RustUnnamed_0 = -8;
pub const WGET_E_CERTIFICATE: C2RustUnnamed_0 = -7;
pub const WGET_E_HANDSHAKE: C2RustUnnamed_0 = -6;
pub const WGET_E_CONNECT: C2RustUnnamed_0 = -5;
pub const WGET_E_TIMEOUT: C2RustUnnamed_0 = -4;
pub const WGET_E_INVALID: C2RustUnnamed_0 = -3;
pub const WGET_E_MEMORY: C2RustUnnamed_0 = -2;
pub const WGET_E_UNKNOWN: C2RustUnnamed_0 = -1;
pub const WGET_E_SUCCESS: C2RustUnnamed_0 = 0;
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_id = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_thread_st {
    pub tid: gl_thread_t,
}
pub type wget_thread = *mut wget_thread_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_thread_mutex_st {
    pub mutex: gl_lock_t,
}
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_thread_cond_st {
    pub cond: gl_cond_t,
}
pub type wget_thread_cond = *mut wget_thread_cond_st;
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_mutex_init(
    mut mutex: *mut wget_thread_mutex,
) -> libc::c_int {
    *mutex = wget_malloc(::core::mem::size_of::<wget_thread_mutex_st>() as libc::c_ulong)
        as wget_thread_mutex;
    if (*mutex).is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    return if 1 as libc::c_int != 0 {
        pthread_mutex_init(&mut (**mutex).mutex, 0 as *const pthread_mutexattr_t)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_mutex_destroy(
    mut mutex: *mut wget_thread_mutex,
) -> libc::c_int {
    let mut rc: libc::c_int = if 1 as libc::c_int != 0 {
        pthread_mutex_destroy(&mut (**mutex).mutex)
    } else {
        0 as libc::c_int
    };
    if !(*mutex).is_null() {
        wget_free.expect("non-null function pointer")(*mutex as *mut libc::c_void);
        *mutex = 0 as wget_thread_mutex;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_mutex_lock(mut mutex: wget_thread_mutex) {
    if 1 as libc::c_int != 0 {
        pthread_mutex_lock(&mut (*mutex).mutex);
    } else {};
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_mutex_unlock(mut mutex: wget_thread_mutex) {
    if 1 as libc::c_int != 0 {
        pthread_mutex_unlock(&mut (*mutex).mutex);
    } else {};
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_cond_init(
    mut cond: *mut wget_thread_cond,
) -> libc::c_int {
    *cond = wget_malloc(::core::mem::size_of::<wget_thread_cond_st>() as libc::c_ulong)
        as wget_thread_cond;
    if (*cond).is_null() {
        return WGET_E_MEMORY as libc::c_int;
    }
    return if 1 as libc::c_int != 0 {
        pthread_cond_init(&mut (**cond).cond, 0 as *const pthread_condattr_t)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_cond_destroy(
    mut cond: *mut wget_thread_cond,
) -> libc::c_int {
    let mut rc: libc::c_int = if 1 as libc::c_int != 0 {
        pthread_cond_destroy(&mut (**cond).cond)
    } else {
        0 as libc::c_int
    };
    if !(*cond).is_null() {
        wget_free.expect("non-null function pointer")(*cond as *mut libc::c_void);
        *cond = 0 as wget_thread_cond;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_cond_signal(
    mut cond: wget_thread_cond,
) -> libc::c_int {
    return if 1 as libc::c_int != 0 {
        pthread_cond_broadcast(&mut (*cond).cond)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_cond_wait(
    mut cond: wget_thread_cond,
    mut mutex: wget_thread_mutex,
    mut ms: libc::c_longlong,
) -> libc::c_int {
    if ms <= 0 as libc::c_int as libc::c_longlong {
        return if 1 as libc::c_int != 0 {
            pthread_cond_wait(&mut (*cond).cond, &mut (*mutex).mutex)
        } else {
            0 as libc::c_int
        };
    }
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut ts);
    ms
        += ts.tv_sec as libc::c_longlong * 1000 as libc::c_longlong
            + (ts.tv_nsec / 1000000 as libc::c_int as __syscall_slong_t)
                as libc::c_longlong;
    ts.tv_sec = (ms / 1000 as libc::c_int as libc::c_longlong) as __time_t;
    ts
        .tv_nsec = (ms % 1000 as libc::c_int as libc::c_longlong
        * 1000000 as libc::c_int as libc::c_longlong) as __syscall_slong_t;
    return if 1 as libc::c_int != 0 {
        pthread_cond_timedwait(&mut (*cond).cond, &mut (*mutex).mutex, &mut ts)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_start(
    mut thread: *mut wget_thread,
    mut start_routine: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    mut arg: *mut libc::c_void,
    mut flags: libc::c_int,
) -> libc::c_int {
    if wget_thread_support() {
        *thread = wget_malloc(::core::mem::size_of::<wget_thread_st>() as libc::c_ulong)
            as wget_thread;
        if (*thread).is_null() {
            return WGET_E_MEMORY as libc::c_int;
        }
        return if 1 as libc::c_int != 0 {
            pthread_create(
                &mut (**thread).tid,
                0 as *const pthread_attr_t,
                start_routine,
                arg,
            )
        } else {
            38 as libc::c_int
        };
    }
    *thread = 0 as wget_thread;
    start_routine.expect("non-null function pointer")(arg);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_cancel(mut thread: wget_thread) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_kill(
    mut thread: wget_thread,
    mut sig: libc::c_int,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_join(mut thread: *mut wget_thread) -> libc::c_int {
    if !thread.is_null() && !(*thread).is_null() && (**thread).tid != 0 {
        let mut rc: libc::c_int = if 1 as libc::c_int != 0 {
            pthread_join((**thread).tid, 0 as *mut *mut libc::c_void)
        } else {
            0 as libc::c_int
        };
        if !(*thread).is_null() {
            wget_free.expect("non-null function pointer")(*thread as *mut libc::c_void);
            *thread = 0 as wget_thread;
        }
        return rc;
    }
    if wget_thread_support() {
        *__errno_location() = 3 as libc::c_int;
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_self() -> wget_thread_id {
    return if 1 as libc::c_int != 0 {
        pthread_self()
    } else {
        0 as libc::c_int as pthread_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_thread_support() -> bool {
    return 1 as libc::c_int != 0;
}
