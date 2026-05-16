use crate::{
    aa_byte_stream::{ArchiveFlags, ByteStream},
    aa_entry_stream::PathList,
    aa_field_key::{FieldKey, FieldKeySet},
    aa_header::Header,
    ffi, util, CompressionError, Result,
};
use std::ffi::c_void;
use std::ptr::NonNull;

#[allow(dead_code)]
#[derive(Debug)]
enum ArchiveStreamUpstream {
    Byte(Box<ByteStream>),
    Archive(Box<ArchiveStream>),
}

#[derive(Debug)]
pub struct ArchiveStream {
    handle: NonNull<c_void>,
    _upstream: Option<ArchiveStreamUpstream>,
    closed: bool,
}

impl ArchiveStream {
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
        })
    }

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
        })
    }

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
        })
    }

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

    pub fn cancel(&mut self) -> Result<()> {
        self.ensure_open()?;
        unsafe { ffi::aa_archive_stream::compression_rs_aa_archive_stream_cancel(self.as_ptr()) };
        Ok(())
    }

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
