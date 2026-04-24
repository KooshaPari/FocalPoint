import SwiftUI

/// Generic async boundary container: shows Coachy during loading, transitions content in on completion.
struct CoachyLoadingView<Content: View>: View {
    enum LoadingState: Equatable {
        case idle
        case loading(reason: String)
        case loaded
        case error(String)
    }

    @State var loadingState: LoadingState
    @ViewBuilder let content: () -> Content
    @EnvironmentObject var presenter: CoachyScenePresenter
    @State private var showContent = false

    var body: some View {
        ZStack {
            switch loadingState {
            case .idle:
                content()
                    .transition(.opacity)

            case .loading(let reason):
                // Show Coachy in loading state
                loadingScene(for: reason)
                    .transition(.opacity)

            case .loaded:
                content()
                    .transition(.asymmetric(insertion: .opacity, removal: .opacity))

            case .error(let message):
                VStack {
                    Image(systemName: "exclamationmark.triangle.fill")
                        .font(.largeTitle)
                        .foregroundColor(.red)
                    Text(message)
                        .font(.body)
                        .foregroundColor(.secondary)
                }
                .transition(.opacity)
            }
        }
        .animation(.easeInOut(duration: 0.3), value: loadingState)
    }

    @ViewBuilder
    private func loadingScene(for reason: String) -> some View {
        ZStack {
            // Gradient background
            LinearGradient(
                gradient: Gradient(colors: [.blue.opacity(0.1), .purple.opacity(0.1)]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            // Coachy in loading pose
            VStack(spacing: 20) {
                CoachyView(
                    state: CoachyState(
                        pose: .curious,
                        emotion: .focused,
                        bubbleText: reason
                    ),
                    size: 200
                )

                ProgressView()
                    .controlSize(.large)
            }
            .padding()
        }
    }
}

/// Preview helper.
#Preview {
    ZStack {
        Color.white.ignoresSafeArea()

        CoachyLoadingView(loadingState: .loading(reason: "Loading...")) {
            VStack {
                Text("Content loaded!")
                    .font(.title)
                Spacer()
            }
        }
        .environmentObject(CoachyScenePresenter())
    }
}
