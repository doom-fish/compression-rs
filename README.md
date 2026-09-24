# compression-rs

Safe Rust bindings for Apple’s `Compression` and `AppleArchive` APIs on macOS.

`compression-rs` uses a Swift bridge in front of the C-only Apple SDK
surfaces. The default API is safe Rust over opaque Swift-owned handles, while
the original `compression.h` FFI remains available behind the `raw-ffi` feature.

## Requirements

- macOS 11 or later. `AeaAuthData` and
  `AeaContext::close_encryption_output_stream` need macOS 11.3;
  `ByteStream::into_random_access_decompression_input`,
  `ByteStream::process_random_access_into` and
  `AeaContext::generate_field_blob` need macOS 12; and
  `AeaContext::decrypt_attributes` needs macOS 13. On older systems they return
  an error.
- Xcode / Swift toolchain available on `PATH` (`build.rs` runs `swift build`)

## Installation

```toml
[dependencies]
compression-rs = "0.3"
```

Enable raw `compression.h` bindings when needed:

```toml
[dependencies]
compression-rs = { version = "0.3", features = ["raw-ffi"] }
```

## Covered areas

- `CompressionStream` via `CompressionStream`, `Encoder`, and `Decoder`
- one-shot `CompressionEncode` / `CompressionDecode` helpers
- `AAByteStream` file, fd, temp-file, shared-buffer, compression, and random-access APIs
- `AAArchiveStream` encode, decode, extract, convert, header, blob, and message-callback APIs
- `AAEntryACLBlob` and `AAEntryXATBlob` helper APIs
- custom `AACustomByteStream` and `AACustomArchiveStream` callback flows
- `AEAContext`, `AEAAuthData`, and `AEAStreams` symmetric archive helpers
- `AAFieldKey` / `AAFieldKeySet`
- `AAHeader`
- requested `AAEntryStream` coverage, mapped to `PathList`, `EntryMessage`, and `EntryAttributes`

See [COVERAGE.md](COVERAGE.md) for the requested-area mapping and
[COVERAGE_AUDIT.md](COVERAGE_AUDIT.md) for the full audited SDK surface.

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
- `compress` uses the stream API when available and retries one-shot encoding
  with a larger buffer for buffer-only algorithms.
- Decoding is bounded. `decompress`, `Decoder::new` and decode
  `CompressionStream`s stop with `CompressionError::OutputLimitExceeded` after
  `DEFAULT_MAX_OUTPUT_LEN` (64 MiB) of output; use
  `decompress_with_max_output_len`, `Decoder::with_max_output_len` or
  `CompressionStream::with_max_output_len` to choose another limit. Buffer-only
  algorithms are decoded once into a buffer of `max_output_len + 1` bytes.
- `ByteStream::read_to_end` takes a `max_len` and fails with
  `CompressionError::ReadLimitExceeded` once the stream yields more, so reading
  an AppleArchive decompression or AEA decryption stream is bounded too.
- `compression_decode_buffer` reports a destination that is too small as
  `OutputLimitExceeded` instead of silently truncating, and accepts inputs that
  decode to zero bytes.
- AppleArchive calls custom stream callbacks and entry message handlers from its
  worker threads, possibly concurrently. `CustomByteStreamCallbacks`,
  `CustomArchiveStreamCallbacks` and `EntryMessageHandler` therefore require
  `Send + Sync` and take `&self`; keep mutable state in a `Mutex` or atomics. A
  panic in a callback fails the AppleArchive operation instead of unwinding
  into it.
- AEA key material (`AeaContext::field_blob`, `main_key`,
  `signature_encryption_key`) is returned as `SecretBytes`, which redacts its
  `Debug` output and wipes its memory on drop.
- `raw-ffi` currently preserves the direct `compression.h` surface. AppleArchive
  stays behind the Swift bridge.
- AppleArchive does not define a concrete `AAEntryStream` type; this crate uses
  the entry-level `AAPathList`, `AAEntryMessage`, and `AAEntryAttributes` APIs
  to cover that requested area.
- Deprecated AppleArchive `*Abort*` compatibility shims remain available as
  deprecated Rust aliases for audited SDK parity; prefer the `cancel` APIs in
  new code.

## Examples

```bash
cargo run --example 01_roundtrip
cargo run --example 04_aa_archive_stream_roundtrip
cargo run --example 05_aa_byte_stream_pipeline
cargo run --example 09_aa_entry_blobs
cargo run --example 10_aea_roundtrip
cargo run --example 11_aa_custom_stream_callbacks
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
- `09_aa_entry_blobs`
- `10_aea_roundtrip`
- `11_aa_custom_stream_callbacks`

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
