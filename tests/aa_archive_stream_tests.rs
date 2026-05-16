mod support;

use compression::{
    ArchiveFlags, ArchiveStream, ByteStream, FieldKey, Header, DEFAULT_FILE_MODE, OPEN_CREATE,
    OPEN_READ_ONLY, OPEN_TRUNCATE, OPEN_WRITE_ONLY,
};
use support::{artifact_dir, path_string};

#[test]
fn archive_stream_round_trips_single_entry() -> Result<(), Box<dyn std::error::Error>> {
    let payload = b"hello archive tests".to_vec();
    let artifact_dir = artifact_dir("aa-archive-stream-test");
    let archive_path = path_string(&artifact_dir.join("payload.aar"));

    let mut header = Header::new()?;
    let regular_file = u64::from(b'F');
    let payload_len = u64::try_from(payload.len())?;
    header.append_field_uint(FieldKey::TYP, regular_file)?;
    header.append_field_string(FieldKey::PAT, "payload.txt")?;
    header.append_field_uint(FieldKey::SIZ, payload_len)?;
    header.append_field_blob(FieldKey::DAT, payload_len)?;

    let byte_stream = ByteStream::open_with_path(
        &archive_path,
        OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut archive = ArchiveStream::encode_output(byte_stream, ArchiveFlags::empty(), 0)?;
    archive.write_header(&header)?;
    archive.write_blob(FieldKey::DAT, &payload)?;
    archive.close()?;

    let byte_stream = ByteStream::open_with_path(&archive_path, OPEN_READ_ONLY, 0)?;
    let mut archive = ArchiveStream::decode_input(byte_stream, ArchiveFlags::empty(), 0)?;
    let mut header = Header::new()?;
    assert!(archive.read_header_into(&mut header)?);
    assert_eq!(header.path()?.as_deref(), Some("payload.txt"));
    let blob = header.blob_with_key(FieldKey::DAT)?.expect("blob field");
    let mut decoded = vec![0_u8; usize::try_from(blob.size)?];
    archive.read_blob(FieldKey::DAT, &mut decoded)?;
    assert_eq!(decoded, payload);
    assert!(!archive.read_header_into(&mut header)?);
    archive.close()?;
    Ok(())
}
