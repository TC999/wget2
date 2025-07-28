#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::libc;

mod bar;
mod blacklist;

#![feature(c_variadic, core_intrinsics, extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type re_dfa_t;
    pub type pcre2_real_general_context_8;
    pub type pcre2_real_compile_context_8;
    pub type pcre2_real_match_context_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_match_data_8;
    pub type wget_list_st;
    pub type wget_vector_st;
    pub type wget_hashmap_st;
    pub type wget_hashmap_iterator_st;
    pub type wget_thread_st;
    pub type wget_thread_mutex_st;
    pub type wget_thread_cond_st;
    pub type wget_decompressor_st;
    pub type wget_cookie_db_st;
    pub type wget_hsts_db_st;
    pub type wget_hpkp_db_st;
    pub type wget_hpkp_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_netrc_db_st;
    pub type wget_http_connection_st;
    pub type wget_robots_st;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rpl_fclose(stream: *mut FILE) -> libc::c_int;
    fn rpl_fopen(filename: *const libc::c_char, mode: *const libc::c_char) -> *mut FILE;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn rand() -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn canonicalize_file_name(__name: *const libc::c_char) -> *mut libc::c_char;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn futimens(__fd: libc::c_int, __times: *const timespec) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn regcomp(
        __preg: *mut regex_t,
        __pattern: *const libc::c_char,
        __cflags: libc::c_int,
    ) -> libc::c_int;
    fn regexec(
        __preg: *const regex_t,
        __String: *const libc::c_char,
        __nmatch: size_t,
        __pmatch: *mut regmatch_t,
        __eflags: libc::c_int,
    ) -> libc::c_int;
    fn regfree(__preg: *mut regex_t);
    fn gettime(_: *mut timespec);
    fn safe_read(fd: libc::c_int, buf: *mut libc::c_void, count: idx_t) -> ptrdiff_t;
    fn safe_write(fd: libc::c_int, buf: *const libc::c_void, count: idx_t) -> ptrdiff_t;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn pcre2_compile_8(
        _: PCRE2_SPTR8,
        _: size_t,
        _: uint32_t,
        _: *mut libc::c_int,
        _: *mut size_t,
        _: *mut pcre2_compile_context_8,
    ) -> *mut pcre2_code_8;
    fn pcre2_code_free_8(_: *mut pcre2_code_8);
    fn pcre2_match_data_create_from_pattern_8(
        _: *const pcre2_code_8,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> libc::c_int;
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
    fn wget_strerror(err: wget_error) -> *const libc::c_char;
    fn wget_global_init(key: libc::c_int, _: ...);
    fn wget_ready_2_write(fd: libc::c_int, timeout: libc::c_int) -> libc::c_int;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_strncasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strncmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_strncasecmp(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
        n: size_t,
    ) -> libc::c_int;
    fn wget_millisleep(ms: libc::c_int);
    fn wget_get_timemillis() -> libc::c_longlong;
    fn wget_match_tail(s: *const libc::c_char, tail: *const libc::c_char) -> libc::c_int;
    fn wget_match_tail_nocase(
        s: *const libc::c_char,
        tail: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_human_readable(
        buf: *mut libc::c_char,
        bufsize: size_t,
        n: uint64_t,
    ) -> *mut libc::c_char;
    fn wget_fdgetline(
        buf: *mut *mut libc::c_char,
        bufsize: *mut size_t,
        fd: libc::c_int,
    ) -> ssize_t;
    fn wget_read_file(
        fname: *const libc::c_char,
        size: *mut size_t,
    ) -> *mut libc::c_char;
    fn wget_memiconv(
        src_encoding: *const libc::c_char,
        src: *const libc::c_void,
        srclen: size_t,
        dst_encoding: *const libc::c_char,
        out: *mut *mut libc::c_char,
        outlen: *mut size_t,
    ) -> libc::c_int;
    fn wget_strscpy(
        dst: *mut libc::c_char,
        src: *const libc::c_char,
        size: size_t,
    ) -> ssize_t;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_calloc_fn: Option::<wget_calloc_function>;
    static mut wget_realloc_fn: Option::<wget_realloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_memdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_void;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_strmemcpy_a(
        s: *mut libc::c_char,
        ssize: size_t,
        m: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_alloc(size: size_t) -> *mut wget_buffer;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_reset(buf: *mut wget_buffer);
    fn wget_buffer_memcpy(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_memcat(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_strcpy(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_strcat(buf: *mut wget_buffer, s: *const libc::c_char) -> size_t;
    fn wget_buffer_memset_append(
        buf: *mut wget_buffer,
        c: libc::c_char,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_printf_append(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_aprintf(fmt: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn wget_printf(fmt: *const libc::c_char, _: ...) -> size_t;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_info_vprintf(fmt: *const libc::c_char, args: ::core::ffi::VaList);
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf_exit(fmt: *const libc::c_char, _: ...) -> !;
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_find(v: *const wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_contains(v: *const wget_vector, elem: *const libc::c_void) -> bool;
    fn wget_vector_add_memdup(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        size: size_t,
    ) -> libc::c_int;
    fn wget_vector_add(v: *mut wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_size(v: *const wget_vector) -> libc::c_int;
    fn wget_vector_browse(
        v: *const wget_vector,
        browse: Option::<wget_vector_browse_fn>,
        ctx: *mut libc::c_void,
    ) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_clear_nofree(v: *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_hashmap_create(
        max: libc::c_int,
        hash: Option::<wget_hashmap_hash_fn>,
        cmp: Option::<wget_hashmap_compare_fn>,
    ) -> *mut wget_hashmap;
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
    fn wget_hashmap_iterator_alloc(h: *mut wget_hashmap) -> *mut wget_hashmap_iterator;
    fn wget_hashmap_iterator_free(iter: *mut *mut wget_hashmap_iterator);
    fn wget_hashmap_iterator_next(
        iter: *mut wget_hashmap_iterator,
        value: *mut *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn wget_stringmap_create(max: libc::c_int) -> *mut wget_stringmap;
    fn wget_stringmap_create_nocase(max: libc::c_int) -> *mut wget_stringmap;
    fn wget_thread_start(
        thread: *mut wget_thread,
        start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        arg: *mut libc::c_void,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn wget_thread_mutex_init(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_destroy(mutex: *mut wget_thread_mutex) -> libc::c_int;
    fn wget_thread_mutex_lock(mutex: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex: wget_thread_mutex);
    fn wget_thread_join(thread: *mut wget_thread) -> libc::c_int;
    fn wget_thread_cond_init(cond: *mut wget_thread_cond) -> libc::c_int;
    fn wget_thread_cond_destroy(cond: *mut wget_thread_cond) -> libc::c_int;
    fn wget_thread_cond_signal(cond: wget_thread_cond) -> libc::c_int;
    fn wget_thread_cond_wait(
        cond: wget_thread_cond,
        mutex: wget_thread_mutex,
        ms: libc::c_longlong,
    ) -> libc::c_int;
    fn wget_thread_support() -> bool;
    fn wget_content_encoding_to_name(
        type_0: wget_content_encoding,
    ) -> *const libc::c_char;
    fn wget_decompress_open(
        encoding: wget_content_encoding,
        data_sink: Option::<wget_decompressor_sink_fn>,
        context: *mut libc::c_void,
    ) -> *mut wget_decompressor;
    fn wget_decompress_close(dc: *mut wget_decompressor);
    fn wget_decompress(
        dc: *mut wget_decompressor,
        src: *const libc::c_char,
        srclen: size_t,
    ) -> libc::c_int;
    fn wget_iri_free(iri: *mut *mut wget_iri);
    fn wget_iri_free_content(iri: *mut wget_iri);
    fn wget_iri_supported(iri: *const wget_iri) -> bool;
    fn wget_iri_compare(iri1: *const wget_iri, iri2: *const wget_iri) -> libc::c_int;
    fn wget_iri_unescape_url_inline(src: *mut libc::c_char) -> *mut libc::c_char;
    fn wget_iri_parse(
        uri: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_iri_parse_base(
        base: *const wget_iri,
        url: *const libc::c_char,
        encoding: *const libc::c_char,
    ) -> *mut wget_iri;
    fn wget_iri_clone(iri: *const wget_iri) -> *mut wget_iri;
    fn wget_iri_get_connection_part(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_iri_relative_to_abs(
        base: *const wget_iri,
        val: *const libc::c_char,
        len: size_t,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_iri_escape_path(
        src: *const libc::c_char,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_iri_get_escaped_resource(
        iri: *const wget_iri,
        buf: *mut wget_buffer,
    ) -> *const libc::c_char;
    fn wget_iri_set_scheme(
        iri: *mut wget_iri,
        scheme: wget_iri_scheme,
    ) -> wget_iri_scheme;
    fn wget_iri_scheme_get_name(scheme: wget_iri_scheme) -> *const libc::c_char;
    fn wget_cookie_normalize_cookies(iri: *const wget_iri, cookies: *const wget_vector);
    fn wget_cookie_store_cookies(
        cookie_db: *mut wget_cookie_db,
        cookies: *mut wget_vector,
    );
    fn wget_cookie_db_save(
        cookie_db: *mut wget_cookie_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_cookie_create_request_header(
        cookie_db: *mut wget_cookie_db,
        iri: *const wget_iri,
    ) -> *mut libc::c_char;
    fn wget_hsts_host_match(
        _: *const wget_hsts_db,
        _: *const libc::c_char,
        _: uint16_t,
    ) -> libc::c_int;
    fn wget_hsts_db_add(
        _: *mut wget_hsts_db,
        _: *const libc::c_char,
        _: uint16_t,
        _: int64_t,
        _: bool,
    );
    fn wget_hsts_db_save(_: *mut wget_hsts_db) -> libc::c_int;
    fn wget_hpkp_set_host(hpkp: *mut wget_hpkp, host: *const libc::c_char);
    fn wget_hpkp_db_add(_: *mut wget_hpkp_db, _: *mut *mut wget_hpkp);
    fn wget_hpkp_db_save(_: *mut wget_hpkp_db) -> libc::c_int;
    fn wget_tls_session_db_save(
        tls_session_db: *mut wget_tls_session_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_tls_session_db_changed(
        tls_session_db: *mut wget_tls_session_db,
    ) -> libc::c_int;
    fn wget_ocsp_db_save(_: *mut wget_ocsp_db) -> libc::c_int;
    fn wget_netrc_db_init(netrc_db: *mut wget_netrc_db) -> *mut wget_netrc_db;
    fn wget_netrc_get(
        netrc_db: *const wget_netrc_db,
        host: *const libc::c_char,
    ) -> *mut wget_netrc;
    fn wget_netrc_db_load(
        netrc_db: *mut wget_netrc_db,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_css_parse_buffer(
        buf: *const libc::c_char,
        len: size_t,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
    fn wget_css_parse_file(
        fname: *const libc::c_char,
        callback_uri: Option::<wget_css_parse_uri_callback>,
        callback_encoding: Option::<wget_css_parse_encoding_callback>,
        user_ctx: *mut libc::c_void,
    );
    fn wget_html_get_urls_inline(
        html: *const libc::c_char,
        additional_tags: *mut wget_vector,
        ignore_tags: *mut wget_vector,
    ) -> *mut wget_html_parsed_result;
    fn wget_html_free_urls_inline(res: *mut *mut wget_html_parsed_result);
    fn wget_sitemap_get_urls_inline(
        sitemap: *const libc::c_char,
        urls: *mut *mut wget_vector,
        sitemap_urls: *mut *mut wget_vector,
    );
    fn wget_atom_get_urls_inline(atom: *const libc::c_char, urls: *mut *mut wget_vector);
    fn wget_rss_get_urls_inline(rss: *const libc::c_char, urls: *mut *mut wget_vector);
    fn wget_xml_decode_entities_inline(src: *mut libc::c_char) -> *mut libc::c_char;
    fn wget_ip_is_family(host: *const libc::c_char, family: libc::c_int) -> bool;
    fn wget_http_get_host(conn: *const wget_http_connection) -> *const libc::c_char;
    fn wget_http_get_port(conn: *const wget_http_connection) -> uint16_t;
    fn wget_http_get_scheme(conn: *const wget_http_connection) -> wget_iri_scheme;
    fn wget_http_get_protocol(conn: *const wget_http_connection) -> libc::c_int;
    fn wget_http_print_date(
        t: int64_t,
        buf: *mut libc::c_char,
        bufsize: size_t,
    ) -> *mut libc::c_char;
    fn wget_http_add_header_printf(
        req: *mut wget_http_request,
        name: *const libc::c_char,
        fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn wget_http_add_header(
        req: *mut wget_http_request,
        name: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_http_add_header_param(
        req: *mut wget_http_request,
        param: *mut wget_http_header_param,
    ) -> libc::c_int;
    fn wget_http_add_credentials(
        req: *mut wget_http_request,
        challenge: *mut wget_http_challenge,
        username: *const libc::c_char,
        password: *const libc::c_char,
        proxied: libc::c_int,
    );
    fn wget_http_abort_connection(conn: *mut wget_http_connection);
    fn wget_http_connection_receive_only(conn: *mut wget_http_connection) -> bool;
    fn wget_http_free_request(req: *mut *mut wget_http_request);
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_get_response_cb(
        conn: *mut wget_http_connection,
    ) -> *mut wget_http_response;
    fn wget_http_open(
        _conn: *mut *mut wget_http_connection,
        iri: *const wget_iri,
    ) -> libc::c_int;
    fn wget_http_create_request(
        iri: *const wget_iri,
        method: *const libc::c_char,
    ) -> *mut wget_http_request;
    fn wget_http_close(conn: *mut *mut wget_http_connection);
    fn wget_http_request_set_header_cb(
        req: *mut wget_http_request,
        cb: Option::<wget_http_header_callback>,
        user_data: *mut libc::c_void,
    );
    fn wget_http_request_set_body_cb(
        req: *mut wget_http_request,
        cb: Option::<wget_http_body_callback>,
        user_data: *mut libc::c_void,
    );
    fn wget_http_request_set_int(
        req: *mut wget_http_request,
        key: libc::c_int,
        value: libc::c_int,
    );
    fn wget_http_request_set_ptr(
        req: *mut wget_http_request,
        key: libc::c_int,
        value: *mut libc::c_void,
    );
    fn wget_http_request_set_body(
        req: *mut wget_http_request,
        mimetype: *const libc::c_char,
        body: *mut libc::c_char,
        length: size_t,
    );
    fn wget_http_send_request(
        conn: *mut wget_http_connection,
        req: *mut wget_http_request,
    ) -> libc::c_int;
    fn wget_metalink_parse(xml: *const libc::c_char) -> *mut wget_metalink;
    fn wget_metalink_free(metalink: *mut *mut wget_metalink);
    fn wget_metalink_sort_mirrors(metalink: *mut wget_metalink);
    fn wget_robots_parse(
        robots: *mut *mut wget_robots,
        data: *const libc::c_char,
        client: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_robots_get_path_count(robots: *mut wget_robots) -> libc::c_int;
    fn wget_robots_get_path(
        robots: *mut wget_robots,
        index: libc::c_int,
    ) -> *mut wget_string;
    fn wget_robots_get_sitemap_count(robots: *mut wget_robots) -> libc::c_int;
    fn wget_robots_get_sitemap(
        robots: *mut wget_robots,
        index: libc::c_int,
    ) -> *const libc::c_char;
    fn wget_bar_screen_resized();
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn blacklist_init();
    fn blacklist_exit();
    fn blacklist_add(iri: *const wget_iri) -> *mut blacklist_entry;
    fn blacklist_get(iri: *const wget_iri) -> *mut blacklist_entry;
    fn blacklist_print();
    fn blacklist_free();
    fn blacklist_set_filename(
        blacklistp: *mut blacklist_entry,
        fname: *const libc::c_char,
    );
    fn get_local_filename(iri: *const wget_iri) -> *mut libc::c_char;
    fn job_init(
        job: *mut JOB,
        blacklistp: *mut blacklist_entry,
        http_fallback: bool,
    ) -> *mut JOB;
    fn job_validate_file(job: *mut JOB) -> libc::c_int;
    fn host_init();
    fn host_exit();
    fn host_add(iri: *const wget_iri) -> *mut HOST;
    fn host_get(iri: *const wget_iri) -> *mut HOST;
    fn host_get_job(host: *mut HOST, pause: *mut libc::c_longlong) -> *mut JOB;
    fn host_add_job(host: *mut HOST, job: *const JOB);
    fn host_add_robotstxt_job(
        host: *mut HOST,
        iri: *const wget_iri,
        encoding: *const libc::c_char,
        http_fallback: bool,
    );
    fn host_release_jobs(host: *mut HOST);
    fn host_remove_job(host: *mut HOST, job: *mut JOB);
    fn hosts_free();
    fn host_increase_failure(host: *mut HOST);
    fn host_final_failure(host: *mut HOST);
    fn host_reset_failure(host: *mut HOST);
    fn queue_size() -> libc::c_int;
    fn queue_empty() -> libc::c_int;
    static mut config: config;
    fn init(argc: libc::c_int, argv: *mut *const libc::c_char) -> libc::c_int;
    fn deinit();
    fn set_exit_status(status: exit_status_e);
    fn get_exit_status() -> exit_status_e;
    fn bar_init() -> bool;
    fn bar_deinit();
    fn bar_print(slot: libc::c_int, s: *const libc::c_char);
    fn bar_printf(slot: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn bar_slot_begin(
        slot: libc::c_int,
        filename: *const libc::c_char,
        new_file: libc::c_int,
        filesize: ssize_t,
    );
    fn bar_set_downloaded(slot: libc::c_int, nbytes: size_t);
    fn bar_slot_deregister(slot: libc::c_int);
    fn bar_update_slots(nslots: libc::c_int);
    fn fsetxattr(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __value: *const libc::c_void,
        __size: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn fgetxattr(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn plugin_db_init();
    fn plugin_db_add_search_paths(paths: *const libc::c_char, separator: libc::c_char);
    fn plugin_db_forward_url(
        iri: *const wget_iri,
        verdict: *mut plugin_db_forward_url_verdict,
    );
    fn plugin_db_forward_url_verdict_free(verdict: *mut plugin_db_forward_url_verdict);
    fn plugin_db_forward_downloaded_file(
        iri: *const wget_iri,
        size: int64_t,
        filename: *const libc::c_char,
        data: *const libc::c_void,
        recurse_iris: *mut wget_vector,
    ) -> libc::c_int;
    fn plugin_db_finalize(exitcode: libc::c_int);
    fn site_stats_print();
    fn stats_site_add(resp: *mut wget_http_response, gpg_info: *mut wget_gpg_info_t);
    fn is_testing() -> bool;
    fn mkdir_path(fname: *const libc::c_char, is_file_0: bool);
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
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_8,
    pub _timer: C2RustUnnamed_7,
    pub _rt: C2RustUnnamed_6,
    pub _sigchld: C2RustUnnamed_5,
    pub _sigfault: C2RustUnnamed_2,
    pub _sigpoll: C2RustUnnamed_1,
    pub _sigsys: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub _addr_bnd: C2RustUnnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISprint: C2RustUnnamed_10 = 16384;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
pub type __re_long_size_t = libc::c_ulong;
pub type reg_syntax_t = libc::c_ulong;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut libc::c_char,
    pub translate: *mut libc::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "libc::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "libc::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regex_t = re_pattern_buffer;
pub type regoff_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}
pub type idx_t = ptrdiff_t;
pub type PCRE2_UCHAR8 = uint8_t;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
pub type pcre2_compile_context_8 = pcre2_real_compile_context_8;
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type pcre2_code_8 = pcre2_real_code_8;
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
pub type wget_error = libc::c_int;
pub const WGET_E_UNSUPPORTED: wget_error = -12;
pub const WGET_E_IO: wget_error = -11;
pub const WGET_E_OPEN: wget_error = -10;
pub const WGET_E_XML_PARSE_ERR: wget_error = -9;
pub const WGET_E_TLS_DISABLED: wget_error = -8;
pub const WGET_E_CERTIFICATE: wget_error = -7;
pub const WGET_E_HANDSHAKE: wget_error = -6;
pub const WGET_E_CONNECT: wget_error = -5;
pub const WGET_E_TIMEOUT: wget_error = -4;
pub const WGET_E_INVALID: wget_error = -3;
pub const WGET_E_MEMORY: wget_error = -2;
pub const WGET_E_UNKNOWN: wget_error = -1;
pub const WGET_E_SUCCESS: wget_error = 0;
pub type wget_list = wget_list_st;
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
pub type wget_vector = wget_vector_st;
pub type wget_vector_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_vector_browse_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_hashmap = wget_hashmap_st;
pub type wget_hashmap_compare_fn = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
) -> libc::c_int;
pub type wget_hashmap_hash_fn = unsafe extern "C" fn(
    *const libc::c_void,
) -> libc::c_uint;
pub type wget_hashmap_key_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_hashmap_iterator = wget_hashmap_iterator_st;
pub type wget_stringmap = wget_hashmap;
pub type wget_stringmap_key_destructor = unsafe extern "C" fn(*mut libc::c_char) -> ();
pub type wget_stringmap_value_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_stringmap_iterator = wget_hashmap_iterator;
pub type wget_thread_id = libc::c_ulong;
pub type wget_thread = *mut wget_thread_st;
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_thread_cond = *mut wget_thread_cond_st;
pub type wget_decompressor = wget_decompressor_st;
pub type wget_decompressor_sink_fn = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
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
pub type wget_cookie_db = wget_cookie_db_st;
pub type wget_hsts_db = wget_hsts_db_st;
pub type wget_hsts_host_match_fn = unsafe extern "C" fn(
    *const wget_hsts_db,
    *const libc::c_char,
    uint16_t,
) -> libc::c_int;
pub type wget_hsts_db_add_fn = unsafe extern "C" fn(
    *mut wget_hsts_db,
    *const libc::c_char,
    uint16_t,
    int64_t,
    bool,
) -> ();
pub type wget_hsts_db_save_fn = unsafe extern "C" fn(*mut wget_hsts_db) -> libc::c_int;
pub type wget_hpkp_db = wget_hpkp_db_st;
pub type wget_hpkp = wget_hpkp_st;
pub type wget_hpkp_db_add_fn = unsafe extern "C" fn(
    *mut wget_hpkp_db,
    *mut *mut wget_hpkp,
) -> ();
pub type wget_hpkp_db_save_fn = unsafe extern "C" fn(*mut wget_hpkp_db) -> libc::c_int;
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_ocsp_db_save_fn = unsafe extern "C" fn(*mut wget_ocsp_db) -> libc::c_int;
pub type wget_netrc_db = wget_netrc_db_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_netrc_st {
    pub host: *const libc::c_char,
    pub login: *const libc::c_char,
    pub password: *const libc::c_char,
    pub port: uint16_t,
    #[bitfield(name = "force", ty = "bool", bits = "0..=0")]
    pub force: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type wget_netrc = wget_netrc_st;
pub type wget_css_parse_uri_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
    size_t,
) -> ();
pub type wget_css_parse_encoding_callback = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_string {
    pub p: *const libc::c_char,
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_html_parsed_url {
    pub url: wget_string,
    pub download: wget_string,
    pub attr: [libc::c_char; 16],
    pub tag: [libc::c_char; 16],
    #[bitfield(name = "link_inline", ty = "bool", bits = "0..=0")]
    pub link_inline: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_html_parsed_result {
    pub uris: *mut wget_vector,
    pub encoding: *const libc::c_char,
    pub base: wget_string,
    #[bitfield(name = "follow", ty = "bool", bits = "0..=0")]
    pub follow: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_header_param {
    pub name: *const libc::c_char,
    pub value: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_link {
    pub uri: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub pri: libc::c_int,
    pub rel: C2RustUnnamed_11,
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const link_rel_duplicate: C2RustUnnamed_11 = 2;
pub const link_rel_describedby: C2RustUnnamed_11 = 1;
pub const link_rel_none: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_http_challenge {
    pub auth_scheme: *const libc::c_char,
    pub params: *mut wget_stringmap,
}
pub type wget_transfer_encoding = libc::c_uint;
pub const wget_transfer_encoding_chunked: wget_transfer_encoding = 1;
pub const wget_transfer_encoding_identity: wget_transfer_encoding = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_response_st {
    pub req: *mut wget_http_request,
    pub links: *mut wget_vector,
    pub digests: *mut wget_vector,
    pub cookies: *mut wget_vector,
    pub challenges: *mut wget_vector,
    pub hpkp: *mut wget_hpkp,
    pub content_type: *const libc::c_char,
    pub content_type_encoding: *const libc::c_char,
    pub content_filename: *const libc::c_char,
    pub location: *const libc::c_char,
    pub etag: *const libc::c_char,
    pub header: *mut wget_buffer,
    pub body: *mut wget_buffer,
    pub response_end: libc::c_longlong,
    pub content_length: size_t,
    pub cur_downloaded: size_t,
    pub accounted_for: size_t,
    pub last_modified: int64_t,
    pub hsts_maxage: int64_t,
    pub reason: [libc::c_char; 32],
    pub icy_metaint: libc::c_int,
    pub major: libc::c_short,
    pub minor: libc::c_short,
    pub code: libc::c_short,
    pub transfer_encoding: wget_transfer_encoding,
    pub content_encoding: libc::c_char,
    pub hsts_include_subdomains: bool,
    pub keep_alive: bool,
    #[bitfield(name = "content_length_valid", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "length_inconsistent", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "hsts", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "csp", ty = "bool", bits = "3..=3")]
    pub content_length_valid_length_inconsistent_hsts_csp: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_http_request {
    pub headers: *mut wget_vector,
    pub body: *const libc::c_char,
    pub header_callback: Option::<wget_http_header_callback>,
    pub body_callback: Option::<wget_http_body_callback>,
    pub user_data: *mut libc::c_void,
    pub header_user_data: *mut libc::c_void,
    pub body_user_data: *mut libc::c_void,
    pub esc_resource: wget_buffer,
    pub esc_host: wget_buffer,
    pub body_length: size_t,
    pub stream_id: int32_t,
    pub scheme: wget_iri_scheme,
    pub esc_resource_buf: [libc::c_char; 256],
    pub esc_host_buf: [libc::c_char; 64],
    pub method: [libc::c_char; 8],
    #[bitfield(name = "response_keepheader", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "response_ignorelength", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "debug_skip_body", ty = "bool", bits = "2..=2")]
    pub response_keepheader_response_ignorelength_debug_skip_body: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub request_start: libc::c_longlong,
    pub first_response_start: libc::c_longlong,
}
pub type wget_http_body_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> libc::c_int;
pub type wget_http_response = wget_http_response_st;
pub type wget_http_header_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
) -> libc::c_int;
pub type wget_http_connection = wget_http_connection_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink_mirror {
    pub iri: *const wget_iri,
    pub priority: libc::c_int,
    pub location: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink_hash {
    pub type_0: [libc::c_char; 16],
    pub hash_hex: [libc::c_char; 129],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink_piece {
    pub hash: wget_metalink_hash,
    pub position: off_t,
    pub length: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_metalink {
    pub name: *const libc::c_char,
    pub mirrors: *mut wget_vector,
    pub hashes: *mut wget_vector,
    pub pieces: *mut wget_vector,
    pub size: off_t,
}
pub type wget_robots = wget_robots_st;
pub type wget_report_speed = libc::c_uint;
pub const WGET_REPORT_SPEED_BITS: wget_report_speed = 1;
pub const WGET_REPORT_SPEED_BYTES: wget_report_speed = 0;
pub type wget_stats_format = libc::c_uint;
pub const WGET_STATS_FORMAT_CSV: wget_stats_format = 1;
pub const WGET_STATS_FORMAT_HUMAN: wget_stats_format = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blacklist_entry {
    pub iri: *const wget_iri,
    pub local_filename: *mut libc::c_char,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct JOB {
    pub iri: *const wget_iri,
    pub original_url: *const wget_iri,
    pub referer: *const wget_iri,
    pub metalink: *mut wget_metalink,
    pub challenges: *mut wget_vector,
    pub proxy_challenges: *mut wget_vector,
    pub parts: *mut wget_vector,
    pub remaining_sig_ext: *mut wget_list,
    pub host: *mut HOST,
    pub blacklist_entry: *const blacklist_entry,
    pub sig_filename: *mut libc::c_char,
    pub sig_req: *mut libc::c_char,
    pub part: *mut PART,
    pub downloader: *mut DOWNLOADER,
    pub used_by: wget_thread_id,
    pub id: libc::c_ulonglong,
    pub parent_id: libc::c_ulonglong,
    pub retry_ts: libc::c_longlong,
    pub level: libc::c_int,
    pub redirection_level: libc::c_int,
    pub auth_failure_count: libc::c_int,
    pub mirror_pos: libc::c_int,
    pub piece_pos: libc::c_int,
    pub failures: libc::c_int,
    #[bitfield(name = "challenges_alloc", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "inuse", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "done", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "sitemap", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "robotstxt", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "head_first", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "requested_by_user", ty = "bool", bits = "6..=6")]
    #[bitfield(name = "ignore_patterns", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "http_fallback", ty = "bool", bits = "8..=8")]
    #[bitfield(name = "recursive_send_head", ty = "bool", bits = "9..=9")]
    #[bitfield(name = "redirect_get", ty = "bool", bits = "10..=10")]
    pub challenges_alloc_inuse_done_sitemap_robotstxt_head_first_requested_by_user_ignore_patterns_http_fallback_recursive_send_head_redirect_get: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 6],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct DOWNLOADER {
    pub thread: wget_thread,
    pub job: *mut JOB,
    pub conn: *mut wget_http_connection,
    pub buf: *mut libc::c_char,
    pub bufsize: size_t,
    pub id: libc::c_int,
    pub cond: wget_thread_cond,
    #[bitfield(name = "final_error", ty = "bool", bits = "0..=0")]
    pub final_error: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct PART {
    pub position: off_t,
    pub length: off_t,
    pub id: libc::c_int,
    pub used_by: wget_thread_id,
    #[bitfield(name = "inuse", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "done", ty = "bool", bits = "1..=1")]
    pub inuse_done: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct HOST {
    pub host: *const libc::c_char,
    pub robot_job: *mut JOB,
    pub robots: *mut wget_robots,
    pub queue: *mut wget_list,
    pub retry_ts: libc::c_longlong,
    pub qsize: libc::c_int,
    pub failures: libc::c_int,
    pub scheme: wget_iri_scheme,
    pub port: uint16_t,
    #[bitfield(name = "blocked", ty = "bool", bits = "0..=0")]
    pub blocked: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
pub type check_certificate_mode = libc::c_uint;
pub const CHECK_CERTIFICATE_LOG_DISABLED: check_certificate_mode = 2;
pub const CHECK_CERTIFICATE_DISABLED: check_certificate_mode = 1;
pub const CHECK_CERTIFICATE_ENABLED: check_certificate_mode = 0;
pub type https_enforce_mode = libc::c_uint;
pub const HTTPS_ENFORCE_HARD: https_enforce_mode = 2;
pub const HTTPS_ENFORCE_SOFT: https_enforce_mode = 1;
pub const HTTPS_ENFORCE_NONE: https_enforce_mode = 0;
pub type gpg_verify_mode = libc::c_uint;
pub const WGET_GPG_VERIFY_SIG_NO_FAIL: gpg_verify_mode = 2;
pub const GPG_VERIFY_SIG_FAIL: gpg_verify_mode = 1;
pub const GPG_VERIFY_DISABLED: gpg_verify_mode = 0;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const DOWNLOAD_ATTR_USEPATH: C2RustUnnamed_12 = 2;
pub const DOWNLOAD_ATTR_STRIPPATH: C2RustUnnamed_12 = 1;
pub const DOWNLOAD_ATTR_NO: C2RustUnnamed_12 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats_args {
    pub filename: *const libc::c_char,
    pub fp: *mut FILE,
    pub format: wget_stats_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config {
    pub base: *mut wget_iri,
    pub post_file: *const libc::c_char,
    pub post_data: *const libc::c_char,
    pub body_file: *const libc::c_char,
    pub body_data: *const libc::c_char,
    pub http_username: *const libc::c_char,
    pub http_password: *const libc::c_char,
    pub http_proxy_username: *const libc::c_char,
    pub http_proxy_password: *const libc::c_char,
    pub input_encoding: *const libc::c_char,
    pub local_encoding: *const libc::c_char,
    pub remote_encoding: *const libc::c_char,
    pub bind_address: *const libc::c_char,
    pub bind_interface: *const libc::c_char,
    pub input_file: *const libc::c_char,
    pub base_url: *const libc::c_char,
    pub default_page: *const libc::c_char,
    pub referer: *const libc::c_char,
    pub directory_prefix: *const libc::c_char,
    pub http_proxy: *const libc::c_char,
    pub https_proxy: *const libc::c_char,
    pub no_proxy: *const libc::c_char,
    pub cookie_suffixes: *const libc::c_char,
    pub load_cookies: *const libc::c_char,
    pub save_cookies: *const libc::c_char,
    pub logfile: *const libc::c_char,
    pub logfile_append: *const libc::c_char,
    pub user_agent: *const libc::c_char,
    pub output_document: *const libc::c_char,
    pub ca_cert: *const libc::c_char,
    pub ca_directory: *const libc::c_char,
    pub cert_file: *const libc::c_char,
    pub crl_file: *const libc::c_char,
    pub egd_file: *const libc::c_char,
    pub private_key: *const libc::c_char,
    pub random_file: *const libc::c_char,
    pub secure_protocol: *const libc::c_char,
    pub accept_regex: *const libc::c_char,
    pub reject_regex: *const libc::c_char,
    pub gnupg_homedir: *const libc::c_char,
    pub stats_all: *const libc::c_char,
    pub system_config: *const libc::c_char,
    pub user_config: *const libc::c_char,
    pub hsts_file: *const libc::c_char,
    pub hsts_preload_file: *const libc::c_char,
    pub hpkp_file: *const libc::c_char,
    pub tls_session_file: *const libc::c_char,
    pub ocsp_server: *const libc::c_char,
    pub ocsp_file: *const libc::c_char,
    pub netrc_file: *const libc::c_char,
    pub use_askpass_bin: *const libc::c_char,
    pub hostname: *const libc::c_char,
    pub dns_cache_preload: *const libc::c_char,
    pub method: *const libc::c_char,
    pub compression: *mut wget_vector,
    pub domains: *mut wget_vector,
    pub exclude_directories: *mut wget_vector,
    pub exclude_domains: *mut wget_vector,
    pub accept_patterns: *mut wget_vector,
    pub reject_patterns: *mut wget_vector,
    pub follow_tags: *mut wget_vector,
    pub ignore_tags: *mut wget_vector,
    pub default_challenges: *mut wget_vector,
    pub headers: *mut wget_vector,
    pub mime_types: *mut wget_vector,
    pub retry_on_http_error: *mut wget_vector,
    pub save_content_on: *mut wget_vector,
    pub compression_methods: [wget_content_encoding; 10],
    pub hsts_db: *mut wget_hsts_db,
    pub hpkp_db: *mut wget_hpkp_db,
    pub tls_session_db: *mut wget_tls_session_db,
    pub ocsp_db: *mut wget_ocsp_db,
    pub netrc_db: *mut wget_netrc_db,
    pub cookie_db: *mut wget_cookie_db,
    pub stats_dns_args: *mut stats_args,
    pub stats_ocsp_args: *mut stats_args,
    pub stats_server_args: *mut stats_args,
    pub stats_site_args: *mut stats_args,
    pub stats_tls_args: *mut stats_args,
    pub password: *mut libc::c_char,
    pub username: *mut libc::c_char,
    pub chunk_size: libc::c_longlong,
    pub quota: libc::c_longlong,
    pub limit_rate: libc::c_longlong,
    pub start_pos: libc::c_longlong,
    pub http2_request_window: libc::c_int,
    pub backups: libc::c_int,
    pub tries: libc::c_int,
    pub wait: libc::c_int,
    pub waitretry: libc::c_int,
    pub restrict_file_names: libc::c_int,
    pub level: libc::c_int,
    pub preferred_family: libc::c_int,
    pub cut_directories: libc::c_int,
    pub connect_timeout: libc::c_int,
    pub dns_timeout: libc::c_int,
    pub read_timeout: libc::c_int,
    pub max_redirect: libc::c_int,
    pub max_threads: libc::c_int,
    pub default_http_port: uint16_t,
    pub default_https_port: uint16_t,
    pub report_speed: wget_report_speed,
    pub check_certificate: check_certificate_mode,
    pub https_enforce: https_enforce_mode,
    pub verify_sig: gpg_verify_mode,
    pub cert_type: libc::c_char,
    pub private_key_type: libc::c_char,
    pub progress: libc::c_char,
    pub regex_type: libc::c_char,
    pub download_attr: libc::c_char,
    pub tls_resume: bool,
    pub content_on_error: bool,
    pub fsync_policy: bool,
    pub netrc: bool,
    pub http2: bool,
    pub http2_only: bool,
    pub ocsp_stapling: bool,
    pub ocsp: bool,
    pub mirror: bool,
    pub backup_converted: bool,
    pub convert_file_only: bool,
    pub convert_links: bool,
    pub ignore_case: bool,
    pub ignore_length: bool,
    pub hsts: bool,
    pub hsts_preload: bool,
    pub hpkp: bool,
    pub random_wait: bool,
    pub trust_server_names: bool,
    pub robots: bool,
    pub parent: bool,
    pub https_only: bool,
    pub content_disposition: bool,
    pub page_requisites: bool,
    pub follow_sitemaps: bool,
    pub force_rss: bool,
    pub force_atom: bool,
    pub force_sitemap: bool,
    pub force_css: bool,
    pub force_html: bool,
    pub force_metalink: bool,
    pub adjust_extension: bool,
    pub save_headers: bool,
    pub clobber: bool,
    pub cache: bool,
    pub inet4_only: bool,
    pub inet6_only: bool,
    pub delete_after: bool,
    pub strict_comments: bool,
    pub protocol_directories: bool,
    pub host_directories: bool,
    pub force_directories: bool,
    pub directories: bool,
    pub timestamping: bool,
    pub use_server_timestamps: bool,
    pub continue_download: bool,
    pub server_response: bool,
    pub keep_alive: bool,
    pub keep_extension: bool,
    pub keep_session_cookies: bool,
    pub cookies: bool,
    pub spider: bool,
    pub dns_caching: bool,
    pub check_hostname: bool,
    pub span_hosts: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub debug: bool,
    pub hyperlink: bool,
    pub metalink: bool,
    pub cut_url_get_vars: bool,
    pub cut_file_get_vars: bool,
    pub proxy: bool,
    pub xattr: bool,
    pub force_progress: bool,
    pub local_db: bool,
    pub dont_write: bool,
    pub filter_urls: bool,
    pub askpass: bool,
    pub verify_save_failed: bool,
    pub retry_connrefused: bool,
    pub unlink: bool,
    pub background: bool,
    pub if_modified_since: bool,
    pub auth_no_challenge: bool,
    pub no_compression: bool,
    pub ocsp_date: bool,
    pub ocsp_nonce: bool,
    pub recursive: bool,
    pub tls_false_start: bool,
    pub tcp_fastopen: bool,
    pub dane: bool,
}
pub type exit_status_e = libc::c_uint;
pub const EXIT_STATUS_GPG_ERROR: exit_status_e = 9;
pub const EXIT_STATUS_REMOTE: exit_status_e = 8;
pub const EXIT_STATUS_PROTOCOL: exit_status_e = 7;
pub const EXIT_STATUS_AUTH: exit_status_e = 6;
pub const EXIT_STATUS_TLS: exit_status_e = 5;
pub const EXIT_STATUS_NETWORK: exit_status_e = 4;
pub const EXIT_STATUS_IO: exit_status_e = 3;
pub const EXIT_STATUS_PARSE_INIT: exit_status_e = 2;
pub const EXIT_STATUS_GENERIC: exit_status_e = 1;
pub const EXIT_STATUS_NO_ERROR: exit_status_e = 0;
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
pub struct wget_gpg_info_t {
    pub bad_sigs: libc::c_int,
    pub missing_sigs: libc::c_int,
    pub invalid_sigs: libc::c_int,
    pub valid_sigs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conversion_t {
    pub filename: *const libc::c_char,
    pub encoding: *const libc::c_char,
    pub base: *const wget_iri,
    pub parsed: *mut wget_html_parsed_result,
    pub content_type: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statistics_t {
    pub ndownloads: libc::c_int,
    pub nredirects: libc::c_int,
    pub nnotmodified: libc::c_int,
    pub nerrors: libc::c_int,
    pub nchunks: libc::c_int,
    pub bytes_body_uncompressed: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct css_context {
    pub job: *mut JOB,
    pub base: *const wget_iri,
    pub encoding: *const libc::c_char,
    pub uri_buf: wget_buffer,
    pub encoding_allocated: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct body_callback_context {
    pub job: *mut JOB,
    pub body: *mut wget_buffer,
    pub max_memory: uint64_t,
    pub length: uint64_t,
    pub outfd: libc::c_int,
    pub progress_slot: libc::c_int,
    pub limit_debt_bytes: libc::c_longlong,
    pub limit_prev_time_ms: libc::c_longlong,
}
pub type actions = libc::c_uint;
pub const ACTION_ERROR: actions = 3;
pub const ACTION_GET_RESPONSE: actions = 2;
pub const ACTION_GET_JOB: actions = 1;
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn c_tolower(mut c: libc::c_int) -> libc::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80
        | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as i32 + 'a' as i32;
        }
        _ => return c,
    };
}
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
unsafe extern "C" fn wget_stringmap_iterator_next(
    mut h: *mut wget_stringmap_iterator,
    mut value: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    return wget_hashmap_iterator_next(h, value);
}
static mut conversions: *mut wget_stringmap = 0 as *const wget_stringmap
    as *mut wget_stringmap;
static mut stats: statistics_t = statistics_t {
    ndownloads: 0,
    nredirects: 0,
    nnotmodified: 0,
    nerrors: 0,
    nchunks: 0,
    bytes_body_uncompressed: 0,
};
static mut etags: *mut wget_stringmap = 0 as *const wget_stringmap
    as *mut wget_stringmap;
static mut known_urls: *mut wget_hashmap = 0 as *const wget_hashmap as *mut wget_hashmap;
static mut downloaders: *mut DOWNLOADER = 0 as *const DOWNLOADER as *mut DOWNLOADER;
static mut progress_thread: wget_thread = 0 as *const wget_thread_st
    as *mut wget_thread_st;
static mut quota_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut quota: libc::c_longlong = 0;
static mut hsts_changed: libc::c_int = 0;
static mut hpkp_changed: libc::c_int = 0;
static mut terminate: bool = false;
static mut nthreads: libc::c_int = 0;
unsafe extern "C" fn fetch_and_add_longlong(
    mut p: *mut libc::c_longlong,
    mut n: libc::c_longlong,
) -> libc::c_longlong {
    return ::core::intrinsics::atomic_xadd_seqcst(p, n);
}
unsafe extern "C" fn atomic_increment_int(mut p: *mut libc::c_int) {
    ::core::intrinsics::atomic_xadd_seqcst(p, 1 as libc::c_int);
}
unsafe extern "C" fn quota_modify_read(mut nbytes: size_t) -> libc::c_longlong {
    return fetch_and_add_longlong(&mut quota, nbytes as libc::c_longlong);
}
unsafe extern "C" fn nop(mut sig: libc::c_int) {
    if sig == 15 as libc::c_int {
        exit(EXIT_STATUS_GENERIC as libc::c_int);
    } else if sig == 2 as libc::c_int {
        if terminate {
            exit(EXIT_STATUS_GENERIC as libc::c_int);
        }
        ::core::ptr::write_volatile(&mut terminate as *mut bool, 1 as libc::c_int != 0);
        wget_http_abort_connection(0 as *mut wget_http_connection);
    } else if sig == 28 as libc::c_int {
        wget_bar_screen_resized();
    }
}
static mut input_tid: wget_thread = 0 as *const wget_thread_st as *mut wget_thread_st;
static mut parents: *mut wget_vector = 0 as *const wget_vector as *mut wget_vector;
static mut downloader_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut main_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut known_urls_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut etag_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut savefile_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut netrc_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut conversion_mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut main_cond: wget_thread_cond = 0 as *const wget_thread_cond_st
    as *mut wget_thread_cond_st;
static mut worker_cond: wget_thread_cond = 0 as *const wget_thread_cond_st
    as *mut wget_thread_cond_st;
unsafe extern "C" fn program_init() {
    let mut sig_action: sigaction = {
        let mut init = sigaction {
            __sigaction_handler: C2RustUnnamed_9 {
                sa_handler: ::core::mem::transmute::<
                    libc::intptr_t,
                    __sighandler_t,
                >(1 as libc::c_int as libc::intptr_t),
            },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        init
    };
    sigaction(13 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    sig_action
        .__sigaction_handler
        .sa_handler = Some(nop as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(15 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    sigaction(28 as libc::c_int, &mut sig_action, 0 as *mut sigaction);
    wget_global_init(0 as libc::c_int);
    blacklist_init();
    host_init();
    wget_thread_mutex_init(&mut downloader_mutex);
    wget_thread_mutex_init(&mut main_mutex);
    wget_thread_mutex_init(&mut known_urls_mutex);
    wget_thread_mutex_init(&mut etag_mutex);
    wget_thread_mutex_init(&mut savefile_mutex);
    wget_thread_mutex_init(&mut netrc_mutex);
    wget_thread_mutex_init(&mut conversion_mutex);
    wget_thread_mutex_init(&mut quota_mutex);
    wget_thread_cond_init(&mut main_cond);
    wget_thread_cond_init(&mut worker_cond);
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    bindtextdomain(
        b"wget\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"wget\0" as *const u8 as *const libc::c_char);
    known_urls = wget_hashmap_create(
        128 as libc::c_int,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_uint>,
            Option::<wget_hashmap_hash_fn>,
        >(Some(hash_url as unsafe extern "C" fn(*const libc::c_char) -> libc::c_uint)),
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            Option::<wget_hashmap_compare_fn>,
        >(
            Some(
                strcmp
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ),
    );
    plugin_db_init();
    plugin_db_add_search_paths(
        b"/usr/local/lib/wget2/plugins\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_char,
    );
}
unsafe extern "C" fn program_deinit() {
    host_exit();
    blacklist_exit();
    wget_thread_mutex_destroy(&mut downloader_mutex);
    wget_thread_mutex_destroy(&mut main_mutex);
    wget_thread_mutex_destroy(&mut known_urls_mutex);
    wget_thread_mutex_destroy(&mut etag_mutex);
    wget_thread_mutex_destroy(&mut savefile_mutex);
    wget_thread_mutex_destroy(&mut netrc_mutex);
    wget_thread_mutex_destroy(&mut conversion_mutex);
    wget_thread_mutex_destroy(&mut quota_mutex);
    wget_thread_cond_destroy(&mut main_cond);
    wget_thread_cond_destroy(&mut worker_cond);
}
unsafe extern "C" fn match_subdir(
    mut dir: *const libc::c_char,
    mut subdir: *const libc::c_char,
    mut ignore_case: bool,
) -> bool {
    if *dir as libc::c_int == '\0' as i32 {
        return strcmp(subdir, b"/\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int;
    }
    if ignore_case {
        while *dir as libc::c_int != 0 && *subdir as libc::c_int != 0
            && c_tolower(*dir as libc::c_int) == c_tolower(*subdir as libc::c_int)
        {
            dir = dir.offset(1);
            dir;
            subdir = subdir.offset(1);
            subdir;
        }
    } else {
        while *dir as libc::c_int != 0 && *subdir as libc::c_int != 0
            && {
                let fresh0 = dir;
                dir = dir.offset(1);
                let fresh1 = subdir;
                subdir = subdir.offset(1);
                *fresh0 as libc::c_int == *fresh1 as libc::c_int
            }
        {}
    }
    return *dir as libc::c_int == 0 as libc::c_int
        && (*subdir as libc::c_int == 0 as libc::c_int
            || *subdir as libc::c_int == '/' as i32);
}
unsafe extern "C" fn in_directory_pattern_list(
    mut v: *const wget_vector,
    mut fname: *const libc::c_char,
) -> libc::c_int {
    let mut pattern: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut default_exclude: bool = false;
    if fname.is_null() {
        return 0 as libc::c_int;
    }
    if *fname as libc::c_int == '/' as i32 {
        fname = fname.offset(1);
        fname;
    }
    let mut e: *const libc::c_char = strrchr(fname, '/' as i32);
    if e.is_null() {
        path = wget_strdup(b"/\0" as *const u8 as *const libc::c_char);
    } else {
        path = wget_strmemdup(
            fname as *const libc::c_void,
            e.offset_from(fname) as libc::c_long as size_t,
        );
    }
    pattern = wget_vector_get(v, 0 as libc::c_int) as *const libc::c_char;
    default_exclude = *pattern as libc::c_int == '+' as i32;
    let mut it: libc::c_int = wget_vector_size(v) - 1 as libc::c_int;
    while it >= 0 as libc::c_int {
        pattern = wget_vector_get(v, it) as *const libc::c_char;
        let mut exclude: bool = *pattern as libc::c_int != '+' as i32;
        pattern = pattern.offset(1);
        pattern;
        if *pattern as libc::c_int == '/' as i32 {
            pattern = pattern.offset(1);
            pattern;
        }
        wget_debug_printf(
            b"directory[%d] '%s' - '%s' %c\n\0" as *const u8 as *const libc::c_char,
            it,
            pattern,
            path,
            (*::core::mem::transmute::<
                &[u8; 3],
                &[libc::c_char; 3],
            >(b"+-\0"))[exclude as usize] as libc::c_int,
        );
        if !(strpbrk(pattern, b"*?[]\0" as *const u8 as *const libc::c_char)).is_null() {
            if fnmatch(
                pattern,
                path,
                (1 as libc::c_int) << 0 as libc::c_int
                    | (if config.ignore_case as libc::c_int != 0 {
                        (1 as libc::c_int) << 4 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }),
            ) == 0
            {
                wget_free.expect("non-null function pointer")(path as *mut libc::c_void);
                return exclude as libc::c_int;
            }
        } else if match_subdir(pattern, path, config.ignore_case) {
            wget_free.expect("non-null function pointer")(path as *mut libc::c_void);
            return exclude as libc::c_int;
        }
        it -= 1;
        it;
    }
    wget_free.expect("non-null function pointer")(path as *mut libc::c_void);
    return default_exclude as libc::c_int;
}
unsafe extern "C" fn in_pattern_list(
    mut v: *const wget_vector,
    mut url: *const libc::c_char,
) -> bool {
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(v) {
        let mut pattern: *const libc::c_char = wget_vector_get(v, it)
            as *const libc::c_char;
        if !(strpbrk(pattern, b"*?[]\0" as *const u8 as *const libc::c_char)).is_null() {
            if fnmatch(
                pattern,
                url,
                if config.ignore_case as libc::c_int != 0 {
                    (1 as libc::c_int) << 4 as libc::c_int
                } else {
                    0 as libc::c_int
                },
            ) == 0
            {
                return 1 as libc::c_int != 0;
            }
        } else if config.ignore_case {
            if wget_match_tail_nocase(url, pattern) != 0 {
                return 1 as libc::c_int != 0;
            }
        } else if wget_match_tail(url, pattern) != 0 {
            return 1 as libc::c_int != 0
        }
        it += 1;
        it;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn in_host_pattern_list(
    mut v: *const wget_vector,
    mut hostname: *const libc::c_char,
) -> libc::c_int {
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(v) {
        let mut pattern: *const libc::c_char = wget_vector_get(v, it)
            as *const libc::c_char;
        wget_debug_printf(
            b"host_pattern[%d] '%s' - %s\n\0" as *const u8 as *const libc::c_char,
            it,
            pattern,
            hostname,
        );
        if !(strpbrk(pattern, b"*?[]\0" as *const u8 as *const libc::c_char)).is_null() {
            if fnmatch(pattern, hostname, 0 as libc::c_int) == 0 {
                return 1 as libc::c_int;
            }
        } else if wget_match_tail(pattern, hostname) != 0 {
            return 1 as libc::c_int
        }
        it += 1;
        it;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn regex_match_posix(
    mut string: *const libc::c_char,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut re: regex_t = re_pattern_buffer {
        buffer: 0 as *mut re_dfa_t,
        allocated: 0,
        used: 0,
        syntax: 0,
        fastmap: 0 as *mut libc::c_char,
        translate: 0 as *mut libc::c_uchar,
        re_nsub: 0,
        can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
        c2rust_padding: [0; 7],
    };
    if regcomp(
        &mut re,
        pattern,
        1 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    status = regexec(
        &mut re,
        string,
        0 as libc::c_int as size_t,
        0 as *mut regmatch_t,
        0 as libc::c_int,
    );
    regfree(&mut re);
    if status != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn regex_match_pcre(
    mut string: *const libc::c_char,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    let mut re: *mut pcre2_code_8 = 0 as *mut pcre2_code_8;
    let mut errornumber: libc::c_int = 0;
    let mut erroroffset: size_t = 0;
    let mut match_data: *mut pcre2_match_data_8 = 0 as *mut pcre2_match_data_8;
    let mut rc: libc::c_int = 0;
    let mut result: libc::c_int = 0 as libc::c_int;
    re = pcre2_compile_8(
        pattern as PCRE2_SPTR8,
        !(0 as libc::c_int as size_t),
        0 as libc::c_int as uint32_t,
        &mut errornumber,
        &mut erroroffset,
        0 as *mut pcre2_compile_context_8,
    );
    if re.is_null() {
        return 0 as libc::c_int;
    }
    match_data = pcre2_match_data_create_from_pattern_8(
        re,
        0 as *mut pcre2_general_context_8,
    );
    rc = pcre2_match_8(
        re,
        string as PCRE2_SPTR8,
        strlen(string),
        0 as libc::c_int as size_t,
        0 as libc::c_int as uint32_t,
        match_data,
        0 as *mut pcre2_match_context_8,
    );
    if rc >= 0 as libc::c_int {
        result = 1 as libc::c_int;
    }
    pcre2_match_data_free_8(match_data);
    pcre2_code_free_8(re);
    return result;
}
unsafe extern "C" fn regex_match(
    mut string: *const libc::c_char,
    mut pattern: *const libc::c_char,
) -> libc::c_int {
    if config.regex_type as libc::c_int == 1 as libc::c_int {
        return regex_match_pcre(string, pattern)
    } else {
        return regex_match_posix(string, pattern)
    };
}
unsafe extern "C" fn parse_localfile(
    mut job: *mut JOB,
    mut fname: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut mimetype: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut fd: libc::c_int = 0;
    let mut level: libc::c_int = if !job.is_null() {
        (*job).level
    } else {
        0 as libc::c_int
    };
    let mut _mimetype: [libc::c_char; 64] = [0; 64];
    let mut _encoding: [libc::c_char; 32] = [0; 32];
    fd = open(fname, 0 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return;
    }
    if mimetype.is_null() {
        if read_xattr_metadata(
            b"user.mime_type\0" as *const u8 as *const libc::c_char,
            _mimetype.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            fd,
        ) < 0 as libc::c_int
        {
            *_mimetype.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        } else if *_mimetype.as_mut_ptr() != 0 {
            mimetype = _mimetype.as_mut_ptr();
        }
    }
    if encoding.is_null() {
        if read_xattr_metadata(
            b"user.charset\0" as *const u8 as *const libc::c_char,
            _encoding.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            fd,
        ) < 0 as libc::c_int
        {
            *_encoding.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        } else if *_encoding.as_mut_ptr() != 0 {
            encoding = _encoding.as_mut_ptr();
        }
    }
    close(fd);
    if !mimetype.is_null() {
        if wget_strcasecmp_ascii(
            mimetype,
            b"text/html\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                mimetype,
                b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            html_parse_localfile(job, level, fname, encoding, base);
        } else if wget_strcasecmp_ascii(
            mimetype,
            b"text/css\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            css_parse_localfile(job, fname, encoding, base);
        } else if wget_strcasecmp_ascii(
            mimetype,
            b"text/xml\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                mimetype,
                b"application/xml\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            sitemap_parse_xml_localfile(
                job,
                fname,
                if !encoding.is_null() {
                    encoding
                } else {
                    b"utf-8\0" as *const u8 as *const libc::c_char
                },
                base,
            );
        } else if wget_strcasecmp_ascii(
            mimetype,
            b"application/atom+xml\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            atom_parse_localfile(
                job,
                fname,
                if !encoding.is_null() {
                    encoding
                } else {
                    b"utf-8\0" as *const u8 as *const libc::c_char
                },
                base,
            );
        } else if wget_strcasecmp_ascii(
            mimetype,
            b"application/rss+xml\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            rss_parse_localfile(
                job,
                fname,
                if !encoding.is_null() {
                    encoding
                } else {
                    b"utf-8\0" as *const u8 as *const libc::c_char
                },
                base,
            );
        }
    } else {
        let mut ext: *const libc::c_char = strrchr(fname, '.' as i32);
        if !ext.is_null() {
            if wget_strcasecmp_ascii(ext, b".html\0" as *const u8 as *const libc::c_char)
                == 0
                || wget_strcasecmp_ascii(
                    ext,
                    b".htm\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                html_parse_localfile(job, level, fname, encoding, base);
            } else if wget_strcasecmp_ascii(
                ext,
                b".css\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                css_parse_localfile(job, fname, encoding, base);
            } else if wget_strcasecmp_ascii(
                ext,
                b".rss\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                rss_parse_localfile(
                    job,
                    fname,
                    if !encoding.is_null() {
                        encoding
                    } else {
                        b"utf-8\0" as *const u8 as *const libc::c_char
                    },
                    base,
                );
            }
        }
    };
}
unsafe extern "C" fn test_modify_hsts(mut iri: *mut wget_iri) {
    let mut match_0: bool = 0 as libc::c_int != 0;
    if config.hsts as libc::c_int != 0
        && wget_hsts_host_match(config.hsts_db, (*iri).host, (*iri).port) != 0
    {
        match_0 = 1 as libc::c_int != 0;
    }
    if match_0 {
        if wget_ip_is_family((*iri).host, 2 as libc::c_int) {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"HSTS in effect for [%s]:%hu\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*iri).host,
                (*iri).port as libc::c_int,
            );
        } else {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"HSTS in effect for %s:%hu\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*iri).host,
                (*iri).port as libc::c_int,
            );
        }
        wget_iri_set_scheme(iri, WGET_IRI_SCHEME_HTTPS);
    }
}
unsafe extern "C" fn add_parent(mut iri: *mut wget_iri) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if parents.is_null() {
        parents = wget_vector_create(4 as libc::c_int, None);
    }
    if ((*iri).path).is_null()
        || {
            p = strrchr((*iri).path, '/' as i32);
            p.is_null()
        }
    {
        (*iri).dirlen = 0 as libc::c_int as size_t;
    } else {
        (*iri)
            .dirlen = (p.offset_from((*iri).path) as libc::c_long
            + 1 as libc::c_int as libc::c_long) as size_t;
    }
    wget_vector_add(parents, iri as *const libc::c_void);
}
unsafe extern "C" fn is_parent(mut iri: *const wget_iri) -> bool {
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(parents) {
        let mut parent: *mut wget_iri = wget_vector_get(parents, it) as *mut wget_iri;
        if wget_iri_compare(parent, iri) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        it += 1;
        it;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn matches_parent(mut iri: *const wget_iri) -> bool {
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(parents) {
        let mut parent: *mut wget_iri = wget_vector_get(parents, it) as *mut wget_iri;
        if wget_strcmp((*parent).host, (*iri).host) == 0 {
            if (*parent).dirlen == 0
                || wget_strncmp((*parent).path, (*iri).path, (*parent).dirlen) == 0
            {
                return 1 as libc::c_int != 0;
            }
        }
        it += 1;
        it;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn queue_url_from_local(
    mut url: *const libc::c_char,
    mut base: *mut wget_iri,
    mut encoding: *const libc::c_char,
    mut flags: libc::c_int,
) {
    let mut iri: *mut wget_iri = 0 as *mut wget_iri;
    let mut new_job: *mut JOB = 0 as *mut JOB;
    let mut job_buf: JOB = JOB {
        iri: 0 as *const wget_iri,
        original_url: 0 as *const wget_iri,
        referer: 0 as *const wget_iri,
        metalink: 0 as *mut wget_metalink,
        challenges: 0 as *mut wget_vector,
        proxy_challenges: 0 as *mut wget_vector,
        parts: 0 as *mut wget_vector,
        remaining_sig_ext: 0 as *mut wget_list,
        host: 0 as *mut HOST,
        blacklist_entry: 0 as *const blacklist_entry,
        sig_filename: 0 as *mut libc::c_char,
        sig_req: 0 as *mut libc::c_char,
        part: 0 as *mut PART,
        downloader: 0 as *mut DOWNLOADER,
        used_by: 0,
        id: 0,
        parent_id: 0,
        retry_ts: 0,
        level: 0,
        redirection_level: 0,
        auth_failure_count: 0,
        mirror_pos: 0,
        piece_pos: 0,
        failures: 0,
        challenges_alloc_inuse_done_sitemap_robotstxt_head_first_requested_by_user_ignore_patterns_http_fallback_recursive_send_head_redirect_get: [0; 2],
        c2rust_padding: [0; 6],
    };
    let mut blacklistp: *mut blacklist_entry = 0 as *mut blacklist_entry;
    let mut host: *mut HOST = 0 as *mut HOST;
    let mut plugin_verdict: plugin_db_forward_url_verdict = plugin_db_forward_url_verdict {
        alt_iri: 0 as *mut wget_iri,
        alt_local_filename: 0 as *mut libc::c_char,
        reject_accept: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut http_fallback: bool = 0 as libc::c_int != 0;
    iri = wget_iri_parse_base(base, url, encoding);
    if iri.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to parse URI '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            url,
        );
        return;
    }
    plugin_db_forward_url(iri, &mut plugin_verdict);
    if plugin_verdict.reject() {
        wget_iri_free(&mut iri);
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        return;
    }
    if !(plugin_verdict.alt_iri).is_null() {
        wget_iri_free(&mut iri);
        iri = plugin_verdict.alt_iri;
        plugin_verdict.alt_iri = 0 as *mut wget_iri;
    }
    if !wget_iri_supported(iri) {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URI scheme not supported: '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            url,
        );
        wget_iri_free(&mut iri);
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        return;
    }
    if (*iri).scheme as libc::c_uint
        == WGET_IRI_SCHEME_HTTP as libc::c_int as libc::c_uint
    {
        test_modify_hsts(iri);
    }
    if (*iri).scheme as libc::c_uint
        == WGET_IRI_SCHEME_HTTP as libc::c_int as libc::c_uint
        && config.https_enforce as libc::c_uint != 0
    {
        wget_iri_set_scheme(iri, WGET_IRI_SCHEME_HTTPS);
        if config.https_enforce as libc::c_uint
            == HTTPS_ENFORCE_SOFT as libc::c_int as libc::c_uint
        {
            http_fallback = 1 as libc::c_int != 0;
        }
    }
    wget_thread_mutex_lock(downloader_mutex);
    blacklistp = blacklist_add(iri);
    if blacklistp.is_null() {
        if flags & (1 as libc::c_int) << 5 as libc::c_int == 0
            || {
                blacklistp = blacklist_get(iri);
                blacklistp.is_null()
            }
        {
            wget_thread_mutex_unlock(downloader_mutex);
            plugin_db_forward_url_verdict_free(&mut plugin_verdict);
            wget_iri_free(&mut iri);
            return;
        }
    }
    if config.recursive {
        if !config.span_hosts && !(config.domains).is_null() {
            if wget_vector_find(config.domains, (*iri).host as *const libc::c_void)
                < 0 as libc::c_int
            {
                wget_vector_add(
                    config.domains,
                    wget_strdup((*iri).host) as *const libc::c_void,
                );
            }
        }
        if !config.parent {
            add_parent(iri);
        }
    }
    if wget_vector_contains(config.exclude_domains, (*iri).host as *const libc::c_void) {
        wget_debug_printf(
            b"not requesting '%s'. (Exclude Domains)\n\0" as *const u8
                as *const libc::c_char,
            (*iri).safe_uri,
        );
        wget_thread_mutex_unlock(downloader_mutex);
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        return;
    }
    if !(plugin_verdict.alt_local_filename).is_null() {
        if !((*blacklistp).local_filename).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*blacklistp).local_filename as *mut libc::c_void);
            (*blacklistp).local_filename = 0 as *mut libc::c_char;
        }
        (*blacklistp).local_filename = plugin_verdict.alt_local_filename;
        plugin_verdict.alt_local_filename = 0 as *mut libc::c_char;
    }
    if !config.clobber && !((*blacklistp).local_filename).is_null()
        && access((*blacklistp).local_filename, 0 as libc::c_int) == 0 as libc::c_int
    {
        wget_debug_printf(
            b"not requesting '%s'. (Exclude Domains)\n\0" as *const u8
                as *const libc::c_char,
            (*iri).safe_uri,
        );
        wget_thread_mutex_unlock(downloader_mutex);
        if config.recursive as libc::c_int != 0
            || config.page_requisites as libc::c_int != 0
        {
            parse_localfile(
                0 as *mut JOB,
                (*blacklistp).local_filename,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                iri,
            );
        }
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        return;
    }
    host = host_add(iri);
    if !host.is_null() {
        if config.recursive {
            if !config.clobber && !((*blacklistp).local_filename).is_null()
                && access((*blacklistp).local_filename, 0 as libc::c_int)
                    == 0 as libc::c_int
            {
                wget_debug_printf(
                    b"not requesting '%s'. (Exclude Domains)\n\0" as *const u8
                        as *const libc::c_char,
                    (*iri).safe_uri,
                );
            } else {
                host_add_robotstxt_job(host, iri, encoding, http_fallback);
            }
        }
    } else {
        host = host_get(iri);
    }
    new_job = job_init(&mut job_buf, blacklistp, http_fallback);
    if plugin_verdict.accept() {
        (*new_job).set_ignore_patterns(1 as libc::c_int != 0);
    } else if config.recursive {
        if !(config.accept_patterns).is_null()
            && !in_pattern_list(config.accept_patterns, (*(*new_job).iri).uri)
            || !(config.accept_regex).is_null()
                && regex_match((*(*new_job).iri).uri, config.accept_regex) == 0
        {
            (*new_job).set_head_first(1 as libc::c_int != 0);
            (*new_job).set_recursive_send_head(1 as libc::c_int != 0);
        }
        if !(config.reject_patterns).is_null()
            && in_pattern_list(config.reject_patterns, (*(*new_job).iri).uri)
                as libc::c_int != 0
            || !(config.reject_regex).is_null()
                && regex_match((*(*new_job).iri).uri, config.reject_regex) != 0
        {
            (*new_job).set_head_first(1 as libc::c_int != 0);
            (*new_job).set_recursive_send_head(1 as libc::c_int != 0);
        }
    }
    if config.recursive {
        (*new_job).set_requested_by_user(1 as libc::c_int != 0);
    }
    if config.spider as libc::c_int != 0 || config.chunk_size != 0
        || !(config.mime_types).is_null()
        || !config.if_modified_since && config.timestamping as libc::c_int != 0
    {
        (*new_job).set_head_first(1 as libc::c_int != 0);
    }
    if config.auth_no_challenge {
        (*new_job).challenges = config.default_challenges;
        (*new_job).set_challenges_alloc(0 as libc::c_int != 0);
    }
    host_add_job(host, new_job);
    wget_thread_mutex_unlock(downloader_mutex);
    plugin_db_forward_url_verdict_free(&mut plugin_verdict);
}
unsafe extern "C" fn queue_url_from_remote(
    mut job: *mut JOB,
    mut encoding: *const libc::c_char,
    mut url: *const libc::c_char,
    mut flags: libc::c_int,
    mut download_name: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut new_job: *mut JOB = 0 as *mut JOB;
    let mut job_buf: JOB = JOB {
        iri: 0 as *const wget_iri,
        original_url: 0 as *const wget_iri,
        referer: 0 as *const wget_iri,
        metalink: 0 as *mut wget_metalink,
        challenges: 0 as *mut wget_vector,
        proxy_challenges: 0 as *mut wget_vector,
        parts: 0 as *mut wget_vector,
        remaining_sig_ext: 0 as *mut wget_list,
        host: 0 as *mut HOST,
        blacklist_entry: 0 as *const blacklist_entry,
        sig_filename: 0 as *mut libc::c_char,
        sig_req: 0 as *mut libc::c_char,
        part: 0 as *mut PART,
        downloader: 0 as *mut DOWNLOADER,
        used_by: 0,
        id: 0,
        parent_id: 0,
        retry_ts: 0,
        level: 0,
        redirection_level: 0,
        auth_failure_count: 0,
        mirror_pos: 0,
        piece_pos: 0,
        failures: 0,
        challenges_alloc_inuse_done_sitemap_robotstxt_head_first_requested_by_user_ignore_patterns_http_fallback_recursive_send_head_redirect_get: [0; 2],
        c2rust_padding: [0; 6],
    };
    let mut iri: *mut wget_iri = 0 as *mut wget_iri;
    let mut host: *mut HOST = 0 as *mut HOST;
    let mut blacklistp: *mut blacklist_entry = 0 as *mut blacklist_entry;
    let mut plugin_verdict: plugin_db_forward_url_verdict = plugin_db_forward_url_verdict {
        alt_iri: 0 as *mut wget_iri,
        alt_local_filename: 0 as *mut libc::c_char,
        reject_accept: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut http_fallback: bool = 0 as libc::c_int != 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if config.cut_url_get_vars {
        p = strchr(url, '?' as i32);
    }
    if !p.is_null() {
        let mut url_cut: *mut libc::c_char = wget_strmemdup(
            url as *const libc::c_void,
            p.offset_from(url) as libc::c_long as size_t,
        );
        iri = wget_iri_parse(url_cut, encoding);
        if !url_cut.is_null() {
            wget_free.expect("non-null function pointer")(url_cut as *mut libc::c_void);
            url_cut = 0 as *mut libc::c_char;
        }
    } else {
        iri = wget_iri_parse(url, encoding);
    }
    if iri.is_null() {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot resolve URI\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return;
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if !job.is_null() && (*job).redirection_level >= config.max_redirect {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"URL '%s' not followed (max redirections exceeded)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*iri).safe_uri,
            );
            wget_iri_free(&mut iri);
            return;
        }
    }
    plugin_db_forward_url(iri, &mut plugin_verdict);
    if plugin_verdict.reject() {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URL '%s' no followed (plugin verdict)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*iri).safe_uri,
        );
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        wget_iri_free(&mut iri);
        return;
    }
    if !(plugin_verdict.alt_iri).is_null() {
        wget_debug_printf(
            b"Plugin changed IRI. %s -> %s\n\0" as *const u8 as *const libc::c_char,
            (*iri).safe_uri,
            (*plugin_verdict.alt_iri).uri,
        );
        wget_iri_free(&mut iri);
        iri = plugin_verdict.alt_iri;
        plugin_verdict.alt_iri = 0 as *mut wget_iri;
    }
    if !wget_iri_supported(iri) {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URL '%s' not followed (unsupported scheme)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*iri).safe_uri,
        );
        wget_iri_free(&mut iri);
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        return;
    }
    if (*iri).scheme as libc::c_uint
        == WGET_IRI_SCHEME_HTTP as libc::c_int as libc::c_uint
    {
        test_modify_hsts(iri);
    }
    if config.https_only as libc::c_int != 0
        && (*iri).scheme as libc::c_uint
            != WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
    {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URL '%s' not followed (https-only requested)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*iri).safe_uri,
        );
        wget_iri_free(&mut iri);
        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
        return;
    }
    if (*iri).scheme as libc::c_uint
        == WGET_IRI_SCHEME_HTTP as libc::c_int as libc::c_uint
        && config.https_enforce as libc::c_uint != 0
        && flags & (1 as libc::c_int) << 2 as libc::c_int == 0
    {
        wget_iri_set_scheme(iri, WGET_IRI_SCHEME_HTTPS);
        if config.https_enforce as libc::c_uint
            == HTTPS_ENFORCE_SOFT as libc::c_int as libc::c_uint
        {
            http_fallback = 1 as libc::c_int != 0;
        }
    }
    if config.recursive {
        let mut reason: *const libc::c_char = 0 as *const libc::c_char;
        if ((*iri).host).is_null() {
            reason = dcgettext(
                0 as *const libc::c_char,
                b"missing ip/host/domain\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            );
        } else if !job.is_null() && strcmp((*(*job).iri).host, (*iri).host) != 0 {
            if !config.span_hosts
                && in_host_pattern_list(config.domains, (*iri).host) == 0
            {
                reason = dcgettext(
                    0 as *const libc::c_char,
                    b"no host-spanning requested\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else if config.span_hosts as libc::c_int != 0
                && in_host_pattern_list(config.exclude_domains, (*iri).host) != 0
            {
                reason = dcgettext(
                    0 as *const libc::c_char,
                    b"domain explicitly excluded\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
        }
        if !reason.is_null() {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"URL '%s' not followed (%s)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*iri).safe_uri,
                reason,
            );
            wget_iri_free(&mut iri);
            plugin_db_forward_url_verdict_free(&mut plugin_verdict);
            return;
        }
    }
    wget_thread_mutex_lock(downloader_mutex);
    blacklistp = blacklist_add(iri);
    if blacklistp.is_null() {
        wget_iri_free(&mut iri);
    } else {
        if !download_name.is_null() {
            if config.download_attr as libc::c_int
                == DOWNLOAD_ATTR_STRIPPATH as libc::c_int
                && !(strchr(download_name, '/' as i32)).is_null()
            {
                download_name = last_component(download_name);
            }
            wget_debug_printf(
                b"Change local file name. %s -> %s\n\0" as *const u8
                    as *const libc::c_char,
                (*blacklistp).local_filename,
                download_name,
            );
            let mut basename: *const libc::c_char = last_component(
                (*blacklistp).local_filename,
            );
            let mut oldlen: size_t = strlen((*blacklistp).local_filename);
            let mut dirlen: size_t = basename.offset_from((*blacklistp).local_filename)
                as libc::c_long as size_t;
            let mut len: size_t = strlen(download_name);
            if dirlen.wrapping_add(len) > oldlen {
                (*blacklistp)
                    .local_filename = wget_realloc(
                    (*blacklistp).local_filename as *mut libc::c_void,
                    dirlen.wrapping_add(len).wrapping_add(1 as libc::c_int as size_t),
                ) as *mut libc::c_char;
            }
            memcpy(
                ((*blacklistp).local_filename).offset(dirlen as isize)
                    as *mut libc::c_void,
                download_name as *const libc::c_void,
                len.wrapping_add(1 as libc::c_int as size_t),
            );
        }
        if config.recursive as libc::c_int != 0 && !config.parent
            && flags & (1 as libc::c_int) << 3 as libc::c_int == 0
        {
            if !job.is_null() && flags & (1 as libc::c_int) << 0 as libc::c_int != 0
                && is_parent((*job).iri) as libc::c_int != 0
            {
                add_parent(iri);
                current_block = 777662472977924419;
            } else if !matches_parent(iri) {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"URL '%s' not followed (parent ascending not allowed)\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    url,
                );
                current_block = 649690548724771692;
            } else {
                current_block = 777662472977924419;
            }
        } else {
            current_block = 777662472977924419;
        }
        match current_block {
            649690548724771692 => {}
            _ => {
                if (config.output_document).is_null() {
                    if !(plugin_verdict.alt_local_filename).is_null() {
                        if !((*blacklistp).local_filename).is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )((*blacklistp).local_filename as *mut libc::c_void);
                            (*blacklistp).local_filename = 0 as *mut libc::c_char;
                        }
                        (*blacklistp).local_filename = plugin_verdict.alt_local_filename;
                        plugin_verdict.alt_local_filename = 0 as *mut libc::c_char;
                    } else if !(flags & (1 as libc::c_int) << 0 as libc::c_int == 0
                        || config.trust_server_names as libc::c_int != 0
                        || job.is_null())
                    {
                        if !((*blacklistp).local_filename).is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )((*blacklistp).local_filename as *mut libc::c_void);
                            (*blacklistp).local_filename = 0 as *mut libc::c_char;
                        }
                        (*blacklistp)
                            .local_filename = wget_strdup(
                            (*(*job).blacklist_entry).local_filename,
                        );
                    }
                    if !config.clobber && !((*blacklistp).local_filename).is_null()
                        && access((*blacklistp).local_filename, 0 as libc::c_int)
                            == 0 as libc::c_int
                    {
                        wget_info_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"URL '%s' not followed (file already exists)\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*iri).safe_uri,
                        );
                        wget_thread_mutex_unlock(downloader_mutex);
                        if config.recursive as libc::c_int != 0
                            && (config.level == 0 || job.is_null()
                                || !job.is_null()
                                    && (*job).level
                                        < config.level + config.page_requisites as libc::c_int)
                        {
                            parse_localfile(
                                job,
                                (*blacklistp).local_filename,
                                encoding,
                                0 as *const libc::c_char,
                                iri,
                            );
                        }
                        plugin_db_forward_url_verdict_free(&mut plugin_verdict);
                        return;
                    }
                }
                host = host_add(iri);
                if !host.is_null() {
                    if config.recursive {
                        if !config.clobber && !((*blacklistp).local_filename).is_null()
                            && access((*blacklistp).local_filename, 0 as libc::c_int)
                                == 0 as libc::c_int
                        {
                            wget_info_printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"URL '%s' not followed (file already exists)\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*iri).safe_uri,
                            );
                        } else {
                            host_add_robotstxt_job(host, iri, encoding, http_fallback);
                        }
                    }
                    current_block = 16972322153429435017;
                } else {
                    host = host_get(iri);
                    if !host.is_null() {
                        if !((*host).robots).is_null() && !((*iri).path).is_null()
                            && config.robots as libc::c_int != 0
                        {
                            let mut it: libc::c_int = 0 as libc::c_int;
                            let mut n: libc::c_int = wget_robots_get_path_count(
                                (*host).robots,
                            );
                            loop {
                                if !(it < n) {
                                    current_block = 16972322153429435017;
                                    break;
                                }
                                let mut path: *mut wget_string = wget_robots_get_path(
                                    (*host).robots,
                                    it,
                                );
                                if (*path).len != 0
                                    && strncmp(
                                        ((*path).p).offset(1 as libc::c_int as isize),
                                        (*iri).path,
                                        ((*path).len).wrapping_sub(1 as libc::c_int as size_t),
                                    ) == 0
                                {
                                    wget_info_printf(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"URL '%s' not followed (disallowed by robots.txt)\n\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                        (*iri).safe_uri,
                                    );
                                    current_block = 649690548724771692;
                                    break;
                                } else {
                                    it += 1;
                                    it;
                                }
                            }
                        } else {
                            current_block = 16972322153429435017;
                        }
                    } else {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to get '%s' from hosts\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            (*iri).host,
                        );
                        current_block = 649690548724771692;
                    }
                }
                match current_block {
                    649690548724771692 => {}
                    _ => {
                        if config.recursive as libc::c_int != 0
                            && config.filter_urls as libc::c_int != 0
                        {
                            if !(config.accept_patterns).is_null()
                                && !in_pattern_list(config.accept_patterns, (*iri).uri)
                                || !(config.accept_regex).is_null()
                                    && regex_match((*iri).uri, config.accept_regex) == 0
                            {
                                wget_info_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"URL '%s' not followed (doesn't match accept pattern)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*iri).safe_uri,
                                );
                                current_block = 649690548724771692;
                            } else if !(config.reject_patterns).is_null()
                                && in_pattern_list(config.reject_patterns, (*iri).uri)
                                    as libc::c_int != 0
                                || !(config.reject_regex).is_null()
                                    && regex_match((*iri).uri, config.reject_regex) != 0
                            {
                                wget_info_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"URL '%s' not followed (matches reject pattern)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*iri).safe_uri,
                                );
                                current_block = 649690548724771692;
                            } else if !(config.exclude_directories).is_null()
                                && in_directory_pattern_list(
                                    config.exclude_directories,
                                    (*iri).path,
                                ) != 0
                            {
                                wget_info_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"URL '%s' not followed (path excluded)\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*iri).safe_uri,
                                );
                                current_block = 649690548724771692;
                            } else {
                                current_block = 10248984122780841972;
                            }
                        } else {
                            current_block = 10248984122780841972;
                        }
                        match current_block {
                            649690548724771692 => {}
                            _ => {
                                wget_info_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Enqueue %s\n\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*iri).safe_uri,
                                );
                                new_job = job_init(&mut job_buf, blacklistp, http_fallback);
                                if !job.is_null() {
                                    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                                        (*new_job).parent_id = (*job).parent_id;
                                        (*new_job).level = (*job).level;
                                        (*new_job)
                                            .redirection_level = (*job).redirection_level
                                            + 1 as libc::c_int;
                                        (*new_job).referer = (*job).referer;
                                        (*new_job).original_url = (*job).iri;
                                        (*new_job).set_redirect_get((*job).redirect_get());
                                        (*new_job).set_robotstxt((*job).robotstxt());
                                        (*new_job)
                                            .set_requested_by_user((*job).requested_by_user());
                                    } else {
                                        (*new_job).parent_id = (*job).id;
                                        (*new_job).level = (*job).level + 1 as libc::c_int;
                                        (*new_job).referer = (*job).iri;
                                        if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                                            if !((*job).sig_req).is_null() {
                                                (*new_job).sig_req = wget_strdup((*job).sig_req);
                                                (*new_job).level = (*job).level;
                                            } else {
                                                (*new_job).sig_req = wget_strdup((*(*job).iri).uri);
                                            }
                                            (*new_job).sig_filename = wget_strdup((*job).sig_filename);
                                            if !((*job).remaining_sig_ext).is_null() {
                                                (*new_job).remaining_sig_ext = (*job).remaining_sig_ext;
                                                (*job).remaining_sig_ext = 0 as *mut wget_list;
                                            }
                                        }
                                    }
                                }
                                if plugin_verdict.accept() {
                                    (*new_job).set_ignore_patterns(1 as libc::c_int != 0);
                                } else if config.recursive {
                                    if !(config.accept_patterns).is_null()
                                        && !in_pattern_list(
                                            config.accept_patterns,
                                            (*(*new_job).iri).uri,
                                        )
                                        || !(config.accept_regex).is_null()
                                            && regex_match((*(*new_job).iri).uri, config.accept_regex)
                                                == 0
                                    {
                                        (*new_job).set_head_first(1 as libc::c_int != 0);
                                        (*new_job).set_recursive_send_head(1 as libc::c_int != 0);
                                    }
                                    if !(config.reject_patterns).is_null()
                                        && in_pattern_list(
                                            config.reject_patterns,
                                            (*(*new_job).iri).uri,
                                        ) as libc::c_int != 0
                                        || !(config.reject_regex).is_null()
                                            && regex_match((*(*new_job).iri).uri, config.reject_regex)
                                                != 0
                                    {
                                        (*new_job).set_head_first(1 as libc::c_int != 0);
                                        (*new_job).set_recursive_send_head(1 as libc::c_int != 0);
                                    }
                                    if !(config.exclude_directories).is_null()
                                        && in_directory_pattern_list(
                                            config.exclude_directories,
                                            (*(*new_job).iri).path,
                                        ) != 0
                                    {
                                        (*new_job).set_head_first(1 as libc::c_int != 0);
                                        (*new_job).set_recursive_send_head(1 as libc::c_int != 0);
                                    }
                                }
                                if config.spider as libc::c_int != 0
                                    || config.chunk_size != 0 || !(config.mime_types).is_null()
                                    || !config.if_modified_since
                                        && config.timestamping as libc::c_int != 0
                                {
                                    (*new_job).set_head_first(1 as libc::c_int != 0);
                                }
                                if config.auth_no_challenge {
                                    (*new_job).challenges = config.default_challenges;
                                }
                                if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                                    (*new_job).set_sitemap(1 as libc::c_int != 0);
                                }
                                host_add_job(host, new_job);
                                wget_thread_cond_signal(worker_cond);
                            }
                        }
                    }
                }
            }
        }
    }
    wget_thread_mutex_unlock(downloader_mutex);
    plugin_db_forward_url_verdict_free(&mut plugin_verdict);
}
unsafe extern "C" fn convert_link_file_only(
    mut filename: *const libc::c_char,
    mut url: *mut wget_string,
    mut buf: *mut wget_buffer,
) {
    if !filename.is_null() && access(filename, 2 as libc::c_int) == 0 as libc::c_int {
        let mut linkname: *const libc::c_char = (*url).p;
        let mut link_basename: *const libc::c_char = 0 as *const libc::c_char;
        let mut p1: *const libc::c_char = 0 as *const libc::c_char;
        let mut p2: *const libc::c_char = 0 as *const libc::c_char;
        let mut localname: *const libc::c_char = filename;
        let mut local_basename: *const libc::c_char = 0 as *const libc::c_char;
        p1 = linkname;
        link_basename = p1;
        while *p1 as libc::c_int != '"' as i32 {
            if *p1 as libc::c_int == '/' as i32 {
                link_basename = p1.offset(1 as libc::c_int as isize);
            }
            p1 = p1.offset(1);
            p1;
        }
        p2 = localname;
        local_basename = p2;
        while *p2 != 0 {
            if *p2 as libc::c_int == '/' as i32 {
                local_basename = p2.offset(1 as libc::c_int as isize);
            }
            p2 = p2.offset(1);
            p2;
        }
        wget_buffer_memcpy(
            buf,
            linkname as *const libc::c_void,
            link_basename.offset_from(linkname) as libc::c_long as size_t,
        );
        wget_buffer_strcat(buf, local_basename);
        wget_debug_printf(
            b"  %.*s -> %s\n\0" as *const u8 as *const libc::c_char,
            (*url).len as libc::c_int,
            linkname,
            localname,
        );
        wget_debug_printf(
            b"       -> %s\n\0" as *const u8 as *const libc::c_char,
            (*buf).data,
        );
    } else {
        wget_buffer_memcpy(buf, (*url).p as *const libc::c_void, (*url).len);
        wget_debug_printf(
            b"  %.*s -> %s\n\0" as *const u8 as *const libc::c_char,
            (*url).len as libc::c_int,
            (*url).p,
            (*buf).data,
        );
    };
}
unsafe extern "C" fn convert_link_whole(
    mut filename: *const libc::c_char,
    mut conversion: *mut conversion_t,
    mut url: *mut wget_string,
    mut buf: *mut wget_buffer,
) {
    if !filename.is_null() && access(filename, 2 as libc::c_int) == 0 as libc::c_int {
        let mut linkpath: *const libc::c_char = filename;
        let mut dir: *const libc::c_char = 0 as *const libc::c_char;
        let mut p1: *const libc::c_char = 0 as *const libc::c_char;
        let mut p2: *const libc::c_char = 0 as *const libc::c_char;
        let mut docpath: *const libc::c_char = (*conversion).filename;
        p1 = linkpath;
        dir = p1;
        p2 = docpath;
        while *p1 as libc::c_int != 0 && *p1 as libc::c_int == *p2 as libc::c_int {
            if *p1 as libc::c_int == '/' as i32 {
                dir = p1.offset(1 as libc::c_int as isize);
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
        }
        wget_buffer_reset(buf);
        while *p2 != 0 {
            let fresh2 = p2;
            p2 = p2.offset(1);
            if *fresh2 as libc::c_int == '/' as i32 {
                wget_buffer_memcat(
                    buf,
                    b"../\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    3 as libc::c_int as size_t,
                );
            }
        }
        wget_iri_escape_path(dir, buf);
        wget_debug_printf(
            b"  %.*s -> %s\n\0" as *const u8 as *const libc::c_char,
            (*url).len as libc::c_int,
            (*url).p,
            linkpath,
        );
        wget_debug_printf(
            b"       -> %s\n\0" as *const u8 as *const libc::c_char,
            (*buf).data,
        );
    } else {
        wget_debug_printf(
            b"  %.*s -> %s\n\0" as *const u8 as *const libc::c_char,
            (*url).len as libc::c_int,
            (*url).p,
            (*buf).data,
        );
    };
}
unsafe extern "C" fn convert_links() {
    let mut fpout: *mut FILE = 0 as *mut FILE;
    let mut conversion: *mut conversion_t = 0 as *mut conversion_t;
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    let mut free_iri: bool = 0 as libc::c_int != 0;
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    let mut iter: *mut wget_stringmap_iterator = wget_hashmap_iterator_alloc(
        conversions,
    );
    while !(wget_stringmap_iterator_next(
        iter,
        &mut conversion as *mut *mut conversion_t as *mut *mut libc::c_void,
    ))
        .is_null()
    {
        let mut data: *const libc::c_char = 0 as *const libc::c_char;
        let mut data_ptr: *const libc::c_char = 0 as *const libc::c_char;
        let mut data_length: size_t = 0;
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"convert %s %s %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*conversion).filename,
            (*(*conversion).base).uri,
            (*conversion).encoding,
        );
        data_ptr = wget_read_file((*conversion).filename, &mut data_length);
        data = data_ptr;
        if data.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"%s not found (%d)\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*conversion).filename,
                *__errno_location(),
            );
        } else {
            let mut it2: libc::c_int = 0 as libc::c_int;
            while it2 < wget_vector_size((*(*conversion).parsed).uris) {
                let mut html_url: *mut wget_html_parsed_url = wget_vector_get(
                    (*(*conversion).parsed).uris,
                    it2,
                ) as *mut wget_html_parsed_url;
                let mut url: *mut wget_string = &mut (*html_url).url;
                (*url).p = data.offset((*url).p as size_t as isize);
                if !((*url).len >= 1 as libc::c_int as size_t
                    && *(*url).p as libc::c_int == '#' as i32)
                {
                    if !(wget_iri_relative_to_abs(
                        (*conversion).base,
                        (*url).p,
                        (*url).len,
                        &mut buf,
                    ))
                        .is_null()
                    {
                        let mut iri: *mut wget_iri = wget_iri_parse(
                            buf.data,
                            (*conversion).encoding,
                        );
                        let mut blacklist_entry: *mut blacklist_entry = 0
                            as *mut blacklist_entry;
                        free_iri = 0 as libc::c_int != 0;
                        if iri.is_null() {
                            wget_info_printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Cannot resolve URI '%s'\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                buf.data,
                            );
                        } else {
                            blacklist_entry = blacklist_add(iri);
                            if blacklist_entry.is_null() {
                                blacklist_entry = blacklist_get(iri);
                                free_iri = 1 as libc::c_int != 0;
                            }
                            let mut filename: *const libc::c_char = (*blacklist_entry)
                                .local_filename;
                            if config.convert_links {
                                convert_link_whole(filename, conversion, url, &mut buf);
                                if !((*iri).fragment).is_null() {
                                    wget_buffer_memcat(
                                        &mut buf,
                                        b"#\0" as *const u8 as *const libc::c_char
                                            as *const libc::c_void,
                                        1 as libc::c_int as size_t,
                                    );
                                    wget_buffer_strcat(&mut buf, (*iri).fragment);
                                }
                            } else if config.convert_file_only {
                                convert_link_file_only(filename, url, &mut buf);
                            }
                            if free_iri {
                                wget_iri_free(&mut iri);
                            }
                            if buf.length != (*url).len
                                || strncmp(buf.data, (*url).p, (*url).len) != 0
                            {
                                if fpout.is_null() {
                                    if config.backup_converted {
                                        let mut dstfile: *mut libc::c_char = wget_aprintf(
                                            b"%s.orig\0" as *const u8 as *const libc::c_char,
                                            (*conversion).filename,
                                        );
                                        if rename((*conversion).filename, dstfile)
                                            == -(1 as libc::c_int)
                                        {
                                            wget_error_printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Failed to rename %s to %s (%d)\0" as *const u8
                                                        as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                                (*conversion).filename,
                                                dstfile,
                                                *__errno_location(),
                                            );
                                        }
                                        if !dstfile.is_null() {
                                            wget_free
                                                .expect(
                                                    "non-null function pointer",
                                                )(dstfile as *mut libc::c_void);
                                            dstfile = 0 as *mut libc::c_char;
                                        }
                                    }
                                    fpout = rpl_fopen(
                                        (*conversion).filename,
                                        b"wb\0" as *const u8 as *const libc::c_char,
                                    );
                                    if fpout.is_null() {
                                        wget_error_printf(
                                            dcgettext(
                                                0 as *const libc::c_char,
                                                b"Failed to write open %s (%d)\0" as *const u8
                                                    as *const libc::c_char,
                                                5 as libc::c_int,
                                            ),
                                            (*conversion).filename,
                                            *__errno_location(),
                                        );
                                    }
                                }
                                if !fpout.is_null() {
                                    fwrite(
                                        data_ptr as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                        ((*url).p).offset_from(data_ptr) as libc::c_long
                                            as libc::c_ulong,
                                        fpout,
                                    );
                                    fwrite(
                                        buf.data as *const libc::c_void,
                                        1 as libc::c_int as libc::c_ulong,
                                        buf.length,
                                        fpout,
                                    );
                                    data_ptr = ((*url).p).offset((*url).len as isize);
                                }
                            }
                        }
                    }
                }
                it2 += 1;
                it2;
            }
            if !fpout.is_null() {
                fwrite(
                    data_ptr as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    data.offset(data_length as isize).offset_from(data_ptr)
                        as libc::c_long as libc::c_ulong,
                    fpout,
                );
                rpl_fclose(fpout);
                fpout = 0 as *mut FILE;
            }
            if !data.is_null() {
                wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
                data = 0 as *const libc::c_char;
            }
        }
    }
    wget_hashmap_iterator_free(&mut iter);
    wget_buffer_deinit(&mut buf);
}
unsafe extern "C" fn print_status(
    mut downloader: *mut DOWNLOADER,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if config.verbose {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        wget_info_vprintf(fmt, args_0.as_va_list());
    }
}
unsafe extern "C" fn print_progress_report(mut start_time: libc::c_longlong) {
    if config.progress as libc::c_int == 1 as libc::c_int {
        let mut quota_buf: [libc::c_char; 16] = [0; 16];
        let mut speed_buf: [libc::c_char; 16] = [0; 16];
        let mut rs_type: libc::c_char = (if config.report_speed as libc::c_uint
            == WGET_REPORT_SPEED_BYTES as libc::c_int as libc::c_uint
        {
            'B' as i32
        } else {
            'b' as i32
        }) as libc::c_char;
        let mut tdiff: libc::c_longlong = wget_get_timemillis() - start_time;
        if tdiff == 0 {
            tdiff = 1 as libc::c_int as libc::c_longlong;
        }
        let mut mod_0: libc::c_uint = (1000 as libc::c_int
            * (if config.report_speed as libc::c_uint
                == WGET_REPORT_SPEED_BYTES as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                8 as libc::c_int
            })) as libc::c_uint;
        if config.spider {
            bar_printf(
                nthreads,
                b"Headers: %d (%d redirects & %d errors) Bytes: %s [%s%c/s] Todo: %d\0"
                    as *const u8 as *const libc::c_char,
                stats.nerrors + stats.ndownloads + stats.nredirects + stats.nnotmodified,
                stats.nredirects,
                stats.nerrors,
                wget_human_readable(
                    quota_buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    quota as uint64_t,
                ),
                wget_human_readable(
                    speed_buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    (quota * mod_0 as libc::c_longlong / tdiff) as uint64_t,
                ),
                rs_type as libc::c_int,
                queue_size(),
            );
        } else {
            bar_printf(
                nthreads,
                b"Files: %d  Bytes: %s [%s%c/s] Redirects: %d  Todo: %d  Errors: %d\0"
                    as *const u8 as *const libc::c_char,
                stats.ndownloads,
                wget_human_readable(
                    quota_buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    quota as uint64_t,
                ),
                wget_human_readable(
                    speed_buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                    (quota * mod_0 as libc::c_longlong / tdiff) as uint64_t,
                ),
                rs_type as libc::c_int,
                stats.nredirects,
                queue_size(),
                stats.nerrors,
            );
        }
    }
}
unsafe extern "C" fn is_tty() -> bool {
    return isatty(1 as libc::c_int) == 1 as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *const libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut quota_buf: [libc::c_char; 16] = [0; 16];
    let mut start_time: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    program_init();
    set_exit_status(EXIT_STATUS_PARSE_INIT);
    n = init(argc, argv);
    if !(n < 0 as libc::c_int) {
        set_exit_status(EXIT_STATUS_NO_ERROR);
        while n < argc {
            queue_url_from_local(
                *argv.offset(n as isize),
                config.base,
                config.local_encoding,
                0 as libc::c_int,
            );
            n += 1;
            n;
        }
        if !(config.input_file).is_null() {
            if config.force_html {
                html_parse_localfile(
                    0 as *mut JOB,
                    0 as libc::c_int,
                    config.input_file,
                    config.input_encoding,
                    config.base,
                );
            } else if config.force_css {
                css_parse_localfile(
                    0 as *mut JOB,
                    config.input_file,
                    config.input_encoding,
                    config.base,
                );
            } else if config.force_sitemap {
                sitemap_parse_xml_localfile(
                    0 as *mut JOB,
                    config.input_file,
                    b"utf-8\0" as *const u8 as *const libc::c_char,
                    config.base,
                );
            } else if config.force_atom {
                atom_parse_localfile(
                    0 as *mut JOB,
                    config.input_file,
                    b"utf-8\0" as *const u8 as *const libc::c_char,
                    config.base,
                );
            } else if config.force_rss {
                rss_parse_localfile(
                    0 as *mut JOB,
                    config.input_file,
                    b"utf-8\0" as *const u8 as *const libc::c_char,
                    config.base,
                );
            } else if config.force_metalink {
                metalink_parse_localfile(config.input_file);
            } else if strcmp(
                config.input_file,
                b"-\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if isatty(0 as libc::c_int) != 0 {
                    let mut len: ssize_t = 0;
                    let mut bufsize: size_t = 0 as libc::c_int as size_t;
                    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
                    loop {
                        len = wget_fdgetline(&mut buf, &mut bufsize, 0 as libc::c_int);
                        if !(len >= 0 as libc::c_int as ssize_t) {
                            break;
                        }
                        url = buf;
                        while len != 0
                            && *(*__ctype_b_loc()).offset(*url as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            url = url.offset(1);
                            url;
                            len -= 1;
                            len;
                        }
                        if *url as libc::c_int == '#' as i32
                            || len <= 0 as libc::c_int as ssize_t
                        {
                            continue;
                        }
                        while len != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *url.offset((len - 1 as libc::c_int as ssize_t) as isize)
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            len -= 1;
                            len;
                        }
                        *url.offset(len as isize) = 0 as libc::c_int as libc::c_char;
                        queue_url_from_local(
                            buf,
                            config.base,
                            config.input_encoding,
                            0 as libc::c_int,
                        );
                    }
                    if !buf.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(buf as *mut libc::c_void);
                        buf = 0 as *mut libc::c_char;
                    }
                } else {
                    rc = wget_thread_start(
                        &mut input_tid,
                        Some(
                            input_thread
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                ) -> *mut libc::c_void,
                        ),
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    if rc != 0 as libc::c_int {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to start downloader, error %d\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            rc,
                        );
                    }
                }
            } else {
                let mut fd: libc::c_int = 0;
                let mut len_0: ssize_t = 0;
                let mut bufsize_0: size_t = 0 as libc::c_int as size_t;
                let mut url_0: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut buf_0: *mut libc::c_char = 0 as *mut libc::c_char;
                fd = open(config.input_file, 0 as libc::c_int | 0 as libc::c_int);
                if fd >= 0 as libc::c_int {
                    loop {
                        len_0 = wget_fdgetline(&mut buf_0, &mut bufsize_0, fd);
                        if !(len_0 >= 0 as libc::c_int as ssize_t) {
                            break;
                        }
                        url_0 = buf_0;
                        while len_0 != 0
                            && *(*__ctype_b_loc()).offset(*url_0 as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            url_0 = url_0.offset(1);
                            url_0;
                            len_0 -= 1;
                            len_0;
                        }
                        if *url_0 as libc::c_int == '#' as i32
                            || len_0 <= 0 as libc::c_int as ssize_t
                        {
                            continue;
                        }
                        while len_0 != 0
                            && *(*__ctype_b_loc())
                                .offset(
                                    *url_0
                                        .offset((len_0 - 1 as libc::c_int as ssize_t) as isize)
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                        {
                            len_0 -= 1;
                            len_0;
                        }
                        *url_0.offset(len_0 as isize) = 0 as libc::c_int as libc::c_char;
                        queue_url_from_local(
                            url_0,
                            config.base,
                            config.input_encoding,
                            0 as libc::c_int,
                        );
                    }
                    if !buf_0.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(buf_0 as *mut libc::c_void);
                        buf_0 = 0 as *mut libc::c_char;
                    }
                    close(fd);
                } else {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to open input file %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        config.input_file,
                    );
                }
            }
        }
        if queue_size() == 0 as libc::c_int && input_tid.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Nothing to do - goodbye\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            if config.background {
                fork_to_background();
            }
            if !wget_thread_support() {
                config.max_threads = 1 as libc::c_int;
                if config.progress as libc::c_int != 0 as libc::c_int {
                    config.progress = 0 as libc::c_int as libc::c_char;
                    wget_info_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Wget2 built without thread support. Disabling progress report\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
            }
            if config.quiet as libc::c_int != 0 || !config.verbose
                || config.debug as libc::c_int != 0
            {
                if !config.force_progress {
                    config.progress = 0 as libc::c_int as libc::c_char;
                }
            }
            if config.progress as libc::c_int != 0 as libc::c_int && !is_tty()
                && !config.force_progress
            {
                config.progress = 0 as libc::c_int as libc::c_char;
            }
            if config.progress as libc::c_int == 1 as libc::c_int {
                if bar_init() {
                    start_time = wget_get_timemillis();
                    rc = wget_thread_start(
                        &mut progress_thread,
                        Some(
                            progress_report
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                ) -> *mut libc::c_void,
                        ),
                        0 as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    if rc != 0 as libc::c_int {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to start progress report thread, error %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            rc,
                        );
                    }
                }
            }
            downloaders = wget_calloc(
                config.max_threads as size_t,
                ::core::mem::size_of::<DOWNLOADER>() as libc::c_ulong,
            ) as *mut DOWNLOADER;
            wget_thread_mutex_lock(main_mutex);
            while !terminate {
                if queue_empty() != 0 && input_tid.is_null() {
                    break;
                }
                while nthreads < config.max_threads && nthreads < queue_size() {
                    (*downloaders.offset(nthreads as isize)).id = nthreads;
                    if config.progress as libc::c_int == 1 as libc::c_int {
                        bar_update_slots(nthreads + 1 as libc::c_int + 1 as libc::c_int);
                    }
                    rc = wget_thread_start(
                        &mut (*downloaders.offset(nthreads as isize)).thread,
                        Some(
                            downloader_thread
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                ) -> *mut libc::c_void,
                        ),
                        &mut *downloaders.offset(nthreads as isize) as *mut DOWNLOADER
                            as *mut libc::c_void,
                        0 as libc::c_int,
                    );
                    if rc != 0 as libc::c_int {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Failed to start downloader, error %d\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            rc,
                        );
                    }
                    nthreads += 1;
                    nthreads;
                }
                print_progress_report(start_time);
                if config.quota != 0 && quota >= config.quota {
                    wget_info_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Quota of %lld bytes reached - stopping.\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        config.quota,
                    );
                    break;
                } else {
                    wget_thread_cond_wait(
                        main_cond,
                        main_mutex,
                        0 as libc::c_int as libc::c_longlong,
                    );
                    wget_debug_printf(
                        b"%s: wake up\n\0" as *const u8 as *const libc::c_char,
                        (*::core::mem::transmute::<
                            &[u8; 5],
                            &[libc::c_char; 5],
                        >(b"main\0"))
                            .as_ptr(),
                    );
                }
            }
            wget_debug_printf(
                b"%s: done\n\0" as *const u8 as *const libc::c_char,
                (*::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"main\0"))
                    .as_ptr(),
            );
            ::core::ptr::write_volatile(
                &mut terminate as *mut bool,
                1 as libc::c_int != 0,
            );
            wget_thread_cond_signal(worker_cond);
            wget_thread_mutex_unlock(main_mutex);
            n = 0 as libc::c_int;
            while n < nthreads {
                rc = wget_thread_join(&mut (*downloaders.offset(n as isize)).thread);
                if rc != 0 as libc::c_int {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to wait for downloader #%d (%d %d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        n,
                        rc,
                        *__errno_location(),
                    );
                }
                n += 1;
                n;
            }
            print_progress_report(start_time);
            if config.progress as libc::c_int == 0 as libc::c_int
                && (config.recursive as libc::c_int != 0
                    || config.page_requisites as libc::c_int != 0
                    || !(config.input_file).is_null()
                        && quota != 0 as libc::c_int as libc::c_longlong) && quota != 0
            {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Downloaded: %d files, %s bytes, %d redirects, %d errors\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    stats.ndownloads,
                    wget_human_readable(
                        quota_buf.as_mut_ptr(),
                        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
                        quota as uint64_t,
                    ),
                    stats.nredirects,
                    stats.nerrors,
                );
            }
            if !(config.save_cookies).is_null() {
                wget_cookie_db_save(config.cookie_db, config.save_cookies);
            }
            if config.hsts as libc::c_int != 0 && !(config.hsts_file).is_null()
                && hsts_changed != 0
            {
                wget_hsts_db_save(config.hsts_db);
            }
            if config.hpkp as libc::c_int != 0 && !(config.hpkp_file).is_null()
                && hpkp_changed != 0
            {
                wget_hpkp_db_save(config.hpkp_db);
            }
            if config.tls_resume as libc::c_int != 0
                && !(config.tls_session_file).is_null()
                && wget_tls_session_db_changed(config.tls_session_db) != 0
            {
                wget_tls_session_db_save(config.tls_session_db, config.tls_session_file);
            }
            if config.ocsp as libc::c_int != 0 && !(config.ocsp_file).is_null() {
                wget_ocsp_db_save(config.ocsp_db);
            }
            if config.delete_after as libc::c_int != 0
                && !(config.output_document).is_null()
            {
                unlink(config.output_document);
            }
            if config.debug {
                blacklist_print();
            }
            if config.convert_links as libc::c_int != 0
                && config.convert_file_only as libc::c_int != 0
            {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"--convert-links and --convert-file-only cannot be used together, error\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                set_exit_status(EXIT_STATUS_PARSE_INIT);
            } else {
                if (config.convert_links as libc::c_int != 0
                    || config.convert_file_only as libc::c_int != 0)
                    && !config.delete_after
                {
                    convert_links();
                    wget_hashmap_free(&mut conversions);
                }
                if !(config.stats_site_args).is_null() {
                    site_stats_print();
                }
            }
        }
    }
    if is_testing() as libc::c_int != 0
        || wget_match_tail(
            *argv.offset(0 as libc::c_int as isize),
            b"wget2_noinstall\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        rc = wget_thread_join(&mut progress_thread);
        if rc != 0 as libc::c_int {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to wait for progress thread (%d %d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                rc,
                *__errno_location(),
            );
        }
        blacklist_free();
        hosts_free();
        if !downloaders.is_null() {
            wget_free
                .expect("non-null function pointer")(downloaders as *mut libc::c_void);
            downloaders = 0 as *mut DOWNLOADER;
        }
        if config.progress as libc::c_int == 1 as libc::c_int {
            bar_deinit();
        }
        wget_vector_clear_nofree(parents);
        wget_vector_free(&mut parents);
        wget_hashmap_free(&mut known_urls);
        wget_stringmap_free(&mut etags);
        deinit();
        program_deinit();
    }
    plugin_db_finalize(get_exit_status() as libc::c_int);
    return get_exit_status() as libc::c_int;
}
unsafe extern "C" fn progress_report(mut p: *mut libc::c_void) -> *mut libc::c_void {
    while !terminate {
        wget_millisleep(1000 as libc::c_int);
        wget_thread_cond_signal(main_cond);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn input_thread(mut p: *mut libc::c_void) -> *mut libc::c_void {
    let mut bufsize: size_t = 0 as libc::c_int as size_t;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    while wget_fdgetline(&mut buf, &mut bufsize, 0 as libc::c_int)
        >= 0 as libc::c_int as ssize_t
    {
        queue_url_from_local(
            buf,
            config.base,
            config.local_encoding,
            (1 as libc::c_int) << 5 as libc::c_int,
        );
        if nthreads < config.max_threads && nthreads < queue_size() {
            wget_thread_cond_signal(main_cond);
        } else {
            wget_thread_cond_signal(worker_cond);
        }
    }
    if !buf.is_null() {
        wget_free.expect("non-null function pointer")(buf as *mut libc::c_void);
        buf = 0 as *mut libc::c_char;
    }
    wget_debug_printf(b"input closed\n\0" as *const u8 as *const libc::c_char);
    wget_thread_cond_signal(main_cond);
    input_tid = 0 as wget_thread;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn try_connection(
    mut downloader: *mut DOWNLOADER,
    mut iri: *const wget_iri,
) -> libc::c_int {
    let mut conn: *mut wget_http_connection = 0 as *mut wget_http_connection;
    let mut rc: libc::c_int = 0;
    conn = (*downloader).conn;
    if !conn.is_null() {
        if wget_strcmp(wget_http_get_host(conn), (*iri).host) == 0
            && wget_http_get_scheme(conn) as libc::c_uint
                == (*iri).scheme as libc::c_uint
            && wget_http_get_port(conn) as libc::c_int == (*iri).port as libc::c_int
        {
            wget_debug_printf(
                b"reuse connection %s\n\0" as *const u8 as *const libc::c_char,
                wget_http_get_host(conn),
            );
            return WGET_E_SUCCESS as libc::c_int;
        }
        wget_debug_printf(
            b"close connection %s\n\0" as *const u8 as *const libc::c_char,
            wget_http_get_host(conn),
        );
        wget_http_close(&mut (*downloader).conn);
    }
    rc = wget_http_open(&mut (*downloader).conn, iri);
    if rc == WGET_E_SUCCESS as libc::c_int {
        wget_debug_printf(
            b"established connection %s\n\0" as *const u8 as *const libc::c_char,
            wget_http_get_host((*downloader).conn),
        );
    } else {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to connect: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            wget_strerror(rc as wget_error),
        );
    }
    return rc;
}
unsafe extern "C" fn establish_connection(
    mut downloader: *mut DOWNLOADER,
    mut iri: *mut *const wget_iri,
) -> libc::c_int {
    let mut rc: libc::c_int = WGET_E_UNKNOWN as libc::c_int;
    (*downloader).set_final_error(0 as libc::c_int != 0);
    if !((*(*downloader).job).part).is_null() {
        let mut job: *mut JOB = (*downloader).job;
        let mut metalink: *mut wget_metalink = (*job).metalink;
        let mut part: *mut PART = (*job).part;
        let mut mirror_count: libc::c_int = wget_vector_size((*metalink).mirrors);
        let mut mirror_index: libc::c_int = 0;
        if mirror_count > 0 as libc::c_int {
            mirror_index = (*downloader).id % mirror_count;
        } else {
            host_final_failure((*(*downloader).job).host);
            set_exit_status(EXIT_STATUS_NETWORK);
            return rc;
        }
        let mut tries: libc::c_int = 0 as libc::c_int;
        while tries < config.tries && !(*part).done() && !terminate {
            wget_millisleep(
                if tries * 1000 as libc::c_int > config.waitretry {
                    config.waitretry
                } else {
                    tries * 1000 as libc::c_int
                },
            );
            if terminate {
                break;
            }
            let mut current_block_15: u64;
            let mut mirrors: libc::c_int = 0 as libc::c_int;
            while mirrors < wget_vector_size((*metalink).mirrors) && !(*part).done() {
                let mut mirror: *mut wget_metalink_mirror = wget_vector_get(
                    (*metalink).mirrors,
                    mirror_index,
                ) as *mut wget_metalink_mirror;
                mirror_index = (mirror_index + 1 as libc::c_int)
                    % wget_vector_size((*metalink).mirrors);
                if !(config.https_only as libc::c_int != 0
                    && (*(*mirror).iri).scheme as libc::c_uint
                        != WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint)
                {
                    if (*(*mirror).iri).scheme as libc::c_uint
                        == WGET_IRI_SCHEME_HTTP as libc::c_int as libc::c_uint
                        && config.https_enforce as libc::c_uint != 0
                    {
                        if config.https_enforce as libc::c_uint
                            != HTTPS_ENFORCE_SOFT as libc::c_int as libc::c_uint
                        {
                            current_block_15 = 2968425633554183086;
                        } else {
                            current_block_15 = 5143058163439228106;
                        }
                    } else {
                        current_block_15 = 5143058163439228106;
                    }
                    match current_block_15 {
                        2968425633554183086 => {}
                        _ => {
                            rc = try_connection(downloader, (*mirror).iri);
                            if rc == WGET_E_SUCCESS as libc::c_int {
                                host_add((*mirror).iri);
                                if !iri.is_null() {
                                    *iri = (*mirror).iri;
                                }
                                return rc;
                            } else if rc == WGET_E_TLS_DISABLED as libc::c_int {
                                tries = config.tries;
                                break;
                            }
                        }
                    }
                }
                mirrors += 1;
                mirrors;
            }
            tries += 1;
            tries;
        }
    } else {
        rc = try_connection(downloader, *iri);
    }
    if rc == WGET_E_HANDSHAKE as libc::c_int || rc == WGET_E_CERTIFICATE as libc::c_int
        || rc == WGET_E_TLS_DISABLED as libc::c_int
    {
        wget_http_close(&mut (*downloader).conn);
        if !(*(*downloader).job).http_fallback() {
            host_final_failure((*(*downloader).job).host);
            set_exit_status(EXIT_STATUS_TLS);
        }
    } else if rc == WGET_E_CONNECT as libc::c_int {
        wget_http_close(&mut (*downloader).conn);
        if !config.retry_connrefused && !(*(*downloader).job).http_fallback() {
            host_final_failure((*(*downloader).job).host);
            set_exit_status(EXIT_STATUS_NETWORK);
        }
    }
    return rc;
}
unsafe extern "C" fn add_statistics(mut resp: *mut wget_http_response) {
    if (*resp).code as libc::c_int == 200 as libc::c_int
        || (*resp).code as libc::c_int == 416 as libc::c_int
            && (*resp).cur_downloaded == 0
    {
        let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
        if !((*job).part).is_null() {
            atomic_increment_int(&mut stats.nchunks);
        } else {
            atomic_increment_int(&mut stats.ndownloads);
        }
    } else if (*resp).code as libc::c_int == 301 as libc::c_int
        || (*resp).code as libc::c_int == 302 as libc::c_int
        || (*resp).code as libc::c_int == 303 as libc::c_int
        || (*resp).code as libc::c_int == 307 as libc::c_int
        || (*resp).code as libc::c_int == 308 as libc::c_int
    {
        atomic_increment_int(&mut stats.nredirects);
    } else if (*resp).code as libc::c_int == 304 as libc::c_int {
        atomic_increment_int(&mut stats.nnotmodified);
    } else {
        atomic_increment_int(&mut stats.nerrors);
    }
    if !(config.stats_site_args).is_null() {
        stats_site_add(resp, 0 as *mut wget_gpg_info_t);
    }
}
unsafe extern "C" fn process_response_header(
    mut resp: *mut wget_http_response,
) -> libc::c_int {
    let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
    let mut downloader: *mut DOWNLOADER = (*job).downloader;
    let mut iri: *const wget_iri = (*job).iri;
    if ((*resp).code as libc::c_int) < 400 as libc::c_int
        || (*resp).code as libc::c_int > 599 as libc::c_int
    {
        print_status(
            downloader,
            b"HTTP response %d %s [%s]\n\0" as *const u8 as *const libc::c_char,
            (*resp).code as libc::c_int,
            ((*resp).reason).as_mut_ptr(),
            (*iri).safe_uri,
        );
    } else {
        print_status(
            downloader,
            b"HTTP ERROR response %d %s [%s]\n\0" as *const u8 as *const libc::c_char,
            (*resp).code as libc::c_int,
            ((*resp).reason).as_mut_ptr(),
            (*iri).safe_uri,
        );
    }
    if (*resp).length_inconsistent() as libc::c_int != 0
        && (*resp).code as libc::c_int == 200 as libc::c_int
    {
        print_status(
            downloader,
            b"Unexpected body length %zu.\0" as *const u8 as *const libc::c_char,
            (*resp).content_length,
        );
        if config.tries != 0
            && {
                (*job).failures += 1;
                (*job).failures < config.tries
            }
        {
            if !((*(*job).blacklist_entry).local_filename).is_null() {
                wget_debug_printf(
                    b"Removing %s\n\0" as *const u8 as *const libc::c_char,
                    (*(*job).blacklist_entry).local_filename,
                );
                unlink((*(*job).blacklist_entry).local_filename);
            }
            (*job).set_done(0 as libc::c_int != 0);
            (*job)
                .retry_ts = wget_get_timemillis()
                + ((*job).failures * 1000 as libc::c_int) as libc::c_longlong;
        }
        return 1 as libc::c_int;
    }
    if (*resp).code as libc::c_int / 100 as libc::c_int == 4 as libc::c_int
        && (*resp).code as libc::c_int != 416 as libc::c_int
    {
        if (*job).head_first() {
            set_exit_status(EXIT_STATUS_REMOTE);
        } else if (*resp).code as libc::c_int == 404 as libc::c_int
            && !(*job).robotstxt()
        {
            set_exit_status(EXIT_STATUS_REMOTE);
        }
    } else if (*resp).code as libc::c_int >= 500 as libc::c_int {
        set_exit_status(EXIT_STATUS_REMOTE);
    }
    wget_debug_printf(
        b"keep_alive=%d\n\0" as *const u8 as *const libc::c_char,
        (*resp).keep_alive as libc::c_int,
    );
    if !(*resp).keep_alive {
        wget_http_close(&mut (*downloader).conn);
    }
    add_statistics(resp);
    wget_cookie_normalize_cookies((*job).iri, (*resp).cookies);
    wget_cookie_store_cookies(config.cookie_db, (*resp).cookies);
    if config.hsts as libc::c_int != 0
        && (*iri).scheme as libc::c_uint
            == WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
        && !(*iri).is_ip_address() && (*resp).hsts() as libc::c_int != 0
    {
        wget_hsts_db_add(
            config.hsts_db,
            (*iri).host,
            (*iri).port,
            (*resp).hsts_maxage,
            (*resp).hsts_include_subdomains,
        );
        hsts_changed = 1 as libc::c_int;
    }
    if config.hpkp as libc::c_int != 0
        && (*iri).scheme as libc::c_uint
            == WGET_IRI_SCHEME_HTTPS as libc::c_int as libc::c_uint
        && !(*iri).is_ip_address() && !((*resp).hpkp).is_null()
    {
        wget_hpkp_set_host((*resp).hpkp, (*iri).host);
        wget_hpkp_db_add(config.hpkp_db, &mut (*resp).hpkp);
        hpkp_changed = 1 as libc::c_int;
    }
    if (*resp).code as libc::c_int == 401 as libc::c_int {
        (*job).auth_failure_count += 1;
        (*job).auth_failure_count;
        if (*job).auth_failure_count > 1 as libc::c_int || ((*resp).challenges).is_null()
        {
            set_exit_status(EXIT_STATUS_AUTH);
            return 1 as libc::c_int;
        }
        (*job).challenges = (*resp).challenges;
        (*job).set_challenges_alloc(1 as libc::c_int != 0);
        (*resp).challenges = 0 as *mut wget_vector;
        (*job).set_done(0 as libc::c_int != 0);
        return 1 as libc::c_int;
    }
    if (*resp).code as libc::c_int == 407 as libc::c_int {
        if !((*job).proxy_challenges).is_null() || ((*resp).challenges).is_null() {
            set_exit_status(EXIT_STATUS_AUTH);
            return 1 as libc::c_int;
        }
        (*job).proxy_challenges = (*resp).challenges;
        (*resp).challenges = 0 as *mut wget_vector;
        (*job).set_done(0 as libc::c_int != 0);
        return 1 as libc::c_int;
    }
    if (*resp).code as libc::c_int == 416 as libc::c_int && (*resp).cur_downloaded == 0 {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"The file is already fully retrieved; nothing to do.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (*resp).code as libc::c_int / 100 as libc::c_int == 2 as libc::c_int
        || (*resp).code as libc::c_int / 100 as libc::c_int >= 4 as libc::c_int
        || (*resp).code as libc::c_int == 304 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if !((*resp).location).is_null() {
        if wget_strcasecmp_ascii(
            ((*(*resp).req).method).as_mut_ptr(),
            b"POST\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*resp).code as libc::c_int == 301 as libc::c_int
                || (*resp).code as libc::c_int == 302 as libc::c_int
                || (*resp).code as libc::c_int == 303 as libc::c_int
            {
                (*job).set_redirect_get(1 as libc::c_int != 0);
            }
        }
        wget_cookie_normalize_cookies((*job).iri, (*resp).cookies);
        wget_cookie_store_cookies(config.cookie_db, (*resp).cookies);
        let mut uri_buf: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        let mut uri_sbuf: [libc::c_char; 1024] = [0; 1024];
        wget_buffer_init(
            &mut uri_buf,
            uri_sbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        wget_iri_relative_to_abs(
            iri,
            (*resp).location,
            -(1 as libc::c_int) as size_t,
            &mut uri_buf,
        );
        if uri_buf.length != 0 {
            queue_url_from_remote(
                job,
                b"utf-8\0" as *const u8 as *const libc::c_char,
                uri_buf.data,
                (1 as libc::c_int) << 0 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        wget_buffer_deinit(&mut uri_buf);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn process_head_response(mut resp: *mut wget_http_response) {
    let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
    let mut local_filename: *const libc::c_char = (*(*job).blacklist_entry)
        .local_filename;
    (*job).set_head_first(0 as libc::c_int != 0);
    let mut file_size: libc::c_longlong = if !local_filename.is_null() {
        get_file_size(local_filename)
    } else {
        -(1 as libc::c_int) as libc::c_longlong
    };
    if config.timestamping as libc::c_int != 0 && !config.if_modified_since
        && (*resp).code as libc::c_int == 200 as libc::c_int
        && file_size == (*resp).content_length as libc::c_longlong
        && (*resp).last_modified <= get_file_lmtime(local_filename)
    {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"File '%s' not modified on server. Omitting download\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            local_filename,
        );
        if config.recursive as libc::c_int != 0
            && (config.level == 0
                || (*job).level < config.level + config.page_requisites as libc::c_int)
        {
            parse_localfile(
                job,
                local_filename,
                (*resp).content_type_encoding,
                (*resp).content_type,
                (*job).iri,
            );
        }
        return;
    }
    if config.spider as libc::c_int != 0
        || config.recursive as libc::c_int != 0
            && (!(config.mime_types).is_null()
                || (*job).recursive_send_head() as libc::c_int != 0)
    {
        if (*resp).code as libc::c_int != 200 as libc::c_int
            || ((*resp).content_type).is_null()
        {
            return;
        }
        if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"text/html\0" as *const u8 as *const libc::c_char,
        ) != 0
            && wget_strcasecmp_ascii(
                (*resp).content_type,
                b"text/css\0" as *const u8 as *const libc::c_char,
            ) != 0
            && wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
            ) != 0
            && wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/atom+xml\0" as *const u8 as *const libc::c_char,
            ) != 0
            && wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/rss+xml\0" as *const u8 as *const libc::c_char,
            ) != 0
            && (!(*job).sitemap()
                || wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"application/xml\0" as *const u8 as *const libc::c_char,
                ) == 0)
            && (!(*job).sitemap()
                || wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"application/x-gzip\0" as *const u8 as *const libc::c_char,
                ) == 0)
            && (!(*job).sitemap()
                || wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"text/plain\0" as *const u8 as *const libc::c_char,
                ) == 0)
            && ((config.mime_types).is_null()
                || !check_mime_list(config.mime_types, (*resp).content_type))
        {
            return;
        }
        if !((*resp).etag).is_null() {
            wget_thread_mutex_lock(etag_mutex);
            if etags.is_null() {
                etags = wget_stringmap_create(128 as libc::c_int);
            }
            let mut rc: libc::c_int = wget_stringmap_put(
                etags,
                (*resp).etag,
                0 as *const libc::c_void,
            );
            (*resp).etag = 0 as *const libc::c_char;
            wget_thread_mutex_unlock(etag_mutex);
            if rc != 0 {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Not scanning '%s' (known ETag)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*job).iri).safe_uri,
                );
                return;
            }
        }
        if config.spider as libc::c_int != 0 && !config.recursive {
            return;
        }
        (*job).set_done(0 as libc::c_int != 0);
        return;
    } else if config.chunk_size != 0
        && (*resp).content_length as libc::c_longlong > config.chunk_size
    {
        let mut piece: wget_metalink_piece = {
            let mut init = wget_metalink_piece {
                hash: wget_metalink_hash {
                    type_0: [0; 16],
                    hash_hex: [0; 129],
                },
                position: 0,
                length: config.chunk_size as off_t,
            };
            init
        };
        let mut mirror: wget_metalink_mirror = {
            let mut init = wget_metalink_mirror {
                iri: (*job).iri,
                priority: 0,
                location: *::core::mem::transmute::<
                    &[u8; 8],
                    &mut [libc::c_char; 8],
                >(b"-\0\0\0\0\0\0\0"),
            };
            init
        };
        let mut metalink: *mut wget_metalink = wget_calloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<wget_metalink>() as libc::c_ulong,
        ) as *mut wget_metalink;
        (*metalink).size = (*resp).content_length as off_t;
        (*metalink)
            .name = wget_strdup(
            if !(config.output_document).is_null() {
                config.output_document
            } else {
                local_filename
            },
        );
        let mut npieces: ssize_t = ((*resp).content_length as libc::c_ulonglong)
            .wrapping_add(config.chunk_size as libc::c_ulonglong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
            .wrapping_div(config.chunk_size as libc::c_ulonglong) as ssize_t;
        (*metalink).pieces = wget_vector_create(npieces as libc::c_int, None);
        let mut it: libc::c_int = 0 as libc::c_int;
        while (it as ssize_t) < npieces {
            piece.position = (it as libc::c_longlong * config.chunk_size) as off_t;
            wget_vector_add_memdup(
                (*metalink).pieces,
                &mut piece as *mut wget_metalink_piece as *const libc::c_void,
                ::core::mem::size_of::<wget_metalink_piece>() as libc::c_ulong,
            );
            it += 1;
            it;
        }
        (*metalink).mirrors = wget_vector_create(1 as libc::c_int, None);
        wget_vector_add_memdup(
            (*metalink).mirrors,
            &mut mirror as *mut wget_metalink_mirror as *const libc::c_void,
            ::core::mem::size_of::<wget_metalink_mirror>() as libc::c_ulong,
        );
        (*job).metalink = metalink;
        if job_validate_file(job) == 0 {
            wget_thread_cond_signal(worker_cond);
            (*job).set_done(0 as libc::c_int != 0);
        }
    } else if config.chunk_size != 0 {
        (*job).set_done(0 as libc::c_int != 0);
    }
    if !(config.mime_types).is_null() {
        (*job)
            .set_done(
                if check_mime_list(
                    config.mime_types,
                    if !((*resp).content_type).is_null() {
                        (*resp).content_type
                    } else {
                        b"application/octet-stream\0" as *const u8 as *const libc::c_char
                    },
                ) as libc::c_int != 0
                {
                    0 as libc::c_int
                } else {
                    1 as libc::c_int
                } != 0,
            );
    } else if config.timestamping as libc::c_int != 0 && !config.if_modified_since
        && config.chunk_size == 0
    {
        (*job).set_done(0 as libc::c_int != 0);
        if config.timestamping as libc::c_int != 0 && !config.if_modified_since
            && file_size >= 0 as libc::c_int as libc::c_longlong
            && file_size != (*resp).content_length as libc::c_longlong
        {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"The sizes do not match (local %lld, remote %zu) -- retrieving\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                file_size,
                (*resp).content_length,
            );
        }
    }
}
unsafe extern "C" fn process_response_part(mut resp: *mut wget_http_response) {
    let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
    let mut downloader: *mut DOWNLOADER = (*job).downloader;
    let mut part: *mut PART = (*job).part;
    if !((*resp).body).is_null() {
        quota_modify_read((*resp).cur_downloaded);
    }
    if (*resp).code as libc::c_int != 200 as libc::c_int
        && (*resp).code as libc::c_int != 206 as libc::c_int
    {
        print_status(
            downloader,
            b"part %d download error %d\n\0" as *const u8 as *const libc::c_char,
            (*part).id,
            (*resp).code as libc::c_int,
        );
    } else if ((*resp).body).is_null() {
        print_status(
            downloader,
            b"part %d download error 'empty body'\n\0" as *const u8
                as *const libc::c_char,
            (*part).id,
        );
    } else if (*(*resp).body).length != (*part).length as size_t {
        print_status(
            downloader,
            b"part %d download error '%zu bytes of %lld expected'\n\0" as *const u8
                as *const libc::c_char,
            (*part).id,
            (*(*resp).body).length,
            (*part).length as libc::c_longlong,
        );
    } else {
        print_status(
            downloader,
            b"part %d downloaded\n\0" as *const u8 as *const libc::c_char,
            (*part).id,
        );
        (*part).set_done(1 as libc::c_int != 0);
    }
    if (*part).done() {
        let mut all_done: libc::c_int = 1 as libc::c_int;
        let mut it: libc::c_int = 0;
        wget_thread_mutex_lock(downloader_mutex);
        it = 0 as libc::c_int;
        while it < wget_vector_size((*job).parts) {
            let mut partp: *mut PART = wget_vector_get((*job).parts, it) as *mut PART;
            if !(*partp).done() {
                all_done = 0 as libc::c_int;
                break;
            } else {
                it += 1;
                it;
            }
        }
        wget_thread_mutex_unlock(downloader_mutex);
        if all_done != 0 {
            if config.progress as libc::c_int == 1 as libc::c_int {
                bar_print(
                    (*downloader).id,
                    b"Checksumming...\0" as *const u8 as *const libc::c_char,
                );
            } else if !((*job).metalink).is_null() {
                print_status(
                    downloader,
                    b"%s checking...\n\0" as *const u8 as *const libc::c_char,
                    (*(*job).metalink).name,
                );
            } else {
                print_status(
                    downloader,
                    b"%s checking...\n\0" as *const u8 as *const libc::c_char,
                    (*(*job).blacklist_entry).local_filename,
                );
            }
            if job_validate_file(job) != 0 {
                if config.progress as libc::c_int == 1 as libc::c_int {
                    bar_print(
                        (*downloader).id,
                        b"Checksum OK\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    wget_debug_printf(
                        b"checksum ok\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                (*job).set_done(1 as libc::c_int != 0);
            } else if config.progress as libc::c_int == 1 as libc::c_int {
                bar_print(
                    (*downloader).id,
                    b"Checksum FAILED\0" as *const u8 as *const libc::c_char,
                );
            } else {
                wget_debug_printf(
                    b"checksum failed\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    } else {
        print_status(
            downloader,
            b"part %d failed\n\0" as *const u8 as *const libc::c_char,
            (*part).id,
        );
        (*part).set_inuse(0 as libc::c_int != 0);
    };
}
unsafe extern "C" fn process_response(mut resp: *mut wget_http_response) {
    let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
    let mut process_decision: libc::c_int = 0 as libc::c_int;
    let mut recurse_decision: libc::c_int = 0 as libc::c_int;
    if !((*resp).body).is_null() {
        quota_modify_read((*resp).cur_downloaded);
    }
    if !((*resp).location).is_null() {
        if config.metalink {
            let mut it: libc::c_int = 0 as libc::c_int;
            while it < wget_vector_size((*resp).links) {
                let mut link: *mut wget_http_link = wget_vector_get((*resp).links, it)
                    as *mut wget_http_link;
                if (*link).rel as libc::c_uint
                    == link_rel_describedby as libc::c_int as libc::c_uint
                {
                    if !((*link).type_0).is_null()
                        && (wget_strcasecmp_ascii(
                            (*link).type_0,
                            b"application/metalink4+xml\0" as *const u8
                                as *const libc::c_char,
                        ) == 0
                            || wget_strcasecmp_ascii(
                                (*link).type_0,
                                b"application/metalink+xml\0" as *const u8
                                    as *const libc::c_char,
                            ) == 0)
                    {
                        queue_url_from_remote(
                            job,
                            b"utf-8\0" as *const u8 as *const libc::c_char,
                            (*link).uri,
                            0 as libc::c_int,
                            0 as *const libc::c_char,
                        );
                        return;
                    }
                }
                it += 1;
                it;
            }
        }
        let mut uri_buf: wget_buffer = wget_buffer {
            data: 0 as *mut libc::c_char,
            length: 0,
            size: 0,
            release_data_release_buf_error: [0; 1],
            c2rust_padding: [0; 7],
        };
        let mut uri_sbuf: [libc::c_char; 1024] = [0; 1024];
        wget_buffer_init(
            &mut uri_buf,
            uri_sbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        let mut location: *const libc::c_char = (*resp).location;
        let mut top_link: *mut wget_http_link = 0 as *mut wget_http_link;
        let mut it_0: libc::c_int = 0 as libc::c_int;
        while it_0 < wget_vector_size((*resp).links) {
            let mut link_0: *mut wget_http_link = wget_vector_get((*resp).links, it_0)
                as *mut wget_http_link;
            if (*link_0).rel as libc::c_uint
                == link_rel_duplicate as libc::c_int as libc::c_uint
            {
                if top_link.is_null() || (*top_link).pri > (*link_0).pri {
                    top_link = link_0;
                    location = (*link_0).uri;
                }
            }
            it_0 += 1;
            it_0;
        }
        if !location.is_null() {
            wget_iri_relative_to_abs(
                (*job).iri,
                location,
                -(1 as libc::c_int) as size_t,
                &mut uri_buf,
            );
            if uri_buf.length != 0 {
                queue_url_from_remote(
                    job,
                    b"utf-8\0" as *const u8 as *const libc::c_char,
                    uri_buf.data,
                    (1 as libc::c_int) << 0 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
        }
        wget_buffer_deinit(&mut uri_buf);
        if !location.is_null() {
            return;
        }
    }
    if config.metalink as libc::c_int != 0 && !((*resp).content_type).is_null() {
        if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"application/metalink4+xml\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/metalink+xml\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if !((*resp).body).is_null() && !((*(*resp).body).data).is_null() {
                (*job).metalink = wget_metalink_parse((*(*resp).body).data);
                if !(config.output_document).is_null() {
                    if !((*(*job).metalink).name).is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )((*(*job).metalink).name as *mut libc::c_void);
                        (*(*job).metalink).name = 0 as *const libc::c_char;
                    }
                    (*(*job).metalink).name = wget_strdup(config.output_document);
                }
            }
        }
        if !((*job).metalink).is_null() {
            if (*(*job).metalink).size <= 0 as libc::c_int as off_t {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"File length %llu - remove job\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*(*job).metalink).size as libc::c_ulonglong,
                );
            } else if ((*(*job).metalink).mirrors).is_null() {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"No download mirrors found - remove job\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else if job_validate_file(job) == 0 {
                wget_metalink_sort_mirrors((*job).metalink);
                wget_thread_cond_signal(worker_cond);
                (*job).set_done(0 as libc::c_int != 0);
            }
            return;
        }
    }
    if (*resp).code as libc::c_int == 200 as libc::c_int
        || (*resp).code as libc::c_int == 206 as libc::c_int
        || (*resp).code as libc::c_int == 416 as libc::c_int
        || (*resp).code as libc::c_int == 304 as libc::c_int
            && config.timestamping as libc::c_int != 0
    {
        process_decision = if !((*(*job).blacklist_entry).local_filename).is_null()
            || !((*resp).body).is_null()
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        recurse_decision = if process_decision != 0
            && config.recursive as libc::c_int != 0
            && (config.level == 0
                || (*job).level < config.level + config.page_requisites as libc::c_int)
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        if process_decision != 0 {
            let mut recurse_iris: *mut wget_vector = 0 as *mut wget_vector;
            let mut data: *const libc::c_void = 0 as *const libc::c_void;
            let mut size: int64_t = 0;
            let mut filename: *const libc::c_char = 0 as *const libc::c_char;
            if config.spider as libc::c_int != 0
                || config.recursive as libc::c_int != 0
                    && !(config.output_document).is_null()
            {
                filename = 0 as *const libc::c_char;
            } else {
                filename = (*(*job).blacklist_entry).local_filename;
            }
            if ((*resp).code as libc::c_int == 304 as libc::c_int
                || (*resp).code as libc::c_int == 416 as libc::c_int
                || (*resp).code as libc::c_int == 206 as libc::c_int)
                && !filename.is_null()
            {
                size = get_file_size(filename) as int64_t;
            } else {
                size = (*resp).content_length as int64_t;
            }
            if ((*resp).code as libc::c_int == 200 as libc::c_int
                || (*resp).code as libc::c_int == 206 as libc::c_int)
                && !((*resp).body).is_null() && (*(*resp).body).length as int64_t == size
            {
                data = (*(*resp).body).data as *const libc::c_void;
            }
            if recurse_decision != 0 {
                recurse_iris = wget_vector_create(16 as libc::c_int, None);
            }
            process_decision = plugin_db_forward_downloaded_file(
                (*job).iri,
                if size > 0 as libc::c_int as int64_t {
                    size
                } else {
                    0 as libc::c_int as int64_t
                },
                filename,
                data,
                recurse_iris,
            );
            if recurse_decision != 0 {
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < wget_vector_size(recurse_iris) {
                    let mut iri: *mut wget_iri = wget_vector_get(recurse_iris, i)
                        as *mut wget_iri;
                    queue_url_from_remote(
                        job,
                        b"utf-8\0" as *const u8 as *const libc::c_char,
                        (*iri).uri,
                        0 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                    wget_iri_free_content(iri);
                    i += 1;
                    i;
                }
                wget_vector_free(&mut recurse_iris);
            }
        }
    }
    if (*job).robotstxt() as libc::c_int != 0 && !((*resp).body).is_null()
        && wget_robots_parse(
            &mut (*(*job).host).robots,
            (*(*resp).body).data,
            b"wget2\0" as *const u8 as *const libc::c_char,
        ) == WGET_E_SUCCESS as libc::c_int && !config.page_requisites
    {
        if config.follow_sitemaps {
            let mut it_1: libc::c_int = 0 as libc::c_int;
            let mut n: libc::c_int = wget_robots_get_sitemap_count(
                (*(*job).host).robots,
            );
            while it_1 < n {
                let mut sitemap: *const libc::c_char = wget_robots_get_sitemap(
                    (*(*job).host).robots,
                    it_1,
                );
                wget_debug_printf(
                    b"adding sitemap '%s'\n\0" as *const u8 as *const libc::c_char,
                    sitemap,
                );
                queue_url_from_remote(
                    job,
                    b"utf-8\0" as *const u8 as *const libc::c_char,
                    sitemap,
                    (1 as libc::c_int) << 1 as libc::c_int,
                    0 as *const libc::c_char,
                );
                it_1 += 1;
                it_1;
            }
        }
    } else if (*resp).code as libc::c_int == 200 as libc::c_int
        || (*resp).code as libc::c_int == 206 as libc::c_int
    {
        if process_decision != 0 && recurse_decision != 0 {
            if !((*resp).content_type).is_null() && !((*resp).body).is_null() {
                if wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"text/html\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    html_parse(
                        job,
                        (*job).level,
                        (*(*job).blacklist_entry).local_filename,
                        (*(*resp).body).data,
                        (*(*resp).body).length,
                        if !((*resp).content_type_encoding).is_null() {
                            (*resp).content_type_encoding
                        } else {
                            config.remote_encoding
                        },
                        (*job).iri,
                    );
                } else if wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    html_parse(
                        job,
                        (*job).level,
                        (*(*job).blacklist_entry).local_filename,
                        (*(*resp).body).data,
                        (*(*resp).body).length,
                        if !((*resp).content_type_encoding).is_null() {
                            (*resp).content_type_encoding
                        } else {
                            config.remote_encoding
                        },
                        (*job).iri,
                    );
                } else if wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"text/css\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    css_parse(
                        job,
                        (*(*resp).body).data,
                        (*(*resp).body).length,
                        if !((*resp).content_type_encoding).is_null() {
                            (*resp).content_type_encoding
                        } else {
                            config.remote_encoding
                        },
                        (*job).iri,
                    );
                } else if wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"application/atom+xml\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    atom_parse(
                        job,
                        (*(*resp).body).data,
                        b"utf-8\0" as *const u8 as *const libc::c_char,
                        (*job).iri,
                    );
                } else if wget_strcasecmp_ascii(
                    (*resp).content_type,
                    b"application/rss+xml\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    rss_parse(
                        job,
                        (*(*resp).body).data,
                        b"utf-8\0" as *const u8 as *const libc::c_char,
                        (*job).iri,
                    );
                } else if (*job).sitemap() {
                    if wget_strcasecmp_ascii(
                        (*resp).content_type,
                        b"application/xml\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        sitemap_parse_xml(
                            job,
                            (*(*resp).body).data,
                            b"utf-8\0" as *const u8 as *const libc::c_char,
                            (*job).iri,
                        );
                    } else if wget_strcasecmp_ascii(
                        (*resp).content_type,
                        b"application/x-gzip\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        sitemap_parse_xml_gz(
                            job,
                            (*resp).body,
                            b"utf-8\0" as *const u8 as *const libc::c_char,
                            (*job).iri,
                        );
                    } else if wget_strcasecmp_ascii(
                        (*resp).content_type,
                        b"text/plain\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        sitemap_parse_text(
                            job,
                            (*(*resp).body).data,
                            b"utf-8\0" as *const u8 as *const libc::c_char,
                            (*job).iri,
                        );
                    }
                }
            }
        } else {
            config.verify_sig as libc::c_uint
                != GPG_VERIFY_DISABLED as libc::c_int as libc::c_uint
                && !((*resp).content_type).is_null();
        }
    } else if (*resp).code as libc::c_int == 304 as libc::c_int
        && config.timestamping as libc::c_int != 0
        || (*resp).code as libc::c_int == 416 as libc::c_int
    {
        if process_decision != 0 && recurse_decision != 0 {
            let mut local_filename: *const libc::c_char = 0 as *const libc::c_char;
            if config.content_disposition as libc::c_int != 0
                && !((*resp).content_filename).is_null()
            {
                let mut iri_0: wget_iri = {
                    let mut init = wget_iri_st {
                        port_given_host_allocated_path_allocated_query_allocated_fragment_allocated_is_ip_address: [0; 1],
                        c2rust_padding: [0; 7],
                        uri: 0 as *const libc::c_char,
                        safe_uri: 0 as *const libc::c_char,
                        display: 0 as *const libc::c_char,
                        userinfo: 0 as *const libc::c_char,
                        password: 0 as *const libc::c_char,
                        host: (*(*job).iri).host,
                        path: (*resp).content_filename,
                        query: 0 as *const libc::c_char,
                        fragment: 0 as *const libc::c_char,
                        connection_part: 0 as *const libc::c_char,
                        dirlen: 0,
                        msize: 0,
                        port: 0,
                        scheme: (*(*job).iri).scheme,
                    };
                    init.set_port_given(false);
                    init.set_host_allocated(false);
                    init.set_path_allocated(false);
                    init.set_query_allocated(false);
                    init.set_fragment_allocated(false);
                    init.set_is_ip_address(false);
                    init
                };
                local_filename = get_local_filename(&mut iri_0);
            } else {
                local_filename = (*(*job).blacklist_entry).local_filename;
            }
            parse_localfile(
                job,
                local_filename,
                (*resp).content_type_encoding,
                (*resp).content_type,
                (*job).iri,
            );
        }
    }
}
unsafe extern "C" fn fallback_to_http(mut job: *mut JOB) {
    if !(*job).robotstxt() {
        let mut http_url: *mut libc::c_char = wget_aprintf(
            b"http://%s\0" as *const u8 as *const libc::c_char,
            ((*(*job).iri).safe_uri).offset(8 as libc::c_int as isize),
        );
        queue_url_from_remote(
            0 as *mut JOB,
            b"utf-8\0" as *const u8 as *const libc::c_char,
            http_url,
            (1 as libc::c_int) << 2 as libc::c_int,
            0 as *const libc::c_char,
        );
        host_remove_job((*job).host, job);
        if !http_url.is_null() {
            wget_free.expect("non-null function pointer")(http_url as *mut libc::c_void);
            http_url = 0 as *mut libc::c_char;
        }
    } else {
        host_remove_job((*job).host, job);
    };
}
unsafe extern "C" fn downloader_thread(mut p: *mut libc::c_void) -> *mut libc::c_void {
    let mut downloader: *mut DOWNLOADER = p as *mut DOWNLOADER;
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut job: *mut JOB = 0 as *mut JOB;
    let mut host: *mut HOST = 0 as *mut HOST;
    let mut pending: libc::c_int = 0 as libc::c_int;
    let mut max_pending: libc::c_int = 1 as libc::c_int;
    let mut locked: libc::c_int = 0;
    let mut pause: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut action: actions = ACTION_GET_JOB;
    wget_thread_mutex_lock(main_mutex);
    locked = 1 as libc::c_int;
    while !terminate {
        wget_debug_printf(
            b"[%d] action=%d pending=%d host=%p goaway=%s\n\0" as *const u8
                as *const libc::c_char,
            (*downloader).id,
            action as libc::c_int,
            pending,
            host as *mut libc::c_void,
            if !((*downloader).conn).is_null() {
                if wget_http_connection_receive_only((*downloader).conn) as libc::c_int
                    != 0
                {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                }
            } else {
                b"n/a\0" as *const u8 as *const libc::c_char
            },
        );
        match action as libc::c_uint {
            1 => {
                if !((*downloader).conn).is_null()
                    && wget_http_connection_receive_only((*downloader).conn)
                        as libc::c_int != 0
                {
                    if pending != 0 {
                        wget_thread_mutex_unlock(main_mutex);
                        locked = 0 as libc::c_int;
                        action = ACTION_GET_RESPONSE;
                    } else {
                        wget_thread_mutex_unlock(main_mutex);
                        locked = 0 as libc::c_int;
                        action = ACTION_ERROR;
                    }
                } else {
                    job = host_get_job(host, &mut pause);
                    if job.is_null() {
                        if pending != 0 {
                            wget_thread_mutex_unlock(main_mutex);
                            locked = 0 as libc::c_int;
                            action = ACTION_GET_RESPONSE;
                        } else if !host.is_null() {
                            wget_http_close(&mut (*downloader).conn);
                            host = 0 as *mut HOST;
                        } else if !wget_thread_support() {
                            if pause == 0 {
                                break;
                            }
                            wget_millisleep(pause as libc::c_int);
                        } else {
                            wget_thread_cond_wait(worker_cond, main_mutex, pause);
                            locked = 1 as libc::c_int;
                        }
                    } else {
                        wget_thread_mutex_unlock(main_mutex);
                        locked = 0 as libc::c_int;
                        let mut iri: *const wget_iri = (*job).iri;
                        (*downloader).job = job;
                        (*job).downloader = downloader;
                        pending += 1;
                        if pending == 1 as libc::c_int {
                            host = (*job).host;
                            if establish_connection(downloader, &mut iri)
                                != WGET_E_SUCCESS as libc::c_int
                            {
                                if (*job).http_fallback() {
                                    fallback_to_http(job);
                                } else {
                                    host_increase_failure(host);
                                }
                                action = ACTION_ERROR;
                                continue;
                            } else {
                                (*job).iri = iri;
                                if config.wait != 0 || !((*job).metalink).is_null()
                                    || ((*downloader).conn).is_null()
                                    || wget_http_get_protocol((*downloader).conn)
                                        != 1 as libc::c_int
                                {
                                    max_pending = 1 as libc::c_int;
                                } else {
                                    max_pending = config.http2_request_window;
                                }
                            }
                        }
                        if config.wait != 0 {
                            if config.random_wait {
                                wget_millisleep(
                                    rand() % config.wait + config.wait / 2 as libc::c_int,
                                );
                            } else {
                                wget_millisleep(config.wait);
                            }
                            if terminate {
                                continue;
                            }
                        }
                        if ((*job).original_url).is_null() {
                            (*job).original_url = iri;
                        }
                        if http_send_request((*job).iri, (*job).original_url, downloader)
                            != WGET_E_SUCCESS as libc::c_int
                        {
                            if (*job).http_fallback() {
                                fallback_to_http(job);
                            } else {
                                host_increase_failure(host);
                            }
                            action = ACTION_ERROR;
                        } else if pending >= max_pending {
                            action = ACTION_GET_RESPONSE;
                        } else {
                            wget_thread_mutex_lock(main_mutex);
                            locked = 1 as libc::c_int;
                        }
                    }
                }
            }
            2 => {
                resp = http_receive_response((*downloader).conn);
                if resp.is_null() {
                    host_increase_failure(host);
                    action = ACTION_ERROR;
                } else {
                    job = (*(*resp).req).user_data as *mut JOB;
                    if process_response_header(resp) == 0 as libc::c_int {
                        if (*job).head_first() {
                            process_head_response(resp);
                        } else if !((*job).part).is_null() {
                            process_response_part(resp);
                        } else {
                            process_response(resp);
                        }
                    }
                    if !(config.retry_on_http_error).is_null()
                        && (*resp).code as libc::c_int != 200 as libc::c_int
                    {
                        if config.tries != 0
                            && {
                                (*job).failures += 1;
                                (*job).failures >= config.tries
                            }
                        {
                            print_status(
                                downloader,
                                b"Job reached max tries.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            (*job).set_done(1 as libc::c_int != 0);
                            if (*resp).code as libc::c_int >= 400 as libc::c_int {
                                set_exit_status(EXIT_STATUS_NETWORK);
                            }
                        } else if check_status_code_list(
                            config.retry_on_http_error,
                            (*resp).code as uint16_t,
                        ) {
                            (*job).set_done(0 as libc::c_int != 0);
                            (*job)
                                .retry_ts = wget_get_timemillis()
                                + ((*job).failures * 1000 as libc::c_int)
                                    as libc::c_longlong;
                        }
                    }
                    host_reset_failure(host);
                    wget_http_free_request(&mut (*resp).req);
                    wget_http_free_response(&mut resp);
                    wget_thread_mutex_lock(main_mutex);
                    locked = 1 as libc::c_int;
                    if (*job).done() {
                        host_remove_job(host, job);
                    } else {
                        (*job).set_inuse(0 as libc::c_int != 0);
                    }
                    wget_thread_cond_signal(main_cond);
                    pending -= 1;
                    pending;
                    action = ACTION_GET_JOB;
                }
            }
            3 => {
                wget_http_close(&mut (*downloader).conn);
                wget_thread_mutex_lock(main_mutex);
                locked = 1 as libc::c_int;
                host_release_jobs(host);
                wget_thread_cond_signal(main_cond);
                host = 0 as *mut HOST;
                pending = 0 as libc::c_int;
                action = ACTION_GET_JOB;
            }
            _ => {
                wget_error_printf_exit(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unhandled action %d\n\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    action as libc::c_int,
                );
            }
        }
    }
    if locked != 0 {
        wget_thread_mutex_unlock(main_mutex);
    }
    wget_http_close(&mut (*downloader).conn);
    wget_thread_cond_signal(worker_cond);
    wget_thread_cond_signal(main_cond);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn free_conversion(mut conversion: *mut libc::c_void) {
    let mut c: *mut conversion_t = conversion as *mut conversion_t;
    if !((*c).filename).is_null() {
        wget_free
            .expect("non-null function pointer")((*c).filename as *mut libc::c_void);
        (*c).filename = 0 as *const libc::c_char;
    }
    if !((*c).encoding).is_null() {
        wget_free
            .expect("non-null function pointer")((*c).encoding as *mut libc::c_void);
        (*c).encoding = 0 as *const libc::c_char;
    }
    wget_iri_free(&mut (*c).base as *mut *const wget_iri as *mut *mut wget_iri);
    wget_html_free_urls_inline(&mut (*c).parsed);
    if !c.is_null() {
        wget_free.expect("non-null function pointer")(c as *mut libc::c_void);
        c = 0 as *mut conversion_t;
    }
}
unsafe extern "C" fn remember_for_conversion(
    mut filename: *const libc::c_char,
    mut base: *const wget_iri,
    mut content_type: libc::c_int,
    mut encoding: *const libc::c_char,
    mut parsed: *mut wget_html_parsed_result,
) {
    wget_thread_mutex_lock(conversion_mutex);
    wget_debug_printf(
        b"conversion: remember %s\n\0" as *const u8 as *const libc::c_char,
        filename,
    );
    if conversions.is_null() {
        conversions = wget_stringmap_create_nocase(128 as libc::c_int);
        wget_stringmap_set_key_destructor(conversions, None);
        wget_stringmap_set_value_destructor(
            conversions,
            Some(free_conversion as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    if wget_stringmap_get(
        conversions,
        filename,
        0 as *mut libc::c_void as *mut *mut libc::c_void,
    ) == 0
    {
        let mut conversion: *mut conversion_t = wget_malloc(
            ::core::mem::size_of::<conversion_t>() as libc::c_ulong,
        ) as *mut conversion_t;
        (*conversion).filename = wget_strdup(filename);
        (*conversion).encoding = wget_strdup(encoding);
        (*conversion).base = wget_iri_clone(base);
        (*conversion).content_type = content_type;
        (*conversion).parsed = parsed;
        wget_stringmap_put(
            conversions,
            (*conversion).filename,
            conversion as *const libc::c_void,
        );
    } else {
        wget_html_free_urls_inline(&mut parsed);
    }
    wget_thread_mutex_unlock(conversion_mutex);
}
unsafe extern "C" fn hash_url(mut url: *const libc::c_char) -> libc::c_uint {
    let mut hash: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while *url != 0 {
        let fresh3 = url;
        url = url.offset(1);
        hash = hash
            .wrapping_mul(101 as libc::c_int as libc::c_uint)
            .wrapping_add(*fresh3 as libc::c_uchar as libc::c_uint);
    }
    return hash;
}
unsafe extern "C" fn normalize_uri(
    mut base: *const wget_iri,
    mut url: *mut wget_string,
    mut encoding: *const libc::c_char,
    mut buf: *mut wget_buffer,
) -> libc::c_int {
    let mut urlpart: *mut libc::c_char = wget_strmemdup(
        (*url).p as *const libc::c_void,
        (*url).len,
    );
    let mut urlpart_encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut urlpart_encoded_length: size_t = 0;
    let mut rc: libc::c_int = 0;
    if (*url).len == 0 as libc::c_int as size_t
        || (*url).len >= 1 as libc::c_int as size_t
            && *(*url).p as libc::c_int == '#' as i32
    {
        if !urlpart.is_null() {
            wget_free.expect("non-null function pointer")(urlpart as *mut libc::c_void);
            urlpart = 0 as *mut libc::c_char;
        }
        return -(1 as libc::c_int);
    }
    wget_xml_decode_entities_inline(urlpart);
    wget_iri_unescape_url_inline(urlpart);
    rc = wget_memiconv(
        encoding,
        urlpart as *const libc::c_void,
        strlen(urlpart),
        b"utf-8\0" as *const u8 as *const libc::c_char,
        &mut urlpart_encoded,
        &mut urlpart_encoded_length,
    );
    if !urlpart.is_null() {
        wget_free.expect("non-null function pointer")(urlpart as *mut libc::c_void);
        urlpart = 0 as *mut libc::c_char;
    }
    if rc != 0 {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URL '%.*s' not followed (conversion failed)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*url).len as libc::c_int,
            (*url).p,
        );
        return -(2 as libc::c_int);
    }
    rc = (wget_iri_relative_to_abs(base, urlpart_encoded, urlpart_encoded_length, buf))
        .is_null() as libc::c_int;
    if !urlpart_encoded.is_null() {
        wget_free
            .expect("non-null function pointer")(urlpart_encoded as *mut libc::c_void);
        urlpart_encoded = 0 as *mut libc::c_char;
    }
    if rc != 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Cannot resolve relative URI %.*s\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*url).len as libc::c_int,
            (*url).p,
        );
        return -(3 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn html_parse(
    mut job: *mut JOB,
    mut level: libc::c_int,
    mut fname: *const libc::c_char,
    mut html: *const libc::c_char,
    mut html_len: size_t,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut allocated_base: *mut wget_iri = 0 as *mut wget_iri;
    let mut reason: *const libc::c_char = 0 as *const libc::c_char;
    let mut utf8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    let mut convert_links_0: libc::c_int = (config.convert_links as libc::c_int != 0
        && !config.delete_after) as libc::c_int;
    let mut convert_file_only: libc::c_int = (config.convert_file_only as libc::c_int
        != 0 && !config.delete_after) as libc::c_int;
    let mut page_requisites: bool = config.recursive as libc::c_int != 0
        && config.page_requisites as libc::c_int != 0 && config.level != 0
        && level < config.level;
    if !encoding.is_null() && encoding == config.remote_encoding {
        reason = dcgettext(
            0 as *const libc::c_char,
            b"set by user\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    } else if *html.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
        == 0xfe as libc::c_int
        && *html.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xff as libc::c_int
    {
        encoding = b"UTF-16BE\0" as *const u8 as *const libc::c_char;
        reason = dcgettext(
            0 as *const libc::c_char,
            b"set by BOM\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        html = html.offset(2 as libc::c_int as isize);
        html_len = html_len.wrapping_sub(2 as libc::c_int as size_t);
    } else if *html.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
        == 0xff as libc::c_int
        && *html.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xfe as libc::c_int
    {
        encoding = b"UTF-16LE\0" as *const u8 as *const libc::c_char;
        reason = dcgettext(
            0 as *const libc::c_char,
            b"set by BOM\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        html = html.offset(2 as libc::c_int as isize);
        html_len = html_len.wrapping_sub(2 as libc::c_int as size_t);
    } else if *html.offset(0 as libc::c_int as isize) as libc::c_uchar as libc::c_int
        == 0xef as libc::c_int
        && *html.offset(1 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xbb as libc::c_int
        && *html.offset(2 as libc::c_int as isize) as libc::c_uchar as libc::c_int
            == 0xbf as libc::c_int
    {
        encoding = b"UTF-8\0" as *const u8 as *const libc::c_char;
        reason = dcgettext(
            0 as *const libc::c_char,
            b"set by BOM\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
        html = html.offset(3 as libc::c_int as isize);
        html_len = html_len.wrapping_sub(3 as libc::c_int as size_t);
    } else {
        reason = dcgettext(
            0 as *const libc::c_char,
            b"set by server response\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        );
    }
    if wget_strncasecmp_ascii(
        encoding,
        b"UTF-16\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    ) == 0
    {
        let mut n: size_t = 0;
        html_len = html_len.wrapping_sub(html_len & 1 as libc::c_int as size_t);
        if wget_memiconv(
            encoding,
            html as *const libc::c_void,
            html_len,
            b"UTF-8\0" as *const u8 as *const libc::c_char,
            &mut utf8,
            &mut n,
        ) == 0 as libc::c_int
        {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Convert non-ASCII encoding '%s' (%s) to UTF-8\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                encoding,
                reason,
            );
            html = utf8;
            if convert_links_0 != 0 {
                convert_links_0 = 0 as libc::c_int;
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Link conversion disabled for '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname,
                );
            } else if convert_file_only != 0 {
                convert_file_only = 0 as libc::c_int;
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Filename conversion disabled for '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname,
                );
            }
        } else {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to convert non-ASCII encoding '%s' (%s) to UTF-8, skip parsing\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                encoding,
                reason,
            );
            return;
        }
    }
    let mut parsed: *mut wget_html_parsed_result = wget_html_get_urls_inline(
        html,
        config.follow_tags,
        config.ignore_tags,
    );
    if !(config.robots as libc::c_int != 0 && !(*parsed).follow()) {
        if encoding.is_null() {
            if !((*parsed).encoding).is_null() {
                encoding = (*parsed).encoding;
                reason = dcgettext(
                    0 as *const libc::c_char,
                    b"set by document\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                );
            } else {
                encoding = b"CP1252\0" as *const u8 as *const libc::c_char;
                reason = dcgettext(
                    0 as *const libc::c_char,
                    b"default, encoding not specified\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                );
            }
        }
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URI content encoding = '%s' (%s)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            encoding,
            reason,
        );
        wget_buffer_init(
            &mut buf,
            sbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        );
        if !((*parsed).base.p).is_null() {
            if normalize_uri(base, &mut (*parsed).base, encoding, &mut buf)
                == 0 as libc::c_int
            {
                if base.is_null() && buf.length == 0 {
                    wget_info_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"BASE '%.*s' not usable (missing absolute base URI)\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*parsed).base.len as libc::c_int,
                        (*parsed).base.p,
                    );
                } else {
                    let mut newbase: *mut wget_iri = wget_iri_parse(
                        buf.data,
                        b"utf-8\0" as *const u8 as *const libc::c_char,
                    );
                    if !newbase.is_null() {
                        allocated_base = newbase;
                        base = allocated_base;
                    }
                }
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Cannot resolve BASE URI %.*s\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*parsed).base.len as libc::c_int,
                    (*parsed).base.p,
                );
            }
        }
        let mut current_block_81: u64;
        let mut it: libc::c_int = 0 as libc::c_int;
        while it < wget_vector_size((*parsed).uris) {
            let mut html_url: *mut wget_html_parsed_url = wget_vector_get(
                (*parsed).uris,
                it,
            ) as *mut wget_html_parsed_url;
            let mut url: *mut wget_string = &mut (*html_url).url;
            if wget_strcasecmp_ascii(
                ((*html_url).attr).as_mut_ptr(),
                b"action\0" as *const u8 as *const libc::c_char,
            ) == 0
                || wget_strcasecmp_ascii(
                    ((*html_url).attr).as_mut_ptr(),
                    b"formaction\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"URL '%.*s' not followed (action/formaction attribute)\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*url).len as libc::c_int,
                    (*url).p,
                );
            } else {
                if page_requisites as libc::c_int != 0
                    && wget_strcasecmp_ascii(
                        ((*html_url).attr).as_mut_ptr(),
                        b"href\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    if config.level != 0 && level >= config.level - 1 as libc::c_int {
                        if c_tolower(*((*html_url).tag).as_mut_ptr() as libc::c_int)
                            == 'a' as i32
                            && ((*html_url).tag[1 as libc::c_int as usize] as libc::c_int
                                == 0 as libc::c_int
                                || wget_strcasecmp_ascii(
                                    ((*html_url).tag).as_mut_ptr(),
                                    b"area\0" as *const u8 as *const libc::c_char,
                                ) == 0) || !(*html_url).link_inline()
                            || wget_strcasecmp_ascii(
                                ((*html_url).tag).as_mut_ptr(),
                                b"embed\0" as *const u8 as *const libc::c_char,
                            ) == 0
                        {
                            wget_info_printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"URL '%.*s' not followed (page requisites + level)\n\0"
                                        as *const u8 as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                (*url).len as libc::c_int,
                                (*url).p,
                            );
                            current_block_81 = 7018308795614528254;
                        } else {
                            current_block_81 = 4888910987971495881;
                        }
                    } else {
                        current_block_81 = 4888910987971495881;
                    }
                } else {
                    current_block_81 = 4888910987971495881;
                }
                match current_block_81 {
                    7018308795614528254 => {}
                    _ => {
                        if !(normalize_uri(base, url, encoding, &mut buf) != 0) {
                            if base.is_null() && buf.length == 0 {
                                wget_info_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"URL '%.*s' not followed (missing base URI)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    (*url).len as libc::c_int,
                                    (*url).p,
                                );
                            } else {
                                wget_thread_mutex_lock(known_urls_mutex);
                                let mut rc: libc::c_int = wget_hashmap_put(
                                    known_urls,
                                    wget_strmemdup(buf.data as *const libc::c_void, buf.length)
                                        as *const libc::c_void,
                                    0 as *const libc::c_void,
                                );
                                wget_thread_mutex_unlock(known_urls_mutex);
                                if rc == 0 as libc::c_int {
                                    let mut download_name: *mut libc::c_char = 0
                                        as *mut libc::c_char;
                                    if config.download_attr as libc::c_int != 0
                                        && !((*html_url).download.p).is_null()
                                    {
                                        download_name = wget_strmemdup(
                                            (*html_url).download.p as *const libc::c_void,
                                            (*html_url).download.len,
                                        );
                                    } else {
                                        download_name = 0 as *mut libc::c_char;
                                    }
                                    queue_url_from_remote(
                                        job,
                                        b"utf-8\0" as *const u8 as *const libc::c_char,
                                        buf.data,
                                        if page_requisites as libc::c_int != 0 {
                                            (1 as libc::c_int) << 3 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        },
                                        download_name,
                                    );
                                    if !download_name.is_null() {
                                        wget_free
                                            .expect(
                                                "non-null function pointer",
                                            )(download_name as *mut libc::c_void);
                                        download_name = 0 as *mut libc::c_char;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            it += 1;
            it;
        }
        wget_buffer_deinit(&mut buf);
        if (convert_links_0 != 0 || convert_file_only != 0) && !config.delete_after {
            let mut it_0: libc::c_int = 0 as libc::c_int;
            while it_0 < wget_vector_size((*parsed).uris) {
                let mut html_url_0: *mut wget_html_parsed_url = wget_vector_get(
                    (*parsed).uris,
                    it_0,
                ) as *mut wget_html_parsed_url;
                (*html_url_0)
                    .url
                    .p = ((*html_url_0).url.p).offset_from(html) as libc::c_long
                    as *const libc::c_char;
                it_0 += 1;
                it_0;
            }
            remember_for_conversion(fname, base, 1 as libc::c_int, encoding, parsed);
            parsed = 0 as *mut wget_html_parsed_result;
        }
        wget_iri_free(&mut allocated_base);
    }
    wget_html_free_urls_inline(&mut parsed);
    if !utf8.is_null() {
        wget_free.expect("non-null function pointer")(utf8 as *mut libc::c_void);
        utf8 = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn html_parse_localfile(
    mut job: *mut JOB,
    mut level: libc::c_int,
    mut fname: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: size_t = 0;
    data = wget_read_file(fname, &mut n);
    if !data.is_null() {
        html_parse(job, level, fname, data, n, encoding, base);
    }
    if !data.is_null() {
        wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
        data = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn sitemap_parse_xml(
    mut job: *mut JOB,
    mut data: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut urls: *mut wget_vector = 0 as *mut wget_vector;
    let mut sitemap_urls: *mut wget_vector = 0 as *mut wget_vector;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut baselen: size_t = 0 as libc::c_int as size_t;
    wget_sitemap_get_urls_inline(data, &mut urls, &mut sitemap_urls);
    if !base.is_null() {
        p = strrchr((*base).uri, '/' as i32);
        if !p.is_null() {
            baselen = (p.offset_from((*base).uri) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t;
        } else {
            baselen = strlen((*base).uri);
        }
    }
    wget_info_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"found %d url(s) (base=%s)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        wget_vector_size(urls),
        if !base.is_null() { (*base).uri } else { 0 as *const libc::c_char },
    );
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(urls) {
        let mut url: *mut wget_string = wget_vector_get(urls, it) as *mut wget_string;
        if baselen != 0
            && ((*url).len <= baselen
                || wget_strncasecmp((*url).p, (*base).uri, baselen) != 0)
        {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"URL '%.*s' not followed (not matching sitemap location)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*url).len as libc::c_int,
                (*url).p,
            );
        } else {
            wget_thread_mutex_lock(known_urls_mutex);
            p = wget_strmemdup((*url).p as *const libc::c_void, (*url).len);
            let mut rc: libc::c_int = wget_hashmap_put(
                known_urls,
                p as *const libc::c_void,
                0 as *const libc::c_void,
            );
            wget_thread_mutex_unlock(known_urls_mutex);
            if rc != 0 {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"URL '%.*s' not followed (already known)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*url).len as libc::c_int,
                    (*url).p,
                );
            } else {
                queue_url_from_remote(
                    job,
                    encoding,
                    p,
                    0 as libc::c_int,
                    0 as *const libc::c_char,
                );
            }
        }
        it += 1;
        it;
    }
    wget_info_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"found %d sitemap url(s) (base=%s)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        wget_vector_size(sitemap_urls),
        if !base.is_null() { (*base).uri } else { 0 as *const libc::c_char },
    );
    let mut it_0: libc::c_int = 0 as libc::c_int;
    while it_0 < wget_vector_size(sitemap_urls) {
        let mut url_0: *mut wget_string = wget_vector_get(sitemap_urls, it_0)
            as *mut wget_string;
        wget_thread_mutex_lock(known_urls_mutex);
        p = wget_strmemdup((*url_0).p as *const libc::c_void, (*url_0).len);
        let mut rc_0: libc::c_int = wget_hashmap_put(
            known_urls,
            p as *const libc::c_void,
            0 as *const libc::c_void,
        );
        wget_thread_mutex_unlock(known_urls_mutex);
        if rc_0 != 0 {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"URL '%.*s' not followed (already known)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*url_0).len as libc::c_int,
                (*url_0).p,
            );
        } else {
            queue_url_from_remote(
                job,
                encoding,
                p,
                (1 as libc::c_int) << 1 as libc::c_int,
                0 as *const libc::c_char,
            );
        }
        it_0 += 1;
        it_0;
    }
    wget_vector_free(&mut urls);
    wget_vector_free(&mut sitemap_urls);
}
unsafe extern "C" fn get_unzipped(
    mut userdata: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    wget_buffer_memcat(
        userdata as *mut wget_buffer,
        data as *const libc::c_void,
        length,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn sitemap_parse_xml_gz(
    mut job: *mut JOB,
    mut gzipped_data: *mut wget_buffer,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut plain: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut dc: *mut wget_decompressor = 0 as *mut wget_decompressor;
    wget_buffer_init(
        &mut plain,
        0 as *mut libc::c_char,
        (*gzipped_data).length * 10 as libc::c_int as size_t,
    );
    dc = wget_decompress_open(
        wget_content_encoding_gzip,
        Some(
            get_unzipped
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
        ),
        &mut plain as *mut wget_buffer as *mut libc::c_void,
    );
    if !dc.is_null() {
        wget_decompress(dc, (*gzipped_data).data, (*gzipped_data).length);
        wget_decompress_close(dc);
        sitemap_parse_xml(job, plain.data, encoding, base);
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Can't scan '%s' because no libz support enabled at compile time\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*(*job).iri).safe_uri,
        );
    }
    wget_buffer_deinit(&mut plain);
}
unsafe extern "C" fn sitemap_parse_xml_localfile(
    mut job: *mut JOB,
    mut fname: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = wget_read_file(fname, 0 as *mut size_t);
    if !data.is_null() {
        sitemap_parse_xml(job, data, encoding, base);
    }
    if !data.is_null() {
        wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
        data = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn sitemap_parse_text(
    mut job: *mut JOB,
    mut data: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut baselen: size_t = 0 as libc::c_int as size_t;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    if !base.is_null() {
        p = strrchr((*base).uri, '/' as i32);
        if !p.is_null() {
            baselen = (p.offset_from((*base).uri) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t;
        } else {
            baselen = strlen((*base).uri);
        }
    }
    end = data;
    line = end;
    while *end as libc::c_int != 0
        && {
            p = strchr(line, '\n' as i32);
            end = (if !p.is_null() { p } else { line.offset(strlen(line) as isize) });
            !end.is_null()
        }
    {
        len = end.offset_from(line) as libc::c_long as size_t;
        while len != 0
            && *(*__ctype_b_loc()).offset(*line as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            line = line.offset(1);
            line;
            len = len.wrapping_sub(1);
            len;
        }
        while len != 0
            && *(*__ctype_b_loc())
                .offset(
                    *line.offset(len.wrapping_sub(1 as libc::c_int as size_t) as isize)
                        as libc::c_int as isize,
                ) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            len = len.wrapping_sub(1);
            len;
        }
        if len != 0 {
            if baselen != 0
                && (len <= baselen || wget_strncasecmp(line, (*base).uri, baselen) != 0)
            {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"URL '%.*s' not followed (not matching sitemap location)\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    len as libc::c_int,
                    line,
                );
            } else {
                let mut urlbuf: [libc::c_char; 1024] = [0; 1024];
                let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
                url = wget_strmemcpy_a(
                    urlbuf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    line as *const libc::c_void,
                    len,
                ) as *mut libc::c_char;
                queue_url_from_remote(
                    job,
                    encoding,
                    url,
                    0 as libc::c_int,
                    0 as *const libc::c_char,
                );
                if url != urlbuf.as_mut_ptr() {
                    if !url.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(url as *mut libc::c_void);
                        url = 0 as *mut libc::c_char;
                    }
                }
            }
        }
        line = end.offset(1 as libc::c_int as isize);
    }
}
unsafe extern "C" fn add_urls(
    mut job: *mut JOB,
    mut urls: *mut wget_vector,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    wget_info_printf(
        dcgettext(
            0 as *const libc::c_char,
            b"found %d url(s) (base=%s)\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
        wget_vector_size(urls),
        if !base.is_null() { (*base).uri } else { 0 as *const libc::c_char },
    );
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(urls) {
        let mut url: *mut wget_string = wget_vector_get(urls, it) as *mut wget_string;
        if !(normalize_uri(base, url, encoding, &mut buf) != 0) {
            if base.is_null() && buf.length == 0 {
                wget_info_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"URL '%.*s' not followed (missing base URI)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    (*url).len as libc::c_int,
                    (*url).p,
                );
            } else {
                wget_thread_mutex_lock(known_urls_mutex);
                let mut rc: libc::c_int = wget_hashmap_put(
                    known_urls,
                    wget_strmemdup(buf.data as *const libc::c_void, buf.length)
                        as *const libc::c_void,
                    0 as *const libc::c_void,
                );
                wget_thread_mutex_unlock(known_urls_mutex);
                if rc != 0 {
                    wget_info_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"URL '%.*s' not followed (already known)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*url).len as libc::c_int,
                        (*url).p,
                    );
                } else {
                    queue_url_from_remote(
                        job,
                        encoding,
                        buf.data,
                        0 as libc::c_int,
                        0 as *const libc::c_char,
                    );
                }
            }
        }
        it += 1;
        it;
    }
    wget_buffer_deinit(&mut buf);
}
unsafe extern "C" fn atom_parse(
    mut job: *mut JOB,
    mut data: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut urls: *mut wget_vector = 0 as *mut wget_vector;
    wget_atom_get_urls_inline(data, &mut urls);
    add_urls(job, urls, encoding, base);
    wget_vector_free(&mut urls);
}
unsafe extern "C" fn atom_parse_localfile(
    mut job: *mut JOB,
    mut fname: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = wget_read_file(fname, 0 as *mut size_t);
    if !data.is_null() {
        atom_parse(job, data, encoding, base);
    }
    if !data.is_null() {
        wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
        data = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn rss_parse(
    mut job: *mut JOB,
    mut data: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut urls: *mut wget_vector = 0 as *mut wget_vector;
    wget_rss_get_urls_inline(data, &mut urls);
    add_urls(job, urls, encoding, base);
    wget_vector_free(&mut urls);
}
unsafe extern "C" fn rss_parse_localfile(
    mut job: *mut JOB,
    mut fname: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = wget_read_file(fname, 0 as *mut size_t);
    if !data.is_null() {
        rss_parse(job, data, encoding, base);
    }
    if !data.is_null() {
        wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
        data = 0 as *mut libc::c_char;
    }
}
unsafe extern "C" fn metalink_parse_localfile(mut fname: *const libc::c_char) {
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = wget_read_file(fname, 0 as *mut size_t);
    if !data.is_null() {
        let mut metalink: *mut wget_metalink = wget_metalink_parse(data);
        if (*metalink).size <= 0 as libc::c_int as off_t {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid file length %llu\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*metalink).size as libc::c_ulonglong,
            );
            wget_metalink_free(&mut metalink);
        } else if ((*metalink).mirrors).is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"No download mirrors found\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            wget_metalink_free(&mut metalink);
        } else {
            let mut job: JOB = {
                let mut init = JOB {
                    challenges_alloc_inuse_done_sitemap_robotstxt_head_first_requested_by_user_ignore_patterns_http_fallback_recursive_send_head_redirect_get: [0; 2],
                    c2rust_padding: [0; 6],
                    iri: 0 as *const wget_iri,
                    original_url: 0 as *const wget_iri,
                    referer: 0 as *const wget_iri,
                    metalink: metalink,
                    challenges: 0 as *mut wget_vector,
                    proxy_challenges: 0 as *mut wget_vector,
                    parts: 0 as *mut wget_vector,
                    remaining_sig_ext: 0 as *mut wget_list,
                    host: 0 as *mut HOST,
                    blacklist_entry: 0 as *const blacklist_entry,
                    sig_filename: 0 as *mut libc::c_char,
                    sig_req: 0 as *mut libc::c_char,
                    part: 0 as *mut PART,
                    downloader: 0 as *mut DOWNLOADER,
                    used_by: 0,
                    id: 0,
                    parent_id: 0,
                    retry_ts: 0,
                    level: 0,
                    redirection_level: 0,
                    auth_failure_count: 0,
                    mirror_pos: 0,
                    piece_pos: 0,
                    failures: 0,
                };
                init.set_challenges_alloc(false);
                init.set_inuse(false);
                init.set_done(false);
                init.set_sitemap(false);
                init.set_robotstxt(false);
                init.set_head_first(false);
                init.set_requested_by_user(false);
                init.set_ignore_patterns(false);
                init.set_http_fallback(false);
                init.set_recursive_send_head(false);
                init.set_redirect_get(false);
                init
            };
            if job_validate_file(&mut job) == 0 {
                wget_metalink_sort_mirrors(metalink);
                let mut mirror: *mut wget_metalink_mirror = wget_vector_get(
                    (*metalink).mirrors,
                    0 as libc::c_int,
                ) as *mut wget_metalink_mirror;
                let mut host: *mut HOST = 0 as *mut HOST;
                host = host_add((*mirror).iri);
                if host.is_null() {
                    host = host_get((*mirror).iri);
                }
                host_add_job(host, &mut job);
            } else {
                wget_metalink_free(&mut metalink);
            }
        }
        if !data.is_null() {
            wget_free.expect("non-null function pointer")(data as *mut libc::c_void);
            data = 0 as *mut libc::c_char;
        }
    }
}
unsafe extern "C" fn css_parse_encoding(
    mut context: *mut libc::c_void,
    mut encoding: *const libc::c_char,
    mut len: size_t,
) {
    let mut ctx: *mut css_context = context as *mut css_context;
    if (*ctx).encoding_allocated == 0
        && wget_strncasecmp_ascii((*ctx).encoding, encoding, len) != 0
    {
        (*ctx).encoding = wget_strmemdup(encoding as *const libc::c_void, len);
        (*ctx).encoding_allocated = 1 as libc::c_int as libc::c_char;
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URI content encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*ctx).encoding,
        );
    }
}
unsafe extern "C" fn css_parse_uri(
    mut context: *mut libc::c_void,
    mut url: *const libc::c_char,
    mut len: size_t,
    mut pos: size_t,
) {
    let mut ctx: *mut css_context = context as *mut css_context;
    let mut u: wget_string = {
        let mut init = wget_string { p: url, len: len };
        init
    };
    if normalize_uri((*ctx).base, &mut u, (*ctx).encoding, &mut (*ctx).uri_buf) != 0 {
        return;
    }
    if ((*ctx).base).is_null() && (*ctx).uri_buf.length == 0 {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URL '%.*s' not followed (missing base URI)\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            len as libc::c_int,
            url,
        );
    } else {
        queue_url_from_remote(
            (*ctx).job,
            (*ctx).encoding,
            (*ctx).uri_buf.data,
            (1 as libc::c_int) << 3 as libc::c_int,
            0 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn css_parse(
    mut job: *mut JOB,
    mut data: *const libc::c_char,
    mut len: size_t,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut context: css_context = {
        let mut init = css_context {
            job: job,
            base: base,
            encoding: encoding,
            uri_buf: wget_buffer {
                data: 0 as *mut libc::c_char,
                length: 0,
                size: 0,
                release_data_release_buf_error: [0; 1],
                c2rust_padding: [0; 7],
            },
            encoding_allocated: 0,
        };
        init
    };
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    wget_buffer_init(
        &mut context.uri_buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if !encoding.is_null() {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URI content encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            encoding,
        );
    }
    wget_css_parse_buffer(
        data,
        len,
        Some(
            css_parse_uri
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        Some(
            css_parse_encoding
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                ) -> (),
        ),
        &mut context as *mut css_context as *mut libc::c_void,
    );
    if context.encoding_allocated != 0 {
        if !(context.encoding).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(context.encoding as *mut libc::c_void);
            context.encoding = 0 as *const libc::c_char;
        }
    }
    wget_buffer_deinit(&mut context.uri_buf);
}
unsafe extern "C" fn css_parse_localfile(
    mut job: *mut JOB,
    mut fname: *const libc::c_char,
    mut encoding: *const libc::c_char,
    mut base: *const wget_iri,
) {
    let mut context: css_context = {
        let mut init = css_context {
            job: job,
            base: base,
            encoding: encoding,
            uri_buf: wget_buffer {
                data: 0 as *mut libc::c_char,
                length: 0,
                size: 0,
                release_data_release_buf_error: [0; 1],
                c2rust_padding: [0; 7],
            },
            encoding_allocated: 0,
        };
        init
    };
    let mut sbuf: [libc::c_char; 1024] = [0; 1024];
    wget_buffer_init(
        &mut context.uri_buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    if !encoding.is_null() {
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"URI content encoding = '%s'\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            encoding,
        );
    }
    wget_css_parse_file(
        fname,
        Some(
            css_parse_uri
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                    size_t,
                ) -> (),
        ),
        Some(
            css_parse_encoding
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                ) -> (),
        ),
        &mut context as *mut css_context as *mut libc::c_void,
    );
    if context.encoding_allocated != 0 {
        if !(context.encoding).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )(context.encoding as *mut libc::c_void);
            context.encoding = 0 as *const libc::c_char;
        }
    }
    wget_buffer_deinit(&mut context.uri_buf);
}
unsafe extern "C" fn get_file_size(mut fname: *const libc::c_char) -> libc::c_longlong {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(fname, &mut st) == 0 as libc::c_int {
        return st.st_size as libc::c_longlong;
    }
    return -(1 as libc::c_int) as libc::c_longlong;
}
unsafe extern "C" fn get_file_mtime(mut fname: *const libc::c_char) -> int64_t {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(fname, &mut st) == 0 as libc::c_int {
        return st.st_mtim.tv_sec;
    }
    return 0 as libc::c_int as int64_t;
}
unsafe extern "C" fn get_file_lmtime(mut fname: *const libc::c_char) -> int64_t {
    let mut ret: int64_t = 0 as libc::c_int as int64_t;
    let mut fp: *mut FILE = 0 as *mut FILE;
    fp = rpl_fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        let mut tbuf: [libc::c_char; 32] = [0; 32];
        if read_xattr_metadata(
            b"user.last_modified\0" as *const u8 as *const libc::c_char,
            tbuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            fileno(fp),
        ) > 0 as libc::c_int
        {
            ret = atoll(tbuf.as_mut_ptr()) as int64_t;
        }
        rpl_fclose(fp);
    }
    if ret == 0 {
        ret = get_file_mtime(fname);
    }
    return ret;
}
unsafe extern "C" fn set_file_mtime(mut fd: libc::c_int, mut modified: int64_t) {
    let mut timespecs: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    let mut tt: time_t = 0;
    gettime(&mut *timespecs.as_mut_ptr().offset(0 as libc::c_int as isize));
    tt = modified;
    timespecs[1 as libc::c_int as usize].tv_sec = tt;
    timespecs[1 as libc::c_int as usize].tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    if isatty(fd) == 0
        && futimens(fd, timespecs.as_mut_ptr() as *const timespec) == -(1 as libc::c_int)
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to set file date (%d)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *__errno_location(),
        );
    }
}
unsafe extern "C" fn wa_open(
    mut fname: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
) -> libc::c_int {
    let mut fd: libc::c_int = open(fname, flags, mode);
    return fd;
}
unsafe extern "C" fn open_unique(
    mut fname: *const libc::c_char,
    mut flags: libc::c_int,
    mut mode: mode_t,
    mut multiple: libc::c_int,
    mut unique: *mut libc::c_char,
    mut unique_len: size_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    if unique_len != 0 && *unique.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        return wa_open(unique, flags, mode);
    }
    fd = wa_open(fname, flags, mode);
    if fd >= 0 as libc::c_int {
        return fd;
    }
    if config.keep_extension {
        let mut ext: *const libc::c_char = strrchr(fname, '.' as i32);
        if ext.is_null() {
            ext = fname.offset(strlen(fname) as isize);
        }
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < 99999 as libc::c_int && fd < 0 as libc::c_int
            && (multiple != 0 && *__errno_location() == 17 as libc::c_int
                || *__errno_location() == 21 as libc::c_int)
        {
            if wget_snprintf(
                unique,
                unique_len,
                b"%.*s_%d%s\0" as *const u8 as *const libc::c_char,
                ext.offset_from(fname) as libc::c_long as libc::c_int,
                fname,
                i,
                ext,
            ) >= unique_len
            {
                return -(1 as libc::c_int);
            }
            fd = wa_open(unique, flags, mode);
            i += 1;
            i;
        }
    } else {
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 < 99999 as libc::c_int && fd < 0 as libc::c_int
            && (multiple != 0 && *__errno_location() == 17 as libc::c_int
                || *__errno_location() == 21 as libc::c_int)
        {
            if wget_snprintf(
                unique,
                unique_len,
                b"%s.%d\0" as *const u8 as *const libc::c_char,
                fname,
                i_0,
            ) >= unique_len
            {
                return -(1 as libc::c_int);
            }
            fd = wa_open(unique, flags, mode);
            i_0 += 1;
            i_0;
        }
    }
    return fd;
}
unsafe extern "C" fn check_mime_list(
    mut list: *mut wget_vector,
    mut mime: *const libc::c_char,
) -> bool {
    let mut result: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < wget_vector_size(list) {
        let mut entry: *mut libc::c_char = wget_vector_get(list, i) as *mut libc::c_char;
        let mut exclude: bool = *entry as libc::c_int == '!' as i32;
        wget_debug_printf(
            b"mime check %s - %s\0" as *const u8 as *const libc::c_char,
            entry,
            mime,
        );
        entry = entry.offset(exclude as libc::c_int as isize);
        if !(strpbrk(entry, b"*?[]\0" as *const u8 as *const libc::c_char)).is_null()
            && fnmatch(entry, mime, (1 as libc::c_int) << 4 as libc::c_int) == 0
        {
            result = !exclude as libc::c_int as libc::c_char;
        } else if wget_strcasecmp(entry, mime) == 0 {
            result = !exclude as libc::c_int as libc::c_char;
        }
        i += 1;
        i;
    }
    wget_debug_printf(
        b"mime check %d\0" as *const u8 as *const libc::c_char,
        result as libc::c_int,
    );
    return result != 0;
}
unsafe extern "C" fn check_statuscode_list(
    mut codes: *mut wget_vector,
    mut code: *const libc::c_char,
) -> bool {
    return check_mime_list(codes, code);
}
unsafe extern "C" fn check_status_code_list(
    mut codes: *mut wget_vector,
    mut code: uint16_t,
) -> bool {
    let mut _code: [libc::c_char; 6] = [0; 6];
    wget_snprintf(
        _code.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        b"%hu\0" as *const u8 as *const libc::c_char,
        code as libc::c_int,
    );
    return check_statuscode_list(codes, _code.as_mut_ptr());
}
unsafe extern "C" fn is_file(mut fname: *const libc::c_char) -> bool {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return stat(fname, &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as __mode_t
            == 0o100000 as libc::c_int as __mode_t;
}
unsafe extern "C" fn is_directory(mut fname: *const libc::c_char) -> bool {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return stat(fname, &mut st) == 0 as libc::c_int
        && st.st_mode & 0o170000 as libc::c_int as __mode_t
            == 0o40000 as libc::c_int as __mode_t;
}
unsafe extern "C" fn prepare_file(
    mut resp: *mut wget_http_response,
    mut fname: *const libc::c_char,
    mut flag: libc::c_int,
    mut uri: *const wget_iri,
    mut original_url: *const wget_iri,
    mut ignore_patterns: libc::c_int,
    mut partial_content: *mut wget_buffer,
    mut max_partial_content: size_t,
    mut actual_file_name: *mut *mut libc::c_char,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut job: *mut JOB = (*(*resp).req).user_data as *mut JOB;
    let mut alloced_fname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fd: libc::c_int = 0;
    let mut multiple: libc::c_int = 0 as libc::c_int;
    let mut oflag: libc::c_int = flag;
    let mut fname_length: size_t = 0;
    let mut old_quota: libc::c_longlong = 0;
    if fname.is_null() {
        return -(1 as libc::c_int);
    }
    if config.spider {
        wget_debug_printf(
            b"not saved '%s' (spider mode enabled)\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        return -(1 as libc::c_int);
    }
    if !(config.mime_types).is_null()
        && !check_mime_list(
            config.mime_types,
            (if !((*resp).content_type).is_null() {
                (*resp).content_type
            } else {
                b"application/octet-stream\0" as *const u8 as *const libc::c_char
            }),
        )
    {
        return -(2 as libc::c_int);
    }
    fname_length = strlen(fname);
    if *fname.offset(fname_length.wrapping_sub(1 as libc::c_int as size_t) as isize)
        as libc::c_int == '/' as i32
    {
        wget_debug_printf(
            b"not saved '%s' (file is a directory)\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        return -(1 as libc::c_int);
    }
    old_quota = quota_modify_read(
        if config.save_headers as libc::c_int != 0 {
            (*(*resp).header).length
        } else {
            0 as libc::c_int as size_t
        },
    );
    if config.quota != 0 && old_quota >= config.quota {
        wget_debug_printf(
            b"not saved '%s' (quota of %lld reached)\n\0" as *const u8
                as *const libc::c_char,
            fname,
            config.quota,
        );
        return -(1 as libc::c_int);
    }
    if fname == config.output_document {
        if strcmp(fname, b"-\0" as *const u8 as *const libc::c_char) == 0 {
            if config.save_headers {
                let mut rc: ptrdiff_t = safe_write(
                    1 as libc::c_int,
                    (*(*resp).header).data as *const libc::c_void,
                    (*(*resp).header).length as idx_t,
                );
                if rc == -(1 as libc::c_int) as ptrdiff_t {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to write to STDOUT (%td, errno=%d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        rc,
                        *__errno_location(),
                    );
                    set_exit_status(EXIT_STATUS_IO);
                }
            }
            return dup(1 as libc::c_int);
        }
        if config.delete_after {
            wget_debug_printf(
                b"not saved '%s' (--delete-after)\n\0" as *const u8
                    as *const libc::c_char,
                fname,
            );
            return -(2 as libc::c_int);
        }
        if strcmp(fname, b"/dev/null\0" as *const u8 as *const libc::c_char) == 0 {
            return -(2 as libc::c_int);
        }
        flag = 0o2000 as libc::c_int;
    }
    if config.adjust_extension as libc::c_int != 0 && !((*resp).content_type).is_null() {
        let mut ext: *const libc::c_char = 0 as *const libc::c_char;
        if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"text/html\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/xhtml+xml\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if wget_match_tail_nocase(
                fname,
                b".html\0" as *const u8 as *const libc::c_char,
            ) == 0
                && wget_match_tail_nocase(
                    fname,
                    b".htm\0" as *const u8 as *const libc::c_char,
                ) == 0
            {
                alloced_fname = wget_aprintf(
                    b"%s%s\0" as *const u8 as *const libc::c_char,
                    fname,
                    b".html\0" as *const u8 as *const libc::c_char,
                );
                fname = alloced_fname;
            }
        } else if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"text/css\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ext = b".css\0" as *const u8 as *const libc::c_char;
        } else if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"application/atom+xml\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ext = b".atom\0" as *const u8 as *const libc::c_char;
        } else if wget_strcasecmp_ascii(
            (*resp).content_type,
            b"application/rss+xml\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            ext = b".rss\0" as *const u8 as *const libc::c_char;
        }
        if !ext.is_null() && wget_match_tail_nocase(fname, ext) == 0 {
            alloced_fname = wget_aprintf(
                b"%s%s\0" as *const u8 as *const libc::c_char,
                fname,
                ext,
            );
            fname = alloced_fname;
        }
    }
    if ignore_patterns == 0 && !config.filter_urls {
        if !(config.accept_patterns).is_null()
            && !in_pattern_list(config.accept_patterns, fname)
            || !(config.accept_regex).is_null()
                && regex_match(fname, config.accept_regex) == 0
        {
            wget_debug_printf(
                b"not saved '%s' (doesn't match accept pattern)\n\0" as *const u8
                    as *const libc::c_char,
                fname,
            );
            if !alloced_fname.is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(alloced_fname as *mut libc::c_void);
                alloced_fname = 0 as *mut libc::c_char;
            }
            return -(2 as libc::c_int);
        }
        if !(config.reject_patterns).is_null()
            && in_pattern_list(config.reject_patterns, fname) as libc::c_int != 0
            || !(config.reject_regex).is_null()
                && regex_match(fname, config.reject_regex) != 0
        {
            wget_debug_printf(
                b"not saved '%s' (matches reject pattern)\n\0" as *const u8
                    as *const libc::c_char,
                fname,
            );
            if !alloced_fname.is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(alloced_fname as *mut libc::c_void);
                alloced_fname = 0 as *mut libc::c_char;
            }
            return -(2 as libc::c_int);
        }
        if !(config.exclude_directories).is_null()
            && in_directory_pattern_list(config.exclude_directories, path) != 0
        {
            wget_debug_printf(
                b"not saved '%s' (directory excluded)\n\0" as *const u8
                    as *const libc::c_char,
                path,
            );
            if !alloced_fname.is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(alloced_fname as *mut libc::c_void);
                alloced_fname = 0 as *mut libc::c_char;
            }
            return -(2 as libc::c_int);
        }
    }
    wget_thread_mutex_lock(savefile_mutex);
    fname_length = fname_length.wrapping_add(16 as libc::c_int as size_t);
    if config.timestamping {
        if oflag == 0o1000 as libc::c_int {
            flag = 0o1000 as libc::c_int;
        }
    } else if !config.clobber
        || config.recursive as libc::c_int != 0 && config.directories as libc::c_int != 0
    {
        if oflag == 0o1000 as libc::c_int
            && (!(config.recursive as libc::c_int != 0
                && config.directories as libc::c_int != 0) || !config.clobber)
        {
            flag = 0o200 as libc::c_int;
        }
    } else if flag != 0o2000 as libc::c_int {
        multiple = 1 as libc::c_int;
        flag = 0o200 as libc::c_int;
        if config.backups != 0 {
            let mut src_size: size_t = fname_length
                .wrapping_add(1 as libc::c_int as size_t);
            let mut dst_size: size_t = fname_length
                .wrapping_add(1 as libc::c_int as size_t);
            let mut src: *mut libc::c_char = wget_malloc(src_size) as *mut libc::c_char;
            let mut dst: *mut libc::c_char = wget_malloc(dst_size) as *mut libc::c_char;
            let mut it: libc::c_int = config.backups;
            while it > 0 as libc::c_int {
                if it > 1 as libc::c_int {
                    wget_snprintf(
                        src,
                        src_size,
                        b"%s.%d\0" as *const u8 as *const libc::c_char,
                        fname,
                        it - 1 as libc::c_int,
                    );
                } else {
                    wget_strscpy(src, fname, src_size);
                }
                wget_snprintf(
                    dst,
                    dst_size,
                    b"%s.%d\0" as *const u8 as *const libc::c_char,
                    fname,
                    it,
                );
                if rename(src, dst) == -(1 as libc::c_int)
                    && *__errno_location() != 2 as libc::c_int
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to rename %s to %s (errno=%d)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        src,
                        dst,
                        *__errno_location(),
                    );
                }
                it -= 1;
                it;
            }
            if !src.is_null() {
                wget_free.expect("non-null function pointer")(src as *mut libc::c_void);
                src = 0 as *mut libc::c_char;
            }
            if !dst.is_null() {
                wget_free.expect("non-null function pointer")(dst as *mut libc::c_void);
                dst = 0 as *mut libc::c_char;
            }
        }
    }
    mkdir_path(fname as *mut libc::c_char, 1 as libc::c_int != 0);
    let mut unique_size: size_t = fname_length.wrapping_add(1 as libc::c_int as size_t);
    let mut unique: *mut libc::c_char = wget_malloc(unique_size) as *mut libc::c_char;
    *unique = 0 as libc::c_int as libc::c_char;
    if !partial_content.is_null() {
        let mut size: libc::c_longlong = get_file_size(
            if *unique.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
                unique as *const libc::c_char
            } else {
                fname
            },
        );
        if size >= 0 as libc::c_int as libc::c_longlong {
            fd = open_unique(
                fname,
                0 as libc::c_int | 0 as libc::c_int,
                0 as libc::c_int as mode_t,
                multiple,
                unique,
                unique_size,
            );
            if fd >= 0 as libc::c_int {
                if size as libc::c_ulonglong > max_partial_content as libc::c_ulonglong {
                    size = max_partial_content as libc::c_longlong;
                }
                wget_buffer_memset_append(
                    partial_content,
                    0 as libc::c_int as libc::c_char,
                    size as size_t,
                );
                let mut rc_0: ptrdiff_t = safe_read(
                    fd,
                    (*partial_content).data as *mut libc::c_void,
                    size as idx_t,
                );
                if rc_0 == -(1 as libc::c_int) as ptrdiff_t
                    || rc_0 as libc::c_longlong != size
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to load partial content from '%s' (errno=%d)\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        fname,
                        *__errno_location(),
                    );
                    set_exit_status(EXIT_STATUS_IO);
                }
                close(fd);
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to load partial content from '%s' (errno=%d)\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    fname,
                    *__errno_location(),
                );
                set_exit_status(EXIT_STATUS_IO);
            }
        }
    }
    if config.unlink as libc::c_int != 0 && flag == 0o1000 as libc::c_int {
        if unlink(fname) < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to unlink '%s' (errno=%d)\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                *__errno_location(),
            );
            set_exit_status(EXIT_STATUS_IO);
            if !unique.is_null() {
                wget_free
                    .expect("non-null function pointer")(unique as *mut libc::c_void);
                unique = 0 as *mut libc::c_char;
            }
            return -(1 as libc::c_int);
        }
    }
    fd = open_unique(
        fname,
        0o1 as libc::c_int | flag | 0o100 as libc::c_int | 0o4000 as libc::c_int
            | 0 as libc::c_int,
        (0o400 as libc::c_int | 0o200 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int
            | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
        multiple,
        unique,
        unique_size,
    );
    *actual_file_name = wget_strdup(
        if *unique.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
            unique as *const libc::c_char
        } else {
            fname
        },
    );
    if !unique.is_null() {
        wget_free.expect("non-null function pointer")(unique as *mut libc::c_void);
        unique = 0 as *mut libc::c_char;
    }
    if fd >= 0 as libc::c_int {
        let mut rc_1: ssize_t = 0;
        if config.hyperlink {
            let mut canon_file_name: *const libc::c_char = canonicalize_file_name(
                *actual_file_name,
            );
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Saving '\x1B]8;;file://%s%s\x1B\\%s\x1B]8;;\x1B\\'\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                config.hostname,
                canon_file_name,
                *actual_file_name,
            );
            if !canon_file_name.is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(canon_file_name as *mut libc::c_void);
                canon_file_name = 0 as *const libc::c_char;
            }
        } else {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Saving '%s'\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *actual_file_name,
            );
        }
        if config.save_headers {
            wget_buffer_memcat(
                (*resp).header,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            rc_1 = write(
                fd,
                (*(*resp).header).data as *const libc::c_void,
                (*(*resp).header).length,
            );
            if rc_1 != (*(*resp).header).length as ssize_t {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to write file %s (%zd, errno=%d)\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *actual_file_name,
                    rc_1,
                    *__errno_location(),
                );
                set_exit_status(EXIT_STATUS_IO);
            }
        }
    } else if fd == -(1 as libc::c_int) {
        if *__errno_location() == 17 as libc::c_int && is_file(fname) as libc::c_int != 0
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"File '%s' already there; not retrieving.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
            if config.page_requisites as libc::c_int != 0 && !config.clobber {
                parse_localfile(
                    job,
                    (*(*job).blacklist_entry).local_filename,
                    config.remote_encoding,
                    (*resp).content_type,
                    (*job).iri,
                );
            }
        } else if *__errno_location() == 21 as libc::c_int
            || is_directory(fname) as libc::c_int != 0
        {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Directory / file name clash - not saving '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
            );
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to open '%s' (%d)\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                fname,
                *__errno_location(),
            );
            set_exit_status(EXIT_STATUS_IO);
        }
    }
    if config.xattr {
        let mut fp: *mut FILE = 0 as *mut FILE;
        fp = rpl_fopen(*actual_file_name, b"ab\0" as *const u8 as *const libc::c_char);
        if !fp.is_null() {
            set_file_metadata(
                uri,
                original_url,
                (*resp).content_type,
                (*resp).content_type_encoding,
                if (*resp).last_modified != 0 {
                    (*resp).last_modified - 1 as libc::c_int as int64_t
                } else {
                    0 as libc::c_int as int64_t
                },
                fp,
            );
            rpl_fclose(fp);
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Failed to save extended attribute %s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                *actual_file_name,
            );
            set_exit_status(EXIT_STATUS_IO);
        }
    }
    wget_thread_mutex_unlock(savefile_mutex);
    blacklist_set_filename(
        (*job).blacklist_entry as *mut blacklist_entry,
        *actual_file_name,
    );
    if !alloced_fname.is_null() {
        wget_free
            .expect("non-null function pointer")(alloced_fname as *mut libc::c_void);
        alloced_fname = 0 as *mut libc::c_char;
    }
    return fd;
}
unsafe extern "C" fn get_requested_range(
    mut ctx: *mut libc::c_void,
    mut elem: *mut libc::c_void,
) -> libc::c_int {
    let mut param: *mut wget_http_header_param = elem as *mut wget_http_header_param;
    let mut ret: *mut libc::c_longlong = ctx as *mut libc::c_longlong;
    if strcmp((*param).name, b"Range\0" as *const u8 as *const libc::c_char) == 0 {
        *ret = atoll(((*param).value).offset(6 as libc::c_int as isize));
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn get_header(
    mut resp: *mut wget_http_response,
    mut context: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ctx: *mut body_callback_context = context as *mut body_callback_context;
    let mut part: *mut PART = 0 as *mut PART;
    let mut dest: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut name_allocated: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut metalink: bool = config.metalink as libc::c_int != 0
        && !((*resp).content_type).is_null()
        && (wget_strcasecmp_ascii(
            (*resp).content_type,
            b"application/metalink4+xml\0" as *const u8 as *const libc::c_char,
        ) == 0
            || wget_strcasecmp_ascii(
                (*resp).content_type,
                b"application/metalink+xml\0" as *const u8 as *const libc::c_char,
            ) == 0);
    if (*(*ctx).job).head_first() as libc::c_int != 0
        || config.metalink as libc::c_int != 0 && metalink as libc::c_int != 0
    {
        name = (*(*(*ctx).job).blacklist_entry).local_filename;
        current_block = 12147880666119273379;
    } else {
        part = (*(*ctx).job).part;
        if !part.is_null() {
            name = (*(*(*ctx).job).metalink).name;
            (*ctx)
                .outfd = open(
                (*(*(*ctx).job).metalink).name,
                0o1 as libc::c_int | 0o100 as libc::c_int | 0o4000 as libc::c_int
                    | 0 as libc::c_int,
                0o400 as libc::c_int | 0o200 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int
                    | 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
            );
            if (*ctx).outfd == -(1 as libc::c_int) {
                set_exit_status(EXIT_STATUS_IO);
                ret = -(1 as libc::c_int);
                current_block = 10171818103386463783;
            } else if lseek((*ctx).outfd, (*part).position, 0 as libc::c_int)
                == -(1 as libc::c_int) as off_t
            {
                close((*ctx).outfd);
                set_exit_status(EXIT_STATUS_IO);
                ret = -(1 as libc::c_int);
                current_block = 10171818103386463783;
            } else {
                current_block = 12147880666119273379;
            }
        } else {
            if config.content_disposition as libc::c_int != 0
                && !((*resp).content_filename).is_null()
            {
                let mut iri: wget_iri = {
                    let mut init = wget_iri_st {
                        port_given_host_allocated_path_allocated_query_allocated_fragment_allocated_is_ip_address: [0; 1],
                        c2rust_padding: [0; 7],
                        uri: 0 as *const libc::c_char,
                        safe_uri: 0 as *const libc::c_char,
                        display: 0 as *const libc::c_char,
                        userinfo: 0 as *const libc::c_char,
                        password: 0 as *const libc::c_char,
                        host: (*(*(*ctx).job).iri).host,
                        path: (*resp).content_filename,
                        query: 0 as *const libc::c_char,
                        fragment: 0 as *const libc::c_char,
                        connection_part: 0 as *const libc::c_char,
                        dirlen: 0,
                        msize: 0,
                        port: 0,
                        scheme: (*(*(*ctx).job).iri).scheme,
                    };
                    init.set_port_given(false);
                    init.set_host_allocated(false);
                    init.set_path_allocated(false);
                    init.set_query_allocated(false);
                    init.set_fragment_allocated(false);
                    init.set_is_ip_address(false);
                    init
                };
                name_allocated = get_local_filename(&mut iri);
                dest = name_allocated;
                name = dest;
            } else if (config.output_document).is_null() {
                dest = (*(*(*ctx).job).blacklist_entry).local_filename;
                name = 0 as *const libc::c_char;
            } else {
                name = config.output_document;
                dest = name;
            }
            current_block = 12147880666119273379;
        }
    }
    match current_block {
        12147880666119273379 => {
            if !dest.is_null()
                && (!(config.save_content_on).is_null()
                    && check_status_code_list(
                        config.save_content_on,
                        (*resp).code as uint16_t,
                    ) as libc::c_int != 0
                    || (config.save_content_on).is_null()
                        && ((*resp).code as libc::c_int == 200 as libc::c_int
                            || (*resp).code as libc::c_int == 206 as libc::c_int
                            || config.content_on_error as libc::c_int != 0))
            {
                if !((*(*ctx).job).sig_filename).is_null() {
                    wget_free
                        .expect(
                            "non-null function pointer",
                        )((*(*ctx).job).sig_filename as *mut libc::c_void);
                    (*(*ctx).job).sig_filename = 0 as *mut libc::c_char;
                }
                (*ctx)
                    .outfd = prepare_file(
                    resp,
                    dest,
                    if (*resp).code as libc::c_int == 206 as libc::c_int {
                        0o2000 as libc::c_int
                    } else {
                        0o1000 as libc::c_int
                    },
                    (*(*ctx).job).iri,
                    (*(*ctx).job).original_url,
                    (*(*ctx).job).ignore_patterns() as libc::c_int,
                    if (*resp).code as libc::c_int == 206 as libc::c_int {
                        (*ctx).body
                    } else {
                        0 as *mut wget_buffer
                    },
                    (*ctx).max_memory,
                    &mut (*(*ctx).job).sig_filename,
                    (*(*(*ctx).job).iri).path,
                );
                if (*ctx).outfd == -(1 as libc::c_int) {
                    ret = -(1 as libc::c_int);
                }
            }
        }
        _ => {}
    }
    if config.progress as libc::c_int == 1 as libc::c_int {
        let mut filename: *const libc::c_char = 0 as *const libc::c_char;
        if name.is_null() {
            filename = (*(*(*ctx).job).blacklist_entry).local_filename;
            if !filename.is_null()
                && {
                    name = strrchr(filename, '/' as i32);
                    !name.is_null()
                }
            {
                name = name.offset(1 as libc::c_int as isize);
            } else {
                name = filename;
            }
        }
        if wget_strcasecmp_ascii(
            ((*(*resp).req).method).as_mut_ptr(),
            b"HEAD\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !((*resp).header).is_null() {
                bar_slot_begin(
                    (*ctx).progress_slot,
                    name,
                    0 as libc::c_int,
                    (*(*resp).header).length as ssize_t,
                );
                bar_set_downloaded((*ctx).progress_slot, (*(*resp).header).length);
            }
        } else if config.continue_download as libc::c_int != 0
            && (*resp).code as libc::c_int == 206 as libc::c_int
        {
            let mut already_downloaded: libc::c_longlong = 0;
            wget_vector_browse(
                (*(*resp).req).headers,
                Some(
                    get_requested_range
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                &mut already_downloaded as *mut libc::c_longlong as *mut libc::c_void,
            );
            bar_slot_begin(
                (*ctx).progress_slot,
                name,
                1 as libc::c_int,
                ((*resp).content_length as libc::c_ulonglong)
                    .wrapping_add(already_downloaded as libc::c_ulonglong) as ssize_t,
            );
            bar_set_downloaded((*ctx).progress_slot, already_downloaded as size_t);
        } else {
            bar_slot_begin(
                (*ctx).progress_slot,
                name,
                if (*resp).code as libc::c_int == 200 as libc::c_int
                    || (*resp).code as libc::c_int == 206 as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                (*resp).content_length as ssize_t,
            );
        }
    }
    if !name_allocated.is_null() {
        wget_free
            .expect("non-null function pointer")(name_allocated as *mut libc::c_void);
        name_allocated = 0 as *mut libc::c_char;
    }
    return ret;
}
unsafe extern "C" fn limit_transfer_rate(
    mut ctx: *mut body_callback_context,
    mut read_bytes: size_t,
) {
    let mut sleep_ms: libc::c_long = 0;
    let mut elapsed_ms: libc::c_long = 0;
    let mut curr_time_ms: libc::c_longlong = 0;
    let mut thread_rate_limit: libc::c_longlong = 0;
    if nthreads > 1 as libc::c_int {
        thread_rate_limit = config.limit_rate / nthreads as libc::c_longlong;
    } else {
        thread_rate_limit = config.limit_rate;
    }
    (*ctx).limit_debt_bytes += read_bytes as libc::c_longlong;
    curr_time_ms = wget_get_timemillis();
    if (*ctx).limit_prev_time_ms != 0 as libc::c_int as libc::c_longlong {
        elapsed_ms = (curr_time_ms - (*ctx).limit_prev_time_ms) as libc::c_long;
        (*ctx).limit_debt_bytes
            -= elapsed_ms as libc::c_longlong * thread_rate_limit
                / 1000 as libc::c_int as libc::c_longlong;
    }
    if (*ctx).limit_debt_bytes <= 0 as libc::c_int as libc::c_longlong {
        (*ctx).limit_debt_bytes = 0 as libc::c_int as libc::c_longlong;
        (*ctx).limit_prev_time_ms = curr_time_ms;
        return;
    }
    sleep_ms = ((*ctx).limit_debt_bytes * 1000 as libc::c_int as libc::c_longlong
        / thread_rate_limit) as libc::c_long;
    wget_millisleep(sleep_ms as libc::c_int);
    (*ctx).limit_prev_time_ms = wget_get_timemillis();
    elapsed_ms = ((*ctx).limit_prev_time_ms - curr_time_ms) as libc::c_long;
    (*ctx)
        .limit_debt_bytes = (sleep_ms - elapsed_ms) as libc::c_longlong
        * thread_rate_limit / 1000 as libc::c_int as libc::c_longlong;
}
unsafe extern "C" fn get_body(
    mut resp: *mut wget_http_response,
    mut context: *mut libc::c_void,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut ctx: *mut body_callback_context = context as *mut body_callback_context;
    if (*ctx).length == 0 as libc::c_int as uint64_t {
        if config.server_response {
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"# got header %zu bytes:\n%s\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*(*resp).header).length,
                (*(*resp).header).data,
            );
        }
    }
    (*ctx)
        .length = ((*ctx).length as libc::c_ulong).wrapping_add(length) as uint64_t
        as uint64_t;
    if (*ctx).outfd >= 0 as libc::c_int {
        let mut written: ptrdiff_t = safe_write(
            (*ctx).outfd,
            data as *const libc::c_void,
            length as idx_t,
        );
        if written == -(1 as libc::c_int) as ptrdiff_t {
            if *__errno_location() == 11 as libc::c_int && !terminate {
                if wget_ready_2_write((*ctx).outfd, 1000 as libc::c_int)
                    > 0 as libc::c_int
                {
                    written = safe_write(
                        (*ctx).outfd,
                        data as *const libc::c_void,
                        length as idx_t,
                    );
                }
            }
        }
        if written == -(1 as libc::c_int) as ptrdiff_t {
            if !terminate {
                wget_debug_printf(
                    b"Failed to write errno=%d\n\0" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
            }
            set_exit_status(EXIT_STATUS_IO);
            return -(1 as libc::c_int);
        }
    }
    if (*ctx).max_memory == 0 as libc::c_int as uint64_t
        || (*ctx).length < (*ctx).max_memory
    {
        wget_buffer_memcat((*ctx).body, data as *const libc::c_void, length);
    }
    if config.progress as libc::c_int == 1 as libc::c_int {
        bar_set_downloaded(
            (*ctx).progress_slot,
            ((*resp).cur_downloaded).wrapping_sub((*resp).accounted_for),
        );
        (*resp).accounted_for = (*resp).cur_downloaded;
    }
    if config.limit_rate != 0 {
        limit_transfer_rate(ctx, length);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn add_authorize_header(
    mut req: *mut wget_http_request,
    mut challenges: *mut wget_vector,
    mut username: *const libc::c_char,
    mut password: *const libc::c_char,
    mut proxied: libc::c_int,
) {
    let mut selected_challenge: *mut wget_http_challenge = 0 as *mut wget_http_challenge;
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < wget_vector_size(challenges) {
        let mut challenge: *mut wget_http_challenge = wget_vector_get(challenges, it)
            as *mut wget_http_challenge;
        if wget_strcasecmp_ascii(
            (*challenge).auth_scheme,
            b"digest\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            selected_challenge = challenge;
            break;
        } else {
            if wget_strcasecmp_ascii(
                (*challenge).auth_scheme,
                b"basic\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if selected_challenge.is_null() {
                    selected_challenge = challenge;
                }
            }
            it += 1;
            it;
        }
    }
    if !selected_challenge.is_null() {
        if !username.is_null() {
            wget_http_add_credentials(
                req,
                selected_challenge,
                username,
                password,
                proxied,
            );
        } else if !(config.netrc_file).is_null() {
            wget_thread_mutex_lock(netrc_mutex);
            if (config.netrc_db).is_null() {
                config.netrc_db = wget_netrc_db_init(0 as *mut wget_netrc_db);
                let mut rc: libc::c_int = wget_netrc_db_load(
                    config.netrc_db,
                    config.netrc_file,
                );
                if rc < 0 as libc::c_int && *__errno_location() != 2 as libc::c_int {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Failed to open .netrc file '%s' (%d): %s\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        config.netrc_file,
                        *__errno_location(),
                        wget_strerror(rc as wget_error),
                    );
                }
            }
            wget_thread_mutex_unlock(netrc_mutex);
            let mut netrc: *mut wget_netrc = wget_netrc_get(
                config.netrc_db,
                (*req).esc_host.data,
            );
            if netrc.is_null() {
                netrc = wget_netrc_get(
                    config.netrc_db,
                    b"default\0" as *const u8 as *const libc::c_char,
                );
            }
            if !netrc.is_null() {
                wget_http_add_credentials(
                    req,
                    selected_challenge,
                    (*netrc).login,
                    (*netrc).password,
                    proxied,
                );
            } else {
                wget_http_add_credentials(
                    req,
                    selected_challenge,
                    username,
                    password,
                    proxied,
                );
            }
        } else {
            wget_http_add_credentials(
                req,
                selected_challenge,
                username,
                password,
                proxied,
            );
        }
    }
}
unsafe extern "C" fn http_create_request(
    mut iri: *const wget_iri,
    mut job: *mut JOB,
) -> *mut wget_http_request {
    let mut req: *mut wget_http_request = 0 as *mut wget_http_request;
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut sbuf: [libc::c_char; 256] = [0; 256];
    let mut method: *const libc::c_char = 0 as *const libc::c_char;
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    if (*job).redirect_get() as libc::c_int != 0
        && (*job).redirection_level > 0 as libc::c_int
    {
        method = b"GET\0" as *const u8 as *const libc::c_char;
    } else if !(config.method).is_null() {
        method = config.method;
    } else if (*job).head_first() {
        method = b"HEAD\0" as *const u8 as *const libc::c_char;
    } else if !(config.post_data).is_null() || !(config.post_file).is_null() {
        method = b"POST\0" as *const u8 as *const libc::c_char;
    } else {
        method = b"GET\0" as *const u8 as *const libc::c_char;
    }
    req = wget_http_create_request(iri, method);
    if req.is_null() {
        return req;
    }
    if config.continue_download as libc::c_int != 0 || config.start_pos != 0
        || config.timestamping as libc::c_int != 0
            && config.if_modified_since as libc::c_int != 0
    {
        let mut local_filename: *const libc::c_char = if !(config.output_document)
            .is_null()
        {
            config.output_document
        } else {
            (*(*job).blacklist_entry).local_filename as *const libc::c_char
        };
        if (*job).robotstxt() as libc::c_int == 1 as libc::c_int {
            unlink(local_filename);
        }
        if config.continue_download {
            let mut file_size: libc::c_longlong = get_file_size(local_filename);
            if file_size >= 0 as libc::c_int as libc::c_longlong {
                wget_http_add_header_printf(
                    req,
                    b"Range\0" as *const u8 as *const libc::c_char,
                    b"bytes=%lld-\0" as *const u8 as *const libc::c_char,
                    file_size,
                );
            }
        }
        if config.start_pos != 0 {
            wget_http_add_header_printf(
                req,
                b"Range\0" as *const u8 as *const libc::c_char,
                b"bytes=%lld-\0" as *const u8 as *const libc::c_char,
                config.start_pos,
            );
        }
        if config.timestamping as libc::c_int != 0
            && config.if_modified_since as libc::c_int != 0
        {
            let mut mtime: int64_t = get_file_lmtime(local_filename);
            if mtime != 0 {
                let mut http_date: [libc::c_char; 32] = [0; 32];
                wget_http_print_date(
                    mtime,
                    http_date.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                );
                wget_http_add_header(
                    req,
                    b"If-Modified-Since\0" as *const u8 as *const libc::c_char,
                    http_date.as_mut_ptr(),
                );
            }
        }
    }
    wget_buffer_reset(&mut buf);
    if !(config.compression).is_null() {
        let mut it: libc::c_int = 0 as libc::c_int;
        while it
            < config
                .compression_methods[wget_content_encoding_max as libc::c_int as usize]
                as libc::c_int
        {
            let mut encoding_method: *const libc::c_char = wget_content_encoding_to_name(
                config.compression_methods[it as usize],
            );
            if buf.length != 0 {
                wget_buffer_strcat(
                    &mut buf,
                    b", \0" as *const u8 as *const libc::c_char,
                );
            }
            wget_buffer_strcat(&mut buf, encoding_method);
            it += 1;
            it;
        }
        if buf.length != 0 {
            wget_http_add_header(
                req,
                b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
                buf.data,
            );
        }
    }
    if !config.no_compression && (config.compression).is_null()
        || !(config.compression).is_null() && buf.length == 0
    {
        wget_buffer_strcat(
            &mut buf,
            if buf.length != 0 {
                b", gzip, deflate\0" as *const u8 as *const libc::c_char
            } else {
                b"gzip, deflate\0" as *const u8 as *const libc::c_char
            },
        );
        wget_buffer_strcat(
            &mut buf,
            if buf.length != 0 {
                b", zstd\0" as *const u8 as *const libc::c_char
            } else {
                b"zstd\0" as *const u8 as *const libc::c_char
            },
        );
        if buf.length == 0 {
            wget_buffer_strcat(
                &mut buf,
                b"identity\0" as *const u8 as *const libc::c_char,
            );
        }
        wget_http_add_header(
            req,
            b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
            buf.data,
        );
    }
    if config.recursive as libc::c_int != 0 || config.page_requisites as libc::c_int != 0
    {
        wget_http_add_header(
            req,
            b"Accept\0" as *const u8 as *const libc::c_char,
            b"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        wget_http_add_header(
            req,
            b"Accept\0" as *const u8 as *const libc::c_char,
            b"*/*\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(config.user_agent).is_null() {
        wget_http_add_header(
            req,
            b"User-Agent\0" as *const u8 as *const libc::c_char,
            config.user_agent,
        );
    }
    if config.keep_alive {
        wget_http_add_header(
            req,
            b"Connection\0" as *const u8 as *const libc::c_char,
            b"keep-alive\0" as *const u8 as *const libc::c_char,
        );
    }
    if !config.cache {
        wget_http_add_header(
            req,
            b"Cache-Control\0" as *const u8 as *const libc::c_char,
            b"no-cache\0" as *const u8 as *const libc::c_char,
        );
        wget_http_add_header(
            req,
            b"Pragma\0" as *const u8 as *const libc::c_char,
            b"no-cache\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(config.referer).is_null() {
        wget_http_add_header(
            req,
            b"Referer\0" as *const u8 as *const libc::c_char,
            config.referer,
        );
    } else if !((*job).referer).is_null() {
        let mut referer: *const wget_iri = (*job).referer;
        wget_buffer_strcpy(&mut buf, wget_iri_scheme_get_name((*referer).scheme));
        wget_buffer_memcat(
            &mut buf,
            b"://\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
        if wget_ip_is_family((*referer).host, 2 as libc::c_int) {
            wget_buffer_memcat(
                &mut buf,
                b"[\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            wget_buffer_strcat(&mut buf, (*referer).host);
            wget_buffer_memcat(
                &mut buf,
                b"]\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        } else {
            wget_buffer_strcat(&mut buf, (*referer).host);
        }
        if (*referer).port_given() {
            wget_buffer_printf_append(
                &mut buf as *mut wget_buffer,
                b":%hu\0" as *const u8 as *const libc::c_char,
                (*referer).port as libc::c_int,
            );
        }
        wget_buffer_memcat(
            &mut buf,
            b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        wget_iri_get_escaped_resource(referer, &mut buf);
        wget_http_add_header(
            req,
            b"Referer\0" as *const u8 as *const libc::c_char,
            buf.data,
        );
    }
    if !((*job).challenges).is_null() {
        add_authorize_header(
            req,
            (*job).challenges,
            config.http_username,
            config.http_password,
            0 as libc::c_int,
        );
    } else if !((*job).proxy_challenges).is_null() {
        add_authorize_header(
            req,
            (*job).proxy_challenges,
            config.http_proxy_username,
            config.http_proxy_password,
            1 as libc::c_int,
        );
    }
    if !((*job).part).is_null() {
        wget_http_add_header_printf(
            req,
            b"Range\0" as *const u8 as *const libc::c_char,
            b"bytes=%llu-%llu\0" as *const u8 as *const libc::c_char,
            (*(*job).part).position as libc::c_ulonglong,
            ((*(*job).part).position as libc::c_ulonglong)
                .wrapping_add((*(*job).part).length as libc::c_ulonglong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulonglong),
        );
    }
    if config.cookies {
        let mut cookie_string: *const libc::c_char = 0 as *const libc::c_char;
        cookie_string = wget_cookie_create_request_header(config.cookie_db, iri);
        if !cookie_string.is_null() {
            wget_http_add_header(
                req,
                b"Cookie\0" as *const u8 as *const libc::c_char,
                cookie_string,
            );
            if !cookie_string.is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(cookie_string as *mut libc::c_void);
                cookie_string = 0 as *const libc::c_char;
            }
        }
    }
    if !(config.post_data).is_null() {
        let mut length: size_t = strlen(config.post_data);
        wget_http_request_set_body(
            req,
            b"application/x-www-form-urlencoded\0" as *const u8 as *const libc::c_char,
            wget_memdup(config.post_data as *const libc::c_void, length)
                as *mut libc::c_char,
            length,
        );
    } else if !(config.post_file).is_null() {
        let mut length_0: size_t = 0;
        let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
        data = wget_read_file(config.post_file, &mut length_0);
        if !data.is_null() {
            wget_http_request_set_body(
                req,
                b"application/x-www-form-urlencoded\0" as *const u8
                    as *const libc::c_char,
                data,
                length_0,
            );
        } else {
            wget_http_free_request(&mut req);
        }
    } else if !(config.body_data).is_null() {
        let mut length_1: size_t = strlen(config.body_data);
        wget_http_request_set_body(
            req,
            b"application/x-www-form-urlencoded\0" as *const u8 as *const libc::c_char,
            wget_memdup(config.body_data as *const libc::c_void, length_1)
                as *mut libc::c_char,
            length_1,
        );
    } else if !(config.body_file).is_null() {
        let mut length_2: size_t = 0;
        let mut data_0: *mut libc::c_char = 0 as *mut libc::c_char;
        data_0 = wget_read_file(config.body_file, &mut length_2);
        if !data_0.is_null() {
            wget_http_request_set_body(
                req,
                b"application/x-www-form-urlencoded\0" as *const u8
                    as *const libc::c_char,
                data_0,
                length_2,
            );
        } else {
            wget_http_free_request(&mut req);
        }
    }
    if !(config.headers).is_null() {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < wget_vector_size(config.headers) {
            let mut param: *mut wget_http_header_param = wget_vector_get(
                config.headers,
                i,
            ) as *mut wget_http_header_param;
            let mut replaced: libc::c_char = 0 as libc::c_int as libc::c_char;
            if wget_strcasecmp_ascii(
                (*param).name,
                b"Cookie\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < wget_vector_size((*req).headers) {
                    let mut h: *mut wget_http_header_param = wget_vector_get(
                        (*req).headers,
                        j,
                    ) as *mut wget_http_header_param;
                    if wget_strcasecmp_ascii((*param).name, (*h).name) == 0 {
                        if !((*h).name).is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )((*h).name as *mut libc::c_void);
                            (*h).name = 0 as *const libc::c_char;
                        }
                        if !((*h).value).is_null() {
                            wget_free
                                .expect(
                                    "non-null function pointer",
                                )((*h).value as *mut libc::c_void);
                            (*h).value = 0 as *const libc::c_char;
                        }
                        (*h).name = wget_strdup((*param).name);
                        (*h).value = wget_strdup((*param).value);
                        replaced = 1 as libc::c_int as libc::c_char;
                    }
                    j += 1;
                    j;
                }
            }
            if replaced == 0 {
                wget_http_add_header_param(req, param);
            }
            i += 1;
            i;
        }
    }
    wget_buffer_deinit(&mut buf);
    return req;
}
unsafe extern "C" fn http_send_request(
    mut iri: *const wget_iri,
    mut original_url: *const wget_iri,
    mut downloader: *mut DOWNLOADER,
) -> libc::c_int {
    let mut conn: *mut wget_http_connection = (*downloader).conn;
    if conn.is_null() {
        return WGET_E_UNKNOWN as libc::c_int;
    }
    let mut job: *mut JOB = (*downloader).job;
    let mut rc: libc::c_int = 0;
    if (*job).head_first() {
        print_status(
            downloader,
            b"[%d] Checking '%s' ...\n\0" as *const u8 as *const libc::c_char,
            (*downloader).id,
            (*iri).safe_uri,
        );
    } else if !((*job).part).is_null() {
        print_status(
            downloader,
            b"downloading part %d/%d (%lld-%lld) %s from %s\n\0" as *const u8
                as *const libc::c_char,
            (*(*job).part).id,
            wget_vector_size((*job).parts),
            (*(*job).part).position as libc::c_longlong,
            ((*(*job).part).position + (*(*job).part).length - 1 as libc::c_int as off_t)
                as libc::c_longlong,
            (*(*job).metalink).name,
            (*iri).host,
        );
    } else if config.progress as libc::c_int == 1 as libc::c_int {
        bar_print((*downloader).id, (*iri).safe_uri);
    } else {
        print_status(
            downloader,
            b"[%d] Downloading '%s' ...\n\0" as *const u8 as *const libc::c_char,
            (*downloader).id,
            (*iri).safe_uri,
        );
    }
    let mut req: *mut wget_http_request = http_create_request(iri, (*downloader).job);
    if req.is_null() {
        return WGET_E_UNKNOWN as libc::c_int;
    }
    wget_http_request_set_ptr(
        req,
        2019 as libc::c_int,
        (*downloader).job as *mut libc::c_void,
    );
    rc = wget_http_send_request(conn, req);
    if rc != 0 {
        wget_http_free_request(&mut req);
        return rc;
    }
    let mut context: *mut body_callback_context = wget_calloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<body_callback_context>() as libc::c_ulong,
    ) as *mut body_callback_context;
    (*context).job = (*downloader).job;
    (*context)
        .max_memory = if !((*(*downloader).job).part).is_null() {
        0 as libc::c_int as uint64_t
    } else {
        10 as libc::c_int as uint64_t
            * ((1 as libc::c_int) << 20 as libc::c_int) as uint64_t
    };
    (*context).outfd = -(1 as libc::c_int);
    (*context).body = wget_buffer_alloc(102400 as libc::c_int as size_t);
    (*context).length = 0 as libc::c_int as uint64_t;
    (*context).progress_slot = (*downloader).id;
    (*(*context).job).original_url = original_url;
    (*context).limit_debt_bytes = 0 as libc::c_int as libc::c_longlong;
    (*context).limit_prev_time_ms = wget_get_timemillis();
    wget_http_request_set_header_cb(
        req,
        Some(
            get_header
                as unsafe extern "C" fn(
                    *mut wget_http_response,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        context as *mut libc::c_void,
    );
    wget_http_request_set_body_cb(
        req,
        Some(
            get_body
                as unsafe extern "C" fn(
                    *mut wget_http_response,
                    *mut libc::c_void,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
        ),
        context as *mut libc::c_void,
    );
    wget_http_request_set_int(
        req,
        2009 as libc::c_int,
        (config.save_headers as libc::c_int != 0
            || config.server_response as libc::c_int != 0
            || config.progress as libc::c_int == 1 as libc::c_int
                && (config.spider as libc::c_int != 0 || config.chunk_size != 0))
            as libc::c_int,
    );
    wget_http_request_set_int(
        req,
        2020 as libc::c_int,
        config.ignore_length as libc::c_int,
    );
    return WGET_E_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn http_receive_response(
    mut conn: *mut wget_http_connection,
) -> *mut wget_http_response {
    if conn.is_null() {
        return 0 as *mut wget_http_response;
    }
    let mut resp: *mut wget_http_response = wget_http_get_response_cb(conn);
    if resp.is_null() {
        return 0 as *mut wget_http_response;
    }
    let mut context: *mut body_callback_context = (*(*resp).req).body_user_data
        as *mut body_callback_context;
    (*resp).body = (*context).body;
    if (*context).outfd >= 0 as libc::c_int {
        if (*resp).last_modified != 0 {
            if config.xattr as libc::c_int != 0 && !terminate {
                write_xattr_last_modified((*resp).last_modified, (*context).outfd);
            }
            if config.use_server_timestamps {
                set_file_mtime(
                    (*context).outfd,
                    (*resp).last_modified
                        - (terminate as libc::c_int != 0
                            || (*resp).length_inconsistent() as libc::c_int != 0)
                            as libc::c_int as int64_t,
                );
            }
        }
        if config.fsync_policy {
            if fsync((*context).outfd) < 0 as libc::c_int
                && *__errno_location() == 5 as libc::c_int
            {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Failed to fsync errno=%d\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    *__errno_location(),
                );
                set_exit_status(EXIT_STATUS_IO);
            }
        }
        close((*context).outfd);
        (*context).outfd = -(1 as libc::c_int);
    }
    if config.progress as libc::c_int == 1 as libc::c_int {
        bar_slot_deregister((*context).progress_slot);
    }
    if (*resp).length_inconsistent() {
        set_exit_status(EXIT_STATUS_PROTOCOL);
    }
    if !context.is_null() {
        wget_free.expect("non-null function pointer")(context as *mut libc::c_void);
        context = 0 as *mut body_callback_context;
    }
    return resp;
}
unsafe extern "C" fn write_xattr_metadata(
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    if !(!name.is_null() && !value.is_null() && fd >= 0 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = if fsetxattr(
        fd,
        name,
        value as *const libc::c_void,
        strlen(value),
        0 as libc::c_int,
    ) < 0 as libc::c_int
    {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    if rc != 0 {
        wget_debug_printf(
            b"Failed to set xattr %s.\n\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return rc;
}
unsafe extern "C" fn read_xattr_metadata(
    mut name: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut size: size_t,
    mut fd: libc::c_int,
) -> libc::c_int {
    if !(!name.is_null() && !value.is_null() && size != 0 && fd >= 0 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    let mut rc: libc::c_int = fgetxattr(
        fd,
        name,
        value as *mut libc::c_void,
        size.wrapping_sub(1 as libc::c_int as size_t),
    ) as libc::c_int;
    if rc < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if rc >= size as libc::c_int {
        rc = size.wrapping_sub(1 as libc::c_int as size_t) as libc::c_int;
    }
    *value.offset(rc as isize) = 0 as libc::c_int as libc::c_char;
    return rc;
}
unsafe extern "C" fn write_xattr_last_modified(
    mut last_modified: int64_t,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut tbuf: [libc::c_char; 32] = [0; 32];
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    wget_snprintf(
        tbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"%lld\0" as *const u8 as *const libc::c_char,
        last_modified as libc::c_longlong,
    );
    return write_xattr_metadata(
        b"user.last_modified\0" as *const u8 as *const libc::c_char,
        tbuf.as_mut_ptr(),
        fd,
    );
}
unsafe extern "C" fn set_file_metadata(
    mut origin_iri: *const wget_iri,
    mut referrer_iri: *const wget_iri,
    mut mime_type: *const libc::c_char,
    mut charset: *const libc::c_char,
    mut last_modified: int64_t,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    if origin_iri.is_null() || fp.is_null() {
        return -(1 as libc::c_int);
    }
    fd = fileno(fp);
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if write_xattr_metadata(
        b"user.mime_type\0" as *const u8 as *const libc::c_char,
        mime_type,
        fd,
    ) < 0 as libc::c_int && *__errno_location() == 95 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    write_xattr_metadata(
        b"user.charset\0" as *const u8 as *const libc::c_char,
        charset,
        fd,
    );
    let mut sbuf: [libc::c_char; 256] = [0; 256];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    wget_iri_get_connection_part(origin_iri, &mut buf);
    wget_buffer_memcat(
        &mut buf,
        b"/\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    wget_iri_get_escaped_resource(origin_iri, &mut buf);
    write_xattr_metadata(
        b"user.xdg.origin.url\0" as *const u8 as *const libc::c_char,
        buf.data,
        fd,
    );
    wget_buffer_reset(&mut buf);
    wget_iri_get_connection_part(referrer_iri, &mut buf);
    write_xattr_metadata(
        b"user.xdg.referrer.url\0" as *const u8 as *const libc::c_char,
        buf.data,
        fd,
    );
    wget_buffer_deinit(&mut buf);
    return write_xattr_last_modified(last_modified, fd);
}
unsafe extern "C" fn create_unique(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut fname: *mut libc::c_char = wget_strdup(name);
    let mut it: libc::c_int = 0 as libc::c_int;
    while it < 9999 as libc::c_int {
        if it != 0 {
            fname = wget_aprintf(
                b"%s.%d\0" as *const u8 as *const libc::c_char,
                name,
                it,
            );
        }
        let mut fd: libc::c_int = open(
            fname,
            0o100 as libc::c_int | 0o1 as libc::c_int | 0o200 as libc::c_int,
            0o600 as libc::c_int,
        );
        if fd != -(1 as libc::c_int) {
            close(fd);
            return fname;
        }
        if !fname.is_null() {
            wget_free.expect("non-null function pointer")(fname as *mut libc::c_void);
            fname = 0 as *mut libc::c_char;
        }
        it += 1;
        it;
    }
    return wget_strdup(name);
}
unsafe extern "C" fn fork_to_background() {
    let mut logfile_changed: bool = 0 as libc::c_int != 0;
    if (config.logfile).is_null()
        && (!config.quiet || config.server_response as libc::c_int != 0)
        && !config.dont_write
    {
        config
            .logfile = create_unique(b"wget-log\0" as *const u8 as *const libc::c_char);
        logfile_changed = wget_strcmp(
            config.logfile,
            b"wget-log\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int;
    }
    let mut pid: pid_t = fork();
    if pid < 0 as libc::c_int {
        wget_error_printf_exit(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to fork (%d)\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            *__errno_location(),
        );
    } else if pid != 0 as libc::c_int {
        wget_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Continuing in background, pid %d.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            pid,
        );
        if logfile_changed {
            wget_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Output will be written to %s.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                config.logfile,
            );
        }
        exit(EXIT_STATUS_NO_ERROR as libc::c_int);
    }
    setsid();
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        stdin,
    ))
        .is_null()
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to redirect stdin to /dev/null.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        stdout,
    ))
        .is_null()
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to redirect stdout to /dev/null.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    if (freopen(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        b"w\0" as *const u8 as *const libc::c_char,
        stderr,
    ))
        .is_null()
    {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Failed to redirect stderr to /dev/null.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *const libc::c_char,
            ) as i32,
        )
    }
}
