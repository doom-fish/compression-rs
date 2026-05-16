use std::ffi::c_void;

pub const COMPRESSION_STREAM_FINALIZE: i32 = 0x0001;
pub const COMPRESSION_STATUS_OK: i32 = 0;
pub const COMPRESSION_STATUS_END: i32 = 1;

unsafe extern "C" {
    pub fn compression_rs_compression_stream_create(operation: i32, algorithm: u32) -> *mut c_void;
    pub fn compression_rs_compression_stream_process(
        handle: *mut c_void,
        src_buffer: *const u8,
        src_size: usize,
        dst_buffer: *mut u8,
        dst_size: usize,
        flags: i32,
        src_remaining: *mut usize,
        dst_remaining: *mut usize,
    ) -> i32;
    pub fn compression_rs_compression_stream_release(handle: *mut c_void);
}
