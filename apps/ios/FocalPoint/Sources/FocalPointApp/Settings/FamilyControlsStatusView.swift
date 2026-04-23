#if canImport(FamilyControls)
import SwiftUI
import FamilyControls

/// Diagnostic surface for the FamilyControls authorization state.
///
/// Present in all builds where `FamilyControls` can be imported (i.e., iOS
/// device + iPadOS). The "Request access" button only does useful work when
/// the `FOCALPOINT_HAS_FAMILYCONTROLS` flag is on AND the entitlement is
/// present in the signed app — otherwise `requestAuthorization` will throw
/// at runtime. We surface the error inline rather than suppress it so it is
/// obvious during the entitlement bring-up.
struct FamilyControlsStatusView: View {
    @State private var status: AuthorizationStatus = AuthorizationCenter.shared.authorizationStatus
    @State private var lastError: String?
    @State private var isRequesting: Bool = false

    var body: some View {
        Form {
            Section("Authorization") {
                HStack {
                    Text("Status")
                    Spacer()
                    Text(statusLabel)
                        .foregroundStyle(.secondary)
                        .monospaced()
                }

                if status == .notDetermined {
                    Button {
                        Task { await request() }
                    } label: {
                        if isRequesting {
                            ProgressView()
                        } else {
                            Text("Request access")
                        }
                    }
                    .disabled(isRequesting)
                }

                if let lastError {
                    Text(lastError)
                        .font(.footnote)
                        .foregroundStyle(.red)
                }
            }

            Section("Build flag") {
                HStack {
                    Text("FOCALPOINT_HAS_FAMILYCONTROLS")
                    Spacer()
                    Text(flagOn ? "on" : "off")
                        .foregroundStyle(.secondary)
                        .monospaced()
                }
                Text("When off, enforcement is log-only regardless of authorization. Flip the flag in project.yml once Apple approves the entitlement.")
                    .font(.footnote)
                    .foregroundStyle(.secondary)
            }
        }
        .navigationTitle("Family Controls")
        .onAppear { refresh() }
    }

    private var statusLabel: String {
        switch status {
        case .notDetermined: return "notDetermined"
        case .denied: return "denied"
        case .approved: return "approved"
        @unknown default: return "unknown"
        }
    }

    private var flagOn: Bool {
        #if FOCALPOINT_HAS_FAMILYCONTROLS
        return true
        #else
        return false
        #endif
    }

    private func refresh() {
        status = AuthorizationCenter.shared.authorizationStatus
    }

    @MainActor
    private func request() async {
        isRequesting = true
        defer { isRequesting = false }
        #if FOCALPOINT_HAS_FAMILYCONTROLS
        do {
            try await AuthorizationCenter.shared.requestAuthorization(for: .individual)
            lastError = nil
        } catch {
            lastError = "requestAuthorization failed: \(error.localizedDescription)"
        }
        refresh()
        #else
        lastError = "Entitlement flag off; enable FOCALPOINT_HAS_FAMILYCONTROLS after Apple approval."
        #endif
    }
}

#Preview {
    NavigationStack { FamilyControlsStatusView() }
}
#endif
