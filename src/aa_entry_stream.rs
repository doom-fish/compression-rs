use crate::{
    aa_byte_stream::ArchiveFlags, aa_header::Timespec, ffi, util, CompressionError, Result,
};
use std::ffi::{c_void, CStr};
use std::ptr::{null, NonNull};

/// Wraps AppleArchive entry message identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum EntryMessage {
    /// Wraps the `SearchPruneDir` variant of `EntryMessage`.
    SearchPruneDir = 10,
    /// Wraps the `SearchExclude` variant of `EntryMessage`.
    SearchExclude = 11,
    /// Wraps the `SearchFail` variant of `EntryMessage`.
    SearchFail = 12,
    /// Wraps the `ExtractBegin` variant of `EntryMessage`.
    ExtractBegin = 20,
    /// Wraps the `ExtractEnd` variant of `EntryMessage`.
    ExtractEnd = 21,
    /// Wraps the `ExtractFail` variant of `EntryMessage`.
    ExtractFail = 22,
    /// Wraps the `ExtractAttributes` variant of `EntryMessage`.
    ExtractAttributes = 23,
    /// Wraps the `ExtractXat` variant of `EntryMessage`.
    ExtractXat = 24,
    /// Wraps the `ExtractAcl` variant of `EntryMessage`.
    ExtractAcl = 25,
    /// Wraps the `EncodeScanning` variant of `EntryMessage`.
    EncodeScanning = 30,
    /// Wraps the `EncodeWriting` variant of `EntryMessage`.
    EncodeWriting = 31,
    /// Wraps the `ConvertExclude` variant of `EntryMessage`.
    ConvertExclude = 40,
    /// Wraps the `ProcessExclude` variant of `EntryMessage`.
    ProcessExclude = 50,
    /// Wraps the `DecodeReading` variant of `EntryMessage`.
    DecodeReading = 60,
}

impl EntryMessage {
    pub(crate) const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            x if x == Self::SearchPruneDir as u32 => Some(Self::SearchPruneDir),
            x if x == Self::SearchExclude as u32 => Some(Self::SearchExclude),
            x if x == Self::SearchFail as u32 => Some(Self::SearchFail),
            x if x == Self::ExtractBegin as u32 => Some(Self::ExtractBegin),
            x if x == Self::ExtractEnd as u32 => Some(Self::ExtractEnd),
            x if x == Self::ExtractFail as u32 => Some(Self::ExtractFail),
            x if x == Self::ExtractAttributes as u32 => Some(Self::ExtractAttributes),
            x if x == Self::ExtractXat as u32 => Some(Self::ExtractXat),
            x if x == Self::ExtractAcl as u32 => Some(Self::ExtractAcl),
            x if x == Self::EncodeScanning as u32 => Some(Self::EncodeScanning),
            x if x == Self::EncodeWriting as u32 => Some(Self::EncodeWriting),
            x if x == Self::ConvertExclude as u32 => Some(Self::ConvertExclude),
            x if x == Self::ProcessExclude as u32 => Some(Self::ProcessExclude),
            x if x == Self::DecodeReading as u32 => Some(Self::DecodeReading),
            _ => None,
        }
    }
}

/// Wraps AppleArchive entry attribute values.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntryAttributes {
    /// Wraps the `bits` field of `EntryAttributes`.
    pub bits: u32,
    /// Wraps the `uid` field of `EntryAttributes`.
    pub uid: u32,
    /// Wraps the `gid` field of `EntryAttributes`.
    pub gid: u32,
    /// Wraps the `flags` field of `EntryAttributes`.
    pub flags: u32,
    /// Wraps the `mode` field of `EntryAttributes`.
    pub mode: u32,
    /// Wraps the `backup_time` field of `EntryAttributes`.
    pub backup_time: Timespec,
    /// Wraps the `creation_time` field of `EntryAttributes`.
    pub creation_time: Timespec,
    /// Wraps the `modification_time` field of `EntryAttributes`.
    pub modification_time: Timespec,
}

impl EntryAttributes {
    /// Wraps the `UID_BIT` AppleArchive entry attribute bit.
    pub const UID_BIT: u32 = 1 << 0;
    /// Wraps the `GID_BIT` AppleArchive entry attribute bit.
    pub const GID_BIT: u32 = 1 << 1;
    /// Wraps the `FLAGS_BIT` AppleArchive entry attribute bit.
    pub const FLAGS_BIT: u32 = 1 << 2;
    /// Wraps the `MODE_BIT` AppleArchive entry attribute bit.
    pub const MODE_BIT: u32 = 1 << 3;
    /// Wraps the `BACKUP_TIME_BIT` AppleArchive entry attribute bit.
    pub const BACKUP_TIME_BIT: u32 = 1 << 4;
    /// Wraps the `CREATION_TIME_BIT` AppleArchive entry attribute bit.
    pub const CREATION_TIME_BIT: u32 = 1 << 5;
    /// Wraps the `MODIFICATION_TIME_BIT` AppleArchive entry attribute bit.
    pub const MODIFICATION_TIME_BIT: u32 = 1 << 6;

    /// Wraps `AAPathListCreateWithDirectoryContents`.
    pub const fn has_uid(self) -> bool {
        self.bits & Self::UID_BIT != 0
    }

    /// Wraps `AAPathListCreateWithDirectoryContents`.
    pub const fn has_gid(self) -> bool {
        self.bits & Self::GID_BIT != 0
    }

    /// Wraps `AAPathListCreateWithDirectoryContents`.
    pub const fn has_flags(self) -> bool {
        self.bits & Self::FLAGS_BIT != 0
    }

    /// Wraps `AAPathListCreateWithDirectoryContents`.
    pub const fn has_mode(self) -> bool {
        self.bits & Self::MODE_BIT != 0
    }
}

/// Wraps an `AAPathList` handle.
#[derive(Debug)]
pub struct PathList {
    handle: NonNull<c_void>,
}

impl PathList {
    /// Wraps `END_NODE`.
    pub const END_NODE: u64 = u64::MAX;

    /// Wraps `AAPathListCreateWithDirectoryContents`.
    pub fn from_directory_contents(
        dir: &str,
        path: Option<&str>,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let path_cstring = path.map(|value| util::cstring("path", value)).transpose()?;
        let handle = unsafe {
            ffi::aa_entry_stream::compression_rs_aa_path_list_create_with_directory_contents(
                dir.as_ptr(),
                path_cstring.as_ref().map_or(null(), |value| value.as_ptr()),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAPathListCreateWithDirectoryContents")?,
        })
    }

    /// Wraps `AAPathListCreateWithPath`.
    pub fn from_path(dir: &str, path: &str) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let path = util::cstring("path", path)?;
        let handle = unsafe {
            ffi::aa_entry_stream::compression_rs_aa_path_list_create_with_path(
                dir.as_ptr(),
                path.as_ptr(),
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAPathListCreateWithPath")?,
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Wraps `AAPathListNodeGetPath`.
    pub fn first_node(&self) -> Option<u64> {
        let node =
            unsafe { ffi::aa_entry_stream::compression_rs_aa_path_list_node_first(self.as_ptr()) };
        (node != Self::END_NODE).then_some(node)
    }

    /// Wraps `AAPathListNodeGetPath`.
    pub fn next_node(&self, node: u64) -> Option<u64> {
        let next = unsafe {
            ffi::aa_entry_stream::compression_rs_aa_path_list_node_next(self.as_ptr(), node)
        };
        (next != Self::END_NODE).then_some(next)
    }

    /// Wraps `AAPathListNodeGetPath`.
    pub fn node_path(&self, node: u64) -> Result<String> {
        let mut length = 0_usize;
        let status = unsafe {
            ffi::aa_entry_stream::compression_rs_aa_path_list_node_get_path(
                self.as_ptr(),
                node,
                0,
                std::ptr::null_mut(),
                &mut length,
            )
        };
        util::status_result("AAPathListNodeGetPath", status)?;

        let mut buffer = vec![0_i8; length.saturating_add(1)];
        let status = unsafe {
            ffi::aa_entry_stream::compression_rs_aa_path_list_node_get_path(
                self.as_ptr(),
                node,
                buffer.len(),
                buffer.as_mut_ptr(),
                &mut length,
            )
        };
        util::status_result("AAPathListNodeGetPath", status)?;

        let value = unsafe { CStr::from_ptr(buffer.as_ptr()) }
            .to_str()
            .map_err(|_| CompressionError::Utf8Error {
                operation: "AAPathListNodeGetPath",
            })?;
        Ok(value.to_string())
    }

    /// Wraps iterative `AAPathListNode*` traversal.
    pub fn paths(&self) -> Result<Vec<String>> {
        let mut paths = Vec::new();
        let mut node = self.first_node();
        while let Some(current) = node {
            paths.push(self.node_path(current)?);
            node = self.next_node(current);
        }
        Ok(paths)
    }
}

impl Drop for PathList {
    fn drop(&mut self) {
        unsafe { ffi::aa_entry_stream::compression_rs_aa_path_list_release(self.as_ptr()) };
    }
}
