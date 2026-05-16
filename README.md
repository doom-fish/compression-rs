# compression-rs

Safe Rust bindings for Apple's `libcompression` APIs on macOS.

`compression-rs` covers the core pieces you typically need first:

- one-shot `compression_encode_buffer` / `compression_decode_buffer`
- streaming `Encoder` / `Decoder` wrappers built on `compression_stream_*`
- algorithms: `LZ4`, `LZFSE`, `LZMA`, `zlib`, and `Brotli`
- ergonomic `compress` / `decompress` helpers that grow output buffers for you

## Status

Initial `0.1.0` coverage focuses on the public `compression.h` surface that is
useful for in-memory tools and streaming pipelines.

## Installation

```toml
[dependencies]
compression-rs = "0.1"
```

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

## Highlights

- Direct wrappers for `compression_encode_buffer` and `compression_decode_buffer`
- Safe `Encoder` / `Decoder` streaming types for chunked workloads
- No Swift bridge required — this crate is pure Rust + C FFI
- `examples/01_roundtrip.rs` smoke test for a 64 KiB LZFSE round-trip

## API notes

- `compression_encode_buffer` and `compression_decode_buffer` expect the caller to
  provide the destination buffer and return the number of bytes written.
- `compress` and `decompress` build on the streaming APIs so they can resize
  output buffers automatically.
- `Algorithm::Brotli` requires a macOS version that ships Brotli support in
  `libcompression`.

## Smoke example

```bash
cargo run --example 01_roundtrip
```

Expected tail output:

```text
✅ compression round-trip OK
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
