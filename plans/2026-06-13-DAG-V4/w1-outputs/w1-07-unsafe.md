# W1-07 — Unsafe / FFI Audit Report for FocalPoint

> Scope: `FocalPoint/src/` and `FocalPoint/crates/` (`.rs` files only)  
> Generated: 2026-06-13  
> Searched patterns: `unsafe` blocks, `unsafe fn`, `extern "C"`, `extern "system"`, `dlopen`, `dlsym`, `libc::`, `ffi::`, `core::ffi`, raw-pointer APIs, `mem::transmute`, etc.

---

## Summary

| Category | Count in Source |
|----------|-----------------|
| `unsafe { ... }` blocks | **0** |
| `unsafe fn` declarations | **0** |
| `unsafe impl` / `unsafe trait` | **0** |
| `extern "C"` declarations | **2** |
| `extern "system"` declarations | **0** |
| `dlopen` / `dlsym` calls | **0** |
| `libc::` usage | **0** |
| Raw pointer APIs (`ptr::`, `from_raw_parts`, `transmute`, etc.) | **0** |

**Note:** `FocalPoint/src/` does not exist; the project uses a workspace-only layout (`crates/`).  
**Note:** Build artifacts under `target/` (e.g., `libsqlite3-sys` bindgen output, `uniffi` generated scaffolding) contain hundreds of `unsafe extern "C"` declarations, but these are **generated**, not hand-written source code, and are therefore excluded from this report.

---

## 1. `unsafe` Blocks

**None found.**

A full-text search of all `.rs` files under `FocalPoint/crates/`, `FocalPoint/apps/`, `FocalPoint/services/`, `FocalPoint/tests/`, `FocalPoint/examples/`, `FocalPoint/fuzz/`, and `FocalPoint/tooling/` returned zero matches for `unsafe {`, `unsafe fn`, `unsafe impl`, or `unsafe trait`.

---

## 2. `unsafe fn` Declarations

**None found.**

---

## 3. FFI Calls — `extern "C"` / `extern "system"`

### 3.1 `extern "C"` — 2 occurrences (both in plugin SDK examples)

| File | Line | Declaration |
|------|------|-------------|
| `crates/focus-plugin-sdk/examples/hello-connector/src/lib.rs` | 7 | `pub extern "C" fn poll(config_ptr: i32, config_len: i32) -> i64` |
| `crates/focus-plugin-sdk/examples/slack-reference/src/lib.rs` | 40 | `pub extern "C" fn poll(_config_ptr: i32, _config_len: i32) -> i64` |

Both are WASM plugin entry points with `#[no_mangle]`. They do **not** contain `unsafe` blocks; they are standard ABI signatures for the plugin sandbox.

### 3.2 `extern "system"`

**None found.**

---

## 4. `dlopen` / `dlsym` / `libc::` / `ffi::` / Raw Pointer APIs

**None found.**

No calls to `dlopen`, `dlsym`, or `libc::` functions. No usage of `std::ffi`, `core::ffi`, `std::os::raw`, `std::ptr::`, `std::mem::transmute`, `Box::from_raw`, `Vec::from_raw_parts`, `CString::new_unchecked`, or `str::from_utf8_unchecked` in any source `.rs` file.

---

## 5. Comments / TODOs mentioning FFI

Two crates contain comments about future FFI integration, but no actual unsafe code:

| File | Line | Context |
|------|------|---------|
| `crates/connector-fitbit/src/auth.rs` | 91 | `// TODO: Call into iOS keychain via FFI (crates/focus-ffi).` |
| `crates/connector-strava/src/auth.rs` | 95 | `// TODO: Call into iOS keychain via FFI (crates/focus-ffi).` |

---

## 6. `focus-ffi` Crate

The crate `crates/focus-ffi/` is the UniFFI bridge layer. Despite its name, its hand-written Rust source contains **no `unsafe` blocks, no `extern` declarations, and no raw pointer operations**. It relies entirely on the `uniffi` macro-generated scaffolding (`uniffi::include_scaffolding!("focus_ffi")`), which lives in `target/` and is excluded from this source audit.

---

## Conclusion

FocalPoint's Rust source code is **100% safe Rust** from an `unsafe` / FFI perspective. The only `extern "C"` functions are two WASM plugin examples in `focus-plugin-sdk`, and neither uses `unsafe`. All actual FFI bridging is delegated to generated code (`uniffi`, `libsqlite3-sys` bindgen) in `target/`.
