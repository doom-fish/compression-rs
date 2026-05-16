mod support;

use compression::{compression_encode_buffer, compression_encode_scratch_buffer_size, Algorithm};
use support::pseudo_random_bytes;

#[test]
fn one_shot_encode_supports_lz4_raw() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(8 * 1024);
    let mut output = vec![0_u8; input.len() * 2];
    let written = compression_encode_buffer(&mut output, &input, Algorithm::Lz4Raw)?;
    assert!(written > 0);
    Ok(())
}

#[test]
fn scratch_size_queries_are_exposed() {
    let lz4 = compression_encode_scratch_buffer_size(Algorithm::Lz4);
    let lzfse = compression_encode_scratch_buffer_size(Algorithm::Lzfse);
    assert!(lzfse >= lz4);
}
