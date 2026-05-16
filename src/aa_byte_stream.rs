use crate::{ffi, util, CompressionError, Result};
use std::ffi::c_void;
use std::fs::File;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::os::fd::{IntoRawFd, RawFd};
use std::ptr::NonNull;

const READ_CHUNK_LEN: usize = 32 * 1024;

pub const OPEN_READ_ONLY: i32 = 0x0000;
pub const OPEN_WRITE_ONLY: i32 = 0x0001;
pub const OPEN_READ_WRITE: i32 = 0x0002;
pub const OPEN_CREATE: i32 = 0x0200;
pub const OPEN_TRUNCATE: i32 = 0x0400;
pub const DEFAULT_FILE_MODE: u32 = 0o644;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ArchiveCompressionAlgorithm {
    None,
    Lz4,
    Zlib,
    Lzma,
    Lzfse,
    Lzbitmap,
}

impl ArchiveCompressionAlgorithm {
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::None => 0x000,
            Self::Lz4 => 0x100,
            Self::Zlib => 0x505,
            Self::Lzma => 0x306,
            Self::Lzfse => 0x801,
            Self::Lzbitmap => 0x702,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ArchiveFlags(u64);

impl ArchiveFlags {
    pub const IGNORE_EPERM: Self = Self(1_u64 << 0);
    pub const ARCHIVE_DEDUPLICATE_DAT: Self = Self(1_u64 << 1);
    pub const ARCHIVE_NO_RESOLVE_ACL_QUALIFIERS: Self = Self(1_u64 << 2);
    pub const REPLACE_ATTRIBUTES: Self = Self(1_u64 << 3);
    pub const EXTRACT_NO_AUTO_DEDUP: Self = Self(1_u64 << 4);
    pub const EXTRACT_NO_AUTO_SPARSE: Self = Self(1_u64 << 5);
    pub const CROSS_VOLUME_BOUNDARIES: Self = Self(1_u64 << 6);
    pub const EXTRACT_AUTO_DEDUP_AS_HARD_LINKS: Self = Self(1_u64 << 7);
    pub const DECODE_INSERT_IDX: Self = Self(1_u64 << 8);
    pub const EXCLUDE_METADATA_ENTRIES: Self = Self(1_u64 << 9);
    pub const PROCESS_RANDOM_ACCESS_OUTPUT: Self = Self(1_u64 << 10);
    pub const VERBOSITY_0: Self = Self(0_u64 << 62);
    pub const VERBOSITY_1: Self = Self(1_u64 << 62);
    pub const VERBOSITY_2: Self = Self(2_u64 << 62);
    pub const VERBOSITY_3: Self = Self(3_u64 << 62);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn verbosity(level: u64) -> Self {
        let clamped = if level > 3 { 3 } else { level };
        Self(clamped << 62)
    }
}

impl BitOr for ArchiveFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ArchiveFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for ArchiveFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for ArchiveFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum ByteStreamUpstream {
    Stream(Box<ByteStream>),
}

#[derive(Debug)]
pub struct ByteStream {
    handle: NonNull<c_void>,
    _upstream: Option<ByteStreamUpstream>,
    closed: bool,
}

impl ByteStream {
    pub fn from_fd(fd: RawFd, automatic_close: bool) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_open_with_fd(fd, automatic_close)
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAFileStreamOpenWithFD")?,
            _upstream: None,
            closed: false,
        })
    }

    pub fn from_file(file: File) -> Result<Self> {
        Self::from_fd(file.into_raw_fd(), true)
    }

    pub fn open_with_path(path: &str, open_flags: i32, open_mode: u32) -> Result<Self> {
        let path = util::cstring("path", path)?;
        let handle = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_open_with_path(
                path.as_ptr(),
                open_flags,
                open_mode,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAFileStreamOpenWithPath")?,
            _upstream: None,
            closed: false,
        })
    }

    pub fn temp_file() -> Result<Self> {
        let handle = unsafe { ffi::aa_byte_stream::compression_rs_aa_temp_file_stream_open() };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AATempFileStreamOpen")?,
            _upstream: None,
            closed: false,
        })
    }

    pub fn shared_buffer_pipe(buffer_capacity: usize) -> Result<(Self, Self)> {
        let mut ostream = std::ptr::null_mut();
        let mut istream = std::ptr::null_mut();
        let status = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_shared_buffer_pipe_open(
                &mut ostream,
                &mut istream,
                buffer_capacity,
            )
        };
        util::status_result("AASharedBufferPipeOpen", status)?;
        Ok((
            Self {
                handle: util::nonnull_handle(ostream, "AASharedBufferPipeOpen(ostream)")?,
                _upstream: None,
                closed: false,
            },
            Self {
                handle: util::nonnull_handle(istream, "AASharedBufferPipeOpen(istream)")?,
                _upstream: None,
                closed: false,
            },
        ))
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    fn ensure_open(&self) -> Result<()> {
        if self.closed {
            Err(CompressionError::Closed {
                resource: "byte stream",
            })
        } else {
            Ok(())
        }
    }

    pub fn into_compression_output(
        self,
        compression_algorithm: ArchiveCompressionAlgorithm,
        block_size: usize,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_compression_output_stream_open(
                self.as_ptr(),
                compression_algorithm.as_raw(),
                block_size,
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AACompressionOutputStreamOpen")?,
            _upstream: Some(ByteStreamUpstream::Stream(Box::new(self))),
            closed: false,
        })
    }

    pub fn into_existing_compression_output(
        self,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_compression_output_stream_open_existing(
                self.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AACompressionOutputStreamOpenExisting")?,
            _upstream: Some(ByteStreamUpstream::Stream(Box::new(self))),
            closed: false,
        })
    }

    pub fn into_decompression_input(self, flags: ArchiveFlags, n_threads: i32) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_decompression_input_stream_open(
                self.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AADecompressionInputStreamOpen")?,
            _upstream: Some(ByteStreamUpstream::Stream(Box::new(self))),
            closed: false,
        })
    }

    pub fn into_random_access_decompression_input(
        self,
        alloc_limit: usize,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_byte_stream::compression_rs_aa_decompression_random_access_input_stream_open(
                self.as_ptr(),
                alloc_limit,
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AADecompressionRandomAccessInputStreamOpen")?,
            _upstream: Some(ByteStreamUpstream::Stream(Box::new(self))),
            closed: false,
        })
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<usize> {
        self.ensure_open()?;
        util::ssize_result("AAByteStreamWrite", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_write(
                self.as_ptr(),
                buffer.as_ptr(),
                buffer.len(),
            )
        })
    }

    pub fn write_all(&mut self, mut buffer: &[u8]) -> Result<()> {
        while !buffer.is_empty() {
            let written = self.write(buffer)?;
            if written == 0 {
                return Err(CompressionError::OperationFailed {
                    operation: "AAByteStreamWrite",
                    code: -1,
                });
            }
            buffer = &buffer[written..];
        }
        Ok(())
    }

    pub fn pwrite(&mut self, buffer: &[u8], offset: i64) -> Result<usize> {
        self.ensure_open()?;
        util::ssize_result("AAByteStreamPWrite", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_pwrite(
                self.as_ptr(),
                buffer.as_ptr(),
                buffer.len(),
                offset,
            )
        })
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.ensure_open()?;
        util::ssize_result("AAByteStreamRead", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_read(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        })
    }

    pub fn pread(&mut self, buffer: &mut [u8], offset: i64) -> Result<usize> {
        self.ensure_open()?;
        util::ssize_result("AAByteStreamPRead", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_pread(
                self.as_ptr(),
                buffer.as_mut_ptr(),
                buffer.len(),
                offset,
            )
        })
    }

    pub fn seek(&mut self, offset: i64, whence: i32) -> Result<u64> {
        self.ensure_open()?;
        util::off_t_result("AAByteStreamSeek", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_seek(self.as_ptr(), offset, whence)
        })
    }

    pub fn read_to_end(&mut self) -> Result<Vec<u8>> {
        let mut output = Vec::new();
        loop {
            let mut buffer = vec![0_u8; READ_CHUNK_LEN];
            let read = self.read(&mut buffer)?;
            if read == 0 {
                return Ok(output);
            }
            output.extend_from_slice(&buffer[..read]);
        }
    }

    pub fn cancel(&mut self) -> Result<()> {
        self.ensure_open()?;
        unsafe { ffi::aa_byte_stream::compression_rs_aa_byte_stream_cancel(self.as_ptr()) };
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        let status =
            unsafe { ffi::aa_byte_stream::compression_rs_aa_byte_stream_close(self.as_ptr()) };
        self.closed = true;
        util::status_result("AAByteStreamClose", status)
    }

    pub fn process_into(&mut self, output: &mut Self) -> Result<u64> {
        self.ensure_open()?;
        output.ensure_open()?;
        util::off_t_result("AAByteStreamProcess", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_process(
                self.as_ptr(),
                output.as_ptr(),
            )
        })
    }

    pub fn process_random_access_into(
        &mut self,
        output: &mut Self,
        max_offset: i64,
        block_size: usize,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<u64> {
        self.ensure_open()?;
        output.ensure_open()?;
        util::off_t_result("AARandomAccessByteStreamProcess", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_random_access_byte_stream_process(
                self.as_ptr(),
                output.as_ptr(),
                max_offset,
                block_size,
                flags.bits(),
                n_threads,
            )
        })
    }
}

impl Drop for ByteStream {
    fn drop(&mut self) {
        unsafe { ffi::aa_byte_stream::compression_rs_aa_byte_stream_release(self.as_ptr()) };
    }
}
