mod support;

use compression::{Algorithm, Decoder, Encoder};
use support::pseudo_random_bytes;

#[test]
fn compression_stream_round_trips_in_chunks() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(32 * 1024);
    let split = input.len() / 2;

    let mut stream_encoder = Encoder::new(Algorithm::Lzfse)?;
    let mut encoded_bytes = stream_encoder.process(&input[..split])?;
    encoded_bytes.extend(stream_encoder.process(&input[split..])?);
    encoded_bytes.extend(stream_encoder.finish()?);
    assert_eq!(stream_encoder.finish()?, [] as [u8; 0]);

    let encoded_split = encoded_bytes.len() / 2;
    let mut stream_decoder = Decoder::new(Algorithm::Lzfse)?;
    let mut decoded_bytes = stream_decoder.process(&encoded_bytes[..encoded_split])?;
    decoded_bytes.extend(stream_decoder.process(&encoded_bytes[encoded_split..])?);
    decoded_bytes.extend(stream_decoder.finish()?);
    assert_eq!(stream_decoder.finish()?, [] as [u8; 0]);

    assert_eq!(decoded_bytes, input);
    Ok(())
}
