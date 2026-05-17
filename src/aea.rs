use crate::{
    aa_byte_stream::{ArchiveCompressionAlgorithm, ArchiveFlags, ByteStream, ByteStreamUpstream},
    aa_entry_blob::NamedBlobEntry,
    ffi, util, CompressionError, Result,
};
use std::ffi::{c_void, CStr};
use std::ptr::{null, null_mut, NonNull};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaProfile {
    HkdfSha256HmacNoneEcdsaP256 = 0,
    HkdfSha256AesctrHmacSymmetricNone = 1,
    HkdfSha256AesctrHmacSymmetricEcdsaP256 = 2,
    HkdfSha256AesctrHmacEcdheP256None = 3,
    HkdfSha256AesctrHmacEcdheP256EcdsaP256 = 4,
    HkdfSha256AesctrHmacScryptNone = 5,
}

impl AeaProfile {
    const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            x if x == Self::HkdfSha256HmacNoneEcdsaP256 as u32 => {
                Some(Self::HkdfSha256HmacNoneEcdsaP256)
            }
            x if x == Self::HkdfSha256AesctrHmacSymmetricNone as u32 => {
                Some(Self::HkdfSha256AesctrHmacSymmetricNone)
            }
            x if x == Self::HkdfSha256AesctrHmacSymmetricEcdsaP256 as u32 => {
                Some(Self::HkdfSha256AesctrHmacSymmetricEcdsaP256)
            }
            x if x == Self::HkdfSha256AesctrHmacEcdheP256None as u32 => {
                Some(Self::HkdfSha256AesctrHmacEcdheP256None)
            }
            x if x == Self::HkdfSha256AesctrHmacEcdheP256EcdsaP256 as u32 => {
                Some(Self::HkdfSha256AesctrHmacEcdheP256EcdsaP256)
            }
            x if x == Self::HkdfSha256AesctrHmacScryptNone as u32 => {
                Some(Self::HkdfSha256AesctrHmacScryptNone)
            }
            _ => None,
        }
    }

    pub const fn raw(self) -> u32 {
        self as u32
    }

    pub const fn ciphersuite(self) -> AeaCiphersuite {
        match self {
            Self::HkdfSha256HmacNoneEcdsaP256 => AeaCiphersuite::HkdfSha256Hmac,
            Self::HkdfSha256AesctrHmacSymmetricNone
            | Self::HkdfSha256AesctrHmacSymmetricEcdsaP256
            | Self::HkdfSha256AesctrHmacEcdheP256None
            | Self::HkdfSha256AesctrHmacEcdheP256EcdsaP256
            | Self::HkdfSha256AesctrHmacScryptNone => AeaCiphersuite::HkdfSha256AesctrHmac,
        }
    }

    pub const fn signature_mode(self) -> AeaSignatureMode {
        match self {
            Self::HkdfSha256HmacNoneEcdsaP256
            | Self::HkdfSha256AesctrHmacSymmetricEcdsaP256
            | Self::HkdfSha256AesctrHmacEcdheP256EcdsaP256 => AeaSignatureMode::EcdsaP256,
            Self::HkdfSha256AesctrHmacSymmetricNone
            | Self::HkdfSha256AesctrHmacEcdheP256None
            | Self::HkdfSha256AesctrHmacScryptNone => AeaSignatureMode::None,
        }
    }

    pub const fn encryption_mode(self) -> AeaEncryptionMode {
        match self {
            Self::HkdfSha256HmacNoneEcdsaP256 => AeaEncryptionMode::None,
            Self::HkdfSha256AesctrHmacSymmetricNone
            | Self::HkdfSha256AesctrHmacSymmetricEcdsaP256 => AeaEncryptionMode::Symmetric,
            Self::HkdfSha256AesctrHmacEcdheP256None
            | Self::HkdfSha256AesctrHmacEcdheP256EcdsaP256 => AeaEncryptionMode::EcdheP256,
            Self::HkdfSha256AesctrHmacScryptNone => AeaEncryptionMode::Scrypt,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaContextField {
    Profile = 0,
    PaddingSize = 1,
    ChecksumMode = 2,
    CompressionAlgorithm = 3,
    CompressionBlockSize = 4,
    AuthData = 5,
    MainKey = 6,
    SigningPublicKey = 7,
    SigningPrivateKey = 8,
    SymmetricKey = 9,
    RecipientPublicKey = 10,
    RecipientPrivateKey = 11,
    SignatureEncryptionKey = 12,
    RawSize = 13,
    ContainerSize = 14,
    BlocksPerCluster = 17,
    ArchiveIdentifier = 18,
    Password = 19,
}

impl AeaContextField {
    pub const fn raw(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaContextFieldRepresentation {
    Raw = 0,
    X963 = 1,
    Generate = 2,
}

impl AeaContextFieldRepresentation {
    const fn raw(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaCiphersuite {
    HkdfSha256Hmac = 0,
    HkdfSha256AesctrHmac = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaSignatureMode {
    None = 0,
    EcdsaP256 = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaEncryptionMode {
    None = 0,
    Symmetric = 1,
    EcdheP256 = 2,
    Scrypt = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaChecksumMode {
    None = 0,
    MurmurHash64 = 1,
    Sha256 = 2,
}

impl AeaChecksumMode {
    const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            x if x == Self::None as u32 => Some(Self::None),
            x if x == Self::MurmurHash64 as u32 => Some(Self::MurmurHash64),
            x if x == Self::Sha256 as u32 => Some(Self::Sha256),
            _ => None,
        }
    }

    const fn raw(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct AeaPadding;

impl AeaPadding {
    pub const NONE: u64 = 0;
    pub const ADAPTIVE: u64 = 1;
    pub const MIN_SIZE: u64 = 16;
}

#[derive(Debug)]
pub struct AeaContext {
    handle: NonNull<c_void>,
}

impl AeaContext {
    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        Ok(Self {
            handle: util::nonnull_handle(handle, operation)?,
        })
    }

    fn open_byte_stream(
        handle: *mut c_void,
        operation: &'static str,
        upstream: ByteStream,
    ) -> Result<ByteStream> {
        ByteStream::from_handle_with_upstream(
            handle,
            operation,
            Some(ByteStreamUpstream::Stream(Box::new(upstream))),
        )
    }

    pub fn with_profile(profile: AeaProfile) -> Result<Self> {
        let handle =
            unsafe { ffi::aea::compression_rs_aea_context_create_with_profile(profile.raw()) };
        Self::from_handle(handle, "AEAContextCreateWithProfile")
    }

    pub fn from_encrypted_stream(stream: &mut ByteStream) -> Result<Self> {
        let handle = unsafe {
            ffi::aea::compression_rs_aea_context_create_with_encrypted_stream(stream.as_ptr())
        };
        Self::from_handle(handle, "AEAContextCreateWithEncryptedStream")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn field_uint(&self, field: AeaContextField) -> Result<u64> {
        let value = unsafe {
            ffi::aea::compression_rs_aea_context_get_field_uint(self.as_ptr(), field.raw())
        };
        if value == u64::MAX {
            Err(CompressionError::OperationFailed {
                operation: "AEAContextGetFieldUInt",
                code: -1,
            })
        } else {
            Ok(value)
        }
    }

    pub fn field_blob(
        &self,
        field: AeaContextField,
        representation: AeaContextFieldRepresentation,
    ) -> Result<Vec<u8>> {
        let mut size = 0_usize;
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_get_field_blob(
                self.as_ptr(),
                field.raw(),
                representation.raw(),
                0,
                null_mut(),
                &mut size,
            )
        };
        util::status_result("AEAContextGetFieldBlob", status)?;

        let mut data = vec![0_u8; size];
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_get_field_blob(
                self.as_ptr(),
                field.raw(),
                representation.raw(),
                data.len(),
                if data.is_empty() {
                    null_mut()
                } else {
                    data.as_mut_ptr()
                },
                &mut size,
            )
        };
        util::status_result("AEAContextGetFieldBlob", status)?;
        Ok(data)
    }

    pub fn set_field_uint(&mut self, field: AeaContextField, value: u64) -> Result<()> {
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_set_field_uint(self.as_ptr(), field.raw(), value)
        };
        util::status_result("AEAContextSetFieldUInt", status)
    }

    pub fn set_field_blob(
        &mut self,
        field: AeaContextField,
        representation: AeaContextFieldRepresentation,
        value: &[u8],
    ) -> Result<()> {
        let value_ptr = if value.is_empty() {
            null()
        } else {
            value.as_ptr()
        };
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_set_field_blob(
                self.as_ptr(),
                field.raw(),
                representation.raw(),
                value_ptr,
                value.len(),
            )
        };
        util::status_result("AEAContextSetFieldBlob", status)
    }

    pub fn generate_field_blob(&mut self, field: AeaContextField) -> Result<()> {
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_generate_field_blob(self.as_ptr(), field.raw())
        };
        util::status_result("AEAContextGenerateFieldBlob", status)
    }

    pub fn decrypt_attributes(&mut self) -> Result<()> {
        let status =
            unsafe { ffi::aea::compression_rs_aea_context_decrypt_attributes(self.as_ptr()) };
        util::status_result("AEAContextDecryptAttributes", status)
    }

    pub fn profile(&self) -> Result<AeaProfile> {
        let raw = u32::try_from(self.field_uint(AeaContextField::Profile)?).unwrap_or(u32::MAX);
        AeaProfile::from_raw(raw).ok_or_else(|| CompressionError::OperationFailed {
            operation: "AEAContextGetProfile",
            code: i32::try_from(raw).unwrap_or(i32::MAX),
        })
    }

    pub fn checksum_mode(&self) -> Result<AeaChecksumMode> {
        let raw =
            u32::try_from(self.field_uint(AeaContextField::ChecksumMode)?).unwrap_or(u32::MAX);
        AeaChecksumMode::from_raw(raw).ok_or_else(|| CompressionError::OperationFailed {
            operation: "AEAContextGetChecksumMode",
            code: i32::try_from(raw).unwrap_or(i32::MAX),
        })
    }

    pub fn compression_algorithm(&self) -> Result<ArchiveCompressionAlgorithm> {
        let raw = u32::try_from(self.field_uint(AeaContextField::CompressionAlgorithm)?)
            .unwrap_or(u32::MAX);
        ArchiveCompressionAlgorithm::from_raw(raw).ok_or_else(|| {
            CompressionError::OperationFailed {
                operation: "AEAContextGetCompressionAlgorithm",
                code: i32::try_from(raw).unwrap_or(i32::MAX),
            }
        })
    }

    pub fn compression_block_size(&self) -> Result<usize> {
        usize::try_from(self.field_uint(AeaContextField::CompressionBlockSize)?).map_err(|_| {
            CompressionError::OperationFailed {
                operation: "AEAContextGetCompressionBlockSize",
                code: i32::MAX,
            }
        })
    }

    pub fn raw_size(&self) -> Result<u64> {
        self.field_uint(AeaContextField::RawSize)
    }

    pub fn container_size(&self) -> Result<u64> {
        self.field_uint(AeaContextField::ContainerSize)
    }

    pub fn auth_data(&self) -> Result<Vec<u8>> {
        self.field_blob(
            AeaContextField::AuthData,
            AeaContextFieldRepresentation::Raw,
        )
    }

    pub fn signature_encryption_key(&self) -> Result<Vec<u8>> {
        self.field_blob(
            AeaContextField::SignatureEncryptionKey,
            AeaContextFieldRepresentation::Raw,
        )
    }

    pub fn archive_identifier(&self) -> Result<Vec<u8>> {
        self.field_blob(
            AeaContextField::ArchiveIdentifier,
            AeaContextFieldRepresentation::Raw,
        )
    }

    pub fn main_key(&self) -> Result<Vec<u8>> {
        self.field_blob(AeaContextField::MainKey, AeaContextFieldRepresentation::Raw)
    }

    pub fn set_compression_algorithm(
        &mut self,
        compression_algorithm: ArchiveCompressionAlgorithm,
    ) -> Result<()> {
        self.set_field_uint(
            AeaContextField::CompressionAlgorithm,
            u64::from(compression_algorithm.as_raw()),
        )
    }

    pub fn set_compression_block_size(&mut self, compression_block_size: usize) -> Result<()> {
        self.set_field_uint(
            AeaContextField::CompressionBlockSize,
            u64::try_from(compression_block_size).unwrap_or(u64::MAX),
        )
    }

    pub fn set_checksum_mode(&mut self, checksum_mode: AeaChecksumMode) -> Result<()> {
        self.set_field_uint(
            AeaContextField::ChecksumMode,
            u64::from(checksum_mode.raw()),
        )
    }

    pub fn set_padding_size(&mut self, padding_size: u64) -> Result<()> {
        self.set_field_uint(AeaContextField::PaddingSize, padding_size)
    }

    pub fn set_auth_data_bytes(&mut self, auth_data: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::AuthData,
            AeaContextFieldRepresentation::Raw,
            auth_data,
        )
    }

    pub fn set_auth_data_blob(&mut self, auth_data: &AeaAuthData) -> Result<()> {
        self.set_auth_data_bytes(&auth_data.encoded_data()?)
    }

    pub fn set_signature_encryption_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SignatureEncryptionKey,
            AeaContextFieldRepresentation::Raw,
            key,
        )
    }

    pub fn set_symmetric_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SymmetricKey,
            AeaContextFieldRepresentation::Raw,
            key,
        )
    }

    pub fn set_password(&mut self, password: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::Password,
            AeaContextFieldRepresentation::Raw,
            password,
        )
    }

    pub fn set_signing_public_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SigningPublicKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    pub fn set_signing_private_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SigningPrivateKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    pub fn set_recipient_public_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::RecipientPublicKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    pub fn set_recipient_private_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::RecipientPrivateKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    pub fn set_main_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::MainKey,
            AeaContextFieldRepresentation::Raw,
            key,
        )
    }

    pub fn encryption_output_stream(
        &self,
        encrypted_stream: ByteStream,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<ByteStream> {
        let handle = unsafe {
            ffi::aea::compression_rs_aea_encryption_output_stream_open(
                encrypted_stream.as_ptr(),
                self.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Self::open_byte_stream(handle, "AEAEncryptionOutputStreamOpen", encrypted_stream)
    }

    pub fn encryption_output_stream_existing(
        &self,
        encrypted_stream: ByteStream,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<ByteStream> {
        let handle = unsafe {
            ffi::aea::compression_rs_aea_encryption_output_stream_open_existing(
                encrypted_stream.as_ptr(),
                self.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Self::open_byte_stream(
            handle,
            "AEAEncryptionOutputStreamOpenExisting",
            encrypted_stream,
        )
    }

    pub fn decryption_input_stream(
        &mut self,
        encrypted_stream: ByteStream,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<ByteStream> {
        let handle = unsafe {
            ffi::aea::compression_rs_aea_decryption_input_stream_open(
                encrypted_stream.as_ptr(),
                self.as_ptr(),
                flags.bits(),
                n_threads,
            )
        };
        Self::open_byte_stream(handle, "AEADecryptionInputStreamOpen", encrypted_stream)
    }

    pub fn decryption_random_access_input_stream(
        &mut self,
        encrypted_stream: ByteStream,
        alloc_limit: usize,
        flags: ArchiveFlags,
        n_threads: i32,
    ) -> Result<ByteStream> {
        let handle = unsafe {
            ffi::aea::compression_rs_aea_decryption_random_access_input_stream_open(
                encrypted_stream.as_ptr(),
                self.as_ptr(),
                alloc_limit,
                flags.bits(),
                n_threads,
            )
        };
        Self::open_byte_stream(
            handle,
            "AEADecryptionRandomAccessInputStreamOpen",
            encrypted_stream,
        )
    }

    pub fn close_encryption_output_stream(&mut self, stream: &mut ByteStream) -> Result<()> {
        let status = unsafe {
            ffi::aea::compression_rs_aea_encryption_output_stream_close_and_update_context(
                stream.as_ptr(),
                self.as_ptr(),
            )
        };
        util::status_result("AEAEncryptionOutputStreamCloseAndUpdateContext", status)?;
        stream.mark_closed();
        Ok(())
    }

    pub fn sign_stream(&self, stream: &mut ByteStream) -> Result<()> {
        let status =
            unsafe { ffi::aea::compression_rs_aea_stream_sign(stream.as_ptr(), self.as_ptr()) };
        util::status_result("AEAStreamSign", status)
    }
}

impl Drop for AeaContext {
    fn drop(&mut self) {
        unsafe { ffi::aea::compression_rs_aea_context_release(self.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct AeaAuthData {
    handle: NonNull<c_void>,
}

impl AeaAuthData {
    fn from_handle(handle: *mut c_void, operation: &'static str) -> Result<Self> {
        Ok(Self {
            handle: util::nonnull_handle(handle, operation)?,
        })
    }

    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aea::compression_rs_aea_auth_data_create() };
        Self::from_handle(handle, "AEAAuthDataCreate")
    }

    pub fn from_context(context: &AeaContext) -> Result<Self> {
        let handle =
            unsafe { ffi::aea::compression_rs_aea_auth_data_create_with_context(context.as_ptr()) };
        Self::from_handle(handle, "AEAAuthDataCreateWithContext")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn entry_count(&self) -> u32 {
        unsafe { ffi::aea::compression_rs_aea_auth_data_get_entry_count(self.as_ptr()) }
    }

    pub fn is_empty(&self) -> bool {
        self.entry_count() == 0
    }

    pub fn entry(&self, index: u32) -> Result<NamedBlobEntry> {
        let mut key_length = 0_usize;
        let mut data_size = 0_usize;
        let status = unsafe {
            ffi::aea::compression_rs_aea_auth_data_get_entry(
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
        util::status_result("AEAAuthDataGetEntry", status)?;

        let mut key = vec![0_i8; key_length.saturating_add(1)];
        let mut value = vec![0_u8; data_size];
        let status = unsafe {
            ffi::aea::compression_rs_aea_auth_data_get_entry(
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
        util::status_result("AEAAuthDataGetEntry", status)?;

        let key = unsafe { CStr::from_ptr(key.as_ptr()) }
            .to_str()
            .map_err(|_| CompressionError::Utf8Error {
                operation: "AEAAuthDataGetEntry",
            })?
            .to_string();

        Ok(NamedBlobEntry { key, value })
    }

    pub fn entries(&self) -> Result<Vec<NamedBlobEntry>> {
        (0..self.entry_count())
            .map(|index| self.entry(index))
            .collect()
    }

    pub fn append_entry(&mut self, entry: &NamedBlobEntry) -> Result<()> {
        let key = util::cstring("key", &entry.key)?;
        let data_ptr = if entry.value.is_empty() {
            null()
        } else {
            entry.value.as_ptr()
        };
        let status = unsafe {
            ffi::aea::compression_rs_aea_auth_data_append_entry(
                self.as_ptr(),
                key.as_ptr(),
                data_ptr,
                entry.value.len(),
            )
        };
        util::status_result("AEAAuthDataAppendEntry", status)
    }

    pub fn set_entry(&mut self, index: u32, entry: &NamedBlobEntry) -> Result<()> {
        let key = util::cstring("key", &entry.key)?;
        let data_ptr = if entry.value.is_empty() {
            null()
        } else {
            entry.value.as_ptr()
        };
        let status = unsafe {
            ffi::aea::compression_rs_aea_auth_data_set_entry(
                self.as_ptr(),
                index,
                key.as_ptr(),
                data_ptr,
                entry.value.len(),
            )
        };
        util::status_result("AEAAuthDataSetEntry", status)
    }

    pub fn clear(&mut self) -> Result<()> {
        let status = unsafe { ffi::aea::compression_rs_aea_auth_data_clear(self.as_ptr()) };
        util::status_result("AEAAuthDataClear", status)
    }

    pub fn remove_entry(&mut self, index: u32) -> Result<()> {
        let status =
            unsafe { ffi::aea::compression_rs_aea_auth_data_remove_entry(self.as_ptr(), index) };
        util::status_result("AEAAuthDataRemoveEntry", status)
    }

    pub fn encoded_size(&self) -> usize {
        unsafe { ffi::aea::compression_rs_aea_auth_data_get_encoded_size(self.as_ptr()) }
    }

    pub fn encoded_data(&self) -> Result<Vec<u8>> {
        let size = self.encoded_size();
        let mut data = vec![0_u8; size];
        if size == 0 {
            return Ok(data);
        }
        let copied = unsafe {
            ffi::aea::compression_rs_aea_auth_data_copy_encoded_data(
                self.as_ptr(),
                data.as_mut_ptr(),
            )
        };
        if copied {
            Ok(data)
        } else {
            Err(CompressionError::OperationFailed {
                operation: "AEAAuthDataGetEncodedData",
                code: -1,
            })
        }
    }
}

impl Clone for AeaAuthData {
    fn clone(&self) -> Self {
        let mut clone = Self::new().expect("AEAAuthDataCreate returned null");
        for entry in self.entries().expect("AEAAuthDataGetEntry failed") {
            clone
                .append_entry(&entry)
                .expect("AEAAuthDataAppendEntry failed");
        }
        clone
    }
}

impl Drop for AeaAuthData {
    fn drop(&mut self) {
        unsafe { ffi::aea::compression_rs_aea_auth_data_release(self.as_ptr()) };
    }
}
