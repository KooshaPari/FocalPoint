# Submit Your Template Pack

## Overview

Template packs are curated collections of focus rules, connector recommendations, and mascot copy. The FocalPoint registry enables community authors to share packs with the broader ecosystem.

This guide walks you through:
1. Creating a pack locally
2. Signing with your personal ed25519 key
3. Opening a PR to `FocalPoint/examples/templates/`
4. Index registration and promotion to verified tier

---

## Step 1: Create Your Pack

A template pack is a single TOML file in `examples/templates/<your-pack-id>.toml`.

### Minimal Example

```toml
id = "my-morning-flow"
name = "My Morning Flow"
version = "0.1.0"
author = "jane-doe"
description = "A personal template for deep work before 10am."
recommended_connectors = ["gcal", "github"]

[mascot_copy]
session_start = "Rise and shine! Distractions are locked."
session_end = "Morning session complete. Great work."

[[rules]]
id = "morning-social-block"
name = "Morning – no social"
priority = 100
cooldown_seconds = 300
duration_seconds = 7200
explanation_template = "Social apps blocked until {rule_name} ends."
enabled = true
trigger = { kind = "schedule", value = "0 6 * * *" }
actions = [
  { type = "block", profile = "social", duration_seconds = 7200, rigidity = "hard" },
]

[[rules]]
id = "morning-email-batch"
name = "Email batching 10am only"
priority = 80
cooldown_seconds = 600
explanation_template = "Email check time: {rule_name}."
enabled = true
trigger = { kind = "schedule", value = "0 10 * * *" }
actions = [
  { type = "grant_credit", amount = 50 },
]
```

### Schema

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `id` | string | yes | Unique slug (lowercase, hyphens). Must be unique in the registry. |
| `name` | string | yes | Human-readable title. |
| `version` | string | yes | SemVer (e.g., `0.1.0`). |
| `author` | string | yes | Your GitHub handle or team name. |
| `description` | string | optional | 1-2 sentences. Appears in search results. |
| `recommended_connectors` | array | optional | Connector IDs to suggest enabling: `gcal`, `github`, `canvas`, `readwise`, `strava`, `fitbit`, `apple-health`. |
| `mascot_copy` | object | optional | Event ID → string message overrides. Keys: `session_start`, `session_end`, `rule_triggered`, `streak_lost`. |
| `rules` | array | optional | Up to 50 rules. See [Rule Format](#rule-format) below. |

### Rule Format

```toml
[[rules]]
id = "rule-id"  # Stable string id (template-scoped)
name = "Rule Name"  # Display name
trigger = { kind = "event\|schedule\|state_change", value = "..." }
conditions = [
  { kind = "weekday", params = { days = ["mon", "tue", "wed"] } },
]
actions = [
  { type = "block", profile = "social", duration_seconds = 3600, rigidity = "hard" },
  { type = "grant_credit", amount = 100 },
]
priority = 80  # Higher runs first
cooldown_seconds = 600  # Silence identical rule within N seconds
duration_seconds = 3600  # Rule action duration (for time-bound actions)
explanation_template = "Message template with {rule_name} variable"
enabled = true  # Default: enabled. Users can toggle.
```

See [Rule Schema Reference](../reference/rule_schema.md) for all trigger, condition, and action types.

---

## Step 2: Test Locally

Before submitting, verify the pack loads:

```bash
# Install the app from source
cargo build --release

# Start the app and import your pack via the UI:
# Settings → Packs → Import → Select your .toml file

# Or via CLI (once shipping):
focus template load examples/templates/my-morning-flow.toml
```

---

## Step 3: Generate Your Signing Key

**One-time per author.**

```bash
# Download FocalPoint source
git clone https://github.com/KooshaPari/FocalPoint.git
cd FocalPoint

# Generate your ed25519 key (cryptographically secure)
python3 << 'EOF'
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.primitives import serialization
from pathlib import Path

# Generate keypair
key = ed25519.Ed25519PrivateKey.generate()
pub = key.public_key()

# Save as hex strings (standard ed25519 encoding)
priv_bytes = key.private_bytes(
    encoding=serialization.Encoding.Raw,
    format=serialization.PrivateFormat.Raw,
    encryption_algorithm=serialization.NoEncryption()
)
pub_bytes = pub.public_bytes(
    encoding=serialization.Encoding.Raw,
    format=serialization.PublicFormat.Raw
)

Path("~/.focalpoint-sign-key.priv").expanduser().write_text(
    priv_bytes.hex()
)
Path("~/.focalpoint-sign-key.pub").expanduser().write_text(
    pub_bytes.hex()
)
print(f"✓ Keys saved to ~/.focalpoint-sign-key.*")
print(f"  Public key: {pub_bytes.hex()}")
EOF

# **Securely back up** `~/.focalpoint-sign-key.priv` 
# (encrypted USB, password manager, etc.)
```

---

## Step 4: Sign Your Pack

```bash
# Sign with your personal key
python3 << 'EOF'
import json
import hashlib
import base64
from pathlib import Path
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.primitives import serialization
import tomllib

# Load your private key
priv_hex = Path("~/.focalpoint-sign-key.priv").expanduser().read_text().strip()
priv_bytes = bytes.fromhex(priv_hex)
signing_key = ed25519.Ed25519PrivateKey.from_private_bytes(priv_bytes)

pack_path = Path("examples/templates/my-morning-flow.toml")
pub_key = signing_key.public_key()
pub_bytes = pub_key.public_bytes(
    encoding=serialization.Encoding.Raw,
    format=serialization.PublicFormat.Raw
)

# Read and canonicalize pack
with open(pack_path, "r") as f:
    pack_dict = tomllib.loads(f.read())

canonical_json = json.dumps(
    pack_dict, sort_keys=True, separators=(',', ':'), ensure_ascii=True
)
canonical_bytes = canonical_json.encode('utf-8')

# Sign
signature = signing_key.sign(canonical_bytes)
sig_b64 = base64.b64encode(signature).decode('ascii')
sha256 = hashlib.sha256(canonical_bytes).hexdigest()
fingerprint = pub_bytes.hex()[:16]

print(json.dumps({
    "id": pack_dict["id"],
    "sha256": sha256,
    "signature": sig_b64,
    "signed_by": fingerprint,
    "author_public_key": pub_bytes.hex()
}, indent=2))
EOF

# Save output (use in next step)
```

---

## Step 5: Add to `index.json`

Update `examples/templates/index.json`:

```json
{
  "version": "1.0.0",
  "catalog": [
    ...existing packs...,
    {
      "id": "my-morning-flow",
      "title": "My Morning Flow",
      "author": "jane-doe",
      "version": "0.1.0",
      "description": "A personal template for deep work before 10am.",
      "sha256": "<from-signing-output>",
      "signature": "<from-signing-output>",
      "signed_by": "<fingerprint>",
      "rules_count": 2,
      "tags": ["morning", "focus", "personal"]
    }
  ]
}
```

---

## Step 6: Open a Pull Request

1. **Fork** `KooshaPari/FocalPoint`
2. **Create a branch:** `git checkout -b packs/my-morning-flow`
3. **Commit:**
   ```bash
   git add examples/templates/my-morning-flow.toml examples/templates/index.json
   git commit -m "feat(packs): add my-morning-flow template (author-signed)"
   ```
4. **Push:** `git push origin packs/my-morning-flow`
5. **PR:** Open PR from your branch to `main`

### PR Template

```markdown
## Adds Template Pack: My Morning Flow

**Author:** jane-doe  
**Signed By:** <your-public-key-fingerprint>  

### Pack Details
- **ID:** my-morning-flow
- **Rules:** 2 (social block, email batch)
- **Connectors:** gcal, github

### Trust Statement
- Signed with my personal ed25519 key
- Fingerprint: `c742e5e5fa536e56` (first 16 chars of public key)
- Public key (for verification): `c742e5e5fa536e56e7b38fee4b91caed9f172c09b3b9d33817c4c87ce9729d1e`

### Verification
Verify the pack:
```bash
curl -s https://raw.githubusercontent.com/KooshaPari/FocalPoint/packs/my-morning-flow/examples/templates/index.json \
  | jq '.catalog[] | select(.id == "my-morning-flow")'
```

### Intent
Sharing a personal morning-focus routine that has worked well for me. Suitable for early risers and deep-work enthusiasts.
```

---

## Trust Graph: Tiers

### Tier 1: Author-Signed (Community)

- **Signature:** Author's ed25519 public key
- **Verification:** Pull request verifies author identity (GitHub login)
- **UI Badge:** "Author-signed" with fingerprint
- **Risk:** No additional security beyond GitHub identity. Install at your own risk.
- **Flow:** 
  1. Author submits PR
  2. Humans review rules for safety
  3. Merge to `main` → registered in catalog
  4. App installs with author's public key visible in UI
  5. User trusts fingerprint (Telegram with author, etc.)

### Tier 2: Org-Verified (FocalPoint Team)

- **Signature:** FocalPoint team's root key in `PHENOTYPE_ROOT_PUBKEYS`
- **Verification:** Code review + security audit
- **UI Badge:** "FocalPoint Verified" (checkmark)
- **Risk:** Org has vetted rules and connector integrations
- **Examples:** All 7 starter packs (deep-work-starter, gym-routine, etc.)
- **Flow:** 
  1. Community submits pack (author-signed)
  2. FocalPoint review: code, rules, connectors, author rep
  3. Team signs pack with ops key → `Tier 2`
  4. Re-sign and merge to `main`
  5. App trusts via `PHENOTYPE_ROOT_PUBKEYS`

### Tier 3: Connector-Backed (Partnership)

- **Signature:** Connector's key (e.g., Canvas LMS, Readwise)
- **Verification:** Connector author verifies + maintains
- **UI Badge:** "Canvas Verified" (connector name)
- **Risk:** Connector vendor endorses the pack
- **Examples:** Student Canvas template (Canvas LMS partnership)
- **Flow:**
  1. Canvas team authors + signs pack with Canvas key
  2. Embedded in Canvas LMS export
  3. FocalPoint app verifies via Canvas's root key
  4. High trust: direct vendor relationship

---

## Tier Promotion & Revocation

### Promotion to Tier 2

A Tier 1 (author-signed) pack can be promoted if:

1. **Community adoption:** ≥50 installs over 2 weeks
2. **Safety:** No reported issues with rules or connectors
3. **Maintenance:** Author responds to issues/PRs
4. **Alignment:** Fits FocalPoint values (health, productivity, transparency)

**Process:**
- File an issue: "Promote `pack-id` to Tier 2"
- FocalPoint team reviews + audits
- If approved: team signs + merges
- App auto-upgrades (signature validation updated)

### Revocation

A signed pack is revoked if:

1. **Compromise:** Author's key is leaked
2. **Malicious:** Pack discovered to violate user consent (e.g., hidden tracking)
3. **Abandonment:** Author unresponsive; critical bugs unfixed for >3 months

**Process:**
- Issue filed with evidence
- Team votes → decision within 1 week
- Pack removed from `index.json`
- Installed instances: warning in Settings → Packs
- User must explicitly re-approve install

---

## FAQ

### Can I update my pack after submission?

Yes. PR a new version bump (e.g., `0.1.0` → `0.1.1`). Update `version` in TOML and re-sign. Existing installs notify users of update available.

### What if I lose my signing key?

- Revoke the old key via GitHub issue
- Generate a new key pair
- Resubmit packs with new key
- Old packs remain with old key (trust doesn't transfer)

### How do I know my pack won't be modified?

- Signature is **detached** (not in TOML)
- SHA-256 digest is commitment to exact pack bytes
- App verifies signature + digest before install
- Modification breaks signature → install rejected

### Can I include closed-source connectors?

No. All connectors must be open-source (same repo, linked public repo, or OSS upstream). Partnership-tier (Tier 3) requires connector vendor to co-sign.

### My pack has 100 rules — is that okay?

Recommended max: **50 rules**. Larger packs should be split or consolidated. Talk to us; we can help refactor.

---

## Support

- **Questions?** File an issue: [FocalPoint/issues](https://github.com/KooshaPari/FocalPoint/issues)
- **Security concerns?** Email: security@focalpoint.app (or GitHub security advisory)
- **Want to chat?** Join [FocalPoint Discord](https://discord.gg/focalpoint-community)

---

## Related

- [Rule Schema Reference](../reference/rule_schema.md)
- [Connector SDK](connector_sdk.md)
- [Template Pack Trust Graph](../governance/template_trust_graph.md)
