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
    @AppStorage("app.onboardingV2") private var onboardingV2: Bool = true
    @State private var showResumeOnboarding = false
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

        // Set up notification delegate for handling user responses to notification actions.
        // This registers the 4 notification categories (COACHY_NUDGE, RITUAL_REMINDER,
        // RULE_FIRED, BACKUP_COMPLETE) and enables foreground notification presentation.
        #if canImport(UserNotifications)
        UNUserNotificationCenter.current().delegate = NotificationPermissionManager.shared
        #endif
    }

    var body: some Scene {
        WindowGroup {
            ZStack {
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
                    } else if onboardingV2 {
                        OnboardingViewV2()
                    } else {
                        OnboardingView()
                    }
                }
                .environmentObject(holder)
                .tint(Color.app.accent)
                .background(Color.app.background.ignoresSafeArea())
                .preferredColorScheme(.dark)
                .withKeyboardShortcuts()
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

                // Resume onboarding overlay: shown when user activates app
                // mid-onboarding (not completed, but has partial progress)
                if showResumeOnboarding {
                    ResumeOnboardingView(
                        isPresented: $showResumeOnboarding,
                        onResume: {
                            // Dismiss resume view; user returns to OnboardingViewV2
                            // at their saved step (coordinated via NavigationStack or
                            // explicit step-jumping in OnboardingCoordinator)
                        },
                        onRestart: {
                            OnboardingResumeState.resetTracking()
                            // OnboardingViewV2 resets to step .welcome
                        }
                    )
                    .transition(.move(edge: .bottom).combined(with: .opacity))
                    .zIndex(1)
                }
            }
            .onAppear {
                // Check if onboarding is incomplete with partial progress
                let isIncomplete = !hasOnboarded
                let hasPartialProgress = OnboardingResumeState.hasPartialProgress()

                if isIncomplete && hasPartialProgress && !showResumeOnboarding {
                    DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                        withAnimation(.easeInOut(duration: 0.4)) {
                            showResumeOnboarding = true
                        }
                    }

                    // Schedule 24h comeback notification if user declines resume
                    scheduleOnboarding24hNotification()
                }
            }
        }

        // Menu commands for macOS (Designed for iPad) and iPadOS with external keyboard.
        FocalPointMenuCommands()
    }

    // MARK: - 24h Comeback Notification

    private func scheduleOnboarding24hNotification() {
        #if canImport(UserNotifications)
        let content = UNMutableNotificationContent()
        content.title = "Coachy is waiting!"
        content.body = "Your focus setup is almost done. Come finish setup 🔥"
        content.sound = .default
        content.badge = NSNumber(value: 1)
        content.userInfo = ["notificationType": "onboarding_24h_reminder"]

        // Request notification at 24h from now
        let trigger = UNTimeIntervalNotificationTrigger(timeInterval: 86400, repeats: false)
        let request = UNNotificationRequest(
            identifier: "com.focalpoint.onboarding-24h",
            content: content,
            trigger: trigger
        )

        UNUserNotificationCenter.current().add(request) { error in
            if let error = error {
                print("Error scheduling 24h onboarding notification: \(error)")
            }
        }
        #endif
    }
}

struct RootTabView: View {
    @ObservedObject var flyInPresenter = RuleFiredFlyInPresenter.shared

    var body: some View {
        ZStack {
            VStack(spacing: 0) {
                // Nudge banner (always-on engine proactive suggestions)
                NudgeBannerView()

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
            }

            // Rule-fired fly-in overlay (top priority)
            RuleFiredFlyInView(presenter: flyInPresenter)
                .zIndex(999)
        }
    }
}
#endif
