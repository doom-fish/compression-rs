use crate::{ffi, util, CompressionError, Result};
use std::ffi::{c_void, CStr};
use std::fmt;
use std::ptr::NonNull;

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
    pub const ACL: Self = Self::from_bytes(*b"ACL");
    pub const BTM: Self = Self::from_bytes(*b"BTM");
    pub const CKS: Self = Self::from_bytes(*b"CKS");
    pub const CLC: Self = Self::from_bytes(*b"CLC");
    pub const CTM: Self = Self::from_bytes(*b"CTM");
    pub const DAT: Self = Self::from_bytes(*b"DAT");
    pub const DEV: Self = Self::from_bytes(*b"DEV");
    pub const DE2: Self = Self::from_bytes(*b"DE2");
    pub const DUZ: Self = Self::from_bytes(*b"DUZ");
    pub const FLG: Self = Self::from_bytes(*b"FLG");
    pub const GID: Self = Self::from_bytes(*b"GID");
    pub const GIN: Self = Self::from_bytes(*b"GIN");
    pub const HLC: Self = Self::from_bytes(*b"HLC");
    pub const IDX: Self = Self::from_bytes(*b"IDX");
    pub const IDZ: Self = Self::from_bytes(*b"IDZ");
    pub const INO: Self = Self::from_bytes(*b"INO");
    pub const LNK: Self = Self::from_bytes(*b"LNK");
    pub const MOD: Self = Self::from_bytes(*b"MOD");
    pub const MTM: Self = Self::from_bytes(*b"MTM");
    pub const NLK: Self = Self::from_bytes(*b"NLK");
    pub const PAT: Self = Self::from_bytes(*b"PAT");
    pub const SH1: Self = Self::from_bytes(*b"SH1");
    pub const SH2: Self = Self::from_bytes(*b"SH2");
    pub const SH3: Self = Self::from_bytes(*b"SH3");
    pub const SH5: Self = Self::from_bytes(*b"SH5");
    pub const SIZ: Self = Self::from_bytes(*b"SIZ");
    pub const SLC: Self = Self::from_bytes(*b"SLC");
    pub const TYP: Self = Self::from_bytes(*b"TYP");
    pub const UID: Self = Self::from_bytes(*b"UID");
    pub const UIN: Self = Self::from_bytes(*b"UIN");
    pub const XAT: Self = Self::from_bytes(*b"XAT");
    pub const YAF: Self = Self::from_bytes(*b"YAF");

    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        Self(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    pub fn parse(value: &str) -> Result<Self> {
        parse_field_key(value)
    }

    pub(crate) const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn as_bytes(self) -> [u8; 3] {
        let [a, b, c, _] = self.0.to_le_bytes();
        [a, b, c]
    }

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

#[derive(Debug)]
pub struct FieldKeySet {
    handle: NonNull<c_void>,
}

impl FieldKeySet {
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_create() };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAFieldKeySetCreate")?,
        })
    }

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

    pub fn clear(&mut self) -> Result<()> {
        let status =
            unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_clear(self.as_ptr()) };
        util::status_result("AAFieldKeySetClear", status)
    }

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

    pub fn insert(&mut self, key: FieldKey) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_insert_key(self.as_ptr(), key.raw())
        };
        util::status_result("AAFieldKeySetInsertKey", status)
    }

    pub fn remove(&mut self, key: FieldKey) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_remove_key(self.as_ptr(), key.raw())
        };
        util::status_result("AAFieldKeySetRemoveKey", status)
    }

    pub fn insert_set(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_insert_key_set(
                self.as_ptr(),
                other.as_ptr(),
            )
        };
        util::status_result("AAFieldKeySetInsertKeySet", status)
    }

    pub fn remove_set(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_remove_key_set(
                self.as_ptr(),
                other.as_ptr(),
            )
        };
        util::status_result("AAFieldKeySetRemoveKeySet", status)
    }

    pub fn select_set(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_field_key::compression_rs_aa_field_key_set_select_key_set(
                self.as_ptr(),
                other.as_ptr(),
            )
        };
        util::status_result("AAFieldKeySetSelectKeySet", status)
    }

    pub fn len(&self) -> u32 {
        unsafe { ffi::aa_field_key::compression_rs_aa_field_key_set_get_key_count(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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
