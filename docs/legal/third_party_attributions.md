# Third-Party Attributions

## Foqos (MIT License)

**Source:** https://github.com/awaseem/foqos  
**Commit:** `5cb17ffcf4463c722d96a103737655f7ae07d01f` (v1.32.1, April 20, 2026)  
**Access Date:** April 23, 2026  
**License:** MIT

### Grafted Code

FocalPoint adopts and adapts the following code from Foqos under the MIT license:

- **`FoqosDeviceMonitor/DeviceActivityMonitorExtension.swift`** — DeviceActivity monitor lifecycle scaffold showing how to hook into Apple's `intervalDidStart` and `intervalDidEnd` callbacks for device activity enforcement.
- **`FoqosDeviceMonitor/FoqosDeviceMonitor.entitlements`** — Reference entitlements configuration showing required `com.apple.developer.family-controls` key declaration.

### Attribution

All grafted files include the SPDX header:
```
SPDX-License-Identifier: MIT
Copyright (c) 2025 awaseem and Foqos contributors
Adapted for FocalPoint
Source: https://github.com/awaseem/foqos
```

### Rationale

Foqos's 755-commit history of FamilyControls integration provides battle-tested boilerplate for:
- Correct DeviceActivityMonitor extension lifecycle (when to start/stop monitoring)
- ManagedSettingsStore namespace isolation for shield configuration
- Safe timeout handling when the app is killed (DeviceActivityCenter schedules teardown)

FocalPoint replaces Foqos's SwiftData persistence layer with a Rust core + SQLite append-only audit chain, improving auditability and portability.

### Full MIT License Text

```
MIT License

Copyright (c) 2025 awaseem and Foqos contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## See Also

- `apps/ios/FocalPoint/donor/Foqos/README.md` — Detailed graft integration guide
- `apps/ios/FocalPoint/Sources/Enforcement/Enforcement.swift` — FocalPoint's FamilyControls driver (inspired by Foqos patterns, reimplemented for our architecture)
