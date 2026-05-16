#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::use_self
)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
compile_error!("compression only supports macOS");

mod aa_archive_stream;
mod aa_byte_stream;
mod aa_entry_blob;
mod aa_entry_stream;
mod aa_field_key;
mod aa_header;
mod compression_decode;
mod compression_encode;
mod compression_stream;
mod aea;
mod error;
mod ffi;
#[cfg(feature = "raw-ffi")]
pub mod raw_ffi;
pub(crate) mod util;

pub use aa_archive_stream::{
    ArchiveStream, CustomArchiveStreamCallbacks, EntryMessageData, EntryMessageEvent,
    EntryMessageHandler,
};
pub use aa_byte_stream::{
    ArchiveCompressionAlgorithm, ArchiveFlags, ByteStream, CustomByteStreamCallbacks,
    DEFAULT_FILE_MODE, OPEN_CREATE,
    OPEN_READ_ONLY, OPEN_READ_WRITE, OPEN_TRUNCATE, OPEN_WRITE_ONLY,
};
pub use aa_entry_blob::{
    AccessControlEntry, AceFlagSet, AcePermSet, AceQualifierType, AceTag, EntryAclBlob,
    EntryXatBlob, NamedBlobEntry,
};
pub use aa_entry_stream::{EntryAttributes, EntryMessage, PathList};
pub use aa_field_key::{FieldKey, FieldKeySet};
pub use aa_header::{
    BlobDescription, EntryType, FieldType, HashFunction, HashValue, Header, HeaderFieldValue,
    Timespec,
};
pub use compression_decode::{
    compression_decode_buffer, compression_decode_scratch_buffer_size, decompress,
};
pub use compression_encode::{
    compress, compression_encode_buffer, compression_encode_scratch_buffer_size,
};
pub use compression_stream::{CompressionStream, Decoder, Encoder, StreamOperation};
pub use aea::{
    AeaAuthData, AeaChecksumMode, AeaCiphersuite, AeaContext, AeaContextField,
    AeaContextFieldRepresentation, AeaEncryptionMode, AeaPadding, AeaProfile,
    AeaSignatureMode,
};
pub use error::{CompressionError, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Algorithm {
    Lz4,
    Zlib,
    Lzma,
    Lz4Raw,
    Brotli,
    Lzfse,
    Lzbitmap,
}

impl Algorithm {
    pub const ALL: [Self; 5] = [Self::Lz4, Self::Zlib, Self::Lzma, Self::Brotli, Self::Lzfse];
    pub const BUFFER_ALL: [Self; 7] = [
        Self::Lz4,
        Self::Zlib,
        Self::Lzma,
        Self::Lz4Raw,
        Self::Brotli,
        Self::Lzfse,
        Self::Lzbitmap,
    ];

    pub const fn supports_streams(self) -> bool {
        !matches!(self, Self::Lz4Raw | Self::Lzbitmap)
    }

    pub(crate) const fn as_raw(self) -> u32 {
        match self {
            Self::Lz4 => 0x100,
            Self::Zlib => 0x205,
            Self::Lzma => 0x306,
            Self::Lz4Raw => 0x101,
            Self::Brotli => 0xB02,
            Self::Lzfse => 0x801,
            Self::Lzbitmap => 0x702,
        }
    }
}
