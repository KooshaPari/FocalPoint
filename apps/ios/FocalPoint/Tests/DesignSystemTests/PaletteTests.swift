#if canImport(UIKit)
import XCTest
import UIKit
@testable import DesignSystem

final class PaletteTests: XCTestCase {
    func testHexParsesSixDigitColor() {
        let c = UIColor(hex: "#7EBAB5")
        var r: CGFloat = 0, g: CGFloat = 0, b: CGFloat = 0, a: CGFloat = 0
        c.getRed(&r, green: &g, blue: &b, alpha: &a)
        XCTAssertEqual(Int(round(r * 255)), 0x7E)
        XCTAssertEqual(Int(round(g * 255)), 0xBA)
        XCTAssertEqual(Int(round(b * 255)), 0xB5)
        XCTAssertEqual(a, 1.0, accuracy: 0.001)
    }

    func testDynamicColorResolvesLightAndDark() {
        let light = UIColor(hex: "#F6F5F5")
        let dark = UIColor(hex: "#0F1012")
        let dyn = UIColor.dynamic(light: light, dark: dark)

        let lightTrait = UITraitCollection(userInterfaceStyle: .light)
        let darkTrait = UITraitCollection(userInterfaceStyle: .dark)

        XCTAssertEqual(dyn.resolvedColor(with: lightTrait), light)
        XCTAssertEqual(dyn.resolvedColor(with: darkTrait), dark)
    }

    func testHexAcceptsLeadingHashOptional() {
        let a = UIColor(hex: "#F07B3F")
        let b = UIColor(hex: "F07B3F")
        var ar: CGFloat = 0, ag: CGFloat = 0, ab: CGFloat = 0, aa: CGFloat = 0
        var br: CGFloat = 0, bg: CGFloat = 0, bb: CGFloat = 0, ba: CGFloat = 0
        a.getRed(&ar, green: &ag, blue: &ab, alpha: &aa)
        b.getRed(&br, green: &bg, blue: &bb, alpha: &ba)
        XCTAssertEqual(ar, br, accuracy: 0.001)
        XCTAssertEqual(ag, bg, accuracy: 0.001)
        XCTAssertEqual(ab, bb, accuracy: 0.001)
    }
}
#endif
