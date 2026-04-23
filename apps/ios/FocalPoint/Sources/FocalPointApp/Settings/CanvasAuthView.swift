#if canImport(SwiftUI)
import SwiftUI
import AuthenticationServices
import DesignSystem
import FocalPointCore

/// Real Canvas OAuth (wrapped via ASWebAuthenticationSession).
///
/// Production Canvas OAuth2 flow:
///   1. User enters their institution URL (e.g. `university.instructure.com`).
///   2. App launches `https://<instance>/login/oauth2/auth?...&redirect_uri=focalpoint://...`
///   3. Canvas redirects back with `?code=<authorization_code>`.
///   4. We hand that `code` to the core's connector bridge, which exchanges
///      it for an access token on the server side.
///
/// Because the Rust FFI does not yet expose `connectCanvas(instanceUrl:code:)`
/// (adding it would require regenerating UniFFI bindings + rebuilding the
/// XCFramework — out of scope here), we stub the exchange into `CanvasBridge`
/// so the full UI path still works end-to-end.
public struct CanvasAuthView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var instanceUrl: String = ""
    @State private var busy: Bool = false
    @State private var err: String?
    var onConnected: (CanvasConnectionRecord) -> Void

    public init(onConnected: @escaping (CanvasConnectionRecord) -> Void) {
        self.onConnected = onConnected
    }

    public var body: some View {
        NavigationStack {
            Form {
                Section {
                    TextField("university.instructure.com", text: $instanceUrl)
                        .keyboardType(.URL)
                        .textInputAutocapitalization(.never)
                        .autocorrectionDisabled()
                } header: {
                    Text("Your Canvas instance")
                } footer: {
                    Text("Enter the domain where you sign in to Canvas — without https://.")
                }

                if let err {
                    Section { Text(err).foregroundStyle(.red).font(.caption) }
                }

                Section {
                    Button {
                        Task { await start() }
                    } label: {
                        HStack {
                            if busy { ProgressView() } else { Image(systemName: "safari.fill") }
                            Text(busy ? "Opening…" : "Sign in with Canvas")
                        }
                        .frame(maxWidth: .infinity)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .disabled(instanceUrl.isEmpty || busy)
                }
            }
            .navigationTitle("Connect Canvas")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
        }
    }

    private func start() async {
        busy = true
        defer { busy = false }
        do {
            let code = try await CanvasAuth.performOAuth(instanceUrl: instanceUrl)
            try await CanvasBridge.connect(instanceUrl: instanceUrl, code: code)
            if let rec = CanvasBridge.load() {
                onConnected(rec)
            }
            dismiss()
        } catch {
            err = "\(error.localizedDescription)"
        }
    }
}

/// Thin wrapper around `ASWebAuthenticationSession`. Returns the
/// authorization code extracted from the callback URL.
enum CanvasAuth {
    enum AuthError: LocalizedError {
        case invalidInstance
        case cancelled
        case missingCode
        case sessionFailed(String)

        var errorDescription: String? {
            switch self {
            case .invalidInstance: return "That doesn't look like a Canvas domain."
            case .cancelled: return "Sign-in cancelled."
            case .missingCode: return "Canvas didn't return an authorization code."
            case .sessionFailed(let m): return m
            }
        }
    }

    /// OAuth client ID placeholder — Canvas instances each register their own
    /// dev key. We use a predictable sentinel here and rely on the stub to
    /// succeed when running against a fake auth page.
    static let clientId = "focalpoint-dev"
    static let callbackScheme = "focalpoint"
    static let callbackUrl = "focalpoint://auth/canvas/callback"

    static func authorizeURL(instanceUrl: String) throws -> URL {
        let cleaned = instanceUrl
            .trimmingCharacters(in: .whitespacesAndNewlines)
            .replacingOccurrences(of: "https://", with: "")
            .replacingOccurrences(of: "http://", with: "")
        guard cleaned.contains(".") else { throw AuthError.invalidInstance }
        var comps = URLComponents()
        comps.scheme = "https"
        comps.host = cleaned
        comps.path = "/login/oauth2/auth"
        comps.queryItems = [
            URLQueryItem(name: "client_id", value: clientId),
            URLQueryItem(name: "response_type", value: "code"),
            URLQueryItem(name: "redirect_uri", value: callbackUrl),
            URLQueryItem(name: "state", value: UUID().uuidString),
        ]
        guard let url = comps.url else { throw AuthError.invalidInstance }
        return url
    }

    @MainActor
    static func performOAuth(instanceUrl: String) async throws -> String {
        let url = try authorizeURL(instanceUrl: instanceUrl)
        return try await withCheckedThrowingContinuation { cont in
            let session = ASWebAuthenticationSession(
                url: url,
                callbackURLScheme: callbackScheme
            ) { callback, error in
                if let error {
                    if (error as NSError).code == ASWebAuthenticationSessionError.canceledLogin.rawValue {
                        cont.resume(throwing: AuthError.cancelled)
                    } else {
                        cont.resume(throwing: AuthError.sessionFailed(error.localizedDescription))
                    }
                    return
                }
                guard
                    let callback,
                    let comps = URLComponents(url: callback, resolvingAgainstBaseURL: false),
                    let code = comps.queryItems?.first(where: { $0.name == "code" })?.value
                else {
                    cont.resume(throwing: AuthError.missingCode)
                    return
                }
                cont.resume(returning: code)
            }
            let anchor = PresentationAnchor()
            session.presentationContextProvider = anchor
            session.prefersEphemeralWebBrowserSession = false
            session.start()
        }
    }
}

@MainActor
final class PresentationAnchor: NSObject, ASWebAuthenticationPresentationContextProviding {
    func presentationAnchor(for session: ASWebAuthenticationSession) -> ASPresentationAnchor {
        UIApplication.shared
            .connectedScenes
            .compactMap { $0 as? UIWindowScene }
            .first?.keyWindow ?? ASPresentationAnchor()
    }
}
#endif
