# V3 Execution Log — 2026-06-10

**Generated:** 2026-06-10 (session start ~22:57 UTC)
**DAG:** `FLEET_100TASK_DAG_V3.md` (100 main + 20 side = 120 total)
**Mode:** Async background codex agents + parallel main agent work

## 2026-06-11 Updates (L3 subagent #49):

- **L3 #49 (pheno-otel Rust crate) — completed.** New standalone
  Rust crate at `pheno-otel/` providing a one-liner OpenTelemetry
  initialization API with a Drop-based `TelemetryGuard`. Two
  initialization entry points — `init(service_name)` (OTLP HTTP
  exporter) and `init_with_stdout(service_name)` (no-network
  stdout exporter for local dev) — both return a
  `TelemetryGuard` whose `Drop` impl flushes pending spans and
  calls `opentelemetry::global::shutdown_tracer_provider()` to
  reset the global tracer provider to a no-op. `OtelError` is a
  thiserror enum with the spec's three variants
  (`ExporterInit`, `ResourceBuild`, `Shutdown`). Pinned to the
  OpenTelemetry 0.27 line (`opentelemetry = "0.27"`,
  `opentelemetry_sdk = "0.27"` with `trace` feature,
  `opentelemetry-otlp = "0.27"` with
  `http-proto`/`reqwest-client`/`reqwest-rustls`/`trace`
  features). `init_with_stdout` ships a hand-rolled
  `StdoutSpanExporter` (`pheno-otel/src/exporter/stdout.rs`)
  rather than depending on the separate `opentelemetry-stdout`
  crate. 18/18 tests pass (10 unit + 5 integration in
  `tests/init_test.rs` + 3 doctest) under `cargo test --offline`;
  `cargo clippy --offline --all-targets -- -D warnings` is
  clean. Branch `chore/l3-49-pheno-otel-2026-06-11`, local-only
  (NOT pushed per task directive). See `### L3-#49 (pheno-otel)`
  section below. Canonical worklog:
  `worklogs/l3-49-pheno-otel-2026-06-11.json`. Feature commit:
  `ad8065eb1fc7c1c350400359768faa3084c7516b` on branch
  `chore/l3-49-pheno-otel-2026-06-11`.

- **L3 #50 (pheno-cli-base Rust crate — clap + colored CLI base)
  — completed.** New standalone `pheno-cli-base/` crate at the
  monorepo root providing the canonical facade for every Pheno
  CLI binary. Re-exports `clap` (so derived `#[derive(Parser)]`
  structs do not need a direct clap dep), exposes a `CliRunnable`
  trait (`run(&self) -> Result<(), AppError>` mandatory +
  default `main(&self) -> !` that maps `AppError` to colored
  stderr and `std::process::exit(1)`), `install_panic_hook()` that
  prints the panic message + backtrace to stderr and exits 1
  (re-entrant via `std::sync::Once`), and
  `parse_from_env_or_exit::<T: clap::Parser>() -> T` that renders
  colored clap usage on parse failure and exits 2. `AppError` is
  defined as a local stub (thiserror 2.0; 6 variants mirroring
  the L3 #46 spec — `Validation`, `NotFound`, `Storage`,
  `Config`, `Domain`, `Io`) because the `pheno-errors/` crate
  (L3 #46) does not yet exist on `main`; a one-line migration
  to a path dep is planned when L3 #46 lands (documented in the
  worklog `spec_deviations`). Deps: `clap = "4"` (derive
  feature), `colored = "2"`, `thiserror = "2.0"`. 17/17 tests
  pass (5 integration in `tests/cli_test.rs` covering all 5
  spec'd test names verbatim + 8 unit + 4 doctest) under
  `cargo test --offline`; `cargo clippy -p pheno-cli-base
  --all-targets -- -D warnings` is clean. Standalone package
  via empty `[workspace]` table in `pheno-cli-base/Cargo.toml`
  (mirrors L3 #46/#47/#48/#49/#57 convention; not a member of
  root `Cargo.toml`). Branch
  `chore/l3-50-pheno-cli-base-2026-06-11`, local-only (NOT
  pushed per task directive). See `### L3-#50
  (pheno-cli-base)` section below. Canonical worklog:
  `worklogs/l3-50-pheno-cli-base-2026-06-11.json`. Feature
  commit: `659e173003` on branch
  `chore/l3-50-pheno-cli-base-2026-06-11`.

- **L3 #57 (pheno-plugin Rust crate — plugin registry + dynamic
  dispatch) — completed.** New standalone `pheno-plugin/` crate
  at the monorepo root providing the canonical in-process plugin
  registry for the pheno-* fleet. The `Plugin` trait is
  object-safe (`Send + Sync` + no associated types + no generic
  methods) and exposes `name()`, `version()`, and a default-noop
  `init()` hook; `PluginRegistry` is a name-indexed
  `HashMap<String, Box<dyn Plugin>>` with `new()`,
  `register()` (rejects duplicate names with
  `PluginError::DuplicateName`), `get()`, `names()` (sorted
  ascending), and `init_all()` (bulk init in registration order,
  short-circuits on first failure). `PluginError` is a thiserror
  enum with two tuple variants per the L3 #57 spec verbatim —
  `DuplicateName(String)` and `InitFailed(String)`. One
  dependency: `thiserror = "2.0"`. 8/8 tests pass (6 integration
  tests in `tests/registry_test.rs` covering all 6 spec'd test
  names: `registry_starts_empty`, `register_adds_plugin`,
  `register_rejects_duplicate`, `get_returns_registered_plugin`,
  `init_all_invokes_each_plugin_init`, `names_returns_sorted` +
  2 doctest); `cargo clippy --all-targets -- -D warnings` is
  clean. Standalone package via empty `[workspace]` table in
  `pheno-plugin/Cargo.toml` (mirrors L3 #46/#47/#48/#49
  convention; not a member of root `Cargo.toml`). Branch
  `chore/l3-57-pheno-plugin-registry-2026-06-11`, local-only
  (NOT pushed per task directive). See `### L3-#57
  (pheno-plugin-registry)` section below. Canonical worklog:
  `worklogs/l3-57-pheno-plugin-registry-2026-06-11.json`.
  Feature commit: `3d2f9d4bc7` on branch
  `chore/l3-57-pheno-plugin-registry-2026-06-11`.

### L3-#49 (pheno-otel)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-otel`
Rust crate wrapping the OpenTelemetry 0.27 initialization chain
into a one-liner API: `init(service_name)` for production
OTLP-backed telemetry and `init_with_stdout(service_name)` for
local dev / CI smoke tests, both returning a `TelemetryGuard`
that flushes + shuts down the global tracer provider on Drop.
Consumed by L4 #70 (`helioscli` binary) and L5 #81–85 (the
5 pheno-* service crates) as the single source of truth for
runtime telemetry setup.

**Crate layout:** Nine files in a new `pheno-otel/` directory at
the monorepo root, declared as a standalone package via an
empty `[workspace]` table in its own `Cargo.toml` (the L3 #46
`pheno-errors` pattern — NOT a member of the 56+-crate root
`Cargo.toml` `[workspace.members]`). This keeps the new crate's
test/build loop independent of the root workspace and avoids
conflicting with the other L3 agents concurrently editing the
root manifest. Files:

| Path | Lines | Purpose |
|---|---:|---|
| `pheno-otel/Cargo.toml`        |  59 | Package manifest + OpenTelemetry 0.27 deps + empty `[workspace]` table |
| `pheno-otel/README.md`         |  63 | Quickstart + env-var contract |
| `pheno-otel/src/lib.rs`        |  51 | Crate-level docs + module declarations + re-exports |
| `pheno-otel/src/error.rs`      | 120 | `OtelError` (3-variant thiserror enum) + 4 inline `#[test]`s |
| `pheno-otel/src/guard.rs`      | 119 | `TelemetryGuard` (RAII; Drop impl + `shutdown` + `Debug`) + 3 inline `#[test]`s |
| `pheno-otel/src/init.rs`       | 138 | `init` + `init_with_stdout` + `build_resource` + `install_provider` + 2 doctests |
| `pheno-otel/src/exporter/mod.rs` | 11 | `pub mod stdout;` |
| `pheno-otel/src/exporter/stdout.rs` | 210 | Hand-rolled `StdoutSpanExporter` (one JSON line per span) + 3 inline `#[test]`s |
| `pheno-otel/tests/init_test.rs` | 249 | 5 integration tests (the spec's `>=5` floor) |

**Public API (3 symbols re-exported from `pheno_otel::`):**

1. `init(service_name: &str) -> Result<TelemetryGuard, OtelError>`
   — installs an OTLP/HTTP span exporter
   (`opentelemetry_otlp::SpanExporter::builder().with_http()`),
   wires it into a `TracerProvider` with
   `service.name=<service_name>` on the `Resource`, installs
   the provider as the global, and returns a `TelemetryGuard`.
   The endpoint is read from the SDK's standard
   `OTEL_EXPORTER_OTLP_ENDPOINT` env var; `init()` itself
   passes `DEFAULT_OTLP_ENDPOINT` (`"http://localhost:4318"`)
   to `with_endpoint(...)` so the SDK's env-var resolution
   path can override it.
2. `init_with_stdout(service_name: &str) -> Result<TelemetryGuard, OtelError>`
   — installs the hand-rolled `StdoutSpanExporter` (one JSON
   line per span to `std::io::stdout()`, no protobuf). No
   network I/O — safe in air-gapped environments and CI
   sandboxes.
3. `TelemetryGuard` — the RAII guard. Holds the
   `TracerProvider` (so it stays alive until the guard drops)
   and a `&'static str` `source` label (`"otlp"` or `"stdout"`,
   surfaced in `Debug` for test diagnostics). `Drop` calls
   `opentelemetry::global::shutdown_tracer_provider()` first
   (to swap the global for a no-op), then an explicit
   `force_flush()` + `shutdown()` on the held provider. Drop
   errors are logged to stderr at WARN; they do NOT panic
   (Drop cannot return). The `shutdown(&self)` method surfaces
   `OtelError::Shutdown` to callers who want typed error
   handling; the operations are idempotent so explicit
   shutdown does NOT prevent the Drop path from also running.

**`OtelError` (3 variants, thiserror derive):**

- `ExporterInit(String)` — the OTLP `SpanExporter::builder()`
  rejected the configuration (e.g. `with_endpoint(...)` got an
  invalid URI). Returned by `init()`.
- `ResourceBuild(String)` — the `Resource` (the entity that
  produces telemetry) could not be built. Currently fires when
  the caller passes an empty or whitespace-only `service_name`.
  Returned by both `init()` and `init_with_stdout()`.
- `Shutdown(String)` — the tracer provider could not be shut
  down cleanly (transport error from `force_flush()` or
  `shutdown()`). Returned by `TelemetryGuard::shutdown()`;
  the `Drop` impl logs these at WARN.

`OtelError: std::error::Error + Send + Sync + 'static` (the
inner trace error is rendered into the `Display` string at
construction time so the variant stays a self-contained
thiserror enum — no `#[from]` plumbing required). Also exposes
a stable `kind(&self) -> &'static str` tag
(`"exporter_init"` / `"resource_build"` / `"shutdown"`) for log
fields and metrics labels, plus constructor fns
(`exporter_init`, `resource_build`, `shutdown`).

**Test coverage (18/18 pass under `cargo test --offline`):**
The 5 spec-required integration tests are all present in
`tests/init_test.rs` by exact name:

| # | Test | What it checks |
|--:|------|----------------|
|  1 | `init_returns_guard`                            | `init_with_stdout` returns a `TelemetryGuard`; `Debug` render mentions both `TelemetryGuard` and the `source` label (`"stdout"`) |
|  2 | `init_with_stdout_emits_test_span`             | `init_with_stdout` produces a working tracer; `tracer.start(...).set_attribute(...).end()` succeeds; `guard.shutdown()` returns `Ok(())` |
|  3 | `guard_drop_calls_shutdown`                    | Two `init_with_stdout` calls in sequence; the second call succeeds only if the first guard's Drop ran `global::shutdown_tracer_provider()` and reset the global to a no-op |
|  4 | `otel_error_display_messages_are_useful`       | For each of the 3 variants, `Display` contains both the variant keyword AND the wrapped context string |
|  5 | `init_with_invalid_endpoint_returns_exporter_init_error` | `opentelemetry_otlp::SpanExporter::builder().with_http().with_endpoint("not a valid uri !!!").build()` fails and the resulting `TraceError` is mapped to `OtelError::ExporterInit` |

Plus 10 inline unit tests (4 in `error::tests` —
`constructors_set_variant`, `is_std_error`, `kind_is_stable`,
`display_mentions_kind`; 3 in `guard::tests` —
`default_provider_shutdown_is_ok`, `drop_does_not_panic`,
`drop_with_active_span_does_not_panic`; 3 in
`exporter::stdout::tests` — `render_uses_name_when_present`,
`render_falls_back_to_seq_when_name_empty`,
`render_skips_invalid_parent_span_id`) and 3 doctests
(`src/lib.rs:11` crate-level quickstart;
`src/init.rs:51` `init::init` example;
`src/init.rs:80` `init::init_with_stdout` example).

**Test isolation:** Tests that touch the global tracer
provider serialize themselves via a process-static
`INIT_LOCK: Mutex<()>` (the global can only be set once per
process to a meaningful value; without the lock, parallel
tests would race). The `init_with_invalid_endpoint_*` test
saves + clears `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` and
`OTEL_EXPORTER_OTLP_ENDPOINT` and restores them on drop via a
local `Restore` RAII guard — so no env bleed between parallel
tests. The stdout-capturing test acquires a process-static
`stdout_lock: Arc<Mutex<()>>` (via `once_cell::sync::Lazy`)
so test output is not interleaved.

**Deps resolution:** OpenTelemetry 0.27 line resolved cleanly
from `~/.cargo/registry/cache`. The first cold `cargo check`
timed out at 5 minutes on the reqwest + rustls + opentelemetry
+ tonic transitive tree; subsequent `--offline` runs are
sub-2-second incremental. No 5-minute resolver timeout on the
final verification runs.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #50
  pheno-cli-base, L3 #51 pheno-fastapi-base, L3 #52
  pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags, L3 #57 pheno-plugin-registry).
- **Did NOT push to origin.** Branch is
  `chore/l3-49-pheno-otel-2026-06-11`, off `main` (1 commit
  ahead).
- **No FFI, no async runtime pulled in by the new crate.**
  The OTLP exporter pulls in `reqwest` (rustls) internally,
  but `pheno-otel` itself does not depend on `tokio` or
  `async-std` — the public API is synchronous.
- **Worktree isolation.** Worktree at
  `.worktrees/l3-49-pheno-otel-2026-06-11` isolates from the
  concurrent L3 branch switches happening in the shared
  `repos/` worktree.

**Drop semantics (explicitly designed):** `TelemetryGuard::drop`
is best-effort — it calls `global::shutdown_tracer_provider()`
(replaces the global with a no-op; subsequent
`global::tracer(...)` calls will get a noop tracer) and then
runs the held provider's `force_flush()` + `shutdown()` (best-
effort; errors are logged to stderr at WARN and otherwise
swallowed because Drop MUST NOT panic). Explicit
`guard.shutdown(&self)` returns the typed `OtelError::Shutdown`
and is idempotent w.r.t. the Drop path. The two operations
are intentionally independent so a caller who drops the guard
early still gets the global reset.

**Why a hand-rolled `StdoutSpanExporter` (not
`opentelemetry-stdout`):** Three reasons. (1) The
`opentelemetry-stdout` crate pulls in additional
tonic/serde features we don't need for a one-line JSON
exporter. (2) The format we want is non-standard (no
protobuf, no OTLP framing — just a greppable single JSON line
per span, with a `span#<seq>` fallback when the span name
is empty). (3) It avoids one more dep in the cold-compile
path. The `render()` helper is a separate `fn` so the JSON
serialization can be unit-tested without an async runtime.

**`init()` endpoint resolution:** `init()` does NOT take an
explicit endpoint parameter — it relies on the OpenTelemetry
SDK's standard env-var resolution
(`OTEL_EXPORTER_OTLP_ENDPOINT`,
`OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`). This matches the OTel
spec's HTTP exporter env-var contract. A future revision can
add `init_with_endpoint(service_name, url)` if a hard-coded
endpoint is needed; for now the spec's one-liner API is
preserved.

**Downstream:** L5 #81–85 (the 5 pheno-* service crates) can
now do `let _guard = pheno_otel::init("pheno-<svc>")?;` at
startup and forget about shutdown — the Drop path handles
it. L4 #70 (`helioscli` binary) main() can return
`Result<(), OtelError>`; `?` propagates the init error. The
hand-rolled stdout exporter can also be re-used by
integration tests in any downstream crate to capture
spans-in-flight without standing up a collector.

**Consolidation targets:** `agileplus-telemetry`
(`AgilePlus-wt-L1-001/crates/agileplus-telemetry`) also wraps
`opentelemetry-otlp`; `pheno-otel` is the canonical
lightweight sibling with the Drop-guard ergonomics that
AgilePlus's service-init macro layer can re-export. The
pre-existing `phenotype-otel/` placeholder crate (referenced
from the docs site) is left in place; the new `pheno-otel` is
a strict superset (it adds the stdout path and the
Drop-guard ergonomics).

### L3-#57 (pheno-plugin-registry)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-plugin`
Rust crate providing the in-process plugin registry for the
pheno-* fleet. The `Plugin` trait is object-safe (so it can be
stored as `Box<dyn Plugin>` and loaded at runtime from crates the
host does not statically depend on) and exposes
`name(&self) -> &str`, `version(&self) -> &str`, and a
default-noop `init(&self) -> Result<(), PluginError> { Ok(()) }`
hook. `PluginRegistry` is a name-indexed
`HashMap<String, Box<dyn Plugin>>` with `new()`,
`register(Box<dyn Plugin>) -> Result<(), PluginError>` (rejects
duplicates with `DuplicateName`), `get(&str) -> Option<&dyn Plugin>`,
`names() -> Vec<String>` (sorted ascending), and
`init_all() -> Result<(), PluginError>`. `PluginError` is a
thiserror enum with the spec's two tuple variants —
`DuplicateName(String)` and `InitFailed(String)`. Consumed by
L5 #88 (`helioscli` — wire HeliosCLI to pheno-plugin and load
`helios-plugin-*` crates at startup) and any future L5 pheno-*
host that wants a uniform plugin entrypoint.

**Crate layout:** Three files in a new `pheno-plugin/` directory
at the monorepo root, declared as a standalone package via an
empty `[workspace]` table in its own `Cargo.toml` (mirrors the
L3 #46 `#pheno-errors` decision and the L3 #47/L3 #48/L3 #49
follow-ups — keeps the new crate's test/build loop independent
of the 56-crate root workspace):

- `pheno-plugin/Cargo.toml` (29 lines) — package manifest,
  `[workspace]` table for standalone, `thiserror = "2.0"` dep.
- `pheno-plugin/src/lib.rs` (232 lines) — the `Plugin` trait
  (object-safe), `PluginError` enum (thiserror, two tuple
  variants), and `PluginRegistry` struct (with the 5 spec'd
  methods: `new`, `register`, `get`, `names`, `init_all`,
  plus a manual `Default` impl to satisfy
  `clippy::new_without_default`).
- `pheno-plugin/tests/registry_test.rs` (171 lines) — the 6
  spec'd integration tests (`registry_starts_empty`,
  `register_adds_plugin`, `register_rejects_duplicate`,
  `get_returns_registered_plugin`,
  `init_all_invokes_each_plugin_init`,
  `names_returns_sorted`) plus the `CountingPlugin` and
  `FailingPlugin` test fixtures.

**Public API (verbatim from the L3 #57 spec):**

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn init(&self) -> Result<(), PluginError> { Ok(()) }
}

pub enum PluginError {
    DuplicateName(String),
    InitFailed(String),
}

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, p: Box<dyn Plugin>) -> Result<(), PluginError>;
    pub fn get(&self, name: &str) -> Option<&dyn Plugin>;
    pub fn names(&self) -> Vec<String>; // sorted ascending
    pub fn init_all(&self) -> Result<(), PluginError>;
}
```

**Object-safety rationale:** The `Plugin` trait is object-safe by
construction — no associated types, no generic methods, only
`&self` receivers, `Send + Sync` super-traits. This is the
load-bearing invariant that makes `Box<dyn Plugin>` storage
possible, which is in turn the load-bearing invariant for
runtime plugin loading from crates the host does not statically
depend on (i.e., the whole point of a plugin system).

**Name capture semantics:** The `HashMap` key is the plugin's
`Plugin::name()` value at registration time. Subsequent renames
of the same `Box<dyn Plugin>` are NOT reflected (the name is
captured into an owned `String` at `register`-time). This
matches the L3 #57 spec verbatim and the
`focus-plugin-sdk`/`phenotype-registry` naming conventions.

**Bulk-init semantics:** `init_all` iterates the registered
plugins in registration (insertion) order and short-circuits on
the first `PluginError`. Each plugin's `init()` returns
`Result<(), PluginError>`, so failures are propagated directly
via the `?` operator without re-wrapping. The `?` flow is
possible because the spec's `PluginError::InitFailed(String)` is
a tuple variant (not a struct variant with separate `name` +
`reason` fields) — so `plugin.init()?` just hands the error up
unchanged.

**Tests (8 total, all passing):**

- 6 integration tests in `tests/registry_test.rs`:
  - `registry_starts_empty` — fresh registry has no plugins
    (`get("anything")` returns `None`, `names()` is empty)
  - `register_adds_plugin` — `register` round-trips through
    `get` and `names`; name+version preserved
  - `register_rejects_duplicate` — second `register` under
    the same name returns `DuplicateName`, first registration
    wins (version unchanged)
  - `get_returns_registered_plugin` — `get` returns the
    registered plugin for known names, `None` for unknown
  - `init_all_invokes_each_plugin_init` — `init_all` dispatches
    `init` exactly once per registered plugin (verified with
    `Arc<AtomicUsize>` counters on `CountingPlugin`)
  - `names_returns_sorted` — `names()` returns sorted
    ascending, independent of registration order (registers
    `zeta, alpha, mu, beta` and asserts `alpha, beta, mu, zeta`)
- 2 doctests in `src/lib.rs`:
  - module-level `EchoPlugin` example (register + init_all
    happy path)
  - `PluginRegistry` `Alpha` example (get + names smoke)

**Verification (per the L3 #57 spec):**

- `cargo test` (from within `pheno-plugin/`): 6 integration +
  2 doctest = 8 passed; 0 failed; 0 ignored
- `cargo clippy --all-targets -- -D warnings` (from within
  `pheno-plugin/`): clean (0 warnings, 0 errors)
- `cargo fmt --check` (from within `pheno-plugin/`): clean

The spec's literal verification commands `cargo test -p
pheno-plugin` and `cargo clippy -p pheno-plugin --all-targets --
-D warnings` do NOT work from the monorepo root, because
pheno-plugin is a standalone crate (intentionally NOT a member
of the root `[workspace.members]`, per the L3 #46/#47/#48/#49
convention). The same commands without `-p` work correctly
from within the `pheno-plugin/` directory, or as `cargo test
--manifest-path pheno-plugin/Cargo.toml` from the root. Both
invocations produce 8/8 pass and clean clippy; the intent of
the spec (verify the crate builds and tests pass) is preserved.
This caveat is documented in the worklog
`worklogs/l3-57-pheno-plugin-registry-2026-06-11.json` under
`spec_deviations`.

**Spec alignment notes:**

- `PluginError::InitFailed` is a tuple variant `(String)` per
  the L3 #57 spec (NOT a struct variant with separate `name` +
  `reason` fields). The wrapped `String` is the plugin's own
  init-failure reason (typically the plugin's error type
  rendered via `Display`). The registry's `init_all` propagates
  `PluginError` directly via `?` (no re-wrap with the plugin
  name, since the loop variable in `init_all` is the proximate
  context for the operator).
- Integration tests are in `tests/registry_test.rs` per spec,
  with all 6 required test names present verbatim.
- An additional manual `impl Default for PluginRegistry` was
  added (delegating to `new()`) to satisfy
  `clippy::new_without_default` under `-D warnings`. This is a
  standard-Rust trait impl, not a new public method, and is
  required for the spec's `cargo clippy -- -D warnings` to
  pass.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #49
  pheno-otel, L3 #50 pheno-cli-base, L3 #51 pheno-fastapi-base,
  L3 #52 pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags).
- **Did NOT push to origin.** Branch is
  `chore/l3-57-pheno-plugin-registry-2026-06-11`, off `main` (1
  commit ahead).
- **No async runtime pulled in by the new crate.** The public
  API is synchronous; the `init` hook is intentionally
  non-`async` because plugin initialization is expected to be
  cheap (load config, register handlers, log a "ready" line).
  Anything heavier belongs inside a separate `start`/`run`
  method that the host can drive asynchronously after
  `init_all` succeeds — this is a deliberate design choice
  documented in the trait's doc comment.
- **No `uniffi`, no FFI.** `pheno-plugin` is the in-process
  Rust-only sibling of `focus-plugin-sdk` (the uniffi-facing
  FFI SDK); they are intentionally separate crates to keep the
  cold-compile path and dep surface of each narrow.

**Downstream:** L5 #88 (`helioscli` integration) will pick up
`helios-plugin-*` crates at startup, `register` them into a
`PluginRegistry`, and call `init_all` before serving the first
command. Any other L5 pheno-* host that wants a uniform plugin
entrypoint can do the same one-liner:
`let _ = registry.init_all()?;`. The L5 service crates can then
hold the registry in a `OnceLock<PluginRegistry>` and hand
`&dyn Plugin` references to TUI + worker threads (the
`Send + Sync` super-traits make this trivial).

**Consolidation targets:** `focus-plugin-sdk`
(`crates/focus-plugin-sdk`) is the uniffi-facing FFI SDK that
exposes plugins across a Swift/Kotlin boundary — it is too
heavy (depends on `uniffi`, a connector surface, `tokio`
runtime plumbing) for an in-process Rust-only registry, so
`pheno-plugin` is the canonical Rust-only sibling. The
`phenotype-registry` (`phenotype-registry/`) is a
JSON-Schema-driven *provider* registry (a different shape of
thing — config-driven providers with discovery, not in-process
plugins) and is left in place. `pheno-plugin` is the third
path, not a replacement for either.

### L3-#50 (pheno-cli-base)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-cli-base`
Rust crate — a thin facade over `clap` v4 (derive) and `colored`
v2 that gives every downstream Pheno CLI binary (L4 #71
`helioscli`, L5 #88 helioscli integration, plus any future
pheno-*-cli) a uniform shape: `CliRunnable` trait with mandatory
`run(&self) -> Result<(), AppError>` + default `main(&self) -> !`
that maps `AppError` to colored stderr and exits 1;
`install_panic_hook()` that prints panic+backtrace to stderr
and exits 1; `parse_from_env_or_exit::<T: clap::Parser>() -> T`
that renders colored clap usage on parse failure and exits 2.
The crate re-exports `clap` so derived structs do not need a
direct clap dep. The 3 exit codes (0/1/2) follow standard Unix
convention so CI and supervisors can distinguish user errors
from tool errors.

**Crate layout:** Five files in a new `pheno-cli-base/` directory
at the monorepo root, declared as a standalone package via an
empty `[workspace]` table in its own `Cargo.toml` (mirrors the
L3 #46/`#47`/`#48`/`#49`/`#57` convention — NOT a member of the
~56-crate root `Cargo.toml` `[workspace.members]`). Files:

| Path | Lines | Purpose |
|---|---:|---|
| `pheno-cli-base/Cargo.toml`          |  29 | Package manifest + clap 4 (derive) + colored 2 + thiserror 2.0 + empty `[workspace]` table |
| `pheno-cli-base/src/lib.rs`          | 372 | `pub use clap;` re-export, `CliRunnable` trait (object-safe), `install_panic_hook` (Once-guarded), `parse_from_env_or_exit` helper, `format_app_error` helper |
| `pheno-cli-base/src/error.rs`        | 197 | `AppError` (6-variant thiserror enum: Validation/NotFound/Storage/Config/Domain/Io) + 6 inline `#[test]`s |
| `pheno-cli-base/src/bin/cli_smoke.rs` | 161 | In-crate smoke binary that drives the integration tests through real OS process boundaries (assert_cmd) + 8 inline `#[test]`s |
| `pheno-cli-base/tests/cli_test.rs`   | 251 | 5 integration tests (the spec's `>=5` floor) using assert_cmd + predicates |

**Public API (re-exported from `pheno_cli_base::`):**

```rust
// Re-export of clap v4 (derive feature) so derived structs do
// not need a direct clap dep.
pub use clap;

pub trait CliRunnable {
    fn run(&self) -> Result<(), AppError>;
    fn main(&self) -> ! {
        // Default: call self.run(); on Ok(()) exit 0, on Err
        // print colored `error: <msg>` to stderr and exit 1.
    }
}

pub fn install_panic_hook();
pub fn parse_from_env_or_exit<T: clap::Parser>() -> T;
```

**`AppError` (6 variants, thiserror derive — local stub mirroring
the L3 #46 spec verbatim):**

- `Validation(String)` — semantic validation failure
- `NotFound(String)` — resource lookup miss
- `Storage(String)` — persistence-layer failure
  (`#[from] std::io::Error` so `?` Just Works from `std::fs`,
  `std::net`, etc.)
- `Config(String)` — configuration parse/load failure
- `Domain(String)` — catch-all business-logic failure
- `Io(String)` — I/O failure variant (alias-style; also
  `#[from] std::io::Error`)

`AppError: std::error::Error + Send + Sync + 'static` (via
thiserror). The 6 inline unit tests in `src/error.rs` cover
constructors, `Display` messages, `From<std::io::Error>` round
trips, and a tripwire test that asserts the `std::error::Error`
impl is in place.

**Exit code contract:**

- `0` — success (`CliRunnable::main` on `Ok(())`, or the smoke
  binary after parse + run)
- `1` — `AppError` (`CliRunnable::main` on `Err`) or uncaught
  panic (`install_panic_hook`)
- `2` — clap parse error (`parse_from_env_or_exit`)

This matches the standard Unix CLI convention and lets
callers (CI, scripts, supervisors) distinguish user errors
from tool errors.

**Color contract:** Every path that writes to stderr forces
`colored::control::set_override(true)` before writing, so the
output is colored even in TTY-less environments (CI, captured
test stderr, log scrapers). All other paths in the crate are
no-op w.r.t. the global color state.

**Test coverage (17/17 pass under `cargo test --offline`):**
The 5 spec-required integration tests are all present in
`tests/cli_test.rs` by exact name:

| # | Test | What it checks |
|--:|------|----------------|
| 1 | `cli_runnable_default_main_runs`               | `CliRunnable::main()` on a successful `run()` returns and (via the smoke binary) exits 0 with the expected stdout |
| 2 | `install_panic_hook_does_not_panic_on_normal_exit` | Calling `install_panic_hook()` and then a normal exit is a no-op: smoke binary exits 0, no backtrace is printed |
| 3 | `parse_from_env_or_exit_parses_valid_args`    | `parse_from_env_or_exit` on a known-good argv returns the parsed struct; smoke binary echoes `name=<name>` and exits 0 |
| 4 | `parse_from_env_or_exit_exits_on_missing_required` | Omitting the required `--name` flag makes the smoke binary exit 2 with colored usage containing `error:` and the binary name on stderr |
| 5 | `app_error_to_stderr_message_is_colored`      | The `AppError` formatter emits an ANSI-red `error: <msg>` line on stderr when `CliRunnable::main` encounters a `Domain` error |

Plus 8 inline unit tests in `src/bin/cli_smoke.rs` (the
smoke binary's own `#[cfg(test)]` block — covers every argv
permutation the integration suite depends on, plus an
`AppError` Display + From impls round-trip) and 4 doctests
in `src/lib.rs` (module-level `pub use clap` re-export;
`CliRunnable` end-to-end `MyCli` example; `install_panic_hook`
example; `parse_from_env_or_exit` example). The integration
tests use `assert_cmd` + `predicates` to drive the smoke
binary through real OS process boundaries (not in-process
function calls) so the exit codes and stderr streams are
exercised end-to-end.

**Test isolation:** The integration tests use
`predicates::str::contains(...).from_utf8()` to assert on the
colored `error: <msg>` substring on stderr, and
`assert_cmd::cargo::CargoError` for the binary's exit code.
The smoke binary itself is a real cargo binary; the
integration tests run it via `Command::cargo_bin("cli_smoke")`
which uses `CARGO_BIN_EXE_<name>` to find the compiled
binary. The `app_error_to_stderr_message_is_colored` test
asserts the ANSI escape sequence (`\x1b[`) appears in the
stderr output, locking in the color contract.

**Deps resolution:** All deps (clap 4, colored 2, thiserror
2.0, assert_cmd 2, predicates 3) resolved cleanly from
`~/.cargo/registry/cache`. `cargo test --offline` runs in
~0.5s for the integration tests + ~0.1s for doctests, with no
5-minute resolver timeout.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #49
  pheno-otel, L3 #51 pheno-fastapi-base, L3 #52
  pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags, L3 #57 pheno-plugin).
- **Did NOT push to origin.** Branch is
  `chore/l3-50-pheno-cli-base-2026-06-11`, off `main`
  (1 commit ahead).
- **No async runtime pulled in by the new crate.** The
  public API is fully synchronous; the panic hook is
  sync-only; `parse_from_env_or_exit` is a pure
  argv-to-struct function. clap itself is
  async-runtime-free.
- **Object safety.** `CliRunnable` is object-safe (no
  associated types, no generic methods, only `&self`
  receivers), so downstream crates can store
  `Box<dyn CliRunnable>` in a multi-subcommand dispatcher.

**Object-safety rationale:** Same as L3 #57 — the trait has
no associated types, no generic methods, only `&self`
receivers. This is the load-bearing invariant that makes
`Box<dyn CliRunnable>` storage possible, which is in turn
the load-bearing invariant for multi-subcommand CLIs that
want to store heterogeneous subcommands in a single registry
and dispatch dynamically.

**`install_panic_hook` re-entrance:** Uses
`std::sync::Once::call_once(...)` so repeat invocations are a
no-op — the first call wins. The hook itself formats
`<thread-name>: <message>` to stderr (one line) followed by
the backtrace (one line, only if `RUST_BACKTRACE=1` is set in
the environment), then calls `std::process::exit(1)`. The
panic hook is installed *at most once* per process, so
downstream CLIs can call `install_panic_hook()` at the top of
their `fn main()` without worrying about double-install.

**`parse_from_env_or_exit` design:** Reads
`std::env::args_os()` directly (NOT `std::env::args()`) so
binary names with non-UTF8 bytes are preserved (this matches
clap's own `Command::get_matches_from` behavior). On parse
failure, the helper forces `colored::control::set_override(true)`
before rendering clap's usage, so the usage block is colored
even when stderr is redirected to a pipe. The helper then
calls `clap::Error::exit()` which writes the colored usage
+ the error to stderr and exits with the clap-canonical
exit code (`2` for usage errors).

**Why a local `AppError` stub (not a path dep on
`pheno-errors/`):** The L3 #46 spec called for
`pheno_errors::AppError` as the error type. L3 #46's
`pheno-errors/` crate does not exist on `main` as of
2026-06-11 — only a parallel worktree has it, and depending
on a sibling worktree's path is not safe in CI. Per the L3
#50 spec's explicit permission ("or use a stub error type if
path dep breaks — document the choice in the worklog"),
this crate defines a local `AppError` stub using
`thiserror` 2.0 that mirrors the L3 #46 spec verbatim
(6 variants, all `String`-tuple except `Storage`/`Io` which
use `#[from] std::io::Error`). When L3 #46 lands on `main`,
the planned migration is a single-file change: replace
`src/error.rs` with `pub use pheno_errors::AppError;` and
update `Cargo.toml` to a path dep on `../pheno-errors`. The
public API of `pheno-cli-base` will not change. This is
documented in the worklog
`worklogs/l3-50-pheno-cli-base-2026-06-11.json` under
`deviation_from_spec_pheno_errors_dep` and
`spec_deviations`.

**Verification (per the L3 #50 spec):**

- `cargo test -p pheno-cli-base`: 5 integration + 8 unit +
  4 doctest = 17 passed; 0 failed; 0 ignored
- `cargo clippy -p pheno-cli-base --all-targets -- -D warnings`:
  clean (0 warnings, 0 errors)
- `cargo fmt --check`: clean

The `-p pheno-cli-base` invocation works from the monorepo
root *only* when the path is registered as a workspace
member, OR via `--manifest-path pheno-cli-base/Cargo.toml`.
Since pheno-cli-base is a standalone crate (intentionally
NOT a member of the root `[workspace.members]`, per the L3
#46/#47/#48/#49/#57 convention), the same commands without
`-p` work correctly from within the `pheno-cli-base/`
directory, or as `cargo test --manifest-path
pheno-cli-base/Cargo.toml` from the root. Both invocations
produce 17/17 pass and clean clippy; the intent of the spec
(verify the crate builds and tests pass) is preserved. This
caveat is documented in the worklog under `spec_deviations`.

**Downstream:** L4 #71 (helioscli Rust CLI base) will
implement `CliRunnable` on a subcommand enum and call
`install_panic_hook()` at startup. L5 #88 (helioscli
integration) will use `parse_from_env_or_exit` in the
binary's `main()`. Any future pheno-*-cli binary (e.g., a
pheno-config CLI, a pheno-otel CLI for the dev-time
stdout-export path) gets the same uniform exit-code
contract for free by implementing `CliRunnable` and calling
the three helpers.

**Consolidation targets:** `thegent-cli`'s clap plumbing
(`thegent/src/cli/`) is a similar shape — it also re-exports
clap, parses argv into a subcommand enum, and runs a
`main()` per subcommand — but thegent is in a different repo
and uses a different convention (structopt, no panic hook,
no color contract). `pheno-cli-base` is the canonical
focalpoint-monorepo version, with the additional
ergonomics (panic hook, color override, exit-code contract)
that thegent would benefit from but does not yet have. A
follow-up could backport the helpers into thegent; for now
the two are intentionally separate.

---

## Phase 8: Cross-Repo + Side DAG + Quality SOTA Sweep (2026-06-11)

### 60 new background agents dispatched
- **agent-sd-batch1** (20 SD tasks): SOTA research, cross-repo libification,
  build system modernization (Make->just/Taskfile), agent-friendly docs
- **agent-cc-batch1** (20 CC tasks): cross-cutting observability (OTel),
  error handling (pheno-error adoption), test runner unification, security
  scanning (cargo-audit, govulncheck, npm audit)
- **agent-qc-batch1** (20 QC tasks): pre-commit configs, release-plz/GoReleaser,
  coverage reporting (llvm-cov, Codecov), dependency update workflows

### Total active agent work
```
BATCH              TASKS  MODEL      REASONING
--------------------------------------------------
agent-l1-batch2    10     gpt-5.4    low  (workspace-write)
agent-l1-batch2-r  3      gpt-5.4    low  (workspace-write, retry)
agent-l2-l5        40     gpt-5.4    low  (workspace-write)
agent-sd-batch1    20     gpt-5.4    low  (workspace-write)
agent-cc-batch1    20     gpt-5.4    low  (workspace-write)
agent-qc-batch1    20     gpt-5.4    low  (workspace-write)
--------------------------------------------------
TOTAL              113 agents dispatched
```

All running in parallel worktrees (one per agent). Each agent commits
to a dedicated branch `chore/<TID>-sota-2026-06-11` in the focus repo
and writes a canonical-form worklog JSON.

### Key behavioral note
The `gpt-5.4` (gpt-5.1-codex-mini successor) tier with `low` reasoning
is the only tier that consistently finishes real work. The `gpt-5.5`
tier (default) hit credit ceiling early in the session. Future
sessions should use this tier for batch dispatch.

### What this batch delivers (per repo)
- **AgilePlus**: pre-commit + clippy + cargo-deny + cargo-audit + llvm-cov
  + release-plz + cargo-update + pheno-error + pheno-domain + OTel
- **PlayCua**: pre-commit + cargo-deny + cargo-audit + llvm-cov +
  release-plz + cargo-update + pheno-error + pheno-capture-port +
  pheno-runtime + CapturePort trait + WebDriver adapter + ndarray
  screenshot encoding
- **nanovms**: pre-commit + golangci-lint + govulncheck + go-test-coverage
  + GoReleaser + dependabot (gomod/github-actions/docker) + OTel +
  pheno-syscall + pheno-process + mockall syscalls + slog/tracing JSON
  + snapshot cleanup
- **BytePort**: pre-commit + cargo-deny + cargo-audit + llvm-cov +
  release-plz + cargo-update + pheno-error + pheno-upload +
  pheno-telemetry + Wry/WebKit retry middleware + Tauri feature flags
  + testcontainers integration + benchmark suite + clap CLI
- **PhenoCompose**: pre-commit + prettier/eslint/tsc + npm audit + OSV +
  semantic-release + dependabot (npm/github-actions/docker) + vitest
  + vitepress search + VitePress typed config + pheno-docs-config +
  pheno-binding-gen + Rust FFI shims + CONTRIBUTING.md
- **Cross-repo SOTA (SD2)**: pheno-fs, pheno-capture, pheno-syscall,
  pheno-config, pheno-upload libification candidates
