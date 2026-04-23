#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

@main
struct FocalPointApp: App {
    @StateObject private var holder = CoreHolder.shared
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false
    @AppStorage("app.hasSeenWake") private var hasSeenWake: Bool = false
    @Namespace private var mascotNS

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
            }
        }
    }
}

struct RootTabView: View {
    var body: some View {
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
            CoachyTabView()
                .tabItem { Label("Coachy", systemImage: "flame.fill") }
            ActivityView()
                .tabItem { Label("Activity", systemImage: "list.bullet.rectangle.portrait") }
            SettingsView()
                .tabItem { Label("Settings", systemImage: "gearshape.fill") }
        }
    }
}
#endif
