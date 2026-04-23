import XCTest
@testable import MascotUI

final class CoachyStateTests: XCTestCase {
    func testPoseHasSevenVariants() {
        XCTAssertEqual(CoachyPose.allCases.count, 7)
    }

    func testEmotionHasEightVariants() {
        XCTAssertEqual(CoachyEmotion.allCases.count, 8)
    }

    func testPlaceholderIsEncouragingHappy() {
        let s = CoachyState.placeholder
        XCTAssertEqual(s.pose, .encouraging)
        XCTAssertEqual(s.emotion, .happy)
        XCTAssertNotNil(s.bubbleText)
    }

    func testStateEquatableByPoseAndEmotion() {
        let a = CoachyState(pose: .stern, emotion: .disappointed, bubbleText: "focus")
        let b = CoachyState(pose: .stern, emotion: .disappointed, bubbleText: "focus")
        XCTAssertEqual(a, b)
    }
}
