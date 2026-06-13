//! End-to-end integration test for the L4 hexagonal port layer.
//!
//! Composes a [`pheno_tracing::port::Layer`] from one mock exporter
//! and one mock filter (alongside the in-tree `StdoutSubscriber`)
//! and verifies that emitting a real `tracing` span flows through
//! the filter and is forwarded to the exporter.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use pheno_tracing::adapters::StdoutSubscriber;
use pheno_tracing::port::{ExporterPort, FilterPort, Layer, SpanSnapshot};

use tracing::Metadata;
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer as TracingLayer;

// ---------------------------------------------------------------------------
// Mock ports
// ---------------------------------------------------------------------------

/// Counts `enabled` consultations. Admits everything.
#[derive(Default)]
struct CountingFilter {
    count: Arc<AtomicUsize>,
}

impl FilterPort for CountingFilter {
    fn enabled(&self, _meta: &Metadata<'_>) -> bool {
        self.count.fetch_add(1, Ordering::SeqCst);
        true
    }
}

/// Captures every snapshot it receives and counts them.
#[derive(Default, Clone)]
struct CapturingExporter {
    count: Arc<AtomicUsize>,
    captured: Arc<Mutex<Vec<SpanSnapshot>>>,
}

impl ExporterPort for CapturingExporter {
    fn export(&self, span: SpanSnapshot) -> Result<(), Error> {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.captured.lock().unwrap().push(span);
        Ok(())
    }
}

// `Error` re-export to keep the impl block above name-resolution
// local to this test file.
use pheno_tracing::port::Error;

// ---------------------------------------------------------------------------
// Bridge: CapturingExporter must be reachable from a tracing Layer
// when installed. We achieve this by embedding a
// `pheno_tracing::port::Layer` directly into a custom Layer type
// that also writes to a `tracing_subscriber::fmt` writer pointing
// at a sink we control. That keeps stdout noise out of the test
// output while still exercising the full registry path.
// ---------------------------------------------------------------------------

use std::io::Write;
use std::sync::Mutex as StdMutex;

struct SinkWriter(Arc<StdMutex<Vec<u8>>>);

impl Write for SinkWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct SinkLayer(Arc<StdMutex<Vec<u8>>>);

impl<S> TracingLayer<S> for SinkLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: Context<'_, S>) {
        use tracing::field::Visit;
        struct V<'a>(&'a mut String);
        impl<'a> Visit for V<'a> {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                use std::fmt::Write;
                let _ = write!(self.0, "{}={:?} ", field.name(), value);
            }
        }
        let mut buf = String::new();
        event.record(&mut V(&mut buf));
        let line = format!("[{target}] {msg}\n", target = event.metadata().target(), msg = buf);
        let _ = writeln!(self.0.lock().unwrap(), "{}", line.trim_end());
    }
}

// ---------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------

#[test]
fn composite_layer_end_to_end() {
    // --- assemble mock ports ---
    let filter = CountingFilter::default();
    let exporter = CapturingExporter::default();

    // Capture the filter / exporter hit counts via the shared Arcs
    // so we can assert on them after emitting spans.
    let filter_count = filter.count.clone();
    let exporter_count = exporter.count.clone();
    let exporter_captured = exporter.captured.clone();

    // --- assemble composite Layer ---
    let sub = StdoutSubscriber;
    let layer = Layer::new(sub, filter, exporter);

    // --- install the composite ---
    // We install `layer` into a `tracing_subscriber::Registry` so
    // that the filter/exporter delegation becomes live. To keep
    // the test hermetic we do NOT call `layer.install()` (which
    // would also try to swap in a stdout writer); instead we add
    // the layer manually to a sink-backed registry.
    use tracing_subscriber::prelude::*;
    let sink_buf = Arc::new(StdMutex::new(Vec::<u8>::new()));
    let _sink = SinkWriter(sink_buf.clone());
    let sink_layer = SinkLayer(sink_buf.clone());

    // Wrap `layer` so it can be combined with the sink via a
    // Tuple Layer. `Layer<StdoutSubscriber, CountingFilter,
    // CapturingExporter>` is itself a `TracingLayer<Registry>`,
    // so we just pair it with the sink layer.
    let _ = tracing_subscriber::registry()
        .with(layer)
        .with(sink_layer)
        .try_init()
        .expect("registry try_init");

    // --- emit a span and an event ---
    {
        let span = tracing::info_span!("op_under_test", key = "value");
        let _enter = span.enter();
        tracing::info!(extra = 42, "hello from composite");
    }

    // Give the non-blocking writers a moment — but everything we
    // use is blocking, so this is just a safety net.
    std::thread::sleep(std::time::Duration::from_millis(10));

    // --- assertions ---
    assert!(
        filter_count.load(Ordering::SeqCst) >= 1,
        "filter must have been consulted at least once"
    );
    assert!(
        exporter_count.load(Ordering::SeqCst) >= 1,
        "exporter must have received at least one snapshot"
    );

    let captured = exporter_captured.lock().unwrap();
    assert!(!captured.is_empty(), "exporter captured zero snapshots");
    let snap = &captured[0];
    assert_eq!(snap.name, "op_under_test");
    assert_eq!(snap.level, "INFO");
    // The fields map is captured via the field visitor; the
    // "key" field should be present and stringified to `"value"`.
    assert_eq!(
        snap.fields.get("key").map(String::as_str),
        Some("\"value\""),
        "field 'key' must be present and debug-formatted"
    );

    // The sink-backed fmt layer should have observed the event
    // text, which proves the registry path is wired correctly.
    let sink_text = String::from_utf8(sink_buf.lock().unwrap().clone())
        .expect("sink text utf-8");
    assert!(
        sink_text.contains("hello from composite"),
        "sink must contain event text, got: {sink_text:?}"
    );
}
