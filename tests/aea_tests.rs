mod support;

use compression::{
    AeaAuthData, AeaChecksumMode, AeaContext, AeaContextField, AeaContextFieldRepresentation,
    AeaEncryptionMode, AeaPadding, AeaProfile, AeaSignatureMode, ArchiveCompressionAlgorithm,
    ArchiveFlags, ByteStream, NamedBlobEntry, DEFAULT_FILE_MODE, OPEN_CREATE, OPEN_READ_ONLY,
    OPEN_READ_WRITE, OPEN_TRUNCATE,
};
use support::{artifact_dir, path_string, pseudo_random_bytes};

#[test]
#[allow(clippy::too_many_lines)]
fn aea_round_trips_and_exposes_context_helpers() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aea-roundtrip");
    let archive_path = path_string(&artifact_dir.join("payload.aea"));
    let payload = pseudo_random_bytes(48 * 1024);
    let extra = pseudo_random_bytes(4096);
    let combined = [payload.clone(), extra.clone()].concat();

    let mut context = AeaContext::with_profile(AeaProfile::HkdfSha256AesctrHmacSymmetricNone)?;
    assert_eq!(context.profile()?, AeaProfile::HkdfSha256AesctrHmacSymmetricNone);
    assert_eq!(context.profile()?.encryption_mode(), AeaEncryptionMode::Symmetric);
    assert_eq!(context.profile()?.signature_mode(), AeaSignatureMode::None);
    context.set_checksum_mode(AeaChecksumMode::Sha256)?;
    context.set_padding_size(AeaPadding::NONE)?;
    context.set_compression_algorithm(ArchiveCompressionAlgorithm::Lzfse)?;
    context.set_compression_block_size(32 * 1024)?;
    context.generate_field_blob(AeaContextField::SymmetricKey)?;
    assert_eq!(
        context.field_uint(AeaContextField::CompressionBlockSize)?,
        32_u64 * 1024
    );
    assert_eq!(
        context.compression_algorithm()?,
        ArchiveCompressionAlgorithm::Lzfse
    );
    assert_eq!(context.checksum_mode()?, AeaChecksumMode::Sha256);

    let mut auth_data = AeaAuthData::new()?;
    auth_data.append_entry(&NamedBlobEntry {
        key: "kind".to_string(),
        value: b"demo".to_vec(),
    })?;
    auth_data.append_entry(&NamedBlobEntry {
        key: "mode".to_string(),
        value: b"stream".to_vec(),
    })?;
    auth_data.set_entry(
        1,
        &NamedBlobEntry {
            key: "mode".to_string(),
            value: b"chunked".to_vec(),
        },
    )?;
    auth_data.remove_entry(0)?;
    assert_eq!(auth_data.entry_count(), 1);
    assert_eq!(auth_data.entry(0)?.key, "mode");
    let cloned_auth = auth_data.clone();
    context.set_auth_data_blob(&cloned_auth)?;
    let mut cleared_auth = cloned_auth.clone();
    cleared_auth.clear()?;
    assert!(cleared_auth.is_empty());

    let stream = ByteStream::open_with_path(
        &archive_path,
        OPEN_READ_WRITE | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut encrypted = context.encryption_output_stream(stream, ArchiveFlags::empty(), 0)?;
    encrypted.write_all(&payload)?;
    context.close_encryption_output_stream(&mut encrypted)?;
    assert_eq!(context.raw_size()?, payload.len() as u64);
    assert!(context.container_size()? > 0);
    assert!(!context.archive_identifier()?.is_empty());

    let symmetric_key =
        context.field_blob(AeaContextField::SymmetricKey, AeaContextFieldRepresentation::Raw)?;

    let mut append_source = ByteStream::open_with_path(&archive_path, OPEN_READ_WRITE, 0)?;
    let mut append_context = AeaContext::from_encrypted_stream(&mut append_source)?;
    append_context.set_symmetric_key(&symmetric_key)?;
    let mut append_output =
        append_context.encryption_output_stream_existing(append_source, ArchiveFlags::empty(), 0)?;
    append_output.write_all(&extra)?;
    append_context.close_encryption_output_stream(&mut append_output)?;
    assert_eq!(append_context.raw_size()?, combined.len() as u64);

    let mut decrypt_source = ByteStream::open_with_path(&archive_path, OPEN_READ_ONLY, 0)?;
    let mut decrypt_context = AeaContext::from_encrypted_stream(&mut decrypt_source)?;
    decrypt_context.set_symmetric_key(&symmetric_key)?;
    decrypt_context.decrypt_attributes()?;
    let extracted_auth = AeaAuthData::from_context(&decrypt_context)?;
    assert_eq!(extracted_auth.entry(0)?.value, b"chunked".to_vec());
    let mut decrypted =
        decrypt_context.decryption_input_stream(decrypt_source, ArchiveFlags::empty(), 0)?;
    assert_eq!(decrypted.read_to_end()?, combined);
    decrypted.close()?;

    let mut random_source = ByteStream::open_with_path(&archive_path, OPEN_READ_ONLY, 0)?;
    let mut random_context = AeaContext::from_encrypted_stream(&mut random_source)?;
    random_context.set_symmetric_key(&symmetric_key)?;
    let mut random_input = random_context.decryption_random_access_input_stream(
        random_source,
        usize::MAX,
        ArchiveFlags::empty(),
        0,
    )?;
    let mut tail = vec![0_u8; extra.len()];
    let offset = i64::try_from(payload.len())?;
    assert_eq!(random_input.pread(&mut tail, offset)?, extra.len());
    assert_eq!(tail, extra);
    random_input.close()?;

    let _ = AeaPadding::ADAPTIVE;
    let _ = AeaPadding::MIN_SIZE;
    let _ = AeaContextField::BlocksPerCluster;
    let _ = AeaContext::main_key as fn(&AeaContext) -> compression::Result<Vec<u8>>;
    let _ = AeaContext::signature_encryption_key as fn(&AeaContext) -> compression::Result<Vec<u8>>;
    let _ = AeaContext::set_main_key as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ = AeaContext::set_password as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ =
        AeaContext::set_signing_public_key as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ =
        AeaContext::set_signing_private_key as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ = AeaContext::set_recipient_public_key
        as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ = AeaContext::set_recipient_private_key
        as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ =
        AeaContext::set_signature_encryption_key as fn(&mut AeaContext, &[u8]) -> compression::Result<()>;
    let _ = AeaContext::sign_stream as fn(&AeaContext, &mut ByteStream) -> compression::Result<()>;

    Ok(())
}
