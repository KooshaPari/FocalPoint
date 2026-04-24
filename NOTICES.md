# Third-Party Notices

FocalPoint incorporates the following third-party software, used under the terms of their respective licenses.

**Full license text for each dependency is available in `docs/legal/license_audit_2026_04.md`.**

## Rust Dependencies (Abbreviated List)

| Crate | License | Link |
|-------|---------|------|
| `tokio` | MIT | https://github.com/tokio-rs/tokio |
| `serde` | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| `reqwest` | MIT OR Apache-2.0 | https://github.com/seanmonstar/reqwest |
| `ring` | ISC | https://github.com/briansmith/ring |
| `sha2` | MIT OR Apache-2.0 | https://github.com/RustCrypto/hashes |
| `uniffi` | MPL-2.0 | https://github.com/mozilla/uniffi-rs |
| `chrono` | MIT OR Apache-2.0 | https://github.com/chronotope/chrono |
| `clap` | MIT OR Apache-2.0 | https://github.com/clap-rs/clap |
| ... and 20 others (full list in `docs/legal/license_audit_2026_04.md`) | ... | ... |

## Swift Dependencies

| Library | License | Link |
|---------|---------|------|
| `sentry-cocoa` | MIT | https://github.com/getsentry/sentry-cocoa |
| `swift-snapshot-testing` | MIT | https://github.com/pointfreeco/swift-snapshot-testing |
| Apple Frameworks (ActivityKit, FamilyControls, etc.) | Apple SDK | https://developer.apple.com |

## Derived Code

### Foqos

- **License:** MIT © 2024 Ali Waseem
- **Source:** https://github.com/awaseem/foqos
- **Usage in FocalPoint:** iOS NFC unlock, QR scanner, Screen Time integration patterns.
- **Attribution:** Retained in source file headers.
- **Text:**

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

### Reef (Deferred)

- **License:** MIT
- **Source:** https://github.com/aload0/Reef
- **Status:** Pending Phase 2 (Android revival).
- **Note:** If forked, formal LICENSE file will be added upstream.

---

## FocalPoint License

FocalPoint is dual-licensed under **MIT OR Apache-2.0**. You may use FocalPoint under the terms of either license at your option.

**MIT License:** Simple, permissive, no warranty.  
**Apache-2.0 License:** Includes explicit patent protection.

See `LICENSE` in the repository root for full text.

---

## Compatibility

All dependencies listed above are compatible with FocalPoint's dual MIT OR Apache-2.0 license. No GPL-family or copyleft licenses are present in the dependency tree.

For detailed analysis, see `docs/legal/license_audit_2026_04.md`.
