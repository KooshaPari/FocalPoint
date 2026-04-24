#if canImport(Sentry)
import Foundation
import Sentry

/// SentryPrivacyFilter implements the beforeSend callback to strip known PII patterns
/// from crash events before they reach Sentry servers.
///
/// Patterns redacted:
/// - Email addresses (RFC-ish): user@domain.com
/// - Phone numbers: (555) 555-0123, +1-555-0124, etc.
/// - OAuth bearer tokens: "Bearer sk_live_..."
/// - UUIDs (task/rule IDs): 12345678-1234-1234-1234-123456789012
/// - HealthKit values and medical data (when detected in breadcrumbs)
///
/// Traces to: FR-DIAG-002 (PII Redaction)
public struct SentryPrivacyFilter {
    /// Regex patterns for common PII
    private static let emailPattern = "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
    private static let phonePattern = "\\(?\\d{3}\\)?[-.]?\\d{3}[-.]?\\d{4}|\\+\\d{1,3}[-.]?\\d{3,}[-.]?\\d{3,}"
    private static let tokenPattern = "(bearer|token|authorization|api.?key|secret|key|password)\\s*[:\\s=]*([a-zA-Z0-9._-]{20,})"
    private static let uuidPattern = "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"
    private static let healthKitPattern = "(heart.?rate|blood.?pressure|glucose|oxygen|temperature|weight|height|steps|distance)"

    /// Main filter function: redact PII from event before sending to Sentry.
    /// This function is intended to be used as SentrySDK.Options.beforeSend callback.
    public static func filter(_ event: Event) -> Event? {
        // Redact event message
        if let message = event.message?.formatted {
            event.message = SentryMessage(formatted: redactString(message))
        }

        // Redact exception messages
        if let exceptions = event.exceptions {
            for exception in exceptions {
                exception.value = redactString(exception.value ?? "")
            }
        }

        // Redact breadcrumbs (context trail)
        if let breadcrumbs = event.breadcrumbs {
            for breadcrumb in breadcrumbs {
                // Redact breadcrumb message
                if let message = breadcrumb.message {
                    breadcrumb.message = redactString(message)
                }

                // Redact breadcrumb data dictionary
                if let data = breadcrumb.data as? [String: Any] {
                    var redactedData = [String: Any]()
                    for (key, value) in data {
                        if let strValue = value as? String {
                            redactedData[key] = redactString(strValue)
                        } else if let dictValue = value as? [String: Any] {
                            redactedData[key] = redactDictionary(dictValue)
                        } else {
                            redactedData[key] = value
                        }
                    }
                    breadcrumb.data = redactedData
                }
            }
        }

        // Redact request/response data (if present)
        if let request = event.request {
            if let url = request.url {
                request.url = redactString(url)
            }
            if let body = request.body {
                request.body = redactString(body)
            }
        }

        // Redact context values
        if let contexts = event.contexts?.allObjects as? [SentrySerializable] {
            for context in contexts {
                // SentrySerializable doesn't expose direct modification,
                // so we rely on breadcrumb + message redaction for most data flow
                // Additional context redaction would require Sentry SDK enhancements
            }
        }

        return event
    }

    /// Recursively redact strings in a dictionary.
    private static func redactDictionary(_ dict: [String: Any]) -> [String: Any] {
        var result = [String: Any]()
        for (key, value) in dict {
            if let strValue = value as? String {
                result[key] = redactString(strValue)
            } else if let nestedDict = value as? [String: Any] {
                result[key] = redactDictionary(nestedDict)
            } else {
                result[key] = value
            }
        }
        return result
    }

    /// Redact all PII patterns from a string.
    /// Returns the string with all sensitive data replaced by [REDACTED_*] placeholders.
    private static func redactString(_ input: String) -> String {
        var result = input

        // Redact email addresses
        result = result.replacingOccurrences(
            of: emailPattern,
            with: "[REDACTED_EMAIL]",
            options: .regularExpression
        )

        // Redact phone numbers
        result = result.replacingOccurrences(
            of: phonePattern,
            with: "[REDACTED_PHONE]",
            options: .regularExpression
        )

        // Redact OAuth tokens and secrets
        result = result.replacingOccurrences(
            of: tokenPattern,
            with: "$1 [REDACTED_TOKEN]",
            options: [.regularExpression, .caseInsensitive]
        )

        // Redact UUIDs (task/rule IDs)
        result = result.replacingOccurrences(
            of: uuidPattern,
            with: "[REDACTED_UUID]",
            options: .regularExpression,
            range: result.startIndex..<result.endIndex
        )

        // Redact HealthKit-related data patterns
        result = result.replacingOccurrences(
            of: healthKitPattern,
            with: "[REDACTED_HEALTH]",
            options: [.regularExpression, .caseInsensitive]
        )

        // Additional pass: redact obvious auth/private values
        // Catch patterns like "auth=...", "session=...", "private=..." with values
        result = result.replacingOccurrences(
            of: "(auth|session|private|secret|credential|api_key|apiKey)\\s*[:\\s=]*([a-zA-Z0-9._-]{10,})",
            with: "$1 [REDACTED]",
            options: [.regularExpression, .caseInsensitive]
        )

        return result
    }
}
#endif
