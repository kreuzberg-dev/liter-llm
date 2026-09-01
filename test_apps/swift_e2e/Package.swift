// swift-tools-version: 6.0
// The first-party dependency pin below is managed by alef (sync.text_replacements); do not edit it by hand.
// alef:hash:cf25e1aa9836370c1ac923dd9b2891aef84a9a096c5d86ba1f4bce71eac3daba
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/xberg-io/liter-llm", branch: "release/swift/1.19.1"),
    ],
    targets: [
        .testTarget(
            name: "LiterLlmE2ETests",
            dependencies: [.product(name: "LiterLlm", package: "liter-llm")]
        ),
    ]
)
