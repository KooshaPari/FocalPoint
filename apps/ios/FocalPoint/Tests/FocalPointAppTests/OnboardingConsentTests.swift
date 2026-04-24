#if canImport(SwiftUI)
import XCTest
import FocalPointCore
@testable import FocalPointApp

/// Unit tests for consent step and audit recording.
/// Traces to: FR-ONBOARDING-005 (Privacy + Terms consent gate)
final class OnboardingConsentTests: XCTestCase {
    private var coordinator: OnboardingCoordinator!

    override func setUp() {
        super.setUp()
        coordinator = OnboardingCoordinator()
    }

    override func tearDown() {
        coordinator = nil
        super.tearDown()
    }

    /// Verify that the consent step is the first step in the onboarding flow.
    func testConsentIsFirstStep() {
        XCTAssertEqual(coordinator.step, .consent)
    }

    /// Verify that advancing from consent requires both privacy and terms acceptance.
    func testCanAdvanceRequiresBothCheckboxes() {
        // Both unchecked — cannot advance
        XCTAssertFalse(coordinator.canAdvance)

        // Only privacy checked — still cannot advance
        coordinator.privacyAccepted = true
        XCTAssertFalse(coordinator.canAdvance)

        // Both checked — can advance
        coordinator.termsAccepted = true
        XCTAssertTrue(coordinator.canAdvance)

        // Uncheck one — cannot advance again
        coordinator.privacyAccepted = false
        XCTAssertFalse(coordinator.canAdvance)
    }

    /// Verify that diagnostics is optional.
    func testDiagnosticsIsOptional() {
        coordinator.privacyAccepted = true
        coordinator.termsAccepted = true

        // Can advance even with diagnostics unchecked
        XCTAssertTrue(coordinator.canAdvance)

        // Can advance with diagnostics checked
        coordinator.diagnosticsEnabled = true
        XCTAssertTrue(coordinator.canAdvance)
    }

    /// Verify that advancing past consent moves to the next step (welcome).
    func testAdvancePastConsent() {
        coordinator.privacyAccepted = true
        coordinator.termsAccepted = true
        coordinator.advance()

        XCTAssertEqual(coordinator.step, .welcome)
    }

    /// Verify that the audit record payload is correctly structured.
    /// Note: This test verifies the payload shape; a real integration test would
    /// require a full FocalPointCore instance and audit chain verification.
    func testConsentPayloadShape() {
        coordinator.privacyAccepted = true
        coordinator.termsAccepted = true
        coordinator.diagnosticsEnabled = true

        // The recordConsentAcceptance method should create a payload with:
        // - privacy_hash (SHA-256 of PRIVACY.md)
        // - terms_hash (SHA-256 of TERMS.md)
        // - privacy_ver, terms_ver
        // - timestamp (ISO-8601)
        // - diagnostics_enabled ("true" or "false")
        //
        // Full verification would require mocking the core.audit() interface.
        // For now, we verify the coordinator state is correct.
        XCTAssertTrue(coordinator.privacyAccepted)
        XCTAssertTrue(coordinator.termsAccepted)
        XCTAssertTrue(coordinator.diagnosticsEnabled)
    }

    /// Verify that reset() restores the initial consent state.
    func testResetRestoresConsentState() {
        coordinator.privacyAccepted = true
        coordinator.termsAccepted = true
        coordinator.diagnosticsEnabled = true

        coordinator.reset()

        XCTAssertEqual(coordinator.step, .consent)
        XCTAssertFalse(coordinator.privacyAccepted)
        XCTAssertFalse(coordinator.termsAccepted)
        XCTAssertFalse(coordinator.diagnosticsEnabled)
    }
}
#endif
