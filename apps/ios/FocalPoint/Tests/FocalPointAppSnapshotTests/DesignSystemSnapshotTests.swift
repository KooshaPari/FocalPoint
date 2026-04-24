import SwiftUI
import SnapshotTesting
import XCTest
@testable import DesignSystem

// Traces to: FR-UI-001 (Design System color + typography verification)

class DesignSystemSnapshotTests: XCTestCase {
    // Set to true on first run to record baselines; false for comparison
    let record = false

    // MARK: - Palette Snapshot Tests

    func testPaletteColorsLight() {
        let view = PalettePreview()
            .preferredColorScheme(.light)

        assertViewSnapshot(
            view: view,
            name: "palette_colors_light",
            record: record
        )
    }

    func testPaletteColorsDark() {
        let view = PalettePreview()
            .preferredColorScheme(.dark)

        assertViewSnapshot(
            view: view,
            name: "palette_colors_dark",
            record: record
        )
    }

    // MARK: - Typography Snapshot Tests

    func testTypographyScale() {
        let view = TypographyPreview()

        assertViewSnapshot(
            view: view,
            name: "typography_scale",
            record: record
        )
    }

    // MARK: - Component Variants

    func testButtonVariants() {
        let view = ButtonVariantsPreview()

        assertViewSnapshot(
            view: view,
            name: "button_variants",
            record: record
        )
    }

    func testCardComponents() {
        let view = CardComponentPreview()

        assertViewSnapshot(
            view: view,
            name: "card_components",
            record: record
        )
    }
}

// MARK: - Preview Helpers

/// Preview showing all palette colors
struct PalettePreview: View {
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 16) {
                Text("Color Palette").font(.headline).padding(.top, 16)

                HStack(spacing: 8) {
                    Circle()
                        .fill(Color.blue)
                        .frame(width: 60, height: 60)
                    Text("Primary Blue")
                }

                HStack(spacing: 8) {
                    Circle()
                        .fill(Color.green)
                        .frame(width: 60, height: 60)
                    Text("Success Green")
                }

                HStack(spacing: 8) {
                    Circle()
                        .fill(Color.orange)
                        .frame(width: 60, height: 60)
                    Text("Warning Orange")
                }

                HStack(spacing: 8) {
                    Circle()
                        .fill(Color.red)
                        .frame(width: 60, height: 60)
                    Text("Danger Red")
                }

                HStack(spacing: 8) {
                    Circle()
                        .fill(Color.gray)
                        .frame(width: 60, height: 60)
                    Text("Neutral Gray")
                }
            }
            .padding(16)
        }
    }
}

/// Preview showing typography hierarchy
struct TypographyPreview: View {
    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 12) {
                Text("Typography Hierarchy").font(.system(.title, design: .default))
                Divider()

                Text("Large Title").font(.system(.largeTitle, design: .default))
                Text("Body: This is regular body text for content.").font(.body)
                Text("Headline").font(.headline)
                Text("Caption: Small supporting text").font(.caption)
                Text("Code: monospaced font").font(.system(.caption, design: .monospaced))
            }
            .padding(16)
        }
    }
}

/// Preview showing button variants
struct ButtonVariantsPreview: View {
    var body: some View {
        ScrollView {
            VStack(spacing: 12) {
                Text("Button Variants").font(.headline)

                Button(action: {}) {
                    Text("Primary Button")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.borderedProminent)

                Button(action: {}) {
                    Text("Secondary Button")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)

                Button(action: {}) {
                    Text("Plain Button")
                }
                .buttonStyle(.plain)

                Button(role: .destructive, action: {}) {
                    Text("Destructive Button")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)
            }
            .padding(16)
        }
    }
}

/// Preview showing card components
struct CardComponentPreview: View {
    var body: some View {
        ScrollView {
            VStack(spacing: 12) {
                Text("Card Components").font(.headline)

                VStack(alignment: .leading, spacing: 8) {
                    Text("Card Title").font(.headline)
                    Text("This is a card component with shadow and rounded corners.")
                        .font(.body)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)
                .shadow(radius: 2)

                VStack(alignment: .leading, spacing: 8) {
                    Text("Rule Card").font(.headline)
                    Text("Instagram: 2 hours/day").font(.system(.caption, design: .monospaced))
                    ProgressView(value: 0.75).tint(.blue)
                }
                .frame(maxWidth: .infinity, alignment: .leading)
                .padding(12)
                .background(Color(.systemBackground))
                .cornerRadius(8)
                .shadow(radius: 2)
            }
            .padding(16)
        }
    }
}
