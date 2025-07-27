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
    pub type __dirstream;
    pub type stack_st;
    pub type stack_st_OPENSSL_STRING;
    pub type bio_st;
    pub type evp_md_st;
    pub type evp_md_ctx_st;
    pub type evp_pkey_st;
    pub type x509_st;
    pub type x509_store_st;
    pub type x509_store_ctx_st;
    pub type x509_lookup_st;
    pub type x509_lookup_method_st;
    pub type engine_st;
    pub type ssl_st;
    pub type ssl_ctx_st;
    pub type ocsp_response_st;
    pub type bio_method_st;
    pub type stack_st_X509;
    pub type ssl_method_st;
    pub type ssl_session_st;
    pub type ocsp_cert_id_st;
    pub type ocsp_one_request_st;
    pub type ocsp_request_st;
    pub type ocsp_single_response_st;
    pub type ocsp_basic_response_st;
    pub type wget_vector_st;
    pub type wget_thread_mutex_st;
    pub type wget_hpkp_db_st;
    pub type wget_hpkp_st;
    pub type wget_tls_session_st;
    pub type wget_tls_session_db_st;
    pub type wget_ocsp_db_st;
    pub type wget_dns_st;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn time(__timer: *mut time_t) -> time_t;
    fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> libc::c_int;
    fn OPENSSL_sk_value(_: *const OPENSSL_STACK, _: libc::c_int) -> *mut libc::c_void;
    fn ASN1_STRING_free(a: *mut ASN1_STRING);
    fn ASN1_TIME_diff(
        pday: *mut libc::c_int,
        psec: *mut libc::c_int,
        from: *const ASN1_TIME,
        to: *const ASN1_TIME,
    ) -> libc::c_int;
    fn ASN1_TIME_adj(
        s: *mut ASN1_TIME,
        t: time_t,
        offset_day: libc::c_int,
        offset_sec: libc::c_long,
    ) -> *mut ASN1_TIME;
    fn ASN1_GENERALIZEDTIME_print(
        fp: *mut BIO,
        a: *const ASN1_GENERALIZEDTIME,
    ) -> libc::c_int;
    fn ERR_peek_last_error() -> libc::c_ulong;
    fn ERR_reason_error_string(e: libc::c_ulong) -> *const libc::c_char;
    fn i2d_PUBKEY(a: *const EVP_PKEY, out: *mut *mut libc::c_uchar) -> libc::c_int;
    fn i2d_X509(a: *const X509, out: *mut *mut libc::c_uchar) -> libc::c_int;
    fn X509_get0_pubkey(x: *const X509) -> *mut EVP_PKEY;
    fn X509_check_issued(issuer: *mut X509, subject: *mut X509) -> libc::c_int;
    fn X509_email_free(sk: *mut stack_st_OPENSSL_STRING);
    fn X509_get1_ocsp(x: *mut X509) -> *mut stack_st_OPENSSL_STRING;
    fn CRYPTO_free(ptr: *mut libc::c_void, file: *const libc::c_char, line: libc::c_int);
    fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;
    fn BIO_read(b: *mut BIO, data: *mut libc::c_void, dlen: libc::c_int) -> libc::c_int;
    fn BIO_free_all(a: *mut BIO);
    fn BIO_s_mem() -> *const BIO_METHOD;
    fn EVP_MD_get_size(md: *const EVP_MD) -> libc::c_int;
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
    fn EVP_DigestInit_ex(
        ctx: *mut EVP_MD_CTX,
        type_0: *const EVP_MD,
        impl_0: *mut ENGINE,
    ) -> libc::c_int;
    fn EVP_DigestUpdate(
        ctx: *mut EVP_MD_CTX,
        d: *const libc::c_void,
        cnt: size_t,
    ) -> libc::c_int;
    fn EVP_DigestFinal_ex(
        ctx: *mut EVP_MD_CTX,
        md: *mut libc::c_uchar,
        s: *mut libc::c_uint,
    ) -> libc::c_int;
    fn EVP_sha1() -> *const EVP_MD;
    fn EVP_sha256() -> *const EVP_MD;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchrnul(__s: *const libc::c_char, __c: libc::c_int) -> *mut libc::c_char;
    fn X509_STORE_set_flags(ctx: *mut X509_STORE, flags: libc::c_ulong) -> libc::c_int;
    fn X509_STORE_add_lookup(
        v: *mut X509_STORE,
        m: *mut X509_LOOKUP_METHOD,
    ) -> *mut X509_LOOKUP;
    fn X509_LOOKUP_file() -> *mut X509_LOOKUP_METHOD;
    fn X509_load_crl_file(
        ctx: *mut X509_LOOKUP,
        file: *const libc::c_char,
        type_0: libc::c_int,
    ) -> libc::c_int;
    fn SSL_set_alpn_protos(
        ssl: *mut SSL,
        protos: *const libc::c_uchar,
        protos_len: libc::c_uint,
    ) -> libc::c_int;
    fn SSL_get0_alpn_selected(
        ssl: *const SSL,
        data: *mut *const libc::c_uchar,
        len: *mut libc::c_uint,
    );
    fn SSL_CTX_set_cipher_list(_: *mut SSL_CTX, str: *const libc::c_char) -> libc::c_int;
    fn SSL_CTX_new(meth: *const SSL_METHOD) -> *mut SSL_CTX;
    fn SSL_CTX_free(_: *mut SSL_CTX);
    fn SSL_CTX_get_cert_store(_: *const SSL_CTX) -> *mut X509_STORE;
    fn SSL_get_fd(s: *const SSL) -> libc::c_int;
    fn SSL_set_fd(s: *mut SSL, fd: libc::c_int) -> libc::c_int;
    fn SSL_SESSION_is_resumable(s: *const SSL_SESSION) -> libc::c_int;
    fn SSL_SESSION_free(ses: *mut SSL_SESSION);
    fn i2d_SSL_SESSION(
        in_0: *const SSL_SESSION,
        pp: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
    fn SSL_set_session(to: *mut SSL, session: *mut SSL_SESSION) -> libc::c_int;
    fn d2i_SSL_SESSION(
        a: *mut *mut SSL_SESSION,
        pp: *mut *const libc::c_uchar,
        length: libc::c_long,
    ) -> *mut SSL_SESSION;
    fn SSL_get_peer_cert_chain(s: *const SSL) -> *mut stack_st_X509;
    fn SSL_CTX_set_verify(ctx: *mut SSL_CTX, mode: libc::c_int, callback: SSL_verify_cb);
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn SSL_set1_host(s: *mut SSL, hostname: *const libc::c_char) -> libc::c_int;
    fn SSL_set_hostflags(s: *mut SSL, flags: libc::c_uint);
    fn SSL_free(ssl: *mut SSL);
    fn SSL_connect(ssl: *mut SSL) -> libc::c_int;
    fn SSL_read(ssl: *mut SSL, buf: *mut libc::c_void, num: libc::c_int) -> libc::c_int;
    fn SSL_write(
        ssl: *mut SSL,
        buf: *const libc::c_void,
        num: libc::c_int,
    ) -> libc::c_int;
    fn SSL_ctrl(
        ssl: *mut SSL,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_CTX_ctrl(
        ctx: *mut SSL_CTX,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn SSL_CTX_callback_ctrl(
        _: *mut SSL_CTX,
        _: libc::c_int,
        _: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_long;
    fn SSL_get_error(s: *const SSL, ret_code: libc::c_int) -> libc::c_int;
    fn TLS_client_method() -> *const SSL_METHOD;
    fn SSL_shutdown(s: *mut SSL) -> libc::c_int;
    fn SSL_version(ssl: *const SSL) -> libc::c_int;
    fn SSL_CTX_set_default_verify_paths(ctx: *mut SSL_CTX) -> libc::c_int;
    fn SSL_CTX_load_verify_locations(
        ctx: *mut SSL_CTX,
        CAfile: *const libc::c_char,
        CApath: *const libc::c_char,
    ) -> libc::c_int;
    fn SSL_get_session(ssl: *const SSL) -> *mut SSL_SESSION;
    fn SSL_get0_verified_chain(s: *const SSL) -> *mut stack_st_X509;
    fn SSL_set_ex_data(
        ssl: *mut SSL,
        idx: libc::c_int,
        data: *mut libc::c_void,
    ) -> libc::c_int;
    fn SSL_get_ex_data(ssl: *const SSL, idx: libc::c_int) -> *mut libc::c_void;
    fn SSL_session_reused(s: *const SSL) -> libc::c_int;
    fn OCSP_CERTID_dup(a: *const OCSP_CERTID) -> *mut OCSP_CERTID;
    fn OCSP_cert_to_id(
        dgst: *const EVP_MD,
        subject: *const X509,
        issuer: *const X509,
    ) -> *mut OCSP_CERTID;
    fn OCSP_request_add0_id(
        req: *mut OCSP_REQUEST,
        cid: *mut OCSP_CERTID,
    ) -> *mut OCSP_ONEREQ;
    fn OCSP_request_add1_nonce(
        req: *mut OCSP_REQUEST,
        val: *mut libc::c_uchar,
        len: libc::c_int,
    ) -> libc::c_int;
    fn OCSP_check_nonce(req: *mut OCSP_REQUEST, bs: *mut OCSP_BASICRESP) -> libc::c_int;
    fn OCSP_response_status(resp: *mut OCSP_RESPONSE) -> libc::c_int;
    fn OCSP_response_get1_basic(resp: *mut OCSP_RESPONSE) -> *mut OCSP_BASICRESP;
    fn OCSP_resp_get0(bs: *mut OCSP_BASICRESP, idx: libc::c_int) -> *mut OCSP_SINGLERESP;
    fn OCSP_single_get0_status(
        single: *mut OCSP_SINGLERESP,
        reason: *mut libc::c_int,
        revtime: *mut *mut ASN1_GENERALIZEDTIME,
        thisupd: *mut *mut ASN1_GENERALIZEDTIME,
        nextupd: *mut *mut ASN1_GENERALIZEDTIME,
    ) -> libc::c_int;
    fn OCSP_id_cmp(a: *const OCSP_CERTID, b: *const OCSP_CERTID) -> libc::c_int;
    fn OCSP_SINGLERESP_get0_id(x: *const OCSP_SINGLERESP) -> *const OCSP_CERTID;
    fn OCSP_BASICRESP_free(a: *mut OCSP_BASICRESP);
    fn OCSP_RESPONSE_free(a: *mut OCSP_RESPONSE);
    fn d2i_OCSP_RESPONSE(
        a: *mut *mut OCSP_RESPONSE,
        in_0: *mut *const libc::c_uchar,
        len: libc::c_long,
    ) -> *mut OCSP_RESPONSE;
    fn OCSP_CERTID_free(a: *mut OCSP_CERTID);
    fn OCSP_REQUEST_new() -> *mut OCSP_REQUEST;
    fn OCSP_REQUEST_free(a: *mut OCSP_REQUEST);
    fn i2d_OCSP_REQUEST(
        a: *const OCSP_REQUEST,
        out: *mut *mut libc::c_uchar,
    ) -> libc::c_int;
    fn OCSP_basic_verify(
        bs: *mut OCSP_BASICRESP,
        certs: *mut stack_st_X509,
        st: *mut X509_STORE,
        flags: libc::c_ulong,
    ) -> libc::c_int;
    fn wget_ready_2_transfer(
        fd: libc::c_int,
        timeout: libc::c_int,
        mode: libc::c_int,
    ) -> libc::c_int;
    fn wget_strcmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
    fn wget_strcasecmp_ascii(
        s1: *const libc::c_char,
        s2: *const libc::c_char,
    ) -> libc::c_int;
    fn wget_memtohex(
        src: *const libc::c_uchar,
        src_len: size_t,
        dst: *mut libc::c_char,
        dst_size: size_t,
    );
    fn wget_match_tail_nocase(
        s: *const libc::c_char,
        tail: *const libc::c_char,
    ) -> libc::c_int;
    static mut wget_malloc_fn: Option::<wget_malloc_function>;
    static mut wget_free: Option::<wget_free_function>;
    fn wget_strdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn wget_strmemdup(m: *const libc::c_void, n: size_t) -> *mut libc::c_char;
    fn wget_buffer_init(
        buf: *mut wget_buffer,
        data: *mut libc::c_char,
        size: size_t,
    ) -> libc::c_int;
    fn wget_buffer_deinit(buf: *mut wget_buffer);
    fn wget_buffer_memcat(
        buf: *mut wget_buffer,
        data: *const libc::c_void,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_memset_append(
        buf: *mut wget_buffer,
        c: libc::c_char,
        length: size_t,
    ) -> size_t;
    fn wget_buffer_printf(
        buf: *mut wget_buffer,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_snprintf(
        str: *mut libc::c_char,
        size: size_t,
        fmt: *const libc::c_char,
        _: ...
    ) -> size_t;
    fn wget_info_printf(fmt: *const libc::c_char, _: ...);
    fn wget_error_printf(fmt: *const libc::c_char, _: ...);
    fn wget_debug_printf(fmt: *const libc::c_char, _: ...);
    fn wget_vector_create(
        max: libc::c_int,
        cmp: Option::<wget_vector_compare_fn>,
    ) -> *mut wget_vector;
    fn wget_vector_set_resize_factor(v: *mut wget_vector, off: libc::c_float);
    fn wget_vector_find(v: *const wget_vector, elem: *const libc::c_void) -> libc::c_int;
    fn wget_vector_insert(
        v: *mut wget_vector,
        elem: *const libc::c_void,
        pos: libc::c_int,
    ) -> libc::c_int;
    fn wget_vector_free(v: *mut *mut wget_vector);
    fn wget_vector_get(v: *const wget_vector, pos: libc::c_int) -> *mut libc::c_void;
    fn wget_vector_set_destructor(
        v: *mut wget_vector,
        destructor: Option::<wget_vector_destructor>,
    );
    fn wget_thread_mutex_lock(mutex_0: wget_thread_mutex);
    fn wget_thread_mutex_unlock(mutex_0: wget_thread_mutex);
    fn wget_hpkp_db_check_pubkey(
        _: *mut wget_hpkp_db,
        _: *const libc::c_char,
        _: *const libc::c_void,
        _: size_t,
    ) -> libc::c_int;
    fn wget_tls_session_new(
        host: *const libc::c_char,
        maxage: int64_t,
        data: *const libc::c_void,
        data_size: size_t,
    ) -> *mut wget_tls_session;
    fn wget_tls_session_get(
        tls_session_db: *const wget_tls_session_db,
        host: *const libc::c_char,
        data: *mut *mut libc::c_void,
        size: *mut size_t,
    ) -> libc::c_int;
    fn wget_tls_session_db_add(
        tls_session_db: *mut wget_tls_session_db,
        tls_session: *mut wget_tls_session,
    );
    fn wget_ocsp_fingerprint_in_cache(
        _: *const wget_ocsp_db,
        _: *const libc::c_char,
        _: *mut libc::c_int,
    ) -> bool;
    fn wget_ocsp_db_add_fingerprint(
        _: *mut wget_ocsp_db,
        _: *const libc::c_char,
        _: int64_t,
        _: bool,
    );
    fn wget_tcp_get_tcp_fastopen(tcp: *mut wget_tcp) -> bool;
    fn wget_ssl_default_cert_dir() -> *const libc::c_char;
    fn wget_ssl_default_ca_bundle_path() -> *const libc::c_char;
    fn wget_http_free_response(resp: *mut *mut wget_http_response);
    fn wget_http_get(first_key: libc::c_int, _: ...) -> *mut wget_http_response;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type OPENSSL_STACK = stack_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
}
pub type ASN1_TIME = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_STRING = asn1_string_st;
pub type BIO = bio_st;
pub type EVP_MD = evp_md_st;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type EVP_PKEY = evp_pkey_st;
pub type X509 = x509_st;
pub type X509_STORE = x509_store_st;
pub type X509_STORE_CTX = x509_store_ctx_st;
pub type X509_LOOKUP = x509_lookup_st;
pub type X509_LOOKUP_METHOD = x509_lookup_method_st;
pub type ENGINE = engine_st;
pub type SSL = ssl_st;
pub type SSL_CTX = ssl_ctx_st;
pub type OCSP_RESPONSE = ocsp_response_st;
pub type BIO_METHOD = bio_method_st;
pub type SSL_METHOD = ssl_method_st;
pub type SSL_SESSION = ssl_session_st;
pub type SSL_verify_cb = Option::<
    unsafe extern "C" fn(libc::c_int, *mut X509_STORE_CTX) -> libc::c_int,
>;
pub type OCSP_CERTID = ocsp_cert_id_st;
pub type OCSP_ONEREQ = ocsp_one_request_st;
pub type OCSP_REQUEST = ocsp_request_st;
pub type OCSP_SINGLERESP = ocsp_single_response_st;
pub type OCSP_BASICRESP = ocsp_basic_response_st;
pub type socklen_t = __socklen_t;
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
pub type wget_vector_destructor = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type wget_thread_mutex = *mut wget_thread_mutex_st;
pub type wget_iri_scheme = libc::c_uint;
pub const WGET_IRI_SCHEME_HTTPS: wget_iri_scheme = 1;
pub const WGET_IRI_SCHEME_HTTP: wget_iri_scheme = 0;
pub type wget_hpkp_db = wget_hpkp_db_st;
pub type wget_hpkp = wget_hpkp_st;
pub type wget_hpkp_db_check_pubkey_fn = unsafe extern "C" fn(
    *mut wget_hpkp_db,
    *const libc::c_char,
    *const libc::c_void,
    size_t,
) -> libc::c_int;
pub type wget_tls_session = wget_tls_session_st;
pub type wget_tls_session_db = wget_tls_session_db_st;
pub type wget_ocsp_db = wget_ocsp_db_st;
pub type wget_ocsp_fingerprint_in_cache_fn = unsafe extern "C" fn(
    *const wget_ocsp_db,
    *const libc::c_char,
    *mut libc::c_int,
) -> bool;
pub type wget_ocsp_db_add_fingerprint_fn = unsafe extern "C" fn(
    *mut wget_ocsp_db,
    *const libc::c_char,
    int64_t,
    bool,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sa_family_t = libc::c_ushort;
pub type wget_dns = wget_dns_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct wget_tcp_st {
    pub ssl_session: *mut libc::c_void,
    pub addrinfo: *mut addrinfo,
    pub bind_addrinfo: *mut addrinfo,
    pub connect_addrinfo: *mut addrinfo,
    pub host: *const libc::c_char,
    pub ssl_hostname: *const libc::c_char,
    pub ip: *const libc::c_char,
    pub bind_interface: *const libc::c_char,
    pub dns: *mut wget_dns,
    pub sockfd: libc::c_int,
    pub dns_timeout: libc::c_int,
    pub connect_timeout: libc::c_int,
    pub timeout: libc::c_int,
    pub family: libc::c_int,
    pub preferred_family: libc::c_int,
    pub protocol: libc::c_int,
    pub hpkp: wget_hpkp_stats_result,
    pub remote_port: uint16_t,
    #[bitfield(name = "ssl", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "tls_false_start", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "tcp_fastopen", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "first_send", ty = "bool", bits = "3..=3")]
    pub ssl_tls_false_start_tcp_fastopen_first_send: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
pub type wget_hpkp_stats_result = libc::c_uint;
pub const WGET_STATS_HPKP_ERROR: wget_hpkp_stats_result = 3;
pub const WGET_STATS_HPKP_NOMATCH: wget_hpkp_stats_result = 2;
pub const WGET_STATS_HPKP_MATCH: wget_hpkp_stats_result = 1;
pub const WGET_STATS_HPKP_NO: wget_hpkp_stats_result = 0;
pub type wget_tcp = wget_tcp_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct config {
    pub secure_protocol: *const libc::c_char,
    pub ca_directory: *const libc::c_char,
    pub ca_file: *const libc::c_char,
    pub cert_file: *const libc::c_char,
    pub key_file: *const libc::c_char,
    pub crl_file: *const libc::c_char,
    pub ocsp_server: *const libc::c_char,
    pub alpn: *const libc::c_char,
    pub ocsp_cert_cache: *mut wget_ocsp_db,
    pub ocsp_host_cache: *mut wget_ocsp_db,
    pub tls_session_cache: *mut wget_tls_session_db,
    pub hpkp_cache: *mut wget_hpkp_db,
    pub ca_type: libc::c_char,
    pub cert_type: libc::c_char,
    pub key_type: libc::c_char,
    #[bitfield(name = "check_certificate", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "check_hostname", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "print_info", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "ocsp", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "ocsp_date", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "ocsp_stapling", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "ocsp_nonce", ty = "bool", bits = "6..=6")]
    pub check_certificate_check_hostname_print_info_ocsp_ocsp_date_ocsp_stapling_ocsp_nonce: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct verification_flags {
    pub certstore: *mut X509_STORE,
    pub ocsp_stapled_cache: *mut wget_vector,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ocsp_stapled_response {
    pub status: libc::c_int,
    pub certid: *mut OCSP_CERTID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_tls_stats_data {
    pub hostname: *const libc::c_char,
    pub alpn_protocol: *const libc::c_char,
    pub tls_secs: libc::c_longlong,
    pub version: libc::c_int,
    pub cert_chain_size: libc::c_int,
    pub http_protocol: libc::c_char,
    pub false_start: bool,
    pub tfo: bool,
    pub tls_con: bool,
    pub resumed: bool,
}
pub type wget_tls_stats_callback = unsafe extern "C" fn(
    *mut wget_tls_stats_data,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wget_ocsp_stats_data {
    pub hostname: *const libc::c_char,
    pub nvalid: libc::c_int,
    pub nrevoked: libc::c_int,
    pub nignored: libc::c_int,
    pub stapling: libc::c_int,
}
pub type wget_ocsp_stats_callback = unsafe extern "C" fn(
    *mut wget_ocsp_stats_data,
    *mut libc::c_void,
) -> ();
pub type wget_http_response = wget_http_response_st;
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
pub type wget_transfer_encoding = libc::c_uint;
pub const wget_transfer_encoding_chunked: wget_transfer_encoding = 1;
pub const wget_transfer_encoding_identity: wget_transfer_encoding = 0;
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
pub type wget_http_header_callback = unsafe extern "C" fn(
    *mut wget_http_response,
    *mut libc::c_void,
) -> libc::c_int;
#[inline]
unsafe extern "C" fn ossl_check_const_OPENSSL_STRING_sk_type(
    mut sk: *const stack_st_OPENSSL_STRING,
) -> *const OPENSSL_STACK {
    return sk as *const OPENSSL_STACK;
}
#[inline]
unsafe extern "C" fn ERR_GET_REASON(mut errcode: libc::c_ulong) -> libc::c_int {
    if errcode
        & (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong
    {
        return (errcode & 2147483647 as libc::c_int as libc::c_uint as libc::c_ulong)
            as libc::c_int;
    }
    return (errcode & 0x7fffff as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ossl_check_const_X509_sk_type(
    mut sk: *const stack_st_X509,
) -> *const OPENSSL_STACK {
    return sk as *const OPENSSL_STACK;
}
#[inline]
unsafe extern "C" fn wget_malloc(mut size: size_t) -> *mut libc::c_void {
    return wget_malloc_fn.expect("non-null function pointer")(size);
}
static mut tls_stats_callback: Option::<wget_tls_stats_callback> = None;
static mut tls_stats_ctx: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut ocsp_stats_callback: Option::<wget_ocsp_stats_callback> = None;
static mut ocsp_stats_ctx: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
static mut config: config = config {
    secure_protocol: 0 as *const libc::c_char,
    ca_directory: 0 as *const libc::c_char,
    ca_file: 0 as *const libc::c_char,
    cert_file: 0 as *const libc::c_char,
    key_file: 0 as *const libc::c_char,
    crl_file: 0 as *const libc::c_char,
    ocsp_server: 0 as *const libc::c_char,
    alpn: 0 as *const libc::c_char,
    ocsp_cert_cache: 0 as *const wget_ocsp_db as *mut wget_ocsp_db,
    ocsp_host_cache: 0 as *const wget_ocsp_db as *mut wget_ocsp_db,
    tls_session_cache: 0 as *const wget_tls_session_db as *mut wget_tls_session_db,
    hpkp_cache: 0 as *const wget_hpkp_db as *mut wget_hpkp_db,
    ca_type: 0,
    cert_type: 0,
    key_type: 0,
    check_certificate_check_hostname_print_info_ocsp_ocsp_date_ocsp_stapling_ocsp_nonce: [0; 1],
    c2rust_padding: [0; 4],
};
static mut init: libc::c_int = 0;
static mut mutex: wget_thread_mutex = 0 as *const wget_thread_mutex_st
    as *mut wget_thread_mutex_st;
static mut _ctx: *mut SSL_CTX = 0 as *const SSL_CTX as *mut SSL_CTX;
static mut ssl_userdata_idx: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_set_config_string(
    mut key: libc::c_int,
    mut value: *const libc::c_char,
) {
    match key {
        1 => {
            config.secure_protocol = value;
        }
        2 => {
            config.ca_directory = value;
        }
        3 => {
            config.ca_file = value;
        }
        4 => {
            config.cert_file = value;
        }
        5 => {
            config.key_file = value;
        }
        13 => {
            config.crl_file = value;
        }
        15 => {
            config.ocsp_server = value;
        }
        18 => {
            config.alpn = value;
        }
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown configuration key %d (maybe this config value should be of another type?)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                key,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_set_config_object(
    mut key: libc::c_int,
    mut value: *mut libc::c_void,
) {
    match key {
        17 => {
            config.ocsp_cert_cache = value as *mut wget_ocsp_db;
        }
        19 => {
            config.tls_session_cache = value as *mut wget_tls_session_db;
        }
        20 => {
            config.hpkp_cache = value as *mut wget_hpkp_db;
        }
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown configuration key %d (maybe this config value should be of another type?)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                key,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_set_config_int(
    mut key: libc::c_int,
    mut value: libc::c_int,
) {
    match key {
        9 => {
            config.set_check_certificate(value != 0);
        }
        23 => {}
        10 => {
            config.set_check_hostname(value != 0);
        }
        11 => {
            config.set_print_info(value != 0);
        }
        6 => {
            config.ca_type = value as libc::c_char;
        }
        7 => {
            config.cert_type = value as libc::c_char;
        }
        8 => {
            config.key_type = value as libc::c_char;
        }
        16 => {
            config.set_ocsp(value != 0);
        }
        14 => {
            config.set_ocsp_stapling(value != 0);
        }
        21 => {
            config.set_ocsp_nonce(value != 0);
        }
        22 => {
            config.set_ocsp_date(value != 0);
        }
        _ => {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown configuration key %d (maybe this config value should be of another type?)\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                key,
            );
        }
    };
}
unsafe extern "C" fn openssl_load_crl(
    mut store: *mut X509_STORE,
    mut crl_file: *const libc::c_char,
) -> libc::c_int {
    let mut lookup: *mut X509_LOOKUP = X509_STORE_add_lookup(store, X509_LOOKUP_file());
    if X509_load_crl_file(lookup, crl_file, 1 as libc::c_int) == 0 {
        return WGET_E_UNKNOWN as libc::c_int;
    }
    if X509_STORE_set_flags(
        store,
        (0x4 as libc::c_int | 0x8 as libc::c_int | 0x2000 as libc::c_int)
            as libc::c_ulong,
    ) == 0
    {
        return WGET_E_UNKNOWN as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn openssl_set_priorities(
    mut ctx: *mut SSL_CTX,
    mut prio: *const libc::c_char,
) -> libc::c_int {
    let mut openssl_ciphers: *const libc::c_char = b"HIGH:!aNULL:!RC4:!MD5:!SRP:!PSK\0"
        as *const u8 as *const libc::c_char;
    SSL_CTX_ctrl(
        ctx,
        123 as libc::c_int,
        0x303 as libc::c_int as libc::c_long,
        0 as *mut libc::c_void,
    );
    SSL_CTX_ctrl(
        ctx,
        124 as libc::c_int,
        0x304 as libc::c_int as libc::c_long,
        0 as *mut libc::c_void,
    );
    if wget_strcasecmp_ascii(prio, b"SSL\0" as *const u8 as *const libc::c_char) == 0 {
        if SSL_CTX_ctrl(
            ctx,
            123 as libc::c_int,
            0x300 as libc::c_int as libc::c_long,
            0 as *mut libc::c_void,
        ) == 0
        {
            return WGET_E_UNKNOWN as libc::c_int;
        }
    } else if wget_strcasecmp_ascii(prio, b"TLSv1\0" as *const u8 as *const libc::c_char)
        == 0
    {
        if SSL_CTX_ctrl(
            ctx,
            123 as libc::c_int,
            0x301 as libc::c_int as libc::c_long,
            0 as *mut libc::c_void,
        ) == 0
        {
            return WGET_E_UNKNOWN as libc::c_int;
        }
    } else if wget_strcasecmp_ascii(
        prio,
        b"TLSv1_1\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if SSL_CTX_ctrl(
            ctx,
            123 as libc::c_int,
            0x302 as libc::c_int as libc::c_long,
            0 as *mut libc::c_void,
        ) == 0
        {
            return WGET_E_UNKNOWN as libc::c_int;
        }
    } else if wget_strcasecmp_ascii(
        prio,
        b"TLSv1_3\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if SSL_CTX_ctrl(
            ctx,
            123 as libc::c_int,
            0x304 as libc::c_int as libc::c_long,
            0 as *mut libc::c_void,
        ) == 0
        {
            return WGET_E_UNKNOWN as libc::c_int;
        }
    } else if wget_strcasecmp_ascii(prio, b"PFS\0" as *const u8 as *const libc::c_char)
        == 0
    {
        openssl_ciphers = b"HIGH:!aNULL:!RC4:!MD5:!SRP:!PSK:!kRSA\0" as *const u8
            as *const libc::c_char;
    } else if !prio.is_null()
        && wget_strcasecmp_ascii(prio, b"AUTO\0" as *const u8 as *const libc::c_char)
            != 0
        && wget_strcasecmp_ascii(prio, b"TLSv1_2\0" as *const u8 as *const libc::c_char)
            != 0
    {
        openssl_ciphers = prio;
    }
    if SSL_CTX_set_cipher_list(ctx, openssl_ciphers) == 0 {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"OpenSSL: Invalid priority string '%s'\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            prio,
        );
        return WGET_E_INVALID as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn openssl_load_trust_file(
    mut ctx: *mut SSL_CTX,
    mut dir: *const libc::c_char,
    mut file: *const libc::c_char,
) -> libc::c_int {
    let mut sbuf: [libc::c_char; 256] = [0; 256];
    let mut buf: wget_buffer = wget_buffer {
        data: 0 as *mut libc::c_char,
        length: 0,
        size: 0,
        release_data_release_buf_error: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut rc: libc::c_int = 0;
    wget_buffer_init(
        &mut buf,
        sbuf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
    );
    wget_buffer_printf(
        &mut buf as *mut wget_buffer,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        dir,
        file,
    );
    rc = if SSL_CTX_load_verify_locations(ctx, buf.data, 0 as *const libc::c_char) != 0 {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    wget_buffer_deinit(&mut buf);
    return rc;
}
unsafe extern "C" fn openssl_load_trust_files_from_directory(
    mut ctx: *mut SSL_CTX,
    mut dirname: *const libc::c_char,
) -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut dp: *mut dirent = 0 as *mut dirent;
    let mut loaded: libc::c_int = 0 as libc::c_int;
    dir = opendir(dirname);
    if !dir.is_null() {
        loop {
            dp = readdir(dir);
            if dp.is_null() {
                break;
            }
            if *((*dp).d_name).as_mut_ptr() as libc::c_int == '.' as i32 {
                continue;
            }
            if wget_match_tail_nocase(
                ((*dp).d_name).as_mut_ptr(),
                b".pem\0" as *const u8 as *const libc::c_char,
            ) != 0
                && openssl_load_trust_file(ctx, dirname, ((*dp).d_name).as_mut_ptr())
                    == 0 as libc::c_int
            {
                loaded += 1;
                loaded;
            }
        }
        closedir(dir);
    }
    return loaded;
}
unsafe extern "C" fn openssl_load_trust_files(
    mut ctx: *mut SSL_CTX,
    mut dir: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0;
    if strcmp(dir, b"system\0" as *const u8 as *const libc::c_char) == 0 {
        if SSL_CTX_set_default_verify_paths(ctx) != 0 {
            retval = 0 as libc::c_int;
            current_block = 6291911124895211048;
        } else {
            dir = wget_ssl_default_cert_dir();
            wget_info_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"OpenSSL: Could not load certificates from default paths. Falling back to '%s'.\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                dir,
            );
            current_block = 7502529970979898288;
        }
    } else {
        current_block = 7502529970979898288;
    }
    match current_block {
        7502529970979898288 => {
            retval = openssl_load_trust_files_from_directory(ctx, dir);
            if retval == 0 as libc::c_int {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"OpenSSL: No certificates could be loaded from directory '%s'\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dir,
                );
            } else if retval > 0 as libc::c_int {
                wget_debug_printf(
                    b"OpenSSL: Loaded %d certificates\n\0" as *const u8
                        as *const libc::c_char,
                    retval,
                );
            } else {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"OpenSSL: Could not open directory '%s'. No certificates were loaded.\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dir,
                );
            }
        }
        _ => {}
    }
    return retval;
}
unsafe extern "C" fn verify_hpkp(
    mut hostname: *const libc::c_char,
    mut subject_cert: *mut X509,
    mut hpkp_stats: *mut wget_hpkp_stats_result,
) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut spki_len: libc::c_int = 0;
    let mut spki: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    spki_len = i2d_PUBKEY(X509_get0_pubkey(subject_cert), &mut spki);
    if spki_len <= 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    retval = wget_hpkp_db_check_pubkey(
        config.hpkp_cache,
        hostname,
        spki as *const libc::c_void,
        spki_len as size_t,
    );
    match retval {
        1 => {
            wget_debug_printf(
                b"Matching HPKP pinning found for host '%s'\n\0" as *const u8
                    as *const libc::c_char,
                hostname,
            );
            *hpkp_stats = WGET_STATS_HPKP_MATCH;
            retval = 0 as libc::c_int;
        }
        0 => {
            wget_debug_printf(
                b"No HPKP pinning found for host '%s'\n\0" as *const u8
                    as *const libc::c_char,
                hostname,
            );
            *hpkp_stats = WGET_STATS_HPKP_NO;
            retval = 1 as libc::c_int;
        }
        -2 => {
            wget_debug_printf(
                b"Public key for host '%s' does not match\n\0" as *const u8
                    as *const libc::c_char,
                hostname,
            );
            *hpkp_stats = WGET_STATS_HPKP_NOMATCH;
            retval = -(1 as libc::c_int);
        }
        _ => {
            wget_debug_printf(
                b"Could not check HPKP pinning for host '%s' (%d)\n\0" as *const u8
                    as *const libc::c_char,
                hostname,
                retval,
            );
            *hpkp_stats = WGET_STATS_HPKP_ERROR;
            retval = 0 as libc::c_int;
        }
    }
    CRYPTO_free(
        spki as *mut libc::c_void,
        b"ssl_openssl.c\0" as *const u8 as *const libc::c_char,
        537 as libc::c_int,
    );
    return retval;
}
unsafe extern "C" fn check_cert_chain_for_hpkp(
    mut certs: *mut stack_st_X509,
    mut hostname: *const libc::c_char,
    mut hpkp_stats: *mut wget_hpkp_stats_result,
) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut pin_ok: libc::c_int = 0 as libc::c_int;
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut cert_list_size: libc::c_uint = OPENSSL_sk_num(
        ossl_check_const_X509_sk_type(certs),
    ) as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < cert_list_size {
        cert = OPENSSL_sk_value(ossl_check_const_X509_sk_type(certs), i as libc::c_int)
            as *mut X509;
        retval = verify_hpkp(hostname, cert, hpkp_stats);
        if retval >= 0 as libc::c_int {
            pin_ok = 1 as libc::c_int;
        }
        if retval == 1 as libc::c_int {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    return pin_ok;
}
unsafe extern "C" fn _ocsp_stapled_response_compare_func(
    mut elem1: *const libc::c_void,
    mut elem2: *const libc::c_void,
) -> libc::c_int {
    let mut certid: *const OCSP_CERTID = elem1 as *const OCSP_CERTID;
    let mut stored: *const ocsp_stapled_response = elem2 as *const ocsp_stapled_response;
    return OCSP_id_cmp(certid, (*stored).certid);
}
unsafe extern "C" fn _ocsp_stapled_response_destroy_func(mut elem: *mut libc::c_void) {
    let mut resp: *mut ocsp_stapled_response = elem as *mut ocsp_stapled_response;
    OCSP_CERTID_free((*resp).certid);
    if !elem.is_null() {
        wget_free.expect("non-null function pointer")(elem);
        elem = 0 as *mut libc::c_void;
    }
}
unsafe extern "C" fn ocsp_create_stapled_response_vector() -> *mut wget_vector {
    let mut vec: *mut wget_vector = wget_vector_create(
        5 as libc::c_int,
        Some(
            _ocsp_stapled_response_compare_func
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if vec.is_null() {
        return 0 as *mut wget_vector;
    }
    wget_vector_set_resize_factor(vec, 1 as libc::c_int as libc::c_float);
    wget_vector_set_destructor(
        vec,
        Some(
            _ocsp_stapled_response_destroy_func
                as unsafe extern "C" fn(*mut libc::c_void) -> (),
        ),
    );
    return vec;
}
unsafe extern "C" fn ocsp_destroy_stapled_response_vector(
    mut vec: *mut *mut wget_vector,
) {
    wget_vector_free(vec);
}
unsafe extern "C" fn ocsp_stapled_responses_add_single(
    mut singleresp: *const OCSP_SINGLERESP,
    mut status: libc::c_int,
    mut arg: *mut libc::c_void,
) {
    let mut vec: *mut wget_vector = arg as *mut wget_vector;
    let mut resp: *mut ocsp_stapled_response = wget_malloc(
        ::core::mem::size_of::<ocsp_stapled_response>() as libc::c_ulong,
    ) as *mut ocsp_stapled_response;
    let mut certid: *mut OCSP_CERTID = OCSP_CERTID_dup(
        OCSP_SINGLERESP_get0_id(singleresp),
    );
    if !resp.is_null() && !certid.is_null() {
        (*resp).status = status;
        (*resp).certid = certid;
        wget_vector_insert(vec, resp as *const libc::c_void, 0 as libc::c_int);
    } else {
        if !certid.is_null() {
            OCSP_CERTID_free(certid);
        }
        if !resp.is_null() {
            wget_free.expect("non-null function pointer")(resp as *mut libc::c_void);
            resp = 0 as *mut ocsp_stapled_response;
        }
    };
}
unsafe extern "C" fn ocsp_stapled_response_get(
    mut cert: *const X509,
    mut issuer: *const X509,
    mut vec: *const wget_vector,
) -> *const ocsp_stapled_response {
    let mut certid: *mut OCSP_CERTID = OCSP_cert_to_id(0 as *const EVP_MD, cert, issuer);
    let mut pos: libc::c_int = wget_vector_find(vec, certid as *const libc::c_void);
    OCSP_CERTID_free(certid);
    return wget_vector_get(vec, pos) as *const ocsp_stapled_response;
}
unsafe extern "C" fn ocsp_lookup_in_cache(
    mut cert: *mut X509,
    mut issuer: *mut X509,
    mut ocsp_stapled_cache: *const wget_vector,
    mut ocsp_cert_cache: *const wget_ocsp_db,
    mut revoked: *mut libc::c_int,
    mut cache_origin: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ocsp_stapled_resp: *const ocsp_stapled_response = 0
        as *const ocsp_stapled_response;
    ocsp_stapled_resp = ocsp_stapled_response_get(cert, issuer, ocsp_stapled_cache);
    if !ocsp_stapled_resp.is_null()
        && ((*ocsp_stapled_resp).status == 0 as libc::c_int
            || (*ocsp_stapled_resp).status == 1 as libc::c_int)
    {
        *revoked = ((*ocsp_stapled_resp).status == 1 as libc::c_int) as libc::c_int;
        *cache_origin = b"stapled\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    if !ocsp_cert_cache.is_null() {
        let mut fingerprint: *mut libc::c_char = compute_cert_fingerprint(cert);
        if fingerprint.is_null() {
            return -(1 as libc::c_int);
        }
        if wget_ocsp_fingerprint_in_cache(ocsp_cert_cache, fingerprint, revoked) {
            if !fingerprint.is_null() {
                wget_free
                    .expect(
                        "non-null function pointer",
                    )(fingerprint as *mut libc::c_void);
                fingerprint = 0 as *mut libc::c_char;
            }
            *cache_origin = b"cached\0" as *const u8 as *const libc::c_char;
            return 1 as libc::c_int;
        }
        if !fingerprint.is_null() {
            wget_free
                .expect("non-null function pointer")(fingerprint as *mut libc::c_void);
            fingerprint = 0 as *mut libc::c_char;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ocsp_resp_cb(
    mut s: *mut SSL,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut ocsp_resp_len: libc::c_long = 0;
    let mut ocsp_resp_raw: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut ocspresp: *mut OCSP_RESPONSE = 0 as *mut OCSP_RESPONSE;
    let mut certstack: *mut stack_st_X509 = 0 as *mut stack_st_X509;
    let mut ocsp_verif: *mut verification_flags = 0 as *mut verification_flags;
    ocsp_verif = SSL_get_ex_data(s, ssl_userdata_idx) as *mut verification_flags;
    if ocsp_verif.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Could not get user data to verify stapled OCSP.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    ocsp_resp_len = SSL_ctrl(
        s,
        70 as libc::c_int,
        0 as libc::c_int as libc::c_long,
        &mut ocsp_resp_raw as *mut *const libc::c_uchar as *mut libc::c_void,
    );
    if ocsp_resp_len == -(1 as libc::c_int) as libc::c_long {
        wget_debug_printf(
            b"No stapled OCSP response was received. Continuing.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    ocspresp = d2i_OCSP_RESPONSE(
        0 as *mut *mut OCSP_RESPONSE,
        &mut ocsp_resp_raw,
        ocsp_resp_len,
    );
    if ocspresp.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Got a stapled OCSP response, but could not parse it. Aborting.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    certstack = SSL_get_peer_cert_chain(s);
    if certstack.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Could not get server's cert stack\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    result = check_ocsp_response(
        ocspresp,
        certstack,
        (*ocsp_verif).certstore,
        0 as libc::c_int != 0,
        Some(
            ocsp_stapled_responses_add_single
                as unsafe extern "C" fn(
                    *const OCSP_SINGLERESP,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> (),
        ),
        (*ocsp_verif).ocsp_stapled_cache as *mut libc::c_void,
    );
    if result == -(1 as libc::c_int) {
        OCSP_RESPONSE_free(ocspresp);
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Could not verify stapled OCSP response. Aborting.\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return 0 as libc::c_int;
    }
    OCSP_RESPONSE_free(ocspresp);
    wget_debug_printf(
        b"*** Stapled OCSP response verified. Length: %ld. Status: OK\n\0" as *const u8
            as *const libc::c_char,
        ocsp_resp_len,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn send_ocsp_request(
    mut uri: *const libc::c_char,
    mut certid: *mut OCSP_CERTID,
    mut response: *mut *mut wget_http_response,
) -> *mut OCSP_REQUEST {
    let mut ocspreq: *mut OCSP_REQUEST = 0 as *mut OCSP_REQUEST;
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut ocspreq_bytes: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ocspreq_bytes_len: size_t = 0;
    ocspreq = OCSP_REQUEST_new();
    if !ocspreq.is_null() {
        if (OCSP_request_add0_id(ocspreq, certid)).is_null() {
            OCSP_REQUEST_free(ocspreq);
            ocspreq = 0 as *mut OCSP_REQUEST;
        } else if config.ocsp_nonce() as libc::c_int != 0
            && OCSP_request_add1_nonce(
                ocspreq,
                0 as *mut libc::c_uchar,
                0 as libc::c_int,
            ) == 0
        {
            OCSP_REQUEST_free(ocspreq);
            ocspreq = 0 as *mut OCSP_REQUEST;
        } else {
            ocspreq_bytes_len = i2d_OCSP_REQUEST(ocspreq, &mut ocspreq_bytes) as size_t;
            if ocspreq_bytes.is_null() || ocspreq_bytes_len == 0 {
                OCSP_REQUEST_free(ocspreq);
                ocspreq = 0 as *mut OCSP_REQUEST;
            } else {
                resp = wget_http_get(
                    2000 as libc::c_int,
                    uri,
                    2016 as libc::c_int,
                    b"POST\0" as *const u8 as *const libc::c_char,
                    2004 as libc::c_int,
                    b"Accept-Encoding\0" as *const u8 as *const libc::c_char,
                    b"identity\0" as *const u8 as *const libc::c_char,
                    2004 as libc::c_int,
                    b"Accept\0" as *const u8 as *const libc::c_char,
                    b"*/*\0" as *const u8 as *const libc::c_char,
                    2004 as libc::c_int,
                    b"Content-Type\0" as *const u8 as *const libc::c_char,
                    b"application/ocsp-request\0" as *const u8 as *const libc::c_char,
                    2010 as libc::c_int,
                    5 as libc::c_int,
                    2017 as libc::c_int,
                    ocspreq_bytes,
                    ocspreq_bytes_len,
                    2021 as libc::c_int,
                    0 as libc::c_int,
                );
                CRYPTO_free(
                    ocspreq_bytes as *mut libc::c_void,
                    b"ssl_openssl.c\0" as *const u8 as *const libc::c_char,
                    768 as libc::c_int,
                );
                if !resp.is_null() {
                    *response = resp;
                } else {
                    OCSP_REQUEST_free(ocspreq);
                    ocspreq = 0 as *mut OCSP_REQUEST;
                }
            }
        }
    }
    return ocspreq;
}
unsafe extern "C" fn get_printable_ocsp_reason_desc(
    mut reason: libc::c_int,
) -> *const libc::c_char {
    match reason {
        -1 => return b"not given\0" as *const u8 as *const libc::c_char,
        0 => return b"unspecified\0" as *const u8 as *const libc::c_char,
        1 => return b"key compromise\0" as *const u8 as *const libc::c_char,
        2 => return b"CA compromise\0" as *const u8 as *const libc::c_char,
        3 => return b"affiliation changed\0" as *const u8 as *const libc::c_char,
        4 => return b"superseded\0" as *const u8 as *const libc::c_char,
        5 => return b"cessation of operation\0" as *const u8 as *const libc::c_char,
        6 => return b"certificate hold\0" as *const u8 as *const libc::c_char,
        8 => return b"remove from CRL\0" as *const u8 as *const libc::c_char,
        _ => return b"unknown reason\0" as *const u8 as *const libc::c_char,
    };
}
unsafe extern "C" fn print_ocsp_response_status(mut status: libc::c_int) {
    let mut msg: [libc::c_char; 64] = [0; 64];
    let mut status_string: *const libc::c_char = 0 as *const libc::c_char;
    match status {
        0 => {
            status_string = b"successful\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            status_string = b"malformed request\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            status_string = b"internal error\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            status_string = b"try later\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            status_string = b"signature required\0" as *const u8 as *const libc::c_char;
        }
        6 => {
            status_string = b"unauthorized\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            wget_snprintf(
                msg.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"unknown status code %d\0" as *const u8 as *const libc::c_char,
                status,
            );
            status_string = msg.as_mut_ptr();
        }
    }
    wget_debug_printf(
        b"*** OCSP response status: %s\n\0" as *const u8 as *const libc::c_char,
        status_string,
    );
}
unsafe extern "C" fn print_ocsp_cert_status(
    mut status: libc::c_int,
    mut reason: libc::c_int,
) {
    let mut msg: [libc::c_char; 64] = [0; 64];
    let mut reason_string: *const libc::c_char = 0 as *const libc::c_char;
    match status {
        0 => {
            reason_string = b"good\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            reason_string = b"unknown\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            wget_snprintf(
                msg.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                b"revoked (%s)\0" as *const u8 as *const libc::c_char,
                get_printable_ocsp_reason_desc(reason),
            );
            reason_string = msg.as_mut_ptr();
        }
        _ => {
            reason_string = b"invalid status code\0" as *const u8 as *const libc::c_char;
        }
    }
    wget_debug_printf(
        b"*** OCSP cert status: %s\n\0" as *const u8 as *const libc::c_char,
        reason_string,
    );
}
unsafe extern "C" fn print_openssl_time(
    mut prefix: *const libc::c_char,
    mut t: *const ASN1_GENERALIZEDTIME,
) {
    let mut nread: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut mem: *mut BIO = BIO_new(BIO_s_mem());
    ASN1_GENERALIZEDTIME_print(mem, t);
    nread = BIO_read(
        mem,
        buf.as_mut_ptr() as *mut libc::c_void,
        (::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
    if nread > 0 as libc::c_int {
        buf[nread as usize] = '\0' as i32 as libc::c_char;
        wget_debug_printf(
            b"%s%s\n\0" as *const u8 as *const libc::c_char,
            prefix,
            buf.as_mut_ptr(),
        );
    } else {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"ERROR: print_openssl_time: BIO_read failed\n\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    }
    BIO_free_all(mem);
}
unsafe extern "C" fn check_ocsp_response_times(
    mut thisupd: *const ASN1_GENERALIZEDTIME,
    mut nextupd: *const ASN1_GENERALIZEDTIME,
) -> libc::c_int {
    let mut day: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut now: *mut ASN1_TIME = 0 as *mut ASN1_TIME;
    now = ASN1_TIME_adj(
        0 as *mut ASN1_TIME,
        time(0 as *mut time_t),
        0 as libc::c_int,
        0 as libc::c_int as libc::c_long,
    );
    if now.is_null() {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Could not get current time!\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        return -(1 as libc::c_int);
    }
    print_openssl_time(
        b"*** OCSP issued time: \0" as *const u8 as *const libc::c_char,
        thisupd,
    );
    if nextupd.is_null() {
        wget_debug_printf(
            b"OCSP nextUpd not set. Checking thisUpd is not too old.\n\0" as *const u8
                as *const libc::c_char,
        );
        if ASN1_TIME_diff(&mut day, &mut sec, now, thisupd) == 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not compute time difference for thisUpd. Aborting.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else if day < -(3 as libc::c_int) {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"*** OCSP response thisUpd is too old. Aborting.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            retval = 0 as libc::c_int;
        }
    } else {
        print_openssl_time(
            b"*** OCSP update time: \0" as *const u8 as *const libc::c_char,
            nextupd,
        );
        if ASN1_TIME_diff(&mut day, &mut sec, now, nextupd) == 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not compute time difference for nextUpd. Aborting.\n\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else if day < 0 as libc::c_int
            || day == 0 as libc::c_int && sec < 0 as libc::c_int
        {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"*** OCSP next update is in the past!\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        } else {
            retval = 0 as libc::c_int;
        }
    }
    ASN1_STRING_free(now);
    return retval;
}
unsafe extern "C" fn check_ocsp_response(
    mut ocspresp: *mut OCSP_RESPONSE,
    mut certstack: *mut stack_st_X509,
    mut certstore: *mut X509_STORE,
    mut check_time: bool,
    mut ocsp_singleresp_callback_func: Option::<
        unsafe extern "C" fn(
            *const OCSP_SINGLERESP,
            libc::c_int,
            *mut libc::c_void,
        ) -> (),
    >,
    mut func_arg: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut status: libc::c_int = 0;
    let mut reason: libc::c_int = 0;
    let mut ocspbs: *mut OCSP_BASICRESP = 0 as *mut OCSP_BASICRESP;
    let mut single: *mut OCSP_SINGLERESP = 0 as *mut OCSP_SINGLERESP;
    let mut revtime: *mut ASN1_GENERALIZEDTIME = 0 as *mut ASN1_GENERALIZEDTIME;
    let mut thisupd: *mut ASN1_GENERALIZEDTIME = 0 as *mut ASN1_GENERALIZEDTIME;
    let mut nextupd: *mut ASN1_GENERALIZEDTIME = 0 as *mut ASN1_GENERALIZEDTIME;
    status = OCSP_response_status(ocspresp);
    print_ocsp_response_status(status);
    if status != 0 as libc::c_int {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Unsuccessful OCSP response\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        ocspbs = OCSP_response_get1_basic(ocspresp);
        if !ocspbs.is_null() {
            if OCSP_basic_verify(
                ocspbs,
                certstack,
                certstore,
                0 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Could not verify OCSP certificate chain\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
            } else {
                single = OCSP_resp_get0(ocspbs, 0 as libc::c_int);
                if single.is_null() {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Could not parse OCSP single response\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                } else {
                    status = OCSP_single_get0_status(
                        single,
                        &mut reason,
                        &mut revtime,
                        &mut thisupd,
                        &mut nextupd,
                    );
                    if status == -(1 as libc::c_int) {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Could not obtain OCSP response status\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    } else {
                        print_ocsp_cert_status(status, reason);
                        if status == 1 as libc::c_int {
                            print_openssl_time(
                                b"*** Certificate revoked by OCSP at: \0" as *const u8
                                    as *const libc::c_char,
                                revtime,
                            );
                            retval = 1 as libc::c_int;
                        } else {
                            if check_time {
                                if thisupd.is_null() {
                                    wget_error_printf(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Could not get 'thisUpd' from OCSP response. Cannot check time.\n\0"
                                                as *const u8 as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    current_block = 4465788416328860201;
                                } else if check_ocsp_response_times(thisupd, nextupd)
                                    < 0 as libc::c_int
                                {
                                    retval = 1 as libc::c_int;
                                    current_block = 4465788416328860201;
                                } else {
                                    current_block = 4495394744059808450;
                                }
                            } else {
                                current_block = 4495394744059808450;
                            }
                            match current_block {
                                4465788416328860201 => {}
                                _ => {
                                    if ocsp_singleresp_callback_func.is_some()
                                        && (status == 0 as libc::c_int
                                            || status == 1 as libc::c_int)
                                    {
                                        ocsp_singleresp_callback_func
                                            .expect(
                                                "non-null function pointer",
                                            )(single, status, func_arg);
                                    }
                                    retval = 0 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !ocspbs.is_null() {
        OCSP_BASICRESP_free(ocspbs);
    }
    return retval;
}
unsafe extern "C" fn verify_ocsp(
    mut ocsp_uri: *const libc::c_char,
    mut subject_cert: *mut X509,
    mut issuer_cert: *mut X509,
    mut certs: *mut stack_st_X509,
    mut certstore: *mut X509_STORE,
    mut check_time: bool,
    mut check_nonce: bool,
) -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0;
    let mut resp: *mut wget_http_response = 0 as *mut wget_http_response;
    let mut body: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut certid: *mut OCSP_CERTID = 0 as *mut OCSP_CERTID;
    let mut ocspreq: *mut OCSP_REQUEST = 0 as *mut OCSP_REQUEST;
    let mut ocspresp: *mut OCSP_RESPONSE = 0 as *mut OCSP_RESPONSE;
    let mut ocspbs: *mut OCSP_BASICRESP = 0 as *mut OCSP_BASICRESP;
    certid = OCSP_cert_to_id(EVP_sha1(), subject_cert, issuer_cert);
    ocspreq = send_ocsp_request(ocsp_uri, certid, &mut resp);
    if ocspreq.is_null() || resp.is_null() || ((*resp).body).is_null() {
        return -(1 as libc::c_int);
    }
    body = (*(*resp).body).data as *const libc::c_uchar;
    ocspresp = d2i_OCSP_RESPONSE(
        0 as *mut *mut OCSP_RESPONSE,
        &mut body,
        (*(*resp).body).length as libc::c_long,
    );
    if ocspresp.is_null() {
        wget_http_free_response(&mut resp);
        OCSP_REQUEST_free(ocspreq);
        return -(1 as libc::c_int);
    }
    retval = check_ocsp_response(
        ocspresp,
        certs,
        certstore,
        check_time,
        None,
        0 as *mut libc::c_void,
    );
    if !(retval != 0 as libc::c_int) {
        if check_nonce {
            ocspbs = OCSP_response_get1_basic(ocspresp);
            if ocspbs.is_null() {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Could not obtain OCSP_BASICRESPONSE\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                retval = -(1 as libc::c_int);
                current_block = 17322723924367293208;
            } else if OCSP_check_nonce(ocspreq, ocspbs) == 0 {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"OCSP nonce does not match\n\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                retval = 1 as libc::c_int;
                current_block = 17322723924367293208;
            } else {
                OCSP_BASICRESP_free(ocspbs);
                ocspbs = 0 as *mut OCSP_BASICRESP;
                current_block = 8457315219000651999;
            }
        } else {
            current_block = 8457315219000651999;
        }
        match current_block {
            17322723924367293208 => {}
            _ => {
                retval = 0 as libc::c_int;
            }
        }
    }
    if !ocspbs.is_null() {
        OCSP_BASICRESP_free(ocspbs);
    }
    wget_http_free_response(&mut resp);
    OCSP_RESPONSE_free(ocspresp);
    OCSP_REQUEST_free(ocspreq);
    return retval;
}
unsafe extern "C" fn read_ocsp_uri_from_certificate(
    mut cert: *mut X509,
) -> *mut libc::c_char {
    let mut str_stack: *mut stack_st_OPENSSL_STRING = X509_get1_ocsp(cert);
    if !str_stack.is_null()
        && OPENSSL_sk_num(ossl_check_const_OPENSSL_STRING_sk_type(str_stack))
            > 0 as libc::c_int
    {
        let mut uri: *mut libc::c_char = wget_strdup(
            OPENSSL_sk_value(
                ossl_check_const_OPENSSL_STRING_sk_type(str_stack),
                0 as libc::c_int,
            ) as *mut libc::c_char,
        );
        X509_email_free(str_stack);
        return uri;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn compute_cert_fingerprint(mut cert: *mut X509) -> *mut libc::c_char {
    let mut mdctx: *mut EVP_MD_CTX = EVP_MD_CTX_new();
    let mut hexstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut der_output: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut digest_output: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut der_length: libc::c_int = 0;
    let mut digest_length: libc::c_int = 0;
    let mut hexstring_length: libc::c_int = 0;
    der_length = i2d_X509(cert, &mut der_output);
    if !(der_length < 0 as libc::c_int) {
        digest_length = EVP_MD_get_size(EVP_sha256());
        digest_output = wget_malloc(digest_length as size_t) as *mut libc::c_uchar;
        if !digest_output.is_null() {
            if !(EVP_DigestInit_ex(mdctx, EVP_sha256(), 0 as *mut ENGINE) == 0) {
                if !(EVP_DigestUpdate(
                    mdctx,
                    der_output as *const libc::c_void,
                    der_length as size_t,
                ) == 0)
                {
                    if !(EVP_DigestFinal_ex(mdctx, digest_output, 0 as *mut libc::c_uint)
                        == 0)
                    {
                        CRYPTO_free(
                            der_output as *mut libc::c_void,
                            b"ssl_openssl.c\0" as *const u8 as *const libc::c_char,
                            1118 as libc::c_int,
                        );
                        der_output = 0 as *mut libc::c_uchar;
                        EVP_MD_CTX_free(mdctx);
                        mdctx = 0 as *mut EVP_MD_CTX;
                        hexstring_length = digest_length * 2 as libc::c_int
                            + 1 as libc::c_int;
                        hexstring = wget_malloc(hexstring_length as size_t)
                            as *mut libc::c_char;
                        if !hexstring.is_null() {
                            wget_memtohex(
                                digest_output,
                                digest_length as size_t,
                                hexstring,
                                hexstring_length as size_t,
                            );
                            if !digest_output.is_null() {
                                wget_free
                                    .expect(
                                        "non-null function pointer",
                                    )(digest_output as *mut libc::c_void);
                                digest_output = 0 as *mut libc::c_uchar;
                            }
                            return hexstring;
                        }
                    }
                }
            }
        }
    }
    if !hexstring.is_null() {
        wget_free.expect("non-null function pointer")(hexstring as *mut libc::c_void);
        hexstring = 0 as *mut libc::c_char;
    }
    if !digest_output.is_null() {
        wget_free
            .expect("non-null function pointer")(digest_output as *mut libc::c_void);
        digest_output = 0 as *mut libc::c_uchar;
    }
    if !der_output.is_null() {
        CRYPTO_free(
            der_output as *mut libc::c_void,
            b"ssl_openssl.c\0" as *const u8 as *const libc::c_char,
            1138 as libc::c_int,
        );
    }
    if !mdctx.is_null() {
        EVP_MD_CTX_free(mdctx);
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn find_issuer_cert(
    mut certs: *const stack_st_X509,
    mut subject: *mut X509,
    mut starting_idx: libc::c_uint,
) -> *mut X509 {
    let mut candidate: *mut X509 = 0 as *mut X509;
    let mut cert_chain_size: libc::c_uint = OPENSSL_sk_num(
        ossl_check_const_X509_sk_type(certs),
    ) as libc::c_uint;
    let mut next: libc::c_uint = starting_idx;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < cert_chain_size.wrapping_sub(1 as libc::c_int as libc::c_uint) {
        next = if next == cert_chain_size.wrapping_sub(1 as libc::c_int as libc::c_uint)
        {
            0 as libc::c_int as libc::c_uint
        } else {
            next.wrapping_add(1 as libc::c_int as libc::c_uint)
        };
        candidate = OPENSSL_sk_value(
            ossl_check_const_X509_sk_type(certs),
            next as libc::c_int,
        ) as *mut X509;
        if !candidate.is_null()
            && X509_check_issued(candidate, subject) == 0 as libc::c_int
        {
            return candidate;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut X509;
}
unsafe extern "C" fn check_cert_chain_for_ocsp(
    mut certs: *mut stack_st_X509,
    mut store: *mut X509_STORE,
    mut hostname: *const libc::c_char,
    mut ocsp_stapled_cache: *mut wget_vector,
) -> libc::c_int {
    let mut stats: wget_ocsp_stats_data = wget_ocsp_stats_data {
        hostname: 0 as *const libc::c_char,
        nvalid: 0,
        nrevoked: 0,
        nignored: 0,
        stapling: 0,
    };
    let mut num_ok: libc::c_int = 0 as libc::c_int;
    let mut num_revoked: libc::c_int = 0 as libc::c_int;
    let mut num_ignored: libc::c_int = 0 as libc::c_int;
    let mut revoked: libc::c_int = 0;
    let mut ocsp_ok: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    let mut ocsp_uri: *const libc::c_char = 0 as *const libc::c_char;
    let mut fingerprint: *const libc::c_char = 0 as *const libc::c_char;
    let mut cache_origin: *const libc::c_char = 0 as *const libc::c_char;
    let mut cert: *mut X509 = 0 as *mut X509;
    let mut issuer_cert: *mut X509 = 0 as *mut X509;
    let mut cert_list_size: libc::c_uint = OPENSSL_sk_num(
        ossl_check_const_X509_sk_type(certs),
    ) as libc::c_uint;
    let mut current_block_43: u64;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < cert_list_size {
        cert = OPENSSL_sk_value(ossl_check_const_X509_sk_type(certs), i as libc::c_int)
            as *mut X509;
        issuer_cert = find_issuer_cert(certs, cert, i);
        if issuer_cert.is_null() {
            break;
        }
        retval = ocsp_lookup_in_cache(
            cert,
            issuer_cert,
            ocsp_stapled_cache,
            config.ocsp_cert_cache,
            &mut revoked,
            &mut cache_origin,
        );
        if retval == 1 as libc::c_int {
            if revoked != 0 {
                wget_debug_printf(
                    b"Certificate %u has been revoked (%s response)\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    cache_origin,
                );
                num_revoked += 1;
                num_revoked;
            } else {
                wget_debug_printf(
                    b"Certificate %u is valid (%s response)\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    cache_origin,
                );
                num_ok += 1;
                num_ok;
            }
        } else {
            if retval == -(1 as libc::c_int) {
                wget_error_printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Could not compute certificate fingerprint for cert %u\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    i,
                );
                return 0 as libc::c_int;
            }
            if (config.ocsp_server).is_null() {
                ocsp_uri = read_ocsp_uri_from_certificate(cert);
                if ocsp_uri.is_null() {
                    wget_debug_printf(
                        b"OCSP URI not given and not found in certificate. Skipping OCSP check for cert %u.\n\0"
                            as *const u8 as *const libc::c_char,
                        i,
                    );
                    num_ignored += 1;
                    num_ignored;
                    current_block_43 = 14916268686031723178;
                } else {
                    current_block_43 = 13797916685926291137;
                }
            } else {
                current_block_43 = 13797916685926291137;
            }
            match current_block_43 {
                14916268686031723178 => {}
                _ => {
                    wget_debug_printf(
                        b"Contacting OCSP server. URI: %s\n\0" as *const u8
                            as *const libc::c_char,
                        if !(config.ocsp_server).is_null() {
                            config.ocsp_server
                        } else {
                            ocsp_uri
                        },
                    );
                    ocsp_ok = verify_ocsp(
                        if !(config.ocsp_server).is_null() {
                            config.ocsp_server
                        } else {
                            ocsp_uri
                        },
                        cert,
                        issuer_cert,
                        certs,
                        store,
                        config.ocsp_date(),
                        config.ocsp_nonce(),
                    );
                    if ocsp_ok == 0 as libc::c_int {
                        num_ok += 1;
                        num_ok;
                    } else if ocsp_ok == 1 as libc::c_int {
                        num_revoked += 1;
                        num_revoked;
                    } else {
                        num_ignored += 1;
                        num_ignored;
                    }
                    fingerprint = compute_cert_fingerprint(cert);
                    if fingerprint.is_null() {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Could not compute certificate fingerprint for cert %u\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            i,
                        );
                        return 0 as libc::c_int;
                    }
                    if ocsp_ok == 0 as libc::c_int || ocsp_ok == 1 as libc::c_int {
                        wget_ocsp_db_add_fingerprint(
                            config.ocsp_cert_cache,
                            fingerprint,
                            time(0 as *mut time_t) + 3600 as libc::c_int as time_t,
                            ocsp_ok == 0 as libc::c_int,
                        );
                    }
                    if !fingerprint.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(fingerprint as *mut libc::c_void);
                        fingerprint = 0 as *const libc::c_char;
                    }
                    if !ocsp_uri.is_null() {
                        wget_free
                            .expect(
                                "non-null function pointer",
                            )(ocsp_uri as *mut libc::c_void);
                        ocsp_uri = 0 as *const libc::c_char;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if ocsp_stats_callback.is_some() {
        stats.hostname = hostname;
        stats.nvalid = num_ok;
        stats.nrevoked = num_revoked;
        stats.nignored = num_ignored;
        stats.stapling = 0 as libc::c_int;
        ocsp_stats_callback
            .expect("non-null function pointer")(&mut stats, ocsp_stats_ctx);
    }
    return (num_revoked == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn openssl_init(mut ctx: *mut SSL_CTX) -> libc::c_int {
    let mut current_block: u64;
    let mut retval: libc::c_int = 0 as libc::c_int;
    let mut store: *mut X509_STORE = 0 as *mut X509_STORE;
    if !config.check_certificate() {
        SSL_CTX_set_verify(ctx, 0 as libc::c_int, None);
        wget_info_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"Certificate check disabled. Peer's certificate will NOT be checked.\n\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        store = SSL_CTX_get_cert_store(ctx);
        if store.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"OpenSSL: Could not obtain cert store\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            retval = WGET_E_UNKNOWN as libc::c_int;
        } else {
            if !(config.ca_directory).is_null()
                && *config.ca_directory as libc::c_int != 0
            {
                retval = openssl_load_trust_files(ctx, config.ca_directory);
                if retval < 0 as libc::c_int {
                    current_block = 14320440274480085798;
                } else {
                    if !(config.crl_file).is_null() {
                        retval = openssl_load_crl(store, config.crl_file);
                        if retval < 0 as libc::c_int {
                            wget_error_printf(
                                dcgettext(
                                    0 as *const libc::c_char,
                                    b"Could not load CRL from '%s' (%d)\n\0" as *const u8
                                        as *const libc::c_char,
                                    5 as libc::c_int,
                                ),
                                config.crl_file,
                                retval,
                            );
                            current_block = 14320440274480085798;
                        } else {
                            current_block = 7651349459974463963;
                        }
                    } else {
                        current_block = 7651349459974463963;
                    }
                    match current_block {
                        14320440274480085798 => {}
                        _ => {
                            SSL_CTX_set_verify(
                                ctx,
                                0x1 as libc::c_int | 0x2 as libc::c_int,
                                None,
                            );
                            current_block = 12800627514080957624;
                        }
                    }
                }
            } else {
                current_block = 12800627514080957624;
            }
            match current_block {
                14320440274480085798 => {}
                _ => {
                    if !(config.ca_file).is_null()
                        && wget_strcmp(
                            config.ca_file,
                            b"system\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        config.ca_file = wget_ssl_default_ca_bundle_path();
                    }
                    if !(config.ca_file).is_null() && *config.ca_file as libc::c_int != 0
                        && SSL_CTX_load_verify_locations(
                            ctx,
                            config.ca_file,
                            0 as *const libc::c_char,
                        ) == 0
                    {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Could not load CA certificate from file '%s'\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            config.ca_file,
                        );
                    }
                    if config.ocsp_stapling() {
                        SSL_CTX_callback_ctrl(
                            ctx,
                            63 as libc::c_int,
                            ::core::mem::transmute::<
                                Option::<
                                    unsafe extern "C" fn(
                                        *mut SSL,
                                        *mut libc::c_void,
                                    ) -> libc::c_int,
                                >,
                                Option::<unsafe extern "C" fn() -> ()>,
                            >(
                                Some(
                                    ocsp_resp_cb
                                        as unsafe extern "C" fn(
                                            *mut SSL,
                                            *mut libc::c_void,
                                        ) -> libc::c_int,
                                ),
                            ),
                        );
                    }
                    retval = openssl_set_priorities(ctx, config.secure_protocol);
                }
            }
        }
    }
    return retval;
}
unsafe extern "C" fn openssl_deinit(mut ctx: *mut SSL_CTX) {
    SSL_CTX_free(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_init() {
    wget_thread_mutex_lock(mutex);
    if init == 0 {
        _ctx = SSL_CTX_new(TLS_client_method());
        if !_ctx.is_null() && openssl_init(_ctx) == 0 as libc::c_int {
            init += 1;
            init;
            wget_debug_printf(
                b"OpenSSL initialized\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Could not initialize OpenSSL\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
    }
    wget_thread_mutex_unlock(mutex);
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_deinit() {
    wget_thread_mutex_lock(mutex);
    if init == 1 as libc::c_int {
        openssl_deinit(_ctx);
    }
    if init > 0 as libc::c_int {
        init -= 1;
        init;
    }
    wget_thread_mutex_unlock(mutex);
}
unsafe extern "C" fn ssl_resume_session(
    mut ssl: *mut SSL,
    mut hostname: *const libc::c_char,
) -> libc::c_int {
    let mut sess: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sesslen: size_t = 0;
    let mut ssl_session: *mut SSL_SESSION = 0 as *mut SSL_SESSION;
    if (config.tls_session_cache).is_null() {
        return 0 as libc::c_int;
    }
    if wget_tls_session_get(config.tls_session_cache, hostname, &mut sess, &mut sesslen)
        == 0 as libc::c_int && !sess.is_null()
    {
        wget_debug_printf(
            b"Found cached session data for host '%s'\n\0" as *const u8
                as *const libc::c_char,
            hostname,
        );
        ssl_session = d2i_SSL_SESSION(
            0 as *mut *mut SSL_SESSION,
            &mut sess as *mut *mut libc::c_void as *mut *const libc::c_uchar,
            sesslen as libc::c_long,
        );
        if ssl_session.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"OpenSSL: Could not parse cached session data.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int);
        }
        if SSL_SESSION_is_resumable(ssl_session) == 0 {
            return -(1 as libc::c_int);
        }
        if SSL_set_session(ssl, ssl_session) == 0 {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"OpenSSL: Could not set session data.\n\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            return -(1 as libc::c_int);
        }
        SSL_SESSION_free(ssl_session);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn ssl_save_session(
    mut ssl: *const SSL,
    mut hostname: *const libc::c_char,
) -> libc::c_int {
    let mut sess: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sesslen: libc::c_ulong = 0;
    let mut ssl_session: *mut SSL_SESSION = SSL_get_session(ssl);
    if ssl_session.is_null() || (config.tls_session_cache).is_null() {
        return 0 as libc::c_int;
    }
    sesslen = i2d_SSL_SESSION(
        ssl_session,
        &mut sess as *mut *mut libc::c_void as *mut *mut libc::c_uchar,
    ) as libc::c_ulong;
    if sesslen != 0 {
        wget_tls_session_db_add(
            config.tls_session_cache,
            wget_tls_session_new(
                hostname,
                (18 as libc::c_int * 3600 as libc::c_int) as int64_t,
                sess,
                sesslen,
            ),
        );
        CRYPTO_free(
            sess,
            b"ssl_openssl.c\0" as *const u8 as *const libc::c_char,
            1439 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn wait_2_read_and_write(
    mut sockfd: libc::c_int,
    mut timeout: libc::c_int,
) -> libc::c_int {
    let mut retval: libc::c_int = wget_ready_2_transfer(
        sockfd,
        timeout,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if retval == 0 as libc::c_int {
        retval = WGET_E_TIMEOUT as libc::c_int;
    }
    return retval;
}
unsafe extern "C" fn ssl_set_alpn_offering(
    mut ssl: *mut SSL,
    mut alpn: *const libc::c_char,
) -> bool {
    let mut ret: libc::c_int = WGET_E_UNKNOWN as libc::c_int;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut e: *const libc::c_char = 0 as *const libc::c_char;
    let mut sbuf: [libc::c_char; 32] = [0; 32];
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
        ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    e = alpn;
    s = e;
    while *e != 0 {
        e = strchrnul(s, ',' as i32);
        if e != s {
            if e.offset_from(s) as libc::c_long > 64 as libc::c_int as libc::c_long {
                wget_debug_printf(
                    b"ALPN protocol too long %.*s\n\0" as *const u8
                        as *const libc::c_char,
                    e.offset_from(s) as libc::c_long as libc::c_int,
                    s,
                );
            } else {
                wget_debug_printf(
                    b"ALPN offering %.*s\n\0" as *const u8 as *const libc::c_char,
                    e.offset_from(s) as libc::c_long as libc::c_int,
                    s,
                );
                wget_buffer_memset_append(
                    &mut buf,
                    (e.offset_from(s) as libc::c_long
                        & 0x7f as libc::c_int as libc::c_long) as libc::c_char,
                    1 as libc::c_int as size_t,
                );
                wget_buffer_memcat(
                    &mut buf,
                    s as *const libc::c_void,
                    e.offset_from(s) as libc::c_long as size_t,
                );
            }
        }
        s = e.offset(1 as libc::c_int as isize);
    }
    if buf.length != 0 {
        if SSL_set_alpn_protos(
            ssl,
            buf.data as *mut libc::c_uchar,
            buf.length as libc::c_uint,
        ) != 0
        {
            wget_debug_printf(
                b"OpenSSL: ALPN: Could not set ALPN offering\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            ret = WGET_E_SUCCESS as libc::c_int;
        }
    }
    wget_buffer_deinit(&mut buf);
    return ret != 0;
}
unsafe extern "C" fn ssl_set_alpn_selected_protocol(
    mut ssl: *const SSL,
    mut tcp: *mut wget_tcp,
    mut stats: *mut wget_tls_stats_data,
) {
    let mut data: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut datalen: libc::c_uint = 0;
    SSL_get0_alpn_selected(ssl, &mut data, &mut datalen);
    if !data.is_null() && datalen != 0 {
        wget_debug_printf(
            b"ALPN: Server accepted protocol '%.*s'\n\0" as *const u8
                as *const libc::c_char,
            datalen as libc::c_int,
            data,
        );
        if !stats.is_null() {
            (*stats)
                .alpn_protocol = wget_strmemdup(
                data as *const libc::c_void,
                datalen as size_t,
            );
        }
        if datalen == 2 as libc::c_int as libc::c_uint
            && *data.offset(0 as libc::c_int as isize) as libc::c_int == 'h' as i32
            && *data.offset(1 as libc::c_int as isize) as libc::c_int == '2' as i32
        {
            (*tcp).protocol = 1 as libc::c_int;
            if !stats.is_null() {
                (*stats).http_protocol = 1 as libc::c_int as libc::c_char;
            }
        }
    }
}
unsafe extern "C" fn get_tls_version(mut ssl: *const SSL) -> libc::c_int {
    let mut version: libc::c_int = SSL_version(ssl);
    match version {
        768 => return 1 as libc::c_int,
        769 => return 2 as libc::c_int,
        770 => return 3 as libc::c_int,
        771 => return 4 as libc::c_int,
        772 => return 5 as libc::c_int,
        _ => return -(1 as libc::c_int),
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_open(mut tcp: *mut wget_tcp) -> libc::c_int {
    let mut current_block: u64;
    let mut ssl: *mut SSL = 0 as *mut SSL;
    let mut store: *mut X509_STORE = 0 as *mut X509_STORE;
    let mut retval: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut resumed: libc::c_int = 0;
    let mut vflags: *mut verification_flags = 0 as *mut verification_flags;
    let mut stats: wget_tls_stats_data = {
        let mut init = wget_tls_stats_data {
            hostname: 0 as *const libc::c_char,
            alpn_protocol: 0 as *const libc::c_char,
            tls_secs: 0,
            version: -(1 as libc::c_int),
            cert_chain_size: 0 as libc::c_int,
            http_protocol: 0 as libc::c_int as libc::c_char,
            false_start: 0 as libc::c_int != 0,
            tfo: 0 as libc::c_int != 0,
            tls_con: false,
            resumed: 0 as libc::c_int != 0,
        };
        init
    };
    let mut stats_p: *mut wget_tls_stats_data = 0 as *mut wget_tls_stats_data;
    if tcp.is_null() || (*tcp).sockfd < 0 as libc::c_int {
        return WGET_E_INVALID as libc::c_int;
    }
    if init == 0 {
        wget_ssl_init();
    }
    ssl = SSL_new(_ctx);
    if ssl.is_null() || SSL_set_fd(ssl, (*tcp).sockfd) == 0 {
        retval = WGET_E_UNKNOWN as libc::c_int;
    } else {
        vflags = wget_malloc(
            ::core::mem::size_of::<verification_flags>() as libc::c_ulong,
        ) as *mut verification_flags;
        if vflags.is_null() {
            retval = WGET_E_MEMORY as libc::c_int;
        } else {
            (*vflags).ocsp_stapled_cache = 0 as *mut wget_vector;
            store = SSL_CTX_get_cert_store(_ctx);
            if store.is_null() {
                retval = WGET_E_UNKNOWN as libc::c_int;
            } else {
                (*vflags).certstore = store;
                (*vflags).ocsp_stapled_cache = ocsp_create_stapled_response_vector();
                SSL_set_ex_data(ssl, ssl_userdata_idx, vflags as *mut libc::c_void);
                if tls_stats_callback.is_some() {
                    stats_p = &mut stats;
                }
                if config.check_hostname() {
                    SSL_set1_host(ssl, (*tcp).ssl_hostname);
                    SSL_set_hostflags(ssl, 0x4 as libc::c_int as libc::c_uint);
                } else {
                    SSL_set_hostflags(ssl, 0x20 as libc::c_int as libc::c_uint);
                    wget_info_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Host name check disabled. Server certificate's subject name will not be checked.\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if config.ocsp_stapling() {
                    if SSL_ctrl(
                        ssl,
                        65 as libc::c_int,
                        1 as libc::c_int as libc::c_long,
                        0 as *mut libc::c_void,
                    ) != 0
                    {
                        wget_debug_printf(
                            b"Sending 'status_request' extension in handshake\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    } else {
                        wget_error_printf(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Could not set 'status_request' extension\n\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                    }
                }
                if !((*tcp).ssl_hostname).is_null()
                    && SSL_ctrl(
                        ssl,
                        55 as libc::c_int,
                        0 as libc::c_int as libc::c_long,
                        (*tcp).ssl_hostname as *mut libc::c_void,
                    ) == 0
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"SNI could not be sent\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                if !(config.alpn).is_null()
                    && ssl_set_alpn_offering(ssl, config.alpn) as libc::c_int != 0
                {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"ALPN offering could not be sent\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                resumed = ssl_resume_session(ssl, (*tcp).ssl_hostname);
                if resumed == 1 as libc::c_int {
                    wget_debug_printf(
                        b"Will try to resume cached TLS session\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if resumed == 0 as libc::c_int {
                    wget_debug_printf(
                        b"No cached TLS session available. Will run a full handshake.\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    wget_error_printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Could not get cached TLS session\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                }
                loop {
                    if (*tcp).connect_timeout != 0
                        && {
                            retval = wait_2_read_and_write(
                                (*tcp).sockfd,
                                (*tcp).connect_timeout,
                            );
                            retval < 0 as libc::c_int
                        }
                    {
                        current_block = 9231834683277348185;
                        break;
                    }
                    retval = SSL_connect(ssl);
                    if retval > 0 as libc::c_int {
                        error = 0 as libc::c_int;
                        resumed = SSL_session_reused(ssl);
                        current_block = 13131896068329595644;
                        break;
                    } else {
                        error = SSL_get_error(ssl, retval);
                        if !(error == 2 as libc::c_int || error == 3 as libc::c_int) {
                            current_block = 13131896068329595644;
                            break;
                        }
                    }
                }
                match current_block {
                    9231834683277348185 => {}
                    _ => {
                        if retval <= 0 as libc::c_int {
                            if error == 1 as libc::c_int {
                                wget_error_printf(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Could not complete TLS handshake: %s\n\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    ERR_reason_error_string(ERR_peek_last_error()),
                                );
                            }
                            retval = if ERR_GET_REASON(ERR_peek_last_error())
                                == 134 as libc::c_int
                            {
                                WGET_E_CERTIFICATE as libc::c_int
                            } else {
                                WGET_E_HANDSHAKE as libc::c_int
                            };
                        } else {
                            wget_debug_printf(
                                b"Handshake completed%s\n\0" as *const u8
                                    as *const libc::c_char,
                                if resumed != 0 {
                                    b" (resumed session)\0" as *const u8 as *const libc::c_char
                                } else {
                                    b" (full handshake - not resumed)\0" as *const u8
                                        as *const libc::c_char
                                },
                            );
                            if !(config.hpkp_cache).is_null() {
                                if check_cert_chain_for_hpkp(
                                    SSL_get0_verified_chain(ssl),
                                    (*tcp).ssl_hostname,
                                    &mut (*tcp).hpkp,
                                ) == 0
                                {
                                    wget_error_printf(
                                        dcgettext(
                                            0 as *const libc::c_char,
                                            b"Public key pinning mismatch\n\0" as *const u8
                                                as *const libc::c_char,
                                            5 as libc::c_int,
                                        ),
                                    );
                                    retval = WGET_E_HANDSHAKE as libc::c_int;
                                    current_block = 9231834683277348185;
                                } else {
                                    current_block = 11763295167351361500;
                                }
                            } else {
                                current_block = 11763295167351361500;
                            }
                            match current_block {
                                9231834683277348185 => {}
                                _ => {
                                    if config.ocsp() {
                                        if check_cert_chain_for_ocsp(
                                            SSL_get0_verified_chain(ssl),
                                            store,
                                            (*tcp).ssl_hostname,
                                            (*vflags).ocsp_stapled_cache,
                                        ) == 0
                                        {
                                            wget_error_printf(
                                                dcgettext(
                                                    0 as *const libc::c_char,
                                                    b"Aborting handshake. Could not verify OCSP chain.\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    5 as libc::c_int,
                                                ),
                                            );
                                            retval = WGET_E_HANDSHAKE as libc::c_int;
                                            current_block = 9231834683277348185;
                                        } else {
                                            current_block = 7420279277351916581;
                                        }
                                    } else {
                                        current_block = 7420279277351916581;
                                    }
                                    match current_block {
                                        9231834683277348185 => {}
                                        _ => {
                                            if !((*vflags).ocsp_stapled_cache).is_null() {
                                                ocsp_destroy_stapled_response_vector(
                                                    &mut (*vflags).ocsp_stapled_cache,
                                                );
                                            }
                                            if ssl_save_session(ssl, (*tcp).ssl_hostname) != 0 {
                                                wget_debug_printf(
                                                    b"TLS session saved in cache\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            } else {
                                                wget_debug_printf(
                                                    b"TLS session discarded\0" as *const u8
                                                        as *const libc::c_char,
                                                );
                                            }
                                            if !(config.alpn).is_null() {
                                                ssl_set_alpn_selected_protocol(ssl, tcp, stats_p);
                                            }
                                            if !stats_p.is_null() {
                                                (*stats_p).version = get_tls_version(ssl);
                                                (*stats_p).hostname = (*tcp).ssl_hostname;
                                                (*stats_p).resumed = resumed != 0;
                                                (*stats_p)
                                                    .cert_chain_size = OPENSSL_sk_num(
                                                    ossl_check_const_X509_sk_type(SSL_get0_verified_chain(ssl)),
                                                );
                                                tls_stats_callback
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(stats_p, tls_stats_ctx);
                                                if !((*stats_p).alpn_protocol).is_null() {
                                                    wget_free
                                                        .expect(
                                                            "non-null function pointer",
                                                        )((*stats_p).alpn_protocol as *mut libc::c_void);
                                                    (*stats_p).alpn_protocol = 0 as *const libc::c_char;
                                                }
                                                (*stats_p).tfo = wget_tcp_get_tcp_fastopen(tcp);
                                            }
                                            (*tcp).ssl_session = ssl as *mut libc::c_void;
                                            if !vflags.is_null() {
                                                wget_free
                                                    .expect(
                                                        "non-null function pointer",
                                                    )(vflags as *mut libc::c_void);
                                                vflags = 0 as *mut verification_flags;
                                            }
                                            return WGET_E_SUCCESS as libc::c_int;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !((*vflags).ocsp_stapled_cache).is_null() {
        ocsp_destroy_stapled_response_vector(&mut (*vflags).ocsp_stapled_cache);
    }
    if !stats_p.is_null() {
        if !((*stats_p).alpn_protocol).is_null() {
            wget_free
                .expect(
                    "non-null function pointer",
                )((*stats_p).alpn_protocol as *mut libc::c_void);
            (*stats_p).alpn_protocol = 0 as *const libc::c_char;
        }
    }
    if !vflags.is_null() {
        wget_free.expect("non-null function pointer")(vflags as *mut libc::c_void);
        vflags = 0 as *mut verification_flags;
    }
    if !ssl.is_null() {
        SSL_free(ssl);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_close(mut session: *mut *mut libc::c_void) {
    let mut ssl: *mut SSL = 0 as *mut SSL;
    let mut retval: libc::c_int = 0;
    if !session.is_null() && !(*session).is_null() {
        ssl = *session as *mut SSL;
        loop {
            retval = SSL_shutdown(ssl);
            if !(retval == 0 as libc::c_int) {
                break;
            }
        }
        SSL_free(ssl);
        *session = 0 as *mut libc::c_void;
    }
}
unsafe extern "C" fn ssl_transfer(
    mut want: libc::c_int,
    mut session: *mut libc::c_void,
    mut timeout: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut ssl: *mut SSL = 0 as *mut SSL;
    let mut fd: libc::c_int = 0;
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    ssl = session as *mut SSL;
    if ssl.is_null() {
        return WGET_E_INVALID as libc::c_int;
    }
    fd = SSL_get_fd(ssl);
    if fd < 0 as libc::c_int {
        return WGET_E_UNKNOWN as libc::c_int;
    }
    if timeout < -(1 as libc::c_int) {
        timeout = -(1 as libc::c_int);
    }
    let mut ops: libc::c_int = want;
    loop {
        let mut retval: libc::c_int = 0;
        if timeout != 0 {
            retval = wget_ready_2_transfer(fd, timeout, ops);
            if retval < 0 as libc::c_int {
                return retval
            } else if retval == 0 as libc::c_int {
                return WGET_E_TIMEOUT as libc::c_int
            }
        }
        if want == 1 as libc::c_int {
            retval = SSL_read(ssl, buf, count);
        } else {
            retval = SSL_write(ssl, buf, count);
        }
        if retval > 0 as libc::c_int {
            return retval;
        }
        let mut error: libc::c_int = SSL_get_error(ssl, retval);
        if error == 2 as libc::c_int || error == 3 as libc::c_int {
            ops = 2 as libc::c_int | 1 as libc::c_int;
            if timeout == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
        } else {
            return WGET_E_HANDSHAKE as libc::c_int
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_read_timeout(
    mut session: *mut libc::c_void,
    mut buf: *mut libc::c_char,
    mut count: size_t,
    mut timeout: libc::c_int,
) -> ssize_t {
    let mut retval: libc::c_int = ssl_transfer(
        1 as libc::c_int,
        session,
        timeout,
        buf as *mut libc::c_void,
        count as libc::c_int,
    );
    if retval == WGET_E_HANDSHAKE as libc::c_int {
        let mut msg: *const libc::c_char = ERR_reason_error_string(
            ERR_peek_last_error(),
        );
        if !msg.is_null() {
            wget_error_printf(
                dcgettext(
                    0 as *const libc::c_char,
                    b"TLS read error: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                msg,
            );
        }
        retval = WGET_E_UNKNOWN as libc::c_int;
    }
    return retval as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_write_timeout(
    mut session: *mut libc::c_void,
    mut buf: *const libc::c_char,
    mut count: size_t,
    mut timeout: libc::c_int,
) -> ssize_t {
    let mut retval: libc::c_int = ssl_transfer(
        2 as libc::c_int,
        session,
        timeout,
        buf as *mut libc::c_void,
        count as libc::c_int,
    );
    if retval == WGET_E_HANDSHAKE as libc::c_int {
        wget_error_printf(
            dcgettext(
                0 as *const libc::c_char,
                b"TLS write error: %s\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            ERR_reason_error_string(ERR_peek_last_error()),
        );
        retval = WGET_E_UNKNOWN as libc::c_int;
    }
    return retval as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_set_stats_callback_tls(
    mut fn_0: Option::<wget_tls_stats_callback>,
    mut ctx: *mut libc::c_void,
) {
    tls_stats_callback = fn_0;
    tls_stats_ctx = ctx;
}
#[no_mangle]
pub unsafe extern "C" fn wget_ssl_set_stats_callback_ocsp(
    mut fn_0: Option::<wget_ocsp_stats_callback>,
    mut ctx: *mut libc::c_void,
) {
    ocsp_stats_callback = fn_0;
    ocsp_stats_ctx = ctx;
}
unsafe extern "C" fn run_static_initializers() {
    config = {
        let mut init = config {
            check_certificate_check_hostname_print_info_ocsp_ocsp_date_ocsp_stapling_ocsp_nonce: [0; 1],
            c2rust_padding: [0; 4],
            secure_protocol: b"AUTO\0" as *const u8 as *const libc::c_char,
            ca_directory: b"system\0" as *const u8 as *const libc::c_char,
            ca_file: b"system\0" as *const u8 as *const libc::c_char,
            cert_file: 0 as *const libc::c_char,
            key_file: 0 as *const libc::c_char,
            crl_file: 0 as *const libc::c_char,
            ocsp_server: 0 as *const libc::c_char,
            alpn: 0 as *const libc::c_char,
            ocsp_cert_cache: 0 as *mut wget_ocsp_db,
            ocsp_host_cache: 0 as *mut wget_ocsp_db,
            tls_session_cache: 0 as *mut wget_tls_session_db,
            hpkp_cache: 0 as *mut wget_hpkp_db,
            ca_type: 0 as libc::c_int as libc::c_char,
            cert_type: 0 as libc::c_int as libc::c_char,
            key_type: 0 as libc::c_int as libc::c_char,
        };
        init.set_check_certificate(1 as libc::c_int != 0);
        init.set_check_hostname(1 as libc::c_int != 0);
        init.set_print_info(false);
        init.set_ocsp(0 as libc::c_int != 0);
        init.set_ocsp_date(false);
        init.set_ocsp_stapling(1 as libc::c_int != 0);
        init.set_ocsp_nonce(false);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
