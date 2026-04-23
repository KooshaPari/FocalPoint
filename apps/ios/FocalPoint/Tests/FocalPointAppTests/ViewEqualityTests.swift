import XCTest
@testable import FocalPointApp
@testable import FocalPointCore
import MascotUI

@MainActor
final class ViewEqualityTests: XCTestCase {
    /// Hand-rolled "snapshot" — assert key structural fields on the ViewModel
    /// state that backs each new view. Full SwiftUI snapshots are out of scope
    /// (no snapshot dependency), so we settle for invariant checks.

    func testHomeViewCoachyBridgingMapsPoses() {
        let state = MascotState(pose: .celebratory, emotion: .excited, sinceIso: "2026-01-01", bubbleText: "Nice")
        let mapped = CoachyBridging.coachyState(from: state)
        XCTAssertEqual(mapped.pose, .celebratory)
        XCTAssertEqual(mapped.emotion, .excited)
        XCTAssertEqual(mapped.bubbleText, "Nice")
    }

    func testRuleTemplatesAreUnique() {
        let ids = RuleTemplates.all.map(\.id)
        XCTAssertEqual(Set(ids).count, ids.count, "template ids must be unique")
        let ruleIds = RuleTemplates.all.map(\.draft.id)
        XCTAssertEqual(Set(ruleIds).count, ruleIds.count, "rule draft ids must be unique")
    }

    func testCanvasAuthorizeUrlShape() throws {
        let url = try CanvasAuth.authorizeURL(instanceUrl: "university.instructure.com")
        XCTAssertEqual(url.host, "university.instructure.com")
        XCTAssertEqual(url.path, "/login/oauth2/auth")
        let comps = URLComponents(url: url, resolvingAgainstBaseURL: false)!
        let q = Dictionary(uniqueKeysWithValues: (comps.queryItems ?? []).map { ($0.name, $0.value ?? "") })
        XCTAssertEqual(q["response_type"], "code")
        XCTAssertEqual(q["redirect_uri"], "focalpoint://auth/canvas/callback")
        XCTAssertFalse((q["state"] ?? "").isEmpty)
    }

    func testCanvasAuthorizeUrlRejectsBareToken() {
        XCTAssertThrowsError(try CanvasAuth.authorizeURL(instanceUrl: "bogus"))
    }

    func testCooldownChoiceRoundTrip() {
        XCTAssertEqual(CooldownChoice.closest(toSeconds: nil), .off)
        XCTAssertEqual(CooldownChoice.closest(toSeconds: 300), .fiveMinutes)
        XCTAssertEqual(CooldownChoice.closest(toSeconds: 3600), .oneHour)
        XCTAssertEqual(CooldownChoice.closest(toSeconds: 86400), .oneDay)
        XCTAssertEqual(CooldownChoice.oneHour.seconds, 3600)
    }
}
