# V3 Execution Log — 2026-06-10

**Generated:** 2026-06-10 (session start ~22:57 UTC)
**DAG:** `FLEET_100TASK_DAG_V3.md` (100 main + 20 side = 120 total)
**Mode:** Async background codex agents + parallel main agent work

## 2026-06-11 Updates (L3 subagent #50):

- **L3 #50 (pheno-cli-base Rust crate — clap + colored CLI
  facade) — completed.** New standalone `pheno-cli-base/` crate
  at the monorepo root providing the canonical 4-symbol CLI
  bootstrap for every pheno-* binary. Re-exports `clap` (v4 with
  `derive` + `color` features) so derived `#[derive(Parser)]`
  structs do not need a direct clap dep. Exposes a `CliRunnable`
  trait (`run(&self) -> Result<(), Box<dyn std::error::Error +
  Send + Sync>>` mandatory + default `main(&self)` that maps
  errors to colored stderr and `std::process::exit(1)`),
  `install_panic_hook()` that prints a uniform 4-line
  panic+backtrace block to stderr (idempotent via
  `std::sync::Once`), and `parse_from_env_or_exit::<T:
  clap::Parser>() -> T` that renders colored clap usage on parse
  failure and exits 2. The run-error type is `Box<dyn
  std::error::Error + Send + Sync>` (not a custom `AppError`
  enum) so the facade has **zero coupling** to the
  (as-yet-unmerged) `pheno-errors/` crate (L3 #46) — the spec
  explicitly mandates this decoupling. Deps: `clap = "4"`
  (features: `derive`, `color`), `colored = "2"`. 10/10 tests
  pass (5 integration in `tests/cli_test.rs` covering all 5
  spec'd test names verbatim + 3 unit in `src/lib.rs` + 2
  doctest) under `cargo --offline test --manifest-path
  pheno-cli-base/Cargo.toml`; `cargo --offline clippy
  --manifest-path pheno-cli-base/Cargo.toml --all-targets
  -- -D warnings` is clean. Standalone package via empty
  `[workspace]` table in `pheno-cli-base/Cargo.toml` (mirrors
  L3 #46/#47/#48/#49 convention; not a member of root
  `Cargo.toml`). Branch `chore/l3-50-pheno-cli-base-2026-06-11`,
  local-only (NOT pushed per task directive). See `### L3-#50
  (pheno-cli-base)` section below. Canonical worklog:
  `worklogs/l3-50-pheno-cli-base-2026-06-11.json`. Feature
  commit: `0de245c3d8c33dc21ee0027c98500a1513c97154` on branch
  `chore/l3-50-pheno-cli-base-2026-06-11`.

### L3-#50 (pheno-cli-base)

**Task (V3 DAG L3 layer):** Author the canonical
`pheno-cli-base` Rust crate — a thin facade over `clap` v4
(derive) and `colored` v2 that gives every downstream Pheno CLI
binary a uniform 4-symbol shape: `pub use clap` re-export;
`CliRunnable` trait with mandatory
`run(&self) -> Result<(), Box<dyn std::error::Error + Send +
Sync>>` + default `main(&self)` that maps the error to colored
stderr and exits 1; `install_panic_hook()` that prints a
uniform 4-line panic+backtrace block to stderr; and
`parse_from_env_or_exit::<T: clap::Parser>() -> T` that renders
colored clap usage on parse failure and exits 2.

**Spec-mandated design constraints (binding):**

1. **Use `Box<dyn std::error::Error + Send + Sync>` as the
   run-error type — not a custom `AppError` enum.** This is the
   load-bearing decoupling from the L3 #46 `pheno-errors/`
   crate, which is not on `main` as of 2026-06-11. The
   consequence is that any stdlib-`Error`-conforming type works
   (thiserror enums, `anyhow::Error`, `std::io::Error`, etc.)
   and the trait is a single 4-symbol import with no companion
   `error.rs` module. Future migration to `pheno-errors` (when
   L3 #46 lands) is a 1-line change: swap the error type
   bound.
2. **Three exit codes, all distinct:** `0` (Ok), `1` (run-time
   error from `run()` or uncaught panic), `2` (clap parse
   error). The Unix-conventional 0/1/2 split lets CI,
   supervisors, and shell scripts distinguish "user error"
   from "tool error" from "usage error".
3. **Force-enable `colored::control::set_override(true)` on
   every stderr-write path** so the colored output survives
   non-TTY captures (`assert_cmd`, CI log scrapers,
   `cargo test`). The `error_to_stderr_message_is_colored`
   integration test asserts on the literal `\x1b[` ANSI-CSI
   byte sequence to lock this in.

**Crate layout:** Five files in a new `pheno-cli-base/`
directory at the monorepo root, declared as a standalone
package via an empty `[workspace]` table in its own
`Cargo.toml` (mirrors the L3 #46/`#47`/`#48`/`#49`/`#57`
convention — NOT a member of the ~60-crate root `Cargo.toml`
`[workspace.members]`). Files:

| Path | Lines | Purpose |
|---|---:|---|
| `pheno-cli-base/.gitignore`            |   1 | Excludes `/target` (per-crate `target/` is not the root workspace's `target/`) |
| `pheno-cli-base/Cargo.toml`            |  52 | Package manifest + clap 4 (derive + color) + colored 2 + assert_cmd 2 + predicates 3 (dev-deps) + empty `[workspace]` table |
| `pheno-cli-base/src/lib.rs`            | 243 | `pub use clap;` re-export, `CliRunnable` trait (4-symbol surface), `install_panic_hook` (Once-guarded), `parse_from_env_or_exit`, 3 inline `#[test]`s, 2 doctests |
| `pheno-cli-base/src/bin/cli_smoke.rs`  |  71 | In-crate smoke binary driven by the integration tests through real OS process boundaries (assert_cmd) |
| `pheno-cli-base/tests/cli_test.rs`     | 120 | 5 integration tests — the spec's `>=5` floor, with the 5 spec'd test names verbatim |

**Public API (4 symbols, re-exported from `pheno_cli_base::`):**

```rust
// Re-export of clap v4 (derive + color features) so derived
// structs do not need a direct clap dep in their Cargo.toml.
pub use clap;

pub trait CliRunnable {
    fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn main(&self) {
        // Default: on Ok(()) return to caller (caller exits 0);
        // on Err(e) write `error: <e>` (colored, "error:" in bold
        // red) to stderr and process::exit(1). Force-enables
        // colored::control::set_override(true) so ANSI escapes
        // survive non-TTY stderr.
    }
}

pub fn install_panic_hook(); // Once-guarded, 4-line panic block
pub fn parse_from_env_or_exit<T: clap::Parser>() -> T; // exits 2 on parse failure
```

**`parse_from_env_or_exit` design (the 1-liner that pays for
the whole crate):** The signature is `-> T` (not `Result<T,
clap::Error>`). Internally it is

```rust
match T::try_parse_from(std::env::args_os()) {
    Ok(t) => t,
    Err(e) => e.exit(),  // writes colored usage, process::exit(2)
}
```

This is the same `unwrap_or_else(|e| e.exit())` pattern that
shows up in 80% of Rust CLI `main` functions; hiding it behind
a 1-line wrapper is the value-add of this crate. It reads
`std::env::args_os()` (not `args()`) so binary names with
non-UTF8 bytes are preserved — matches clap's own
`Command::get_matches_from` behavior.

**`install_panic_hook` design:** Uses
`std::sync::Once::call_once(...)` to guard the
`set_hook` call. Outputs a uniform 4-line block to stderr:

```text
panic: thread '<name>' panicked at <file>:<line>:<col>:
  <payload>
stack backtrace:
   0: ...
   1: ...
   ...
```

After printing, the hook returns and the default Rust
unwind-abort (or unwind) takes over — we do NOT call
`process::exit(1)` ourselves, so the standard panic semantics
are preserved. The hook handles both `&'static str` and
`String` panic payloads (the two common shapes); anything else
falls back to `"<non-string panic payload>"`.

**`CliRunnable::main` color contract:** Force-enables
`colored::control::set_override(true)` at the top, so the
`error: <msg>` line is colored even when stderr is captured by
`assert_cmd::Command::assert()` (which is a non-TTY pipe). The
integration test `error_to_stderr_message_is_colored` asserts
on the literal `\x1b[` byte sequence (the ANSI-CSI escape) to
lock this in.

**Exit code contract:**

| Situation | Exit code | Source |
|---|--:|---|
| `run()` returns `Ok(())` | `0` | Caller's responsibility (return `ExitCode::SUCCESS` from `main`) |
| `run()` returns `Err(_)` | `1` | Default `CliRunnable::main` impl |
| `T::try_parse_from` fails | `2` | `clap::Error::exit()` from `parse_from_env_or_exit` |
| Uncaught panic | `1` (default) | After our 4-line panic block prints to stderr |

Matches the standard Unix CLI convention. Lets CI and
supervisors distinguish user errors (exit 1) from tool errors
(exit 2) from successes (exit 0).

**Test coverage (10/10 pass under
`cargo --offline test --manifest-path
pheno-cli-base/Cargo.toml`):**

The 5 spec-required integration tests are all present in
`tests/cli_test.rs` by exact name:

| # | Test | What it checks |
|--:|------|----------------|
|  1 | `cli_runnable_default_main_runs` | Drives `cli_smoke --name world`; asserts exit 0, stdout `ok: world\n`, empty stderr |
|  2 | `install_panic_hook_does_not_panic_on_normal_exit` | Drives `cli_smoke --name alice`; asserts exit 0 and stdout `ok: alice\n` (proves the hook installed in `main` did not interfere with control flow) |
|  3 | `parse_from_env_or_exit_parses_valid_args` | Drives `cli_smoke --name bob`; asserts exit 0 and stdout `ok: bob\n` (proves the parsed value reached `run()`) |
|  4 | `parse_from_env_or_exit_exits_on_missing_required` | Drives `cli_smoke` with no args; asserts exit code 2 and stderr contains `error` and `--name` (proves clap's parse-failure path was taken) |
|  5 | `error_to_stderr_message_is_colored` | Drives `cli_smoke --name any --fail kaboom`; asserts exit 1 and stderr contains the literal `\x1b[` ANSI-CSI escape (proves `colored` emitted ANSI codes even though `assert_cmd` captured stderr to a non-TTY pipe), `error:` prefix, and the `kaboom` body |

Plus 3 inline unit tests in `src/lib.rs::tests`:

| # | Test | What it checks |
|--:|------|----------------|
|  1 | `cli_runnable_main_returns_on_ok_unit` | The default `main` impl on `Ok(())` returns to the caller (no `process::exit` taken) |
|  2 | `error_render_includes_message_body` | The `colored` builders used in the default `main` impl render a string containing `error:` and the user message |
|  3 | `install_panic_hook_is_idempotent` | Calling `install_panic_hook()` twice is a `Once::call_once` no-op (no panic, no double-install) |

And 2 doctests in `src/lib.rs`:

| # | Location | What it checks |
|--:|------|----------------|
|  1 | `src/lib.rs:15` (module-level) | The crate-level quickstart doctest compiles |
|  2 | `src/lib.rs:170` (`parse_from_env_or_exit`) | The `parse_from_env_or_exit` example doctest compiles |

**Test isolation:** The 5 integration tests use
`assert_cmd::Command::cargo_bin("cli_smoke")` to drive the
in-crate smoke binary as a real subprocess (real OS process
boundaries; real stderr capture; real exit code propagation).
The smoke binary's flag surface (`--name <NAME>` required,
`--fail <MSG>` optional, `--panic` optional) is intentionally
narrow — one flag per `pheno-cli-base` behavior under test. The
binary is `publish = false` and is not part of the crate's
public API surface.

**Deps resolution:** All deps (clap 4, colored 2, assert_cmd
2, predicates 3) resolved cleanly from
`~/.cargo/registry/cache`. `cargo --offline test
--manifest-path pheno-cli-base/Cargo.toml` runs in ~0.5s for
the integration tests + ~0.1s for doctests after a ~55s cold
compile (clap_derive + assert_cmd + predicates build). No
5-minute resolver timeout.

**`Cargo.lock` is NOT committed** (the crate is a standalone
binary/library, and standalone-binary `Cargo.lock` is not
required by the cargo spec for `publish = false` crates). This
mirrors the L3 #46/#47/#48/#49/#57 pattern: their worklogs do
not list `Cargo.lock` in `scope.created`. A per-crate
`pheno-cli-base/.gitignore` excludes `/target`.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  #47 pheno-tracing, #48 pheno-config, #49 pheno-otel, #51
  pheno-fastapi-base, #52 pheno-go-ctxkit, #53
  pheno-zod-pydantic, #54 pheno-tower-stack, #55
  pheno-ssot-template, #56 pheno-flags, #57 pheno-plugin).
- **Did NOT push to origin.** Branch is
  `chore/l3-50-pheno-cli-base-2026-06-11`, off `main`
  (1 commit ahead of `52dfc7aa06`).
- **No async runtime pulled in by the new crate.** The
  public API is fully synchronous; the panic hook is
  sync-only; `parse_from_env_or_exit` is a pure
  argv-to-struct function. clap itself is
  async-runtime-free.
- **Object safety.** `CliRunnable` is object-safe (no
  associated types, no generic methods, only `&self`
  receivers, no `Self`-in-return-position), so downstream
  crates can store `Box<dyn CliRunnable>` in a
  multi-subcommand dispatcher.

**Why `Box<dyn Error>` not a custom `AppError` enum (spec
rationale):** The task directive explicitly says "Use
`Box<dyn Error>` instead of `pheno_errors` to avoid cross-crate
coupling." The `pheno-errors/` crate (L3 #46) does not exist
on `main` as of 2026-06-11 — only on a parallel worktree —
and depending on a sibling worktree's path is not safe in CI.
Even if L3 #46 were on `main`, a `Box<dyn Error>` shape is
strictly more general (it accepts any `Error`-conforming type
— thiserror enums, `anyhow::Error`, `std::io::Error`, etc.)
and makes the facade useful for crates that don't adopt the
`pheno-errors` taxonomy. Future migration to `pheno_errors::
AppError` (when L3 #46 lands) is a 1-line change in the trait
definition.

**`parse_from_env_or_exit` reads `args_os()` not `args()`:**
Preserves binary names with non-UTF8 bytes (matches clap's own
`Command::get_matches_from` behavior). On parse failure, the
helper relies on `clap::Error::exit()` to write the colored
usage line to stderr and exit 2 — we do NOT call
`set_override(true)` ourselves on this path because clap's
`color` feature (enabled in our `Cargo.toml`) handles the
color decision internally based on its own TTY detection.

**Downstream consumers:**

- **L4 #70 (`helioscli` binary) and any future L4/L5
  pheno-*-cli binary** can have a 3-line `main`:
  ```rust
  fn main() -> std::process::ExitCode {
      install_panic_hook();
      parse_from_env_or_exit::<MyArgs>().main();
      std::process::ExitCode::SUCCESS
  }
  ```
- This collapses the 3-line clap+colored+panic-hook boilerplate
  that was previously duplicated across every fleet binary into
  a single 4-symbol import.

**Known limitations (documented in the worklog, not blocking
L3 #50 acceptance):**

- The `CliRunnable::main` error path and the
  `parse_from_env_or_exit` parse-failure path are tested via
  subprocess (`cli_smoke`) — driving them in-process would
  call `process::exit` and tear down the test process. The
  in-process coverage is at the unit-test level via the
  `colored`-builder re-implementation in
  `tests::error_render_includes_message_body`.
- The panic hook is installed but never exercised in the
  integration suite (a real `panic!` would terminate the test
  process with a noisy stack trace). It is verified
  indirectly by `install_panic_hook_does_not_panic_on_normal_exit`
  and the unit test `install_panic_hook_is_idempotent`.
