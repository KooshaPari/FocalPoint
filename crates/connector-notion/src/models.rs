//! Notion data models.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotionPage {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub created_time: String,
    pub last_edited_time: String,
    pub url: String,
}

fn notion_items(json: &Value) -> Vec<&Value> {
    if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
        results.iter().collect()
    } else if json.get("object").and_then(|o| o.as_str()) == Some("page") {
        vec![json]
    } else {
        vec![]
    }
}

fn notion_title(properties: &Value, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|key| {
            properties
                .get(*key)
                .and_then(|t| t.get("title"))
                .and_then(|arr| arr.as_array())
                .and_then(|arr| arr.first())
                .and_then(|t| {
                    t.get("plain_text")
                        .or_else(|| t.get("text").and_then(|text| text.get("content")))
                })
                .and_then(|t| t.as_str())
        })
        .unwrap_or("Untitled")
        .to_string()
}

impl NotionPage {
    pub fn from_notion_json(json: &Value) -> Vec<NotionPage> {
        notion_items(json)
            .into_iter()
            .filter_map(|page| {
                let properties = page.get("properties").unwrap_or(&Value::Null);
                Some(NotionPage {
                    id: page.get("id")?.as_str()?.into(),
                    title: notion_title(properties, &["title", "Name", "name"]),
                    icon: page
                        .get("icon")
                        .and_then(|i| i.get("emoji"))
                        .and_then(|e| e.as_str())
                        .map(|s| s.into()),
                    created_time: page
                        .get("created_time")
                        .and_then(|t| t.as_str())
                        .unwrap_or_default()
                        .into(),
                    last_edited_time: page
                        .get("last_edited_time")
                        .and_then(|t| t.as_str())
                        .unwrap_or_default()
                        .into(),
                    url: page
                        .get("url")
                        .and_then(|u| u.as_str())
                        .unwrap_or_default()
                        .into(),
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotionTask {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub due_date: Option<String>,
    pub last_edited_time: String,
}

impl NotionTask {
    pub fn from_notion_json(json: &Value) -> Vec<NotionTask> {
        notion_items(json)
            .into_iter()
            .filter_map(|task| {
                let properties = task.get("properties").unwrap_or(&Value::Null);
                let status_done = properties
                    .get("status")
                    .and_then(|s| s.get("select"))
                    .and_then(|s| s.get("name"))
                    .and_then(|s| s.as_str())
                    .map(|status| status.eq_ignore_ascii_case("done"))
                    .unwrap_or(false);
                let completed = properties
                    .get("Completed")
                    .and_then(|c| c.get("checkbox"))
                    .and_then(|c| c.as_bool())
                    .unwrap_or(status_done);

                Some(NotionTask {
                    id: task.get("id")?.as_str()?.into(),
                    title: notion_title(properties, &["title", "Name", "name"]),
                    completed,
                    due_date: properties
                        .get("Due")
                        .and_then(|d| d.get("date"))
                        .and_then(|d| d.get("start"))
                        .and_then(|s| s.as_str())
                        .map(|s| s.into()),
                    last_edited_time: task
                        .get("last_edited_time")
                        .and_then(|t| t.as_str())
                        .unwrap_or_default()
                        .into(),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-NOTION-MODELS-001
    #[test]
    fn page_from_json() {
        let json = serde_json::json!({
            "results": [
                {
                    "id": "123",
                    "url": "https://notion.so/123",
                    "created_time": "2026-04-23T10:00:00Z",
                    "last_edited_time": "2026-04-23T10:00:00Z",
                    "properties": {
                        "title": {
                            "title": [
                                { "plain_text": "My Page" }
                            ]
                        }
                    }
                }
            ]
        });
        let pages = NotionPage::from_notion_json(&json);
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title, "My Page");
    }

    // Traces to: FR-NOTION-MODELS-001
    #[test]
    fn task_from_json() {
        let json = serde_json::json!({
            "results": [
                {
                    "id": "456",
                    "last_edited_time": "2026-04-23T10:00:00Z",
                    "properties": {
                        "title": {
                            "title": [
                                { "plain_text": "Complete task" }
                            ]
                        },
                        "Completed": {
                            "checkbox": true
                        }
                    }
                }
            ]
        });
        let tasks = NotionTask::from_notion_json(&json);
        assert_eq!(tasks.len(), 1);
        assert!(tasks[0].completed);
    }
}
