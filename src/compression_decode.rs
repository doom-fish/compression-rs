use crate::{ffi, Algorithm, CompressionError, Decoder, Result};

fn decode_len(dst: &mut [u8], src: &[u8], algorithm: Algorithm) -> usize {
    unsafe {
        ffi::compression_decode::compression_rs_compression_decode_buffer(
            dst.as_mut_ptr(),
            dst.len(),
            src.as_ptr(),
            src.len(),
            algorithm.as_raw(),
        )
    }
}

pub fn compression_decode_scratch_buffer_size(algorithm: Algorithm) -> usize {
    unsafe {
        ffi::compression_decode::compression_rs_compression_decode_scratch_buffer_size(
            algorithm.as_raw(),
        )
    }
}

pub fn compression_decode_buffer(
    dst: &mut [u8],
    src: &[u8],
    algorithm: Algorithm,
) -> Result<usize> {
    let written = decode_len(dst, src, algorithm);
    if src.is_empty() || written > 0 {
        Ok(written)
    } else {
        Err(CompressionError::BufferOperationFailed {
            operation: "compression_decode_buffer",
            algorithm,
            input_len: src.len(),
            output_capacity: dst.len(),
        })
    }
}

pub fn decompress(input: &[u8], algorithm: Algorithm) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if algorithm.supports_streams() {
        let mut decoder = Decoder::new(algorithm)?;
        let mut output = decoder.process(input)?;
        output.extend(decoder.finish()?);
        return Ok(output);
    }

    let mut capacity = 4 * 1024;
    let max_capacity = 64 * 1024 * 1024;
    loop {
        let mut output = vec![0_u8; capacity];
        let written = decode_len(&mut output, input, algorithm);
        if written == 0 {
            return Err(CompressionError::BufferOperationFailed {
                operation: "compression_decode_buffer",
                algorithm,
                input_len: input.len(),
                output_capacity: capacity,
            });
        }
        if written < capacity {
            output.truncate(written);
            return Ok(output);
        }
        if capacity >= max_capacity {
            return Err(CompressionError::BufferOperationFailed {
                operation: "compression_decode_buffer",
                algorithm,
                input_len: input.len(),
                output_capacity: capacity,
            });
        }
        capacity = capacity.saturating_mul(2).min(max_capacity);
    }
}
