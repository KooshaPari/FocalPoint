# Ephemeral Demo Signing Keys

**SECURITY WARNING:** These keys are ephemeral demo keys **FOR DEVELOPMENT AND TESTING ONLY**. They are checked into git and shared publicly. Do **NOT** use for production template signing.

## Files

- `demo.priv` — Private ed25519 key (64 hex chars). Decoding yields 32-byte scalar.
- `demo.pub` — Public ed25519 key (64 hex chars). Decoding yields 32-byte point.

## Usage

1. **CLI**: `focus template sign examples/templates/deep-work-starter.toml --key-file examples/templates/_keys/demo.priv`
2. **Rust**: `focus_templates::signing::sign_pack(&pack, &signing_key)`
3. **Verification**: `focus_templates::parse_root_pubkey("demo.pub")` then `verify_pack(...)`

## Trust Model

- **Phenotype-signed packs** (production) are signed with the ops-managed key in `PHENOTYPE_ROOT_PUBKEYS`.
- **Demo packs** (development) are signed with this ephemeral key. The app trusts it explicitly for e2e testing only.
- **User packs** (community) can be signed with user-owned keys and verified on first-use via fingerprint display.

## Rotation

Replace `demo.priv` / `demo.pub` before shipping Phase 1 to production. Generate a permanent ops key via: `ed25519-dalek::SigningKey::generate(&mut rand_core::OsRng)` and store securely offline.
