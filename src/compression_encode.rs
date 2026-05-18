use crate::{ffi, Algorithm, CompressionError, Encoder, Result};

fn encode_len(dst: &mut [u8], src: &[u8], algorithm: Algorithm) -> usize {
    unsafe {
        ffi::compression_encode::compression_rs_compression_encode_buffer(
            dst.as_mut_ptr(),
            dst.len(),
            src.as_ptr(),
            src.len(),
            algorithm.as_raw(),
        )
    }
}

/// Wraps `compression_encode_buffer`.
pub fn compression_encode_scratch_buffer_size(algorithm: Algorithm) -> usize {
    unsafe {
        ffi::compression_encode::compression_rs_compression_encode_scratch_buffer_size(
            algorithm.as_raw(),
        )
    }
}

/// Wraps `compression_encode_buffer`.
pub fn compression_encode_buffer(
    dst: &mut [u8],
    src: &[u8],
    algorithm: Algorithm,
) -> Result<usize> {
    let written = encode_len(dst, src, algorithm);
    if src.is_empty() || written > 0 {
        Ok(written)
    } else {
        Err(CompressionError::BufferOperationFailed {
            operation: "compression_encode_buffer",
            algorithm,
            input_len: src.len(),
            output_capacity: dst.len(),
        })
    }
}

/// Wraps `compression_encode_buffer`.
pub fn compress(input: &[u8], algorithm: Algorithm) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if algorithm.supports_streams() {
        let mut encoder = Encoder::new(algorithm)?;
        let mut output = encoder.process(input)?;
        output.extend(encoder.finish()?);
        return Ok(output);
    }

    let max_capacity = input
        .len()
        .saturating_mul(8)
        .saturating_add(1 << 20)
        .max(64);
    let mut capacity = input
        .len()
        .saturating_add(input.len() / 8)
        .saturating_add(64)
        .max(64);

    loop {
        let mut output = vec![0_u8; capacity];
        let written = encode_len(&mut output, input, algorithm);
        if written > 0 {
            output.truncate(written);
            return Ok(output);
        }
        if capacity >= max_capacity {
            return Err(CompressionError::BufferOperationFailed {
                operation: "compression_encode_buffer",
                algorithm,
                input_len: input.len(),
                output_capacity: capacity,
            });
        }
        capacity = capacity.saturating_mul(2).min(max_capacity);
    }
}
