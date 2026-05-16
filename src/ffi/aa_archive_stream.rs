use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aa_extract_archive_output_stream_open(
        dir: *const c_char,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_convert_archive_output_stream_open(
        handle: *mut c_void,
        insert_key_set: *mut c_void,
        remove_key_set: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_encode_archive_output_stream_open(
        handle: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_decode_archive_input_stream_open(
        handle: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_archive_stream_write_header(
        handle: *mut c_void,
        header: *mut c_void,
    ) -> i32;
    pub fn compression_rs_aa_archive_stream_write_blob(
        handle: *mut c_void,
        key: u32,
        buffer: *const u8,
        length: usize,
    ) -> i32;
    pub fn compression_rs_aa_archive_stream_read_header_new(
        handle: *mut c_void,
        status: *mut i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_archive_stream_read_header_into(
        handle: *mut c_void,
        header: *mut c_void,
    ) -> i32;
    pub fn compression_rs_aa_archive_stream_read_blob(
        handle: *mut c_void,
        key: u32,
        buffer: *mut u8,
        length: usize,
    ) -> i32;
    pub fn compression_rs_aa_archive_stream_cancel(handle: *mut c_void);
    pub fn compression_rs_aa_archive_stream_close(handle: *mut c_void) -> i32;
    pub fn compression_rs_aa_archive_stream_write_path_list(
        handle: *mut c_void,
        path_list: *mut c_void,
        key_set: *mut c_void,
        dir: *const c_char,
        flags: u64,
        n_threads: i32,
    ) -> i32;
    pub fn compression_rs_aa_archive_stream_process(
        input: *mut c_void,
        output: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> i64;
    pub fn compression_rs_aa_archive_stream_release(handle: *mut c_void);
}
