import WidgetKit
import SwiftUI

struct CreditsWidgetEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
}

struct CreditsWidgetEntryView: View {
    var entry: CreditsWidgetEntry
    @Environment(\.widgetFamily) var family

    var body: some View {
        switch family {
        case .systemSmall:
            smallView
        case .systemMedium, .systemLarge:
            mediumView
        case .accessoryCircular:
            accessoryCircularView
        case .accessoryRectangular:
            accessoryRectangularView
        case .accessoryInline:
            accessoryInlineView
        default:
            mediumView
        }
    }

    private var smallView: some View {
        VStack(alignment: .leading, spacing: 8) {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Credits")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    Text("\(entry.snapshot.creditsBalance)")
                        .font(.title2.weight(.bold))
                }
                Spacer()
                Image(systemName: "star.fill")
                    .font(.title3)
                    .foregroundStyle(.orange)
            }
            Divider()
            VStack(alignment: .leading, spacing: 4) {
                Label("\(entry.snapshot.streakDays) day streak", systemImage: "🔥")
                    .font(.caption)
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .containerBackground(.fill, for: .widget)
    }

    private var mediumView: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Credits")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    Text("\(entry.snapshot.creditsBalance)")
                        .font(.title.weight(.bold))
                }
                Spacer()
                VStack(alignment: .center, spacing: 2) {
                    Image(systemName: "flame.fill")
                        .font(.title3)
                        .foregroundStyle(.orange)
                    Text("\(entry.snapshot.streakDays)")
                        .font(.caption.weight(.semibold))
                }
            }

            Divider()

            VStack(alignment: .leading, spacing: 4) {
                Text("Coachy")
                    .font(.caption.weight(.semibold))
                    .foregroundStyle(.secondary)
                Text(entry.snapshot.coachyPoseName.capitalized)
                    .font(.body.weight(.medium))
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .containerBackground(.fill, for: .widget)
    }

    private var accessoryCircularView: some View {
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

    private var accessoryRectangularView: some View {
        VStack(alignment: .leading, spacing: 2) {
            Text("Credits & Streak")
                .font(.caption2)
                .foregroundStyle(.secondary)

            HStack(spacing: 8) {
                Text("\(entry.snapshot.creditsBalance) credits")
                    .font(.system(.caption, design: .monospaced))
                    .foregroundStyle(.primary)

                Divider()
                    .frame(height: 12)

                Text("🔥 \(entry.snapshot.streakDays)")
                    .font(.system(.caption, design: .monospaced))
                    .foregroundStyle(.primary)
            }
        }
        .containerBackground(.fill, for: .accessoryRectangular)
    }

    private var accessoryInlineView: some View {
        Text("🔥 \(entry.snapshot.streakDays) · \(entry.snapshot.creditsBalance)¢")
            .font(.system(.caption, design: .monospaced))
    }
}

struct CreditsWidgetProvider: TimelineProvider {
    typealias Entry = CreditsWidgetEntry

    func placeholder(in context: Context) -> CreditsWidgetEntry {
        CreditsWidgetEntry(
            date: Date(),
            snapshot: WidgetSnapshot(creditsBalance: 0, streakDays: 0)
        )
    }

    func getSnapshot(in context: Context, completion: @escaping (CreditsWidgetEntry) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = CreditsWidgetEntry(date: Date(), snapshot: snapshot)
        completion(entry)
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<CreditsWidgetEntry>) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = CreditsWidgetEntry(date: Date(), snapshot: snapshot)

        // Refresh every 15 minutes.
        let nextRefresh = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date()
        let timeline = Timeline(entries: [entry], policy: .after(nextRefresh))
        completion(timeline)
    }

}

struct CreditsWidget: Widget {
    let kind: String = "CreditsWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: CreditsWidgetProvider()) { entry in
            CreditsWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Credits Widget")
        .description("See your focus credits and streak on the home screen.")
        .supportedFamilies([.systemSmall, .systemMedium, .accessoryCircular, .accessoryRectangular, .accessoryInline])
    }
}

