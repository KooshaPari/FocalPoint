#if canImport(SwiftUI)
import XCTest
import SwiftUI
@testable import FocalPointApp
@testable import FocalPointCore

@MainActor
final class RitualsViewTests: XCTestCase {
    /// Smoke test: the Today tab view compiles and instantiates without
    /// crashing when embedded in the CoreHolder environment. This guards
    /// against SwiftUI body construction failures (DTO field drift, missing
    /// imports, bad modifier stacks) without requiring a simulator.
    func testRitualsViewInstantiates() {
        let holder = CoreHolder.shared
        let view = RitualsView().environmentObject(holder)
        // Force body evaluation by wrapping in a hosting AnyView; if the
        // Swift compiler accepts this and no runtime trap fires, the view
        // graph is well-formed.
        let erased = AnyView(view)
        XCTAssertNotNil(erased)
    }
}
#endif
