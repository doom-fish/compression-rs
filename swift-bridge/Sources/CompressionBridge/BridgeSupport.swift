import AppleArchive
import Darwin

func retain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

func unretained<T: AnyObject>(_ handle: UnsafeMutableRawPointer, as type: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(handle).takeUnretainedValue()
}

func release<T: AnyObject>(_ handle: UnsafeMutableRawPointer?, as type: T.Type) {
    guard let handle else { return }
    Unmanaged<T>.fromOpaque(handle).release()
}

func aaFieldKey(_ rawValue: UInt32) -> __AAFieldKey {
    __AAFieldKey(ikey: rawValue)
}

func rawFieldKey(_ key: __AAFieldKey) -> UInt32 {
    key.ikey
}

final class AAByteStreamBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    func close() -> Int32 {
        guard let raw else { return 0 }
        let status = Int32(__AAByteStreamClose(raw))
        self.raw = nil
        return status
    }

    deinit {
        if let raw {
            _ = __AAByteStreamClose(raw)
        }
    }
}

final class AAArchiveStreamBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    func close() -> Int32 {
        guard let raw else { return 0 }
        let status = Int32(__AAArchiveStreamClose(raw))
        self.raw = nil
        return status
    }

    deinit {
        if let raw {
            _ = __AAArchiveStreamClose(raw)
        }
    }
}

final class AAHeaderBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            __AAHeaderDestroy(raw)
        }
    }
}

final class AAFieldKeySetBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            __AAFieldKeySetDestroy(raw)
        }
    }
}

final class AAPathListBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            __AAPathListDestroy(raw)
        }
    }
}

final class AAEntryACLBlobBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            __AAEntryACLBlobDestroy(raw)
        }
    }
}

final class AAEntryXATBlobBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            __AAEntryXATBlobDestroy(raw)
        }
    }
}

final class AEAAuthDataBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            if #available(macOS 11.3, *) {
                __AEAAuthDataDestroy(raw)
            }
        }
    }
}

final class AEAContextBox {
    var raw: OpaquePointer?

    init(raw: OpaquePointer) {
        self.raw = raw
    }

    deinit {
        if let raw {
            __AEAContextDestroy(raw)
        }
    }
}
