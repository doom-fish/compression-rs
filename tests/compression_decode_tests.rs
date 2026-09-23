mod support;

use compression::{
    compress, compression_decode_buffer, compression_encode_buffer, decompress,
    decompress_with_max_output_len, Algorithm, CompressionError, Decoder, DEFAULT_MAX_OUTPUT_LEN,
};
use support::pseudo_random_bytes;

fn sample() -> Vec<u8> {
    let mut data = b"doom fish ".repeat(400);
    data.extend(pseudo_random_bytes(1000));
    data
}

fn encoded(input: &[u8], algorithm: Algorithm) -> Vec<u8> {
    let mut output = vec![0_u8; input.len() * 2 + 1024];
    let written = compression_encode_buffer(&mut output, input, algorithm).expect("encode");
    output.truncate(written);
    output
}

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

#[test]
fn decode_buffer_accepts_exact_fits_and_rejects_truncation() {
    let input = sample();
    for algorithm in Algorithm::BUFFER_ALL {
        let encoded = encoded(&input, algorithm);

        let mut exact = vec![0_u8; input.len()];
        assert_eq!(
            compression_decode_buffer(&mut exact, &encoded, algorithm),
            Ok(input.len()),
            "{algorithm:?}"
        );
        assert_eq!(exact, input, "{algorithm:?}");

        let mut roomy = vec![0_u8; input.len() + 100];
        assert_eq!(
            compression_decode_buffer(&mut roomy, &encoded, algorithm),
            Ok(input.len()),
            "{algorithm:?}"
        );

        let mut short = vec![0_u8; input.len() - 1];
        assert_eq!(
            compression_decode_buffer(&mut short, &encoded, algorithm),
            Err(CompressionError::OutputLimitExceeded {
                operation: "compression_decode_buffer",
                algorithm,
                max_output_len: input.len() - 1,
            }),
            "{algorithm:?}"
        );
    }
}

#[test]
fn encodings_of_empty_input_decode_to_empty_output() {
    for algorithm in Algorithm::BUFFER_ALL {
        let empty = encoded(&[], algorithm);
        let mut dst = [0_u8; 16];
        assert_eq!(
            compression_decode_buffer(&mut dst, &empty, algorithm),
            Ok(0),
            "{algorithm:?}"
        );
        assert_eq!(compression_decode_buffer(&mut [], &empty, algorithm), Ok(0));
        assert_eq!(
            decompress(&empty, algorithm),
            Ok(Vec::new()),
            "{algorithm:?}"
        );
    }
}

#[test]
fn invalid_and_truncated_input_is_an_error() {
    let input = sample();
    for algorithm in Algorithm::BUFFER_ALL {
        let mut dst = vec![0_u8; input.len()];
        assert!(
            compression_decode_buffer(&mut dst, b"definitely not compressed", algorithm).is_err(),
            "{algorithm:?}"
        );
        assert!(decompress(b"definitely not compressed", algorithm).is_err());
    }
    for algorithm in Algorithm::ALL {
        let encoded = encoded(&input, algorithm);
        let truncated = &encoded[..encoded.len() - 4];
        let mut dst = vec![0_u8; input.len()];
        assert!(
            compression_decode_buffer(&mut dst, truncated, algorithm).is_err(),
            "{algorithm:?}"
        );
        assert!(decompress(truncated, algorithm).is_err(), "{algorithm:?}");
    }
}

#[test]
fn decompress_honors_the_output_limit_on_every_path() {
    let input = sample();
    for algorithm in Algorithm::BUFFER_ALL {
        let encoded = encoded(&input, algorithm);
        assert_eq!(
            decompress_with_max_output_len(&encoded, algorithm, input.len()),
            Ok(input.clone()),
            "{algorithm:?}"
        );
        assert!(
            matches!(
                decompress_with_max_output_len(&encoded, algorithm, input.len() - 1),
                Err(CompressionError::OutputLimitExceeded { max_output_len, .. })
                    if max_output_len == input.len() - 1
            ),
            "{algorithm:?}"
        );
    }
}

#[test]
fn default_limit_stops_a_decompression_bomb() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(DEFAULT_MAX_OUTPUT_LEN, 64 * 1024 * 1024);
    let bomb = compress(&vec![0_u8; DEFAULT_MAX_OUTPUT_LEN + 1], Algorithm::Zlib)?;
    assert!(bomb.len() < 1024 * 1024);
    assert!(matches!(
        decompress(&bomb, Algorithm::Zlib),
        Err(CompressionError::OutputLimitExceeded {
            max_output_len: DEFAULT_MAX_OUTPUT_LEN,
            ..
        })
    ));
    assert_eq!(
        decompress_with_max_output_len(&bomb, Algorithm::Zlib, DEFAULT_MAX_OUTPUT_LEN + 1)?.len(),
        DEFAULT_MAX_OUTPUT_LEN + 1
    );
    Ok(())
}

#[test]
fn decoder_limit_is_cumulative_across_chunks() -> Result<(), Box<dyn std::error::Error>> {
    let input = sample();
    let encoded = compress(&input, Algorithm::Lzfse)?;
    let mut decoder = Decoder::with_max_output_len(Algorithm::Lzfse, input.len() - 1)?;
    let mut result = Ok(Vec::new());
    for chunk in encoded.chunks(64) {
        result = decoder.process(chunk);
        if result.is_err() {
            break;
        }
    }
    if result.is_ok() {
        result = decoder.finish();
    }
    assert!(matches!(
        result,
        Err(CompressionError::OutputLimitExceeded { .. })
    ));
    assert!(matches!(
        decoder.process(&[]),
        Err(CompressionError::StreamFinished { .. })
    ));

    let mut decoder = Decoder::with_max_output_len(Algorithm::Lzfse, input.len())?;
    let mut output = Vec::new();
    for chunk in encoded.chunks(64) {
        output.extend(decoder.process(chunk)?);
    }
    output.extend(decoder.finish()?);
    assert_eq!(output, input);
    Ok(())
}
