use libc::*;

use *;

pub enum PKCS12 {}

extern "C" {
    pub fn PKCS12_free(p12: *mut PKCS12);
    pub fn i2d_PKCS12(a: *const PKCS12, buf: *mut *mut u8) -> c_int;
    pub fn d2i_PKCS12(a: *mut *mut PKCS12, pp: *mut *const u8, length: size_t) -> *mut PKCS12;

    pub fn PKCS12_parse(
        p12: *const PKCS12,
        pass: *const c_char,
        pkey: *mut *mut EVP_PKEY,
        cert: *mut *mut X509,
        ca: *mut *mut stack_st_X509,
    ) -> c_int;
}

extern "C" {
    pub fn PKCS12_create(
        pass: *const c_char,
        friendly_name: *const c_char,
        pkey: *const EVP_PKEY,
        cert: *mut X509,
        ca: *const stack_st_X509,
        nid_key: c_int,
        nid_cert: c_int,
        iter: c_int,
        mac_iter: c_int,
        keytype: c_int,
    ) -> *mut PKCS12;
}

extern "C" {
    pub fn i2d_PKCS12_bio(b: *mut BIO, a: *const PKCS12) -> c_int;
}
