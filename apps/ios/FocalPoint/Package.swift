// swift-tools-version: 5.9
import PackageDescription

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
        .library(name: "UnlockProof", targets: ["UnlockProof"]),
        .library(name: "FocalPointCore", targets: ["FocalPointCore"]),
        .executable(name: "FocalPointApp", targets: ["FocalPointApp"]),
    ],
    dependencies: [],
    targets: [
        .target(
            name: "DesignSystem",
            path: "Sources/DesignSystem"
        ),
        .target(
            name: "FocalPointCore",
            path: "Sources/FocalPointCore",
            // UniFFI-generated bindings require a linked Rust static lib.
            // For SwiftPM host builds + initial iOS simulator deploy we exclude
            // them; a follow-up Phase 1 task wraps the Rust staticlib in an
            // XCFramework and re-adds focus_ffi.swift + modulemap.
            exclude: [
                "focus_ffi.swift",
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
        .target(
            name: "UnlockProof",
            path: "Sources/UnlockProof"
        ),
        .executableTarget(
            name: "FocalPointApp",
            dependencies: [
                "DesignSystem",
                "MascotUI",
                "Enforcement",
                "UnlockProof",
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
            name: "EnforcementTests",
            dependencies: ["Enforcement"],
            path: "Tests/EnforcementTests"
        ),
    ]
)
