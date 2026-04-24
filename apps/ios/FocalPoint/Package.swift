// swift-tools-version: 5.9
import PackageDescription

// UnlockProof + EnforcementTests targets archived 2026-04-22 (connector+reward
// gamification prioritized; QR/NFC proofs deferred). See
// `.archive/unlock-proof-v0/`.

let package = Package(
    name: "FocalPoint",
    defaultLocalization: "en",
    platforms: [
        .iOS(.v17),
        .macOS(.v13),
    ],
    products: [
        .library(name: "DesignSystem", targets: ["DesignSystem"]),
        .library(name: "MascotUI", targets: ["MascotUI"]),
        .library(name: "Enforcement", targets: ["Enforcement"]),
        .library(name: "FocalPointCore", targets: ["FocalPointCore"]),
        .executable(name: "FocalPointApp", targets: ["FocalPointApp"]),
    ],
    dependencies: [
        .package(url: "https://github.com/pointfreeco/swift-snapshot-testing.git", from: "1.16.0"),
    ],
    targets: [
        .target(
            name: "DesignSystem",
            path: "Sources/DesignSystem"
        ),
        .binaryTarget(
            name: "focus_ffiFFI",
            path: "Frameworks/FocusFFI.xcframework"
        ),
        .target(
            name: "FocalPointCore",
            dependencies: ["focus_ffiFFI"],
            path: "Sources/FocalPointCore",
            // C header + modulemap ship inside the XCFramework; exclude the
            // SwiftPM-local copies to avoid duplicate module definitions.
            exclude: [
                "focus_ffiFFI.h",
                "focus_ffiFFI.modulemap",
            ]
        ),
        .target(
            name: "MascotUI",
            dependencies: ["DesignSystem", "FocalPointCore"],
            path: "Sources/MascotUI"
        ),
        .target(
            name: "Enforcement",
            dependencies: ["FocalPointCore"],
            path: "Sources/Enforcement"
        ),
        .executableTarget(
            name: "FocalPointApp",
            dependencies: [
                "DesignSystem",
                "MascotUI",
                "Enforcement",
                "FocalPointCore",
            ],
            path: "Sources/FocalPointApp"
        ),
        .testTarget(
            name: "DesignSystemTests",
            dependencies: ["DesignSystem"],
            path: "Tests/DesignSystemTests"
        ),
        .testTarget(
            name: "MascotUITests",
            dependencies: ["MascotUI"],
            path: "Tests/MascotUITests"
        ),
        .testTarget(
            name: "FocalPointCoreTests",
            dependencies: ["FocalPointCore"],
            path: "Tests/FocalPointCoreTests"
        ),
        .testTarget(
            name: "FocalPointAppTests",
            dependencies: ["FocalPointApp", "FocalPointCore"],
            path: "Tests/FocalPointAppTests"
        ),
        .testTarget(
            name: "FocalPointAppSnapshotTests",
            dependencies: [
                "FocalPointApp",
                "FocalPointCore",
                "DesignSystem",
                "MascotUI",
                "Enforcement",
                .product(name: "SnapshotTesting", package: "swift-snapshot-testing"),
            ],
            path: "Tests/FocalPointAppSnapshotTests"
        ),
    ]
)
