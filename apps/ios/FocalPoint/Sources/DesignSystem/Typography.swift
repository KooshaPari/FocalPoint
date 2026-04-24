#if canImport(SwiftUI)
import SwiftUI

public enum AppTypography {
    // MARK: - Core typography
    public static let display = Font.system(size: 34, weight: .bold, design: .rounded)
    public static let title = Font.system(size: 22, weight: .semibold, design: .rounded)
    public static let body = Font.system(size: 16, weight: .regular, design: .default)
    public static let caption = Font.system(size: 12, weight: .medium, design: .default)
    public static let mono = Font.system(size: 14, weight: .regular, design: .monospaced)

    // MARK: - Extended typography
    // Strong body (emphasis, card titles)
    public static let bodyStrong = Font.system(size: 16, weight: .semibold, design: .default)

    // Large counter / numeric display (48–56pt range)
    public static let counterLarge = Font.system(size: 56, weight: .bold, design: .rounded)
    public static let heroNumber = Font.system(size: 48, weight: .bold, design: .rounded)
    public static let statsHeader = Font.system(size: 40, weight: .bold, design: .rounded)

    // Timer display (monospaced variant for countdown)
    public static let timerLarge = Font.system(size: 56, weight: .bold, design: .rounded).monospacedDigit()

    // Icon sizing (not a font, but convention)
    public static let icon = Font.system(size: 28, weight: .semibold, design: .default)
    public static let heroIcon = Font.system(size: 56, weight: .semibold, design: .default)

    // MARK: - Coachy display sizes (reference, not Font)
    // Use with CoachyView(state: ..., size: N):
    // - hero = 220 (primary focus, full-attention mascot)
    // - medium = 120 (secondary focus, card content)
    // - small = 80 (inline, accent role)
    // - chip = 44 (badge, tiny preview)
}
#endif
