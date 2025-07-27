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
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
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
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type wget_iri_scheme = libc::c_uint;
pub const WGET_IRI_SCHEME_HTTPS: wget_iri_scheme = 1;
pub const WGET_IRI_SCHEME_HTTP: wget_iri_scheme = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_iri_st {
    pub uri: *const libc::c_char,
    pub safe_uri: *const libc::c_char,
    pub display: *const libc::c_char,
    pub userinfo: *const libc::c_char,
    pub password: *const libc::c_char,
    pub host: *const libc::c_char,
    pub path: *const libc::c_char,
    pub query: *const libc::c_char,
    pub fragment: *const libc::c_char,
    pub connection_part: *const libc::c_char,
    pub dirlen: size_t,
    pub msize: size_t,
    pub port: uint16_t,
    pub scheme: wget_iri_scheme,
    #[bitfield(name = "port_given", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "host_allocated", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "path_allocated", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "query_allocated", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "fragment_allocated", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "is_ip_address", ty = "bool", bits = "5..=5")]
    pub port_given_host_allocated_path_allocated_query_allocated_fragment_allocated_is_ip_address: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type wget_iri = wget_iri_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_plugin_vtable {
    pub get_name: Option::<
        unsafe extern "C" fn(*mut wget_plugin) -> *const libc::c_char,
    >,
    pub register_finalizer: Option::<
        unsafe extern "C" fn(*mut wget_plugin, Option::<wget_plugin_finalizer_fn>) -> (),
    >,
    pub register_argp: Option::<
        unsafe extern "C" fn(
            *mut wget_plugin,
            Option::<wget_plugin_option_callback>,
        ) -> (),
    >,
    pub action_reject: Option::<unsafe extern "C" fn(*mut wget_intercept_action) -> ()>,
    pub action_accept: Option::<unsafe extern "C" fn(*mut wget_intercept_action) -> ()>,
    pub action_set_alt_url: Option::<
        unsafe extern "C" fn(*mut wget_intercept_action, *const wget_iri) -> (),
    >,
    pub action_set_local_filename: Option::<
        unsafe extern "C" fn(*mut wget_intercept_action, *const libc::c_char) -> (),
    >,
    pub register_url_filter: Option::<
        unsafe extern "C" fn(
            *mut wget_plugin,
            Option::<wget_plugin_url_filter_callback>,
        ) -> (),
    >,
    pub file_get_source_url: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> *const wget_iri,
    >,
    pub file_get_local_filename: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> *const libc::c_char,
    >,
    pub file_get_size: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> uint64_t,
    >,
    pub file_get_contents: Option::<
        unsafe extern "C" fn(
            *mut wget_downloaded_file,
            *mut *const libc::c_void,
            *mut size_t,
        ) -> libc::c_int,
    >,
    pub file_open_stream: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> *mut FILE,
    >,
    pub file_get_recurse: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file) -> bool,
    >,
    pub file_add_recurse_url: Option::<
        unsafe extern "C" fn(*mut wget_downloaded_file, *const wget_iri) -> (),
    >,
    pub register_post_processor: Option::<
        unsafe extern "C" fn(
            *mut wget_plugin,
            Option::<wget_plugin_post_processor>,
        ) -> (),
    >,
}
pub type wget_plugin_post_processor = unsafe extern "C" fn(
    *mut wget_plugin,
    *mut wget_downloaded_file,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_downloaded_file {
    pub vtable: *mut wget_plugin_vtable,
}
pub type wget_plugin = wget_plugin_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_plugin_st {
    pub plugin_data: *mut libc::c_void,
    pub vtable: *mut wget_plugin_vtable,
}
pub type wget_plugin_url_filter_callback = unsafe extern "C" fn(
    *mut wget_plugin,
    *const wget_iri,
    *mut wget_intercept_action,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_intercept_action {
    pub vtable: *mut wget_plugin_vtable,
}
pub type wget_plugin_option_callback = unsafe extern "C" fn(
    *mut wget_plugin,
    *const libc::c_char,
    *const libc::c_char,
) -> libc::c_int;
pub type wget_plugin_finalizer_fn = unsafe extern "C" fn(
    *mut wget_plugin,
    libc::c_int,
) -> ();
#[no_mangle]
pub unsafe extern "C" fn wget_plugin_get_name(
    mut plugin: *mut wget_plugin,
) -> *const libc::c_char {
    return ((*(*plugin).vtable).get_name).expect("non-null function pointer")(plugin);
}
#[no_mangle]
pub unsafe extern "C" fn wget_plugin_register_finalizer(
    mut plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_finalizer_fn>,
) {
    ((*(*plugin).vtable).register_finalizer)
        .expect("non-null function pointer")(plugin, fn_0);
}
#[no_mangle]
pub unsafe extern "C" fn wget_plugin_register_option_callback(
    mut plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_option_callback>,
) {
    ((*(*plugin).vtable).register_argp)
        .expect("non-null function pointer")(plugin, fn_0);
}
#[no_mangle]
pub unsafe extern "C" fn wget_intercept_action_reject(
    mut action: *mut wget_intercept_action,
) {
    ((*(*action).vtable).action_reject).expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn wget_intercept_action_accept(
    mut action: *mut wget_intercept_action,
) {
    ((*(*action).vtable).action_accept).expect("non-null function pointer")(action);
}
#[no_mangle]
pub unsafe extern "C" fn wget_intercept_action_set_alt_url(
    mut action: *mut wget_intercept_action,
    mut iri: *const wget_iri,
) {
    ((*(*action).vtable).action_set_alt_url)
        .expect("non-null function pointer")(action, iri);
}
#[no_mangle]
pub unsafe extern "C" fn wget_intercept_action_set_local_filename(
    mut action: *mut wget_intercept_action,
    mut local_filename: *const libc::c_char,
) {
    ((*(*action).vtable).action_set_local_filename)
        .expect("non-null function pointer")(action, local_filename);
}
#[no_mangle]
pub unsafe extern "C" fn wget_plugin_register_url_filter_callback(
    mut plugin: *mut wget_plugin,
    mut filter_fn: Option::<wget_plugin_url_filter_callback>,
) {
    ((*(*plugin).vtable).register_url_filter)
        .expect("non-null function pointer")(plugin, filter_fn);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_get_source_url(
    mut file: *mut wget_downloaded_file,
) -> *const wget_iri {
    return ((*(*file).vtable).file_get_source_url)
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_get_local_filename(
    mut file: *mut wget_downloaded_file,
) -> *const libc::c_char {
    return ((*(*file).vtable).file_get_local_filename)
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_get_size(
    mut file: *mut wget_downloaded_file,
) -> uint64_t {
    return ((*(*file).vtable).file_get_size).expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_get_contents(
    mut file: *mut wget_downloaded_file,
    mut data: *mut *const libc::c_void,
    mut size: *mut size_t,
) -> libc::c_int {
    return ((*(*file).vtable).file_get_contents)
        .expect("non-null function pointer")(file, data, size);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_open_stream(
    mut file: *mut wget_downloaded_file,
) -> *mut FILE {
    return ((*(*file).vtable).file_open_stream)
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_get_recurse(
    mut file: *mut wget_downloaded_file,
) -> bool {
    return ((*(*file).vtable).file_get_recurse)
        .expect("non-null function pointer")(file);
}
#[no_mangle]
pub unsafe extern "C" fn wget_downloaded_file_add_recurse_url(
    mut file: *mut wget_downloaded_file,
    mut iri: *const wget_iri,
) {
    ((*(*file).vtable).file_add_recurse_url)
        .expect("non-null function pointer")(file, iri);
}
#[no_mangle]
pub unsafe extern "C" fn wget_plugin_register_post_processor(
    mut plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_post_processor>,
) {
    ((*(*plugin).vtable).register_post_processor)
        .expect("non-null function pointer")(plugin, fn_0);
}
