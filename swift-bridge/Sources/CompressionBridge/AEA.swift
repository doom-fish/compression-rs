import AppleArchive
import Darwin

@_cdecl("compression_rs_aea_context_create_with_profile")
public func compressionRsAEAContextCreateWithProfile(_ profile: UInt32) -> UnsafeMutableRawPointer? {
    guard let raw = __AEAContextCreateWithProfile(profile) else { return nil }
    return retain(AEAContextBox(raw: raw))
}

@_cdecl("compression_rs_aea_context_create_with_encrypted_stream")
public func compressionRsAEAContextCreateWithEncryptedStream(
    _ streamHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let streamHandle else { return nil }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    guard let stream = streamBox.raw,
          let raw = __AEAContextCreateWithEncryptedStream(stream)
    else {
        return nil
    }
    return retain(AEAContextBox(raw: raw))
}

@_cdecl("compression_rs_aea_context_get_field_uint")
public func compressionRsAEAContextGetFieldUInt(
    _ handle: UnsafeMutableRawPointer?,
    _ field: UInt32
) -> UInt64 {
    guard let handle else { return UInt64.max }
    let box: AEAContextBox = unretained(handle, as: AEAContextBox.self)
    guard let raw = box.raw else { return UInt64.max }
    return __AEAContextGetFieldUInt(raw, field)
}

@_cdecl("compression_rs_aea_context_get_field_blob")
public func compressionRsAEAContextGetFieldBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ field: UInt32,
    _ representation: UInt32,
    _ bufCapacity: Int,
    _ buf: UnsafeMutablePointer<UInt8>?,
    _ bufSize: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle else { return -1 }
    let box: AEAContextBox = unretained(handle, as: AEAContextBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAContextGetFieldBlob(raw, field, representation, bufCapacity, buf, bufSize))
}

@_cdecl("compression_rs_aea_context_set_field_uint")
public func compressionRsAEAContextSetFieldUInt(
    _ handle: UnsafeMutableRawPointer?,
    _ field: UInt32,
    _ value: UInt64
) -> Int32 {
    guard let handle else { return -1 }
    let box: AEAContextBox = unretained(handle, as: AEAContextBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAContextSetFieldUInt(raw, field, value))
}

@_cdecl("compression_rs_aea_context_set_field_blob")
public func compressionRsAEAContextSetFieldBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ field: UInt32,
    _ representation: UInt32,
    _ buf: UnsafePointer<UInt8>?,
    _ bufSize: Int
) -> Int32 {
    guard let handle else { return -1 }
    let box: AEAContextBox = unretained(handle, as: AEAContextBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAContextSetFieldBlob(raw, field, representation, buf, bufSize))
}

@_cdecl("compression_rs_aea_context_generate_field_blob")
public func compressionRsAEAContextGenerateFieldBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ field: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    guard #available(macOS 12.0, *) else { return -1 }
    let box: AEAContextBox = unretained(handle, as: AEAContextBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAContextGenerateFieldBlob(raw, field))
}

@_cdecl("compression_rs_aea_context_decrypt_attributes")
public func compressionRsAEAContextDecryptAttributes(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return -1 }
    guard #available(macOS 13.0, *) else { return -1 }
    let box: AEAContextBox = unretained(handle, as: AEAContextBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAContextDecryptAttributes(raw))
}

@_cdecl("compression_rs_aea_context_release")
public func compressionRsAEAContextRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AEAContextBox.self)
}

@_cdecl("compression_rs_aea_encryption_output_stream_open")
public func compressionRsAEAEncryptionOutputStreamOpen(
    _ streamHandle: UnsafeMutableRawPointer?,
    _ contextHandle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let streamHandle, let contextHandle else { return nil }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let stream = streamBox.raw, let context = contextBox.raw,
          let raw = __AEAEncryptionOutputStreamOpen(stream, context, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aea_encryption_output_stream_open_existing")
public func compressionRsAEAEncryptionOutputStreamOpenExisting(
    _ streamHandle: UnsafeMutableRawPointer?,
    _ contextHandle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let streamHandle, let contextHandle else { return nil }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let stream = streamBox.raw, let context = contextBox.raw,
          let raw = __AEAEncryptionOutputStreamOpenExisting(stream, context, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aea_encryption_output_stream_close_and_update_context")
public func compressionRsAEAEncryptionOutputStreamCloseAndUpdateContext(
    _ streamHandle: UnsafeMutableRawPointer?,
    _ contextHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let streamHandle, let contextHandle else { return -1 }
    guard #available(macOS 11.3, *) else { return -1 }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let stream = streamBox.raw, let context = contextBox.raw else { return -1 }
    let status = Int32(__AEAEncryptionOutputStreamCloseAndUpdateContext(stream, context))
    if status >= 0 {
        streamBox.raw = nil
    }
    return status
}

@_cdecl("compression_rs_aea_decryption_input_stream_open")
public func compressionRsAEADecryptionInputStreamOpen(
    _ streamHandle: UnsafeMutableRawPointer?,
    _ contextHandle: UnsafeMutableRawPointer?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let streamHandle, let contextHandle else { return nil }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let stream = streamBox.raw, let context = contextBox.raw,
          let raw = __AEADecryptionInputStreamOpen(stream, context, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aea_decryption_random_access_input_stream_open")
public func compressionRsAEADecryptionRandomAccessInputStreamOpen(
    _ streamHandle: UnsafeMutableRawPointer?,
    _ contextHandle: UnsafeMutableRawPointer?,
    _ allocLimit: Int,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let streamHandle, let contextHandle else { return nil }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let stream = streamBox.raw, let context = contextBox.raw,
          let raw = __AEADecryptionRandomAccessInputStreamOpen(stream, context, allocLimit, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAByteStreamBox(raw: raw))
}

@_cdecl("compression_rs_aea_stream_sign")
public func compressionRsAEAStreamSign(
    _ streamHandle: UnsafeMutableRawPointer?,
    _ contextHandle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let streamHandle, let contextHandle else { return -1 }
    let streamBox: AAByteStreamBox = unretained(streamHandle, as: AAByteStreamBox.self)
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let stream = streamBox.raw, let context = contextBox.raw else { return -1 }
    return Int32(__AEAStreamSign(stream, context))
}

@_cdecl("compression_rs_aea_auth_data_create")
public func compressionRsAEAAuthDataCreate() -> UnsafeMutableRawPointer? {
    guard #available(macOS 11.3, *) else { return nil }
    guard let raw = __AEAAuthDataCreate() else { return nil }
    return retain(AEAAuthDataBox(raw: raw))
}

@_cdecl("compression_rs_aea_auth_data_create_with_context")
public func compressionRsAEAAuthDataCreateWithContext(
    _ contextHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let contextHandle else { return nil }
    guard #available(macOS 11.3, *) else { return nil }
    let contextBox: AEAContextBox = unretained(contextHandle, as: AEAContextBox.self)
    guard let context = contextBox.raw,
          let raw = __AEAAuthDataCreateWithContext(context)
    else {
        return nil
    }
    return retain(AEAAuthDataBox(raw: raw))
}

@_cdecl("compression_rs_aea_auth_data_get_entry_count")
public func compressionRsAEAAuthDataGetEntryCount(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else { return 0 }
    guard #available(macOS 11.3, *) else { return 0 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return 0 }
    return __AEAAuthDataGetEntryCount(raw)
}

@_cdecl("compression_rs_aea_auth_data_get_entry")
public func compressionRsAEAAuthDataGetEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ keyCapacity: Int,
    _ key: UnsafeMutablePointer<CChar>?,
    _ keyLength: UnsafeMutablePointer<Int>?,
    _ dataCapacity: Int,
    _ data: UnsafeMutablePointer<UInt8>?,
    _ dataSize: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle else { return -1 }
    guard #available(macOS 11.3, *) else { return -1 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAAuthDataGetEntry(raw, index, keyCapacity, key, keyLength, dataCapacity, data, dataSize))
}

@_cdecl("compression_rs_aea_auth_data_append_entry")
public func compressionRsAEAAuthDataAppendEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> Int32 {
    guard let handle, let key else { return -1 }
    guard #available(macOS 11.3, *) else { return -1 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAAuthDataAppendEntry(raw, key, data, dataSize))
}

@_cdecl("compression_rs_aea_auth_data_set_entry")
public func compressionRsAEAAuthDataSetEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UnsafePointer<CChar>?,
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> Int32 {
    guard let handle, let key else { return -1 }
    guard #available(macOS 11.3, *) else { return -1 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAAuthDataSetEntry(raw, index, key, data, dataSize))
}

@_cdecl("compression_rs_aea_auth_data_clear")
public func compressionRsAEAAuthDataClear(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return -1 }
    guard #available(macOS 11.3, *) else { return -1 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAAuthDataClear(raw))
}

@_cdecl("compression_rs_aea_auth_data_remove_entry")
public func compressionRsAEAAuthDataRemoveEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    guard #available(macOS 11.3, *) else { return -1 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AEAAuthDataRemoveEntry(raw, index))
}

@_cdecl("compression_rs_aea_auth_data_get_encoded_size")
public func compressionRsAEAAuthDataGetEncodedSize(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else { return 0 }
    guard #available(macOS 11.3, *) else { return 0 }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return 0 }
    return __AEAAuthDataGetEncodedSize(raw)
}

@_cdecl("compression_rs_aea_auth_data_copy_encoded_data")
public func compressionRsAEAAuthDataCopyEncodedData(
    _ handle: UnsafeMutableRawPointer?,
    _ dst: UnsafeMutablePointer<UInt8>?
) -> Bool {
    guard let handle else { return false }
    guard #available(macOS 11.3, *) else { return false }
    let box: AEAAuthDataBox = unretained(handle, as: AEAAuthDataBox.self)
    guard let raw = box.raw else { return false }
    let size = __AEAAuthDataGetEncodedSize(raw)
    if size == 0 {
        return true
    }
    guard let src = __AEAAuthDataGetEncodedData(raw), let dst else { return false }
    memcpy(dst, src, size)
    return true
}

@_cdecl("compression_rs_aea_auth_data_release")
public func compressionRsAEAAuthDataRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AEAAuthDataBox.self)
}
