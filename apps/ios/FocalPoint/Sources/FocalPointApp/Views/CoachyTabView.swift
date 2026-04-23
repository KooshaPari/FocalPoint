#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI

struct CoachyTabView: View {
    @State private var pose: CoachyPose = .encouraging
    @State private var emotion: CoachyEmotion = .happy

    var body: some View {
        NavigationStack {
            ZStack {
                Color.app.background.ignoresSafeArea()
                VStack(spacing: 24) {
                    CoachyView(state: CoachyState(
                        pose: pose,
                        emotion: emotion,
                        bubbleText: "\(pose.rawValue.capitalized) · \(emotion.rawValue)"
                    ))
                    Picker("Pose", selection: $pose) {
                        ForEach(CoachyPose.allCases, id: \.self) { p in
                            Text(p.rawValue.capitalized).tag(p)
                        }
                    }
                    .pickerStyle(.menu)

                    Picker("Emotion", selection: $emotion) {
                        ForEach(CoachyEmotion.allCases, id: \.self) { e in
                            Text(e.rawValue.capitalized).tag(e)
                        }
                    }
                    .pickerStyle(.menu)
                }
                .padding()
            }
            .navigationTitle("Coachy")
        }
    }
}
#endif
