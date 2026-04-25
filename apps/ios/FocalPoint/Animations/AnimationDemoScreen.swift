import SwiftUI

/// AnimationDemoScreen: Showcase Rive + Lottie animations
/// Usage: NavigationLink(destination: AnimationDemoScreen()) { Text("Animations") }

@available(iOS 14, *)
public struct AnimationDemoScreen: View {
    @State private var selectedTab: Int = 0

    public init() {}

    public var body: some View {
        NavigationView {
            TabView(selection: $selectedTab) {
                // Tab 1: Rive State Machine (Mascot)
                VStack(spacing: 16) {
                    Text("Coachy State Machine")
                        .font(.headline)

                    RiveAnimationView(stateMachine: "coachy-state-machine", autoplay: true)
                        .frame(height: 320)
                        .border(Color.gray, width: 1)

                    VStack(alignment: .leading, spacing: 8) {
                        Text("20 States (5 expressions × 4 intensities)")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text("Expressions: idle, happy, focused, concerned, sleeping")
                            .font(.caption2)
                        Text("Intensities: calm, active, intense, post-rule")
                            .font(.caption2)
                    }
                    .frame(maxWidth: .infinity, alignment: .leading)
                    .padding(.horizontal)

                    Spacer()
                }
                .padding()
                .tabItem {
                    Label("Rive", systemImage: "sparkles")
                }
                .tag(0)

                // Tab 2: Lottie Animations
                ScrollView {
                    VStack(spacing: 20) {
                        Text("Lottie Animations")
                            .font(.headline)

                        AnimationRow(
                            title: "Rule Create",
                            description: "UI appear on rule creation",
                            animationName: "rule-create"
                        )

                        AnimationRow(
                            title: "Rule Fire",
                            description: "Visual pulse when rule triggered",
                            animationName: "rule-fire"
                        )

                        AnimationRow(
                            title: "Emergency Exit",
                            description: "Rapid red flash on exit",
                            animationName: "emergency-exit"
                        )

                        AnimationRow(
                            title: "Success Checkmark",
                            description: "Checkmark stroke on success",
                            animationName: "success-checkmark"
                        )

                        AnimationRow(
                            title: "Mascot Blink",
                            description: "Quick eye close/open",
                            animationName: "mascot-blink"
                        )

                        AnimationRow(
                            title: "Sync Pulse",
                            description: "Data sync shimmer",
                            animationName: "sync-pulse"
                        )

                        Spacer()
                    }
                    .padding()
                }
                .tabItem {
                    Label("Lottie", systemImage: "list.dash")
                }
                .tag(1)

                // Tab 3: Animation Index
                VStack(alignment: .leading, spacing: 16) {
                    Text("12 Micro-Animations")
                        .font(.headline)

                    List(animationIndex, id: \.name) { item in
                        VStack(alignment: .leading, spacing: 4) {
                            Text(item.name)
                                .font(.subheadline)
                                .fontWeight(.semibold)
                            Text(item.description)
                                .font(.caption)
                                .foregroundColor(.secondary)
                            HStack(spacing: 8) {
                                Label("\(item.frames)f", systemImage: "film")
                                    .font(.caption2)
                                Label("\(item.size)KB", systemImage: "arrow.left.and.right")
                                    .font(.caption2)
                            }
                            .foregroundColor(.blue)
                        }
                        .padding(.vertical, 4)
                    }
                }
                .navigationTitle("Animations")
                .tabItem {
                    Label("Index", systemImage: "doc.list")
                }
                .tag(2)
            }
            .navigationTitle("Animation Gallery")
        }
    }

    private var animationIndex: [AnimationItem] {
        [
            AnimationItem(name: "rule-create", description: "UI appear on rule creation", frames: 30, size: 12),
            AnimationItem(name: "rule-fire", description: "Visual pulse when rule triggered", frames: 15, size: 8),
            AnimationItem(name: "intervention-warn", description: "Warning before intervention", frames: 40, size: 15),
            AnimationItem(name: "emergency-exit", description: "Rapid red flash on exit", frames: 50, size: 18),
            AnimationItem(name: "achievement-unlock", description: "Achievement earned", frames: 45, size: 20),
            AnimationItem(name: "mascot-blink", description: "Quick eye close/open", frames: 5, size: 2),
            AnimationItem(name: "mascot-yawn", description: "Mascot yawn animation", frames: 30, size: 12),
            AnimationItem(name: "focus-start", description: "Focus session starting", frames: 20, size: 10),
            AnimationItem(name: "focus-end", description: "Focus session ending", frames: 20, size: 10),
            AnimationItem(name: "sync-pulse", description: "Data sync shimmer", frames: 20, size: 9),
            AnimationItem(name: "error-shake", description: "Error state shake", frames: 12, size: 5),
            AnimationItem(name: "success-checkmark", description: "Success indication", frames: 25, size: 11),
        ]
    }

    private struct AnimationItem {
        let name: String
        let description: String
        let frames: Int
        let size: Int
    }
}

/// AnimationRow: Compact animation preview
@available(iOS 14, *)
private struct AnimationRow: View {
    let title: String
    let description: String
    let animationName: String

    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(title)
                .font(.subheadline)
                .fontWeight(.semibold)

            LottieAnimationView(name: animationName, loopMode: .loop)
                .frame(height: 120)
                .border(Color.gray, width: 1)

            Text(description)
                .font(.caption)
                .foregroundColor(.secondary)
        }
    }
}

#Preview {
    AnimationDemoScreen()
}
