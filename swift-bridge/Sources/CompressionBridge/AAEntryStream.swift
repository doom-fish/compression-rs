import AppleArchive
import Darwin

@_cdecl("compression_rs_aa_path_list_create_with_directory_contents")
public func compressionRsAAPathListCreateWithDirectoryContents(
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?,
    _ flags: UInt64,
    _ nThreads: Int32
) -> UnsafeMutableRawPointer? {
    guard let dir,
          let raw = __AAPathListCreateWithDirectoryContents(dir, path, nil, nil, __AAFlagSet(flags), nThreads)
    else {
        return nil
    }
    return retain(AAPathListBox(raw: raw))
}

@_cdecl("compression_rs_aa_path_list_create_with_path")
public func compressionRsAAPathListCreateWithPath(
    _ dir: UnsafePointer<CChar>?,
    _ path: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let dir, let path else { return nil }
    guard #available(macOS 12.0, *) else { return nil }
    guard let raw = __AAPathListCreateWithPath(dir, path) else { return nil }
    return retain(AAPathListBox(raw: raw))
}

@_cdecl("compression_rs_aa_path_list_release")
public func compressionRsAAPathListRelease(_ handle: UnsafeMutableRawPointer?) {
    release(handle, as: AAPathListBox.self)
}

@_cdecl("compression_rs_aa_path_list_node_get_path")
public func compressionRsAAPathListNodeGetPath(
    _ handle: UnsafeMutableRawPointer?,
    _ node: UInt64,
    _ capacity: Int,
    _ path: UnsafeMutablePointer<CChar>?,
    _ length: UnsafeMutablePointer<Int>?
) -> Int32 {
    guard let handle else { return -1 }
    let box: AAPathListBox = unretained(handle, as: AAPathListBox.self)
    guard let raw = box.raw else { return -1 }
    return Int32(__AAPathListNodeGetPath(raw, node, capacity, path, length))
}

@_cdecl("compression_rs_aa_path_list_node_first")
public func compressionRsAAPathListNodeFirst(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle else { return UInt64.max }
    let box: AAPathListBox = unretained(handle, as: AAPathListBox.self)
    guard let raw = box.raw else { return UInt64.max }
    return __AAPathListNodeFirst(raw)
}

@_cdecl("compression_rs_aa_path_list_node_next")
public func compressionRsAAPathListNodeNext(
    _ handle: UnsafeMutableRawPointer?,
    _ node: UInt64
) -> UInt64 {
    guard let handle else { return UInt64.max }
    let box: AAPathListBox = unretained(handle, as: AAPathListBox.self)
    guard let raw = box.raw else { return UInt64.max }
    return __AAPathListNodeNext(raw, node)
}
