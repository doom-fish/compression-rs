mod common;

use common::{artifact_dir, path_string};
use compression::{
    ArchiveFlags, ArchiveStream, ByteStream, FieldKey, Header, DEFAULT_FILE_MODE, OPEN_CREATE,
    OPEN_READ_ONLY, OPEN_TRUNCATE, OPEN_WRITE_ONLY,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = b"hello from apple archive".to_vec();
    let artifact_dir = artifact_dir("aa-archive-stream");
    let archive_path = artifact_dir.join("sample.aar");
    let archive_path = path_string(&archive_path);

    let mut header = Header::new()?;
    let regular_file = u64::from(b'F');
    let data_len = u64::try_from(data.len())?;
    header.append_field_uint(FieldKey::TYP, regular_file)?;
    header.append_field_string(FieldKey::PAT, "greeting.txt")?;
    header.append_field_uint(FieldKey::SIZ, data_len)?;
    header.append_field_blob(FieldKey::DAT, data_len)?;

    let byte_stream = ByteStream::open_with_path(
        &archive_path,
        OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut archive = ArchiveStream::encode_output(byte_stream, ArchiveFlags::empty(), 0)?;
    archive.write_header(&header)?;
    archive.write_blob(FieldKey::DAT, &data)?;
    archive.close()?;

    let byte_stream = ByteStream::open_with_path(&archive_path, OPEN_READ_ONLY, 0)?;
    let mut archive = ArchiveStream::decode_input(byte_stream, ArchiveFlags::empty(), 0)?;
    let header = archive.read_header()?.expect("archive entry");
    assert_eq!(header.path()?.as_deref(), Some("greeting.txt"));
    let blob = header.blob_with_key(FieldKey::DAT)?.expect("blob field");
    let mut decoded = vec![0_u8; usize::try_from(blob.size)?];
    archive.read_blob(FieldKey::DAT, &mut decoded)?;
    assert_eq!(decoded, data);
    assert!(archive.read_header()?.is_none());
    archive.close()?;

    println!("archive path={archive_path}");
    println!("✅ AppleArchive round-trip OK");
    Ok(())
}
