import AppleArchive
import Darwin

private func requiredConstBytePointer(_ pointer: UnsafePointer<UInt8>?) -> UnsafePointer<UInt8> {
    pointer ?? UnsafePointer<UInt8>(bitPattern: 0x1)!
}

@_cdecl("compression_rs_aa_entry_acl_blob_create")
public func compressionRsAAEntryACLBlobCreate() -> UnsafeMutableRawPointer? {
    guard let raw = __AAEntryACLBlobCreate() else { return nil }
    return retain(AAEntryACLBlobBox(raw: raw))
}

@_cdecl("compression_rs_aa_entry_acl_blob_create_with_encoded_data")
public func compressionRsAAEntryACLBlobCreateWithEncodedData(
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw = __AAEntryACLBlobCreateWithEncodedData(requiredConstBytePointer(data), dataSize) else { return nil }
    return retain(AAEntryACLBlobBox(raw: raw))
}

@_cdecl("compression_rs_aa_entry_acl_blob_create_with_path")
public func compressionRsAAEntryACLBlobCreateWithPath(
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ flags: UInt64
) -> UnsafeMutableRawPointer? {
    guard let dir, let path,
          let raw = __AAEntryACLBlobCreateWithPath(dir, path, __AAFlagSet(flags))
    else {
        return nil
    }
    return retain(AAEntryACLBlobBox(raw: raw))
}

@_cdecl("compression_rs_aa_entry_acl_blob_apply_to_path")
public func compressionRsAAEntryACLBlobApplyToPath(
    _ handle: UnsafeMutableRawPointer?,
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ flags: UInt64
) -> Int32 {
    guard let handle, let dir, let path else { return -1 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryACLBlobApplyToPath(raw, dir, path, __AAFlagSet(flags)))
}

@_cdecl("compression_rs_aa_entry_acl_blob_get_entry_count")
public func compressionRsAAEntryACLBlobGetEntryCount(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else { return 0 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAEntryACLBlobGetEntryCount(raw)
}

@_cdecl("compression_rs_aa_entry_acl_blob_get_entry")
public func compressionRsAAEntryACLBlobGetEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ tag: UnsafeMutablePointer<UInt32>?,
    _ perms: UnsafeMutablePointer<UInt64>?,
    _ flags: UnsafeMutablePointer<UInt64>?,
    _ qualifierType: UnsafeMutablePointer<UInt32>?,
    _ qualifierCapacity: Int,
    _ qualifierValue: UnsafeMutablePointer<UInt8>?,
    _ qualifierSize: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return -1 }
    var entry = __AAAccessControlEntry()
    let status = Int32(__AAEntryACLBlobGetEntry(raw, index, &entry, qualifierCapacity, qualifierValue, qualifierSize))
    tag?.pointee = entry.tag
    perms?.pointee = entry.perms
    flags?.pointee = entry.flags
    qualifierType?.pointee = entry.qualifier_type
    return status
}

@_cdecl("compression_rs_aa_entry_acl_blob_append_entry")
public func compressionRsAAEntryACLBlobAppendEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ tag: UInt32,
    _ perms: UInt64,
    _ flags: UInt64,
    _ qualifierType: UInt32,
    _ qualifierValue: UnsafePointer<UInt8>?,
    _ qualifierSize: Int
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return -1 }
    var entry = __AAAccessControlEntry()
    entry.tag = tag
    entry.perms = perms
    entry.flags = flags
    entry.qualifier_type = qualifierType
    return Int32(__AAEntryACLBlobAppendEntry(raw, &entry, requiredConstBytePointer(qualifierValue), qualifierSize))
}

@_cdecl("compression_rs_aa_entry_acl_blob_set_entry")
public func compressionRsAAEntryACLBlobSetEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ tag: UInt32,
    _ perms: UInt64,
    _ flags: UInt64,
    _ qualifierType: UInt32,
    _ qualifierValue: UnsafePointer<UInt8>?,
    _ qualifierSize: Int
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return -1 }
    var entry = __AAAccessControlEntry()
    entry.tag = tag
    entry.perms = perms
    entry.flags = flags
    entry.qualifier_type = qualifierType
    return Int32(__AAEntryACLBlobSetEntry(raw, index, &entry, requiredConstBytePointer(qualifierValue), qualifierSize))
}

@_cdecl("compression_rs_aa_entry_acl_blob_clear")
public func compressionRsAAEntryACLBlobClear(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryACLBlobClear(raw))
}

@_cdecl("compression_rs_aa_entry_acl_blob_remove_entry")
public func compressionRsAAEntryACLBlobRemoveEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryACLBlobRemoveEntry(raw, index))
}

@_cdecl("compression_rs_aa_entry_acl_blob_get_encoded_size")
public func compressionRsAAEntryACLBlobGetEncodedSize(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else { return 0 }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAEntryACLBlobGetEncodedSize(raw)
}

@_cdecl("compression_rs_aa_entry_acl_blob_copy_encoded_data")
public func compressionRsAAEntryACLBlobCopyEncodedData(
    _ handle: UnsafeMutableRawPointer?,
    _ dst: UnsafeMutablePointer<UInt8>?
) -> Bool {
    guard let handle else { return false }
    let box: AAEntryACLBlobBox = unretained(handle, as: AAEntryACLBlobBox.self)
    guard let raw = box.raw else { return false }
    let size = __AAEntryACLBlobGetEncodedSize(raw)
    if size == 0 {
        return true
    }
    guard let src = __AAEntryACLBlobGetEncodedData(raw), let dst else { return false }
    memcpy(dst, src, size)
    return true
}

@_cdecl("compression_rs_aa_entry_acl_blob_clone_from_raw")
public func compressionRsAAEntryACLBlobCloneFromRaw(
    _ rawHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let rawHandle else { return nil }
    let raw = OpaquePointer(rawHandle)
    let size = __AAEntryACLBlobGetEncodedSize(raw)
    if size == 0 {
        guard let clone = __AAEntryACLBlobCreate() else { return nil }
        return retain(AAEntryACLBlobBox(raw: clone))
    }
    guard let data = __AAEntryACLBlobGetEncodedData(raw),
          let clone = __AAEntryACLBlobCreateWithEncodedData(data, size)
    else {
        return nil
    }
    return retain(AAEntryACLBlobBox(raw: clone))
}

@_cdecl("compression_rs_aa_entry_acl_blob_clear_raw")
public func compressionRsAAEntryACLBlobClearRaw(_ rawHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let rawHandle else { return -1 }
    return Int32(__AAEntryACLBlobClear(OpaquePointer(rawHandle)))
}

@_cdecl("compression_rs_aa_entry_acl_blob_append_entry_raw")
public func compressionRsAAEntryACLBlobAppendEntryRaw(
    _ rawHandle: UnsafeMutableRawPointer?,
    _ tag: UInt32,
    _ perms: UInt64,
    _ flags: UInt64,
    _ qualifierType: UInt32,
    _ qualifierValue: UnsafePointer<UInt8>?,
    _ qualifierSize: Int
) -> Int32 {
    guard let rawHandle else { return -1 }
    var entry = __AAAccessControlEntry()
    entry.tag = tag
    entry.perms = perms
    entry.flags = flags
    entry.qualifier_type = qualifierType
    return Int32(__AAEntryACLBlobAppendEntry(OpaquePointer(rawHandle), &entry, requiredConstBytePointer(qualifierValue), qualifierSize))
}

@_cdecl("compression_rs_aa_entry_acl_blob_release")
public func compressionRsAAEntryACLBlobRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAEntryACLBlobBox.self)
}

@_cdecl("compression_rs_aa_entry_xat_blob_create")
public func compressionRsAAEntryXATBlobCreate() -> UnsafeMutableRawPointer? {
    guard let raw = __AAEntryXATBlobCreate() else { return nil }
    return retain(AAEntryXATBlobBox(raw: raw))
}

@_cdecl("compression_rs_aa_entry_xat_blob_create_with_encoded_data")
public func compressionRsAAEntryXATBlobCreateWithEncodedData(
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw = __AAEntryXATBlobCreateWithEncodedData(data, dataSize) else { return nil }
    return retain(AAEntryXATBlobBox(raw: raw))
}

@_cdecl("compression_rs_aa_entry_xat_blob_create_with_path")
public func compressionRsAAEntryXATBlobCreateWithPath(
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ flags: UInt64
) -> UnsafeMutableRawPointer? {
    guard let dir, let path,
          let raw = __AAEntryXATBlobCreateWithPath(dir, path, __AAFlagSet(flags))
    else {
        return nil
    }
    return retain(AAEntryXATBlobBox(raw: raw))
}

@_cdecl("compression_rs_aa_entry_xat_blob_apply_to_path")
public func compressionRsAAEntryXATBlobApplyToPath(
    _ handle: UnsafeMutableRawPointer?,
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ flags: UInt64
) -> Int32 {
    guard let handle, let dir, let path else { return -1 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryXATBlobApplyToPath(raw, dir, path, __AAFlagSet(flags)))
}

@_cdecl("compression_rs_aa_entry_xat_blob_get_entry_count")
public func compressionRsAAEntryXATBlobGetEntryCount(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else { return 0 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAEntryXATBlobGetEntryCount(raw)
}

@_cdecl("compression_rs_aa_entry_xat_blob_get_entry")
public func compressionRsAAEntryXATBlobGetEntry(
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
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryXATBlobGetEntry(raw, index, keyCapacity, key, keyLength, dataCapacity, data, dataSize))
}

@_cdecl("compression_rs_aa_entry_xat_blob_append_entry")
public func compressionRsAAEntryXATBlobAppendEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> Int32 {
    guard let handle, let key else { return -1 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryXATBlobAppendEntry(raw, key, data, dataSize))
}

@_cdecl("compression_rs_aa_entry_xat_blob_set_entry")
public func compressionRsAAEntryXATBlobSetEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32,
    _ key: UnsafePointer<CChar>?,
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> Int32 {
    guard let handle, let key else { return -1 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryXATBlobSetEntry(raw, index, key, data, dataSize))
}

@_cdecl("compression_rs_aa_entry_xat_blob_clear")
public func compressionRsAAEntryXATBlobClear(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryXATBlobClear(raw))
}

@_cdecl("compression_rs_aa_entry_xat_blob_remove_entry")
public func compressionRsAAEntryXATBlobRemoveEntry(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAEntryXATBlobRemoveEntry(raw, index))
}

@_cdecl("compression_rs_aa_entry_xat_blob_get_encoded_size")
public func compressionRsAAEntryXATBlobGetEncodedSize(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle else { return 0 }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAEntryXATBlobGetEncodedSize(raw)
}

@_cdecl("compression_rs_aa_entry_xat_blob_copy_encoded_data")
public func compressionRsAAEntryXATBlobCopyEncodedData(
    _ handle: UnsafeMutableRawPointer?,
    _ dst: UnsafeMutablePointer<UInt8>?
) -> Bool {
    guard let handle else { return false }
    let box: AAEntryXATBlobBox = unretained(handle, as: AAEntryXATBlobBox.self)
    guard let raw = box.raw else { return false }
    let size = __AAEntryXATBlobGetEncodedSize(raw)
    if size == 0 {
        return true
    }
    guard let src = __AAEntryXATBlobGetEncodedData(raw), let dst else { return false }
    memcpy(dst, src, size)
    return true
}

@_cdecl("compression_rs_aa_entry_xat_blob_clone_from_raw")
public func compressionRsAAEntryXATBlobCloneFromRaw(
    _ rawHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let rawHandle else { return nil }
    let raw = OpaquePointer(rawHandle)
    let size = __AAEntryXATBlobGetEncodedSize(raw)
    if size == 0 {
        guard let clone = __AAEntryXATBlobCreate() else { return nil }
        return retain(AAEntryXATBlobBox(raw: clone))
    }
    guard let data = __AAEntryXATBlobGetEncodedData(raw),
          let clone = __AAEntryXATBlobCreateWithEncodedData(data, size)
    else {
        return nil
    }
    return retain(AAEntryXATBlobBox(raw: clone))
}

@_cdecl("compression_rs_aa_entry_xat_blob_clear_raw")
public func compressionRsAAEntryXATBlobClearRaw(_ rawHandle: UnsafeMutableRawPointer?) -> Int32 {
    guard let rawHandle else { return -1 }
    return Int32(__AAEntryXATBlobClear(OpaquePointer(rawHandle)))
}

@_cdecl("compression_rs_aa_entry_xat_blob_append_entry_raw")
public func compressionRsAAEntryXATBlobAppendEntryRaw(
    _ rawHandle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ data: UnsafePointer<UInt8>?,
    _ dataSize: Int
) -> Int32 {
    guard let rawHandle, let key else { return -1 }
    return Int32(__AAEntryXATBlobAppendEntry(OpaquePointer(rawHandle), key, data, dataSize))
}

@_cdecl("compression_rs_aa_entry_xat_blob_release")
public func compressionRsAAEntryXATBlobRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAEntryXATBlobBox.self)
}
