#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Today tab — Morning Brief + Evening Shutdown.
///
/// Consumes `RitualsApi` via `holder.core.rituals()` to render a mascot-first
/// planning ritual. The morning brief surfaces top priorities, a schedule
/// preview with Hard/Soft conflict annotations, and an intention capture.
/// The evening shutdown auto-unlocks after 6pm local (or via "Wrap up early")
/// and reports against the `actualsToday` stub collector fed by "Mark done"
/// taps on scheduled-window cards.
///
/// Traces to: planning-coach ritual (Morning Brief + Evening Shutdown).
struct RitualsView: View {
    @EnvironmentObject private var holder: CoreHolder

    // Morning Brief state.
    @State private var brief: MorningBriefDto?
    @State private var briefLoading: Bool = true
    @State private var intentionDraft: String = ""
    @State private var intentionCaptured: Bool = false

    // Actuals collector stub — populated when the user taps "Mark done" on a
    // scheduled-window card. At shutdown time, any scheduled window that was
    // never marked done is reported as a skipped TaskActual (cancelled=true).
    @State private var actualsToday: [TaskActualDto] = []
    @State private var markedDoneIds: Set<String> = []

    // Evening Shutdown state.
    @State private var shutdown: EveningShutdownDto?
    @State private var shutdownLoading: Bool = false
    @State private var wrapUpEarly: Bool = false

    // Error surface.
    @State private var alertMessage: String?
    @State private var showAlert: Bool = false

    private var eveningUnlocked: Bool {
        wrapUpEarly || Calendar.current.component(.hour, from: Date()) >= 18
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 24) {
                    morningBriefSection
                    eveningSection
                }
                .padding()
            }
            .navigationTitle("Today")
            .background(Color.app.background.ignoresSafeArea())
        }
        .task(id: holder.revision) { await loadMorningBrief() }
        .alert("Ritual error", isPresented: $showAlert, presenting: alertMessage) { _ in
            Button("OK", role: .cancel) { alertMessage = nil }
        } message: { msg in
            Text(msg)
        }
    }

    // MARK: - Morning Brief

    @ViewBuilder
    private var morningBriefSection: some View {
        if briefLoading {
            coachyLoadingView(
                isLoading: $briefLoading,
                pose: .curious,
                emotion: .focused,
                reason: "Generating your brief…"
            )
        } else if let brief {
            VStack(spacing: 20) {
                coachyHero(opening: brief.coachyOpening, pose: .confident, emotion: .focused)

                intentionCaptureCard(current: brief.intention)

                if brief.topPriorities.isEmpty && brief.schedulePreview.windows.isEmpty {
                    emptyStateCard
                } else {
                    topPrioritiesCard(brief.topPriorities)
                    schedulePreviewCard(brief.schedulePreview)
                }
            }
        } else {
            emptyStateCard
        }
    }

    private var loadingCard: some View {
        VStack(spacing: 16) {
            CoachyView(
                state: CoachyState(pose: .curious, emotion: .focused, bubbleText: "Pulling your day together…"),
                size: 180
            )
            .scaleEffect(briefLoading ? 1.04 : 1.0)
            .animation(
                .easeInOut(duration: 1.2).repeatForever(autoreverses: true),
                value: briefLoading
            )
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 40)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private var emptyStateCard: some View {
        VStack(spacing: 12) {
            CoachyView(
                state: CoachyState(pose: .encouraging, emotion: .happy, bubbleText: "Nothing on deck yet — let's change that."),
                size: 180
            )
            NavigationLink {
                RulesListView()
            } label: {
                Text("No tasks yet — go add one")
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, 18).padding(.vertical, 12)
                    .background(Color.app.accent)
                    .foregroundStyle(Color.app.accentOn)
                    .clipShape(Capsule())
            }
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 28)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func coachyHero(opening: String, pose: CoachyPose, emotion: CoachyEmotion) -> some View {
        VStack(spacing: 8) {
            Text("Today's Brief")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
                .frame(maxWidth: .infinity, alignment: .leading)
            CoachyView(
                state: CoachyState(pose: pose, emotion: emotion, bubbleText: opening),
                size: 240
            )
            .frame(maxWidth: .infinity)
        }
        .padding()
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func intentionCaptureCard(current: String?) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                Text("Intention")
                    .font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                Spacer()
                if intentionCaptured {
                    Label("Captured", systemImage: "checkmark.circle.fill")
                        .labelStyle(.iconOnly)
                        .foregroundStyle(Color.app.accent)
                        .transition(.opacity)
                }
            }
            TextField("Today I want to…", text: $intentionDraft)
                .textFieldStyle(.roundedBorder)
                .submitLabel(.done)
                .onSubmit { captureIntention() }
            if let existing = current, !existing.isEmpty, !intentionCaptured {
                Text("Previously: \(existing)")
                    .font(.caption2)
                    .foregroundStyle(Color.app.foreground.opacity(0.55))
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func topPrioritiesCard(_ items: [TopPriorityLineDto]) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            Text("Top priorities")
                .font(.caption.weight(.semibold))
                .foregroundStyle(Color.app.foreground.opacity(0.7))
            ForEach(items.prefix(3), id: \.taskId) { p in
                priorityRow(p)
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func priorityRow(_ p: TopPriorityLineDto) -> some View {
        HStack(alignment: .top, spacing: 12) {
            VStack(alignment: .leading, spacing: 4) {
                Text(p.title)
                    .font(.body.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                HStack(spacing: 6) {
                    Text(p.deadlineLabel)
                        .font(.caption2)
                        .foregroundStyle(Color.app.foreground.opacity(0.6))
                    rigidityPill(p.rigidityTag)
                    Text("\(p.estimatedDurationMinutes)m")
                        .font(.caption2.weight(.medium))
                        .foregroundStyle(Color.app.accent)
                }
            }
            Spacer()
        }
        .padding(.vertical, 6)
    }

    private func rigidityPill(_ tag: String) -> some View {
        Text(tag)
            .font(.caption2.weight(.bold))
            .padding(.horizontal, 8).padding(.vertical, 3)
            .background(
                Capsule().fill(Color.app.accent.opacity(0.18))
            )
            .foregroundStyle(Color.app.accent)
    }

    private func schedulePreviewCard(_ preview: SchedulePreviewDto) -> some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                Text("Schedule")
                    .font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                Spacer()
                if preview.hardConflicts > 0 {
                    conflictBadge(count: Int(preview.hardConflicts), kind: .hard)
                }
                if preview.softConflicts > 0 {
                    conflictBadge(count: Int(preview.softConflicts), kind: .soft)
                }
            }
            ScrollView(.horizontal, showsIndicators: false) {
                HStack(spacing: 10) {
                    ForEach(Array(preview.windows.enumerated()), id: \.offset) { _, w in
                        scheduleWindowCard(w)
                    }
                }
            }
        }
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private enum ConflictKind { case hard, soft }
    private func conflictBadge(count: Int, kind: ConflictKind) -> some View {
        let color: Color = (kind == .hard) ? .red : .orange
        let label = (kind == .hard) ? "Hard" : "Soft"
        return Text("\(count) \(label)")
            .font(.caption2.weight(.bold))
            .padding(.horizontal, 8).padding(.vertical, 3)
            .overlay(
                Capsule().strokeBorder(color.opacity(0.8), lineWidth: 1)
            )
            .foregroundStyle(color)
    }

    private func scheduleWindowCard(_ w: ScheduleWindowLineDto) -> some View {
        let isHard = w.kind.lowercased() == "hard"
        let isSoft = w.kind.lowercased() == "soft"
        let done = markedDoneIds.contains(windowId(w))
        return VStack(alignment: .leading, spacing: 6) {
            Text(timeLabel(w.startsAtIso))
                .font(.caption2.weight(.bold))
                .foregroundStyle(Color.app.accent)
            Text(w.title)
                .font(.footnote.weight(.semibold))
                .foregroundStyle(Color.app.foreground)
                .lineLimit(2)
            Text("\(timeLabel(w.startsAtIso))–\(timeLabel(w.endsAtIso))")
                .font(.caption2)
                .foregroundStyle(Color.app.foreground.opacity(0.6))
            Button {
                markDone(w)
            } label: {
                Label(done ? "Done" : "Mark done",
                      systemImage: done ? "checkmark.circle.fill" : "circle")
                    .font(.caption2.weight(.semibold))
            }
            .buttonStyle(.plain)
            .foregroundStyle(done ? Color.app.accent : Color.app.foreground.opacity(0.7))
            .disabled(done)
        }
        .frame(width: 160, alignment: .leading)
        .padding(12)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.background)
        )
        .overlay(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .strokeBorder(
                    isHard ? Color.red.opacity(0.85)
                    : (isSoft ? Color.orange.opacity(0.75)
                       : Color.app.accent.opacity(0.3)),
                    lineWidth: isHard ? 2 : 1
                )
        )
    }

    // MARK: - Evening Shutdown

    @ViewBuilder
    private var eveningSection: some View {
        VStack(alignment: .leading, spacing: 14) {
            HStack {
                Text("Evening Shutdown")
                    .font(.headline)
                    .foregroundStyle(Color.app.foreground)
                Spacer()
                if !eveningUnlocked {
                    Button("Wrap up early") {
                        withAnimation { wrapUpEarly = true }
                        Task { await runShutdown() }
                    }
                    .font(.caption.weight(.semibold))
                    .buttonStyle(.bordered)
                    .tint(Color.app.accent)
                }
            }

            if eveningUnlocked {
                if shutdownLoading {
                    coachyLoadingView(
                        isLoading: $shutdownLoading,
                        pose: .curious,
                        emotion: .focused,
                        reason: "Reviewing your day…"
                    )
                } else if let s = shutdown {
                    shutdownContent(s)
                } else {
                    Button("Run shutdown now") {
                        Task { await runShutdown() }
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                }
            } else {
                Text("Unlocks after 6pm, or tap Wrap up early.")
                    .font(.caption)
                    .foregroundStyle(Color.app.foreground.opacity(0.6))
            }
        }
        .frame(maxWidth: .infinity, alignment: .leading)
        .padding(16)
        .background(
            RoundedRectangle(cornerRadius: 16, style: .continuous)
                .fill(Color.app.surface)
        )
    }

    private func shutdownContent(_ s: EveningShutdownDto) -> some View {
        let shippedCount = s.shipped.count
        let pose: CoachyPose = shippedCount >= 1 ? .encouraging : .sleepy
        let emotion: CoachyEmotion = shippedCount >= 1 ? .proud : .disappointed

        return VStack(alignment: .leading, spacing: 12) {
            CoachyView(
                state: CoachyState(pose: pose, emotion: emotion, bubbleText: s.coachyClosing),
                size: 180
            )
            .frame(maxWidth: .infinity)

            if !s.shipped.isEmpty {
                Text("Shipped").font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                ForEach(s.shipped, id: \.id) { t in
                    HStack(spacing: 8) {
                        Image(systemName: "checkmark.circle.fill")
                            .foregroundStyle(Color.app.accent)
                        Text(t.title).font(.subheadline)
                        Spacer()
                        Text("\(t.actualMinutes)/\(t.plannedMinutes)m")
                            .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.6))
                    }
                }
            }

            if !s.slipped.isEmpty {
                Text("Slipped").font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                ForEach(s.slipped, id: \.id) { t in
                    HStack(spacing: 8) {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundStyle(.orange)
                        Text(t.title).font(.subheadline)
                        Spacer()
                        Text(t.reason)
                            .font(.caption2.weight(.semibold))
                            .padding(.horizontal, 6).padding(.vertical, 2)
                            .background(Capsule().fill(Color.orange.opacity(0.18)))
                            .foregroundStyle(.orange)
                    }
                }
            }

            if !s.carryover.isEmpty {
                Text("Carryover").font(.caption.weight(.semibold))
                    .foregroundStyle(Color.app.foreground.opacity(0.7))
                ForEach(s.carryover, id: \.self) { c in
                    HStack {
                        Image(systemName: "arrow.right.circle")
                            .foregroundStyle(Color.app.foreground.opacity(0.7))
                        Text(c).font(.subheadline)
                        Spacer()
                    }
                }
            }

            if !s.streakDeltas.isEmpty {
                HStack(spacing: 8) {
                    ForEach(Array(s.streakDeltas.keys.sorted()), id: \.self) { name in
                        let delta = s.streakDeltas[name] ?? 0
                        let sign = delta >= 0 ? "+" : ""
                        Text("\(name) \(sign)\(delta)")
                            .font(.caption2.weight(.bold))
                            .padding(.horizontal, 8).padding(.vertical, 3)
                            .background(Capsule().fill(Color.app.accent.opacity(0.18)))
                            .foregroundStyle(Color.app.accent)
                    }
                }
            }

            Text(s.winsSummary)
                .font(.caption)
                .foregroundStyle(Color.app.foreground.opacity(0.7))
        }
    }

    // MARK: - Actions

    private func captureIntention() {
        let trimmed = intentionDraft.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !trimmed.isEmpty else { return }
        let dateStr = isoDateToday()
        do {
            try holder.core.rituals().captureIntention(date: dateStr, intention: trimmed)
            withAnimation { intentionCaptured = true }
            holder.bump()
        } catch {
            showError("Couldn't save intention: \(error)")
        }
    }

    private func markDone(_ w: ScheduleWindowLineDto) {
        let id = windowId(w)
        guard !markedDoneIds.contains(id) else { return }
        let planned = durationMinutes(startIso: w.startsAtIso, endIso: w.endsAtIso)
        let actual = TaskActualDto(
            taskId: id,
            actualMinutes: UInt32(planned),
            completedAtIso: ISO8601DateFormatter().string(from: Date()),
            cancelled: false
        )
        // Replace any prior entry for this window id, then insert.
        actualsToday.removeAll { $0.taskId == id }
        actualsToday.append(actual)
        markedDoneIds.insert(id)
    }

    @MainActor
    private func loadMorningBrief() async {
        briefLoading = true
        defer { briefLoading = false }
        do {
            let b = try holder.core.rituals().generateMorningBrief()
            withAnimation { brief = b }
            if let existing = b.intention, !existing.isEmpty {
                intentionDraft = existing
                intentionCaptured = true
            }
        } catch {
            showError("Couldn't load morning brief: \(error)")
        }
    }

    @MainActor
    private func runShutdown() async {
        shutdownLoading = true
        defer { shutdownLoading = false }

        // Stub collector: any scheduled window that was NOT marked done
        // becomes a skipped (cancelled) TaskActual. This lets the Rust
        // Rituals engine compute slippage reasons without a real task store.
        var merged = actualsToday
        if let windows = brief?.schedulePreview.windows {
            for w in windows {
                let id = windowId(w)
                guard !markedDoneIds.contains(id) else { continue }
                merged.append(TaskActualDto(
                    taskId: id,
                    actualMinutes: 0,
                    completedAtIso: nil,
                    cancelled: true
                ))
            }
        }

        do {
            let s = try holder.core.rituals().generateEveningShutdown(actuals: merged)
            withAnimation { shutdown = s }
        } catch {
            showError("Couldn't run evening shutdown: \(error)")
        }
    }

    private func showError(_ msg: String) {
        alertMessage = msg
        showAlert = true
    }

    // MARK: - Helpers

    @ViewBuilder
    private func coachyLoadingView(
        isLoading: Binding<Bool>,
        pose: CoachyPose,
        emotion: CoachyEmotion,
        reason: String
    ) -> some View {
        ZStack {
            LinearGradient(
                gradient: Gradient(colors: [Color.app.surface, Color.app.background]),
                startPoint: .topLeading,
                endPoint: .bottomTrailing
            )
            .ignoresSafeArea()

            VStack(spacing: 20) {
                CoachyView(
                    state: CoachyState(pose: pose, emotion: emotion, bubbleText: reason),
                    size: 200
                )
                ProgressView()
                    .controlSize(.large)
            }
            .padding()
        }
    }

    private func windowId(_ w: ScheduleWindowLineDto) -> String {
        "\(w.startsAtIso)|\(w.title)"
    }

    private func timeLabel(_ iso: String) -> String {
        let f = ISO8601DateFormatter()
        f.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        let d = f.date(from: iso) ?? ISO8601DateFormatter().date(from: iso)
        guard let d else { return iso.prefix(5).description }
        let out = DateFormatter()
        out.dateFormat = "h:mma"
        out.amSymbol = "a"
        out.pmSymbol = "p"
        return out.string(from: d).lowercased()
    }

    private func durationMinutes(startIso: String, endIso: String) -> Int {
        let f = ISO8601DateFormatter()
        f.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        let s = f.date(from: startIso) ?? ISO8601DateFormatter().date(from: startIso)
        let e = f.date(from: endIso) ?? ISO8601DateFormatter().date(from: endIso)
        guard let s, let e else { return 0 }
        return max(0, Int(e.timeIntervalSince(s) / 60.0))
    }

    private func isoDateToday() -> String {
        let f = DateFormatter()
        f.dateFormat = "yyyy-MM-dd"
        f.timeZone = .current
        return f.string(from: Date())
    }
}
#endif
