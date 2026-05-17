import AppleArchive
import Darwin

@_cdecl("compression_rs_aa_extract_archive_output_stream_open")
public func compressionRsAAExtractArchiveOutputStreamOpen(
    _ dir: UnsafePointer<CChar>?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let dir,
          let raw = __AAExtractArchiveOutputStreamOpen(dir, nil, nil, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_convert_archive_output_stream_open")
public func compressionRsAAConvertArchiveOutputStreamOpen(
    _ handle: UnsafeMutableRawPointer?,
    _ insertKeySet: UnsafeMutableRawPointer?,
    _ removeKeySet: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle, let insertKeySet, let removeKeySet else { return nil }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    let insertBox: AAFieldKeySetBox = unretained(insertKeySet, as: AAFieldKeySetBox.self)
    let removeBox: AAFieldKeySetBox = unretained(removeKeySet, as: AAFieldKeySetBox.self)
    guard let raw = box.raw, let insertRaw = insertBox.raw, let removeRaw = removeBox.raw,
          let stream = __AAConvertArchiveOutputStreamOpen(raw, insertRaw, removeRaw, nil, nil, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_encode_archive_output_stream_open")
public func compressionRsAAEncodeArchiveOutputStreamOpen(
    _ handle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw,
          let stream = __AAEncodeArchiveOutputStreamOpen(raw, nil, nil, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_decode_archive_input_stream_open")
public func compressionRsAADecodeArchiveInputStreamOpen(
    _ handle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    guard let raw = box.raw,
          let stream = __AADecodeArchiveInputStreamOpen(raw, nil, nil, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_archive_stream_write_header")
public func compressionRsAAArchiveStreamWriteHeader(
    _ handle: UnsafeMutableRawPointer?,
    _ header: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let header else { return -1 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    let headerBox: AAHeaderBox = unretained(header, as: AAHeaderBox.self)
    guard let raw = box.raw, let headerRaw = headerBox.raw else { return -1 }
    return Int32(__AAArchiveStreamWriteHeader(raw, headerRaw))
}

@_cdecl("compression_rs_aa_archive_stream_write_blob")
public func compressionRsAAArchiveStreamWriteBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UInt32,
    _ buffer: UnsafePointer<UInt8>?,
    _ length: Int
) -> Int32 {
    guard let handle, let buffer else { return -1 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAArchiveStreamWriteBlob(raw, aaFieldKey(key), buffer, length))
}

@_cdecl("compression_rs_aa_archive_stream_read_header_new")
public func compressionRsAAArchiveStreamReadHeaderNew(
    _ handle: UnsafeMutableRawPointer?,
    _ status: UnsafeMutablePointer<Int32>?
) -> UnsafeMutableRawPointer? {
    guard let handle else {
        status?.pointee = -1
        return nil
    }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else {
        status?.pointee = -1
        return nil
    }
    var header: OpaquePointer?
    let value = Int32(__AAArchiveStreamReadHeader(raw, &header))
    status?.pointee = value
    guard value == 1, let header else { return nil }
    return retain(AAHeaderBox(raw: header))
}

@_cdecl("compression_rs_aa_archive_stream_read_header_into")
public func compressionRsAAArchiveStreamReadHeaderInto(
    _ handle: UnsafeMutableRawPointer?,
    _ header: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let header else { return -1 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    let headerBox: AAHeaderBox = unretained(header, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    var headerRaw = headerBox.raw
    let status = Int32(__AAArchiveStreamReadHeader(raw, &headerRaw))
    if status == 1 {
        headerBox.raw = headerRaw
    }
    return status
}

@_cdecl("compression_rs_aa_archive_stream_read_blob")
public func compressionRsAAArchiveStreamReadBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UInt32,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ length: Int
) -> Int32 {
    guard let handle, let buffer else { return -1 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAArchiveStreamReadBlob(raw, aaFieldKey(key), buffer, length))
}

@_cdecl("compression_rs_aa_archive_stream_cancel")
public func compressionRsAAArchiveStreamCancel(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    __AAArchiveStreamCancel(raw)
}

@_cdecl("compression_rs_aa_archive_stream_abort")
public func compressionRsAAArchiveStreamAbort(_ handle: UnsafeMutableRawPointer?) {
    compressionRsAAArchiveStreamCancel(handle)
}

@_cdecl("compression_rs_aa_archive_stream_close")
public func compressionRsAAArchiveStreamClose(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return 0 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    return box.close()
}

@_cdecl("compression_rs_aa_archive_stream_write_path_list")
public func compressionRsAAArchiveStreamWritePathList(
    _ handle: UnsafeMutableRawPointer?,
    _ pathList: UnsafeMutableRawPointer?,
    _ keySet: UnsafeMutableRawPointer?,
    _ dir: UnsafePointer<CChar>?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> Int32 {
    guard let handle, let pathList, let keySet, let dir else { return -1 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    let pathListBox: AAPathListBox = unretained(pathList, as: AAPathListBox.self)
    let keySetBox: AAFieldKeySetBox = unretained(keySet, as: AAFieldKeySetBox.self)
    guard let raw = box.raw, let pathListRaw = pathListBox.raw, let keySetRaw = keySetBox.raw else {
        return -1
    }
    return Int32(__AAArchiveStreamWritePathList(raw, pathListRaw, keySetRaw, dir, nil, nil, __AAFlagSet(flags), nThreads))
}

@_cdecl("compression_rs_aa_archive_stream_process")
public func compressionRsAAArchiveStreamProcess(
    _ input: UnsafeMutableRawPointer?,
    _ output: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> Int64 {
    guard let input, let output else { return -1 }
    let inputBox: AAArchiveStreamBox = unretained(input, as: AAArchiveStreamBox.self)
    let outputBox: AAArchiveStreamBox = unretained(output, as: AAArchiveStreamBox.self)
    guard let inputRaw = inputBox.raw, let outputRaw = outputBox.raw else { return -1 }
    return Int64(__AAArchiveStreamProcess(inputRaw, outputRaw, nil, nil, __AAFlagSet(flags), nThreads))
}

@_cdecl("compression_rs_aa_archive_stream_release")
public func compressionRsAAArchiveStreamRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAArchiveStreamBox.self)
}

public typealias CompressionRsAAArchiveStreamWriteHeaderProc = @convention(c) (UnsafeMutableRawPointer?, OpaquePointer) -> Int32
public typealias CompressionRsAAArchiveStreamWriteBlobProc = @convention(c) (UnsafeMutableRawPointer?, __AAFieldKey, UnsafeRawPointer, Int) -> Int32
public typealias CompressionRsAAArchiveStreamReadHeaderProc = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutablePointer<OpaquePointer?>) -> Int32
public typealias CompressionRsAAArchiveStreamReadBlobProc = @convention(c) (UnsafeMutableRawPointer?, __AAFieldKey, UnsafeMutableRawPointer, Int) -> Int32
public typealias CompressionRsAAArchiveStreamCancelProc = @convention(c) (UnsafeMutableRawPointer?) -> Void
public typealias CompressionRsAAArchiveStreamCloseProc = @convention(c) (UnsafeMutableRawPointer?) -> Int32

@_cdecl("compression_rs_aa_custom_archive_stream_open")
public func compressionRsAACustomArchiveStreamOpen() -> UnsafeMutableRawPointer? {
    guard let raw = __AACustomArchiveStreamOpen() else { return nil }
    return retain(AAArchiveStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_data")
public func compressionRsAACustomArchiveStreamSetData(
    _ handle: UnsafeMutableRawPointer?,
    _ data: UnsafeMutableRawPointer?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomArchiveStreamSetData(raw, data)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_write_header_proc")
public func compressionRsAACustomArchiveStreamSetWriteHeaderProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer) -> Int32)?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    let typedProc = proc.map { unsafeBitCast($0, to: CompressionRsAAArchiveStreamWriteHeaderProc.self) }
    __AACustomArchiveStreamSetWriteHeaderProc(raw, typedProc)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_write_blob_proc")
public func compressionRsAACustomArchiveStreamSetWriteBlobProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: (@convention(c) (UnsafeMutableRawPointer?, UInt32, UnsafeRawPointer, Int) -> Int32)?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    let typedProc = proc.map { unsafeBitCast($0, to: CompressionRsAAArchiveStreamWriteBlobProc.self) }
    __AACustomArchiveStreamSetWriteBlobProc(raw, typedProc)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_read_header_proc")
public func compressionRsAACustomArchiveStreamSetReadHeaderProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: (@convention(c) (UnsafeMutableRawPointer?, UnsafeMutablePointer<UnsafeMutableRawPointer?>) -> Int32)?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    let typedProc = proc.map { unsafeBitCast($0, to: CompressionRsAAArchiveStreamReadHeaderProc.self) }
    __AACustomArchiveStreamSetReadHeaderProc(raw, typedProc)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_read_blob_proc")
public func compressionRsAACustomArchiveStreamSetReadBlobProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: (@convention(c) (UnsafeMutableRawPointer?, UInt32, UnsafeMutableRawPointer, Int) -> Int32)?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    let typedProc = proc.map { unsafeBitCast($0, to: CompressionRsAAArchiveStreamReadBlobProc.self) }
    __AACustomArchiveStreamSetReadBlobProc(raw, typedProc)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_cancel_proc")
public func compressionRsAACustomArchiveStreamSetCancelProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAArchiveStreamCancelProc?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomArchiveStreamSetCancelProc(raw, proc)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_abort_proc")
public func compressionRsAACustomArchiveStreamSetAbortProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAArchiveStreamCancelProc?
) {
    compressionRsAACustomArchiveStreamSetCancelProc(handle, proc)
}

@_cdecl("compression_rs_aa_custom_archive_stream_set_close_proc")
public func compressionRsAACustomArchiveStreamSetCloseProc(
    _ handle: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAArchiveStreamCloseProc?
) {
    guard let handle else { return }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    guard let raw = box.raw else { return }
    __AACustomArchiveStreamSetCloseProc(raw, proc)
}

public typealias CompressionRsAAEntryMessageProc = @convention(c) (UnsafeMutableRawPointer?, UInt32, UnsafePointer<CChar>, UnsafeMutableRawPointer?) -> Int32

@_cdecl("compression_rs_aa_extract_archive_output_stream_open_with_messages")
public func compressionRsAAExtractArchiveOutputStreamOpenWithMessages(
    _ dir: UnsafePointer<CChar>?,
    _ flags: UInt64,
    _ nThreads: Int32,
    _ arg: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAEntryMessageProc?
) -> UnsafeMutableRawPointer? {
    guard let dir else { return nil }
    let typedProc = proc.map { unsafeBitCast($0, to: __AAEntryMessageProc.self) }
    guard let raw = __AAExtractArchiveOutputStreamOpen(dir, arg, typedProc, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: raw))
}

@_cdecl("compression_rs_aa_encode_archive_output_stream_open_with_messages")
public func compressionRsAAEncodeArchiveOutputStreamOpenWithMessages(
    _ handle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32,
    _ arg: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAEntryMessageProc?
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    let typedProc = proc.map { unsafeBitCast($0, to: __AAEntryMessageProc.self) }
    guard let raw = box.raw,
          let stream = __AAEncodeArchiveOutputStreamOpen(raw, arg, typedProc, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_decode_archive_input_stream_open_with_messages")
public func compressionRsAADecodeArchiveInputStreamOpenWithMessages(
    _ handle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32,
    _ arg: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAEntryMessageProc?
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAByteStreamBox = unretained(handle, as: AAByteStreamBox.self)
    let typedProc = proc.map { unsafeBitCast($0, to: __AAEntryMessageProc.self) }
    guard let raw = box.raw,
          let stream = __AADecodeArchiveInputStreamOpen(raw, arg, typedProc, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_convert_archive_output_stream_open_with_messages")
public func compressionRsAAConvertArchiveOutputStreamOpenWithMessages(
    _ handle: UnsafeMutableRawPointer?,
    _ insertKeySet: UnsafeMutableRawPointer?,
    _ removeKeySet: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32,
    _ arg: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAEntryMessageProc?
) -> UnsafeMutableRawPointer? {
    guard let handle, let insertKeySet, let removeKeySet else { return nil }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    let insertBox: AAFieldKeySetBox = unretained(insertKeySet, as: AAFieldKeySetBox.self)
    let removeBox: AAFieldKeySetBox = unretained(removeKeySet, as: AAFieldKeySetBox.self)
    let typedProc = proc.map { unsafeBitCast($0, to: __AAEntryMessageProc.self) }
    guard let raw = box.raw,
          let insertRaw = insertBox.raw,
          let removeRaw = removeBox.raw,
          let stream = __AAConvertArchiveOutputStreamOpen(raw, insertRaw, removeRaw, arg, typedProc, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAArchiveStreamBox(raw: stream))
}

@_cdecl("compression_rs_aa_archive_stream_write_path_list_with_messages")
public func compressionRsAAArchiveStreamWritePathListWithMessages(
    _ handle: UnsafeMutableRawPointer?,
    _ pathList: UnsafeMutableRawPointer?,
    _ keySet: UnsafeMutableRawPointer?,
    _ dir: UnsafePointer<CChar>?,
    _ flags: UInt64,
    _ nThreads: Int32,
    _ arg: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAEntryMessageProc?
) -> Int32 {
    guard let handle, let pathList, let keySet, let dir else { return -1 }
    let box: AAArchiveStreamBox = unretained(handle, as: AAArchiveStreamBox.self)
    let pathListBox: AAPathListBox = unretained(pathList, as: AAPathListBox.self)
    let keySetBox: AAFieldKeySetBox = unretained(keySet, as: AAFieldKeySetBox.self)
    let typedProc = proc.map { unsafeBitCast($0, to: __AAEntryMessageProc.self) }
    guard let raw = box.raw, let pathListRaw = pathListBox.raw, let keySetRaw = keySetBox.raw else {
        return -1
    }
    return Int32(__AAArchiveStreamWritePathList(raw, pathListRaw, keySetRaw, dir, arg, typedProc, __AAFlagSet(flags), nThreads))
}

@_cdecl("compression_rs_aa_archive_stream_process_with_messages")
public func compressionRsAAArchiveStreamProcessWithMessages(
    _ input: UnsafeMutableRawPointer?,
    _ output: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32,
    _ arg: UnsafeMutableRawPointer?,
    _ proc: CompressionRsAAEntryMessageProc?
) -> Int64 {
    guard let input, let output else { return -1 }
    let inputBox: AAArchiveStreamBox = unretained(input, as: AAArchiveStreamBox.self)
    let outputBox: AAArchiveStreamBox = unretained(output, as: AAArchiveStreamBox.self)
    let typedProc = proc.map { unsafeBitCast($0, to: __AAEntryMessageProc.self) }
    guard let inputRaw = inputBox.raw, let outputRaw = outputBox.raw else { return -1 }
    return Int64(__AAArchiveStreamProcess(inputRaw, outputRaw, arg, typedProc, __AAFlagSet(flags), nThreads))
}
