import WidgetKit
import SwiftUI

struct TodayBriefWidgetEntry: TimelineEntry {
    let date: Date
    let snapshot: WidgetSnapshot
}

struct TodayBriefWidgetEntryView: View {
    var entry: TodayBriefWidgetEntry
    @Environment(\.widgetFamily) var family

    var body: some View {
        switch family {
        case .systemMedium, .systemLarge:
            mediumLargeView
        default:
            mediumLargeView
        }
    }

    private var mediumLargeView: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Today's Top Tasks")
                        .font(.headline.weight(.semibold))
                    Text("From your Morning Brief")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                Spacer()
                Image(systemName: "checklist")
                    .font(.title3)
                    .foregroundStyle(.blue)
            }

            Divider()

            if entry.snapshot.topPriorities.isEmpty {
                VStack(alignment: .center, spacing: 8) {
                    Image(systemName: "doc.text")
                        .font(.title2)
                        .foregroundStyle(.secondary)
                    Text("No tasks yet")
                        .font(.subheadline)
                        .foregroundStyle(.secondary)
                    Text("Open FocalPoint to get started")
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
                .frame(maxWidth: .infinity, alignment: .center)
                .padding(.vertical, 20)
            } else {
                VStack(alignment: .leading, spacing: 8) {
                    ForEach(Array(entry.snapshot.topPriorities.enumerated()), id: \.offset) { idx, task in
                        HStack(spacing: 8) {
                            Circle()
                                .fill(Color.blue)
                                .frame(width: 4, height: 4)
                            Text(task)
                                .font(.subheadline)
                                .lineLimit(1)
                            Spacer()
                        }
                    }
                }
            }
        }
        .padding()
        .background(Color(.systemBackground))
        .containerBackground(.fill, for: .widget)
    }
}

struct TodayBriefWidgetProvider: TimelineProvider {
    typealias Entry = TodayBriefWidgetEntry

    func placeholder(in context: Context) -> TodayBriefWidgetEntry {
        TodayBriefWidgetEntry(
            date: Date(),
            snapshot: WidgetSnapshot()
        )
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<TodayBriefWidgetEntry>) -> Void) {
        let snapshot = WidgetCoreAccess.fetchSnapshot()
        let entry = TodayBriefWidgetEntry(date: Date(), snapshot: snapshot)

        // Refresh every 15 minutes.
        let nextRefresh = Calendar.current.date(byAdding: .minute, value: 15, to: Date()) ?? Date()
        let timeline = Timeline(entries: [entry], policy: .after(nextRefresh))
        completion(timeline)
    }

    func recommendations() -> [WidgetRecommendation] {
        [WidgetRecommendation(intent: TodayBriefWidgetConfigIntent())]
    }
}

struct TodayBriefWidget: Widget {
    let kind: String = "TodayBriefWidget"

    var body: some WidgetConfiguration {
        StaticConfiguration(kind: kind, provider: TodayBriefWidgetProvider()) { entry in
            TodayBriefWidgetEntryView(entry: entry)
        }
        .configurationDisplayName("Today's Priorities")
        .description("See your top 3 priorities from the Morning Brief without opening the app.")
        .supportedFamilies([.systemMedium, .systemLarge])
    }
}

// Stub intent for configuration (not fully implemented).
fileprivate struct TodayBriefWidgetConfigIntent: AppIntent {
    static var title: LocalizedStringResource = "Today's Priorities"
    static var description: IntentDescription = "Show your top tasks from the Morning Brief."

    func perform() async throws -> some IntentResult {
        .result()
    }
}
