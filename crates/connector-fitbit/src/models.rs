//! Fitbit API response models and domain objects.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub summary: ActivitySummary,
    pub activities: Vec<LoggedActivity>,
}

impl Activity {
    pub fn from_fitbit_json(json: &Value) -> Self {
        let summary = json
            .get("summary")
            .and_then(|s| serde_json::from_value(s.clone()).ok())
            .unwrap_or_default();
        let activities = json
            .get("activities")
            .and_then(|a| a.as_array().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();
        Activity { summary, activities }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivitySummary {
    #[serde(default)]
    pub steps: i32,
    #[serde(default)]
    pub caloriesBurned: i32,
    #[serde(default)]
    pub distance: f64,
    #[serde(default)]
    pub veryActiveMinutes: i32,
    #[serde(default)]
    pub fairlyActiveMinutes: i32,
    #[serde(default)]
    pub lightlyActiveMinutes: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoggedActivity {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub duration: i32, // milliseconds
    #[serde(default)]
    pub calories: i32,
    #[serde(default)]
    pub distance: f64,
    #[serde(default)]
    pub startTime: String, // ISO8601
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sleep {
    pub sleep: Vec<SleepSession>,
    pub summary: SleepSummary,
}

impl Sleep {
    pub fn from_fitbit_json(json: &Value) -> Self {
        let sleep = json
            .get("sleep")
            .and_then(|s| s.as_array().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();
        let summary = json
            .get("summary")
            .and_then(|s| serde_json::from_value(s.clone()).ok())
            .unwrap_or_default();
        Sleep { sleep, summary }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SleepSession {
    #[serde(default)]
    pub duration: i32, // milliseconds
    #[serde(default)]
    pub efficiency: i32, // percentage
    #[serde(default)]
    pub startTime: String, // ISO8601
    #[serde(default)]
    pub endTime: String, // ISO8601
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SleepSummary {
    #[serde(default)]
    pub totalMinutesAsleep: i32,
    #[serde(default)]
    pub totalSleepRecords: i32,
    #[serde(default)]
    pub totalTimeInBed: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartRate {
    #[serde(rename = "activities-heart")]
    pub heart_data: Vec<HeartRateEntry>,
}

impl HeartRate {
    pub fn from_fitbit_json(json: &Value) -> Self {
        let heart_data = json
            .get("activities-heart")
            .and_then(|h| h.as_array().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| serde_json::from_value(v).ok())
            .collect();
        HeartRate { heart_data }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeartRateEntry {
    #[serde(default)]
    pub dateTime: String,
    #[serde(default)]
    pub value: HeartRateValue,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeartRateValue {
    #[serde(default)]
    pub restingHeartRate: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-FITBIT-001 (models)
    #[test]
    fn activity_from_json() {
        let json = serde_json::json!({
            "summary": {
                "steps": 8500,
                "caloriesBurned": 1800,
                "distance": 6.5
            },
            "activities": [
                {
                    "name": "Running",
                    "duration": 3600000,
                    "calories": 400,
                    "startTime": "2026-04-23T09:00:00Z"
                }
            ]
        });
        let activity = Activity::from_fitbit_json(&json);
        assert_eq!(activity.summary.steps, 8500);
        assert_eq!(activity.activities.len(), 1);
    }

    // Traces to: FR-FITBIT-001 (models)
    #[test]
    fn sleep_from_json() {
        let json = serde_json::json!({
            "sleep": [
                {
                    "duration": 28800000,
                    "efficiency": 92,
                    "startTime": "2026-04-23T22:00:00Z",
                    "endTime": "2026-04-24T06:00:00Z"
                }
            ],
            "summary": {
                "totalMinutesAsleep": 480,
                "totalTimeInBed": 510
            }
        });
        let sleep = Sleep::from_fitbit_json(&json);
        assert_eq!(sleep.summary.totalMinutesAsleep, 480);
        assert_eq!(sleep.sleep.len(), 1);
    }

    // Traces to: FR-FITBIT-001 (models)
    #[test]
    fn heart_rate_from_json() {
        let json = serde_json::json!({
            "activities-heart": [
                {
                    "dateTime": "2026-04-23",
                    "value": {
                        "restingHeartRate": 62
                    }
                }
            ]
        });
        let hr = HeartRate::from_fitbit_json(&json);
        assert_eq!(hr.heart_data.len(), 1);
        assert_eq!(hr.heart_data[0].value.restingHeartRate, 62);
    }
}
