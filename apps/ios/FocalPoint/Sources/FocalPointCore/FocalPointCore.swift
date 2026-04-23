import Foundation

// UniFFI-generated bindings (focus_ffi.swift) now provide the real
// `FocalPointCore` class. Keep Swift-only placeholder types (RuleId,
// ActiveRule) that the UI layer already references; they are not in the UDL.

/// Mirrors `focus_domain::RuleId` (opaque identifier).
public struct RuleId: Hashable, Codable, Sendable {
    public let raw: String
    public init(_ raw: String) { self.raw = raw }
}

/// Minimal placeholder for an active rule; the Rust side owns the real schema.
public struct ActiveRule: Hashable, Codable, Sendable {
    public let id: RuleId
    public let title: String
    public let endsAt: Date?

    public init(id: RuleId, title: String, endsAt: Date?) {
        self.id = id
        self.title = title
        self.endsAt = endsAt
    }
}
