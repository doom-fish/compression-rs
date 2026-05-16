mod support;

use compression::{compression_decode_buffer, compression_encode_buffer, Algorithm};
use support::pseudo_random_bytes;

#[test]
fn one_shot_decode_round_trips_lz4_raw() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(8 * 1024);
    let mut encoded = vec![0_u8; input.len() * 2];
    let encoded_len = compression_encode_buffer(&mut encoded, &input, Algorithm::Lz4Raw)?;

    let mut decoded = vec![0_u8; input.len()];
    let decoded_len =
        compression_decode_buffer(&mut decoded, &encoded[..encoded_len], Algorithm::Lz4Raw)?;

    assert_eq!(decoded_len, input.len());
    assert_eq!(&decoded[..decoded_len], input.as_slice());
    Ok(())
}
