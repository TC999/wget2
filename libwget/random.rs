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
    pub type wget_thread_mutex_st;
    fn random_r(__buf: *mut random_data, __result: *mut int32_t) -> libc::c_int;
    fn initstate_r(
        __seed: libc::c_uint,
        __statebuf: *mut libc::c_char,
        __statelen: size_t,
        __buf: *mut random_data,
    ) -> libc::c_int;
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn wget_thread_mutex_init(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex_0: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex_0: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex_0: wget_thread_mutex);
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct random_data {
    pub fptr: *mut int32_t,
    pub rptr: *mut int32_t,
    pub state: *mut int32_t,
    pub rand_type: libc::c_int,
    pub rand_deg: libc::c_int,
    pub rand_sep: libc::c_int,
    pub end_ptr: *mut int32_t,
}
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
static mut seeded: libc::c_int = 0;
static mut statebuf: [libc::c_char; 64] = [0; 64];
static mut state: random_data = random_data {
    fptr: 0 as *const int32_t as *mut int32_t,
    rptr: 0 as *const int32_t as *mut int32_t,
    state: 0 as *const int32_t as *mut int32_t,
    rand_type: 0,
    rand_deg: 0,
    rand_sep: 0,
    end_ptr: 0 as *const int32_t as *mut int32_t,
};
static mut mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut initialized: bool = false;
unsafe extern "C" fn random_exit() {
    if initialized {
        wget_thread_mutex_destroy(&mut mutex);
        initialized = 0 as libc::c_int != 0;
    }
}
unsafe extern "C" fn random_init() {
    if !initialized {
        wget_thread_mutex_init(&mut mutex);
        initialized = 1 as libc::c_int != 0;
        atexit(Some(random_exit as unsafe extern "C" fn() -> ()));
    }
}
#[no_mangle]
pub unsafe extern "C" fn wget_random_init() {
    random_init();
}
#[no_mangle]
pub unsafe extern "C" fn wget_random_exit() {
    random_exit();
}
#[no_mangle]
pub unsafe extern "C" fn wget_random() -> libc::c_int {
    let mut r: int32_t = 0;
    wget_thread_mutex_lock(mutex);
    if seeded == 0 {
        initstate_r(
            (time(0 as *mut time_t) ^ getpid() as time_t) as libc::c_uint,
            statebuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            &mut state,
        );
        seeded = 1 as libc::c_int;
    }
    if random_r(&mut state, &mut r) != 0 {
        r = 0 as libc::c_int;
    }
    wget_thread_mutex_unlock(mutex);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn wget_srandom(mut seed: libc::c_uint) {
    wget_thread_mutex_lock(mutex);
    initstate_r(
        seed,
        statebuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        &mut state,
    );
    seeded = 1 as libc::c_int;
    wget_thread_mutex_unlock(mutex);
}
