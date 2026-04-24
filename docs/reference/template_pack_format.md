# Template Pack Format Specification

**Traces to:** FR-TEMPLATE-PACK-001, FR-TEMPLATE-SIGN-001

## Overview

A **template pack** is a distributable bundle of focus Rules, mascot copy overrides, and connector recommendations. Packs are authored in TOML, optionally signed with ed25519, and installed via the `focus template install` CLI command.

## Pack Structure

### On Disk

```
my-pack/
├── pack.toml              # Main pack definition (rules, connectors, copy)
├── manifest.toml          # Metadata + signature + digest (optional)
└── assets/
    └── (future: custom images, styles)
```

### Pack File (pack.toml)

A pack is a TOML file with the following required and optional fields:

```toml
# Required
id = "deep-work-starter"              # Stable identifier, no spaces
name = "Deep Work Starter Pack"       # Display name
version = "0.1.0"                     # Semantic version
author = "focalpoint-team"            # Author or team

# Optional
description = "Blocks social apps and notifications during deep work sessions"
recommended_connectors = ["gcal", "github", "slack"]

[mascot_copy]
session_start = "Let's get into the zone."
session_end = "Nice work!"

[[rules]]
id = "deep-work-social-block"
name = "Deep Work — Social Apps Blocked"
trigger = { kind = "event", value = "focus:session_started" }
conditions = []
actions = [
  { type = "block", profile = "social", duration_seconds = 3600, rigidity = "hard" },
]
priority = 80
cooldown_seconds = 600
duration_seconds = 3600
explanation_template = "Social media locked during {rule_name}."
enabled = true
```

### Manifest File (manifest.toml)

Metadata and integrity information accompanying a pack:

```toml
# Required
id = "deep-work-starter"
version = "0.1.0"
author = "focalpoint-team"
sha256 = "a1b2c3d4..." # SHA-256 of canonical pack bytes (hex)

# Optional: ed25519 signature if author signed the pack
signature = "base64-encoded-64-byte-signature"
signed_by = "1st-16-chars-of-pubkey-hex"  # Fingerprint for UI
```

## Serialization & Canonicalization

### Canonical Bytes

When computing a digest or signature, packs are serialized as **canonical JSON**:

1. Deserialize TOML pack into `TemplatePack` struct.
2. Serialize to `serde_json::Value`.
3. **Sort all object keys alphabetically** (BTreeMap order).
4. Re-serialize with `serde_json::to_vec()`.
5. Compute SHA-256 over these bytes.

This ensures any language (Rust, Swift, Kotlin, JavaScript) can reproduce the same bytes for verification, independent of TOML formatting choices or whitespace.

## Signature Algorithm

- **Algorithm:** ed25519 (IETF variant, RFC 8032)
- **Message:** Canonical JSON bytes (SHA-256 serialization above)
- **Encoding:** Raw 64-byte signature encoded as base64 (standard alphabet, no URL-safe variant)
- **Verification:** Signature is detached; pack and manifest are separate files

## Trust Model

### Root Keys

The app ships with a list of **compile-time trusted root keys** (currently empty; populated when FocalPoint ops generates the first signing keypair):

```rust
const PHENOTYPE_ROOT_PUBKEYS: &[&str] = &[
    // "hex-encoded-32-byte-ed25519-pubkey-1",
    // "hex-encoded-32-byte-ed25519-pubkey-2",
];
```

### Verification Policy

1. **Unsigned pack (`signature` is None):**
   - Default: allowed unless `--require-signature` is set on install.
   - Digest (SHA-256) is still verified.

2. **Signed pack (`signature` is Some):**
   - Signature MUST verify against at least one key in the trusted set.
   - If no trusted key verifies the signature, install fails.

3. **User-supplied keys:**
   - Users can register additional trusted keys in `~/.config/focalpoint/trusted-keys.toml`:
     ```toml
     keys = [
       "hex-encoded-public-key-1",
       "hex-encoded-public-key-2",
     ]
     ```
   - User keys are consulted alongside root keys during verification.

### Web-of-Trust (Future)

Future versions may support:
- Key delegation (root key signs a "sub-signer" certificate)
- Revocation lists (keys compromised; install blocks them)
- Key rotation ceremonies (documented in `docs/governance/signing_ceremony.md`)

## Format Stability

The pack TOML schema is **additive-only**:
- Unknown fields in the TOML are tolerated and ignored (via `#[serde(default)]`).
- Removing a field is a **breaking change** that bumps the pack format version number.
- Adding a field is **not** a breaking change; existing parsers continue to work.

## Tampering Detection

The SHA-256 digest catches accidental and intentional tampering:
- Bit flip in any field → digest mismatch → install fails.
- Signature verification catches replay attacks (replacing an old pack with a newer one using an old key).

## CLI Usage

### Install with signature verification

```bash
# Load pack and manifest, verify signature against trusted keys
focus template install /path/to/pack.toml --manifest /path/to/manifest.toml

# Require signature (fail if pack is unsigned)
focus template install pack-id --require-signature
```

### Install unsigned (development)

```bash
# Install unsigned pack (digest only)
focus template install /path/to/pack.toml
```

### List bundled packs

```bash
focus template list
```

## Implementation Reference

- **Signing:** `crates/focus-templates/src/signing.rs`
  - `sign_pack(pack: &TemplatePack, key: &SigningKey) -> Result<Signature>`
  - `verify_pack(pack: &TemplatePack, sig: &Signature, pubkey: &VerifyingKey) -> Result<()>`
  - `digest_pack(pack: &TemplatePack) -> Result<String>`

- **Pack validation:** `crates/focus-templates/src/lib.rs`
  - `TemplatePack::verify_and_apply(store, manifest, trusted_roots, require_signature)`

- **CLI:** `crates/focus-cli/src/main.rs`
  - `TemplatesCmd::Install { pack_id, manifest, require_signature }`
  - `load_trusted_keys()` → reads `~/.config/focalpoint/trusted-keys.toml`
