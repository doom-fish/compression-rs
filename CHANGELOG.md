# Changelog

## [0.2.3] - 2026-05-18

### Changed

- Added `///` one-line API docs across the public `src/` surface outside `src/ffi/`, bringing the requested documentation pass to full coverage for that target.

## [0.2.2] - 2026-05-17

### Added

- Deprecated compatibility aliases for `AAArchiveStreamAbort`,
  `AAByteStreamAbort`, `AACustomArchiveStreamSetAbortProc`, and
  `AACustomByteStreamSetAbortProc`, all routed through the modern cancel-based
  implementations.

### Changed

- `COVERAGE_AUDIT.md` now records 377 verified public symbols out of 377 audited
  macOS 26.2 symbols, with no remaining gaps or exemptions.

## [0.2.1] - 2026-05-16

### Added

- Safe wrappers for `AAEntryACLBlob`, `AAEntryXATBlob`, `AACustomByteStream`,
  `AACustomArchiveStream`, `AAEntryMessageProc`, `AEAContext`, `AEAAuthData`,
  and the `AEAStreams` open/close helpers.
- Integration tests covering ACL/XAT blob round-trips, symmetric AEA archive
  round-trips, custom byte/archive callbacks, and archive message handlers.
- Example programs for ACL/XAT blobs, AEA round-trips, and custom callback
  streams.

### Changed

- `COVERAGE_AUDIT.md` now records full audited macOS 26.2 symbol coverage
  except for the four deprecated SDK shims that remain intentionally exempt.

## [0.2.0] - 2026-05-16

### Added

- Swift bridge package and `build.rs` integration for the C-only `Compression`
  and `AppleArchive` SDK APIs.
- Safe Rust modules for `CompressionStream`, `CompressionEncode`,
  `CompressionDecode`, `AAArchiveStream`, `AAByteStream`, `AAEntryStream`
  coverage, `AAFieldKey`, and `AAHeader`.
- One example and one integration test for each requested logical area.
- AppleArchive-safe wrappers for archive streams, byte streams, field-key sets,
  path lists, headers, flags, and random-access processing.
- `raw-ffi` feature exposing direct `compression.h` bindings through
  `compression::raw_ffi`.
- `Algorithm` support for `Lz4Raw` and `Lzbitmap`, plus `Algorithm::BUFFER_ALL`
  and `Algorithm::supports_streams()`.
- `COVERAGE.md` documenting the requested-area mapping and deferred SDK surface.

### Changed

- The default API now routes through opaque Swift-owned bridge handles instead
  of direct Rust ownership of C handles.
- `compress` / `decompress` now use stream processing when available and grow
  one-shot buffers for buffer-only algorithms.
- `CompressionStream::finish()` is now idempotent after end-of-stream.

### Fixed

- Swift compatibility library search/rpath handling during `cargo test` and
  example builds.
- Header hash decoding for `AAHeaderGetFieldHash`.

## [0.1.0] - 2026-05-16

### Added

- `Algorithm` coverage for `LZ4`, `LZFSE`, `LZMA`, `zlib`, and `Brotli`.
- Safe wrappers for `compression_encode_buffer` and `compression_decode_buffer`.
- Streaming `Encoder` and `Decoder` types backed by `compression_stream_init`,
  `compression_stream_process`, and `compression_stream_destroy`.
- Ergonomic `compress` / `decompress` helpers for in-memory round trips.
- `examples/01_roundtrip.rs` smoke example covering a 64 KiB LZFSE round trip.
