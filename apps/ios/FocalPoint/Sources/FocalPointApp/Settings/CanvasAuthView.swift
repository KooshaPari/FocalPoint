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
/// The code is handed to `FocalPointCore.connector().connectCanvas(...)`,
/// which performs the OAuth2 token exchange in Rust and persists the token in
/// the iOS keychain (service=`focalpoint`, account=`canvas:<instance>`).
/// `CanvasBridge.load()` is still used to read UI-side metadata.
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
            try CoreHolder.shared.core.connector()
                .connectCanvas(instanceUrl: instanceUrl, code: code)
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

    /// OAuth client ID. Each Canvas instance requires its own Developer Key
    /// registered under that institution's admin panel; there's no universal
    /// FocalPoint client ID. We pull it from Info.plist key
    /// `FocalpointCanvasClientId`, which is set per-build via an xcconfig or
    /// the iOS app's Info.plist. If unset, surface an explicit error instead
    /// of sending `client_id=` (empty) which makes Canvas return
    /// `invalid_client` with no context.
    static let callbackScheme = "focalpoint"
    static let callbackUrl = "focalpoint://auth/canvas/callback"

    static func resolvedClientId() throws -> String {
        let raw = Bundle.main.object(forInfoDictionaryKey: "FocalpointCanvasClientId") as? String
        let trimmed = raw?.trimmingCharacters(in: .whitespacesAndNewlines) ?? ""
        if trimmed.isEmpty || trimmed == "FOCALPOINT_CANVAS_CLIENT_ID_NOT_CONFIGURED" {
            throw AuthError.sessionFailed("Canvas OAuth client id not configured. Add 'FocalpointCanvasClientId' to Info.plist with the Developer Key ID your admin registered for this app on that Canvas instance.")
        }
        return trimmed
    }

    static func authorizeURL(instanceUrl: String) throws -> URL {
        let cleaned = instanceUrl
            .trimmingCharacters(in: .whitespacesAndNewlines)
            .replacingOccurrences(of: "https://", with: "")
            .replacingOccurrences(of: "http://", with: "")
        guard cleaned.contains(".") else { throw AuthError.invalidInstance }
        let clientId = try resolvedClientId()
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
