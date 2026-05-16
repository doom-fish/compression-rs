mod common;

use common::pseudo_random_bytes;
use compression::{Algorithm, Decoder, Encoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(64 * 1024);
    let split = input.len() / 2;

    let mut encoder = Encoder::new(Algorithm::Lzfse)?;
    let mut compressed = encoder.process(&input[..split])?;
    compressed.extend(encoder.process(&input[split..])?);
    compressed.extend(encoder.finish()?);

    let mut decoder = Decoder::new(Algorithm::Lzfse)?;
    let encoded_split = compressed.len() / 2;
    let mut round_trip = decoder.process(&compressed[..encoded_split])?;
    round_trip.extend(decoder.process(&compressed[encoded_split..])?);
    round_trip.extend(decoder.finish()?);

    assert_eq!(round_trip, input);
    let input_len = input.len();
    let compressed_len = compressed.len();
    println!("input={input_len} compressed={compressed_len}");
    println!("✅ compression stream round-trip OK");
    Ok(())
}
