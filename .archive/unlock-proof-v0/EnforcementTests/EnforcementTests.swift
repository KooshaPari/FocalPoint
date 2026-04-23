import XCTest
@testable import Enforcement

final class EnforcementTests: XCTestCase {
    func testStubDriverApplyAndRetractDoNotThrow() {
        let driver = StubEnforcementDriver()
        let policy = EnforcementPolicy(
            blockedBundleIds: ["com.example.social"],
            endsAt: Date().addingTimeInterval(60)
        )
        driver.apply(policy: policy)
        driver.retract()
    }

    func testPolicyValuesStored() {
        let ends = Date().addingTimeInterval(30)
        let p = EnforcementPolicy(blockedBundleIds: ["a", "b"], endsAt: ends)
        XCTAssertEqual(p.blockedBundleIds, ["a", "b"])
        XCTAssertEqual(p.endsAt, ends)
    }
}
