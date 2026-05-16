unsafe extern "C" {
    pub fn compression_rs_compression_decode_scratch_buffer_size(algorithm: u32) -> usize;
    pub fn compression_rs_compression_decode_buffer(
        dst_buffer: *mut u8,
        dst_size: usize,
        src_buffer: *const u8,
        src_size: usize,
        algorithm: u32,
    ) -> usize;
}
