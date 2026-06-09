// Stub adapter: returns an empty list. Real Yabai/Amethyst/AeroSpace adapters live
// in Sources/FocalPointCore/Adapters/{Yabai,Amethyst,AeroSpace}.swift in T66 follow-up.
import Foundation
public final class StubWindowManager: WindowManager {
    public let name = "stub"
    public init() {}
    public func focus(windowId: UInt32) async throws {}
    public func move(windowId: UInt32, toScreen screen: UInt32) async throws {}
    public func resize(windowId: UInt32, width: UInt32, height: UInt32) async throws {}
    public func listWindows() async throws -> [Window] { [] }
}
