use phenotype_error_core::Result;
use serde_json::Value;
use std::collections::HashMap;

/// Semantic version for events.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EventVersion {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl EventVersion {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn initial() -> Self {
        Self::new(1, 0, 0)
    }
}

impl std::fmt::Display for EventVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Trait for event upcasters.
/// An upcaster transforms an event from an older schema version to a newer one.
pub trait EventUpcaster: Send + Sync {
    fn upcast(&self, version: EventVersion, payload: Value) -> Result<Value>;
    fn target_version(&self) -> EventVersion;
    fn event_type(&self) -> &str;
}

/// Registry of upcasters for multiple event types.
#[derive(Default)]
pub struct UpcasterRegistry {
    upcasters: HashMap<(String, EventVersion), Box<dyn EventUpcaster>>,
}

impl std::fmt::Debug for UpcasterRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpcasterRegistry")
            .field("upcaster_count", &self.upcasters.len())
            .finish()
    }
}

impl UpcasterRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, upcaster: Box<dyn EventUpcaster>) {
        let key = (upcaster.event_type().to_string(), upcaster.target_version());
        self.upcasters.insert(key, upcaster);
    }

    pub fn upcast(&self, event_type: &str, version: EventVersion, payload: Value) -> Result<Value> {
        let key = (event_type.to_string(), version.clone());
        if let Some(upcaster) = self.upcasters.get(&key) {
            upcaster.upcast(version, payload)
        } else {
            Ok(payload)
        }
    }
}
