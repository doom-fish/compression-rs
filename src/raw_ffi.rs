#![allow(non_camel_case_types)]

use std::ffi::c_void;

/// Wraps `compression_algorithm` from `compression.h`.
pub type compression_algorithm = u32;
/// Wraps `compression_stream_operation` from `compression.h`.
pub type compression_stream_operation = i32;
/// Wraps `compression_status` from `compression.h`.
pub type compression_status = i32;

/// Wraps `COMPRESSION_LZ4` from `compression.h`.
pub const COMPRESSION_LZ4: compression_algorithm = 0x100;
/// Wraps `COMPRESSION_ZLIB` from `compression.h`.
pub const COMPRESSION_ZLIB: compression_algorithm = 0x205;
/// Wraps `COMPRESSION_LZMA` from `compression.h`.
pub const COMPRESSION_LZMA: compression_algorithm = 0x306;
/// Wraps `COMPRESSION_LZ4_RAW` from `compression.h`.
pub const COMPRESSION_LZ4_RAW: compression_algorithm = 0x101;
/// Wraps `COMPRESSION_BROTLI` from `compression.h`.
pub const COMPRESSION_BROTLI: compression_algorithm = 0xB02;
/// Wraps `COMPRESSION_LZFSE` from `compression.h`.
pub const COMPRESSION_LZFSE: compression_algorithm = 0x801;
/// Wraps `COMPRESSION_LZBITMAP` from `compression.h`.
pub const COMPRESSION_LZBITMAP: compression_algorithm = 0x702;

/// Wraps `COMPRESSION_STREAM_ENCODE` from `compression.h`.
pub const COMPRESSION_STREAM_ENCODE: compression_stream_operation = 0;
/// Wraps `COMPRESSION_STREAM_DECODE` from `compression.h`.
pub const COMPRESSION_STREAM_DECODE: compression_stream_operation = 1;

/// Wraps `COMPRESSION_STREAM_FINALIZE` from `compression.h`.
pub const COMPRESSION_STREAM_FINALIZE: i32 = 0x0001;

/// Wraps `COMPRESSION_STATUS_OK` from `compression.h`.
pub const COMPRESSION_STATUS_OK: compression_status = 0;
/// Wraps `COMPRESSION_STATUS_ERROR` from `compression.h`.
pub const COMPRESSION_STATUS_ERROR: compression_status = -1;
/// Wraps `COMPRESSION_STATUS_END` from `compression.h`.
pub const COMPRESSION_STATUS_END: compression_status = 1;

/// Wraps `compression_stream` values used by Compression or AppleArchive.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct compression_stream {
    /// Wraps the `dst_ptr` field of `compression_stream`.
    pub dst_ptr: *mut u8,
    /// Wraps the `dst_size` field of `compression_stream`.
    pub dst_size: usize,
    /// Wraps the `src_ptr` field of `compression_stream`.
    pub src_ptr: *const u8,
    /// Wraps the `src_size` field of `compression_stream`.
    pub src_size: usize,
    /// Wraps the `state` field of `compression_stream`.
    pub state: *mut c_void,
}

unsafe extern "C" {
    /// Wraps the `compression_encode_scratch_buffer_size` convenience for `Compression`.
    pub fn compression_encode_scratch_buffer_size(algorithm: compression_algorithm) -> usize;
    /// Wraps the `compression_encode_buffer` convenience for `Compression`.
    pub fn compression_encode_buffer(
        dst_buffer: *mut u8,
        dst_size: usize,
        src_buffer: *const u8,
        src_size: usize,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> usize;
    /// Wraps the `compression_decode_scratch_buffer_size` convenience for `Compression`.
    pub fn compression_decode_scratch_buffer_size(algorithm: compression_algorithm) -> usize;
    /// Wraps the `compression_decode_buffer` convenience for `Compression`.
    pub fn compression_decode_buffer(
        dst_buffer: *mut u8,
        dst_size: usize,
        src_buffer: *const u8,
        src_size: usize,
        scratch_buffer: *mut c_void,
        algorithm: compression_algorithm,
    ) -> usize;
    /// Wraps the `compression_stream_init` convenience for `Compression`.
    pub fn compression_stream_init(
        stream: *mut compression_stream,
        operation: compression_stream_operation,
        algorithm: compression_algorithm,
    ) -> compression_status;
    /// Wraps the `compression_stream_process` convenience for `Compression`.
    pub fn compression_stream_process(
        stream: *mut compression_stream,
        flags: i32,
    ) -> compression_status;
    /// Wraps the `compression_stream_destroy` convenience for `Compression`.
    pub fn compression_stream_destroy(stream: *mut compression_stream) -> compression_status;
}
