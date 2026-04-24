#if canImport(SwiftUI)
import Foundation
import UIKit
import os

#if canImport(Sentry)
import Sentry
#endif

/// Sentry crash reporting setup with user consent, PII redaction, and graceful
/// degradation. Traces to: FR-DIAG-001
public class SentrySetup {
    /// Singleton instance for shared state
    static let shared = SentrySetup()

    private var isConfigured = false
    private let osVersion = UIDevice.current.systemVersion
    private let appVersion = Bundle.main.appVersion

    private init() {}

    /// Initialize Sentry with DSN from bundle plist, respecting user consent.
    /// If DSN is missing or empty, Sentry remains inactive (logged at debug level).
    /// If user toggles consent via @AppStorage, this is called to install/uninstall.
    public func setupIfConsented(userOptedIn: Bool) {
        // Only allow one call to configure Sentry
        if isConfigured {
            if !userOptedIn {
                // User disabled Sentry after opt-in; uninstall by setting a no-op client
                #if canImport(Sentry)
                SentrySDK.close()
                #endif
                isConfigured = false
            }
            return
        }

        let dsn = Bundle.main.object(forInfoDictionaryKey: "FocalpointSentryDsn") as? String
        let dsnTrimmed = dsn?.trimmingCharacters(in: .whitespaces) ?? ""

        if dsnTrimmed.isEmpty {
            // No DSN configured; log once at debug and exit gracefully
            os_log(
                "Sentry: No DSN found in bundle (FocalpointSentryDsn). Crash reporting inactive.",
                log: OSLog(subsystem: "com.focalpoint.app", category: "SentrySetup"),
                type: .debug
            )
            return
        }

        guard userOptedIn else {
            // DSN exists but user has not consented; no-op until they do
            os_log(
                "Sentry: DSN configured but user consent disabled. Waiting for opt-in.",
                log: OSLog(subsystem: "com.focalpoint.app", category: "SentrySetup"),
                type: .debug
            )
            return
        }

        // User has consented and DSN is available; configure Sentry
        #if canImport(Sentry)
        SentrySDK.start { options in
            options.dsn = dsnTrimmed
            options.release = "\(self.appVersion)"
            options.environment = self.buildEnvironment()
            options.tracesSampleRate = 0.1 // 10% of transactions for performance monitoring
            options.enableCrashHandler = true
            options.enableAutoPerformanceTracking = false

            // PII Redaction: beforeSend callback strips emails, UUIDs, and token fragments
            options.beforeSend = { event in
                return self.redactPiiFromEvent(event)
            }
        }

        isConfigured = true
        os_log(
            "Sentry: Initialized with release %@ in %@ environment",
            log: OSLog(subsystem: "com.focalpoint.app", category: "SentrySetup"),
            type: .info,
            self.appVersion,
            self.buildEnvironment()
        )
        #else
        os_log(
            "Sentry: SDK not available; crash reporting disabled.",
            log: OSLog(subsystem: "com.focalpoint.app", category: "SentrySetup"),
            type: .default
        )
        #endif
    }

    /// Derive environment from build configuration (Debug vs Release).
    private func buildEnvironment() -> String {
        #if DEBUG
        return "debug"
        #else
        return "production"
        #endif
    }

    /// Redact PII from crash events: emails, UUIDs (task/rule IDs), and OAuth fragments.
    /// Traces to: FR-DIAG-002
    #if canImport(Sentry)
    private func redactPiiFromEvent(_ event: Event) -> Event? {
        guard let breadcrumbs = event.breadcrumbs else { return event }

        for breadcrumb in breadcrumbs {
            if let message = breadcrumb.message {
                breadcrumb.message = redactString(message)
            }
            if let data = breadcrumb.data as? [String: Any] {
                for (key, value) in data {
                    if let str = value as? String {
                        breadcrumb.data?[key] = redactString(str)
                    }
                }
            }
        }

        // Redact message
        if let message = event.message?.formatted {
            event.message = SentryMessage(formatted: redactString(message))
        }

        // Redact exception messages
        if let exceptions = event.exceptions {
            for exception in exceptions {
                exception.value = redactString(exception.value ?? "")
            }
        }

        return event
    }
    #else
    private func redactPiiFromEvent(_ event: Any?) -> Any? {
        // No-op when Sentry is unavailable
        return nil
    }
    #endif

    /// Redact sensitive strings: emails (contain @), UUIDs, and token fragments.
    private func redactString(_ input: String) -> String {
        var result = input

        // Redact email addresses (any string with @)
        result = result.replacingOccurrences(
            of: "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}",
            with: "[REDACTED_EMAIL]",
            options: .regularExpression
        )

        // Redact UUIDs (task/rule IDs)
        result = result.replacingOccurrences(
            of: "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
            with: "[REDACTED_UUID]",
            options: .regularExpression,
            range: result.startIndex..<result.endIndex
        )

        // Redact OAuth token fragments (common patterns like "Bearer ", "token=", etc.)
        result = result.replacingOccurrences(
            of: "(bearer|token|authorization|api.?key)\\s*[:\\s]*([a-zA-Z0-9._-]{20,})",
            with: "$1 [REDACTED_TOKEN]",
            options: [.regularExpression, .caseInsensitive]
        )

        return result
    }
}

// MARK: - Bundle Extension for App Version

extension Bundle {
    var appVersion: String {
        let version = object(forInfoDictionaryKey: "CFBundleShortVersionString") as? String ?? "0.0.0"
        let build = object(forInfoDictionaryKey: "CFBundleVersion") as? String ?? "0"
        return "\(version)+\(build)"
    }
}

#endif
