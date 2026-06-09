// SPDX-License-Identifier: MIT OR Apache-2.0
// T66: FocalPoint hexagonal port — WindowManager.
// 3 adapters: YabaiAdapter, AmethystAdapter, AeroSpaceAdapter.
import Foundation
public protocol WindowManager: Sendable {
    var name: String { get }
    func focus(windowId: UInt32) async throws
    func move(windowId: UInt32, toScreen screen: UInt32) async throws
    func resize(windowId: UInt32, width: UInt32, height: UInt32) async throws
    func listWindows() async throws -> [Window]
}
public struct Window: Sendable, Equatable {
    public let id: UInt32
    public let title: String
    public let app: String
    public let screen: UInt32
    public let frame: CGRect
    public init(id: UInt32, title: String, app: String, screen: UInt32, frame: CGRect) {
        self.id = id; self.title = title; self.app = app; self.screen = screen; self.frame = frame
    }
}
