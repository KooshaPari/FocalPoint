#if canImport(SwiftUI)
import SwiftUI
import DesignSystem
import MascotUI

@main
struct FocalPointApp: App {
    var body: some Scene {
        WindowGroup {
            RootTabView()
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
            RulesView()
                .tabItem { Label("Rules", systemImage: "list.bullet.rectangle") }
            CoachyTabView()
                .tabItem { Label("Coachy", systemImage: "flame.fill") }
            SettingsView()
                .tabItem { Label("Settings", systemImage: "gearshape.fill") }
        }
    }
}
#endif
