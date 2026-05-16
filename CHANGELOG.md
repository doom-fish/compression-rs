# Changelog

## [0.1.0] - 2026-05-16

### Added

- `Algorithm` coverage for `LZ4`, `LZFSE`, `LZMA`, `zlib`, and `Brotli`.
- Safe wrappers for `compression_encode_buffer` and `compression_decode_buffer`.
- Streaming `Encoder` and `Decoder` types backed by `compression_stream_init`,
  `compression_stream_process`, and `compression_stream_destroy`.
- Ergonomic `compress` / `decompress` helpers for in-memory round trips.
- `examples/01_roundtrip.rs` smoke example covering a 64 KiB LZFSE round trip.
