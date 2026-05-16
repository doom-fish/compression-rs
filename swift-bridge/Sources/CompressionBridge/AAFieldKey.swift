import AppleArchive
import Darwin

@_cdecl("compression_rs_aa_field_key_set_create")
public func compressionRsAAFieldKeySetCreate() -> UnsafeMutableRawPointer? {
    guard let raw = __AAFieldKeySetCreate() else { return nil }
    return retain(AAFieldKeySetBox(raw: raw))
}

@_cdecl("compression_rs_aa_field_key_set_create_with_string")
public func compressionRsAAFieldKeySetCreateWithString(
    _ value: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let value, let raw = __AAFieldKeySetCreateWithString(value) else { return nil }
    return retain(AAFieldKeySetBox(raw: raw))
}

@_cdecl("compression_rs_aa_field_key_set_clone")
public func compressionRsAAFieldKeySetClone(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw, let clone = __AAFieldKeySetClone(raw) else { return nil }
    return retain(AAFieldKeySetBox(raw: clone))
}

@_cdecl("compression_rs_aa_field_key_set_release")
public func compressionRsAAFieldKeySetRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAFieldKeySetBox.self)
}

@_cdecl("compression_rs_aa_field_key_set_clear")
public func compressionRsAAFieldKeySetClear(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let handle else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAFieldKeySetClear(raw))
}

@_cdecl("compression_rs_aa_field_key_set_contains_key")
public func compressionRsAAFieldKeySetContainsKey(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAFieldKeySetContainsKey(raw, aaFieldKey(key)))
}

@_cdecl("compression_rs_aa_field_key_set_insert_key")
public func compressionRsAAFieldKeySetInsertKey(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAFieldKeySetInsertKey(raw, aaFieldKey(key)))
}

@_cdecl("compression_rs_aa_field_key_set_remove_key")
public func compressionRsAAFieldKeySetRemoveKey(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UInt32
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAFieldKeySetRemoveKey(raw, aaFieldKey(key)))
}

@_cdecl("compression_rs_aa_field_key_set_insert_key_set")
public func compressionRsAAFieldKeySetInsertKeySet(
    _ handle: UnsafeMutableRawPointer?,
    _ other: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let other else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    let otherBox: AAFieldKeySetBox = unretained(other, as: AAFieldKeySetBox.self)
    guard let raw = box.raw, let otherRaw = otherBox.raw else { return -1 }
    return Int32(__AAFieldKeySetInsertKeySet(raw, otherRaw))
}

@_cdecl("compression_rs_aa_field_key_set_remove_key_set")
public func compressionRsAAFieldKeySetRemoveKeySet(
    _ handle: UnsafeMutableRawPointer?,
    _ other: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let other else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    let otherBox: AAFieldKeySetBox = unretained(other, as: AAFieldKeySetBox.self)
    guard let raw = box.raw, let otherRaw = otherBox.raw else { return -1 }
    return Int32(__AAFieldKeySetRemoveKeySet(raw, otherRaw))
}

@_cdecl("compression_rs_aa_field_key_set_select_key_set")
public func compressionRsAAFieldKeySetSelectKeySet(
    _ handle: UnsafeMutableRawPointer?,
    _ other: UnsafeMutableRawPointer?
) -> Int32 {
    guard let handle, let other else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    let otherBox: AAFieldKeySetBox = unretained(other, as: AAFieldKeySetBox.self)
    guard let raw = box.raw, let otherRaw = otherBox.raw else { return -1 }
    return Int32(__AAFieldKeySetSelectKeySet(raw, otherRaw))
}

@_cdecl("compression_rs_aa_field_key_set_get_key_count")
public func compressionRsAAFieldKeySetGetKeyCount(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    guard let handle else { return 0 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return 0 }
    return __AAFieldKeySetGetKeyCount(raw)
}

@_cdecl("compression_rs_aa_field_key_set_get_key")
public func compressionRsAAFieldKeySetGetKey(
    _ handle: UnsafeMutableRawPointer?,
    _ index: UInt32
) -> UInt32 {
    guard let handle else { return 0 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return 0 }
    return rawFieldKey(__AAFieldKeySetGetKey(raw, index))
}

@_cdecl("compression_rs_aa_field_key_set_serialize")
public func compressionRsAAFieldKeySetSerialize(
    _ handle: UnsafeMutableRawPointer?,
    _ capacity: Int,
    _ buffer: UnsafeMutablePointer<CChar>?
) -> Int32 {
    guard let handle, let buffer else { return -1 }
    let box: AAFieldKeySetBox = unretained(handle, as: AAFieldKeySetBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAFieldKeySetSerialize(raw, capacity, buffer))
}
