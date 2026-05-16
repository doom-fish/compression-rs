mod support;

use compression::{
    ArchiveCompressionAlgorithm, ArchiveFlags, ByteStream, DEFAULT_FILE_MODE, OPEN_CREATE,
    OPEN_READ_ONLY, OPEN_TRUNCATE, OPEN_WRITE_ONLY,
};
use std::fs;
use support::{artifact_dir, path_string, pseudo_random_bytes};

#[test]
fn byte_stream_supports_random_access_file_io() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aa-byte-stream-file-io");
    let file_path = path_string(&artifact_dir.join("payload.bin"));

    let mut stream = ByteStream::open_with_path(
        &file_path,
        OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    stream.pwrite(b"abc", 0)?;
    stream.pwrite(b"XYZ", 3)?;
    stream.close()?;

    let mut stream = ByteStream::open_with_path(&file_path, OPEN_READ_ONLY, 0)?;
    let mut output = vec![0_u8; 6];
    assert_eq!(stream.read(&mut output)?, 6);
    assert_eq!(&output, b"abcXYZ");
    stream.close()?;
    Ok(())
}

#[test]
fn byte_stream_processes_compressed_output() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(16 * 1024);
    let artifact_dir = artifact_dir("aa-byte-stream-process");
    let plain_path = artifact_dir.join("plain.bin");
    let compressed_path = artifact_dir.join("payload.pbzx");
    fs::write(&plain_path, &input)?;

    let mut input_stream =
        ByteStream::open_with_path(&path_string(&plain_path), OPEN_READ_ONLY, 0)?;
    let compressed_stream = ByteStream::open_with_path(
        &path_string(&compressed_path),
        OPEN_WRITE_ONLY | OPEN_CREATE | OPEN_TRUNCATE,
        DEFAULT_FILE_MODE,
    )?;
    let mut compressed_stream = compressed_stream.into_compression_output(
        ArchiveCompressionAlgorithm::Lzfse,
        64 * 1024,
        ArchiveFlags::empty(),
        0,
    )?;
    input_stream.process_into(&mut compressed_stream)?;
    compressed_stream.close()?;

    let compressed_stream =
        ByteStream::open_with_path(&path_string(&compressed_path), OPEN_READ_ONLY, 0)?;
    let mut decompressed_stream =
        compressed_stream.into_decompression_input(ArchiveFlags::empty(), 0)?;
    let output = decompressed_stream.read_to_end()?;
    assert_eq!(output, input);
    decompressed_stream.close()?;
    Ok(())
}
