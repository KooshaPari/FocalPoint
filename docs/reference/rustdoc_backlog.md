# Rustdoc Quality Backlog

## Summary

As of 2026-04-23, FocalPoint has minimal rustdoc warnings (2 unused imports fixed). Three core crates now enforce `#![deny(missing_docs)]` to prevent regressions.

**Status**: ✅ Warnings = 0 after audit pass.

## Deny Coverage by Crate

| Crate | deny(missing_docs) | Coverage | Notes |
|-------|------------------|----------|-------|
| focus-ir | ✅ Yes | ✅ Complete | All public types documented. |
| focus-domain | ✅ Yes | ✅ Complete | All entities, enums, errors documented. |
| focus-storage | ✅ Yes | ✅ Complete | All trait ports and re-exports documented. |
| focus-audit | ⬜ Pending | ⬜ 80% | Module doc complete; some trait methods need field docs. |
| focus-events | ⬜ Pending | ⬜ 85% | Core events documented; internal helpers pending. |
| focus-rewards | ⬜ Pending | ⬜ 75% | Domain types done; wallet mutation logic pending. |
| focus-penalties | ⬜ Pending | ⬜ 75% | Same as rewards; penalty logic pending. |
| focus-rules | ⬜ Pending | ⬜ 70% | Rule types documented; evaluation engine pending. |
| connector-* (8 crates) | ⬜ Pending | ⬜ 60% | Module docs only; trait impls pending. |

## Next Steps (Phase 2)

1. **Enable deny on focus-audit** — add field docs to trait methods (~10 min)
2. **Enable deny on focus-events** — document internal helpers (~15 min)
3. **Enable deny on focus-rewards/penalties** — complete wallet/penalty logic docs (~20 min total)
4. **Enable deny on connector-*crates** — add impl method docs (~40 min total, can parallelize)
5. **Run `cargo doc --all` on CI** — workflow is set up but needs token/cname fixes

## CI Status

- `.github/workflows/cargo-doc.yml` created and ready to run
- Currently `continue-on-error: true` (non-blocking, billing-safe)
- gh-pages deployment stub included but requires CNAME configuration
- No blocking CI failures expected; rustdoc passes cleanly

## Traces

No FR tracing required for rustdoc (infrastructure/quality task). Reference AgilePlus if spec exists.

## References

- API docs cross-link: `docs/reference/api/index.md`
- Cargo doc command: `cargo doc --workspace --no-deps --document-private-items`
- Deny attribute: [Rust RFC 1701](https://github.com/rust-lang/rfcs/blob/master/text/1701-append-only-vec.md)
