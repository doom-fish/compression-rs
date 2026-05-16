// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CompressionBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "CompressionBridge",
            type: .static,
            targets: ["CompressionBridge"]
        )
    ],
    targets: [
        .target(
            name: "CompressionBridge",
            path: "Sources/CompressionBridge"
        )
    ]
)
