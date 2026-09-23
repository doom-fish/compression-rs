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

#[test]
fn shared_buffer_pipe_ends_work_on_different_threads() -> Result<(), Box<dyn std::error::Error>> {
    let (mut writer, mut reader) = ByteStream::shared_buffer_pipe(4096)?;
    let payload = pseudo_random_bytes(256 * 1024);
    let expected = payload.clone();
    let producer = std::thread::spawn(move || -> compression::Result<()> {
        writer.write_all(&payload)?;
        writer.close()
    });
    let received = reader.read_to_end()?;
    producer.join().expect("producer thread")?;
    assert_eq!(received, expected);
    Ok(())
}

#[test]
fn owned_fds_and_files_are_handed_to_the_stream() -> Result<(), Box<dyn std::error::Error>> {
    let artifact_dir = artifact_dir("aa-byte-stream-owned-fd");
    let path = artifact_dir.join("owned.bin");
    fs::write(&path, b"owned descriptor")?;

    let mut from_fd = ByteStream::from_fd(std::os::fd::OwnedFd::from(fs::File::open(&path)?))?;
    let mut buffer = vec![0_u8; 16];
    assert_eq!(from_fd.read(&mut buffer)?, 16);
    assert_eq!(&buffer, b"owned descriptor");
    from_fd.close()?;

    let mut from_file = ByteStream::from_file(fs::File::open(&path)?)?;
    assert_eq!(from_file.read_to_end()?, b"owned descriptor");
    from_file.close()?;
    Ok(())
}
