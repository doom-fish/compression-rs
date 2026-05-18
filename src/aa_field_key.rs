use crate::{ffi, util, CompressionError, Result};
use std::ffi::{c_void, CStr};
use std::fmt;
use std::ptr::NonNull;

/// Wraps an `AAFieldKey` value.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldKey(u32);

fn parse_field_key(value: &str) -> Result<FieldKey> {
    if value.len() != 3 || !value.bytes().all(|byte| byte.is_ascii() && byte != 0) {
        return Err(CompressionError::InvalidFieldKey {
            key: value.to_string(),
        });
    }

    let upper = value.to_ascii_uppercase();
    let bytes = upper.as_bytes();
    Ok(FieldKey::from_bytes([bytes[0], bytes[1], bytes[2]]))
}

impl FieldKey {
    /// Wraps the `ACL` AppleArchive field key.
    pub const ACL: Self = Self::from_bytes(*b"ACL");
    /// Wraps the `BTM` AppleArchive field key.
    pub const BTM: Self = Self::from_bytes(*b"BTM");
    /// Wraps the `CKS` AppleArchive field key.
    pub const CKS: Self = Self::from_bytes(*b"CKS");
    /// Wraps the `CLC` AppleArchive field key.
    pub const CLC: Self = Self::from_bytes(*b"CLC");
    /// Wraps the `CTM` AppleArchive field key.
    pub const CTM: Self = Self::from_bytes(*b"CTM");
    /// Wraps the `DAT` AppleArchive field key.
    pub const DAT: Self = Self::from_bytes(*b"DAT");
    /// Wraps the `DEV` AppleArchive field key.
    pub const DEV: Self = Self::from_bytes(*b"DEV");
    /// Wraps the `DE2` AppleArchive field key.
    pub const DE2: Self = Self::from_bytes(*b"DE2");
    /// Wraps the `DUZ` AppleArchive field key.
    pub const DUZ: Self = Self::from_bytes(*b"DUZ");
    /// Wraps the `FLG` AppleArchive field key.
    pub const FLG: Self = Self::from_bytes(*b"FLG");
    /// Wraps the `GID` AppleArchive field key.
    pub const GID: Self = Self::from_bytes(*b"GID");
    /// Wraps the `GIN` AppleArchive field key.
    pub const GIN: Self = Self::from_bytes(*b"GIN");
    /// Wraps the `HLC` AppleArchive field key.
    pub const HLC: Self = Self::from_bytes(*b"HLC");
    /// Wraps the `IDX` AppleArchive field key.
    pub const IDX: Self = Self::from_bytes(*b"IDX");
    /// Wraps the `IDZ` AppleArchive field key.
    pub const IDZ: Self = Self::from_bytes(*b"IDZ");
    /// Wraps the `INO` AppleArchive field key.
    pub const INO: Self = Self::from_bytes(*b"INO");
    /// Wraps the `LNK` AppleArchive field key.
    pub const LNK: Self = Self::from_bytes(*b"LNK");
    /// Wraps the `MOD` AppleArchive field key.
    pub const MOD: Self = Self::from_bytes(*b"MOD");
    /// Wraps the `MTM` AppleArchive field key.
    pub const MTM: Self = Self::from_bytes(*b"MTM");
    /// Wraps the `NLK` AppleArchive field key.
    pub const NLK: Self = Self::from_bytes(*b"NLK");
    /// Wraps the `PAT` AppleArchive field key.
    pub const PAT: Self = Self::from_bytes(*b"PAT");
    /// Wraps the `SH1` AppleArchive field key.
    pub const SH1: Self = Self::from_bytes(*b"SH1");
    /// Wraps the `SH2` AppleArchive field key.
    pub const SH2: Self = Self::from_bytes(*b"SH2");
    /// Wraps the `SH3` AppleArchive field key.
    pub const SH3: Self = Self::from_bytes(*b"SH3");
    /// Wraps the `SH5` AppleArchive field key.
    pub const SH5: Self = Self::from_bytes(*b"SH5");
    /// Wraps the `SIZ` AppleArchive field key.
    pub const SIZ: Self = Self::from_bytes(*b"SIZ");
    /// Wraps the `SLC` AppleArchive field key.
    pub const SLC: Self = Self::from_bytes(*b"SLC");
    /// Wraps the `TYP` AppleArchive field key.
    pub const TYP: Self = Self::from_bytes(*b"TYP");
    /// Wraps the `UID` AppleArchive field key.
    pub const UID: Self = Self::from_bytes(*b"UID");
    /// Wraps the `UIN` AppleArchive field key.
    pub const UIN: Self = Self::from_bytes(*b"UIN");
    /// Wraps the `XAT` AppleArchive field key.
    pub const XAT: Self = Self::from_bytes(*b"XAT");
    /// Wraps the `YAF` AppleArchive field key.
    pub const YAF: Self = Self::from_bytes(*b"YAF");

    /// Wraps three-byte `AAFieldKey` construction.
    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    /// Wraps string parsing for `AAFieldKey` values.
    pub fn parse(value: &str) -> Result<Self> {
        parse_field_key(value)
    }

    pub(crate) const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Wraps raw `AAFieldKey` values.
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Wraps three-byte `AAFieldKey` values.
    pub const fn as_bytes(self) -> [u8; 3] {
        let [a, b, c, _] = self.0.to_le_bytes();
        [a, b, c]
    }

    /// Wraps string conversion for `AAFieldKey` values.
    pub fn as_string(self) -> String {
        String::from_utf8_lossy(&self.as_bytes()).into_owned()
    }
}

impl fmt::Debug for FieldKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldKey({self})")
    }
}

impl std::str::FromStr for FieldKey {
    type Err = CompressionError;

    fn from_str(value: &str) -> Result<Self> {
        parse_field_key(value)
    }
}

impl fmt::Display for FieldKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_string())
    }
}

impl TryFrom<&str> for FieldKey {
    type Error = CompressionError;

    fn try_from(value: &str) -> Result<Self> {
        parse_field_key(value)
    }
}

/// Wraps an `AAFieldKeySet` handle.
#[derive(Debug)]
pub struct FieldKeySet {
    handle: NonNull<c_void>,
}

impl FieldKeySet {
    /// Wraps `AAFieldKeySetCreate`.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_create() };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAFieldKeySetCreate")?,
        })
    }

    /// Wraps `AAFieldKeySetCreateWithString`.
    pub fn from_csv(value: &str) -> Result<Self> {
        let value = util::cstring("value", value)?;
        let handle = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_create_with_string(value.as_ptr())
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAFieldKeySetCreateWithString")?,
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Wraps `AAFieldKeySetClear`.
    pub fn clear(&mut self) -> Result<()> {
        let status =
            unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_clear(self.as_ptr()) };
        util::status_result("AAFieldKeySetClear", status)
    }

    /// Wraps `AAFieldKeySetContainsKey`.
    pub fn contains(&self, key: FieldKey) -> Result<bool> {
        match unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_contains_key(
                self.as_ptr(),
                key.raw(),
            )
        } {
            value if value < 0 => Err(CompressionError::OperationFailed {
                operation: "AAFieldKeySetContainsKey",
                code: value,
            }),
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    /// Wraps `AAFieldKeySetInsertKey`.
    pub fn insert(&mut self, key: FieldKey) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_insert_key(self.as_ptr(), key.raw())
        };
        util::status_result("AAFieldKeySetInsertKey", status)
    }

    /// Wraps `AAFieldKeySetRemoveKey`.
    pub fn remove(&mut self, key: FieldKey) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_remove_key(self.as_ptr(), key.raw())
        };
        util::status_result("AAFieldKeySetRemoveKey", status)
    }

    /// Wraps `AAFieldKeySetInsertKeySet`.
    pub fn insert_set(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_insert_key_set(
                self.as_ptr(),
                other.as_ptr(),
            )
        };
        util::status_result("AAFieldKeySetInsertKeySet", status)
    }

    /// Wraps `AAFieldKeySetRemoveKeySet`.
    pub fn remove_set(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_remove_key_set(
                self.as_ptr(),
                other.as_ptr(),
            )
        };
        util::status_result("AAFieldKeySetRemoveKeySet", status)
    }

    /// Wraps `AAFieldKeySetSelectKeySet`.
    pub fn select_set(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_select_key_set(
                self.as_ptr(),
                other.as_ptr(),
            )
        };
        util::status_result("AAFieldKeySetSelectKeySet", status)
    }

    /// Wraps `AAFieldKeySetGetKey`.
    pub fn len(&self) -> u32 {
        unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_get_key_count(self.as_ptr()) }
    }

    /// Wraps `AAFieldKeySetGetKey`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Wraps `AAFieldKeySetGetKey`.
    pub fn key(&self, index: u32) -> Result<FieldKey> {
        if index >= self.len() {
            return Err(CompressionError::OperationFailed {
                operation: "AAFieldKeySetGetKey",
                code: -1,
            });
        }

        Ok(FieldKey::from_raw(unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_get_key(self.as_ptr(), index)
        }))
    }

    /// Wraps `AAFieldKeySetSerialize`.
    pub fn serialize(&self) -> Result<String> {
        let capacity = (self.len() as usize)
            .saturating_mul(4)
            .saturating_add(1)
            .max(1);
        let mut buffer = vec![0_i8; capacity];
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_serialize(
                self.as_ptr(),
                buffer.len(),
                buffer.as_mut_ptr(),
            )
        };
        util::status_result("AAFieldKeySetSerialize", status)?;

        let value = unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_str()
            .map_err(|_| CompressionError::Utf8Error {
                operation: "AAFieldKeySetSerialize",
            })?;
        Ok(value.to_string())
    }
}

impl Clone for FieldKeySet {
    fn clone(&self) -> Self {
        let handle =
            unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_clone(self.as_ptr()) };
        Self {
            handle: util::nonnull_handle(handle, "AAFieldKeySetClone")
                .expect("AAFieldKeySetClone returned null"),
        }
    }
}

impl Drop for FieldKeySet {
    fn drop(&mut self) {
        unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_release(self.as_ptr()) };
    }
}
