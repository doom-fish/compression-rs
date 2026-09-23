use crate::{CompressionError, Result};
use std::ffi::{c_void, CStr, CString};
use std::ptr::NonNull;

fn operation_code(value: i64) -> i32 {
    match i32::try_from(value) {
        Ok(code) => code,
        Err(_) if value < 0 => i32::MIN,
        Err(_) => i32::MAX,
    }
}

pub fn cstring(argument: &'static str, value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| CompressionError::NulByte { argument })
}

pub fn nonnull_handle(handle: *mut c_void, operation: &'static str) -> Result<NonNull<c_void>> {
    NonNull::new(handle).ok_or(CompressionError::NullHandle { operation })
}

pub fn status_result(operation: &'static str, status: i32) -> Result<()> {
    if status < 0 {
        Err(CompressionError::OperationFailed {
            operation,
            code: status,
        })
    } else {
        Ok(())
    }
}

pub fn ssize_result(operation: &'static str, value: i64) -> Result<usize> {
    if value < 0 {
        Err(CompressionError::OperationFailed {
            operation,
            code: operation_code(value),
        })
    } else {
        usize::try_from(value).map_err(|_| CompressionError::OperationFailed {
            operation,
            code: i32::MAX,
        })
    }
}

pub fn off_t_result(operation: &'static str, value: i64) -> Result<u64> {
    if value < 0 {
        Err(CompressionError::OperationFailed {
            operation,
            code: operation_code(value),
        })
    } else {
        Ok(u64::try_from(value).expect("nonnegative i64 always fits in u64"))
    }
}

pub fn string_from_nul_buffer(operation: &'static str, buffer: &[u8]) -> Result<String> {
    CStr::from_bytes_until_nul(buffer)
        .map_err(|_| CompressionError::OperationFailed {
            operation,
            code: -1,
        })?
        .to_str()
        .map(str::to_owned)
        .map_err(|_| CompressionError::Utf8Error { operation })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nul_buffers_are_parsed_within_their_bounds() {
        assert_eq!(
            string_from_nul_buffer("op", b"key\0\0\0"),
            Ok("key".to_string())
        );
        assert_eq!(string_from_nul_buffer("op", b"\0"), Ok(String::new()));
        assert_eq!(
            string_from_nul_buffer("op", b"unterminated"),
            Err(CompressionError::OperationFailed {
                operation: "op",
                code: -1
            })
        );
        assert_eq!(
            string_from_nul_buffer("op", b"\xff\xfe\0"),
            Err(CompressionError::Utf8Error { operation: "op" })
        );
    }
}
