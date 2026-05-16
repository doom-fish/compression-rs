#![allow(non_camel_case_types)]

use std::ffi::c_void;

pub type compression_algorithm = u32;
pub type compression_stream_operation = i32;
pub type compression_status = i32;

pub const COMPRESSION_LZ4: compression_algorithm = 0x100;
pub const COMPRESSION_ZLIB: compression_algorithm = 0x205;
pub const COMPRESSION_LZMA: compression_algorithm = 0x306;
pub const COMPRESSION_BROTLI: compression_algorithm = 0xB02;
pub const COMPRESSION_LZFSE: compression_algorithm = 0x801;

pub const COMPRESSION_STREAM_ENCODE: compression_stream_operation = 0;
pub const COMPRESSION_STREAM_DECODE: compression_stream_operation = 1;

pub const COMPRESSION_STREAM_FINALIZE: i32 = 0x0001;

pub const COMPRESSION_STATUS_OK: compression_status = 0;
pub const COMPRESSION_STATUS_END: compression_status = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct compression_stream {
    pub dst_ptr: *mut u8,
    pub dst_size: usize,
    pub src_ptr: *const u8,
    pub src_size: usize,
    pub state: *mut c_void,
}

unsafe extern "C" {
    pub fn compression_encode_scratch_buffer_size(algorithm: compression_algorithm) -> usize;
    pub fn compression_encode_buffer(
        dst_buffer: *mut u8,
        dst_size: usize,
        src_buffer: *const u8,
        src_size: usize,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> usize;
    pub fn compression_decode_scratch_buffer_size(algorithm: compression_algorithm) -> usize;
    pub fn compression_decode_buffer(
        dst_buffer: *mut u8,
        dst_size: usize,
        src_buffer: *const u8,
        src_size: usize,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> usize;
    pub fn compression_stream_init(
        stream: *mut compression_stream,
        operation: compression_stream_operation,
        algorithm: compression_algorithm,
    ) -> compression_status;
    pub fn compression_stream_process(
        stream: *mut compression_stream,
        flags: i32,
    ) -> compression_status;
    pub fn compression_stream_destroy(stream: *mut compression_stream) -> compression_status;
}
