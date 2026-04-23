#if canImport(SwiftUI)
import Foundation
import FocalPointCore
import MascotUI

/// Maps UniFFI-generated `Pose`/`Emotion` enums to the `MascotUI`-level
/// `CoachyPose`/`CoachyEmotion`. The Swift UI layer was designed first; the
/// FFI enum set is close but not identical, so we bridge explicitly.
enum CoachyBridging {
    static func pose(from ffi: Pose) -> CoachyPose {
        switch ffi {
        case .confident: return .confident
        case .encouraging: return .encouraging
        case .curiousThinking: return .curious
        case .sternToughLove: return .stern
        case .celebratory: return .celebratory
        case .sleepyDisappointed: return .sleepy
        case .idle: return .idle
        }
    }

    static func emotion(from ffi: Emotion) -> CoachyEmotion {
        switch ffi {
        case .neutral: return .neutral
        case .happy: return .happy
        case .proud: return .proud
        case .concerned: return .concerned
        case .stern: return .focused // no direct Coachy equivalent; closest match
        case .excited: return .excited
        case .tired: return .tired
        case .warm: return .happy
        }
    }

    static func coachyState(from mascot: MascotState) -> CoachyState {
        CoachyState(
            pose: pose(from: mascot.pose),
            emotion: emotion(from: mascot.emotion),
            bubbleText: mascot.bubbleText
        )
    }
}
#endif
