use crate::{
    aa_byte_stream::{ArchiveFlags, ByteStream},
    aa_entry_blob::{EntryAclBlob, EntryXatBlob},
    aa_entry_stream::{EntryAttributes, EntryMessage, PathList},
    aa_field_key::{FieldKey, FieldKeySet},
    aa_header::Header,
    ffi, util, CompressionError, Result,
};
use std::ffi::{c_char, c_void, CStr};
use std::ptr::NonNull;

#[allow(dead_code)]
#[derive(Debug)]
enum ArchiveStreamUpstream {
    Byte(Box<ByteStream>),
    Archive(Box<ArchiveStream>),
}

/// Wraps an `AAArchiveStream` handle.
#[derive(Debug)]
pub struct ArchiveStream {
    handle: NonNull<c_void>,
    _upstream: Option<ArchiveStreamUpstream>,
    closed: bool,
    _message_handler: Option<Box<ArchiveMessageState>>,
}

impl ArchiveStream {
    /// Wraps `AAExtractArchiveOutputStreamOpen`.
    pub fn extract_output(dir: &str, flags: ArchiveFlags, n_threads: i32) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_extract_archive_output_stream_open(
                dir.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAExtractArchiveOutputStreamOpen")?,
            _upstream: None,
            closed: false,
            _message_handler: None,
        })
    }

    /// Wraps `AAEncodeArchiveOutputStreamOpen`.
    pub fn encode_output(stream: ByteStream, flags: ArchiveFlags, n_threads: i32) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_encode_archive_output_stream_open(
                stream.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAEncodeArchiveOutputStreamOpen")?,
            _upstream: Some(ArchiveStreamUpstream::Byte(Box::new(stream))),
            closed: false,
            _message_handler: None,
        })
    }

    /// Wraps `AADecodeArchiveInputStreamOpen`.
    pub fn decode_input(stream: ByteStream, flags: ArchiveFlags, n_threads: i32) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_decode_archive_input_stream_open(
                stream.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AADecodeArchiveInputStreamOpen")?,
            _upstream: Some(ArchiveStreamUpstream::Byte(Box::new(stream))),
            closed: false,
            _message_handler: None,
        })
    }

    /// Wraps `AAConvertArchiveOutputStreamOpen`.
    pub fn convert_output(
        stream: ArchiveStream,
        insert_key_set: &FieldKeySet,
        remove_key_set: &FieldKeySet,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<Self> {
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_convert_archive_output_stream_open(
                stream.as_ptr(),
                insert_key_set.as_ptr(),
                remove_key_set.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAConvertArchiveOutputStreamOpen")?,
            _upstream: Some(ArchiveStreamUpstream::Archive(Box::new(stream))),
            closed: false,
            _message_handler: None,
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    fn ensure_open(&self) -> Result<()> {
        if self.closed {
            Err(CompressionError::Closed {
                resource: "archive stream",
            })
        } else {
            Ok(())
        }
    }

    /// Wraps `AAArchiveStreamWriteHeader`.
    pub fn write_header(&mut self, header: &Header) -> Result<()> {
        self.ensure_open()?;
        let status = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_write_header(
                self.as_ptr(),
                header.as_ptr(),
            )
        };
        util::status_result("AAArchiveStreamWriteHeader", status)
    }

    /// Wraps `AAArchiveStreamWriteBlob`.
    pub fn write_blob(&mut self, key: FieldKey, buffer: &[u8]) -> Result<()> {
        self.ensure_open()?;
        let status = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_write_blob(
                self.as_ptr(),
                key.raw(),
                buffer.as_ptr(),
                buffer.len(),
            )
        };
        util::status_result("AAArchiveStreamWriteBlob", status)
    }

    /// Wraps `AAArchiveStreamReadHeader`.
    pub fn read_header(&mut self) -> Result<Option<Header>> {
        self.ensure_open()?;
        let mut status = 0_i32;
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_read_header_new(
                self.as_ptr(),
                &mut status,
            )
        };
        match status {
            1 => Ok(Some(Header::from_handle(
                handle,
                "AAArchiveStreamReadHeader",
            )?)),
            0 => Ok(None),
            code => Err(CompressionError::OperationFailed {
                operation: "AAArchiveStreamReadHeader",
                code,
            }),
        }
    }

    /// Wraps `AAArchiveStreamReadHeader`.
    pub fn read_header_into(&mut self, header: &mut Header) -> Result<bool> {
        self.ensure_open()?;
        match unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_read_header_into(
                self.as_ptr(),
                header.as_ptr(),
            )
        } {
            1 => Ok(true),
            0 => Ok(false),
            code => Err(CompressionError::OperationFailed {
                operation: "AAArchiveStreamReadHeader",
                code,
            }),
        }
    }

    /// Wraps `AAArchiveStreamReadBlob`.
    pub fn read_blob(&mut self, key: FieldKey, buffer: &mut [u8]) -> Result<()> {
        self.ensure_open()?;
        let status = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_read_blob(
                self.as_ptr(),
                key.raw(),
                buffer.as_mut_ptr(),
                buffer.len(),
            )
        };
        util::status_result("AAArchiveStreamReadBlob", status)
    }

    /// Wraps `AAArchiveStreamWritePathList`.
    pub fn write_path_list(
        &mut self,
        path_list: &PathList,
        key_set: &FieldKeySet,
        dir: &str,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<()> {
        self.ensure_open()?;
        let dir = util::cstring("dir", dir)?;
        let status = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_write_path_list(
                self.as_ptr(),
                path_list.as_ptr(),
                key_set.as_ptr(),
                dir.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        util::status_result("AAArchiveStreamWritePathList", status)
    }

    /// Wraps `AAArchiveStreamProcess`.
    pub fn process_into(
        &mut self,
        output: &mut Self,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<u64> {
        self.ensure_open()?;
        output.ensure_open()?;
        util::off_t_result("AAArchiveStreamProcess", unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_process(
                self.as_ptr(),
                output.as_ptr(),
                flags.bits(),
                n_threads,
            )
        })
    }

    /// Wraps `AAArchiveStreamClose`.
    pub fn cancel(&mut self) -> Result<()> {
        self.ensure_open()?;
        unsafe { ffi::aa_archive_stream::compression_rs_aa_archive_stream_cancel(self.as_ptr()) };
        Ok(())
    }

    #[deprecated(
        since = "0.2.2",
        note = "Use ArchiveStream::cancel; AAArchiveStreamAbort is a deprecated AppleArchive compatibility shim."
    )]
    /// Wraps `AAArchiveStreamClose`.
    pub fn abort(&mut self) -> Result<()> {
        self.ensure_open()?;
        unsafe { ffi::aa_archive_stream::compression_rs_aa_archive_stream_abort(self.as_ptr()) };
        Ok(())
    }

    /// Wraps `AAArchiveStreamClose`.
    pub fn close(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }
        let status = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_close(self.as_ptr())
        };
        self.closed = true;
        util::status_result("AAArchiveStreamClose", status)
    }
}

impl Drop for ArchiveStream {
    fn drop(&mut self) {
        unsafe { ffi::aa_archive_stream::compression_rs_aa_archive_stream_release(self.as_ptr()) };
    }
}

fn custom_archive_stream_error(operation: &'static str) -> CompressionError {
    CompressionError::OperationFailed {
        operation,
        code: -1,
    }
}

fn custom_archive_stream_code(error: &CompressionError) -> i32 {
    match error {
        CompressionError::OperationFailed { code, .. } if *code < 0 => *code,
        _ => -1,
    }
}

struct CustomArchiveStreamState {
    callbacks: Box<dyn CustomArchiveStreamCallbacks>,
}

/// Wraps callbacks installed by `AACustomArchiveStreamSet*Proc`.
pub trait CustomArchiveStreamCallbacks {
    /// Wraps `AAArchiveStreamWriteHeader`.
    fn write_header(&mut self, _header: &Header) -> Result<()> {
        Err(custom_archive_stream_error("AAArchiveStreamWriteHeader"))
    }

    /// Wraps `AAArchiveStreamWriteBlob`.
    fn write_blob(&mut self, _key: FieldKey, _buffer: &[u8]) -> Result<()> {
        Err(custom_archive_stream_error("AAArchiveStreamWriteBlob"))
    }

    /// Wraps `AAArchiveStreamReadHeader`.
    fn read_header(&mut self) -> Result<Option<Header>> {
        Err(custom_archive_stream_error("AAArchiveStreamReadHeader"))
    }

    /// Wraps `AAArchiveStreamReadBlob`.
    fn read_blob(&mut self, _key: FieldKey, _buffer: &mut [u8]) -> Result<()> {
        Err(custom_archive_stream_error("AAArchiveStreamReadBlob"))
    }

    /// Wraps the `cancel` convenience for `CustomArchiveStreamCallbacks`.
    fn cancel(&mut self) {}

    /// Wraps the `close` convenience for `CustomArchiveStreamCallbacks`.
    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}

unsafe fn custom_archive_stream_state(
    arg: *mut c_void,
) -> Option<&'static mut CustomArchiveStreamState> {
    if arg.is_null() {
        None
    } else {
        Some(unsafe { &mut *arg.cast::<CustomArchiveStreamState>() })
    }
}

unsafe fn custom_archive_stream_slice<'a>(
    buffer: *const c_void,
    length: usize,
) -> Option<&'a [u8]> {
    if length == 0 {
        Some(&[])
    } else if buffer.is_null() {
        None
    } else {
        Some(unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), length) })
    }
}

unsafe fn custom_archive_stream_slice_mut<'a>(
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

/// Run a user archive-stream callback body, converting a panic into the
/// failure status `-1` instead of letting it unwind across the C ABI (UB).
fn guard_archive_status(f: impl FnOnce() -> i32) -> i32 {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or(-1)
}

unsafe extern "C" fn custom_archive_stream_write_header(
    arg: *mut c_void,
    header: *mut c_void,
) -> i32 {
    let Some(state) = (unsafe { custom_archive_stream_state(arg) }) else {
        return -1;
    };
    if header.is_null() {
        return -1;
    }
    match Header::from_raw_clone(header, "AAHeaderClone") {
        Ok(header) => guard_archive_status(|| match state.callbacks.write_header(&header) {
            Ok(()) => 0,
            Err(error) => custom_archive_stream_code(&error),
        }),
        Err(error) => custom_archive_stream_code(&error),
    }
}

unsafe extern "C" fn custom_archive_stream_write_blob(
    arg: *mut c_void,
    key: u32,
    buffer: *const c_void,
    length: usize,
) -> i32 {
    let Some(state) = (unsafe { custom_archive_stream_state(arg) }) else {
        return -1;
    };
    let Some(buffer) = (unsafe { custom_archive_stream_slice(buffer, length) }) else {
        return -1;
    };
    guard_archive_status(|| match state.callbacks.write_blob(FieldKey::from_raw(key), buffer) {
        Ok(()) => 0,
        Err(error) => custom_archive_stream_code(&error),
    })
}

unsafe extern "C" fn custom_archive_stream_read_header(
    arg: *mut c_void,
    header_out: *mut *mut c_void,
) -> i32 {
    let Some(state) = (unsafe { custom_archive_stream_state(arg) }) else {
        return -1;
    };
    let Ok(header) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        state.callbacks.read_header()
    })) else {
        return -1;
    };
    match header {
        Ok(Some(header)) => {
            if header_out.is_null() {
                return -1;
            }
            let raw = match header.clone_raw() {
                Ok(raw) => raw,
                Err(error) => return custom_archive_stream_code(&error),
            };
            unsafe { *header_out = raw };
            1
        }
        Ok(None) => 0,
        Err(error) => custom_archive_stream_code(&error),
    }
}

unsafe extern "C" fn custom_archive_stream_read_blob(
    arg: *mut c_void,
    key: u32,
    buffer: *mut c_void,
    length: usize,
) -> i32 {
    let Some(state) = (unsafe { custom_archive_stream_state(arg) }) else {
        return -1;
    };
    let Some(buffer) = (unsafe { custom_archive_stream_slice_mut(buffer, length) }) else {
        return -1;
    };
    guard_archive_status(|| match state.callbacks.read_blob(FieldKey::from_raw(key), buffer) {
        Ok(()) => 0,
        Err(error) => custom_archive_stream_code(&error),
    })
}

unsafe extern "C" fn custom_archive_stream_cancel(arg: *mut c_void) {
    if let Some(state) = unsafe { custom_archive_stream_state(arg) } {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| state.callbacks.cancel()));
    }
}

unsafe extern "C" fn custom_archive_stream_close(arg: *mut c_void) -> i32 {
    if arg.is_null() {
        return 0;
    }
    let mut state = unsafe { Box::from_raw(arg.cast::<CustomArchiveStreamState>()) };
    guard_archive_status(|| match state.callbacks.close() {
        Ok(()) => 0,
        Err(error) => custom_archive_stream_code(&error),
    })
}

impl ArchiveStream {
    /// Wraps `AACustomArchiveStreamOpen`.
    pub fn custom<T: CustomArchiveStreamCallbacks + 'static>(callbacks: T) -> Result<Self> {
        let handle =
            unsafe { ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_open() };
        let stream = Self {
            handle: util::nonnull_handle(handle, "AACustomArchiveStreamOpen")?,
            _upstream: None,
            closed: false,
            _message_handler: None,
        };
        let state = Box::new(CustomArchiveStreamState {
            callbacks: Box::new(callbacks),
        });
        let data = Box::into_raw(state).cast::<c_void>();
        unsafe {
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_data(
                stream.as_ptr(),
                data,
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_write_header_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_write_header),
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_write_blob_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_write_blob),
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_read_header_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_read_header),
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_read_blob_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_read_blob),
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_cancel_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_cancel),
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_abort_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_cancel),
            );
            ffi::aa_archive_stream::compression_rs_aa_custom_archive_stream_set_close_proc(
                stream.as_ptr(),
                Some(custom_archive_stream_close),
            );
        }
        Ok(stream)
    }
}

/// Wraps payloads delivered through `AAEntryMessageProc`.
pub enum EntryMessageData {
    /// Wraps the `None` payload delivered by `AAEntryMessageProc`.
    None,
    /// Wraps the `Header` payload delivered by `AAEntryMessageProc`.
    Header(Header),
    /// Wraps the `EntryIds` payload delivered by `AAEntryMessageProc`.
    EntryIds {
        /// Wraps the `idx` field delivered by `AAEntryMessageProc`.
        idx: u64,
        /// Wraps the `idz` field delivered by `AAEntryMessageProc`.
        idz: u64,
    },
    /// Wraps the `Progress` payload delivered by `AAEntryMessageProc`.
    Progress {
        /// Wraps the `total` field delivered by `AAEntryMessageProc`.
        total: u64,
        /// Wraps the `current` field delivered by `AAEntryMessageProc`.
        current: u64,
    },
    /// Wraps the `Attributes` payload delivered by `AAEntryMessageProc`.
    Attributes(EntryAttributes),
    /// Wraps the `Xat` payload delivered by `AAEntryMessageProc`.
    Xat(EntryXatBlob),
    /// Wraps the `Acl` payload delivered by `AAEntryMessageProc`.
    Acl(EntryAclBlob),
}

impl std::fmt::Debug for EntryMessageData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Header(_) => f.write_str("Header(..)"),
            Self::EntryIds { idx, idz } => f
                .debug_struct("EntryIds")
                .field("idx", idx)
                .field("idz", idz)
                .finish(),
            Self::Progress { total, current } => f
                .debug_struct("Progress")
                .field("total", total)
                .field("current", current)
                .finish(),
            Self::Attributes(attributes) => f.debug_tuple("Attributes").field(attributes).finish(),
            Self::Xat(_) => f.write_str("Xat(..)"),
            Self::Acl(_) => f.write_str("Acl(..)"),
        }
    }
}

/// Wraps `AAEntryMessageProc` event data.
#[derive(Debug)]
pub struct EntryMessageEvent {
    /// Wraps the `message` field of `EntryMessageEvent`.
    pub message: EntryMessage,
    /// Wraps the `path` field of `EntryMessageEvent`.
    pub path: String,
    /// Wraps the `data` field of `EntryMessageEvent`.
    pub data: EntryMessageData,
}

/// Wraps handlers installed through `AAEntryMessageProc`.
pub trait EntryMessageHandler {
    /// Wraps the `handle` convenience for `EntryMessageHandler`.
    fn handle(&mut self, event: &mut EntryMessageEvent) -> Result<i32>;
}

impl<F> EntryMessageHandler for F
where
    F: FnMut(&mut EntryMessageEvent) -> Result<i32>,
{
    fn handle(&mut self, event: &mut EntryMessageEvent) -> Result<i32> {
        self(event)
    }
}

struct ArchiveMessageState {
    handler: Box<dyn EntryMessageHandler>,
}

impl std::fmt::Debug for ArchiveMessageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ArchiveMessageState(..)")
    }
}

fn archive_message_code(error: &CompressionError) -> i32 {
    match error {
        CompressionError::OperationFailed { code, .. } if *code < 0 => *code,
        _ => -1,
    }
}

unsafe fn archive_message_state(arg: *mut c_void) -> Option<&'static mut ArchiveMessageState> {
    if arg.is_null() {
        None
    } else {
        Some(unsafe { &mut *arg.cast::<ArchiveMessageState>() })
    }
}

unsafe fn archive_message_pair(data: *mut c_void) -> Option<(u64, u64)> {
    if data.is_null() {
        None
    } else {
        let values = unsafe { std::slice::from_raw_parts(data.cast::<u64>(), 2) };
        Some((values[0], values[1]))
    }
}

fn archive_message_data(message: EntryMessage, data: *mut c_void) -> Result<EntryMessageData> {
    match message {
        EntryMessage::SearchPruneDir
        | EntryMessage::SearchExclude
        | EntryMessage::SearchFail
        | EntryMessage::EncodeScanning => Ok(EntryMessageData::None),
        EntryMessage::ExtractBegin
        | EntryMessage::ConvertExclude
        | EntryMessage::ProcessExclude => {
            if data.is_null() {
                Ok(EntryMessageData::None)
            } else {
                Ok(EntryMessageData::Header(Header::from_raw_clone(
                    data,
                    "AAHeaderClone",
                )?))
            }
        }
        EntryMessage::ExtractEnd | EntryMessage::ExtractFail => {
            if let Some((idx, idz)) = unsafe { archive_message_pair(data) } {
                Ok(EntryMessageData::EntryIds { idx, idz })
            } else {
                Ok(EntryMessageData::None)
            }
        }
        EntryMessage::EncodeWriting | EntryMessage::DecodeReading => {
            if let Some((total, current)) = unsafe { archive_message_pair(data) } {
                Ok(EntryMessageData::Progress { total, current })
            } else {
                Ok(EntryMessageData::None)
            }
        }
        EntryMessage::ExtractAttributes => {
            if data.is_null() {
                Ok(EntryMessageData::None)
            } else {
                Ok(EntryMessageData::Attributes(unsafe {
                    *data.cast::<EntryAttributes>()
                }))
            }
        }
        EntryMessage::ExtractXat => {
            if data.is_null() {
                Ok(EntryMessageData::None)
            } else {
                Ok(EntryMessageData::Xat(EntryXatBlob::clone_from_raw(data)?))
            }
        }
        EntryMessage::ExtractAcl => {
            if data.is_null() {
                Ok(EntryMessageData::None)
            } else {
                Ok(EntryMessageData::Acl(EntryAclBlob::clone_from_raw(data)?))
            }
        }
    }
}

fn archive_message_sync(
    message: EntryMessage,
    data_ptr: *mut c_void,
    data: &EntryMessageData,
) -> Result<()> {
    match (message, data) {
        (EntryMessage::ExtractAttributes, EntryMessageData::Attributes(attributes)) => {
            if !data_ptr.is_null() {
                unsafe { *data_ptr.cast::<EntryAttributes>() = *attributes };
            }
            Ok(())
        }
        (EntryMessage::ExtractXat, EntryMessageData::Xat(xat)) => {
            if data_ptr.is_null() {
                Ok(())
            } else {
                EntryXatBlob::sync_into_raw(data_ptr, xat)
            }
        }
        (EntryMessage::ExtractAcl, EntryMessageData::Acl(acl)) => {
            if data_ptr.is_null() {
                Ok(())
            } else {
                EntryAclBlob::sync_into_raw(data_ptr, acl)
            }
        }
        _ => Ok(()),
    }
}

unsafe extern "C" fn archive_entry_message_proc(
    arg: *mut c_void,
    message_raw: u32,
    path: *const c_char,
    data: *mut c_void,
) -> i32 {
    let Some(state) = (unsafe { archive_message_state(arg) }) else {
        return -1;
    };
    let Some(message) = EntryMessage::from_raw(message_raw) else {
        return -1;
    };
    if path.is_null() {
        return -1;
    }
    let mut event = match archive_message_data(message, data) {
        Ok(event_data) => EntryMessageEvent {
            message,
            path: unsafe { CStr::from_ptr(path) }
                .to_string_lossy()
                .into_owned(),
            data: event_data,
        },
        Err(error) => return archive_message_code(&error),
    };
    match state.handler.handle(&mut event) {
        Ok(code) => match archive_message_sync(message, data, &event.data) {
            Ok(()) => code,
            Err(error) => archive_message_code(&error),
        },
        Err(error) => archive_message_code(&error),
    }
}

impl ArchiveStream {
    /// Wraps `AAExtractArchiveOutputStreamOpen`.
    pub fn extract_output_with_messages<T: EntryMessageHandler + 'static>(
        dir: &str,
        flags: ArchiveFlags,
        n_threads: i32,
        handler: T,
    ) -> Result<Self> {
        let dir = util::cstring("dir", dir)?;
        let mut message_handler = Box::new(ArchiveMessageState {
            handler: Box::new(handler),
        });
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_extract_archive_output_stream_open_with_messages(
                dir.as_ptr(),
                flags.bits(),
                n_threads,
                std::ptr::addr_of_mut!(*message_handler).cast::<c_void>(),
                Some(archive_entry_message_proc),
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAExtractArchiveOutputStreamOpen")?,
            _upstream: None,
            closed: false,
            _message_handler: Some(message_handler),
        })
    }

    /// Wraps `AAEncodeArchiveOutputStreamOpen`.
    pub fn encode_output_with_messages<T: EntryMessageHandler + 'static>(
        stream: ByteStream,
        flags: ArchiveFlags,
        n_threads: i32,
        handler: T,
    ) -> Result<Self> {
        let mut message_handler = Box::new(ArchiveMessageState {
            handler: Box::new(handler),
        });
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_encode_archive_output_stream_open_with_messages(
                stream.as_ptr(),
                flags.bits(),
                n_threads,
                std::ptr::addr_of_mut!(*message_handler).cast::<c_void>(),
                Some(archive_entry_message_proc),
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAEncodeArchiveOutputStreamOpen")?,
            _upstream: Some(ArchiveStreamUpstream::Byte(Box::new(stream))),
            closed: false,
            _message_handler: Some(message_handler),
        })
    }

    /// Wraps `AADecodeArchiveInputStreamOpen`.
    pub fn decode_input_with_messages<T: EntryMessageHandler + 'static>(
        stream: ByteStream,
        flags: ArchiveFlags,
        n_threads: i32,
        handler: T,
    ) -> Result<Self> {
        let mut message_handler = Box::new(ArchiveMessageState {
            handler: Box::new(handler),
        });
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_decode_archive_input_stream_open_with_messages(
                stream.as_ptr(),
                flags.bits(),
                n_threads,
                std::ptr::addr_of_mut!(*message_handler).cast::<c_void>(),
                Some(archive_entry_message_proc),
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AADecodeArchiveInputStreamOpen")?,
            _upstream: Some(ArchiveStreamUpstream::Byte(Box::new(stream))),
            closed: false,
            _message_handler: Some(message_handler),
        })
    }

    /// Wraps `AAConvertArchiveOutputStreamOpen`.
    pub fn convert_output_with_messages<T: EntryMessageHandler + 'static>(
        stream: ArchiveStream,
        insert_key_set: &FieldKeySet,
        remove_key_set: &FieldKeySet,
        flags: ArchiveFlags,
        n_threads: i32,
        handler: T,
    ) -> Result<Self> {
        let mut message_handler = Box::new(ArchiveMessageState {
            handler: Box::new(handler),
        });
        let handle = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_convert_archive_output_stream_open_with_messages(
                stream.as_ptr(),
                insert_key_set.as_ptr(),
                remove_key_set.as_ptr(),
                flags.bits(),
                n_threads,
                std::ptr::addr_of_mut!(*message_handler).cast::<c_void>(),
                Some(archive_entry_message_proc),
            )
        };
        Ok(Self {
            handle: util::nonnull_handle(handle, "AAConvertArchiveOutputStreamOpen")?,
            _upstream: Some(ArchiveStreamUpstream::Archive(Box::new(stream))),
            closed: false,
            _message_handler: Some(message_handler),
        })
    }

    /// Wraps `AAArchiveStreamWritePathList`.
    pub fn write_path_list_with_messages<T: EntryMessageHandler + 'static>(
        &mut self,
        path_list: &PathList,
        key_set: &FieldKeySet,
        dir: &str,
        flags: ArchiveFlags,
        n_threads: i32,
        handler: T,
    ) -> Result<()> {
        self.ensure_open()?;
        let dir = util::cstring("dir", dir)?;
        let mut message_handler = Box::new(ArchiveMessageState {
            handler: Box::new(handler),
        });
        let status = unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_write_path_list_with_messages(
                self.as_ptr(),
                path_list.as_ptr(),
                key_set.as_ptr(),
                dir.as_ptr(),
                flags.bits(),
                n_threads,
                std::ptr::addr_of_mut!(*message_handler).cast::<c_void>(),
                Some(archive_entry_message_proc),
            )
        };
        util::status_result("AAArchiveStreamWritePathList", status)
    }

    /// Wraps `AAArchiveStreamProcess`.
    pub fn process_into_with_messages<T: EntryMessageHandler + 'static>(
        &mut self,
        output: &mut Self,
        flags: ArchiveFlags,
        n_threads: i32,
        handler: T,
    ) -> Result<u64> {
        self.ensure_open()?;
        output.ensure_open()?;
        let mut message_handler = Box::new(ArchiveMessageState {
            handler: Box::new(handler),
        });
        util::off_t_result("AAArchiveStreamProcess", unsafe {
            ffi::aa_archive_stream::compression_rs_aa_archive_stream_process_with_messages(
                self.as_ptr(),
                output.as_ptr(),
                flags.bits(),
                n_threads,
                std::ptr::addr_of_mut!(*message_handler).cast::<c_void>(),
                Some(archive_entry_message_proc),
            )
        })
    }
}
