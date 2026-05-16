import AppleArchive
import Darwin

@_cdecl("compression_rs_aa_byte_stream_open_with_fd")
public func compressionRsAAByteStreamOpenWithFD(
    _ fd: Int32,
    _ automaticClose: Bool
) -> UnsafeMutableRawPointer? {
    guard let raw = __AAFileStreamOpenWithFD(fd, automaticClose ? 1 : 0) else { return nil }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_byte_stream_open_with_path")
public func compressionRsAAByteStreamOpenWithPath(
    _ path: UnsafePointer<CChar>?,
    _ openFlags: Int32,
    _ openMode: UInt32
) -> UnsafeMutableRawPointer? {
    guard let path,
          let raw = __AAFileStreamOpenWithPath(path, openFlags, mode_t(openMode))
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_temp_file_stream_open")
public func compressionRsAATempFileStreamOpen() -> UnsafeMutableRawPointer? {
    guard let raw = __AATempFileStreamOpen() else { return nil }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_shared_buffer_pipe_open")
public func compressionRsAASharedBufferPipeOpen(
    _ ostream: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ istream: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ bufferCapacity: Int
) -> Int32 {
    let outputPtr = UnsafeMutablePointer<OpaquePointer>.allocate(capacity: 1)
    let inputPtr = UnsafeMutablePointer<OpaquePointer>.allocate(capacity: 1)
    defer {
        outputPtr.deallocate()
        inputPtr.deallocate()
    }
    outputPtr.initialize(to: OpaquePointer(bitPattern: 1)!)
    inputPtr.initialize(to: OpaquePointer(bitPattern: 1)!)
    let status = Int32(__AASharedBufferPipeOpen(outputPtr, inputPtr, bufferCapacity))
    if status == 0 {
        ostream?.pointee = retain(AAByteStreamBox(raw: outputPtr.pointee))
        istream?.pointee = retain(AAByteStreamBox(raw: inputPtr.pointee))
    } else {
        ostream?.pointee = nil
        istream?.pointee = nil
    }
    return status
}

@_cdecl("compression_rs_aa_compression_output_stream_open")
public func compressionRsAACompressionOutputStreamOpen(
    _ handle: UnsafeMutableRawPointer?,
    _ compressionAlgorithm: UInt32,
    _ blockSize: Int,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw,
          let stream = __AACompressionOutputStreamOpen(raw, __AACompressionAlgorithm(compressionAlgorithm), blockSize, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_compression_output_stream_open_existing")
public func compressionRsAACompressionOutputStreamOpenExisting(
    _ handle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw,
          let stream = __AACompressionOutputStreamOpenExisting(raw, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_decompression_input_stream_open")
public func compressionRsAADecompressionInputStreamOpen(
    _ handle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw,
          let stream = __AADecompressionInputStreamOpen(raw, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_decompression_random_access_input_stream_open")
public func compressionRsAADecompressionRandomAccessInputStreamOpen(
    _ handle: UnsafeMutableRawPointer?,
    _ allocLimit: Int,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    guard #available(macOS 12.0, *) else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw,
          let stream = __AADecompressionRandomAccessInputStreamOpen(raw, allocLimit, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_byte_stream_write")
public func compressionRsAAByteStreamWrite(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafePointer<UInt8>?,
    _ length: Int
) -> Int64 {
    guard let handle, let buffer else { return -1 }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int64(__AAByteStreamWrite(raw, buffer, length))
}

@_cdecl("compression_rs_aa_byte_stream_pwrite")
public func compressionRsAAByteStreamPWrite(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafePointer<UInt8>?,
    _ length: Int,
    _ offset: Int64
) -> Int64 {
    guard let handle, let buffer else { return -1 }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int64(__AAByteStreamPWrite(raw, buffer, length, offset))
}

@_cdecl("compression_rs_aa_byte_stream_read")
public func compressionRsAAByteStreamRead(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ length: Int
) -> Int64 {
    guard let handle, let buffer else { return -1 }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int64(__AAByteStreamRead(raw, buffer, length))
}

@_cdecl("compression_rs_aa_byte_stream_pread")
public func compressionRsAAByteStreamPRead(
    _ handle: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ length: Int,
    _ offset: Int64
) -> Int64 {
    guard let handle, let buffer else { return -1 }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int64(__AAByteStreamPRead(raw, buffer, length, offset))
}

@_cdecl("compression_rs_aa_byte_stream_seek")
public func compressionRsAAByteStreamSeek(
    _ handle: UnsafeMutableRawPointer?,
    _ offset: Int64,
    _ whence: Int32
) -> Int64 {
    guard let handle else { return -1 }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int64(__AAByteStreamSeek(raw, offset, whence))
}

@_cdecl("compression_rs_aa_byte_stream_cancel")
public func compressionRsAAByteStreamCancel(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AAByteStreamCancel(raw)
}

@_cdecl("compression_rs_aa_byte_stream_close")
public func compressionRsAAByteStreamClose(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return 0 }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    return box.close()
}

@_cdecl("compression_rs_aa_byte_stream_process")
public func compressionRsAAByteStreamProcess(
    _ input: UnsafeMutableRawPointer?,
    _ output: UnsafeMutableRawPointer?
) -> Int64 {
    guard let input, let output else { return -1 }
    let inputBox: AAByteStreamBox = unretained(input, as: AAByteStreamBox.self)
    let outputBox: AAByteStreamBox = unretained(output, as: AAByteStreamBox.self)
    guard let inputRaw = inputBox.raw, let outputRaw = outputBox.raw else { return -1 }
    return Int64(__AAByteStreamProcess(inputRaw, outputRaw))
}

@_cdecl("compression_rs_aa_random_access_byte_stream_process")
public func compressionRsAARandomAccessByteStreamProcess(
    _ input: UnsafeMutableRawPointer?,
    _ output: UnsafeMutableRawPointer?,
    _ maxOffset: Int64,
    _ blockSize: Int,
    _ flags: UInt64,
    _ nThreads: Int32
) -> Int64 {
    guard let input, let output else { return -1 }
    guard #available(macOS 12.0, *) else { return -1 }
    let inputBox: AAByteStreamBox = unretained(input, as: AAByteStreamBox.self)
    let outputBox: AAByteStreamBox = unretained(output, as: AAByteStreamBox.self)
    guard let inputRaw = inputBox.raw, let outputRaw = outputBox.raw else { return -1 }
    return Int64(__AARandomAccessByteStreamProcess(inputRaw, outputRaw, maxOffset, blockSize, __AAFlagSet(flags), nThreads))
}

@_cdecl("compression_rs_aa_byte_stream_release")
public func compressionRsAAByteStreamRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAByteStreamBox.self)
}
