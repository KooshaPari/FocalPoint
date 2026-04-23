import SwiftUI

#if canImport(UIKit)
import UIKit

extension UIColor {
    /// Create a UIColor from a 6-digit hex string ("#RRGGBB" or "RRGGBB").
    public convenience init(hex: String) {
        var s = hex.trimmingCharacters(in: .whitespacesAndNewlines).uppercased()
        if s.hasPrefix("#") { s.removeFirst() }
        var rgb: UInt64 = 0
        Scanner(string: s).scanHexInt64(&rgb)
        let r = CGFloat((rgb & 0xFF0000) >> 16) / 255.0
        let g = CGFloat((rgb & 0x00FF00) >> 8) / 255.0
        let b = CGFloat(rgb & 0x0000FF) / 255.0
        self.init(red: r, green: g, blue: b, alpha: 1.0)
    }

    /// Build a dynamic UIColor that resolves to `dark` in dark mode and `light` otherwise.
    public static func dynamic(light: UIColor, dark: UIColor) -> UIColor {
        UIColor { trait in
            trait.userInterfaceStyle == .dark ? dark : light
        }
    }
}

private func dynColor(light: String, dark: String) -> Color {
    Color(UIColor.dynamic(light: UIColor(hex: light), dark: UIColor(hex: dark)))
}
private func solidColor(_ hex: String) -> Color {
    Color(UIColor(hex: hex))
}
#else
// macOS / non-UIKit fallback used only so `swift build` on host compiles.
// The real UI runs on iOS; this path keeps APIs shape-compatible.
private func hexToRGB(_ hex: String) -> (Double, Double, Double) {
    var s = hex.trimmingCharacters(in: .whitespacesAndNewlines).uppercased()
    if s.hasPrefix("#") { s.removeFirst() }
    var rgb: UInt64 = 0
    Scanner(string: s).scanHexInt64(&rgb)
    let r = Double((rgb & 0xFF0000) >> 16) / 255.0
    let g = Double((rgb & 0x00FF00) >> 8) / 255.0
    let b = Double(rgb & 0x0000FF) / 255.0
    return (r, g, b)
}
private func dynColor(light: String, dark: String) -> Color {
    // macOS fallback: pick the dark variant for visual parity with app default.
    let (r, g, b) = hexToRGB(dark)
    return Color(red: r, green: g, blue: b)
}
private func solidColor(_ hex: String) -> Color {
    let (r, g, b) = hexToRGB(hex)
    return Color(red: r, green: g, blue: b)
}
#endif

// MARK: - Semantic color groups

public enum AppColors {
    public static let background = dynColor(light: "#F6F5F5", dark: "#0F1012")
    public static let foreground = dynColor(light: "#0F1012", dark: "#F6F5F5")
    public static let surface = dynColor(light: "#E8E7E7", dark: "#353A40")
    public static let accent = solidColor("#7EBAB5")
    public static let accentOn = solidColor("#0F1012")
}

public enum CoachyColors {
    public static let flameCore = solidColor("#F07B3F")
    public static let flameEdge = solidColor("#F8B26A")
    public static let flameBase = solidColor("#E05A26")
    public static let cape = solidColor("#D4462E")
    public static let buckleGold = solidColor("#F9C86A")
    public static let eyes = solidColor("#121212")
}

// MARK: - Namespaced access: Color.app.* / Color.coachy.*

extension Color {
    public static var app: AppColors.Type { AppColors.self }
    public static var coachy: CoachyColors.Type { CoachyColors.self }
}
