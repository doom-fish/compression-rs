use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aea_context_create_with_profile(profile: u32) -> *mut c_void;
    pub fn compression_rs_aea_context_create_with_encrypted_stream(
        stream: *mut c_void,
    ) -> *mut c_void;
    pub fn compression_rs_aea_context_get_field_uint(handle: *mut c_void, field: u32) -> u64;
    pub fn compression_rs_aea_context_get_field_blob(
        handle: *mut c_void,
        field: u32,
        representation: u32,
        buf_capacity: usize,
        buf: *mut u8,
        buf_size: *mut usize,
    ) -> i32;
    pub fn compression_rs_aea_context_set_field_uint(
        handle: *mut c_void,
        field: u32,
        value: u64,
    ) -> i32;
    pub fn compression_rs_aea_context_set_field_blob(
        handle: *mut c_void,
        field: u32,
        representation: u32,
        buf: *const u8,
        buf_size: usize,
    ) -> i32;
    pub fn compression_rs_aea_context_generate_field_blob(handle: *mut c_void, field: u32) -> i32;
    pub fn compression_rs_aea_context_decrypt_attributes(handle: *mut c_void) -> i32;
    pub fn compression_rs_aea_context_release(handle: *mut c_void);

    pub fn compression_rs_aea_encryption_output_stream_open(
        stream: *mut c_void,
        context: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aea_encryption_output_stream_open_existing(
        stream: *mut c_void,
        context: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aea_encryption_output_stream_close_and_update_context(
        stream: *mut c_void,
        context: *mut c_void,
    ) -> i32;
    pub fn compression_rs_aea_decryption_input_stream_open(
        stream: *mut c_void,
        context: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aea_decryption_random_access_input_stream_open(
        stream: *mut c_void,
        context: *mut c_void,
        alloc_limit: usize,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aea_stream_sign(stream: *mut c_void, context: *mut c_void) -> i32;

    pub fn compression_rs_aea_auth_data_create() -> *mut c_void;
    pub fn compression_rs_aea_auth_data_create_with_context(context: *mut c_void) -> *mut c_void;
    pub fn compression_rs_aea_auth_data_get_entry_count(handle: *mut c_void) -> u32;
    pub fn compression_rs_aea_auth_data_get_entry(
        handle: *mut c_void,
        index: u32,
        key_capacity: usize,
        key: *mut c_char,
        key_length: *mut usize,
        data_capacity: usize,
        data: *mut u8,
        data_size: *mut usize,
    ) -> i32;
    pub fn compression_rs_aea_auth_data_append_entry(
        handle: *mut c_void,
        key: *const c_char,
        data: *const u8,
        data_size: usize,
    ) -> i32;
    pub fn compression_rs_aea_auth_data_set_entry(
        handle: *mut c_void,
        index: u32,
        key: *const c_char,
        data: *const u8,
        data_size: usize,
    ) -> i32;
    pub fn compression_rs_aea_auth_data_clear(handle: *mut c_void) -> i32;
    pub fn compression_rs_aea_auth_data_remove_entry(handle: *mut c_void, index: u32) -> i32;
    pub fn compression_rs_aea_auth_data_get_encoded_size(handle: *mut c_void) -> usize;
    pub fn compression_rs_aea_auth_data_copy_encoded_data(
        handle: *mut c_void,
        dst: *mut u8,
    ) -> bool;
    pub fn compression_rs_aea_auth_data_release(handle: *mut c_void);
}
