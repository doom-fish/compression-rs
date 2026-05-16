import AppleArchive
import Darwin

@_cdecl("compression_rs_aa_header_create")
public func compressionRsAAHeaderCreate() -> UnsafeMutableRawPointer? {
    guard let raw = __AAHeaderCreate() else { return nil }
    return retain(AAHeaderBox(raw: raw))
}

@_cdecl("compression_rs_aa_header_create_with_encoded_data")
public func compressionRsAAHeaderCreateWithEncodedData(
    _ dataSize: Int,
    _ data: UnsafePointer<UInt8>?
) -> UnsafeMutableRawPointer? {
    guard let data, let raw = __AAHeaderCreateWithEncodedData(dataSize, data) else { return nil }
    return retain(AAHeaderBox(raw: raw))
}

@_cdecl("compression_rs_aa_header_clone")
public func compressionRsAAHeaderClone(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw, let clone = __AAHeaderClone(raw) else { return nil }
    return retain(AAHeaderBox(raw: clone))
}

@_cdecl("compression_rs_aa_header_create_with_path")
public func compressionRsAAHeaderCreateWithPath(
    _ keySet: UnsafeMutableRawPointer?,
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ flags: UInt64
) -> UnsafeMutableRawPointer? {
    guard let keySet, let dir, let path else { return nil }
    let keySetBox: AAFieldKeySetBox = unretained(keySet, as: AAFieldKeySetBox.self)
    guard let keySetRaw = keySetBox.raw,
          let raw = __AAHeaderCreateWithPath(keySetRaw, dir, path, __AAFlagSet(flags))
    else {
        return nil
    }
    return retain(AAHeaderBox(raw: raw))
}

@_cdecl("compression_rs_aa_header_release")
public func compressionRsAAHeaderRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAHeaderBox.self)
}

@_cdecl("compression_rs_aa_header_assign")
public func compressionRsAAHeaderAssign(
    _ handle: UnsafeMutableRawPointer?,
    _ other: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let other else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    let otherBox: AAHeaderBox = unretained(other, as: AAHeaderBox.self)
    guard let raw = box.raw, let otherRaw = otherBox.raw else { return -1 }
    return Int32(__AAHeaderAssign(raw, otherRaw))
}

@_cdecl("compression_rs_aa_header_get_field_count")
public func compressionRsAAHeaderGetFieldCount(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else { return 0 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAHeaderGetFieldCount(raw)
}

@_cdecl("compression_rs_aa_header_get_key_index")
public func compressionRsAAHeaderGetKeyIndex(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderGetKeyIndex(raw, aaFieldKey(key)))
}

@_cdecl("compression_rs_aa_header_get_field_type")
public func compressionRsAAHeaderGetFieldType(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderGetFieldType(raw, index))
}

@_cdecl("compression_rs_aa_header_get_field_key")
public func compressionRsAAHeaderGetFieldKey(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> UInt32 {
    guard let handle else { return 0 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return 0 }
    return rawFieldKey(__AAHeaderGetFieldKey(raw, index))
}

@_cdecl("compression_rs_aa_header_get_payload_size")
public func compressionRsAAHeaderGetPayloadSize(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else { return 0 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAHeaderGetPayloadSize(raw)
}

@_cdecl("compression_rs_aa_header_remove_field")
public func compressionRsAAHeaderRemoveField(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderRemoveField(raw, index))
}

@_cdecl("compression_rs_aa_header_clear")
public func compressionRsAAHeaderClear(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderClear(raw))
}

@_cdecl("compression_rs_aa_header_set_field_flag")
public func compressionRsAAHeaderSetFieldFlag(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderSetFieldFlag(raw, index, aaFieldKey(key)))
}

@_cdecl("compression_rs_aa_header_set_field_uint")
public func compressionRsAAHeaderSetFieldUInt(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UInt32,
    _ value: UInt64
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderSetFieldUInt(raw, index, aaFieldKey(key), value))
}

@_cdecl("compression_rs_aa_header_set_field_string")
public func compressionRsAAHeaderSetFieldString(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UInt32,
    _ value: UnsafePointer<CChar>?,
    _ length: Int
) -> Int32 {
    guard let handle, let value else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderSetFieldString(raw, index, aaFieldKey(key), value, length))
}

@_cdecl("compression_rs_aa_header_set_field_hash")
public func compressionRsAAHeaderSetFieldHash(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UInt32,
    _ hashFunction: UInt32,
    _ value: UnsafePointer<UInt8>?
) -> Int32 {
    guard let handle, let value else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderSetFieldHash(raw, index, aaFieldKey(key), __AAHashFunction(hashFunction), value))
}

@_cdecl("compression_rs_aa_header_set_field_timespec")
public func compressionRsAAHeaderSetFieldTimespec(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UInt32,
    _ seconds: Int64,
    _ nanoseconds: Int64
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    var value = timespec(tv_sec: Int(seconds), tv_nsec: Int(nanoseconds))
    return Int32(__AAHeaderSetFieldTimespec(raw, index, aaFieldKey(key), &value))
}

@_cdecl("compression_rs_aa_header_set_field_blob")
public func compressionRsAAHeaderSetFieldBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UInt32,
    _ size: UInt64
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderSetFieldBlob(raw, index, aaFieldKey(key), size))
}

@_cdecl("compression_rs_aa_header_get_field_uint")
public func compressionRsAAHeaderGetFieldUInt(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ value: UnsafeMutablePointer<UInt64>?
) -> Int32 {
    guard let handle, let value else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderGetFieldUInt(raw, index, value))
}

@_cdecl("compression_rs_aa_header_get_field_string")
public func compressionRsAAHeaderGetFieldString(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ capacity: Int,
    _ value: UnsafeMutablePointer<CChar>?,
    _ length: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderGetFieldString(raw, index, capacity, value, length))
}

@_cdecl("compression_rs_aa_header_get_field_hash")
public func compressionRsAAHeaderGetFieldHash(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ capacity: Int,
    _ hashFunction: UnsafeMutablePointer<UInt32>?,
    _ value: UnsafeMutablePointer<UInt8>?
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    var hashFunctionStorage = __AAHashFunction(0)
    let status = Int32(__AAHeaderGetFieldHash(raw, index, capacity, &hashFunctionStorage, value))
    hashFunction?.pointee = hashFunctionStorage
    return status
}

@_cdecl("compression_rs_aa_header_get_field_timespec")
public func compressionRsAAHeaderGetFieldTimespec(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ seconds: UnsafeMutablePointer<Int64>?,
    _ nanoseconds: UnsafeMutablePointer<Int64>?
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    var value = timespec()
    let status = Int32(__AAHeaderGetFieldTimespec(raw, index, &value))
    if status == 0 {
        seconds?.pointee = Int64(value.tv_sec)
        nanoseconds?.pointee = Int64(value.tv_nsec)
    }
    return status
}

@_cdecl("compression_rs_aa_header_get_field_blob")
public func compressionRsAAHeaderGetFieldBlob(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ size: UnsafeMutablePointer<UInt64>?,
    _ offset: UnsafeMutablePointer<UInt64>?
) -> Int32 {
    guard let handle, let size, let offset else { return -1 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAHeaderGetFieldBlob(raw, index, size, offset))
}

@_cdecl("compression_rs_aa_header_get_encoded_size")
public func compressionRsAAHeaderGetEncodedSize(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else { return 0 }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAHeaderGetEncodedSize(raw)
}

@_cdecl("compression_rs_aa_header_copy_encoded_data")
public func compressionRsAAHeaderCopyEncodedData(
    _ handle: UnsafeMutableRawPointer?,
    _ dst: UnsafeMutablePointer<UInt8>?
) -> Bool {
    guard let handle else { return false }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw else { return false }
    let size = __AAHeaderGetEncodedSize(raw)
    if size == 0 {
        return true
    }
    guard let src = __AAHeaderGetEncodedData(raw), let dst else { return false }
    memcpy(dst, src, size)
    return true
}

@_cdecl("compression_rs_aa_header_clone_from_raw")
public func compressionRsAAHeaderCloneFromRaw(
    _ rawHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let rawHandle,
          let clone = __AAHeaderClone(OpaquePointer(rawHandle))
    else {
        return nil
    }
    return retain(AAHeaderBox(raw: clone))
}

@_cdecl("compression_rs_aa_header_clone_raw")
public func compressionRsAAHeaderCloneRaw(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAHeaderBox = unretained(handle, as: AAHeaderBox.self)
    guard let raw = box.raw,
          let clone = __AAHeaderClone(raw)
    else {
        return nil
    }
    return UnsafeMutableRawPointer(clone)
}
