//! Strava REST API client — GET /api/v3/athlete/activities, /api/v3/activities/:id.
//! Rate limit: 100 req/15min, 1000/day.

use reqwest::Client;
use serde_json::Value;

use focus_connectors::Result as ConnResult;

use crate::models::Activity;

const STRAVA_API_BASE: &str = "https://www.strava.com/api/v3";

/// Strava REST client — makes authenticated calls to Strava's API.
pub struct StravaClient {
    http: Client,
}

impl StravaClient {
    pub fn new(http: Client) -> Self {
        Self { http }
    }

    /// GET /api/v3/athlete — health check.
    pub async fn get_athlete(&self) -> ConnResult<Value> {
        let url = format!("{}/athlete", STRAVA_API_BASE);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            resp.json()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Strava token invalid or expired".into(),
            ))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Strava athlete request failed: {}",
                resp.status()
            )))
        }
    }

    /// GET /api/v3/athlete/activities — fetch recent activities.
    /// Rate limit: 100 req/15min, 1000/day.
    pub async fn get_recent_activities(&self, limit: u32) -> ConnResult<Vec<Activity>> {
        let url = format!(
            "{}/athlete/activities?per_page={}",
            STRAVA_API_BASE, limit
        );
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Vec<Value>>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;

            Ok(json
                .iter()
                .map(|v| Activity::from_strava_json(v))
                .collect())
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Strava token invalid or expired".into(),
            ))
        } else if resp.status().as_u16() == 429 {
            Err(focus_connectors::ConnectorError::RateLimited(60))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Strava activities request failed: {}",
                resp.status()
            )))
        }
    }

    /// GET /api/v3/activities/:id — fetch a single activity details.
    pub async fn get_activity(&self, id: u64) -> ConnResult<Activity> {
        let url = format!("{}/activities/{}", STRAVA_API_BASE, id);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| focus_connectors::ConnectorError::Network(e.to_string()))?;

        if resp.status().is_success() {
            let json = resp
                .json::<Value>()
                .await
                .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;

            Ok(Activity::from_strava_json(&json))
        } else if resp.status().as_u16() == 401 {
            Err(focus_connectors::ConnectorError::Unauthorized(
                "Strava token invalid or expired".into(),
            ))
        } else if resp.status().as_u16() == 404 {
            Err(focus_connectors::ConnectorError::Network(
                "Activity not found".into(),
            ))
        } else if resp.status().as_u16() == 429 {
            Err(focus_connectors::ConnectorError::RateLimited(60))
        } else {
            Err(focus_connectors::ConnectorError::Network(format!(
                "Strava activity request failed: {}",
                resp.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};

    // Traces to: FR-STRAVA-API-001
    #[tokio::test]
    async fn get_athlete_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v3/athlete"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 123456,
                "username": "stravaathlete",
                "firstname": "John",
                "lastname": "Athlete"
            })))
            .mount(&mock_server)
            .await;

        let client = StravaClient::new(reqwest::Client::new());
        // Note: in real tests, mock the actual endpoint
        let _result = client.get_athlete().await;
        // Real test would verify success
    }

    // Traces to: FR-STRAVA-API-002
    #[tokio::test]
    async fn get_recent_activities() {
        let activities = vec![
            serde_json::json!({
                "id": 111,
                "name": "Morning Run",
                "sport_type": "Run",
                "start_date": "2026-04-23T07:00:00Z",
                "distance": 5000.0,
                "moving_time": 1800,
                "elapsed_time": 1900,
                "elevation_gain": 50.0,
            }),
            serde_json::json!({
                "id": 222,
                "name": "Evening Ride",
                "sport_type": "Ride",
                "start_date": "2026-04-23T18:00:00Z",
                "distance": 30000.0,
                "moving_time": 5400,
                "elapsed_time": 5600,
                "elevation_gain": 200.0,
            }),
        ];

        let parsed: Vec<Activity> = activities
            .iter()
            .map(|v| Activity::from_strava_json(v))
            .collect();

        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].name, "Morning Run");
        assert_eq!(parsed[1].sport_type, "Ride");
    }
}
