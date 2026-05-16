import Compression

private let danglingMutableUInt8 = UnsafeMutablePointer<UInt8>(bitPattern: 1)!
private let danglingUInt8 = UnsafePointer<UInt8>(bitPattern: 1)!

final class CompressionStreamBox {
    var stream = compression_stream(
        dst_ptr: danglingMutableUInt8,
        dst_size: 0,
        src_ptr: danglingUInt8,
        src_size: 0,
        state: nil
    )

    init?(operation: Int32, algorithm: UInt32) {
        let status = compression_stream_init(
            &stream,
            compression_stream_operation(rawValue: UInt32(bitPattern: operation)),
            compression_algorithm(rawValue: algorithm)
        )
        guard status == COMPRESSION_STATUS_OK else {
            return nil
        }
    }

    deinit {
        if stream.state != nil {
            _ = compression_stream_destroy(&stream)
        }
    }
}

@_cdecl("compression_rs_compression_stream_create")
public func compressionRsCompressionStreamCreate(
    _ operation: Int32,
    _ algorithm: UInt32
) -> UnsafeMutableRawPointer? {
    guard let box = CompressionStreamBox(operation: operation, algorithm: algorithm) else {
        return nil
    }
    return retain(box)
}

@_cdecl("compression_rs_compression_stream_process")
public func compressionRsCompressionStreamProcess(
    _ handle: UnsafeMutableRawPointer?,
    _ srcBuffer: UnsafePointer<UInt8>?,
    _ srcSize: Int,
    _ dstBuffer: UnsafeMutablePointer<UInt8>?,
    _ dstSize: Int,
    _ flags: Int32,
    _ srcRemaining: UnsafeMutablePointer<Int>?,
    _ dstRemaining: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle, let srcBuffer, let dstBuffer else {
        return COMPRESSION_STATUS_ERROR.rawValue
    }
    let box: CompressionStreamBox = unretained(handle, as: CompressionStreamBox.self)
    box.stream.src_ptr = srcBuffer
    box.stream.src_size = srcSize
    box.stream.dst_ptr = dstBuffer
    box.stream.dst_size = dstSize

    let status = compression_stream_process(&box.stream, flags)
    srcRemaining?.pointee = box.stream.src_size
    dstRemaining?.pointee = box.stream.dst_size
    return status.rawValue
}

@_cdecl("compression_rs_compression_stream_release")
public func compressionRsCompressionStreamRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: CompressionStreamBox.self)
}
