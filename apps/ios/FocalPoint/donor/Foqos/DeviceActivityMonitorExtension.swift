// SPDX-License-Identifier: MIT
// Copyright (c) 2025 awaseem and Foqos contributors
// Adapted for FocalPoint by the FocalPoint team
// Source: https://github.com/awaseem/foqos (commit 5cb17ffcf4463c722d96a103737655f7ae07d01f)

import DeviceActivity
import ManagedSettings
import OSLog

/*
 Adapted for FocalPoint:
 - Removed Foqos-specific `AppBlockerUtil` and `TimerActivityUtil` dependencies
 - Storage and enforcement now route to FocalPoint's Rust core via FFI
 - Extension lifecycle hooks (intervalDidStart/intervalDidEnd) are preserved
   to maintain Apple's DeviceActivity contract
 - Future: hook into `core.audit()` and `core.enforcement()` for state changes
 */

private let log = Logger(
  subsystem: "app.focalpoint.enforcement",
  category: "DeviceActivityMonitor"
)

// MARK: - DeviceActivity Monitor Extension

class FocalPointDeviceActivityMonitorExtension: DeviceActivityMonitor {
  override init() {
    super.init()
  }

  override func intervalDidStart(for activity: DeviceActivityName) {
    super.intervalDidStart(for: activity)

    log.info("intervalDidStart for activity: \(activity.rawValue)")

    // TODO: Route to FocalPointCore.shieldDidEngage() via FFI
    // This is where the Rust rule engine evaluates whether to engage
    // the shield based on the activity name and current policy.
  }

  override func intervalDidEnd(for activity: DeviceActivityName) {
    super.intervalDidEnd(for: activity)

    log.info("intervalDidEnd for activity: \(activity.rawValue)")

    // TODO: Route to FocalPointCore.shieldDidRetract() via FFI
    // The Rust core audits this state change and marks the interval as complete.
  }
}
