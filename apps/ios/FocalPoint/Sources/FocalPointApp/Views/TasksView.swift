#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

/// Tasks tab — user-facing CRUD over `TaskApi` (FFI sub-API over
/// `SqliteTaskStore`). Sorted by priority desc then title asc. Tap-and-hold
/// gestures: tap row → mark done; swipe → delete. "+ Add" opens a sheet.
///
/// Traces to: FR-PLAN-001 (Task model exposure).
struct TasksView: View {
    @EnvironmentObject private var holder: CoreHolder

    @State private var tasks: [TaskSummaryDto] = []
    @State private var loading: Bool = true
    @State private var showAdd: Bool = false

    @State private var alertMessage: String?
    @State private var showAlert: Bool = false

    var body: some View {
        NavigationStack {
            Group {
                if loading {
                    ProgressView().controlSize(.large).frame(maxWidth: .infinity, maxHeight: .infinity)
                } else if tasks.isEmpty {
                    emptyState
                } else {
                    taskList
                }
            }
            .navigationTitle("Tasks")
            .background(Color.app.background.ignoresSafeArea())
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button {
                        showAdd = true
                    } label: {
                        Label(String(localized: "Add task", defaultValue: "Add task"), systemImage: "plus")
                    }
                    .accessibilityLabel(String(localized: "Add task", defaultValue: "Add task"))
                }
            }
            .sheet(isPresented: $showAdd) {
                AddTaskSheet(onSubmit: { input in
                    addTask(input)
                })
            }
            .alert(String(localized: "Task error", defaultValue: "Task error"), isPresented: $showAlert, presenting: alertMessage) { _ in
                Button(String(localized: "OK", defaultValue: "OK"), role: .cancel) { alertMessage = nil }
            } message: { msg in
                Text(msg)
            }
            .task(id: holder.revision) { reload() }
        }
    }

    // MARK: - Empty state (Coachy-first)

    private var emptyState: some View {
        VStack(spacing: 16) {
            CoachyView(
                state: CoachyState(
                    pose: .encouraging,
                    emotion: .happy,
                    bubbleText: String(localized: "Ready for your first task?", defaultValue: "Ready for your first task?")
                ),
                size: 240
            )
            .accessibilityLabel(String(localized: "Coachy mascot", defaultValue: "Coachy mascot"))
            Button {
                showAdd = true
            } label: {
                Label(String(localized: "Add task", defaultValue: "Add task"), systemImage: "plus.circle.fill")
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, 18).padding(.vertical, 12)
                    .background(Color.app.accent)
                    .foregroundStyle(Color.app.accentOn)
                    .clipShape(Capsule())
            }
            .accessibilityLabel(String(localized: "Add task", defaultValue: "Add task"))
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
        .padding()
    }

    // MARK: - List

    private var sorted: [TaskSummaryDto] {
        tasks.sorted { a, b in
            if a.priorityWeight != b.priorityWeight { return a.priorityWeight > b.priorityWeight }
            return a.title.localizedCaseInsensitiveCompare(b.title) == .orderedAscending
        }
    }

    private var taskList: some View {
        List {
            ForEach(sorted, id: \.id) { t in
                TaskRow(task: t)
                    .listRowBackground(Color.app.surface)
                    .contentShape(Rectangle())
                    .onTapGesture { markDone(t) }
                    .accessibilityHint(String(localized: "Tap to mark done", defaultValue: "Tap to mark done"))
                    .swipeActions(edge: .trailing, allowsFullSwipe: true) {
                        Button(role: .destructive) {
                            remove(t)
                        } label: {
                            Label(String(localized: "Delete", defaultValue: "Delete"), systemImage: "trash")
                        }
                    }
            }
        }
        .listStyle(.insetGrouped)
        .scrollContentBackground(.hidden)
        .background(Color.app.background)
    }

    // MARK: - Actions

    private func reload() {
        loading = true
        do {
            tasks = try holder.core.tasks().list()
        } catch {
            alertMessage = "Load failed: \(error.localizedDescription)"
            showAlert = true
            tasks = []
        }
        loading = false
    }

    private func addTask(_ input: TaskInputDto) {
        do {
            _ = try holder.core.tasks().add(input: input)
            holder.bump()
            reload()
        } catch {
            alertMessage = "Add failed: \(error.localizedDescription)"
            showAlert = true
        }
    }

    private func remove(_ t: TaskSummaryDto) {
        do {
            try holder.core.tasks().remove(taskId: t.id)
            holder.bump()
            reload()
        } catch {
            alertMessage = "Delete failed: \(error.localizedDescription)"
            showAlert = true
        }
    }

    private func markDone(_ t: TaskSummaryDto) {
        guard t.status != "done" else { return }
        do {
            try holder.core.tasks().markDone(taskId: t.id)
            holder.bump()
            reload()
        } catch {
            alertMessage = "Mark done failed: \(error.localizedDescription)"
            showAlert = true
        }
    }
}

// MARK: - Row

private struct TaskRow: View {
    let task: TaskSummaryDto

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            HStack(spacing: 8) {
                if task.status == "done" {
                    Image(systemName: "checkmark.circle.fill")
                        .foregroundStyle(Color.app.accent)
                        .accessibilityLabel(String(localized: "Completed", defaultValue: "Completed"))
                }
                Text(task.title)
                    .font(.body.weight(.semibold))
                    .foregroundStyle(Color.app.foreground)
                    .strikethrough(task.status == "done")
                Spacer()
                durationChip
            }
            HStack(spacing: 10) {
                priorityBar
                if let label = deadlineLabel {
                    Text(label)
                        .font(.caption2.weight(.medium))
                        .foregroundStyle(deadlineColor)
                        .padding(.horizontal, 6).padding(.vertical, 2)
                        .background(deadlineColor.opacity(0.15))
                        .clipShape(Capsule())
                        .accessibilityLabel(String(localized: "Deadline: \(label)", defaultValue: "Deadline: \(label)"))
                }
            }
        }
        .padding(.vertical, 4)
        .accessibilityElement(children: .combine)
        .accessibilityLabel(String(localized: task.title, defaultValue: task.title))
        .accessibilityHint(String(localized: "\(task.durationMinutes)m duration, \(task.status)", defaultValue: "\(task.durationMinutes)m duration, \(task.status)"))
    }

    private var durationChip: some View {
        Text("\(task.durationMinutes)m")
            .font(.caption2.weight(.semibold))
            .padding(.horizontal, 8).padding(.vertical, 3)
            .background(Color.app.foreground.opacity(0.10))
            .foregroundStyle(Color.app.foreground.opacity(0.8))
            .clipShape(Capsule())
    }

    private var priorityBar: some View {
        let pct = CGFloat(max(0, min(1, task.priorityWeight)))
        return GeometryReader { geo in
            ZStack(alignment: .leading) {
                Capsule().fill(Color.app.foreground.opacity(0.12))
                Capsule()
                    .fill(Color.app.accent)
                    .frame(width: max(6, geo.size.width * pct))
            }
        }
        .frame(width: 80, height: 6)
    }

    private var deadlineLabel: String? {
        guard let iso = task.deadlineIso else { return nil }
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        let parsed = formatter.date(from: iso) ?? ISO8601DateFormatter().date(from: iso)
        guard let date = parsed else { return iso }
        let df = DateFormatter()
        df.dateStyle = .medium
        df.timeStyle = .short
        return "\(df.string(from: date)) · \(task.deadlineRigidity)"
    }

    private var deadlineColor: Color {
        guard let iso = task.deadlineIso else { return Color.app.foreground.opacity(0.6) }
        let formatter = ISO8601DateFormatter()
        formatter.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
        let parsed = formatter.date(from: iso) ?? ISO8601DateFormatter().date(from: iso)
        guard let date = parsed else { return Color.app.foreground.opacity(0.6) }
        let isHard = task.deadlineRigidity == "hard"
        let now = Date()
        if isHard && date < now { return .red }
        if isHard && date.timeIntervalSince(now) < 60 * 60 * 24 { return .orange }
        return Color.app.foreground.opacity(0.6)
    }
}

// MARK: - Add sheet

private struct AddTaskSheet: View {
    @Environment(\.dismiss) private var dismiss
    let onSubmit: (TaskInputDto) -> Void

    @State private var title: String = ""
    @State private var durationMinutes: Int = 30
    @State private var priority: Double = 0.5
    @State private var hasDeadline: Bool = false
    @State private var deadline: Date = Date().addingTimeInterval(60 * 60 * 24)
    @State private var rigidity: String = "soft"

    private let rigidities: [String] = ["hard", "semi", "soft"]

    var body: some View {
        NavigationStack {
            Form {
                Section(String(localized: "Task", defaultValue: "Task")) {
                    TextField(String(localized: "Title", defaultValue: "Title"), text: $title)
                        .accessibilityLabel(String(localized: "Task title", defaultValue: "Task title"))
                }
                Section(String(localized: "Duration", defaultValue: "Duration")) {
                    Stepper(value: $durationMinutes, in: 5...240, step: 5) {
                        HStack {
                            Text(String(localized: "Minutes", defaultValue: "Minutes"))
                            Spacer()
                            Text("\(durationMinutes)")
                                .foregroundStyle(.secondary)
                        }
                    }
                    .accessibilityLabel(String(localized: "Duration in minutes: \(durationMinutes)", defaultValue: "Duration in minutes: \(durationMinutes)"))
                }
                Section(String(localized: "Priority", defaultValue: "Priority")) {
                    VStack(alignment: .leading) {
                        Slider(value: $priority, in: 0...1)
                            .accessibilityLabel(String(localized: "Priority slider", defaultValue: "Priority slider"))
                            .accessibilityValue(String(localized: "\(String(format: "%.0f", priority * 100))%", defaultValue: "\(String(format: "%.0f", priority * 100))%"))
                        Text(String(localized: "Weight: \(String(format: "%.2f", priority))", defaultValue: "Weight: \(String(format: "%.2f", priority))"))
                            .font(.caption).foregroundStyle(.secondary)
                    }
                }
                Section(String(localized: "Deadline", defaultValue: "Deadline")) {
                    Toggle(String(localized: "Has deadline", defaultValue: "Has deadline"), isOn: $hasDeadline)
                    if hasDeadline {
                        DatePicker(String(localized: "When", defaultValue: "When"), selection: $deadline)
                            .accessibilityLabel(String(localized: "Deadline date", defaultValue: "Deadline date"))
                        Picker(String(localized: "Rigidity", defaultValue: "Rigidity"), selection: $rigidity) {
                            ForEach(rigidities, id: \.self) { r in
                                Text(r.capitalized).tag(r)
                            }
                        }
                        .pickerStyle(.segmented)
                        .accessibilityLabel(String(localized: "Deadline rigidity", defaultValue: "Deadline rigidity"))
                    }
                }
            }
            .navigationTitle(String(localized: "New Task", defaultValue: "New Task"))
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button(String(localized: "Cancel", defaultValue: "Cancel")) { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button(String(localized: "Add", defaultValue: "Add")) {
                        submit()
                    }
                    .disabled(title.trimmingCharacters(in: .whitespaces).isEmpty)
                }
            }
        }
    }

    private func submit() {
        let iso: String? = hasDeadline ? ISO8601DateFormatter().string(from: deadline) : nil
        let input = TaskInputDto(
            title: title,
            durationMinutes: UInt32(durationMinutes),
            priorityWeight: Float(priority),
            deadlineIso: iso,
            deadlineRigidity: hasDeadline ? rigidity : "soft"
        )
        onSubmit(input)
        dismiss()
    }
}
#endif
