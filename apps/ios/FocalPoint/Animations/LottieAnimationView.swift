import SwiftUI

/// LottieAnimationView: Display Lottie JSON animation in SwiftUI
/// Renders Lottie JSON via Lottie iOS library (pod 'lottie-ios')
/// Sample usage: LottieAnimationView(name: "rule-fire", loopMode: .loop, speed: 1.0)

@available(iOS 14, *)
public struct LottieAnimationView: UIViewRepresentable {
    let animationName: String
    let loopMode: LottieLoopMode
    let speed: CGFloat

    public enum LottieLoopMode {
        case loop
        case once
        case autoReverse
    }

    public init(
        name: String,
        loopMode: LottieLoopMode = .loop,
        speed: CGFloat = 1.0
    ) {
        self.animationName = name
        self.loopMode = loopMode
        self.speed = speed
    }

    public func makeUIView(context: Context) -> UIView {
        let container = UIView()
        container.backgroundColor = .clear

        // Load Lottie animation (requires: pod 'lottie-ios')
        // Expected: assets/motion/lottie/{name}.json bundled in app

        let bundle = Bundle(for: type(of: self))
        guard let path = bundle.path(forResource: animationName, ofType: "json"),
              let data = try? Data(contentsOf: URL(fileURLWithPath: path)) else {
            let errorLabel = UILabel()
            errorLabel.text = "Lottie file not found: \(animationName).json"
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

        // Placeholder: actual Lottie initialization requires LottieAnimationView
        // See: https://github.com/airbnb/lottie-ios

        let lottieView = UILabel()  // Replace with actual LottieAnimationView
        lottieView.text = "Lottie: \(animationName)"
        lottieView.textColor = .gray
        lottieView.font = .systemFont(ofSize: 14)
        lottieView.backgroundColor = UIColor(white: 0.95, alpha: 1)

        container.addSubview(lottieView)
        lottieView.translatesAutoresizingMaskIntoConstraints = false
        NSLayoutConstraint.activate([
            lottieView.topAnchor.constraint(equalTo: container.topAnchor),
            lottieView.bottomAnchor.constraint(equalTo: container.bottomAnchor),
            lottieView.leadingAnchor.constraint(equalTo: container.leadingAnchor),
            lottieView.trailingAnchor.constraint(equalTo: container.trailingAnchor),
        ])

        return container
    }

    public func updateUIView(_ uiView: UIView, context: Context) {
        // Update animation parameters if needed
    }
}

#Preview {
    VStack(spacing: 20) {
        Text("Lottie Animation Preview")
            .font(.headline)

        LottieAnimationView(name: "rule-fire", loopMode: .loop, speed: 1.0)
            .frame(height: 300)
            .border(Color.gray, width: 1)

        LottieAnimationView(name: "success-checkmark", loopMode: .once, speed: 1.2)
            .frame(height: 200)
            .border(Color.green, width: 1)
    }
    .padding()
}
