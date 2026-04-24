import WidgetKit
import SwiftUI

// MARK: - Accessory Circular Widget

struct AccessoryCircularWidgetEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
}

struct AccessoryCircularWidgetEntryView: View {
    var entry: AccessoryCircularWidgetEntry

    var body: some View {
        ZStack {
            AccessoryWidgetBackground()

            VStack(spacing: 2) {
                Image(systemName: "flame.fill")
                    .font(.system(size: 14, weight: .semibold))
                    .foregroundStyle(.orange)

                Text("\(entry.snapshot.streakDays)")
                    .font(.system(size: 20, weight: .bold))
                    .foregroundStyle(.primary)
            }
        }
    }
}

struct AccessoryCircularWidgetProvider: TimelineProvider {
    typealias Entry = AccessoryCircularWidgetEntry

    func placeholder(in context: Context) -> AccessoryCircularWidgetEntry {
        AccessoryCircularWidgetEntry(
            date: Date(),
            snapshot: WidgetSnapshot()
        )
    }

    func getSnapshot(in context: Context, completion: @escaping (AccessoryCircularWidgetEntry) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = AccessoryCircularWidgetEntry(date: Date(), snapshot: snapshot)
        completion(entry)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<AccessoryCircularWidgetEntry>) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = AccessoryCircularWidgetEntry(date: Date(), snapshot: snapshot)

        let nextRefresh = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date()
        let timeline = Timeline(entries: [entry], policy: .after(nextRefresh))
        completion(timeline)
    }
}

struct AccessoryCircularWidget: Widget {
    let kind: String = "AccessoryCircularWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: AccessoryCircularWidgetProvider()) { entry in
            AccessoryCircularWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Streak Badge")
        .description("Displays your current focus streak on lock screen and StandBy.")
        .supportedFamilies([.accessoryCircular])
    }
}

// MARK: - Accessory Rectangular Widget

struct AccessoryRectangularWidgetEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
}

struct AccessoryRectangularWidgetEntryView: View {
    var entry: AccessoryRectangularWidgetEntry

    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            Text("Credits & Focus Time")
                .font(.caption2)
                .foregroundStyle(.secondary)

            HStack(spacing: 8) {
                Text("\(entry.snapshot.creditsBalance) credits")
                    .font(.system(.caption, design: .monospaced))
                    .foregroundStyle(.primary)

                Divider()
                    .frame(height: 12)

                Text("15 min next")
                    .font(.system(.caption, design: .monospaced))
                    .foregroundStyle(.primary)
            }
        }
        .containerBackground(.fill, for: .accessoryRectangular)
    }
}

struct AccessoryRectangularWidgetProvider: TimelineProvider {
    typealias Entry = AccessoryRectangularWidgetEntry

    func placeholder(in context: Context) -> AccessoryRectangularWidgetEntry {
        AccessoryRectangularWidgetEntry(
            date: Date(),
            snapshot: WidgetSnapshot()
        )
    }

    func getSnapshot(in context: Context, completion: @escaping (AccessoryRectangularWidgetEntry) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = AccessoryRectangularWidgetEntry(date: Date(), snapshot: snapshot)
        completion(entry)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<AccessoryRectangularWidgetEntry>) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = AccessoryRectangularWidgetEntry(date: Date(), snapshot: snapshot)

        let nextRefresh = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date()
        let timeline = Timeline(entries: [entry], policy: .after(nextRefresh))
        completion(timeline)
    }
}

struct AccessoryRectangularWidget: Widget {
    let kind: String = "AccessoryRectangularWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: AccessoryRectangularWidgetProvider()) { entry in
            AccessoryRectangularWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Credits & Focus")
        .description("Shows credits balance and next focus time on lock screen.")
        .supportedFamilies([.accessoryRectangular])
    }
}

// MARK: - Accessory Inline Widget

struct AccessoryInlineWidgetEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
}

struct AccessoryInlineWidgetEntryView: View {
    var entry: AccessoryInlineWidgetEntry

    var body: some View {
        Text("🔥 \(entry.snapshot.streakDays) · \(entry.snapshot.creditsBalance)¢")
            .font(.system(.caption, design: .monospaced))
    }
}

struct AccessoryInlineWidgetProvider: TimelineProvider {
    typealias Entry = AccessoryInlineWidgetEntry

    func placeholder(in context: Context) -> AccessoryInlineWidgetEntry {
        AccessoryInlineWidgetEntry(
            date: Date(),
            snapshot: WidgetSnapshot()
        )
    }

    func getSnapshot(in context: Context, completion: @escaping (AccessoryInlineWidgetEntry) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = AccessoryInlineWidgetEntry(date: Date(), snapshot: snapshot)
        completion(entry)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<AccessoryInlineWidgetEntry>) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = AccessoryInlineWidgetEntry(date: Date(), snapshot: snapshot)

        let nextRefresh = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date()
        let timeline = Timeline(entries: [entry], policy: .after(nextRefresh))
        completion(timeline)
    }
}

struct AccessoryInlineWidget: Widget {
    let kind: String = "AccessoryInlineWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: AccessoryInlineWidgetProvider()) { entry in
            AccessoryInlineWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Streak & Credits")
        .description("Compact streak and credit display for lock screen.")
        .supportedFamilies([.accessoryInline])
    }
}
