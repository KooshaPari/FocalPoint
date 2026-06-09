import XCTest
@testable import FocalPointCore
final class WindowManagerTests: XCTestCase {
    func testStubName() { XCTAssertEqual(StubWindowManager().name, "stub") }
    func testFocusNoOp() async throws { try await StubWindowManager().focus(windowId: 1) }
    func testMoveNoOp() async throws { try await StubWindowManager().move(windowId: 1, toScreen: 0) }
    func testResizeNoOp() async throws { try await StubWindowManager().resize(windowId: 1, width: 100, height: 100) }
    func testListWindowsEmpty() async throws {
        let w = try await StubWindowManager().listWindows()
        XCTAssertTrue(w.isEmpty)
    }
}
