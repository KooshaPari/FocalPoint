#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

@main
struct FocalPointApp: App {
    @StateObject private var holder = CoreHolder.shared
    @StateObject private var flyInPresenter = RuleFiredFlyInPresenter.shared
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false
    @AppStorage("app.hasSeenWake") private var hasSeenWake: Bool = false
    @AppStorage("app.sentryEnabled") private var sentryEnabled: Bool = false
    @AppStorage("app.flyInsEnabled") private var flyInsEnabled: Bool = true
    @Namespace private var mascotNS
    @Environment(\.scenePhase) private var scenePhase

    init() {
        // Register BGTaskScheduler handler during app init (required
        // before didFinishLaunching returns). Schedules the first refresh
        // when we hit background.
        #if canImport(BackgroundTasks)
        BackgroundSync.register()
        #endif

        // Initialize Sentry if user has opted in (default off per Apple guidelines).
        // FR-DIAG-001: User must explicitly enable crash reporting in Settings.
        SentrySetup.shared.setupIfConsented(userOptedIn: sentryEnabled)
    }

    var body: some Scene {
        WindowGroup {
            Group {
                if !hasSeenWake {
                    // First cold launch: Coachy sleep → wake sequence. Plays
                    // once; thereafter `hasSeenWake` short-circuits this path.
                    LaunchCoachyView(
                        onFinish: { hasSeenWake = true },
                        namespace: mascotNS
                    )
                } else if hasOnboarded {
                    RootTabView()
                } else {
                    OnboardingView()
                }
            }
            .environmentObject(holder)
            .tint(Color.app.accent)
            .background(Color.app.background.ignoresSafeArea())
            .preferredColorScheme(.dark)
            .task {
                // FR-CAL-001: wire EventKit → Rust CalendarPort on first scene.
                await holder.attachEventKitCalendar()
                // Foreground heartbeat — ticks the sync orchestrator every
                // 60s while the scene is active. Registered connectors
                // (Canvas, GCal, GitHub) pull events on their own cadence
                // behind this heartbeat.
                holder.startForegroundSync(interval: 60)
                // Notification & rule-fired fly-in heartbeat
                while true {
                    try? await Task.sleep(for: .seconds(5))
                    NotificationDispatcher.shared.tick(core: holder.core, flyInsEnabled: flyInsEnabled)
                }
            }
            .onChange(of: scenePhase) { _, phase in
                #if canImport(BackgroundTasks)
                if phase == .background {
                    // User just left the app — schedule a BGAppRefreshTask
                    // so rules keep firing. iOS controls actual cadence.
                    BackgroundSync.schedule(earliestMinutes: 15)
                }
                #endif
            }
        }
    }
}

struct RootTabView: View {
    @ObservedObject var flyInPresenter = RuleFiredFlyInPresenter.shared

    var body: some View {
        ZStack {
            TabView {
                RitualsView()
                    .tabItem { Label("Today", systemImage: "sun.max.fill") }
                FocusModeView()
                    .tabItem { Label("Focus", systemImage: "timer") }
                TasksView()
                    .tabItem { Label("Tasks", systemImage: "checklist") }
                HomeView()
                    .tabItem { Label("Home", systemImage: "house.fill") }
                RulesListView()
                    .tabItem { Label("Rules", systemImage: "list.bullet.rectangle") }
                WalletView()
                    .tabItem { Label("Rewards", systemImage: "diamond.fill") }
                StatsView()
                    .tabItem { Label("Stats", systemImage: "chart.bar.fill") }
                CoachyTabView()
                    .tabItem { Label("Coachy", systemImage: "flame.fill") }
                ActivityView()
                    .tabItem { Label("Activity", systemImage: "list.bullet.rectangle.portrait") }
                SettingsView()
                    .tabItem { Label("Settings", systemImage: "gearshape.fill") }
            }

            // Rule-fired fly-in overlay (top priority)
            RuleFiredFlyInView(presenter: flyInPresenter)
                .zIndex(999)
        }
    }
}
#endif
