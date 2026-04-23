#if canImport(SwiftUI)
import SwiftUI
import AuthenticationServices
import DesignSystem
import FocalPointCore

/// Google Calendar OAuth via `ASWebAuthenticationSession`.
///
/// Flow:
///   1. Launch `https://accounts.google.com/o/oauth2/v2/auth?...` with
///      `response_type=code`, offline access, forced consent, and the
///      calendar-readonly scope.
///   2. Google redirects back to `focalpoint://auth/gcal/callback?code=...`.
///   3. We hand the authorization code to
///      `FocalPointCore.connector().connectGcal(code:)`, which exchanges it
///      for a refresh/access token on the Rust side and persists it via the
///      iOS keychain.
///
/// The Google OAuth client ID is read from Info.plist
/// (`FocalpointGCalClientId`). If the key is missing, the view refuses to
/// start the flow and surfaces a clear error panel rather than crashing.
public struct GCalAuthView: View {
    @Environment(\.dismiss) private var dismiss
    @State private var busy: Bool = false
    @State private var err: String?
    var onConnected: () -> Void

    public init(onConnected: @escaping () -> Void) {
        self.onConnected = onConnected
    }

    private var clientId: String? {
        Bundle.main.object(forInfoDictionaryKey: "FocalpointGCalClientId") as? String
    }

    public var body: some View {
        NavigationStack {
            Form {
                Section {
                    Text("Sign in with your Google account to let FocalPoint read your calendar. Only read-only access to events is requested.")
                        .font(.caption)
                        .foregroundStyle(Color.app.foreground.opacity(0.7))
                } header: {
                    Text("Google Calendar")
                }

                if clientId == nil {
                    Section {
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Google OAuth client id not configured.")
                                .font(.caption.weight(.semibold))
                                .foregroundStyle(.red)
                            Text("Set `FocalpointGCalClientId` in Info.plist.")
                                .font(.caption2)
                                .foregroundStyle(Color.app.foreground.opacity(0.6))
                        }
                    }
                }

                Section {
                    Button {
                        Task { await start() }
                    } label: {
                        HStack {
                            if busy { ProgressView() } else { Image(systemName: "calendar") }
                            Text(busy ? "Opening…" : "Sign in with Google")
                        }
                        .frame(maxWidth: .infinity)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .disabled(busy || clientId == nil)
                }
            }
            .navigationTitle("Connect Google Calendar")
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
            }
            .alert("Connection failed", isPresented: Binding(
                get: { err != nil },
                set: { if !$0 { err = nil } }
            )) {
                Button("OK", role: .cancel) { err = nil }
            } message: {
                Text(err ?? "")
            }
        }
    }

    private func start() async {
        busy = true
        defer { busy = false }
        do {
            let id = clientId ?? "FOCALPOINT_GCAL_CLIENT_ID_NOT_CONFIGURED"
            let code = try await GCalAuth.performOAuth(clientId: id)
            try CoreHolder.shared.core.connector().connectGcal(code: code)
            onConnected()
            dismiss()
        } catch {
            err = "\(error.localizedDescription)"
        }
    }
}

/// Thin wrapper around `ASWebAuthenticationSession` for Google OAuth2.
enum GCalAuth {
    enum AuthError: LocalizedError {
        case cancelled
        case missingCode
        case invalidURL
        case sessionFailed(String)

        var errorDescription: String? {
            switch self {
            case .cancelled: return "Sign-in cancelled."
            case .missingCode: return "Google didn't return an authorization code."
            case .invalidURL: return "Couldn't build the Google OAuth URL."
            case .sessionFailed(let m): return m
            }
        }
    }

    static let callbackScheme = "focalpoint"
    static let callbackUrl = "focalpoint://auth/gcal/callback"
    static let scope = "https://www.googleapis.com/auth/calendar.readonly"

    static func authorizeURL(clientId: String) throws -> URL {
        var comps = URLComponents()
        comps.scheme = "https"
        comps.host = "accounts.google.com"
        comps.path = "/o/oauth2/v2/auth"
        comps.queryItems = [
            URLQueryItem(name: "client_id", value: clientId),
            URLQueryItem(name: "response_type", value: "code"),
            URLQueryItem(name: "scope", value: scope),
            URLQueryItem(name: "redirect_uri", value: callbackUrl),
            URLQueryItem(name: "access_type", value: "offline"),
            URLQueryItem(name: "prompt", value: "consent"),
            URLQueryItem(name: "state", value: UUID().uuidString),
        ]
        guard let url = comps.url else { throw AuthError.invalidURL }
        return url
    }

    @MainActor
    static func performOAuth(clientId: String) async throws -> String {
        let url = try authorizeURL(clientId: clientId)
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
#endif
