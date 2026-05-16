use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aa_path_list_create_with_directory_contents(
        dir: *const c_char,
        path: *const c_char,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_path_list_create_with_path(
        dir: *const c_char,
        path: *const c_char,
    ) -> *mut c_void;
    pub fn compression_rs_aa_path_list_release(handle: *mut c_void);
    pub fn compression_rs_aa_path_list_node_get_path(
        handle: *mut c_void,
        node: u64,
        capacity: usize,
        path: *mut c_char,
        length: *mut usize,
    ) -> i32;
    pub fn compression_rs_aa_path_list_node_first(handle: *mut c_void) -> u64;
    pub fn compression_rs_aa_path_list_node_next(handle: *mut c_void, node: u64) -> u64;
}
