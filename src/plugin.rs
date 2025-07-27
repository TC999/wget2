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
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type dl_file_st;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fmemopen(
        __s: *mut libc::c_void,
        __len: size_t,
        __modes: *const libc::c_char,
    ) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn wget_read_file(
        fname: *const libc::c_char,
        size: *mut size_t,
    ) -> *mut libc::c_char;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_clear(v: *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_hashmap_put(
        h: *mut wget_hashmap,
        key: *const libc::c_void,
        value: *const libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_free(h: *mut *mut wget_hashmap);
    fn wget_hashmap_get(
        h: *const wget_hashmap,
        key: *const libc::c_void,
        value: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_hashmap_set_key_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_key_destructor>,
    );
    fn wget_hashmap_set_value_destructor(
        h: *mut wget_hashmap,
        destructor: Option::<wget_hashmap_value_destructor>,
    );
    fn wget_stringmap_create(max: libc::c_int) -> *mut wget_stringmap;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_clone(iri: *const wget_iri) -> *mut wget_iri;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn dl_error_set(e: *mut dl_error_t, msg: *const libc::c_char);
    fn dl_error_set_printf(e: *mut dl_error_t, format: *const libc::c_char, _: ...);
    fn dl_file_open(filename: *const libc::c_char, e: *mut dl_error_t) -> *mut dl_file_t;
    fn dl_file_lookup(
        dm: *mut dl_file_t,
        symbol: *const libc::c_char,
        e: *mut dl_error_t,
    ) -> *mut libc::c_void;
    fn dl_file_close(dm: *mut dl_file_t);
    fn dl_get_name_from_path(
        path: *const libc::c_char,
        strict: libc::c_int,
    ) -> *mut libc::c_char;
    fn dl_search(
        name: *const libc::c_char,
        dirs: *const wget_vector,
    ) -> *mut libc::c_char;
    fn dl_list(dirs: *const wget_vector, names_out: *mut wget_vector);
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
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
pub type wget_malloc_function = unsafe extern "C" fn(size_t) -> *mut libc::c_void;
pub type wget_free_function = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_stringmap = wget_hashmap;
pub type wget_stringmap_key_destructor = unsafe extern "C" fn(*mut libc::c_char) -> ();
pub type wget_stringmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
pub type wget_plugin_initializer_fn = unsafe extern "C" fn(
    *mut wget_plugin,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dl_error_t {
    pub msg: *const libc::c_char,
}
pub type dl_file_t = dl_file_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_t {
    pub parent: wget_plugin,
    pub name: *mut libc::c_char,
    pub dm: *mut dl_file_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plugin_priv_t {
    pub parent: plugin_t,
    pub finalizer: Option::<wget_plugin_finalizer_fn>,
    pub argp: Option::<wget_plugin_option_callback>,
    pub url_filter: Option::<wget_plugin_url_filter_callback>,
    pub post_processor: Option::<wget_plugin_post_processor>,
    pub name_buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct downloaded_file {
    pub parent: wget_downloaded_file,
    pub iri: *const wget_iri,
    pub filename: *const libc::c_char,
    pub size: uint64_t,
    pub data: *const libc::c_void,
    pub data_buf: *mut libc::c_void,
    pub recurse_iris: *mut wget_vector,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct plugin_db_forward_url_verdict {
    pub alt_iri: *mut wget_iri,
    pub alt_local_filename: *mut libc::c_char,
    #[bitfield(name = "reject", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "accept", ty = "bool", bits = "1..=1")]
    pub reject_accept: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intercept_action {
    pub parent: wget_intercept_action,
    pub verdict: plugin_db_forward_url_verdict,
}
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
#[inline]
unsafe extern "C" fn wget_stringmap_put(
    mut h: *mut wget_stringmap,
    mut key: *const libc::c_char,
    mut value: *const libc::c_void,
) -> libc::c_int {
    return wget_hashmap_put(h, key as *const libc::c_void, value);
}
#[inline]
unsafe extern "C" fn wget_stringmap_get(
    mut h: *const wget_stringmap,
    mut key: *const libc::c_char,
    mut value: *mut *mut libc::c_void,
) -> libc::c_int {
    return wget_hashmap_get(h, key as *const libc::c_void, value);
}
#[inline]
unsafe extern "C" fn wget_stringmap_free(mut h: *mut *mut wget_stringmap) {
    wget_hashmap_free(h);
}
#[inline]
unsafe extern "C" fn wget_stringmap_set_key_destructor(
    mut h: *mut wget_hashmap,
    mut destructor: Option::<wget_stringmap_key_destructor>,
) {
    wget_hashmap_set_key_destructor(
        h,
        ::core::mem::transmute::<
            Option::<wget_stringmap_key_destructor>,
            Option::<wget_hashmap_key_destructor>,
        >(destructor),
    );
}
#[inline]
unsafe extern "C" fn wget_stringmap_set_value_destructor(
    mut h: *mut wget_hashmap,
    mut destructor: Option::<wget_stringmap_value_destructor>,
) {
    wget_hashmap_set_value_destructor(
        h,
        ::core::mem::transmute::<
            Option::<wget_stringmap_value_destructor>,
            Option::<wget_hashmap_value_destructor>,
        >(destructor),
    );
}
#[inline]
unsafe extern "C" fn dl_error_init(mut e: *mut dl_error_t) {
    (*e).msg = 0 as *const libc::c_char;
}
#[inline]
unsafe extern "C" fn dl_error_get_msg(mut e: *mut dl_error_t) -> *const libc::c_char {
    return (*e).msg;
}
static mut plugin_list_envvar: *const libc::c_char = b"WGET2_PLUGINS\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn split_string(
    mut str: *const libc::c_char,
    mut separator: libc::c_char,
    mut v: *mut wget_vector,
) {
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pmark: *const libc::c_char = 0 as *const libc::c_char;
    ptr = str;
    pmark = ptr;
    while *ptr != 0 {
        ptr = strchrnul(pmark, separator as libc::c_int);
        if ptr > pmark {
            wget_vector_add(
                v,
                wget_strmemdup(
                    pmark as *const libc::c_void,
                    ptr.offset_from(pmark) as libc::c_long as size_t,
                ) as *const libc::c_void,
            );
        }
        pmark = ptr.offset(1 as libc::c_int as isize);
    }
}
static mut initialized: bool = 0 as libc::c_int != 0;
static mut search_paths: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut plugin_list: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut plugin_name_index: *mut wget_stringmap = 0 as *const wget_stringmap
    as *mut wget_stringmap;
static mut plugin_help_forwarded: bool = false;
#[no_mangle]
pub unsafe extern "C" fn plugin_db_add_search_paths(
    mut paths: *const libc::c_char,
    mut separator: libc::c_char,
) {
    split_string(paths, separator, search_paths);
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_clear_search_paths() {
    wget_vector_clear(search_paths);
}
unsafe extern "C" fn impl_register_finalizer(
    mut p_plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_finalizer_fn>,
) {
    let mut priv_0: *mut plugin_priv_t = p_plugin as *mut plugin_priv_t;
    (*priv_0).finalizer = fn_0;
}
unsafe extern "C" fn impl_get_name(
    mut p_plugin: *mut wget_plugin,
) -> *const libc::c_char {
    let mut plugin: *mut plugin_t = p_plugin as *mut plugin_t;
    return (*plugin).name;
}
unsafe extern "C" fn impl_register_argp(
    mut p_plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_option_callback>,
) {
    let mut priv_0: *mut plugin_priv_t = p_plugin as *mut plugin_priv_t;
    (*priv_0).argp = fn_0;
}
unsafe extern "C" fn impl_action_reject(mut p_action: *mut wget_intercept_action) {
    let mut action: *mut intercept_action = p_action as *mut intercept_action;
    ((*action).verdict).set_reject(1 as libc::c_int != 0);
}
unsafe extern "C" fn impl_action_accept(mut p_action: *mut wget_intercept_action) {
    let mut action: *mut intercept_action = p_action as *mut intercept_action;
    ((*action).verdict).set_accept(1 as libc::c_int != 0);
}
unsafe extern "C" fn impl_action_set_alt_url(
    mut p_action: *mut wget_intercept_action,
    mut iri: *const wget_iri,
) {
    let mut action: *mut intercept_action = p_action as *mut intercept_action;
    if !((*action).verdict.alt_iri).is_null() {
        wget_iri_free(&mut (*action).verdict.alt_iri);
    }
    (*action).verdict.alt_iri = wget_iri_clone(iri);
}
unsafe extern "C" fn impl_action_set_local_filename(
    mut p_action: *mut wget_intercept_action,
    mut local_filename: *const libc::c_char,
) {
    let mut action: *mut intercept_action = p_action as *mut intercept_action;
    if !((*action).verdict.alt_local_filename).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*action).verdict.alt_local_filename as *mut libc::c_void);
    }
    (*action).verdict.alt_local_filename = wget_strdup(local_filename);
}
unsafe extern "C" fn impl_register_url_filter(
    mut p_plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_url_filter_callback>,
) {
    let mut priv_0: *mut plugin_priv_t = p_plugin as *mut plugin_priv_t;
    (*priv_0).url_filter = fn_0;
}
unsafe extern "C" fn impl_file_get_source_url(
    mut p_file: *mut wget_downloaded_file,
) -> *const wget_iri {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    return (*file).iri;
}
unsafe extern "C" fn impl_file_get_local_filename(
    mut p_file: *mut wget_downloaded_file,
) -> *const libc::c_char {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    return (*file).filename;
}
unsafe extern "C" fn impl_file_get_size(
    mut p_file: *mut wget_downloaded_file,
) -> uint64_t {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    return (*file).size;
}
unsafe extern "C" fn impl_file_get_contents(
    mut p_file: *mut wget_downloaded_file,
    mut data: *mut *const libc::c_void,
    mut size: *mut size_t,
) -> libc::c_int {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    if ((*file).data).is_null() && !((*file).filename).is_null() {
        let mut dummy: size_t = 0;
        (*file)
            .data_buf = wget_read_file((*file).filename, &mut dummy)
            as *mut libc::c_void;
        if ((*file).data_buf).is_null() {
            return -(1 as libc::c_int);
        }
        (*file).data = (*file).data_buf;
    }
    *data = (*file).data;
    *size = (*file).size;
    return 0 as libc::c_int;
}
unsafe extern "C" fn impl_file_open_stream(
    mut p_file: *mut wget_downloaded_file,
) -> *mut FILE {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    if !((*file).data).is_null() {
        return fmemopen(
            (*file).data as *mut libc::c_void,
            (*file).size,
            b"rb\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*file).filename).is_null() {
        return rpl_fopen((*file).filename, b"rb\0" as *const u8 as *const libc::c_char);
    }
    return 0 as *mut FILE;
}
unsafe extern "C" fn impl_file_get_recurse(
    mut p_file: *mut wget_downloaded_file,
) -> bool {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    return if !((*file).recurse_iris).is_null() {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn impl_file_add_recurse_url(
    mut p_file: *mut wget_downloaded_file,
    mut iri: *const wget_iri,
) {
    let mut file: *mut downloaded_file = p_file as *mut downloaded_file;
    if !((*file).recurse_iris).is_null() {
        wget_vector_add(
            (*file).recurse_iris,
            wget_iri_clone(iri) as *const libc::c_void,
        );
    }
}
unsafe extern "C" fn impl_register_post_processor(
    mut p_plugin: *mut wget_plugin,
    mut fn_0: Option::<wget_plugin_post_processor>,
) {
    let mut priv_0: *mut plugin_priv_t = p_plugin as *mut plugin_priv_t;
    (*priv_0).post_processor = fn_0;
}
static mut vtable: wget_plugin_vtable = unsafe {
    {
        let mut init = wget_plugin_vtable {
            get_name: Some(
                impl_get_name
                    as unsafe extern "C" fn(*mut wget_plugin) -> *const libc::c_char,
            ),
            register_finalizer: Some(
                impl_register_finalizer
                    as unsafe extern "C" fn(
                        *mut wget_plugin,
                        Option::<wget_plugin_finalizer_fn>,
                    ) -> (),
            ),
            register_argp: Some(
                impl_register_argp
                    as unsafe extern "C" fn(
                        *mut wget_plugin,
                        Option::<wget_plugin_option_callback>,
                    ) -> (),
            ),
            action_reject: Some(
                impl_action_reject
                    as unsafe extern "C" fn(*mut wget_intercept_action) -> (),
            ),
            action_accept: Some(
                impl_action_accept
                    as unsafe extern "C" fn(*mut wget_intercept_action) -> (),
            ),
            action_set_alt_url: Some(
                impl_action_set_alt_url
                    as unsafe extern "C" fn(
                        *mut wget_intercept_action,
                        *const wget_iri,
                    ) -> (),
            ),
            action_set_local_filename: Some(
                impl_action_set_local_filename
                    as unsafe extern "C" fn(
                        *mut wget_intercept_action,
                        *const libc::c_char,
                    ) -> (),
            ),
            register_url_filter: Some(
                impl_register_url_filter
                    as unsafe extern "C" fn(
                        *mut wget_plugin,
                        Option::<wget_plugin_url_filter_callback>,
                    ) -> (),
            ),
            file_get_source_url: Some(
                impl_file_get_source_url
                    as unsafe extern "C" fn(*mut wget_downloaded_file) -> *const wget_iri,
            ),
            file_get_local_filename: Some(
                impl_file_get_local_filename
                    as unsafe extern "C" fn(
                        *mut wget_downloaded_file,
                    ) -> *const libc::c_char,
            ),
            file_get_size: Some(
                impl_file_get_size
                    as unsafe extern "C" fn(*mut wget_downloaded_file) -> uint64_t,
            ),
            file_get_contents: Some(
                impl_file_get_contents
                    as unsafe extern "C" fn(
                        *mut wget_downloaded_file,
                        *mut *const libc::c_void,
                        *mut size_t,
                    ) -> libc::c_int,
            ),
            file_open_stream: Some(
                impl_file_open_stream
                    as unsafe extern "C" fn(*mut wget_downloaded_file) -> *mut FILE,
            ),
            file_get_recurse: Some(
                impl_file_get_recurse
                    as unsafe extern "C" fn(*mut wget_downloaded_file) -> bool,
            ),
            file_add_recurse_url: Some(
                impl_file_add_recurse_url
                    as unsafe extern "C" fn(
                        *mut wget_downloaded_file,
                        *const wget_iri,
                    ) -> (),
            ),
            register_post_processor: Some(
                impl_register_post_processor
                    as unsafe extern "C" fn(
                        *mut wget_plugin,
                        Option::<wget_plugin_post_processor>,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn plugin_free(mut plugin: *mut plugin_t) {
    dl_file_close((*plugin).dm);
    wget_free.expect("non-null function pointer")(plugin as *mut libc::c_void);
}
unsafe extern "C" fn load_plugin(
    mut name: *const libc::c_char,
    mut path: *const libc::c_char,
    mut e: *mut dl_error_t,
) -> *mut plugin_t {
    let mut name_len: size_t = 0;
    let mut dm: *mut dl_file_t = 0 as *mut dl_file_t;
    let mut plugin: *mut plugin_t = 0 as *mut plugin_t;
    let mut priv_0: *mut plugin_priv_t = 0 as *mut plugin_priv_t;
    let mut init_fn: Option::<wget_plugin_initializer_fn> = None;
    name_len = strlen(name);
    dm = dl_file_open(path, e);
    if dm.is_null() {
        return 0 as *mut plugin_t;
    }
    plugin = wget_malloc(
        (::core::mem::size_of::<plugin_priv_t>() as libc::c_ulong)
            .wrapping_add(name_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut plugin_t;
    priv_0 = plugin as *mut plugin_priv_t;
    (*priv_0).finalizer = None;
    (*priv_0).argp = None;
    (*priv_0).url_filter = None;
    (*priv_0).post_processor = None;
    wget_strscpy(
        ((*priv_0).name_buf).as_mut_ptr(),
        name,
        name_len.wrapping_add(1 as libc::c_int as size_t),
    );
    (*plugin).parent.plugin_data = 0 as *mut libc::c_void;
    (*plugin).parent.vtable = &mut vtable;
    (*plugin).name = ((*priv_0).name_buf).as_mut_ptr();
    (*plugin).dm = dm;
    let ref mut fresh0 = *(&mut init_fn as *mut Option::<wget_plugin_initializer_fn>
        as *mut *mut libc::c_void);
    *fresh0 = dl_file_lookup(
        dm,
        b"wget_plugin_initializer\0" as *const u8 as *const libc::c_char,
        e,
    );
    if init_fn.is_none() {
        plugin_free(plugin);
        return 0 as *mut plugin_t;
    }
    if init_fn.expect("non-null function pointer")(plugin as *mut wget_plugin)
        != 0 as libc::c_int
    {
        dl_error_set(
            e,
            b"Plugin failed to initialize\0" as *const u8 as *const libc::c_char,
        );
        plugin_free(plugin);
        return 0 as *mut plugin_t;
    }
    wget_vector_add(plugin_list, plugin as *const libc::c_void);
    wget_stringmap_put(plugin_name_index, (*plugin).name, plugin as *const libc::c_void);
    return plugin;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_load_from_path(
    mut path: *const libc::c_char,
    mut e: *mut dl_error_t,
) -> *mut plugin_t {
    let mut name: *mut libc::c_char = dl_get_name_from_path(path, 0 as libc::c_int);
    let mut plugin: *mut plugin_t = load_plugin(name, path, e);
    wget_free.expect("non-null function pointer")(name as *mut libc::c_void);
    return plugin;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_load_from_name(
    mut name: *const libc::c_char,
    mut e: *mut dl_error_t,
) -> *mut plugin_t {
    let mut plugin: *mut plugin_t = 0 as *mut plugin_t;
    let mut filename: *mut libc::c_char = dl_search(name, search_paths);
    if filename.is_null() {
        dl_error_set_printf(
            e,
            b"Plugin '%s' not found in any of the plugin search paths.\0" as *const u8
                as *const libc::c_char,
            name,
        );
        return 0 as *mut plugin_t;
    }
    plugin = load_plugin(name, filename, e);
    wget_free.expect("non-null function pointer")(filename as *mut libc::c_void);
    return plugin;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_load_from_envvar() -> libc::c_int {
    let mut e: [dl_error_t; 1] = [dl_error_t {
        msg: 0 as *const libc::c_char,
    }; 1];
    let mut v: *mut wget_vector = 0 as *mut wget_vector;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    str = getenv(plugin_list_envvar);
    if !str.is_null() {
        let mut sep: libc::c_char = ':' as i32 as libc::c_char;
        dl_error_init(e.as_mut_ptr());
        v = wget_vector_create(16 as libc::c_int, None);
        split_string(str, sep, v);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < wget_vector_size(v) && ret == 0 as libc::c_int {
            let mut plugin: *mut plugin_t = 0 as *mut plugin_t;
            let mut local: libc::c_int = 0 as libc::c_int;
            str = wget_vector_get(v, i) as *const libc::c_char;
            if !(strchr(str, '/' as i32)).is_null() {
                local = 1 as libc::c_int;
            }
            if local != 0 {
                plugin = plugin_db_load_from_path(str, e.as_mut_ptr());
            } else {
                plugin = plugin_db_load_from_name(str, e.as_mut_ptr());
            }
            if plugin.is_null() {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Plugin '%s' failed to load: %s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    str,
                    dl_error_get_msg(e.as_mut_ptr()),
                );
                dl_error_set(e.as_mut_ptr(), 0 as *const libc::c_char);
                ret = -(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        wget_vector_free(&mut v);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_list(mut names_out: *mut wget_vector) {
    dl_list(search_paths, names_out);
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_forward_option(
    mut plugin_option: *const libc::c_char,
    mut e: *mut dl_error_t,
) -> libc::c_int {
    let mut plugin_option_copy: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plugin_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut plugin: *mut plugin_t = 0 as *mut plugin_t;
    let mut priv_0: *mut plugin_priv_t = 0 as *mut plugin_priv_t;
    let mut op_res: libc::c_int = 0;
    plugin_option_copy = wget_strdup(plugin_option);
    ptr = strchr(plugin_option_copy, '.' as i32);
    if ptr.is_null() {
        dl_error_set_printf(
            e,
            b"'%s': '.' is missing (separates plugin name and option)\0" as *const u8
                as *const libc::c_char,
            plugin_option,
        );
        wget_free
            .expect(
                "non-null function pointer",
            )(plugin_option_copy as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if ptr == plugin_option_copy {
        dl_error_set_printf(
            e,
            b"'%s': Plugin name is missing.\0" as *const u8 as *const libc::c_char,
            plugin_option,
        );
        wget_free
            .expect(
                "non-null function pointer",
            )(plugin_option_copy as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    *ptr = 0 as libc::c_int as libc::c_char;
    plugin_name = plugin_option_copy;
    option = ptr.offset(1 as libc::c_int as isize);
    ptr = strchr(option, '=' as i32);
    if !ptr.is_null() {
        *ptr = 0 as libc::c_int as libc::c_char;
        value = ptr.offset(1 as libc::c_int as isize);
    } else {
        value = 0 as *mut libc::c_char;
    }
    if *option as libc::c_int == 0 as libc::c_int {
        dl_error_set_printf(
            e,
            b"'%s': An option is required (after '.', and before '=' if present)\0"
                as *const u8 as *const libc::c_char,
            plugin_option,
        );
        wget_free
            .expect(
                "non-null function pointer",
            )(plugin_option_copy as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if strcmp(option, b"help\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
    {
        if !value.is_null() {
            dl_error_set_printf(
                e,
                b"'help' option does not accept arguments\n\0" as *const u8
                    as *const libc::c_char,
            );
            wget_free
                .expect(
                    "non-null function pointer",
                )(plugin_option_copy as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        plugin_help_forwarded = 1 as libc::c_int != 0;
    }
    if wget_stringmap_get(
        plugin_name_index,
        plugin_name,
        &mut plugin as *mut *mut plugin_t as *mut *mut libc::c_void,
    ) == 0
    {
        dl_error_set_printf(
            e,
            b"Plugin '%s' is not loaded.\0" as *const u8 as *const libc::c_char,
            plugin_name,
        );
        wget_free
            .expect(
                "non-null function pointer",
            )(plugin_option_copy as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    priv_0 = plugin as *mut plugin_priv_t;
    if ((*priv_0).argp).is_none() {
        dl_error_set_printf(
            e,
            b"Plugin '%s' does not accept options.\0" as *const u8
                as *const libc::c_char,
            (*plugin).name,
        );
        wget_free
            .expect(
                "non-null function pointer",
            )(plugin_option_copy as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    op_res = ((*priv_0).argp)
        .expect("non-null function pointer")(plugin as *mut wget_plugin, option, value);
    if op_res < 0 as libc::c_int {
        dl_error_set_printf(
            e,
            b"Plugin '%s' did not accept option '%s'\0" as *const u8
                as *const libc::c_char,
            (*plugin).name,
            strchrnul(plugin_option, '.' as i32),
        );
        wget_free
            .expect(
                "non-null function pointer",
            )(plugin_option_copy as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    wget_free
        .expect("non-null function pointer")(plugin_option_copy as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_show_help() {
    let mut n_plugins: libc::c_int = wget_vector_size(plugin_list);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_plugins {
        let mut plugin: *mut plugin_t = wget_vector_get(plugin_list, i) as *mut plugin_t;
        let mut priv_0: *mut plugin_priv_t = plugin as *mut plugin_priv_t;
        if ((*priv_0).argp).is_some() {
            printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Options for %s:\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*plugin).name,
            );
            ((*priv_0).argp)
                .expect(
                    "non-null function pointer",
                )(
                plugin as *mut wget_plugin,
                b"help\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    plugin_help_forwarded = 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_help_forwarded() -> bool {
    return plugin_help_forwarded;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_forward_url(
    mut iri: *const wget_iri,
    mut verdict: *mut plugin_db_forward_url_verdict,
) {
    let mut action: intercept_action = {
        let mut init = intercept_action {
            parent: {
                let mut init = wget_intercept_action {
                    vtable: &mut vtable,
                };
                init
            },
            verdict: plugin_db_forward_url_verdict {
                alt_iri: 0 as *mut wget_iri,
                alt_local_filename: 0 as *mut libc::c_char,
                reject_accept: [0; 1],
                c2rust_padding: [0; 7],
            },
        };
        init
    };
    let mut n_plugins: libc::c_int = wget_vector_size(plugin_list);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_plugins {
        let mut plugin: *mut plugin_t = wget_vector_get(plugin_list, i) as *mut plugin_t;
        let mut priv_0: *mut plugin_priv_t = plugin as *mut plugin_priv_t;
        if ((*priv_0).url_filter).is_some() {
            let mut cur_iri: *const wget_iri = action.verdict.alt_iri;
            if cur_iri.is_null() {
                cur_iri = iri;
            }
            ((*priv_0).url_filter)
                .expect(
                    "non-null function pointer",
                )(
                plugin as *mut wget_plugin,
                cur_iri,
                &mut action as *mut intercept_action as *mut wget_intercept_action,
            );
            if (action.verdict).reject() as libc::c_int != 0
                || (action.verdict).accept() as libc::c_int != 0
            {
                break;
            }
        }
        i += 1;
        i;
    }
    *verdict = action.verdict;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_forward_url_verdict_free(
    mut verdict: *mut plugin_db_forward_url_verdict,
) {
    if !((*verdict).alt_iri).is_null() {
        wget_iri_free(&mut (*verdict).alt_iri);
    }
    if !((*verdict).alt_local_filename).is_null() {
        wget_free
            .expect(
                "non-null function pointer",
            )((*verdict).alt_local_filename as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_forward_downloaded_file(
    mut iri: *const wget_iri,
    mut size: int64_t,
    mut filename: *const libc::c_char,
    mut data: *const libc::c_void,
    mut recurse_iris: *mut wget_vector,
) -> libc::c_int {
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut file: downloaded_file = {
        let mut init = downloaded_file {
            parent: {
                let mut init = wget_downloaded_file {
                    vtable: &mut vtable,
                };
                init
            },
            iri: iri,
            filename: filename,
            size: size as uint64_t,
            data: data,
            data_buf: 0 as *mut libc::c_void,
            recurse_iris: recurse_iris,
        };
        init
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < wget_vector_size(plugin_list) {
        let mut plugin: *mut plugin_t = wget_vector_get(plugin_list, i) as *mut plugin_t;
        let mut priv_0: *mut plugin_priv_t = plugin as *mut plugin_priv_t;
        if ((*priv_0).post_processor).is_some() {
            if ((*priv_0).post_processor)
                .expect(
                    "non-null function pointer",
                )(
                plugin as *mut wget_plugin,
                &mut file as *mut downloaded_file as *mut wget_downloaded_file,
            ) == 0 as libc::c_int
            {
                ret = 0 as libc::c_int;
                break;
            }
        }
        i += 1;
        i;
    }
    if !(file.data_buf).is_null() {
        wget_free.expect("non-null function pointer")(file.data_buf);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_init() {
    if !initialized {
        search_paths = wget_vector_create(16 as libc::c_int, None);
        plugin_list = wget_vector_create(16 as libc::c_int, None);
        wget_vector_set_destructor(
            plugin_list,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut plugin_t) -> ()>,
                Option::<wget_vector_destructor>,
            >(Some(plugin_free as unsafe extern "C" fn(*mut plugin_t) -> ())),
        );
        plugin_name_index = wget_stringmap_create(16 as libc::c_int);
        wget_stringmap_set_key_destructor(plugin_name_index, None);
        wget_stringmap_set_value_destructor(plugin_name_index, None);
        plugin_help_forwarded = 0 as libc::c_int != 0;
        initialized = 1 as libc::c_int != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plugin_db_finalize(mut exitcode: libc::c_int) {
    if !initialized {
        return;
    }
    let mut n_plugins: libc::c_int = wget_vector_size(plugin_list);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n_plugins {
        let mut plugin: *mut plugin_t = wget_vector_get(plugin_list, i) as *mut plugin_t;
        let mut priv_0: *mut plugin_priv_t = plugin as *mut plugin_priv_t;
        if ((*priv_0).finalizer).is_some() {
            ((*priv_0).finalizer)
                .expect(
                    "non-null function pointer",
                )(plugin as *mut wget_plugin, exitcode);
        }
        i += 1;
        i;
    }
    wget_vector_free(&mut plugin_list);
    wget_stringmap_free(&mut plugin_name_index);
    wget_vector_free(&mut search_paths);
    initialized = 0 as libc::c_int != 0;
}
