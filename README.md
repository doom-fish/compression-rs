# compression-rs

Safe Rust bindings for Apple’s `Compression` and `AppleArchive` APIs on macOS.

`compression-rs` 0.2.0 uses a Swift bridge in front of the C-only Apple SDK
surfaces. The default API is safe Rust over opaque Swift-owned handles, while
the original `compression.h` FFI remains available behind the `raw-ffi` feature.

## Requirements

- macOS
- Xcode / Swift toolchain available on `PATH` (`build.rs` runs `swift build`)

## Installation

```toml
[dependencies]
compression-rs = "0.2"
```

Enable raw `compression.h` bindings when needed:

```toml
[dependencies]
compression-rs = { version = "0.2", features = ["raw-ffi"] }
```

## Covered areas

- `CompressionStream` via `CompressionStream`, `Encoder`, and `Decoder`
- one-shot `CompressionEncode` / `CompressionDecode` helpers
- `AAByteStream` file, fd, temp-file, shared-buffer, compression, and random-access APIs
- `AAArchiveStream` encode, decode, extract, convert, header, and blob APIs
- `AAFieldKey` / `AAFieldKeySet`
- `AAHeader`
- requested `AAEntryStream` coverage, mapped to `PathList`, `EntryMessage`, and `EntryAttributes`

See [COVERAGE.md](COVERAGE.md) for the per-area SDK mapping and deferred surface.

## Quick start

```rust
use compression::{compress, decompress, Algorithm};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = b"doom fish doom fish doom fish";
    let compressed = compress(input, Algorithm::Lzfse)?;
    let round_trip = decompress(&compressed, Algorithm::Lzfse)?;

    assert_eq!(round_trip, input);
    Ok(())
}
```

## API notes

- `Algorithm::ALL` contains the stream-capable algorithms.
- `Algorithm::BUFFER_ALL` additionally includes buffer-only algorithms such as
  `Lz4Raw` and `Lzbitmap`.
- `compress` / `decompress` use stream APIs when available and automatically
  fall back to repeated one-shot buffer calls for buffer-only algorithms.
- `raw-ffi` currently preserves the direct `compression.h` surface. AppleArchive
  stays behind the Swift bridge.
- AppleArchive does not define a concrete `AAEntryStream` type; this crate uses
  the entry-level `AAPathList`, `AAEntryMessage`, and `AAEntryAttributes` APIs
  to cover that requested area.

## Examples

```bash
cargo run --example 01_roundtrip
cargo run --example 04_aa_archive_stream_roundtrip
cargo run --example 05_aa_byte_stream_pipeline
```

This release also includes examples and tests for all requested logical areas:

- `01_roundtrip`
- `02_compression_encode_one_shot`
- `03_compression_decode_one_shot`
- `04_aa_archive_stream_roundtrip`
- `05_aa_byte_stream_pipeline`
- `06_aa_entry_stream_path_list`
- `07_aa_field_key_set`
- `08_aa_header_roundtrip`

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
