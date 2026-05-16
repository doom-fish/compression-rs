use crate::Algorithm;
use std::{error::Error, fmt};

pub type Result<T> = std::result::Result<T, CompressionError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompressionError {
    BufferOperationFailed {
        operation: &'static str,
        algorithm: Algorithm,
        input_len: usize,
        output_capacity: usize,
    },
    StreamInitFailed {
        operation: &'static str,
        algorithm: Algorithm,
    },
    StreamProcessFailed {
        operation: &'static str,
        algorithm: Algorithm,
        status: i32,
    },
    StreamStalled {
        operation: &'static str,
        algorithm: Algorithm,
    },
    StreamFinished {
        operation: &'static str,
        algorithm: Algorithm,
    },
    OperationFailed {
        operation: &'static str,
        code: i32,
    },
    NullHandle {
        operation: &'static str,
    },
    Closed {
        resource: &'static str,
    },
    InvalidFieldKey {
        key: String,
    },
    InvalidHashLength {
        expected: usize,
        actual: usize,
    },
    NulByte {
        argument: &'static str,
    },
    Utf8Error {
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
