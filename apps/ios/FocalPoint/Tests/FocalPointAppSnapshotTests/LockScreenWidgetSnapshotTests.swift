import SwiftUI
import WidgetKit
import SnapshotTesting
@testable import FocalPointWidget

final class LockScreenWidgetSnapshotTests: XCTestCase {

    func testAccessoryCircularWidget_withStreak() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 850,
            streakDays: 7,
            coachyPoseName: "celebrating"
        )
        let entry = AccessoryCircularWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryCircularWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryCircular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenCircular),
            named: "accessory_circular_with_streak"
        )
    }

    func testAccessoryCircularWidget_zeroStreak() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 100,
            streakDays: 0,
            coachyPoseName: "neutral"
        )
        let entry = AccessoryCircularWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryCircularWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryCircular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenCircular),
            named: "accessory_circular_zero_streak"
        )
    }

    func testAccessoryCircularWidget_darkMode() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 500,
            streakDays: 14,
            coachyPoseName: "celebrating"
        )
        let entry = AccessoryCircularWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryCircularWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryCircular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenCircularDark),
            named: "accessory_circular_dark_mode"
        )
    }

    func testAccessoryRectangularWidget_standard() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 850,
            streakDays: 7,
            coachyPoseName: "neutral"
        )
        let entry = AccessoryRectangularWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryRectangularWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryRectangular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenRectangular),
            named: "accessory_rectangular_standard"
        )
    }

    func testAccessoryRectangularWidget_highCredits() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 2500,
            streakDays: 30,
            coachyPoseName: "celebrating"
        )
        let entry = AccessoryRectangularWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryRectangularWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryRectangular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenRectangular),
            named: "accessory_rectangular_high_credits"
        )
    }

    func testAccessoryRectangularWidget_darkMode() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 500,
            streakDays: 10,
            coachyPoseName: "neutral"
        )
        let entry = AccessoryRectangularWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryRectangularWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryRectangular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenRectangularDark),
            named: "accessory_rectangular_dark_mode"
        )
    }

    func testAccessoryInlineWidget_standard() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 850,
            streakDays: 7,
            coachyPoseName: "neutral"
        )
        let entry = AccessoryInlineWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryInlineWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryInline)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenInline),
            named: "accessory_inline_standard"
        )
    }

    func testAccessoryInlineWidget_largeNumbers() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 9999,
            streakDays: 99,
            coachyPoseName: "celebrating"
        )
        let entry = AccessoryInlineWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryInlineWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryInline)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenInline),
            named: "accessory_inline_large_numbers"
        )
    }

    func testAccessoryInlineWidget_darkMode() {
        let snapshot = WidgetSnapshot(
            creditsBalance: 500,
            streakDays: 10,
            coachyPoseName: "neutral"
        )
        let entry = AccessoryInlineWidgetEntry(date: Date(), snapshot: snapshot)
        let view = AccessoryInlineWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryInline)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenInlineDark),
            named: "accessory_inline_dark_mode"
        )
    }

    func testCreditsWidget_accessoryCircular() {
        let snapshot = WidgetSnapshot(creditsBalance: 750, streakDays: 5)
        let entry = CreditsWidgetEntry(date: Date(), snapshot: snapshot)
        let view = CreditsWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryCircular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenCircular),
            named: "credits_widget_accessory_circular"
        )
    }

    func testCreditsWidget_accessoryRectangular() {
        let snapshot = WidgetSnapshot(creditsBalance: 750, streakDays: 5)
        let entry = CreditsWidgetEntry(date: Date(), snapshot: snapshot)
        let view = CreditsWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryRectangular)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenRectangular),
            named: "credits_widget_accessory_rectangular"
        )
    }

    func testCreditsWidget_accessoryInline() {
        let snapshot = WidgetSnapshot(creditsBalance: 750, streakDays: 5)
        let entry = CreditsWidgetEntry(date: Date(), snapshot: snapshot)
        let view = CreditsWidgetEntryView(entry: entry)
            .environment(\.widgetFamily, .accessoryInline)

        assertSnapshot(
            of: view,
            as: .image(on: .lockScreenInline),
            named: "credits_widget_accessory_inline"
        )
    }
}

// MARK: - Lock Screen ViewImageConfig Extensions

extension ViewImageConfig {
    static let lockScreenCircular = ViewImageConfig(
        size: .init(width: 180, height: 180),
        traits: .init(userInterfaceStyle: .light)
    )

    static let lockScreenCircularDark = ViewImageConfig(
        size: .init(width: 180, height: 180),
        traits: .init(userInterfaceStyle: .dark)
    )

    static let lockScreenRectangular = ViewImageConfig(
        size: .init(width: 320, height: 100),
        traits: .init(userInterfaceStyle: .light)
    )

    static let lockScreenRectangularDark = ViewImageConfig(
        size: .init(width: 320, height: 100),
        traits: .init(userInterfaceStyle: .dark)
    )

    static let lockScreenInline = ViewImageConfig(
        size: .init(width: 320, height: 40),
        traits: .init(userInterfaceStyle: .light)
    )

    static let lockScreenInlineDark = ViewImageConfig(
        size: .init(width: 320, height: 40),
        traits: .init(userInterfaceStyle: .dark)
    )
}
