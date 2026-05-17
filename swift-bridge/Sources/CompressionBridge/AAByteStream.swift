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

@_cdecl("compression_rs_aa_byte_stream_abort")
public func compressionRsAAByteStreamAbort(_ handle: UnsafeMutableRawPointer?) {
    compressionRsAAByteStreamCancel(handle)
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

public typealias CompressionRsAAByteStreamWriteProc = @convention(c) (UnsafeMutableRawPointer?, UnsafeRawPointer, Int) -> Int
public typealias CompressionRsAAByteStreamPWriteProc = @convention(c) (UnsafeMutableRawPointer?, UnsafeRawPointer, Int, Int64) -> Int
public typealias CompressionRsAAByteStreamReadProc = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer, Int) -> Int
public typealias CompressionRsAAByteStreamPReadProc = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer, Int, Int64) -> Int
public typealias CompressionRsAAByteStreamSeekProc = @convention(c) (UnsafeMutableRawPointer?, Int64, Int32) -> Int64
public typealias CompressionRsAAByteStreamCancelProc = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias CompressionRsAAByteStreamCloseProc = @convention(c) (UnsafeMutableRawPointer?) -> Int32

@_cdecl("compression_rs_aa_custom_byte_stream_open")
public func compressionRsAACustomByteStreamOpen() -> UnsafeMutableRawPointer? {
    guard let raw = __AACustomByteStreamOpen() else { return nil }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_data")
public func compressionRsAACustomByteStreamSetData(
    _ handle: UnsafeMutableRawPointer?,
    _ data: UnsafeMutableRawPointer?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetData(raw, data)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_write_proc")
public func compressionRsAACustomByteStreamSetWriteProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamWriteProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetWriteProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_pwrite_proc")
public func compressionRsAACustomByteStreamSetPWriteProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamPWriteProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetPWriteProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_read_proc")
public func compressionRsAACustomByteStreamSetReadProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamReadProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetReadProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_pread_proc")
public func compressionRsAACustomByteStreamSetPReadProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamPReadProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetPReadProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_seek_proc")
public func compressionRsAACustomByteStreamSetSeekProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamSeekProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetSeekProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_cancel_proc")
public func compressionRsAACustomByteStreamSetCancelProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamCancelProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetCancelProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_abort_proc")
public func compressionRsAACustomByteStreamSetAbortProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamCancelProc?
) {
    compressionRsAACustomByteStreamSetCancelProc(handle, proc)
}

@_cdecl("compression_rs_aa_custom_byte_stream_set_close_proc")
public func compressionRsAACustomByteStreamSetCloseProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAByteStreamCloseProc?
) {
    guard let handle else { return }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomByteStreamSetCloseProc(raw, proc)
}
