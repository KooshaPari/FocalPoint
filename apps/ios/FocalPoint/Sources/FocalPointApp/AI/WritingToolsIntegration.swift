#if canImport(SwiftUI) && canImport(UIKit)
import SwiftUI
import UIKit

/// Apple Intelligence Writing Tools integration for iOS 18+.
/// Enables on-device text rewriting, tone shifts, and summarization for
/// coaching messages, morning briefs, and rule explanations.
/// No cloud access; all processing on-device via Apple's system models.
public extension View {
    /// Apply writing tools behavior to coaching message TextViews.
    /// iOS 18+: enables rewrite/proofread/grammar context menu.
    /// Earlier iOS: no-op.
    @available(iOS 18, *)
    func coachyWritingTools() -> some View {
        if #available(iOS 18, *) {
            return AnyView(
                self.writingToolsBehavior(.complete)
            )
        } else {
            return AnyView(self)
        }
    }

    /// Disable writing tools for security-sensitive fields
    /// (OAuth token display, audit chain output).
    /// Prevents accidental rewriting or exposure of sensitive data.
    @available(iOS 18, *)
    func securitySensitiveDisableWritingTools() -> some View {
        if #available(iOS 18, *) {
            return AnyView(
                self.writingToolsBehavior(.disabled)
            )
        } else {
            return AnyView(self)
        }
    }
}

/// Tone transformation options for morning brief rewriting.
/// Each applies an Apple Intelligence tone shift:
/// - Friendly: Warm, conversational, encouraging
/// - Coach: Direct, motivational, action-oriented
/// - Concise: Short, punchy, bullet-point style
/// - Motivational: Inspiring, goal-focused, energetic
public enum BriefToneOption: String, CaseIterable {
    case friendly = "Friendly"
    case coach = "Coach"
    case concise = "Concise"
    case motivational = "Motivational"

    var description: String {
        switch self {
        case .friendly:
            return "Warm and conversational"
        case .coach:
            return "Direct and motivational"
        case .concise:
            return "Short and punchy"
        case .motivational:
            return "Inspiring and energetic"
        }
    }

    /// System prompt fragment for tone transformation.
    /// Used by WritingToolsClient to apply tone via on-device API.
    var systemPrompt: String {
        switch self {
        case .friendly:
            return "Rewrite in a warm, conversational, and encouraging tone. Use friendly language and maintain warmth throughout."
        case .coach:
            return "Rewrite in a direct, motivational coaching tone. Be action-oriented and inspiring while staying concise."
        case .concise:
            return "Rewrite to be concise and punchy. Use short sentences, bullet points where appropriate, and eliminate unnecessary words."
        case .motivational:
            return "Rewrite to be inspiring and energetic. Focus on goals, celebrate progress, and motivate action."
        }
    }
}

/// Helper to manage Apple Intelligence writing tools requests.
/// Wraps the iOS 18 WritingToolsClient for controlled tone transformations.
@available(iOS 18, *)
public struct WritingToolsClient {
    /// Transform text using a specified tone via on-device API.
    /// Returns the rewritten text or nil if the system request fails.
    ///
    /// - Parameters:
    ///   - text: Original text to transform
    ///   - tone: Desired tone transformation (friendly, coach, concise, motivational)
    /// - Returns: Transformed text, or nil if transformation unavailable
    static func transformText(_ text: String, tone: BriefToneOption) -> String? {
        // On iOS 18+, Apple provides WritingToolsClient for programmatic tone transformations.
        // For now, this is a placeholder that will integrate with the actual API when available.
        // In production, this would:
        // 1. Call WritingToolsClient.transformText(text, tone: tone.systemPrompt)
        // 2. Return the transformed result
        // 3. Fail gracefully if on-device processing is unavailable

        // Stub: return the original text.
        // Real implementation will use WritingToolsClient once iOS 18 SDK is finalized.
        return text
    }
}

/// Settings state for Apple Intelligence features.
/// Persisted to UserDefaults, checked at app launch.
public struct AppleIntelligenceSettings {
    @AppStorage("app.writingToolsEnabled") public var writingToolsEnabled: Bool = true
    @AppStorage("app.writingToolsOptInShown") public var optInShown: Bool = false

    public init() {}

    /// Check if writing tools are available and enabled.
    /// Returns true only if:
    /// - iOS 18+ is running
    /// - User has not disabled in Settings
    static func isAvailable() -> Bool {
        if #available(iOS 18, *) {
            let settings = AppleIntelligenceSettings()
            return settings.writingToolsEnabled
        }
        return false
    }
}
#endif
