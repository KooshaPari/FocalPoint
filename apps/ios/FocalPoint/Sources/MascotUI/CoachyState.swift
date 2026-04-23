import Foundation

/// Mirrors Rust `focus_mascot::Pose` (7 variants).
public enum CoachyPose: String, CaseIterable, Hashable, Sendable, Codable {
    case idle
    case confident
    case encouraging
    case curious
    case stern
    case celebratory
    case sleepy
}

/// Mirrors Rust `focus_mascot::Emotion` (8 variants).
public enum CoachyEmotion: String, CaseIterable, Hashable, Sendable, Codable {
    case neutral
    case happy
    case excited
    case proud
    case concerned
    case disappointed
    case tired
    case focused
}

/// Bridge struct populated from Rust once UniFFI lands.
public struct CoachyState: Hashable, Sendable, Codable {
    public var pose: CoachyPose
    public var emotion: CoachyEmotion
    public var bubbleText: String?

    public init(
        pose: CoachyPose = .idle,
        emotion: CoachyEmotion = .neutral,
        bubbleText: String? = nil
    ) {
        self.pose = pose
        self.emotion = emotion
        self.bubbleText = bubbleText
    }

    public static let placeholder = CoachyState(
        pose: .encouraging,
        emotion: .happy,
        bubbleText: "Let's focus."
    )
}
