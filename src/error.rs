use crate::Algorithm;
use std::{error::Error, fmt};

/// Wraps crate results returned by Compression and AppleArchive helpers.
pub type Result<T> = std::result::Result<T, CompressionError>;

/// Wraps failures returned by Compression and AppleArchive helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompressionError {
    /// Wraps the `BufferOperationFailed` failure case reported by Compression or AppleArchive helpers.
    BufferOperationFailed {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
        /// Wraps the algorithm associated with the failing Compression call.
        algorithm: Algorithm,
        /// Wraps the input length passed to the failing buffer operation.
        input_len: usize,
        /// Wraps the destination capacity passed to the failing buffer operation.
        output_capacity: usize,
    },
    /// Wraps the `StreamInitFailed` failure case reported by Compression or AppleArchive helpers.
    StreamInitFailed {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
        /// Wraps the algorithm associated with the failing Compression call.
        algorithm: Algorithm,
    },
    /// Wraps the `StreamProcessFailed` failure case reported by Compression or AppleArchive helpers.
    StreamProcessFailed {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
        /// Wraps the algorithm associated with the failing Compression call.
        algorithm: Algorithm,
        /// Wraps the `compression_stream_process` status code.
        status: i32,
    },
    /// Wraps the `StreamStalled` failure case reported by Compression or AppleArchive helpers.
    StreamStalled {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
        /// Wraps the algorithm associated with the failing Compression call.
        algorithm: Algorithm,
    },
    /// Wraps the `StreamFinished` failure case reported by Compression or AppleArchive helpers.
    StreamFinished {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
        /// Wraps the algorithm associated with the failing Compression call.
        algorithm: Algorithm,
    },
    /// Wraps the `OperationFailed` failure case reported by Compression or AppleArchive helpers.
    OperationFailed {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
        /// Wraps the failing Compression or AppleArchive status code.
        code: i32,
    },
    /// Wraps the `NullHandle` failure case reported by Compression or AppleArchive helpers.
    NullHandle {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
    },
    /// Wraps the `Closed` failure case reported by Compression or AppleArchive helpers.
    Closed {
        /// Wraps the closed resource name reported by AppleArchive helpers.
        resource: &'static str,
    },
    /// Wraps the `InvalidFieldKey` failure case reported by Compression or AppleArchive helpers.
    InvalidFieldKey {
        /// Wraps the invalid AppleArchive field key.
        key: String,
    },
    /// Wraps the `InvalidHashLength` failure case reported by Compression or AppleArchive helpers.
    InvalidHashLength {
        /// Wraps the expected hash length.
        expected: usize,
        /// Wraps the actual hash length.
        actual: usize,
    },
    /// Wraps the `NulByte` failure case reported by Compression or AppleArchive helpers.
    NulByte {
        /// Wraps the argument name that contained an interior NUL byte.
        argument: &'static str,
    },
    /// Wraps the `Utf8Error` failure case reported by Compression or AppleArchive helpers.
    Utf8Error {
        /// Wraps the failing Compression or AppleArchive entry-point name.
        operation: &'static str,
    },
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BufferOperationFailed {
                operation,
                algorithm,
                input_len,
                output_capacity,
            } => write!(
                f,
                "{operation} failed for {algorithm:?} (input_len={input_len}, output_capacity={output_capacity})"
            ),
            Self::StreamInitFailed {
                operation,
                algorithm,
            } => write!(f, "{operation} failed for {algorithm:?}"),
            Self::StreamProcessFailed {
                operation,
                algorithm,
                status,
            } => write!(
                f,
                "{operation} failed for {algorithm:?} (status={status})"
            ),
            Self::StreamStalled {
                operation,
                algorithm,
            } => write!(
                f,
                "{operation} made no progress for {algorithm:?}; the input may be truncated or corrupt"
            ),
            Self::StreamFinished {
                operation,
                algorithm,
            } => write!(f, "{operation} called after the {algorithm:?} stream already finished"),
            Self::OperationFailed { operation, code } => {
                if *code < 0 {
                    let os_code = -*code;
                    write!(
                        f,
                        "{operation} failed with {code} ({})",
                        std::io::Error::from_raw_os_error(os_code)
                    )
                } else {
                    write!(f, "{operation} failed with {code}")
                }
            }
            Self::NullHandle { operation } => write!(f, "{operation} returned a null handle"),
            Self::Closed { resource } => write!(f, "{resource} is already closed"),
            Self::InvalidFieldKey { key } => write!(f, "invalid AppleArchive field key: {key:?}"),
            Self::InvalidHashLength { expected, actual } => {
                write!(f, "invalid hash length: expected {expected} bytes, got {actual}")
            }
            Self::NulByte { argument } => write!(f, "{argument} contains an interior NUL byte"),
            Self::Utf8Error { operation } => write!(f, "{operation} returned data that is not valid UTF-8"),
        }
    }
}

impl Error for CompressionError {}
