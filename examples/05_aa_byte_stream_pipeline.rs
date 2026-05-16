mod common;

use common::{artifact_dir, path_string, pseudo_random_bytes};
use compression::{
    ArchiveCompressionAlgorithm, ArchiveFlags, ByteStream, DEFAULT_FILE_MODE, OPEN_CREATE,
    OPEN_READ_ONLY, OPEN_TRUNCATE, OPEN_WRITE_ONLY,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = pseudo_random_bytes(32 * 1024);
    let artifact_dir = artifact_dir("aa-byte-stream");
    let plain_path = artifact_dir.join("payload.bin");
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

    let compressed_len = fs::metadata(&compressed_path)?.len();
    println!("compressed bytes={compressed_len}");
    println!("✅ AppleArchive byte-stream pipeline OK");
    Ok(())
}
