use crate::{ffi, util, CompressionError, Result};
use std::ffi::c_void;
use std::fs::File;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};
use std::os::fd::{IntoRawFd, RawFd};
use std::ptr::NonNull;

const READ_CHUNK_LEN: usize = 32 * 1024;

/// Wraps `OPEN_READ_ONLY`.
pub const OPEN_READ_ONLY: i32 = 0x0000;
/// Wraps `OPEN_WRITE_ONLY`.
pub const OPEN_WRITE_ONLY: i32 = 0x0001;
/// Wraps `OPEN_READ_WRITE`.
pub const OPEN_READ_WRITE: i32 = 0x0002;
/// Wraps `OPEN_CREATE`.
pub const OPEN_CREATE: i32 = 0x0200;
/// Wraps `OPEN_TRUNCATE`.
pub const OPEN_TRUNCATE: i32 = 0x0400;
/// Wraps `DEFAULT_FILE_MODE`.
pub const DEFAULT_FILE_MODE: u32 = 0o644;

/// Wraps AppleArchive compression algorithm identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ArchiveCompressionAlgorithm {
    /// Wraps the `None` variant of `ArchiveCompressionAlgorithm`.
    None,
    /// Wraps the `Lz4` variant of `ArchiveCompressionAlgorithm`.
    Lz4,
    /// Wraps the `Zlib` variant of `ArchiveCompressionAlgorithm`.
    Zlib,
    /// Wraps the `Lzma` variant of `ArchiveCompressionAlgorithm`.
    Lzma,
    /// Wraps the `Lzfse` variant of `ArchiveCompressionAlgorithm`.
    Lzfse,
    /// Wraps the `Lzbitmap` variant of `ArchiveCompressionAlgorithm`.
    Lzbitmap,
}

impl ArchiveCompressionAlgorithm {
    /// Wraps raw AppleArchive compression algorithm values.
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0x000 => Some(Self::None),
            0x100 => Some(Self::Lz4),
            0x505 => Some(Self::Zlib),
            0x306 => Some(Self::Lzma),
            0x801 => Some(Self::Lzfse),
            0x702 => Some(Self::Lzbitmap),
            _ => None,
        }
    }

    /// Wraps raw AppleArchive compression algorithm values.
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

/// Wraps AppleArchive archive-processing flags.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct ArchiveFlags(u64);

impl ArchiveFlags {
    /// Wraps the `IGNORE_EPERM` AppleArchive archive flag bit.
    pub const IGNORE_EPERM: Self = Self(1_u64 << 0);
    /// Wraps the `ARCHIVE_DEDUPLICATE_DAT` AppleArchive archive flag bit.
    pub const ARCHIVE_DEDUPLICATE_DAT: Self = Self(1_u64 << 1);
    /// Wraps the `ARCHIVE_NO_RESOLVE_ACL_QUALIFIERS` AppleArchive archive flag bit.
    pub const ARCHIVE_NO_RESOLVE_ACL_QUALIFIERS: Self = Self(1_u64 << 2);
    /// Wraps the `REPLACE_ATTRIBUTES` AppleArchive archive flag bit.
    pub const REPLACE_ATTRIBUTES: Self = Self(1_u64 << 3);
    /// Wraps the `EXTRACT_NO_AUTO_DEDUP` AppleArchive archive flag bit.
    pub const EXTRACT_NO_AUTO_DEDUP: Self = Self(1_u64 << 4);
    /// Wraps the `EXTRACT_NO_AUTO_SPARSE` AppleArchive archive flag bit.
    pub const EXTRACT_NO_AUTO_SPARSE: Self = Self(1_u64 << 5);
    /// Wraps the `CROSS_VOLUME_BOUNDARIES` AppleArchive archive flag bit.
    pub const CROSS_VOLUME_BOUNDARIES: Self = Self(1_u64 << 6);
    /// Wraps the `EXTRACT_AUTO_DEDUP_AS_HARD_LINKS` AppleArchive archive flag bit.
    pub const EXTRACT_AUTO_DEDUP_AS_HARD_LINKS: Self = Self(1_u64 << 7);
    /// Wraps the `DECODE_INSERT_IDX` AppleArchive archive flag bit.
    pub const DECODE_INSERT_IDX: Self = Self(1_u64 << 8);
    /// Wraps the `EXCLUDE_METADATA_ENTRIES` AppleArchive archive flag bit.
    pub const EXCLUDE_METADATA_ENTRIES: Self = Self(1_u64 << 9);
    /// Wraps the `PROCESS_RANDOM_ACCESS_OUTPUT` AppleArchive archive flag bit.
    pub const PROCESS_RANDOM_ACCESS_OUTPUT: Self = Self(1_u64 << 10);
    /// Wraps the `VERBOSITY_0` AppleArchive archive flag bit.
    pub const VERBOSITY_0: Self = Self(0_u64 << 62);
    /// Wraps the `VERBOSITY_1` AppleArchive archive flag bit.
    pub const VERBOSITY_1: Self = Self(1_u64 << 62);
    /// Wraps the `VERBOSITY_2` AppleArchive archive flag bit.
    pub const VERBOSITY_2: Self = Self(2_u64 << 62);
    /// Wraps the `VERBOSITY_3` AppleArchive archive flag bit.
    pub const VERBOSITY_3: Self = Self(3_u64 << 62);

    /// Wraps an empty AppleArchive archive flag set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Wraps the raw AppleArchive archive flag bits.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps raw AppleArchive archive flag bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Wraps AppleArchive archive flag containment checks.
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Wraps AppleArchive archive verbosity flag construction.
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
pub enum ByteStreamUpstream {
    Stream(Box<ByteStream>),
}

/// Wraps an `AAByteStream` handle.
#[derive(Debug)]
pub struct ByteStream {
    handle: NonNull<c_void>,
    _upstream: Option<ByteStreamUpstream>,
    closed: bool,
}

impl ByteStream {
    pub(crate) fn from_handle_with_upstream(
        handle: *mut c_void,
        operation: &'static str,
        upstream: Option<ByteStreamUpstream>,
    ) -> Result<Self> {
        Ok(Self {
            handle: util::nonnull_handle(handle, operation)?,
            _upstream: upstream,
            closed: false,
        })
    }

    /// Wraps `AAFileStreamOpenWithFD`.
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

    /// Wraps `AAFileStreamOpenWithFD` through `File` ownership.
    pub fn from_file(file: File) -> Result<Self> {
        Self::from_fd(file.into_raw_fd(), true)
    }

    /// Wraps `AAFileStreamOpenWithPath`.
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

    /// Wraps `AATempFileStreamOpen`.
    pub fn temp_file() -> Result<Self> {
        let handle = unsafe { ffi::aa_byte_stream::compression_rs_aa_temp_file_stream_open() };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AATempFileStreamOpen")?,
            _upstream: None,
            closed: false,
        })
    }

    /// Wraps `AASharedBufferPipeOpen`.
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

    pub(crate) fn mark_closed(&mut self) {
        self.closed = true;
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

    /// Wraps `AACompressionOutputStreamOpen`.
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

    /// Wraps `AACompressionOutputStreamOpenExisting`.
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

    /// Wraps `AADecompressionInputStreamOpen`.
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

    /// Wraps `AADecompressionRandomAccessInputStreamOpen`.
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

    /// Wraps `AAByteStreamWrite`.
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

    /// Wraps `AAByteStreamWrite`.
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

    /// Wraps `AAByteStreamPWrite`.
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

    /// Wraps `AAByteStreamRead`.
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

    /// Wraps `AAByteStreamPRead`.
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

    /// Wraps `AAByteStreamSeek`.
    pub fn seek(&mut self, offset: i64, whence: i32) -> Result<u64> {
        self.ensure_open()?;
        util::off_t_result("AAByteStreamSeek", unsafe {
            ffi::aa_byte_stream::compression_rs_aa_byte_stream_seek(self.as_ptr(), offset, whence)
        })
    }

    /// Wraps repeated `AAByteStreamRead` calls.
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

    /// Wraps `AAByteStreamClose`.
    pub fn cancel(&mut self) -> Result<()> {
        self.ensure_open()?;
        unsafe { ffi::aa_byte_stream::compression_rs_aa_byte_stream_cancel(self.as_ptr()) };
        Ok(())
    }

    #[deprecated(
        since = "0.2.2",
        note = "Use ByteStream::cancel; AAByteStreamAbort is a deprecated AppleArchive compatibility shim."
    )]
    /// Wraps `AAByteStreamClose`.
    pub fn abort(&mut self) -> Result<()> {
        self.ensure_open()?;
        unsafe { ffi::aa_byte_stream::compression_rs_aa_byte_stream_abort(self.as_ptr()) };
        Ok(())
    }

    /// Wraps `AAByteStreamClose`.
    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        let status =
            unsafe { ffi::aa_byte_stream::compression_rs_aa_byte_stream_close(self.as_ptr()) };
        self.closed = true;
        util::status_result("AAByteStreamClose", status)
    }

    /// Wraps `AAByteStreamProcess`.
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

    /// Wraps `AARandomAccessByteStreamProcess`.
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

fn custom_byte_stream_error(operation: &'static str) -> CompressionError {
    CompressionError::OperationFailed {
        operation,
        code: -1,
    }
}

fn custom_byte_stream_code(error: &CompressionError) -> i32 {
    match error {
        CompressionError::OperationFailed { code, .. } if *code < 0 => *code,
        _ => -1,
    }
}

struct CustomByteStreamState {
    callbacks: Box<dyn CustomByteStreamCallbacks>,
}

/// Wraps callbacks installed by `AACustomByteStreamSet*Proc`.
pub trait CustomByteStreamCallbacks {
    /// Wraps `AAByteStreamWrite`.
    fn write(&mut self, _buffer: &[u8]) -> Result<usize> {
        Err(custom_byte_stream_error("AAByteStreamWrite"))
    }

    /// Wraps `AAByteStreamPWrite`.
    fn pwrite(&mut self, _buffer: &[u8], _offset: i64) -> Result<usize> {
        Err(custom_byte_stream_error("AAByteStreamPWrite"))
    }

    /// Wraps `AAByteStreamRead`.
    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize> {
        Err(custom_byte_stream_error("AAByteStreamRead"))
    }

    /// Wraps `AAByteStreamPRead`.
    fn pread(&mut self, _buffer: &mut [u8], _offset: i64) -> Result<usize> {
        Err(custom_byte_stream_error("AAByteStreamPRead"))
    }

    /// Wraps `AAByteStreamSeek`.
    fn seek(&mut self, _offset: i64, _whence: i32) -> Result<i64> {
        Err(custom_byte_stream_error("AAByteStreamSeek"))
    }

    /// Wraps the `cancel` convenience for `CustomByteStreamCallbacks`.
    fn cancel(&mut self) {}

    /// Wraps the `close` convenience for `CustomByteStreamCallbacks`.
    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

unsafe fn custom_byte_stream_slice<'a>(buffer: *const c_void, length: usize) -> Option<&'a [u8]> {
    if length == 0 {
        Some(&[])
    } else if buffer.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), length) })
    }
}

unsafe fn custom_byte_stream_slice_mut<'a>(
    buffer: *mut c_void,
    length: usize,
) -> Option<&'a mut [u8]> {
    if length == 0 {
        Some(&mut [])
    } else if buffer.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts_mut(buffer.cast::<u8>(), length) })
    }
}

unsafe fn custom_byte_stream_state(arg: *mut c_void) -> Option<&'static mut CustomByteStreamState> {
    if arg.is_null() {
        None
    } else {
        Some(unsafe { &mut *arg.cast::<CustomByteStreamState>() })
    }
}

unsafe extern "C" fn custom_byte_stream_write(
    arg: *mut c_void,
    buffer: *const c_void,
    length: usize,
) -> i64 {
    let Some(state) = (unsafe { custom_byte_stream_state(arg) }) else {
        return -1;
    };
    let Some(buffer) = (unsafe { custom_byte_stream_slice(buffer, length) }) else {
        return -1;
    };
    match state.callbacks.write(buffer) {
        Ok(count) => i64::try_from(count).unwrap_or(i64::MAX),
        Err(error) => i64::from(custom_byte_stream_code(&error)),
    }
}

unsafe extern "C" fn custom_byte_stream_pwrite(
    arg: *mut c_void,
    buffer: *const c_void,
    length: usize,
    offset: i64,
) -> i64 {
    let Some(state) = (unsafe { custom_byte_stream_state(arg) }) else {
        return -1;
    };
    let Some(buffer) = (unsafe { custom_byte_stream_slice(buffer, length) }) else {
        return -1;
    };
    match state.callbacks.pwrite(buffer, offset) {
        Ok(count) => i64::try_from(count).unwrap_or(i64::MAX),
        Err(error) => i64::from(custom_byte_stream_code(&error)),
    }
}

unsafe extern "C" fn custom_byte_stream_read(
    arg: *mut c_void,
    buffer: *mut c_void,
    length: usize,
) -> i64 {
    let Some(state) = (unsafe { custom_byte_stream_state(arg) }) else {
        return -1;
    };
    let Some(buffer) = (unsafe { custom_byte_stream_slice_mut(buffer, length) }) else {
        return -1;
    };
    match state.callbacks.read(buffer) {
        Ok(count) => i64::try_from(count).unwrap_or(i64::MAX),
        Err(error) => i64::from(custom_byte_stream_code(&error)),
    }
}

unsafe extern "C" fn custom_byte_stream_pread(
    arg: *mut c_void,
    buffer: *mut c_void,
    length: usize,
    offset: i64,
) -> i64 {
    let Some(state) = (unsafe { custom_byte_stream_state(arg) }) else {
        return -1;
    };
    let Some(buffer) = (unsafe { custom_byte_stream_slice_mut(buffer, length) }) else {
        return -1;
    };
    match state.callbacks.pread(buffer, offset) {
        Ok(count) => i64::try_from(count).unwrap_or(i64::MAX),
        Err(error) => i64::from(custom_byte_stream_code(&error)),
    }
}

unsafe extern "C" fn custom_byte_stream_seek(arg: *mut c_void, offset: i64, whence: i32) -> i64 {
    let Some(state) = (unsafe { custom_byte_stream_state(arg) }) else {
        return -1;
    };
    match state.callbacks.seek(offset, whence) {
        Ok(position) => position,
        Err(error) => i64::from(custom_byte_stream_code(&error)),
    }
}

unsafe extern "C" fn custom_byte_stream_cancel(arg: *mut c_void) {
    if let Some(state) = unsafe { custom_byte_stream_state(arg) } {
        state.callbacks.cancel();
    }
}

unsafe extern "C" fn custom_byte_stream_close(arg: *mut c_void) -> i32 {
    if arg.is_null() {
        return 0;
    }
    let mut state = unsafe { Box::from_raw(arg.cast::<CustomByteStreamState>()) };
    match state.callbacks.close() {
        Ok(()) => 0,
        Err(error) => custom_byte_stream_code(&error),
    }
}

impl ByteStream {
    /// Wraps `AACustomByteStreamOpen`.
    pub fn custom<T: CustomByteStreamCallbacks + 'static>(callbacks: T) -> Result<Self> {
        let handle = unsafe { ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_open() };
        let stream = Self::from_handle_with_upstream(handle, "AACustomByteStreamOpen", None)?;
        let state = Box::new(CustomByteStreamState {
            callbacks: Box::new(callbacks),
        });
        let data = Box::into_raw(state).cast::<c_void>();
        unsafe {
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_data(
                stream.as_ptr(),
                data,
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_write_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_write),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_pwrite_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_pwrite),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_read_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_read),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_pread_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_pread),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_seek_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_seek),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_cancel_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_cancel),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_abort_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_cancel),
            );
            ffi::aa_byte_stream::compression_rs_aa_custom_byte_stream_set_close_proc(
                stream.as_ptr(),
                Some(custom_byte_stream_close),
            );
        }
        Ok(stream)
    }
}
