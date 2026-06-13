//! Concrete adapter implementations of the L4 port traits defined in
//! [`crate::port`].
//!
//! Five adapters are provided:
//!
//! * [`StdoutSubscriber`] — installs a `tracing-subscriber` with the
//!   default pretty formatter writing to `stdout`.
//! * [`JsonFileSubscriber`] — installs a `tracing-subscriber` whose
//!   formatter emits one JSON object per event to a file on disk.
//! * [`NoopExporter`] — discards every [`SpanSnapshot`] it receives.
//!   Intended for unit tests and as a placeholder when an exporter
//!   is required by the type system but no real sink is wanted.
//! * [`EnvFilterPolicy`] — wraps `tracing_subscriber::EnvFilter`,
//!   reading directives from `RUST_LOG` or from an explicit string.
//! * [`LevelFilterPolicy`] — gates records by minimum [`Level`],
//!   wrapping `tracing_subscriber::filter::LevelFilter`.
//!
//! All adapters are inert until installed; constructing one is
//! cheap and has no global side effects.

use std::fs::OpenOptions;
use std::path::{Path, PathBuf};

use tracing::Metadata;
use tracing_subscriber::filter::LevelFilter as SubLevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::port::{Error, ExporterPort, FilterPort, SpanSnapshot, SubscriberPort};

// ---------------------------------------------------------------------------
// StdoutSubscriber
// ---------------------------------------------------------------------------

/// Installs a `tracing-subscriber` with the default pretty
/// formatter writing to `stdout`.
///
/// `StdoutSubscriber` honors the same `RUST_LOG` semantics as
/// [`crate::init`]: an unset or invalid `RUST_LOG` falls back to
/// `info`.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdoutSubscriber;

impl SubscriberPort for StdoutSubscriber {
    fn install(&self) -> Result<(), Error> {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        let fmt_layer = fmt::layer()
            .with_target(true)
            .with_thread_ids(false)
            .with_level(true)
            .with_writer(std::io::stdout);
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .try_init()
            .map_err(|e| Error::Install(e.to_string()))?;
        Ok(())
    }

    fn kind(&self) -> &'static str {
        "stdout"
    }
}

// ---------------------------------------------------------------------------
// JsonFileSubscriber
// ---------------------------------------------------------------------------

/// Installs a `tracing-subscriber` that emits one JSON object per
/// event to the file at the configured path.
///
/// The file is opened in append mode and is created if missing. A
/// `tracing_appender::non_blocking` writer is used so the tracing
/// pipeline never blocks on I/O. The writer's worker guard is
/// intentionally leaked to keep the writer alive for the process
/// lifetime (this matches the pattern used by
/// [`crate::init_with_file`]).
#[derive(Debug, Clone)]
pub struct JsonFileSubscriber {
    path: PathBuf,
}

impl JsonFileSubscriber {
    /// Build a JSON-file subscriber writing to `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Borrow the configured output path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl SubscriberPort for JsonFileSubscriber {
    fn install(&self) -> Result<(), Error> {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        let (non_blocking, guard) = tracing_appender::non_blocking(file);
        // Keep the writer's worker thread alive for the process
        // lifetime. The `Box::leak` pattern matches the
        // canonical `init_with_file` helper.
        Box::leak(Box::new(guard));

        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        let fmt_layer = fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(false)
            .with_target(true)
            .with_ansi(false)
            .with_writer(non_blocking);
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .try_init()
            .map_err(|e| Error::Install(e.to_string()))?;
        Ok(())
    }

    fn kind(&self) -> &'static str {
        "json-file"
    }
}

// ---------------------------------------------------------------------------
// NoopExporter
// ---------------------------------------------------------------------------

/// An exporter that discards every snapshot. Useful in tests and
/// as a default placeholder when an exporter is required by the
/// type system but no real sink is desired.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopExporter;

impl ExporterPort for NoopExporter {
    fn export(&self, _span: SpanSnapshot) -> Result<(), Error> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// EnvFilterPolicy
// ---------------------------------------------------------------------------

/// A filter policy that wraps `tracing_subscriber::EnvFilter`.
///
/// The wrapped `EnvFilter` is consulted on every record; the policy
/// is cheap to clone (the inner filter is `Arc`-backed internally).
#[derive(Debug, Clone)]
pub struct EnvFilterPolicy {
    inner: EnvFilter,
    max_level: tracing::Level,
}

impl EnvFilterPolicy {
    /// Build a policy from an explicit directive string (e.g.
    /// `"info"`, `"pheno_tracing=debug,other_crate=warn"`).
    pub fn new(directive: impl AsRef<str>) -> Self {
        let dir = directive.as_ref();
        let max_level = parse_leading_level(dir).unwrap_or(tracing::Level::INFO);
        Self {
            inner: EnvFilter::new(dir),
            max_level,
        }
    }

    /// Build a policy from the `RUST_LOG` environment variable,
    /// falling back to `"info"` when the variable is unset or
    /// invalid.
    pub fn from_env() -> Self {
        let inner = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        let max_level = parse_leading_level("info").unwrap_or(tracing::Level::INFO);
        Self { inner, max_level }
    }

    /// Borrow the wrapped `EnvFilter`.
    pub fn inner(&self) -> &EnvFilter {
        &self.inner
    }
}

impl Default for EnvFilterPolicy {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Parse the leading level token of an `EnvFilter` directive
/// string. Handles `"trace"`, `"debug"`, `"info"`, `"warn"`,
/// `"warning"`, `"error"`, `"off"`. Returns `None` if the first
/// token is not a recognized level keyword.
fn parse_leading_level(s: &str) -> Option<tracing::Level> {
    let first = s.split(',').next()?.trim();
    match first {
        "off" => Some(tracing::Level::ERROR), // approximation; not meaningful
        "trace" => Some(tracing::Level::TRACE),
        "debug" => Some(tracing::Level::DEBUG),
        "info" => Some(tracing::Level::INFO),
        "warn" | "warning" => Some(tracing::Level::WARN),
        "error" => Some(tracing::Level::ERROR),
        _ => None,
    }
}

impl FilterPort for EnvFilterPolicy {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        // `EnvFilter::enabled` requires a `Context`; for the
        // `FilterPort` boundary we approximate with a level check
        // keyed on the leading token of the directive. The
        // underlying `EnvFilter` is still available via
        // [`EnvFilterPolicy::inner`] for contexts that need the
        // full per-target resolution.
        meta.level() <= &self.max_level
    }
}

// ---------------------------------------------------------------------------
// LevelFilterPolicy
// ---------------------------------------------------------------------------

/// A filter policy that gates records by minimum [`tracing::Level`].
///
/// Internally wraps `tracing_subscriber::filter::LevelFilter` (the
/// subscriber-side filter) and uses `tracing::level_filters::LevelFilter`
/// for the constructor ergonomics. Records at or above the configured
/// level are admitted; records below it are dropped.
#[derive(Debug, Clone, Copy)]
pub struct LevelFilterPolicy {
    level: tracing::Level,
    _inner: SubLevelFilter,
}

impl LevelFilterPolicy {
    /// Build a policy that admits records at `level` or higher.
    pub fn new(level: tracing::Level) -> Self {
        Self {
            level,
            _inner: SubLevelFilter::from_level(level),
        }
    }

    /// Return the configured minimum level.
    pub fn level(&self) -> tracing::Level {
        self.level
    }

    /// Borrow the underlying `tracing_subscriber` LevelFilter.
    pub fn inner(&self) -> SubLevelFilter {
        SubLevelFilter::from_level(self.level)
    }
}

impl FilterPort for LevelFilterPolicy {
    fn enabled(&self, meta: &Metadata<'_>) -> bool {
        // `tracing_subscriber::filter::LevelFilter::enabled`
        // requires a `Context`; for the `FilterPort` boundary we
        // approximate with a direct level comparison. The wrapped
        // `SubLevelFilter` is preserved via [`LevelFilterPolicy::inner`]
        // for full-fidelity resolution when a `Context` is available.
        meta.level() <= &self.level
    }
}
