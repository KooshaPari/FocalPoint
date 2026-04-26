# W-71 Phase 1 Discovery — Templates-Registry Modernization

## Executive Summary
**multipart 0.18 Usage:** 1 site (handlers.rs, line 104–135). Upload endpoint is **stubbed**: TODO comment on line 129 indicates multipart parsing not yet implemented.

**Signing/Integrity Flow:** SHA-256 digest computed post-load via `focus_templates::signing::digest_pack()` (db.rs:73). Signature is detached ed25519 (base64-encoded, stored in DB). No streaming hash during upload (stub prevents it).

**Cargo-deny Status:** 19 advisories (5 CVE + 14 unmaintained). Root cause: multipart 0.18 → hyper 0.10 + iron + nickel transitive chain.

## Multipart Use Sites
- **handlers.rs:104–135** — `upload_pack()` handler. Bearer token auth works (lines 107–127). **TODO on line 129: multipart handling stub — not implemented.**
- **Cargo.toml:31** — `multipart = "0.18"` declared; no other uses in source.
- **No active consumers:** grep for `/templates/upload` across FocalPoint returned zero matches (endpoints defined but unreachable via handlers).

## Wire Shape & Constraints

### Upload Endpoint Contract (POST /api/v1/packs)
- **Fields:** pack.tar.zst + signature.ed25519 (detached signature).
- **Response:** UploadResponse { status, id, sha256 } (models.rs:98–105).
- **Auth:** Bearer token (constant-time comparison in auth.rs).
- **Rate Limit:** 10 req/min per IP.
- **Body Size Limit:** Default axum `DefaultBodyLimit` = 2 MB (does NOT support large bundles; risk flag in proposal).

### Signing & Integrity
- **Digest Location:** `focus_templates::signing::digest_pack()` (db.rs:73) — computes SHA-256 post-load.
- **Signature Encoding:** Base64 (models.rs:58).
- **Signature Algorithm:** ed25519 (Cargo.toml:21 `ed25519-dalek = "2"`).
- **Streaming Hash:** NOT implemented (stub prevents it); Phase 3 must co-locate digest with stream reader.

### Rating Submission (Unaffected)
- POST /api/v1/packs/:id/rate uses standard JSON body; no multipart. Rate limit: 10 req/min; IP hash via SHA-256 (handlers.rs:88–91).

## Baseline Audit
- **Warnings:** 1 `#[allow(dead_code)]` in models.rs (UploadResponse, line 100—correctly flagged as unused until upload impl).
- **Tests:** 8 tests (handlers, auth); none hit upload endpoint (stub prevents it).
- **Dependencies:** `axum = "0.7"`, `tokio`, `tower-http`, `rusqlite`, `ed25519-dalek = "2"`, `sha2 = "0.10"`, `base64 = "0.21"`.

## Phase 2 Recommendation
**Default Path:** `axum::extract::Multipart` (present transitively; zero new deps; tower integration provides body limits and streaming).
**Migration Cost:** Low (upload is stubbed; no existing contract to preserve except response shape).
**Critical Assertion:** Phase 3 must verify digest computed on raw archive bytes (not decoded) to prevent signature bypass.

## Cargo-deny Snapshot
```
[19 advisories]
hyper 0.10 (x2) — unmaintained, blocks multipart 0.18 drop
iron — unmaintained, blocks multipart 0.18 drop
nickel — unmaintained, blocks multipart 0.18 drop
idna, protobuf, time CVEs — all transitive from multipart 0.18
```

All 19 cleared post-multipart drop (verified in proposal.md).
