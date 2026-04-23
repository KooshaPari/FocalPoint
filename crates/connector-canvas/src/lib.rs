//! Canvas LMS connector — OAuth2 auth, REST client, event mapping, `Connector` impl.

pub mod api;
pub mod auth;
pub mod events;
pub mod models;

use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;
use tracing::warn;
use uuid::Uuid;

use focus_connectors::{
    AuthStrategy, Connector, ConnectorError, ConnectorManifest, HealthState, Result, SyncMode,
    SyncOutcome,
};

use crate::api::CanvasClient;
use crate::auth::{CanvasOAuth2, InMemoryTokenStore, TokenStore};
use crate::events::CanvasEventMapper;

/// Canvas connector.
pub struct CanvasConnector {
    manifest: ConnectorManifest,
    account_id: Uuid,
    token_store: Arc<dyn TokenStore>,
    oauth: Option<Arc<CanvasOAuth2>>,
    client: Mutex<CanvasClient>,
}

pub struct CanvasConnectorBuilder {
    base_url: String,
    account_id: Uuid,
    token_store: Option<Arc<dyn TokenStore>>,
    oauth: Option<Arc<CanvasOAuth2>>,
    http: Option<reqwest::Client>,
}

impl CanvasConnectorBuilder {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            account_id: Uuid::nil(),
            token_store: None,
            oauth: None,
            http: None,
        }
    }

    pub fn account_id(mut self, id: Uuid) -> Self {
        self.account_id = id;
        self
    }

    pub fn token_store(mut self, s: Arc<dyn TokenStore>) -> Self {
        self.token_store = Some(s);
        self
    }

    pub fn oauth(mut self, o: Arc<CanvasOAuth2>) -> Self {
        self.oauth = Some(o);
        self
    }

    pub fn http(mut self, h: reqwest::Client) -> Self {
        self.http = Some(h);
        self
    }

    pub fn build(self) -> CanvasConnector {
        let http = self.http.unwrap_or_default();
        let store = self.token_store.unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
        let client = CanvasClient::with_http(&self.base_url, "", http);
        CanvasConnector {
            manifest: default_manifest(),
            account_id: self.account_id,
            token_store: store,
            oauth: self.oauth,
            client: Mutex::new(client),
        }
    }
}

fn default_manifest() -> ConnectorManifest {
    ConnectorManifest {
        id: "canvas".into(),
        version: "0.1.0".into(),
        display_name: "Canvas LMS".into(),
        auth_strategy: AuthStrategy::OAuth2 {
            scopes: vec![
                "url:GET|/api/v1/courses".into(),
                "url:GET|/api/v1/users/:user_id/courses".into(),
                "url:GET|/api/v1/courses/:course_id/assignments".into(),
                "url:GET|/api/v1/courses/:course_id/assignments/:assignment_id/submissions".into(),
            ],
        },
        sync_mode: SyncMode::Polling { cadence_seconds: 900 },
        capabilities: vec![],
        entity_types: vec!["course".into(), "assignment".into(), "submission".into()],
        event_types: vec![
            "assignment_due".into(),
            "assignment_graded".into(),
            "course_enrolled".into(),
        ],
    }
}

impl CanvasConnector {
    pub fn builder(base_url: impl Into<String>) -> CanvasConnectorBuilder {
        CanvasConnectorBuilder::new(base_url)
    }

    /// Load token from store and push into the HTTP client.
    async fn refresh_client_token(&self) -> Result<()> {
        let tok = self
            .token_store
            .load()
            .await?
            .ok_or_else(|| ConnectorError::Auth("no token".into()))?;
        let mut c = self.client.lock().await;
        c.set_access_token(tok.access_token);
        Ok(())
    }

    /// Try to refresh via OAuth if we have the machinery, else surface auth error.
    async fn try_token_refresh(&self) -> Result<()> {
        let oauth = self
            .oauth
            .as_ref()
            .ok_or_else(|| ConnectorError::Auth("no oauth configured".into()))?;
        let existing = self
            .token_store
            .load()
            .await?
            .ok_or_else(|| ConnectorError::Auth("no token to refresh".into()))?;
        let refresh = existing
            .refresh_token
            .clone()
            .ok_or_else(|| ConnectorError::Auth("no refresh token".into()))?;
        let http = reqwest::Client::new();
        let new = oauth.refresh(&refresh, &http).await?;
        self.token_store.save(&new).await?;
        self.refresh_client_token().await
    }
}

impl Default for CanvasConnector {
    fn default() -> Self {
        CanvasConnector::builder("https://canvas.instructure.com").build()
    }
}

#[async_trait]
impl Connector for CanvasConnector {
    fn manifest(&self) -> &ConnectorManifest {
        &self.manifest
    }

    async fn health(&self) -> HealthState {
        if self.refresh_client_token().await.is_err() {
            return HealthState::Unauthenticated;
        }
        let client = self.client.lock().await.clone();
        match client.get_self().await {
            Ok(_) => HealthState::Healthy,
            Err(ConnectorError::Auth(_)) => HealthState::Unauthenticated,
            Err(e) => HealthState::Failing(e.to_string()),
        }
    }

    async fn sync(&self, cursor: Option<String>) -> Result<SyncOutcome> {
        // Ensure token is loaded.
        self.refresh_client_token().await?;
        let client = { self.client.lock().await.clone() };

        // Courses page. Cursor here is a course-listing cursor. Once courses are
        // exhausted we don't paginate assignments within a single sync; we emit
        // what we have and hand back next_cursor so the driver can continue.
        let course_page = match client.list_courses(None, cursor.clone()).await {
            Ok(p) => p,
            Err(ConnectorError::Auth(_)) => {
                // Try a refresh and a single retry.
                self.try_token_refresh().await?;
                let client = self.client.lock().await.clone();
                client.list_courses(None, cursor).await?
            }
            Err(e) => return Err(e),
        };

        let mut events = Vec::new();
        for course in &course_page.items {
            events.push(CanvasEventMapper::map_course_enrolled(course, self.account_id));

            // Pull first page of assignments for this course.
            match client.list_assignments(course.id, None).await {
                Ok(asg_page) => {
                    for a in &asg_page.items {
                        events.push(CanvasEventMapper::map_assignment(a, self.account_id));

                        // Pull first page of submissions for this assignment.
                        if let Ok(sub_page) = client.list_submissions(a.id, course.id, None).await {
                            for s in &sub_page.items {
                                events.push(CanvasEventMapper::map_submission(s, self.account_id));
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!(course_id = course.id, error = %e, "skipping assignments");
                }
            }
        }

        Ok(SyncOutcome { events, next_cursor: course_page.next_cursor, partial: false })
    }
}

// Legacy re-exports preserved for callers of the old stub API.
pub struct CanvasEntity;
