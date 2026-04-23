#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

/// Coachy — AI Focus Coach mascot. Flame-bodied character with a red cape and
/// gold-star belt buckle. This is a native SwiftUI rendering (no 3D runtime);
/// it composes a flame body + eyes + mouth + cape + star buckle and parameterizes
/// each pose by eye/mouth shape + arm position. Matches the key art palette
/// from docs/reference/design_tokens.md.
public struct CoachyView: View {
    public let state: CoachyState
    public var size: CGFloat

    public init(state: CoachyState, size: CGFloat = 240) {
        self.state = state
        self.size = size
    }

    public var body: some View {
        VStack(spacing: 18) {
            ZStack {
                // Soft accent glow behind Coachy.
                Circle()
                    .fill(Color.app.accent.opacity(0.12))
                    .frame(width: size * 1.1, height: size * 1.1)
                    .blur(radius: 30)

                coachyFigure
                    .frame(width: size, height: size * 1.15)
                    .animation(.easeInOut(duration: 0.4), value: state.pose)
            }
            .accessibilityElement(children: .combine)
            .accessibilityLabel(Text("Coachy is \(state.pose.accessibilityLabel), feeling \(state.emotion.accessibilityLabel)"))

            if let text = state.bubbleText, !text.isEmpty {
                Text(text)
                    .chatBubble()
                    .transition(.opacity.combined(with: .scale(scale: 0.95)))
                    .id(text)
            }
        }
        .padding(.vertical, 20)
    }

    // MARK: Figure composition

    private var coachyFigure: some View {
        GeometryReader { proxy in
            let w = proxy.size.width
            let h = proxy.size.height
            ZStack {
                capeShape(w: w, h: h)
                flameBody(w: w, h: h)
                facialFeatures(w: w, h: h)
                beltAndBuckle(w: w, h: h)
                armsForPose(w: w, h: h)
            }
        }
    }

    private func flameBody(w: CGFloat, h: CGFloat) -> some View {
        Canvas { ctx, size in
            // Main flame body (teardrop-ish)
            var body = Path()
            let cx = size.width / 2
            body.move(to: CGPoint(x: cx, y: size.height * 0.02))
            body.addCurve(
                to: CGPoint(x: size.width * 0.92, y: size.height * 0.70),
                control1: CGPoint(x: size.width * 0.95, y: size.height * 0.22),
                control2: CGPoint(x: size.width * 0.98, y: size.height * 0.52)
            )
            body.addCurve(
                to: CGPoint(x: size.width * 0.5, y: size.height * 0.95),
                control1: CGPoint(x: size.width * 0.88, y: size.height * 0.88),
                control2: CGPoint(x: size.width * 0.72, y: size.height * 0.96)
            )
            body.addCurve(
                to: CGPoint(x: size.width * 0.08, y: size.height * 0.70),
                control1: CGPoint(x: size.width * 0.28, y: size.height * 0.96),
                control2: CGPoint(x: size.width * 0.12, y: size.height * 0.88)
            )
            body.addCurve(
                to: CGPoint(x: cx, y: size.height * 0.02),
                control1: CGPoint(x: size.width * 0.02, y: size.height * 0.52),
                control2: CGPoint(x: size.width * 0.05, y: size.height * 0.22)
            )

            let gradient = Gradient(colors: [
                Color.coachy.flameEdge,
                Color.coachy.flameCore,
                Color.coachy.flameBase,
            ])
            ctx.fill(
                body,
                with: .linearGradient(
                    gradient,
                    startPoint: CGPoint(x: cx, y: 0),
                    endPoint: CGPoint(x: cx, y: size.height)
                )
            )

            // Wisp at top — a little flicker
            var wisp = Path()
            wisp.move(to: CGPoint(x: cx - 8, y: size.height * 0.06))
            wisp.addQuadCurve(
                to: CGPoint(x: cx + 4, y: size.height * -0.02),
                control: CGPoint(x: cx + 14, y: size.height * 0.02)
            )
            wisp.addQuadCurve(
                to: CGPoint(x: cx - 8, y: size.height * 0.06),
                control: CGPoint(x: cx - 4, y: size.height * 0.10)
            )
            ctx.fill(wisp, with: .color(Color.coachy.flameEdge))
        }
        .frame(width: w, height: h)
    }

    private func capeShape(w: CGFloat, h: CGFloat) -> some View {
        // Draw a cape behind the body peeking from shoulders to ankles.
        Canvas { ctx, size in
            var cape = Path()
            cape.move(to: CGPoint(x: size.width * 0.08, y: size.height * 0.55))
            cape.addQuadCurve(
                to: CGPoint(x: size.width * 0.5, y: size.height * 1.00),
                control: CGPoint(x: size.width * 0.2, y: size.height * 0.92)
            )
            cape.addQuadCurve(
                to: CGPoint(x: size.width * 0.92, y: size.height * 0.55),
                control: CGPoint(x: size.width * 0.8, y: size.height * 0.92)
            )
            cape.addQuadCurve(
                to: CGPoint(x: size.width * 0.08, y: size.height * 0.55),
                control: CGPoint(x: size.width * 0.5, y: size.height * 0.50)
            )
            ctx.fill(cape, with: .color(Color.coachy.cape))
        }
        .frame(width: w, height: h)
    }

    @ViewBuilder
    private func facialFeatures(w: CGFloat, h: CGFloat) -> some View {
        // Eyes + mouth, pose-parameterized.
        let eyeY = h * 0.38
        let eyeDx = w * 0.15
        let cx = w / 2
        let eyeW = w * 0.14
        let eyeH: CGFloat = {
            switch state.pose {
            case .sleepy: return eyeW * 0.2   // squinting
            case .stern: return eyeW * 0.6
            default: return eyeW * 1.05
            }
        }()
        ZStack {
            Ellipse().fill(.white).frame(width: eyeW, height: eyeH)
                .position(x: cx - eyeDx, y: eyeY)
            Ellipse().fill(.white).frame(width: eyeW, height: eyeH)
                .position(x: cx + eyeDx, y: eyeY)
            if state.pose != .sleepy {
                Circle().fill(Color.coachy.eyes).frame(width: eyeW * 0.45, height: eyeW * 0.45)
                    .position(x: cx - eyeDx + pupilOffset, y: eyeY)
                Circle().fill(Color.coachy.eyes).frame(width: eyeW * 0.45, height: eyeW * 0.45)
                    .position(x: cx + eyeDx + pupilOffset, y: eyeY)
            }
            mouthShape.stroke(Color.coachy.eyes, style: StrokeStyle(lineWidth: 3, lineCap: .round))
                .frame(width: w * 0.18, height: h * 0.05)
                .position(x: cx, y: h * 0.50)
        }
    }

    private var pupilOffset: CGFloat {
        switch state.pose {
        case .confident, .encouraging, .celebratory: return -2
        case .curious: return 6
        case .stern: return 0
        default: return 0
        }
    }

    private var mouthShape: Path {
        Path { path in
            switch state.pose {
            case .celebratory, .encouraging:
                // Big open smile
                path.move(to: CGPoint(x: 0, y: 0))
                path.addQuadCurve(to: CGPoint(x: 40, y: 0), control: CGPoint(x: 20, y: 18))
            case .confident:
                path.move(to: CGPoint(x: 0, y: 4))
                path.addQuadCurve(to: CGPoint(x: 40, y: 4), control: CGPoint(x: 20, y: 14))
            case .stern:
                path.move(to: CGPoint(x: 4, y: 6))
                path.addLine(to: CGPoint(x: 36, y: 6))
            case .curious:
                path.addArc(center: CGPoint(x: 20, y: 6), radius: 5, startAngle: .degrees(0), endAngle: .degrees(360), clockwise: false)
            case .sleepy:
                path.move(to: CGPoint(x: 10, y: 6))
                path.addQuadCurve(to: CGPoint(x: 30, y: 6), control: CGPoint(x: 20, y: 0))
            case .idle:
                path.move(to: CGPoint(x: 6, y: 6))
                path.addQuadCurve(to: CGPoint(x: 34, y: 6), control: CGPoint(x: 20, y: 10))
            }
        }
    }

    // Belt + star buckle removed per user feedback (jarring, unexplained).
    // A proper waist treatment — if we want one — should be an organic
    // cape-fold rendered as part of the contiguous body shape, not a
    // contrasting rectangle.
    private func beltAndBuckle(w: CGFloat, h: CGFloat) -> some View {
        EmptyView()
    }

    @ViewBuilder
    private func armsForPose(w: CGFloat, h: CGFloat) -> some View {
        switch state.pose {
        case .celebratory:
            // Arms up
            armGlyph(at: CGPoint(x: w * 0.1, y: h * 0.35), rotation: -70)
            armGlyph(at: CGPoint(x: w * 0.9, y: h * 0.35), rotation: 70)
        case .encouraging:
            // One arm thumbs-up
            armGlyph(at: CGPoint(x: w * 0.15, y: h * 0.50), rotation: -15)
        case .stern:
            // Crossed arms suggestion — two horizontal strokes
            RoundedRectangle(cornerRadius: 6)
                .fill(Color.coachy.flameBase)
                .frame(width: w * 0.5, height: h * 0.04)
                .position(x: w * 0.5, y: h * 0.57)
        case .confident:
            // Pointing finger
            armGlyph(at: CGPoint(x: w * 0.85, y: h * 0.45), rotation: 45)
        case .curious:
            // Chin-hand
            armGlyph(at: CGPoint(x: w * 0.62, y: h * 0.50), rotation: -30)
        case .sleepy, .idle:
            EmptyView()
        }
    }

    private func armGlyph(at center: CGPoint, rotation degrees: Double) -> some View {
        Capsule()
            .fill(Color.coachy.flameCore)
            .frame(width: size * 0.10, height: size * 0.30)
            .rotationEffect(.degrees(degrees))
            .position(center)
    }
}

// MARK: - Star shape

struct StarShape: Shape {
    let points: Int

    func path(in rect: CGRect) -> Path {
        var path = Path()
        let center = CGPoint(x: rect.midX, y: rect.midY)
        let r = min(rect.width, rect.height) / 2
        let rInner = r * 0.45
        let step = .pi * 2 / Double(points * 2)
        for i in 0..<(points * 2) {
            let angle = Double(i) * step - .pi / 2
            let radius = i.isMultiple(of: 2) ? r : rInner
            let x = center.x + CGFloat(cos(angle)) * radius
            let y = center.y + CGFloat(sin(angle)) * radius
            if i == 0 {
                path.move(to: CGPoint(x: x, y: y))
            } else {
                path.addLine(to: CGPoint(x: x, y: y))
            }
        }
        path.closeSubpath()
        return path
    }
}

// MARK: - Accessibility labels

private extension CoachyPose {
    var accessibilityLabel: String {
        switch self {
        case .idle: return "idle"
        case .confident: return "confident"
        case .encouraging: return "encouraging"
        case .curious: return "thinking"
        case .stern: return "stern"
        case .celebratory: return "celebrating"
        case .sleepy: return "sleepy"
        }
    }
}

private extension CoachyEmotion {
    var accessibilityLabel: String {
        switch self {
        case .neutral: return "neutral"
        case .happy: return "happy"
        case .excited: return "excited"
        case .proud: return "proud"
        case .concerned: return "concerned"
        case .disappointed: return "disappointed"
        case .tired: return "tired"
        case .focused: return "focused"
        }
    }
}

// MARK: - Chat bubble modifier

public struct ChatBubble: ViewModifier {
    public func body(content: Content) -> some View {
        content
            .font(.system(size: 15, weight: .medium, design: .rounded))
            .foregroundStyle(Color.app.foreground)
            .multilineTextAlignment(.center)
            .padding(.horizontal, 16)
            .padding(.vertical, 12)
            .background(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .fill(Color.app.surface)
            )
            .overlay(
                RoundedRectangle(cornerRadius: 16, style: .continuous)
                    .strokeBorder(Color.app.accent.opacity(0.4), lineWidth: 1)
            )
    }
}

public extension View {
    func chatBubble() -> some View { modifier(ChatBubble()) }
}
#endif
