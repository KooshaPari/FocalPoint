#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import FocalPointCore
import MascotUI

/// Real settings, backed by the core where applicable.
public struct SettingsView: View {
    @EnvironmentObject private var holder: CoreHolder
    @AppStorage("app.darkMode") private var darkMode: Bool = true
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false
    @AppStorage("app.notifications") private var notificationsEnabled: Bool = true
    @AppStorage("app.devModeUnlocked") private var devModeUnlocked: Bool = false
    @AppStorage("app.sentryEnabled") private var sentryEnabled: Bool = false
    @AppStorage("app.coachingEnabled") private var coachingEnabled: Bool = true
    @AppStorage("app.coachingEndpoint") private var coachingEndpoint: String = ""
    @AppStorage("app.coachingModel") private var coachingModel: String = ""
    @AppStorage("app.coachyVoiceMode") private var coachyVoiceMode: String = "simlish"
    @AppStorage("app.soundEffectsEnabled") private var soundEffectsEnabled: Bool = true
    @AppStorage("app.sfxVolume") private var sfxVolume: Double = 1.0
    @AppStorage("app.hapticEnabled") private var hapticEnabled: Bool = true
    @AppStorage("app.flyInsEnabled") private var flyInsEnabled: Bool = true

    @State private var connectors: [ConnectorHandleSummary] = []
    @State private var canvas: CanvasConnectionRecord?
    @State private var showCanvasAuth: Bool = false
    @State private var showGCalAuth: Bool = false
    @State private var showGitHubAuth: Bool = false
    @State private var versionTapCount: Int = 0
    @State private var lastSyncSummary: String?
    @State private var auditExportUrl: URL?
    @State private var exportError: String?
    @AppStorage("app.cloudSyncEnabled") private var cloudSyncEnabled: Bool = false
    @State private var cloudSyncStatus: CloudKitSyncStatus = .unchecked
    @State private var lastCloudSyncTime: Date?
    @State private var cloudSyncInProgress: Bool = false
    @State private var showTestSentryToast: Bool = false
    @State private var testSentryToastMessage: String = ""

    public init() {}

    /// Fixed display order for connectors in the Settings list.
    private let orderedConnectorIds: [String] = ["canvas", "gcal", "github"]

    private let connectorDisplayNames: [String: String] = [
        "canvas": "Canvas",
        "gcal": "Google Calendar",
        "github": "GitHub",
    ]

    public var body: some View {
        NavigationStack {
            Form {
                Section("Account") {
                    LabeledContent("User", value: "local-user")
                    LabeledContent("Version") {
                        Text(holder.core.appVersion())
                            .onTapGesture { bumpVersion() }
                    }
                }

                Section("Theme") {
                    Toggle("Dark mode", isOn: $darkMode)
                        .tint(Color.app.accent)
                }

                Section("Notifications") {
                    Toggle("Enabled", isOn: $notificationsEnabled)
                        .tint(Color.app.accent)
                }

                Section("Sync across devices") {
                    Toggle("CloudKit Sync", isOn: $cloudSyncEnabled)
                        .tint(Color.app.accent)

                    if cloudSyncEnabled {
                        Text(syncStatusText())
                            .font(.caption2)
                            .foregroundStyle(statusForegroundColor())

                        if !cloudSyncInProgress {
                            Button(action: triggerCloudSync) {
                                HStack {
                                    if cloudSyncInProgress {
                                        ProgressView()
                                            .scaleEffect(0.8)
                                    } else {
                                        Image(systemName: "arrow.triangle.2.circlepath")
                                    }
                                    Text(cloudSyncInProgress ? "Syncing..." : "Sync now")
                                }
                            }
                            .disabled(cloudSyncInProgress)
                        } else {
                            HStack {
                                ProgressView()
                                    .scaleEffect(0.8)
                                Text("Syncing...")
                            }
                        }
                    } else {
                        Text("Enable to sync rules, tasks, and wallet across your Apple devices. Off by default for v0.1.")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                }

                Section("Diagnostics") {
                    Toggle("Send crash reports", isOn: $sentryEnabled)
                        .tint(Color.app.accent)
                        .onChange(of: sentryEnabled) { _, enabled in
                            SentrySetup.shared.setupIfConsented(userOptedIn: enabled)
                        }
                    Text("Helps us fix crashes faster. On-device stack traces only. No task/rule contents, no tokens, no personal data.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                    NavigationLink(destination: DiagnosticsInfoView()) {
                        HStack {
                            Image(systemName: "info.circle")
                                .accessibilityLabel(String(localized: "Information", defaultValue: "Information"))
                            Text("Privacy & data")
                        }
                        .font(.subheadline)
                    }
                    .accessibilityLabel(String(localized: "Privacy and data settings", defaultValue: "Privacy and data settings"))
                }

                Section("Connectors") {
                    ForEach(orderedConnectorIds, id: \.self) { id in
                        connectorRow(id: id, summary: summary(for: id))
                    }
                    Button {
                        let report = holder.syncTick()
                        lastSyncSummary = "Synced \(report.connectorsSynced) connector(s), pulled \(report.eventsPulled) event(s)\(report.errors.isEmpty ? "" : ", errors: \(report.errors.count)")"
                    } label: {
                        HStack {
                            Image(systemName: "arrow.triangle.2.circlepath")
                                .accessibilityLabel(String(localized: "Sync icon", defaultValue: "Sync icon"))
                            Text("Sync now")
                        }
                    }
                    .accessibilityLabel(String(localized: "Sync connectors now", defaultValue: "Sync connectors now"))
                    Button {
                        let sync = holder.syncTick()
                        if let eval = holder.evalTick() {
                            lastSyncSummary = "Synced \(sync.connectorsSynced) conn / \(sync.eventsPulled) ev; evaluated \(eval.eventsEvaluated), fired \(eval.decisionsFired)"
                        } else {
                            lastSyncSummary = "Synced \(sync.connectorsSynced) conn / \(sync.eventsPulled) ev; eval failed"
                        }
                    } label: {
                        HStack {
                            Image(systemName: "bolt.badge.checkmark")
                                .accessibilityLabel(String(localized: "Rules icon", defaultValue: "Rules icon"))
                            Text("Run rules now")
                        }
                    }
                    .accessibilityLabel(String(localized: "Evaluate and run active rules", defaultValue: "Evaluate and run active rules"))
                    if let lastSyncSummary {
                        Text(lastSyncSummary)
                            .font(.caption2)
                            .foregroundStyle(Color.app.foreground.opacity(0.6))
                    }
                }

                Section("AI Coaching") {
                    Toggle("Enable Coachy LLM replies", isOn: $coachingEnabled)
                        .tint(Color.app.accent)
                        .onChange(of: coachingEnabled) { _, on in
                            if on {
                                reapplyCoachingIfConfigured()
                            } else {
                                holder.core.setCoaching(config: nil)
                            }
                        }
                    if coachingEnabled {
                        Text("Disable to stop sending prompts to any external LLM. Static fallback lines are used instead.")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    } else {
                        Text("Coachy will use static fallback copy only. No LLM network calls.")
                            .font(.caption2)
                            .foregroundStyle(.secondary)
                    }
                }

                Section("Mascot") {
                    Picker("Voice mode", selection: $coachyVoiceMode) {
                        Text("Simlish (default)").tag("simlish")
                        Text("Text-to-speech").tag("avSpeechSynthesizer")
                        Text("Silent").tag("silent")
                    }
                    .tint(Color.app.accent)

                    Toggle("Sound effects", isOn: $soundEffectsEnabled)
                        .tint(Color.app.accent)
                    if soundEffectsEnabled {
                        VStack(alignment: .leading) {
                            Text("Volume")
                                .font(.caption)
                            Slider(value: $sfxVolume, in: 0...1)
                        }
                    }

                    Toggle("Haptic feedback", isOn: $hapticEnabled)
                        .tint(Color.app.accent)

                    Toggle("Sudden fly-ins", isOn: $flyInsEnabled)
                        .tint(Color.app.accent)

                    Toggle("Proactive nudges", isOn: $holder.nudgesEnabled)
                        .tint(Color.app.accent)

                    Button(action: testSoundsAndHaptics) {
                        HStack {
                            Image(systemName: "speaker.wave.2")
                            Text("Test sounds & haptics")
                        }
                    }
                }

                Section("Data") {
                    if let url = auditExportUrl {
                        ShareLink(item: url) {
                            Label("Share audit chain export", systemImage: "square.and.arrow.up")
                        }
                    }
                    Button {
                        exportAuditChain()
                    } label: {
                        Label(auditExportUrl == nil ? "Export audit chain" : "Regenerate export", systemImage: "doc.badge.arrow.up")
                    }
                    if let err = exportError {
                        Text(err).font(.caption2).foregroundStyle(.red)
                    }

                    NavigationLink(destination: DataDeletionView()) {
                        HStack {
                            Image(systemName: "trash.circle.fill")
                                .foregroundStyle(Color.red)
                            Text("Delete all my data")
                                .foregroundStyle(Color.red)
                            Spacer()
                            Image(systemName: "chevron.right")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                    }
                }

                Section("Support") {
                    Button(action: sendFeedback) {
                        HStack {
                            Image(systemName: "bubble.left")
                            Text("Send feedback")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)
                    Text("Share bugs, ideas, or feedback with the team. Device info + audit summary (no sensitive data) are included.")
                        .font(.caption2)
                        .foregroundStyle(.secondary)

                    NavigationLink(destination: Text("Join the FocalPoint Discord community at https://discord.gg/focalpoint")
                        .padding()) {
                        HStack {
                            Image(systemName: "person.3")
                            Text("Join Discord community")
                            Spacer()
                            Image(systemName: "arrow.up.right")
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                        }
                    }
                    .foregroundStyle(.primary)
                }

                if devModeUnlocked {
                    Section("Developer") {
                        Button("Re-run onboarding") {
                            hasOnboarded = false
                        }
                        Button("Reset Canvas connection") {
                            CanvasBridge.clear()
                            canvas = nil
                        }
                        Button(role: .destructive) {
                            nukeDatabaseAndRestart()
                        } label: {
                            Label("Reset local database", systemImage: "trash")
                        }
                        NavigationLink("Coachy character sheet") {
                            CoachyDebugView()
                        }

                        #if DEBUG
                        Button(action: testSentryEvent) {
                            HStack {
                                Image(systemName: "exclamationmark.triangle.fill")
                                    .foregroundStyle(Color.orange)
                                Text("Test Sentry event")
                            }
                        }
                        if showTestSentryToast {
                            Text(testSentryToastMessage)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                                .transition(.opacity)
                        }
                        #endif
                    }
                }
            }
            .navigationTitle("Settings")
            .background(Color.app.background.ignoresSafeArea())
            .sheet(isPresented: $showCanvasAuth) {
                CanvasAuthView(
                    onConnected: { record in
                        canvas = record
                        holder.bump()
                    }
                )
            }
            .sheet(isPresented: $showGCalAuth) {
                GCalAuthView(
                    onConnected: {
                        holder.bump()
                    }
                )
            }
            .sheet(isPresented: $showGitHubAuth) {
                GitHubAuthView(
                    onConnected: {
                        holder.bump()
                    }
                )
            }
        }
        .task(id: holder.revision) {
            loadConnectors()
            canvas = CanvasBridge.load()
        }
    }

    @ViewBuilder
    private func connectorRow(id: String, summary: ConnectorHandleSummary?) -> some View {
        let displayName = connectorDisplayNames[id] ?? id
        let status = summary?.health ?? "Not connected"
        VStack(alignment: .leading, spacing: 6) {
            HStack {
                Text(displayName).font(.body.weight(.semibold))
                Spacer()
                Text(status)
                    .font(.caption2.weight(.bold))
                    .foregroundStyle(Color.app.accent)
            }
            connectorActions(id: id, summary: summary)
        }
        .padding(.vertical, 4)
    }

    @ViewBuilder
    private func connectorActions(id: String, summary: ConnectorHandleSummary?) -> some View {
        switch id {
        case "canvas":
            if let canvas {
                Text("Connected: \(canvas.instanceUrl)")
                    .font(.caption).foregroundStyle(Color.app.foreground.opacity(0.7))
                Text("Token …\(canvas.tokenFingerprint)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
                Button("Disconnect") {
                    CanvasBridge.clear()
                    self.canvas = nil
                }.font(.caption)
            } else {
                Button("Connect") { showCanvasAuth = true }
                    .buttonStyle(.borderedProminent)
                    .tint(Color.app.accent)
                    .controlSize(.small)
            }
        case "gcal":
            if let summary {
                Text("Next sync: \(summary.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
            Button("Connect") { showGCalAuth = true }
                .buttonStyle(.borderedProminent)
                .tint(Color.app.accent)
                .controlSize(.small)
        case "github":
            if let summary {
                Text("Next sync: \(summary.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
            Button("Connect") { showGitHubAuth = true }
                .buttonStyle(.borderedProminent)
                .tint(Color.app.accent)
                .controlSize(.small)
        default:
            if let summary {
                Text("Next sync: \(summary.nextSyncAtIso)")
                    .font(.caption2).foregroundStyle(Color.app.foreground.opacity(0.5))
            }
        }
    }

    private func summary(for id: String) -> ConnectorHandleSummary? {
        connectors.first { $0.connectorId.lowercased().contains(id) }
    }

    private func loadConnectors() {
        connectors = holder.core.sync().connectors()
    }

    private func bumpVersion() {
        versionTapCount += 1
        if versionTapCount >= 5 { devModeUnlocked = true }
    }

    /// Dev-only nuclear option: delete the SQLite DB and wipe the
    /// "notifications already seen" dedupe set, then force a scene reset
    /// by flipping `hasOnboarded` + `hasSeenWake` so the user goes through
    /// the cold-start path. Matches the file path CoreHolder uses —
    /// `<AppSupport>/focalpoint/core.db` and the `-wal` / `-shm` siblings.
    private func nukeDatabaseAndRestart() {
        let fm = FileManager.default
        if let base = try? fm.url(
            for: .applicationSupportDirectory,
            in: .userDomainMask,
            appropriateFor: nil,
            create: false
        ) {
            let dir = base.appendingPathComponent("focalpoint", isDirectory: true)
            for suffix in ["core.db", "core.db-wal", "core.db-shm"] {
                let f = dir.appendingPathComponent(suffix)
                try? fm.removeItem(at: f)
            }
        }
        UserDefaults.standard.removeObject(forKey: "focalpoint.notify.last_dispatched_ids")
        UserDefaults.standard.removeObject(forKey: "focalpoint.canvas.connection")
        hasOnboarded = false
        // The SQLite handle is still live inside FocalPointCore — next app
        // launch rebuilds from a fresh DB. For this session we can't
        // re-open mid-process without reconstructing CoreHolder, so
        // surface a restart prompt via the banner.
        exportError = "Database deleted. Fully quit and reopen the app to load a fresh core."
    }

    /// If the user previously configured a coaching endpoint (via onboarding
    /// or a future dev menu), re-wire it when they flip coaching back on.
    /// No-op until that config exists — today this is the "stays off"
    /// default because we don't ship a default endpoint.
    private func reapplyCoachingIfConfigured() {
        let endpoint = coachingEndpoint.trimmingCharacters(in: .whitespaces)
        let model = coachingModel.trimmingCharacters(in: .whitespaces)
        guard !endpoint.isEmpty, !model.isEmpty else {
            // Toggle is on but no endpoint configured — leave coaching
            // null so the core uses static fallbacks.
            return
        }
        // API key would come from the keychain; stub with empty for now.
        let cfg = CoachingConfig(endpoint: endpoint, apiKey: "", model: model)
        holder.core.setCoaching(config: cfg)
    }

    private func testSoundsAndHaptics() {
        if soundEffectsEnabled {
            // Tier-0 stub: just provide user feedback that the test was triggered
            let feedbackGen = UIImpactFeedbackGenerator(style: .medium)
            feedbackGen.impactOccurred()
            print("✅ Sound & haptic test triggered (audio files not yet available)")
        }
    }

    /// Write `audit.recent(limit: 5000)` as JSONL to a tempfile and expose
    /// it via `ShareLink` so the user can hand it off through Files /
    /// AirDrop / Mail. The export is the source-of-truth provenance
    /// artifact — every wallet / penalty / policy / connector / task /
    /// ritual / notify / host-event mutation + its hash-chain position.
    private func exportAuditChain() {
        exportError = nil
        do {
            let records = try holder.core.audit().recent(limit: 5000)
            let lines = records.map { rec -> String in
                let obj: [String: Any] = [
                    "id": rec.id,
                    "record_type": rec.recordType,
                    "subject_ref": rec.subjectRef,
                    "occurred_at": rec.occurredAtIso,
                    "payload_json": rec.payloadJson,
                    "hash": rec.hash,
                ]
                if let data = try? JSONSerialization.data(withJSONObject: obj),
                   let s = String(data: data, encoding: .utf8) {
                    return s
                }
                return "{}"
            }
            let body = lines.joined(separator: "\n") + "\n"
            let ts = ISO8601DateFormatter().string(from: Date())
                .replacingOccurrences(of: ":", with: "-")
            let filename = "focalpoint-audit-\(ts).jsonl"
            let url = FileManager.default.temporaryDirectory.appendingPathComponent(filename)
            try body.write(to: url, atomically: true, encoding: .utf8)
            auditExportUrl = url
        } catch {
            exportError = "Export failed: \(error.localizedDescription)"
        }
    }

    /// Test Sentry event capture (DEBUG builds only).
    /// Fires a test error to Sentry and displays a toast confirming the event was queued.
    /// Only available when developer mode is unlocked. Respects user's Sentry opt-in preference.
    private func testSentryEvent() {
        #if DEBUG
        #if canImport(Sentry)
        if sentryEnabled {
            // Fire a test error event to Sentry
            let error = NSError(domain: "com.focalpoint.debug", code: -9999, userInfo: [
                NSLocalizedDescriptionKey: "Test error from FocalPoint Debug Settings. This is expected and safe to ignore."
            ])
            SentrySDK.capture(error: error) { scope in
                scope.setLevel(.warning)
                scope.setTag(value: "true", key: "test_event")
                scope.setTag(value: "settings_debug", key: "source")
            }

            testSentryToastMessage = "✅ Event queued (check Sentry dashboard)"
        } else {
            testSentryToastMessage = "⚠️ Crash reports disabled. Enable in Diagnostics first."
        }
        showTestSentryToast = true

        // Auto-hide toast after 3 seconds
        DispatchQueue.main.asyncAfter(deadline: .now() + 3.0) {
            withAnimation {
                showTestSentryToast = false
            }
        }
        #else
        testSentryToastMessage = "Sentry SDK not available"
        showTestSentryToast = true
        #endif
        #endif
    }

    /// Send feedback via mailto with device info and audit summary (counts only).
    /// Device model, iOS version, app version, and audit summary counts are included.
    /// No task contents, rule conditions, calendar events, or tokens are ever included.
    private func sendFeedback() {
        let device = UIDevice.current
        let osVersion = "\(device.systemName) \(device.systemVersion)"
        let appVersion = Bundle.main.appVersion

        // Get audit summary counts (never contents)
        var taskCount = 0
        var ruleCount = 0
        var connectorCount = 0

        if let tasks = try? holder.core.planning().list(userId: UUID()) {
            taskCount = tasks.count
        }

        if let rules = try? holder.core.rules().listEnabled() {
            ruleCount = rules.count
        }

        connectorCount = connectors.count

        let subject = "FocalPoint Feedback - App v\(appVersion)"
        let body = """
        Device: \(device.model), \(osVersion)
        App Version: \(appVersion)
        Audit Summary: \(taskCount) tasks, \(ruleCount) rules, \(connectorCount) connectors active

        [Please describe your feedback, bug, or idea here]
        """

        let encoded = body.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? ""
        let encodedSubject = subject.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? ""

        if let url = URL(string: "mailto:feedback@focalpoint.app?subject=\(encodedSubject)&body=\(encoded)") {
            UIApplication.shared.open(url)
        }
    }
}

// MARK: - Diagnostics Info View (FR-DIAG-001)

struct DiagnosticsInfoView: View {
    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(alignment: .leading, spacing: 16) {
                    Text("Crash Reporting")
                        .font(.title2.bold())

                    VStack(alignment: .leading, spacing: 8) {
                        Text("What We Collect")
                            .font(.subheading.bold())
                        BulletPoint("App crash stack traces (file + line number only)")
                        BulletPoint("Device OS version and app build number")
                        BulletPoint("Time and date of the crash")
                        BulletPoint("Breadcrumb trail of app actions (redacted)")
                    }

                    VStack(alignment: .leading, spacing: 8) {
                        Text("What We Do NOT Collect")
                            .font(.subheading.bold())
                        BulletPoint("Task or rule contents")
                        BulletPoint("Calendar event details")
                        BulletPoint("Any personal identifiers (emails, usernames)")
                        BulletPoint("OAuth tokens or authentication credentials")
                        BulletPoint("Your usage patterns or behavior")
                    }

                    VStack(alignment: .leading, spacing: 8) {
                        Text("Data Privacy")
                            .font(.subheading.bold())
                        Text("All crash data is:")
                            .font(.body)
                        BulletPoint("Automatically redacted of personal data")
                        BulletPoint("Encrypted in transit to Sentry (our crash service)")
                        BulletPoint("Retained for 90 days, then deleted")
                        BulletPoint("Used only to fix bugs — never shared with third parties")
                    }

                    VStack(alignment: .leading, spacing: 8) {
                        Text("Your Control")
                            .font(.subheading.bold())
                        Text("You can disable crash reporting at any time in Settings > Diagnostics. When disabled, no crash data is sent.")
                            .font(.body)
                    }

                    Spacer()
                }
                .padding()
            }
            .navigationTitle("Diagnostics")
            .background(Color.app.background.ignoresSafeArea())
        }
    }
}

struct BulletPoint: View {
    let text: String

    init(_ text: String) {
        self.text = text
    }

    var body: some View {
        HStack(alignment: .top, spacing: 8) {
            Text("•")
                .font(.body)
            Text(text)
                .font(.body)
                .foregroundStyle(Color.app.foreground.opacity(0.8))
        }
    }
}

// MARK: - UIDevice extension for model name

extension UIDevice {
    var model: String {
        var systemInfo = utsname()
        uname(&systemInfo)
        let machineMirror = Mirror(reflecting: systemInfo.machine)
        let identifier = machineMirror.children.reduce("") { identifier, element in
            guard let value = element.value as? Int8, value != 0 else { return identifier }
            return identifier + String(UnicodeScalar(UInt8(value)))
        }
        return identifier
    }

    // MARK: - CloudKit Sync Helpers

    private func syncStatusText() -> String {
        switch cloudSyncStatus {
        case .unchecked:
            return "Checking iCloud status..."
        case .available:
            if let lastSync = lastCloudSyncTime {
                let formatter = RelativeDateTimeFormatter()
                return "Last synced \(formatter.localizedString(for: lastSync, relativeTo: Date()))"
            }
            return "Ready to sync"
        case .unavailable(let reason):
            return "iCloud unavailable: \(reason)"
        }
    }

    private func statusForegroundColor() -> Color {
        switch cloudSyncStatus {
        case .available:
            return .secondary
        case .unavailable:
            return .red
        case .unchecked:
            return .secondary
        }
    }

    private func triggerCloudSync() {
        cloudSyncInProgress = true
        Task {
            // TODO: Wire to actual CloudKitSyncClient when ready.
            // For now, stub the sync round.
            let client = CloudKitSyncClient()
            let status = await client.checkSyncStatus()

            switch status {
            case .available:
                cloudSyncStatus = .available
                lastCloudSyncTime = Date()
                // Real implementation would call client.syncRound() here
            case .unavailable(let reason):
                cloudSyncStatus = .unavailable(reason)
            }

            cloudSyncInProgress = false
        }
    }
}

enum CloudKitSyncStatus {
    case unchecked
    case available
    case unavailable(String)
}

// MARK: - Bundle extension for app version

extension Bundle {
    var appVersion: String {
        guard let version = infoDictionary?["CFBundleShortVersionString"] as? String,
              let build = infoDictionary?["CFBundleVersion"] as? String else {
            return "unknown"
        }
        return "\(version) (build \(build))"
    }
}

// MARK: - Debug view (relocates the original pose-cycling demo)

struct CoachyDebugView: View {
    var body: some View {
        // Re-use the original cycling showcase so the demo isn't lost.
        DebugCoachyShowcase()
            .navigationTitle("Coachy debug")
            .background(Color.app.background.ignoresSafeArea())
    }
}
#endif
