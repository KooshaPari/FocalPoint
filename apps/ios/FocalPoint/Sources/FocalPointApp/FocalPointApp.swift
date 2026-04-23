#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI
import FocalPointCore

@main
struct FocalPointApp: App {
    @StateObject private var holder = CoreHolder.shared
    @AppStorage("app.hasOnboarded") private var hasOnboarded: Bool = false

    var body: some Scene {
        WindowGroup {
            Group {
                if hasOnboarded {
                    RootTabView()
                } else {
                    OnboardingView()
                }
            }
            .environmentObject(holder)
            .tint(Color.app.accent)
            .background(Color.app.background.ignoresSafeArea())
            .preferredColorScheme(.dark)
        }
    }
}

struct RootTabView: View {
    var body: some View {
        TabView {
            HomeView()
                .tabItem { Label("Home", systemImage: "house.fill") }
            RulesListView()
                .tabItem { Label("Rules", systemImage: "list.bullet.rectangle") }
            CoachyTabView()
                .tabItem { Label("Coachy", systemImage: "flame.fill") }
            SettingsView()
                .tabItem { Label("Settings", systemImage: "gearshape.fill") }
        }
    }
}
#endif
