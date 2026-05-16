#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::use_self
)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
compile_error!("compression only supports macOS");

mod error;
mod ffi;
mod stream;

pub use error::{CompressionError, Result};
pub use stream::{Decoder, Encoder};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Algorithm {
    Lz4,
    Zlib,
    Lzma,
    Brotli,
    Lzfse,
}

impl Algorithm {
    pub const ALL: [Self; 5] = [Self::Lz4, Self::Zlib, Self::Lzma, Self::Brotli, Self::Lzfse];

    fn as_ffi(self) -> ffi::compression_algorithm {
        match self {
            Self::Lz4 => ffi::COMPRESSION_LZ4,
            Self::Zlib => ffi::COMPRESSION_ZLIB,
            Self::Lzma => ffi::COMPRESSION_LZMA,
            Self::Brotli => ffi::COMPRESSION_BROTLI,
            Self::Lzfse => ffi::COMPRESSION_LZFSE,
        }
    }
}

pub fn compression_encode_scratch_buffer_size(algorithm: Algorithm) -> usize {
    unsafe { ffi::compression_encode_scratch_buffer_size(algorithm.as_ffi()) }
}

pub fn compression_decode_scratch_buffer_size(algorithm: Algorithm) -> usize {
    unsafe { ffi::compression_decode_scratch_buffer_size(algorithm.as_ffi()) }
}

pub fn compression_encode_buffer(
    dst: &mut [u8],
    src: &[u8],
    algorithm: Algorithm,
) -> Result<usize> {
    let written = unsafe {
        ffi::compression_encode_buffer(
            dst.as_mut_ptr(),
            dst.len(),
            src.as_ptr(),
            src.len(),
            std::ptr::null_mut(),
            algorithm.as_ffi(),
        )
    };

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

pub fn compression_decode_buffer(
    dst: &mut [u8],
    src: &[u8],
    algorithm: Algorithm,
) -> Result<usize> {
    let written = unsafe {
        ffi::compression_decode_buffer(
            dst.as_mut_ptr(),
            dst.len(),
            src.as_ptr(),
            src.len(),
            std::ptr::null_mut(),
            algorithm.as_ffi(),
        )
    };

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

pub fn compress(input: &[u8], algorithm: Algorithm) -> Result<Vec<u8>> {
    let mut encoder = Encoder::new(algorithm)?;
    let mut output = encoder.process(input)?;
    output.extend(encoder.finish()?);
    Ok(output)
}

pub fn decompress(input: &[u8], algorithm: Algorithm) -> Result<Vec<u8>> {
    let mut decoder = Decoder::new(algorithm)?;
    let mut output = decoder.process(input)?;
    output.extend(decoder.finish()?);
    Ok(output)
}
