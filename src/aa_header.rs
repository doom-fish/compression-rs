use crate::{
    aa_byte_stream::ArchiveFlags,
    aa_field_key::{FieldKey, FieldKeySet},
    ffi, util, CompressionError, Result,
};
use std::ffi::{c_void, CStr};
use std::ptr::{null_mut, NonNull};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HashFunction {
    Crc32 = 1,
    Sha1 = 2,
    Sha256 = 3,
    Sha384 = 4,
    Sha512 = 5,
}

impl HashFunction {
    pub const fn digest_len(self) -> usize {
        match self {
            Self::Crc32 => 4,
            Self::Sha1 => 20,
            Self::Sha256 => 32,
            Self::Sha384 => 48,
            Self::Sha512 => 64,
        }
    }

    const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            1 => Some(Self::Crc32),
            2 => Some(Self::Sha1),
            3 => Some(Self::Sha256),
            4 => Some(Self::Sha384),
            5 => Some(Self::Sha512),
            _ => None,
        }
    }

    const fn as_raw(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EntryType {
    RegularFile = 'F' as isize,
    Directory = 'D' as isize,
    SymbolicLink = 'L' as isize,
    Fifo = 'P' as isize,
    CharacterDevice = 'C' as isize,
    BlockDevice = 'B' as isize,
    Socket = 'S' as isize,
    Whiteout = 'W' as isize,
    Door = 'R' as isize,
    Port = 'T' as isize,
    Metadata = 'M' as isize,
}

impl EntryType {
    pub fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            x if x == Self::RegularFile as u32 => Some(Self::RegularFile),
            x if x == Self::Directory as u32 => Some(Self::Directory),
            x if x == Self::SymbolicLink as u32 => Some(Self::SymbolicLink),
            x if x == Self::Fifo as u32 => Some(Self::Fifo),
            x if x == Self::CharacterDevice as u32 => Some(Self::CharacterDevice),
            x if x == Self::BlockDevice as u32 => Some(Self::BlockDevice),
            x if x == Self::Socket as u32 => Some(Self::Socket),
            x if x == Self::Whiteout as u32 => Some(Self::Whiteout),
            x if x == Self::Door as u32 => Some(Self::Door),
            x if x == Self::Port as u32 => Some(Self::Port),
            x if x == Self::Metadata as u32 => Some(Self::Metadata),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum FieldType {
    Flag = 0,
    UInt = 1,
    String = 2,
    Hash = 3,
    Timespec = 4,
    Blob = 5,
}

impl FieldType {
    fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            0 => Some(Self::Flag),
            1 => Some(Self::UInt),
            2 => Some(Self::String),
            3 => Some(Self::Hash),
            4 => Some(Self::Timespec),
            5 => Some(Self::Blob),
            _ => None,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Timespec {
    pub seconds: i64,
    pub nanoseconds: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HashValue {
    pub function: HashFunction,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct BlobDescription {
    pub size: u64,
    pub offset: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HeaderFieldValue {
    Flag,
    UInt(u64),
    String(String),
    Hash(HashValue),
    Timespec(Timespec),
    Blob(BlobDescription),
}

#[derive(Debug)]
pub struct Header {
    handle: NonNull<c_void>,
}

impl Header {
    pub(crate) fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        Ok(Self {
            handle: util::nonnull_handle(handle, operation)?,
        })
    }

    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aa_header::compression_rs_aa_header_create() };
        Self::from_handle(handle, "AAHeaderCreate")
    }

    pub fn from_encoded_data(data: &[u8]) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_header::compression_rs_aa_header_create_with_encoded_data(
                data.len(),
                data.as_ptr(),
            )
        };
        Self::from_handle(handle, "AAHeaderCreateWithEncodedData")
    }

    pub fn from_path(
        key_set: &FieldKeySet,
        dir: &str,
        path: &str,
        flags: ArchiveFlags,
    ) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let path = util::cstring("path", path)?;
        let handle = unsafe {
            ffi::aa_header::compression_rs_aa_header_create_with_path(
                key_set.as_ptr(),
                dir.as_ptr(),
                path.as_ptr(),
                flags.bits(),
            )
        };
        Self::from_handle(handle, "AAHeaderCreateWithPath")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn assign(&mut self, other: &Self) -> Result<()> {
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_assign(self.as_ptr(), other.as_ptr())
        };
        util::status_result("AAHeaderAssign", status)
    }

    pub fn field_count(&self) -> u32 {
        unsafe { ffi::aa_header::compression_rs_aa_header_get_field_count(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.field_count() == 0
    }

    pub fn key_index(&self, key: FieldKey) -> Result<Option<u32>> {
        match unsafe {
            ffi::aa_header::compression_rs_aa_header_get_key_index(self.as_ptr(), key.raw())
        } {
            -1 => Ok(None),
            value if value < -1 => Err(CompressionError::OperationFailed {
                operation: "AAHeaderGetKeyIndex",
                code: value,
            }),
            value => Ok(Some(value.unsigned_abs())),
        }
    }

    pub fn field_type(&self, index: u32) -> Result<FieldType> {
        let raw = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_type(self.as_ptr(), index)
        };
        if raw < 0 {
            return Err(CompressionError::OperationFailed {
                operation: "AAHeaderGetFieldType",
                code: raw,
            });
        }
        FieldType::from_raw(raw).ok_or(CompressionError::OperationFailed {
            operation: "AAHeaderGetFieldType",
            code: raw,
        })
    }

    pub fn field_key(&self, index: u32) -> Result<FieldKey> {
        if index >= self.field_count() {
            return Err(CompressionError::OperationFailed {
                operation: "AAHeaderGetFieldKey",
                code: -1,
            });
        }

        Ok(FieldKey::from_raw(unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_key(self.as_ptr(), index)
        }))
    }

    pub fn payload_size(&self) -> u64 {
        unsafe { ffi::aa_header::compression_rs_aa_header_get_payload_size(self.as_ptr()) }
    }

    pub fn remove_field(&mut self, index: u32) -> Result<()> {
        let status =
            unsafe { ffi::aa_header::compression_rs_aa_header_remove_field(self.as_ptr(), index) };
        util::status_result("AAHeaderRemoveField", status)
    }

    pub fn clear(&mut self) -> Result<()> {
        let status = unsafe { ffi::aa_header::compression_rs_aa_header_clear(self.as_ptr()) };
        util::status_result("AAHeaderClear", status)
    }

    pub fn set_field_flag(&mut self, index: u32, key: FieldKey) -> Result<()> {
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_set_field_flag(self.as_ptr(), index, key.raw())
        };
        util::status_result("AAHeaderSetFieldFlag", status)
    }

    pub fn set_field_uint(&mut self, index: u32, key: FieldKey, value: u64) -> Result<()> {
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_set_field_uint(
                self.as_ptr(),
                index,
                key.raw(),
                value,
            )
        };
        util::status_result("AAHeaderSetFieldUInt", status)
    }

    pub fn set_field_string(&mut self, index: u32, key: FieldKey, value: &str) -> Result<()> {
        let value = util::cstring("value", value)?;
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_set_field_string(
                self.as_ptr(),
                index,
                key.raw(),
                value.as_ptr(),
                value.as_bytes().len(),
            )
        };
        util::status_result("AAHeaderSetFieldString", status)
    }

    pub fn set_field_hash(
        &mut self,
        index: u32,
        key: FieldKey,
        function: HashFunction,
        value: &[u8],
    ) -> Result<()> {
        if value.len() != function.digest_len() {
            return Err(CompressionError::InvalidHashLength {
                expected: function.digest_len(),
                actual: value.len(),
            });
        }
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_set_field_hash(
                self.as_ptr(),
                index,
                key.raw(),
                function.as_raw(),
                value.as_ptr(),
            )
        };
        util::status_result("AAHeaderSetFieldHash", status)
    }

    pub fn set_field_timespec(&mut self, index: u32, key: FieldKey, value: Timespec) -> Result<()> {
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_set_field_timespec(
                self.as_ptr(),
                index,
                key.raw(),
                value.seconds,
                value.nanoseconds,
            )
        };
        util::status_result("AAHeaderSetFieldTimespec", status)
    }

    pub fn set_field_blob(&mut self, index: u32, key: FieldKey, size: u64) -> Result<()> {
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_set_field_blob(
                self.as_ptr(),
                index,
                key.raw(),
                size,
            )
        };
        util::status_result("AAHeaderSetFieldBlob", status)
    }

    pub fn append_field_flag(&mut self, key: FieldKey) -> Result<()> {
        self.set_field_flag(u32::MAX, key)
    }

    pub fn append_field_uint(&mut self, key: FieldKey, value: u64) -> Result<()> {
        self.set_field_uint(u32::MAX, key, value)
    }

    pub fn append_field_string(&mut self, key: FieldKey, value: &str) -> Result<()> {
        self.set_field_string(u32::MAX, key, value)
    }

    pub fn append_field_hash(
        &mut self,
        key: FieldKey,
        function: HashFunction,
        value: &[u8],
    ) -> Result<()> {
        self.set_field_hash(u32::MAX, key, function, value)
    }

    pub fn append_field_timespec(&mut self, key: FieldKey, value: Timespec) -> Result<()> {
        self.set_field_timespec(u32::MAX, key, value)
    }

    pub fn append_field_blob(&mut self, key: FieldKey, size: u64) -> Result<()> {
        self.set_field_blob(u32::MAX, key, size)
    }

    pub fn field_uint(&self, index: u32) -> Result<u64> {
        let mut value = 0_u64;
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_uint(
                self.as_ptr(),
                index,
                &mut value,
            )
        };
        util::status_result("AAHeaderGetFieldUInt", status)?;
        Ok(value)
    }

    pub fn field_string(&self, index: u32) -> Result<String> {
        let mut length = 0_usize;
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_string(
                self.as_ptr(),
                index,
                0,
                null_mut(),
                &mut length,
            )
        };
        util::status_result("AAHeaderGetFieldString", status)?;

        let mut buffer = vec![0_i8; length.saturating_add(1)];
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_string(
                self.as_ptr(),
                index,
                buffer.len(),
                buffer.as_mut_ptr(),
                &mut length,
            )
        };
        util::status_result("AAHeaderGetFieldString", status)?;

        let value = unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_str()
            .map_err(|_| CompressionError::Utf8Error {
                operation: "AAHeaderGetFieldString",
            })?;
        Ok(value.to_string())
    }

    pub fn field_hash(&self, index: u32) -> Result<HashValue> {
        let mut function = 0_u32;
        let mut bytes = vec![0_u8; HashFunction::Sha512.digest_len()];
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_hash(
                self.as_ptr(),
                index,
                bytes.len(),
                &mut function,
                bytes.as_mut_ptr(),
            )
        };
        util::status_result("AAHeaderGetFieldHash", status)?;
        let function =
            HashFunction::from_raw(function).ok_or(CompressionError::OperationFailed {
                operation: "AAHeaderGetFieldHash",
                code: -1,
            })?;
        bytes.truncate(function.digest_len());
        Ok(HashValue { function, bytes })
    }

    pub fn field_timespec(&self, index: u32) -> Result<Timespec> {
        let mut seconds = 0_i64;
        let mut nanoseconds = 0_i64;
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_timespec(
                self.as_ptr(),
                index,
                &mut seconds,
                &mut nanoseconds,
            )
        };
        util::status_result("AAHeaderGetFieldTimespec", status)?;
        Ok(Timespec {
            seconds,
            nanoseconds,
        })
    }

    pub fn field_blob(&self, index: u32) -> Result<BlobDescription> {
        let mut size = 0_u64;
        let mut offset = 0_u64;
        let status = unsafe {
            ffi::aa_header::compression_rs_aa_header_get_field_blob(
                self.as_ptr(),
                index,
                &mut size,
                &mut offset,
            )
        };
        util::status_result("AAHeaderGetFieldBlob", status)?;
        Ok(BlobDescription { size, offset })
    }

    pub fn field_value(&self, index: u32) -> Result<HeaderFieldValue> {
        match self.field_type(index)? {
            FieldType::Flag => Ok(HeaderFieldValue::Flag),
            FieldType::UInt => self.field_uint(index).map(HeaderFieldValue::UInt),
            FieldType::String => self.field_string(index).map(HeaderFieldValue::String),
            FieldType::Hash => self.field_hash(index).map(HeaderFieldValue::Hash),
            FieldType::Timespec => self.field_timespec(index).map(HeaderFieldValue::Timespec),
            FieldType::Blob => self.field_blob(index).map(HeaderFieldValue::Blob),
        }
    }

    pub fn uint_with_key(&self, key: FieldKey) -> Result<Option<u64>> {
        self.key_index(key)?
            .map(|index| self.field_uint(index))
            .transpose()
    }

    pub fn string_with_key(&self, key: FieldKey) -> Result<Option<String>> {
        self.key_index(key)?
            .map(|index| self.field_string(index))
            .transpose()
    }

    pub fn hash_with_key(&self, key: FieldKey) -> Result<Option<HashValue>> {
        self.key_index(key)?
            .map(|index| self.field_hash(index))
            .transpose()
    }

    pub fn timespec_with_key(&self, key: FieldKey) -> Result<Option<Timespec>> {
        self.key_index(key)?
            .map(|index| self.field_timespec(index))
            .transpose()
    }

    pub fn blob_with_key(&self, key: FieldKey) -> Result<Option<BlobDescription>> {
        self.key_index(key)?
            .map(|index| self.field_blob(index))
            .transpose()
    }

    pub fn value_with_key(&self, key: FieldKey) -> Result<Option<HeaderFieldValue>> {
        self.key_index(key)?
            .map(|index| self.field_value(index))
            .transpose()
    }

    pub fn entry_type(&self) -> Result<Option<EntryType>> {
        Ok(self
            .uint_with_key(FieldKey::TYP)?
            .and_then(|raw| u32::try_from(raw).ok())
            .and_then(EntryType::from_raw))
    }

    pub fn path(&self) -> Result<Option<String>> {
        self.string_with_key(FieldKey::PAT)
    }

    pub fn encoded_size(&self) -> usize {
        unsafe { ffi::aa_header::compression_rs_aa_header_get_encoded_size(self.as_ptr()) }
    }

    pub fn encoded_data(&self) -> Result<Vec<u8>> {
        let size = self.encoded_size();
        let mut data = vec![0_u8; size];
        if size == 0 {
            return Ok(data);
        }
        let copied = unsafe {
            ffi::aa_header::compression_rs_aa_header_copy_encoded_data(
                self.as_ptr(),
                data.as_mut_ptr(),
            )
        };
        if copied {
            Ok(data)
        } else {
            Err(CompressionError::OperationFailed {
                operation: "AAHeaderGetEncodedData",
                code: -1,
            })
        }
    }
}

impl Clone for Header {
    fn clone(&self) -> Self {
        let handle = unsafe { ffi::aa_header::compression_rs_aa_header_clone(self.as_ptr()) };
        Self::from_handle(handle, "AAHeaderClone").expect("AAHeaderClone returned null")
    }
}

impl Drop for Header {
    fn drop(&mut self) {
        unsafe { ffi::aa_header::compression_rs_aa_header_release(self.as_ptr()) };
    }
}

impl Header {
    pub(crate) fn from_raw_clone(raw: *mut c_void, operation: &'static str) -> Result<Self> {
        let handle = unsafe { ffi::aa_header::compression_rs_aa_header_clone_from_raw(raw) };
        Self::from_handle(handle, operation)
    }

    pub(crate) fn clone_raw(&self) -> Result<*mut c_void> {
        let raw = unsafe { ffi::aa_header::compression_rs_aa_header_clone_raw(self.as_ptr()) };
        if raw.is_null() {
            Err(CompressionError::NullHandle {
                operation: "AAHeaderClone",
            })
        } else {
            Ok(raw)
        }
    }
}
