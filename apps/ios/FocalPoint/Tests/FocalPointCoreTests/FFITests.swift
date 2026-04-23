import XCTest
@testable import FocalPointCore

final class FFITests: XCTestCase {
    func testFocalPointCoreIdleEvent() throws {
        let core = FocalPointCore()
        let state = core.pushMascotEvent(event: .idle)
        XCTAssertEqual(state.pose, .idle)
    }

    func testAppVersionNonEmpty() throws {
        let core = FocalPointCore()
        XCTAssertFalse(core.appVersion().isEmpty)
    }
}
