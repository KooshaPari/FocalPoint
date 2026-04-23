//! Fixture replay + mock harness for connector authors.

use focus_connectors::Connector;

pub struct FixtureReplay;
pub struct MockSyncRunner;

pub struct ConnectorTestHarness<C: Connector> {
    pub connector: C,
}

impl<C: Connector> ConnectorTestHarness<C> {
    pub fn new(connector: C) -> Self {
        Self { connector }
    }
}
