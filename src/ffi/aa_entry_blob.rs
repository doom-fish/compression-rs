use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aa_entry_acl_blob_create() -> *mut c_void;
    pub fn compression_rs_aa_entry_acl_blob_create_with_encoded_data(
        data: *const u8,
        data_size: usize,
    ) -> *mut c_void;
    pub fn compression_rs_aa_entry_acl_blob_create_with_path(
        dir: *const c_char,
        path: *const c_char,
        flags: u64,
    ) -> *mut c_void;
    pub fn compression_rs_aa_entry_acl_blob_apply_to_path(
        handle: *mut c_void,
        dir: *const c_char,
        path: *const c_char,
        flags: u64,
    ) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_get_entry_count(handle: *mut c_void) -> u32;
    pub fn compression_rs_aa_entry_acl_blob_get_entry(
        handle: *mut c_void,
        index: u32,
        tag: *mut u32,
        perms: *mut u64,
        flags: *mut u64,
        qualifier_type: *mut u32,
        qualifier_capacity: usize,
        qualifier_value: *mut u8,
        qualifier_size: *mut usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_append_entry(
        handle: *mut c_void,
        tag: u32,
        perms: u64,
        flags: u64,
        qualifier_type: u32,
        qualifier_value: *const u8,
        qualifier_size: usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_set_entry(
        handle: *mut c_void,
        index: u32,
        tag: u32,
        perms: u64,
        flags: u64,
        qualifier_type: u32,
        qualifier_value: *const u8,
        qualifier_size: usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_clear(handle: *mut c_void) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_remove_entry(handle: *mut c_void, index: u32) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_get_encoded_size(handle: *mut c_void) -> usize;
    pub fn compression_rs_aa_entry_acl_blob_copy_encoded_data(
        handle: *mut c_void,
        dst: *mut u8,
    ) -> bool;
    pub fn compression_rs_aa_entry_acl_blob_clone_from_raw(raw: *mut c_void) -> *mut c_void;
    pub fn compression_rs_aa_entry_acl_blob_clear_raw(raw: *mut c_void) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_append_entry_raw(
        raw: *mut c_void,
        tag: u32,
        perms: u64,
        flags: u64,
        qualifier_type: u32,
        qualifier_value: *const u8,
        qualifier_size: usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_acl_blob_release(handle: *mut c_void);

    pub fn compression_rs_aa_entry_xat_blob_create() -> *mut c_void;
    pub fn compression_rs_aa_entry_xat_blob_create_with_encoded_data(
        data: *const u8,
        data_size: usize,
    ) -> *mut c_void;
    pub fn compression_rs_aa_entry_xat_blob_create_with_path(
        dir: *const c_char,
        path: *const c_char,
        flags: u64,
    ) -> *mut c_void;
    pub fn compression_rs_aa_entry_xat_blob_apply_to_path(
        handle: *mut c_void,
        dir: *const c_char,
        path: *const c_char,
        flags: u64,
    ) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_get_entry_count(handle: *mut c_void) -> u32;
    pub fn compression_rs_aa_entry_xat_blob_get_entry(
        handle: *mut c_void,
        index: u32,
        key_capacity: usize,
        key: *mut c_char,
        key_length: *mut usize,
        data_capacity: usize,
        data: *mut u8,
        data_size: *mut usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_append_entry(
        handle: *mut c_void,
        key: *const c_char,
        data: *const u8,
        data_size: usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_set_entry(
        handle: *mut c_void,
        index: u32,
        key: *const c_char,
        data: *const u8,
        data_size: usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_clear(handle: *mut c_void) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_remove_entry(handle: *mut c_void, index: u32) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_get_encoded_size(handle: *mut c_void) -> usize;
    pub fn compression_rs_aa_entry_xat_blob_copy_encoded_data(
        handle: *mut c_void,
        dst: *mut u8,
    ) -> bool;
    pub fn compression_rs_aa_entry_xat_blob_clone_from_raw(raw: *mut c_void) -> *mut c_void;
    pub fn compression_rs_aa_entry_xat_blob_clear_raw(raw: *mut c_void) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_append_entry_raw(
        raw: *mut c_void,
        key: *const c_char,
        data: *const u8,
        data_size: usize,
    ) -> i32;
    pub fn compression_rs_aa_entry_xat_blob_release(handle: *mut c_void);
}
