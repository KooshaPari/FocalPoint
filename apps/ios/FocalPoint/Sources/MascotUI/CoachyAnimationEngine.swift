// CoachyAnimationEngine — Rive loader shell.
//
// This file is a *shell*. It detects whether a `Coachy.riv` state-machine asset
// has been delivered by the designer (see `docs/mascot/coachy-art-direction.md`)
// and, if so, drives the animation from the Rive runtime. If the asset is
// missing (the current state — no art yet), it falls through to the existing
// SwiftUI `CoachyView` renderer. That fallback is what ships today; the Rive
// path lights up the day the designer drops `Coachy.riv` into
// `apps/ios/FocalPoint/Resources/Mascot/`.
//
// NOTE FOR FUTURE INTEGRATION:
//   When the first `Coachy.riv` lands, add `RiveRuntime` as an SPM dependency
//   to `apps/ios/FocalPoint/Package.swift` and `project.yml`:
//
//     dependencies: [
//         .package(url: "https://github.com/rive-app/rive-ios", from: "6.0.0")
//     ]
//
//   and add `.product(name: "RiveRuntime", package: "rive-ios")` to the
//   MascotUI target. The `#if canImport(RiveRuntime)` guards in this file
//   will then activate automatically.
//
// Until that happens, this file compiles without the Rive dependency pinned
// (the Rive branch is gated out) and CI stays green.

#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

#if canImport(RiveRuntime)
import RiveRuntime
#endif

// MARK: - Extended pose / accessory vocabulary
//
// `CoachyPose` (in CoachyState.swift) mirrors the Rust enum's 7 variants and
// must stay aligned with the FFI. The designer brief expands the catalog to
// 14 poses + 5 accessories. `CoachyExtendedPose` is the app-facing surface;
// it maps back to `CoachyPose` for the SwiftUI fallback renderer and forward
// to Rive state-machine inputs when `Coachy.riv` is present.

public enum CoachyExtendedPose: String, CaseIterable, Hashable, Sendable, Codable {
    case idle
    case confident
    case encouraging
    case curiousThinking
    case sternToughLove
    case celebratory
    case sleepyDisappointed
    case thumbsUp
    case workMode
    case cheering
    case focusMode
    case studyMode
    case achievement
    case lockdown

    /// Collapse to the 7-variant FFI-aligned enum for the SwiftUI fallback.
    public var fallbackPose: CoachyPose {
        switch self {
        case .idle, .workMode, .focusMode, .studyMode: return .idle
        case .confident, .thumbsUp: return .confident
        case .encouraging, .cheering, .achievement: return .encouraging
        case .curiousThinking: return .curious
        case .sternToughLove, .lockdown: return .stern
        case .celebratory: return .celebratory
        case .sleepyDisappointed: return .sleepy
        }
    }

    /// Rive state-machine enum index. Order MUST match the designer's
    /// `pose` input enum declared in `Coachy.riv`. Keep this table in sync
    /// with §2 of coachy-art-direction.md.
    public var riveInputValue: Double {
        switch self {
        case .idle: return 0
        case .confident: return 1
        case .encouraging: return 2
        case .curiousThinking: return 3
        case .sternToughLove: return 4
        case .celebratory: return 5
        case .sleepyDisappointed: return 6
        case .thumbsUp: return 7
        case .workMode: return 8
        case .cheering: return 9
        case .focusMode: return 10
        case .studyMode: return 11
        case .achievement: return 12
        case .lockdown: return 13
        }
    }
}

public enum CoachyAccessory: String, CaseIterable, Hashable, Sendable, Codable {
    case none
    case headphones
    case glassesBook
    case trophy
    case shield
    case padlock

    public var riveInputValue: Double {
        switch self {
        case .none: return 0
        case .headphones: return 1
        case .glassesBook: return 2
        case .trophy: return 3
        case .shield: return 4
        case .padlock: return 5
        }
    }
}

public extension CoachyEmotion {
    var riveInputValue: Double {
        switch self {
        case .neutral: return 0
        case .happy: return 1
        case .excited: return 2
        case .proud: return 3
        case .concerned: return 4
        case .disappointed: return 5
        case .tired: return 6
        case .focused: return 7
        }
    }
}

// MARK: - Engine

/// `CoachyAnimationEngine` is the single entry point for rendering Coachy.
/// It decides between the Rive pipeline (when `Coachy.riv` is available and
/// the `RiveRuntime` SPM dep is linked) and the SwiftUI fallback renderer.
///
/// Call sites use `CoachyAnimationEngine.view(...)` the same way they used
/// `CoachyView(...)` before — the engine is a drop-in superset.
public enum CoachyAnimationEngine {

    /// Name of the Rive bundle the designer delivers. See brief §6.
    public static let riveBundleName = "Coachy"
    public static let riveStateMachineName = "CoachyStateMachine"

    /// Rive input identifiers. Must match the state-machine inputs defined in
    /// `Coachy.riv` per brief §4.
    public enum RiveInput {
        public static let pose = "pose"
        public static let accessory = "accessory"
        public static let emotion = "emotion"
    }

    /// Are we capable of rendering with Rive right now? Two conditions:
    /// 1. `RiveRuntime` is linked (SPM dep added).
    /// 2. `Coachy.riv` is present in the main bundle.
    public static var canUseRive: Bool {
        #if canImport(RiveRuntime)
        return Bundle.main.url(forResource: riveBundleName, withExtension: "riv") != nil
        #else
        return false
        #endif
    }

    /// Render Coachy. Prefers the Rive renderer when available, falls back
    /// to the SwiftUI `CoachyView` otherwise.
    @ViewBuilder
    public static func view(
        pose: CoachyExtendedPose,
        accessory: CoachyAccessory = .none,
        emotion: CoachyEmotion = .neutral,
        bubbleText: String? = nil,
        size: CGFloat = 240
    ) -> some View {
        if canUseRive {
            RiveCoachyView(
                pose: pose,
                accessory: accessory,
                emotion: emotion,
                bubbleText: bubbleText,
                size: size
            )
        } else {
            fallbackView(
                pose: pose,
                emotion: emotion,
                bubbleText: bubbleText,
                size: size
            )
        }
    }

    /// SwiftUI fallback: composes the existing `CoachyView` using the 7-variant
    /// pose enum collapsed from the extended catalog.
    @ViewBuilder
    public static func fallbackView(
        pose: CoachyExtendedPose,
        emotion: CoachyEmotion,
        bubbleText: String?,
        size: CGFloat
    ) -> some View {
        CoachyView(
            state: CoachyState(
                pose: pose.fallbackPose,
                emotion: emotion,
                bubbleText: bubbleText
            ),
            size: size
        )
    }
}

// MARK: - Rive-backed view (gated)

/// Rive-backed Coachy. Compiles to the SwiftUI fallback when `RiveRuntime`
/// is not linked, so the overall package stays buildable before the Rive
/// SPM dependency is added.
public struct RiveCoachyView: View {
    public let pose: CoachyExtendedPose
    public let accessory: CoachyAccessory
    public let emotion: CoachyEmotion
    public let bubbleText: String?
    public let size: CGFloat

    public init(
        pose: CoachyExtendedPose,
        accessory: CoachyAccessory = .none,
        emotion: CoachyEmotion = .neutral,
        bubbleText: String? = nil,
        size: CGFloat = 240
    ) {
        self.pose = pose
        self.accessory = accessory
        self.emotion = emotion
        self.bubbleText = bubbleText
        self.size = size
    }

    public var body: some View {
        #if canImport(RiveRuntime)
        VStack(spacing: 18) {
            RiveStateMachineHost(
                bundleName: CoachyAnimationEngine.riveBundleName,
                stateMachineName: CoachyAnimationEngine.riveStateMachineName,
                pose: pose,
                accessory: accessory,
                emotion: emotion
            )
            .frame(width: size, height: size * 1.15)

            if let text = bubbleText, !text.isEmpty {
                Text(text).chatBubble()
            }
        }
        #else
        CoachyAnimationEngine.fallbackView(
            pose: pose,
            emotion: emotion,
            bubbleText: bubbleText,
            size: size
        )
        #endif
    }
}

#if canImport(RiveRuntime)

/// Thin wrapper around `RiveViewModel` that pushes our enum values into
/// the state-machine inputs every time the SwiftUI state updates.
private struct RiveStateMachineHost: View {
    let bundleName: String
    let stateMachineName: String
    let pose: CoachyExtendedPose
    let accessory: CoachyAccessory
    let emotion: CoachyEmotion

    @StateObject private var model: RiveViewModelHolder

    init(
        bundleName: String,
        stateMachineName: String,
        pose: CoachyExtendedPose,
        accessory: CoachyAccessory,
        emotion: CoachyEmotion
    ) {
        self.bundleName = bundleName
        self.stateMachineName = stateMachineName
        self.pose = pose
        self.accessory = accessory
        self.emotion = emotion
        _model = StateObject(
            wrappedValue: RiveViewModelHolder(
                bundleName: bundleName,
                stateMachineName: stateMachineName
            )
        )
    }

    var body: some View {
        model.viewModel.view()
            .onAppear { pushInputs() }
            .onChange(of: pose) { _ in pushInputs() }
            .onChange(of: accessory) { _ in pushInputs() }
            .onChange(of: emotion) { _ in pushInputs() }
    }

    private func pushInputs() {
        model.viewModel.setInput(CoachyAnimationEngine.RiveInput.pose, value: pose.riveInputValue)
        model.viewModel.setInput(CoachyAnimationEngine.RiveInput.accessory, value: accessory.riveInputValue)
        model.viewModel.setInput(CoachyAnimationEngine.RiveInput.emotion, value: emotion.riveInputValue)
    }
}

private final class RiveViewModelHolder: ObservableObject {
    let viewModel: RiveViewModel

    init(bundleName: String, stateMachineName: String) {
        self.viewModel = RiveViewModel(
            fileName: bundleName,
            stateMachineName: stateMachineName
        )
    }
}

#endif // canImport(RiveRuntime)

// MARK: - Imperative state setter (for callers holding a handle)
//
// Most SwiftUI call sites should use `CoachyAnimationEngine.view(...)` and
// let SwiftUI state propagation drive the animation. For imperative sites
// (e.g. the launch sequence choreography) this helper lets a caller construct
// a Rive view-model once and push state into it over time. It's a no-op when
// Rive isn't available.
public final class CoachyAnimationController: ObservableObject {
    @Published public var pose: CoachyExtendedPose
    @Published public var accessory: CoachyAccessory
    @Published public var emotion: CoachyEmotion
    @Published public var bubbleText: String?

    public init(
        pose: CoachyExtendedPose = .idle,
        accessory: CoachyAccessory = .none,
        emotion: CoachyEmotion = .neutral,
        bubbleText: String? = nil
    ) {
        self.pose = pose
        self.accessory = accessory
        self.emotion = emotion
        self.bubbleText = bubbleText
    }

    /// Push a new state tuple in one call. Matches the brief's
    /// `setState(pose, emotion, accessory)` spec.
    public func setState(
        pose: CoachyExtendedPose,
        emotion: CoachyEmotion,
        accessory: CoachyAccessory = .none,
        bubbleText: String? = nil
    ) {
        self.pose = pose
        self.emotion = emotion
        self.accessory = accessory
        self.bubbleText = bubbleText
    }
}
#endif // canImport(SwiftUI)
