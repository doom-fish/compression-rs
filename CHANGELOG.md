# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - Unreleased

### Security

- **Breaking:** decoding is bounded. `Decoder::new`, decode `CompressionStream`s and `decompress` fail with the new `CompressionError::OutputLimitExceeded` after `DEFAULT_MAX_OUTPUT_LEN` (64 MiB) of output, so a small zlib, LZMA or Brotli bomb can no longer exhaust memory. `Decoder::with_max_output_len`, `CompressionStream::with_max_output_len` and `decompress_with_max_output_len` choose another limit.
- **Breaking:** AEA key material from `AeaContext::field_blob`, `main_key` and `signature_encryption_key` is returned as `SecretBytes` instead of `Vec<u8>`; it redacts its `Debug` output and wipes its memory on drop.

### Fixed

- **Breaking (soundness):** AppleArchive calls custom stream callbacks and entry message handlers from its worker threads, where non-thread-safe state raced and concurrent calls aliased `&mut`. `CustomByteStreamCallbacks`, `CustomArchiveStreamCallbacks` and `EntryMessageHandler` now require `Send + Sync` and take `&self` (`close` keeps `&mut self`); handler closures must be `Fn + Send + Sync`.
- A panic in an entry message handler, or while the custom stream state is dropped on close, no longer aborts the process; every callback goes through `doom_fish_utils::panic_safe` and a panic fails the operation.
- `compression_decode_buffer` reports a destination that is too small as `OutputLimitExceeded` instead of returning `Ok(dst.len())`, and accepts valid encodings of empty data. The buffer-only `decompress` path decodes once instead of up to 15 times.
- The Swift bridge copies of encoded `AAHeader`, `AEAAuthData`, ACL and XAT data now respect the destination capacity, entry values are truncated to their reported size, and returned strings are parsed within their buffers.
- `AeaContext::field_blob` truncates to the size reported by its second call.
- **Breaking (soundness):** `ByteStream::from_fd` takes an `OwnedFd`, so the stream can no longer close a descriptor it does not own. The previous behavior is available as the `unsafe` `ByteStream::from_raw_fd`.
- `CompressionStream::process` returns `StreamStalled` instead of looping forever when the stream stops making progress.

### Changed

- `ByteStream`, `ArchiveStream`, `Header`, `EntryXatBlob` and `EntryAclBlob` are `Send`.
- `rust-version` is now 1.82, which the crate's `unsafe extern` blocks already required.
- Depends on `doom-fish-utils` 0.4.1 for panic containment and callback contexts.

### Added

- `DEFAULT_MAX_OUTPUT_LEN`, `decompress_with_max_output_len`, `Decoder::with_max_output_len`, `CompressionStream::with_max_output_len`, `CompressionError::OutputLimitExceeded`, `SecretBytes` and `ByteStream::from_raw_fd`.

### Removed

- The unused `src/stream.rs`, which was published but not compiled.

## [0.2.4] - 2026-06-06

### Fixed

- AppleArchive custom byte and archive stream callbacks catch panics instead of unwinding across the C ABI.

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
