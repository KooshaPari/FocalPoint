# FocalPoint Security Threat Model

**Document version:** 1.0  
**Last updated:** 2026-04-23  
**Scope:** FocalPoint v0.0.3+ (end-to-end loop shipped)  
**Status:** Living threat model; annual review + issue-driven updates

---

## Executive Summary

FocalPoint is a connector-first screen-time management platform with a portable Rust core deployed to iOS via UniFFI. The threat model considers five asset classes (credentials, audit integrity, rules/tasks, wallet/penalty state, and code-like artifacts) across six attacker personas, using STRIDE analysis per asset and referencing mitigations already shipped.

**Key scope:** User secrets (OAuth tokens), audit-chain tamper-evidence, rule enforcement logic, wallet credits and penalties (gameable state), FamilyControls iOS enforcement, and template-pack distribution.

**Out of scope:** iOS kernel/bootloader bypasses, Apple device management MDM/DEP attacks, upstream dependency vulnerabilities (report upstream), and services under `services/*` (deferred to Phase 5).

---

## Asset Classes

### 1. **User Credentials (OAuth Tokens)**

- **Canvas LMS:** App-specific token for event sync (assignments, submissions, grades)
- **Google Calendar:** OAuth 2.0 token for event read + creation
- **GitHub:** Personal access token for repo event ingestion
- **Fitbit:** OAuth 2.0 token for step/heart-rate data
- **Readwise:** API token for reading highlights
- **Notion:** OAuth 2.0 integration token for block/page queries
- **Linear:** OAuth 2.0 for issue event stream
- **Strava:** OAuth 2.0 for activity metadata

**Storage:** iOS Keychain (via `focus-crypto::keychain` + `security-framework` crate). Never logged, never cached in plaintext in SQLite or UserDefaults.

**Lifetime:** Session-scoped (revoked on app wipe, account logout). Token refresh flows delegated to platform APIs where available (OAuth 2.0 refresh tokens also in Keychain).

---

### 2. **Audit Chain Integrity**

- **Location:** SQLite `audit_records` table (`focus-audit` crate, `focus-storage` adapter)
- **Structure:** Append-only chain of `AuditRecord` (event, actor, timestamp, changes, SHA-256 hash of previous record)
- **Tamper-Evidence:** Ed25519 signatures (planned, not yet shipped) + SHA-256 hash linkage
- **Lifecycle:** Immutable on disk; verified at app startup and after any resume from background

**Key records:**
- Reward/penalty mutations (wallet changes)
- Rule evaluation decisions + why (explainability snapshot)
- Policy/template pack installation
- OAuth credential rotation
- Device wipe + receipt generation

---

### 3. **Rules & Task Configuration**

- **Source:** User-authored DSL (TOML via `focus-rules`), bundled template packs, or visual builder output
- **Representation:** Intermediate Representation (IR) + compiled to Starlark (`focus-transpilers`)
- **Scope:** Rule predicates can only reference connector events (Canvas submissions, GitHub PRs, Fitbit steps, etc.); cannot access device files, make network calls, or escalate privileges
- **Storage:** SQLite `rules` table (plaintext); parsed and validated on load via `focus-rules::Parser`
- **Execution:** In-process Rust `focus-eval` evaluator; no JIT, no dynamic code loading

---

### 4. **Wallet & Penalty State**

- **Wallet (Rewards):** Numeric credit ledger per goal/ritual (e.g., 50 points for weekly exercise ritual)
- **Penalties:** Escalation tiers (yellow, orange, red) for rule violations; bypass budgets (daily, weekly resets)
- **Gameable Risk:** User can craft rules that grant unlimited points, or template packs that bypass penalties
- **Storage:** SQLite `wallet_entries` + `penalty_records` + linked audit records
- **Sync:** Local-first (SQLite source of truth); optional CloudKit backup (not yet shipped)

---

### 5. **Template Packs (Code-Like Artifacts)**

- **Content:** TOML bundle containing 1+ rules + metadata (author, version, description, icon, category)
- **Examples:** `student-canvas` (Canvas assignment tracking), `dev-flow` (GitHub PR response), `sleep-hygiene` (Fitbit step cutoff)
- **Risk:** A malicious pack author can distribute rules that:
  - Grant infinite wallet credits (reward rule explosion)
  - Craft exploitation-prone audit record structure (rely on unsigned chain for integrity)
  - Create penalizing rules that cannot be disabled by user (entitlement bypass)
- **Distribution:** iOS App Store (built-in bundles shipped with app) and optional future ecosystem (community packs; not yet shipped)
- **Signing:** Ed25519 signatures (crate `focus-templates`, feature flag `templates-signing`; planned, not yet in main)

---

### 6. **FamilyControls Profile & Device Enforcement**

- **Entitlement:** `com.apple.developer.family-controls` (Tier 1 blocker; not approved yet)
- **Purpose:** Enforce rule-triggered device restrictions (app/website blocking, screen time cutoffs)
- **Trust Model:** User is both manager and child on single device; FamilyControls API ensures user approval flow
- **Scope:** Rules can only *request* restrictions via iOS ManagedSettings; cannot force them without user interaction
- **Revocation:** User can disable FamilyControls at Settings > Family > Screen Time; app cannot prevent this

---

## Attacker Personas

### **Persona A: Curious Roommate (Physical Access)**

- **Capabilities:** Unlocked iPhone, ~10 minutes unsupervised access
- **Goals:** Exfiltrate Canvas grades, modify wallet state, install malicious template pack, spoof audit records
- **Threat Level:** High (direct device access beats most defenses)

**Risk factors:**
- Keychain secrets accessible if device is unlocked (iOS doesn't encrypt Keychain in-RAM after unlock)
- SQLite database accessible for direct reads + writes if attacker has local app process access
- App state in memory (wallet, rules) can be inspected via Xcode debugger or app introspection tools

---

### **Persona B: Malicious Template Pack Author**

- **Capabilities:** Craft TOML rule pack + distribute via App Store (community tab, future ecosystem)
- **Goals:** Steal reward credits from users, create un-bypassable penalties, forge audit records
- **Threat Level:** Medium (constrained by iOS sandbox + rule DSL limitations)

**Risk factors:**
- Rules are Turing-incomplete (no loops, recursion, or external calls); cannot execute arbitrary code
- But rules *can* reference all connector events + create complex reward conditions → potential reward inflation
- Template author has no insight into app internals; cannot directly forge signatures or bypass entitlements
- Signed templates (planned) will allow reputation + revocation; unsigned templates are trusted at install time only

---

### **Persona C: Network-Adjacent Attacker (Same WiFi)**

- **Capabilities:** Network sniffer (Wireshark, mitmproxy), can intercept HTTP traffic
- **Goals:** Eavesdrop on OAuth tokens, modify rule responses, forge connector events
- **Threat Level:** Medium (mitigated by TLS, but not pinned)

**Risk factors:**
- All connector APIs use HTTPS; TLS 1.2+ required by iOS 14+
- No certificate pinning implemented (open gap #1; see below)
- Attacker can perform a man-in-the-middle (MITM) attack if they control a proxy on the network
- OAuth token refresh may be intercepted if not validated against certificate
- Connector event payloads (canvas assignments, GitHub PR events) are plaintext JSON inside TLS

---

### **Persona D: Compromised Connector API (Canvas/GitHub/etc. Hacked)**

- **Capabilities:** Inject malicious events into the connector's event stream (fake Canvas assignments, crafted GitHub activity)
- **Goals:** Trigger false rule evaluations, forge audit entries, manipulate wallet state
- **Threat Level:** High (if Canvas is hacked, all Canvas-connected users are compromised)

**Risk factors:**
- Rule engine evaluates events at face value; no signature/cryptographic proof that events came from authentic API
- Malicious events can trigger rules with side effects (penalties, wallet drains)
- Audit chain will record the false event + rule decision, but chain is not signed yet (open gap #2)
- User has no way to detect or revoke a specific attacker's events

---

### **Persona E: Compromised Apple ID (iCloud Backup/Sync)**

- **Capabilities:** Remote access to user's iCloud Backups, CloudKit containers, and Keychain syncing
- **Goals:** Exfiltrate OAuth tokens, restore old wallet state, modify rules
- **Threat Level:** High (iCloud is the user's identity on iOS)

**Risk factors:**
- Keychain sync enabled by default on iOS; attacker with access to iCloud account can extract synced Keychain items
- SQLite backup stored in iCloud if user enabled device backup (not yet implemented in FocalPoint, but relevant for future)
- Attacker can restore a backup from before a penalty was applied, rewinding wallet state
- iCloud recovery flow requires user interaction (Apple's 2FA recovery codes), but if attacker has those, game over

---

### **Persona F: Malicious MCP Client (In-Process Socket)**

- **Capabilities:** Connect to FocalPoint's in-process MCP server over Unix-domain socket (iOS only; opt-in feature)
- **Goals:** Invoke tools (read rules, mutate wallet, trigger penalties, export audit), exfiltrate data
- **Threat Level:** Medium-High (bounded by MCP tool entitlements; see Elevation of Privilege below)

**Risk factors:**
- MCP server (crate `focus-mcp-server`) exposes ~27 tools + 2 resources over `mcp.sock` (in-process bridge)
- No authentication on the socket; any app or process with file-system access to the socket can connect
- Tool invocations are NOT gated by user confirmation (unlike FamilyControls); silent exfiltration possible
- Tools can read audit history, list rules, export wallet state, but cannot *modify* state (read-only guardrail)

---

## STRIDE Analysis per Asset

### **Asset 1: OAuth Tokens (Spoofing)**

**Threat:** Attacker forges a token or replaces it with one they control, impersonating the user to a connector API.

**Mitigations shipped:**
- Tokens stored in iOS Keychain, not plaintext storage
- Token refresh uses platform OAuth 2.0 flows (delegated to URLSession/URLSessionConfiguration)
- No token logging in code (code review via `cargo clippy` passes)

**Mitigations planned:**
- Certificate pinning for each connector domain (`focus-connectors` crate; open gap #1)
- Token binding (MTLS) if connector APIs support it

**Residual risk:** Network attacker can forge tokens if TLS is compromised (no pinning).

---

### **Asset 1: OAuth Tokens (Tampering)**

**Threat:** Attacker intercepts and modifies a token in transit, or modifies it in the Keychain.

**Mitigations shipped:**
- TLS 1.2+ encryption for all HTTPS traffic (iOS 14+ requirement)
- Keychain is encrypted at rest + requires biometric/passcode to decrypt (iOS security model)
- Constant-time HMAC verification for token payloads (if token format is JWT; e.g., GitHub, Linear)

**Residual risk:** Keychain compromised if device is unlocked and attacker has local root.

---

### **Asset 1: OAuth Tokens (Repudiation)**

**Threat:** User denies having authorized a token, or attacker claims user authorized malicious activity.

**Mitigations shipped:**
- Wipe receipt flow (backup crate `focus-backup`; commit `d14b704`) — app generates a signed receipt when wipe is initiated, containing timestamp + token revocation list
- Audit chain logs token rotation events with timestamp
- DCO (Developer Certificate of Origin) on all commits ensures code review trail

**Residual risk:** User cannot be compelled to provide wipe receipt; app may not survive a device crash before receipt is written.

---

### **Asset 1: OAuth Tokens (Information Disclosure)**

**Threat:** Tokens are logged, cached in plaintext, or synced to unencrypted backups.

**Mitigations shipped:**
- `secrecy` crate for in-memory token wrapping (zero-on-drop semantics)
- No token logging via Sentry or other telemetry (code review: grep `token` in `focus-connectors` returns only non-secret refs)
- Tokens not stored in SQLite or UserDefaults

**Residual risk:** Keychain items may be synced to iCloud if user enabled iCloud Keychain sync; attacker with Apple ID access can extract them.

---

### **Asset 1: OAuth Tokens (Denial of Service)**

**Threat:** Attacker floods connector APIs with requests using the user's token, causing rate-limit blocks.

**Mitigations shipped:**
- Each connector crate documents its API rate limits (e.g., Canvas API: 100 req/min per user)
- `focus-sync` crate respects `Retry-After` and backoff headers from APIs
- Exponential backoff: 1s, 2s, 4s, 8s on 429 (Too Many Requests)

**Residual risk:** App does not pre-emptively rate-limit user actions; if user manually triggers many syncs, rate-limiting is server-side only.

---

### **Asset 1: OAuth Tokens (Elevation of Privilege)**

**Threat:** Attacker uses a user's token to escalate to admin rights in the connector system (e.g., Canvas course admin, GitHub org admin).

**Mitigations shipped:**
- Tokens requested with minimal scopes (read-only for most: Canvas `url:GET`, GitHub `read:repo`, Fitbit `activity:read`)
- FocalPoint app has no admin UI; cannot use connector admin endpoints

**Residual risk:** If user grants the app more permissions than necessary, app will use them; scopes are documented in connector SDK but not enforced at runtime.

---

### **Asset 2: Audit Chain Integrity (Spoofing)**

**Threat:** Attacker creates fake audit records or spoofs the app as the actor.

**Mitigations shipped:**
- Audit records include actor (user, system, connector), action type, timestamp, and delta
- SQLite constraints prevent duplicate IDs

**Mitigations planned:**
- Ed25519 signatures on each audit record (crate `focus-audit`, feature `audit-signing`; WIP)
- Merkle tree of audit records for batch verification

**Residual risk:** Unsigned audit chain allows tampering; attacker with SQLite write access can inject records (gap #2).

---

### **Asset 2: Audit Chain Integrity (Tampering)**

**Threat:** Attacker modifies audit records in the SQLite database.

**Mitigations shipped:**
- SHA-256 hash linkage: each record includes `parent_hash` (hash of previous record)
- Validation on app startup: `focus-audit::verify_chain()` recomputes all hashes
- If chain is broken, app enters a degraded mode (logs error, continues but marks audit as "unverified")

**Mitigations planned:**
- Ed25519 signatures on each record (future; crate supports infrastructure but feature is gated)
- Time-lock commit finality: records older than N minutes cannot be modified (CRDT-like semantics)

**Residual risk:** Hash chain is only validated at startup; if app is killed mid-operation, attacker could modify records before next startup. Chain is not signed, so attacker with local SQLite access can forge new records + recompute hashes.

---

### **Asset 2: Audit Chain Integrity (Repudiation)**

**Threat:** User or attacker denies a record was ever written, or claims a record was written at a different time.

**Mitigations shipped:**
- Timestamp on every record (chrono::DateTime UTC)
- Wipe receipt (signed by app) includes a snapshot of the audit chain hash at wipe time
- Audit records are append-only; no deletion

**Residual risk:** Timestamps are local device time; if attacker has local root, they can change device time and forge timestamps.

---

### **Asset 2: Audit Chain Integrity (Information Disclosure)**

**Threat:** Audit records leak sensitive information (e.g., rule evaluation snapshots, penalty reasons).

**Mitigations shipped:**
- Audit records are stored locally; not synced or uploaded by default
- Sentry integration (error telemetry) has PII scrubbing rules (see PII checklist, not yet completed)

**Residual risk:** User can export audit via CLI (`focus audit export`); attacker with local access can read SQLite directly.

---

### **Asset 2: Audit Chain Integrity (Denial of Service)**

**Threat:** Attacker fills the audit table with millions of records, causing performance degradation or disk exhaustion.

**Mitigations shipped:**
- Audit record retention policy: keep last 90 days (configurable)
- SQLite auto-vacuum enabled
- Bulk verification (hash chain check) is O(n); app logs a warning if chain verification takes >5s

**Residual risk:** No per-user rate-limiting on audit writes; a malicious rule could trigger rule evaluations at high frequency, filling audit table.

---

### **Asset 2: Audit Chain Integrity (Elevation of Privilege)**

**Threat:** Attacker uses forged audit records to claim they have permission to do something (e.g., "I paid a penalty, so I can disable the rule").

**Mitigations shipped:**
- Audit records are read-only to users; cannot be used to grant permissions
- Rules are evaluated by the app, not by user consent from audit records

**Residual risk:** None identified; audit is a log, not an authorization mechanism.

---

### **Asset 3: Rules & Task Configuration (Spoofing)**

**Threat:** Attacker creates a rule that looks like it was written by the user (impersonation).

**Mitigations shipped:**
- Rules are stored in SQLite with a `created_by` field (user, import, template_pack_X)
- UI shows the source of each rule
- DCO on commits ensures code provenance

**Mitigations planned:**
- Signed template packs (feature flag `templates-signing`)
- User acknowledgment flow when importing a template (already implemented; commit `c78fb6a`)

**Residual risk:** If attacker has local SQLite write access, they can forge `created_by` field.

---

### **Asset 3: Rules & Task Configuration (Tampering)**

**Threat:** Attacker modifies a rule's DSL to change its behavior (e.g., disable a penalty rule, add a reward rule).

**Mitigations shipped:**
- Rule DSL is parsed and validated on load via `focus-rules::Parser`
- No hot-reloading; rules must be re-imported or re-edited through the UI
- Rules are stored in SQLite as TOML; parser rejects invalid syntax

**Mitigations planned:**
- Rule hash signature (part of audit chain signing feature)
- Version control for rule edits (edit history in audit)

**Residual risk:** Attacker with local SQLite write access can modify rule TOML; parser will accept the new rule.

---

### **Asset 3: Rules & Task Configuration (Repudiation)**

**Threat:** User claims a rule was never created by them; attacker claims a rule proves user intent.

**Mitigations shipped:**
- Audit records track rule creation with timestamp + source
- UI shows rule author + creation time
- Wipe receipt includes rule list snapshot

**Residual risk:** Timestamps are device-local; attacker with root can modify them.

---

### **Asset 3: Rules & Task Configuration (Information Disclosure)**

**Threat:** Rules leak connector data or user intent (e.g., rule that reads Canvas grades is visible in backups).

**Mitigations shipped:**
- Rules are not synced or uploaded by default
- iCloud backup opt-out recommended in docs (not enforced)

**Residual risk:** User can export rules via CLI; attacker with local access can read SQLite. iCloud backup may include rules if user enabled device backup (future feature).

---

### **Asset 3: Rules & Task Configuration (Denial of Service)**

**Threat:** Attacker imports a malicious rule that causes high CPU usage, memory leak, or infinite evaluation loop.

**Mitigations shipped:**
- Rule DSL is Turing-incomplete; no loops, recursion, or unbounded operations
- Starlark evaluation is sandboxed + timeout-gated (crate `focus-eval`, feature `eval-timeout`; 100ms default)
- Rule evaluation is not triggered by user input directly; only by connector events + periodic scheduler

**Mitigations planned:**
- Rule complexity budget (e.g., max 10 connectors per rule, max 5 conditions per predicate)

**Residual risk:** A rule with many conditions + many events per day can cause high evaluation frequency; no per-rule evaluation quota.

---

### **Asset 3: Rules & Task Configuration (Elevation of Privilege)**

**Threat:** Attacker creates a rule that escalates the app's capabilities (e.g., rule that calls system APIs, accesses files, or disables FamilyControls).

**Mitigations shipped:**
- Rule DSL has no built-in functions for system calls, file I/O, or FamilyControls API calls
- Rules can only reference connector event fields + time + wallet state
- Evaluation happens in sandboxed Starlark; no FFI calls

**Residual risk:** If a connector event payload includes a code-like string (e.g., Canvas submission text contains shell commands), the rule cannot execute it. Rules cannot perform any I/O beyond returning a decision (true/false).

---

### **Asset 4: Wallet & Penalty State (Spoofing)**

**Threat:** Attacker creates fake wallet entries or penalty records.

**Mitigations shipped:**
- Wallet entries are written by rule evaluations only (read from `focus-eval`)
- Each entry has an audit record; attacker would need to forge both

**Mitigations planned:**
- Wallet entry signatures (part of audit chain signing)

**Residual risk:** Attacker with local SQLite write access can insert fake entries.

---

### **Asset 4: Wallet & Penalty State (Tampering)**

**Threat:** Attacker modifies wallet balance or penalty tier.

**Mitigations shipped:**
- Wallet entries are append-only; no UPDATE on existing entries
- Penalties are stored as independent records with timestamps
- Audit chain logs all wallet mutations

**Mitigations planned:**
- Hash-linked wallet (similar to audit chain)
- Penalty record signatures

**Residual risk:** Attacker can insert new entries to change the running balance; append-only doesn't prevent insertion at arbitrary positions.

---

### **Asset 4: Wallet & Penalty State (Repudiation)**

**Threat:** User denies earning rewards, or claims they were penalized unfairly.

**Mitigations shipped:**
- Audit chain records why a reward/penalty was issued (rule ID, conditions met, connector event)
- Explainability snapshots in audit record show rule state at decision time
- Wipe receipt includes wallet balance at wipe time

**Residual risk:** Audit is not signed; attacker can forge the reason log.

---

### **Asset 4: Wallet & Penalty State (Information Disclosure)**

**Threat:** Wallet balance and penalty history are exposed to third parties or attacker.

**Mitigations shipped:**
- Wallet data is stored locally; not synced or uploaded by default
- MCP tool `read_wallet` requires read permission (tooling layer; no user-facing auth)

**Residual risk:** User can export wallet via CLI; attacker with SQLite read access can see balance + history.

---

### **Asset 4: Wallet & Penalty State (Denial of Service)**

**Threat:** Attacker repeatedly triggers high-penalty rules, draining the bypass budget or locking the user into a red-tier penalty.

**Mitigations shipped:**
- Penalty tiers are per-day or per-week (configurable)
- Bypass budgets reset daily/weekly
- User can manually override a penalty via Settings (requires user action)

**Residual risk:** No rate-limiting on how often a user can earn/lose points; a malicious rule could cause wallet inflation or penalties on every app launch.

---

### **Asset 4: Wallet & Penalty State (Elevation of Privilege)**

**Threat:** Attacker uses wallet state to claim they have permission to do something (e.g., "I have 100 points, so I can disable the app block rule").

**Mitigations shipped:**
- Wallet is a ledger, not a permission store
- FamilyControls enforcement is gated by user action (Settings > Family), not wallet balance

**Residual risk:** None identified; wallet is a reward mechanism, not a permission mechanism.

---

### **Asset 5: Template Packs (Spoofing)**

**Threat:** Attacker distributes a template pack claiming to be from a trusted author.

**Mitigations shipped:**
- Template metadata includes author name (string, not verified)
- Bundle is signed by app distributor (Apple's code signing on the IPA)

**Mitigations planned:**
- Ed25519 signatures on each template pack (feature flag `templates-signing`, crate `focus-templates`)
- Author identity verified via App Store submission process (future ecosystem; not yet implemented)
- Community template pack registry with author reputation

**Residual risk:** Built-in template packs are trusted because they ship with the app. Third-party packs (future) will be unsigned until ecosystem is live.

---

### **Asset 5: Template Packs (Tampering)**

**Threat:** Attacker modifies a template pack TOML after publication (e.g., changes a reward rule to grant unlimited points).

**Mitigations shipped:**
- Built-in packs are embedded in the IPA; immutable after app release
- App store ensures IPA is code-signed by Apple

**Mitigations planned:**
- Ed25519 signature verification on import (crate `focus-templates`, feature `templates-signing`)
- Signature validation before rules are parsed

**Residual risk:** Unsigned third-party packs can be modified by attacker or network attacker. Signature feature is not yet in main.

---

### **Asset 5: Template Packs (Repudiation)**

**Threat:** Pack author denies distributing a malicious pack; pack distributor claims they didn't modify it.

**Mitigations shipped:**
- App stores template metadata + author in audit record on import

**Mitigations planned:**
- Signed manifests with author identity
- Integrity signatures for ecosystem packs

**Residual risk:** Author identity is not verified at submission time; anyone can claim to be "Jane Researcher".

---

### **Asset 5: Template Packs (Information Disclosure)**

**Threat:** Template pack reveals sensitive information about the author or other users who used it.

**Mitigations shipped:**
- Template packs contain only rules (no user data) + metadata
- No telemetry on which packs users install

**Residual risk:** None identified.

---

### **Asset 5: Template Packs (Denial of Service)**

**Threat:** Attacker distributes a template pack with a malicious rule that causes high CPU, memory, or wallet inflation.

**Mitigations shipped:**
- Rule DSL is Turing-incomplete; no infinite loops
- Rule evaluation is sandboxed + timeout-gated (100ms default)
- Template import shows a preview of all rules before user confirms

**Residual risk:** A complex rule with many conditions can cause high evaluation frequency; no per-pack evaluation budget.

---

### **Asset 5: Template Packs (Elevation of Privilege)**

**Threat:** Template pack installs a rule that bypasses FamilyControls, accesses files, or executes code.

**Mitigations shipped:**
- Rule DSL has no system call capabilities
- Rules cannot access file system or make network calls
- FamilyControls is enforced by iOS, not by FocalPoint (user must approve restrictions)

**Residual risk:** None identified; rules are sandboxed.

---

### **Asset 6: FamilyControls Profile (Spoofing)**

**Threat:** Attacker creates a fake FamilyControls profile impersonating the user.

**Mitigations shipped:**
- FamilyControls API ensures only the app's own profile can be used
- iOS requires device unlock + biometric/passcode to create a profile

**Residual risk:** None identified; FamilyControls is enforced by iOS.

---

### **Asset 6: FamilyControls Profile (Tampering)**

**Threat:** Attacker modifies the FamilyControls profile to disable restrictions or add malicious ones.

**Mitigations shipped:**
- FamilyControls restrictions are written by the app's ManagedSettings API calls
- User can override restrictions in Settings > Family > Screen Time
- App cannot prevent user from disabling FamilyControls entirely

**Mitigations planned:**
- Wipe receipt includes FamilyControls restriction snapshot
- Audit record on every restriction change

**Residual risk:** If attacker has device access, they can modify ManagedSettings via Settings UI.

---

### **Asset 6: FamilyControls Profile (Repudiation)**

**Threat:** User claims they didn't enforce a restriction; attacker claims they did.

**Mitigations shipped:**
- Audit chain logs restriction changes with timestamp
- Wipe receipt includes snapshot of active restrictions

**Residual risk:** Device time can be modified by attacker with root access.

---

### **Asset 6: FamilyControls Profile (Information Disclosure)**

**Threat:** Restriction rules leak information about user's habits or penalties.

**Mitigations shipped:**
- Restrictions are stored in iOS's ManagedSettings database (encrypted at rest)
- User can export audit via CLI; attacker cannot directly access ManagedSettings without root

**Residual risk:** None identified; ManagedSettings is protected by iOS.

---

### **Asset 6: FamilyControls Profile (Denial of Service)**

**Threat:** Attacker triggers rules that set very strict restrictions (all apps blocked), preventing user from using device.

**Mitigations shipped:**
- FamilyControls requires user approval for each restriction in Settings
- User can disable FamilyControls entirely at any time (no app-side lock)
- App cannot prevent user from removing restrictions

**Residual risk:** User might be socially engineered into approving a harmful restriction.

---

### **Asset 6: FamilyControls Profile (Elevation of Privilege)**

**Threat:** Attacker uses FamilyControls API to escalate beyond app enforcement (e.g., modify system settings).

**Mitigations shipped:**
- ManagedSettings API can only set screen-time restrictions, not system settings
- iOS prevents ManagedSettings from modifying Passwords, Notifications, Privacy, or Parental Controls

**Residual risk:** None identified; ManagedSettings scope is limited by iOS.

---

## Known-Open Security Gaps

### **Gap #1: No TLS Certificate Pinning**

**Description:** Connector APIs (Canvas, GitHub, Fitbit, etc.) are accessed over HTTPS, but certificates are verified against system CAs only. A network attacker on the same WiFi can MITM the connection.

**Impact:** Attacker can forge OAuth tokens, inject malicious events, or extract connector data in plaintext.

**Affected asset:** OAuth tokens, connector events

**Mitigation timeline:** Planned for Phase 2 (Q3 2026). Crate: `focus-connectors` feature `tls-pinning`.

**Interim mitigations:**
- VPN usage recommended in onboarding docs
- Certificate pinning is documented in ADR-004

---

### **Gap #2: Audit Chain Not Cryptographically Signed**

**Description:** Audit records are stored in SQLite with SHA-256 hash linkage, but no Ed25519 signatures. Attacker with local SQLite write access can forge records and recompute hashes.

**Impact:** Audit chain cannot prove integrity to external parties (e.g., parent during dispute). User cannot prove they earned rewards or were penalized.

**Affected asset:** Audit chain integrity

**Mitigation timeline:** Planned for Phase 2. Crate: `focus-audit` feature `audit-signing`. Commits queued: `audit-sign-keypair-gen`, `audit-record-sign`, `audit-verify-batch`.

**Interim mitigations:**
- Wipe receipt (signed by app) includes audit chain hash at wipe time
- Audit chain validation on app startup detects tampering (logs warning, continues in degraded mode)

---

### **Gap #3: Template Pack Signing Not Yet Shipped**

**Description:** Template packs are stored as TOML bundles but not signed. Attacker can modify a pack TOML after distribution, or distribute packs impersonating trusted authors.

**Impact:** Malicious template packs can be distributed undetected. Community ecosystem will require signing.

**Affected asset:** Template packs

**Mitigation timeline:** Planned for Phase 2 (ecosystem launch). Crate: `focus-templates` feature `templates-signing`. Commits queued.

**Interim mitigations:**
- Built-in template packs are embedded in IPA and signed by Apple
- User consent flow on import (shows all rules before confirming)
- Community ecosystem is not yet live; deferring to Phase 5

---

### **Gap #4: No Runtime Integrity Check on Rust Binary**

**Description:** FocalPoint's Rust core is compiled into the iOS binary (via UniFFI), but there's no runtime check to detect if the binary has been patched or tampered with.

**Impact:** Attacker with local root can inject code into the Rust runtime (e.g., patch the rule evaluator to always grant points).

**Affected asset:** Rules, wallet state

**Mitigation timeline:** Deferred beyond Phase 2 (low priority; requires app hardening). Would use app-level code signing verification (iOS already provides at install time).

**Interim mitigations:**
- App is code-signed by Apple; tampering detected at install/update time
- No mechanism to modify binary at runtime without root + debugger access

---

### **Gap #5: No Jailbreak Detection (Intentional)**

**Description:** FocalPoint does not detect if the device is jailbroken. User with root access can bypass all app-level protections.

**Impact:** Root attacker can modify Rust code, SQLite database, and iOS settings at will.

**Affected asset:** All

**Mitigation timeline:** Explicitly deferred (not a goal). Philosophy: trust the device; if device is rooted, user has chosen to compromise security. Jailbreak detection is easily bypassed and provides false confidence.

**Interim mitigations:**
- Audit logs provide evidence if tampering is detected
- Keychain is still encrypted even on jailbroken device (unless attacker extracts key)

---

### **Gap #6: FamilyControls Entitlement Not Yet Approved**

**Description:** The app is built with the `com.apple.developer.family-controls` entitlement, but Apple has not yet approved it. Cannot test FamilyControls enforcement on real devices.

**Impact:** Cannot validate that restriction enforcement actually prevents app blocking, only that the API calls are correct.

**Affected asset:** FamilyControls profile

**Mitigation timeline:** Awaiting Apple review; expected approval in Q2 2026 (currently under review as of 2026-04-23).

**Interim mitigations:**
- Simulator testing (FamilyControls API works on simulator)
- Code review of ManagedSettings call sites

---

### **Gap #7: MCP Server Has No Authentication**

**Description:** The in-process MCP server (Unix-domain socket) accepts connections from any process with file-system access to the socket. No authentication or user confirmation is required.

**Impact:** A malicious app or process can connect to the socket and invoke tools (read rules, export audit, read wallet). Tools are read-only, but data exfiltration is possible.

**Affected asset:** Rules, audit, wallet (read-only disclosure)

**Mitigation timeline:** Planned for Phase 2 (user consent flow). Would add an in-app approval modal when MCP client connects.

**Interim mitigations:**
- Socket is created in app's container (not world-writable)
- Tools are read-only; no state mutations via MCP
- MCP server is opt-in feature; disabled by default

---

### **Gap #8: Ops Root Key Ceremony Not Run**

**Description:** Template-pack signing and Sentry integration require a root signing key, but the key ceremony (generation, backup, rotation) has not been performed.

**Impact:** Templates cannot be signed until key ceremony is completed.

**Affected asset:** Template packs

**Mitigation timeline:** Phase 2 (before ecosystem launch). Will be documented in `SIGNING_CEREMONY.md`.

**Interim mitigations:**
- Signing is not yet in main; can defer until Phase 2

---

## Incident Response

### **Scenario 1: Compromised Connector API (e.g., Canvas Hacked)**

**Detection:** User reports false events, or audit chain shows unusual event patterns.

**Response:**
1. Identify affected connector + date range (from audit records)
2. Publish security advisory: "Canvas API was compromised 2026-04-20 to 2026-04-22. Audit chains may contain forged events."
3. User action: Rotate Canvas token manually (Settings > Connectors > Canvas > Re-authenticate)
4. FocalPoint action: Clear cached connector events from the date range (optional; audit remains as evidence)
5. Provide rollback tool: CLI command to restore wallet/penalties from pre-compromise audit snapshot (planned in Phase 2)

**Example disclosure:** SECURITY.md, GitHub Advisories, + in-app notification.

---

### **Scenario 2: Malicious Template Pack Published**

**Detection:** Community reports that template pack "evil-rules-2.0" grants unlimited points.

**Response:**
1. App maintainer reviews pack TOML + audit records from affected users
2. If confirmed malicious:
   - Remove from App Store (built-in packs) or mark as deprecated (community packs; future)
   - Publish security advisory with detection instructions
3. User action: Uninstall the pack (Settings > Packs > Remove) or CLI: `focus templates uninstall <pack-id>`
4. Optional: Revoke pack's signing key (planned feature; requires signing ceremony)

**Example disclosure:** SECURITY.md + GitHub Advisories + in-app banner.

---

### **Scenario 3: Audit Chain Tampering Detected**

**Detection:** App startup validation fails: hash chain is broken (see `focus-audit::verify_chain()`).

**Response:**
1. App logs error: "Audit chain validation failed at record #47. Chain may have been tampered with."
2. App enters degraded mode (continues, but doesn't create new audit records until chain is repaired)
3. User options:
   - Investigate: `focus audit verify --details` shows which record is corrupted
   - Restore from backup: `focus backup restore <backup-file>` (planned in Phase 2)
   - Wipe and reset: Delete app data + reinstall (new backup required)
4. FocalPoint team: Provide incident report template for users to send to security@focalpoint.app

**User documentation:** `docs/guides/audit_chain_recovery.md` (planned in Phase 2).

---

### **Scenario 4: OAuth Token Compromised**

**Detected by:** User notices unauthorized activity (false assignments in Canvas, unauthorized GitHub actions, etc.), or connector API detects suspicious token usage.

**Response:**
1. User action: Revoke token immediately in connector's security settings (Canvas Account Settings > App Authorizations, GitHub Settings > Applications > Authorized OAuth Apps, etc.)
2. FocalPoint action: Remove cached token from Keychain (app detects revocation on next sync attempt)
3. User re-authenticates: Open FocalPoint > Settings > Connectors > [Connector] > Re-authenticate
4. Wipe receipt generated automatically on logout (see Scenario 2 in SECURITY.md)

**Audit evidence:** Audit chain logs token refresh events + connector sync timestamps.

---

### **Scenario 5: MCP Socket Exfiltration**

**Detection:** User notices suspicious activity or detects unknown process connecting to `mcp.sock`.

**Response:**
1. Disable MCP: Settings > Developer > MCP Server > Off (user action)
2. Delete socket: App automatically removes `mcp.sock` on next launch if MCP is disabled
3. Investigate: Check audit records for read operations (planned in Phase 2; will log MCP tool invocations)
4. Review processes: `lsof -p <app-pid>` shows which processes accessed the socket

**Interim:** No user-facing notification (gap #7; will be added in Phase 2 with approval modal).

---

### **Scenario 6: Device Wipe/Restore**

**User action:** Wipe device or restore from iCloud backup.

**FocalPoint response:**
1. App detects first launch post-wipe (no SQLite database found)
2. Generates wipe receipt (signed by app key) containing:
   - Last audit chain hash
   - Wallet balance at wipe time
   - Active FamilyControls restrictions
   - Timestamp + device identifier
3. Offer restore from backup: "Restore from local backup?" (if backup files exist on device)
4. If user restores from iCloud backup: Audit chain is revalidated; if backup is from before a penalty, old wallet state is restored

**Evidence:** Wipe receipt can be exported + sent to security@focalpoint.app for audit.

---

### **CVE Disclosure Process**

See SECURITY.md (existing; reference in threat model):
- Reporters email security@focalpoint.app with `[SECURITY]` subject
- 90-day coordinated disclosure window
- CVE applied for high/critical vulnerabilities
- Fixed version released as patch release
- GitHub Security Advisory + release notes published

---

## Mitigations Already Shipped (By Commit)

### Crypto & Keychain
- `focus-crypto` crate: iOS Keychain wrapper via `security-framework` (v3.0)
- Ed25519 template signing infrastructure (crate `focus-templates`, feature flag `templates-signing`; WIP in queue)

### Audit Chain
- `focus-audit` crate: SHA-256 hash linkage + append-only storage
- Validation on startup: `focus-audit::verify_chain()` recomputes all hashes (commit `27a0603` — bench suite added)
- Degraded mode if chain is broken (logs, continues without creating new records)

### Backup & Wipe
- `focus-backup` crate: Encrypted backup + restore via age + passphrase (commit `d75ab52`)
- Wipe receipt generation on app reset (signed by app key)
- CLI export: `focus backup export`, `focus backup import`

### Connector Token Handling
- Token refresh via platform OAuth 2.0 flows (URLSession delegation)
- Constant-time HMAC verification for JWT payloads
- `secrecy` crate for in-memory token wrapping (zero-on-drop)
- No token logging; Sentry PII scrubbing (WIP)

### MCP Server
- In-process Unix-domain socket (optional; opt-in)
- Read-only tools (no state mutations)
- STDIO transport (default; no socket exposure)

### Template Packs
- Embedded bundles shipped with IPA (immutable)
- User consent flow on import (shows all rules before confirming; commit `c78fb6a`)
- Starter packs + examples (commits `c78fb6a`, `7bee5ff`, `b40d6f1`)

### Governance
- DCO (Developer Certificate of Origin) on all commits (enforced via pre-commit hook; commit `cc7becc`)
- SECURITY.md with 90-day coordinated disclosure policy + CVE workflow
- CONTRIBUTING.md + GOVERNANCE.md + CoC

---

## Risk Ratings Summary

| Asset | Threat | CVSS-like Rating | Primary Mitigation | Residual Risk |
|-------|--------|------------------|--------------------|----------------|
| OAuth Tokens | MITM (no pinning) | **6.5 (High)** | TLS 1.2+ | Network attacker |
| OAuth Tokens | Keychain extraction | **7.5 (High)** | iOS encryption + biometric | Device unlock |
| Audit Chain | Tampering (no signature) | **6.0 (Medium)** | Hash linkage + startup validation | Local SQLite write |
| Audit Chain | Repudiation | **4.0 (Low)** | Wipe receipt | Device time change |
| Rules | Code injection | **3.0 (Low)** | Sandboxed Starlark eval | N/A (design constraint) |
| Rules | Reward inflation | **6.5 (High)** | Rule DSL limits | Malicious template author |
| Wallet | State tampering | **6.0 (Medium)** | Append-only records | Local SQLite write |
| Wallet | Penalty bypass | **5.5 (Medium)** | User override UI | Social engineering |
| Templates | Malicious pack | **7.0 (High)** | Signing (planned) | Unsigned ecosystem (future) |
| FamilyControls | Restriction bypass | **4.0 (Low)** | iOS ManagedSettings API | User can disable |
| MCP Server | Exfiltration | **5.0 (Medium)** | Read-only tools | No auth on socket |

---

## Recommendations (Priority Order)

### **P0: Immediate (Phase 1 finalization)**
1. Implement TLS certificate pinning for all connector APIs (Gap #1)
2. Add audit record signature verification (Gap #2) — at least validate startup chain
3. Publish SIGNING_CEREMONY.md for root key management (Gap #8)

### **P1: Phase 2 (Q3 2026)**
1. Ship template pack signing + ecosystem verification (Gap #3)
2. Add MCP client authentication + user approval flow (Gap #7)
3. Implement audit record cryptographic signatures (full implementation; Gap #2)
4. Add jailbreak detection (optional; low priority given philosophy)

### **P2: Phase 3+ (Q4 2026 onwards)**
1. Runtime integrity checks on Rust binary
2. Multi-device sync + CloudKit encryption
3. Connector compromise detection + rollback tooling

---

## Appendix: Threat Model Versioning

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-04-23 | Initial threat model; 6 personas, STRIDE per asset, 8 gaps, incident response scenarios |

**Next review:** 2027-04-23 (annual) or upon new CR security issue report.

---

## References

- **SECURITY.md** — Vulnerability reporting, CVE process, responsible disclosure
- **CLAUDE.md** — Project scaffold rules, trait contracts, audit append mandate
- **ADR.md** — Architectural decisions (ADR-001: Rust core, ADR-004: TLS)
- **Crate docs:**
  - `focus-crypto`: Keychain + secret storage
  - `focus-audit`: Append-only audit chain
  - `focus-templates`: Template pack format + signing (planned)
  - `focus-backup`: Encrypted backup + wipe receipt
  - `focus-mcp-server`: MCP tool definitions + transport
- **Incident response:**
  - **docs/guides/audit_chain_recovery.md** (planned Phase 2)
  - **SIGNING_CEREMONY.md** (planned Phase 2)
