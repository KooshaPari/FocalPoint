# Rustdoc Warning Backlog

## Summary

After the rustdoc pass on 2026-04-24, the remaining warnings are structural (unused variables, unused function parameters) rather than documentation issues. Total: 21 warnings across 5 crates.

## Breakdown by Crate

### focus-backup (14 warnings)
- **12 lib warnings**: unused variables, unused function parameters in fns that are stubs or future-facing
  - `encrypt_with_passphrase` — `_plaintext` unused
  - `decrypt_with_passphrase` — `_ciphertext` unused
  - Several other encoding/decoding stubs
- **2 doc warnings**: `Vec<u8>` HTML tag (low priority)

**Priority**: Low. Stubs are intentionally minimal. Can be fixed in FR-DATA-003 (backup impl).

### connector-github (2 warnings)
- Unused variables in test helpers
- **Priority**: Low. Non-blocking.

### focus-rituals (2 warnings)
- Unused variables in cadence computation helpers
- **Priority**: Low. Non-blocking.

### connector-gcal (1 warning)
- Unused `mut` on URL builder
- **Priority**: Trivial. Can be fixed in next touch.

### focus-backup (doc) (2 warnings)
- Already counted above under lib warnings

## Completed (0 warnings)

The following crates now have clean doc builds:
- ✅ focus-domain
- ✅ focus-ir
- ✅ focus-rules
- ✅ focus-events
- ✅ focus-policy
- ✅ focus-wallet
- ✅ focus-storage
- ✅ focus-sync
- ✅ focus-connectors
- ✅ focus-crypto
- ✅ focus-ffi
- ✅ focus-mcp-server
- ✅ connector-canvas

## Warnings by Type

| Type | Count | Examples |
|------|-------|----------|
| Unused variables | 16 | `plaintext`, `ciphertext`, loop vars |
| Unused params | 4 | `_` prefixing needed |
| Doc HTML tags | 2 | backtick-escaping |
| **Total** | **21** | |

## Action Items

- **Phase 1 (This Pass)**: Doc-link issues only → Done ✅
- **Phase 2 (Next Pass)**: Unused param warnings
  - Add `_` prefix or `#[allow(dead_code)]` to intentionally-stubbed functions
  - Target: reduce to <5 warnings
- **Phase 3 (On-going)**: Structural cleanup as stubs are implemented
  - Each stub → real impl eliminates warnings naturally

## Notes

- No `#![deny(missing_docs)]` re-enabled yet; will add after Phase 2
- Focus on FR implementations (backup, encryption, etc.) to naturally eliminate stubs
- Backlog is not blocking; workspace builds cleanly with warnings
