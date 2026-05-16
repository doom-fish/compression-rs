use crate::{ffi, util, Algorithm, CompressionError, Result};
use std::ffi::c_void;
use std::ptr::NonNull;

const OUTPUT_CHUNK_LEN: usize = 32 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamOperation {
    Encode,
    Decode,
}

impl StreamOperation {
    const fn operation_name(self) -> &'static str {
        match self {
            Self::Encode => "compression_stream_process(encode)",
            Self::Decode => "compression_stream_process(decode)",
        }
    }

    const fn init_name(self) -> &'static str {
        match self {
            Self::Encode => "compression_stream_init(encode)",
            Self::Decode => "compression_stream_init(decode)",
        }
    }

    const fn as_raw(self) -> i32 {
        match self {
            Self::Encode => 0,
            Self::Decode => 1,
        }
    }
}

#[derive(Debug)]
pub struct CompressionStream {
    handle: NonNull<c_void>,
    algorithm: Algorithm,
    operation: StreamOperation,
    finished: bool,
}

impl CompressionStream {
    pub fn new(operation: StreamOperation, algorithm: Algorithm) -> Result<Self> {
        let handle = unsafe {
            ffi::compression_stream::compression_rs_compression_stream_create(
                operation.as_raw(),
                algorithm.as_raw(),
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, operation.init_name())?,
            algorithm,
            operation,
            finished: false,
        })
    }

    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn process(&mut self, mut input: &[u8], finalize: bool) -> Result<Vec<u8>> {
        if self.finished {
            return Err(CompressionError::StreamFinished {
                operation: self.operation.operation_name(),
                algorithm: self.algorithm,
            });
        }

        let flags = if finalize {
            ffi::compression_stream::COMPRESSION_STREAM_FINALIZE
        } else {
            0
        };
        let mut output = Vec::new();

        loop {
            let mut buffer = vec![0_u8; OUTPUT_CHUNK_LEN];
            let mut src_remaining = input.len();
            let mut dst_remaining = buffer.len();
            let status = unsafe {
                ffi::compression_stream::compression_rs_compression_stream_process(
                    self.as_ptr(),
                    input.as_ptr(),
                    input.len(),
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    flags,
                    &mut src_remaining,
                    &mut dst_remaining,
                )
            };

            let consumed = input.len() - src_remaining;
            let produced = buffer.len() - dst_remaining;
            output.extend_from_slice(&buffer[..produced]);
            input = &input[consumed..];

            match status {
                ffi::compression_stream::COMPRESSION_STATUS_END => {
                    self.finished = true;
                    return Ok(output);
                }
                ffi::compression_stream::COMPRESSION_STATUS_OK => {
                    let input_consumed = src_remaining == 0;
                    let destination_exhausted = dst_remaining == 0;

                    if !finalize && input_consumed {
                        return Ok(output);
                    }

                    if destination_exhausted || !input_consumed {
                        continue;
                    }

                    if produced == 0 {
                        return Err(CompressionError::StreamStalled {
                            operation: self.operation.operation_name(),
                            algorithm: self.algorithm,
                        });
                    }
                }
                other => {
                    return Err(CompressionError::StreamProcessFailed {
                        operation: self.operation.operation_name(),
                        algorithm: self.algorithm,
                        status: other,
                    });
                }
            }
        }
    }

    pub fn finish(&mut self) -> Result<Vec<u8>> {
        if self.finished {
            Ok(Vec::new())
        } else {
            self.process(&[], true)
        }
    }
}

impl Drop for CompressionStream {
    fn drop(&mut self) {
        unsafe {
            ffi::compression_stream::compression_rs_compression_stream_release(self.as_ptr());
        }
    }
}

#[derive(Debug)]
pub struct Encoder {
    inner: CompressionStream,
}

impl Encoder {
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        Ok(Self {
            inner: CompressionStream::new(StreamOperation::Encode, algorithm)?,
        })
    }

    pub fn process(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        self.inner.process(input, false)
    }

    pub fn finish(&mut self) -> Result<Vec<u8>> {
        self.inner.finish()
    }
}

#[derive(Debug)]
pub struct Decoder {
    inner: CompressionStream,
}

impl Decoder {
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        Ok(Self {
            inner: CompressionStream::new(StreamOperation::Decode, algorithm)?,
        })
    }

    pub fn process(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        self.inner.process(input, false)
    }

    pub fn finish(&mut self) -> Result<Vec<u8>> {
        self.inner.finish()
    }
}
