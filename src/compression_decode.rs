use crate::compression_encode::encode_len;
use crate::compression_stream::{CompressionStream, StreamOperation, DEFAULT_MAX_OUTPUT_LEN};
use crate::{ffi, Algorithm, CompressionError, Decoder, Result};

const OPERATION: &str = "compression_decode_buffer";

unsafe fn decode_raw(dst: *mut u8, dst_len: usize, src: &[u8], algorithm: Algorithm) -> usize {
    unsafe {
        ffi::compression_decode::compression_rs_compression_decode_buffer(
            dst,
            dst_len,
            src.as_ptr(),
            src.len(),
            algorithm.as_raw(),
        )
    }
}

fn is_empty_encoding(src: &[u8], algorithm: Algorithm) -> bool {
    let mut encoded = [0_u8; 64];
    let written = encode_len(&mut encoded, &[], algorithm);
    written > 0 && encoded.get(..written) == Some(src)
}

const fn buffer_failed(
    algorithm: Algorithm,
    input_len: usize,
    output_capacity: usize,
) -> CompressionError {
    CompressionError::BufferOperationFailed {
        operation: OPERATION,
        algorithm,
        input_len,
        output_capacity,
    }
}

const fn output_limit_exceeded(algorithm: Algorithm, max_output_len: usize) -> CompressionError {
    CompressionError::OutputLimitExceeded {
        operation: OPERATION,
        algorithm,
        max_output_len,
    }
}

fn stream_decode_into(dst: &mut [u8], src: &[u8], algorithm: Algorithm) -> Result<usize> {
    let mut stream = CompressionStream::new(StreamOperation::Decode, algorithm)?;
    let mut input = src;
    let mut written = 0;
    let mut probe = [0_u8; 1];
    loop {
        let full = written == dst.len();
        let window: &mut [u8] = if full {
            &mut probe
        } else {
            &mut dst[written..]
        };
        let (status, consumed, produced) = stream.step(
            input,
            window,
            ffi::compression_stream::COMPRESSION_STREAM_FINALIZE,
        );
        if full && produced > 0 {
            return Err(output_limit_exceeded(algorithm, dst.len()));
        }
        written += produced;
        input = &input[consumed..];
        match status {
            ffi::compression_stream::COMPRESSION_STATUS_END => return Ok(written),
            ffi::compression_stream::COMPRESSION_STATUS_OK if consumed == 0 && produced == 0 => {
                return Err(CompressionError::StreamStalled {
                    operation: OPERATION,
                    algorithm,
                });
            }
            ffi::compression_stream::COMPRESSION_STATUS_OK => {}
            _ => return Err(buffer_failed(algorithm, src.len(), dst.len())),
        }
    }
}

/// Wraps `compression_decode_buffer`.
pub fn compression_decode_scratch_buffer_size(algorithm: Algorithm) -> usize {
    unsafe {
        ffi::compression_decode::compression_rs_compression_decode_scratch_buffer_size(
            algorithm.as_raw(),
        )
    }
}

/// Wraps `compression_decode_buffer`.
pub fn compression_decode_buffer(
    dst: &mut [u8],
    src: &[u8],
    algorithm: Algorithm,
) -> Result<usize> {
    if src.is_empty() {
        return Ok(0);
    }
    if algorithm.supports_streams() {
        return stream_decode_into(dst, src, algorithm);
    }
    let written = unsafe { decode_raw(dst.as_mut_ptr(), dst.len(), src, algorithm) };
    if written == 0 {
        return if is_empty_encoding(src, algorithm) {
            Ok(0)
        } else {
            Err(buffer_failed(algorithm, src.len(), dst.len()))
        };
    }
    if written < dst.len() {
        return Ok(written);
    }
    let probe_len = dst.len().saturating_add(1);
    let mut probe = Vec::<u8>::new();
    probe
        .try_reserve_exact(probe_len)
        .map_err(|_| buffer_failed(algorithm, src.len(), probe_len))?;
    let probe_written = unsafe { decode_raw(probe.as_mut_ptr(), probe_len, src, algorithm) };
    if probe_written > dst.len() {
        Err(output_limit_exceeded(algorithm, dst.len()))
    } else {
        Ok(written)
    }
}

/// Wraps `compression_decode_buffer`.
pub fn decompress(input: &[u8], algorithm: Algorithm) -> Result<Vec<u8>> {
    decompress_with_max_output_len(input, algorithm, DEFAULT_MAX_OUTPUT_LEN)
}

pub fn decompress_with_max_output_len(
    input: &[u8],
    algorithm: Algorithm,
    max_output_len: usize,
) -> Result<Vec<u8>> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    if algorithm.supports_streams() {
        let mut decoder = Decoder::with_max_output_len(algorithm, max_output_len)?;
        let mut output = decoder.process(input)?;
        output.extend(decoder.finish()?);
        return Ok(output);
    }

    let capacity = max_output_len.saturating_add(1);
    let mut output = Vec::<u8>::new();
    output
        .try_reserve_exact(capacity)
        .map_err(|_| buffer_failed(algorithm, input.len(), capacity))?;
    let written = unsafe { decode_raw(output.as_mut_ptr(), capacity, input, algorithm) };
    if written == 0 {
        return if is_empty_encoding(input, algorithm) {
            Ok(Vec::new())
        } else {
            Err(buffer_failed(algorithm, input.len(), capacity))
        };
    }
    if written > max_output_len {
        return Err(output_limit_exceeded(algorithm, max_output_len));
    }
    unsafe { output.set_len(written) };
    output.shrink_to_fit();
    Ok(output)
}
