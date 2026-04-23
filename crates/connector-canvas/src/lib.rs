//! Canvas LMS adapter (OAuth2, courses/assignments).

use focus_connectors::{Connector, ConnectorManifest};

pub struct CanvasConnector {
    manifest: ConnectorManifest,
}

impl CanvasConnector {
    pub fn new() -> Self {
        // Stub manifest
        Self {
            manifest: ConnectorManifest {
                id: "canvas".into(),
                version: "0.0.1".into(),
                display_name: "Canvas LMS".into(),
                auth_strategy: focus_connectors::AuthStrategy::OAuth2 {
                    scopes: vec!["url:GET|/api/v1/courses".into()],
                },
                sync_mode: focus_connectors::SyncMode::Polling { cadence_seconds: 900 },
                capabilities: vec![],
                entity_types: vec!["course".into(), "assignment".into()],
                event_types: vec![
                    "assignment_due".into(),
                    "assignment_graded".into(),
                    "course_enrolled".into(),
                ],
            },
        }
    }
}

impl Default for CanvasConnector {
    fn default() -> Self {
        Self::new()
    }
}

// Connector trait impl deferred until OAuth flow + HTTP client are wired.
#[async_trait::async_trait]
impl Connector for CanvasConnector {
    fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    async fn health(&self) -> focus_connectors::HealthState {
        focus_connectors::HealthState::Unauthenticated
    }

    async fn sync(
        &self,
        _cursor: Option<String>,
    ) -> focus_connectors::Result<focus_connectors::SyncOutcome> {
        Err(focus_connectors::ConnectorError::Auth("not implemented".into()))
    }
}

pub struct CanvasEntity;
pub struct CanvasEventMapper;
