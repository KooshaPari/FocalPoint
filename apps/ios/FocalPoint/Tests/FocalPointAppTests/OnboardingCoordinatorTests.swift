import XCTest
@testable import FocalPointApp
@testable import FocalPointCore

@MainActor
final class OnboardingCoordinatorTests: XCTestCase {
    func testInitialStepIsWelcome() {
        let c = OnboardingCoordinator()
        XCTAssertEqual(c.step, .welcome)
        XCTAssertTrue(c.canAdvance)
    }

    func testGoalSelectionMinMax() {
        let c = OnboardingCoordinator()
        c.jump(to: .goals)
        XCTAssertFalse(c.canAdvance, "0 goals shouldn't advance")
        c.toggleGoal(.school)
        XCTAssertTrue(c.canAdvance)
        c.toggleGoal(.work)
        c.toggleGoal(.sleep)
        XCTAssertEqual(c.goals.count, 3)
        c.toggleGoal(.exercise) // should be rejected — cap is 3
        XCTAssertEqual(c.goals.count, 3)
    }

    func testAdvanceBlockedWithoutTemplate() {
        let c = OnboardingCoordinator()
        c.jump(to: .pickTemplate)
        XCTAssertFalse(c.canAdvance)
        c.selectedTemplateId = RuleTemplates.deepWorkSocialBlock.id
        XCTAssertTrue(c.canAdvance)
    }

    func testFullFlowProgression() {
        let c = OnboardingCoordinator()
        c.advance() // welcome -> goals
        XCTAssertEqual(c.step, .goals)
        c.toggleGoal(.school)
        c.advance() // goals -> connect
        XCTAssertEqual(c.step, .connect)
        c.advance() // connect -> pickTemplate
        XCTAssertEqual(c.step, .pickTemplate)
        c.selectedTemplateId = RuleTemplates.eveningWindDown.id
        c.advance() // pickTemplate -> permissions
        XCTAssertEqual(c.step, .permissions)
        XCTAssertTrue(c.isFinalStep)
    }

    func testBackTraversal() {
        let c = OnboardingCoordinator()
        c.jump(to: .pickTemplate)
        c.back()
        XCTAssertEqual(c.step, .connect)
        c.back()
        XCTAssertEqual(c.step, .goals)
        c.back()
        XCTAssertEqual(c.step, .welcome)
        c.back() // at 0 — should stay
        XCTAssertEqual(c.step, .welcome)
    }
}
