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
    },
    StreamStalled {
        operation: &'static str,
        algorithm: Algorithm,
    },
    StreamFinished {
        operation: &'static str,
        algorithm: Algorithm,
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
            }
            | Self::StreamProcessFailed {
                operation,
                algorithm,
            } => write!(f, "{operation} failed for {algorithm:?}"),
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
        }
    }
}

impl Error for CompressionError {}
