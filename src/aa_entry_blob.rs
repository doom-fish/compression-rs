use crate::{aa_byte_stream::ArchiveFlags, ffi, util, CompressionError, Result};
use std::ffi::{c_void, CStr};
use std::ptr::{null, null_mut, NonNull};

pub type AceTag = u32;
pub type AcePermSet = u64;
pub type AceFlagSet = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AceQualifierType {
    User = 'U' as u32,
    Group = 'G' as u32,
    Sid = 'S' as u32,
    Uuid = 'I' as u32,
}

impl AceQualifierType {
    const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            x if x == Self::User as u32 => Some(Self::User),
            x if x == Self::Group as u32 => Some(Self::Group),
            x if x == Self::Sid as u32 => Some(Self::Sid),
            x if x == Self::Uuid as u32 => Some(Self::Uuid),
            _ => None,
        }
    }

    const fn as_raw(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessControlEntry {
    pub tag: AceTag,
    pub perms: AcePermSet,
    pub flags: AceFlagSet,
    pub qualifier_type: AceQualifierType,
    pub qualifier: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NamedBlobEntry {
    pub key: String,
    pub value: Vec<u8>,
}

#[derive(Debug)]
pub struct EntryAclBlob {
    handle: NonNull<c_void>,
}

impl EntryAclBlob {
    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        Ok(Self {
            handle: util::nonnull_handle(handle, operation)?,
        })
    }

    pub(crate) fn clone_from_raw(raw: *mut c_void) -> Result<Self> {
        let handle = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_clone_from_raw(raw) };
        Self::from_handle(handle, "AAEntryACLBlobClone")
    }

    pub(crate) fn sync_into_raw(raw: *mut c_void, value: &Self) -> Result<()> {
        let status = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_clear_raw(raw) };
        util::status_result("AAEntryACLBlobClear", status)?;
        for entry in value.entries()? {
            let qualifier_ptr = if entry.qualifier.is_empty() {
                null()
            } else {
                entry.qualifier.as_ptr()
            };
            let status = unsafe {
                ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_append_entry_raw(
                    raw,
                    entry.tag,
                    entry.perms,
                    entry.flags,
                    entry.qualifier_type.as_raw(),
                    qualifier_ptr,
                    entry.qualifier.len(),
                )
            };
            util::status_result("AAEntryACLBlobAppendEntry", status)?;
        }
        Ok(())
    }

    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_create() };
        Self::from_handle(handle, "AAEntryACLBlobCreate")
    }

    pub fn from_encoded_data(data: &[u8]) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_create_with_encoded_data(
                data.as_ptr(),
                data.len(),
            )
        };
        Self::from_handle(handle, "AAEntryACLBlobCreateWithEncodedData")
    }

    pub fn from_path(dir: &str, path: &str, flags: ArchiveFlags) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let path = util::cstring("path", path)?;
        let handle = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_create_with_path(
                dir.as_ptr(),
                path.as_ptr(),
                flags.bits(),
            )
        };
        Self::from_handle(handle, "AAEntryACLBlobCreateWithPath")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn apply_to_path(&self, dir: &str, path: &str, flags: ArchiveFlags) -> Result<()> {
        let dir = util::cstring("dir", dir)?;
        let path = util::cstring("path", path)?;
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_apply_to_path(
                self.as_ptr(),
                dir.as_ptr(),
                path.as_ptr(),
                flags.bits(),
            )
        };
        util::status_result("AAEntryACLBlobApplyToPath", status)
    }

    pub fn entry_count(&self) -> u32 {
        unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_get_entry_count(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.entry_count() == 0
    }

    pub fn entry(&self, index: u32) -> Result<AccessControlEntry> {
        let mut tag = 0_u32;
        let mut perms = 0_u64;
        let mut flags = 0_u64;
        let mut qualifier_type = 0_u32;
        let mut qualifier_size = 0_usize;
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_get_entry(
                self.as_ptr(),
                index,
                &mut tag,
                &mut perms,
                &mut flags,
                &mut qualifier_type,
                0,
                null_mut(),
                &mut qualifier_size,
            )
        };
        util::status_result("AAEntryACLBlobGetEntry", status)?;

        let mut qualifier = vec![0_u8; qualifier_size];
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_get_entry(
                self.as_ptr(),
                index,
                &mut tag,
                &mut perms,
                &mut flags,
                &mut qualifier_type,
                qualifier.len(),
                if qualifier.is_empty() {
                    null_mut()
                } else {
                    qualifier.as_mut_ptr()
                },
                &mut qualifier_size,
            )
        };
        util::status_result("AAEntryACLBlobGetEntry", status)?;

        Ok(AccessControlEntry {
            tag,
            perms,
            flags,
            qualifier_type: AceQualifierType::from_raw(qualifier_type).ok_or_else(|| CompressionError::OperationFailed {
                operation: "AAEntryACLBlobGetEntry",
                code: i32::try_from(qualifier_type).unwrap_or(i32::MAX),
            })?,
            qualifier,
        })
    }

    pub fn entries(&self) -> Result<Vec<AccessControlEntry>> {
        (0..self.entry_count()).map(|index| self.entry(index)).collect()
    }

    pub fn append_entry(&mut self, entry: &AccessControlEntry) -> Result<()> {
        let qualifier_ptr = if entry.qualifier.is_empty() {
            null()
        } else {
            entry.qualifier.as_ptr()
        };
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_append_entry(
                self.as_ptr(),
                entry.tag,
                entry.perms,
                entry.flags,
                entry.qualifier_type.as_raw(),
                qualifier_ptr,
                entry.qualifier.len(),
            )
        };
        util::status_result("AAEntryACLBlobAppendEntry", status)
    }

    pub fn set_entry(&mut self, index: u32, entry: &AccessControlEntry) -> Result<()> {
        let qualifier_ptr = if entry.qualifier.is_empty() {
            null()
        } else {
            entry.qualifier.as_ptr()
        };
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_set_entry(
                self.as_ptr(),
                index,
                entry.tag,
                entry.perms,
                entry.flags,
                entry.qualifier_type.as_raw(),
                qualifier_ptr,
                entry.qualifier.len(),
            )
        };
        util::status_result("AAEntryACLBlobSetEntry", status)
    }

    pub fn clear(&mut self) -> Result<()> {
        let status = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_clear(self.as_ptr()) };
        util::status_result("AAEntryACLBlobClear", status)
    }

    pub fn remove_entry(&mut self, index: u32) -> Result<()> {
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_remove_entry(self.as_ptr(), index)
        };
        util::status_result("AAEntryACLBlobRemoveEntry", status)
    }

    pub fn encoded_size(&self) -> usize {
        unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_get_encoded_size(self.as_ptr()) }
    }

    pub fn encoded_data(&self) -> Result<Vec<u8>> {
        let size = self.encoded_size();
        let mut data = vec![0_u8; size];
        if size == 0 {
            return Ok(data);
        }
        let copied = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_copy_encoded_data(
                self.as_ptr(),
                data.as_mut_ptr(),
            )
        };
        if copied {
            Ok(data)
        } else {
            Err(CompressionError::OperationFailed {
                operation: "AAEntryACLBlobGetEncodedData",
                code: -1,
            })
        }
    }
}

impl Clone for EntryAclBlob {
    fn clone(&self) -> Self {
        Self::from_encoded_data(
            &self
                .encoded_data()
                .expect("AAEntryACLBlobGetEncodedData returned invalid data"),
        )
        .expect("AAEntryACLBlobCreateWithEncodedData returned null")
    }
}

impl Drop for EntryAclBlob {
    fn drop(&mut self) {
        unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_acl_blob_release(self.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct EntryXatBlob {
    handle: NonNull<c_void>,
}

impl EntryXatBlob {
    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        Ok(Self {
            handle: util::nonnull_handle(handle, operation)?,
        })
    }

    pub(crate) fn clone_from_raw(raw: *mut c_void) -> Result<Self> {
        let handle = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_clone_from_raw(raw) };
        Self::from_handle(handle, "AAEntryXATBlobClone")
    }

    pub(crate) fn sync_into_raw(raw: *mut c_void, value: &Self) -> Result<()> {
        let status = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_clear_raw(raw) };
        util::status_result("AAEntryXATBlobClear", status)?;
        for entry in value.entries()? {
            let key = util::cstring("key", &entry.key)?;
            let data_ptr = if entry.value.is_empty() {
                null()
            } else {
                entry.value.as_ptr()
            };
            let status = unsafe {
                ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_append_entry_raw(
                    raw,
                    key.as_ptr(),
                    data_ptr,
                    entry.value.len(),
                )
            };
            util::status_result("AAEntryXATBlobAppendEntry", status)?;
        }
        Ok(())
    }

    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_create() };
        Self::from_handle(handle, "AAEntryXATBlobCreate")
    }

    pub fn from_encoded_data(data: &[u8]) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_create_with_encoded_data(
                data.as_ptr(),
                data.len(),
            )
        };
        Self::from_handle(handle, "AAEntryXATBlobCreateWithEncodedData")
    }

    pub fn from_path(dir: &str, path: &str, flags: ArchiveFlags) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let path = util::cstring("path", path)?;
        let handle = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_create_with_path(
                dir.as_ptr(),
                path.as_ptr(),
                flags.bits(),
            )
        };
        Self::from_handle(handle, "AAEntryXATBlobCreateWithPath")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn apply_to_path(&self, dir: &str, path: &str, flags: ArchiveFlags) -> Result<()> {
        let dir = util::cstring("dir", dir)?;
        let path = util::cstring("path", path)?;
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_apply_to_path(
                self.as_ptr(),
                dir.as_ptr(),
                path.as_ptr(),
                flags.bits(),
            )
        };
        util::status_result("AAEntryXATBlobApplyToPath", status)
    }

    pub fn entry_count(&self) -> u32 {
        unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_get_entry_count(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.entry_count() == 0
    }

    pub fn entry(&self, index: u32) -> Result<NamedBlobEntry> {
        let mut key_length = 0_usize;
        let mut data_size = 0_usize;
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_get_entry(
                self.as_ptr(),
                index,
                0,
                null_mut(),
                &mut key_length,
                0,
                null_mut(),
                &mut data_size,
            )
        };
        util::status_result("AAEntryXATBlobGetEntry", status)?;

        let mut key = vec![0_i8; key_length.saturating_add(1)];
        let mut value = vec![0_u8; data_size];
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_get_entry(
                self.as_ptr(),
                index,
                key.len(),
                key.as_mut_ptr(),
                &mut key_length,
                value.len(),
                if value.is_empty() {
                    null_mut()
                } else {
                    value.as_mut_ptr()
                },
                &mut data_size,
            )
        };
        util::status_result("AAEntryXATBlobGetEntry", status)?;

        let key = unsafe { CStr::from_ptr(key.as_ptr()) }
            .to_str()
            .map_err(|_| CompressionError::Utf8Error {
                operation: "AAEntryXATBlobGetEntry",
            })?
            .to_string();

        Ok(NamedBlobEntry { key, value })
    }

    pub fn entries(&self) -> Result<Vec<NamedBlobEntry>> {
        (0..self.entry_count()).map(|index| self.entry(index)).collect()
    }

    pub fn append_entry(&mut self, entry: &NamedBlobEntry) -> Result<()> {
        let key = util::cstring("key", &entry.key)?;
        let data_ptr = if entry.value.is_empty() {
            null()
        } else {
            entry.value.as_ptr()
        };
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_append_entry(
                self.as_ptr(),
                key.as_ptr(),
                data_ptr,
                entry.value.len(),
            )
        };
        util::status_result("AAEntryXATBlobAppendEntry", status)
    }

    pub fn set_entry(&mut self, index: u32, entry: &NamedBlobEntry) -> Result<()> {
        let key = util::cstring("key", &entry.key)?;
        let data_ptr = if entry.value.is_empty() {
            null()
        } else {
            entry.value.as_ptr()
        };
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_set_entry(
                self.as_ptr(),
                index,
                key.as_ptr(),
                data_ptr,
                entry.value.len(),
            )
        };
        util::status_result("AAEntryXATBlobSetEntry", status)
    }

    pub fn clear(&mut self) -> Result<()> {
        let status = unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_clear(self.as_ptr()) };
        util::status_result("AAEntryXATBlobClear", status)
    }

    pub fn remove_entry(&mut self, index: u32) -> Result<()> {
        let status = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_remove_entry(self.as_ptr(), index)
        };
        util::status_result("AAEntryXATBlobRemoveEntry", status)
    }

    pub fn encoded_size(&self) -> usize {
        unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_get_encoded_size(self.as_ptr()) }
    }

    pub fn encoded_data(&self) -> Result<Vec<u8>> {
        let size = self.encoded_size();
        let mut data = vec![0_u8; size];
        if size == 0 {
            return Ok(data);
        }
        let copied = unsafe {
            ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_copy_encoded_data(
                self.as_ptr(),
                data.as_mut_ptr(),
            )
        };
        if copied {
            Ok(data)
        } else {
            Err(CompressionError::OperationFailed {
                operation: "AAEntryXATBlobGetEncodedData",
                code: -1,
            })
        }
    }
}

impl Clone for EntryXatBlob {
    fn clone(&self) -> Self {
        Self::from_encoded_data(
            &self
                .encoded_data()
                .expect("AAEntryXATBlobGetEncodedData returned invalid data"),
        )
        .expect("AAEntryXATBlobCreateWithEncodedData returned null")
    }
}

impl Drop for EntryXatBlob {
    fn drop(&mut self) {
        unsafe { ffi::aa_entry_blob::compression_rs_aa_entry_xat_blob_release(self.as_ptr()) };
    }
}
