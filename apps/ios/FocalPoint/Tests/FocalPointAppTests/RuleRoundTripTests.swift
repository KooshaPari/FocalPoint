import XCTest
@testable import FocalPointApp
@testable import FocalPointCore

@MainActor
final class RuleRoundTripTests: XCTestCase {
    private func makeCore() throws -> FocalPointCore {
        let dir = FileManager.default.temporaryDirectory
            .appendingPathComponent("focalpoint-rule-tests-\(UUID().uuidString)", isDirectory: true)
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        return try FocalPointCore(storagePath: dir.appendingPathComponent("core.db").path)
    }

    func testTemplateRoundTrip() throws {
        let core = try makeCore()
        let t = RuleTemplates.deepWorkSocialBlock
        try core.mutations().upsert(rule: t.draft)
        let listed = try core.rules().listEnabled()
        let found = listed.first(where: { $0.id == t.draft.id })
        XCTAssertNotNil(found, "rule should round-trip through the core")
        XCTAssertEqual(found?.name, t.draft.name)
        XCTAssertEqual(found?.priority, t.draft.priority)
        XCTAssertEqual(found?.explanationTemplate, t.draft.explanationTemplate)
        XCTAssertEqual(found?.enabled, t.draft.enabled)
    }

    func testToggleEnabledReflectsInList() throws {
        let core = try makeCore()
        let t = RuleTemplates.eveningWindDown
        try core.mutations().upsert(rule: t.draft)
        try core.mutations().setEnabled(ruleId: t.draft.id, enabled: false)
        let listed = try core.rules().listEnabled()
        XCTAssertNil(
            listed.first(where: { $0.id == t.draft.id }),
            "disabled rule should not appear in listEnabled()"
        )
    }
}
