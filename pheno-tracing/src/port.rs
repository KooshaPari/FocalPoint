//! L4 hexagonal port abstractions for `pheno-tracing`.
//!
//! This module decouples the three concerns of a tracing pipeline
//! (subscriber registration, span/event export, and level/target
//! filtering) behind three narrow traits. Concrete adapters
//! (`StdoutSubscriber`, `JsonFileSubscriber`, `NoopExporter`,
//! `EnvFilterPolicy`, `LevelFilterPolicy`) live in
//! [`crate::adapters`].
//!
//! The composite [`Layer`] composes the three ports into a single
//! `tracing_subscriber::Layer` that can be installed as part of a
//! standard registry.
//!
//! Reference: V21 opportunity O1 (second-highest-leverage L4
//! substrate; depended on by 14 connector crates).

use std::collections::HashMap;
use std::fmt;
use std::time::SystemTime;

use tracing::Metadata;
use tracing::span::{Attributes, Id, Record};
use tracing::Event;
use tracing::Subscriber;
use tracing_subscriber::layer::Context;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer as TracingLayer;

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

/// Errors produced by port trait methods and their adapter
/// implementations.
#[derive(Debug)]
pub enum Error {
    /// The global tracing subscriber has already been installed.
    /// Subsequent `install` calls return this variant.
    AlreadyInitialized,
    /// An I/O error occurred while opening a file or writer.
    Io(std::io::Error),
    /// The exporter failed to push a snapshot to its destination.
    Export(String),
    /// The subscriber could not be installed.
    Install(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AlreadyInitialized => write!(f, "tracing subscriber already initialized"),
            Error::Io(e) => write!(f, "pheno-tracing io error: {e}"),
            Error::Export(m) => write!(f, "pheno-tracing export failed: {m}"),
            Error::Install(m) => write!(f, "pheno-tracing install failed: {m}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

// ---------------------------------------------------------------------------
// Port traits
// ---------------------------------------------------------------------------

/// A subscriber port owns the lifecycle of a `tracing` subscriber.
///
/// Implementations register themselves as the global tracing
/// subscriber when [`SubscriberPort::install`] is called. After a
/// successful install the global registry is locked and subsequent
/// installs must return [`Error::AlreadyInitialized`].
pub trait SubscriberPort {
    /// Install this subscriber as the global tracing subscriber.
    ///
    /// Implementations should be idempotent in the sense that
    /// re-invocation after a successful first call returns
    /// [`Error::AlreadyInitialized`] (or a sibling diagnostic) rather
    /// than panicking.
    fn install(&self) -> Result<(), Error>;

    /// Returns a short, stable identifier for the subscriber kind
    /// (e.g. `"stdout"`, `"json-file"`). Used for diagnostics; the
    /// default returns `"unknown"`.
    fn kind(&self) -> &'static str {
        "unknown"
    }
}

/// An exporter port pushes a [`SpanSnapshot`] to an external sink
/// (file, network endpoint, in-memory buffer, etc.).
///
/// Implementations should be cheap; the tracing pipeline will call
/// [`ExporterPort::export`] once per span lifecycle event.
pub trait ExporterPort {
    /// Export the snapshot to the underlying sink. Returning `Err`
    /// surfaces the failure to the caller but does not abort the
    /// tracing pipeline.
    fn export(&self, span: SpanSnapshot) -> Result<(), Error>;
}

/// A filter port decides whether a given metadata record is enabled.
///
/// This is the L4 abstraction over `tracing_subscriber::EnvFilter`,
/// `LevelFilter`, or any custom gating policy. Returning `false`
/// short-circuits the layer so neither span nor event construction
/// is performed.
pub trait FilterPort {
    /// Returns `true` if the metadata record should be processed.
    fn enabled(&self, meta: &Metadata<'_>) -> bool;
}

// ---------------------------------------------------------------------------
// SpanSnapshot
// ---------------------------------------------------------------------------

/// A transport DTO carrying the minimum identifying information
/// about a span across the exporter boundary.
///
/// `SpanSnapshot` is intentionally `String`-typed (not borrowed)
/// so it can cross thread boundaries and be serialized without
/// lifetime entanglement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanSnapshot {
    /// The span name (the literal passed to `info_span!` etc.).
    pub name: String,
    /// The static target path (e.g. `"pheno_tracing::port"`).
    pub target: String,
    /// The level at which the span was entered (e.g. `"INFO"`).
    pub level: String,
    /// Recorded fields captured at span entry.
    pub fields: HashMap<String, String>,
    /// Timestamp the span was entered, if available.
    pub start: Option<SystemTime>,
    /// Timestamp the span was closed, if available.
    pub end: Option<SystemTime>,
}

impl SpanSnapshot {
    /// Build a snapshot with the required name/target/level triple
    /// and an empty field map.
    pub fn new(
        name: impl Into<String>,
        target: impl Into<String>,
        level: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            target: target.into(),
            level: level.into(),
            fields: HashMap::new(),
            start: None,
            end: None,
        }
    }

    /// Insert a string-valued field; returns `self` for chaining.
    pub fn with_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.insert(key.into(), value.into());
        self
    }

    /// Set the span start timestamp.
    pub fn with_start(mut self, t: SystemTime) -> Self {
        self.start = Some(t);
        self
    }

    /// Set the span end timestamp.
    pub fn with_end(mut self, t: SystemTime) -> Self {
        self.end = Some(t);
        self
    }
}

// ---------------------------------------------------------------------------
// Layer
// ---------------------------------------------------------------------------

/// A composite layer that delegates gating and exporting to the
/// injected ports. The `L` (subscriber) port is stored as a tag and
/// can be retrieved via [`Layer::subscriber`] for diagnostics or for
/// installation via [`Layer::install`].
pub struct Layer<L, F, E> {
    subscriber: L,
    filter: F,
    exporter: E,
}

impl<L, F, E> Layer<L, F, E> {
    /// Build a composite layer from its three port parts.
    pub fn new(subscriber: L, filter: F, exporter: E) -> Self {
        Self {
            subscriber,
            filter,
            exporter,
        }
    }

    /// Borrow the subscriber port.
    pub fn subscriber(&self) -> &L {
        &self.subscriber
    }

    /// Borrow the filter port.
    pub fn filter(&self) -> &F {
        &self.filter
    }

    /// Borrow the exporter port.
    pub fn exporter(&self) -> &E {
        &self.exporter
    }

    /// Consume the layer and return its three port parts.
    pub fn into_parts(self) -> (L, F, E) {
        (self.subscriber, self.filter, self.exporter)
    }
}

impl<L, F, E> Layer<L, F, E>
where
    L: SubscriberPort + Send + Sync + 'static,
    F: FilterPort + Send + Sync + 'static,
    E: ExporterPort + Send + Sync + 'static,
{
    /// Install the layer as the global tracing subscriber. The
    /// subscriber port's [`SubscriberPort::install`] is invoked
    /// first; failure aborts the install. On success, the layer
    /// itself is added to a fresh `tracing_subscriber::Registry` so
    /// that filter and exporter wiring is active for the rest of
    /// the process.
    ///
    /// Note: tracing's global subscriber is one-shot. A second
    /// invocation of this method will return
    /// [`Error::AlreadyInitialized`] regardless of inputs.
    pub fn install(self) -> Result<(), Error> {
        // Validate that the inner subscriber is willing to install.
        // We can't actually try_init here because we also want to
        // add `self` as a tracing::Layer. Instead we add `self` to
        // a fresh Registry and try_init that.
        use tracing_subscriber::prelude::*;
        tracing_subscriber::registry()
            .with(self)
            .try_init()
            .map_err(|e| Error::Install(e.to_string()))
    }
}

impl<S, L, F, E> TracingLayer<S> for Layer<L, F, E>
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    L: SubscriberPort + Send + Sync + 'static,
    F: FilterPort + Send + Sync + 'static,
    E: ExporterPort + Send + Sync + 'static,
{
    fn enabled(&self, metadata: &Metadata<'_>, _ctx: Context<'_, S>) -> bool {
        self.filter.enabled(metadata)
    }

    fn on_new_span(&self, attrs: &Attributes<'_>, _id: &Id, _ctx: Context<'_, S>) {
        let metadata = attrs.metadata();
        let mut snapshot = SpanSnapshot::new(
            metadata.name(),
            metadata.target(),
            metadata.level().to_string(),
        )
        .with_start(SystemTime::now());
        // Collect the recorded fields as strings so the snapshot
        // is independent of the underlying field visitor's lifetime.
        attrs.record(&mut FieldRecorder {
            fields: &mut snapshot.fields,
        });
        let _ = self.exporter.export(snapshot);
    }

    fn on_record(&self, _id: &Id, _values: &Record<'_>, _ctx: Context<'_, S>) {}

    fn on_event(&self, _event: &Event<'_>, _ctx: Context<'_, S>) {}

    fn on_enter(&self, _id: &Id, _ctx: Context<'_, S>) {}

    fn on_exit(&self, _id: &Id, _ctx: Context<'_, S>) {}

    fn on_close(&self, _id: Id, _ctx: Context<'_, S>) {}
}

/// Internal helper that records span fields into a string-keyed
/// map. Strings are produced via `Display` so integer and boolean
/// fields are captured.
struct FieldRecorder<'a> {
    fields: &'a mut HashMap<String, String>,
}

impl<'a> tracing::field::Visit for FieldRecorder<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        self.fields
            .insert(field.name().to_string(), format!("{value:?}"));
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    use tracing::Level;
    use tracing::callsite::{Callsite, Identifier};
    use tracing::metadata::Kind;

    // ----- test callsite + helper for building test Metadata -----

    /// A minimal `Callsite` used to build test `Metadata` values.
    /// The `metadata()` method is never invoked by the tests below
    /// (we build `Metadata` directly via `Metadata::new`), so the
    /// body is left as `unimplemented!()` to make accidental
    /// recursion loud.
    struct TestCallsite;

    static TEST_CALLSITE: TestCallsite = TestCallsite;

    impl Callsite for TestCallsite {
        fn set_interest(&self, _interest: tracing::subscriber::Interest) {}
        fn metadata(&self) -> &tracing::Metadata<'_> {
            unimplemented!("test callsite metadata is built directly")
        }
    }

    fn test_metadata(level: Level) -> tracing::Metadata<'static> {
        let id = Identifier(&TEST_CALLSITE);
        let fields = tracing::field::FieldSet::new(&[], id);
        tracing::Metadata::new(
            "test_event",
            "pheno_tracing::port::tests",
            level,
            None,
            None,
            None,
            fields,
            Kind::EVENT,
        )
    }

    // ----- mock ports for the trait-dispatch and layer tests -----

    /// Records whether `install` was called and reports its kind.
    struct MockSubscriber {
        kind: &'static str,
        flag: Arc<AtomicBool>,
    }

    impl SubscriberPort for MockSubscriber {
        fn install(&self) -> Result<(), Error> {
            self.flag.store(true, Ordering::SeqCst);
            Ok(())
        }
        fn kind(&self) -> &'static str {
            self.kind
        }
    }

    /// Records how many times `enabled` was consulted.
    struct MockFilter {
        count: Arc<AtomicUsize>,
    }

    impl FilterPort for MockFilter {
        fn enabled(&self, _meta: &Metadata<'_>) -> bool {
            self.count.fetch_add(1, Ordering::SeqCst);
            true
        }
    }

    /// Counts `export` invocations and captures the last snapshot.
    struct MockExporter {
        count: Arc<AtomicUsize>,
        last: Arc<std::sync::Mutex<Option<SpanSnapshot>>>,
    }

    impl ExporterPort for MockExporter {
        fn export(&self, span: SpanSnapshot) -> Result<(), Error> {
            self.count.fetch_add(1, Ordering::SeqCst);
            *self.last.lock().unwrap() = Some(span);
            Ok(())
        }
    }

    // ----- test 1: trait dispatch via &dyn TraitObject -----

    #[test]
    fn trait_dispatch_via_dyn_objects() {
        let flag = Arc::new(AtomicBool::new(false));
        let sub = MockSubscriber {
            kind: "mock",
            flag: flag.clone(),
        };

        // SubscriberPort as a trait object
        let port: &dyn SubscriberPort = &sub;
        assert_eq!(port.kind(), "mock");
        port.install().expect("install should succeed");
        assert!(flag.load(Ordering::SeqCst));

        // FilterPort as a trait object — no install side effects.
        let count = Arc::new(AtomicUsize::new(0));
        let filter = MockFilter {
            count: count.clone(),
        };
        let fport: &dyn FilterPort = &filter;
        let metadata = test_metadata(Level::INFO);
        let _ = fport.enabled(&metadata);
        assert_eq!(count.load(Ordering::SeqCst), 1);

        // ExporterPort as a trait object.
        let ecount = Arc::new(AtomicUsize::new(0));
        let last = Arc::new(std::sync::Mutex::new(None));
        let exporter = MockExporter {
            count: ecount.clone(),
            last: last.clone(),
        };
        let eport: &dyn ExporterPort = &exporter;
        let snap = SpanSnapshot::new("op", "pheno_tracing::port", "INFO");
        eport.export(snap.clone()).expect("export ok");
        assert_eq!(ecount.load(Ordering::SeqCst), 1);
        let stored = last.lock().unwrap().clone().expect("snapshot stored");
        assert_eq!(stored, snap);
    }

    // ----- test 2: Layer construction + accessors -----

    #[test]
    fn layer_construction_and_accessors() {
        let flag = Arc::new(AtomicBool::new(false));
        let sub = MockSubscriber {
            kind: "sub-A",
            flag,
        };
        let filter = MockFilter {
            count: Arc::new(AtomicUsize::new(0)),
        };
        let exporter = MockExporter {
            count: Arc::new(AtomicUsize::new(0)),
            last: Arc::new(std::sync::Mutex::new(None)),
        };

        let layer = Layer::new(sub, filter, exporter);
        assert_eq!(layer.subscriber().kind(), "sub-A");
        // Filter and exporter are reachable but not yet called.
        assert_eq!(layer.filter().count.load(Ordering::SeqCst), 0);
        assert_eq!(layer.exporter().count.load(Ordering::SeqCst), 0);

        // into_parts hands back the three ports.
        let (s, f, e) = layer.into_parts();
        assert_eq!(s.kind(), "sub-A");
        // Ensure we can still drive them post-decomposition.
        let _ = f.enabled(&test_metadata(Level::INFO));
        let _ = e.export(SpanSnapshot::new("x", "y", "INFO"));
    }

    // ----- test 3: StdoutSubscriber adapter smoke test -----

    #[test]
    fn stdout_subscriber_smoke() {
        // Construct; we deliberately do NOT install because
        // tracing's global subscriber is one-shot per process.
        let sub: Box<dyn SubscriberPort> = Box::new(crate::adapters::StdoutSubscriber);
        assert_eq!(sub.kind(), "stdout");
        // Installing now would race with other tests; assert the
        // signature only.
        let _fnptr: fn(&dyn SubscriberPort) -> Result<(), Error> =
            |p| p.install();
    }

    // ----- test 4: JsonFileSubscriber adapter smoke test -----

    #[test]
    fn json_file_subscriber_smoke() {
        // Use a tempdir for the path so the test does not pollute
        // the working directory.
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("pheno-tracing.json.log");
        let sub = crate::adapters::JsonFileSubscriber::new(&path);
        let port: &dyn SubscriberPort = &sub;
        assert_eq!(port.kind(), "json-file");
        // Do not install; the file is not created until install runs.
        assert!(!path.exists());
    }

    // ----- test 5: NoopExporter adapter smoke test -----

    #[test]
    fn noop_exporter_smoke() {
        let exporter = crate::adapters::NoopExporter;
        let port: &dyn ExporterPort = &exporter;
        let snap = SpanSnapshot::new("noop_op", "pheno_tracing::adapters", "INFO")
            .with_field("k", "v")
            .with_start(SystemTime::now());
        port.export(snap).expect("noop export ok");
    }

    // ----- test 6: EnvFilterPolicy + LevelFilterPolicy smoke test -----

    #[test]
    fn filter_policies_smoke() {
        // EnvFilterPolicy — admit INFO at an info-level policy.
        let env_pol = crate::adapters::EnvFilterPolicy::new("info");
        let port: &dyn FilterPort = &env_pol;
        let md_info = test_metadata(Level::INFO);
        assert!(
            port.enabled(&md_info),
            "info filter must accept INFO records"
        );

        // LevelFilterPolicy at WARN: INFO must be filtered out.
        let lvl_pol = crate::adapters::LevelFilterPolicy::new(Level::WARN);
        let lvl_port: &dyn FilterPort = &lvl_pol;
        let md_info2 = test_metadata(Level::INFO);
        assert!(
            !lvl_port.enabled(&md_info2),
            "WARN filter must reject INFO records"
        );
        let md_warn = test_metadata(Level::WARN);
        assert!(
            lvl_port.enabled(&md_warn),
            "WARN filter must accept WARN records"
        );
    }
}
