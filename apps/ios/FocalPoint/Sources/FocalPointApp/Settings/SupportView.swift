#if canImport(SwiftUI)
import SwiftUI
import DesignSystem

/// Help and Support section with FAQ, troubleshooting, bug reports, and community links.
public struct SupportView: View {
    @State private var showMailComposer = false
    @State private var deviceInfo = ""

    public init() {}

    public var body: some View {
        NavigationStack {
            Form {
                // MARK: - Help Sections
                Section("Resources") {
                    Link(destination: URL(string: "https://focalpoint.app/faq")!) {
                        HStack {
                            Image(systemName: "questionmark.circle")
                                .foregroundStyle(Color.app.accent)
                            Text("FAQ (25+ questions)")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Link(destination: URL(string: "https://focalpoint.app/troubleshooting")!) {
                        HStack {
                            Image(systemName: "wrench.and.screwdriver")
                                .foregroundStyle(Color.app.accent)
                            Text("Troubleshooting Guide (15 issues)")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Text("Find answers to common questions and step-by-step solutions for known issues.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                // MARK: - Report & Request
                Section("Report & Request") {
                    Link(destination: URL(string: "https://github.com/KooshaPari/FocalPoint/issues/new?template=bug_report.yml")!) {
                        HStack {
                            Image(systemName: "exclamationmark.circle")
                                .foregroundStyle(.red)
                            Text("Report a bug")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Link(destination: URL(string: "https://github.com/KooshaPari/FocalPoint/issues/new?template=feature_request.yml")!) {
                        HStack {
                            Image(systemName: "lightbulb")
                                .foregroundStyle(.yellow)
                            Text("Request a feature")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Button(action: sendSupportEmail) {
                        HStack {
                            Image(systemName: "envelope")
                                .foregroundStyle(Color.app.accent)
                            Text("Contact support")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Text("Your device info is included to help us support you better.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                // MARK: - Community
                Section("Community") {
                    Link(destination: URL(string: "https://discord.gg/focalpoint")!) {
                        HStack {
                            Image(systemName: "person.2.circle")
                                .foregroundStyle(Color(red: 0.7, green: 0.7, blue: 1.0))
                            Text("Join Discord community")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Link(destination: URL(string: "https://github.com/KooshaPari/FocalPoint")!) {
                        HStack {
                            Image(systemName: "star.circle")
                                .foregroundStyle(.gray)
                            Text("Star on GitHub")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)

                    Text("Meet other users, share tips, and help shape FocalPoint's future.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }

                // MARK: - Licensing & Commercial
                Section("Licensing") {
                    Text("FocalPoint is open-source under MIT OR Apache-2.0.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)

                    Link(destination: URL(string: "mailto:commercial@focalpoint.app?subject=FocalPoint%20Commercial%20Inquiry")!) {
                        HStack {
                            Image(systemName: "briefcase")
                                .foregroundStyle(Color.app.accent)
                            Text("Enterprise or commercial use?")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)
                }
            }
            .navigationTitle("Support")
            .background(Color.app.background.ignoresSafeArea())
        }
        .onAppear {
            populateDeviceInfo()
        }
    }

    // MARK: - Helper: Device Info

    /// Gather device and app version info for support email.
    private func populateDeviceInfo() {
        let device = UIDevice.current
        let osVersion = "\(device.systemName) \(device.systemVersion)"
        let appVersion = Bundle.main.appVersion
        let model = device.model

        deviceInfo = """
        Device: \(model)
        OS: \(osVersion)
        App Version: \(appVersion)
        """
    }

    /// Send support email with device diagnostics pre-populated.
    private func sendSupportEmail() {
        let subject = "FocalPoint Support Request"
        let body = """
        [Please describe your issue or question below]

        ---
        Device Info (auto-populated):
        \(deviceInfo)
        """

        let encodedSubject = subject.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? ""
        let encodedBody = body.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? ""

        if let url = URL(string: "mailto:support@focalpoint.app?subject=\(encodedSubject)&body=\(encodedBody)") {
            UIApplication.shared.open(url)
        }
    }
}

// MARK: - Bundle extension for app version

extension Bundle {
    var appVersion: String {
        guard let version = infoDictionary?["CFBundleShortVersionString"] as? String,
              let build = infoDictionary?["CFBundleVersion"] as? String else {
            return "unknown"
        }
        return "\(version) (build \(build))"
    }
}

// MARK: - UIDevice extension for model name

extension UIDevice {
    var model: String {
        var systemInfo = utsname()
        uname(&systemInfo)
        let machineMirror = Mirror(reflecting: systemInfo.machine)
        let identifier = machineMirror.children.reduce("") { identifier, element in
            guard let value = element.value as? Int8, value != 0 else { return identifier }
            return identifier + String(UnicodeScalar(UInt8(value)))
        }
        return identifier
    }
}

#if DEBUG
#Preview {
    SupportView()
        .environmentObject(CoreHolder.preview)
}
#endif
#endif
