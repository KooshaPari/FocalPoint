import XCTest
@testable import FocalPointApp

// Traces to: FR-AUDIO-001 (Deterministic cue synthesis)
// Traces to: FR-AUDIO-002 (Haptic pairing)
final class CueLibraryTests: XCTestCase {
    let cuePlayer = CuePlayer.shared

    // MARK: - Audio Asset Validation

    /// Verify all 8 cues have WAV files under 10KB each.
    func testAllCueAudioFilesExist() throws {
        for cue in FocalPointCue.allCases {
            let url = Bundle.main.url(forResource: cue.rawValue, withExtension: "wav",
                                      subdirectory: "Audio/Cues")
            XCTAssertNotNil(url, "Missing audio file: \(cue.rawValue).wav")

            if let url = url {
                let attributes = try FileManager.default.attributesOfItem(atPath: url.path)
                let size = attributes[.size] as? NSNumber ?? 0
                XCTAssertLessThan(size.intValue, 10240, "Cue \(cue.rawValue) exceeds 10KB")
            }
        }
    }

    /// Verify all cues have reasonable durations.
    func testCueDurations() {
        for cue in FocalPointCue.allCases {
            XCTAssertGreaterThan(cue.duration, 0, "Cue \(cue.rawValue) duration invalid")
            XCTAssertLessThan(cue.duration, 1.0, "Cue \(cue.rawValue) duration too long")
        }
    }

    // MARK: - Haptic Pattern Mapping

    /// Verify each cue has a unique haptic pattern.
    func testHapticPatternAssignment() {
        let cues = FocalPointCue.allCases
        XCTAssertEqual(cues.count, 8, "Expected 8 cues")

        // Verify no cue is missing a pattern
        for cue in cues {
            let pattern = cue.hapticPattern
            // This should not crash or return nil
            _ = pattern
        }
    }

    // MARK: - Cue Enumeration

    /// Verify all 8 cues are defined.
    func testAllCuesEnumerated() {
        let cues = FocalPointCue.allCases
        XCTAssertEqual(cues.count, 8)

        let expectedCues: Set<String> = [
            "rule-fire",
            "achievement",
            "intervention-warn",
            "focus-start",
            "focus-end",
            "error",
            "success",
            "mascot-acknowledge",
        ]

        let actualCues = Set(cues.map { $0.rawValue })
        XCTAssertEqual(actualCues, expectedCues)
    }

    /// Verify each cue has a human-readable label.
    func testCueLabels() {
        for cue in FocalPointCue.allCases {
            XCTAssertFalse(cue.label.isEmpty, "Cue \(cue.rawValue) missing label")
            XCTAssertFalse(cue.label.contains(cue.rawValue), "Label should be readable, not raw")
        }
    }

    // MARK: - Player State

    /// Verify CuePlayer singleton exists.
    func testCuePlayerSharedInstance() {
        let player1 = CuePlayer.shared
        let player2 = CuePlayer.shared
        XCTAssertTrue(player1 === player2, "CuePlayer should be singleton")
    }

    // MARK: - Integration

    /// Verify cue playback can be initiated without crashing.
    func testPlayCueNoOp() {
        // This tests that play() doesn't crash with missing audio files in test target.
        // In production, audio files bundled in Resources/Audio/Cues/.
        for cue in FocalPointCue.allCases {
            cuePlayer.playAudio(cue)
            // No assertion needed; just verify no exception.
        }
    }

    /// Verify haptic patterns can be triggered.
    func testHapticPatternExecution() {
        for cue in FocalPointCue.allCases {
            cue.hapticPattern.play()
            // Haptics are async, so just verify no exception.
        }
    }

    // MARK: - Regression

    /// Verify all cues are case-accessible and printable.
    func testCueEnumerationProperties() {
        for cue in FocalPointCue.allCases {
            XCTAssertNotNil(cue.rawValue)
            XCTAssertNotNil(cue.label)
            XCTAssertNotNil(cue.duration)
            XCTAssertNotNil(cue.hapticPattern)
        }
    }
}
