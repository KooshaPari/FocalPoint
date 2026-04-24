# Template Pack Signing Ceremony

**Status:** Planning phase. Ceremony will execute when first release-ready template pack ships.

## Objective

Establish root-of-trust ed25519 keys for signing template packs distributed to end users. Keys are held in offline custody with multi-signature checkpoints to prevent unauthorized or accidental signing.

## Key Ceremony Phases

### Phase 1: Key Generation (Offline)

**Participants:** At least 2 ops team members, physically isolated (no network).

1. Generate 3 ed25519 keypairs on separate offline devices (USB-booted Linux):
   ```bash
   # Device A
   openssl genpkey -algorithm ED25519 -out keyA.pem
   openssl pkey -in keyA.pem -pubout -out keyA.pub

   # Device B, C (same process)
   ```

2. Extract raw 32-byte public keys:
   ```bash
   openssl pkey -in keyA.pub -pubin -text -noout | grep -A 1 'pub key:' | tail -1 | tr -d ' :' | xxd -r -p | xxd -p -c 64
   ```

3. Verify consistency across 2 independent extractions per key. Store raw hex keys (64 hex chars each).

4. Securely store private keys in separate encrypted USB drives, one per team member.
   - Encryption: `gpg --armor --symmetric keyA.pem`
   - Passphrase: generate 25-char diceware phrase, store in offline vault (paper or HSM).

### Phase 2: Multi-Signature Enrollment

**Participants:** 3 ops team members (each holds one key).

1. Define signing policy: **2-of-3 multi-signature**.
   - Any 2 keys can sign a template pack.
   - No single key can sign unilaterally.

2. Each team member publishes their public key to:
   - `docs/keys/focalpoint-root-key-{A,B,C}.txt` (checked into git)
   - Git tags: `signing-key-{A,B,C}-2026-Q1` (anchors key rotation)

3. Publish the combined root-key list in code:
   ```rust
   // crates/focus-templates/src/signing.rs
   pub const PHENOTYPE_ROOT_PUBKEYS: &[&str] = &[
       "9f8e7d6c5b4a3f2e1d0c9b8a7f6e5d4c...",  // Key A
       "a1b2c3d4e5f6...",                       // Key B
       "7f6e5d4c3b2a...",                       // Key C
   ];
   ```

4. Announce keys publicly on:
   - FocalPoint GitHub (security-note issue)
   - Official docs

### Phase 3: Signing Operations

**When:** Whenever a template pack is released (e.g., "official-deep-work-v1.0").

**Participants:** 2 team members with offline keys (minimum).

**Process:**

1. Author or team prepares pack TOML on air-gapped machine.

2. **First signer:**
   - Retrieves private key from secure storage (decrypt with passphrase).
   - Computes canonical JSON bytes from pack.
   - Signs with ed25519:
     ```rust
     // Pseudocode (actual signing in CI or offline tool)
     let key = load_key("keyA.pem");
     let sig = key.sign(&canonical_bytes);
     let sig_b64 = base64::encode(&sig);
     ```
   - Publishes detached signature (64-byte ed25519 sig as base64).
   - Securely deletes temporary key material.

3. **Second signer:**
   - Verifies first signer's output (digest, signature format).
   - Retrieves own private key.
   - Signs the **same** pack bytes (canonical JSON must match).
   - Publishes second signature.

4. **Manifest assembly:**
   - Combine pack + manifest with both signatures:
     ```toml
     id = "official-deep-work"
     version = "1.0.0"
     author = "focalpoint-team"
     sha256 = "computed-from-canonical-json"
     signatures = [
       "first-signer-base64-sig",
       "second-signer-base64-sig",
     ]
     signed_by = ["key-A-fingerprint", "key-B-fingerprint"]
     ```
   - Note: Current implementation supports 1 signature; multi-sig is future enhancement.

5. **Distribution:**
   - Pack + manifest published to CDN or GitHub Releases.
   - Users install via `focus template install <url> --manifest <url>`.

### Phase 4: Key Rotation (Annual)

**Trigger:** Key compromise, team member departure, or annual ceremony (Jan 1).

1. Generate new keypairs (Phase 1 process).
2. Publish new root-key list.
3. Old keys are revoked: added to `docs/keys/revoked-keys.txt`.
4. Version bump in code triggers client re-trust (users re-import new keys).

## Key Storage & Access Control

| Component | Storage | Access | Backup |
|-----------|---------|--------|--------|
| Private keys | Encrypted USB, air-gapped | 2 team members, 1 per USB | Paper key escrow (HSM future) |
| Passphrases | Offline vault (paper) | 1 team member (custodian) | Tiered: 2-of-3 recovery |
| Public keys | Git (docs/keys/) | Everyone | Distributed with app code |
| Revocation list | Git (docs/keys/revoked.txt) | Ops team | Committed + tagged |

## Future Enhancements

1. **Hardware Security Module (HSM):**
   - Store private keys in Yubico or Ledger HSM.
   - Signing becomes: `hsm sign <msg>` (no key export).

2. **Multi-Signature in Manifest:**
   - Support 2-of-3 and M-of-N thresholds in manifest.
   - Verify that N unique signers signed.

3. **Delegation Certificates:**
   - Root keys sign "sub-signer" certs (ops engineer can sign without physical device).
   - Certs include expiry + scope (e.g., "sign packs only during Q1 2026").

4. **Revocation & Trust On First Use (TOFU):**
   - Pinned keys in `~/.config/focalpoint/trusted-keys.toml`.
   - TOFU for user-imported packs (accept + persist first key).

## Security Checklist

- [ ] Private keys never written to disk unencrypted.
- [ ] Signing environment is air-gapped (no network).
- [ ] Canonical JSON matches across 2+ signers (prevents tampering).
- [ ] Signatures are detached (pack remains plain TOML).
- [ ] Public keys are distributed with app code (no external fetch).
- [ ] Revocation list is committed to git (audit trail).
- [ ] At least 2 signatures required per pack (no unilateral signing).
- [ ] Key rotation documented in changelog + announced publicly.
