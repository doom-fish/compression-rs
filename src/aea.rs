use crate::{
    aa_byte_stream::{ArchiveCompressionAlgorithm, ArchiveFlags, ByteStream, ByteStreamUpstream},
    aa_entry_blob::NamedBlobEntry,
    ffi, util, CompressionError, Result,
};
use std::ffi::{c_void, CStr};
use std::ptr::{null, null_mut, NonNull};

/// Wraps AEA profile identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaProfile {
    /// Wraps the `HkdfSha256HmacNoneEcdsaP256` variant of `AeaProfile`.
    HkdfSha256HmacNoneEcdsaP256 = 0,
    /// Wraps the `HkdfSha256AesctrHmacSymmetricNone` variant of `AeaProfile`.
    HkdfSha256AesctrHmacSymmetricNone = 1,
    /// Wraps the `HkdfSha256AesctrHmacSymmetricEcdsaP256` variant of `AeaProfile`.
    HkdfSha256AesctrHmacSymmetricEcdsaP256 = 2,
    /// Wraps the `HkdfSha256AesctrHmacEcdheP256None` variant of `AeaProfile`.
    HkdfSha256AesctrHmacEcdheP256None = 3,
    /// Wraps the `HkdfSha256AesctrHmacEcdheP256EcdsaP256` variant of `AeaProfile`.
    HkdfSha256AesctrHmacEcdheP256EcdsaP256 = 4,
    /// Wraps the `HkdfSha256AesctrHmacScryptNone` variant of `AeaProfile`.
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

    /// Wraps raw AEA profile identifiers.
    pub const fn raw(self) -> u32 {
        self as u32
    }

    /// Wraps the ciphersuite encoded by an AEA profile.
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

    /// Wraps the signature mode encoded by an AEA profile.
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

    /// Wraps the encryption mode encoded by an AEA profile.
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

/// Wraps AEA context field identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaContextField {
    /// Wraps the `Profile` variant of `AeaContextField`.
    Profile = 0,
    /// Wraps the `PaddingSize` variant of `AeaContextField`.
    PaddingSize = 1,
    /// Wraps the `ChecksumMode` variant of `AeaContextField`.
    ChecksumMode = 2,
    /// Wraps the `CompressionAlgorithm` variant of `AeaContextField`.
    CompressionAlgorithm = 3,
    /// Wraps the `CompressionBlockSize` variant of `AeaContextField`.
    CompressionBlockSize = 4,
    /// Wraps the `AuthData` variant of `AeaContextField`.
    AuthData = 5,
    /// Wraps the `MainKey` variant of `AeaContextField`.
    MainKey = 6,
    /// Wraps the `SigningPublicKey` variant of `AeaContextField`.
    SigningPublicKey = 7,
    /// Wraps the `SigningPrivateKey` variant of `AeaContextField`.
    SigningPrivateKey = 8,
    /// Wraps the `SymmetricKey` variant of `AeaContextField`.
    SymmetricKey = 9,
    /// Wraps the `RecipientPublicKey` variant of `AeaContextField`.
    RecipientPublicKey = 10,
    /// Wraps the `RecipientPrivateKey` variant of `AeaContextField`.
    RecipientPrivateKey = 11,
    /// Wraps the `SignatureEncryptionKey` variant of `AeaContextField`.
    SignatureEncryptionKey = 12,
    /// Wraps the `RawSize` variant of `AeaContextField`.
    RawSize = 13,
    /// Wraps the `ContainerSize` variant of `AeaContextField`.
    ContainerSize = 14,
    /// Wraps the `BlocksPerCluster` variant of `AeaContextField`.
    BlocksPerCluster = 17,
    /// Wraps the `ArchiveIdentifier` variant of `AeaContextField`.
    ArchiveIdentifier = 18,
    /// Wraps the `Password` variant of `AeaContextField`.
    Password = 19,
}

impl AeaContextField {
    /// Wraps raw AEA context field identifiers.
    pub const fn raw(self) -> u32 {
        self as u32
    }
}

/// Wraps AEA context field representation identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaContextFieldRepresentation {
    /// Wraps the `Raw` variant of `AeaContextFieldRepresentation`.
    Raw = 0,
    /// Wraps the `X963` variant of `AeaContextFieldRepresentation`.
    X963 = 1,
    /// Wraps the `Generate` variant of `AeaContextFieldRepresentation`.
    Generate = 2,
}

impl AeaContextFieldRepresentation {
    const fn raw(self) -> u32 {
        self as u32
    }
}

/// Wraps AEA ciphersuite identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaCiphersuite {
    /// Wraps the `HkdfSha256Hmac` variant of `AeaCiphersuite`.
    HkdfSha256Hmac = 0,
    /// Wraps the `HkdfSha256AesctrHmac` variant of `AeaCiphersuite`.
    HkdfSha256AesctrHmac = 1,
}

/// Wraps AEA signature mode identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaSignatureMode {
    /// Wraps the `None` variant of `AeaSignatureMode`.
    None = 0,
    /// Wraps the `EcdsaP256` variant of `AeaSignatureMode`.
    EcdsaP256 = 1,
}

/// Wraps AEA encryption mode identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaEncryptionMode {
    /// Wraps the `None` variant of `AeaEncryptionMode`.
    None = 0,
    /// Wraps the `Symmetric` variant of `AeaEncryptionMode`.
    Symmetric = 1,
    /// Wraps the `EcdheP256` variant of `AeaEncryptionMode`.
    EcdheP256 = 2,
    /// Wraps the `Scrypt` variant of `AeaEncryptionMode`.
    Scrypt = 3,
}

/// Wraps AEA checksum mode identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AeaChecksumMode {
    /// Wraps the `None` variant of `AeaChecksumMode`.
    None = 0,
    /// Wraps the `MurmurHash64` variant of `AeaChecksumMode`.
    MurmurHash64 = 1,
    /// Wraps the `Sha256` variant of `AeaChecksumMode`.
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

/// Wraps AEA padding constants.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct AeaPadding;

impl AeaPadding {
    /// Wraps the `NONE` AEA padding constant.
    pub const NONE: u64 = 0;
    /// Wraps the `ADAPTIVE` AEA padding constant.
    pub const ADAPTIVE: u64 = 1;
    /// Wraps the `MIN_SIZE` AEA padding constant.
    pub const MIN_SIZE: u64 = 16;
}

/// Wraps an `AEAContext` handle.
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

    /// Wraps `AEAContextCreateWithProfile`.
    pub fn with_profile(profile: AeaProfile) -> Result<Self> {
        let handle =
            unsafe { ffi::aea::compression_rs_aea_context_create_with_profile(profile.raw()) };
        Self::from_handle(handle, "AEAContextCreateWithProfile")
    }

    /// Wraps `AEAContextCreateWithEncryptedStream`.
    pub fn from_encrypted_stream(stream: &mut ByteStream) -> Result<Self> {
        let handle = unsafe {
            ffi::aea::compression_rs_aea_context_create_with_encrypted_stream(stream.as_ptr())
        };
        Self::from_handle(handle, "AEAContextCreateWithEncryptedStream")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Wraps `AEAContextGetFieldUInt`.
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

    /// Wraps `AEAContextGetFieldBlob`.
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

    /// Wraps `AEAContextSetFieldUInt`.
    pub fn set_field_uint(&mut self, field: AeaContextField, value: u64) -> Result<()> {
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_set_field_uint(self.as_ptr(), field.raw(), value)
        };
        util::status_result("AEAContextSetFieldUInt", status)
    }

    /// Wraps `AEAContextSetFieldBlob`.
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

    /// Wraps `AEAContextGenerateFieldBlob`.
    pub fn generate_field_blob(&mut self, field: AeaContextField) -> Result<()> {
        let status = unsafe {
            ffi::aea::compression_rs_aea_context_generate_field_blob(self.as_ptr(), field.raw())
        };
        util::status_result("AEAContextGenerateFieldBlob", status)
    }

    /// Wraps `AEAContextDecryptAttributes`.
    pub fn decrypt_attributes(&mut self) -> Result<()> {
        let status =
            unsafe { ffi::aea::compression_rs_aea_context_decrypt_attributes(self.as_ptr()) };
        util::status_result("AEAContextDecryptAttributes", status)
    }

    /// Wraps `AEAContextGetProfile`.
    pub fn profile(&self) -> Result<AeaProfile> {
        let raw = u32::try_from(self.field_uint(AeaContextField::Profile)?).unwrap_or(u32::MAX);
        AeaProfile::from_raw(raw).ok_or_else(|| CompressionError::OperationFailed {
            operation: "AEAContextGetProfile",
            code: i32::try_from(raw).unwrap_or(i32::MAX),
        })
    }

    /// Wraps `AEAContextGetChecksumMode`.
    pub fn checksum_mode(&self) -> Result<AeaChecksumMode> {
        let raw =
            u32::try_from(self.field_uint(AeaContextField::ChecksumMode)?).unwrap_or(u32::MAX);
        AeaChecksumMode::from_raw(raw).ok_or_else(|| CompressionError::OperationFailed {
            operation: "AEAContextGetChecksumMode",
            code: i32::try_from(raw).unwrap_or(i32::MAX),
        })
    }

    /// Wraps `AEAContextGetCompressionAlgorithm`.
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

    /// Wraps `AEAContextGetCompressionBlockSize`.
    pub fn compression_block_size(&self) -> Result<usize> {
        usize::try_from(self.field_uint(AeaContextField::CompressionBlockSize)?).map_err(|_| {
            CompressionError::OperationFailed {
                operation: "AEAContextGetCompressionBlockSize",
                code: i32::MAX,
            }
        })
    }

    /// Wraps `AEAContextGetFieldUInt` for `RawSize`.
    pub fn raw_size(&self) -> Result<u64> {
        self.field_uint(AeaContextField::RawSize)
    }

    /// Wraps `AEAContextGetFieldUInt` for `ContainerSize`.
    pub fn container_size(&self) -> Result<u64> {
        self.field_uint(AeaContextField::ContainerSize)
    }

    /// Wraps the `auth_data` convenience for `AeaContext`.
    pub fn auth_data(&self) -> Result<Vec<u8>> {
        self.field_blob(
            AeaContextField::AuthData,
            AeaContextFieldRepresentation::Raw,
        )
    }

    /// Wraps the `signature_encryption_key` convenience for `AeaContext`.
    pub fn signature_encryption_key(&self) -> Result<Vec<u8>> {
        self.field_blob(
            AeaContextField::SignatureEncryptionKey,
            AeaContextFieldRepresentation::Raw,
        )
    }

    /// Wraps the `archive_identifier` convenience for `AeaContext`.
    pub fn archive_identifier(&self) -> Result<Vec<u8>> {
        self.field_blob(
            AeaContextField::ArchiveIdentifier,
            AeaContextFieldRepresentation::Raw,
        )
    }

    /// Wraps the `main_key` convenience for `AeaContext`.
    pub fn main_key(&self) -> Result<Vec<u8>> {
        self.field_blob(AeaContextField::MainKey, AeaContextFieldRepresentation::Raw)
    }

    /// Wraps the `set_compression_algorithm` convenience for `AeaContext`.
    pub fn set_compression_algorithm(
        &mut self,
        compression_algorithm: ArchiveCompressionAlgorithm,
    ) -> Result<()> {
        self.set_field_uint(
            AeaContextField::CompressionAlgorithm,
            u64::from(compression_algorithm.as_raw()),
        )
    }

    /// Wraps the `set_compression_block_size` convenience for `AeaContext`.
    pub fn set_compression_block_size(&mut self, compression_block_size: usize) -> Result<()> {
        self.set_field_uint(
            AeaContextField::CompressionBlockSize,
            u64::try_from(compression_block_size).unwrap_or(u64::MAX),
        )
    }

    /// Wraps the `set_checksum_mode` convenience for `AeaContext`.
    pub fn set_checksum_mode(&mut self, checksum_mode: AeaChecksumMode) -> Result<()> {
        self.set_field_uint(
            AeaContextField::ChecksumMode,
            u64::from(checksum_mode.raw()),
        )
    }

    /// Wraps the `set_padding_size` convenience for `AeaContext`.
    pub fn set_padding_size(&mut self, padding_size: u64) -> Result<()> {
        self.set_field_uint(AeaContextField::PaddingSize, padding_size)
    }

    /// Wraps the `set_auth_data_bytes` convenience for `AeaContext`.
    pub fn set_auth_data_bytes(&mut self, auth_data: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::AuthData,
            AeaContextFieldRepresentation::Raw,
            auth_data,
        )
    }

    /// Wraps the `set_auth_data_blob` convenience for `AeaContext`.
    pub fn set_auth_data_blob(&mut self, auth_data: &AeaAuthData) -> Result<()> {
        self.set_auth_data_bytes(&auth_data.encoded_data()?)
    }

    /// Wraps the `set_signature_encryption_key` convenience for `AeaContext`.
    pub fn set_signature_encryption_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SignatureEncryptionKey,
            AeaContextFieldRepresentation::Raw,
            key,
        )
    }

    /// Wraps the `set_symmetric_key` convenience for `AeaContext`.
    pub fn set_symmetric_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SymmetricKey,
            AeaContextFieldRepresentation::Raw,
            key,
        )
    }

    /// Wraps the `set_password` convenience for `AeaContext`.
    pub fn set_password(&mut self, password: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::Password,
            AeaContextFieldRepresentation::Raw,
            password,
        )
    }

    /// Wraps the `set_signing_public_key` convenience for `AeaContext`.
    pub fn set_signing_public_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SigningPublicKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    /// Wraps `AEAEncryptionOutputStreamOpen`.
    pub fn set_signing_private_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::SigningPrivateKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    /// Wraps `AEAEncryptionOutputStreamOpen`.
    pub fn set_recipient_public_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::RecipientPublicKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    /// Wraps `AEAEncryptionOutputStreamOpen`.
    pub fn set_recipient_private_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::RecipientPrivateKey,
            AeaContextFieldRepresentation::X963,
            key,
        )
    }

    /// Wraps `AEAEncryptionOutputStreamOpen`.
    pub fn set_main_key(&mut self, key: &[u8]) -> Result<()> {
        self.set_field_blob(
            AeaContextField::MainKey,
            AeaContextFieldRepresentation::Raw,
            key,
        )
    }

    /// Wraps `AEAEncryptionOutputStreamOpen`.
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

    /// Wraps `AEAEncryptionOutputStreamOpenExisting`.
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

    /// Wraps `AEADecryptionInputStreamOpen`.
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

    /// Wraps `AEADecryptionRandomAccessInputStreamOpen`.
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

    /// Wraps `AEAEncryptionOutputStreamCloseAndUpdateContext`.
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

    /// Wraps `AEAStreamSign`.
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

/// Wraps an `AEAAuthData` handle.
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

    /// Wraps `AEAAuthDataCreate`.
    pub fn new() -> Result<Self> {
        let handle = unsafe { ffi::aea::compression_rs_aea_auth_data_create() };
        Self::from_handle(handle, "AEAAuthDataCreate")
    }

    /// Wraps `AEAAuthDataCreateWithContext`.
    pub fn from_context(context: &AeaContext) -> Result<Self> {
        let handle =
            unsafe { ffi::aea::compression_rs_aea_auth_data_create_with_context(context.as_ptr()) };
        Self::from_handle(handle, "AEAAuthDataCreateWithContext")
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    /// Wraps `AEAAuthDataGetEntry`.
    pub fn entry_count(&self) -> u32 {
        unsafe { ffi::aea::compression_rs_aea_auth_data_get_entry_count(self.as_ptr()) }
    }

    /// Wraps `AEAAuthDataGetEntry`.
    pub fn is_empty(&self) -> bool {
        self.entry_count() == 0
    }

    /// Wraps `AEAAuthDataGetEntry`.
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

    /// Wraps `AEAAuthDataAppendEntry`.
    pub fn entries(&self) -> Result<Vec<NamedBlobEntry>> {
        (0..self.entry_count())
            .map(|index| self.entry(index))
            .collect()
    }

    /// Wraps `AEAAuthDataAppendEntry`.
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

    /// Wraps `AEAAuthDataSetEntry`.
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

    /// Wraps `AEAAuthDataClear`.
    pub fn clear(&mut self) -> Result<()> {
        let status = unsafe { ffi::aea::compression_rs_aea_auth_data_clear(self.as_ptr()) };
        util::status_result("AEAAuthDataClear", status)
    }

    /// Wraps `AEAAuthDataRemoveEntry`.
    pub fn remove_entry(&mut self, index: u32) -> Result<()> {
        let status =
            unsafe { ffi::aea::compression_rs_aea_auth_data_remove_entry(self.as_ptr(), index) };
        util::status_result("AEAAuthDataRemoveEntry", status)
    }

    /// Wraps `AEAAuthDataGetEncodedData`.
    pub fn encoded_size(&self) -> usize {
        unsafe { ffi::aea::compression_rs_aea_auth_data_get_encoded_size(self.as_ptr()) }
    }

    /// Wraps `AEAAuthDataGetEncodedData`.
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
