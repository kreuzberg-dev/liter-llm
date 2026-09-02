// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/xberg-io/liter-llm", branch: "release/swift/1.19.2"),
    ],
    targets: [
        .testTarget(
            name: "LiterLlmE2ETests",
            dependencies: [.product(name: "LiterLlm", package: "liter-llm")]
        ),
    ]
)
