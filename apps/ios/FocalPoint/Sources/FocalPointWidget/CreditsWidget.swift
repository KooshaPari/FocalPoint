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
}

struct CreditsWidgetProvider: TimelineProvider {
    typealias Entry = CreditsWidgetEntry

    func placeholder(in context: Context) -> CreditsWidgetEntry {
        CreditsWidgetEntry(
            date: Date(),
            snapshot: WidgetSnapshot(creditsBalance: 0, streakDays: 0)
        )
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<CreditsWidgetEntry>) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = CreditsWidgetEntry(date: Date(), snapshot: snapshot)

        // Refresh every 15 minutes.
        let nextRefresh = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date()
        let timeline = Timeline(entries: [entry], policy: .after(nextRefresh))
        completion(timeline)
    }

    func recommendations() -> [WidgetRecommendation] {
        [WidgetRecommendation(intent: CreditsWidgetConfigIntent())]
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
        .supportedFamilies([.systemSmall, .systemMedium])
    }
}

// Stub intent for configuration (not fully implemented).
fileprivate struct CreditsWidgetConfigIntent: AppIntent {
    static var title: LocalizedStringResource = "Credits Widget"
    static var description: IntentDescription = "Show your FocalPoint credits and streak."

    func perform() async throws -> some IntentResult {
        .result()
    }
}
