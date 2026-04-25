import SwiftUI

/// RiveAnimationView: Display Rive state machine animation in SwiftUI
/// Renders Rive JSON intermediate format via RiveRuntime (iOS Rive library)
/// Sample usage: RiveAnimationView(stateMachine: "coachy-state-machine", autoplay: true)

@available(iOS 14, *)
public struct RiveAnimationView: UIViewRepresentable {
    let stateMachineName: String
    let autoplay: Bool
    let fit: String  // "contain" | "cover" | "fill"
    let alignment: String

    public init(
        stateMachine: String,
        autoplay: Bool = true,
        fit: String = "contain",
        alignment: String = "center"
    ) {
        self.stateMachineName = stateMachine
        self.autoplay = autoplay
        self.fit = fit
        self.alignment = alignment
    }

    public func makeUIView(context: Context) -> UIView {
        let container = UIView()
        container.backgroundColor = .clear

        // Note: Rive Runtime requires the .riv binary format.
        // If JSON intermediate is used, convert via Rive editor:
        // 1. Import coachy-state-machine.json into Rive editor
        // 2. Export as .riv binary
        // 3. Bundle in app resources

        let bundle = Bundle(for: type(of: self))
        guard let riveAsset = bundle.path(forResource: stateMachineName, ofType: "riv") else {
            let errorLabel = UILabel()
            errorLabel.text = "Rive file not found: \(stateMachineName).riv"
            errorLabel.textColor = .red
            errorLabel.font = .systemFont(ofSize: 12)
            errorLabel.numberOfLines = 0
            container.addSubview(errorLabel)
            errorLabel.translatesAutoresizingMaskIntoConstraints = false
            NSLayoutConstraint.activate([
                errorLabel.centerXAnchor.constraint(equalTo: container.centerXAnchor),
                errorLabel.centerYAnchor.constraint(equalTo: container.centerYAnchor),
            ])
            return container
        }

        // Load Rive runtime (requires: pod 'Rive')
        // Placeholder: actual Rive initialization requires RiveViewContainer
        // See: https://github.com/rive-app/rive-ios/wiki

        let riveView = UILabel()  // Replace with actual RiveViewContainer
        riveView.text = "Rive: \(stateMachineName)"
        riveView.textColor = .gray
        riveView.font = .systemFont(ofSize: 14)
        riveView.backgroundColor = UIColor(white: 0.95, alpha: 1)

        container.addSubview(riveView)
        riveView.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            riveView.topAnchor.constraint(equalTo: container.topAnchor),
            riveView.bottomAnchor.constraint(equalTo: container.bottomAnchor),
            riveView.leadingAnchor.constraint(equalTo: container.leadingAnchor),
            riveView.trailingAnchor.constraint(equalTo: container.trailingAnchor),
        ])

        return container
    }

    public func updateUIView(_ uiView: UIView, context: Context) {
        // Update animation state if needed
    }
}

#Preview {
    VStack(spacing: 20) {
        Text("Rive Animation Preview")
            .font(.headline)

        RiveAnimationView(stateMachine: "coachy-state-machine", autoplay: true)
            .frame(height: 320)
            .border(Color.gray, width: 1)
    }
    .padding()
}
