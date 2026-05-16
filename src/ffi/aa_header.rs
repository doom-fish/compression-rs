use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aa_header_create() -> *mut c_void;
    pub fn compression_rs_aa_header_create_with_encoded_data(
        data_size: usize,
        data: *const u8,
    ) -> *mut c_void;
    pub fn compression_rs_aa_header_clone(handle: *mut c_void) -> *mut c_void;
    pub fn compression_rs_aa_header_create_with_path(
        key_set: *mut c_void,
        dir: *const c_char,
        path: *const c_char,
        flags: u64,
    ) -> *mut c_void;
    pub fn compression_rs_aa_header_release(handle: *mut c_void);
    pub fn compression_rs_aa_header_assign(handle: *mut c_void, other: *mut c_void) -> i32;
    pub fn compression_rs_aa_header_get_field_count(handle: *mut c_void) -> u32;
    pub fn compression_rs_aa_header_get_key_index(handle: *mut c_void, key: u32) -> i32;
    pub fn compression_rs_aa_header_get_field_type(handle: *mut c_void, index: u32) -> i32;
    pub fn compression_rs_aa_header_get_field_key(handle: *mut c_void, index: u32) -> u32;
    pub fn compression_rs_aa_header_get_payload_size(handle: *mut c_void) -> u64;
    pub fn compression_rs_aa_header_remove_field(handle: *mut c_void, index: u32) -> i32;
    pub fn compression_rs_aa_header_clear(handle: *mut c_void) -> i32;
    pub fn compression_rs_aa_header_set_field_flag(
        handle: *mut c_void,
        index: u32,
        key: u32,
    ) -> i32;
    pub fn compression_rs_aa_header_set_field_uint(
        handle: *mut c_void,
        index: u32,
        key: u32,
        value: u64,
    ) -> i32;
    pub fn compression_rs_aa_header_set_field_string(
        handle: *mut c_void,
        index: u32,
        key: u32,
        value: *const c_char,
        length: usize,
    ) -> i32;
    pub fn compression_rs_aa_header_set_field_hash(
        handle: *mut c_void,
        index: u32,
        key: u32,
        hash_function: u32,
        value: *const u8,
    ) -> i32;
    pub fn compression_rs_aa_header_set_field_timespec(
        handle: *mut c_void,
        index: u32,
        key: u32,
        seconds: i64,
        nanoseconds: i64,
    ) -> i32;
    pub fn compression_rs_aa_header_set_field_blob(
        handle: *mut c_void,
        index: u32,
        key: u32,
        size: u64,
    ) -> i32;
    pub fn compression_rs_aa_header_get_field_uint(
        handle: *mut c_void,
        index: u32,
        value: *mut u64,
    ) -> i32;
    pub fn compression_rs_aa_header_get_field_string(
        handle: *mut c_void,
        index: u32,
        capacity: usize,
        value: *mut c_char,
        length: *mut usize,
    ) -> i32;
    pub fn compression_rs_aa_header_get_field_hash(
        handle: *mut c_void,
        index: u32,
        capacity: usize,
        hash_function: *mut u32,
        value: *mut u8,
    ) -> i32;
    pub fn compression_rs_aa_header_get_field_timespec(
        handle: *mut c_void,
        index: u32,
        seconds: *mut i64,
        nanoseconds: *mut i64,
    ) -> i32;
    pub fn compression_rs_aa_header_get_field_blob(
        handle: *mut c_void,
        index: u32,
        size: *mut u64,
        offset: *mut u64,
    ) -> i32;
    pub fn compression_rs_aa_header_get_encoded_size(handle: *mut c_void) -> usize;
    pub fn compression_rs_aa_header_copy_encoded_data(handle: *mut c_void, dst: *mut u8) -> bool;
    pub fn compression_rs_aa_header_clone_from_raw(raw: *mut c_void) -> *mut c_void;
    pub fn compression_rs_aa_header_clone_raw(handle: *mut c_void) -> *mut c_void;
}
