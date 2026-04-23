//! Google Calendar v3 domain types. Field names match Google's REST API.
//!
//! Google returns `dateTime` as RFC3339 with offset, and `date` as `YYYY-MM-DD`
//! for all-day events. We keep both as `Option<String>` and let the event
//! mapper parse when possible — trying to force chrono here loses fidelity for
//! all-day events.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarListEntry {
    pub id: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub primary: Option<bool>,
    #[serde(default)]
    pub time_zone: Option<String>,
    #[serde(default, rename = "accessRole")]
    pub access_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarList {
    #[serde(default)]
    pub items: Vec<CalendarListEntry>,
    #[serde(default, rename = "nextPageToken")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EventDateTime {
    #[serde(default, rename = "dateTime")]
    pub date_time: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default, rename = "timeZone")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GCalEvent {
    pub id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub start: Option<EventDateTime>,
    #[serde(default)]
    pub end: Option<EventDateTime>,
    #[serde(default, rename = "htmlLink")]
    pub html_link: Option<String>,
    #[serde(default, rename = "iCalUID")]
    pub ical_uid: Option<String>,
    #[serde(default, rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(default, rename = "recurringEventId")]
    pub recurring_event_id: Option<String>,
    #[serde(default)]
    pub transparency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventList {
    #[serde(default)]
    pub items: Vec<GCalEvent>,
    #[serde(default, rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    #[serde(default, rename = "nextSyncToken")]
    pub next_sync_token: Option<String>,
    #[serde(default, rename = "timeZone")]
    pub time_zone: Option<String>,
}

/// User info returned by `/oauth2/v2/userinfo`. Used as a health-check probe
/// (parallel to Canvas's `/users/self`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GCalUser {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
}

#[cfg(test)]
#[allow(clippy::disallowed_methods)]
mod tests {
    use super::*;

    #[test]
    fn parses_calendar_list_minimal() {
        let j = r#"{"items":[{"id":"primary","summary":"Me","primary":true}]}"#;
        let l: CalendarList = serde_json::from_str(j).unwrap();
        assert_eq!(l.items.len(), 1);
        assert_eq!(l.items[0].id, "primary");
        assert_eq!(l.items[0].primary, Some(true));
    }

    #[test]
    fn parses_event_timed() {
        let j = r#"{
            "id":"evt1",
            "summary":"Standup",
            "start":{"dateTime":"2026-05-01T09:00:00-07:00","timeZone":"America/Los_Angeles"},
            "end":{"dateTime":"2026-05-01T09:30:00-07:00"},
            "htmlLink":"https://cal.example/evt1",
            "status":"confirmed"
        }"#;
        let e: GCalEvent = serde_json::from_str(j).unwrap();
        assert_eq!(e.id, "evt1");
        assert_eq!(e.summary, "Standup");
        assert_eq!(
            e.start.as_ref().and_then(|s| s.date_time.as_deref()),
            Some("2026-05-01T09:00:00-07:00")
        );
    }

    #[test]
    fn parses_event_all_day() {
        let j = r#"{"id":"evt2","summary":"Holiday","start":{"date":"2026-07-04"},"end":{"date":"2026-07-05"}}"#;
        let e: GCalEvent = serde_json::from_str(j).unwrap();
        assert_eq!(e.start.as_ref().and_then(|s| s.date.as_deref()), Some("2026-07-04"));
        assert!(e.start.as_ref().unwrap().date_time.is_none());
    }

    #[test]
    fn parses_event_list_with_paging() {
        let j = r#"{"items":[{"id":"a","summary":"A"}],"nextPageToken":"tok","timeZone":"UTC"}"#;
        let l: EventList = serde_json::from_str(j).unwrap();
        assert_eq!(l.items.len(), 1);
        assert_eq!(l.next_page_token.as_deref(), Some("tok"));
        assert_eq!(l.time_zone.as_deref(), Some("UTC"));
    }
}
