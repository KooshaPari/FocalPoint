import XCTest
@testable import FocalPointApp

/// Comprehensive snapshot tests for Coachy conversational responses.
/// Covers four key scenarios: fresh-morning, mid-day-all-done, overdue-state, and low-credits state.
final class CoachyConversationTests: XCTestCase {
    /// Test Coachy's response on a fresh Monday morning with pending tasks.
    func testFreshMorningResponse() {
        let state = CoachyState(
            taskCount: 5,
            completedToday: 0,
            credits: 300,
            streakDays: 3,
            ritualStatus: "Morning Brief pending",
            weekdayIndex: 1,  // Monday
            timeOfDay: .morning
        )

        let (response, emotion) = CoachyResponseSynth.generateResponse(state: state)

        XCTAssertFalse(response.isEmpty, "Response should not be empty")
        XCTAssertTrue(
            response.contains("task") || response.contains("Task"),
            "Should mention tasks"
        )
        XCTAssertTrue(
            response.contains("300") || response.contains("credits"),
            "Should mention credits"
        )
        // On Monday (weekdayIndex 1), phrase index 1 is used
        XCTAssertTrue(
            response.contains("Status") || response.contains("status"),
            "Morning response should reference status"
        )
        // Emotion should be neutral or supportive for pending work
        XCTAssertTrue(
            emotion == .neutral || emotion == .concerned,
            "Fresh morning with pending work should be neutral or concerned"
        )
    }

    /// Test Coachy's response when all tasks are completed mid-day.
    func testMidDayAllDoneResponse() {
        let state = CoachyState(
            taskCount: 4,
            completedToday: 4,  // All done
            credits: 450,
            streakDays: 7,
            ritualStatus: "all caught up",
            weekdayIndex: 3,  // Wednesday
            timeOfDay: .afternoon
        )

        let (response, emotion) = CoachyResponseSynth.generateResponse(state: state)

        XCTAssertFalse(response.isEmpty)
        XCTAssertTrue(
            response.lowercased().contains("crushed") || response.lowercased().contains("all"),
            "Should acknowledge task completion"
        )
        XCTAssertTrue(
            emotion == .excited || emotion == .happy || emotion == .proud,
            "Should be positive when all tasks are done"
        )
        XCTAssertTrue(
            response.contains("7") || response.contains("streak"),
            "Should reference 7-day streak"
        )
    }

    /// Test Coachy's response when user is low on credits and has overdue work.
    func testOverdueStateLowCreditsResponse() {
        let state = CoachyState(
            taskCount: 8,
            completedToday: 1,
            credits: 25,
            streakDays: 0,
            ritualStatus: "Evening Ritual overdue",
            weekdayIndex: 5,  // Friday
            timeOfDay: .evening
        )

        let (response, emotion) = CoachyResponseSynth.generateResponse(state: state)

        XCTAssertFalse(response.isEmpty)
        XCTAssertTrue(
            response.contains("7 tasks") || response.contains("overdue"),
            "Should emphasize remaining work"
        )
        XCTAssertTrue(
            response.contains("25") || response.lowercased().contains("low"),
            "Should indicate low credit balance"
        )
        XCTAssertTrue(
            emotion == .concerned || emotion == .disappointed,
            "Should be concerned for overdue state"
        )
    }

    /// Test Coachy's response when user has zero tasks and zero credits.
    func testZeroStateResponse() {
        let state = CoachyState(
            taskCount: 0,
            completedToday: 0,
            credits: 0,
            streakDays: 0,
            ritualStatus: "no tasks",
            weekdayIndex: 0,  // Sunday
            timeOfDay: .night
        )

        let (response, emotion) = CoachyResponseSynth.generateResponse(state: state)

        XCTAssertFalse(response.isEmpty)
        XCTAssertTrue(
            response.lowercased().contains("haven't added") || response.lowercased().contains("out of credits"),
            "Should acknowledge zero state"
        )
        // Emotion should be encouraging or neutral
        XCTAssertTrue(
            emotion == .neutral || emotion == .encouraging,
            "Should be neutral or encouraging for empty state"
        )
    }

    /// Test that Coachy response varies by weekday (phrase rotation).
    func testResponseVariationByWeekday() {
        let baseState = CoachyState(
            taskCount: 3,
            completedToday: 1,
            credits: 200,
            streakDays: 2,
            ritualStatus: "all caught up",
            weekdayIndex: 0,
            timeOfDay: .afternoon
        )

        var responses = Set<String>()
        for dayIndex in 0..<7 {
            var dayState = baseState
            dayState.weekdayIndex = dayIndex
            let (response, _) = CoachyResponseSynth.generateResponse(state: dayState)
            responses.insert(response)
        }

        // Should generate at least 4 unique responses across different weekdays
        XCTAssertGreaterThanOrEqual(
            responses.count,
            4,
            "Should produce varied responses across different days"
        )
    }

    /// Test that Coachy's voice profile defaults correctly.
    func testVoiceProfileDefault() {
        let voice = CoachyVoiceProfile.default
        // Should return one of the two valid voice types
        let description = String(describing: voice)
        XCTAssertTrue(
            description.contains("systemSiri") || description.contains("avSynthesizer"),
            "Voice should default to either systemSiri or avSynthesizer"
        )
    }

    /// Test emotion-based voice parameters.
    func testEmotionVoiceParametersVariation() {
        let happyParams = CoachyVoiceProfile.parametersForEmotion(.happy)
        let tiredParams = CoachyVoiceProfile.parametersForEmotion(.tired)
        let neutralParams = CoachyVoiceProfile.parametersForEmotion(.neutral)

        // Happy should have higher pitch than tired
        XCTAssertGreaterThan(happyParams.pitch, tiredParams.pitch)

        // Happy should be faster than tired
        XCTAssertGreaterThan(happyParams.rate, tiredParams.rate)

        // Neutral should be baseline
        XCTAssertEqual(neutralParams.pitch, 1.0)
    }

    /// Test CoachyConversationIntent metadata.
    func testCoachyIntentMetadata() {
        let intent = CoachyConversationIntent(question: nil)
        XCTAssertEqual(
            CoachyConversationIntent.title.description,
            "Ask Coachy"
        )
        XCTAssertFalse(
            CoachyConversationIntent.openAppWhenRun,
            "Coachy should not open app when run"
        )
    }

    /// Test that intent is registered in AppShortcutsProvider.
    func testCoachyRegisteredInProvider() {
        let shortcuts = FocalPointAppShortcutsProvider.appShortcuts
        // Should have 7 shortcuts (original 6 + Coachy)
        XCTAssertGreaterThanOrEqual(shortcuts.count, 7)

        let coachyShortcut = shortcuts.first { shortcut in
            shortcut.shortTitle.description.contains("Coachy")
        }
        XCTAssertNotNil(coachyShortcut, "Coachy should be registered in AppShortcutsProvider")
    }
}
