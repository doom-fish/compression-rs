use crate::{ffi, util, Algorithm, CompressionError, Result};
use std::ffi::c_void;
use std::ptr::NonNull;

const OUTPUT_CHUNK_LEN: usize = 32 * 1024;

pub const DEFAULT_MAX_OUTPUT_LEN: usize = 64 * 1024 * 1024;

/// Wraps `compression_stream_operation` values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamOperation {
    /// Wraps `COMPRESSION_STREAM_ENCODE`.
    Encode,
    /// Wraps `COMPRESSION_STREAM_DECODE`.
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

/// Wraps a `compression_stream` handle.
#[derive(Debug)]
pub struct CompressionStream {
    handle: NonNull<c_void>,
    algorithm: Algorithm,
    operation: StreamOperation,
    finished: bool,
    max_output_len: usize,
    output_len: usize,
}

impl CompressionStream {
    /// Wraps `compression_stream_create`.
    pub fn new(operation: StreamOperation, algorithm: Algorithm) -> Result<Self> {
        let max_output_len = match operation {
            StreamOperation::Encode => usize::MAX,
            StreamOperation::Decode => DEFAULT_MAX_OUTPUT_LEN,
        };
        Self::with_max_output_len(operation, algorithm, max_output_len)
    }

    pub fn with_max_output_len(
        operation: StreamOperation,
        algorithm: Algorithm,
        max_output_len: usize,
    ) -> Result<Self> {
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
            max_output_len,
            output_len: 0,
        })
    }

    fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub(crate) fn step(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        flags: i32,
    ) -> (i32, usize, usize) {
        let mut src_remaining = input.len();
        let mut dst_remaining = output.len();
        let status = unsafe {
            ffi::compression_stream::compression_rs_compression_stream_process(
                self.as_ptr(),
                input.as_ptr(),
                input.len(),
                output.as_mut_ptr(),
                output.len(),
                flags,
                &raw mut src_remaining,
                &raw mut dst_remaining,
            )
        };
        (
            status,
            input.len().saturating_sub(src_remaining),
            output.len().saturating_sub(dst_remaining),
        )
    }

    /// Wraps `compression_stream_process`.
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
        let mut buffer = vec![0_u8; OUTPUT_CHUNK_LEN];

        loop {
            let allowance = self.max_output_len.saturating_sub(self.output_len);
            let window = OUTPUT_CHUNK_LEN.min(allowance.saturating_add(1));
            let (status, consumed, produced) = self.step(input, &mut buffer[..window], flags);
            if produced > allowance {
                self.finished = true;
                return Err(CompressionError::OutputLimitExceeded {
                    operation: self.operation.operation_name(),
                    algorithm: self.algorithm,
                    max_output_len: self.max_output_len,
                });
            }
            self.output_len += produced;
            output.extend_from_slice(&buffer[..produced]);
            input = &input[consumed..];

            match status {
                ffi::compression_stream::COMPRESSION_STATUS_END => {
                    self.finished = true;
                    return Ok(output);
                }
                ffi::compression_stream::COMPRESSION_STATUS_OK => {
                    if !finalize && input.is_empty() {
                        return Ok(output);
                    }
                    if consumed == 0 && produced == 0 {
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

    /// Wraps `compression_stream_release`.
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

/// Wraps `compression_stream_process` for encoding.
#[derive(Debug)]
pub struct Encoder {
    inner: CompressionStream,
}

impl Encoder {
    /// Wraps `new` convenience around `compression_stream_process`.
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        Ok(Self {
            inner: CompressionStream::new(StreamOperation::Encode, algorithm)?,
        })
    }

    /// Wraps `process` convenience around `compression_stream_process`.
    pub fn process(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        self.inner.process(input, false)
    }

    /// Wraps `finish` convenience around `compression_stream_process`.
    pub fn finish(&mut self) -> Result<Vec<u8>> {
        self.inner.finish()
    }
}

/// Wraps `compression_stream_process` for decoding.
#[derive(Debug)]
pub struct Decoder {
    inner: CompressionStream,
}

impl Decoder {
    /// Wraps `new` convenience around `compression_stream_process`.
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        Self::with_max_output_len(algorithm, DEFAULT_MAX_OUTPUT_LEN)
    }

    pub fn with_max_output_len(algorithm: Algorithm, max_output_len: usize) -> Result<Self> {
        Ok(Self {
            inner: CompressionStream::with_max_output_len(
                StreamOperation::Decode,
                algorithm,
                max_output_len,
            )?,
        })
    }

    /// Wraps `process` convenience around `compression_stream_process`.
    pub fn process(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        self.inner.process(input, false)
    }

    /// Wraps `finish` convenience around `compression_stream_process`.
    pub fn finish(&mut self) -> Result<Vec<u8>> {
        self.inner.finish()
    }
}
