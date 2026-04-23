# Third-Party Licenses

FocalPoint incorporates code derived from the following third-party projects.

## Foqos

- Upstream: https://github.com/awaseem/foqos
- License: MIT © 2024 Ali Waseem
- Usage in FocalPoint: donor codebase for iOS NFC unlock, QR scanner, Screen
  Time integration patterns (`Foqos/Utils/NFCScannerUtil.swift`,
  `NFCWriter.swift`, `PhysicalReader.swift`, `QRCodeScanner.swift`,
  `AppBlockerUtil.swift`, `FamilyActivityUtil.swift`, `StrategyManager.swift`,
  `DeviceActivityCenterUtil.swift`, session aggregators, date/timer utilities).
- Adaptation: files carried into FocalPoint retain an attribution header:
  ```swift
  // Derived from Foqos (https://github.com/awaseem/foqos),
  // MIT © 2024 Ali Waseem.
  ```
- NOT carried: Apple entitlement files, bundle identifiers, provisioning
  profiles, Xcode project files, Live Activity widget bundle IDs.
- Private mirror: `KooshaPari/foqos-private` (fetch upstream + internal fork).

### Foqos MIT License text

```
MIT License

Copyright (c) 2024 Ali Waseem

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

## Reef (deferred)

Pending Android revival. Upstream: https://github.com/aload0/Reef. MIT via
README (no standalone LICENSE file; trademark-style reservation on name + icon).
If/when forked, rebrand required + proper LICENSE file upstream PR recommended.

## FocalPoint's own license

FocalPoint is dual-licensed MIT OR Apache-2.0. See `LICENSE`.
