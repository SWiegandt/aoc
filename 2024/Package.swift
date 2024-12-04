// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let executableTargets = (1...4).map { d in
    Target.executableTarget(name: "Day\(d)", dependencies: ["Util"])
}

let package = Package(
    name: "aoc",
    platforms: [.macOS("14.6")],
    targets: [
        .target(name: "Util"),
        .testTarget(
            name: "aoc",
            dependencies: executableTargets.map { .byName(name: $0.name) }
        ),
    ] + executableTargets
)
