# Coverage

`compression-rs` 0.2.2 follows the Swift-bridge pattern used in
`screencapturekit-rs`: Rust owns opaque pointers to Swift box objects, and the
Swift layer owns the underlying C-only `Compression` / `AppleArchive` handles.

This document tracks **requested logical-area coverage for v0.2.2** and the
additional AppleArchive / AEA surface added in this patch release. Full symbol
coverage is tracked in `COVERAGE_AUDIT.md`.

## Requested logical areas

| Requested area | Apple SDK mapping | Rust module | Swift bridge | Example | Test | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `CompressionStream` | `compression_stream_init`, `compression_stream_process`, `compression_stream_destroy` | `src/compression_stream.rs` | `swift-bridge/Sources/CompressionBridge/CompressionStream.swift` | `examples/01_roundtrip.rs` | `tests/compression_stream_tests.rs` | Complete |
| `CompressionEncode` | `compression_encode_buffer`, `compression_encode_scratch_buffer_size` | `src/compression_encode.rs` | `swift-bridge/Sources/CompressionBridge/CompressionEncode.swift` | `examples/02_compression_encode_one_shot.rs` | `tests/compression_encode_tests.rs` | Complete |
| `CompressionDecode` | `compression_decode_buffer`, `compression_decode_scratch_buffer_size` | `src/compression_decode.rs` | `swift-bridge/Sources/CompressionBridge/CompressionDecode.swift` | `examples/03_compression_decode_one_shot.rs` | `tests/compression_decode_tests.rs` | Complete |
| `AAArchiveStream` | archive encode/decode/extract/convert streams plus header/blob/process helpers | `src/aa_archive_stream.rs` | `swift-bridge/Sources/CompressionBridge/AAArchiveStream.swift` | `examples/04_aa_archive_stream_roundtrip.rs` | `tests/aa_archive_stream_tests.rs` | Complete |
| `AAByteStream` | file/fd/temp/shared-buffer streams, compression/decompression streams, random-access processing | `src/aa_byte_stream.rs` | `swift-bridge/Sources/CompressionBridge/AAByteStream.swift` | `examples/05_aa_byte_stream_pipeline.rs` | `tests/aa_byte_stream_tests.rs` | Complete |
| `AAEntryStream` | no concrete SDK type exists; covered via `AAPathList`, `AAEntryMessage`, and `AAEntryAttributes` | `src/aa_entry_stream.rs` | `swift-bridge/Sources/CompressionBridge/AAEntryStream.swift` | `examples/06_aa_entry_stream_path_list.rs` | `tests/aa_entry_stream_tests.rs` | Complete for requested logical area |
| `AAFieldKey` | `AAFieldKey`, `AAFieldKeySet` | `src/aa_field_key.rs` | `swift-bridge/Sources/CompressionBridge/AAFieldKey.swift` | `examples/07_aa_field_key_set.rs` | `tests/aa_field_key_tests.rs` | Complete |
| `AAHeader` | `AAHeader` creation, cloning, assignment, getters, setters, encoded-data round-trip | `src/aa_header.rs` | `swift-bridge/Sources/CompressionBridge/AAHeader.swift` | `examples/08_aa_header_roundtrip.rs` | `tests/aa_header_tests.rs` | Complete |
| `AAEntryACLBlob` / `AAEntryXATBlob` | ACL and extended-attribute blob helpers | `src/aa_entry_blob.rs` | `swift-bridge/Sources/CompressionBridge/AAEntryBlob.swift` | `examples/09_aa_entry_blobs.rs` | `tests/aa_entry_blob_tests.rs` | Complete |
| `AEAContext` / `AEAStreams` | encrypted-archive contexts, auth data, and stream open/close helpers | `src/aea.rs` | `swift-bridge/Sources/CompressionBridge/AEA.swift` | `examples/10_aea_roundtrip.rs` | `tests/aea_tests.rs` | Complete |
| `AACustomByteStream` / `AACustomArchiveStream` | custom callback streams plus archive message handlers | `src/aa_byte_stream.rs`, `src/aa_archive_stream.rs` | `swift-bridge/Sources/CompressionBridge/AAByteStream.swift`, `swift-bridge/Sources/CompressionBridge/AAArchiveStream.swift` | `examples/11_aa_custom_stream_callbacks.rs` | `tests/custom_stream_tests.rs` | Complete |

## Bridge and raw-FFI decisions

- The safe public API uses Swift-owned boxes for the C handles. Rust never owns
  raw AppleArchive handles directly.
- `ByteStream` and `ArchiveStream` keep upstream streams alive in Rust so the
  AppleArchive close-order rules are respected.
- The `raw-ffi` Cargo feature preserves the direct `compression.h` bindings in
  `src/raw_ffi.rs`.
- AppleArchive is intentionally exposed only through the Swift bridge in this
  release.

## Coverage notes

- `Algorithm::ALL` contains the stream-safe algorithms.
- `Algorithm::BUFFER_ALL` also includes the buffer-only algorithms `Lz4Raw` and
  `Lzbitmap`.
- `FieldKey` supports known constants plus arbitrary three-character keys via
  `from_bytes`, `parse`, and `from_raw`.
- `AAHeaderGetFieldHash` is handled by reading the maximum digest size and then
  truncating according to the returned hash function.

## Audit summary

- `COVERAGE_AUDIT.md` now records 377 verified public symbols out of 377 audited
  macOS 26.2 symbols.
- The four deprecated `AppleArchive.h` compatibility shims are wrapped as
  deprecated Rust aliases or compatibility registrations, so the audited surface
  is at 100% with no remaining gaps or exemptions.
