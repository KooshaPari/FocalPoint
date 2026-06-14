//! Linear event mapping — transforms API responses into normalized events.

use chrono::Utc;
use focus_events::{EventFactory, EventType, NormalizedEvent, TraceRef};
use uuid::Uuid;

use crate::models::LinearIssue;

pub struct LinearEventMapper {
    account_id: Uuid,
}

impl LinearEventMapper {
    pub fn new(account_id: Uuid) -> Self {
        Self { account_id }
    }

    /// Map Linear issues into issue_created and issue_closed events.
    pub fn map_issues(&self, issues: Vec<LinearIssue>) -> Vec<NormalizedEvent> {
        issues
            .into_iter()
            .flat_map(|issue| {
                let mut events = Vec::new();

                let created_at = chrono::DateTime::parse_from_rfc3339(&issue.created_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                let updated_at = chrono::DateTime::parse_from_rfc3339(&issue.updated_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                // Emit issue_created on first sync.
                let create_key = EventFactory::new_dedupe_key(
                    "linear",
                    &format!("{}-created", issue.id),
                    created_at,
                );

                events.push(NormalizedEvent {
                    event_id: Uuid::new_v4(),
                    connector_id: "linear".to_string(),
                    account_id: self.account_id,
                    event_type: EventType::Custom("linear:issue_created".to_string()),
                    occurred_at: created_at,
                    effective_at: Utc::now(),
                    dedupe_key: create_key,
                    confidence: 0.99,
                    payload: serde_json::json!({
                        "issue_id": issue.id,
                        "issue_identifier": issue.identifier,
                        "title": issue.title,
                        "created_at_iso": issue.created_at,
                    }),
                    raw_ref: Some(TraceRef {
                        source: "linear-api".to_string(),
                        id: issue.id.clone(),
                    }),
                });

                // Emit issue_closed if state is closed.
                if issue.state.to_lowercase().contains("done")
                    || issue.state.to_lowercase().contains("closed")
                {
                    let close_key = EventFactory::new_dedupe_key(
                        "linear",
                        &format!("{}-closed", issue.id),
                        updated_at,
                    );

                    events.push(NormalizedEvent {
                        event_id: Uuid::new_v4(),
                        connector_id: "linear".to_string(),
                        account_id: self.account_id,
                        event_type: EventType::Custom("linear:issue_closed".to_string()),
                        occurred_at: updated_at,
                        effective_at: Utc::now(),
                        dedupe_key: close_key,
                        confidence: 0.99,
                        payload: serde_json::json!({
                            "issue_id": issue.id,
                            "issue_identifier": issue.identifier,
                            "title": issue.title,
                            "state": issue.state,
                            "closed_at_iso": issue.updated_at,
                        }),
                        raw_ref: Some(TraceRef {
                            source: "linear-api".to_string(),
                            id: format!("{}-closed", issue.id),
                        }),
                    });
                }

                events
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-LINEAR-EVENTS-001
    #[test]
    fn map_issues_created() {
        let account_id = Uuid::new_v4();
        let mapper = LinearEventMapper::new(account_id);
        let issue = LinearIssue {
            id: "iss-1".to_string(),
            identifier: "ENG-123".to_string(),
            title: "Fix bug".to_string(),
            state: "To Do".to_string(),
            created_at: "2026-04-23T09:00:00Z".to_string(),
            updated_at: "2026-04-23T09:00:00Z".to_string(),
        };

        let events = mapper.map_issues(vec![issue]);
        assert_eq!(events.len(), 1);
        assert!(events[0].event_type.to_string().contains("issue_created"));
    }

    // Traces to: FR-LINEAR-EVENTS-001
    #[test]
    fn map_issues_closed() {
        let account_id = Uuid::new_v4();
        let mapper = LinearEventMapper::new(account_id);
        let issue = LinearIssue {
            id: "iss-2".to_string(),
            identifier: "ENG-456".to_string(),
            title: "Completed task".to_string(),
            state: "Done".to_string(),
            created_at: "2026-04-20T09:00:00Z".to_string(),
            updated_at: "2026-04-23T10:00:00Z".to_string(),
        };

        let events = mapper.map_issues(vec![issue]);
        // Should emit both created and closed
        assert_eq!(events.len(), 2);
        assert!(events.iter().any(|e| e.event_type.to_string().contains("issue_created")));
        assert!(events.iter().any(|e| e.event_type.to_string().contains("issue_closed")));
    }
}
