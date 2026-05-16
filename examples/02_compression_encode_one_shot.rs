mod common;

use common::pseudo_random_bytes;
use compression::{compression_encode_buffer, compression_encode_scratch_buffer_size, Algorithm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(8 * 1024);
    let mut output = vec![0_u8; input.len() * 2];
    let written = compression_encode_buffer(&mut output, &input, Algorithm::Lz4Raw)?;
    assert!(written > 0);
    let scratch_size = compression_encode_scratch_buffer_size(Algorithm::Lz4Raw);
    println!("scratch={scratch_size} bytes written={written}");
    println!("✅ one-shot compression encode OK");
    Ok(())
}
