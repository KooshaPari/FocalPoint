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
                        Label("Add task", systemImage: "plus")
                    }
                }
            }
            .sheet(isPresented: $showAdd) {
                AddTaskSheet(onSubmit: { input in
                    addTask(input)
                })
            }
            .alert("Task error", isPresented: $showAlert, presenting: alertMessage) { _ in
                Button("OK", role: .cancel) { alertMessage = nil }
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
                    bubbleText: "Ready for your first task?"
                ),
                size: 240
            )
            Button {
                showAdd = true
            } label: {
                Label("Add one", systemImage: "plus.circle.fill")
                    .font(.subheadline.weight(.semibold))
                    .padding(.horizontal, 18).padding(.vertical, 12)
                    .background(Color.app.accent)
                    .foregroundStyle(Color.app.accentOn)
                    .clipShape(Capsule())
            }
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
                    .swipeActions(edge: .trailing, allowsFullSwipe: true) {
                        Button(role: .destructive) {
                            remove(t)
                        } label: {
                            Label("Delete", systemImage: "trash")
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
                }
            }
        }
        .padding(.vertical, 4)
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
                Section("Task") {
                    TextField("Title", text: $title)
                }
                Section("Duration") {
                    Stepper(value: $durationMinutes, in: 5...240, step: 5) {
                        HStack {
                            Text("Minutes")
                            Spacer()
                            Text("\(durationMinutes)")
                                .foregroundStyle(.secondary)
                        }
                    }
                }
                Section("Priority") {
                    VStack(alignment: .leading) {
                        Slider(value: $priority, in: 0...1)
                        Text("Weight: \(String(format: "%.2f", priority))")
                            .font(.caption).foregroundStyle(.secondary)
                    }
                }
                Section("Deadline") {
                    Toggle("Has deadline", isOn: $hasDeadline)
                    if hasDeadline {
                        DatePicker("When", selection: $deadline)
                        Picker("Rigidity", selection: $rigidity) {
                            ForEach(rigidities, id: \.self) { r in
                                Text(r.capitalized).tag(r)
                            }
                        }
                        .pickerStyle(.segmented)
                    }
                }
            }
            .navigationTitle("New Task")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .cancellationAction) {
                    Button("Cancel") { dismiss() }
                }
                ToolbarItem(placement: .confirmationAction) {
                    Button("Add") {
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
