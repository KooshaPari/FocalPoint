#if canImport(SwiftUI)
import SwiftUI

public enum AppTypography {
    public static let display = Font.system(size: 34, weight: .bold, design: .rounded)
    public static let title = Font.system(size: 22, weight: .semibold, design: .rounded)
    public static let body = Font.system(size: 16, weight: .regular, design: .default)
    public static let caption = Font.system(size: 12, weight: .medium, design: .default)
    public static let mono = Font.system(size: 14, weight: .regular, design: .monospaced)
}
#endif
