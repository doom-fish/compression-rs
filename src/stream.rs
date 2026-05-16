use crate::{error::Result, ffi, Algorithm, CompressionError};

const OUTPUT_CHUNK_LEN: usize = 32 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StreamKind {
    Encode,
    Decode,
}

impl StreamKind {
    fn operation_name(self) -> &'static str {
        match self {
            Self::Encode => "compression_stream_process(encode)",
            Self::Decode => "compression_stream_process(decode)",
        }
    }

    fn init_name(self) -> &'static str {
        match self {
            Self::Encode => "compression_stream_init(encode)",
            Self::Decode => "compression_stream_init(decode)",
        }
    }

    fn to_ffi(self) -> ffi::compression_stream_operation {
        match self {
            Self::Encode => ffi::COMPRESSION_STREAM_ENCODE,
            Self::Decode => ffi::COMPRESSION_STREAM_DECODE,
        }
    }
}

#[derive(Debug)]
struct CompressionStream {
    raw: ffi::compression_stream,
    algorithm: Algorithm,
    kind: StreamKind,
    finished: bool,
}

impl CompressionStream {
    fn new(kind: StreamKind, algorithm: Algorithm) -> Result<Self> {
        let mut raw = ffi::compression_stream::default();
        let status =
            unsafe { ffi::compression_stream_init(&mut raw, kind.to_ffi(), algorithm.as_ffi()) };
        if status != ffi::COMPRESSION_STATUS_OK {
            return Err(CompressionError::StreamInitFailed {
                operation: kind.init_name(),
                algorithm,
            });
        }

        Ok(Self {
            raw,
            algorithm,
            kind,
            finished: false,
        })
    }

    fn process(&mut self, input: &[u8], finalize: bool) -> Result<Vec<u8>> {
        if self.finished {
            return Err(CompressionError::StreamFinished {
                operation: self.kind.operation_name(),
                algorithm: self.algorithm,
            });
        }

        self.raw.src_ptr = input.as_ptr();
        self.raw.src_size = input.len();
        let flags = if finalize {
            ffi::COMPRESSION_STREAM_FINALIZE
        } else {
            0
        };

        let mut output = Vec::new();

        loop {
            let mut buffer = vec![0_u8; OUTPUT_CHUNK_LEN];
            self.raw.dst_ptr = buffer.as_mut_ptr();
            self.raw.dst_size = buffer.len();

            let status = unsafe { ffi::compression_stream_process(&mut self.raw, flags) };
            let produced = buffer.len() - self.raw.dst_size;
            output.extend_from_slice(&buffer[..produced]);

            match status {
                ffi::COMPRESSION_STATUS_END => {
                    self.finished = true;
                    return Ok(output);
                }
                ffi::COMPRESSION_STATUS_OK => {
                    let input_consumed = self.raw.src_size == 0;
                    let destination_exhausted = self.raw.dst_size == 0;

                    if !finalize && input_consumed {
                        return Ok(output);
                    }

                    if destination_exhausted || !input_consumed {
                        continue;
                    }

                    if produced == 0 {
                        return Err(CompressionError::StreamStalled {
                            operation: self.kind.operation_name(),
                            algorithm: self.algorithm,
                        });
                    }
                }
                _ => {
                    return Err(CompressionError::StreamProcessFailed {
                        operation: self.kind.operation_name(),
                        algorithm: self.algorithm,
                    });
                }
            }
        }
    }
}

impl Drop for CompressionStream {
    fn drop(&mut self) {
        if !self.raw.state.is_null() {
            let _ = unsafe { ffi::compression_stream_destroy(&mut self.raw) };
            self.raw.state = std::ptr::null_mut();
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
            inner: CompressionStream::new(StreamKind::Encode, algorithm)?,
        })
    }

    pub fn process(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        self.inner.process(input, false)
    }

    pub fn finish(&mut self) -> Result<Vec<u8>> {
        self.inner.process(&[], true)
    }
}

#[derive(Debug)]
pub struct Decoder {
    inner: CompressionStream,
}

impl Decoder {
    pub fn new(algorithm: Algorithm) -> Result<Self> {
        Ok(Self {
            inner: CompressionStream::new(StreamKind::Decode, algorithm)?,
        })
    }

    pub fn process(&mut self, input: &[u8]) -> Result<Vec<u8>> {
        self.inner.process(input, false)
    }

    pub fn finish(&mut self) -> Result<Vec<u8>> {
        self.inner.process(&[], true)
    }
}
