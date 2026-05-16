use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn compression_rs_aa_byte_stream_open_with_fd(
        fd: i32,
        automatic_close: bool,
    ) -> *mut c_void;
    pub fn compression_rs_aa_byte_stream_open_with_path(
        path: *const c_char,
        open_flags: i32,
        open_mode: u32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_temp_file_stream_open() -> *mut c_void;
    pub fn compression_rs_aa_shared_buffer_pipe_open(
        ostream: *mut *mut c_void,
        istream: *mut *mut c_void,
        buffer_capacity: usize,
    ) -> i32;
    pub fn compression_rs_aa_compression_output_stream_open(
        handle: *mut c_void,
        compression_algorithm: u32,
        block_size: usize,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_compression_output_stream_open_existing(
        handle: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_decompression_input_stream_open(
        handle: *mut c_void,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_decompression_random_access_input_stream_open(
        handle: *mut c_void,
        alloc_limit: usize,
        flags: u64,
        n_threads: i32,
    ) -> *mut c_void;
    pub fn compression_rs_aa_byte_stream_write(
        handle: *mut c_void,
        buffer: *const u8,
        length: usize,
    ) -> i64;
    pub fn compression_rs_aa_byte_stream_pwrite(
        handle: *mut c_void,
        buffer: *const u8,
        length: usize,
        offset: i64,
    ) -> i64;
    pub fn compression_rs_aa_byte_stream_read(
        handle: *mut c_void,
        buffer: *mut u8,
        length: usize,
    ) -> i64;
    pub fn compression_rs_aa_byte_stream_pread(
        handle: *mut c_void,
        buffer: *mut u8,
        length: usize,
        offset: i64,
    ) -> i64;
    pub fn compression_rs_aa_byte_stream_seek(handle: *mut c_void, offset: i64, whence: i32)
        -> i64;
    pub fn compression_rs_aa_byte_stream_cancel(handle: *mut c_void);
    pub fn compression_rs_aa_byte_stream_close(handle: *mut c_void) -> i32;
    pub fn compression_rs_aa_byte_stream_process(input: *mut c_void, output: *mut c_void) -> i64;
    pub fn compression_rs_aa_random_access_byte_stream_process(
        input: *mut c_void,
        output: *mut c_void,
        max_offset: i64,
        block_size: usize,
        flags: u64,
        n_threads: i32,
    ) -> i64;
    pub fn compression_rs_aa_byte_stream_release(handle: *mut c_void);
}
