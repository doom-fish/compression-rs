use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aa_field_key_set_create() -> *mut c_void;
    pub fn compression_rs_aa_field_key_set_create_with_string(value: *const c_char) -> *mut c_void;
    pub fn compression_rs_aa_field_key_set_clone(handle: *mut c_void) -> *mut c_void;
    pub fn compression_rs_aa_field_key_set_release(handle: *mut c_void);
    pub fn compression_rs_aa_field_key_set_clear(handle: *mut c_void) -> i32;
    pub fn compression_rs_aa_field_key_set_contains_key(handle: *mut c_void, key: u32) -> i32;
    pub fn compression_rs_aa_field_key_set_insert_key(handle: *mut c_void, key: u32) -> i32;
    pub fn compression_rs_aa_field_key_set_remove_key(handle: *mut c_void, key: u32) -> i32;
    pub fn compression_rs_aa_field_key_set_insert_key_set(
        handle: *mut c_void,
        other: *mut c_void,
    ) -> i32;
    pub fn compression_rs_aa_field_key_set_remove_key_set(
        handle: *mut c_void,
        other: *mut c_void,
    ) -> i32;
    pub fn compression_rs_aa_field_key_set_select_key_set(
        handle: *mut c_void,
        other: *mut c_void,
    ) -> i32;
    pub fn compression_rs_aa_field_key_set_get_key_count(handle: *mut c_void) -> u32;
    pub fn compression_rs_aa_field_key_set_get_key(handle: *mut c_void, index: u32) -> u32;
    pub fn compression_rs_aa_field_key_set_serialize(
        handle: *mut c_void,
        capacity: usize,
        buffer: *mut c_char,
    ) -> i32;
}
