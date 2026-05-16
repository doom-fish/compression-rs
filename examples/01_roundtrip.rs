use compression::{compress, decompress, Algorithm};

fn pseudo_random_bytes(len: usize) -> Vec<u8> {
    let mut state = 0x0123_4567_89ab_cdef_u64;
    let mut bytes = Vec::with_capacity(len);
    for _ in 0..len {
        state ^= state << 7;
        state ^= state >> 9;
        state ^= state << 8;
        bytes.push(state.to_le_bytes()[0]);
    }
    bytes
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(64 * 1024);
    let compressed = compress(&input, Algorithm::Lzfse)?;
    let decompressed = decompress(&compressed, Algorithm::Lzfse)?;

    assert_eq!(decompressed, input);
    println!("input={} compressed={}", input.len(), compressed.len());
    println!("✅ compression round-trip OK");
    Ok(())
}
