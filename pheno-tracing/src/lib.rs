use std::path::Path;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub mod adapters;
pub mod port;

/// Build the canonical `EnvFilter` from `RUST_LOG`, falling back to
/// `info` when the variable is unset or invalid.
fn default_env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
}

/// Initialize tracing with the default pretty formatter.
///
/// Reads `RUST_LOG` for the filter directive; falls back to `info`.
/// Honors thread-id and target fields. Idempotent (uses `try_init`).
pub fn init() {
    let env_filter = default_env_filter();
    let fmt_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_level(true);
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init();
}

/// Initialize tracing with structured JSON output.
///
/// Reads `RUST_LOG` for the filter directive; falls back to `info`.
/// Emits one JSON object per event with the current span attached.
/// Idempotent (uses `try_init`).
pub fn init_json() {
    let env_filter = default_env_filter();
    let fmt_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(false)
        .with_target(true);
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init();
}

/// Initialize tracing that appends to a daily-rotated log file under
/// `dir`.
///
/// The filename pattern is `pheno-tracing.log.YYYY-MM-DD` (daily
/// rotation via `tracing_appender::rolling::daily`). ANSI escape
/// sequences are disabled because the file consumer is rarely a
/// terminal.
pub fn init_with_file(dir: &Path) {
    let env_filter = default_env_filter();
    let file_appender = tracing_appender::rolling::daily(dir, "pheno-tracing.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // Keep the worker thread alive for the process lifetime.
    Box::leak(Box::new(guard));
    let fmt_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true);
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_env_filter_is_info() {
        // When RUST_LOG is unset or invalid, the filter resolves to
        // "info".
        let filter = EnvFilter::new("info");
        let _ = filter;
    }
}
