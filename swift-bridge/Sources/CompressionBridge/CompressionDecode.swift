import Compression

@_cdecl("compression_rs_compression_decode_scratch_buffer_size")
public func compressionRsCompressionDecodeScratchBufferSize(_ algorithm: UInt32) -> Int {
    Int(compression_decode_scratch_buffer_size(compression_algorithm(rawValue: algorithm)))
}

@_cdecl("compression_rs_compression_decode_buffer")
public func compressionRsCompressionDecodeBuffer(
    _ dstBuffer: UnsafeMutablePointer<UInt8>?,
    _ dstSize: Int,
    _ srcBuffer: UnsafePointer<UInt8>?,
    _ srcSize: Int,
    _ algorithm: UInt32
) -> Int {
    guard let dstBuffer, let srcBuffer else { return 0 }
    return Int(
        compression_decode_buffer(
            dstBuffer,
            dstSize,
            srcBuffer,
            srcSize,
            nil,
            compression_algorithm(rawValue: algorithm)
        )
    )
}
