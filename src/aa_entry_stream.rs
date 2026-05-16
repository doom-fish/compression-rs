use crate::{
    aa_byte_stream::ArchiveFlags, aa_header::Timespec, ffi, util, CompressionError, Result,
};
use std::ffi::{c_void, CStr};
use std::ptr::{null, NonNull};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum EntryMessage {
    SearchPruneDir = 10,
    SearchExclude = 11,
    SearchFail = 12,
    ExtractBegin = 20,
    ExtractEnd = 21,
    ExtractFail = 22,
    ExtractAttributes = 23,
    ExtractXat = 24,
    ExtractAcl = 25,
    EncodeScanning = 30,
    EncodeWriting = 31,
    ConvertExclude = 40,
    ProcessExclude = 50,
    DecodeReading = 60,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct EntryAttributes {
    pub bits: u32,
    pub uid: u32,
    pub gid: u32,
    pub flags: u32,
    pub mode: u32,
    pub backup_time: Timespec,
    pub creation_time: Timespec,
    pub modification_time: Timespec,
}

impl EntryAttributes {
    pub const UID_BIT: u32 = 1 << 0;
    pub const GID_BIT: u32 = 1 << 1;
    pub const FLAGS_BIT: u32 = 1 << 2;
    pub const MODE_BIT: u32 = 1 << 3;
    pub const BACKUP_TIME_BIT: u32 = 1 << 4;
    pub const CREATION_TIME_BIT: u32 = 1 << 5;
    pub const MODIFICATION_TIME_BIT: u32 = 1 << 6;

    pub const fn has_uid(self) -> bool {
        self.bits & Self::UID_BIT != 0
    }

    pub const fn has_gid(self) -> bool {
        self.bits & Self::GID_BIT != 0
    }

    pub const fn has_flags(self) -> bool {
        self.bits & Self::FLAGS_BIT != 0
    }

    pub const fn has_mode(self) -> bool {
        self.bits & Self::MODE_BIT != 0
    }
}

#[derive(Debug)]
pub struct PathList {
    handle: NonNull<c_void>,
}

impl PathList {
    pub const END_NODE: u64 = u64::MAX;

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

    pub fn first_node(&self) -> Option<u64> {
        let node =
            unsafe { ffi::aa_entry_stream::compression_rs_aa_path_list_node_first(self.as_ptr()) };
        (node != Self::END_NODE).then_some(node)
    }

    pub fn next_node(&self, node: u64) -> Option<u64> {
        let next = unsafe {
            ffi::aa_entry_stream::compression_rs_aa_path_list_node_next(self.as_ptr(), node)
        };
        (next != Self::END_NODE).then_some(next)
    }

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
