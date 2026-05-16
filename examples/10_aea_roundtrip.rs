mod common;

use common::{artifact_dir, path_string, pseudo_random_bytes};
use compression::{
    AeaContext, AeaContextField, AeaContextFieldRepresentation, AeaPadding, AeaProfile,
    ArchiveCompressionAlgorithm, ArchiveFlags, ByteStream, DEFAULT_FILE_MODE, OPEN_CREATE,
    OPEN_READ_ONLY, OPEN_READ_WRITE, OPEN_TRUNCATE,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aea-roundtrip");
    let archive_path = path_string(&artifact_dir.join("example.aea"));
    let payload = pseudo_random_bytes(16 * 1024);

    let mut context = AeaContext::with_profile(AeaProfile::HkdfSha256AesctrHmacSymmetricNone)?;
    context.set_padding_size(AeaPadding::NONE)?;
    context.set_compression_algorithm(ArchiveCompressionAlgorithm::Lzfse)?;
    context.generate_field_blob(AeaContextField::SymmetricKey)?;

    let stream = ByteStream::open_with_path(
        &archive_path,
        OPEN_READ_WRITE | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut encrypted = context.encryption_output_stream(stream, ArchiveFlags::empty(), 0)?;
    encrypted.write_all(&payload)?;
    context.close_encryption_output_stream(&mut encrypted)?;

    let symmetric_key =
        context.field_blob(AeaContextField::SymmetricKey, AeaContextFieldRepresentation::Raw)?;
    let mut input = ByteStream::open_with_path(&archive_path, OPEN_READ_ONLY, 0)?;
    let mut decrypt_context = AeaContext::from_encrypted_stream(&mut input)?;
    decrypt_context.set_symmetric_key(&symmetric_key)?;
    let mut decrypted = decrypt_context.decryption_input_stream(input, ArchiveFlags::empty(), 0)?;
    assert_eq!(decrypted.read_to_end()?, payload);

    println!("raw_size={} container_size={}", context.raw_size()?, context.container_size()?);
    println!("✅ AppleEncryptedArchive roundtrip OK");
    Ok(())
}
