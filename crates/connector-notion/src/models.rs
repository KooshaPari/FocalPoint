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

impl NotionPage {
    pub fn from_notion_json(json: &Value) -> Vec<NotionPage> {
        let pages: Vec<&Value> = json
            .get("results")
            .and_then(|r| r.as_array())
            .map(|results| results.iter().collect())
            .unwrap_or_else(|| {
                if json.get("object").and_then(|o| o.as_str()) == Some("page") {
                    vec![json]
                } else {
                    vec![]
                }
            });

        pages
            .into_iter()
            .filter_map(|page| {
                let title = notion_title(page).unwrap_or("Untitled");

                Some(NotionPage {
                    id: page.get("id")?.as_str()?.into(),
                    title: title.into(),
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

fn notion_title(page: &Value) -> Option<&str> {
    page.get("properties")?
        .as_object()?
        .values()
        .find_map(|property| {
            property
                .get("title")
                .and_then(|arr| arr.as_array())
                .and_then(|arr| arr.first())
                .and_then(|title| {
                    title
                        .get("plain_text")
                        .or_else(|| title.get("text").and_then(|text| text.get("content")))
                })
                .and_then(|title| title.as_str())
        })
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
        let tasks: Vec<&Value> = json
            .get("results")
            .and_then(|r| r.as_array())
            .map(|results| results.iter().collect())
            .unwrap_or_else(|| {
                if json.get("object").and_then(|o| o.as_str()) == Some("page") {
                    vec![json]
                } else {
                    vec![]
                }
            });

        tasks
            .into_iter()
            .filter_map(|task| {
                let title = notion_title(task).unwrap_or("Untitled");
                let completed = task
                    .get("properties")
                    .and_then(|p| p.get("Completed"))
                    .and_then(|c| c.get("checkbox"))
                    .and_then(|c| c.as_bool())
                    .unwrap_or(false);

                Some(NotionTask {
                    id: task.get("id")?.as_str()?.into(),
                    title: title.into(),
                    completed,
                    due_date: task
                        .get("properties")
                        .and_then(|p| p.get("Due"))
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
