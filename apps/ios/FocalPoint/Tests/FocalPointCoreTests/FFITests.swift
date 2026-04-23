import XCTest
@testable import FocalPointCore

final class FFITests: XCTestCase {
    private func makeCore() throws -> FocalPointCore {
        let dir = FileManager.default.temporaryDirectory
            .appendingPathComponent("focalpoint-ffi-tests-\(UUID().uuidString)", isDirectory: true)
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        let db = dir.appendingPathComponent("core.db").path
        return try FocalPointCore(storagePath: db)
    }

    func testFocalPointCoreIdleEvent() throws {
        let core = try makeCore()
        let state = core.pushMascotEvent(event: .idle)
        XCTAssertEqual(state.pose, .idle)
    }

    func testAppVersionNonEmpty() throws {
        let core = try makeCore()
        XCTAssertFalse(core.appVersion().isEmpty)
    }
}
