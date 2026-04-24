#if canImport(EventKit)
import EventKit
import Foundation
import FocalPointCore

/// `CalendarHost` backed by the device's EventKit calendar store. Plugs into
/// the Rust core via `FocalPointCore.setCalendarHost(...)` so the Rituals
/// engine's Morning Brief reads the user's real agenda for schedule previews
/// and conflict detection.
///
/// Traces to: FR-CONNECTOR-001 — real CalendarPort implementation (replaces
/// `InMemoryCalendarPort` stub in `RitualsApi`).
public final class EventKitCalendarHost: CalendarHost {
    private let store: EKEventStore
    private let isoFormatter: ISO8601DateFormatter

    public init(store: EKEventStore = EKEventStore()) {
        self.store = store
        let fmt = ISO8601DateFormatter()
        fmt.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        self.isoFormatter = fmt
    }

    /// Request full-access permission. iOS 17+ requires
    /// `requestFullAccessToEvents`; older APIs are gated by deployment target
    /// 17.0 so we only branch on iOS 17+. Call from UI before wiring the host.
    public func requestAccess() async throws -> Bool {
        if #available(iOS 17.0, *) {
            return try await store.requestFullAccessToEvents()
        } else {
            return try await withCheckedThrowingContinuation { cont in
                store.requestAccess(to: .event) { granted, err in
                    if let err = err { cont.resume(throwing: err) }
                    else { cont.resume(returning: granted) }
                }
            }
        }
    }

    // MARK: - CalendarHost

    /// Synchronous — the Rust side offloads to `spawn_blocking`, so blocking
    /// here on EventKit's synchronous predicate API is safe.
    public func listEvents(startIso: String, endIso: String) -> [CalendarEventDto] {
        guard
            let start = parseDate(startIso),
            let end = parseDate(endIso),
            start < end
        else {
            return []
        }

        // Only pull from calendars the user has granted access to.
        let calendars = store.calendars(for: .event)
        let predicate = store.predicateForEvents(
            withStart: start,
            end: end,
            calendars: calendars
        )
        let ekEvents = store.events(matching: predicate)

        return ekEvents.compactMap { ev -> CalendarEventDto? in
            guard let evStart = ev.startDate, let evEnd = ev.endDate else {
                return nil
            }
            // Hard: all-day events, events with alarms, or explicit availability
            // = .busy. Soft: tentative / free / everything else.
            let kind: CalendarEventKindDto
            if ev.isAllDay || (ev.hasAlarms && ev.availability == .busy) {
                kind = .hard
            } else if ev.availability == .busy {
                kind = .hard
            } else {
                kind = .soft
            }
            // Stable id: EKEvent.eventIdentifier survives across launches;
            // fall back to a content hash if missing (declined invites).
            let id = ev.eventIdentifier ?? "ekevent:\(evStart.timeIntervalSince1970):\(ev.title ?? "")"
            return CalendarEventDto(
                id: id,
                title: ev.title ?? "(untitled)",
                startIso: isoFormatter.string(from: evStart),
                endIso: isoFormatter.string(from: evEnd),
                kind: kind
            )
        }
    }

    // MARK: - Helpers

    private func parseDate(_ iso: String) -> Date? {
        // RFC3339 parser — tolerates with/without fractional seconds.
        if let d = isoFormatter.date(from: iso) { return d }
        let fallback = ISO8601DateFormatter()
        fallback.formatOptions = [.withInternetDateTime]
        return fallback.date(from: iso)
    }
}
#endif
