#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

/// Placeholder renderer for Coachy. Will be replaced by a Spline scene later.
/// Today it draws a simple flame shape with SF Symbols overlaying to reflect pose.
public struct CoachyView: View {
    public let state: CoachyState

    public init(state: CoachyState) {
        self.state = state
    }

    public var body: some View {
        VStack(spacing: 16) {
            ZStack {
                flameBody
                    .frame(width: 160, height: 200)
                poseGlyph
                    .font(.system(size: 44, weight: .bold))
                    .foregroundStyle(Color.coachy.eyes)
                    .offset(y: -10)
                    .accessibilityHidden(true)
            }
            .accessibilityLabel(Text("Coachy \(state.pose.rawValue), \(state.emotion.rawValue)"))

            if let text = state.bubbleText, !text.isEmpty {
                Text(text)
                    .chatBubble()
            }
        }
        .padding()
    }

    private var flameBody: some View {
        Canvas { ctx, size in
            let rect = CGRect(origin: .zero, size: size)
            let base = Path(ellipseIn: rect.insetBy(dx: 10, dy: size.height * 0.45))
            ctx.fill(base, with: .color(Color.coachy.flameBase))

            var flame = Path()
            let w = size.width, h = size.height
            flame.move(to: CGPoint(x: w * 0.5, y: 0))
            flame.addQuadCurve(
                to: CGPoint(x: w * 0.95, y: h * 0.75),
                control: CGPoint(x: w * 0.95, y: h * 0.25)
            )
            flame.addQuadCurve(
                to: CGPoint(x: w * 0.05, y: h * 0.75),
                control: CGPoint(x: w * 0.5, y: h)
            )
            flame.addQuadCurve(
                to: CGPoint(x: w * 0.5, y: 0),
                control: CGPoint(x: w * 0.05, y: h * 0.25)
            )
            ctx.fill(flame, with: .color(Color.coachy.flameCore))

            var highlight = Path()
            highlight.move(to: CGPoint(x: w * 0.5, y: h * 0.1))
            highlight.addQuadCurve(
                to: CGPoint(x: w * 0.75, y: h * 0.6),
                control: CGPoint(x: w * 0.8, y: h * 0.3)
            )
            highlight.addQuadCurve(
                to: CGPoint(x: w * 0.5, y: h * 0.1),
                control: CGPoint(x: w * 0.55, y: h * 0.4)
            )
            ctx.fill(highlight, with: .color(Color.coachy.flameEdge))
        }
    }

    private var poseGlyph: some View {
        Image(systemName: state.pose.sfSymbol)
    }
}

extension CoachyPose {
    /// SF Symbol approximating each pose, used in the placeholder renderer.
    var sfSymbol: String {
        switch self {
        case .idle: return "circle"
        case .confident: return "flame.fill"
        case .encouraging: return "hands.sparkles.fill"
        case .curious: return "questionmark.circle.fill"
        case .stern: return "exclamationmark.triangle.fill"
        case .celebratory: return "sparkles"
        case .sleepy: return "moon.zzz.fill"
        }
    }
}

// MARK: - Chat bubble modifier

public struct ChatBubble: ViewModifier {
    public func body(content: Content) -> some View {
        content
            .font(.system(size: 14, weight: .medium))
            .foregroundStyle(Color.app.foreground)
            .padding(.horizontal, 14)
            .padding(.vertical, 10)
            .background(
                RoundedRectangle(cornerRadius: 14, style: .continuous)
                    .fill(Color.app.surface)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 14, style: .continuous)
                    .strokeBorder(Color.app.accent.opacity(0.4), lineWidth: 1)
            )
    }
}

extension View {
    public func chatBubble() -> some View {
        modifier(ChatBubble())
    }
}
#endif
