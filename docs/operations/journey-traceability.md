# Journey Traceability

**Repo:** FocalPoint  
**Traceability manifest:** `docs/traceability/fr-nfr-traceability.json`  
**Journey manifests:** `docs/journeys/manifests/`

## Contract

Every MVP user-facing FR/NFR must map to:

1. Spec reference
2. Code reference
3. Test reference
4. Documentation reference
5. Gate command
6. Journey manifest when the user can observe the behavior

## Honest rich-media stubs

When capture is blocked, use blank media placeholders and mark the step as:

- `capture_status: "NEEDS_CAPTURE"`
- `blind_eval: "skip"`
- `media_stub_reason`: one sentence naming the blocker

Do not use text-heavy fake screenshots, gradient cards, or mock renders that allow a vision judge to pass by reading the placeholder. This follows the hwLedger `GUI_CAPTURE_PENDING.md` precedent.

## MVP gate policy

| Missing evidence | Default gate | Strict gate |
|---|---|---|
| Missing spec/code/test/doc path | FAIL | FAIL |
| Missing journey manifest for user-facing FR | FAIL | FAIL |
| `NEEDS_CAPTURE` honest blank stub | WARN | FAIL with `--no-skip-allowed` |
| Text-heavy fake media stub | FAIL | FAIL |

## Capture stubs

Expected rich media files live under `docs/journeys/media/`. Until real captures exist, stubs must be blank files or machine-readable manifest rows only; never create fake screenshots that imply completed capture.
