W3-15: Workspace hygiene check
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:1:
 //! Canvas REST client.
 
[31m-use phenotype_observably_macros::async_instrumented;
(B[m use focus_connectors::ConnectorError;
[32m+use phenotype_observably_macros::async_instrumented;
(B[m use reqwest::header::{HeaderMap, AUTHORIZATION, LINK, RETRY_AFTER};
 use reqwest::StatusCode;
 use serde::de::DeserializeOwned;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:81:
 
         match status {
             s if s.is_success() => {
[31m-                let body: T =
(B[m[31m-                    resp.json().await.map_err(|e| ConnectorError::Schema(e.to_string()))?;
(B[m[32m+                let body: T = resp
(B[m[32m+                    .json()
(B[m[32m+                    .await
(B[m[32m+                    .map_err(|e| ConnectorError::Schema(e.to_string()))?;
(B[m                 Ok((body, headers))
             }
             StatusCode::UNAUTHORIZED => Err(ConnectorError::Auth("401 from Canvas".into())),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:131:
         user_id: Option<u64>,
         cursor: Option<String>,
     ) -> Result<Page<Course>, ConnectorError> {
[31m-        let who = user_id.map(|i| i.to_string()).unwrap_or_else(|| "self".into());
(B[m[32m+        let who = user_id
(B[m[32m+            .map(|i| i.to_string())
(B[m[32m+            .unwrap_or_else(|| "self".into());
(B[m         let url = format!(
             "{}/api/v1/users/{}/courses?per_page=50&enrollment_state=active",
             self.base_url, who
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:145:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<Assignment>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/assignments?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/assignments?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:198:
         course_id: u64,
         user_id: Option<u64>,
     ) -> Result<CourseProgress, ConnectorError> {
[31m-        let who = user_id.map(|i| i.to_string()).unwrap_or_else(|| "self".into());
(B[m[31m-        let url = format!("{}/api/v1/users/{}/courses/{}/progress", self.base_url, who, course_id);
(B[m[32m+        let who = user_id
(B[m[32m+            .map(|i| i.to_string())
(B[m[32m+            .unwrap_or_else(|| "self".into());
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/users/{}/courses/{}/progress",
(B[m[32m+            self.base_url, who, course_id
(B[m[32m+        );
(B[m         let (p, _) = self.get_json::<CourseProgress>(&url).await?;
         Ok(p)
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:227:
         course_id: u64,
         assignment_id: u64,
     ) -> Result<Assignment, ConnectorError> {
[31m-        let url =
(B[m[31m-            format!("{}/api/v1/courses/{}/assignments/{}", self.base_url, course_id, assignment_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/assignments/{}",
(B[m[32m+            self.base_url, course_id, assignment_id
(B[m[32m+        );
(B[m         let (a, _) = self.get_json::<Assignment>(&url).await?;
         Ok(a)
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:268:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<DiscussionTopic>, ConnectorError> {
[31m-        let url =
(B[m[31m-            format!("{}/api/v1/courses/{}/discussion_topics?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/discussion_topics?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:297:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<Quiz>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/quizzes?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/quizzes?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:325:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<Module>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/modules?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/modules?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:353:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<WikiPage>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/pages?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/pages?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:429:
         group_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<GroupMembership>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/groups/{}/memberships?per_page=50", self.base_url, group_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/groups/{}/memberships?per_page=50",
(B[m[32m+            self.base_url, group_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:441:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<File>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/files?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/files?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:453:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<Rubric>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/rubrics?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/rubrics?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:480:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<Outcome>, ConnectorError> {
[31m-        let url = format!("{}/api/v1/courses/{}/outcomes?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/outcomes?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:492:
         course_id: u64,
         cursor: Option<String>,
     ) -> Result<Page<OutcomeResult>, ConnectorError> {
[31m-        let url =
(B[m[31m-            format!("{}/api/v1/courses/{}/outcome_results?per_page=50", self.base_url, course_id);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/api/v1/courses/{}/outcome_results?per_page=50",
(B[m[32m+            self.base_url, course_id
(B[m[32m+        );
(B[m         self.list_paginated(url, cursor).await
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:505:
         let seg = part.trim();
         // Example: <https://canvas/.../?page=2>; rel="next"
         let (url_part, rel_part) = seg.split_once(';')?;
[31m-        let url = url_part.trim().trim_start_matches('<').trim_end_matches('>');
(B[m[32m+        let url = url_part
(B[m[32m+            .trim()
(B[m[32m+            .trim_start_matches('<')
(B[m[32m+            .trim_end_matches('>');
(B[m         if rel_part.contains("rel=\"next\"") {
             return Some(url.to_string());
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/api.rs:561:
             .and(path_regex(r"^/api/v1/users/self/courses$"))
             .and(header("authorization", "Bearer TOK"))
             .respond_with(
[31m-                ResponseTemplate::new(200).insert_header("Link", link_hdr.as_str()).set_body_json(
(B[m[31m-                    serde_json::json!([{"id":1,"name":"A","workflow_state":"available"}]),
(B[m[31m-                ),
(B[m[32m+                ResponseTemplate::new(200)
(B[m[32m+                    .insert_header("Link", link_hdr.as_str())
(B[m[32m+                    .set_body_json(
(B[m[32m+                        serde_json::json!([{"id":1,"name":"A","workflow_state":"available"}]),
(B[m[32m+                    ),
(B[m             )
             .mount(&server)
             .await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/auth.rs:85:
         Self::default()
     }
     pub fn with_token(token: CanvasToken) -> Self {
[31m-        Self { inner: Mutex::new(Some(token)) }
(B[m[32m+        Self {
(B[m[32m+            inner: Mutex::new(Some(token)),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/auth.rs:127:
         account: impl Into<String>,
         inner: std::sync::Arc<dyn focus_crypto::SecureSecretStore>,
     ) -> Self {
[31m-        Self { account: account.into(), inner }
(B[m[32m+        Self {
(B[m[32m+            account: account.into(),
(B[m[32m+            inner,
(B[m[32m+        }
(B[m     }
 
     /// Convenience: build using [`focus_crypto::default_secure_store`] for
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/auth.rs:249:
 
 fn to_token(resp: &oauth2::basic::BasicTokenResponse) -> CanvasToken {
     let now = Utc::now();
[31m-    let expires_at =
(B[m[31m-        resp.expires_in().and_then(|d| chrono::Duration::from_std(d).ok()).map(|d| now + d);
(B[m[32m+    let expires_at = resp
(B[m[32m+        .expires_in()
(B[m[32m+        .and_then(|d| chrono::Duration::from_std(d).ok())
(B[m[32m+        .map(|d| now + d);
(B[m     CanvasToken {
         access_token: resp.access_token().secret().clone(),
         refresh_token: resp.refresh_token().map(|r| r.secret().clone()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:210:
                 "workflow_state": c.workflow_state,
                 "enrollment_term_id": c.enrollment_term_id,
             }),
[31m-            raw_ref: Some(TraceRef { source: CONNECTOR_ID.into(), id: format!("course:{}", c.id) }),
(B[m[32m+            raw_ref: Some(TraceRef {
(B[m[32m+                source: CONNECTOR_ID.into(),
(B[m[32m+                id: format!("course:{}", c.id),
(B[m[32m+            }),
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:221:
         account_id: Uuid,
         course_id: u64,
     ) -> NormalizedEvent {
[31m-        let occurred = ann.posted_at.or(ann.delayed_post_at).unwrap_or_else(Utc::now);
(B[m[32m+        let occurred = ann
(B[m[32m+            .posted_at
(B[m[32m+            .or(ann.delayed_post_at)
(B[m[32m+            .unwrap_or_else(Utc::now);
(B[m         NormalizedEvent {
             event_id: Uuid::new_v4(),
             connector_id: CONNECTOR_ID.into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:410:
         quiz_id: u64,
         course_id: u64,
     ) -> Option<NormalizedEvent> {
[31m-        let occurred = submission.submitted_at.or(submission.finished_at).unwrap_or_else(Utc::now);
(B[m[32m+        let occurred = submission
(B[m[32m+            .submitted_at
(B[m[32m+            .or(submission.finished_at)
(B[m[32m+            .unwrap_or_else(Utc::now);
(B[m         Some(NormalizedEvent {
             event_id: Uuid::new_v4(),
             connector_id: CONNECTOR_ID.into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:701:
             Some(Utc.with_ymd_and_hms(2026, 5, 1, 12, 0, 0).unwrap()),
         );
         let ev = CanvasEventMapper::map_assignment(&a, acct(), None);
[31m-        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::AssignmentDue));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::AssignmentDue)
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["course_id"], 42);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:734:
         let now = Utc::now();
         let a = assignment_with_due(1, Some(1), Some(now + Duration::hours(6)));
         let ev = CanvasEventMapper::map_assignment_due_soon(&a, acct(), now, None).unwrap();
[31m-        assert_eq!(ev.event_type, EventType::Custom("canvas:assignment_due_soon".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::Custom("canvas:assignment_due_soon".into())
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:755:
         let now = Utc::now();
         let a = assignment_with_due(1, Some(1), Some(now - Duration::hours(5)));
         let ev = CanvasEventMapper::map_assignment_overdue(&a, acct(), now, false, None).unwrap();
[31m-        assert_eq!(ev.event_type, EventType::Custom("canvas:assignment_overdue".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::Custom("canvas:assignment_overdue".into())
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["hours_overdue"], 5);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:763:
     fn overdue_skips_when_submission_exists_or_not_past_due() {
         let now = Utc::now();
         let past = assignment_with_due(1, Some(1), Some(now - Duration::hours(5)));
[31m-        assert!(CanvasEventMapper::map_assignment_overdue(&past, acct(), now, true, None).is_none());
(B[m[32m+        assert!(
(B[m[32m+            CanvasEventMapper::map_assignment_overdue(&past, acct(), now, true, None).is_none()
(B[m[32m+        );
(B[m 
         let future = assignment_with_due(2, Some(1), Some(now + Duration::hours(5)));
         assert!(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:786:
             missing: None,
         };
         let ev = CanvasEventMapper::map_submission(&s, acct());
[31m-        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::AssignmentGraded));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::AssignmentGraded)
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["score"], 95.0);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:805:
             missing: None,
         };
         let ev = CanvasEventMapper::map_grade_posted(&s, acct()).unwrap();
[31m-        assert_eq!(ev.event_type, EventType::Custom("canvas:grade_posted".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::Custom("canvas:grade_posted".into())
(B[m[32m+        );
(B[m 
         s.workflow_state = "submitted".into();
         assert!(CanvasEventMapper::map_grade_posted(&s, acct()).is_none());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:827:
             end_at: None,
         };
         let ev = CanvasEventMapper::map_course_enrolled(&c, acct());
[31m-        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::CourseEnrolled));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::CourseEnrolled)
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["course_id"], 42);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/events.rs:843:
             context_code: Some("course_42".into()),
         };
         let ev = CanvasEventMapper::map_announcement_posted(&ann, acct(), 42);
[31m-        assert_eq!(ev.event_type, EventType::Custom("canvas:announcement_posted".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::Custom("canvas:announcement_posted".into())
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["course_id"], 42);
         assert_eq!(ev.payload["announcement_id"], 55);
         assert!(ev.dedupe_key.0.starts_with("canvas:announcement:55:"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:90:
 
     pub fn build(self) -> CanvasConnector {
         let http = self.http.unwrap_or_default();
[31m-        let store = self.token_store.unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
(B[m[32m+        let store = self
(B[m[32m+            .token_store
(B[m[32m+            .unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
(B[m         let client = CanvasClient::with_http(&self.base_url, "", http);
         CanvasConnector {
             manifest: default_manifest(self.scopes.unwrap_or_default()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:111:
         // Developer Key + OAuth flow handles this correctly; hard-coded
         // `url:GET|...` scopes 400 on instances that haven't enabled them.
         auth_strategy: AuthStrategy::OAuth2 { scopes },
[31m-        sync_mode: SyncMode::Polling { cadence_seconds: 900 },
(B[m[32m+        sync_mode: SyncMode::Polling {
(B[m[32m+            cadence_seconds: 900,
(B[m[32m+        },
(B[m         capabilities: vec![],
         entity_types: vec![
             "course".into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:291:
         let now = Utc::now();
         let mut events = Vec::new();
         for course in &course_page.items {
[31m-            events.push(CanvasEventMapper::map_course_enrolled(course, self.account_id));
(B[m[32m+            events.push(CanvasEventMapper::map_course_enrolled(
(B[m[32m+                course,
(B[m[32m+                self.account_id,
(B[m[32m+            ));
(B[m 
             // Fully paginate assignments for this course.
             let assignments = {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:312:
             };
 
             for a in &assignments {
[31m-                events.push(CanvasEventMapper::map_assignment(a, self.account_id, Some(course.id)));
(B[m[32m+                events.push(CanvasEventMapper::map_assignment(
(B[m[32m+                    a,
(B[m[32m+                    self.account_id,
(B[m[32m+                    Some(course.id),
(B[m[32m+                ));
(B[m 
                 // Fully paginate submissions for this assignment. Collect
                 // them so we can compute due-soon/overdue with accurate
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:337:
                     Vec::new()
                 });
 
[31m-                let has_submission =
(B[m[31m-                    submissions.iter().any(|s| s.submitted_at.is_some() || s.score.is_some());
(B[m[32m+                let has_submission = submissions
(B[m[32m+                    .iter()
(B[m[32m+                    .any(|s| s.submitted_at.is_some() || s.score.is_some());
(B[m 
                 for s in &submissions {
                     events.push(CanvasEventMapper::map_submission(s, self.account_id));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:392:
             }
         }
 
[31m-        Ok(SyncOutcome { events, next_cursor: course_page.next_cursor, partial: false })
(B[m[32m+        Ok(SyncOutcome {
(B[m[32m+            events,
(B[m[32m+            next_cursor: course_page.next_cursor,
(B[m[32m+            partial: false,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:408:
     fn default_manifest_scopes_are_empty() {
         let m = default_manifest(vec![]);
         if let AuthStrategy::OAuth2 { scopes } = &m.auth_strategy {
[31m-            assert!(scopes.is_empty(), "default scopes must be empty to avoid invalid_scope 400");
(B[m[32m+            assert!(
(B[m[32m+                scopes.is_empty(),
(B[m[32m+                "default scopes must be empty to avoid invalid_scope 400"
(B[m[32m+            );
(B[m         } else {
             panic!("expected OAuth2 strategy");
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/src/lib.rs:429:
     #[test]
     fn manifest_declares_new_event_types() {
         let m = default_manifest(vec![]);
[31m-        for want in
(B[m[31m-            ["assignment_due_soon", "assignment_overdue", "grade_posted", "announcement_posted"]
(B[m[31m-        {
(B[m[31m-            assert!(m.event_types.iter().any(|e| e == want), "missing event: {want}");
(B[m[32m+        for want in [
(B[m[32m+            "assignment_due_soon",
(B[m[32m+            "assignment_overdue",
(B[m[32m+            "grade_posted",
(B[m[32m+            "announcement_posted",
(B[m[32m+        ] {
(B[m[32m+            assert!(
(B[m[32m+                m.event_types.iter().any(|e| e == want),
(B[m[32m+                "missing event: {want}"
(B[m[32m+            );
(B[m         }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:12:
 use wiremock::{Mock, MockServer, ResponseTemplate};
 
 fn load_fixture(name: &str) -> Value {
[31m-    let p =
(B[m[31m-        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join(name);
(B[m[32m+    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
(B[m[32m+        .join("tests")
(B[m[32m+        .join("fixtures")
(B[m[32m+        .join(name);
(B[m     let s = std::fs::read_to_string(p).expect("fixture");
     serde_json::from_str(&s).expect("json")
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:51:
         .await;
 
     Mock::given(method("GET"))
[31m-        .and(path_regex(r"^/api/v1/courses/101/assignments/9001/submissions$"))
(B[m[32m+        .and(path_regex(
(B[m[32m+            r"^/api/v1/courses/101/assignments/9001/submissions$",
(B[m[32m+        ))
(B[m         .respond_with(ResponseTemplate::new(200).set_body_json(load_fixture("submissions.json")))
         .mount(&server)
         .await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:58:
 
     let store = seeded_store("ACC").await;
[31m-    let conn =
(B[m[31m-        CanvasConnector::builder(server.uri()).account_id(Uuid::nil()).token_store(store).build();
(B[m[32m+    let conn = CanvasConnector::builder(server.uri())
(B[m[32m+        .account_id(Uuid::nil())
(B[m[32m+        .token_store(store)
(B[m[32m+        .build();
(B[m 
     let out = conn.sync(None).await.expect("sync ok");
     // 2 courses enrolled + 1 assignment + 1 submission + 1 grade_posted = 5.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:65:
     // (Announcement endpoint not mocked; sync warns + continues.)
     assert_eq!(out.events.len(), 5);
[31m-    let kinds: Vec<_> = out.events.iter().map(|e| format!("{:?}", e.event_type)).collect();
(B[m[32m+    let kinds: Vec<_> = out
(B[m[32m+        .events
(B[m[32m+        .iter()
(B[m[32m+        .map(|e| format!("{:?}", e.event_type))
(B[m[32m+        .collect();
(B[m     assert!(kinds.iter().any(|k| k.contains("CourseEnrolled")));
     assert!(kinds.iter().any(|k| k.contains("AssignmentDue")));
     assert!(kinds.iter().any(|k| k.contains("AssignmentGraded")));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:88:
         .mount(&server)
         .await;
 
[31m-    let conn = CanvasConnector::builder(&base).token_store(seeded_store("ACC").await).build();
(B[m[32m+    let conn = CanvasConnector::builder(&base)
(B[m[32m+        .token_store(seeded_store("ACC").await)
(B[m[32m+        .build();
(B[m 
     let out = conn.sync(None).await.unwrap();
     assert_eq!(out.next_cursor.as_deref(), Some(next_url.as_str()));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:105:
         .mount(&server)
         .await;
 
[31m-    let conn =
(B[m[31m-        CanvasConnector::builder(server.uri()).token_store(seeded_store("ACC").await).build();
(B[m[32m+    let conn = CanvasConnector::builder(server.uri())
(B[m[32m+        .token_store(seeded_store("ACC").await)
(B[m[32m+        .build();
(B[m     assert_eq!(conn.health().await, HealthState::Healthy);
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:184:
     let profile = client.get_user_profile().await.unwrap();
     assert_eq!(profile.name, "Alice Student");
     assert_eq!(profile.email, Some("alice@example.edu".into()));
[31m-    assert_eq!(profile.avatar_url, Some("https://canvas.example.com/images/avatars/42.png".into()));
(B[m[32m+    assert_eq!(
(B[m[32m+        profile.avatar_url,
(B[m[32m+        Some("https://canvas.example.com/images/avatars/42.png".into())
(B[m[32m+    );
(B[m     assert_eq!(profile.locale, Some("en".into()));
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration.rs:420:
 async fn list_discussion_entries_happy_path() {
     let server = MockServer::start().await;
     Mock::given(method("GET"))
[31m-        .and(path_regex(r"^/api/v1/courses/101/discussion_topics/5/entries$"))
(B[m[32m+        .and(path_regex(
(B[m[32m+            r"^/api/v1/courses/101/discussion_topics/5/entries$",
(B[m[32m+        ))
(B[m         .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
             {
                 "id": 10,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration_live.rs:36:
 
     let courses = client.list_courses(None, None).await.expect("list_courses");
     eprintln!("got {} courses", courses.items.len());
[31m-    assert!(!courses.items.is_empty(), "expected at least one enrolled course in sandbox");
(B[m[32m+    assert!(
(B[m[32m+        !courses.items.is_empty(),
(B[m[32m+        "expected at least one enrolled course in sandbox"
(B[m[32m+    );
(B[m 
     let first = &courses.items[0];
     eprintln!("course[0] id={} name={}", first.id, first.name);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-canvas/tests/integration_live.rs:43:
 
[31m-    let assignments = client.list_assignments(first.id, None).await.expect("list_assignments");
(B[m[31m-    eprintln!("got {} assignments in course {}", assignments.items.len(), first.id);
(B[m[32m+    let assignments = client
(B[m[32m+        .list_assignments(first.id, None)
(B[m[32m+        .await
(B[m[32m+        .expect("list_assignments");
(B[m[32m+    eprintln!(
(B[m[32m+        "got {} assignments in course {}",
(B[m[32m+        assignments.items.len(),
(B[m[32m+        first.id
(B[m[32m+    );
(B[m 
     if let Some(a) = assignments.items.first() {
         let subs = client.list_submissions(a.id, first.id, None).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/events.rs:25:
                     .map(|dt| dt.with_timezone(&Utc))
                     .unwrap_or_else(|_| Utc::now());
 
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "fitbit",
(B[m[31m-                    &logged.name,
(B[m[31m-                    started_at,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("fitbit", &logged.name, started_at);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/events.rs:71:
                     .map(|dt| dt.with_timezone(&Utc))
                     .unwrap_or_else(|_| Utc::now());
 
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "fitbit",
(B[m[31m-                    "sleep",
(B[m[31m-                    in_bed_at,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("fitbit", "sleep", in_bed_at);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/events.rs:105:
     pub fn map_steps(&self, activity: &Activity) -> Vec<NormalizedEvent> {
         if activity.summary.steps >= 10000 {
             let now = Utc::now();
[31m-            let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                "fitbit",
(B[m[31m-                "daily_steps_milestone",
(B[m[31m-                now,
(B[m[31m-            );
(B[m[32m+            let dedupe_key = EventFactory::new_dedupe_key("fitbit", "daily_steps_milestone", now);
(B[m 
             vec![NormalizedEvent {
                 event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/events.rs:142:
             .filter(|entry| entry.value.resting_heart_rate > 0)
             .map(|entry| {
                 let now = Utc::now();
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "fitbit",
(B[m[31m-                    "resting_heart_rate",
(B[m[31m-                    now,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("fitbit", "resting_heart_rate", now);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/lib.rs:12:
 use tracing::{debug, info, warn};
 use uuid::Uuid;
 
[31m-use focus_connectors::{AuthStrategy, Connector, ConnectorError, ConnectorManifest, HealthState, Result, SyncMode, SyncOutcome, VerificationTier};
(B[m[32m+use focus_connectors::{
(B[m[32m+    AuthStrategy, Connector, ConnectorError, ConnectorManifest, HealthState, Result, SyncMode,
(B[m[32m+    SyncOutcome, VerificationTier,
(B[m[32m+};
(B[m 
 use crate::api::FitbitClient;
 use crate::auth::{FitbitOAuth2, KeychainTokenStore, TokenStore};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/lib.rs:94:
         version: "0.1.0".into(),
         display_name: "Fitbit".into(),
         auth_strategy: AuthStrategy::OAuth2 {
[31m-            scopes: vec![
(B[m[31m-                "activity".into(),
(B[m[31m-                "sleep".into(),
(B[m[31m-                "heartrate".into(),
(B[m[31m-            ],
(B[m[32m+            scopes: vec!["activity".into(), "sleep".into(), "heartrate".into()],
(B[m         },
         sync_mode: SyncMode::Polling {
             cadence_seconds: 300,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-fitbit/src/models.rs:22:
             .into_iter()
             .filter_map(|v| serde_json::from_value(v).ok())
             .collect();
[31m-        Activity { summary, activities }
(B[m[32m+        Activity {
(B[m[32m+            summary,
(B[m[32m+            activities,
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:7:
 use serde::de::DeserializeOwned;
 use tracing::warn;
 
[31m-use crate::models::{CalendarList, CalendarListEntry, EventList, GCalEvent, GCalUser, WatchResponse};
(B[m[32m+use crate::models::{
(B[m[32m+    CalendarList, CalendarListEntry, EventList, GCalEvent, GCalUser, WatchResponse,
(B[m[32m+};
(B[m 
 pub const GOOGLE_API_BASE: &str = "https://www.googleapis.com";
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:70:
         let headers = resp.headers().clone();
 
         match status {
[31m-            s if s.is_success() => {
(B[m[31m-                resp.json::<T>().await.map_err(|e| ConnectorError::Schema(e.to_string()))
(B[m[31m-            }
(B[m[32m+            s if s.is_success() => resp
(B[m[32m+                .json::<T>()
(B[m[32m+                .await
(B[m[32m+                .map_err(|e| ConnectorError::Schema(e.to_string())),
(B[m             StatusCode::UNAUTHORIZED => Err(ConnectorError::Auth("401 from Google".into())),
             StatusCode::FORBIDDEN => {
                 // Google's 403 is either:
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:123:
         let headers = resp.headers().clone();
 
         match status {
[31m-            s if s.is_success() => {
(B[m[31m-                resp.json::<R>().await.map_err(|e| ConnectorError::Schema(e.to_string()))
(B[m[31m-            }
(B[m[32m+            s if s.is_success() => resp
(B[m[32m+                .json::<R>()
(B[m[32m+                .await
(B[m[32m+                .map_err(|e| ConnectorError::Schema(e.to_string())),
(B[m             StatusCode::UNAUTHORIZED => Err(ConnectorError::Auth("401 from Google".into())),
             StatusCode::FORBIDDEN => {
                 let body_text = resp.text().await.unwrap_or_default();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:145:
             }
             other => {
                 let body_text = resp.text().await.unwrap_or_default();
[31m-                Err(ConnectorError::Network(format!("HTTP {other}: {}", truncate(&body_text, 128))))
(B[m[32m+                Err(ConnectorError::Network(format!(
(B[m[32m+                    "HTTP {other}: {}",
(B[m[32m+                    truncate(&body_text, 128)
(B[m[32m+                )))
(B[m             }
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:158:
         &self,
         cursor: Option<String>,
     ) -> Result<Page<CalendarListEntry>, ConnectorError> {
[31m-        let mut url = format!("{}/calendar/v3/users/me/calendarList?maxResults=250", self.base_url);
(B[m[32m+        let mut url = format!(
(B[m[32m+            "{}/calendar/v3/users/me/calendarList?maxResults=250",
(B[m[32m+            self.base_url
(B[m[32m+        );
(B[m         if let Some(tok) = cursor {
             url.push_str("&pageToken=");
             url.push_str(&urlencode(&tok));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:165:
         }
         let body: CalendarList = self.get_json(&url).await?;
[31m-        Ok(Page { items: body.items, next_cursor: body.next_page_token })
(B[m[32m+        Ok(Page {
(B[m[32m+            items: body.items,
(B[m[32m+            next_cursor: body.next_page_token,
(B[m[32m+        })
(B[m     }
 
     /// List events on a single calendar, expanded as single instances and
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:193:
             url.push_str(&urlencode(&tok));
         }
         let body: EventList = self.get_json(&url).await?;
[31m-        Ok(Page { items: body.items, next_cursor: body.next_page_token })
(B[m[32m+        Ok(Page {
(B[m[32m+            items: body.items,
(B[m[32m+            next_cursor: body.next_page_token,
(B[m[32m+        })
(B[m     }
 
     /// Fetch the user's identity for health-check purposes.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:269:
         &self,
         calendar_id: &str,
     ) -> Result<WatchResponse, ConnectorError> {
[31m-        let webhook_url = std::env::var("FOCALPOINT_GCAL_WEBHOOK_URL")
(B[m[31m-            .map_err(|_| {
(B[m[31m-                ConnectorError::Auth(
(B[m[31m-                    "FOCALPOINT_GCAL_WEBHOOK_URL not set; cannot enable watch notifications"
(B[m[31m-                        .into(),
(B[m[31m-                )
(B[m[31m-            })?;
(B[m[32m+        let webhook_url = std::env::var("FOCALPOINT_GCAL_WEBHOOK_URL").map_err(|_| {
(B[m[32m+            ConnectorError::Auth(
(B[m[32m+                "FOCALPOINT_GCAL_WEBHOOK_URL not set; cannot enable watch notifications".into(),
(B[m[32m+            )
(B[m[32m+        })?;
(B[m 
         let url = format!(
             "{}/calendar/v3/calendars/{}/events/watch",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:289:
             "address": webhook_url,
         });
 
[31m-        self.post_json::<serde_json::Value, WatchResponse>(&url, &req).await
(B[m[32m+        self.post_json::<serde_json::Value, WatchResponse>(&url, &req)
(B[m[32m+            .await
(B[m     }
 
     /// Stop push notifications for a watch channel.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:310:
             "resourceId": resource_id,
         });
 
[31m-        self.post_json::<serde_json::Value, serde_json::Value>(&url, &req).await?;
(B[m[32m+        self.post_json::<serde_json::Value, serde_json::Value>(&url, &req)
(B[m[32m+            .await?;
(B[m         Ok(())
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:333:
         );
 
         let body: EventList = self.get_json(&url).await?;
[31m-        Ok(Page { items: body.items, next_cursor: body.next_page_token })
(B[m[32m+        Ok(Page {
(B[m[32m+            items: body.items,
(B[m[32m+            next_cursor: body.next_page_token,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:420:
 
         let client = GCalClient::with_http(server.uri(), "TOK", reqwest::Client::new());
         let page = client
[31m-            .list_events("primary", "2026-05-01T00:00:00Z", "2026-05-08T00:00:00Z", None)
(B[m[32m+            .list_events(
(B[m[32m+                "primary",
(B[m[32m+                "2026-05-01T00:00:00Z",
(B[m[32m+                "2026-05-08T00:00:00Z",
(B[m[32m+                None,
(B[m[32m+            )
(B[m             .await
             .unwrap();
         assert_eq!(page.items.len(), 1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:447:
         Mock::given(method("GET"))
             .and(path("/oauth2/v2/userinfo"))
             .respond_with(
[31m-                ResponseTemplate::new(403).insert_header("Retry-After", "42").set_body_json(
(B[m[31m-                    serde_json::json!({
(B[m[32m+                ResponseTemplate::new(403)
(B[m[32m+                    .insert_header("Retry-After", "42")
(B[m[32m+                    .set_body_json(serde_json::json!({
(B[m                         "error": {
                             "code": 403,
                             "errors": [{"reason": "rateLimitExceeded"}]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:455:
                         }
[31m-                    }),
(B[m[31m-                ),
(B[m[32m+                    })),
(B[m             )
             .mount(&server)
             .await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:516:
     #[test]
     fn urlencode_escapes_nonalpha() {
         assert_eq!(urlencode("a@b.com"), "a%40b.com");
[31m-        assert_eq!(urlencode("2026-05-01T00:00:00Z"), "2026-05-01T00%3A00%3A00Z");
(B[m[32m+        assert_eq!(
(B[m[32m+            urlencode("2026-05-01T00:00:00Z"),
(B[m[32m+            "2026-05-01T00%3A00%3A00Z"
(B[m[32m+        );
(B[m         assert_eq!(urlencode("primary"), "primary");
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:565:
             .await;
 
         let client = GCalClient::with_http(server.uri(), "TOK", reqwest::Client::new());
[31m-        let events = client.batch_get_events("primary", &["e1", "e2"]).await.unwrap();
(B[m[32m+        let events = client
(B[m[32m+            .batch_get_events("primary", &["e1", "e2"])
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(events.len(), 2);
         assert_eq!(events[0].id, "e1");
         assert_eq!(events[1].id, "e2");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:663:
 
         let client = GCalClient::with_http(server.uri(), "TOK", reqwest::Client::new());
         let page = client
[31m-            .expand_recurring_events("primary", "recurring1", "2026-05-01T00:00:00Z", "2026-05-31T23:59:59Z")
(B[m[32m+            .expand_recurring_events(
(B[m[32m+                "primary",
(B[m[32m+                "recurring1",
(B[m[32m+                "2026-05-01T00:00:00Z",
(B[m[32m+                "2026-05-31T23:59:59Z",
(B[m[32m+            )
(B[m             .await
             .unwrap();
         assert_eq!(page.items.len(), 2);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:670:
[31m-        assert!(page.items.iter().all(|e| e.recurring_event_id.as_deref() == Some("recurring1")));
(B[m[32m+        assert!(page
(B[m[32m+            .items
(B[m[32m+            .iter()
(B[m[32m+            .all(|e| e.recurring_event_id.as_deref() == Some("recurring1")));
(B[m     }
 
     #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/api.rs:694:
             .mount(&server)
             .await;
         let client = GCalClient::with_http(server.uri(), "t", reqwest::Client::new());
[31m-        let err = client.batch_get_events("primary", &["e1"]).await.unwrap_err();
(B[m[32m+        let err = client
(B[m[32m+            .batch_get_events("primary", &["e1"])
(B[m[32m+            .await
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, ConnectorError::Auth(_)));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/auth.rs:89:
         Self::default()
     }
     pub fn with_token(token: GCalToken) -> Self {
[31m-        Self { inner: Mutex::new(Some(token)) }
(B[m[32m+        Self {
(B[m[32m+            inner: Mutex::new(Some(token)),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/auth.rs:123:
         account: impl Into<String>,
         inner: std::sync::Arc<dyn focus_crypto::SecureSecretStore>,
     ) -> Self {
[31m-        Self { account: account.into(), inner }
(B[m[32m+        Self {
(B[m[32m+            account: account.into(),
(B[m[32m+            inner,
(B[m[32m+        }
(B[m     }
 
     pub fn with_default_backend(service: &str, account: impl Into<String>) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/auth.rs:204:
         Ok(Self {
             config,
             client,
[31m-            token_url_override: if token_url == GOOGLE_TOKEN_URL { None } else { Some(token_url) },
(B[m[31m-            auth_url_override: if auth_url == GOOGLE_AUTH_URL { None } else { Some(auth_url) },
(B[m[32m+            token_url_override: if token_url == GOOGLE_TOKEN_URL {
(B[m[32m+                None
(B[m[32m+            } else {
(B[m[32m+                Some(token_url)
(B[m[32m+            },
(B[m[32m+            auth_url_override: if auth_url == GOOGLE_AUTH_URL {
(B[m[32m+                None
(B[m[32m+            } else {
(B[m[32m+                Some(auth_url)
(B[m[32m+            },
(B[m         })
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/auth.rs:282:
 
 fn to_token(resp: &oauth2::basic::BasicTokenResponse) -> GCalToken {
     let now = Utc::now();
[31m-    let expires_at =
(B[m[31m-        resp.expires_in().and_then(|d| chrono::Duration::from_std(d).ok()).map(|d| now + d);
(B[m[32m+    let expires_at = resp
(B[m[32m+        .expires_in()
(B[m[32m+        .and_then(|d| chrono::Duration::from_std(d).ok())
(B[m[32m+        .map(|d| now + d);
(B[m     GCalToken {
         access_token: resp.access_token().secret().clone(),
         refresh_token: resp.refresh_token().map(|r| r.secret().clone()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:50:
                 "transparency": ev.transparency,
                 "all_day": is_all_day(ev),
             }),
[31m-            raw_ref: Some(TraceRef { source: CONNECTOR_ID.into(), id: format!("event:{}", ev.id) }),
(B[m[32m+            raw_ref: Some(TraceRef {
(B[m[32m+                source: CONNECTOR_ID.into(),
(B[m[32m+                id: format!("event:{}", ev.id),
(B[m[32m+            }),
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:76:
                 "summary": ev.summary,
                 "end_at": end,
             }),
[31m-            raw_ref: Some(TraceRef { source: CONNECTOR_ID.into(), id: format!("event:{}", ev.id) }),
(B[m[32m+            raw_ref: Some(TraceRef {
(B[m[32m+                source: CONNECTOR_ID.into(),
(B[m[32m+                id: format!("event:{}", ev.id),
(B[m[32m+            }),
(B[m         })
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:116:
 pub fn start_datetime(dt: Option<&EventDateTime>) -> Option<DateTime<Utc>> {
     let dt = dt?;
     if let Some(s) = dt.date_time.as_deref() {
[31m-        return DateTime::parse_from_rfc3339(s).ok().map(|d| d.with_timezone(&Utc));
(B[m[32m+        return DateTime::parse_from_rfc3339(s)
(B[m[32m+            .ok()
(B[m[32m+            .map(|d| d.with_timezone(&Utc));
(B[m     }
     if let Some(s) = dt.date.as_deref() {
         return NaiveDate::parse_from_str(s, "%Y-%m-%d")
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:134:
 
 /// An event is "all day" iff the start has a `date` field (not `dateTime`).
 pub fn is_all_day(ev: &GCalEvent) -> bool {
[31m-    ev.start.as_ref().map(|s| s.date.is_some() && s.date_time.is_none()).unwrap_or(false)
(B[m[32m+    ev.start
(B[m[32m+        .as_ref()
(B[m[32m+        .map(|s| s.date.is_some() && s.date_time.is_none())
(B[m[32m+        .unwrap_or(false)
(B[m }
 
 #[cfg(test)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:179:
     fn maps_timed_event_to_event_started() {
         let e = timed_event("e1", "2026-05-01T09:00:00Z", "2026-05-01T10:00:00Z");
         let ev = GCalEventMapper::map_event_started(&e, acct(), "primary");
[31m-        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::EventStarted));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::EventStarted)
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["calendar_id"], "primary");
         assert_eq!(ev.payload["all_day"], false);
         assert!(ev.dedupe_key.0.starts_with("gcal:event_started:e1:"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:189:
     fn maps_event_ended_when_end_present() {
         let e = timed_event("e1", "2026-05-01T09:00:00Z", "2026-05-01T10:00:00Z");
         let ev = GCalEventMapper::map_event_ended(&e, acct(), "primary").unwrap();
[31m-        assert_eq!(ev.event_type, EventType::WellKnown(WellKnownEventType::EventEnded));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::EventEnded)
(B[m[32m+        );
(B[m         assert!(ev.dedupe_key.0.starts_with("gcal:event_ended:e1:"));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:230:
         };
         let ev = GCalEventMapper::map_event_started(&e, acct(), "primary");
         assert_eq!(ev.payload["all_day"], true);
[31m-        assert_eq!(ev.occurred_at, Utc.with_ymd_and_hms(2026, 7, 4, 0, 0, 0).unwrap());
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.occurred_at,
(B[m[32m+            Utc.with_ymd_and_hms(2026, 7, 4, 0, 0, 0).unwrap()
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/events.rs:246:
             color: None,
         };
         let ev = GCalEventMapper::map_calendar_subscribed(&c, acct());
[31m-        assert_eq!(ev.event_type, EventType::Custom("gcal:calendar_subscribed".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ev.event_type,
(B[m[32m+            EventType::Custom("gcal:calendar_subscribed".into())
(B[m[32m+        );
(B[m         assert_eq!(ev.payload["calendar_id"], "primary");
         assert_eq!(ev.payload["primary"], true);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/lib.rs:87:
 
     pub fn build(self) -> GCalConnector {
         let http = self.http.unwrap_or_default();
[31m-        let store = self.token_store.unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
(B[m[32m+        let store = self
(B[m[32m+            .token_store
(B[m[32m+            .unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
(B[m         let client = GCalClient::with_http(&self.base_url, "", http);
[31m-        let scopes = self.scopes.unwrap_or_else(|| vec![CALENDAR_READONLY_SCOPE.into()]);
(B[m[32m+        let scopes = self
(B[m[32m+            .scopes
(B[m[32m+            .unwrap_or_else(|| vec![CALENDAR_READONLY_SCOPE.into()]);
(B[m         GCalConnector {
             manifest: default_manifest(scopes),
             account_id: self.account_id,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/lib.rs:106:
         version: "0.1.0".into(),
         display_name: "Google Calendar".into(),
         auth_strategy: AuthStrategy::OAuth2 { scopes },
[31m-        sync_mode: SyncMode::Polling { cadence_seconds: 900 },
(B[m[32m+        sync_mode: SyncMode::Polling {
(B[m[32m+            cadence_seconds: 900,
(B[m[32m+        },
(B[m         capabilities: vec![],
         entity_types: vec!["calendar".into(), "event".into()],
         event_types: vec![
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/lib.rs:234:
 
         let mut events = Vec::new();
         for cal in &cal_page.items {
[31m-            events.push(GCalEventMapper::map_calendar_subscribed(cal, self.account_id));
(B[m[32m+            events.push(GCalEventMapper::map_calendar_subscribed(
(B[m[32m+                cal,
(B[m[32m+                self.account_id,
(B[m[32m+            ));
(B[m 
             let gcal_events = {
                 let c = client.clone();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/lib.rs:260:
             };
 
             for e in &gcal_events {
[31m-                events.push(GCalEventMapper::map_event_started(e, self.account_id, &cal.id));
(B[m[32m+                events.push(GCalEventMapper::map_event_started(
(B[m[32m+                    e,
(B[m[32m+                    self.account_id,
(B[m[32m+                    &cal.id,
(B[m[32m+                ));
(B[m                 if let Some(end_ev) = GCalEventMapper::map_event_ended(e, self.account_id, &cal.id)
                 {
                     events.push(end_ev);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/lib.rs:268:
             }
         }
 
[31m-        Ok(SyncOutcome { events, next_cursor: cal_page.next_cursor, partial: false })
(B[m[32m+        Ok(SyncOutcome {
(B[m[32m+            events,
(B[m[32m+            next_cursor: cal_page.next_cursor,
(B[m[32m+            partial: false,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/lib.rs:305:
     fn manifest_declares_event_types() {
         let m = default_manifest(vec![]);
         for want in ["event_started", "event_ended", "gcal:calendar_subscribed"] {
[31m-            assert!(m.event_types.iter().any(|e| e == want), "missing event: {want}");
(B[m[32m+            assert!(
(B[m[32m+                m.event_types.iter().any(|e| e == want),
(B[m[32m+                "missing event: {want}"
(B[m[32m+            );
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-gcal/src/models.rs:207:
     fn parses_event_all_day() {
         let j = r#"{"id":"evt2","summary":"Holiday","start":{"date":"2026-07-04"},"end":{"date":"2026-07-05"}}"#;
         let e: GCalEvent = serde_json::from_str(j).unwrap();
[31m-        assert_eq!(e.start.as_ref().and_then(|s| s.date.as_deref()), Some("2026-07-04"));
(B[m[32m+        assert_eq!(
(B[m[32m+            e.start.as_ref().and_then(|s| s.date.as_deref()),
(B[m[32m+            Some("2026-07-04")
(B[m[32m+        );
(B[m         assert!(e.start.as_ref().unwrap().date_time.is_none());
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:56:
         token: GitHubToken,
         http: reqwest::Client,
     ) -> Self {
[31m-        Self { base_url: base_url.into().trim_end_matches('/').to_string(), token, http }
(B[m[32m+        Self {
(B[m[32m+            base_url: base_url.into().trim_end_matches('/').to_string(),
(B[m[32m+            token,
(B[m[32m+            http,
(B[m[32m+        }
(B[m     }
 
     fn auth_headers(&self) -> HeaderMap {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:65:
             h.insert(AUTHORIZATION, v);
         }
         h.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
[31m-        h.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
(B[m[31m-        h.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));
(B[m[32m+        h.insert(
(B[m[32m+            ACCEPT,
(B[m[32m+            HeaderValue::from_static("application/vnd.github+json"),
(B[m[32m+        );
(B[m[32m+        h.insert(
(B[m[32m+            "X-GitHub-Api-Version",
(B[m[32m+            HeaderValue::from_static("2022-11-28"),
(B[m[32m+        );
(B[m         h
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:87:
 
         match status {
             s if s.is_success() => {
[31m-                let body: T =
(B[m[31m-                    resp.json().await.map_err(|e| ConnectorError::Schema(e.to_string()))?;
(B[m[32m+                let body: T = resp
(B[m[32m+                    .json()
(B[m[32m+                    .await
(B[m[32m+                    .map_err(|e| ConnectorError::Schema(e.to_string()))?;
(B[m                 Ok((body, headers))
             }
             StatusCode::UNAUTHORIZED => Err(ConnectorError::Unauthorized("401 from GitHub".into())),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:178:
         &self,
         cursor: Option<String>,
     ) -> Result<Page<GitHubRepository>, ConnectorError> {
[31m-        let initial = format!("{}/user/repos?affiliation=owner,collaborator&sort=pushed&per_page=100", self.base_url);
(B[m[32m+        let initial = format!(
(B[m[32m+            "{}/user/repos?affiliation=owner,collaborator&sort=pushed&per_page=100",
(B[m[32m+            self.base_url
(B[m[32m+        );
(B[m         let mut url = cursor.unwrap_or(initial);
         let mut items: Vec<GitHubRepository> = Vec::new();
         let mut next_cursor: Option<String> = None;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:246:
         repo: &str,
         number: u32,
     ) -> Result<GitHubPullRequest, ConnectorError> {
[31m-        let url = format!("{}/repos/{}/{}/pulls/{}", self.base_url, owner, repo, number);
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/repos/{}/{}/pulls/{}",
(B[m[32m+            self.base_url, owner, repo, number
(B[m[32m+        );
(B[m         let (pr, _) = self.get_json::<GitHubPullRequest>(&url).await?;
         Ok(pr)
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:262:
             "{}/repos/{}/{}/commits/{}/check-runs?per_page=100",
             self.base_url, owner, repo, commit_ref
         );
[31m-        let (resp, headers) = self.get_json::<GitHubPaginatedList<GitHubCheckRun>>(&url).await?;
(B[m[32m+        let (resp, headers) = self
(B[m[32m+            .get_json::<GitHubPaginatedList<GitHubCheckRun>>(&url)
(B[m[32m+            .await?;
(B[m         let next_cursor = parse_next_link(headers.get(LINK).and_then(|v| v.to_str().ok()));
[31m-        Ok(Page { items: resp.items, next_cursor })
(B[m[32m+        Ok(Page {
(B[m[32m+            items: resp.items,
(B[m[32m+            next_cursor,
(B[m[32m+        })
(B[m     }
 
     /// `GET /repos/{owner}/{repo}/actions/runs` — list workflow runs.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:273:
         owner: &str,
         repo: &str,
     ) -> Result<Page<GitHubWorkflowRun>, ConnectorError> {
[31m-        let url = format!("{}/repos/{}/{}/actions/runs?per_page=25", self.base_url, owner, repo);
(B[m[31m-        let (resp, headers) = self.get_json::<GitHubPaginatedList<GitHubWorkflowRun>>(&url).await?;
(B[m[32m+        let url = format!(
(B[m[32m+            "{}/repos/{}/{}/actions/runs?per_page=25",
(B[m[32m+            self.base_url, owner, repo
(B[m[32m+        );
(B[m[32m+        let (resp, headers) = self
(B[m[32m+            .get_json::<GitHubPaginatedList<GitHubWorkflowRun>>(&url)
(B[m[32m+            .await?;
(B[m         let next_cursor = parse_next_link(headers.get(LINK).and_then(|v| v.to_str().ok()));
[31m-        Ok(Page { items: resp.items, next_cursor })
(B[m[32m+        Ok(Page {
(B[m[32m+            items: resp.items,
(B[m[32m+            next_cursor,
(B[m[32m+        })
(B[m     }
 
     /// POST /graphql — execute a GraphQL query.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:283:
     /// Returns the raw GraphQL response (data + errors if present).
[31m-    pub async fn graphql(
(B[m[31m-        &self,
(B[m[31m-        query: &str,
(B[m[31m-    ) -> Result<serde_json::Value, ConnectorError> {
(B[m[32m+    pub async fn graphql(&self, query: &str) -> Result<serde_json::Value, ConnectorError> {
(B[m         let url = format!("{}/graphql", self.base_url);
         let req_body = serde_json::json!({"query": query});
         let resp = self
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:299:
         let status = resp.status();
         match status {
             s if s.is_success() => {
[31m-                let body = resp.json::<serde_json::Value>().await.map_err(|e| {
(B[m[31m-                    ConnectorError::Schema(e.to_string())
(B[m[31m-                })?;
(B[m[32m+                let body = resp
(B[m[32m+                    .json::<serde_json::Value>()
(B[m[32m+                    .await
(B[m[32m+                    .map_err(|e| ConnectorError::Schema(e.to_string()))?;
(B[m                 Ok(body)
             }
             StatusCode::UNAUTHORIZED => Err(ConnectorError::Unauthorized("401 from GitHub".into())),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:320:
             }
             other => {
                 let body_text = resp.text().await.unwrap_or_default();
[31m-                Err(ConnectorError::Network(format!("HTTP {other}: {}", truncate(&body_text, 128))))
(B[m[32m+                Err(ConnectorError::Network(format!(
(B[m[32m+                    "HTTP {other}: {}",
(B[m[32m+                    truncate(&body_text, 128)
(B[m[32m+                )))
(B[m             }
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:333:
         let seg = part.trim();
         // <https://api.github.com/...?page=2>; rel="next"
         let (url_part, rel_part) = seg.split_once(';')?;
[31m-        let url = url_part.trim().trim_start_matches('<').trim_end_matches('>');
(B[m[32m+        let url = url_part
(B[m[32m+            .trim()
(B[m[32m+            .trim_start_matches('<')
(B[m[32m+            .trim_end_matches('>');
(B[m         if rel_part.contains("rel=\"next\"") {
             return Some(url.to_string());
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:402:
         let server = wiremock::MockServer::start().await;
         wiremock::Mock::given(wiremock::matchers::method("GET"))
             .and(wiremock::matchers::path("/user/repos"))
[31m-            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!([
(B[m[31m-                {
(B[m[31m-                    "id": 1,
(B[m[31m-                    "name": "repo1",
(B[m[31m-                    "full_name": "user/repo1",
(B[m[31m-                    "private": false,
(B[m[31m-                    "owner": {"id": 100, "login": "user"},
(B[m[31m-                    "stargazers_count": 5,
(B[m[31m-                    "pushed_at": "2026-05-01T10:00:00Z"
(B[m[31m-                }
(B[m[31m-            ])))
(B[m[32m+            .respond_with(
(B[m[32m+                wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!([
(B[m[32m+                    {
(B[m[32m+                        "id": 1,
(B[m[32m+                        "name": "repo1",
(B[m[32m+                        "full_name": "user/repo1",
(B[m[32m+                        "private": false,
(B[m[32m+                        "owner": {"id": 100, "login": "user"},
(B[m[32m+                        "stargazers_count": 5,
(B[m[32m+                        "pushed_at": "2026-05-01T10:00:00Z"
(B[m[32m+                    }
(B[m[32m+                ])),
(B[m[32m+            )
(B[m             .mount(&server)
             .await;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:428:
         let server = wiremock::MockServer::start().await;
         wiremock::Mock::given(wiremock::matchers::method("GET"))
             .and(wiremock::matchers::path("/issues"))
[31m-            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!([
(B[m[31m-                {
(B[m[31m-                    "id": 1,
(B[m[31m-                    "number": 42,
(B[m[31m-                    "title": "Bug in feature X",
(B[m[31m-                    "state": "open",
(B[m[31m-                    "user": {"id": 100, "login": "user"},
(B[m[31m-                    "repository_url": "https://api.github.com/repos/user/repo1",
(B[m[31m-                    "html_url": "https://github.com/user/repo1/issues/42",
(B[m[31m-                    "created_at": "2026-05-01T09:00:00Z",
(B[m[31m-                    "updated_at": "2026-05-02T10:00:00Z"
(B[m[31m-                }
(B[m[31m-            ])))
(B[m[32m+            .respond_with(
(B[m[32m+                wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!([
(B[m[32m+                    {
(B[m[32m+                        "id": 1,
(B[m[32m+                        "number": 42,
(B[m[32m+                        "title": "Bug in feature X",
(B[m[32m+                        "state": "open",
(B[m[32m+                        "user": {"id": 100, "login": "user"},
(B[m[32m+                        "repository_url": "https://api.github.com/repos/user/repo1",
(B[m[32m+                        "html_url": "https://github.com/user/repo1/issues/42",
(B[m[32m+                        "created_at": "2026-05-01T09:00:00Z",
(B[m[32m+                        "updated_at": "2026-05-02T10:00:00Z"
(B[m[32m+                    }
(B[m[32m+                ])),
(B[m[32m+            )
(B[m             .mount(&server)
             .await;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:456:
         let server = wiremock::MockServer::start().await;
         wiremock::Mock::given(wiremock::matchers::method("GET"))
             .and(wiremock::matchers::path("/repos/owner/repo/pulls/123"))
[31m-            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[31m-                "id": 1,
(B[m[31m-                "number": 123,
(B[m[31m-                "title": "Add feature Y",
(B[m[31m-                "state": "merged",
(B[m[31m-                "user": {"id": 100, "login": "contributor"},
(B[m[31m-                "merged": true,
(B[m[31m-                "merged_at": "2026-05-03T12:00:00Z",
(B[m[31m-                "html_url": "https://github.com/owner/repo/pull/123",
(B[m[31m-                "created_at": "2026-05-01T09:00:00Z",
(B[m[31m-                "updated_at": "2026-05-03T12:00:00Z",
(B[m[31m-                "review_comments": 5
(B[m[31m-            })))
(B[m[32m+            .respond_with(
(B[m[32m+                wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[32m+                    "id": 1,
(B[m[32m+                    "number": 123,
(B[m[32m+                    "title": "Add feature Y",
(B[m[32m+                    "state": "merged",
(B[m[32m+                    "user": {"id": 100, "login": "contributor"},
(B[m[32m+                    "merged": true,
(B[m[32m+                    "merged_at": "2026-05-03T12:00:00Z",
(B[m[32m+                    "html_url": "https://github.com/owner/repo/pull/123",
(B[m[32m+                    "created_at": "2026-05-01T09:00:00Z",
(B[m[32m+                    "updated_at": "2026-05-03T12:00:00Z",
(B[m[32m+                    "review_comments": 5
(B[m[32m+                })),
(B[m[32m+            )
(B[m             .mount(&server)
             .await;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:484:
     async fn list_check_runs_succeeds() {
         let server = wiremock::MockServer::start().await;
         wiremock::Mock::given(wiremock::matchers::method("GET"))
[31m-            .and(wiremock::matchers::path("/repos/owner/repo/commits/abc123/check-runs"))
(B[m[31m-            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[31m-                "total_count": 1,
(B[m[31m-                "items": [
(B[m[31m-                    {
(B[m[31m-                        "id": 1,
(B[m[31m-                        "name": "build",
(B[m[31m-                        "status": "completed",
(B[m[31m-                        "conclusion": "success",
(B[m[31m-                        "started_at": "2026-05-01T09:00:00Z",
(B[m[31m-                        "completed_at": "2026-05-01T09:30:00Z",
(B[m[31m-                        "html_url": "https://github.com/owner/repo/runs/1"
(B[m[31m-                    }
(B[m[31m-                ]
(B[m[31m-            })))
(B[m[32m+            .and(wiremock::matchers::path(
(B[m[32m+                "/repos/owner/repo/commits/abc123/check-runs",
(B[m[32m+            ))
(B[m[32m+            .respond_with(
(B[m[32m+                wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[32m+                    "total_count": 1,
(B[m[32m+                    "items": [
(B[m[32m+                        {
(B[m[32m+                            "id": 1,
(B[m[32m+                            "name": "build",
(B[m[32m+                            "status": "completed",
(B[m[32m+                            "conclusion": "success",
(B[m[32m+                            "started_at": "2026-05-01T09:00:00Z",
(B[m[32m+                            "completed_at": "2026-05-01T09:30:00Z",
(B[m[32m+                            "html_url": "https://github.com/owner/repo/runs/1"
(B[m[32m+                        }
(B[m[32m+                    ]
(B[m[32m+                })),
(B[m[32m+            )
(B[m             .mount(&server)
             .await;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:505:
         let token = GitHubToken::new("test_token");
         let client = GitHubClient::with_http(server.uri(), token, reqwest::Client::new());
[31m-        let page = client.list_check_runs("owner", "repo", "abc123").await.unwrap();
(B[m[32m+        let page = client
(B[m[32m+            .list_check_runs("owner", "repo", "abc123")
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(page.items.len(), 1);
         assert_eq!(page.items[0].conclusion.as_deref(), Some("success"));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:514:
         let server = wiremock::MockServer::start().await;
         wiremock::Mock::given(wiremock::matchers::method("GET"))
             .and(wiremock::matchers::path("/repos/owner/repo/actions/runs"))
[31m-            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[31m-                "total_count": 1,
(B[m[31m-                "items": [
(B[m[31m-                    {
(B[m[31m-                        "id": 1,
(B[m[31m-                        "name": "CI",
(B[m[31m-                        "status": "completed",
(B[m[31m-                        "conclusion": "success",
(B[m[31m-                        "created_at": "2026-05-01T09:00:00Z",
(B[m[31m-                        "updated_at": "2026-05-01T10:00:00Z",
(B[m[31m-                        "html_url": "https://github.com/owner/repo/actions/runs/1"
(B[m[31m-                    }
(B[m[31m-                ]
(B[m[31m-            })))
(B[m[32m+            .respond_with(
(B[m[32m+                wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[32m+                    "total_count": 1,
(B[m[32m+                    "items": [
(B[m[32m+                        {
(B[m[32m+                            "id": 1,
(B[m[32m+                            "name": "CI",
(B[m[32m+                            "status": "completed",
(B[m[32m+                            "conclusion": "success",
(B[m[32m+                            "created_at": "2026-05-01T09:00:00Z",
(B[m[32m+                            "updated_at": "2026-05-01T10:00:00Z",
(B[m[32m+                            "html_url": "https://github.com/owner/repo/actions/runs/1"
(B[m[32m+                        }
(B[m[32m+                    ]
(B[m[32m+                })),
(B[m[32m+            )
(B[m             .mount(&server)
             .await;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:543:
         let server = wiremock::MockServer::start().await;
         wiremock::Mock::given(wiremock::matchers::method("POST"))
             .and(wiremock::matchers::path("/graphql"))
[31m-            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[31m-                "data": {
(B[m[31m-                    "viewer": {"login": "testuser"}
(B[m[31m-                }
(B[m[31m-            })))
(B[m[32m+            .respond_with(
(B[m[32m+                wiremock::ResponseTemplate::new(200).set_body_json(serde_json::json!({
(B[m[32m+                    "data": {
(B[m[32m+                        "viewer": {"login": "testuser"}
(B[m[32m+                    }
(B[m[32m+                })),
(B[m[32m+            )
(B[m             .mount(&server)
             .await;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/api.rs:583:
 
         let token = GitHubToken::new("test_token");
         let client = GitHubClient::with_http(server.uri(), token, reqwest::Client::new());
[31m-        let err = client.get_pull_request("owner", "repo", 123).await.unwrap_err();
(B[m[32m+        let err = client
(B[m[32m+            .get_pull_request("owner", "repo", 123)
(B[m[32m+            .await
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, ConnectorError::Forbidden(_)));
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/auth.rs:44:
 
 impl GitHubToken {
     pub fn new(pat: impl Into<String>) -> Self {
[31m-        Self { access_token: SecretString::from(pat.into()), captured_at: Utc::now() }
(B[m[32m+        Self {
(B[m[32m+            access_token: SecretString::from(pat.into()),
(B[m[32m+            captured_at: Utc::now(),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/auth.rs:83:
         Self::default()
     }
     pub fn with_token(token: GitHubToken) -> Self {
[31m-        Self { inner: std::sync::Mutex::new(Some(token)) }
(B[m[32m+        Self {
(B[m[32m+            inner: std::sync::Mutex::new(Some(token)),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/auth.rs:119:
         account: impl Into<String>,
         inner: std::sync::Arc<dyn focus_crypto::SecureSecretStore>,
     ) -> Self {
[31m-        Self { account: account.into(), inner }
(B[m[32m+        Self {
(B[m[32m+            account: account.into(),
(B[m[32m+            inner,
(B[m[32m+        }
(B[m     }
 
     pub fn with_default_backend(service: &str, account: impl Into<String>) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:44:
             dedupe_key: dedupe_key(&ev.id),
             confidence: 1.0,
             payload: build_payload(ev),
[31m-            raw_ref: Some(TraceRef { source: CONNECTOR_ID.into(), id: format!("event:{}", ev.id) }),
(B[m[32m+            raw_ref: Some(TraceRef {
(B[m[32m+                source: CONNECTOR_ID.into(),
(B[m[32m+                id: format!("event:{}", ev.id),
(B[m[32m+            }),
(B[m         })
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:142:
         GitHubEvent {
             id: "12345".into(),
             event_type: ty.into(),
[31m-            actor: GitHubActor { id: 1, login: "octocat".into() },
(B[m[31m-            repo: GitHubRepo { id: 42, name: "octo/repo".into() },
(B[m[32m+            actor: GitHubActor {
(B[m[32m+                id: 1,
(B[m[32m+                login: "octocat".into(),
(B[m[32m+            },
(B[m[32m+            repo: GitHubRepo {
(B[m[32m+                id: 42,
(B[m[32m+                name: "octo/repo".into(),
(B[m[32m+            },
(B[m             created_at: Utc.with_ymd_and_hms(2026, 4, 1, 12, 0, 0).unwrap(),
             public: true,
             payload,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:161:
 
     #[test]
     fn maps_pr_opened() {
[31m-        let e =
(B[m[31m-            ev("PullRequestEvent", json!({"action": "opened", "pull_request": {"merged": false}}));
(B[m[32m+        let e = ev(
(B[m[32m+            "PullRequestEvent",
(B[m[32m+            json!({"action": "opened", "pull_request": {"merged": false}}),
(B[m[32m+        );
(B[m         let ne = GitHubEventMapper::map(&e, Uuid::nil()).unwrap();
         assert_eq!(ne.event_type, EventType::Custom("github.pr.opened".into()));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:169:
 
     #[test]
     fn maps_pr_merged_vs_closed() {
[31m-        let merged =
(B[m[31m-            ev("PullRequestEvent", json!({"action": "closed", "pull_request": {"merged": true}}));
(B[m[32m+        let merged = ev(
(B[m[32m+            "PullRequestEvent",
(B[m[32m+            json!({"action": "closed", "pull_request": {"merged": true}}),
(B[m[32m+        );
(B[m         let ne = GitHubEventMapper::map(&merged, Uuid::nil()).unwrap();
         assert_eq!(ne.event_type, EventType::Custom("github.pr.merged".into()));
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:177:
[31m-        let closed =
(B[m[31m-            ev("PullRequestEvent", json!({"action": "closed", "pull_request": {"merged": false}}));
(B[m[32m+        let closed = ev(
(B[m[32m+            "PullRequestEvent",
(B[m[32m+            json!({"action": "closed", "pull_request": {"merged": false}}),
(B[m[32m+        );
(B[m         let ne2 = GitHubEventMapper::map(&closed, Uuid::nil()).unwrap();
         assert_eq!(ne2.event_type, EventType::Custom("github.pr.closed".into()));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:184:
     fn maps_issue_closed() {
         let e = ev("IssuesEvent", json!({"action": "closed"}));
         let ne = GitHubEventMapper::map(&e, Uuid::nil()).unwrap();
[31m-        assert_eq!(ne.event_type, EventType::Custom("github.issue.closed".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ne.event_type,
(B[m[32m+            EventType::Custom("github.issue.closed".into())
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:231:
     fn maps_pr_review_submitted() {
         let e = ev("PullRequestReviewEvent", json!({"action": "submitted"}));
         let ne = GitHubEventMapper::map(&e, Uuid::nil()).unwrap();
[31m-        assert_eq!(ne.event_type, EventType::Custom("github.pr.review_submitted".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ne.event_type,
(B[m[32m+            EventType::Custom("github.pr.review_submitted".into())
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/events.rs:238:
     fn maps_pr_review_requested() {
         let e = ev("PullRequestReviewEvent", json!({"action": "requested"}));
         let ne = GitHubEventMapper::map(&e, Uuid::nil()).unwrap();
[31m-        assert_eq!(ne.event_type, EventType::Custom("github.pr.review_requested".into()));
(B[m[32m+        assert_eq!(
(B[m[32m+            ne.event_type,
(B[m[32m+            EventType::Custom("github.pr.review_requested".into())
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:82:
 
     pub fn build(self) -> GitHubConnector {
         let http = self.http.unwrap_or_default();
[31m-        let store = self.token_store.unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
(B[m[32m+        let store = self
(B[m[32m+            .token_store
(B[m[32m+            .unwrap_or_else(|| Arc::new(InMemoryTokenStore::new()));
(B[m         GitHubConnector {
             manifest: default_manifest(),
             account_id: self.account_id,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:107:
         display_name: "GitHub".into(),
         // PAT is an opaque bearer — treat as ApiKey from the manifest's POV.
         auth_strategy: AuthStrategy::ApiKey,
[31m-        sync_mode: SyncMode::Polling { cadence_seconds: 900 },
(B[m[32m+        sync_mode: SyncMode::Polling {
(B[m[32m+            cadence_seconds: 900,
(B[m[32m+        },
(B[m         capabilities: vec![],
         entity_types: vec!["event".into()],
         event_types: vec![
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:136:
             .load()
             .await?
             .ok_or_else(|| ConnectorError::Unauthorized("no github token stored".into()))?;
[31m-        Ok(GitHubClient::with_http(&self.base_url, token, self.http.clone()))
(B[m[32m+        Ok(GitHubClient::with_http(
(B[m[32m+            &self.base_url,
(B[m[32m+            token,
(B[m[32m+            self.http.clone(),
(B[m[32m+        ))
(B[m     }
 
     async fn ensure_login(&self, client: &GitHubClient) -> Result<String> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:189:
             }
         }
         let partial = page.next_cursor.is_some();
[31m-        Ok(SyncOutcome { events, next_cursor: page.next_cursor, partial })
(B[m[32m+        Ok(SyncOutcome {
(B[m[32m+            events,
(B[m[32m+            next_cursor: page.next_cursor,
(B[m[32m+            partial,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:206:
 
     #[test]
     fn token_serde_roundtrip_preserves_secret() {
[31m-        let t = GitHubToken { access_token: "ghp_xxx".into(), captured_at: Utc::now() };
(B[m[32m+        let t = GitHubToken {
(B[m[32m+            access_token: "ghp_xxx".into(),
(B[m[32m+            captured_at: Utc::now(),
(B[m[32m+        };
(B[m         let j = serde_json::to_string(&t).unwrap();
         assert!(j.contains("ghp_xxx"));
         let back: GitHubToken = serde_json::from_str(&j).unwrap();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:216:
 
     #[test]
     fn token_debug_redacts_secret() {
[31m-        let t = GitHubToken { access_token: "ghp_supersecret".into(), captured_at: Utc::now() };
(B[m[32m+        let t = GitHubToken {
(B[m[32m+            access_token: "ghp_supersecret".into(),
(B[m[32m+            captured_at: Utc::now(),
(B[m[32m+        };
(B[m         let dbg = format!("{t:?}");
         assert!(!dbg.contains("ghp_supersecret"));
         assert!(dbg.contains("redacted"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:225:
     #[test]
     fn manifest_declares_contribution_event_types() {
         let m = default_manifest();
[31m-        for want in ["github.push", "github.pr.opened", "github.pr.merged", "github.issue.closed"] {
(B[m[32m+        for want in [
(B[m[32m+            "github.push",
(B[m[32m+            "github.pr.opened",
(B[m[32m+            "github.pr.merged",
(B[m[32m+            "github.issue.closed",
(B[m[32m+        ] {
(B[m             assert!(m.event_types.iter().any(|e| e == want), "missing: {want}");
         }
         assert!(matches!(m.auth_strategy, AuthStrategy::ApiKey));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:233:
 
     #[tokio::test]
     async fn sync_unauthorized_when_no_token() {
[31m-        let c = GitHubConnector::builder().base_url("http://unused.invalid").build();
(B[m[32m+        let c = GitHubConnector::builder()
(B[m[32m+            .base_url("http://unused.invalid")
(B[m[32m+            .build();
(B[m         let err = c.sync(None).await.unwrap_err();
         assert!(matches!(err, ConnectorError::Unauthorized(_)));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:248:
             .await;
         let store: Arc<dyn TokenStore> =
             Arc::new(InMemoryTokenStore::with_token(GitHubToken::new("bad")));
[31m-        let c = GitHubConnector::builder().base_url(server.uri()).token_store(store).build();
(B[m[32m+        let c = GitHubConnector::builder()
(B[m[32m+            .base_url(server.uri())
(B[m[32m+            .token_store(store)
(B[m[32m+            .build();
(B[m         assert!(matches!(c.health().await, HealthState::Unauthenticated));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:268:
             .await;
         let store: Arc<dyn TokenStore> =
             Arc::new(InMemoryTokenStore::with_token(GitHubToken::new("ok")));
[31m-        let c = GitHubConnector::builder().base_url(server.uri()).token_store(store).build();
(B[m[32m+        let c = GitHubConnector::builder()
(B[m[32m+            .base_url(server.uri())
(B[m[32m+            .token_store(store)
(B[m[32m+            .build();
(B[m         match c.health().await {
             HealthState::Failing(msg) => {
                 assert!(msg.contains("rate_limited_until"), "got: {msg}");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:319:
             .and(path_regex(r"^/users/octocat/events$"))
             .and(wiremock::matchers::query_param("per_page", "100"))
             .respond_with(
[31m-                ResponseTemplate::new(200).insert_header("Link", link.as_str()).set_body_json(
(B[m[31m-                    json!([
(B[m[32m+                ResponseTemplate::new(200)
(B[m[32m+                    .insert_header("Link", link.as_str())
(B[m[32m+                    .set_body_json(json!([
(B[m                         {
                             "id": "1",
                             "type": "PushEvent",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:339:
                             "created_at": "2026-04-01T12:01:00Z",
                             "payload": {"action": "started"}
                         }
[31m-                    ]),
(B[m[31m-                ),
(B[m[32m+                    ])),
(B[m             )
             .mount(&server)
             .await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/lib.rs:347:
 
         let store: Arc<dyn TokenStore> =
             Arc::new(InMemoryTokenStore::with_token(GitHubToken::new("pat")));
[31m-        let c = GitHubConnector::builder().base_url(server.uri()).token_store(store).build();
(B[m[32m+        let c = GitHubConnector::builder()
(B[m[32m+            .base_url(server.uri())
(B[m[32m+            .token_store(store)
(B[m[32m+            .build();
(B[m         let out = c.sync(None).await.unwrap();
         // 3 GitHub events in total; WatchEvent dropped → 2 mapped.
         assert_eq!(out.events.len(), 2);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/webhook.rs:35:
                 .ok_or_else(|| ConnectorError::Schema("missing 'type' field".to_string()))?
                 .to_string(),
             actor: crate::models::GitHubActor {
[31m-                id: payload.get("actor").and_then(|a| a.get("id")).and_then(|v| v.as_u64()).unwrap_or(0),
(B[m[32m+                id: payload
(B[m[32m+                    .get("actor")
(B[m[32m+                    .and_then(|a| a.get("id"))
(B[m[32m+                    .and_then(|v| v.as_u64())
(B[m[32m+                    .unwrap_or(0),
(B[m                 login: payload
                     .get("actor")
                     .and_then(|a| a.get("login"))
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/webhook.rs:44:
                     .to_string(),
             },
             repo: crate::models::GitHubRepo {
[31m-                id: payload.get("repo").and_then(|r| r.get("id")).and_then(|v| v.as_u64()).unwrap_or(0),
(B[m[32m+                id: payload
(B[m[32m+                    .get("repo")
(B[m[32m+                    .and_then(|r| r.get("id"))
(B[m[32m+                    .and_then(|v| v.as_u64())
(B[m[32m+                    .unwrap_or(0),
(B[m                 name: payload
                     .get("repo")
                     .and_then(|r| r.get("name"))
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-github/src/webhook.rs:53:
                     .to_string(),
             },
             created_at: chrono::Utc::now(),
[31m-            public: payload.get("public").and_then(|v| v.as_bool()).unwrap_or(false),
(B[m[32m+            public: payload
(B[m[32m+                .get("public")
(B[m[32m+                .and_then(|v| v.as_bool())
(B[m[32m+                .unwrap_or(false),
(B[m             payload: payload.clone(),
         };
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/api.rs:1:
 //! Linear GraphQL API client — issues, viewer endpoints.
 
[32m+use phenotype_observably_macros::async_instrumented;
(B[m use reqwest::Client;
 use serde_json::Value;
[31m-use phenotype_observably_macros::async_instrumented;
(B[m 
 use focus_connectors::Result as ConnResult;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/auth.rs:136:
         store.set_token("shared_token".into()).await;
 
         let store_clone = Arc::clone(&store);
[31m-        let task = tokio::spawn(async move {
(B[m[31m-            store_clone.get_token().await
(B[m[31m-        });
(B[m[32m+        let task = tokio::spawn(async move { store_clone.get_token().await });
(B[m 
         let result = task.await.unwrap();
         assert_eq!(result, Some("shared_token".into()));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/events.rs:137:
         let events = mapper.map_issues(vec![issue]);
         // Should emit both created and closed
         assert_eq!(events.len(), 2);
[31m-        assert!(events.iter().any(|e| e.event_type.to_string().contains("issue_created")));
(B[m[31m-        assert!(events.iter().any(|e| e.event_type.to_string().contains("issue_closed")));
(B[m[32m+        assert!(events
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.event_type.to_string().contains("issue_created")));
(B[m[32m+        assert!(events
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.event_type.to_string().contains("issue_closed")));
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/lib.rs:93:
         },
         capabilities: vec![],
         entity_types: vec!["issue".into()],
[31m-        event_types: vec![
(B[m[31m-            "linear:issue_created".into(),
(B[m[31m-            "linear:issue_closed".into(),
(B[m[31m-        ],
(B[m[32m+        event_types: vec!["linear:issue_created".into(), "linear:issue_closed".into()],
(B[m         tier: VerificationTier::Verified,
         health_indicators: vec!["last_sync_ok".into(), "api_key_valid".into()],
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/lib.rs:149:
     #[test]
     fn linear_builder_constructs() {
         let account_id = Uuid::new_v4();
[31m-        let connector = LinearConnectorBuilder::new()
(B[m[31m-            .account_id(account_id)
(B[m[31m-            .build();
(B[m[32m+        let connector = LinearConnectorBuilder::new().account_id(account_id).build();
(B[m         assert_eq!(connector.manifest().id, "linear");
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/lib.rs:160:
     fn test_auth_strategy_is_apikey() {
         let manifest = default_manifest();
         match &manifest.auth_strategy {
[31m-            AuthStrategy::ApiKey => {},
(B[m[32m+            AuthStrategy::ApiKey => {}
(B[m             _ => panic!("Expected ApiKey auth strategy"),
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/lib.rs:177:
     #[test]
     fn test_manifest_event_types() {
         let manifest = default_manifest();
[31m-        assert!(manifest.event_types.contains(&"linear:issue_created".to_string()));
(B[m[31m-        assert!(manifest.event_types.contains(&"linear:issue_closed".to_string()));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .contains(&"linear:issue_created".to_string()));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .contains(&"linear:issue_closed".to_string()));
(B[m         assert_eq!(manifest.event_types.len(), 2);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-linear/src/lib.rs:196:
     fn linear_manifest_has_events() {
         let manifest = default_manifest();
         assert_eq!(manifest.event_types.len(), 2);
[31m-        assert!(manifest.event_types.iter().any(|e| e.contains("issue_created")));
(B[m[31m-        assert!(manifest.event_types.iter().any(|e| e.contains("issue_closed")));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.contains("issue_created")));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.contains("issue_closed")));
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/api.rs:1:
 //! Notion API v1 client — /users/me, /databases, /pages endpoints.
 
[32m+use phenotype_observably_macros::async_instrumented;
(B[m use reqwest::Client;
 use serde_json::Value;
[31m-use phenotype_observably_macros::async_instrumented;
(B[m 
 use focus_connectors::Result as ConnResult;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/events.rs:24:
                     .map(|dt| dt.with_timezone(&Utc))
                     .unwrap_or_else(|_| Utc::now());
 
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "notion",
(B[m[31m-                    &p.id,
(B[m[31m-                    edited_at,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("notion", &p.id, edited_at);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/events.rs:65:
                     .map(|dt| dt.with_timezone(&Utc))
                     .unwrap_or_else(|_| Utc::now());
 
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "notion",
(B[m[31m-                    &t.id,
(B[m[31m-                    edited_at,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("notion", &t.id, edited_at);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/lib.rs:84:
         },
         capabilities: vec![],
         entity_types: vec!["page".into(), "task".into()],
[31m-        event_types: vec![
(B[m[31m-            "notion:page_updated".into(),
(B[m[31m-            "notion:task_completed".into(),
(B[m[31m-        ],
(B[m[32m+        event_types: vec!["notion:page_updated".into(), "notion:task_completed".into()],
(B[m         tier: VerificationTier::Verified,
         health_indicators: vec!["last_sync_ok".into(), "integration_token_valid".into()],
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/lib.rs:147:
     #[test]
     fn notion_builder_constructs() {
         let account_id = Uuid::new_v4();
[31m-        let connector = NotionConnectorBuilder::new()
(B[m[31m-            .account_id(account_id)
(B[m[31m-            .build();
(B[m[32m+        let connector = NotionConnectorBuilder::new().account_id(account_id).build();
(B[m         assert_eq!(connector.manifest().id, "notion");
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/lib.rs:158:
     fn notion_manifest_has_events() {
         let manifest = default_manifest();
         assert_eq!(manifest.event_types.len(), 2);
[31m-        assert!(manifest.event_types.iter().any(|e| e.contains("page_updated")));
(B[m[31m-        assert!(manifest.event_types.iter().any(|e| e.contains("task_completed")));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.contains("page_updated")));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.contains("task_completed")));
(B[m     }
 
     // Test 3: Auth error handling — integration token validation
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/lib.rs:167:
     fn test_notion_auth_strategy_is_apikey() {
         let manifest = default_manifest();
         match &manifest.auth_strategy {
[31m-            AuthStrategy::ApiKey => {},
(B[m[32m+            AuthStrategy::ApiKey => {}
(B[m             _ => panic!("Expected ApiKey auth strategy"),
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-notion/src/lib.rs:188:
         assert_eq!(manifest.id, "notion");
         assert_eq!(manifest.version, "0.1.0");
         assert_eq!(manifest.display_name, "Notion");
[31m-        assert!(manifest.health_indicators.contains(&"last_sync_ok".to_string()));
(B[m[32m+        assert!(manifest
(B[m[32m+            .health_indicators
(B[m[32m+            .contains(&"last_sync_ok".to_string()));
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/api.rs:1:
 //! Readwise Reader API client — /documents, /highlights endpoints.
 
[32m+use phenotype_observably_macros::async_instrumented;
(B[m use reqwest::Client;
 use serde_json::Value;
[31m-use phenotype_observably_macros::async_instrumented;
(B[m 
 use focus_connectors::Result as ConnResult;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/api.rs:157:
 
         let highlights = Highlight::from_readwise_json(&highlight_json);
         assert!(!highlights.is_empty());
[31m-        assert_eq!(highlights[0].text, "This is a highlighted quote from the article");
(B[m[32m+        assert_eq!(
(B[m[32m+            highlights[0].text,
(B[m[32m+            "This is a highlighted quote from the article"
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-READWISE-API-004 (parse multiple articles)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/auth.rs:109:
     fn readwise_auth_special_chars() {
         let token_with_dash = ReadwiseAuth::new("token-with-dashes");
         let token_with_underscore = ReadwiseAuth::new("token_with_underscores");
[31m-        assert!(token_with_dash.bearer_header().contains("token-with-dashes"));
(B[m[31m-        assert!(token_with_underscore.bearer_header().contains("token_with_underscores"));
(B[m[32m+        assert!(token_with_dash
(B[m[32m+            .bearer_header()
(B[m[32m+            .contains("token-with-dashes"));
(B[m[32m+        assert!(token_with_underscore
(B[m[32m+            .bearer_header()
(B[m[32m+            .contains("token_with_underscores"));
(B[m     }
 
     // Traces to: FR-READWISE-AUTH-006
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/events.rs:24:
                     .map(|dt| dt.with_timezone(&Utc))
                     .unwrap_or_else(|_| Utc::now());
 
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "readwise",
(B[m[31m-                    &h.id,
(B[m[31m-                    created_at,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("readwise", &h.id, created_at);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/events.rs:64:
                     .map(|dt| dt.with_timezone(&Utc))
                     .unwrap_or_else(|_| Utc::now());
 
[31m-                let dedupe_key = EventFactory::new_dedupe_key(
(B[m[31m-                    "readwise",
(B[m[31m-                    &a.id,
(B[m[31m-                    updated_at,
(B[m[31m-                );
(B[m[32m+                let dedupe_key = EventFactory::new_dedupe_key("readwise", &a.id, updated_at);
(B[m 
                 NormalizedEvent {
                     event_id: Uuid::new_v4(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/events.rs:118:
 
         let events = mapper.map_highlights(vec![highlight]);
         assert_eq!(events.len(), 1);
[31m-        assert!(events[0].event_type.to_string().contains("highlight_created"));
(B[m[32m+        assert!(events[0]
(B[m[32m+            .event_type
(B[m[32m+            .to_string()
(B[m[32m+            .contains("highlight_created"));
(B[m     }
 
     // Traces to: FR-READWISE-EVENTS-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/lib.rs:158:
     fn readwise_manifest_has_events() {
         let manifest = default_manifest();
         assert_eq!(manifest.event_types.len(), 2);
[31m-        assert!(manifest.event_types.iter().any(|e| e.contains("highlight_created")));
(B[m[31m-        assert!(manifest.event_types.iter().any(|e| e.contains("article_read")));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.contains("highlight_created")));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .iter()
(B[m[32m+            .any(|e| e.contains("article_read")));
(B[m     }
 
     // Test 3: Auth error handling — API token validation
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/lib.rs:167:
     fn test_readwise_auth_strategy_is_apikey() {
         let manifest = default_manifest();
         match &manifest.auth_strategy {
[31m-            AuthStrategy::ApiKey => {},
(B[m[32m+            AuthStrategy::ApiKey => {}
(B[m             _ => panic!("Expected ApiKey auth strategy"),
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-readwise/src/models.rs:25:
                         id: doc.get("id")?.as_str()?.into(),
                         title: doc.get("title")?.as_str()?.into(),
                         author: doc.get("author").and_then(|a| a.as_str()).map(|s| s.into()),
[31m-                        source_url: doc.get("source_url").and_then(|u| u.as_str()).map(|s| s.into()),
(B[m[31m-                        cover_image_url: doc.get("cover_image_url").and_then(|u| u.as_str()).map(|s| s.into()),
(B[m[31m-                        published_date: doc.get("published_date").and_then(|d| d.as_str()).map(|s| s.into()),
(B[m[32m+                        source_url: doc
(B[m[32m+                            .get("source_url")
(B[m[32m+                            .and_then(|u| u.as_str())
(B[m[32m+                            .map(|s| s.into()),
(B[m[32m+                        cover_image_url: doc
(B[m[32m+                            .get("cover_image_url")
(B[m[32m+                            .and_then(|u| u.as_str())
(B[m[32m+                            .map(|s| s.into()),
(B[m[32m+                        published_date: doc
(B[m[32m+                            .get("published_date")
(B[m[32m+                            .and_then(|d| d.as_str())
(B[m[32m+                            .map(|s| s.into()),
(B[m                         created_at: doc.get("created_at")?.as_str()?.into(),
                         updated_at: doc.get("updated_at")?.as_str()?.into(),
                     })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/api.rs:1:
 //! Strava REST API client — GET /api/v3/athlete/activities, /api/v3/activities/:id.
 //! Rate limit: 100 req/15min, 1000/day.
 
[32m+use phenotype_observably_macros::async_instrumented;
(B[m use reqwest::Client;
 use serde_json::Value;
[31m-use phenotype_observably_macros::async_instrumented;
(B[m 
 use focus_connectors::Result as ConnResult;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/api.rs:52:
     /// Rate limit: 100 req/15min, 1000/day.
     #[async_instrumented]
     pub async fn get_recent_activities(&self, limit: u32) -> ConnResult<Vec<Activity>> {
[31m-        let url = format!(
(B[m[31m-            "{}/athlete/activities?per_page={}",
(B[m[31m-            STRAVA_API_BASE, limit
(B[m[31m-        );
(B[m[32m+        let url = format!("{}/athlete/activities?per_page={}", STRAVA_API_BASE, limit);
(B[m         let resp = self
             .http
             .get(&url)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/api.rs:69:
                 .await
                 .map_err(|e| focus_connectors::ConnectorError::Schema(e.to_string()))?;
 
[31m-            Ok(json
(B[m[31m-                .iter()
(B[m[31m-                .map(Activity::from_strava_json)
(B[m[31m-                .collect())
(B[m[32m+            Ok(json.iter().map(Activity::from_strava_json).collect())
(B[m         } else if resp.status().as_u16() == 401 {
             Err(focus_connectors::ConnectorError::Unauthorized(
                 "Strava token invalid or expired".into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/events.rs:160:
 
         let events = mapper.map_activities(activities);
         assert_eq!(events.len(), 2);
[32m+        assert!(events.iter().any(|e| e
(B[m[32m+            .event_type
(B[m[32m+            .to_string()
(B[m[32m+            .contains("strava:activity_completed")));
(B[m         assert!(events
             .iter()
[31m-            .any(|e| e.event_type.to_string().contains("strava:activity_completed")));
(B[m[31m-        assert!(events
(B[m[31m-            .iter()
(B[m             .any(|e| e.event_type.to_string().contains("strava:pr_earned")));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/events.rs:197:
 
         // Dedupe key should include strava + activity id + timestamp
         assert!(!activity_event.dedupe_key.0.is_empty());
[31m-        assert_eq!(activity_event.event_type.to_string(), "strava:activity_completed");
(B[m[32m+        assert_eq!(
(B[m[32m+            activity_event.event_type.to_string(),
(B[m[32m+            "strava:activity_completed"
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-STRAVA-EVENTS-002
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/lib.rs:12:
 use tracing::{debug, info, warn};
 use uuid::Uuid;
 
[31m-use focus_connectors::{AuthStrategy, Connector, ConnectorError, ConnectorManifest, HealthState, Result, SyncMode, SyncOutcome, VerificationTier};
(B[m[32m+use focus_connectors::{
(B[m[32m+    AuthStrategy, Connector, ConnectorError, ConnectorManifest, HealthState, Result, SyncMode,
(B[m[32m+    SyncOutcome, VerificationTier,
(B[m[32m+};
(B[m 
 use crate::api::StravaClient;
 use crate::auth::{KeychainTokenStore, StravaOAuth2, TokenStore};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/models.rs:67:
                 .and_then(|v| v.as_str())
                 .unwrap_or("1970-01-01T00:00:00")
                 .to_string(),
[31m-            distance: json
(B[m[31m-                .get("distance")
(B[m[31m-                .and_then(|v| v.as_f64())
(B[m[31m-                .unwrap_or(0.0),
(B[m[32m+            distance: json.get("distance").and_then(|v| v.as_f64()).unwrap_or(0.0),
(B[m             moving_time: json
                 .get("moving_time")
                 .and_then(|v| v.as_u64())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/models.rs:83:
                 .get("elevation_gain")
                 .and_then(|v| v.as_f64())
                 .unwrap_or(0.0),
[31m-            type_: json.get("type").and_then(|v| v.as_str()).map(|s| s.to_string()),
(B[m[32m+            type_: json
(B[m[32m+                .get("type")
(B[m[32m+                .and_then(|v| v.as_str())
(B[m[32m+                .map(|s| s.to_string()),
(B[m             average_speed: json.get("average_speed").and_then(|v| v.as_f64()),
             max_speed: json.get("max_speed").and_then(|v| v.as_f64()),
             total_elevation_gain: json.get("total_elevation_gain").and_then(|v| v.as_f64()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-strava/src/models.rs:90:
             has_kudos: json.get("has_kudos").and_then(|v| v.as_bool()),
[31m-            pr_count: json.get("pr_count").and_then(|v| v.as_u64()).map(|v| v as u32),
(B[m[32m+            pr_count: json
(B[m[32m+                .get("pr_count")
(B[m[32m+                .and_then(|v| v.as_u64())
(B[m[32m+                .map(|v| v as u32),
(B[m             achievement_count: json
                 .get("achievement_count")
                 .and_then(|v| v.as_u64())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-testkit/tests/dedupe_contract.rs:26:
             version: "0.0.1".into(),
             display_name: "Mock".into(),
             auth_strategy: AuthStrategy::None,
[31m-            sync_mode: SyncMode::Polling { cadence_seconds: 60 },
(B[m[32m+            sync_mode: SyncMode::Polling {
(B[m[32m+                cadence_seconds: 60,
(B[m[32m+            },
(B[m             capabilities: vec![],
             entity_types: vec![],
             event_types: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/connector-testkit/tests/dedupe_contract.rs:61:
 
     // Returns the *same* event each sync call — simulates duplicate delivery.
     async fn sync(&self, _cursor: Option<String>) -> ConnResult<SyncOutcome> {
[31m-        Ok(SyncOutcome { events: vec![self.event.clone()], next_cursor: None, partial: false })
(B[m[32m+        Ok(SyncOutcome {
(B[m[32m+            events: vec![self.event.clone()],
(B[m[32m+            next_cursor: None,
(B[m[32m+            partial: false,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:3:
 
 use async_trait::async_trait;
 use chrono::{DateTime, Datelike, Timelike, Utc};
[32m+use phenotype_observably_macros::async_instrumented;
(B[m use serde::{Deserialize, Serialize};
 use std::collections::HashMap;
 use std::sync::Arc;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:9:
 use tokio::sync::Mutex;
[31m-use phenotype_observably_macros::async_instrumented;
(B[m 
 pub use focus_events::NormalizedEvent;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:74:
     ///
     /// Returns 0 or 1 proposals per call. If no prediction meets confidence threshold,
     /// returns None.
[31m-    async fn predict_next_nudge(&self, now: DateTime<Utc>) -> anyhow::Result<Option<NudgeProposal>>;
(B[m[32m+    async fn predict_next_nudge(&self, now: DateTime<Utc>)
(B[m[32m+        -> anyhow::Result<Option<NudgeProposal>>;
(B[m 
     /// Get the likely productive hours for a given day of week (0–6: Mon–Sun).
     ///
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:116:
 
             // Count a session as "success" if duration >= 25 min (inferred from payload or default).
             let is_success = {
[31m-                let duration_min = event.payload.get("duration_minutes")
(B[m[32m+                let duration_min = event
(B[m[32m+                    .payload
(B[m[32m+                    .get("duration_minutes")
(B[m                     .and_then(|v| v.as_i64())
                     .unwrap_or(25); // Default: assume success if not specified
                 duration_min >= 25
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:156:
 #[async_trait]
 impl HabitPredictor for RollingAverageHabitPredictor {
     #[async_instrumented]
[31m-    async fn predict_next_nudge(&self, now: DateTime<Utc>) -> anyhow::Result<Option<NudgeProposal>> {
(B[m[32m+    async fn predict_next_nudge(
(B[m[32m+        &self,
(B[m[32m+        now: DateTime<Utc>,
(B[m[32m+    ) -> anyhow::Result<Option<NudgeProposal>> {
(B[m         let weekday = now.weekday().number_from_monday() - 1; // 0–6: Mon–Sun
         let hour = now.hour();
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:187:
         let activity = self.activity.lock().await;
         let mut hours: Vec<_> = (0..24)
             .filter(|h| {
[31m-                !Self::is_sleep_hour(*h) && activity.get(&(day_of_week, *h)).copied().unwrap_or(0.0) > 0.6
(B[m[32m+                !Self::is_sleep_hour(*h)
(B[m[32m+                    && activity.get(&(day_of_week, *h)).copied().unwrap_or(0.0) > 0.6
(B[m             })
             .collect();
         hours.sort();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:198:
         let activity = self.activity.lock().await;
         let mut hours: Vec<_> = (0..24)
             .filter(|h| {
[31m-                !Self::is_sleep_hour(*h) && activity.get(&(day_of_week, *h)).copied().unwrap_or(0.0) < 0.3
(B[m[32m+                !Self::is_sleep_hour(*h)
(B[m[32m+                    && activity.get(&(day_of_week, *h)).copied().unwrap_or(0.0) < 0.3
(B[m             })
             .collect();
         hours.sort();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:227:
         predictor: Arc<dyn HabitPredictor>,
         nudge_tx: tokio::sync::mpsc::UnboundedSender<NudgeProposal>,
     ) -> Self {
[31m-        Self { predictor, nudge_tx }
(B[m[32m+        Self {
(B[m[32m+            predictor,
(B[m[32m+            nudge_tx,
(B[m[32m+        }
(B[m     }
 
     /// Perform a single tick of the engine (called every 60 seconds in production).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:292:
 
         {
             let mut activity = predictor.activity.lock().await;
[31m-            activity.insert((0, 9), 0.8);  // High productivity
(B[m[32m+            activity.insert((0, 9), 0.8); // High productivity
(B[m             activity.insert((0, 10), 0.7); // High productivity
             activity.insert((0, 11), 0.4); // Low productivity
             activity.insert((0, 23), 0.9); // Would be sleep time (not included)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:311:
         {
             let mut activity = predictor.activity.lock().await;
             activity.insert((0, 23), 0.9); // High confidence, but sleep hour
[31m-            activity.insert((0, 5), 0.9);  // High confidence, but sleep hour
(B[m[32m+            activity.insert((0, 5), 0.9); // High confidence, but sleep hour
(B[m         }
 
         // 23:00 (11 PM) is a sleep hour.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:318:
[31m-        let proposal = predictor.predict_next_nudge(Utc::now().with_hour(23).unwrap()).await.unwrap();
(B[m[32m+        let proposal = predictor
(B[m[32m+            .predict_next_nudge(Utc::now().with_hour(23).unwrap())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(proposal.is_none(), "Should not nudge during sleep hours");
 
         // 5:00 AM is a sleep hour.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:322:
[31m-        let proposal = predictor.predict_next_nudge(Utc::now().with_hour(5).unwrap()).await.unwrap();
(B[m[32m+        let proposal = predictor
(B[m[32m+            .predict_next_nudge(Utc::now().with_hour(5).unwrap())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(proposal.is_none(), "Should not nudge during sleep hours");
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-always-on/src/lib.rs:369:
                     "Confidence should be deterministic"
                 );
             }
[31m-            (None, None) => {}, // Both None is also deterministic.
(B[m[32m+            (None, None) => {} // Both None is also deterministic.
(B[m             (Some(_), None) | (None, Some(_)) => {
                 panic!("Non-deterministic prediction from same input time");
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/lib.rs:92:
                 if i >= parts.len() {
                     return Err(anyhow!("--duration requires a value"));
                 }
[31m-                trim_duration = Some(parts[i].parse::<f32>().context("--duration must be a float")?);
(B[m[32m+                trim_duration = Some(
(B[m[32m+                    parts[i]
(B[m[32m+                        .parse::<f32>()
(B[m[32m+                        .context("--duration must be a float")?,
(B[m[32m+                );
(B[m             }
             "--pitch" => {
                 i += 1;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/lib.rs:245:
 }
 
 /// Download a single asset; uses cache if URL hash matches.
[31m-pub fn download_asset(
(B[m[31m-    asset: &AssetSpec,
(B[m[31m-    config: &FetcherConfig,
(B[m[31m-) -> Result<PathBuf> {
(B[m[32m+pub fn download_asset(asset: &AssetSpec, config: &FetcherConfig) -> Result<PathBuf> {
(B[m     // Respect robots.txt conceptually (simple delay)
     std::thread::sleep(Duration::from_millis(config.request_delay_ms));
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:1:
 use anyhow::{Context, Result};
 use clap::Parser;
[31m-use focus_asset_fetcher::{
(B[m[31m-    download_asset, parse_sound_sources, FetcherConfig,
(B[m[31m-};
(B[m[32m+use focus_asset_fetcher::{download_asset, parse_sound_sources, FetcherConfig};
(B[m use std::fs;
 use std::path::PathBuf;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:51:
     // Resolve paths
     let sources_file = args
         .sources_file
[31m-        .unwrap_or_else(|| {
(B[m[31m-            PathBuf::from("apps/ios/FocalPoint/Resources/Audio/SOUND_SOURCES.md")
(B[m[31m-        });
(B[m[32m+        .unwrap_or_else(|| PathBuf::from("apps/ios/FocalPoint/Resources/Audio/SOUND_SOURCES.md"));
(B[m 
     let cache_dir = args
         .cache_dir
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:61:
 
     let output_sfx_dir = args
         .output_sfx_dir
[31m-        .unwrap_or_else(|| {
(B[m[31m-            PathBuf::from("apps/ios/FocalPoint/Resources/Audio/SFX")
(B[m[31m-        });
(B[m[32m+        .unwrap_or_else(|| PathBuf::from("apps/ios/FocalPoint/Resources/Audio/SFX"));
(B[m 
     let output_simlish_dir = args
         .output_simlish_dir
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:70:
[31m-        .unwrap_or_else(|| {
(B[m[31m-            PathBuf::from("apps/ios/FocalPoint/Resources/Audio/Simlish")
(B[m[31m-        });
(B[m[32m+        .unwrap_or_else(|| PathBuf::from("apps/ios/FocalPoint/Resources/Audio/Simlish"));
(B[m 
     // Load main sources
[31m-    let sources_content = fs::read_to_string(&sources_file)
(B[m[31m-        .context(format!("read {}", sources_file.display()))?;
(B[m[32m+    let sources_content =
(B[m[32m+        fs::read_to_string(&sources_file).context(format!("read {}", sources_file.display()))?;
(B[m 
[31m-    let assets = parse_sound_sources(&sources_content)
(B[m[31m-        .context("parse SOUND_SOURCES.md")?;
(B[m[32m+    let assets = parse_sound_sources(&sources_content).context("parse SOUND_SOURCES.md")?;
(B[m 
     println!("Fetching {} SFX assets...", assets.len());
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:84:
         println!("\n[DRY RUN] Plan:");
     }
 
[31m-    let mut config = FetcherConfig::new(cache_dir.clone(), output_sfx_dir.clone(), output_simlish_dir.clone());
(B[m[32m+    let mut config = FetcherConfig::new(
(B[m[32m+        cache_dir.clone(),
(B[m[32m+        output_sfx_dir.clone(),
(B[m[32m+        output_simlish_dir.clone(),
(B[m[32m+    );
(B[m     config.dry_run = args.dry_run;
     if let Some(delay) = args.request_delay_ms {
         config.request_delay_ms = delay;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:95:
 
     // Fetch all assets
     for asset in assets {
[31m-        let cached_path = download_asset(&asset, &config)
(B[m[31m-            .context(format!("fetch {}", asset.name))?;
(B[m[32m+        let cached_path =
(B[m[32m+            download_asset(&asset, &config).context(format!("fetch {}", asset.name))?;
(B[m 
         if !args.dry_run {
             // Create output directory
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:103:
[31m-            fs::create_dir_all(&config.output_sfx_dir)
(B[m[31m-                .context("create output SFX dir")?;
(B[m[32m+            fs::create_dir_all(&config.output_sfx_dir).context("create output SFX dir")?;
(B[m 
             // For now, just copy cached file (post-processing would happen here with ffmpeg check)
             let output_path = config.output_sfx_dir.join(format!("{}.m4a", asset.name));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:116:
             let simlish_content = fs::read_to_string(&simlish_file)
                 .context(format!("read {}", simlish_file.display()))?;
 
[31m-            let simlish_assets = parse_sound_sources(&simlish_content)
(B[m[31m-                .context("parse Simlish/SOURCES.md")?;
(B[m[32m+            let simlish_assets =
(B[m[32m+                parse_sound_sources(&simlish_content).context("parse Simlish/SOURCES.md")?;
(B[m 
             println!("Fetching {} Simlish phonemes...", simlish_assets.len());
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:124:
             for asset in simlish_assets {
[31m-                let cached_path = download_asset(&asset, &config)
(B[m[31m-                    .context(format!("fetch {}", asset.name))?;
(B[m[32m+                let cached_path =
(B[m[32m+                    download_asset(&asset, &config).context(format!("fetch {}", asset.name))?;
(B[m 
                 if !args.dry_run {
                     fs::create_dir_all(&config.output_simlish_dir)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-asset-fetcher/src/main.rs:130:
                         .context("create output Simlish dir")?;
 
[31m-                    let output_path = config.output_simlish_dir.join(format!("{}.m4a", asset.name));
(B[m[32m+                    let output_path = config
(B[m[32m+                        .output_simlish_dir
(B[m[32m+                        .join(format!("{}.m4a", asset.name));
(B[m                     fs::copy(&cached_path, &output_path)
                         .context(format!("copy asset to {}", output_path.display()))?;
                 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:12:
 
 pub mod canonical;
 
[32m+use focus_observability::{AuditSpanAttrs, MetricsRegistry};
(B[m use serde::{Deserialize, Serialize};
 use sha2::{Digest, Sha256};
 use std::sync::{Arc, Mutex};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:18:
 use std::time::Instant;
[31m-use focus_observability::{AuditSpanAttrs, MetricsRegistry};
(B[m 
 /// Sentinel `prev_hash` for the first record in a chain.
 pub const GENESIS_PREV_HASH: &str = "genesis";
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:26:
     #[error("chain is empty")]
     Empty,
     #[error("hash mismatch at index {index}: expected {expected}, got {actual}")]
[31m-    HashMismatch { index: usize, expected: String, actual: String },
(B[m[32m+    HashMismatch {
(B[m[32m+        index: usize,
(B[m[32m+        expected: String,
(B[m[32m+        actual: String,
(B[m[32m+    },
(B[m     #[error("prev_hash link broken at index {index}")]
     PrevHashBroken { index: usize },
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:87:
 
 impl AuditChain {
     pub fn new() -> Self {
[31m-        Self { records: Vec::new() }
(B[m[32m+        Self {
(B[m[32m+            records: Vec::new(),
(B[m[32m+        }
(B[m     }
 
     pub fn len(&self) -> usize {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:100:
 
     /// Return the hash of the tip record, or [`GENESIS_PREV_HASH`] if empty.
     pub fn head_hash(&self) -> &str {
[31m-        self.records.last().map(|r| r.hash.as_str()).unwrap_or(GENESIS_PREV_HASH)
(B[m[32m+        self.records
(B[m[32m+            .last()
(B[m[32m+            .map(|r| r.hash.as_str())
(B[m[32m+            .unwrap_or(GENESIS_PREV_HASH)
(B[m     }
 
     /// Append a new record, computing its `prev_hash` and `hash`.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:199:
 ) -> anyhow::Result<AuditRecord> {
     let payload_value = serde_json::to_value(payload)
         .map_err(|e| anyhow::anyhow!("serialize audit payload: {e}"))?;
[31m-    let prev_hash = store.head_hash()?.unwrap_or_else(|| GENESIS_PREV_HASH.to_string());
(B[m[32m+    let prev_hash = store
(B[m[32m+        .head_hash()?
(B[m[32m+        .unwrap_or_else(|| GENESIS_PREV_HASH.to_string());
(B[m     let hash =
         AuditRecord::compute_hash(record_type, subject_ref, &now, &prev_hash, &payload_value);
     let record = AuditRecord {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:269:
 
 impl InMemoryAuditStore {
     pub fn new() -> Self {
[31m-        Self { chain: Mutex::new(AuditChain::new()) }
(B[m[32m+        Self {
(B[m[32m+            chain: Mutex::new(AuditChain::new()),
(B[m[32m+        }
(B[m     }
 
     /// Return the most recent `limit` records in newest-first order. Used
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:288:
 
 impl AuditStore for InMemoryAuditStore {
     fn append(&self, record: AuditRecord) -> anyhow::Result<()> {
[31m-        let mut chain =
(B[m[31m-            self.chain.lock().map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
(B[m[32m+        let mut chain = self
(B[m[32m+            .chain
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
(B[m         // Caller-constructed record; trust-but-verify its prev_hash link.
         let expected_prev = chain.head_hash().to_string();
         if record.prev_hash != expected_prev {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:303:
     }
 
     fn verify_chain(&self) -> anyhow::Result<bool> {
[31m-        let chain =
(B[m[31m-            self.chain.lock().map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
(B[m[32m+        let chain = self
(B[m[32m+            .chain
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
(B[m         match chain.verify() {
             Ok(()) => Ok(true),
             Err(ChainError::Empty) => Ok(true),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:313:
     }
 
     fn head_hash(&self) -> anyhow::Result<Option<String>> {
[31m-        let chain =
(B[m[31m-            self.chain.lock().map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
(B[m[31m-        Ok(if chain.is_empty() { None } else { Some(chain.head_hash().to_string()) })
(B[m[32m+        let chain = self
(B[m[32m+            .chain
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("audit chain mutex poisoned: {e}"))?;
(B[m[32m+        Ok(if chain.is_empty() {
(B[m[32m+            None
(B[m[32m+        } else {
(B[m[32m+            Some(chain.head_hash().to_string())
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:333:
 }
 
 /// A single captured mutation: `(record_type, subject_ref, payload, occurred_at)`.
[31m-pub type CapturedRecord = (String, String, serde_json::Value, chrono::DateTime<chrono::Utc>);
(B[m[32m+pub type CapturedRecord = (
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    serde_json::Value,
(B[m[32m+    chrono::DateTime<chrono::Utc>,
(B[m[32m+);
(B[m 
 /// Test helper sink that captures every mutation for later inspection.
 #[derive(Debug, Default)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:348:
 
     pub fn snapshot(
         &self,
[31m-    ) -> Vec<(String, String, serde_json::Value, chrono::DateTime<chrono::Utc>)> {
(B[m[31m-        self.records.lock().expect("capturing audit sink poisoned").clone()
(B[m[32m+    ) -> Vec<(
(B[m[32m+        String,
(B[m[32m+        String,
(B[m[32m+        serde_json::Value,
(B[m[32m+        chrono::DateTime<chrono::Utc>,
(B[m[32m+    )> {
(B[m[32m+        self.records
(B[m[32m+            .lock()
(B[m[32m+            .expect("capturing audit sink poisoned")
(B[m[32m+            .clone()
(B[m     }
 
     pub fn len(&self) -> usize {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:356:
[31m-        self.records.lock().expect("capturing audit sink poisoned").len()
(B[m[32m+        self.records
(B[m[32m+            .lock()
(B[m[32m+            .expect("capturing audit sink poisoned")
(B[m[32m+            .len()
(B[m     }
 
     pub fn is_empty(&self) -> bool {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:373:
             .records
             .lock()
             .map_err(|e| anyhow::anyhow!("capturing audit sink poisoned: {e}"))?;
[31m-        g.push((record_type.to_string(), subject_ref.to_string(), payload, now));
(B[m[32m+        g.push((
(B[m[32m+            record_type.to_string(),
(B[m[32m+            subject_ref.to_string(),
(B[m[32m+            payload,
(B[m[32m+            now,
(B[m[32m+        ));
(B[m         Ok(())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:540:
     #[test]
     fn noop_audit_sink_does_nothing_but_succeeds() {
         let sink = NoopAuditSink;
[31m-        sink.record_mutation("x", "y", serde_json::json!({}), ts(0)).unwrap();
(B[m[32m+        sink.record_mutation("x", "y", serde_json::json!({}), ts(0))
(B[m[32m+            .unwrap();
(B[m     }
 
     // Traces to: FR-STATE-004
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:547:
     #[test]
     fn capturing_sink_captures_record() {
         let sink = CapturingAuditSink::new();
[31m-        sink.record_mutation("policy.built", "user-3", serde_json::json!({"n": 1}), ts(7)).unwrap();
(B[m[32m+        sink.record_mutation("policy.built", "user-3", serde_json::json!({"n": 1}), ts(7))
(B[m[32m+            .unwrap();
(B[m         let snap = sink.snapshot();
         assert_eq!(snap.len(), 1);
         assert_eq!(snap[0].0, "policy.built");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-audit/src/lib.rs:647:
         assert_eq!(chain.records[2].prev_hash, tier2.hash);
     }
 }
[31m-
(B[m 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/lib.rs:74:
 
 impl Default for BackupConfig {
     fn default() -> Self {
[31m-        Self { device_id: uuid::Uuid::new_v4().to_string(), version: Some("0.0.1".to_string()) }
(B[m[32m+        Self {
(B[m[32m+            device_id: uuid::Uuid::new_v4().to_string(),
(B[m[32m+            version: Some("0.0.1".to_string()),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/lib.rs:174:
     let manifest_hash = hasher.finalize();
 
     // Phase 4: Tar + compress the manifest
[31m-    let tar_blob = tar_builder::build_tar(&manifest_json, manifest_hash.as_ref())
(B[m[31m-        .map_err(BackupError::Tar)?;
(B[m[32m+    let tar_blob =
(B[m[32m+        tar_builder::build_tar(&manifest_json, manifest_hash.as_ref()).map_err(BackupError::Tar)?;
(B[m 
     // Phase 5: Zstd compress
     let compressed = zstd::encode_all(tar_blob.as_slice(), 3)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/lib.rs:289:
 async fn load_all_data(
     _adapter: &SqliteAdapter,
 ) -> Result<
[31m-    (Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>, Vec<String>),
(B[m[32m+    (
(B[m[32m+        Vec<String>,
(B[m[32m+        Vec<String>,
(B[m[32m+        Vec<String>,
(B[m[32m+        Vec<String>,
(B[m[32m+        Vec<String>,
(B[m[32m+        Vec<String>,
(B[m[32m+        Vec<String>,
(B[m[32m+    ),
(B[m     BackupError,
 > {
     // Stub: in production, these would hydrate from the adapter's stores
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/lib.rs:311:
 fn decrypt_with_passphrase(_ciphertext: &[u8], passphrase: &str) -> Result<Vec<u8>, BackupError> {
     // Placeholder: real impl uses age crate's Scrypt KDF
     let _ = passphrase;
[31m-    Err(BackupError::DecryptionFailed("placeholder: real age decryption not yet wired".to_string()))
(B[m[32m+    Err(BackupError::DecryptionFailed(
(B[m[32m+        "placeholder: real age decryption not yet wired".to_string(),
(B[m[32m+    ))
(B[m }
 
 // ---------------------------------------------------------------------------
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/manifest.rs:19:
 
 impl BackupManifest {
     pub fn new(version: String, device_id: String, contents: Vec<ContentSection>) -> Self {
[31m-        Self { version, created_at: Utc::now().to_rfc3339(), device_id, contents }
(B[m[32m+        Self {
(B[m[32m+            version,
(B[m[32m+            created_at: Utc::now().to_rfc3339(),
(B[m[32m+            device_id,
(B[m[32m+            contents,
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/tar_builder.rs:15:
     let mut header = tar::Header::new_gnu();
     header.set_size(manifest_json.len() as u64);
     header.set_mtime(
[31m-        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
(B[m[32m+        std::time::SystemTime::now()
(B[m[32m+            .duration_since(std::time::UNIX_EPOCH)
(B[m[32m+            .unwrap()
(B[m[32m+            .as_secs(),
(B[m     );
     builder
         .append_data(&mut header, "manifest.json", manifest_json)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/tar_builder.rs:30:
         .append_data(&mut header, "manifest.json.sha256", hash_bytes)
         .map_err(|e| format!("failed to append hash file: {}", e))?;
 
[31m-    builder.finish().map_err(|e| format!("failed to finish tar: {}", e))?;
(B[m[32m+    builder
(B[m[32m+        .finish()
(B[m[32m+        .map_err(|e| format!("failed to finish tar: {}", e))?;
(B[m     drop(builder);
 
     Ok(tar_buffer)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-backup/src/tar_builder.rs:44:
 
     for entry_result in archive.entries().map_err(|e| e.to_string())? {
         let mut entry = entry_result.map_err(|e| e.to_string())?;
[31m-        let path = entry.path().map_err(|e| e.to_string())?.to_string_lossy().to_string();
(B[m[32m+        let path = entry
(B[m[32m+            .path()
(B[m[32m+            .map_err(|e| e.to_string())?
(B[m[32m+            .to_string_lossy()
(B[m[32m+            .to_string();
(B[m 
         if path == "manifest.json" {
             let mut buf = Vec::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-calendar/src/lib.rs:100:
 impl CalendarPort for InMemoryCalendarPort {
     async fn list_events(&self, range: DateRange) -> anyhow::Result<Vec<CalendarEvent>> {
         let guard = self.inner.read().await;
[31m-        let mut out: Vec<CalendarEvent> =
(B[m[31m-            guard.iter().filter(|e| range.contains_any(e)).cloned().collect();
(B[m[32m+        let mut out: Vec<CalendarEvent> = guard
(B[m[32m+            .iter()
(B[m[32m+            .filter(|e| range.contains_any(e))
(B[m[32m+            .cloned()
(B[m[32m+            .collect();
(B[m         out.sort_by_key(|e| e.starts_at);
         Ok(out)
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-calendar/src/lib.rs:156:
     async fn in_memory_roundtrip_create_and_list() {
         let cal = InMemoryCalendarPort::new();
         let _ = cal.create_event(&draft("standup", 0, 30)).await.unwrap();
[31m-        let evs = cal.list_events(DateRange::new(t0(), t0() + Duration::hours(2))).await.unwrap();
(B[m[32m+        let evs = cal
(B[m[32m+            .list_events(DateRange::new(t0(), t0() + Duration::hours(2)))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(evs.len(), 1);
         assert_eq!(evs[0].title, "standup");
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-calendar/src/lib.rs:168:
         cal.create_event(&draft("late", 90, 30)).await.unwrap();
         cal.create_event(&draft("early", 0, 30)).await.unwrap();
         cal.create_event(&draft("middle", 45, 30)).await.unwrap();
[31m-        let evs = cal.list_events(DateRange::new(t0(), t0() + Duration::hours(3))).await.unwrap();
(B[m[32m+        let evs = cal
(B[m[32m+            .list_events(DateRange::new(t0(), t0() + Duration::hours(3)))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(evs.len(), 3);
         assert_eq!(evs[0].title, "early");
         assert_eq!(evs[1].title, "middle");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-calendar/src/lib.rs:191:
         let cal = InMemoryCalendarPort::new();
         cal.create_event(&draft("in", 10, 15)).await.unwrap();
         cal.create_event(&draft("out", 500, 15)).await.unwrap();
[31m-        let evs = cal.list_events(DateRange::new(t0(), t0() + Duration::hours(2))).await.unwrap();
(B[m[32m+        let evs = cal
(B[m[32m+            .list_events(DateRange::new(t0(), t0() + Duration::hours(2)))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(evs.len(), 1);
         assert_eq!(evs[0].title, "in");
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher/src/lib.rs:130:
         };
         categories.insert(
             "Fixed".to_string(),
[31m-            vec![format!("CI **FAILED** for {}\n\n```\n{}\n```", repo_name, truncated)],
(B[m[32m+            vec![format!(
(B[m[32m+                "CI **FAILED** for {}\n\n```\n{}\n```",
(B[m[32m+                repo_name, truncated
(B[m[32m+            )],
(B[m         );
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher/src/lib.rs:139:
         if success { "✅" } else { "❌" },
         Utc::now().format("%Y-%m-%d %H:%M UTC")
     ))
[31m-    .with_category(if success { "Added" } else { "Fixed" }, categories
(B[m[31m-        .remove(if success { "Added" } else { "Fixed" })
(B[m[31m-        .unwrap_or_default())
(B[m[32m+    .with_category(
(B[m[32m+        if success { "Added" } else { "Fixed" },
(B[m[32m+        categories
(B[m[32m+            .remove(if success { "Added" } else { "Fixed" })
(B[m[32m+            .unwrap_or_default(),
(B[m[32m+    )
(B[m }
 
 #[cfg(test)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher/src/main.rs:52:
         anyhow::bail!("Could not determine remote URL");
     }
 
[31m-    Ok(String::from_utf8(output.stdout)?
(B[m[31m-        .trim()
(B[m[31m-        .to_string())
(B[m[32m+    Ok(String::from_utf8(output.stdout)?.trim().to_string())
(B[m }
 
 #[tokio::main]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher/src/main.rs:92:
             Ok(current_sha) => {
                 if let Some(ref prev) = last_sha {
                     if prev != &current_sha {
[31m-                        info!("🔄 New commit detected: {} -> {}", &prev[..8], &current_sha[..8]);
(B[m[31m-                        handle_new_commit(&args, &current_sha, &repo_url, &webhook_url)
(B[m[31m-                            .await;
(B[m[32m+                        info!(
(B[m[32m+                            "🔄 New commit detected: {} -> {}",
(B[m[32m+                            &prev[..8],
(B[m[32m+                            &current_sha[..8]
(B[m[32m+                        );
(B[m[32m+                        handle_new_commit(&args, &current_sha, &repo_url, &webhook_url).await;
(B[m                         last_sha = Some(current_sha);
                     }
                 } else {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher/src/main.rs:111:
     }
 }
 
[31m-async fn handle_new_commit(
(B[m[31m-    args: &Args,
(B[m[31m-    sha: &str,
(B[m[31m-    repo_url: &str,
(B[m[31m-    webhook_url: &str,
(B[m[31m-) {
(B[m[32m+async fn handle_new_commit(args: &Args, sha: &str, repo_url: &str, webhook_url: &str) {
(B[m     info!("🚀 Running CI for {}", &sha[..8]);
 
     let sandbox = match create_sandbox(&args.temp_base) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ci-watcher/src/main.rs:142:
                         }
 
                         // Post to Discord
[31m-                        let payload =
(B[m[31m-                            format_ci_result(success, sha, &output, "FocalPoint");
(B[m[32m+                        let payload = format_ci_result(success, sha, &output, "FocalPoint");
(B[m                         if let Err(e) =
                             focus_release_bot::post_to_webhook_blocking(webhook_url, payload)
                         {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/examples/backup_restore.rs:57:
 
     match cli.command {
         Commands::Backup { action } => match action {
[31m-            BackupAction::Create { out, passphrase_from_env } => {
(B[m[32m+            BackupAction::Create {
(B[m[32m+                out,
(B[m[32m+                passphrase_from_env,
(B[m[32m+            } => {
(B[m                 let _passphrase = get_passphrase(&passphrase_from_env)?;
                 // Placeholder: real impl calls focus_backup::create_backup
                 println!("Creating encrypted backup to {}", out.display());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/examples/backup_restore.rs:67:
                 );
                 Ok(())
             }
[31m-            BackupAction::Restore { in_file, passphrase_from_env } => {
(B[m[32m+            BackupAction::Restore {
(B[m[32m+                in_file,
(B[m[32m+                passphrase_from_env,
(B[m[32m+            } => {
(B[m                 let _data = fs::read(&in_file)?;
                 let _passphrase = get_passphrase(&passphrase_from_env)?;
                 println!("Restoring backup from {}", in_file.display());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:299:
     /// List all tasks for the default user.
     #[command(about = "List all tasks (optionally filtered by user_id)")]
     List {
[31m-        #[arg(long, help = "Filter by user UUID (default: 00000000-0000-0000-0000-000000000000)")]
(B[m[32m+        #[arg(
(B[m[32m+            long,
(B[m[32m+            help = "Filter by user UUID (default: 00000000-0000-0000-0000-000000000000)"
(B[m[32m+        )]
(B[m         user_id: Option<String>,
     },
     /// Add a new task.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:403:
     /// Upsert a rule from a file (.toml, .fpl, or .json).
     #[command(about = "Create or update rule from TOML/FPL/JSON file")]
     Upsert {
[31m-        #[arg(long, help = ".toml (template-pack), .fpl (focus-lang), or .json (IR doc)")]
(B[m[32m+        #[arg(
(B[m[32m+            long,
(B[m[32m+            help = ".toml (template-pack), .fpl (focus-lang), or .json (IR doc)"
(B[m[32m+        )]
(B[m         file: PathBuf,
     },
     /// Bulk import rules from CSV or YAML.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:535:
         #[arg(long, default_value = "md")]
         format: String,
         /// Synthesize release notes via LLM (requires FOCALPOINT_RELEASE_NOTES_LLM env var)
[31m-        #[arg(long, help = "Use LLM to synthesize release notes (requires LLM endpoint env var)")]
(B[m[32m+        #[arg(
(B[m[32m+            long,
(B[m[32m+            help = "Use LLM to synthesize release notes (requires LLM endpoint env var)"
(B[m[32m+        )]
(B[m         synthesize: bool,
     },
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:578:
     }
 }
 
[31m-
(B[m fn resolve_db_path(explicit: Option<PathBuf>) -> anyhow::Result<PathBuf> {
     if let Some(p) = explicit {
         return Ok(p);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:593:
 
 fn open_adapter(db: &std::path::Path) -> anyhow::Result<SqliteAdapter> {
     if !db.exists() {
[31m-        anyhow::bail!("db not found at {} — launch the app once first, or pass --db", db.display());
(B[m[32m+        anyhow::bail!(
(B[m[32m+            "db not found at {} — launch the app once first, or pass --db",
(B[m[32m+            db.display()
(B[m[32m+        );
(B[m     }
     SqliteAdapter::open(db).map_err(|e| anyhow::anyhow!("open db: {e}"))
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:655:
     let store = SqliteTaskStore::from_adapter(&adapter);
     match cmd {
         TasksCmd::List { user_id } => {
[31m-            let uid = user_id.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(Uuid::nil());
(B[m[32m+            let uid = user_id
(B[m[32m+                .map(|s| Uuid::parse_str(&s))
(B[m[32m+                .transpose()?
(B[m[32m+                .unwrap_or(Uuid::nil());
(B[m             let tasks = store.list(uid)?;
             if json_output {
                 let json_tasks: Vec<TaskJson> = tasks
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:685:
             }
             Ok(())
         }
[31m-        TasksCmd::Add { title, minutes, priority, deadline } => {
(B[m[32m+        TasksCmd::Add {
(B[m[32m+            title,
(B[m[32m+            minutes,
(B[m[32m+            priority,
(B[m[32m+            deadline,
(B[m[32m+        } => {
(B[m             let uid = Uuid::nil();
             let prio = match priority {
                 Some('h') | Some('H') => focus_planning::Priority::clamped(0.8),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:741:
         }
         TasksCmd::Done { id } => {
             let task_id = Uuid::parse_str(&id)?;
[31m-            let mut task =
(B[m[31m-                store.get(task_id)?.ok_or_else(|| anyhow::anyhow!("task not found: {}", id))?;
(B[m[31m-            if !task.status.can_transition_to(&focus_planning::TaskStatus::Completed) {
(B[m[31m-                anyhow::bail!("task status {:?} cannot transition to Completed", task.status);
(B[m[32m+            let mut task = store
(B[m[32m+                .get(task_id)?
(B[m[32m+                .ok_or_else(|| anyhow::anyhow!("task not found: {}", id))?;
(B[m[32m+            if !task
(B[m[32m+                .status
(B[m[32m+                .can_transition_to(&focus_planning::TaskStatus::Completed)
(B[m[32m+            {
(B[m[32m+                anyhow::bail!(
(B[m[32m+                    "task status {:?} cannot transition to Completed",
(B[m[32m+                    task.status
(B[m[32m+                );
(B[m             }
             task.status = focus_planning::TaskStatus::Completed;
             task.updated_at = Utc::now();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:791:
             if json_output {
                 println!("{}", serde_json::to_string(&import_result)?);
             } else {
[31m-                println!("Parsed {} tasks", import_result.validation_report.valid_count);
(B[m[32m+                println!(
(B[m[32m+                    "Parsed {} tasks",
(B[m[32m+                    import_result.validation_report.valid_count
(B[m[32m+                );
(B[m                 if !import_result.validation_report.errors.is_empty() {
                     println!(
                         "Skipped {} rows with errors:",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:852:
                     );
                 }
             } else if json_output {
[31m-                println!("{{ \"mode\": \"dry_run\", \"tasks_to_import\": {} }}", import_result.validation_report.valid_count);
(B[m[32m+                println!(
(B[m[32m+                    "{{ \"mode\": \"dry_run\", \"tasks_to_import\": {} }}",
(B[m[32m+                    import_result.validation_report.valid_count
(B[m[32m+                );
(B[m             } else {
[31m-                println!("[DRY RUN] Would import {} tasks", import_result.validation_report.valid_count);
(B[m[32m+                println!(
(B[m[32m+                    "[DRY RUN] Would import {} tasks",
(B[m[32m+                    import_result.validation_report.valid_count
(B[m[32m+                );
(B[m             }
             Ok(())
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:897:
             };
 
             if let Some(output_path) = output {
[31m-                std::fs::write(&output_path, &content)
(B[m[31m-                    .map_err(|e| anyhow::anyhow!("failed to write {}: {}", output_path.display(), e))?;
(B[m[32m+                std::fs::write(&output_path, &content).map_err(|e| {
(B[m[32m+                    anyhow::anyhow!("failed to write {}: {}", output_path.display(), e)
(B[m[32m+                })?;
(B[m                 if json_output {
[31m-                    println!("{{ \"exported\": {}, \"file\": \"{}\" }}", tasks.len(), output_path.display());
(B[m[32m+                    println!(
(B[m[32m+                        "{{ \"exported\": {}, \"file\": \"{}\" }}",
(B[m[32m+                        tasks.len(),
(B[m[32m+                        output_path.display()
(B[m[32m+                    );
(B[m                 } else {
[31m-                    println!("Exported {} tasks to {}", tasks.len(), output_path.display());
(B[m[32m+                    println!(
(B[m[32m+                        "Exported {} tasks to {}",
(B[m[32m+                        tasks.len(),
(B[m[32m+                        output_path.display()
(B[m[32m+                    );
(B[m                 }
             } else {
                 println!("{}", content);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:922:
             let dir = std::env::var("FOCALPOINT_EXAMPLES")
                 .map(PathBuf::from)
                 .ok()
[31m-                .or_else(|| std::env::current_dir().ok().map(|p| p.join("examples/templates")))
(B[m[32m+                .or_else(|| {
(B[m[32m+                    std::env::current_dir()
(B[m[32m+                        .ok()
(B[m[32m+                        .map(|p| p.join("examples/templates"))
(B[m[32m+                })
(B[m                 .ok_or_else(|| anyhow::anyhow!("examples/templates not found"))?;
             if !dir.is_dir() {
                 anyhow::bail!("{} is not a directory", dir.display());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:965:
             }
             Ok(())
         }
[31m-        TemplatesCmd::Install { pack_id, manifest, require_signature } => {
(B[m[32m+        TemplatesCmd::Install {
(B[m[32m+            pack_id,
(B[m[32m+            manifest,
(B[m[32m+            require_signature,
(B[m[32m+        } => {
(B[m             // Try to load pack_id as a file path first, then fall back to bundled registry.
             let path = PathBuf::from(&pack_id);
             let text = if path.is_file() {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:976:
                 let example_dir = std::env::var("FOCALPOINT_EXAMPLES")
                     .map(PathBuf::from)
                     .ok()
[31m-                    .or_else(|| std::env::current_dir().ok().map(|p| p.join("examples/templates")))
(B[m[32m+                    .or_else(|| {
(B[m[32m+                        std::env::current_dir()
(B[m[32m+                            .ok()
(B[m[32m+                            .map(|p| p.join("examples/templates"))
(B[m[32m+                    })
(B[m                     .ok_or_else(|| anyhow::anyhow!("examples/templates not found"))?;
                 let bundled = example_dir.join(format!("{}.toml", pack_id));
                 std::fs::read_to_string(&bundled)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1179:
             if json_output {
                 println!("{}", serde_json::to_string(&import_result)?);
             } else {
[31m-                println!("Parsed {} rules", import_result.validation_report.valid_count);
(B[m[32m+                println!(
(B[m[32m+                    "Parsed {} rules",
(B[m[32m+                    import_result.validation_report.valid_count
(B[m[32m+                );
(B[m                 if !import_result.validation_report.errors.is_empty() {
                     println!(
                         "Skipped {} rows with errors:",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1201:
                             "Event" => focus_rules::Trigger::Event(
                                 rule_yaml.event_type.clone().unwrap_or_default(),
                             ),
[31m-                            "Schedule" => {
(B[m[31m-                                focus_rules::Trigger::Schedule(rule_yaml.event_type.clone().unwrap_or_default())
(B[m[31m-                            }
(B[m[31m-                            "StateChange" => {
(B[m[31m-                                focus_rules::Trigger::StateChange(rule_yaml.event_type.clone().unwrap_or_default())
(B[m[31m-                            }
(B[m[32m+                            "Schedule" => focus_rules::Trigger::Schedule(
(B[m[32m+                                rule_yaml.event_type.clone().unwrap_or_default(),
(B[m[32m+                            ),
(B[m[32m+                            "StateChange" => focus_rules::Trigger::StateChange(
(B[m[32m+                                rule_yaml.event_type.clone().unwrap_or_default(),
(B[m[32m+                            ),
(B[m                             _ => continue,
                         },
                         conditions: Vec::new(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1225:
                             "Unblock" => focus_rules::Action::Unblock {
                                 profile: "default".to_string(),
                             },
[31m-                            "Notify" => {
(B[m[31m-                                focus_rules::Action::Notify("Notification".to_string())
(B[m[31m-                            }
(B[m[32m+                            "Notify" => focus_rules::Action::Notify("Notification".to_string()),
(B[m                             _ => focus_rules::Action::Notify("Imported action".to_string()),
                         }],
                         priority: rule_yaml.priority,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1234:
                         cooldown: rule_yaml.cooldown.as_ref().map(|s| {
                             chrono::Duration::minutes(
[31m-                                s.trim_end_matches('m')
(B[m[31m-                                    .parse::<i64>()
(B[m[31m-                                    .unwrap_or(5),
(B[m[32m+                                s.trim_end_matches('m').parse::<i64>().unwrap_or(5),
(B[m                             )
                         }),
                         duration: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1256:
                     );
                 }
             } else if json_output {
[31m-                println!("{{ \"mode\": \"dry_run\", \"rules_to_import\": {} }}", import_result.validation_report.valid_count);
(B[m[32m+                println!(
(B[m[32m+                    "{{ \"mode\": \"dry_run\", \"rules_to_import\": {} }}",
(B[m[32m+                    import_result.validation_report.valid_count
(B[m[32m+                );
(B[m             } else {
[31m-                println!("[DRY RUN] Would import {} rules", import_result.validation_report.valid_count);
(B[m[32m+                println!(
(B[m[32m+                    "[DRY RUN] Would import {} rules",
(B[m[32m+                    import_result.validation_report.valid_count
(B[m[32m+                );
(B[m             }
             Ok(())
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1384:
             };
 
             if let Some(output_path) = output {
[31m-                std::fs::write(&output_path, &content)
(B[m[31m-                    .map_err(|e| anyhow::anyhow!("failed to write {}: {}", output_path.display(), e))?;
(B[m[32m+                std::fs::write(&output_path, &content).map_err(|e| {
(B[m[32m+                    anyhow::anyhow!("failed to write {}: {}", output_path.display(), e)
(B[m[32m+                })?;
(B[m                 if json_output {
[31m-                    println!("{{ \"exported\": {}, \"file\": \"{}\" }}", rules.len(), output_path.display());
(B[m[32m+                    println!(
(B[m[32m+                        "{{ \"exported\": {}, \"file\": \"{}\" }}",
(B[m[32m+                        rules.len(),
(B[m[32m+                        output_path.display()
(B[m[32m+                    );
(B[m                 } else {
[31m-                    println!("Exported {} rules to {}", rules.len(), output_path.display());
(B[m[32m+                    println!(
(B[m[32m+                        "Exported {} rules to {}",
(B[m[32m+                        rules.len(),
(B[m[32m+                        output_path.display()
(B[m[32m+                    );
(B[m                 }
             } else {
                 println!("{}", content);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1407:
     let user_id = Uuid::nil();
     match cmd {
         WalletCmd::Balance { user_id: uid_opt } => {
[31m-            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(user_id);
(B[m[32m+            let uid = uid_opt
(B[m[32m+                .map(|s| Uuid::parse_str(&s))
(B[m[32m+                .transpose()?
(B[m[32m+                .unwrap_or(user_id);
(B[m             let wallet = rt.block_on((&adapter as &dyn WalletStore).load(uid))?;
             if json_output {
                 let result = WalletState {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1435:
             }
             Ok(())
         }
[31m-        WalletCmd::Grant { amount, purpose, user_id: uid_opt } => {
(B[m[31m-            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(user_id);
(B[m[32m+        WalletCmd::Grant {
(B[m[32m+            amount,
(B[m[32m+            purpose,
(B[m[32m+            user_id: uid_opt,
(B[m[32m+        } => {
(B[m[32m+            let uid = uid_opt
(B[m[32m+                .map(|s| Uuid::parse_str(&s))
(B[m[32m+                .transpose()?
(B[m[32m+                .unwrap_or(user_id);
(B[m             if amount <= 0 {
                 anyhow::bail!("amount must be positive, got {}", amount);
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1443:
[31m-            let before = rt.block_on((&adapter as &dyn WalletStore).load(uid))?.balance();
(B[m[32m+            let before = rt
(B[m[32m+                .block_on((&adapter as &dyn WalletStore).load(uid))?
(B[m[32m+                .balance();
(B[m             let mutation = focus_rewards::WalletMutation::GrantCredit(focus_rewards::Credit {
                 amount,
                 source_rule_id: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1447:
                 granted_at: Utc::now(),
             });
             rt.block_on((&adapter as &dyn WalletStore).apply(uid, mutation))?;
[31m-            let after = rt.block_on((&adapter as &dyn WalletStore).load(uid))?.balance();
(B[m[32m+            let after = rt
(B[m[32m+                .block_on((&adapter as &dyn WalletStore).load(uid))?
(B[m[32m+                .balance();
(B[m             if json_output {
                 let result = WalletOperation {
                     balance_before: before,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1461:
             }
             Ok(())
         }
[31m-        WalletCmd::Spend { amount, purpose, user_id: uid_opt } => {
(B[m[31m-            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(user_id);
(B[m[32m+        WalletCmd::Spend {
(B[m[32m+            amount,
(B[m[32m+            purpose,
(B[m[32m+            user_id: uid_opt,
(B[m[32m+        } => {
(B[m[32m+            let uid = uid_opt
(B[m[32m+                .map(|s| Uuid::parse_str(&s))
(B[m[32m+                .transpose()?
(B[m[32m+                .unwrap_or(user_id);
(B[m             if amount <= 0 {
                 anyhow::bail!("amount must be positive, got {}", amount);
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1469:
[31m-            let before = rt.block_on((&adapter as &dyn WalletStore).load(uid))?.balance();
(B[m[31m-            let mutation =
(B[m[31m-                focus_rewards::WalletMutation::SpendCredit { amount, purpose: purpose.clone() };
(B[m[32m+            let before = rt
(B[m[32m+                .block_on((&adapter as &dyn WalletStore).load(uid))?
(B[m[32m+                .balance();
(B[m[32m+            let mutation = focus_rewards::WalletMutation::SpendCredit {
(B[m[32m+                amount,
(B[m[32m+                purpose: purpose.clone(),
(B[m[32m+            };
(B[m             rt.block_on((&adapter as &dyn WalletStore).apply(uid, mutation))?;
[31m-            let after = rt.block_on((&adapter as &dyn WalletStore).load(uid))?.balance();
(B[m[32m+            let after = rt
(B[m[32m+                .block_on((&adapter as &dyn WalletStore).load(uid))?
(B[m[32m+                .balance();
(B[m             if json_output {
                 let result = WalletOperation {
                     balance_before: before,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1494:
     let rt = tokio::runtime::Runtime::new()?;
     match cmd {
         PenaltyCmd::Show { user_id: uid_opt } => {
[31m-            let uid = uid_opt.map(|s| Uuid::parse_str(&s)).transpose()?.unwrap_or(Uuid::nil());
(B[m[32m+            let uid = uid_opt
(B[m[32m+                .map(|s| Uuid::parse_str(&s))
(B[m[32m+                .transpose()?
(B[m[32m+                .unwrap_or(Uuid::nil());
(B[m             let state = rt.block_on((&adapter as &dyn PenaltyStore).load(uid))?;
             if json_output {
                 let lockout_windows: Vec<LockoutWindow> = state
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1636:
                 };
                 println!("{}", serde_json::to_string(&result)?);
             } else {
[31m-                println!("focus:session_started (minutes={}) [test event emitted]", minutes);
(B[m[32m+                println!(
(B[m[32m+                    "focus:session_started (minutes={}) [test event emitted]",
(B[m[32m+                    minutes
(B[m[32m+                );
(B[m             }
             Ok(())
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1649:
                 };
                 println!("{}", serde_json::to_string(&result)?);
             } else {
[31m-                println!("focus:session_completed (minutes={}) [test event emitted]", minutes);
(B[m[32m+                println!(
(B[m[32m+                    "focus:session_completed (minutes={}) [test event emitted]",
(B[m[32m+                    minutes
(B[m[32m+                );
(B[m             }
             Ok(())
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1668:
 
 fn run_release_notes(cmd: ReleaseNotesCmd, json_output: bool) -> anyhow::Result<()> {
     match cmd {
[31m-        ReleaseNotesCmd::Generate { since, format, synthesize } => {
(B[m[32m+        ReleaseNotesCmd::Generate {
(B[m[32m+            since,
(B[m[32m+            format,
(B[m[32m+            synthesize,
(B[m[32m+        } => {
(B[m             let commits = fetch_git_log(&since)?;
             let grouped = group_commits_by_type(&commits);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1708:
 
 fn fetch_git_log(since: &str) -> anyhow::Result<Vec<CommitInfo>> {
     let output = Command::new("git")
[31m-        .args(["log", &format!("{}..HEAD", since), "--oneline", "--pretty=format:%H|%s|%b"])
(B[m[32m+        .args([
(B[m[32m+            "log",
(B[m[32m+            &format!("{}..HEAD", since),
(B[m[32m+            "--oneline",
(B[m[32m+            "--pretty=format:%H|%s|%b",
(B[m[32m+        ])
(B[m         .output()?;
 
     if !output.status.success() {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1715:
[31m-        anyhow::bail!("git log failed: {}", String::from_utf8_lossy(&output.stderr));
(B[m[32m+        anyhow::bail!(
(B[m[32m+            "git log failed: {}",
(B[m[32m+            String::from_utf8_lossy(&output.stderr)
(B[m[32m+        );
(B[m     }
 
     let text = String::from_utf8(output.stdout)?;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1781:
             let (category, _) = get_category_display(typ);
             println!("\n### {}", category);
             for commit in commits {
[31m-                let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
(B[m[32m+                let subject = commit
(B[m[32m+                    .subject
(B[m[32m+                    .split(':')
(B[m[32m+                    .nth(1)
(B[m[32m+                    .unwrap_or(&commit.subject)
(B[m[32m+                    .trim();
(B[m                 println!("- {} ({})", subject, &commit.hash[..7]);
             }
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1808:
             let (category, emoji) = get_category_display(typ);
             println!("{} **{}**", emoji, category);
             for commit in commits {
[31m-                let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
(B[m[32m+                let subject = commit
(B[m[32m+                    .subject
(B[m[32m+                    .split(':')
(B[m[32m+                    .nth(1)
(B[m[32m+                    .unwrap_or(&commit.subject)
(B[m[32m+                    .trim();
(B[m                 println!("  • {}", subject);
             }
             println!();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1829:
             let (category, _) = get_category_display(typ);
             output.push_str(&format!("\n{}:\n", category));
             for commit in commits {
[31m-                let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
(B[m[32m+                let subject = commit
(B[m[32m+                    .subject
(B[m[32m+                    .split(':')
(B[m[32m+                    .nth(1)
(B[m[32m+                    .unwrap_or(&commit.subject)
(B[m[32m+                    .trim();
(B[m                 let line = format!("• {}\n", subject);
                 if output.len() + line.len() > max_len {
                     output.push_str("...[truncated]");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1854:
             let items: Vec<String> = commits
                 .iter()
                 .map(|commit| {
[31m-                    commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim().to_string()
(B[m[32m+                    commit
(B[m[32m+                        .subject
(B[m[32m+                        .split(':')
(B[m[32m+                        .nth(1)
(B[m[32m+                        .unwrap_or(&commit.subject)
(B[m[32m+                        .trim()
(B[m[32m+                        .to_string()
(B[m                 })
                 .collect();
[31m-            sections.push(ReleaseSection { category: category.to_string(), items });
(B[m[32m+            sections.push(ReleaseSection {
(B[m[32m+                category: category.to_string(),
(B[m[32m+                items,
(B[m[32m+            });
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1878:
     for (category, commits) in grouped {
         commit_list.push_str(&format!("\n{}:\n", category));
         for commit in commits {
[31m-            let subject = commit.subject.split(':').nth(1).unwrap_or(&commit.subject).trim();
(B[m[32m+            let subject = commit
(B[m[32m+                .subject
(B[m[32m+                .split(':')
(B[m[32m+                .nth(1)
(B[m[32m+                .unwrap_or(&commit.subject)
(B[m[32m+                .trim();
(B[m             commit_list.push_str(&format!("- {}\n", subject));
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1888:
         format, commit_list
     );
 
[31m-    let client =
(B[m[31m-        reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(30)).build()?;
(B[m[32m+    let client = reqwest::blocking::Client::builder()
(B[m[32m+        .timeout(std::time::Duration::from_secs(30))
(B[m[32m+        .build()?;
(B[m 
     let body = serde_json::json!({
         "prompt": prompt,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1920:
     let registry_url = std::env::var("FOCALPOINT_TEMPLATE_REGISTRY")
         .unwrap_or_else(|_| "https://packs.focalpoint.app/api/v1/search".to_string());
 
[31m-    let client =
(B[m[31m-        reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(10)).build()?;
(B[m[32m+    let client = reqwest::blocking::Client::builder()
(B[m[32m+        .timeout(std::time::Duration::from_secs(10))
(B[m[32m+        .build()?;
(B[m 
     let search_url = format!("{}?q={}", registry_url, urlencoding::encode(query));
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1956:
     let dir = std::env::var("FOCALPOINT_EXAMPLES")
         .map(PathBuf::from)
         .ok()
[31m-        .or_else(|| std::env::current_dir().ok().map(|p| p.join("examples/templates")))
(B[m[32m+        .or_else(|| {
(B[m[32m+            std::env::current_dir()
(B[m[32m+                .ok()
(B[m[32m+                .map(|p| p.join("examples/templates"))
(B[m[32m+        })
(B[m         .ok_or_else(|| anyhow::anyhow!("examples/templates not found"))?;
 
     if !dir.is_dir() {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:1999:
             println!("no templates found matching '{}'", query);
         } else {
             for result in results {
[31m-                println!("{}  {}  (local)  by {}", result.id, result.name, result.author);
(B[m[32m+                println!(
(B[m[32m+                    "{}  {}  (local)  by {}",
(B[m[32m+                    result.id, result.name, result.author
(B[m[32m+                );
(B[m             }
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:2012:
     let registry_url = std::env::var("FOCALPOINT_TEMPLATE_REGISTRY")
         .unwrap_or_else(|_| "https://packs.focalpoint.app/api/v1".to_string());
 
[31m-    let client =
(B[m[31m-        reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(10)).build()?;
(B[m[32m+    let client = reqwest::blocking::Client::builder()
(B[m[32m+        .timeout(std::time::Duration::from_secs(10))
(B[m[32m+        .build()?;
(B[m 
     let show_url = format!("{}/packs/{}", registry_url, pack_id);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:2047:
     let dir = std::env::var("FOCALPOINT_EXAMPLES")
         .map(PathBuf::from)
         .ok()
[31m-        .or_else(|| std::env::current_dir().ok().map(|p| p.join("examples/templates")))
(B[m[32m+        .or_else(|| {
(B[m[32m+            std::env::current_dir()
(B[m[32m+                .ok()
(B[m[32m+                .map(|p| p.join("examples/templates"))
(B[m[32m+        })
(B[m         .ok_or_else(|| anyhow::anyhow!("examples/templates not found"))?;
 
     let bundled = dir.join(format!("{}.toml", pack_id));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:2087:
 
     let token = std::env::var("FOCALPOINT_TEMPLATE_TOKEN").ok();
 
[31m-    let client =
(B[m[31m-        reqwest::blocking::Client::builder().timeout(std::time::Duration::from_secs(10)).build()?;
(B[m[32m+    let client = reqwest::blocking::Client::builder()
(B[m[32m+        .timeout(std::time::Duration::from_secs(10))
(B[m[32m+        .build()?;
(B[m 
     let rate_url = format!("{}/packs/{}/rate", registry_url, pack_id);
     let body = serde_json::json!({ "rating": rating });
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:2149:
     let trusted_file = config_dir.join("trusted-keys.toml");
     if !trusted_file.exists() {
         // Fallback to compile-time roots
[31m-        return Ok(focus_templates::PHENOTYPE_ROOT_PUBKEYS.iter().map(|s| s.to_string()).collect());
(B[m[32m+        return Ok(focus_templates::PHENOTYPE_ROOT_PUBKEYS
(B[m[32m+            .iter()
(B[m[32m+            .map(|s| s.to_string())
(B[m[32m+            .collect());
(B[m     }
 
     let text = std::fs::read_to_string(&trusted_file)?;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/main.rs:2701:
     })
 }
 
[31m-async fn run_replay(
(B[m[31m-    sub: replay::ReplayCmd,
(B[m[31m-    db_path: &Path,
(B[m[31m-    json: bool,
(B[m[31m-) -> anyhow::Result<()> {
(B[m[32m+async fn run_replay(sub: replay::ReplayCmd, db_path: &Path, json: bool) -> anyhow::Result<()> {
(B[m     let adapter = Arc::new(SqliteAdapter::open(db_path)?);
     let result = replay::execute(adapter, sub).await?;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/replay.rs:35:
 }
 
 /// Execute a replay operation.
[31m-pub async fn execute(
(B[m[31m-    adapter: Arc<SqliteAdapter>,
(B[m[31m-    cmd: ReplayCmd,
(B[m[31m-) -> anyhow::Result<String> {
(B[m[32m+pub async fn execute(adapter: Arc<SqliteAdapter>, cmd: ReplayCmd) -> anyhow::Result<String> {
(B[m     match cmd {
         ReplayCmd::Window {
             since,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/replay.rs:104:
     match format {
         "json" => Ok(serde_json::to_string_pretty(&report)?),
         "markdown" => Ok(report.to_markdown()),
[31m-        other => Err(anyhow!("Unknown format: {}. Use 'json' or 'markdown'", other)),
(B[m[32m+        other => Err(anyhow!(
(B[m[32m+            "Unknown format: {}. Use 'json' or 'markdown'",
(B[m[32m+            other
(B[m[32m+        )),
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/src/replay.rs:112:
 fn parse_toml_ruleset(content: &str) -> anyhow::Result<Vec<Rule>> {
     // Stub: in production, this would deserialize TOML into Rule structs.
     // For now, return an empty vec (tests will handle the parsing).
[31m-    let _rules: toml::Value = toml::from_str(content)
(B[m[31m-        .map_err(|e| anyhow!("Invalid TOML: {}", e))?;
(B[m[32m+    let _rules: toml::Value =
(B[m[32m+        toml::from_str(content).map_err(|e| anyhow!("Invalid TOML: {}", e))?;
(B[m     Ok(Vec::new())
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:8:
 
 fn test_db_path() -> PathBuf {
     // Use a test database; in real tests, create temporary DB with fixtures.
[31m-    std::env::var("FOCALPOINT_DB").map(PathBuf::from).unwrap_or_else(|_| {
(B[m[31m-        PathBuf::from(std::env::home_dir().unwrap_or_default())
(B[m[31m-            .join("Library/Application Support/focalpoint/core.db")
(B[m[31m-    })
(B[m[32m+    std::env::var("FOCALPOINT_DB")
(B[m[32m+        .map(PathBuf::from)
(B[m[32m+        .unwrap_or_else(|_| {
(B[m[32m+            PathBuf::from(std::env::home_dir().unwrap_or_default())
(B[m[32m+                .join("Library/Application Support/focalpoint/core.db")
(B[m[32m+        })
(B[m }
 
 fn setup_test_db() -> PathBuf {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:28:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("audit").arg("verify");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("audit")
(B[m[32m+        .arg("verify");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:47:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("audit").arg("tail").arg("--limit").arg("5");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("audit")
(B[m[32m+        .arg("tail")
(B[m[32m+        .arg("--limit")
(B[m[32m+        .arg("5");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:71:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("audit").arg("head");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("audit")
(B[m[32m+        .arg("head");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:90:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("tasks").arg("list");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("tasks")
(B[m[32m+        .arg("list");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:150:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("rules").arg("list");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("rules")
(B[m[32m+        .arg("list");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:178:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("wallet").arg("balance");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("wallet")
(B[m[32m+        .arg("balance");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:256:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("penalty").arg("show");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("penalty")
(B[m[32m+        .arg("show");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:278:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("focus").arg("start").arg("45");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("focus")
(B[m[32m+        .arg("start")
(B[m[32m+        .arg("45");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:298:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("--json").arg("focus").arg("complete").arg("45");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("--json")
(B[m[32m+        .arg("focus")
(B[m[32m+        .arg("complete")
(B[m[32m+        .arg("45");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:305:
 
     let result: Value = serde_json::from_str(&json_str).expect("valid json");
     assert!(result.is_object());
[31m-    assert_eq!(result["event_type"].as_str(), Some("focus:session_completed"));
(B[m[32m+    assert_eq!(
(B[m[32m+        result["event_type"].as_str(),
(B[m[32m+        Some("focus:session_completed")
(B[m[32m+    );
(B[m     assert_eq!(result["minutes"].as_i64(), Some(45));
     assert!(result["timestamp"].is_string());
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:338:
 #[ignore = "TBD: see test fixture in tests/fixtures/release-notes/"]
 fn test_release_notes_json() {
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--json").arg("release-notes").arg("generate").arg("--since").arg("v0.0.3");
(B[m[32m+    cmd.arg("--json")
(B[m[32m+        .arg("release-notes")
(B[m[32m+        .arg("generate")
(B[m[32m+        .arg("--since")
(B[m[32m+        .arg("v0.0.3");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/json_output_test.rs:384:
     }
 
     let mut cmd = Command::cargo_bin("focus").expect("bin exists");
[31m-    cmd.arg("--db").arg(&db).arg("-j").arg("wallet").arg("balance");
(B[m[32m+    cmd.arg("--db")
(B[m[32m+        .arg(&db)
(B[m[32m+        .arg("-j")
(B[m[32m+        .arg("wallet")
(B[m[32m+        .arg("balance");
(B[m 
     let output = cmd.output().expect("command ran");
     let json_str = String::from_utf8(output.stdout).expect("valid utf8");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/release_notes_llm.rs:9:
     /// Should attempt to POST grouped commits to LLM endpoint.
     #[test]
     fn test_release_notes_synthesize_with_env_var() {
[31m-        env::set_var("FOCALPOINT_RELEASE_NOTES_LLM", "http://localhost:8000/synthesize");
(B[m[32m+        env::set_var(
(B[m[32m+            "FOCALPOINT_RELEASE_NOTES_LLM",
(B[m[32m+            "http://localhost:8000/synthesize",
(B[m[32m+        );
(B[m         assert_eq!(
             env::var("FOCALPOINT_RELEASE_NOTES_LLM").unwrap(),
             "http://localhost:8000/synthesize"
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-cli/tests/template_marketplace.rs:161:
     /// Test: Environment variable configuration for registry URL
     #[test]
     fn test_template_registry_url_env_var() {
[31m-        env::set_var("FOCALPOINT_TEMPLATE_REGISTRY", "https://packs.example.com/api/v1");
(B[m[32m+        env::set_var(
(B[m[32m+            "FOCALPOINT_TEMPLATE_REGISTRY",
(B[m[32m+            "https://packs.example.com/api/v1",
(B[m[32m+        );
(B[m         assert_eq!(
             env::var("FOCALPOINT_TEMPLATE_REGISTRY").unwrap(),
             "https://packs.example.com/api/v1"
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:132:
         }
         let mut messages = Vec::with_capacity(2);
         if let Some(sys) = system {
[31m-            messages.push(ChatMessage { role: "system", content: sys });
(B[m[32m+            messages.push(ChatMessage {
(B[m[32m+                role: "system",
(B[m[32m+                content: sys,
(B[m[32m+            });
(B[m         }
[31m-        messages.push(ChatMessage { role: "user", content: prompt });
(B[m[31m-        let req = ChatRequest { model: &self.model, messages, max_tokens, temperature: 0.3 };
(B[m[32m+        messages.push(ChatMessage {
(B[m[32m+            role: "user",
(B[m[32m+            content: prompt,
(B[m[32m+        });
(B[m[32m+        let req = ChatRequest {
(B[m[32m+            model: &self.model,
(B[m[32m+            messages,
(B[m[32m+            max_tokens,
(B[m[32m+            temperature: 0.3,
(B[m[32m+        };
(B[m         let url = format!("{}/chat/completions", self.endpoint.trim_end_matches('/'));
         info!(
             target: "coaching.request",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:216:
 
 impl StubCoachingProvider {
     pub fn new(responses: Vec<String>) -> Self {
[31m-        Self { responses: Arc::new(responses), cursor: Arc::new(Mutex::new(0)) }
(B[m[32m+        Self {
(B[m[32m+            responses: Arc::new(responses),
(B[m[32m+            cursor: Arc::new(Mutex::new(0)),
(B[m[32m+        }
(B[m     }
     pub fn single(resp: impl Into<String>) -> Self {
         Self::new(vec![resp.into()])
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:273:
 
 impl RateLimitedProvider {
     pub fn new(inner: Arc<dyn CoachingProvider>, capacity: u32, window: Duration) -> Self {
[31m-        Self { inner, bucket: Arc::new(Mutex::new(Bucket { capacity, window, calls: Vec::new() })) }
(B[m[32m+        Self {
(B[m[32m+            inner,
(B[m[32m+            bucket: Arc::new(Mutex::new(Bucket {
(B[m[32m+                capacity,
(B[m[32m+                window,
(B[m[32m+                calls: Vec::new(),
(B[m[32m+            })),
(B[m[32m+        }
(B[m     }
     /// Default: 10 calls per 60s.
     pub fn default_limits(inner: Arc<dyn CoachingProvider>) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:306:
 // --------------------------------------------------------------------------
 
 fn kill_switch_on() -> bool {
[31m-    std::env::var(KILL_SWITCH_ENV).map(|v| v == "1").unwrap_or(false)
(B[m[32m+    std::env::var(KILL_SWITCH_ENV)
(B[m[32m+        .map(|v| v == "1")
(B[m[32m+        .unwrap_or(false)
(B[m }
 
 /// Convenience: check-and-maybe-call. Respects the env kill switch even for
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:343:
     #[tokio::test]
     async fn stub_returns_canned_then_wraps() {
         let p = StubCoachingProvider::new(vec!["a".into(), "b".into()]);
[31m-        assert_eq!(p.complete("x", None, 8).await.unwrap().as_deref(), Some("a"));
(B[m[31m-        assert_eq!(p.complete("x", None, 8).await.unwrap().as_deref(), Some("b"));
(B[m[31m-        assert_eq!(p.complete("x", None, 8).await.unwrap().as_deref(), Some("a"));
(B[m[32m+        assert_eq!(
(B[m[32m+            p.complete("x", None, 8).await.unwrap().as_deref(),
(B[m[32m+            Some("a")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            p.complete("x", None, 8).await.unwrap().as_deref(),
(B[m[32m+            Some("b")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            p.complete("x", None, 8).await.unwrap().as_deref(),
(B[m[32m+            Some("a")
(B[m[32m+        );
(B[m     }
 
     #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:352:
     async fn stub_single_helper() {
         let p = StubCoachingProvider::single("one");
[31m-        assert_eq!(p.complete("x", None, 8).await.unwrap().as_deref(), Some("one"));
(B[m[32m+        assert_eq!(
(B[m[32m+            p.complete("x", None, 8).await.unwrap().as_deref(),
(B[m[32m+            Some("one")
(B[m[32m+        );
(B[m     }
 
     #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:374:
         let _g = ENV_LOCK.lock().expect("env lock");
         std::env::set_var(KILL_SWITCH_ENV, "1");
         let p = StubCoachingProvider::single("nope");
[31m-        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("rt");
(B[m[32m+        let rt = tokio::runtime::Builder::new_current_thread()
(B[m[32m+            .enable_all()
(B[m[32m+            .build()
(B[m[32m+            .expect("rt");
(B[m         let r = rt.block_on(complete_guarded(&p, "x", None, 8)).expect("ok");
         std::env::remove_var(KILL_SWITCH_ENV);
         assert!(r.is_none());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:385:
         let _g = ENV_LOCK.lock().expect("env lock");
         std::env::remove_var(KILL_SWITCH_ENV);
         let p = StubCoachingProvider::single("yes");
[31m-        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("rt");
(B[m[32m+        let rt = tokio::runtime::Builder::new_current_thread()
(B[m[32m+            .enable_all()
(B[m[32m+            .build()
(B[m[32m+            .expect("rt");
(B[m         let r = rt.block_on(complete_guarded(&p, "x", None, 8)).expect("ok");
         assert_eq!(r.as_deref(), Some("yes"));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-coaching/src/lib.rs:414:
     #[test]
     #[ignore = "TBD: feature spec not yet implemented"]
     fn test_fr_ux_002_connector_auth_platform_native() {
[31m-        unimplemented!("Connector auth flow is platform-native (SFSafariViewController / Custom Tabs)")
(B[m[32m+        unimplemented!(
(B[m[32m+            "Connector auth flow is platform-native (SFSafariViewController / Custom Tabs)"
(B[m[32m+        )
(B[m     }
 
     // Traces to: FR-UX-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/builder.rs:55:
         }
 
         impl $builder {
[31m-            pub fn new(
(B[m[31m-                client_id: impl Into<String>,
(B[m[31m-                client_secret: impl Into<String>,
(B[m[31m-            ) -> Self {
(B[m[32m+            pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
(B[m                 Self {
                     client_id: client_id.into(),
                     client_secret: client_secret.into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/builder.rs:74:
                 self
             }
 
[31m-            pub fn token_store(
(B[m[31m-                mut self,
(B[m[31m-                s: std::sync::Arc<$token_store>,
(B[m[31m-            ) -> Self {
(B[m[32m+            pub fn token_store(mut self, s: std::sync::Arc<$token_store>) -> Self {
(B[m                 self.token_store = Some(s);
                 self
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/derived.rs:65:
             version: "0.1.0".into(),
             display_name: display_name.into(),
             auth_strategy: crate::AuthStrategy::None,
[31m-            sync_mode: SyncMode::Polling { cadence_seconds: 60 },
(B[m[32m+            sync_mode: SyncMode::Polling {
(B[m[32m+                cadence_seconds: 60,
(B[m[32m+            },
(B[m             capabilities: vec![],
             entity_types: vec![],
             event_types,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/derived.rs:72:
             tier: VerificationTier::Verified,
             health_indicators: vec!["base_bases_healthy".into()],
         };
[31m-        Self { bases, transform, manifest }
(B[m[32m+        Self {
(B[m[32m+            bases,
(B[m[32m+            transform,
(B[m[32m+            manifest,
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/derived.rs:119:
             }
         }
         let derived = self.transform.transform(&combined);
[31m-        Ok(SyncOutcome { events: derived, next_cursor: max_cursor, partial })
(B[m[32m+        Ok(SyncOutcome {
(B[m[32m+            events: derived,
(B[m[32m+            next_cursor: max_cursor,
(B[m[32m+            partial,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/derived.rs:150:
                     version: "0.0.1".into(),
                     display_name: id.into(),
                     auth_strategy: AuthStrategy::None,
[31m-                    sync_mode: SyncMode::Polling { cadence_seconds: 60 },
(B[m[32m+                    sync_mode: SyncMode::Polling {
(B[m[32m+                        cadence_seconds: 60,
(B[m[32m+                    },
(B[m                     capabilities: vec![],
                     entity_types: vec![],
                     event_types: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:192:
 
     /// Filter catalog by tier for "Show only verified" UI toggles.
     pub fn catalog_by_tier(&self, tier: VerificationTier) -> Vec<ConnectorListing> {
[31m-        self.catalog().into_iter().filter(|l| l.manifest.tier == tier).collect()
(B[m[32m+        self.catalog()
(B[m[32m+            .into_iter()
(B[m[32m+            .filter(|l| l.manifest.tier == tier)
(B[m[32m+            .collect()
(B[m     }
 
     pub fn get(&self, connector_id: &str) -> Option<ConnectorListing> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:205:
     }
 
     pub fn len(&self) -> usize {
[31m-        self.listings.read().expect("connector registry poisoned").len()
(B[m[32m+        self.listings
(B[m[32m+            .read()
(B[m[32m+            .expect("connector registry poisoned")
(B[m[32m+            .len()
(B[m     }
 
     pub fn is_empty(&self) -> bool {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:257:
 /// runtime inbound HTTP layer (out of scope for this crate) needs.
 #[derive(Default)]
 pub struct WebhookRegistry {
[31m-    handlers: std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<dyn WebhookHandler>>>,
(B[m[32m+    handlers:
(B[m[32m+        std::sync::RwLock<std::collections::HashMap<String, std::sync::Arc<dyn WebhookHandler>>>,
(B[m }
 
 impl WebhookRegistry {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:265:
         Self::default()
     }
 
[31m-    pub fn register(&self, connector_id: impl Into<String>, handler: std::sync::Arc<dyn WebhookHandler>) {
(B[m[32m+    pub fn register(
(B[m[32m+        &self,
(B[m[32m+        connector_id: impl Into<String>,
(B[m[32m+        handler: std::sync::Arc<dyn WebhookHandler>,
(B[m[32m+    ) {
(B[m         let mut g = self.handlers.write().expect("webhook registry poisoned");
         g.insert(connector_id.into(), handler);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:276:
     }
 
     pub fn len(&self) -> usize {
[31m-        self.handlers.read().expect("webhook registry poisoned").len()
(B[m[32m+        self.handlers
(B[m[32m+            .read()
(B[m[32m+            .expect("webhook registry poisoned")
(B[m[32m+            .len()
(B[m     }
 
     pub fn is_empty(&self) -> bool {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:286:
     /// Dispatch a delivery to the registered handler for its `connector_id`.
     /// Returns `ConnectorError::NotFound` if no handler is registered.
     pub async fn dispatch(&self, delivery: &WebhookDelivery) -> Result<Vec<NormalizedEvent>> {
[31m-        let handler = self
(B[m[31m-            .get(&delivery.connector_id)
(B[m[31m-            .ok_or_else(|| ConnectorError::Schema(format!("no handler for {}", delivery.connector_id)))?;
(B[m[32m+        let handler = self.get(&delivery.connector_id).ok_or_else(|| {
(B[m[32m+            ConnectorError::Schema(format!("no handler for {}", delivery.connector_id))
(B[m[32m+        })?;
(B[m         handler.handle(delivery).await
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:313:
                 version: "0.0.1".into(),
                 display_name: id.into(),
                 auth_strategy: AuthStrategy::ApiKey,
[31m-                sync_mode: SyncMode::Polling { cadence_seconds: 60 },
(B[m[32m+                sync_mode: SyncMode::Polling {
(B[m[32m+                    cadence_seconds: 60,
(B[m[32m+                },
(B[m                 capabilities: vec![],
                 entity_types: vec![],
                 event_types: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:334:
         reg.register(mk_listing("gcal", VerificationTier::Official, 0));
         reg.register(mk_listing("private-x", VerificationTier::Private, 0));
         reg.register(mk_listing("github", VerificationTier::Verified, 0));
[31m-        let ids: Vec<_> = reg.catalog().iter().map(|l| l.manifest.id.clone()).collect();
(B[m[32m+        let ids: Vec<_> = reg
(B[m[32m+            .catalog()
(B[m[32m+            .iter()
(B[m[32m+            .map(|l| l.manifest.id.clone())
(B[m[32m+            .collect();
(B[m         assert_eq!(ids, vec!["gcal", "canvas", "github", "mcp-x", "private-x"]);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:364:
         reg.register(mk_listing("gcal", VerificationTier::Official, 1));
         let official = reg.catalog_by_tier(VerificationTier::Official);
         assert_eq!(official.len(), 2);
[31m-        assert!(official.iter().all(|l| l.manifest.tier == VerificationTier::Official));
(B[m[32m+        assert!(official
(B[m[32m+            .iter()
(B[m[32m+            .all(|l| l.manifest.tier == VerificationTier::Official));
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:390:
                 event_id: uuid::Uuid::new_v4(),
                 connector_id: self.id.clone(),
                 account_id: uuid::Uuid::nil(),
[31m-                event_type: focus_events::EventType::Custom(format!("{}.{}", self.id, delivery.kind)),
(B[m[32m+                event_type: focus_events::EventType::Custom(format!(
(B[m[32m+                    "{}.{}",
(B[m[32m+                    self.id, delivery.kind
(B[m[32m+                )),
(B[m                 occurred_at: delivery.received_at,
                 effective_at: delivery.received_at,
[31m-                dedupe_key: focus_events::DedupeKey(format!("{}:{}", self.id, delivery.received_at.timestamp_nanos_opt().unwrap_or(0))),
(B[m[32m+                dedupe_key: focus_events::DedupeKey(format!(
(B[m[32m+                    "{}:{}",
(B[m[32m+                    self.id,
(B[m[32m+                    delivery.received_at.timestamp_nanos_opt().unwrap_or(0)
(B[m[32m+                )),
(B[m                 confidence: 1.0,
                 payload: serde_json::json!({"kind": delivery.kind, "bytes": delivery.body.len()}),
                 raw_ref: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:414:
     #[tokio::test]
     async fn registry_dispatches_to_matching_handler() {
         let reg = WebhookRegistry::new();
[31m-        reg.register("github", Arc::new(EchoHandler { id: "github".into() }));
(B[m[32m+        reg.register(
(B[m[32m+            "github",
(B[m[32m+            Arc::new(EchoHandler {
(B[m[32m+                id: "github".into(),
(B[m[32m+            }),
(B[m[32m+        );
(B[m         let events = reg.dispatch(&mk_delivery("github", b"{}")).await.unwrap();
         assert_eq!(events.len(), 1);
         assert_eq!(events[0].connector_id, "github");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:423:
     #[tokio::test]
     async fn registry_errors_when_no_handler() {
         let reg = WebhookRegistry::new();
[31m-        let err = reg.dispatch(&mk_delivery("unknown", b"{}")).await.unwrap_err();
(B[m[32m+        let err = reg
(B[m[32m+            .dispatch(&mk_delivery("unknown", b"{}"))
(B[m[32m+            .await
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, ConnectorError::Schema(_)));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:490:
             auth_strategy: AuthStrategy::OAuth2 {
                 scopes: vec!["repo".into(), "user".into()],
             },
[31m-            sync_mode: SyncMode::Polling { cadence_seconds: 300 },
(B[m[32m+            sync_mode: SyncMode::Polling {
(B[m[32m+                cadence_seconds: 300,
(B[m[32m+            },
(B[m             capabilities: vec![ConnectorCapability {
                 name: "issues".into(),
                 params_schema: serde_json::json!({"type": "object"}),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:545:
             version: "0.1.0".into(),
             display_name: "Readwise".into(),
             auth_strategy: AuthStrategy::ApiKey,
[31m-            sync_mode: SyncMode::Polling { cadence_seconds: 3600 },
(B[m[32m+            sync_mode: SyncMode::Polling {
(B[m[32m+                cadence_seconds: 3600,
(B[m[32m+            },
(B[m             capabilities: vec![],
             entity_types: vec!["highlight".into()],
             event_types: vec!["HighlightAdded".into()],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:566:
                 version: "0.1.0".into(),
                 display_name: "Test".into(),
                 auth_strategy: AuthStrategy::ApiKey,
[31m-                sync_mode: SyncMode::Polling { cadence_seconds: 60 },
(B[m[32m+                sync_mode: SyncMode::Polling {
(B[m[32m+                    cadence_seconds: 60,
(B[m[32m+                },
(B[m                 capabilities: vec![],
                 entity_types: vec![],
                 event_types: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/lib.rs:617:
         assert!(true);
     }
 }
[31m-
(B[m 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/mcp_bridge.rs:35:
         mcp_endpoint: impl Into<String>,
         event_map: HashMap<String, String>,
     ) -> Self {
[31m-        Self { manifest, mcp_endpoint: mcp_endpoint.into(), event_map }
(B[m[32m+        Self {
(B[m[32m+            manifest,
(B[m[32m+            mcp_endpoint: mcp_endpoint.into(),
(B[m[32m+            event_map,
(B[m[32m+        }
(B[m     }
 
     pub fn mcp_endpoint(&self) -> &str {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/mcp_bridge.rs:67:
     }
 
     async fn sync(&self, _cursor: Option<String>) -> Result<SyncOutcome> {
[31m-        Err(ConnectorError::Network("MCP bridge not yet wired to MCP client".into()))
(B[m[32m+        Err(ConnectorError::Network(
(B[m[32m+            "MCP bridge not yet wired to MCP client".into(),
(B[m[32m+        ))
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/mcp_bridge.rs:83:
             version: "0.1.0".into(),
             display_name: "Notes via MCP".into(),
             auth_strategy: AuthStrategy::None,
[31m-            sync_mode: SyncMode::Polling { cadence_seconds: 300 },
(B[m[32m+            sync_mode: SyncMode::Polling {
(B[m[32m+                cadence_seconds: 300,
(B[m[32m+            },
(B[m             capabilities: vec![],
             entity_types: vec!["note".into()],
             event_types: vec!["task_added".into()],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/mcp_bridge.rs:142:
     async fn sync_returns_not_wired_error() {
         let c =
             MCPBridgedConnector::new(mk_manifest(), "stdio:/usr/local/bin/my-mcp", HashMap::new());
[31m-        let err = c.sync(None).await.expect_err("sync should error until MCP wired");
(B[m[32m+        let err = c
(B[m[32m+            .sync(None)
(B[m[32m+            .await
(B[m[32m+            .expect_err("sync should error until MCP wired");
(B[m         match err {
             ConnectorError::Network(msg) => assert!(msg.contains("MCP")),
             other => panic!("expected Network error, got {other:?}"),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:89:
     use_: String,
     #[serde(rename = "use")]
     use_field: Option<String>,
[31m-    n: Option<String>, // RSA modulus
(B[m[31m-    e: Option<String>, // RSA exponent
(B[m[32m+    n: Option<String>,        // RSA modulus
(B[m[32m+    e: Option<String>,        // RSA exponent
(B[m     x5c: Option<Vec<String>>, // X.509 cert chain
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:171:
             .ok_or_else(|| anyhow!("missing x-canvas-lti-jwt header"))?;
 
         // Decode JWT header to extract `kid`
[31m-        let header = jsonwebtoken::decode_header(jwt)
(B[m[31m-            .map_err(|e| anyhow!("invalid jwt header: {}", e))?;
(B[m[32m+        let header =
(B[m[32m+            jsonwebtoken::decode_header(jwt).map_err(|e| anyhow!("invalid jwt header: {}", e))?;
(B[m         let kid = header
             .kid
             .ok_or_else(|| anyhow!("missing kid in jwt header"))?;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:220:
         }
         if let Some(ref expected_iss) = self.expected_iss {
             if claims_json.iss != *expected_iss {
[31m-                return Err(anyhow!("iss mismatch: expected {}, got {}", expected_iss, claims_json.iss));
(B[m[32m+                return Err(anyhow!(
(B[m[32m+                    "iss mismatch: expected {}, got {}",
(B[m[32m+                    expected_iss,
(B[m[32m+                    claims_json.iss
(B[m[32m+                ));
(B[m             }
         }
         if let Some(ref expected_aud) = self.expected_aud {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:227:
             if claims_json.aud != *expected_aud {
[31m-                return Err(anyhow!("aud mismatch: expected {}, got {}", expected_aud, claims_json.aud));
(B[m[32m+                return Err(anyhow!(
(B[m[32m+                    "aud mismatch: expected {}, got {}",
(B[m[32m+                    expected_aud,
(B[m[32m+                    claims_json.aud
(B[m[32m+                ));
(B[m             }
         }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:329:
 
     #[tokio::test]
     async fn test_canvas_lti_missing_header() {
[31m-        let verifier = CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m[32m+        let verifier =
(B[m[32m+            CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m         let headers = HashMap::new();
         assert!(verifier.verify(&headers, b"").await.is_err());
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:336:
 
     #[tokio::test]
     async fn test_canvas_lti_invalid_jwt_format() {
[31m-        let verifier = CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m[32m+        let verifier =
(B[m[32m+            CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m         let mut headers = HashMap::new();
[31m-        headers.insert("x-canvas-lti-jwt".to_string(), "not.valid.jwt.parts".to_string());
(B[m[32m+        headers.insert(
(B[m[32m+            "x-canvas-lti-jwt".to_string(),
(B[m[32m+            "not.valid.jwt.parts".to_string(),
(B[m[32m+        );
(B[m         // Should fail because header decode will fail (missing kid or invalid encoding)
         let result = verifier.verify(&headers, b"").await;
         assert!(result.is_err());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:347:
     #[tokio::test]
     async fn test_canvas_lti_expired_jwt() {
         // Create an expired JWT (iat=0, exp=1, now >> 1)
[31m-        let verifier = CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m[32m+        let verifier =
(B[m[32m+            CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m 
         // Manually craft an expired JWT: header.payload.signature
         // header: {"alg":"HS256","typ":"JWT"}
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:364:
     #[tokio::test]
     async fn test_canvas_lti_future_issued_jwt() {
         // Create a JWT issued in the future
[31m-        let verifier = CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m[32m+        let verifier =
(B[m[32m+            CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string());
(B[m 
         // Future-issued JWT
         let future_iat = chrono::Utc::now().timestamp() + 3600; // +1 hour
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:375:
             "iat": future_iat,
             "exp": exp
         });
[31m-        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
(B[m[31m-            .encode(claims.to_string().as_bytes());
(B[m[32m+        let payload =
(B[m[32m+            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(claims.to_string().as_bytes());
(B[m 
         let jwt = format!("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.{}.fake", payload);
         let mut headers = HashMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:389:
 
     #[tokio::test]
     async fn test_canvas_lti_iss_mismatch() {
[31m-        let verifier = CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string())
(B[m[31m-            .with_iss("expected.issuer.com".to_string());
(B[m[32m+        let verifier =
(B[m[32m+            CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string())
(B[m[32m+                .with_iss("expected.issuer.com".to_string());
(B[m 
         let now = chrono::Utc::now().timestamp();
         let claims = serde_json::json!({
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:399:
             "iat": now,
             "exp": now + 3600
         });
[31m-        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
(B[m[31m-            .encode(claims.to_string().as_bytes());
(B[m[32m+        let payload =
(B[m[32m+            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(claims.to_string().as_bytes());
(B[m 
         let jwt = format!("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.{}.fake", payload);
         let mut headers = HashMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:412:
 
     #[tokio::test]
     async fn test_canvas_lti_aud_mismatch() {
[31m-        let verifier = CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string())
(B[m[31m-            .with_aud("https://expected.aud".to_string());
(B[m[32m+        let verifier =
(B[m[32m+            CanvasLtiVerifier::new("https://canvas.example.com/.well-known/jwks.json".to_string())
(B[m[32m+                .with_aud("https://expected.aud".to_string());
(B[m 
         let now = chrono::Utc::now().timestamp();
         let claims = serde_json::json!({
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:422:
             "iat": now,
             "exp": now + 3600
         });
[31m-        let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
(B[m[31m-            .encode(claims.to_string().as_bytes());
(B[m[32m+        let payload =
(B[m[32m+            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(claims.to_string().as_bytes());
(B[m 
         let jwt = format!("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.{}.fake", payload);
         let mut headers = HashMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:441:
         };
 
         let mut headers = HashMap::new();
[31m-        headers.insert("x-goog-channel-token".to_string(), "channel-secret-123".to_string());
(B[m[32m+        headers.insert(
(B[m[32m+            "x-goog-channel-token".to_string(),
(B[m[32m+            "channel-secret-123".to_string(),
(B[m[32m+        );
(B[m 
         assert!(verifier.verify(&headers, b"").await.is_ok());
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors/src/signature_verifiers.rs:454:
         };
 
         let mut headers = HashMap::new();
[31m-        headers.insert("x-goog-channel-token".to_string(), "wrong-secret".to_string());
(B[m[32m+        headers.insert(
(B[m[32m+            "x-goog-channel-token".to_string(),
(B[m[32m+            "wrong-secret".to_string(),
(B[m[32m+        );
(B[m 
         assert!(verifier.verify(&headers, b"").await.is_err());
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/lib.rs:116:
 
     /// Skip to a specific timestamp (for demo choreography).
     pub fn advance_to(&self, target: DateTime<Utc>) {
[31m-        if let Some(ds) = self.time_source.as_any().downcast_ref::<DeterministicTimeSource>() {
(B[m[32m+        if let Some(ds) = self
(B[m[32m+            .time_source
(B[m[32m+            .as_any()
(B[m[32m+            .downcast_ref::<DeterministicTimeSource>()
(B[m[32m+        {
(B[m             ds.set_now(target);
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/lib.rs:123:
 
[31m-    fn make_event(
(B[m[31m-        &self,
(B[m[31m-        kind: SyntheticEventKind,
(B[m[31m-        now: DateTime<Utc>,
(B[m[31m-    ) -> NormalizedEvent {
(B[m[32m+    fn make_event(&self, kind: SyntheticEventKind, now: DateTime<Utc>) -> NormalizedEvent {
(B[m         let event_type = kind.to_event_type();
         let dedupe_key = DedupeKey(format!(
             "mock-familycontrols:{}:{}",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/lib.rs:212:
         assert_eq!(manifest.id, "mock-familycontrols");
         assert_eq!(manifest.display_name, "Mock FamilyControls (POC)");
         assert_eq!(manifest.tier, VerificationTier::Private);
[31m-        assert!(manifest.event_types.contains(&"AppLaunchAttempt".to_string()));
(B[m[32m+        assert!(manifest
(B[m[32m+            .event_types
(B[m[32m+            .contains(&"AppLaunchAttempt".to_string()));
(B[m     }
 
     // Traces to: FR-MOCK-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/lib.rs:230:
     #[tokio::test]
     async fn sync_generates_events_from_schedule() {
         let conn = MockFamilyControls::new();
[31m-        conn.load_scenario("standard_day").expect("failed to load scenario");
(B[m[32m+        conn.load_scenario("standard_day")
(B[m[32m+            .expect("failed to load scenario");
(B[m 
         let outcome = conn.sync(None).await.expect("sync failed");
         assert!(!outcome.events.is_empty());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/lib.rs:270:
 
         let outcome = conn.sync(None).await.expect("sync failed");
         assert_eq!(outcome.events.len(), 1);
[31m-        assert_eq!(outcome.events[0].event_type, EventType::Custom("emergency_exit".to_string()));
(B[m[32m+        assert_eq!(
(B[m[32m+            outcome.events[0].event_type,
(B[m[32m+            EventType::Custom("emergency_exit".to_string())
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-MOCK-002
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/lib.rs:284:
 
         let outcome = conn.sync(None).await.expect("sync failed");
         let event = &outcome.events[0];
[31m-        assert_eq!(event.event_type, EventType::Custom("intervention_triggered".to_string()));
(B[m[32m+        assert_eq!(
(B[m[32m+            event.event_type,
(B[m[32m+            EventType::Custom("intervention_triggered".to_string())
(B[m[32m+        );
(B[m         assert!(event.payload.get("rule_id").is_some());
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:12:
 #[derive(Debug, Clone, PartialEq, Eq)]
 pub enum SyntheticEventKind {
     /// App launch attempt (may be blocked or allowed).
[31m-    AppLaunch {
(B[m[31m-        bundle_id: String,
(B[m[31m-        app_name: String,
(B[m[31m-    },
(B[m[32m+    AppLaunch { bundle_id: String, app_name: String },
(B[m 
     /// Screen-time accumulation over a time window.
     ScreenTimeAccumulation {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:31:
 
     /// Emergency exit (override, biometric/passcode unlock).
     /// Traces to: FR-MOCK-004.
[31m-    EmergencyExit {
(B[m[31m-        reason: String,
(B[m[31m-        auth_method: String,
(B[m[31m-    },
(B[m[32m+    EmergencyExit { reason: String, auth_method: String },
(B[m 
     /// Intervention cleared (e.g., rule expired, manual reset).
[31m-    InterventionCleared {
(B[m[31m-        rule_id: String,
(B[m[31m-        reason: String,
(B[m[31m-    },
(B[m[32m+    InterventionCleared { rule_id: String, reason: String },
(B[m }
 
 impl SyntheticEventKind {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-connectors-mock-familycontrols/src/synthetic_events.rs:60:
 
     pub fn to_payload(&self) -> Value {
         match self {
[31m-            SyntheticEventKind::AppLaunch { bundle_id, app_name } => {
(B[m[32m+            SyntheticEventKind::AppLaunch {
(B[m[32m+                bundle_id,
(B[m[32m+                app_name,
(B[m[32m+            } => {
(B[m                 json!({
                     "bundle_id": bundle_id,
                     "app_name": app_name,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:49:
     }
 
     fn load(&self, key: &str) -> anyhow::Result<Option<SecretString>> {
[31m-        let guard = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned mutex: {e}"))?;
(B[m[32m+        let guard = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned mutex: {e}"))?;
(B[m         Ok(guard.get(key).map(|v| SecretString::from(v.clone())))
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:56:
     fn delete(&self, key: &str) -> anyhow::Result<()> {
[31m-        self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned mutex: {e}"))?.remove(key);
(B[m[32m+        self.inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned mutex: {e}"))?
(B[m[32m+            .remove(key);
(B[m         Ok(())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:112:
 
     impl AppleKeychainStore {
         pub fn new(service_name: impl Into<String>) -> Self {
[31m-            Self { service_name: service_name.into() }
(B[m[32m+            Self {
(B[m[32m+                service_name: service_name.into(),
(B[m[32m+            }
(B[m         }
 
         pub fn service_name(&self) -> &str {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:179:
 
     impl LinuxSecretServiceStore {
         pub fn new(service_name: impl Into<String>) -> Self {
[31m-            Self { service_name: service_name.into() }
(B[m[32m+            Self {
(B[m[32m+                service_name: service_name.into(),
(B[m[32m+            }
(B[m         }
 
         fn attrs<'a>(&'a self, key: &'a str) -> HashMap<&'a str, &'a str> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:219:
             let Some(item) = found else {
                 return Ok(None);
             };
[31m-            let secret =
(B[m[31m-                item.get_secret().map_err(|e| anyhow::anyhow!("secret-service get_secret: {e}"))?;
(B[m[32m+            let secret = item
(B[m[32m+                .get_secret()
(B[m[32m+                .map_err(|e| anyhow::anyhow!("secret-service get_secret: {e}"))?;
(B[m             let s = String::from_utf8(secret)
                 .map_err(|e| anyhow::anyhow!("secret-service value not utf8: {e}"))?;
             Ok(Some(SecretString::from(s)))
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:233:
                 .search_items(self.attrs(key))
                 .map_err(|e| anyhow::anyhow!("secret-service search: {e}"))?;
             for item in items.unlocked.iter().chain(items.locked.iter()) {
[31m-                item.delete().map_err(|e| anyhow::anyhow!("secret-service delete: {e}"))?;
(B[m[32m+                item.delete()
(B[m[32m+                    .map_err(|e| anyhow::anyhow!("secret-service delete: {e}"))?;
(B[m             }
             Ok(())
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-crypto/src/keychain.rs:343:
         assert!(store.load(&key).unwrap().is_none());
 
         store.store(&key, SecretString::from("hunter2")).unwrap();
[31m-        assert_eq!(store.load(&key).unwrap().unwrap().expose_secret(), "hunter2");
(B[m[32m+        assert_eq!(
(B[m[32m+            store.load(&key).unwrap().unwrap().expose_secret(),
(B[m[32m+            "hunter2"
(B[m[32m+        );
(B[m 
[31m-        store.store(&key, SecretString::from("correcthorse")).unwrap();
(B[m[31m-        assert_eq!(store.load(&key).unwrap().unwrap().expose_secret(), "correcthorse");
(B[m[32m+        store
(B[m[32m+            .store(&key, SecretString::from("correcthorse"))
(B[m[32m+            .unwrap();
(B[m[32m+        assert_eq!(
(B[m[32m+            store.load(&key).unwrap().unwrap().expose_secret(),
(B[m[32m+            "correcthorse"
(B[m[32m+        );
(B[m 
         store.delete(&key).unwrap();
         assert!(store.load(&key).unwrap().is_none());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:21:
 use focus_domain::Rigidity;
 use focus_planning::{Deadline, DurationSpec, Priority, Task, TaskStatus, TaskStore};
 use focus_rules::{Action, Rule, Trigger};
[31m-use focus_storage::SqliteAdapter;
(B[m use focus_storage::sqlite::audit_store::SqliteAuditStore;
 use focus_storage::sqlite::rule_store::upsert_rule;
 use focus_storage::sqlite::task_store::SqliteTaskStore;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:28:
[32m+use focus_storage::SqliteAdapter;
(B[m use serde::{Deserialize, Serialize};
 use serde_json::json;
 use uuid::Uuid;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:98:
     let audit_store = SqliteAuditStore::from_adapter(adapter);
 
     // Load all audit records to find demo entities
[31m-    let records = audit_store.load_all().await
(B[m[32m+    let records = audit_store
(B[m[32m+        .load_all()
(B[m[32m+        .await
(B[m         .context("load audit records for reset")?;
 
     // Extract task IDs and rule IDs created by demo
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:136:
         "source": "demo",
         "timestamp": Utc::now().to_rfc3339(),
     });
[31m-    focus_audit::append_mutation(
(B[m[31m-        &audit_store,
(B[m[31m-        "demo.reset",
(B[m[31m-        "system",
(B[m[31m-        &payload,
(B[m[31m-        Utc::now(),
(B[m[31m-    ).context("append reset completion record")?;
(B[m[32m+    focus_audit::append_mutation(&audit_store, "demo.reset", "system", &payload, Utc::now())
(B[m[32m+        .context("append reset completion record")?;
(B[m 
     tracing::info!("reset_demo_data: cleared all demo records from database");
     Ok(())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:189:
             updated_at: now,
         };
 
[31m-        task_store.upsert(user_id, &task)
(B[m[32m+        task_store
(B[m[32m+            .upsert(user_id, &task)
(B[m             .context(format!("seed task: {}", title))?;
         tracing::debug!("seeded task: {} (id={})", title, task_id);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:210:
     let count = rule_examples.len();
     for (rule_id, name, priority) in rule_examples {
         let rule = Rule {
[31m-            id: Uuid::parse_str(rule_id)
(B[m[31m-                .unwrap_or_else(|_| {
(B[m[31m-                    // Fallback: generate deterministic UUID from rule_id string
(B[m[31m-                    let mut bytes = [0u8; 16];
(B[m[31m-                    for (i, b) in rule_id.as_bytes().iter().enumerate().take(16) {
(B[m[31m-                        bytes[i] = *b;
(B[m[31m-                    }
(B[m[31m-                    Uuid::from_bytes(bytes)
(B[m[31m-                }),
(B[m[32m+            id: Uuid::parse_str(rule_id).unwrap_or_else(|_| {
(B[m[32m+                // Fallback: generate deterministic UUID from rule_id string
(B[m[32m+                let mut bytes = [0u8; 16];
(B[m[32m+                for (i, b) in rule_id.as_bytes().iter().enumerate().take(16) {
(B[m[32m+                    bytes[i] = *b;
(B[m[32m+                }
(B[m[32m+                Uuid::from_bytes(bytes)
(B[m[32m+            }),
(B[m             name: name.to_string(),
             trigger: Trigger::Event(rule_id.to_string()),
             conditions: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:230:
             enabled: true,
         };
 
[31m-        upsert_rule(adapter, rule).await
(B[m[32m+        upsert_rule(adapter, rule)
(B[m[32m+            .await
(B[m             .context(format!("seed rule: {}", name))?;
         tracing::debug!("seeded rule: {} (id={})", name, rule_id);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:275:
             });
 
             // Append to the continuous chain
[31m-            let record = chain.append(
(B[m[31m-                "wallet.grant",
(B[m[31m-                user_id.to_string(),
(B[m[31m-                payload,
(B[m[31m-                ts,
(B[m[31m-            );
(B[m[32m+            let record = chain.append("wallet.grant", user_id.to_string(), payload, ts);
(B[m 
             // Append to store
[31m-            audit_store.append(record)
(B[m[32m+            audit_store
(B[m[32m+                .append(record)
(B[m                 .context(format!("append wallet grant audit on day {}", day_offset))?;
             audit_count += 1;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:290:
[31m-            tracing::debug!("audit: wallet_grant amount={} on day_offset={}", amount, day_offset);
(B[m[32m+            tracing::debug!(
(B[m[32m+                "audit: wallet_grant amount={} on day_offset={}",
(B[m[32m+                amount,
(B[m[32m+                day_offset
(B[m[32m+            );
(B[m         }
 
         // Session start/complete (1 per day minimum)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:297:
             "duration_minutes": 45,
             "source": "demo",
         });
[31m-        let record = chain.append(
(B[m[31m-            "session.complete",
(B[m[31m-            user_id.to_string(),
(B[m[31m-            payload,
(B[m[31m-            ts,
(B[m[31m-        );
(B[m[31m-        audit_store.append(record)
(B[m[31m-            .context(format!("append session complete audit on day {}", day_offset))?;
(B[m[32m+        let record = chain.append("session.complete", user_id.to_string(), payload, ts);
(B[m[32m+        audit_store.append(record).context(format!(
(B[m[32m+            "append session complete audit on day {}",
(B[m[32m+            day_offset
(B[m[32m+        ))?;
(B[m         audit_count += 1;
 
         // Rule fire (varies by day)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:316:
                 "action": "grant_credit",
                 "source": "demo",
             });
[31m-            let record = chain.append(
(B[m[31m-                "rule.fired",
(B[m[31m-                user_id.to_string(),
(B[m[31m-                payload,
(B[m[31m-                ts,
(B[m[31m-            );
(B[m[31m-            audit_store.append(record)
(B[m[32m+            let record = chain.append("rule.fired", user_id.to_string(), payload, ts);
(B[m[32m+            audit_store
(B[m[32m+                .append(record)
(B[m                 .context(format!("append rule fired audit on day {}", day_offset))?;
             audit_count += 1;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:409:
         let adapter = SqliteAdapter::open_in_memory()?;
         let user_id = Uuid::nil();
         let count = seed_demo_rituals(&adapter, user_id).await?;
[31m-        assert_eq!(count, 14, "should seed 14 ritual completions (7 days × 2 rituals)");
(B[m[32m+        assert_eq!(
(B[m[32m+            count, 14,
(B[m[32m+            "should seed 14 ritual completions (7 days × 2 rituals)"
(B[m[32m+        );
(B[m         Ok(())
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:418:
         let adapter = SqliteAdapter::open_in_memory()?;
         let user_id = Uuid::nil();
         let (balance, streak, audit_count) = seed_demo_wallet_and_audit(&adapter, user_id).await?;
[31m-        assert!(balance > 0, "wallet should have credits after audit mutations");
(B[m[32m+        assert!(
(B[m[32m+            balance > 0,
(B[m[32m+            "wallet should have credits after audit mutations"
(B[m[32m+        );
(B[m         assert_eq!(streak, 7, "wallet should have 7-day streak");
[31m-        assert!(audit_count >= 20, "should have ~30 audit records, got {}", audit_count);
(B[m[32m+        assert!(
(B[m[32m+            audit_count >= 20,
(B[m[32m+            "should have ~30 audit records, got {}",
(B[m[32m+            audit_count
(B[m[32m+        );
(B[m 
         // Verify audit records persisted
         let audit_store = SqliteAuditStore::from_adapter(&adapter);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:427:
         let all_records = audit_store.load_all().await?;
[31m-        assert!(all_records.len() >= 20, "all audit records should persist in DB");
(B[m[32m+        assert!(
(B[m[32m+            all_records.len() >= 20,
(B[m[32m+            "all audit records should persist in DB"
(B[m[32m+        );
(B[m         Ok(())
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:436:
 
         assert_eq!(report.tasks_count, 10, "should seed exactly 10 tasks");
         assert_eq!(report.rules_count, 5, "should seed exactly 5 rules");
[31m-        assert_eq!(report.connectors_connected, 3, "should connect exactly 3 connectors");
(B[m[32m+        assert_eq!(
(B[m[32m+            report.connectors_connected, 3,
(B[m[32m+            "should connect exactly 3 connectors"
(B[m[32m+        );
(B[m         assert!(report.wallet_balance > 0, "wallet should have credits");
[31m-        assert_eq!(report.wallet_streak_days, 7, "wallet should have 7-day streak");
(B[m[31m-        assert_eq!(report.ritual_completions_count, 14, "should seed 14 ritual completions (7 days × 2 rituals)");
(B[m[31m-        assert!(report.audit_records_count >= 20, "should have ~30+ audit records, got {}", report.audit_records_count);
(B[m[32m+        assert_eq!(
(B[m[32m+            report.wallet_streak_days, 7,
(B[m[32m+            "wallet should have 7-day streak"
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            report.ritual_completions_count, 14,
(B[m[32m+            "should seed 14 ritual completions (7 days × 2 rituals)"
(B[m[32m+        );
(B[m[32m+        assert!(
(B[m[32m+            report.audit_records_count >= 20,
(B[m[32m+            "should have ~30+ audit records, got {}",
(B[m[32m+            report.audit_records_count
(B[m[32m+        );
(B[m 
         // Verify data persistence: check SQLite
         let task_store = SqliteTaskStore::from_adapter(&adapter);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-demo-seed/src/lib.rs:477:
             }
         }
         // Just verify that reset completed; tasks are deleted via storage layer
[31m-        assert!(all_records.iter().any(|r| r.record_type == "demo.reset"), "reset record should exist");
(B[m[32m+        assert!(
(B[m[32m+            all_records.iter().any(|r| r.record_type == "demo.reset"),
(B[m[32m+            "reset record should exist"
(B[m[32m+        );
(B[m 
         Ok(())
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-domain/src/lib.rs:176:
             _ => panic!("expected FrictionDelay"),
         }
         let ap = Rigidity::Semi(RigidityCost::AccountabilityPing);
[31m-        assert!(matches!(ap.semi_cost(), Some(RigidityCost::AccountabilityPing)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            ap.semi_cost(),
(B[m[32m+            Some(RigidityCost::AccountabilityPing)
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-RIGIDITY-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-entitlements/src/lib.rs:15:
 mod tests;
 
 /// Subscription tier.
[31m-#[derive(
(B[m[31m-    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
(B[m[31m-)]
(B[m[32m+#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
(B[m #[serde(rename_all = "lowercase")]
 pub enum Tier {
     Free,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-entitlements/src/lib.rs:246:
 /// Free: false, Plus+: true
 /// Traces to: FR-ENTITLEMENTS-002
 pub fn can_use_live_activity(entitlement: &Entitlement) -> bool {
[31m-    matches!(
(B[m[31m-        entitlement.tier,
(B[m[31m-        Tier::Plus | Tier::Pro | Tier::Family
(B[m[31m-    )
(B[m[32m+    matches!(entitlement.tier, Tier::Plus | Tier::Pro | Tier::Family)
(B[m }
 
 /// Check if HomeKit widget is available.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-entitlements/src/lib.rs:256:
 /// Free: false, Plus+: true
 /// Traces to: FR-ENTITLEMENTS-002
 pub fn can_use_homekit_widget(entitlement: &Entitlement) -> bool {
[31m-    matches!(
(B[m[31m-        entitlement.tier,
(B[m[31m-        Tier::Plus | Tier::Pro | Tier::Family
(B[m[31m-    )
(B[m[32m+    matches!(entitlement.tier, Tier::Plus | Tier::Pro | Tier::Family)
(B[m }
 
 /// Get audit retention in days.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-entitlements/src/lib.rs:278:
 /// Free: false, Plus+: true
 /// Traces to: FR-ENTITLEMENTS-002
 pub fn can_use_cloudkit_sync(entitlement: &Entitlement) -> bool {
[31m-    matches!(
(B[m[31m-        entitlement.tier,
(B[m[31m-        Tier::Plus | Tier::Pro | Tier::Family
(B[m[31m-    )
(B[m[32m+    matches!(entitlement.tier, Tier::Plus | Tier::Pro | Tier::Family)
(B[m }
 
 /// Get daily nudge limit.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-entitlements/src/lib.rs:347:
 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub enum SupportPriority {
     Community,
[31m-    Standard,  // 48h
(B[m[31m-    Priority,  // 24h
(B[m[32m+    Standard, // 48h
(B[m[32m+    Priority, // 24h
(B[m }
 
 pub fn support_priority(entitlement: &Entitlement) -> SupportPriority {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-errors/src/lib.rs:4:
 //! `focus-*` crate naming convention, plus domain-specific convenience methods
 //! that focus crates frequently need.
 
[31m-pub use phenotype_error_core::{
(B[m[31m-    PhenotypeError, Result,
(B[m[31m-};
(B[m[32m+pub use phenotype_error_core::{PhenotypeError, Result};
(B[m 
 /// Alias for [`PhenotypeError`] within the `focus-*` crate namespace.
 pub type FocusError = PhenotypeError;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-errors/src/lib.rs:19:
     /// Create a crypto-related error.
     fn crypto(message: impl Into<String>) -> Self;
     /// Create a transpilation-related error.
[31m-    fn transpilation(source: impl Into<String>, target: impl Into<String>, message: impl Into<String>) -> Self;
(B[m[32m+    fn transpilation(
(B[m[32m+        source: impl Into<String>,
(B[m[32m+        target: impl Into<String>,
(B[m[32m+        message: impl Into<String>,
(B[m[32m+    ) -> Self;
(B[m     /// Create an event-related error.
     fn event(message: impl Into<String>) -> Self;
     /// Create a connector-related error.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-errors/src/lib.rs:35:
         }
     }
 
[31m-    fn transpilation(source: impl Into<String>, target: impl Into<String>, message: impl Into<String>) -> Self {
(B[m[32m+    fn transpilation(
(B[m[32m+        source: impl Into<String>,
(B[m[32m+        target: impl Into<String>,
(B[m[32m+        message: impl Into<String>,
(B[m[32m+    ) -> Self {
(B[m         Self::Internal {
[31m-            message: format!("transpilation {} -> {}: {}", source.into(), target.into(), message.into()),
(B[m[32m+            message: format!(
(B[m[32m+                "transpilation {} -> {}: {}",
(B[m[32m+                source.into(),
(B[m[32m+                target.into(),
(B[m[32m+                message.into()
(B[m[32m+            ),
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-errors/src/lib.rs:79:
     #[test]
     fn test_transpilation_error() {
         let err = FocusError::transpilation("toml", "json", "missing field");
[31m-        assert!(err.to_string().contains("transpilation toml -> json: missing field"));
(B[m[32m+        assert!(err
(B[m[32m+            .to_string()
(B[m[32m+            .contains("transpilation toml -> json: missing field"));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/benches/eval_batched.rs:1:
 use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
[31m-use focus_events::{NormalizedEvent, WellKnownEventType, DedupeKey, EventType};
(B[m[32m+use focus_events::{DedupeKey, EventType, NormalizedEvent, WellKnownEventType};
(B[m use focus_rules::{Action, Rule, Trigger};
 use uuid::Uuid;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/benches/eval_tick.rs:1:
 use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
[31m-use focus_events::{NormalizedEvent, WellKnownEventType, DedupeKey, EventType};
(B[m[32m+use focus_events::{DedupeKey, EventType, NormalizedEvent, WellKnownEventType};
(B[m use focus_rules::{Action, Rule, Trigger};
 use uuid::Uuid;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:23:
 use tracing::{debug, warn};
 use uuid::Uuid;
 
[31m-use crate::{
(B[m[31m-    DecisionSink, EvaluationReport, RULE_EVAL_CONNECTOR_ID, RULE_EVAL_ENTITY_TYPE,
(B[m[31m-};
(B[m[32m+use crate::{DecisionSink, EvaluationReport, RULE_EVAL_CONNECTOR_ID, RULE_EVAL_ENTITY_TYPE};
(B[m 
 /// Parallelism threshold: only use rayon when event count > 50 AND rule count > 10.
 const PARALLELISM_EVENT_THRESHOLD: usize = 50;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:107:
 
         let raw = self.event_store.since_cursor(None, self.batch_size).await?;
         let events: Vec<NormalizedEvent> = match cursor.as_deref() {
[31m-            Some(c) => {
(B[m[31m-                raw.into_iter()
(B[m[31m-                    .filter(|e| e.occurred_at.to_rfc3339().as_str() > c)
(B[m[31m-                    .collect()
(B[m[31m-            }
(B[m[32m+            Some(c) => raw
(B[m[32m+                .into_iter()
(B[m[32m+                .filter(|e| e.occurred_at.to_rfc3339().as_str() > c)
(B[m[32m+                .collect(),
(B[m             None => raw,
         };
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:142:
         }
 
         // Decide whether to parallelize based on input size.
[31m-        let use_parallel = events.len() > PARALLELISM_EVENT_THRESHOLD
(B[m[31m-            && rules.len() > PARALLELISM_RULE_THRESHOLD;
(B[m[32m+        let use_parallel =
(B[m[32m+            events.len() > PARALLELISM_EVENT_THRESHOLD && rules.len() > PARALLELISM_RULE_THRESHOLD;
(B[m 
         // Evaluate all events against all rules (sequential or parallel).
         let results = if use_parallel {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:168:
                         self.decision_sink.record(decision);
                     }
                     RuleDecision::Suppressed { .. } => {
[31m-                        report.decisions_suppressed =
(B[m[31m-                            report.decisions_suppressed.saturating_add(1);
(B[m[32m+                        report.decisions_suppressed = report.decisions_suppressed.saturating_add(1);
(B[m                     }
                     RuleDecision::Skipped { .. } => {
                         report.decisions_skipped = report.decisions_skipped.saturating_add(1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:290:
                     }
                 }
                 Action::Block { .. } | Action::Unblock { .. } => {
[31m-                    debug!(?action, "policy-affecting action — stashed in decision sink");
(B[m[32m+                    debug!(
(B[m[32m+                        ?action,
(B[m[32m+                        "policy-affecting action — stashed in decision sink"
(B[m[32m+                    );
(B[m                 }
                 Action::Notify(message) => {
                     let payload = json!({
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:415:
             "actions_count": actions.len(),
             "priority": decision.priority,
         });
[31m-        if let Err(e) = self.audit.record_mutation(
(B[m[31m-            "rule.fired",
(B[m[31m-            &self.user_id.to_string(),
(B[m[31m-            payload,
(B[m[31m-            now,
(B[m[31m-        ) {
(B[m[32m+        if let Err(e) =
(B[m[32m+            self.audit
(B[m[32m+                .record_mutation("rule.fired", &self.user_id.to_string(), payload, now)
(B[m[32m+        {
(B[m             warn!(error = %e, "rule.fired audit append failed");
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:429:
 #[cfg(test)]
 mod tests {
     use super::*;
[32m+    use crate::{
(B[m[32m+        InMemoryEventStore, InMemoryPenaltyStore, InMemoryRuleStore, InMemoryWalletStore,
(B[m[32m+        NoopDecisionSink,
(B[m[32m+    };
(B[m     use focus_audit::CapturingAuditSink;
     use focus_rules::Trigger;
[31m-    use crate::{NoopDecisionSink, InMemoryEventStore, InMemoryPenaltyStore, InMemoryRuleStore, InMemoryWalletStore};
(B[m     use focus_sync::InMemoryCursorStore;
 
     fn mk_event(id: usize) -> NormalizedEvent {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:465:
                 name: format!("rule_{}", i),
                 trigger: Trigger::Event("AppFocus".into()),
                 conditions: vec![],
[31m-                actions: vec![Action::GrantCredit {
(B[m[31m-                    amount: 1,
(B[m[31m-                }],
(B[m[32m+                actions: vec![Action::GrantCredit { amount: 1 }],
(B[m                 priority: i as i32,
                 cooldown: None,
                 duration: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:525:
             name: "determinism_rule".into(),
             trigger: Trigger::Event("AppFocus".into()),
             conditions: vec![],
[31m-            actions: vec![Action::GrantCredit {
(B[m[31m-                amount: 10,
(B[m[31m-            }],
(B[m[32m+            actions: vec![Action::GrantCredit { amount: 10 }],
(B[m             priority: 0,
             cooldown: None,
             duration: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/batched.rs:580:
             name: "zero_batch_rule".into(),
             trigger: Trigger::Event("AppFocus".into()),
             conditions: vec![],
[31m-            actions: vec![Action::GrantCredit {
(B[m[31m-                amount: 5,
(B[m[31m-            }],
(B[m[32m+            actions: vec![Action::GrantCredit { amount: 5 }],
(B[m             priority: 0,
             cooldown: None,
             duration: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:169:
         // highest `occurred_at` ISO string we've seen is the resume point.
         let raw = self.event_store.since_cursor(None, self.batch_size).await?;
         let events: Vec<NormalizedEvent> = match cursor.as_deref() {
[31m-            Some(c) => {
(B[m[31m-                raw.into_iter().filter(|e| e.occurred_at.to_rfc3339().as_str() > c).collect()
(B[m[31m-            }
(B[m[32m+            Some(c) => raw
(B[m[32m+                .into_iter()
(B[m[32m+                .filter(|e| e.occurred_at.to_rfc3339().as_str() > c)
(B[m[32m+                .collect(),
(B[m             None => raw,
         };
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:213:
                             "rule.evaluate span (fired)"
                         );
 
[31m-                        if let Err(e) = self.dispatch_actions(actions, &decision, event, now).await {
(B[m[32m+                        if let Err(e) = self.dispatch_actions(actions, &decision, event, now).await
(B[m[32m+                        {
(B[m                             warn!(error = %e, "dispatch_actions failed");
                         }
                         self.audit_fired(&decision, event, actions, now);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:315:
                     // Policy-side: the decision itself is recorded by the
                     // caller into `recent_decisions`; `PolicyApi::build_
                     // from_recent_decisions` reads that buffer.
[31m-                    debug!(?action, "policy-affecting action — stashed in decision sink");
(B[m[32m+                    debug!(
(B[m[32m+                        ?action,
(B[m[32m+                        "policy-affecting action — stashed in decision sink"
(B[m[32m+                    );
(B[m                 }
                 Action::Notify(message) => {
                     // Emit a dedicated `notify.dispatched` audit line so
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:376:
                         warn!(error = %e, "notification.dispatched audit append failed");
                     }
                 }
[31m-                Action::EmergencyExit { profiles, duration, bypass_cost, reason } => {
(B[m[32m+                Action::EmergencyExit {
(B[m[32m+                    profiles,
(B[m[32m+                    duration,
(B[m[32m+                    bypass_cost,
(B[m[32m+                    reason,
(B[m[32m+                } => {
(B[m                     // Rate-limit: 1 per hour to prevent gaming
                     let now_instant = Instant::now();
                     let mut rate_limit_guard = match self.emergency_exit_rate_limit.lock() {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:445:
                         warn!(error = %e, "focus:session_completed (emergency) audit append failed");
                     }
                 }
[31m-                Action::ScheduledUnlockWindow { profile, starts_at, ends_at, credit_cost } => {
(B[m[32m+                Action::ScheduledUnlockWindow {
(B[m[32m+                    profile,
(B[m[32m+                    starts_at,
(B[m[32m+                    ends_at,
(B[m[32m+                    credit_cost,
(B[m[32m+                } => {
(B[m                     // Activate time-boxed override and record window
                     let payload = json!({
                         "rule_id": decision.rule_id.to_string(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:486:
             "explanation": format!("rule {} fired on event {}", decision.rule_id, event.event_id),
         });
         if let Err(e) =
[31m-            self.audit.record_mutation("rule.fired", &self.user_id.to_string(), payload, now)
(B[m[32m+            self.audit
(B[m[32m+                .record_mutation("rule.fired", &self.user_id.to_string(), payload, now)
(B[m         {
             warn!(error = %e, "rule.fired audit append failed");
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:531:
 impl EventStore for InMemoryEventStore {
     #[async_instrumented]
     async fn append(&self, event: NormalizedEvent) -> anyhow::Result<()> {
[31m-        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         if g.iter().any(|e| e.dedupe_key == event.dedupe_key) {
             return Ok(());
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:545:
         cursor: Option<&str>,
         limit: usize,
     ) -> anyhow::Result<Vec<NormalizedEvent>> {
[31m-        let g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         let mut out: Vec<NormalizedEvent> = g
             .iter()
             .filter(|e| match cursor {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:554:
             })
             .cloned()
             .collect();
[31m-        out.sort_by(|a, b| a.occurred_at.cmp(&b.occurred_at).then(a.event_id.cmp(&b.event_id)));
(B[m[32m+        out.sort_by(|a, b| {
(B[m[32m+            a.occurred_at
(B[m[32m+                .cmp(&b.occurred_at)
(B[m[32m+                .then(a.event_id.cmp(&b.event_id))
(B[m[32m+        });
(B[m         out.truncate(limit);
         Ok(out)
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:568:
 
 impl InMemoryRuleStore {
     pub fn new(rules: Vec<Rule>) -> Self {
[31m-        Self { inner: Arc::new(Mutex::new(rules)) }
(B[m[32m+        Self {
(B[m[32m+            inner: Arc::new(Mutex::new(rules)),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:576:
 impl RuleStore for InMemoryRuleStore {
     #[async_instrumented]
     async fn get(&self, id: Uuid) -> anyhow::Result<Option<Rule>> {
[31m-        let g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         Ok(g.iter().find(|r| r.id == id).cloned())
     }
     #[async_instrumented]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:583:
     async fn list_enabled(&self) -> anyhow::Result<Vec<Rule>> {
[31m-        let g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         Ok(g.iter().filter(|r| r.enabled).cloned().collect())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:607:
 impl WalletStore for InMemoryWalletStore {
     #[async_instrumented]
     async fn load(&self, user_id: Uuid) -> anyhow::Result<focus_rewards::RewardWallet> {
[31m-        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         g.user_id = user_id;
         Ok(g.clone())
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:614:
     #[async_instrumented]
     async fn apply(&self, user_id: Uuid, mutation: WalletMutation) -> anyhow::Result<()> {
[31m-        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         g.user_id = user_id;
         g.apply(mutation, Utc::now(), self.audit.as_ref())
             .map_err(|e| anyhow::anyhow!("wallet apply: {e}"))?;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:638:
 #[async_trait]
 impl PenaltyStore for InMemoryPenaltyStore {
     async fn load(&self, user_id: Uuid) -> anyhow::Result<focus_penalties::PenaltyState> {
[31m-        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         g.user_id = user_id;
         Ok(g.clone())
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:645:
     async fn apply(&self, user_id: Uuid, mutation: PenaltyMutation) -> anyhow::Result<()> {
[31m-        let mut g = self.inner.lock().map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?;
(B[m         g.user_id = user_id;
         g.apply(mutation, Utc::now(), self.audit.as_ref())
             .map_err(|e| anyhow::anyhow!("penalty apply: {e}"))?;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:757:
         let events = Arc::new(InMemoryEventStore::new());
         events.append(mk_event(0)).await.unwrap();
         events.append(mk_event(1)).await.unwrap();
[31m-        let rules =
(B[m[31m-            Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(5, Some(Duration::hours(1)))]));
(B[m[32m+        let rules = Arc::new(InMemoryRuleStore::new(vec![mk_rule_grant(
(B[m[32m+            5,
(B[m[32m+            Some(Duration::hours(1)),
(B[m[32m+        )]));
(B[m         let engine = Arc::new(RwLock::new(RuleEngine::new()));
         let wallet = Arc::new(InMemoryWalletStore::new());
         let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1112:
             let engine = Arc::new(RwLock::new(RuleEngine::new()));
             let wallet = Arc::new(InMemoryWalletStore::new());
             let cursor: Arc<dyn CursorStore> = Arc::new(InMemoryCursorStore::new());
[31m-            let pipeline = mk_pipeline(events.clone(), rules, engine, wallet.clone(), cursor.clone());
(B[m[32m+            let pipeline = mk_pipeline(
(B[m[32m+                events.clone(),
(B[m[32m+                rules,
(B[m[32m+                engine,
(B[m[32m+                wallet.clone(),
(B[m[32m+                cursor.clone(),
(B[m[32m+            );
(B[m 
             pipeline.tick(Utc::now()).await.unwrap();
             assert_eq!(wallet.snapshot().earned_credits, 10);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1578:
         let snap = capturing.snapshot();
 
         let kinds: Vec<&str> = snap.iter().map(|r| r.0.as_str()).collect();
[31m-        assert!(kinds.contains(&"intervention.triggered") && kinds.contains(&"notification.dispatched"));
(B[m[32m+        assert!(
(B[m[32m+            kinds.contains(&"intervention.triggered") && kinds.contains(&"notification.dispatched")
(B[m[32m+        );
(B[m 
[31m-        let intervention = snap.iter().find(|r| r.0 == "intervention.triggered").unwrap();
(B[m[31m-        assert_eq!(intervention.2.get("severity").and_then(|v| v.as_str()), Some("gentle"));
(B[m[32m+        let intervention = snap
(B[m[32m+            .iter()
(B[m[32m+            .find(|r| r.0 == "intervention.triggered")
(B[m[32m+            .unwrap();
(B[m[32m+        assert_eq!(
(B[m[32m+            intervention.2.get("severity").and_then(|v| v.as_str()),
(B[m[32m+            Some("gentle")
(B[m[32m+        );
(B[m 
[31m-        let notification = snap.iter().find(|r| r.0 == "notification.dispatched").unwrap();
(B[m[31m-        assert_eq!(notification.2.get("category").and_then(|v| v.as_str()), Some("COACHY_NUDGE"));
(B[m[32m+        let notification = snap
(B[m[32m+            .iter()
(B[m[32m+            .find(|r| r.0 == "notification.dispatched")
(B[m[32m+            .unwrap();
(B[m[32m+        assert_eq!(
(B[m[32m+            notification.2.get("category").and_then(|v| v.as_str()),
(B[m[32m+            Some("COACHY_NUDGE")
(B[m[32m+        );
(B[m     }
 
     /// Traces to: FR-ENF-004 — EmergencyExit action force-completes focus session.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1635:
         let snap = capturing.snapshot();
 
         let kinds: Vec<&str> = snap.iter().map(|r| r.0.as_str()).collect();
[31m-        assert!(kinds.contains(&"emergency.exit_triggered") && kinds.contains(&"focus:session_completed"));
(B[m[32m+        assert!(
(B[m[32m+            kinds.contains(&"emergency.exit_triggered")
(B[m[32m+                && kinds.contains(&"focus:session_completed")
(B[m[32m+        );
(B[m 
[31m-        let emergency = snap.iter().find(|r| r.0 == "emergency.exit_triggered").unwrap();
(B[m[31m-        assert_eq!(emergency.2.get("bypass_cost").and_then(|v| v.as_i64()), Some(50));
(B[m[32m+        let emergency = snap
(B[m[32m+            .iter()
(B[m[32m+            .find(|r| r.0 == "emergency.exit_triggered")
(B[m[32m+            .unwrap();
(B[m[32m+        assert_eq!(
(B[m[32m+            emergency.2.get("bypass_cost").and_then(|v| v.as_i64()),
(B[m[32m+            Some(50)
(B[m[32m+        );
(B[m     }
 
     /// Traces to: FR-ENF-005 — ScheduledUnlockWindow activates time-boxed override.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1692:
         pipeline.tick(now).await.unwrap();
         let snap = capturing.snapshot();
 
[31m-        let window_audit = snap.iter().find(|r| r.0 == "unlock_window.activated").unwrap();
(B[m[31m-        assert_eq!(window_audit.2.get("profile").and_then(|v| v.as_str()), Some("email"));
(B[m[31m-        assert_eq!(window_audit.2.get("credit_cost").and_then(|v| v.as_i64()), Some(25));
(B[m[32m+        let window_audit = snap
(B[m[32m+            .iter()
(B[m[32m+            .find(|r| r.0 == "unlock_window.activated")
(B[m[32m+            .unwrap();
(B[m[32m+        assert_eq!(
(B[m[32m+            window_audit.2.get("profile").and_then(|v| v.as_str()),
(B[m[32m+            Some("email")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            window_audit.2.get("credit_cost").and_then(|v| v.as_i64()),
(B[m[32m+            Some(25)
(B[m[32m+        );
(B[m     }
 
     /// Traces to: FR-ENF-003 — Urgent intervention maps to high priority RULE_FIRED.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1744:
         pipeline.tick(Utc::now()).await.unwrap();
         let snap = capturing.snapshot();
 
[31m-        let notification = snap.iter().find(|r| r.0 == "notification.dispatched").unwrap();
(B[m[31m-        assert_eq!(notification.2.get("category").and_then(|v| v.as_str()), Some("RULE_FIRED"));
(B[m[31m-        assert_eq!(notification.2.get("priority").and_then(|v| v.as_str()), Some("high"));
(B[m[32m+        let notification = snap
(B[m[32m+            .iter()
(B[m[32m+            .find(|r| r.0 == "notification.dispatched")
(B[m[32m+            .unwrap();
(B[m[32m+        assert_eq!(
(B[m[32m+            notification.2.get("category").and_then(|v| v.as_str()),
(B[m[32m+            Some("RULE_FIRED")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            notification.2.get("priority").and_then(|v| v.as_str()),
(B[m[32m+            Some("high")
(B[m[32m+        );
(B[m     }
 
     /// Traces to: FR-ENF-006 — EmergencyExit rate-limit (1 per hour) prevents gaming.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1798:
         // First tick: first event fires EmergencyExit
         pipeline.tick(Utc::now()).await.unwrap();
         let snap1 = capturing.snapshot();
[31m-        let first_fires = snap1.iter().filter(|r| r.0 == "emergency.exit_triggered").count();
(B[m[32m+        let first_fires = snap1
(B[m[32m+            .iter()
(B[m[32m+            .filter(|r| r.0 == "emergency.exit_triggered")
(B[m[32m+            .count();
(B[m         assert_eq!(first_fires, 1);
 
         // Second tick: second event within same hour is rate-limited
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1805:
         pipeline.tick(Utc::now()).await.unwrap();
         let snap2 = capturing.snapshot();
[31m-        let rate_limited = snap2.iter().filter(|r| r.0 == "emergency.exit_rate_limited").count();
(B[m[32m+        let rate_limited = snap2
(B[m[32m+            .iter()
(B[m[32m+            .filter(|r| r.0 == "emergency.exit_rate_limited")
(B[m[32m+            .count();
(B[m 
         // Second event is rate-limited
         assert_eq!(rate_limited, 1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-eval/src/lib.rs:1811:
     }
 }
[31m-
(B[m 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:41:
     payload: &serde_json::Value,
 ) -> DedupeResult<DedupeKey> {
     // Normalize payload by sorting keys (JSON object key order doesn't matter)
[31m-    let normalized = normalize_json_keys(payload)
(B[m[31m-        .map_err(|e| DedupeError::HashFailed(e.to_string()))?;
(B[m[32m+    let normalized =
(B[m[32m+        normalize_json_keys(payload).map_err(|e| DedupeError::HashFailed(e.to_string()))?;
(B[m 
     // Construct deterministic input: connector_id || event_type || normalized_json
[31m-    let input = format!(
(B[m[31m-        "{}||{}||{}",
(B[m[31m-        connector_id,
(B[m[31m-        event_type,
(B[m[31m-        normalized
(B[m[31m-    );
(B[m[32m+    let input = format!("{}||{}||{}", connector_id, event_type, normalized);
(B[m 
     let mut hasher = Sha256::new();
     hasher.update(input.as_bytes());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:165:
     #[test]
     fn canonical_hash_deterministic_same_payload() {
         let payload = json!({ "id": "123", "value": 42 });
[31m-        let hash1 = compute_canonical_hash("connector-a", "event_type_x", &payload)
(B[m[31m-            .expect("hash1");
(B[m[31m-        let hash2 = compute_canonical_hash("connector-a", "event_type_x", &payload)
(B[m[31m-            .expect("hash2");
(B[m[32m+        let hash1 = compute_canonical_hash("connector-a", "event_type_x", &payload).expect("hash1");
(B[m[32m+        let hash2 = compute_canonical_hash("connector-a", "event_type_x", &payload).expect("hash2");
(B[m         assert_eq!(hash1, hash2);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:177:
     fn canonical_hash_different_key_order_same_hash() {
         let payload1 = json!({ "id": "123", "value": 42 });
         let payload2 = json!({ "value": 42, "id": "123" });
[31m-        let hash1 = compute_canonical_hash("connector-a", "event_type_x", &payload1)
(B[m[31m-            .expect("hash1");
(B[m[31m-        let hash2 = compute_canonical_hash("connector-a", "event_type_x", &payload2)
(B[m[31m-            .expect("hash2");
(B[m[32m+        let hash1 =
(B[m[32m+            compute_canonical_hash("connector-a", "event_type_x", &payload1).expect("hash1");
(B[m[32m+        let hash2 =
(B[m[32m+            compute_canonical_hash("connector-a", "event_type_x", &payload2).expect("hash2");
(B[m         assert_eq!(hash1, hash2, "JSON key order should not affect hash");
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:188:
     #[test]
     fn canonical_hash_different_connector_different_hash() {
         let payload = json!({ "id": "123", "value": 42 });
[31m-        let hash_a = compute_canonical_hash("connector-a", "event_type_x", &payload)
(B[m[31m-            .expect("hash_a");
(B[m[31m-        let hash_b = compute_canonical_hash("connector-b", "event_type_x", &payload)
(B[m[31m-            .expect("hash_b");
(B[m[32m+        let hash_a =
(B[m[32m+            compute_canonical_hash("connector-a", "event_type_x", &payload).expect("hash_a");
(B[m[32m+        let hash_b =
(B[m[32m+            compute_canonical_hash("connector-b", "event_type_x", &payload).expect("hash_b");
(B[m         assert_ne!(hash_a, hash_b);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:211:
     fn canonical_hash_different_payload_different_hash() {
         let payload1 = json!({ "id": "123", "value": 42 });
         let payload2 = json!({ "id": "123", "value": 99 });
[31m-        let hash1 = compute_canonical_hash("connector-a", "event_type_x", &payload1)
(B[m[31m-            .expect("hash1");
(B[m[31m-        let hash2 = compute_canonical_hash("connector-a", "event_type_x", &payload2)
(B[m[31m-            .expect("hash2");
(B[m[32m+        let hash1 =
(B[m[32m+            compute_canonical_hash("connector-a", "event_type_x", &payload1).expect("hash1");
(B[m[32m+        let hash2 =
(B[m[32m+            compute_canonical_hash("connector-a", "event_type_x", &payload2).expect("hash2");
(B[m         assert_ne!(hash1, hash2);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:265:
         let input = json!({ "z": 1, "a": 2 });
         let normalized = normalize_json_keys(&input).expect("normalize");
         // Keys should be sorted
[31m-        let keys: Vec<_> = normalized
(B[m[31m-            .as_object()
(B[m[31m-            .expect("object")
(B[m[31m-            .keys()
(B[m[31m-            .collect();
(B[m[32m+        let keys: Vec<_> = normalized.as_object().expect("object").keys().collect();
(B[m         assert_eq!(keys, vec!["a", "z"]);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/dedup.rs:282:
         });
         let normalized = normalize_json_keys(&input).expect("normalize");
         let outer = normalized.get("outer").expect("outer");
[31m-        let outer_keys: Vec<_> = outer
(B[m[31m-            .as_object()
(B[m[31m-            .expect("object")
(B[m[31m-            .keys()
(B[m[31m-            .collect();
(B[m[32m+        let outer_keys: Vec<_> = outer.as_object().expect("object").keys().collect();
(B[m         assert_eq!(outer_keys, vec!["a", "z"]);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events/src/lib.rs:253:
     #[test]
     fn from_manifest_string_canonical_yields_well_known() {
         let et = EventType::from_manifest_string("canvas", "assignment_due");
[31m-        assert!(matches!(et, EventType::WellKnown(WellKnownEventType::AssignmentDue)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            et,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::AssignmentDue)
(B[m[32m+        ));
(B[m         let et2 = EventType::from_manifest_string("canvas", "AssignmentGraded");
[31m-        assert!(matches!(et2, EventType::WellKnown(WellKnownEventType::AssignmentGraded)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            et2,
(B[m[32m+            EventType::WellKnown(WellKnownEventType::AssignmentGraded)
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events-core/src/lib.rs:16:
 use std::sync::Arc;
 
 use chrono::{DateTime, Utc};
[31m-use focus_result::FocusResult;
(B[m use focus_events::{EventType, NormalizedEvent};
[32m+use focus_result::FocusResult;
(B[m use serde::{Deserialize, Serialize};
 use tokio::sync::{broadcast, RwLock};
 use tracing::debug;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-events-core/src/lib.rs:137:
     pub async fn subscribe(&self, topic: &str) -> FocusResult<EventSubscription> {
         let mut topics = self.topics.write().await;
 
[31m-        let sender = topics.entry(topic.to_string()).or_insert_with(|| {
(B[m[31m-            broadcast::channel(self.config.max_subscribers).0
(B[m[31m-        });
(B[m[32m+        let sender = topics
(B[m[32m+            .entry(topic.to_string())
(B[m[32m+            .or_insert_with(|| broadcast::channel(self.config.max_subscribers).0);
(B[m 
         let receiver = sender.subscribe();
         let subscription = EventSubscription {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/bin/android_bindings.rs:17:
 use std::process::Command;
 
 const ANDROID_ABIS: &[(&str, &str)] = &[
[31m-    ("aarch64-linux-android", "arm64-v8a"),       // Primary (most devices)
(B[m[31m-    ("armv7-linux-androideabi", "armeabi-v7a"),   // Older devices
(B[m[31m-    ("x86_64-linux-android", "x86_64"),           // Emulator
(B[m[31m-    ("i686-linux-android", "x86"),                // Legacy
(B[m[32m+    ("aarch64-linux-android", "arm64-v8a"), // Primary (most devices)
(B[m[32m+    ("armv7-linux-androideabi", "armeabi-v7a"), // Older devices
(B[m[32m+    ("x86_64-linux-android", "x86_64"),     // Emulator
(B[m[32m+    ("i686-linux-android", "x86"),          // Legacy
(B[m ];
 
 fn main() -> Result<()> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/bin/android_bindings.rs:57:
     let udl_file = ffi_crate.join("src/focus_ffi.udl");
 
     if !udl_file.exists() {
[31m-        return Err(anyhow!(
(B[m[31m-            "UDL file not found: {}",
(B[m[31m-            udl_file.display()
(B[m[31m-        ));
(B[m[32m+        return Err(anyhow!("UDL file not found: {}", udl_file.display()));
(B[m     }
 
     eprintln!();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:245:
 
 #[derive(Debug, Clone)]
 pub enum RuleActionDto {
[31m-    GrantCredit { amount: i32 },
(B[m[31m-    DeductCredit { amount: i32 },
(B[m[31m-    Block { profile: String, duration_seconds: i64 },
(B[m[31m-    Unblock { profile: String },
(B[m[31m-    StreakIncrement { name: String },
(B[m[31m-    StreakReset { name: String },
(B[m[31m-    Notify { message: String },
(B[m[32m+    GrantCredit {
(B[m[32m+        amount: i32,
(B[m[32m+    },
(B[m[32m+    DeductCredit {
(B[m[32m+        amount: i32,
(B[m[32m+    },
(B[m[32m+    Block {
(B[m[32m+        profile: String,
(B[m[32m+        duration_seconds: i64,
(B[m[32m+    },
(B[m[32m+    Unblock {
(B[m[32m+        profile: String,
(B[m[32m+    },
(B[m[32m+    StreakIncrement {
(B[m[32m+        name: String,
(B[m[32m+    },
(B[m[32m+    StreakReset {
(B[m[32m+        name: String,
(B[m[32m+    },
(B[m[32m+    Notify {
(B[m[32m+        message: String,
(B[m[32m+    },
(B[m }
 
 #[derive(Debug, Clone)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:272:
         match a {
             RuleActionDto::GrantCredit { amount } => CoreAction::GrantCredit { amount },
             RuleActionDto::DeductCredit { amount } => CoreAction::DeductCredit { amount },
[31m-            RuleActionDto::Block { profile, duration_seconds } => CoreAction::Block {
(B[m[32m+            RuleActionDto::Block {
(B[m                 profile,
[32m+                duration_seconds,
(B[m[32m+            } => CoreAction::Block {
(B[m[32m+                profile,
(B[m                 duration: ChronoDuration::seconds(duration_seconds),
                 rigidity: focus_domain::Rigidity::Hard,
             },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:334:
 
 #[derive(Debug, Clone)]
 pub enum WalletMutationDto {
[31m-    GrantCredit { amount: i64 },
(B[m[31m-    SpendCredit { amount: i64, purpose: String },
(B[m[31m-    StreakIncrement { name: String },
(B[m[31m-    StreakReset { name: String },
(B[m[31m-    SetMultiplier { current: f32, expires_iso: Option<String> },
(B[m[32m+    GrantCredit {
(B[m[32m+        amount: i64,
(B[m[32m+    },
(B[m[32m+    SpendCredit {
(B[m[32m+        amount: i64,
(B[m[32m+        purpose: String,
(B[m[32m+    },
(B[m[32m+    StreakIncrement {
(B[m[32m+        name: String,
(B[m[32m+    },
(B[m[32m+    StreakReset {
(B[m[32m+        name: String,
(B[m[32m+    },
(B[m[32m+    SetMultiplier {
(B[m[32m+        current: f32,
(B[m[32m+        expires_iso: Option<String>,
(B[m[32m+    },
(B[m }
 
 fn parse_iso(s: &str) -> Result<DateTime<Utc>, FfiError> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:366:
                 CoreWalletMutation::StreakIncrement(name)
             }
             WalletMutationDto::StreakReset { name } => CoreWalletMutation::StreakReset(name),
[31m-            WalletMutationDto::SetMultiplier { current, expires_iso } => {
(B[m[31m-                CoreWalletMutation::SetMultiplier(MultiplierState {
(B[m[31m-                    current,
(B[m[31m-                    expires_at: parse_iso_opt(expires_iso)?,
(B[m[31m-                })
(B[m[31m-            }
(B[m[32m+            WalletMutationDto::SetMultiplier {
(B[m[32m+                current,
(B[m[32m+                expires_iso,
(B[m[32m+            } => CoreWalletMutation::SetMultiplier(MultiplierState {
(B[m[32m+                current,
(B[m[32m+                expires_at: parse_iso_opt(expires_iso)?,
(B[m[32m+            }),
(B[m         })
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:430:
         "Restricted" => EscalationTier::Restricted,
         "Strict" => EscalationTier::Strict,
         other => {
[31m-            return Err(FfiError::InvalidArgument(format!("unknown escalation tier: {other}")))
(B[m[32m+            return Err(FfiError::InvalidArgument(format!(
(B[m[32m+                "unknown escalation tier: {other}"
(B[m[32m+            )))
(B[m         }
     })
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:452:
                 })
             }
             PenaltyMutationDto::ClearLockouts => CorePenaltyMutation::ClearLockouts,
[31m-            PenaltyMutationDto::SetStrictMode { until_iso } => {
(B[m[31m-                CorePenaltyMutation::SetStrictMode { until: parse_iso(&until_iso)? }
(B[m[31m-            }
(B[m[32m+            PenaltyMutationDto::SetStrictMode { until_iso } => CorePenaltyMutation::SetStrictMode {
(B[m[32m+                until: parse_iso(&until_iso)?,
(B[m[32m+            },
(B[m             PenaltyMutationDto::Clear => CorePenaltyMutation::Clear,
         })
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:669:
         MorningBriefDto {
             date: v.date.to_string(),
             intention: v.intention,
[31m-            top_priorities: v.top_priorities.iter().map(TopPriorityLineDto::from).collect(),
(B[m[32m+            top_priorities: v
(B[m[32m+                .top_priorities
(B[m[32m+                .iter()
(B[m[32m+                .map(TopPriorityLineDto::from)
(B[m[32m+                .collect(),
(B[m             schedule_preview: SchedulePreviewDto::from(&v.schedule_preview),
             coachy_opening: v.coachy_opening,
             generated_at_iso: v.generated_at.to_rfc3339(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:744:
             credits_earned: v.credits_earned,
             credits_spent: v.credits_spent,
             top_rules: v.top_rules.iter().map(RuleSummaryDto::from).collect(),
[31m-            streaks_extended: v.streaks_extended.iter().map(StreakSnapshotDto::from).collect(),
(B[m[32m+            streaks_extended: v
(B[m[32m+                .streaks_extended
(B[m[32m+                .iter()
(B[m[32m+                .map(StreakSnapshotDto::from)
(B[m[32m+                .collect(),
(B[m             tasks_completed: v.tasks_completed,
             tasks_slipped: v.tasks_slipped,
             wins_summary: v.wins_summary,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:886:
         &self,
         _draft: &focus_calendar::CalendarEventDraft,
     ) -> anyhow::Result<CoreCalendarEvent> {
[31m-        Err(anyhow::anyhow!("HostBackedCalendarPort is read-only (device calendar)"))
(B[m[32m+        Err(anyhow::anyhow!(
(B[m[32m+            "HostBackedCalendarPort is read-only (device calendar)"
(B[m[32m+        ))
(B[m     }
 
     async fn delete_event(&self, _id: &str) -> anyhow::Result<()> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:893:
[31m-        Err(anyhow::anyhow!("HostBackedCalendarPort is read-only (device calendar)"))
(B[m[32m+        Err(anyhow::anyhow!(
(B[m[32m+            "HostBackedCalendarPort is read-only (device calendar)"
(B[m[32m+        ))
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:929:
 impl RuleQuery {
     pub fn list_enabled(&self) -> Result<Vec<RuleSummary>, FfiError> {
         let adapter = self.ctx.adapter.clone();
[31m-        let rules =
(B[m[31m-            self.ctx.runtime.block_on(async move { RuleStore::list_enabled(&adapter).await })?;
(B[m[32m+        let rules = self
(B[m[32m+            .ctx
(B[m[32m+            .runtime
(B[m[32m+            .block_on(async move { RuleStore::list_enabled(&adapter).await })?;
(B[m         Ok(rules.iter().map(rule_to_summary).collect())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:958:
     pub fn upsert(&self, rule: RuleDraft) -> Result<(), FfiError> {
         let core = draft_to_core(rule)?;
         let adapter = self.ctx.adapter.clone();
[31m-        self.ctx.runtime.block_on(async move { upsert_rule(&adapter, core).await })?;
(B[m[32m+        self.ctx
(B[m[32m+            .runtime
(B[m[32m+            .block_on(async move { upsert_rule(&adapter, core).await })?;
(B[m         Ok(())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:971:
     pub fn load(&self) -> Result<WalletSummary, FfiError> {
         let adapter = self.ctx.adapter.clone();
         let user_id = self.ctx.user_id;
[31m-        let wallet =
(B[m[31m-            self.ctx.runtime.block_on(async move { WalletStore::load(&adapter, user_id).await })?;
(B[m[32m+        let wallet = self
(B[m[32m+            .ctx
(B[m[32m+            .runtime
(B[m[32m+            .block_on(async move { WalletStore::load(&adapter, user_id).await })?;
(B[m         let multiplier = wallet.effective_multiplier(Utc::now());
         let streaks = wallet
             .streaks
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1054:
             .ctx
             .runtime
             .block_on(async move { PenaltyStore::load(&adapter, user_id).await })?;
[31m-        let quote = state.quote_bypass(cost).map_err(|e| FfiError::Domain(e.to_string()))?;
(B[m[32m+        let quote = state
(B[m[32m+            .quote_bypass(cost)
(B[m[32m+            .map_err(|e| FfiError::Domain(e.to_string()))?;
(B[m         Ok(BypassQuoteDto {
             cost: quote.cost,
             remaining_after: quote.remaining_after,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1105:
             .recent_decisions
             .lock()
             .map_err(|e| FfiError::Storage(format!("decisions mutex poisoned: {e}")))?;
[31m-        let n = if limit <= 0 { recent.len() } else { (limit as usize).min(recent.len()) };
(B[m[32m+        let n = if limit <= 0 {
(B[m[32m+            recent.len()
(B[m[32m+        } else {
(B[m[32m+            (limit as usize).min(recent.len())
(B[m[32m+        };
(B[m         let slice: Vec<PrioritizedDecision> = recent.iter().rev().take(n).cloned().collect();
         let policy =
             PolicyBuilder::from_rule_decisions(&slice, Utc::now(), &focus_audit::NoopAuditSink);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1136:
 
 impl AuditApi {
     pub fn verify_chain(&self) -> Result<bool, FfiError> {
[31m-        self.ctx.audit.verify_chain().map_err(|e| FfiError::Storage(e.to_string()))
(B[m[32m+        self.ctx
(B[m[32m+            .audit
(B[m[32m+            .verify_chain()
(B[m[32m+            .map_err(|e| FfiError::Storage(e.to_string()))
(B[m     }
 
     pub fn head_hash(&self) -> Result<Option<String>, FfiError> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1143:
[31m-        self.ctx.audit.head_hash().map_err(|e| FfiError::Storage(e.to_string()))
(B[m[32m+        self.ctx
(B[m[32m+            .audit
(B[m[32m+            .head_hash()
(B[m[32m+            .map_err(|e| FfiError::Storage(e.to_string()))
(B[m     }
 
     /// Return the most recent `limit` audit records in newest-first order.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1189:
         let engine = self.engine.clone();
         let user_id = self.ctx.user_id;
         let brief = self.ctx.runtime.block_on(async move {
[31m-            engine.generate_morning_brief(&tasks, user_id, Utc::now()).await
(B[m[32m+            engine
(B[m[32m+                .generate_morning_brief(&tasks, user_id, Utc::now())
(B[m[32m+                .await
(B[m         })?;
         Ok(MorningBriefDto::from(brief))
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1224:
             .list(self.ctx.user_id)
             .map_err(|e| FfiError::Storage(format!("task store list: {e}")))?;
         let engine = self.engine.clone();
[31m-        let converted: Vec<CoreTaskActual> =
(B[m[31m-            actuals.into_iter().map(|a| a.into_core()).collect::<Result<_, _>>()?;
(B[m[32m+        let converted: Vec<CoreTaskActual> = actuals
(B[m[32m+            .into_iter()
(B[m[32m+            .map(|a| a.into_core())
(B[m[32m+            .collect::<Result<_, _>>()?;
(B[m         let now = Utc::now();
         let schedule = self.ctx.runtime.block_on(async move {
[31m-            engine.scheduler.plan(&tasks, &[], now, ChronoDuration::hours(24)).await
(B[m[32m+            engine
(B[m[32m+                .scheduler
(B[m[32m+                .plan(&tasks, &[], now, ChronoDuration::hours(24))
(B[m[32m+                .await
(B[m         })?;
         let engine2 = self.engine.clone();
         let shutdown = self.ctx.runtime.block_on(async move {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1235:
[31m-            engine2.generate_evening_shutdown(&schedule, &converted, now).await
(B[m[32m+            engine2
(B[m[32m+                .generate_evening_shutdown(&schedule, &converted, now)
(B[m[32m+                .await
(B[m         })?;
         Ok(EveningShutdownDto::from(shutdown))
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1240:
     pub fn generate_weekly_review(&self) -> Result<WeeklyReviewDto, FfiError> {
         let engine = self.weekly_engine.clone();
         let now = Utc::now();
[31m-        let review =
(B[m[31m-            self.ctx.runtime.block_on(async move { engine.generate_weekly_review(now).await })?;
(B[m[32m+        let review = self
(B[m[32m+            .ctx
(B[m[32m+            .runtime
(B[m[32m+            .block_on(async move { engine.generate_weekly_review(now).await })?;
(B[m         Ok(WeeklyReviewDto::from(review))
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1248:
     pub fn generate_monthly_retro(&self) -> Result<MonthlyRetroDto, FfiError> {
         let engine = self.monthly_engine.clone();
         let now = Utc::now();
[31m-        let retro =
(B[m[31m-            self.ctx.runtime.block_on(async move { engine.generate_monthly_retro(now).await })?;
(B[m[32m+        let retro = self
(B[m[32m+            .ctx
(B[m[32m+            .runtime
(B[m[32m+            .block_on(async move { engine.generate_monthly_retro(now).await })?;
(B[m         Ok(MonthlyRetroDto::from(retro))
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1285:
         // Default Semi cost — the v1 input surface does not expose the
         // specific cost variant; we pick a neutral credit cost so the task is
         // still schedulable and audit-distinguishable from Hard/Soft.
[31m-        "semi" => Ok(focus_domain::Rigidity::Semi(focus_domain::RigidityCost::CreditCost(0))),
(B[m[32m+        "semi" => Ok(focus_domain::Rigidity::Semi(
(B[m[32m+            focus_domain::RigidityCost::CreditCost(0),
(B[m[32m+        )),
(B[m         other => Err(FfiError::InvalidArgument(format!(
             "deadline_rigidity must be hard|semi|soft, got: {other}"
         ))),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1313:
 fn task_to_summary(t: &Task) -> TaskSummaryDto {
     let minutes = t.duration.planning_duration().num_minutes().max(0) as u32;
     let (deadline_iso, rigidity) = match t.deadline.when {
[31m-        Some(w) => (Some(w.to_rfc3339()), rigidity_tag(&t.deadline.rigidity).to_string()),
(B[m[32m+        Some(w) => (
(B[m[32m+            Some(w.to_rfc3339()),
(B[m[32m+            rigidity_tag(&t.deadline.rigidity).to_string(),
(B[m[32m+        ),
(B[m         // No deadline: report rigidity as "soft" (the `Deadline::none()` default)
         // but elide the ISO string so the caller can render "no deadline".
         None => (None, rigidity_tag(&t.deadline.rigidity).to_string()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1341:
             return Err(FfiError::InvalidArgument("title must not be empty".into()));
         }
         if input.duration_minutes == 0 {
[31m-            return Err(FfiError::InvalidArgument("duration_minutes must be > 0".into()));
(B[m[32m+            return Err(FfiError::InvalidArgument(
(B[m[32m+                "duration_minutes must be > 0".into(),
(B[m[32m+            ));
(B[m         }
         let rigidity = parse_rigidity(&input.deadline_rigidity)?;
[31m-        let deadline = match input.deadline_iso.as_deref().map(str::trim).filter(|s| !s.is_empty())
(B[m[32m+        let deadline = match input
(B[m[32m+            .deadline_iso
(B[m[32m+            .as_deref()
(B[m[32m+            .map(str::trim)
(B[m[32m+            .filter(|s| !s.is_empty())
(B[m         {
             Some(iso) => {
                 let when = DateTime::parse_from_rfc3339(iso)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1351:
                     .map_err(|e| FfiError::InvalidArgument(format!("deadline_iso: {e}")))?
                     .with_timezone(&Utc);
[31m-                CoreDeadline { when: Some(when), rigidity }
(B[m[32m+                CoreDeadline {
(B[m[32m+                    when: Some(when),
(B[m[32m+                    rigidity,
(B[m[32m+                }
(B[m             }
[31m-            None => CoreDeadline { when: None, rigidity },
(B[m[32m+            None => CoreDeadline {
(B[m[32m+                when: None,
(B[m[32m+                rigidity,
(B[m[32m+            },
(B[m         };
 
         let now = Utc::now();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1399:
             .map_err(|e| FfiError::Storage(format!("task get: {e}")))?
             .map(|t| t.title)
             .unwrap_or_default();
[31m-        let removed =
(B[m[31m-            self.store.delete(id).map_err(|e| FfiError::Storage(format!("task delete: {e}")))?;
(B[m[32m+        let removed = self
(B[m[32m+            .store
(B[m[32m+            .delete(id)
(B[m[32m+            .map_err(|e| FfiError::Storage(format!("task delete: {e}")))?;
(B[m         if !removed {
             return Err(FfiError::Storage(format!("not found: {id}")));
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1476:
     cadence: StdDuration,
     now: DateTime<Utc>,
 ) -> Result<(), FfiError> {
[31m-    match orch.register(id.to_string(), connector.clone(), cadence, now).await {
(B[m[32m+    match orch
(B[m[32m+        .register(id.to_string(), connector.clone(), cadence, now)
(B[m[32m+        .await
(B[m[32m+    {
(B[m         Ok(()) => Ok(()),
         Err(OrchestratorError::AlreadyRegistered(_)) => {
             // Drop the stale handle and insert the fresh one.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1565:
         let sync = self.ctx.sync.clone();
         self.ctx.runtime.block_on(async move {
             let mut guard = sync.lock().await;
[31m-            register_or_refresh(&mut guard, &manifest_id, connector, CANVAS_SYNC_CADENCE, now).await
(B[m[32m+            register_or_refresh(
(B[m[32m+                &mut guard,
(B[m[32m+                &manifest_id,
(B[m[32m+                connector,
(B[m[32m+                CANVAS_SYNC_CADENCE,
(B[m[32m+                now,
(B[m[32m+            )
(B[m[32m+            .await
(B[m         })?;
 
         let mut chain = self
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1726:
         let sync = self.ctx.sync.clone();
         self.ctx.runtime.block_on(async move {
             let mut guard = sync.lock().await;
[31m-            register_or_refresh(&mut guard, &manifest_id, connector, GITHUB_SYNC_CADENCE, now).await
(B[m[32m+            register_or_refresh(
(B[m[32m+                &mut guard,
(B[m[32m+                &manifest_id,
(B[m[32m+                connector,
(B[m[32m+                GITHUB_SYNC_CADENCE,
(B[m[32m+                now,
(B[m[32m+            )
(B[m[32m+            .await
(B[m         })?;
 
         let mut chain = self
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1778:
 impl EvalApi {
     pub fn tick(&self) -> Result<EvaluationReportDto, FfiError> {
         let pipeline = self.ctx.eval_pipeline.clone();
[31m-        let report = self.ctx.runtime.block_on(async move { pipeline.tick(Utc::now()).await })?;
(B[m[32m+        let report = self
(B[m[32m+            .ctx
(B[m[32m+            .runtime
(B[m[32m+            .block_on(async move { pipeline.tick(Utc::now()).await })?;
(B[m         Ok(report.into())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1846:
     pub fn emit(&self, dto: HostEventDto) -> Result<(), FfiError> {
         let event_type_raw = dto.event_type.trim();
         if event_type_raw.is_empty() {
[31m-            return Err(FfiError::InvalidArgument("event_type must be non-empty".into()));
(B[m[32m+            return Err(FfiError::InvalidArgument(
(B[m[32m+                "event_type must be non-empty".into(),
(B[m[32m+            ));
(B[m         }
         if !dto.confidence.is_finite() || dto.confidence < 0.0 || dto.confidence > 1.0 {
             return Err(FfiError::InvalidArgument(format!(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1888:
             payload,
             raw_ref: None,
         };
[31m-        event.validate().map_err(|e| FfiError::InvalidArgument(format!("invalid event: {e}")))?;
(B[m[32m+        event
(B[m[32m+            .validate()
(B[m[32m+            .map_err(|e| FfiError::InvalidArgument(format!("invalid event: {e}")))?;
(B[m 
         let sink = self.ctx.event_sink.clone();
         let event_for_append = event.clone();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1925:
 /// TOML source for every starter pack shipped in `examples/templates/`,
 /// bundled at build time via `include_str!`.
 const BUNDLED_TEMPLATES: &[(&str, &str)] = &[
[31m-    ("deep-work-starter", include_str!("../../../examples/templates/deep-work-starter.toml")),
(B[m[31m-    ("student-canvas", include_str!("../../../examples/templates/student-canvas.toml")),
(B[m[31m-    ("dev-flow", include_str!("../../../examples/templates/dev-flow.toml")),
(B[m[31m-    ("sleep-hygiene", include_str!("../../../examples/templates/sleep-hygiene.toml")),
(B[m[32m+    (
(B[m[32m+        "deep-work-starter",
(B[m[32m+        include_str!("../../../examples/templates/deep-work-starter.toml"),
(B[m[32m+    ),
(B[m[32m+    (
(B[m[32m+        "student-canvas",
(B[m[32m+        include_str!("../../../examples/templates/student-canvas.toml"),
(B[m[32m+    ),
(B[m[32m+    (
(B[m[32m+        "dev-flow",
(B[m[32m+        include_str!("../../../examples/templates/dev-flow.toml"),
(B[m[32m+    ),
(B[m[32m+    (
(B[m[32m+        "sleep-hygiene",
(B[m[32m+        include_str!("../../../examples/templates/sleep-hygiene.toml"),
(B[m[32m+    ),
(B[m ];
 
 #[derive(Debug, Clone)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1972:
         BUNDLED_TEMPLATES
             .iter()
             .filter_map(|(_, toml)| {
[31m-                focus_templates::TemplatePack::from_toml_str(toml).ok().map(|pack| {
(B[m[31m-                    TemplatePackSummary {
(B[m[32m+                focus_templates::TemplatePack::from_toml_str(toml)
(B[m[32m+                    .ok()
(B[m[32m+                    .map(|pack| TemplatePackSummary {
(B[m                         id: pack.id.clone(),
                         name: pack.name.clone(),
                         version: pack.version.clone(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:1981:
                         description: pack.description.clone(),
                         recommended_connectors: pack.recommended_connectors.clone(),
                         rule_count: pack.rules.len() as u32,
[31m-                    }
(B[m[31m-                })
(B[m[32m+                    })
(B[m             })
             .collect()
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2003:
             runtime: self.ctx.runtime.as_ref(),
             installed: Vec::new(),
         };
[31m-        let n =
(B[m[31m-            pack.apply(&mut shim).map_err(|e| FfiError::Storage(format!("template apply: {e}")))?;
(B[m[32m+        let n = pack
(B[m[32m+            .apply(&mut shim)
(B[m[32m+            .map_err(|e| FfiError::Storage(format!("template apply: {e}")))?;
(B[m 
         self.ctx
             .audit
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2281:
                 .map_err(|e| FfiError::Storage(format!("wipe_all: {e}")))?;
 
             // Save receipt to disk and update the DTO with the path.
[31m-            receipt.save().map_err(|e| FfiError::Storage(format!("save wipe receipt: {e}")))?;
(B[m[32m+            receipt
(B[m[32m+                .save()
(B[m[32m+                .map_err(|e| FfiError::Storage(format!("save wipe receipt: {e}")))?;
(B[m 
             Ok(WipeReceiptDto::from(receipt))
         })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2327:
 
 impl Default for SuggesterApi {
     fn default() -> Self {
[31m-        Self { dismissed: Mutex::new(std::collections::HashSet::new()) }
(B[m[32m+        Self {
(B[m[32m+            dismissed: Mutex::new(std::collections::HashSet::new()),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2345:
     pub fn apply(&self, suggestion_id: String) -> Result<(), FfiError> {
         // In production: deserialize proposed rule from suggestion and call
         // rules_mut().upsert() to persist it. For now, accept idempotently.
[31m-        let mut dismissed = self.dismissed.lock()
(B[m[32m+        let mut dismissed = self
(B[m[32m+            .dismissed
(B[m[32m+            .lock()
(B[m             .map_err(|e| FfiError::Poisoned(format!("dismissed lock: {}", e)))?;
         dismissed.remove(&suggestion_id);
         Ok(())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2352:
     }
 
     pub fn dismiss(&self, suggestion_id: String) -> Result<(), FfiError> {
[31m-        let mut dismissed = self.dismissed.lock()
(B[m[32m+        let mut dismissed = self
(B[m[32m+            .dismissed
(B[m[32m+            .lock()
(B[m             .map_err(|e| FfiError::Poisoned(format!("dismissed lock: {}", e)))?;
         dismissed.insert(suggestion_id);
         Ok(())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2371:
 
 impl CoachingConfig {
     pub fn new(endpoint: String, api_key: String, model: String) -> Self {
[31m-        Self { endpoint, api_key: SecretString::from(api_key), model }
(B[m[32m+        Self {
(B[m[32m+            endpoint,
(B[m[32m+            api_key: SecretString::from(api_key),
(B[m[32m+            model,
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2418:
         // Wire the SqliteAdapter as the sync-side EventSink so connector
         // events are durably appended to the events table on every sync
         // rather than silently dropped.
[31m-        let event_sink_adapter: Arc<dyn EventSink> =
(B[m[31m-            Arc::new(SqliteEventSinkAdapter { adapter: adapter.clone() });
(B[m[32m+        let event_sink_adapter: Arc<dyn EventSink> = Arc::new(SqliteEventSinkAdapter {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         let orchestrator =
             SyncOrchestrator::with_default_retry().with_event_sink(event_sink_adapter.clone());
         let user_id = Uuid::nil();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2505:
     pub fn set_calendar_host(&self, host: Box<dyn CalendarHost>) {
         let host: Arc<dyn CalendarHost> = Arc::from(host);
         let port: Arc<dyn CalendarPort> = Arc::new(HostBackedCalendarPort::new(host));
[31m-        let mut guard = self.rituals_calendar.write().expect("rituals_calendar rwlock poisoned");
(B[m[32m+        let mut guard = self
(B[m[32m+            .rituals_calendar
(B[m[32m+            .write()
(B[m[32m+            .expect("rituals_calendar rwlock poisoned");
(B[m         *guard = port;
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2575:
     }
 
     pub fn rules(&self) -> Arc<RuleQuery> {
[31m-        Arc::new(RuleQuery { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(RuleQuery {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn mutations(&self) -> Arc<RuleMutation> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2582:
[31m-        Arc::new(RuleMutation { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(RuleMutation {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn wallet(&self) -> Arc<WalletApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2586:
[31m-        Arc::new(WalletApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(WalletApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn penalty(&self) -> Arc<PenaltyApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2590:
[31m-        Arc::new(PenaltyApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(PenaltyApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn policy(&self) -> Arc<PolicyApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2594:
[31m-        Arc::new(PolicyApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(PolicyApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn audit(&self) -> Arc<AuditApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2598:
[31m-        Arc::new(AuditApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(AuditApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn sync(&self) -> Arc<SyncApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2602:
[31m-        Arc::new(SyncApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(SyncApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn eval(&self) -> Arc<EvalApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2606:
[31m-        Arc::new(EvalApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(EvalApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     pub fn connector(&self) -> Arc<ConnectorApi> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2610:
[31m-        Arc::new(ConnectorApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(ConnectorApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     /// Access the Planning Coach rituals surface (Morning Brief + Evening
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2619:
             g.unwrap_or_else(|| Arc::new(NoopCoachingProvider))
         };
         let scheduler = Arc::new(Scheduler::new(WorkingHoursSpec::default()));
[31m-        let calendar: Arc<dyn focus_calendar::CalendarPort> =
(B[m[31m-            self.rituals_calendar.read().expect("rituals_calendar rwlock poisoned").clone();
(B[m[32m+        let calendar: Arc<dyn focus_calendar::CalendarPort> = self
(B[m[32m+            .rituals_calendar
(B[m[32m+            .read()
(B[m[32m+            .expect("rituals_calendar rwlock poisoned")
(B[m[32m+            .clone();
(B[m         let engine = Arc::new(RitualsEngine::new(
             scheduler,
             calendar,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2640:
 
     /// User-facing CRUD over the persistent task pool.
     pub fn tasks(&self) -> Arc<TaskApi> {
[31m-        Arc::new(TaskApi { ctx: self.ctx.clone(), store: self.task_store.clone() })
(B[m[32m+        Arc::new(TaskApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+            store: self.task_store.clone(),
(B[m[32m+        })
(B[m     }
 
     /// Bundled starter-pack template library. Backed by TOML files in
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2647:
     /// `examples/templates/` embedded at build time via `include_str!`.
     pub fn templates(&self) -> Arc<TemplateApi> {
[31m-        Arc::new(TemplateApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(TemplateApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     /// Host-event injection surface. iOS/Android call this to emit synthetic
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2654:
     /// durable pipeline that connector sync uses; rule evaluation picks them
     /// up on the next `eval().tick()`.
     pub fn host_events(&self) -> Arc<HostEventApi> {
[31m-        Arc::new(HostEventApi { ctx: self.ctx.clone() })
(B[m[32m+        Arc::new(HostEventApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+        })
(B[m     }
 
     /// Proactive nudge proposals from the always-on engine. Backed by a habit
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2662:
     /// confidence exceeds the threshold. Called every 60 seconds from the iOS
     /// foreground heartbeat (after `syncTick()` + `evalTick()`).
     pub fn always_on(&self) -> Arc<AlwaysOnApi> {
[31m-        Arc::new(AlwaysOnApi { ctx: self.ctx.clone(), engine: self.always_on_engine.clone() })
(B[m[32m+        Arc::new(AlwaysOnApi {
(B[m[32m+            ctx: self.ctx.clone(),
(B[m[32m+            engine: self.always_on_engine.clone(),
(B[m[32m+        })
(B[m     }
 
     /// Encrypted full-backup and restore surface.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2710:
             self.task_store.delete(t.id).expect("task delete");
         }
         for t in &new {
[31m-            self.task_store.upsert(self.ctx.user_id, t).expect("task upsert");
(B[m[32m+            self.task_store
(B[m[32m+                .upsert(self.ctx.user_id, t)
(B[m[32m+                .expect("task upsert");
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2733:
     /// tests and (eventually) by the rule engine runner. Not exposed over FFI.
     #[doc(hidden)]
     pub fn record_decision_for_test(&self, decision: PrioritizedDecision) {
[31m-        let mut recent = self.ctx.recent_decisions.lock().expect("decisions poisoned");
(B[m[32m+        let mut recent = self
(B[m[32m+            .ctx
(B[m[32m+            .recent_decisions
(B[m[32m+            .lock()
(B[m[32m+            .expect("decisions poisoned");
(B[m         recent.push(decision);
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2778:
         let (_d, core) = mk_core();
         let s0 = core.mascot_state();
         assert!(matches!(s0.pose, Pose::Idle));
[31m-        let s1 = core
(B[m[31m-            .push_mascot_event(MascotEvent::StreakIncremented { name: "study".into(), count: 2 });
(B[m[32m+        let s1 = core.push_mascot_event(MascotEvent::StreakIncremented {
(B[m[32m+            name: "study".into(),
(B[m[32m+            count: 2,
(B[m[32m+        });
(B[m         assert!(matches!(s1.pose, Pose::Encouraging));
         assert_eq!(core.app_version(), env!("CARGO_PKG_VERSION"));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2814:
     fn wallet_grant_then_spend_through_ffi() {
         let (_d, core) = mk_core();
         let wallet = core.wallet();
[31m-        wallet.apply_mutation(WalletMutationDto::GrantCredit { amount: 100 }).expect("grant");
(B[m[32m+        wallet
(B[m[32m+            .apply_mutation(WalletMutationDto::GrantCredit { amount: 100 })
(B[m[32m+            .expect("grant");
(B[m         let s = wallet.load().expect("load");
         assert_eq!(s.earned, 100);
         assert_eq!(s.balance, 100);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2821:
         wallet
[31m-            .apply_mutation(WalletMutationDto::SpendCredit { amount: 40, purpose: "unlock".into() })
(B[m[32m+            .apply_mutation(WalletMutationDto::SpendCredit {
(B[m[32m+                amount: 40,
(B[m[32m+                purpose: "unlock".into(),
(B[m[32m+            })
(B[m             .expect("spend");
         let s2 = wallet.load().expect("load2");
         assert_eq!(s2.balance, 60);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2829:
     fn penalty_escalate_quote_and_audit_chain_grows() {
         let (_d, core) = mk_core();
         let penalty = core.penalty();
[31m-        penalty.apply(PenaltyMutationDto::GrantBypass { amount: 10 }).expect("grant bypass");
(B[m[32m+        penalty
(B[m[32m+            .apply(PenaltyMutationDto::GrantBypass { amount: 10 })
(B[m[32m+            .expect("grant bypass");
(B[m         let q = penalty.quote_bypass(4).expect("quote");
         assert_eq!(q.cost, 4);
         assert_eq!(q.remaining_after, 6);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2836:
[31m-        penalty.apply(PenaltyMutationDto::Escalate { tier: "Warning".into() }).expect("escalate");
(B[m[32m+        penalty
(B[m[32m+            .apply(PenaltyMutationDto::Escalate {
(B[m[32m+                tier: "Warning".into(),
(B[m[32m+            })
(B[m[32m+            .expect("escalate");
(B[m         let s = penalty.load().expect("load");
         assert_eq!(s.tier, "Warning");
         assert_eq!(s.bypass_budget, 10);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2879:
         let provider: Arc<dyn CoachingProvider> =
             Arc::new(focus_coaching::StubCoachingProvider::single("Nice work!"));
         core.set_coaching_provider_for_test(provider);
[31m-        let out =
(B[m[31m-            core.generate_bubble(MascotEvent::FocusSessionCompleted { minutes: 30 }).expect("some");
(B[m[32m+        let out = core
(B[m[32m+            .generate_bubble(MascotEvent::FocusSessionCompleted { minutes: 30 })
(B[m[32m+            .expect("some");
(B[m         assert_eq!(out, "Nice work!");
         // Main mascot state should NOT have mutated.
         assert!(matches!(core.mascot_state().pose, Pose::Idle));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2906:
         let provider: Arc<dyn CoachingProvider> =
             Arc::new(focus_coaching::StubCoachingProvider::single(json_rule));
         core.set_coaching_provider_for_test(provider);
[31m-        let summary =
(B[m[31m-            core.propose_rule_from_nl("grant 3 credits on task complete".into()).expect("nl");
(B[m[32m+        let summary = core
(B[m[32m+            .propose_rule_from_nl("grant 3 credits on task complete".into())
(B[m[32m+            .expect("nl");
(B[m         assert_eq!(summary.name, "FFI Rule");
         assert_eq!(summary.priority, 7);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2941:
     #[test]
     fn connect_canvas_rejects_bogus_instance_url() {
         let (_d, core) = mk_core();
[31m-        let err =
(B[m[31m-            core.connector().connect_canvas("not-a-host".into(), "the-code".into()).unwrap_err();
(B[m[32m+        let err = core
(B[m[32m+            .connector()
(B[m[32m+            .connect_canvas("not-a-host".into(), "the-code".into())
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, FfiError::InvalidArgument(_)));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2966:
 
         impl CalendarHost for MockCalendarHost {
             fn list_events(&self, start_iso: String, end_iso: String) -> Vec<CalendarEventDto> {
[31m-                self.calls.lock().unwrap().push((start_iso.clone(), end_iso.clone()));
(B[m[32m+                self.calls
(B[m[32m+                    .lock()
(B[m[32m+                    .unwrap()
(B[m[32m+                    .push((start_iso.clone(), end_iso.clone()));
(B[m                 vec![
                     CalendarEventDto {
                         id: "e-2".into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:2986:
             }
         }
 
[31m-        let host = Arc::new(MockCalendarHost { calls: StdMutex::new(Vec::new()) });
(B[m[32m+        let host = Arc::new(MockCalendarHost {
(B[m[32m+            calls: StdMutex::new(Vec::new()),
(B[m[32m+        });
(B[m         let port = HostBackedCalendarPort::new(host.clone());
         let runtime = tokio::runtime::Runtime::new().unwrap();
[31m-        let start =
(B[m[31m-            DateTime::parse_from_rfc3339("2026-05-01T08:00:00+00:00").unwrap().with_timezone(&Utc);
(B[m[31m-        let end =
(B[m[31m-            DateTime::parse_from_rfc3339("2026-05-01T18:00:00+00:00").unwrap().with_timezone(&Utc);
(B[m[32m+        let start = DateTime::parse_from_rfc3339("2026-05-01T08:00:00+00:00")
(B[m[32m+            .unwrap()
(B[m[32m+            .with_timezone(&Utc);
(B[m[32m+        let end = DateTime::parse_from_rfc3339("2026-05-01T18:00:00+00:00")
(B[m[32m+            .unwrap()
(B[m[32m+            .with_timezone(&Utc);
(B[m         let events = runtime
             .block_on(async move { port.list_events(CoreDateRange::new(start, end)).await })
             .expect("list_events");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:3069:
         let (_d, core) = mk_core();
         let api = core.tasks();
         let mut bad = sample_input("");
[31m-        assert!(matches!(api.add(bad.clone()), Err(FfiError::InvalidArgument(_))));
(B[m[32m+        assert!(matches!(
(B[m[32m+            api.add(bad.clone()),
(B[m[32m+            Err(FfiError::InvalidArgument(_))
(B[m[32m+        ));
(B[m         bad.title = "ok".into();
         bad.duration_minutes = 0;
[31m-        assert!(matches!(api.add(bad.clone()), Err(FfiError::InvalidArgument(_))));
(B[m[32m+        assert!(matches!(
(B[m[32m+            api.add(bad.clone()),
(B[m[32m+            Err(FfiError::InvalidArgument(_))
(B[m[32m+        ));
(B[m         bad.duration_minutes = 25;
         bad.deadline_rigidity = "nope".into();
[31m-        assert!(matches!(api.add(bad.clone()), Err(FfiError::InvalidArgument(_))));
(B[m[32m+        assert!(matches!(
(B[m[32m+            api.add(bad.clone()),
(B[m[32m+            Err(FfiError::InvalidArgument(_))
(B[m[32m+        ));
(B[m         bad.deadline_rigidity = "hard".into();
         bad.deadline_iso = Some("not-a-date".into());
         assert!(matches!(api.add(bad), Err(FfiError::InvalidArgument(_))));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:3123:
         let (_d, core) = mk_core();
         let api = core.templates();
         let packs = api.list_bundled();
[31m-        assert!(packs.len() >= 4, "expected ≥4 bundled packs, got {}", packs.len());
(B[m[32m+        assert!(
(B[m[32m+            packs.len() >= 4,
(B[m[32m+            "expected ≥4 bundled packs, got {}",
(B[m[32m+            packs.len()
(B[m[32m+        );
(B[m         let ids: Vec<_> = packs.iter().map(|p| p.id.as_str()).collect();
[31m-        for expected in ["deep-work-starter", "student-canvas", "dev-flow", "sleep-hygiene"] {
(B[m[32m+        for expected in [
(B[m[32m+            "deep-work-starter",
(B[m[32m+            "student-canvas",
(B[m[32m+            "dev-flow",
(B[m[32m+            "sleep-hygiene",
(B[m[32m+        ] {
(B[m             assert!(ids.contains(&expected), "missing bundled pack: {expected}");
         }
         // Summaries carry meaningful metadata.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:3132:
[31m-        let deep = packs.iter().find(|p| p.id == "deep-work-starter").expect("deep-work");
(B[m[32m+        let deep = packs
(B[m[32m+            .iter()
(B[m[32m+            .find(|p| p.id == "deep-work-starter")
(B[m[32m+            .expect("deep-work");
(B[m         assert!(!deep.name.is_empty());
         assert!(deep.rule_count >= 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:3164:
         assert!(
             recent.iter().any(|r| r.record_type == "host.event.emitted"),
             "expected host.event.emitted audit record, got {:?}",
[31m-            recent.iter().map(|r| r.record_type.clone()).collect::<Vec<_>>()
(B[m[32m+            recent
(B[m[32m+                .iter()
(B[m[32m+                .map(|r| r.record_type.clone())
(B[m[32m+                .collect::<Vec<_>>()
(B[m         );
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:3230:
         let before_ids: std::collections::HashSet<String> =
             before.iter().map(|r| r.id.clone()).collect();
 
[31m-        let n = core.templates().install("deep-work-starter".into()).expect("install");
(B[m[32m+        let n = core
(B[m[32m+            .templates()
(B[m[32m+            .install("deep-work-starter".into())
(B[m[32m+            .expect("install");
(B[m         assert!(n >= 1, "expected ≥1 rule installed, got {n}");
 
         let after = core.rules().list_enabled().expect("list after");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/src/lib.rs:3237:
         // Exactly `n` new rule ids should be present after install.
[31m-        let new_ids: Vec<_> = after.iter().filter(|r| !before_ids.contains(&r.id)).collect();
(B[m[31m-        assert_eq!(new_ids.len() as u32, n, "install count must match persisted delta");
(B[m[32m+        let new_ids: Vec<_> = after
(B[m[32m+            .iter()
(B[m[32m+            .filter(|r| !before_ids.contains(&r.id))
(B[m[32m+            .collect();
(B[m[32m+        assert_eq!(
(B[m[32m+            new_ids.len() as u32,
(B[m[32m+            n,
(B[m[32m+            "install count must match persisted delta"
(B[m[32m+        );
(B[m 
         // Idempotent: re-installing the same pack does not create duplicates.
[31m-        let n2 = core.templates().install("deep-work-starter".into()).expect("reinstall");
(B[m[32m+        let n2 = core
(B[m[32m+            .templates()
(B[m[32m+            .install("deep-work-starter".into())
(B[m[32m+            .expect("reinstall");
(B[m         assert_eq!(n2, n);
         let after2 = core.rules().list_enabled().expect("list after2");
[31m-        assert_eq!(after2.len(), after.len(), "reinstall must upsert, not duplicate");
(B[m[32m+        assert_eq!(
(B[m[32m+            after2.len(),
(B[m[32m+            after.len(),
(B[m[32m+            "reinstall must upsert, not duplicate"
(B[m[32m+        );
(B[m 
         // Audit row recorded.
         let recent = core.audit().recent(8).expect("audit recent");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ffi/tests/connector_registration.rs:53:
 
         // 4. Orchestrator now holds a live handle for the canvas connector.
         let handles = core.sync().connectors();
[31m-        assert_eq!(handles.len(), 1, "expected exactly one registered connector");
(B[m[32m+        assert_eq!(
(B[m[32m+            handles.len(),
(B[m[32m+            1,
(B[m[32m+            "expected exactly one registered connector"
(B[m[32m+        );
(B[m         assert_eq!(
             handles[0].connector_id, "canvas",
             "registered id must match CanvasConnector::manifest().id"
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-hash/src/lib.rs:18:
 
 use focus_errors::FocusError;
 use focus_result::FocusResult;
[31m-use phenotype_crypto::{Hasher, Hash as CryptoHash};
(B[m[32m+use phenotype_crypto::{Hash as CryptoHash, Hasher};
(B[m 
 pub use phenotype_crypto::HashAlgorithm;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-icon-gen/src/bin.rs:6:
 #[derive(Parser)]
 #[command(name = "focalpoint-icon-gen")]
 #[command(about = "Generate FocalPoint app icons for App Store submission")]
[31m-#[command(long_about = "Procedural icon generator: Coachy flame silhouette with gradient background. Renders all required iOS sizes and generates XCAssets Contents.json manifest.")]
(B[m[32m+#[command(
(B[m[32m+    long_about = "Procedural icon generator: Coachy flame silhouette with gradient background. Renders all required iOS sizes and generates XCAssets Contents.json manifest."
(B[m[32m+)]
(B[m struct Args {
     /// Output directory for generated icons (default: ../../apps/ios/FocalPoint/Sources/FocalPointApp/Resources/Assets.xcassets/AppIcon.appiconset/)
     #[arg(short, long)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-icon-gen/src/lib.rs:311:
 
         // Verify each PNG is non-empty and valid
         for (size, _name, png_data) in &sizes {
[31m-            assert!(!png_data.is_empty(), "PNG for size {} must not be empty", size);
(B[m             assert!(
[32m+                !png_data.is_empty(),
(B[m[32m+                "PNG for size {} must not be empty",
(B[m[32m+                size
(B[m[32m+            );
(B[m[32m+            assert!(
(B[m                 png_data.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]),
                 "PNG for size {} must have valid PNG signature",
                 size
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-icon-gen/src/lib.rs:324:
     #[test]
     fn test_contents_json_valid() {
         let gen = IconGenerator::new();
[31m-        let json_str = gen.generate_contents_json().expect("Generate Contents.json");
(B[m[31m-        let parsed: serde_json::Value = serde_json::from_str(&json_str)
(B[m[31m-            .expect("Contents.json must be valid JSON");
(B[m[32m+        let json_str = gen
(B[m[32m+            .generate_contents_json()
(B[m[32m+            .expect("Generate Contents.json");
(B[m[32m+        let parsed: serde_json::Value =
(B[m[32m+            serde_json::from_str(&json_str).expect("Contents.json must be valid JSON");
(B[m 
         assert!(parsed["images"].is_array(), "images field must be array");
[31m-        assert!(parsed["info"]["version"].is_number(), "info.version must be present");
(B[m[32m+        assert!(
(B[m[32m+            parsed["info"]["version"].is_number(),
(B[m[32m+            "info.version must be present"
(B[m[32m+        );
(B[m 
         let images = parsed["images"].as_array().expect("images array");
         assert!(!images.is_empty(), "images array must not be empty");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/benches/ir_hash.rs:30:
                 "created_at": "2026-04-23T00:00:00Z"
             }
         })
[31m-        .to_string()
(B[m[32m+        .to_string(),
(B[m     );
 
     c.bench_function("content_hash_small_document", |b| {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/benches/ir_hash.rs:68:
             "namespace": "focus.rules.ir",
             "rules": rules
         })
[31m-        .to_string()
(B[m[32m+        .to_string(),
(B[m     );
 
     c.bench_function("content_hash_1000_rule_document", |b| {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:109:
     }
 
     // Trigger as JSON
[31m-    let trigger_json = serde_json::to_string(&ir.trigger)
(B[m[31m-        .unwrap_or_else(|_| "{}".to_string());
(B[m[31m-    write!(&mut output, " --trigger '{}'", escape_single_quote(&trigger_json)).unwrap();
(B[m[32m+    let trigger_json = serde_json::to_string(&ir.trigger).unwrap_or_else(|_| "{}".to_string());
(B[m[32m+    write!(
(B[m[32m+        &mut output,
(B[m[32m+        " --trigger '{}'",
(B[m[32m+        escape_single_quote(&trigger_json)
(B[m[32m+    )
(B[m[32m+    .unwrap();
(B[m 
     // Conditions as JSON array
[31m-    let conditions_json = serde_json::to_string(&ir.conditions)
(B[m[31m-        .unwrap_or_else(|_| "[]".to_string());
(B[m[32m+    let conditions_json =
(B[m[32m+        serde_json::to_string(&ir.conditions).unwrap_or_else(|_| "[]".to_string());
(B[m     write!(
         &mut output,
         " --conditions '{}'",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:124:
     .unwrap();
 
     // Actions as JSON array
[31m-    let actions_json = serde_json::to_string(&ir.actions)
(B[m[31m-        .unwrap_or_else(|_| "[]".to_string());
(B[m[32m+    let actions_json = serde_json::to_string(&ir.actions).unwrap_or_else(|_| "[]".to_string());
(B[m     write!(
         &mut output,
         " --actions '{}'",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:251:
             .unwrap();
         }
         ConditionIr::DayOfWeek { days } => {
[31m-            let day_list = days.iter().map(|d| format!("\"{}\"", d)).collect::<Vec<_>>().join(", ");
(B[m[32m+            let day_list = days
(B[m[32m+                .iter()
(B[m[32m+                .map(|d| format!("\"{}\"", d))
(B[m[32m+                .collect::<Vec<_>>()
(B[m[32m+                .join(", ");
(B[m             writeln!(output, "{}day_of_week(days = [{}])", ind, day_list).unwrap();
         }
         ConditionIr::UserAttribute { key, value } => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:265:
             .unwrap();
         }
         ConditionIr::EventProperty { property, expected } => {
[31m-            let expected_str = serde_json::to_string(&expected)
(B[m[31m-                .unwrap_or_else(|_| "null".to_string());
(B[m[32m+            let expected_str =
(B[m[32m+                serde_json::to_string(&expected).unwrap_or_else(|_| "null".to_string());
(B[m             writeln!(
                 output,
                 "{}event_property(property = \"{}\", expected = {})",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:277:
             .unwrap();
         }
         ConditionIr::CustomPredicate { name, args } => {
[31m-            let args_str = serde_json::to_string(&args)
(B[m[31m-                .unwrap_or_else(|_| "{}".to_string());
(B[m[32m+            let args_str = serde_json::to_string(&args).unwrap_or_else(|_| "{}".to_string());
(B[m             writeln!(
                 output,
                 "{}custom_predicate(name = \"{}\", args = {})",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:297:
 
     match action {
         ActionIr::EnforcePolicy { policy_id, params } => {
[31m-            write!(output, "{}enforce_policy(policy_id = \"{}\"", ind, escape_string(policy_id))
(B[m[31m-                .unwrap();
(B[m[32m+            write!(
(B[m[32m+                output,
(B[m[32m+                "{}enforce_policy(policy_id = \"{}\"",
(B[m[32m+                ind,
(B[m[32m+                escape_string(policy_id)
(B[m[32m+            )
(B[m[32m+            .unwrap();
(B[m             if !params.is_empty() {
                 write!(output, ", params = {{").unwrap();
                 for (k, v) in params {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:313:
             event_type,
             payload,
         } => {
[31m-            write!(output, "{}emit_event(event_type = \"{}\"", ind, escape_string(event_type))
(B[m[31m-                .unwrap();
(B[m[32m+            write!(
(B[m[32m+                output,
(B[m[32m+                "{}emit_event(event_type = \"{}\"",
(B[m[32m+                ind,
(B[m[32m+                escape_string(event_type)
(B[m[32m+            )
(B[m[32m+            .unwrap();
(B[m             if !payload.is_empty() {
                 write!(output, ", payload = {{").unwrap();
                 for (k, v) in payload {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:351:
             delay_ms,
             params,
         } => {
[31m-            write!(output, "{}schedule_task(task_id = \"{}\"", ind, escape_string(task_id))
(B[m[31m-                .unwrap();
(B[m[32m+            write!(
(B[m[32m+                output,
(B[m[32m+                "{}schedule_task(task_id = \"{}\"",
(B[m[32m+                ind,
(B[m[32m+                escape_string(task_id)
(B[m[32m+            )
(B[m[32m+            .unwrap();
(B[m             if let Some(delay) = delay_ms {
                 write!(output, ", delay_ms = {}", delay).unwrap();
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/codegen.rs:511:
                         end_hour: 17,
                     },
                     ConditionIr::DayOfWeek {
[31m-                        days: vec![
(B[m[31m-                            "Mon".to_string(),
(B[m[31m-                            "Tue".to_string(),
(B[m[31m-                            "Wed".to_string(),
(B[m[31m-                        ],
(B[m[32m+                        days: vec!["Mon".to_string(), "Tue".to_string(), "Wed".to_string()],
(B[m                     },
                 ],
             }],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:140:
     WebhookReceived { path: String, method: String },
 
     #[serde(rename = "UserAction")]
[31m-    UserAction {
(B[m[31m-        action_type: String,
(B[m[31m-        target: String,
(B[m[31m-    },
(B[m[32m+    UserAction { action_type: String, target: String },
(B[m 
     #[serde(rename = "ConditionMet")]
     ConditionMet { condition: Box<ConditionIr> },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:417:
     pub id: String,
     pub name: String,
     pub character: String, // "default", "mentor", "cheerleader"
[31m-    pub pose: String, // "neutral", "thumbs_up", "thinking", "excited"
(B[m[31m-    pub emotion: String, // "happy", "neutral", "sad", "confused"
(B[m[32m+    pub pose: String,      // "neutral", "thumbs_up", "thinking", "excited"
(B[m[32m+    pub emotion: String,   // "happy", "neutral", "sad", "confused"
(B[m     #[serde(skip_serializing_if = "Option::is_none")]
     pub accessory: Option<String>, // "glasses", "hat", "none"
     #[serde(skip_serializing_if = "Option::is_none")]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:462:
 pub struct CoachingConfigIr {
     pub id: String,
     pub name: String,
[31m-    pub tone: String, // "encouraging", "neutral", "challenging", "humorous"
(B[m[32m+    pub tone: String,     // "encouraging", "neutral", "challenging", "humorous"
(B[m     pub language: String, // "en", "es", "fr"
     #[serde(skip_serializing_if = "Option::is_none")]
     pub voice_profile: Option<VoiceProfileIr>,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:697:
 #[cfg(test)]
 mod tests {
     use super::*;
[31m-    use uuid::Uuid;
(B[m     use chrono::Timelike;
[32m+    use uuid::Uuid;
(B[m 
     // Round-Trip Conversions (focus_rules::Rule <-> RuleIr)
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:709:
             id: rule.id.to_string(),
             name: rule.name.clone(),
             trigger: trigger_to_ir(&rule.trigger),
[31m-            conditions: rule
(B[m[31m-                .conditions
(B[m[31m-                .iter()
(B[m[31m-                .map(condition_to_ir)
(B[m[31m-                .collect(),
(B[m[32m+            conditions: rule.conditions.iter().map(condition_to_ir).collect(),
(B[m             actions: rule.actions.iter().map(action_to_ir).collect(),
             priority: rule.priority,
             cooldown_seconds: rule.cooldown.map(|d| d.num_seconds()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:727:
     #[allow(dead_code)]
     pub fn ir_to_rule(ir: &RuleIr) -> Result<focus_rules::Rule, FocusError> {
         Ok(focus_rules::Rule {
[31m-            id: Uuid::parse_str(&ir.id)
(B[m[31m-                .map_err(|_| FocusError::invalid_input("document","Invalid rule ID UUID".to_string()))?,
(B[m[32m+            id: Uuid::parse_str(&ir.id).map_err(|_| {
(B[m[32m+                FocusError::invalid_input("document", "Invalid rule ID UUID".to_string())
(B[m[32m+            })?,
(B[m             name: ir.name.clone(),
             trigger: ir_to_trigger(&ir.trigger)?,
             conditions: ir
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:781:
             } if action_type == "state_change" => {
                 Ok(focus_rules::Trigger::StateChange(target.clone()))
             }
[31m-            _ => Err(FocusError::invalid_input("document",
(B[m[32m+            _ => Err(FocusError::invalid_input(
(B[m[32m+                "document",
(B[m                 "Unsupported trigger type".to_string(),
             )),
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:802:
                 kind: name.clone(),
                 params: args.clone(),
             }),
[31m-            _ => Err(FocusError::invalid_input("document",
(B[m[32m+            _ => Err(FocusError::invalid_input(
(B[m[32m+                "document",
(B[m                 "Complex conditions not yet supported in round-trip".to_string(),
             )),
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:815:
                 event_type: "grant_credit".to_string(),
                 payload: {
                     let mut m = BTreeMap::new();
[31m-                    m.insert("amount".to_string(), serde_json::Value::Number((*amount).into()));
(B[m[32m+                    m.insert(
(B[m[32m+                        "amount".to_string(),
(B[m[32m+                        serde_json::Value::Number((*amount).into()),
(B[m[32m+                    );
(B[m                     m
                 },
             },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:823:
                 event_type: "deduct_credit".to_string(),
                 payload: {
                     let mut m = BTreeMap::new();
[31m-                    m.insert("amount".to_string(), serde_json::Value::Number((*amount).into()));
(B[m[32m+                    m.insert(
(B[m[32m+                        "amount".to_string(),
(B[m[32m+                        serde_json::Value::Number((*amount).into()),
(B[m[32m+                    );
(B[m                     m
                 },
             },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:836:
                 params: {
                     let mut m = BTreeMap::new();
                     m.insert("profile".to_string(), serde_json::json!(profile));
[31m-                    m.insert("duration_secs".to_string(), serde_json::json!(duration.num_seconds()));
(B[m[31m-                    m.insert("rigidity".to_string(), serde_json::json!(format!("{:?}", rigidity)));
(B[m[32m+                    m.insert(
(B[m[32m+                        "duration_secs".to_string(),
(B[m[32m+                        serde_json::json!(duration.num_seconds()),
(B[m[32m+                    );
(B[m[32m+                    m.insert(
(B[m[32m+                        "rigidity".to_string(),
(B[m[32m+                        serde_json::json!(format!("{:?}", rigidity)),
(B[m[32m+                    );
(B[m                     m
                 },
             },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:883:
                         "profiles".to_string(),
                         serde_json::json!(profiles.iter().collect::<Vec<_>>()),
                     );
[31m-                    m.insert("duration_secs".to_string(), serde_json::json!(duration.num_seconds()));
(B[m[32m+                    m.insert(
(B[m[32m+                        "duration_secs".to_string(),
(B[m[32m+                        serde_json::json!(duration.num_seconds()),
(B[m[32m+                    );
(B[m                     m.insert("bypass_cost".to_string(), serde_json::json!(bypass_cost));
                     m.insert("reason".to_string(), serde_json::json!(reason));
                     m
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:908:
                 params: {
                     let mut m = BTreeMap::new();
                     m.insert("profile".to_string(), serde_json::json!(profile));
[31m-                    m.insert("starts_at".to_string(), serde_json::json!(starts_at.to_rfc3339()));
(B[m[31m-                    m.insert("ends_at".to_string(), serde_json::json!(ends_at.to_rfc3339()));
(B[m[32m+                    m.insert(
(B[m[32m+                        "starts_at".to_string(),
(B[m[32m+                        serde_json::json!(starts_at.to_rfc3339()),
(B[m[32m+                    );
(B[m[32m+                    m.insert(
(B[m[32m+                        "ends_at".to_string(),
(B[m[32m+                        serde_json::json!(ends_at.to_rfc3339()),
(B[m[32m+                    );
(B[m                     m.insert("credit_cost".to_string(), serde_json::json!(credit_cost));
                     m
                 },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:924:
             ActionIr::EmitEvent {
                 event_type,
                 payload,
[31m-            } => {
(B[m[31m-                match event_type.as_str() {
(B[m[31m-                    "grant_credit" => {
(B[m[31m-                        let amount = payload
(B[m[31m-                            .get("amount")
(B[m[31m-                            .and_then(|v| v.as_i64())
(B[m[31m-                            .unwrap_or(0) as i32;
(B[m[31m-                        Ok(focus_rules::Action::GrantCredit { amount })
(B[m[31m-                    }
(B[m[31m-                    "deduct_credit" => {
(B[m[31m-                        let amount = payload
(B[m[31m-                            .get("amount")
(B[m[31m-                            .and_then(|v| v.as_i64())
(B[m[31m-                            .unwrap_or(0) as i32;
(B[m[31m-                        Ok(focus_rules::Action::DeductCredit { amount })
(B[m[31m-                    }
(B[m[31m-                    _ => Err(FocusError::invalid_input("document","Unknown event type".to_string())),
(B[m[32m+            } => match event_type.as_str() {
(B[m[32m+                "grant_credit" => {
(B[m[32m+                    let amount = payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
(B[m[32m+                    Ok(focus_rules::Action::GrantCredit { amount })
(B[m                 }
[31m-            }
(B[m[31m-            _ => Err(FocusError::invalid_input("document",
(B[m[32m+                "deduct_credit" => {
(B[m[32m+                    let amount = payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
(B[m[32m+                    Ok(focus_rules::Action::DeductCredit { amount })
(B[m[32m+                }
(B[m[32m+                _ => Err(FocusError::invalid_input(
(B[m[32m+                    "document",
(B[m[32m+                    "Unknown event type".to_string(),
(B[m[32m+                )),
(B[m[32m+            },
(B[m[32m+            _ => Err(FocusError::invalid_input(
(B[m[32m+                "document",
(B[m                 "Action type not yet supported in IR->Rule conversion".to_string(),
             )),
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1008:
         }
         let hash2 = doc.content_hash().expect("Second hash");
 
[31m-        assert_ne!(hash1, hash2, "Content hash must change when document changes");
(B[m[32m+        assert_ne!(
(B[m[32m+            hash1, hash2,
(B[m[32m+            "Content hash must change when document changes"
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1104:
                         end_hour: 17,
                     },
                     ConditionIr::DayOfWeek {
[31m-                        days: vec!["Monday".to_string(), "Tuesday".to_string(), "Wednesday".to_string()],
(B[m[32m+                        days: vec![
(B[m[32m+                            "Monday".to_string(),
(B[m[32m+                            "Tuesday".to_string(),
(B[m[32m+                            "Wednesday".to_string(),
(B[m[32m+                        ],
(B[m                     },
                 ],
             }],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1145:
             version: "1.0".to_string(),
             display_name: "Test".to_string(),
             auth_strategy: AuthStrategyIr::None,
[31m-            sync_mode: SyncModeIr::Polling { cadence_seconds: 60 },
(B[m[32m+            sync_mode: SyncModeIr::Polling {
(B[m[32m+                cadence_seconds: 60,
(B[m[32m+            },
(B[m             capabilities: vec![],
             entity_types: vec![],
             event_types: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1232:
     #[allow(dead_code)]
     pub fn ir_to_task(ir: &TaskIr) -> Result<focus_planning::Task, FocusError> {
         Ok(focus_planning::Task {
[31m-            id: Uuid::parse_str(&ir.id)
(B[m[31m-                .map_err(|_| FocusError::invalid_input("document","Invalid task ID UUID".to_string()))?,
(B[m[32m+            id: Uuid::parse_str(&ir.id).map_err(|_| {
(B[m[32m+                FocusError::invalid_input("document", "Invalid task ID UUID".to_string())
(B[m[32m+            })?,
(B[m             title: ir.title.clone(),
             duration: ir_to_duration_spec(&ir.duration_spec)?,
             priority: focus_planning::Priority::clamped(ir.priority_weight),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1260:
         }
     }
 
[31m-    fn ir_to_duration_spec(ir: &DurationSpecIr) -> Result<focus_planning::DurationSpec, FocusError> {
(B[m[32m+    fn ir_to_duration_spec(
(B[m[32m+        ir: &DurationSpecIr,
(B[m[32m+    ) -> Result<focus_planning::DurationSpec, FocusError> {
(B[m         Ok(focus_planning::DurationSpec {
             fixed: ir.fixed_minutes.map(chrono::Duration::minutes),
             estimate: ir.estimate.as_ref().map(|e| focus_planning::Estimate {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1285:
                     None => None,
                     Some(s) => Some(
                         chrono::DateTime::parse_from_rfc3339(s)
[31m-                            .map_err(|_| FocusError::invalid_input("document","Invalid ISO8601 datetime".to_string()))?
(B[m[32m+                            .map_err(|_| {
(B[m[32m+                                FocusError::invalid_input(
(B[m[32m+                                    "document",
(B[m[32m+                                    "Invalid ISO8601 datetime".to_string(),
(B[m[32m+                                )
(B[m[32m+                            })?
(B[m                             .with_timezone(&chrono::Utc),
                     ),
                 };
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1352:
                 end_hour,
                 days,
             } => {
[31m-                let start = chrono::NaiveTime::from_hms_opt(*start_hour as u32, 0, 0)
(B[m[31m-                    .ok_or_else(|| FocusError::invalid_input("document","Invalid start hour".to_string()))?;
(B[m[31m-                let end = chrono::NaiveTime::from_hms_opt(*end_hour as u32, 0, 0)
(B[m[31m-                    .ok_or_else(|| FocusError::invalid_input("document","Invalid end hour".to_string()))?;
(B[m[32m+                let start =
(B[m[32m+                    chrono::NaiveTime::from_hms_opt(*start_hour as u32, 0, 0).ok_or_else(|| {
(B[m[32m+                        FocusError::invalid_input("document", "Invalid start hour".to_string())
(B[m[32m+                    })?;
(B[m[32m+                let end =
(B[m[32m+                    chrono::NaiveTime::from_hms_opt(*end_hour as u32, 0, 0).ok_or_else(|| {
(B[m[32m+                        FocusError::invalid_input("document", "Invalid end hour".to_string())
(B[m[32m+                    })?;
(B[m                 let days_parsed = days
                     .iter()
                     .filter_map(|s| match s.as_str() {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1377:
             }
             ConstraintIr::NoEarlierThan { when_iso8601 } => {
                 let dt = chrono::DateTime::parse_from_rfc3339(when_iso8601)
[31m-                    .map_err(|_| FocusError::invalid_input("document","Invalid ISO8601 datetime".to_string()))?
(B[m[32m+                    .map_err(|_| {
(B[m[32m+                        FocusError::invalid_input(
(B[m[32m+                            "document",
(B[m[32m+                            "Invalid ISO8601 datetime".to_string(),
(B[m[32m+                        )
(B[m[32m+                    })?
(B[m                     .with_timezone(&chrono::Utc);
                 Ok(focus_planning::Constraint::NoEarlierThan(dt))
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1384:
             ConstraintIr::NoLaterThan { when_iso8601 } => {
                 let dt = chrono::DateTime::parse_from_rfc3339(when_iso8601)
[31m-                    .map_err(|_| FocusError::invalid_input("document","Invalid ISO8601 datetime".to_string()))?
(B[m[32m+                    .map_err(|_| {
(B[m[32m+                        FocusError::invalid_input(
(B[m[32m+                            "document",
(B[m[32m+                            "Invalid ISO8601 datetime".to_string(),
(B[m[32m+                        )
(B[m[32m+                    })?
(B[m                     .with_timezone(&chrono::Utc);
                 Ok(focus_planning::Constraint::NoLaterThan(dt))
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1390:
[31m-            ConstraintIr::Buffer { duration_minutes } => {
(B[m[31m-                Ok(focus_planning::Constraint::Buffer(
(B[m[31m-                    chrono::Duration::minutes(*duration_minutes),
(B[m[31m-                ))
(B[m[31m-            }
(B[m[32m+            ConstraintIr::Buffer { duration_minutes } => Ok(focus_planning::Constraint::Buffer(
(B[m[32m+                chrono::Duration::minutes(*duration_minutes),
(B[m[32m+            )),
(B[m             ConstraintIr::EnergyTier { tier } => {
                 let energy = match tier.as_str() {
                     "DeepFocus" => focus_planning::EnergyTier::DeepFocus,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1431:
                 let parsed = chunks
                     .iter()
                     .map(|tb| {
[31m-                        let task_id = Uuid::parse_str(&tb.task_id)
(B[m[31m-                            .map_err(|_| FocusError::invalid_input("document","Invalid task ID in chunk".to_string()))?;
(B[m[32m+                        let task_id = Uuid::parse_str(&tb.task_id).map_err(|_| {
(B[m[32m+                            FocusError::invalid_input(
(B[m[32m+                                "document",
(B[m[32m+                                "Invalid task ID in chunk".to_string(),
(B[m[32m+                            )
(B[m[32m+                        })?;
(B[m                         let starts_at = chrono::DateTime::parse_from_rfc3339(&tb.starts_at_iso8601)
[31m-                            .map_err(|_| FocusError::invalid_input("document","Invalid start timestamp".to_string()))?
(B[m[32m+                            .map_err(|_| {
(B[m[32m+                                FocusError::invalid_input(
(B[m[32m+                                    "document",
(B[m[32m+                                    "Invalid start timestamp".to_string(),
(B[m[32m+                                )
(B[m[32m+                            })?
(B[m                             .with_timezone(&chrono::Utc);
                         let ends_at = chrono::DateTime::parse_from_rfc3339(&tb.ends_at_iso8601)
[31m-                            .map_err(|_| FocusError::invalid_input("document","Invalid end timestamp".to_string()))?
(B[m[32m+                            .map_err(|_| {
(B[m[32m+                                FocusError::invalid_input(
(B[m[32m+                                    "document",
(B[m[32m+                                    "Invalid end timestamp".to_string(),
(B[m[32m+                                )
(B[m[32m+                            })?
(B[m                             .with_timezone(&chrono::Utc);
                         let rigidity = match tb.rigidity.as_str() {
                             "Hard" => focus_domain::Rigidity::Hard,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1576:
         };
 
         let json = serde_json::to_string(&doc).expect("Serialize Schedule document");
[31m-        let restored: Document = serde_json::from_str(&json).expect("Deserialize Schedule document");
(B[m[32m+        let restored: Document =
(B[m[32m+            serde_json::from_str(&json).expect("Deserialize Schedule document");
(B[m 
         match &restored.body {
             Body::Schedule(s) => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/src/lib.rs:1635:
             auth_strategy: AuthStrategyIr::OAuth2 {
                 scopes: vec!["repo".to_string(), "user".to_string()],
             },
[31m-            sync_mode: SyncModeIr::Polling { cadence_seconds: 3600 },
(B[m[32m+            sync_mode: SyncModeIr::Polling {
(B[m[32m+                cadence_seconds: 3600,
(B[m[32m+            },
(B[m             capabilities: vec![ConnectorCapabilityIr {
                 name: "fetch_issues".to_string(),
                 params_schema: serde_json::json!({"owner": "string"}),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/tests/differential.rs:47:
 
         // Serialize and deserialize
         let json = serde_json::to_string(&rule).expect("serialize");
[31m-        let deserialized: RuleIr =
(B[m[31m-            serde_json::from_str(&json).expect("deserialize");
(B[m[32m+        let deserialized: RuleIr = serde_json::from_str(&json).expect("deserialize");
(B[m 
         let hash2 = canonical_hash(&deserialized);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ir/tests/differential.rs:137:
         };
 
         let json = serde_json::to_string(&rule).expect("serialize");
[31m-        let deserialized: RuleIr =
(B[m[31m-            serde_json::from_str(&json).expect("deserialize");
(B[m[32m+        let deserialized: RuleIr = serde_json::from_str(&json).expect("deserialize");
(B[m 
         let hash_original = canonical_hash(&rule);
         let hash_roundtrip = canonical_hash(&deserialized);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/benches/starlark_compile.rs:108:
 
             // Pass 2: AST construction
             let rule_count = large_program.matches("rule ").count();
[31m-            let condition_ops = large_program.matches("all_of").count()
(B[m[31m-                + large_program.matches("any_of").count();
(B[m[32m+            let condition_ops =
(B[m[32m+                large_program.matches("all_of").count() + large_program.matches("any_of").count();
(B[m             let action_count = large_program.matches("grant_credit").count();
             let metadata_count = large_program.matches("metadata:").count();
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/benches/starlark_compile.rs:120:
                 && action_count > 0
                 && metadata_count > 0;
 
[31m-            black_box((rule_count, token_count, condition_ops, action_count, metadata_count))
(B[m[32m+            black_box((
(B[m[32m+                rule_count,
(B[m[32m+                token_count,
(B[m[32m+                condition_ops,
(B[m[32m+                action_count,
(B[m[32m+                metadata_count,
(B[m[32m+            ))
(B[m         });
     });
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:165:
         }
     }
 
[31m-    Ok(BulkRuleImport { rules, validation_report })
(B[m[32m+    Ok(BulkRuleImport {
(B[m[32m+        rules,
(B[m[32m+        validation_report,
(B[m[32m+    })
(B[m }
 
 /// Parse YAML file of rules.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:228:
         }
     }
 
[31m-    Ok(BulkTaskImport { tasks, validation_report })
(B[m[32m+    Ok(BulkTaskImport {
(B[m[32m+        tasks,
(B[m[32m+        validation_report,
(B[m[32m+    })
(B[m }
 
 /// Parse YAML file of tasks.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:260:
 }
 
 /// Validate a CSV rule record and convert to YAML format.
[31m-fn validate_rule_record(rec: &RuleCsvRecord, row_idx: usize) -> Result<RuleYamlRecord, ValidationError> {
(B[m[32m+fn validate_rule_record(
(B[m[32m+    rec: &RuleCsvRecord,
(B[m[32m+    row_idx: usize,
(B[m[32m+) -> Result<RuleYamlRecord, ValidationError> {
(B[m     if rec.name.is_empty() {
         return Err(ValidationError {
             row_index: row_idx,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:273:
         return Err(ValidationError {
             row_index: row_idx,
             field: "trigger_kind".to_string(),
[31m-            reason: format!("unknown trigger: {} (valid: {:?})", rec.trigger_kind, VALID_TRIGGERS),
(B[m[32m+            reason: format!(
(B[m[32m+                "unknown trigger: {} (valid: {:?})",
(B[m[32m+                rec.trigger_kind, VALID_TRIGGERS
(B[m[32m+            ),
(B[m         });
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:281:
         return Err(ValidationError {
             row_index: row_idx,
             field: "action_kind".to_string(),
[31m-            reason: format!("unknown action: {} (valid: {:?})", rec.action_kind, VALID_ACTIONS),
(B[m[32m+            reason: format!(
(B[m[32m+                "unknown action: {} (valid: {:?})",
(B[m[32m+                rec.action_kind, VALID_ACTIONS
(B[m[32m+            ),
(B[m         });
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:299:
 }
 
 /// Validate a YAML rule record.
[31m-fn validate_rule_yaml(rec: &RuleYamlRecord, row_idx: usize) -> Result<RuleYamlRecord, ValidationError> {
(B[m[32m+fn validate_rule_yaml(
(B[m[32m+    rec: &RuleYamlRecord,
(B[m[32m+    row_idx: usize,
(B[m[32m+) -> Result<RuleYamlRecord, ValidationError> {
(B[m     if rec.name.is_empty() {
         return Err(ValidationError {
             row_index: row_idx,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:312:
         return Err(ValidationError {
             row_index: row_idx,
             field: "trigger_kind".to_string(),
[31m-            reason: format!("unknown trigger: {} (valid: {:?})", rec.trigger_kind, VALID_TRIGGERS),
(B[m[32m+            reason: format!(
(B[m[32m+                "unknown trigger: {} (valid: {:?})",
(B[m[32m+                rec.trigger_kind, VALID_TRIGGERS
(B[m[32m+            ),
(B[m         });
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:320:
         return Err(ValidationError {
             row_index: row_idx,
             field: "action_kind".to_string(),
[31m-            reason: format!("unknown action: {} (valid: {:?})", rec.action_kind, VALID_ACTIONS),
(B[m[32m+            reason: format!(
(B[m[32m+                "unknown action: {} (valid: {:?})",
(B[m[32m+                rec.action_kind, VALID_ACTIONS
(B[m[32m+            ),
(B[m         });
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:328:
 }
 
 /// Validate a CSV task record and convert to YAML format.
[31m-fn validate_task_record(rec: &TaskCsvRecord, row_idx: usize) -> Result<TaskYamlRecord, ValidationError> {
(B[m[32m+fn validate_task_record(
(B[m[32m+    rec: &TaskCsvRecord,
(B[m[32m+    row_idx: usize,
(B[m[32m+) -> Result<TaskYamlRecord, ValidationError> {
(B[m     if rec.title.is_empty() {
         return Err(ValidationError {
             row_index: row_idx,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:365:
 }
 
 /// Validate a YAML task record.
[31m-fn validate_task_yaml(rec: &TaskYamlRecord, row_idx: usize) -> Result<TaskYamlRecord, ValidationError> {
(B[m[32m+fn validate_task_yaml(
(B[m[32m+    rec: &TaskYamlRecord,
(B[m[32m+    row_idx: usize,
(B[m[32m+) -> Result<TaskYamlRecord, ValidationError> {
(B[m     if rec.title.is_empty() {
         return Err(ValidationError {
             row_index: row_idx,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:388:
 }
 
 /// Export rules to CSV format.
[31m-pub fn export_rules_csv<T: Into<String> + Clone>(
(B[m[31m-    rules: Vec<RuleCsvRow<T>>,
(B[m[31m-) -> BulkResult<String> {
(B[m[32m+pub fn export_rules_csv<T: Into<String> + Clone>(rules: Vec<RuleCsvRow<T>>) -> BulkResult<String> {
(B[m     let mut wtr = csv::Writer::from_writer(vec![]);
 
     wtr.write_record([
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:419:
         .map_err(|e| BulkError::CsvError(e.to_string()))?;
     }
 
[31m-    let data = wtr.into_inner().map_err(|e| BulkError::CsvError(e.to_string()))?;
(B[m[32m+    let data = wtr
(B[m[32m+        .into_inner()
(B[m[32m+        .map_err(|e| BulkError::CsvError(e.to_string()))?;
(B[m     String::from_utf8(data).map_err(|e| BulkError::CsvError(e.to_string()))
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:426:
 /// Task export tuple: (title, priority, deadline_str, duration_min, tags)
[31m-type TaskTuple<T> = (T, Option<f32>, Option<String>, Option<i32>, Option<Vec<String>>);
(B[m[32m+type TaskTuple<T> = (
(B[m[32m+    T,
(B[m[32m+    Option<f32>,
(B[m[32m+    Option<String>,
(B[m[32m+    Option<i32>,
(B[m[32m+    Option<Vec<String>>,
(B[m[32m+);
(B[m 
 /// Export tasks to CSV format.
[31m-pub fn export_tasks_csv<T: Into<String> + Clone>(
(B[m[31m-    tasks: Vec<TaskTuple<T>>,
(B[m[31m-) -> BulkResult<String> {
(B[m[32m+pub fn export_tasks_csv<T: Into<String> + Clone>(tasks: Vec<TaskTuple<T>>) -> BulkResult<String> {
(B[m     let mut wtr = csv::Writer::from_writer(vec![]);
 
     wtr.write_record(["title", "priority", "deadline", "duration_min", "tags"])
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:446:
         .map_err(|e| BulkError::CsvError(e.to_string()))?;
     }
 
[31m-    let data = wtr.into_inner().map_err(|e| BulkError::CsvError(e.to_string()))?;
(B[m[32m+    let data = wtr
(B[m[32m+        .into_inner()
(B[m[32m+        .map_err(|e| BulkError::CsvError(e.to_string()))?;
(B[m     String::from_utf8(data).map_err(|e| BulkError::CsvError(e.to_string()))
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/bulk.rs:522:
     fn test_parse_rules_csv_with_malformed_rows() {
         let dir = tempdir().unwrap();
         let file_path = dir.path().join("rules.csv");
[31m-        let csv_content = "name,trigger_kind,event_type,action_kind,amount,cooldown,priority,enabled\n\
(B[m[32m+        let csv_content =
(B[m[32m+            "name,trigger_kind,event_type,action_kind,amount,cooldown,priority,enabled\n\
(B[m                            rule1,Event,app_launch,GrantCredit,100,5m,1,true\n\
                            rule2,InvalidTrigger,event,Block,50,,1,false\n\
                            rule3,Schedule,,Notify,,,0,true";
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:7:
 pub mod macros;
 
 use focus_ir::{
[31m-    ActionIr, AuditQueryIr, Body, CoachingConfigIr, ConditionIr,
(B[m[31m-    ConnectorIr, Document, DocKind, EnforcementPolicyIr, EventFilterIr, MascotSceneIr,
(B[m[31m-    RitualIr, RuleIr, ScheduleIr, SoundCueIr, TaskIr, TriggerIr, WalletMutationIr,
(B[m[32m+    ActionIr, AuditQueryIr, Body, CoachingConfigIr, ConditionIr, ConnectorIr, DocKind, Document,
(B[m[32m+    EnforcementPolicyIr, EventFilterIr, MascotSceneIr, RitualIr, RuleIr, ScheduleIr, SoundCueIr,
(B[m[32m+    TaskIr, TriggerIr, WalletMutationIr,
(B[m };
 use serde_json::{json, Value};
 use std::collections::BTreeMap;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:48:
 pub fn compile_fpl(source: &str) -> Result<Vec<Document>, CompileError> {
     // Prepend helper function definitions to the source.
     // Includes both base helpers and high-level macro library.
[31m-    let full_source = format!("{}\n{}\n{}", STARLARK_HELPERS, macros::MACRO_LIBRARY, source);
(B[m[32m+    let full_source = format!(
(B[m[32m+        "{}\n{}\n{}",
(B[m[32m+        STARLARK_HELPERS,
(B[m[32m+        macros::MACRO_LIBRARY,
(B[m[32m+        source
(B[m[32m+    );
(B[m 
     // Use starlark::eval directly to evaluate.
     use starlark::environment::{Globals, Module};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:59:
     let module = Module::new();
 
     // Parse the module.
[31m-    let ast = AstModule::parse(
(B[m[31m-        "fpl",
(B[m[31m-        full_source,
(B[m[31m-        &starlark::syntax::Dialect::Standard,
(B[m[31m-    ).map_err(|e| {
(B[m[31m-        let msg = format!("{:?}", e);
(B[m[31m-        let line = extract_line_number(&msg).unwrap_or(1);
(B[m[31m-        CompileError::ParseError {
(B[m[31m-            line,
(B[m[31m-            message: msg,
(B[m[31m-        }
(B[m[31m-    })?;
(B[m[32m+    let ast = AstModule::parse("fpl", full_source, &starlark::syntax::Dialect::Standard).map_err(
(B[m[32m+        |e| {
(B[m[32m+            let msg = format!("{:?}", e);
(B[m[32m+            let line = extract_line_number(&msg).unwrap_or(1);
(B[m[32m+            CompileError::ParseError { line, message: msg }
(B[m[32m+        },
(B[m[32m+    )?;
(B[m 
     // Evaluate.
     let mut evaluator = Evaluator::new(&module);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:77:
[31m-    let _result = evaluator.eval_module(ast, &globals)
(B[m[32m+    let _result = evaluator
(B[m[32m+        .eval_module(ast, &globals)
(B[m         .map_err(|e| CompileError::EvalError(format!("{:?}", e)))?;
 
     // Collect all primitives from thread-local registries.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:82:
     let tasks = TASK_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
     let schedules = SCHEDULE_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
     let connectors = CONNECTOR_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
[31m-    let mascot_scenes = MASCOT_SCENE_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
(B[m[32m+    let mascot_scenes =
(B[m[32m+        MASCOT_SCENE_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
(B[m     let coachings = COACHING_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
     let enforcements = ENFORCEMENT_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
     let wallet_ops = WALLET_OP_REGISTRY.with(|r| r.borrow_mut().drain(..).collect::<Vec<_>>());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:379:
 /// Build an IR Document from collected rule data.
 fn build_rule_document(data: &RuleData) -> Result<Document, CompileError> {
     let trigger_ir = build_trigger_ir(&data.trigger)?;
[31m-    let conditions_ir = data.conditions.iter()
(B[m[32m+    let conditions_ir = data
(B[m[32m+        .conditions
(B[m[32m+        .iter()
(B[m         .map(build_condition_ir)
         .collect::<Result<Vec<_>, _>>()?;
[31m-    let actions_ir = data.actions.iter()
(B[m[32m+    let actions_ir = data
(B[m[32m+        .actions
(B[m[32m+        .iter()
(B[m         .map(build_action_ir)
         .collect::<Result<Vec<_>, _>>()?;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:410:
 
 fn build_trigger_ir(trigger: &TriggerData) -> Result<TriggerIr, CompileError> {
     match trigger {
[31m-        TriggerData::Event(name) => {
(B[m[31m-            Ok(TriggerIr::EventFired {
(B[m[31m-                event_name: name.clone(),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        TriggerData::Schedule(cron, tz) => {
(B[m[31m-            Ok(TriggerIr::ScheduleCron {
(B[m[31m-                cron_expression: cron.clone(),
(B[m[31m-                timezone: tz.clone(),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        TriggerData::StateChange(path) => {
(B[m[31m-            Ok(TriggerIr::UserAction {
(B[m[31m-                action_type: "state_change".to_string(),
(B[m[31m-                target: path.clone(),
(B[m[31m-            })
(B[m[31m-        }
(B[m[32m+        TriggerData::Event(name) => Ok(TriggerIr::EventFired {
(B[m[32m+            event_name: name.clone(),
(B[m[32m+        }),
(B[m[32m+        TriggerData::Schedule(cron, tz) => Ok(TriggerIr::ScheduleCron {
(B[m[32m+            cron_expression: cron.clone(),
(B[m[32m+            timezone: tz.clone(),
(B[m[32m+        }),
(B[m[32m+        TriggerData::StateChange(path) => Ok(TriggerIr::UserAction {
(B[m[32m+            action_type: "state_change".to_string(),
(B[m[32m+            target: path.clone(),
(B[m[32m+        }),
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:433:
 fn build_condition_ir(cond: &ConditionData) -> Result<ConditionIr, CompileError> {
     match cond {
[31m-        ConditionData::ConfidenceGte(threshold) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "confidence_gte".to_string(),
(B[m[31m-                args: json!({"threshold": threshold}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::PayloadEq(path, value) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "payload_eq".to_string(),
(B[m[31m-                args: json!({"path": path, "value": value}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::PayloadIn(path, values) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "payload_in".to_string(),
(B[m[31m-                args: json!({"path": path, "values": values}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::PayloadGte(path, value) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "payload_gte".to_string(),
(B[m[31m-                args: json!({"path": path, "value": value}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::PayloadLte(path, value) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "payload_lte".to_string(),
(B[m[31m-                args: json!({"path": path, "value": value}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::PayloadExists(path) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "payload_exists".to_string(),
(B[m[31m-                args: json!({"path": path}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::PayloadMatches(path, regex) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "payload_matches".to_string(),
(B[m[31m-                args: json!({"path": path, "regex": regex}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::SourceEq(source) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "source_eq".to_string(),
(B[m[31m-                args: json!({"source": source}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ConditionData::OccurredWithin(seconds) => {
(B[m[31m-            Ok(ConditionIr::CustomPredicate {
(B[m[31m-                name: "occurred_within".to_string(),
(B[m[31m-                args: json!({"seconds": seconds}),
(B[m[31m-            })
(B[m[31m-        }
(B[m[32m+        ConditionData::ConfidenceGte(threshold) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "confidence_gte".to_string(),
(B[m[32m+            args: json!({"threshold": threshold}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::PayloadEq(path, value) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "payload_eq".to_string(),
(B[m[32m+            args: json!({"path": path, "value": value}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::PayloadIn(path, values) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "payload_in".to_string(),
(B[m[32m+            args: json!({"path": path, "values": values}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::PayloadGte(path, value) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "payload_gte".to_string(),
(B[m[32m+            args: json!({"path": path, "value": value}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::PayloadLte(path, value) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "payload_lte".to_string(),
(B[m[32m+            args: json!({"path": path, "value": value}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::PayloadExists(path) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "payload_exists".to_string(),
(B[m[32m+            args: json!({"path": path}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::PayloadMatches(path, regex) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "payload_matches".to_string(),
(B[m[32m+            args: json!({"path": path, "regex": regex}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::SourceEq(source) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "source_eq".to_string(),
(B[m[32m+            args: json!({"source": source}),
(B[m[32m+        }),
(B[m[32m+        ConditionData::OccurredWithin(seconds) => Ok(ConditionIr::CustomPredicate {
(B[m[32m+            name: "occurred_within".to_string(),
(B[m[32m+            args: json!({"seconds": seconds}),
(B[m[32m+        }),
(B[m         ConditionData::AllOf(conds) => {
[31m-            let inner = conds.iter()
(B[m[32m+            let inner = conds
(B[m[32m+                .iter()
(B[m                 .map(|c| build_condition_ir(c))
                 .collect::<Result<Vec<_>, _>>()?;
             Ok(ConditionIr::And { conditions: inner })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:494:
         }
         ConditionData::AnyOf(conds) => {
[31m-            let inner = conds.iter()
(B[m[32m+            let inner = conds
(B[m[32m+                .iter()
(B[m                 .map(|c| build_condition_ir(c))
                 .collect::<Result<Vec<_>, _>>()?;
             Ok(ConditionIr::Or { conditions: inner })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:509:
 
 fn build_action_ir(action: &ActionData) -> Result<ActionIr, CompileError> {
     match action {
[31m-        ActionData::GrantCredit(amount) => {
(B[m[31m-            Ok(ActionIr::ApplyMutation {
(B[m[31m-                mutation_id: "grant_credit".to_string(),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("amount".to_string(), Value::Number((*amount).into()));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ActionData::DeductCredit(amount) => {
(B[m[31m-            Ok(ActionIr::ApplyMutation {
(B[m[31m-                mutation_id: "deduct_credit".to_string(),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("amount".to_string(), Value::Number((*amount).into()));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            })
(B[m[31m-        }
(B[m[32m+        ActionData::GrantCredit(amount) => Ok(ActionIr::ApplyMutation {
(B[m[32m+            mutation_id: "grant_credit".to_string(),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("amount".to_string(), Value::Number((*amount).into()));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        }),
(B[m[32m+        ActionData::DeductCredit(amount) => Ok(ActionIr::ApplyMutation {
(B[m[32m+            mutation_id: "deduct_credit".to_string(),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("amount".to_string(), Value::Number((*amount).into()));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        }),
(B[m         ActionData::Block {
             profile,
             duration_seconds,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:535:
             rigidity,
[31m-        } => {
(B[m[31m-            Ok(ActionIr::EnforcePolicy {
(B[m[31m-                policy_id: format!("block-{}", profile),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("profile".to_string(), Value::String(profile.clone()));
(B[m[31m-                    m.insert("duration_seconds".to_string(), Value::Number((*duration_seconds).into()));
(B[m[31m-                    m.insert("rigidity".to_string(), Value::String(rigidity.clone()));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ActionData::Unblock(profile) => {
(B[m[31m-            Ok(ActionIr::EnforcePolicy {
(B[m[31m-                policy_id: format!("unblock-{}", profile),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("profile".to_string(), Value::String(profile.clone()));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ActionData::StreakIncrement(streak_id) => {
(B[m[31m-            Ok(ActionIr::ApplyMutation {
(B[m[31m-                mutation_id: "streak_increment".to_string(),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("streak_id".to_string(), Value::String(streak_id.clone()));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ActionData::StreakReset(streak_id) => {
(B[m[31m-            Ok(ActionIr::ApplyMutation {
(B[m[31m-                mutation_id: "streak_reset".to_string(),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("streak_id".to_string(), Value::String(streak_id.clone()));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            })
(B[m[31m-        }
(B[m[31m-        ActionData::Notify(msg) => {
(B[m[31m-            Ok(ActionIr::ShowNotification {
(B[m[31m-                notification_id: "notify".to_string(),
(B[m[31m-                text: msg.clone(),
(B[m[31m-                duration_ms: None,
(B[m[31m-            })
(B[m[31m-        }
(B[m[32m+        } => Ok(ActionIr::EnforcePolicy {
(B[m[32m+            policy_id: format!("block-{}", profile),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("profile".to_string(), Value::String(profile.clone()));
(B[m[32m+                m.insert(
(B[m[32m+                    "duration_seconds".to_string(),
(B[m[32m+                    Value::Number((*duration_seconds).into()),
(B[m[32m+                );
(B[m[32m+                m.insert("rigidity".to_string(), Value::String(rigidity.clone()));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        }),
(B[m[32m+        ActionData::Unblock(profile) => Ok(ActionIr::EnforcePolicy {
(B[m[32m+            policy_id: format!("unblock-{}", profile),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("profile".to_string(), Value::String(profile.clone()));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        }),
(B[m[32m+        ActionData::StreakIncrement(streak_id) => Ok(ActionIr::ApplyMutation {
(B[m[32m+            mutation_id: "streak_increment".to_string(),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("streak_id".to_string(), Value::String(streak_id.clone()));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        }),
(B[m[32m+        ActionData::StreakReset(streak_id) => Ok(ActionIr::ApplyMutation {
(B[m[32m+            mutation_id: "streak_reset".to_string(),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("streak_id".to_string(), Value::String(streak_id.clone()));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        }),
(B[m[32m+        ActionData::Notify(msg) => Ok(ActionIr::ShowNotification {
(B[m[32m+            notification_id: "notify".to_string(),
(B[m[32m+            text: msg.clone(),
(B[m[32m+            duration_ms: None,
(B[m[32m+        }),
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:600:
         ideal_chunk_minutes: 45,
     };
 
[31m-    let deadline = data.deadline.as_ref().map(|deadline_str| focus_ir::DeadlineIr {
(B[m[31m-        when_iso8601: Some(deadline_str.clone()),
(B[m[31m-        rigidity: data.rigidity.clone(),
(B[m[31m-    });
(B[m[32m+    let deadline = data
(B[m[32m+        .deadline
(B[m[32m+        .as_ref()
(B[m[32m+        .map(|deadline_str| focus_ir::DeadlineIr {
(B[m[32m+            when_iso8601: Some(deadline_str.clone()),
(B[m[32m+            rigidity: data.rigidity.clone(),
(B[m[32m+        });
(B[m 
     let task_ir = TaskIr {
         id: data.id.clone(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:616:
         priority_weight: data.priority,
         deadline,
         chunking: chunking_policy,
[31m-        constraints: data.constraints.iter()
(B[m[32m+        constraints: data
(B[m[32m+            .constraints
(B[m[32m+            .iter()
(B[m             .filter_map(|_v| {
                 // Parse constraint from JSON Value as passthrough for now
                 // TODO: structured constraint parsing once focus-ir is finalized
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:1259:
 
         let _docs = result.unwrap();
         // Golden test will be completed once rule collection is wired up
[31m-
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:1271:
 "#;
         let result = compile_fpl(source);
         // Should parse without error (even though tasks aren't collected yet)
[31m-        assert!(result.is_ok(), "Task helper should parse: {:?}", result.err());
(B[m[32m+        assert!(
(B[m[32m+            result.is_ok(),
(B[m[32m+            "Task helper should parse: {:?}",
(B[m[32m+            result.err()
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:1283:
 "#;
         let result = compile_fpl(source);
         // Should parse without error (even though schedules aren't collected yet)
[31m-        assert!(result.is_ok(), "Schedule helper should parse: {:?}", result.err());
(B[m[32m+        assert!(
(B[m[32m+            result.is_ok(),
(B[m[32m+            "Schedule helper should parse: {:?}",
(B[m[32m+            result.err()
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/lib.rs:1304:
 "#;
         let result = compile_fpl(source);
         // Should parse without error
[31m-        assert!(result.is_ok(), "Mixed example should parse: {:?}", result.err());
(B[m[32m+        assert!(
(B[m[32m+            result.is_ok(),
(B[m[32m+            "Mixed example should parse: {:?}",
(B[m[32m+            result.err()
(B[m[32m+        );
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:11:
 reward("focus:session_completed", credits=15, streak=1)
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "reward macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "reward macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `reward()` macro parses with streak disabled.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:21:
 reward("task:completed", credits=5, streak=0)
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "reward without streak should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "reward without streak should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `penalize()` macro parses correctly.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:31:
 penalize("distraction:triggered", credits=10)
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "penalize macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "penalize macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `remind()` macro parses with timezone.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:41:
 remind("0 9 * * *", "Time for standup", at="America/New_York")
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "remind macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "remind macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `celebrate()` macro parses with sound parameter.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:51:
 celebrate("milestone:unlocked", "Great work!", sound="confetti")
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "celebrate macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "celebrate macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `block()` macro parses with single app.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:61:
 block(["Instagram"], "work_hours")
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "block macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "block macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `block()` macro parses with multiple apps.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:71:
 block(["Instagram", "TikTok"], "evening")
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "block macro with app list should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "block macro with app list should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `unlock_after()` macro parses correctly.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:81:
 unlock_after("goal:completed", 2)
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "unlock_after macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "unlock_after macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `track_streak()` macro parses correctly.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:91:
 track_streak("focus:session_ended", "Daily Focus")
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "track_streak macro should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "track_streak macro should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `if_pattern()` with named pattern parses.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:101:
 conds = if_pattern("weekday")
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "if_pattern should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "if_pattern should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test that `if_pattern()` with custom conditions parses.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:111:
 conds = if_pattern("custom_time", [payload_exists("hour")])
 "#;
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "if_pattern with custom conditions should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "if_pattern with custom conditions should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test round-trip macro parsing consistency.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:143:
 "#;
 
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "all 8 macros should compile together: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "all 8 macros should compile together: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test multiple macro invocations with different parameters.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:157:
 "#;
 
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "multiple macro calls should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "multiple macro calls should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test macro with negative parameter values.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:168:
 "#;
 
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "macro should parse with negative values: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "macro should parse with negative values: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test pattern conditions integration.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:181:
 "#;
 
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "pattern-based conditions should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "pattern-based conditions should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test macros with minimal and default parameters.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-lang/src/macros/tests.rs:196:
 "#;
 
     let result = compile_fpl(source);
[31m-    assert!(result.is_ok(), "macros with default params should parse: {:?}", result.err());
(B[m[32m+    assert!(
(B[m[32m+        result.is_ok(),
(B[m[32m+        "macros with default params should parse: {:?}",
(B[m[32m+        result.err()
(B[m[32m+    );
(B[m }
 
 /// Test macro-generated IR serialization (if any rules are collected).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:87:
 
 impl MascotState {
     pub fn new(pose: Pose, emotion: Emotion, bubble: Option<String>) -> Self {
[31m-        Self { pose, emotion, since: Utc::now(), bubble_text: bubble }
(B[m[32m+        Self {
(B[m[32m+            pose,
(B[m[32m+            emotion,
(B[m[32m+            since: Utc::now(),
(B[m[32m+            bubble_text: bubble,
(B[m[32m+        }
(B[m     }
 
     /// MVP copy bank — deterministic strings per pose. Swap for LLM later.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:106:
 
 impl Default for MascotState {
     fn default() -> Self {
[31m-        Self::new(Pose::Idle, Emotion::Neutral, Some(Self::default_bubble_for(Pose::Idle).into()))
(B[m[32m+        Self::new(
(B[m[32m+            Pose::Idle,
(B[m[32m+            Emotion::Neutral,
(B[m[32m+            Some(Self::default_bubble_for(Pose::Idle).into()),
(B[m[32m+        )
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:124:
 
 impl MascotMachine {
     pub fn new() -> Self {
[31m-        Self { state: MascotState::default(), coaching: None }
(B[m[32m+        Self {
(B[m[32m+            state: MascotState::default(),
(B[m[32m+            coaching: None,
(B[m[32m+        }
(B[m     }
 
     /// Attach an LLM-backed bubble text provider. Opt-in — sync `on_event`
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:254:
     #[test]
     fn streak_shows_proud_encouraging() {
         let mut m = MascotMachine::new();
[31m-        let s = m.on_event(MascotEvent::StreakIncremented { name: "study".into(), count: 3 });
(B[m[32m+        let s = m.on_event(MascotEvent::StreakIncremented {
(B[m[32m+            name: "study".into(),
(B[m[32m+            count: 3,
(B[m[32m+        });
(B[m         assert_eq!(s.pose, Pose::Encouraging);
         assert_eq!(s.emotion, Emotion::Proud);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:289:
             Arc::new(StubCoachingProvider::single("Nice streak — keep rolling."));
         let mut m = MascotMachine::new().with_coaching(provider);
         let s = m
[31m-            .on_event_with_bubble(MascotEvent::StreakIncremented { name: "study".into(), count: 4 })
(B[m[32m+            .on_event_with_bubble(MascotEvent::StreakIncremented {
(B[m[32m+                name: "study".into(),
(B[m[32m+                count: 4,
(B[m[32m+            })
(B[m             .await;
[31m-        assert_eq!(s.bubble_text.as_deref(), Some("Nice streak — keep rolling."));
(B[m[32m+        assert_eq!(
(B[m[32m+            s.bubble_text.as_deref(),
(B[m[32m+            Some("Nice streak — keep rolling.")
(B[m[32m+        );
(B[m         assert_eq!(s.pose, Pose::Encouraging);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:300:
         let provider: Arc<dyn focus_coaching::CoachingProvider> = Arc::new(NoopCoachingProvider);
         let mut m = MascotMachine::new().with_coaching(provider);
         let s = m.on_event_with_bubble(MascotEvent::Idle).await;
[31m-        assert_eq!(s.bubble_text.as_deref(), Some(MascotState::default_bubble_for(Pose::Idle)));
(B[m[32m+        assert_eq!(
(B[m[32m+            s.bubble_text.as_deref(),
(B[m[32m+            Some(MascotState::default_bubble_for(Pose::Idle))
(B[m[32m+        );
(B[m     }
 
     #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mascot/src/lib.rs:307:
     async fn llm_bubble_without_provider_uses_static() {
         let mut m = MascotMachine::new();
[31m-        let s = m.on_event_with_bubble(MascotEvent::FocusSessionCompleted { minutes: 50 }).await;
(B[m[32m+        let s = m
(B[m[32m+            .on_event_with_bubble(MascotEvent::FocusSessionCompleted { minutes: 50 })
(B[m[32m+            .await;
(B[m         assert_eq!(s.pose, Pose::Celebratory);
         assert_eq!(
             s.bubble_text.as_deref(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/server.rs:3:
 use crate::tools::FocalPointToolsImpl;
 use anyhow::Result;
 use mcp_sdk::server::Server;
[31m-use mcp_sdk::transport::{Transport, ServerStdioTransport};
(B[m[32m+use mcp_sdk::transport::{ServerStdioTransport, Transport};
(B[m use tracing::info;
 
 /// Run the MCP server over STDIO transport (default).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:34:
         let adapter = self.adapter.clone();
 
         // Read-only tools (15)
[31m-        tools.add_tool(TasksListTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesListTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(WalletBalanceTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(PenaltyShowTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(AuditRecentTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(AuditVerifyTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(AuditExportTool { adapter: adapter.clone() });
(B[m[32m+        tools.add_tool(TasksListTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesListTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(WalletBalanceTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(PenaltyShowTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(AuditRecentTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(AuditVerifyTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(AuditExportTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         tools.add_tool(TemplatesListBundledTool);
         tools.add_tool(TemplatesCatalogTool);
         tools.add_tool(ConnectorsListTool);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:51:
         tools.add_tool(SyncTickStatusTool);
 
         // Write tools (12)
[31m-        tools.add_tool(TasksAddTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(TasksMarkDoneTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesEnableTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesDisableTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesUpsertTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesUpsertFromFplTool { adapter: adapter.clone() });
(B[m[32m+        tools.add_tool(TasksAddTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(TasksMarkDoneTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesEnableTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesDisableTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesUpsertTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesUpsertFromFplTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         tools.add_tool(TemplatesInstallTool);
         tools.add_tool(FocusEmitSessionStartedTool);
         tools.add_tool(FocusEmitSessionCompletedTool);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:63:
         tools.add_tool(FocusCancelTool);
[31m-        tools.add_tool(WalletSpendTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(WalletGrantTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(PenaltyApplyTool { adapter: adapter.clone() });
(B[m[32m+        tools.add_tool(WalletSpendTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(WalletGrantTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(PenaltyApplyTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         tools.add_tool(ConnectorsConnectCanvasTool);
         tools.add_tool(ConnectorsConnectGcalTool);
         tools.add_tool(ConnectorsConnectGithubTool);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:833:
         })
     }
     fn call(&self, input: Option<Value>) -> Result<CallToolResponse> {
[31m-        let rule_id = input.as_ref()
(B[m[32m+        let rule_id = input
(B[m[32m+            .as_ref()
(B[m             .and_then(|v| v.get("rule_id"))
             .and_then(|v| v.as_str())
             .map(String::from)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/http_sse.rs:59:
 
     fn check(&mut self, key: &str) -> bool {
         let now = Instant::now();
[31m-        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
(B[m[31m-            tokens: RATE_LIMIT_REQ_PER_MIN,
(B[m[31m-            last_refill: now,
(B[m[31m-        });
(B[m[32m+        let bucket = self
(B[m[32m+            .buckets
(B[m[32m+            .entry(key.to_string())
(B[m[32m+            .or_insert_with(|| TokenBucket {
(B[m[32m+                tokens: RATE_LIMIT_REQ_PER_MIN,
(B[m[32m+                last_refill: now,
(B[m[32m+            });
(B[m 
         // Refill based on elapsed time
         let elapsed = now.duration_since(bucket.last_refill).as_secs_f32();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/http_sse.rs:102:
     let bearer_token = std::env::var("FOCALPOINT_MCP_HTTP_TOKEN")
         .unwrap_or_else(|_| "focalpoint-default-insecure-token".to_string());
 
[31m-    let bind_addr = std::env::var("FOCALPOINT_MCP_HTTP_ADDR")
(B[m[31m-        .unwrap_or_else(|_| "127.0.0.1:8473".to_string());
(B[m[32m+    let bind_addr =
(B[m[32m+        std::env::var("FOCALPOINT_MCP_HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:8473".to_string());
(B[m 
     let (tx, _) = broadcast::channel(100);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/http_sse.rs:187:
     headers: HeaderMap,
     Json(_input): Json<Value>,
 ) -> Result<Json<Value>, (StatusCode, String)> {
[31m-    check_auth(&state, &headers).map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;
(B[m[32m+    check_auth(&state, &headers)
(B[m[32m+        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;
(B[m 
     // Rate limit check
     let client_id = headers
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:6:
 
 use crate::tools::FocalPointToolsImpl;
 use anyhow::Result;
[32m+use futures::stream::StreamExt;
(B[m[32m+use futures::SinkExt;
(B[m use serde_json::{json, Value};
 use std::time::Instant;
 use tokio::net::TcpListener;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:12:
 use tokio_tungstenite::accept_async;
 use tokio_tungstenite::tungstenite::Message;
 use tracing::info;
[31m-use futures::stream::StreamExt;
(B[m[31m-use futures::SinkExt;
(B[m 
 const RATE_LIMIT_REQ_PER_MIN: f32 = 100.0;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:54:
 }
 
 /// Start WebSocket server on configured bind address.
[31m-pub async fn start_websocket(_db_path: std::path::PathBuf, _tools_impl: FocalPointToolsImpl) -> Result<()> {
(B[m[32m+pub async fn start_websocket(
(B[m[32m+    _db_path: std::path::PathBuf,
(B[m[32m+    _tools_impl: FocalPointToolsImpl,
(B[m[32m+) -> Result<()> {
(B[m     let expected_token = std::env::var("FOCALPOINT_MCP_HTTP_TOKEN")
         .unwrap_or_else(|_| "focalpoint-default-insecure-token".to_string());
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:61:
[31m-    let bind_addr = std::env::var("FOCALPOINT_MCP_WS_ADDR")
(B[m[31m-        .unwrap_or_else(|_| "127.0.0.1:8474".to_string());
(B[m[32m+    let bind_addr =
(B[m[32m+        std::env::var("FOCALPOINT_MCP_WS_ADDR").unwrap_or_else(|_| "127.0.0.1:8474".to_string());
(B[m 
     let listener = TcpListener::bind(&bind_addr).await?;
     info!("WebSocket server listening on ws://{}/mcp/ws", bind_addr);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:178:
         let result = match method {
             Some("focalpoint.tasks.list") => json!({ "tasks": [], "status": "ok" }),
             Some("focalpoint.rules.list") => json!({ "rules": [], "status": "ok" }),
[31m-            Some("focalpoint.wallet.balance") => json!({ "balance": 0, "currency": "focus", "status": "ok" }),
(B[m[32m+            Some("focalpoint.wallet.balance") => {
(B[m[32m+                json!({ "balance": 0, "currency": "focus", "status": "ok" })
(B[m[32m+            }
(B[m             Some(tool) => json!({
                 "error": format!("Unknown tool: {}", tool)
             }),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/main.rs:46:
 
     let cli = Cli::parse();
 
[31m-    let db_path = cli.db.or_else(|| {
(B[m[31m-        std::env::var("FOCALPOINT_DB")
(B[m[31m-            .ok()
(B[m[31m-            .map(PathBuf::from)
(B[m[31m-    }).or_else(|| {
(B[m[31m-        // Platform default: macOS Application Support
(B[m[31m-        #[cfg(target_os = "macos")]
(B[m[31m-        {
(B[m[31m-            let mut path = dirs::home_dir()?;
(B[m[31m-            path.push("Library/Application Support/focalpoint/core.db");
(B[m[31m-            Some(path)
(B[m[31m-        }
(B[m[31m-        #[cfg(not(target_os = "macos"))]
(B[m[31m-        {
(B[m[31m-            let mut path = dirs::data_local_dir()?;
(B[m[31m-            path.push("focalpoint/core.db");
(B[m[31m-            Some(path)
(B[m[31m-        }
(B[m[31m-    });
(B[m[32m+    let db_path = cli
(B[m[32m+        .db
(B[m[32m+        .or_else(|| std::env::var("FOCALPOINT_DB").ok().map(PathBuf::from))
(B[m[32m+        .or_else(|| {
(B[m[32m+            // Platform default: macOS Application Support
(B[m[32m+            #[cfg(target_os = "macos")]
(B[m[32m+            {
(B[m[32m+                let mut path = dirs::home_dir()?;
(B[m[32m+                path.push("Library/Application Support/focalpoint/core.db");
(B[m[32m+                Some(path)
(B[m[32m+            }
(B[m[32m+            #[cfg(not(target_os = "macos"))]
(B[m[32m+            {
(B[m[32m+                let mut path = dirs::data_local_dir()?;
(B[m[32m+                path.push("focalpoint/core.db");
(B[m[32m+                Some(path)
(B[m[32m+            }
(B[m[32m+        });
(B[m 
     if let Some(path) = &db_path {
         tracing::info!("Using database: {}", path.display());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/main.rs:76:
 
     // Load database adapter
     let db_path_for_open = db_path.clone();
[31m-    let adapter = tokio::task::spawn_blocking(move || {
(B[m[31m-        focus_storage::SqliteAdapter::open(&db_path_for_open)
(B[m[31m-    })
(B[m[31m-    .await??;
(B[m[32m+    let adapter =
(B[m[32m+        tokio::task::spawn_blocking(move || focus_storage::SqliteAdapter::open(&db_path_for_open))
(B[m[32m+            .await??;
(B[m 
     let tools_impl = crate::tools::FocalPointToolsImpl::new(adapter);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/server.rs:3:
 use crate::tools::FocalPointToolsImpl;
 use anyhow::Result;
 use mcp_sdk::server::Server;
[31m-use mcp_sdk::transport::{Transport, ServerStdioTransport};
(B[m[32m+use mcp_sdk::transport::{ServerStdioTransport, Transport};
(B[m use tracing::info;
 
 /// Run the MCP server over STDIO transport (default).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:34:
         let adapter = self.adapter.clone();
 
         // Read-only tools (15)
[31m-        tools.add_tool(TasksListTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesListTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(WalletBalanceTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(PenaltyShowTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(AuditRecentTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(AuditVerifyTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(AuditExportTool { adapter: adapter.clone() });
(B[m[32m+        tools.add_tool(TasksListTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesListTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(WalletBalanceTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(PenaltyShowTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(AuditRecentTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(AuditVerifyTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(AuditExportTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         tools.add_tool(TemplatesListBundledTool);
         tools.add_tool(TemplatesCatalogTool);
         tools.add_tool(ConnectorsListTool);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:51:
         tools.add_tool(SyncTickStatusTool);
 
         // Write tools (12)
[31m-        tools.add_tool(TasksAddTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(TasksMarkDoneTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesEnableTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesDisableTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesUpsertTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(RulesUpsertFromFplTool { adapter: adapter.clone() });
(B[m[32m+        tools.add_tool(TasksAddTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(TasksMarkDoneTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesEnableTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesDisableTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesUpsertTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(RulesUpsertFromFplTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         tools.add_tool(TemplatesInstallTool);
         tools.add_tool(FocusEmitSessionStartedTool);
         tools.add_tool(FocusEmitSessionCompletedTool);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:63:
         tools.add_tool(FocusCancelTool);
[31m-        tools.add_tool(WalletSpendTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(WalletGrantTool { adapter: adapter.clone() });
(B[m[31m-        tools.add_tool(PenaltyApplyTool { adapter: adapter.clone() });
(B[m[32m+        tools.add_tool(WalletSpendTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(WalletGrantTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m[32m+        tools.add_tool(PenaltyApplyTool {
(B[m[32m+            adapter: adapter.clone(),
(B[m[32m+        });
(B[m         tools.add_tool(ConnectorsConnectCanvasTool);
         tools.add_tool(ConnectorsConnectGcalTool);
         tools.add_tool(ConnectorsConnectGithubTool);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/tools.rs:833:
         })
     }
     fn call(&self, input: Option<Value>) -> Result<CallToolResponse> {
[31m-        let rule_id = input.as_ref()
(B[m[32m+        let rule_id = input
(B[m[32m+            .as_ref()
(B[m             .and_then(|v| v.get("rule_id"))
             .and_then(|v| v.as_str())
             .map(String::from)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/http_sse.rs:59:
 
     fn check(&mut self, key: &str) -> bool {
         let now = Instant::now();
[31m-        let bucket = self.buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
(B[m[31m-            tokens: RATE_LIMIT_REQ_PER_MIN,
(B[m[31m-            last_refill: now,
(B[m[31m-        });
(B[m[32m+        let bucket = self
(B[m[32m+            .buckets
(B[m[32m+            .entry(key.to_string())
(B[m[32m+            .or_insert_with(|| TokenBucket {
(B[m[32m+                tokens: RATE_LIMIT_REQ_PER_MIN,
(B[m[32m+                last_refill: now,
(B[m[32m+            });
(B[m 
         // Refill based on elapsed time
         let elapsed = now.duration_since(bucket.last_refill).as_secs_f32();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/http_sse.rs:102:
     let bearer_token = std::env::var("FOCALPOINT_MCP_HTTP_TOKEN")
         .unwrap_or_else(|_| "focalpoint-default-insecure-token".to_string());
 
[31m-    let bind_addr = std::env::var("FOCALPOINT_MCP_HTTP_ADDR")
(B[m[31m-        .unwrap_or_else(|_| "127.0.0.1:8473".to_string());
(B[m[32m+    let bind_addr =
(B[m[32m+        std::env::var("FOCALPOINT_MCP_HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:8473".to_string());
(B[m 
     let (tx, _) = broadcast::channel(100);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/http_sse.rs:187:
     headers: HeaderMap,
     Json(_input): Json<Value>,
 ) -> Result<Json<Value>, (StatusCode, String)> {
[31m-    check_auth(&state, &headers).map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;
(B[m[32m+    check_auth(&state, &headers)
(B[m[32m+        .map_err(|_| (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))?;
(B[m 
     // Rate limit check
     let client_id = headers
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:6:
 
 use crate::tools::FocalPointToolsImpl;
 use anyhow::Result;
[32m+use futures::stream::StreamExt;
(B[m[32m+use futures::SinkExt;
(B[m use serde_json::{json, Value};
 use std::time::Instant;
 use tokio::net::TcpListener;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:12:
 use tokio_tungstenite::accept_async;
 use tokio_tungstenite::tungstenite::Message;
 use tracing::info;
[31m-use futures::stream::StreamExt;
(B[m[31m-use futures::SinkExt;
(B[m 
 const RATE_LIMIT_REQ_PER_MIN: f32 = 100.0;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:54:
 }
 
 /// Start WebSocket server on configured bind address.
[31m-pub async fn start_websocket(_db_path: std::path::PathBuf, _tools_impl: FocalPointToolsImpl) -> Result<()> {
(B[m[32m+pub async fn start_websocket(
(B[m[32m+    _db_path: std::path::PathBuf,
(B[m[32m+    _tools_impl: FocalPointToolsImpl,
(B[m[32m+) -> Result<()> {
(B[m     let expected_token = std::env::var("FOCALPOINT_MCP_HTTP_TOKEN")
         .unwrap_or_else(|_| "focalpoint-default-insecure-token".to_string());
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:61:
[31m-    let bind_addr = std::env::var("FOCALPOINT_MCP_WS_ADDR")
(B[m[31m-        .unwrap_or_else(|_| "127.0.0.1:8474".to_string());
(B[m[32m+    let bind_addr =
(B[m[32m+        std::env::var("FOCALPOINT_MCP_WS_ADDR").unwrap_or_else(|_| "127.0.0.1:8474".to_string());
(B[m 
     let listener = TcpListener::bind(&bind_addr).await?;
     info!("WebSocket server listening on ws://{}/mcp/ws", bind_addr);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/src/transport/websocket.rs:178:
         let result = match method {
             Some("focalpoint.tasks.list") => json!({ "tasks": [], "status": "ok" }),
             Some("focalpoint.rules.list") => json!({ "rules": [], "status": "ok" }),
[31m-            Some("focalpoint.wallet.balance") => json!({ "balance": 0, "currency": "focus", "status": "ok" }),
(B[m[32m+            Some("focalpoint.wallet.balance") => {
(B[m[32m+                json!({ "balance": 0, "currency": "focus", "status": "ok" })
(B[m[32m+            }
(B[m             Some(tool) => json!({
                 "error": format!("Unknown tool: {}", tool)
             }),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/http_sse_tests.rs:21:
 
     // Verify tools can be built (server initialization phase)
     let mcp_tools = tools.build_mcp_tools();
[31m-    assert!(!mcp_tools.list_tools().is_empty(), "Server should have at least one tool");
(B[m[32m+    assert!(
(B[m[32m+        !mcp_tools.list_tools().is_empty(),
(B[m[32m+        "Server should have at least one tool"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/http_sse_tests.rs:56:
     let now = Instant::now();
 
     // Initial bucket with full capacity
[31m-    buckets.insert("test".to_string(), TokenBucket {
(B[m[31m-        tokens: limit,
(B[m[31m-        last_refill: now,
(B[m[31m-    });
(B[m[32m+    buckets.insert(
(B[m[32m+        "test".to_string(),
(B[m[32m+        TokenBucket {
(B[m[32m+            tokens: limit,
(B[m[32m+            last_refill: now,
(B[m[32m+        },
(B[m[32m+    );
(B[m 
     let bucket = &buckets["test"];
[31m-    assert_eq!(bucket.tokens, limit, "Initial capacity should be 100 tokens");
(B[m[32m+    assert_eq!(
(B[m[32m+        bucket.tokens, limit,
(B[m[32m+        "Initial capacity should be 100 tokens"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/http_sse_tests.rs:99:
     let mcp_tools = tools.build_mcp_tools();
 
     // Verify that tools are available (404 behavior would be at HTTP layer)
[31m-    assert!(!mcp_tools.list_tools().is_empty(), "Should have tools for 404 detection");
(B[m[32m+    assert!(
(B[m[32m+        !mcp_tools.list_tools().is_empty(),
(B[m[32m+        "Should have tools for 404 detection"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/http_sse_tests.rs:109:
     let mcp_tools = tools.build_mcp_tools();
 
     // Verify core tools are present
[31m-    let tool_names: Vec<String> = mcp_tools.list_tools().iter().map(|t| t.name.clone()).collect();
(B[m[32m+    let tool_names: Vec<String> = mcp_tools
(B[m[32m+        .list_tools()
(B[m[32m+        .iter()
(B[m[32m+        .map(|t| t.name.clone())
(B[m[32m+        .collect();
(B[m 
[31m-    assert!(tool_names.contains(&"focalpoint.tasks.list".to_string()), "Should have tasks.list tool");
(B[m[31m-    assert!(tool_names.contains(&"focalpoint.rules.list".to_string()), "Should have rules.list tool");
(B[m[31m-    assert!(tool_names.contains(&"focalpoint.wallet.balance".to_string()), "Should have wallet.balance tool");
(B[m[32m+    assert!(
(B[m[32m+        tool_names.contains(&"focalpoint.tasks.list".to_string()),
(B[m[32m+        "Should have tasks.list tool"
(B[m[32m+    );
(B[m[32m+    assert!(
(B[m[32m+        tool_names.contains(&"focalpoint.rules.list".to_string()),
(B[m[32m+        "Should have rules.list tool"
(B[m[32m+    );
(B[m[32m+    assert!(
(B[m[32m+        tool_names.contains(&"focalpoint.wallet.balance".to_string()),
(B[m[32m+        "Should have wallet.balance tool"
(B[m[32m+    );
(B[m }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:18:
     let tool_defs = mcp_tools.list_tools();
 
     // Verify tools are registered (expected count per design doc)
[31m-    assert!(tool_defs.len() >= 27, "Expected at least 27 tools, got {}", tool_defs.len());
(B[m[32m+    assert!(
(B[m[32m+        tool_defs.len() >= 27,
(B[m[32m+        "Expected at least 27 tools, got {}",
(B[m[32m+        tool_defs.len()
(B[m[32m+    );
(B[m 
     // Verify all expected tool names
     let names: Vec<&str> = tool_defs.iter().map(|t| t.name.as_str()).collect();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:93:
         fn input_schema(&self) -> serde_json::Value {
             serde_json::json!({})
         }
[31m-        fn call(&self, _input: Option<serde_json::Value>) -> anyhow::Result<mcp_sdk::types::CallToolResponse> {
(B[m[32m+        fn call(
(B[m[32m+            &self,
(B[m[32m+            _input: Option<serde_json::Value>,
(B[m[32m+        ) -> anyhow::Result<mcp_sdk::types::CallToolResponse> {
(B[m             Ok(mcp_sdk::types::CallToolResponse {
                 content: vec![mcp_sdk::types::ToolResponseContent::Text {
                     text: serde_json::json!({
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:129:
         fn input_schema(&self) -> serde_json::Value {
             json!({})
         }
[31m-        fn call(&self, _input: Option<serde_json::Value>) -> anyhow::Result<mcp_sdk::types::CallToolResponse> {
(B[m[32m+        fn call(
(B[m[32m+            &self,
(B[m[32m+            _input: Option<serde_json::Value>,
(B[m[32m+        ) -> anyhow::Result<mcp_sdk::types::CallToolResponse> {
(B[m             let content = json!({
                 "packs": [
                     { "id": "starter-social-block", "name": "Social Media Blocker" },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:176:
 
     // Simulate a tool call without user_id (should fail).
     let tools = mcp_tools.list_tools();
[31m-    let wallet_tool = tools.iter()
(B[m[32m+    let wallet_tool = tools
(B[m[32m+        .iter()
(B[m         .find(|t| t.name == "focalpoint.wallet.balance")
         .expect("wallet.balance tool");
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:193:
     let mcp_tools = impl_tools.build_mcp_tools();
 
     let tools = mcp_tools.list_tools();
[31m-    let penalty_tool = tools.iter()
(B[m[32m+    let penalty_tool = tools
(B[m[32m+        .iter()
(B[m         .find(|t| t.name == "focalpoint.penalty.show")
         .expect("penalty.show tool");
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:209:
     let mcp_tools = impl_tools.build_mcp_tools();
 
     let tools = mcp_tools.list_tools();
[31m-    let connectors_tool = tools.iter()
(B[m[32m+    let connectors_tool = tools
(B[m[32m+        .iter()
(B[m         .find(|t| t.name == "focalpoint.connectors.list")
         .expect("connectors.list tool");
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:225:
     let mcp_tools = impl_tools.build_mcp_tools();
 
     let tools = mcp_tools.list_tools();
[31m-    let audit_tool = tools.iter()
(B[m[32m+    let audit_tool = tools
(B[m[32m+        .iter()
(B[m         .find(|t| t.name == "focalpoint.audit.verify")
         .expect("audit.verify tool");
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:242:
     let mcp_tools = impl_tools.build_mcp_tools();
 
     let tools = mcp_tools.list_tools();
[31m-    let session_tool = tools.iter()
(B[m[32m+    let session_tool = tools
(B[m[32m+        .iter()
(B[m         .find(|t| t.name == "focalpoint.focus.emit_session_started")
         .expect("focus.emit_session_started tool");
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/integration_tests.rs:258:
     let mcp_tools = impl_tools.build_mcp_tools();
 
     let tools = mcp_tools.list_tools();
[31m-    let rules_tool = tools.iter()
(B[m[32m+    let rules_tool = tools
(B[m[32m+        .iter()
(B[m         .find(|t| t.name == "focalpoint.rules.upsert")
         .expect("rules.upsert tool");
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/websocket_tests.rs:17:
     let tools = FocalPointToolsImpl::new(adapter);
 
     let mcp_tools = tools.build_mcp_tools();
[31m-    assert!(!mcp_tools.list_tools().is_empty(), "Server should have tools for WS endpoint");
(B[m[32m+    assert!(
(B[m[32m+        !mcp_tools.list_tools().is_empty(),
(B[m[32m+        "Server should have tools for WS endpoint"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/websocket_tests.rs:29:
         "id": 1
     });
 
[31m-    assert_eq!(request.get("jsonrpc").and_then(|v| v.as_str()), Some("2.0"), "Should be JSON-RPC 2.0");
(B[m[31m-    assert_eq!(request.get("method").and_then(|v| v.as_str()), Some("focalpoint.tasks.list"), "Method should be set");
(B[m[31m-    assert_eq!(request.get("id").and_then(|v| v.as_i64()), Some(1), "ID should be preserved");
(B[m[32m+    assert_eq!(
(B[m[32m+        request.get("jsonrpc").and_then(|v| v.as_str()),
(B[m[32m+        Some("2.0"),
(B[m[32m+        "Should be JSON-RPC 2.0"
(B[m[32m+    );
(B[m[32m+    assert_eq!(
(B[m[32m+        request.get("method").and_then(|v| v.as_str()),
(B[m[32m+        Some("focalpoint.tasks.list"),
(B[m[32m+        "Method should be set"
(B[m[32m+    );
(B[m[32m+    assert_eq!(
(B[m[32m+        request.get("id").and_then(|v| v.as_i64()),
(B[m[32m+        Some(1),
(B[m[32m+        "ID should be preserved"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/websocket_tests.rs:53:
     let expected_token = "correct-token";
     let provided_token = "wrong-token";
 
[31m-    assert_ne!(provided_token, expected_token, "Should reject mismatched tokens");
(B[m[32m+    assert_ne!(
(B[m[32m+        provided_token, expected_token,
(B[m[32m+        "Should reject mismatched tokens"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/websocket_tests.rs:68:
         "id": 1
     });
 
[31m-    assert_eq!(error_response.get("error").and_then(|e| e.get("code")).and_then(|c| c.as_i64()), Some(-32000), "Should have rate limit error code");
(B[m[32m+    assert_eq!(
(B[m[32m+        error_response
(B[m[32m+            .get("error")
(B[m[32m+            .and_then(|e| e.get("code"))
(B[m[32m+            .and_then(|c| c.as_i64()),
(B[m[32m+        Some(-32000),
(B[m[32m+        "Should have rate limit error code"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/websocket_tests.rs:81:
     });
 
     let response_id = request.get("id").cloned();
[31m-    assert_eq!(response_id, Some(json!(request_id)), "Response should preserve request ID");
(B[m[32m+    assert_eq!(
(B[m[32m+        response_id,
(B[m[32m+        Some(json!(request_id)),
(B[m[32m+        "Response should preserve request ID"
(B[m[32m+    );
(B[m }
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-mcp-server/tests/websocket_tests.rs:99:
     ];
 
     assert_eq!(requests.len(), 3, "Should have 3 test requests");
[31m-    assert!(!mcp_tools.list_tools().is_empty(), "Server should handle multiple requests");
(B[m[32m+    assert!(
(B[m[32m+        !mcp_tools.list_tools().is_empty(),
(B[m[32m+        "Server should handle multiple requests"
(B[m[32m+    );
(B[m }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/integration_tests.rs:4:
 #[cfg(test)]
 mod tests {
     use crate::{
[31m-        init_tracing, ConnectorSpanAttrs, MetricsRegistry, RuleSpanAttrs, AuditSpanAttrs,
(B[m[32m+        init_tracing, AuditSpanAttrs, ConnectorSpanAttrs, MetricsRegistry, RuleSpanAttrs,
(B[m         SpanPrivacyFilter,
     };
     use serde_json::json;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/lib.rs:58:
 
 pub use metrics::MetricsRegistry;
 pub use privacy_filter::SpanPrivacyFilter;
[31m-pub use spans::{
(B[m[31m-    AuditSpanAttrs, ConnectorSpanAttrs, RuleSpanAttrs, SpanKind, WalletSpanAttrs,
(B[m[31m-};
(B[m[32m+pub use spans::{AuditSpanAttrs, ConnectorSpanAttrs, RuleSpanAttrs, SpanKind, WalletSpanAttrs};
(B[m 
 /// Initialize tracing with JSON or pretty console output.
 /// Honors `RUST_LOG` and `FOCALPOINT_LOG_LEVEL` env vars.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/lib.rs:71:
         .or_else(|| std::env::var("FOCALPOINT_LOG_LEVEL").ok())
         .unwrap_or_else(|| "info".to_string());
 
[31m-    let env_filter = EnvFilter::try_from_default_env()
(B[m[31m-        .unwrap_or_else(|_| EnvFilter::new(level_str.as_str()));
(B[m[32m+    let env_filter =
(B[m[32m+        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level_str.as_str()));
(B[m 
[31m-    let format_str = std::env::var("FOCALPOINT_LOG_FORMAT")
(B[m[31m-        .unwrap_or_else(|_| "json".to_string());
(B[m[32m+    let format_str = std::env::var("FOCALPOINT_LOG_FORMAT").unwrap_or_else(|_| "json".to_string());
(B[m 
     let registry = tracing_subscriber::registry().with(env_filter);
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/lib.rs:120:
     // For now, we initialize OTEL config but don't panic on failure.
     // In production, you would wire this with tracing-opentelemetry + opentelemetry-otlp.
     // This is simplified to avoid runtime dependency on the tokio runtime.
[31m-    info!(endpoint = endpoint, "OpenTelemetry OTLP export configured (local export ready)");
(B[m[32m+    info!(
(B[m[32m+        endpoint = endpoint,
(B[m[32m+        "OpenTelemetry OTLP export configured (local export ready)"
(B[m[32m+    );
(B[m     Ok(())
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/lib.rs:159:
             .or_else(|| std::env::var("FOCALPOINT_LOG_LEVEL").ok())
             .unwrap_or_else(|| "info".to_string());
         assert_eq!(level_str, "info");
[31m-        let format_str = std::env::var("FOCALPOINT_LOG_FORMAT")
(B[m[31m-            .unwrap_or_else(|_| "json".to_string());
(B[m[32m+        let format_str =
(B[m[32m+            std::env::var("FOCALPOINT_LOG_FORMAT").unwrap_or_else(|_| "json".to_string());
(B[m         assert_eq!(format_str, "pretty");
         std::env::remove_var("FOCALPOINT_LOG_FORMAT");
         std::env::remove_var("FOCALPOINT_LOG_LEVEL");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/metrics.rs:4:
 //! Counters and histograms are thread-safe and can be incremented from any span context.
 
 use parking_lot::RwLock;
[31m-use prometheus::{
(B[m[31m-    HistogramVec, IntCounterVec, Registry,
(B[m[31m-};
(B[m[32m+use prometheus::{HistogramVec, IntCounterVec, Registry};
(B[m use std::sync::Arc;
 use tracing::error;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/metrics.rs:30:
         let registry = Arc::new(RwLock::new(Registry::new()));
 
         let connector_syncs = IntCounterVec::new(
[31m-            prometheus::Opts::new(
(B[m[31m-                "connector_syncs_total",
(B[m[31m-                "Total connector sync operations",
(B[m[31m-            ),
(B[m[32m+            prometheus::Opts::new("connector_syncs_total", "Total connector sync operations"),
(B[m             &["connector_id"],
         )?;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/metrics.rs:40:
         let rule_evaluations = IntCounterVec::new(
[31m-            prometheus::Opts::new(
(B[m[31m-                "rule_evaluations_total",
(B[m[31m-                "Total rule evaluations",
(B[m[31m-            ),
(B[m[32m+            prometheus::Opts::new("rule_evaluations_total", "Total rule evaluations"),
(B[m             &["rule_id"],
         )?;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/metrics.rs:166:
         let registry = MetricsRegistry::new().expect("registry creation failed");
         // Increment a counter so we have data
         registry.inc_connector_syncs("test", 1.0);
[31m-        let output = registry.gather_text_format().expect("should gather metrics");
(B[m[32m+        let output = registry
(B[m[32m+            .gather_text_format()
(B[m[32m+            .expect("should gather metrics");
(B[m         // Verify metrics were gathered
[31m-        assert!(!output.is_empty(), "metrics output should contain data after increment");
(B[m[32m+        assert!(
(B[m[32m+            !output.is_empty(),
(B[m[32m+            "metrics output should contain data after increment"
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/metrics.rs:212:
         let m2 = MetricsRegistry::global();
 
         // Both should be the same instance
[31m-        assert_eq!(
(B[m[31m-            Arc::as_ptr(&m1) as *const _,
(B[m[31m-            Arc::as_ptr(&m2) as *const _
(B[m[31m-        );
(B[m[32m+        assert_eq!(Arc::as_ptr(&m1) as *const _, Arc::as_ptr(&m2) as *const _);
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/privacy_filter.rs:19:
 
 fn patterns() -> &'static PiiPatterns {
     PII_PATTERNS.get_or_init(|| PiiPatterns {
[31m-        email: Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
(B[m[31m-            .unwrap(),
(B[m[31m-        phone: Regex::new(
(B[m[31m-            r"\+?1?[-.\s]?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}"
(B[m[32m+        email: Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
(B[m[32m+        phone: Regex::new(r"\+?1?[-.\s]?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}").unwrap(),
(B[m[32m+        token: Regex::new(
(B[m[32m+            r"(?:Bearer|token|api[_-]?key|sk[_-]?live|pk[_-]?live)[\s:]*([a-zA-Z0-9_\-\.]+)",
(B[m         )
         .unwrap(),
[31m-        token: Regex::new(r"(?:Bearer|token|api[_-]?key|sk[_-]?live|pk[_-]?live)[\s:]*([a-zA-Z0-9_\-\.]+)")
(B[m[31m-            .unwrap(),
(B[m         url_with_auth: Regex::new(r"https?://[^/\s:]+:[^/\s@]+@").unwrap(),
     })
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-observability/src/privacy_filter.rs:48:
         let mut result = value.to_string();
 
         // Email
[31m-        result = p
(B[m[31m-            .email
(B[m[31m-            .replace_all(&result, "[REDACTED_EMAIL]")
(B[m[31m-            .to_string();
(B[m[32m+        result = p.email.replace_all(&result, "[REDACTED_EMAIL]").to_string();
(B[m 
         // Phone
[31m-        result = p
(B[m[31m-            .phone
(B[m[31m-            .replace_all(&result, "[REDACTED_PHONE]")
(B[m[31m-            .to_string();
(B[m[32m+        result = p.phone.replace_all(&result, "[REDACTED_PHONE]").to_string();
(B[m 
         // API tokens
[31m-        result = p
(B[m[31m-            .token
(B[m[31m-            .replace_all(&result, "[REDACTED_TOKEN]")
(B[m[31m-            .to_string();
(B[m[32m+        result = p.token.replace_all(&result, "[REDACTED_TOKEN]").to_string();
(B[m 
         // URLs with auth
         result = p
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:98:
     RepayDebt(i64),
     AddLockout(LockoutWindow),
     ClearLockouts,
[31m-    SetStrictMode { until: DateTime<Utc> },
(B[m[32m+    SetStrictMode {
(B[m[32m+        until: DateTime<Utc>,
(B[m[32m+    },
(B[m     Clear,
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:313:
             &NoopAuditSink,
         )
         .unwrap();
[31m-        s.apply(PenaltyMutation::Clear, t(2026, 1, 1, 1), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(PenaltyMutation::Clear, t(2026, 1, 1, 1), &NoopAuditSink)
(B[m[32m+            .unwrap();
(B[m         assert_eq!(s.escalation_tier, EscalationTier::Clear);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:321:
     #[test]
     fn bypass_budget_nonnegative() {
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::GrantBypass(10), t(2026, 1, 1, 0), &NoopAuditSink).unwrap();
(B[m[31m-        s.apply(PenaltyMutation::SpendBypass(7), t(2026, 1, 1, 1), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::GrantBypass(10),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::SpendBypass(7),
(B[m[32m+            t(2026, 1, 1, 1),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(s.bypass_budget, 3);
         let err = s
[31m-            .apply(PenaltyMutation::SpendBypass(10), t(2026, 1, 1, 2), &NoopAuditSink)
(B[m[32m+            .apply(
(B[m[32m+                PenaltyMutation::SpendBypass(10),
(B[m[32m+                t(2026, 1, 1, 2),
(B[m[32m+                &NoopAuditSink,
(B[m[32m+            )
(B[m             .unwrap_err();
         assert!(matches!(err, PenaltyError::InsufficientBypass { .. }));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:335:
     fn strict_mode_auto_clears_after_expiry() {
         let mut s = PenaltyState::default();
         s.apply(
[31m-            PenaltyMutation::SetStrictMode { until: t(2026, 1, 1, 10) },
(B[m[32m+            PenaltyMutation::SetStrictMode {
(B[m[32m+                until: t(2026, 1, 1, 10),
(B[m[32m+            },
(B[m             t(2026, 1, 1, 9),
             &NoopAuditSink,
         )
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:342:
         .unwrap();
         assert!(s.is_strict(t(2026, 1, 1, 9)));
[31m-        s.apply(PenaltyMutation::ClearLockouts, t(2026, 1, 1, 11), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::ClearLockouts,
(B[m[32m+            t(2026, 1, 1, 11),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert!(!s.is_strict(t(2026, 1, 1, 11)));
         assert!(s.strict_mode_until.is_none());
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:356:
             reason: "x".into(),
             rigidity: Rigidity::Hard,
         });
[31m-        s.apply(PenaltyMutation::GrantBypass(0), t(2026, 1, 1, 5), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::GrantBypass(0),
(B[m[32m+            t(2026, 1, 1, 5),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert!(s.lockout_windows.is_empty());
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:364:
     #[test]
     fn quote_happy_path() {
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::GrantBypass(10), t(2026, 1, 1, 0), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::GrantBypass(10),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         let q = s.quote_bypass(4).unwrap();
         assert_eq!(q.cost, 4);
         assert_eq!(q.remaining_after, 6);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:413:
     fn escalate_records_audit_line() {
         let mut s = PenaltyState::default();
         let sink = CapturingAuditSink::new();
[31m-        s.apply(PenaltyMutation::Escalate(EscalationTier::Strict), t(2026, 1, 1, 0), &sink)
(B[m[31m-            .unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::Escalate(EscalationTier::Strict),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &sink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         let snap = sink.snapshot();
         assert_eq!(snap.len(), 1);
         assert_eq!(snap[0].0, "penalty.escalate");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:426:
     fn failed_escalation_does_not_audit() {
         let mut s = PenaltyState::default();
         let sink = CapturingAuditSink::new();
[31m-        s.apply(PenaltyMutation::Escalate(EscalationTier::Restricted), t(2026, 1, 1, 0), &sink)
(B[m[31m-            .unwrap();
(B[m[31m-        let _ =
(B[m[31m-            s.apply(PenaltyMutation::Escalate(EscalationTier::Warning), t(2026, 1, 1, 1), &sink);
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::Escalate(EscalationTier::Restricted),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &sink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        let _ = s.apply(
(B[m[32m+            PenaltyMutation::Escalate(EscalationTier::Warning),
(B[m[32m+            t(2026, 1, 1, 1),
(B[m[32m+            &sink,
(B[m[32m+        );
(B[m         // Only the first succeeded and audited; the rejected downgrade did not.
         assert_eq!(sink.len(), 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:439:
     fn bypass_spend_and_grant_audit() {
         let mut s = PenaltyState::default();
         let sink = CapturingAuditSink::new();
[31m-        s.apply(PenaltyMutation::GrantBypass(10), t(2026, 1, 1, 0), &sink).unwrap();
(B[m[31m-        s.apply(PenaltyMutation::SpendBypass(4), t(2026, 1, 1, 1), &sink).unwrap();
(B[m[32m+        s.apply(PenaltyMutation::GrantBypass(10), t(2026, 1, 1, 0), &sink)
(B[m[32m+            .unwrap();
(B[m[32m+        s.apply(PenaltyMutation::SpendBypass(4), t(2026, 1, 1, 1), &sink)
(B[m[32m+            .unwrap();
(B[m         let snap = sink.snapshot();
         assert_eq!(snap.len(), 2);
         assert_eq!(snap[0].0, "penalty.grant_bypass");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:453:
     #[test]
     fn spend_or_debt_drains_budget_then_accrues_debt() {
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::GrantBypass(10), t(2026, 1, 1, 0), &NoopAuditSink).unwrap();
(B[m[31m-        s.apply(PenaltyMutation::SpendBypassOrDebt(15), t(2026, 1, 1, 1), &NoopAuditSink)
(B[m[31m-            .unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::GrantBypass(10),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::SpendBypassOrDebt(15),
(B[m[32m+            t(2026, 1, 1, 1),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(s.bypass_budget, 0);
         assert_eq!(s.debt_balance, 5);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:463:
     #[test]
     fn spend_or_debt_no_budget_all_to_debt() {
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::SpendBypassOrDebt(7), t(2026, 1, 1, 0), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::SpendBypassOrDebt(7),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(s.bypass_budget, 0);
         assert_eq!(s.debt_balance, 7);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:472:
     fn spend_or_debt_rejects_negative() {
         let mut s = PenaltyState::default();
         let err = s
[31m-            .apply(PenaltyMutation::SpendBypassOrDebt(-1), t(2026, 1, 1, 0), &NoopAuditSink)
(B[m[32m+            .apply(
(B[m[32m+                PenaltyMutation::SpendBypassOrDebt(-1),
(B[m[32m+                t(2026, 1, 1, 0),
(B[m[32m+                &NoopAuditSink,
(B[m[32m+            )
(B[m             .unwrap_err();
         assert!(matches!(err, PenaltyError::NegativeAmount(_)));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:480:
     #[test]
     fn repay_debt_reduces_balance_clamps_at_zero() {
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::SpendBypassOrDebt(10), t(2026, 1, 1, 0), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::SpendBypassOrDebt(10),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(s.debt_balance, 10);
[31m-        s.apply(PenaltyMutation::RepayDebt(6), t(2026, 1, 1, 1), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::RepayDebt(6),
(B[m[32m+            t(2026, 1, 1, 1),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(s.debt_balance, 4);
         // Overpayment — clamps, does NOT credit budget.
[31m-        s.apply(PenaltyMutation::RepayDebt(100), t(2026, 1, 1, 2), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::RepayDebt(100),
(B[m[32m+            t(2026, 1, 1, 2),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(s.debt_balance, 0);
         assert_eq!(s.bypass_budget, 0);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:493:
     #[test]
     fn clear_zeros_debt_balance() {
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::SpendBypassOrDebt(5), t(2026, 1, 1, 0), &NoopAuditSink).unwrap();
(B[m[31m-        s.apply(PenaltyMutation::Clear, t(2026, 1, 1, 1), &NoopAuditSink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::SpendBypassOrDebt(5),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        s.apply(PenaltyMutation::Clear, t(2026, 1, 1, 1), &NoopAuditSink)
(B[m[32m+            .unwrap();
(B[m         assert_eq!(s.debt_balance, 0);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-penalties/src/lib.rs:502:
     fn debt_mutations_emit_audit_lines() {
         let sink = CapturingAuditSink::new();
         let mut s = PenaltyState::default();
[31m-        s.apply(PenaltyMutation::SpendBypassOrDebt(3), t(2026, 1, 1, 0), &sink).unwrap();
(B[m[31m-        s.apply(PenaltyMutation::RepayDebt(1), t(2026, 1, 1, 1), &sink).unwrap();
(B[m[32m+        s.apply(
(B[m[32m+            PenaltyMutation::SpendBypassOrDebt(3),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &sink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        s.apply(PenaltyMutation::RepayDebt(1), t(2026, 1, 1, 1), &sink)
(B[m[32m+            .unwrap();
(B[m         let snap = sink.snapshot();
         assert_eq!(snap.len(), 2);
         assert_eq!(snap[0].0, "penalty.spend_bypass_or_debt");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:36:
 
 impl DurationSpec {
     pub fn fixed(d: Duration) -> Self {
[31m-        Self { fixed: Some(d), estimate: None }
(B[m[32m+        Self {
(B[m[32m+            fixed: Some(d),
(B[m[32m+            estimate: None,
(B[m[32m+        }
(B[m     }
 
     pub fn estimated(p50: Duration, p90: Duration) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:43:
[31m-        Self { fixed: None, estimate: Some(Estimate { p50, p90 }) }
(B[m[32m+        Self {
(B[m[32m+            fixed: None,
(B[m[32m+            estimate: Some(Estimate { p50, p90 }),
(B[m[32m+        }
(B[m     }
 
     /// Best-guess duration to feed the scheduler. Fixed wins; else p90 (so we
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:77:
     }
 
     pub fn clamped(weight: f32) -> Self {
[31m-        Self { weight: weight.clamp(0.0, 1.0) }
(B[m[32m+        Self {
(B[m[32m+            weight: weight.clamp(0.0, 1.0),
(B[m[32m+        }
(B[m     }
 
     /// Returns a new Priority whose weight is nudged toward 1.0 by `bumps`
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:87:
         for _ in 0..bumps {
             w += (1.0 - w) * 0.10;
         }
[31m-        Priority { weight: w.clamp(0.0, 1.0) }
(B[m[32m+        Priority {
(B[m[32m+            weight: w.clamp(0.0, 1.0),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:112:
 
 impl Deadline {
     pub fn none() -> Self {
[31m-        Self { when: None, rigidity: Rigidity::Soft }
(B[m[32m+        Self {
(B[m[32m+            when: None,
(B[m[32m+            rigidity: Rigidity::Soft,
(B[m[32m+        }
(B[m     }
 
     pub fn hard(when: DateTime<Utc>) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:119:
[31m-        Self { when: Some(when), rigidity: Rigidity::Hard }
(B[m[32m+        Self {
(B[m[32m+            when: Some(when),
(B[m[32m+            rigidity: Rigidity::Hard,
(B[m[32m+        }
(B[m     }
 
     pub fn soft(when: DateTime<Utc>) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:123:
[31m-        Self { when: Some(when), rigidity: Rigidity::Soft }
(B[m[32m+        Self {
(B[m[32m+            when: Some(when),
(B[m[32m+            rigidity: Rigidity::Soft,
(B[m[32m+        }
(B[m     }
 
     pub fn is_hard(&self) -> bool {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:320:
 
 impl TaskStore for MemoryTaskStore {
     fn list(&self, user_id: uuid::Uuid) -> anyhow::Result<Vec<Task>> {
[31m-        let g =
(B[m[31m-            self.inner.lock().map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m[31m-        Ok(g.iter().filter(|(u, _)| *u == user_id).map(|(_, t)| t.clone()).collect())
(B[m[32m+        let g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m[32m+        Ok(g.iter()
(B[m[32m+            .filter(|(u, _)| *u == user_id)
(B[m[32m+            .map(|(_, t)| t.clone())
(B[m[32m+            .collect())
(B[m     }
 
     fn get(&self, id: uuid::Uuid) -> anyhow::Result<Option<Task>> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:329:
[31m-        let g =
(B[m[31m-            self.inner.lock().map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m[32m+        let g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m         Ok(g.iter().find(|(_, t)| t.id == id).map(|(_, t)| t.clone()))
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:334:
     fn upsert(&self, user_id: uuid::Uuid, task: &Task) -> anyhow::Result<()> {
[31m-        let mut g =
(B[m[31m-            self.inner.lock().map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m         if let Some(slot) = g.iter_mut().find(|(_, t)| t.id == task.id) {
             slot.1 = task.clone();
         } else {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:343:
     }
 
     fn delete(&self, id: uuid::Uuid) -> anyhow::Result<bool> {
[31m-        let mut g =
(B[m[31m-            self.inner.lock().map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m[32m+        let mut g = self
(B[m[32m+            .inner
(B[m[32m+            .lock()
(B[m[32m+            .map_err(|e| anyhow::anyhow!("memory task store poisoned: {e}"))?;
(B[m         let before = g.len();
         g.retain(|(_, t)| t.id != id);
         Ok(g.len() != before)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:414:
     fn deadline_hardness_reflects_rigidity() {
         let hard = Deadline::hard(t0());
         let soft = Deadline::soft(t0());
[31m-        let semi =
(B[m[31m-            Deadline { when: Some(t0()), rigidity: Rigidity::Semi(RigidityCost::CreditCost(10)) };
(B[m[32m+        let semi = Deadline {
(B[m[32m+            when: Some(t0()),
(B[m[32m+            rigidity: Rigidity::Semi(RigidityCost::CreditCost(10)),
(B[m[32m+        };
(B[m         let none = Deadline::none();
         assert!(hard.is_hard());
         assert!(!soft.is_hard());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:439:
             ..Task::new("compose", DurationSpec::fixed(Duration::hours(1)), t0())
         };
         assert_eq!(task.constraints.len(), 3);
[31m-        assert!(matches!(task.constraints[2], Constraint::EnergyTier(EnergyTier::DeepFocus)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            task.constraints[2],
(B[m[32m+            Constraint::EnergyTier(EnergyTier::DeepFocus)
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-PLAN-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:483:
         let store = MemoryTaskStore::new();
         let alice = Uuid::new_v4();
         let bob = Uuid::new_v4();
[31m-        let a1 = Task::new("alice-one", DurationSpec::fixed(Duration::minutes(25)), t0());
(B[m[31m-        let a2 = Task::new("alice-two", DurationSpec::fixed(Duration::minutes(50)), t0());
(B[m[32m+        let a1 = Task::new(
(B[m[32m+            "alice-one",
(B[m[32m+            DurationSpec::fixed(Duration::minutes(25)),
(B[m[32m+            t0(),
(B[m[32m+        );
(B[m[32m+        let a2 = Task::new(
(B[m[32m+            "alice-two",
(B[m[32m+            DurationSpec::fixed(Duration::minutes(50)),
(B[m[32m+            t0(),
(B[m[32m+        );
(B[m         let b1 = Task::new("bob-one", DurationSpec::fixed(Duration::minutes(30)), t0());
         store.upsert(alice, &a1).unwrap();
         store.upsert(alice, &a2).unwrap();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-planning/src/lib.rs:492:
 
         assert_eq!(store.list(alice).unwrap().len(), 2);
         assert_eq!(store.list(bob).unwrap().len(), 1);
[31m-        assert_eq!(store.get(a1.id).unwrap().as_ref().map(|t| t.title.as_str()), Some("alice-one"));
(B[m[32m+        assert_eq!(
(B[m[32m+            store.get(a1.id).unwrap().as_ref().map(|t| t.title.as_str()),
(B[m[32m+            Some("alice-one")
(B[m[32m+        );
(B[m 
         // Upsert updates in place.
         let mut a1_mut = a1.clone();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:5:
 //! URL allowlist enforced from plugin.toml `[capabilities.http.allowlist]`.
 
 use crate::PluginError;
[32m+use anyhow::Result;
(B[m[32m+use chrono::{DateTime, Duration, Utc};
(B[m use serde::{Deserialize, Serialize};
 use std::collections::HashMap;
 use std::sync::Mutex;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:11:
[31m-use chrono::{DateTime, Duration, Utc};
(B[m[31m-use anyhow::Result;
(B[m 
 /// HTTP request sent by plugin (serialized in linear memory).
 #[derive(Debug, Clone, Serialize, Deserialize)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:16:
 pub struct HttpRequest {
[31m-    pub method: String,      // GET, POST, PUT, DELETE, etc.
(B[m[32m+    pub method: String, // GET, POST, PUT, DELETE, etc.
(B[m     pub url: String,
     pub headers: HashMap<String, String>,
     pub body: Option<Vec<u8>>,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:58:
             if let Some(domain) = parsed.domain() {
                 // Check exact match or wildcard.
                 for allowed in &self.allowlist {
[31m-                    if allowed == domain || allowed.starts_with("*.") && domain.ends_with(&allowed[1..]) {
(B[m[32m+                    if allowed == domain
(B[m[32m+                        || allowed.starts_with("*.") && domain.ends_with(&allowed[1..])
(B[m[32m+                    {
(B[m                         return true;
                     }
                 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:97:
     ) -> Result<HttpResponse, PluginError> {
         // Check allowlist.
         if !self.is_url_allowed(&req.url) {
[31m-            return Err(PluginError::CapabilityDenied(
(B[m[31m-                format!("URL not in allowlist: {}", req.url),
(B[m[31m-            ));
(B[m[32m+            return Err(PluginError::CapabilityDenied(format!(
(B[m[32m+                "URL not in allowlist: {}",
(B[m[32m+                req.url
(B[m[32m+            )));
(B[m         }
 
         // Check rate limit.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:135:
         // Execute with 5s timeout.
         let timeout = std::time::Duration::from_secs(5);
         let response = client
[31m-            .execute(request.timeout(timeout).build().map_err(|e| {
(B[m[31m-                PluginError::ConfigError(format!("HTTP build error: {}", e))
(B[m[31m-            })?)
(B[m[32m+            .execute(
(B[m[32m+                request
(B[m[32m+                    .timeout(timeout)
(B[m[32m+                    .build()
(B[m[32m+                    .map_err(|e| PluginError::ConfigError(format!("HTTP build error: {}", e)))?,
(B[m[32m+            )
(B[m             .await
[31m-            .map_err(|e| PluginError::RuntimeError(anyhow::anyhow!("HTTP request failed: {}", e)))?;
(B[m[32m+            .map_err(|e| {
(B[m[32m+                PluginError::RuntimeError(anyhow::anyhow!("HTTP request failed: {}", e))
(B[m[32m+            })?;
(B[m 
         let status = response.status().as_u16();
         let mut headers = HashMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/capabilities/http.rs:164:
             ));
         }
 
[31m-        Ok(HttpResponse { status, headers, body })
(B[m[32m+        Ok(HttpResponse {
(B[m[32m+            status,
(B[m[32m+            headers,
(B[m[32m+            body,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:111:
 events = []
 "#;
 
[31m-        let manifest: PluginManifest = toml::from_str(manifest_toml)
(B[m[31m-            .expect("failed to parse manifest");
(B[m[32m+        let manifest: PluginManifest =
(B[m[32m+            toml::from_str(manifest_toml).expect("failed to parse manifest");
(B[m 
         assert_eq!(manifest.plugin.name, "connector-hello");
         assert_eq!(manifest.plugin.version, "0.1.0");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/lib.rs:121:
 
     #[test]
     fn test_signature_verification_flow() {
[31m-        use ed25519_dalek::{SigningKey, Signer};
(B[m[32m+        use ed25519_dalek::{Signer, SigningKey};
(B[m         use rand_core::OsRng;
         use sha2::Digest;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/plugin.rs:57:
 
         let ndjson = event.to_ndjson();
         assert!(ndjson.ends_with('\n'));
[31m-        let parsed: NdjsonEvent = serde_json::from_str(ndjson.trim())
(B[m[31m-            .expect("failed to parse NDJSON");
(B[m[32m+        let parsed: NdjsonEvent =
(B[m[32m+            serde_json::from_str(ndjson.trim()).expect("failed to parse NDJSON");
(B[m         assert_eq!(parsed.id, event.id);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/runtime.rs:76:
     pub fn poll(&self, config_json: &[u8]) -> Result<Vec<u8>, PluginError> {
         // Phase-1: Basic execution cap (memory and timeout enforced by OS + timeout handler).
         if config_json.len() > 1024 * 1024 {
[31m-            return Err(PluginError::ConfigError(
(B[m[31m-                "Config exceeds 1MB".to_string(),
(B[m[31m-            ));
(B[m[32m+            return Err(PluginError::ConfigError("Config exceeds 1MB".to_string()));
(B[m         }
 
         // Placeholder: actual invocation requires full module export inspection.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-plugin-sdk/src/signing.rs:1:
 //! Plugin signature verification using Ed25519.
 
[31m-use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer};
(B[m[32m+use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
(B[m use serde::{Deserialize, Serialize};
 use sha2::{Digest, Sha256};
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/benches/focus_policy_benchmarks.rs:1:
[31m-use criterion::{black_box, criterion_group, criterion_main, Criterion};
(B[m[31m-use focus_policy::{EnforcementPolicy, BlockProfile, PolicyBuilder};
(B[m use chrono::Utc;
[32m+use criterion::{black_box, criterion_group, criterion_main, Criterion};
(B[m[32m+use focus_policy::{BlockProfile, EnforcementPolicy, PolicyBuilder};
(B[m use std::collections::HashMap;
 
 fn policy_builder_creation(c: &mut Criterion) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/benches/focus_policy_benchmarks.rs:41:
                 categories: (0..10)
                     .map(|i| format!("category-{}", i))
                     .collect::<Vec<_>>(),
[31m-                exceptions: (0..5)
(B[m[31m-                    .map(|i| format!("app-{}", i))
(B[m[31m-                    .collect::<Vec<_>>(),
(B[m[32m+                exceptions: (0..5).map(|i| format!("app-{}", i)).collect::<Vec<_>>(),
(B[m             };
             let _ = black_box(profile);
         });
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:117:
             let mut local: HashMap<String, ProfileState> = HashMap::new();
             for action in actions {
                 match action {
[31m-                    Action::Block { profile, duration, rigidity } => {
(B[m[32m+                    Action::Block {
(B[m[32m+                        profile,
(B[m[32m+                        duration,
(B[m[32m+                        rigidity,
(B[m[32m+                    } => {
(B[m                         let rigidity = rigidity.clone();
[31m-                        local.entry(profile.clone()).or_insert_with(|| ProfileState::Blocked {
(B[m[31m-                            ends_at: now + clamp_duration(*duration),
(B[m[31m-                            rigidity,
(B[m[31m-                        });
(B[m[32m+                        local
(B[m[32m+                            .entry(profile.clone())
(B[m[32m+                            .or_insert_with(|| ProfileState::Blocked {
(B[m[32m+                                ends_at: now + clamp_duration(*duration),
(B[m[32m+                                rigidity,
(B[m[32m+                            });
(B[m                     }
                     Action::Unblock { profile } => {
                         // Force-overwrite within the same decision.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:139:
             // Accumulate scheduled windows for any Block action (informational).
             for action in actions {
                 if let Action::Block { duration, .. } = action {
[31m-                    scheduled_windows
(B[m[31m-                        .push(Window { starts_at: now, ends_at: now + clamp_duration(*duration) });
(B[m[32m+                    scheduled_windows.push(Window {
(B[m[32m+                        starts_at: now,
(B[m[32m+                        ends_at: now + clamp_duration(*duration),
(B[m[32m+                    });
(B[m                 }
             }
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:147:
 
[31m-        let any_blocked =
(B[m[31m-            profile_states.values().any(|s| matches!(s, ProfileState::Blocked { .. }));
(B[m[32m+        let any_blocked = profile_states
(B[m[32m+            .values()
(B[m[32m+            .any(|s| matches!(s, ProfileState::Blocked { .. }));
(B[m 
         // Union of targets across every Blocked profile, deduped in insertion
         // order. Only Blocked profiles contribute; Unblocked ones cannot
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:221:
 #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
 pub enum EnforcementCallback {
     /// The driver successfully applied `policy_id`.
[31m-    ApplySucceeded { policy_id: uuid::Uuid, at: DateTime<Utc> },
(B[m[32m+    ApplySucceeded {
(B[m[32m+        policy_id: uuid::Uuid,
(B[m[32m+        at: DateTime<Utc>,
(B[m[32m+    },
(B[m     /// Apply failed with a reason the driver can share (e.g. user revoked
     /// FamilyControls authorization, Accessibility service killed).
[31m-    ApplyFailed { policy_id: uuid::Uuid, reason: String, at: DateTime<Utc> },
(B[m[32m+    ApplyFailed {
(B[m[32m+        policy_id: uuid::Uuid,
(B[m[32m+        reason: String,
(B[m[32m+        at: DateTime<Utc>,
(B[m[32m+    },
(B[m     /// Retract succeeded (policy no longer in effect).
[31m-    RetractSucceeded { policy_id: uuid::Uuid, at: DateTime<Utc> },
(B[m[32m+    RetractSucceeded {
(B[m[32m+        policy_id: uuid::Uuid,
(B[m[32m+        at: DateTime<Utc>,
(B[m[32m+    },
(B[m     /// User attempted to launch a target currently in a Blocked state.
     /// `target_key` is the stringified AppTarget (see `app_target_key`).
[31m-    BlockAttempted { target_key: String, profile: String, at: DateTime<Utc> },
(B[m[32m+    BlockAttempted {
(B[m[32m+        target_key: String,
(B[m[32m+        profile: String,
(B[m[32m+        at: DateTime<Utc>,
(B[m[32m+    },
(B[m     /// User invoked the bypass UI. Quote/confirmation is upstream; this is
     /// only the observed intent.
     BypassRequested { profile: String, at: DateTime<Utc> },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:342:
         );
         let p = PolicyBuilder::from_rule_decisions(&[d], t(), &NoopAuditSink);
         assert!(p.active);
[31m-        assert!(matches!(p.profile_states.get("games"), Some(ProfileState::Blocked { .. })));
(B[m[32m+        assert!(matches!(
(B[m[32m+            p.profile_states.get("games"),
(B[m[32m+            Some(ProfileState::Blocked { .. })
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-ENF-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:356:
                     duration: Duration::minutes(30),
                     rigidity: Rigidity::Hard,
                 },
[31m-                Action::Unblock { profile: "games".into() },
(B[m[32m+                Action::Unblock {
(B[m[32m+                    profile: "games".into(),
(B[m[32m+                },
(B[m             ],
         );
         let p = PolicyBuilder::from_rule_decisions(&[d], t(), &NoopAuditSink);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:363:
[31m-        assert_eq!(p.profile_states.get("games"), Some(&ProfileState::Unblocked));
(B[m[32m+        assert_eq!(
(B[m[32m+            p.profile_states.get("games"),
(B[m[32m+            Some(&ProfileState::Unblocked)
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-ENF-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:367:
     #[test]
     fn higher_priority_rule_wins_across_decisions() {
[31m-        let low = fired(1, vec![Action::Unblock { profile: "social".into() }]);
(B[m[32m+        let low = fired(
(B[m[32m+            1,
(B[m[32m+            vec![Action::Unblock {
(B[m[32m+                profile: "social".into(),
(B[m[32m+            }],
(B[m[32m+        );
(B[m         let high = fired(
             100,
             vec![Action::Block {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:377:
         );
         // Input order intentionally low-first to prove sort.
         let p = PolicyBuilder::from_rule_decisions(&[low, high], t(), &NoopAuditSink);
[31m-        assert!(matches!(p.profile_states.get("social"), Some(ProfileState::Blocked { .. })));
(B[m[32m+        assert!(matches!(
(B[m[32m+            p.profile_states.get("social"),
(B[m[32m+            Some(ProfileState::Blocked { .. })
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-ENF-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:404:
                     duration: Duration::minutes(30),
                     rigidity: Rigidity::Hard,
                 },
[31m-                Action::Unblock { profile: "education".into() },
(B[m[32m+                Action::Unblock {
(B[m[32m+                    profile: "education".into(),
(B[m[32m+                },
(B[m             ],
         );
         let p = PolicyBuilder::from_rule_decisions(&[d], t(), &NoopAuditSink);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:411:
[31m-        assert!(matches!(p.profile_states.get("games"), Some(ProfileState::Blocked { .. })));
(B[m[31m-        assert_eq!(p.profile_states.get("education"), Some(&ProfileState::Unblocked));
(B[m[32m+        assert!(matches!(
(B[m[32m+            p.profile_states.get("games"),
(B[m[32m+            Some(ProfileState::Blocked { .. })
(B[m[32m+        ));
(B[m[32m+        assert_eq!(
(B[m[32m+            p.profile_states.get("education"),
(B[m[32m+            Some(&ProfileState::Unblocked)
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-STATE-004
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:435:
     // Traces to: FR-STATE-004
     #[test]
     fn policy_audit_payload_includes_decision_ids() {
[31m-        let d1 = fired(10, vec![Action::Unblock { profile: "x".into() }]);
(B[m[31m-        let d2 = fired(5, vec![Action::Unblock { profile: "y".into() }]);
(B[m[32m+        let d1 = fired(
(B[m[32m+            10,
(B[m[32m+            vec![Action::Unblock {
(B[m[32m+                profile: "x".into(),
(B[m[32m+            }],
(B[m[32m+        );
(B[m[32m+        let d2 = fired(
(B[m[32m+            5,
(B[m[32m+            vec![Action::Unblock {
(B[m[32m+                profile: "y".into(),
(B[m[32m+            }],
(B[m[32m+        );
(B[m         let ids: Vec<String> = vec![d1.rule_id.to_string(), d2.rule_id.to_string()];
         let sink = CapturingAuditSink::new();
         let _ = PolicyBuilder::from_rule_decisions(&[d1, d2], t(), &sink);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:443:
         let snap = sink.snapshot();
[31m-        let decisions = snap[0].2["decision_ids"].as_array().expect("decision_ids array");
(B[m[31m-        let got: Vec<String> = decisions.iter().map(|v| v.as_str().unwrap().to_string()).collect();
(B[m[32m+        let decisions = snap[0].2["decision_ids"]
(B[m[32m+            .as_array()
(B[m[32m+            .expect("decision_ids array");
(B[m[32m+        let got: Vec<String> = decisions
(B[m[32m+            .iter()
(B[m[32m+            .map(|v| v.as_str().unwrap().to_string())
(B[m[32m+            .collect();
(B[m         // Order in payload matches input order (we preserve input order).
         assert_eq!(got, ids);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:458:
                     duration: Duration::minutes(30),
                     rigidity: Rigidity::Hard,
                 },
[31m-                Action::Unblock { profile: "education".into() },
(B[m[32m+                Action::Unblock {
(B[m[32m+                    profile: "education".into(),
(B[m[32m+                },
(B[m             ],
         );
         let sink = CapturingAuditSink::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:489:
                 AppTarget::Domain("twitter.com".into()),
             ],
         );
[31m-        let p = PolicyBuilder::from_rule_decisions_with_targets(
(B[m[31m-            &[d],
(B[m[31m-            &targets,
(B[m[31m-            t(),
(B[m[31m-            &NoopAuditSink,
(B[m[31m-        );
(B[m[32m+        let p =
(B[m[32m+            PolicyBuilder::from_rule_decisions_with_targets(&[d], &targets, t(), &NoopAuditSink);
(B[m         assert_eq!(p.app_targets.len(), 3);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:517:
             }],
         );
         let mut targets: HashMap<String, Vec<AppTarget>> = HashMap::new();
[31m-        targets.insert("social".into(), vec![AppTarget::Domain("twitter.com".into())]);
(B[m[32m+        targets.insert(
(B[m[32m+            "social".into(),
(B[m[32m+            vec![AppTarget::Domain("twitter.com".into())],
(B[m[32m+        );
(B[m         targets.insert("news".into(), vec![AppTarget::Domain("twitter.com".into())]);
         let p = PolicyBuilder::from_rule_decisions_with_targets(
             &[d1, d2],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:525:
             t(),
             &NoopAuditSink,
         );
[31m-        assert_eq!(p.app_targets.len(), 1, "twitter.com appeared in both profiles, should dedupe");
(B[m[32m+        assert_eq!(
(B[m[32m+            p.app_targets.len(),
(B[m[32m+            1,
(B[m[32m+            "twitter.com appeared in both profiles, should dedupe"
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:538:
                     duration: Duration::hours(1),
                     rigidity: Rigidity::Hard,
                 },
[31m-                Action::Unblock { profile: "education".into() },
(B[m[32m+                Action::Unblock {
(B[m[32m+                    profile: "education".into(),
(B[m[32m+                },
(B[m             ],
         );
         let mut targets: HashMap<String, Vec<AppTarget>> = HashMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:545:
         targets.insert("social".into(), vec![AppTarget::BundleId("com.x".into())]);
[31m-        targets.insert("education".into(), vec![AppTarget::BundleId("com.edu".into())]);
(B[m[31m-        let p = PolicyBuilder::from_rule_decisions_with_targets(
(B[m[31m-            &[d],
(B[m[31m-            &targets,
(B[m[31m-            t(),
(B[m[31m-            &NoopAuditSink,
(B[m[32m+        targets.insert(
(B[m[32m+            "education".into(),
(B[m[32m+            vec![AppTarget::BundleId("com.edu".into())],
(B[m         );
[32m+        let p =
(B[m[32m+            PolicyBuilder::from_rule_decisions_with_targets(&[d], &targets, t(), &NoopAuditSink);
(B[m         // Only "social" is blocked → only com.x present.
         let bundles: Vec<_> = p
             .app_targets
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:577:
         port.record(EnforcementCallback::AuthorizationRevoked { at: now });
         let snap = port.snapshot();
         assert_eq!(snap.len(), 3);
[31m-        assert!(matches!(snap[0], EnforcementCallback::ApplySucceeded { .. }));
(B[m[32m+        assert!(matches!(
(B[m[32m+            snap[0],
(B[m[32m+            EnforcementCallback::ApplySucceeded { .. }
(B[m[32m+        ));
(B[m         if let EnforcementCallback::BlockAttempted { target_key, .. } = &snap[1] {
             assert_eq!(target_key, "bundle:com.x");
         } else {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:589:
     fn callback_roundtrips_serde() {
         let now = Utc::now();
         let cases = vec![
[31m-            EnforcementCallback::ApplySucceeded { policy_id: Uuid::new_v4(), at: now },
(B[m[32m+            EnforcementCallback::ApplySucceeded {
(B[m[32m+                policy_id: Uuid::new_v4(),
(B[m[32m+                at: now,
(B[m[32m+            },
(B[m             EnforcementCallback::ApplyFailed {
                 policy_id: Uuid::new_v4(),
                 reason: "auth revoked".into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:596:
                 at: now,
             },
[31m-            EnforcementCallback::RetractSucceeded { policy_id: Uuid::new_v4(), at: now },
(B[m[32m+            EnforcementCallback::RetractSucceeded {
(B[m[32m+                policy_id: Uuid::new_v4(),
(B[m[32m+                at: now,
(B[m[32m+            },
(B[m             EnforcementCallback::BlockAttempted {
                 target_key: "bundle:com.x".into(),
                 profile: "social".into(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:602:
                 at: now,
             },
[31m-            EnforcementCallback::BypassRequested { profile: "games".into(), at: now },
(B[m[32m+            EnforcementCallback::BypassRequested {
(B[m[32m+                profile: "games".into(),
(B[m[32m+                at: now,
(B[m[32m+            },
(B[m             EnforcementCallback::AuthorizationRevoked { at: now },
         ];
         for c in cases {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:639:
                 rigidity: Rigidity::Hard,
             }],
         );
[31m-        let active_policy =
(B[m[31m-            PolicyBuilder::from_rule_decisions(&[block_decision], t(), &sink);
(B[m[32m+        let active_policy = PolicyBuilder::from_rule_decisions(&[block_decision], t(), &sink);
(B[m         assert!(active_policy.active);
 
         // Verify activation was recorded
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-policy/src/lib.rs:655:
         let skipped_decision = PrioritizedDecision {
             rule_id: Uuid::new_v4(),
             priority: 5,
[31m-            decision: RuleDecision::Skipped { reason: "not triggered".into() },
(B[m[32m+            decision: RuleDecision::Skipped {
(B[m[32m+                reason: "not triggered".into(),
(B[m[32m+            },
(B[m         };
[31m-        let inactive_policy =
(B[m[31m-            PolicyBuilder::from_rule_decisions(&[skipped_decision], t(), &sink2);
(B[m[32m+        let inactive_policy = PolicyBuilder::from_rule_decisions(&[skipped_decision], t(), &sink2);
(B[m         assert!(!inactive_policy.active);
 
         // Verify deactivation was recorded
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-release-bot/src/lib.rs:142:
     let message = payload.to_discord_message();
     let client = reqwest::Client::new();
 
[31m-    let response = client
(B[m[31m-        .post(webhook_url)
(B[m[31m-        .json(&message)
(B[m[31m-        .send()
(B[m[31m-        .await?;
(B[m[32m+    let response = client.post(webhook_url).json(&message).send().await?;
(B[m 
     if !response.status().is_success() {
         return Err(BotError::WebhookError(format!(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-release-bot/src/lib.rs:171:
     let message = payload.to_discord_message();
     let client = reqwest::blocking::Client::new();
 
[31m-    let response = client
(B[m[31m-        .post(webhook_url)
(B[m[31m-        .json(&message)
(B[m[31m-        .send()?;
(B[m[32m+    let response = client.post(webhook_url).json(&message).send()?;
(B[m 
     if !response.status().is_success() {
         return Err(BotError::WebhookError(format!(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-release-bot/src/lib.rs:194:
     #[test]
     fn test_release_notes_serialization() {
         let payload = ReleaseNotesPayload::new("0.0.4")
[31m-            .with_category(
(B[m[31m-                "Added",
(B[m[31m-                vec!["New release notes generator".to_string()],
(B[m[31m-            )
(B[m[32m+            .with_category("Added", vec!["New release notes generator".to_string()])
(B[m             .with_category("Fixed", vec!["CLI formatting".to_string()]);
 
         let msg = payload.to_discord_message();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:118:
                         for action in actions {
                             match action {
                                 Action::GrantCredit { amount } => {
[31m-                                    *action_deltas.entry("credit_delta".to_string()).or_insert(0) +=
(B[m[31m-                                        amount;
(B[m[32m+                                    *action_deltas
(B[m[32m+                                        .entry("credit_delta".to_string())
(B[m[32m+                                        .or_insert(0) += amount;
(B[m                                 }
                                 Action::DeductCredit { amount } => {
[31m-                                    *action_deltas.entry("credit_delta".to_string()).or_insert(0) -=
(B[m[31m-                                        amount;
(B[m[32m+                                    *action_deltas
(B[m[32m+                                        .entry("credit_delta".to_string())
(B[m[32m+                                        .or_insert(0) -= amount;
(B[m                                 }
                                 Action::StreakIncrement(key) => {
[31m-                                    streak_changes
(B[m[31m-                                        .entry(key.clone())
(B[m[31m-                                        .or_default()
(B[m[31m-                                        .increments += 1;
(B[m[32m+                                    streak_changes.entry(key.clone()).or_default().increments += 1;
(B[m                                 }
                                 Action::StreakReset(key) => {
[31m-                                    streak_changes
(B[m[31m-                                        .entry(key.clone())
(B[m[31m-                                        .or_default()
(B[m[31m-                                        .resets += 1;
(B[m[32m+                                    streak_changes.entry(key.clone()).or_default().resets += 1;
(B[m                                 }
                                 _ => {}
                             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:338:
             for (idx, diff) in self.diffs.iter().enumerate() {
                 md.push_str(&format!("### Diff {}\n\n", idx + 1));
                 match diff {
[31m-                    ReplayDiff::FiredDecisionDelta { baseline, alternate } => {
(B[m[32m+                    ReplayDiff::FiredDecisionDelta {
(B[m[32m+                        baseline,
(B[m[32m+                        alternate,
(B[m[32m+                    } => {
(B[m                         md.push_str(&format!(
                             "**Rule Fire Count Changed:** {} → {}\n",
                             baseline, alternate
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:359:
                         baseline,
                         alternate,
                     } => {
[32m+                        md.push_str(&format!("**Streak '{}' Changed:**\n", key));
(B[m                         md.push_str(&format!(
[31m-                            "**Streak '{}' Changed:**\n",
(B[m[31m-                            key
(B[m[31m-                        ));
(B[m[31m-                        md.push_str(&format!(
(B[m                             "- Baseline: +{} increments, {} resets\n",
                             baseline.increments, baseline.resets
                         ));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:397:
         };
         let alternate = baseline.clone();
 
[31m-        let diffs = ReplayEngine::compute_diff(&baseline, &alternate)
(B[m[31m-            .expect("diff should succeed");
(B[m[32m+        let diffs = ReplayEngine::compute_diff(&baseline, &alternate).expect("diff should succeed");
(B[m         assert!(diffs.is_empty(), "identical rulesets should have no diffs");
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:415:
         let mut alternate = baseline.clone();
         alternate.fired_decisions = 5; // 3 more fires
 
[31m-        let diffs = ReplayEngine::compute_diff(&baseline, &alternate)
(B[m[31m-            .expect("diff should succeed");
(B[m[32m+        let diffs = ReplayEngine::compute_diff(&baseline, &alternate).expect("diff should succeed");
(B[m         assert!(!diffs.is_empty(), "added rule should produce diffs");
[31m-        assert!(diffs.iter().any(|d| matches!(
(B[m[31m-            d,
(B[m[31m-            ReplayDiff::FiredDecisionDelta { .. }
(B[m[31m-        )));
(B[m[32m+        assert!(diffs
(B[m[32m+            .iter()
(B[m[32m+            .any(|d| matches!(d, ReplayDiff::FiredDecisionDelta { .. })));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:430:
             events_seen: 10,
             decisions: 10,
             fired_decisions: 5,
[31m-            action_deltas: [("credit_delta".to_string(), 100)].iter().cloned().collect(),
(B[m[32m+            action_deltas: [("credit_delta".to_string(), 100)]
(B[m[32m+                .iter()
(B[m[32m+                .cloned()
(B[m[32m+                .collect(),
(B[m             streak_changes: HashMap::new(),
         };
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:438:
             events_seen: 10,
             decisions: 10,
             fired_decisions: 5,
[31m-            action_deltas: [("credit_delta".to_string(), 150)].iter().cloned().collect(),
(B[m[32m+            action_deltas: [("credit_delta".to_string(), 150)]
(B[m[32m+                .iter()
(B[m[32m+                .cloned()
(B[m[32m+                .collect(),
(B[m             streak_changes: HashMap::new(),
         };
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:445:
[31m-        let diffs = ReplayEngine::compute_diff(&baseline, &alternate)
(B[m[31m-            .expect("diff should succeed");
(B[m[32m+        let diffs = ReplayEngine::compute_diff(&baseline, &alternate).expect("diff should succeed");
(B[m         assert!(!diffs.is_empty(), "modified action should produce diffs");
         assert!(diffs.iter().any(|d| matches!(
             d,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-replay/src/lib.rs:460:
         let baseline = ReplayReport::default();
         let alternate = ReplayReport::default();
 
[31m-        let diffs = ReplayEngine::compute_diff(&baseline, &alternate)
(B[m[31m-            .expect("diff should succeed");
(B[m[32m+        let diffs = ReplayEngine::compute_diff(&baseline, &alternate).expect("diff should succeed");
(B[m         assert!(
             diffs.is_empty(),
             "zero-event window should have no diffs for identical rulesets"
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:112:
                     "granted_at": c.granted_at,
                 }),
             ),
[31m-            WalletMutation::SpendCredit { amount, purpose } => {
(B[m[31m-                ("wallet.spend_credit", json!({ "amount": amount, "purpose": purpose }))
(B[m[31m-            }
(B[m[32m+            WalletMutation::SpendCredit { amount, purpose } => (
(B[m[32m+                "wallet.spend_credit",
(B[m[32m+                json!({ "amount": amount, "purpose": purpose }),
(B[m[32m+            ),
(B[m             WalletMutation::StreakIncrement(name) => {
                 ("wallet.streak_increment", json!({ "name": name }))
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:227:
         )
         .unwrap();
         w.apply(
[31m-            WalletMutation::SpendCredit { amount: 40, purpose: "unlock".into() },
(B[m[32m+            WalletMutation::SpendCredit {
(B[m[32m+                amount: 40,
(B[m[32m+                purpose: "unlock".into(),
(B[m[32m+            },
(B[m             t(2026, 1, 1, 1),
             &NoopAuditSink,
         )
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:241:
         let mut w = RewardWallet::default();
         let err = w
             .apply(
[31m-                WalletMutation::SpendCredit { amount: 5, purpose: "x".into() },
(B[m[32m+                WalletMutation::SpendCredit {
(B[m[32m+                    amount: 5,
(B[m[32m+                    purpose: "x".into(),
(B[m[32m+                },
(B[m                 t(2026, 1, 1, 0),
                 &NoopAuditSink,
             )
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:253:
     #[test]
     fn streak_increments_only_once_per_utc_day() {
         let mut w = RewardWallet::default();
[31m-        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 1, 8), &NoopAuditSink)
(B[m[31m-            .unwrap();
(B[m[31m-        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 1, 23), &NoopAuditSink)
(B[m[31m-            .unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakIncrement("daily".into()),
(B[m[32m+            t(2026, 1, 1, 8),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakIncrement("daily".into()),
(B[m[32m+            t(2026, 1, 1, 23),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(w.streaks["daily"].count, 1);
[31m-        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 2, 0), &NoopAuditSink)
(B[m[31m-            .unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakIncrement("daily".into()),
(B[m[32m+            t(2026, 1, 2, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(w.streaks["daily"].count, 2);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:277:
         )
         .unwrap();
         assert_eq!(w.effective_multiplier(t(2026, 1, 1, 9)), 2.0);
[31m-        w.apply(WalletMutation::StreakReset("noop".into()), t(2026, 1, 1, 11), &NoopAuditSink)
(B[m[31m-            .unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakReset("noop".into()),
(B[m[32m+            t(2026, 1, 1, 11),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(w.effective_multiplier(t(2026, 1, 1, 11)), 1.0);
         assert!(w.multiplier_state.expires_at.is_none());
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:305:
     #[test]
     fn streak_reset_clears_count() {
         let mut w = RewardWallet::default();
[31m-        w.apply(WalletMutation::StreakIncrement("s".into()), t(2026, 1, 1, 0), &NoopAuditSink)
(B[m[31m-            .unwrap();
(B[m[31m-        w.apply(WalletMutation::StreakReset("s".into()), t(2026, 1, 1, 1), &NoopAuditSink).unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakIncrement("s".into()),
(B[m[32m+            t(2026, 1, 1, 0),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakReset("s".into()),
(B[m[32m+            t(2026, 1, 1, 1),
(B[m[32m+            &NoopAuditSink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         assert_eq!(w.streaks["s"].count, 0);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:338:
         let mut w = RewardWallet::default();
         let sink = CapturingAuditSink::new();
         let _ = w.apply(
[31m-            WalletMutation::SpendCredit { amount: 10, purpose: "x".into() },
(B[m[32m+            WalletMutation::SpendCredit {
(B[m[32m+                amount: 10,
(B[m[32m+                purpose: "x".into(),
(B[m[32m+            },
(B[m             t(2026, 1, 1, 0),
             &sink,
         );
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:350:
     fn idempotent_streak_does_not_audit_twice() {
         let mut w = RewardWallet::default();
         let sink = CapturingAuditSink::new();
[31m-        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 1, 8), &sink).unwrap();
(B[m[31m-        w.apply(WalletMutation::StreakIncrement("daily".into()), t(2026, 1, 1, 23), &sink).unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakIncrement("daily".into()),
(B[m[32m+            t(2026, 1, 1, 8),
(B[m[32m+            &sink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m[32m+        w.apply(
(B[m[32m+            WalletMutation::StreakIncrement("daily".into()),
(B[m[32m+            t(2026, 1, 1, 23),
(B[m[32m+            &sink,
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         // First increment audits, second same-day no-op does not.
         assert_eq!(sink.len(), 1);
         assert_eq!(sink.snapshot()[0].0, "wallet.streak_increment");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rewards/src/lib.rs:373:
         )
         .unwrap();
         w.apply(
[31m-            WalletMutation::SpendCredit { amount: 20, purpose: "unlock-games".into() },
(B[m[32m+            WalletMutation::SpendCredit {
(B[m[32m+                amount: 20,
(B[m[32m+                purpose: "unlock-games".into(),
(B[m[32m+            },
(B[m             t(2026, 1, 1, 1),
             &sink,
         )
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:145:
 
 impl TaskActual {
     pub fn skipped(task_id: Uuid) -> Self {
[31m-        Self { task_id, actual_minutes: 0, completed_at: None, cancelled: false }
(B[m[32m+        Self {
(B[m[32m+            task_id,
(B[m[32m+            actual_minutes: 0,
(B[m[32m+            completed_at: None,
(B[m[32m+            cancelled: false,
(B[m[32m+        }
(B[m     }
     pub fn completed(task_id: Uuid, actual_minutes: u32, at: DateTime<Utc>) -> Self {
[31m-        Self { task_id, actual_minutes, completed_at: Some(at), cancelled: false }
(B[m[32m+        Self {
(B[m[32m+            task_id,
(B[m[32m+            actual_minutes,
(B[m[32m+            completed_at: Some(at),
(B[m[32m+            cancelled: false,
(B[m[32m+        }
(B[m     }
     pub fn cancelled(task_id: Uuid) -> Self {
[31m-        Self { task_id, actual_minutes: 0, completed_at: None, cancelled: true }
(B[m[32m+        Self {
(B[m[32m+            task_id,
(B[m[32m+            actual_minutes: 0,
(B[m[32m+            completed_at: None,
(B[m[32m+            cancelled: true,
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:179:
         coaching: Arc<dyn CoachingProvider>,
         mascot: Arc<Mutex<MascotMachine>>,
     ) -> Self {
[31m-        Self { scheduler, calendar, coaching, mascot }
(B[m[32m+        Self {
(B[m[32m+            scheduler,
(B[m[32m+            calendar,
(B[m[32m+            coaching,
(B[m[32m+            mascot,
(B[m[32m+        }
(B[m     }
 
     /// FR-RITUAL-001 — build today's Morning Brief.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:197:
         let range = DateRange::new(now, now + horizon);
         let calendar_events = self.calendar.list_events(range).await.unwrap_or_default();
         // 2. Plan against those events so conflicts propagate.
[31m-        let schedule = self.scheduler.plan(tasks, &calendar_events, now, horizon).await?;
(B[m[32m+        let schedule = self
(B[m[32m+            .scheduler
(B[m[32m+            .plan(tasks, &calendar_events, now, horizon)
(B[m[32m+            .await?;
(B[m 
         // 3. Top-3 priorities by schedule order (scheduler sorts by start_at,
         //    but deterministic score was applied internally; take earliest
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:248:
             *planned_minutes_by_task.entry(b.task_id).or_insert(0) += mins;
             // Titles aren't in TimeBlock; use short id surrogate. Hosts who
             // want real titles can join before calling.
[31m-            title_by_task.entry(b.task_id).or_insert_with(|| short_title(&b.task_id));
(B[m[32m+            title_by_task
(B[m[32m+                .entry(b.task_id)
(B[m[32m+                .or_insert_with(|| short_title(&b.task_id));
(B[m         }
 
         for actual in actuals {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:255:
[31m-            let planned_minutes =
(B[m[31m-                planned_minutes_by_task.get(&actual.task_id).copied().unwrap_or(0);
(B[m[32m+            let planned_minutes = planned_minutes_by_task
(B[m[32m+                .get(&actual.task_id)
(B[m[32m+                .copied()
(B[m[32m+                .unwrap_or(0);
(B[m             let title = title_by_task
                 .get(&actual.task_id)
                 .cloned()
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:282:
         slipped.sort_by_key(|s| s.id);
 
         // Carryover: slipped but not cancelled.
[31m-        let carryover: Vec<Uuid> =
(B[m[31m-            slipped.iter().filter(|s| s.reason != SlipReason::Cancelled).map(|s| s.id).collect();
(B[m[32m+        let carryover: Vec<Uuid> = slipped
(B[m[32m+            .iter()
(B[m[32m+            .filter(|s| s.reason != SlipReason::Cancelled)
(B[m[32m+            .map(|s| s.id)
(B[m[32m+            .collect();
(B[m 
         // Streak deltas: +1 focus streak if ≥3h shipped total.
         let mut streak_deltas: HashMap<String, i32> = HashMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:338:
         now: DateTime<Utc>,
     ) -> anyhow::Result<Schedule> {
         let new_end = now + Duration::minutes(i64::from(overrun.actual_minutes));
[31m-        let changes = vec![ScheduleChange::BlockOverran { task_id: overrun.task_id, new_end }];
(B[m[32m+        let changes = vec![ScheduleChange::BlockOverran {
(B[m[32m+            task_id: overrun.task_id,
(B[m[32m+            new_end,
(B[m[32m+        }];
(B[m         // Replan from the live task pool so reflow has a real base schedule to
         // layer the overrun onto, instead of synthesizing an empty one.
         let horizon = Duration::hours(24);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:362:
             "Given these priorities today: {}. Write a ≤80-char morning greeting that names one of them specifically.",
             names.join(", ")
         );
[31m-        complete_guarded(self.coaching.as_ref(), &prompt, None, 80).await.ok().flatten()
(B[m[32m+        complete_guarded(self.coaching.as_ref(), &prompt, None, 80)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 
     async fn ask_closing(&self, shipped: u32, slipped: u32) -> Option<String> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:370:
             "Close out the day. {} tasks shipped, {} slipped. Tell the user what went well in ≤60 chars, and what to carry over in ≤60 chars.",
             shipped, slipped
         );
[31m-        complete_guarded(self.coaching.as_ref(), &prompt, None, 120).await.ok().flatten()
(B[m[32m+        complete_guarded(self.coaching.as_ref(), &prompt, None, 120)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:387:
     if actual.cancelled {
         return Classification::Slipped(SlipReason::Cancelled);
     }
[31m-    match (actual.completed_at.is_some(), actual.actual_minutes, planned_minutes) {
(B[m[32m+    match (
(B[m[32m+        actual.completed_at.is_some(),
(B[m[32m+        actual.actual_minutes,
(B[m[32m+        planned_minutes,
(B[m[32m+    ) {
(B[m         (false, 0, _) => Classification::Slipped(SlipReason::Skipped),
         (true, act, plan) if plan > 0 && act > plan => Classification::Slipped(SlipReason::Overran),
         (true, act, plan) if plan > 0 && act < plan => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:430:
         }
     }
     seen.iter()
[31m-        .filter_map(|id| tasks.iter().find(|t| t.id == *id).map(task_to_priority_line))
(B[m[32m+        .filter_map(|id| {
(B[m[32m+            tasks
(B[m[32m+                .iter()
(B[m[32m+                .find(|t| t.id == *id)
(B[m[32m+                .map(task_to_priority_line)
(B[m[32m+        })
(B[m         .collect()
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:482:
             .count() as u32;
     let soft_conflicts = schedule.rigidity_cost.soft_overrides;
 
[31m-    SchedulePreview { windows, soft_conflicts, hard_conflicts }
(B[m[32m+    SchedulePreview {
(B[m[32m+        windows,
(B[m[32m+        soft_conflicts,
(B[m[32m+        hard_conflicts,
(B[m[32m+    }
(B[m }
 
 fn time_block_to_window(b: &TimeBlock) -> ScheduleWindowLine {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:517:
 
 fn build_wins_summary(shipped: &[ShippedTask], slipped: &[SlippedTask]) -> String {
     let focus_minutes: u32 = shipped.iter().map(|s| s.actual_minutes).sum();
[31m-    format!("{} shipped ({} min focus), {} slipped.", shipped.len(), focus_minutes, slipped.len())
(B[m[32m+    format!(
(B[m[32m+        "{} shipped ({} min focus), {} slipped.",
(B[m[32m+        shipped.len(),
(B[m[32m+        focus_minutes,
(B[m[32m+        slipped.len()
(B[m[32m+    )
(B[m }
 
 fn truncate(s: &str, max: usize) -> String {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:582:
             let coaching: Arc<dyn CoachingProvider> =
                 Arc::new(StubCoachingProvider::single("Start with write the spec."));
             let (_mascot, engine) = mk_engine(coaching);
[31m-            let tasks = vec![mk_task("write the spec", 60, 0.9), mk_task("review PRs", 30, 0.5)];
(B[m[31m-            engine.generate_morning_brief(&tasks, Uuid::new_v4(), t0()).await.unwrap()
(B[m[32m+            let tasks = vec![
(B[m[32m+                mk_task("write the spec", 60, 0.9),
(B[m[32m+                mk_task("review PRs", 30, 0.5),
(B[m[32m+            ];
(B[m[32m+            engine
(B[m[32m+                .generate_morning_brief(&tasks, Uuid::new_v4(), t0())
(B[m[32m+                .await
(B[m[32m+                .unwrap()
(B[m         });
         assert!(!brief.top_priorities.is_empty());
         assert_eq!(brief.coachy_opening, "Start with write the spec.");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:597:
             let coaching: Arc<dyn CoachingProvider> = Arc::new(NoopCoachingProvider);
             let (_m, engine) = mk_engine(coaching);
             let tasks = vec![mk_task("ship it", 45, 0.7)];
[31m-            engine.generate_morning_brief(&tasks, Uuid::nil(), t0()).await.unwrap()
(B[m[32m+            engine
(B[m[32m+                .generate_morning_brief(&tasks, Uuid::nil(), t0())
(B[m[32m+                .await
(B[m[32m+                .unwrap()
(B[m         });
         assert!(brief.coachy_opening.contains("ship it"));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:610:
         let t_ship = mk_task("ship", 60, 0.9);
         let t_skip = mk_task("skip", 30, 0.5);
         let tasks = vec![t_ship.clone(), t_skip.clone()];
[31m-        let sched = engine.scheduler.plan(&tasks, &[], t0(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = engine
(B[m[32m+            .scheduler
(B[m[32m+            .plan(&tasks, &[], t0(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let actuals = vec![
             TaskActual::completed(t_ship.id, 60, t0() + Duration::hours(1)),
             TaskActual::skipped(t_skip.id),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:632:
         let a = mk_task("a", 30, 0.5);
         let b = mk_task("b", 30, 0.5);
         let tasks = vec![a.clone(), b.clone()];
[31m-        let sched = engine.scheduler.plan(&tasks, &[], t0(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = engine
(B[m[32m+            .scheduler
(B[m[32m+            .plan(&tasks, &[], t0(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let actuals = vec![TaskActual::skipped(a.id), TaskActual::cancelled(b.id)];
         let sd = engine
             .generate_evening_shutdown(&sched, &actuals, t0() + Duration::hours(8))
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:649:
         let coaching: Arc<dyn CoachingProvider> = Arc::new(NoopCoachingProvider);
         let (_m, engine) = mk_engine(coaching);
         let long = mk_task("long", 200, 0.9);
[31m-        let sched =
(B[m[31m-            engine.scheduler.plan(&[long.clone()], &[], t0(), Duration::hours(8)).await.unwrap();
(B[m[31m-        let actuals = vec![TaskActual::completed(long.id, 200, t0() + Duration::hours(4))];
(B[m[32m+        let sched = engine
(B[m[32m+            .scheduler
(B[m[32m+            .plan(&[long.clone()], &[], t0(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        let actuals = vec![TaskActual::completed(
(B[m[32m+            long.id,
(B[m[32m+            200,
(B[m[32m+            t0() + Duration::hours(4),
(B[m[32m+        )];
(B[m         let sd = engine
             .generate_evening_shutdown(&sched, &actuals, t0() + Duration::hours(9))
             .await
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:666:
             let coaching: Arc<dyn CoachingProvider> =
                 Arc::new(StubCoachingProvider::single("Pick one small thing."));
             let (_m, engine) = mk_engine(coaching);
[31m-            engine.generate_morning_brief(&[], Uuid::nil(), t0()).await.unwrap()
(B[m[32m+            engine
(B[m[32m+                .generate_morning_brief(&[], Uuid::nil(), t0())
(B[m[32m+                .await
(B[m[32m+                .unwrap()
(B[m         });
         assert!(brief.top_priorities.is_empty());
         assert_eq!(brief.coachy_opening, "Pick one small thing.");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:710:
             .unwrap();
         let preview = build_preview(
             &sched,
[31m-            &calendar.list_events(DateRange::new(t0(), t0() + Duration::hours(8))).await.unwrap(),
(B[m[32m+            &calendar
(B[m[32m+                .list_events(DateRange::new(t0(), t0() + Duration::hours(8)))
(B[m[32m+                .await
(B[m[32m+                .unwrap(),
(B[m         );
         assert!(preview.hard_conflicts >= 1);
         // Also verify morning_brief threads this through (the hard event is
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:728:
         let coaching: Arc<dyn CoachingProvider> = Arc::new(StubCoachingProvider::single("Hi."));
         let (_m, engine) = mk_engine(coaching);
         let tasks = vec![mk_task("x", 30, 0.5)];
[31m-        let mut brief = engine.generate_morning_brief(&tasks, Uuid::nil(), t0()).await.unwrap();
(B[m[32m+        let mut brief = engine
(B[m[32m+            .generate_morning_brief(&tasks, Uuid::nil(), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let before = brief.clone();
         let sink = focus_audit::CapturingAuditSink::new();
         engine.capture_intention(&mut brief, "finish the spec".into(), t0(), &sink);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:736:
         assert_eq!(records.len(), 1);
         assert_eq!(records[0].0, "ritual.intention.captured");
         assert_eq!(
[31m-            records[0].2.get("intention").and_then(serde_json::Value::as_str),
(B[m[32m+            records[0]
(B[m[32m+                .2
(B[m[32m+                .get("intention")
(B[m[32m+                .and_then(serde_json::Value::as_str),
(B[m             Some("finish the spec")
         );
         assert_eq!(brief.intention.as_deref(), Some("finish the spec"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:751:
         let coaching: Arc<dyn CoachingProvider> = Arc::new(StubCoachingProvider::single("x"));
         let (_m, engine) = mk_engine(coaching);
         let tasks = vec![mk_task("focus", 60, 0.9)];
[31m-        let _brief = engine.generate_morning_brief(&tasks, Uuid::nil(), t0()).await.unwrap();
(B[m[32m+        let _brief = engine
(B[m[32m+            .generate_morning_brief(&tasks, Uuid::nil(), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let overrun = TaskActual::completed(tasks[0].id, 120, t0() + Duration::hours(2));
         let sched = engine.suggest_reflow(&tasks, &overrun, t0()).await.unwrap();
         // Reflow now runs against a real replanned base — the assignment for
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:770:
                 Arc::new(StubCoachingProvider::single("Fixed opening."));
             let (_m, engine) = mk_engine(coaching);
             let id = Uuid::new_v4();
[31m-            let t = Task { id, ..mk_task("fixed", 60, 0.5) };
(B[m[32m+            let t = Task {
(B[m[32m+                id,
(B[m[32m+                ..mk_task("fixed", 60, 0.5)
(B[m[32m+            };
(B[m             let tasks = vec![t];
[31m-            let a = engine.generate_morning_brief(&tasks, Uuid::nil(), t0()).await.unwrap();
(B[m[32m+            let a = engine
(B[m[32m+                .generate_morning_brief(&tasks, Uuid::nil(), t0())
(B[m[32m+                .await
(B[m[32m+                .unwrap();
(B[m             let coaching2: Arc<dyn CoachingProvider> =
                 Arc::new(StubCoachingProvider::single("Fixed opening."));
             let (_m2, engine2) = mk_engine(coaching2);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:779:
[31m-            let b = engine2.generate_morning_brief(&tasks, Uuid::nil(), t0()).await.unwrap();
(B[m[32m+            let b = engine2
(B[m[32m+                .generate_morning_brief(&tasks, Uuid::nil(), t0())
(B[m[32m+                .await
(B[m[32m+                .unwrap();
(B[m             (a, b)
         });
         assert_eq!(a.top_priorities, b.top_priorities);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:791:
         let coaching: Arc<dyn CoachingProvider> = Arc::new(NoopCoachingProvider);
         let (_m, engine) = mk_engine(coaching);
         let tasks = vec![mk_task("x", 30, 0.5)];
[31m-        let brief = engine.generate_morning_brief(&tasks, Uuid::nil(), t0()).await.unwrap();
(B[m[32m+        let brief = engine
(B[m[32m+            .generate_morning_brief(&tasks, Uuid::nil(), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let json = serde_json::to_string(&brief).unwrap();
         let back: MorningBrief = serde_json::from_str(&json).unwrap();
         assert_eq!(brief, back);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:798:
 
[31m-        let sched = engine.scheduler.plan(&tasks, &[], t0(), Duration::hours(4)).await.unwrap();
(B[m[31m-        let actuals = vec![TaskActual::completed(tasks[0].id, 30, t0() + Duration::minutes(45))];
(B[m[32m+        let sched = engine
(B[m[32m+            .scheduler
(B[m[32m+            .plan(&tasks, &[], t0(), Duration::hours(4))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        let actuals = vec![TaskActual::completed(
(B[m[32m+            tasks[0].id,
(B[m[32m+            30,
(B[m[32m+            t0() + Duration::minutes(45),
(B[m[32m+        )];
(B[m         let sd = engine
             .generate_evening_shutdown(&sched, &actuals, t0() + Duration::hours(5))
             .await
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:828:
         // behavior in ask_opening path — verify static fallback via flag.
         let _lock = ENV_MUTEX.lock().expect("env lock");
         std::env::set_var(focus_coaching::KILL_SWITCH_ENV, "1");
[31m-        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
(B[m[32m+        let rt = tokio::runtime::Builder::new_current_thread()
(B[m[32m+            .enable_all()
(B[m[32m+            .build()
(B[m[32m+            .unwrap();
(B[m         let coaching: Arc<dyn CoachingProvider> =
             Arc::new(StubCoachingProvider::single("should-be-ignored"));
         let scheduler = Arc::new(Scheduler::new(WorkingHoursSpec::default()));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:837:
         let engine = RitualsEngine::new(scheduler, calendar, coaching, mascot);
         let brief = rt
             .block_on(async {
[31m-                engine.generate_morning_brief(&[mk_task("thing", 30, 0.5)], Uuid::nil(), t0()).await
(B[m[32m+                engine
(B[m[32m+                    .generate_morning_brief(&[mk_task("thing", 30, 0.5)], Uuid::nil(), t0())
(B[m[32m+                    .await
(B[m             })
             .unwrap();
         std::env::remove_var(focus_coaching::KILL_SWITCH_ENV);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:856:
     {
         let _g = ENV_MUTEX.lock().expect("env lock");
         std::env::remove_var(focus_coaching::KILL_SWITCH_ENV);
[31m-        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("rt");
(B[m[32m+        let rt = tokio::runtime::Builder::new_current_thread()
(B[m[32m+            .enable_all()
(B[m[32m+            .build()
(B[m[32m+            .expect("rt");
(B[m         rt.block_on(fut)
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/lib.rs:865:
     fn slip_reason_classification_matrix() {
         // cancelled wins regardless
         let cancelled = TaskActual::cancelled(Uuid::nil());
[31m-        assert!(matches!(classify(&cancelled, 30), Classification::Slipped(SlipReason::Cancelled)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            classify(&cancelled, 30),
(B[m[32m+            Classification::Slipped(SlipReason::Cancelled)
(B[m[32m+        ));
(B[m 
         let skipped = TaskActual::skipped(Uuid::nil());
[31m-        assert!(matches!(classify(&skipped, 30), Classification::Slipped(SlipReason::Skipped)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            classify(&skipped, 30),
(B[m[32m+            Classification::Slipped(SlipReason::Skipped)
(B[m[32m+        ));
(B[m 
         let over = TaskActual::completed(Uuid::nil(), 90, Utc::now());
[31m-        assert!(matches!(classify(&over, 60), Classification::Slipped(SlipReason::Overran)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            classify(&over, 60),
(B[m[32m+            Classification::Slipped(SlipReason::Overran)
(B[m[32m+        ));
(B[m 
         let deferred = TaskActual::completed(Uuid::nil(), 10, Utc::now());
[31m-        assert!(matches!(classify(&deferred, 60), Classification::Slipped(SlipReason::Deferred)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            classify(&deferred, 60),
(B[m[32m+            Classification::Slipped(SlipReason::Deferred)
(B[m[32m+        ));
(B[m 
         let shipped = TaskActual::completed(Uuid::nil(), 60, Utc::now());
         assert!(matches!(classify(&shipped, 60), Classification::Shipped));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/monthly.rs:63:
         // Aggregate data from event store, wallet, audit — stub for now.
         // Real impl: query event store for month's focus sessions by week,
         // fetch prior month's totals, compute deltas, derive theme.
[31m-        let (total_focus_hours, weekly_breakdown) =
(B[m[31m-            (52.0, vec![11.5, 12.0, 13.5, 15.0]); // 4 weeks of activity
(B[m[32m+        let (total_focus_hours, weekly_breakdown) = (52.0, vec![11.5, 12.0, 13.5, 15.0]); // 4 weeks of activity
(B[m         let tasks_completed = 68;
 
         // Compare to prior month (stub).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/monthly.rs:122:
              Suggest a one-word or two-word theme (e.g., 'momentum', 'breakthrough', 'consistency').",
             total_focus_hours, tasks_completed
         );
[31m-        complete_guarded(self.coaching.as_ref(), &prompt, None, 40).await.ok().flatten()
(B[m[32m+        complete_guarded(self.coaching.as_ref(), &prompt, None, 40)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 
     async fn ask_monthly_reflection(&self, theme: &str) -> Option<String> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/monthly.rs:130:
             "The month's theme was '{}'. Write a ≤100-char reflection on growth and next month's focus.",
             theme
         );
[31m-        complete_guarded(self.coaching.as_ref(), &prompt, None, 100).await.ok().flatten()
(B[m[32m+        complete_guarded(self.coaching.as_ref(), &prompt, None, 100)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/monthly.rs:196:
     async fn monthly_retro_delta_populated() {
         let engine = mk_monthly_engine();
         let retro = engine.generate_monthly_retro(t0()).await.unwrap();
[31m-        assert!(retro.compared_to_prior_month.trend_direction == "up"
(B[m[31m-            || retro.compared_to_prior_month.trend_direction == "down"
(B[m[31m-            || retro.compared_to_prior_month.trend_direction == "stable");
(B[m[32m+        assert!(
(B[m[32m+            retro.compared_to_prior_month.trend_direction == "up"
(B[m[32m+                || retro.compared_to_prior_month.trend_direction == "down"
(B[m[32m+                || retro.compared_to_prior_month.trend_direction == "stable"
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-RITUAL-004
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/weekly.rs:147:
         let prompt = format!(
             "Summarize the week: {:.1}h focus, {} tasks completed, rules firing: {}. \
              Write a ≤120-char narrative of wins.",
[31m-            focus_hours, tasks_completed,
(B[m[32m+            focus_hours,
(B[m[32m+            tasks_completed,
(B[m             rule_names.join(", ")
         );
[31m-        complete_guarded(self.coaching.as_ref(), &prompt, None, 120).await.ok().flatten()
(B[m[32m+        complete_guarded(self.coaching.as_ref(), &prompt, None, 120)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 
     async fn ask_growth_area(&self, tasks_slipped: u32) -> Option<String> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/weekly.rs:162:
                 tasks_slipped
             )
         };
[31m-        complete_guarded(self.coaching.as_ref(), &prompt, None, 50).await.ok().flatten()
(B[m[32m+        complete_guarded(self.coaching.as_ref(), &prompt, None, 50)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 
     async fn ask_weekly_closing(&self) -> Option<String> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/weekly.rs:169:
[31m-        let prompt = "Write a closing line for the weekly review, encouraging next week's work (≤80 chars).";
(B[m[31m-        complete_guarded(self.coaching.as_ref(), prompt, None, 80).await.ok().flatten()
(B[m[32m+        let prompt =
(B[m[32m+            "Write a closing line for the weekly review, encouraging next week's work (≤80 chars).";
(B[m[32m+        complete_guarded(self.coaching.as_ref(), prompt, None, 80)
(B[m[32m+            .await
(B[m[32m+            .ok()
(B[m[32m+            .flatten()
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/weekly.rs:178:
 const STATIC_WEEKLY_CLOSING_FALLBACK: &str = "Strong week ahead. Keep the streak alive.";
 
 fn static_wins_summary(focus_hours: f32, tasks_completed: u32) -> String {
[31m-    format!("{:.1}h focused, {} tasks shipped. Solid week.", focus_hours, tasks_completed)
(B[m[32m+    format!(
(B[m[32m+        "{:.1}h focused, {} tasks shipped. Solid week.",
(B[m[32m+        focus_hours, tasks_completed
(B[m[32m+    )
(B[m }
 
 fn static_growth_area(tasks_slipped: u32) -> String {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rituals/src/weekly.rs:185:
     if tasks_slipped == 0 {
         "Increase task complexity or duration — you're ready for bigger challenges.".to_string()
     } else {
[31m-        format!("Reduce slip rate — {} tasks slipped. Focus on estimation or scope.", tasks_slipped)
(B[m[32m+        format!(
(B[m[32m+            "Reduce slip rate — {} tasks slipped. Focus on estimation or scope.",
(B[m[32m+            tasks_slipped
(B[m[32m+        )
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:64:
 
 impl RuleSuggester {
     pub fn new() -> Self {
[31m-        Self { dismissed: std::collections::HashSet::new() }
(B[m[32m+        Self {
(B[m[32m+            dismissed: std::collections::HashSet::new(),
(B[m[32m+        }
(B[m     }
 
     pub fn with_dismissed(mut self, ids: Vec<Uuid>) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:97:
         let audit_records = self.load_recent_audits(audit_store, cutoff)?;
 
         // Filter events to window
[31m-        let recent_events: Vec<_> =
(B[m[31m-            events.iter().filter(|e| e.occurred_at >= cutoff && e.occurred_at <= now).collect();
(B[m[32m+        let recent_events: Vec<_> = events
(B[m[32m+            .iter()
(B[m[32m+            .filter(|e| e.occurred_at >= cutoff && e.occurred_at <= now)
(B[m[32m+            .collect();
(B[m 
         let mut suggestions = Vec::new();
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:117:
         }
 
         // H3: Missed Check-ins (combining audit records + any audit-like events)
[31m-        let combined_records = [audit_records.clone(), self.extract_audit_like_events(&recent_events)].concat();
(B[m[32m+        let combined_records = [
(B[m[32m+            audit_records.clone(),
(B[m[32m+            self.extract_audit_like_events(&recent_events),
(B[m[32m+        ]
(B[m[32m+        .concat();
(B[m         if let Some(h3) = self.heuristic_missed_checkins(&combined_records, window_days) {
             if !self.dismissed.contains(&h3.id) {
                 suggestions.push(h3);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:125:
         }
 
         // H4: Unlinked Actions (GitHub PRs → wallet grants)
[31m-        let combined_records_h4 = [audit_records.clone(), self.extract_audit_like_events(&recent_events)].concat();
(B[m[32m+        let combined_records_h4 = [
(B[m[32m+            audit_records.clone(),
(B[m[32m+            self.extract_audit_like_events(&recent_events),
(B[m[32m+        ]
(B[m[32m+        .concat();
(B[m         if let Some(h4) = self.heuristic_unlinked_actions(&combined_records_h4, &recent_events) {
             if !self.dismissed.contains(&h4.id) {
                 suggestions.push(h4);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:152:
                 matches!(
                     e.event_type,
                     focus_events::EventType::WellKnown(WellKnownEventType::EventStarted)
[31m-                )
(B[m[31m-                    && e.payload.get("source").and_then(|v| v.as_str()) == Some("focus_session")
(B[m[32m+                ) && e.payload.get("source").and_then(|v| v.as_str()) == Some("focus_session")
(B[m             })
             .collect();
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:167:
         let mut sample_times = Vec::new();
 
         for event in &focus_starts {
[31m-            let hour = event.occurred_at.format("%H").to_string().parse::<u32>().unwrap_or(0);
(B[m[32m+            let hour = event
(B[m[32m+                .occurred_at
(B[m[32m+                .format("%H")
(B[m[32m+                .to_string()
(B[m[32m+                .parse::<u32>()
(B[m[32m+                .unwrap_or(0);
(B[m             let weekday = event.occurred_at.weekday().number_from_monday();
             let bucket = (weekday, hour);
             *time_buckets.entry(bucket).or_insert(0) += 1;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:177:
         }
 
         // Find if any time slot repeats >= 3 times (indicating a pattern)
[31m-        let (best_bucket, count) =
(B[m[31m-            time_buckets.iter().max_by_key(|(_, c)| *c).unwrap_or((&(0, 0), &0));
(B[m[32m+        let (best_bucket, count) = time_buckets
(B[m[32m+            .iter()
(B[m[32m+            .max_by_key(|(_, c)| *c)
(B[m[32m+            .unwrap_or((&(0, 0), &0));
(B[m 
         if *count < 3 {
             return None;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:306:
     ) -> Option<RuleSuggestion> {
         let checkin_records: Vec<_> = audit_records
             .iter()
[31m-            .filter(|r| r.record_type.contains("daily_checkin") || r.record_type.contains("checkin"))
(B[m[32m+            .filter(|r| {
(B[m[32m+                r.record_type.contains("daily_checkin") || r.record_type.contains("checkin")
(B[m[32m+            })
(B[m             .collect();
 
         if checkin_records.len() < 2 {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:341:
             ),
             proposed_rule: ProposedRule {
                 name: "Earlier morning check-in".to_string(),
[31m-                description: "Remind you to check in earlier in the day."
(B[m[31m-                    .to_string(),
(B[m[32m+                description: "Remind you to check in earlier in the day.".to_string(),
(B[m                 trigger: "schedule:0 8 ? ? *".to_string(),
                 conditions: vec![],
[31m-                actions: vec!["Notify { message: 'Time for your daily check-in!'.to_string() }"
(B[m[31m-                    .to_string()],
(B[m[32m+                actions: vec![
(B[m[32m+                    "Notify { message: 'Time for your daily check-in!'.to_string() }".to_string(),
(B[m[32m+                ],
(B[m                 priority: 30,
                 cooldown_seconds: None,
             },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:382:
         }
 
         // Check if wallet grants follow PRs (within 1 hour)
[31m-        let grants: Vec<_> =
(B[m[31m-            audit_records.iter().filter(|r| r.record_type.contains("wallet.grant")).collect();
(B[m[32m+        let grants: Vec<_> = audit_records
(B[m[32m+            .iter()
(B[m[32m+            .filter(|r| r.record_type.contains("wallet.grant"))
(B[m[32m+            .collect();
(B[m 
         if grants.is_empty() {
             return None;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:418:
             ),
             proposed_rule: ProposedRule {
                 name: "Credit PR merges".to_string(),
[31m-                description: "Grant credits when you merge a GitHub PR."
(B[m[31m-                    .to_string(),
(B[m[32m+                description: "Grant credits when you merge a GitHub PR.".to_string(),
(B[m                 trigger: "event:github_pr_merged".to_string(),
                 conditions: vec!["is_your_pr".to_string()],
                 actions: vec!["GrantCredit { amount: 10 }".to_string()],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:502:
         let suggester = RuleSuggester::new();
 
         // Create mock events for focus sessions
[31m-        let events = vec![mock_focus_event(ts(1), 9),
(B[m[32m+        let events = vec![
(B[m[32m+            mock_focus_event(ts(1), 9),
(B[m             mock_focus_event(ts(2), 9),
             mock_focus_event(ts(8), 9),
[31m-            mock_focus_event(ts(9), 9)];
(B[m[32m+            mock_focus_event(ts(9), 9),
(B[m[32m+        ];
(B[m 
[31m-        let suggestions = suggester.suggest_rules(&NoopAuditStore, &events, 30).unwrap();
(B[m[32m+        let suggestions = suggester
(B[m[32m+            .suggest_rules(&NoopAuditStore, &events, 30)
(B[m[32m+            .unwrap();
(B[m         if let Some(first) = suggestions.first() {
             let mut suggester_with_dismiss = RuleSuggester::new();
             suggester_with_dismiss.dismiss(first.id);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:514:
[31m-            let filtered =
(B[m[31m-                suggester_with_dismiss.suggest_rules(&NoopAuditStore, &events, 30).unwrap();
(B[m[32m+            let filtered = suggester_with_dismiss
(B[m[32m+                .suggest_rules(&NoopAuditStore, &events, 30)
(B[m[32m+                .unwrap();
(B[m             assert!(!filtered.iter().any(|s| s.id == first.id));
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:526:
             events.push(mock_task_completed_event(ts(i as i64)));
         }
 
[31m-        let suggestions = suggester.suggest_rules(&NoopAuditStore, &events, 30).unwrap();
(B[m[32m+        let suggestions = suggester
(B[m[32m+            .suggest_rules(&NoopAuditStore, &events, 30)
(B[m[32m+            .unwrap();
(B[m         assert!(suggestions
             .iter()
             .any(|s| s.heuristic_name == "MissingCelebrations"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rule-suggester/src/lib.rs:598:
 
     impl MockAuditStore {
         fn new() -> Self {
[31m-            Self { records: Vec::new() }
(B[m[32m+            Self {
(B[m[32m+                records: Vec::new(),
(B[m[32m+            }
(B[m         }
 
         fn add_checkin(&mut self, dt: DateTime<Utc>) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/benches/rule_evaluation.rs:1:
[32m+use chrono::Utc;
(B[m use criterion::{black_box, criterion_group, criterion_main, Criterion};
[31m-use focus_events::{NormalizedEvent, WellKnownEventType, DedupeKey, EventType};
(B[m[32m+use focus_events::{DedupeKey, EventType, NormalizedEvent, WellKnownEventType};
(B[m use focus_rules::{Action, Condition, Rule, Trigger};
[31m-use chrono::Utc;
(B[m use std::collections::HashMap;
 use uuid::Uuid;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/benches/rule_evaluation.rs:106:
                 .collect::<Vec<_>>(),
         );
 
[31m-        let events = black_box(
(B[m[31m-            (0..1000)
(B[m[31m-                .map(|_| make_event())
(B[m[31m-                .collect::<Vec<_>>(),
(B[m[31m-        );
(B[m[32m+        let events = black_box((0..1000).map(|_| make_event()).collect::<Vec<_>>());
(B[m 
         b.iter(|| {
             let mut decisions = 0;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:35:
             name: name.into(),
             kind: kind.into(),
             required,
[31m-            description: if description.is_empty() { None } else { Some(description.into()) },
(B[m[32m+            description: if description.is_empty() {
(B[m[32m+                None
(B[m[32m+            } else {
(B[m[32m+                Some(description.into())
(B[m[32m+            },
(B[m         }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:111:
     let conditions = vec![
         DslConditionSpec {
             kind: "confidence_gte".into(),
[31m-            params: vec![DslParam::new("min", "number", true, "Minimum event.confidence (0..1).")],
(B[m[32m+            params: vec![DslParam::new(
(B[m[32m+                "min",
(B[m[32m+                "number",
(B[m[32m+                true,
(B[m[32m+                "Minimum event.confidence (0..1).",
(B[m[32m+            )],
(B[m             description: "Only fire when the source event's confidence meets the threshold.".into(),
         },
         DslConditionSpec {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:126:
             kind: "payload_in".into(),
             params: vec![
                 DslParam::new("path", "string", true, "Dotted payload path."),
[31m-                DslParam::new("values", "array<any>", true, "Set of permitted JSON values."),
(B[m[32m+                DslParam::new(
(B[m[32m+                    "values",
(B[m[32m+                    "array<any>",
(B[m[32m+                    true,
(B[m[32m+                    "Set of permitted JSON values.",
(B[m[32m+                ),
(B[m             ],
             description: "Payload value at `path` is one of `values`.".into(),
         },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:148:
         },
         DslConditionSpec {
             kind: "payload_exists".into(),
[31m-            params: vec![DslParam::new("path", "string", true, "Dotted payload path.")],
(B[m[32m+            params: vec![DslParam::new(
(B[m[32m+                "path",
(B[m[32m+                "string",
(B[m[32m+                true,
(B[m[32m+                "Dotted payload path.",
(B[m[32m+            )],
(B[m             description: "Payload key at `path` is present (null counts).".into(),
         },
         DslConditionSpec {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:214:
     let actions = vec![
         DslActionSpec {
             kind: "GrantCredit".into(),
[31m-            params: vec![DslParam::new("amount", "integer", true, "Credits to grant.")],
(B[m[32m+            params: vec![DslParam::new(
(B[m[32m+                "amount",
(B[m[32m+                "integer",
(B[m[32m+                true,
(B[m[32m+                "Credits to grant.",
(B[m[32m+            )],
(B[m             description: "Add credit to the wallet.".into(),
         },
         DslActionSpec {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:221:
             kind: "DeductCredit".into(),
[31m-            params: vec![DslParam::new("amount", "integer", true, "Credits to deduct.")],
(B[m[32m+            params: vec![DslParam::new(
(B[m[32m+                "amount",
(B[m[32m+                "integer",
(B[m[32m+                true,
(B[m[32m+                "Credits to deduct.",
(B[m[32m+            )],
(B[m             description: "Remove credit from the wallet.".into(),
         },
         DslActionSpec {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:226:
             kind: "Block".into(),
             params: vec![
                 DslParam::new("profile", "string", true, "Enforcement profile id."),
[31m-                DslParam::new("duration_seconds", "integer", true, "Block duration in seconds."),
(B[m                 DslParam::new(
[32m+                    "duration_seconds",
(B[m[32m+                    "integer",
(B[m[32m+                    true,
(B[m[32m+                    "Block duration in seconds.",
(B[m[32m+                ),
(B[m[32m+                DslParam::new(
(B[m                     "rigidity",
                     "enum<Hard|Soft>",
                     false,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:238:
         },
         DslActionSpec {
             kind: "Unblock".into(),
[31m-            params: vec![DslParam::new("profile", "string", true, "Enforcement profile id.")],
(B[m[32m+            params: vec![DslParam::new(
(B[m[32m+                "profile",
(B[m[32m+                "string",
(B[m[32m+                true,
(B[m[32m+                "Enforcement profile id.",
(B[m[32m+            )],
(B[m             description: "Deactivate the named enforcement profile.".into(),
         },
         DslActionSpec {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:253:
         },
         DslActionSpec {
             kind: "Notify".into(),
[31m-            params: vec![DslParam::new("message", "string", true, "Notification body.")],
(B[m[32m+            params: vec![DslParam::new(
(B[m[32m+                "message",
(B[m[32m+                "string",
(B[m[32m+                true,
(B[m[32m+                "Notification body.",
(B[m[32m+            )],
(B[m             description: "Send a local notification to the user.".into(),
         },
         DslActionSpec {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:260:
             kind: "EmergencyExit".into(),
             params: vec![
[31m-                DslParam::new("profiles", "array<string>", true, "Profiles to short-circuit."),
(B[m[31m-                DslParam::new("duration_seconds", "integer", true, "Exit window in seconds."),
(B[m[32m+                DslParam::new(
(B[m[32m+                    "profiles",
(B[m[32m+                    "array<string>",
(B[m[32m+                    true,
(B[m[32m+                    "Profiles to short-circuit.",
(B[m[32m+                ),
(B[m[32m+                DslParam::new(
(B[m[32m+                    "duration_seconds",
(B[m[32m+                    "integer",
(B[m[32m+                    true,
(B[m[32m+                    "Exit window in seconds.",
(B[m[32m+                ),
(B[m                 DslParam::new("bypass_cost", "integer", true, "Bypass budget to consume."),
                 DslParam::new("reason", "string", true, "User-visible rationale."),
             ],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:291:
         },
     ];
 
[31m-    DslCatalog { triggers, conditions, actions }
(B[m[32m+    DslCatalog {
(B[m[32m+        triggers,
(B[m[32m+        conditions,
(B[m[32m+        actions,
(B[m[32m+    }
(B[m }
 
 /// Fluent builder for [`Rule`]s. Primarily used by tests and future
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:326:
     }
 
     pub fn condition(mut self, kind: impl Into<String>, params: serde_json::Value) -> Self {
[31m-        self.rule.conditions.push(Condition { kind: kind.into(), params });
(B[m[32m+        self.rule.conditions.push(Condition {
(B[m[32m+            kind: kind.into(),
(B[m[32m+            params,
(B[m[32m+        });
(B[m         self
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:397:
             "any_of",
             "not",
         ];
[31m-        assert_eq!(names.len(), expected.len(), "condition count drift: {names:?}");
(B[m[32m+        assert_eq!(
(B[m[32m+            names.len(),
(B[m[32m+            expected.len(),
(B[m[32m+            "condition count drift: {names:?}"
(B[m[32m+        );
(B[m         for e in expected {
             assert!(names.contains(&e), "missing condition {e}");
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/builder.rs:445:
     fn every_action_has_required_params_where_expected() {
         let cat = describe_dsl();
         // EmergencyExit/Intervention/ScheduledUnlockWindow all require >1 param.
[31m-        for kind in ["EmergencyExit", "Intervention", "ScheduledUnlockWindow", "Block"] {
(B[m[31m-            let spec =
(B[m[31m-                cat.actions.iter().find(|a| a.kind == kind).unwrap_or_else(|| panic!("{kind}"));
(B[m[31m-            assert!(spec.params.iter().any(|p| p.required), "{kind} should have required params");
(B[m[32m+        for kind in [
(B[m[32m+            "EmergencyExit",
(B[m[32m+            "Intervention",
(B[m[32m+            "ScheduledUnlockWindow",
(B[m[32m+            "Block",
(B[m[32m+        ] {
(B[m[32m+            let spec = cat
(B[m[32m+                .actions
(B[m[32m+                .iter()
(B[m[32m+                .find(|a| a.kind == kind)
(B[m[32m+                .unwrap_or_else(|| panic!("{kind}"));
(B[m[32m+            assert!(
(B[m[32m+                spec.params.iter().any(|p| p.required),
(B[m[32m+                "{kind} should have required params"
(B[m[32m+            );
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:3:
 //! Traces to FR-RULE-001..005.
 
 pub mod builder;
[31m-pub use builder::{describe_dsl, DslActionSpec, DslCatalog, DslConditionSpec, DslParam, DslTriggerSpec, RuleBuilder};
(B[m[32m+pub use builder::{
(B[m[32m+    describe_dsl, DslActionSpec, DslCatalog, DslConditionSpec, DslParam, DslTriggerSpec,
(B[m[32m+    RuleBuilder,
(B[m[32m+};
(B[m 
 use chrono::{DateTime, Duration, Utc};
 use focus_coaching::{complete_guarded, prompts, CoachingProvider};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:126:
             (GrantCredit { amount: a }, GrantCredit { amount: b }) => a == b,
             (DeductCredit { amount: a }, DeductCredit { amount: b }) => a == b,
             (
[31m-                Block { profile: p1, duration: d1, rigidity: r1 },
(B[m[31m-                Block { profile: p2, duration: d2, rigidity: r2 },
(B[m[32m+                Block {
(B[m[32m+                    profile: p1,
(B[m[32m+                    duration: d1,
(B[m[32m+                    rigidity: r1,
(B[m[32m+                },
(B[m[32m+                Block {
(B[m[32m+                    profile: p2,
(B[m[32m+                    duration: d2,
(B[m[32m+                    rigidity: r2,
(B[m[32m+                },
(B[m             ) => p1 == p2 && d1 == d2 && r1 == r2,
             (Unblock { profile: p1 }, Unblock { profile: p2 }) => p1 == p2,
             (StreakIncrement(a), StreakIncrement(b)) => a == b,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:148:
                 },
             ) => p1 == p2 && d1 == d2 && c1 == c2 && r1 == r2,
             (
[31m-                Intervention { message: m1, severity: s1 },
(B[m[31m-                Intervention { message: m2, severity: s2 },
(B[m[32m+                Intervention {
(B[m[32m+                    message: m1,
(B[m[32m+                    severity: s1,
(B[m[32m+                },
(B[m[32m+                Intervention {
(B[m[32m+                    message: m2,
(B[m[32m+                    severity: s2,
(B[m[32m+                },
(B[m             ) => m1 == m2 && s1 == s2,
             (
                 ScheduledUnlockWindow {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:185:
 
 impl RuleEngine {
     pub fn new() -> Self {
[31m-        Self { cooldowns: HashMap::new() }
(B[m[32m+        Self {
(B[m[32m+            cooldowns: HashMap::new(),
(B[m[32m+        }
(B[m     }
 
     /// Seed cooldowns (e.g. from persisted state).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:210:
     ) -> RuleDecision {
         // FR-RULE-001: disabled rules skip.
         if !rule.enabled {
[31m-            return RuleDecision::Skipped { reason: "disabled".into() };
(B[m[32m+            return RuleDecision::Skipped {
(B[m[32m+                reason: "disabled".into(),
(B[m[32m+            };
(B[m         }
 
         // FR-RULE-001: trigger must match event.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:217:
         match &rule.trigger {
             Trigger::Event(expected) => {
                 if !event_type_matches(&event.event_type, expected) {
[31m-                    return RuleDecision::Skipped { reason: "trigger_mismatch".into() };
(B[m[32m+                    return RuleDecision::Skipped {
(B[m[32m+                        reason: "trigger_mismatch".into(),
(B[m[32m+                    };
(B[m                 }
             }
             Trigger::Schedule(_) | Trigger::StateChange(_) => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:224:
[31m-                return RuleDecision::Skipped { reason: "non_event_trigger".into() };
(B[m[32m+                return RuleDecision::Skipped {
(B[m[32m+                    reason: "non_event_trigger".into(),
(B[m[32m+                };
(B[m             }
         }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:228:
         // FR-RULE-003: evaluate conditions (best-effort built-ins).
         for cond in &rule.conditions {
             if !condition_matches(cond, event) {
[31m-                return RuleDecision::Skipped { reason: format!("condition_failed:{}", cond.kind) };
(B[m[32m+                return RuleDecision::Skipped {
(B[m[32m+                    reason: format!("condition_failed:{}", cond.kind),
(B[m[32m+                };
(B[m             }
         }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:236:
         if let Some(cooldown) = rule.cooldown {
             if let Some(last) = self.cooldowns.get(&rule.id) {
                 if now.signed_duration_since(*last) < cooldown {
[31m-                    return RuleDecision::Suppressed { reason: "cooldown".into() };
(B[m[32m+                    return RuleDecision::Suppressed {
(B[m[32m+                        reason: "cooldown".into(),
(B[m[32m+                    };
(B[m                 }
             }
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:310:
         now: DateTime<Utc>,
     ) -> RuleDecision {
         if !rule.enabled {
[31m-            return RuleDecision::Skipped { reason: "disabled".into() };
(B[m[32m+            return RuleDecision::Skipped {
(B[m[32m+                reason: "disabled".into(),
(B[m[32m+            };
(B[m         }
         let key = match &rule.trigger {
             Trigger::StateChange(k) => k,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:317:
[31m-            _ => return RuleDecision::Skipped { reason: "non_state_change_trigger".into() },
(B[m[32m+            _ => {
(B[m[32m+                return RuleDecision::Skipped {
(B[m[32m+                    reason: "non_state_change_trigger".into(),
(B[m[32m+                }
(B[m[32m+            }
(B[m         };
         let b = resolve_path(before, key);
         let a = resolve_path(after, key);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:321:
         if b == a {
[31m-            return RuleDecision::Skipped { reason: "no_change".into() };
(B[m[32m+            return RuleDecision::Skipped {
(B[m[32m+                reason: "no_change".into(),
(B[m[32m+            };
(B[m         }
         // Cooldown check (identical to event eval).
         if let Some(cooldown) = rule.cooldown {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:326:
             if let Some(last) = self.cooldowns.get(&rule.id) {
                 if now.signed_duration_since(*last) < cooldown {
[31m-                    return RuleDecision::Suppressed { reason: "cooldown".into() };
(B[m[32m+                    return RuleDecision::Suppressed {
(B[m[32m+                        reason: "cooldown".into(),
(B[m[32m+                    };
(B[m                 }
             }
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:368:
     /// used by the `cron` crate. Examples:
     /// - `"0 0 9 * * *"` — every day at 09:00:00.
     /// - `"0 */15 * * * *"` — every 15 minutes on the minute.
[31m-    pub fn evaluate_schedule_tick(
(B[m[31m-        &mut self,
(B[m[31m-        rule: &Rule,
(B[m[31m-        now: DateTime<Utc>,
(B[m[31m-    ) -> RuleDecision {
(B[m[32m+    pub fn evaluate_schedule_tick(&mut self, rule: &Rule, now: DateTime<Utc>) -> RuleDecision {
(B[m         if !rule.enabled {
[31m-            return RuleDecision::Skipped { reason: "disabled".into() };
(B[m[32m+            return RuleDecision::Skipped {
(B[m[32m+                reason: "disabled".into(),
(B[m[32m+            };
(B[m         }
         let cron_spec = match &rule.trigger {
             Trigger::Schedule(s) => s,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:381:
[31m-            _ => return RuleDecision::Skipped { reason: "non_schedule_trigger".into() },
(B[m[32m+            _ => {
(B[m[32m+                return RuleDecision::Skipped {
(B[m[32m+                    reason: "non_schedule_trigger".into(),
(B[m[32m+                }
(B[m[32m+            }
(B[m         };
         let schedule = match cron_spec.parse::<cron::Schedule>() {
             Ok(s) => s,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:389:
             }
         };
         // Most recent scheduled slot at or before `now`.
[31m-        let Some(most_recent) = schedule.after(&(now - chrono::Duration::days(365))).take_while(|t| *t <= now).last() else {
(B[m[31m-            return RuleDecision::Skipped { reason: "no_slot_in_window".into() };
(B[m[32m+        let Some(most_recent) = schedule
(B[m[32m+            .after(&(now - chrono::Duration::days(365)))
(B[m[32m+            .take_while(|t| *t <= now)
(B[m[32m+            .last()
(B[m[32m+        else {
(B[m[32m+            return RuleDecision::Skipped {
(B[m[32m+                reason: "no_slot_in_window".into(),
(B[m[32m+            };
(B[m         };
         // Dedupe against cooldown map, treating the slot as the firing key.
         if let Some(last) = self.cooldowns.get(&rule.id) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:397:
             if *last >= most_recent {
[31m-                return RuleDecision::Suppressed { reason: "already_fired_for_slot".into() };
(B[m[32m+                return RuleDecision::Suppressed {
(B[m[32m+                    reason: "already_fired_for_slot".into(),
(B[m[32m+                };
(B[m             }
         }
         self.cooldowns.insert(rule.id, most_recent);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:419:
         indexed.sort_by(|a, b| b.1.priority.cmp(&a.1.priority).then(a.0.cmp(&b.0)));
         for (_, rule) in indexed {
             let decision = self.evaluate(rule, event, now);
[31m-            out.push(PrioritizedDecision { rule_id: rule.id, priority: rule.priority, decision });
(B[m[32m+            out.push(PrioritizedDecision {
(B[m[32m+                rule_id: rule.id,
(B[m[32m+                priority: rule.priority,
(B[m[32m+                decision,
(B[m[32m+            });
(B[m         }
         out
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:477:
         event_type_name(&event.event_type),
         payload,
     );
[31m-    match complete_guarded(coaching, &user, Some(prompts::RULE_EXPLANATION_SYSTEM_PROMPT), 220)
(B[m[31m-        .await
(B[m[32m+    match complete_guarded(
(B[m[32m+        coaching,
(B[m[32m+        &user,
(B[m[32m+        Some(prompts::RULE_EXPLANATION_SYSTEM_PROMPT),
(B[m[32m+        220,
(B[m[32m+    )
(B[m[32m+    .await
(B[m     {
         Ok(Some(text)) => Ok(text),
         Ok(None) => Ok(fallback),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:547:
             let Some(expected) = cond.params.get("value") else {
                 return false;
             };
[31m-            resolve_path(&event.payload, path).map(|v| v == expected).unwrap_or(false)
(B[m[32m+            resolve_path(&event.payload, path)
(B[m[32m+                .map(|v| v == expected)
(B[m[32m+                .unwrap_or(false)
(B[m         }
         "payload_in" => {
             let Some(path) = cond.params.get("path").and_then(|v| v.as_str()) else {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:564:
             let Some(path) = cond.params.get("path").and_then(|v| v.as_str()) else {
                 return false;
             };
[31m-            let key = if cond.kind == "payload_gte" { "min" } else { "max" };
(B[m[32m+            let key = if cond.kind == "payload_gte" {
(B[m[32m+                "min"
(B[m[32m+            } else {
(B[m[32m+                "max"
(B[m[32m+            };
(B[m             let Some(threshold) = cond.params.get(key).and_then(|v| v.as_f64()) else {
                 return false;
             };
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:621:
             .params
             .get("conditions")
             .and_then(|v| v.as_array())
[31m-            .map(|arr| arr.iter().all(|c| parse_sub(c).map(|s| condition_matches(&s, event)).unwrap_or(false)))
(B[m[32m+            .map(|arr| {
(B[m[32m+                arr.iter().all(|c| {
(B[m[32m+                    parse_sub(c)
(B[m[32m+                        .map(|s| condition_matches(&s, event))
(B[m[32m+                        .unwrap_or(false)
(B[m[32m+                })
(B[m[32m+            })
(B[m             .unwrap_or(false),
         "any_of" => cond
             .params
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:628:
             .get("conditions")
             .and_then(|v| v.as_array())
[31m-            .map(|arr| arr.iter().any(|c| parse_sub(c).map(|s| condition_matches(&s, event)).unwrap_or(false)))
(B[m[32m+            .map(|arr| {
(B[m[32m+                arr.iter().any(|c| {
(B[m[32m+                    parse_sub(c)
(B[m[32m+                        .map(|s| condition_matches(&s, event))
(B[m[32m+                        .unwrap_or(false)
(B[m[32m+                })
(B[m[32m+            })
(B[m             .unwrap_or(false),
         "not" => cond
             .params
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:723:
         let mut eng = RuleEngine::new();
         let mut rule = mk_rule("r", "TaskCompleted", vec![], 0);
         rule.enabled = false;
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Skipped { .. }));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, now),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-RULE-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:733:
     fn trigger_mismatch_is_skipped() {
         let mut eng = RuleEngine::new();
         let rule = mk_rule("r", "TaskCompleted", vec![], 0);
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::SleepRecorded), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::SleepRecorded),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
         match eng.evaluate(&rule, &ev, now) {
             RuleDecision::Skipped { reason } => assert_eq!(reason, "trigger_mismatch"),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:745:
     #[test]
     fn matching_event_fires_rule() {
         let mut eng = RuleEngine::new();
[31m-        let rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 5 }], 0);
(B[m[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let rule = mk_rule(
(B[m[32m+            "r",
(B[m[32m+            "TaskCompleted",
(B[m[32m+            vec![Action::GrantCredit { amount: 5 }],
(B[m[32m+            0,
(B[m[32m+        );
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
         match eng.evaluate(&rule, &ev, now) {
             RuleDecision::Fired(actions) => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:760:
     #[test]
     fn cooldown_suppresses_repeat_within_window() {
         let mut eng = RuleEngine::new();
[31m-        let mut rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 1 }], 0);
(B[m[32m+        let mut rule = mk_rule(
(B[m[32m+            "r",
(B[m[32m+            "TaskCompleted",
(B[m[32m+            vec![Action::GrantCredit { amount: 1 }],
(B[m[32m+            0,
(B[m[32m+        );
(B[m         rule.cooldown = Some(Duration::minutes(10));
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let t0 = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, t0), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, t0),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m         let t1 = t0 + Duration::minutes(5);
         match eng.evaluate(&rule, &ev, t1) {
             RuleDecision::Suppressed { reason } => assert_eq!(reason, "cooldown"),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:776:
     #[test]
     fn cooldown_expires_allows_refire() {
         let mut eng = RuleEngine::new();
[31m-        let mut rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 1 }], 0);
(B[m[32m+        let mut rule = mk_rule(
(B[m[32m+            "r",
(B[m[32m+            "TaskCompleted",
(B[m[32m+            vec![Action::GrantCredit { amount: 1 }],
(B[m[32m+            0,
(B[m[32m+        );
(B[m         rule.cooldown = Some(Duration::minutes(10));
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let t0 = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
         let _ = eng.evaluate(&rule, &ev, t0);
         let t2 = t0 + Duration::minutes(11);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:785:
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, t2), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, t2),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-RULE-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:790:
     fn condition_confidence_gate_filters() {
         let mut eng = RuleEngine::new();
         let mut rule = mk_rule("r", "TaskCompleted", vec![], 0);
[31m-        rule.conditions
(B[m[31m-            .push(Condition { kind: "confidence_gte".into(), params: json!({"min": 0.9}) });
(B[m[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 0.5, json!({}));
(B[m[32m+        rule.conditions.push(Condition {
(B[m[32m+            kind: "confidence_gte".into(),
(B[m[32m+            params: json!({"min": 0.9}),
(B[m[32m+        });
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            0.5,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
         match eng.evaluate(&rule, &ev, now) {
             RuleDecision::Skipped { reason } => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:806:
     #[test]
     fn explanation_template_substitutes_placeholders() {
         let rule = mk_rule("MyRule", "TaskCompleted", vec![], 0);
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let rendered = RuleEngine::render_explanation(&rule, &ev);
         assert!(rendered.contains("MyRule"));
         assert!(rendered.contains("TaskCompleted"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:816:
     #[test]
     fn evaluate_all_orders_by_priority_desc() {
         let mut eng = RuleEngine::new();
[31m-        let low =
(B[m[31m-            mk_rule("low", "TaskCompleted", vec![Action::Unblock { profile: "games".into() }], 1);
(B[m[32m+        let low = mk_rule(
(B[m[32m+            "low",
(B[m[32m+            "TaskCompleted",
(B[m[32m+            vec![Action::Unblock {
(B[m[32m+                profile: "games".into(),
(B[m[32m+            }],
(B[m[32m+            1,
(B[m[32m+        );
(B[m         let high = mk_rule(
             "high",
             "TaskCompleted",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:828:
             }],
             100,
         );
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
         let decisions = eng.evaluate_all(&[low, high], &ev, now);
         assert_eq!(decisions.len(), 2);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:840:
     #[test]
     fn trigger_exact_match_against_custom_event_type() {
         let mut eng = RuleEngine::new();
[31m-        let rule = mk_rule("r", "canvas:quiz_posted", vec![Action::GrantCredit { amount: 1 }], 0);
(B[m[31m-        let ev = mk_event(EventType::Custom("canvas:quiz_posted".into()), 1.0, json!({}));
(B[m[32m+        let rule = mk_rule(
(B[m[32m+            "r",
(B[m[32m+            "canvas:quiz_posted",
(B[m[32m+            vec![Action::GrantCredit { amount: 1 }],
(B[m[32m+            0,
(B[m[32m+        );
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::Custom("canvas:quiz_posted".into()),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, now),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-EVT-VOCAB-001, FR-RULE-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:851:
     fn trigger_prefix_glob_matches_custom_namespace() {
         let mut eng = RuleEngine::new();
         let rule = mk_rule("r", "canvas:*", vec![Action::GrantCredit { amount: 1 }], 0);
[31m-        let ev = mk_event(EventType::Custom("canvas:quiz_posted".into()), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::Custom("canvas:quiz_posted".into()),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, now), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, now),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-EVT-VOCAB-001, FR-RULE-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:874:
     fn evaluate_is_deterministic() {
         let mut eng_a = RuleEngine::new();
         let mut eng_b = RuleEngine::new();
[31m-        let rule = mk_rule("r", "TaskCompleted", vec![Action::GrantCredit { amount: 7 }], 0);
(B[m[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let rule = mk_rule(
(B[m[32m+            "r",
(B[m[32m+            "TaskCompleted",
(B[m[32m+            vec![Action::GrantCredit { amount: 7 }],
(B[m[32m+            0,
(B[m[32m+        );
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let now = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
         let a = eng_a.evaluate(&rule, &ev, now);
         let b = eng_b.evaluate(&rule, &ev, now);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:926:
     #[tokio::test]
     async fn propose_rule_errors_on_garbage() {
         let provider = StubCoachingProvider::single("not even close to json");
[31m-        let err = propose_rule_from_nl("whatever", &provider).await.unwrap_err();
(B[m[32m+        let err = propose_rule_from_nl("whatever", &provider)
(B[m[32m+            .await
(B[m[32m+            .unwrap_err();
(B[m         assert!(err.to_string().contains("invalid Rule JSON"));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:948:
             1.0,
             json!({"title": "Essay"}),
         );
[31m-        let out = render_llm_explanation(&rule, &ev, &provider).await.expect("explain");
(B[m[32m+        let out = render_llm_explanation(&rule, &ev, &provider)
(B[m[32m+            .await
(B[m[32m+            .expect("explain");
(B[m         assert!(out.contains("+5 credits"));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:955:
     #[tokio::test]
     async fn render_llm_explanation_falls_back_when_noop() {
         let rule = mk_rule("Reward", "TaskCompleted", vec![], 0);
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let provider = NoopCoachingProvider;
[31m-        let out = render_llm_explanation(&rule, &ev, &provider).await.expect("explain");
(B[m[32m+        let out = render_llm_explanation(&rule, &ev, &provider)
(B[m[32m+            .await
(B[m[32m+            .expect("explain");
(B[m         assert!(out.contains("Reward"));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:983:
         // 09:00 UTC every day
         let rule = mk_schedule_rule("0 0 9 * * *");
         let now = Utc.with_ymd_and_hms(2026, 4, 23, 9, 30, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate_schedule_tick(&rule, now), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate_schedule_tick(&rule, now),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:991:
         let mut eng = RuleEngine::default();
         let rule = mk_schedule_rule("0 0 9 * * *");
         let now = Utc.with_ymd_and_hms(2026, 4, 23, 9, 30, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate_schedule_tick(&rule, now), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate_schedule_tick(&rule, now),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m         let second = eng.evaluate_schedule_tick(&rule, now + Duration::minutes(5));
         assert!(matches!(second, RuleDecision::Suppressed { .. }));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1002:
         let rule = mk_schedule_rule("0 0 9 * * *");
         let d1 = Utc.with_ymd_and_hms(2026, 4, 23, 9, 30, 0).unwrap();
         let d2 = Utc.with_ymd_and_hms(2026, 4, 24, 9, 30, 0).unwrap();
[31m-        assert!(matches!(eng.evaluate_schedule_tick(&rule, d1), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate_schedule_tick(&rule, d2), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate_schedule_tick(&rule, d1),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate_schedule_tick(&rule, d2),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1030:
 
     // Traces to: FR-RULE-003 (condition DSL)
     fn cond(kind: &str, params: serde_json::Value) -> Condition {
[31m-        Condition { kind: kind.into(), params }
(B[m[32m+        Condition {
(B[m[32m+            kind: kind.into(),
(B[m[32m+            params,
(B[m[32m+        }
(B[m     }
 
     // Traces to: FR-RULE-006 (evaluation audit trail)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1039:
         let mut eng = RuleEngine::default();
         let mut rule = mk_rule("Reward", "TaskCompleted", vec![], 0);
         rule.explanation_template = "Good job on {{event.type}}".into();
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let (decision, eval) = eng.evaluate_with_trace(&rule, &ev, Utc::now());
         assert!(matches!(decision, RuleDecision::Fired(_)));
         assert_eq!(eval.rule_id, rule.id);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1051:
     fn evaluate_with_trace_skipped_has_reason_explanation() {
         let mut eng = RuleEngine::default();
         let rule = mk_rule("X", "TaskCompleted", vec![], 0);
[31m-        let ev = mk_event(EventType::WellKnown(WellKnownEventType::AssignmentDue), 1.0, json!({}));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::AssignmentDue),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         let (decision, eval) = eng.evaluate_with_trace(&rule, &ev, Utc::now());
         assert!(matches!(decision, RuleDecision::Skipped { .. }));
         assert!(eval.explanation.contains("skipped"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1073:
         let mut eng = RuleEngine::default();
         let rule = {
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("payload_eq", json!({"path":"assignment.late","value":true})));
(B[m[32m+            r.conditions.push(cond(
(B[m[32m+                "payload_eq",
(B[m[32m+                json!({"path":"assignment.late","value":true}),
(B[m[32m+            ));
(B[m             r
         };
[31m-        let ev_late = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"assignment":{"late":true}}));
(B[m[31m-        let ev_not = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"assignment":{"late":false}}));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_late, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_not, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        let ev_late = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"assignment":{"late":true}}),
(B[m[32m+        );
(B[m[32m+        let ev_not = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"assignment":{"late":false}}),
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_late, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_not, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1087:
         let mut eng = RuleEngine::default();
         let rule = {
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("payload_in", json!({"path":"status","values":["done","graded"]})));
(B[m[32m+            r.conditions.push(cond(
(B[m[32m+                "payload_in",
(B[m[32m+                json!({"path":"status","values":["done","graded"]}),
(B[m[32m+            ));
(B[m             r
         };
[31m-        let ev_done = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"status":"done"}));
(B[m[31m-        let ev_other = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"status":"draft"}));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_done, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_other, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        let ev_done = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"status":"done"}),
(B[m[32m+        );
(B[m[32m+        let ev_other = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"status":"draft"}),
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_done, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_other, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1101:
         let mut eng = RuleEngine::default();
         let gte = {
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("payload_gte", json!({"path":"points","min":80.0})));
(B[m[32m+            r.conditions
(B[m[32m+                .push(cond("payload_gte", json!({"path":"points","min":80.0})));
(B[m             r
         };
         let lte = {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1108:
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("payload_lte", json!({"path":"points","max":50.0})));
(B[m[32m+            r.conditions
(B[m[32m+                .push(cond("payload_lte", json!({"path":"points","max":50.0})));
(B[m             r
         };
[31m-        let ev90 = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"points":90.0}));
(B[m[31m-        let ev40 = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"points":40.0}));
(B[m[31m-        assert!(matches!(eng.evaluate(&gte, &ev90, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&gte, &ev40, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[31m-        assert!(matches!(eng.evaluate(&lte, &ev40, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&lte, &ev90, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        let ev90 = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"points":90.0}),
(B[m[32m+        );
(B[m[32m+        let ev40 = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"points":40.0}),
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&gte, &ev90, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&gte, &ev40, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&lte, &ev40, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&lte, &ev90, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1122:
         let mut eng = RuleEngine::default();
         let rule = {
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("payload_matches", json!({"path":"url","pattern":r"^https://.*\.edu/"})));
(B[m[32m+            r.conditions.push(cond(
(B[m[32m+                "payload_matches",
(B[m[32m+                json!({"path":"url","pattern":r"^https://.*\.edu/"}),
(B[m[32m+            ));
(B[m             r
         };
[31m-        let ev_ok = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"url":"https://mit.edu/assign/1"}));
(B[m[31m-        let ev_no = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"url":"http://example.com/"}));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_ok, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_no, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        let ev_ok = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"url":"https://mit.edu/assign/1"}),
(B[m[32m+        );
(B[m[32m+        let ev_no = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"url":"http://example.com/"}),
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_ok, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_no, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1148:
             ));
             r
         };
[31m-        let yes = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"kind":"a","blocked":false}));
(B[m[31m-        let no_blocked = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"kind":"a","blocked":true}));
(B[m[31m-        let no_kind = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"kind":"c","blocked":false}));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &yes, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &no_blocked, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &no_kind, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        let yes = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"kind":"a","blocked":false}),
(B[m[32m+        );
(B[m[32m+        let no_blocked = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"kind":"a","blocked":true}),
(B[m[32m+        );
(B[m[32m+        let no_kind = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"kind":"c","blocked":false}),
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &yes, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &no_blocked, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &no_kind, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1161:
         let mut eng = RuleEngine::default();
         let rule = {
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("payload_exists", json!({"path":"maybe"})));
(B[m[32m+            r.conditions
(B[m[32m+                .push(cond("payload_exists", json!({"path":"maybe"})));
(B[m             r
         };
[31m-        let ev_null = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({"maybe":null}));
(B[m[31m-        let ev_missing = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_null, Utc::now()), RuleDecision::Fired(_)));
(B[m[31m-        assert!(matches!(eng.evaluate(&rule, &ev_missing, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        let ev_null = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({"maybe":null}),
(B[m[32m+        );
(B[m[32m+        let ev_missing = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_null, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev_missing, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1175:
         let mut eng = RuleEngine::default();
         let rule = {
             let mut r = mk_rule("x", "TaskCompleted", vec![], 0);
[31m-            r.conditions.push(cond("source_eq", json!({"source":"canvas"})));
(B[m[32m+            r.conditions
(B[m[32m+                .push(cond("source_eq", json!({"source":"canvas"})));
(B[m             r
         };
[31m-        let mut ev = mk_event(EventType::WellKnown(WellKnownEventType::TaskCompleted), 1.0, json!({}));
(B[m[32m+        let mut ev = mk_event(
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+            1.0,
(B[m[32m+            json!({}),
(B[m[32m+        );
(B[m         ev.connector_id = "canvas".into();
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, Utc::now()), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, Utc::now()),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m         ev.connector_id = "gcal".into();
[31m-        assert!(matches!(eng.evaluate(&rule, &ev, Utc::now()), RuleDecision::Skipped { .. }));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate(&rule, &ev, Utc::now()),
(B[m[32m+            RuleDecision::Skipped { .. }
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-RULE-008 (expanded Action catalog)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1206:
             InterventionSeverity::Firm,
             InterventionSeverity::Urgent,
         ] {
[31m-            let a = Action::Intervention { message: "take a walk".into(), severity: sev };
(B[m[32m+            let a = Action::Intervention {
(B[m[32m+                message: "take a walk".into(),
(B[m[32m+                severity: sev,
(B[m[32m+            };
(B[m             let s = serde_json::to_string(&a).unwrap();
             let back: Action = serde_json::from_str(&s).unwrap();
             assert_eq!(a, back);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-rules/src/lib.rs:1284:
         let a = json!({"penalty":{"tier":"Clear"}});
         let b = json!({"penalty":{"tier":"Warn"}});
         let c = json!({"penalty":{"tier":"Strict"}});
[31m-        assert!(matches!(eng.evaluate_state_change(&rule, &a, &b, t0), RuleDecision::Fired(_)));
(B[m[32m+        assert!(matches!(
(B[m[32m+            eng.evaluate_state_change(&rule, &a, &b, t0),
(B[m[32m+            RuleDecision::Fired(_)
(B[m[32m+        ));
(B[m         // 5 min later, another transition → within cooldown.
         let d = eng.evaluate_state_change(&rule, &b, &c, t0 + Duration::minutes(5));
         assert!(matches!(d, RuleDecision::Suppressed { .. }));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/benches/packing.rs:53:
                 starts_at: now() + start_offset,
                 ends_at: now() + start_offset + Duration::minutes(45),
                 source: "calendar".into(),
[31m-                rigidity: if i % 3 == 0 { Rigidity::Hard } else { Rigidity::Soft },
(B[m[32m+                rigidity: if i % 3 == 0 {
(B[m[32m+                    Rigidity::Hard
(B[m[32m+                } else {
(B[m[32m+                    Rigidity::Soft
(B[m[32m+                },
(B[m             }
         })
         .collect()
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/benches/packing.rs:72:
                 let scheduler = Scheduler::new(WorkingHoursSpec::default());
                 let tasks = black_box(create_benchmark_tasks(50));
                 let events = black_box(create_benchmark_calendar_events(10));
[31m-                let _sched =
(B[m[31m-                    scheduler.plan(&tasks, &events, now(), Duration::hours(24)).await.unwrap();
(B[m[32m+                let _sched = scheduler
(B[m[32m+                    .plan(&tasks, &events, now(), Duration::hours(24))
(B[m[32m+                    .await
(B[m[32m+                    .unwrap();
(B[m             })
         });
     });
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/benches/packing.rs:85:
                 let scheduler = Scheduler::new(WorkingHoursSpec::default());
                 let tasks = black_box(create_benchmark_tasks(100));
                 let events = black_box(create_benchmark_calendar_events(20));
[31m-                let _sched =
(B[m[31m-                    scheduler.plan(&tasks, &events, now(), Duration::hours(24)).await.unwrap();
(B[m[32m+                let _sched = scheduler
(B[m[32m+                    .plan(&tasks, &events, now(), Duration::hours(24))
(B[m[32m+                    .await
(B[m[32m+                    .unwrap();
(B[m             })
         });
     });
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/benches/packing.rs:98:
                 let scheduler = Scheduler::new(WorkingHoursSpec::default());
                 let tasks = black_box(create_benchmark_tasks(200));
                 let events = black_box(create_benchmark_calendar_events(40));
[31m-                let _sched =
(B[m[31m-                    scheduler.plan(&tasks, &events, now(), Duration::hours(24)).await.unwrap();
(B[m[32m+                let _sched = scheduler
(B[m[32m+                    .plan(&tasks, &events, now(), Duration::hours(24))
(B[m[32m+                    .await
(B[m[32m+                    .unwrap();
(B[m             })
         });
     });
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:54:
                     *self.semi_cost_spent.entry("tier_bump".into()).or_insert(0) += 1;
                 }
                 RigidityCost::StreakRisk => {
[31m-                    *self.semi_cost_spent.entry("streak_risk".into()).or_insert(0) += 1;
(B[m[32m+                    *self
(B[m[32m+                        .semi_cost_spent
(B[m[32m+                        .entry("streak_risk".into())
(B[m[32m+                        .or_insert(0) += 1;
(B[m                 }
                 RigidityCost::FrictionDelay(d) => {
[31m-                    *self.semi_cost_spent.entry("friction_delay_sec".into()).or_insert(0) +=
(B[m[31m-                        d.as_secs() as i64;
(B[m[32m+                    *self
(B[m[32m+                        .semi_cost_spent
(B[m[32m+                        .entry("friction_delay_sec".into())
(B[m[32m+                        .or_insert(0) += d.as_secs() as i64;
(B[m                 }
                 RigidityCost::AccountabilityPing => {
[31m-                    *self.semi_cost_spent.entry("accountability_ping".into()).or_insert(0) += 1;
(B[m[32m+                    *self
(B[m[32m+                        .semi_cost_spent
(B[m[32m+                        .entry("accountability_ping".into())
(B[m[32m+                        .or_insert(0) += 1;
(B[m                 }
             },
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:83:
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub enum ScheduleChange {
     /// A previously-running block ran long; its new end is `new_end`.
[31m-    BlockOverran { task_id: Uuid, new_end: DateTime<Utc> },
(B[m[32m+    BlockOverran {
(B[m[32m+        task_id: Uuid,
(B[m[32m+        new_end: DateTime<Utc>,
(B[m[32m+    },
(B[m     /// A task was cancelled; its placements are freed.
     TaskCancelled(Uuid),
     /// A new calendar event landed on the timeline.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:108:
         Self {
             start: NaiveTime::from_hms_opt(9, 0, 0).expect("invariant: 09:00 is valid"),
             end: NaiveTime::from_hms_opt(17, 0, 0).expect("invariant: 17:00 is valid"),
[31m-            days: vec![Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu, Weekday::Fri],
(B[m[32m+            days: vec![
(B[m[32m+                Weekday::Mon,
(B[m[32m+                Weekday::Tue,
(B[m[32m+                Weekday::Wed,
(B[m[32m+                Weekday::Thu,
(B[m[32m+                Weekday::Fri,
(B[m[32m+            ],
(B[m         }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:124:
 
 impl Scheduler {
     pub fn new(working_hours_default: WorkingHoursSpec) -> Self {
[31m-        Self { default_working_hours: working_hours_default }
(B[m[32m+        Self {
(B[m[32m+            default_working_hours: working_hours_default,
(B[m[32m+        }
(B[m     }
 
     pub async fn plan(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:138:
 
         // 1. Sort tasks by priority-weight * deadline-urgency multiplier, desc.
         //    Tiebreak: earlier created_at wins (deterministic).
[31m-        let mut indexed: Vec<(usize, f64)> =
(B[m[31m-            tasks.iter().enumerate().map(|(i, t)| (i, score(t, now, end_horizon))).collect();
(B[m[32m+        let mut indexed: Vec<(usize, f64)> = tasks
(B[m[32m+            .iter()
(B[m[32m+            .enumerate()
(B[m[32m+            .map(|(i, t)| (i, score(t, now, end_horizon)))
(B[m[32m+            .collect();
(B[m         indexed.sort_by(|a, b| {
             b.1.partial_cmp(&a.1)
                 .unwrap_or(std::cmp::Ordering::Equal)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:156:
             let needed = task.duration.planning_duration();
 
             if needed <= Duration::zero() {
[31m-                unplaced
(B[m[31m-                    .push((task.id, UnplacedReason::ConstraintViolation("zero duration".into())));
(B[m[32m+                unplaced.push((
(B[m[32m+                    task.id,
(B[m[32m+                    UnplacedReason::ConstraintViolation("zero duration".into()),
(B[m[32m+                ));
(B[m                 continue;
             }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:189:
             let chunk_target = if task.duration.is_fixed() || !task.chunking.allow_split {
                 needed
             } else {
[31m-                task.chunking.ideal_chunk.min(needed).max(task.chunking.min_chunk)
(B[m[32m+                task.chunking
(B[m[32m+                    .ideal_chunk
(B[m[32m+                    .min(needed)
(B[m[32m+                    .max(task.chunking.min_chunk)
(B[m             };
 
             let mut remaining = needed;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:253:
         assignments.sort_by_key(|b| (b.starts_at, b.task_id));
         unplaced.sort_by_key(|(id, _)| *id);
 
[31m-        Ok(Schedule { assignments, unplaced, rigidity_cost, generated_at: now })
(B[m[32m+        Ok(Schedule {
(B[m[32m+            assignments,
(B[m[32m+            unplaced,
(B[m[32m+            rigidity_cost,
(B[m[32m+            generated_at: now,
(B[m[32m+        })
(B[m     }
 
     /// Reflow: take existing schedule, apply changes, recompute minimally.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:291:
                 // Past; pin it unless overrun extends it (still keep — history).
                 return true;
             }
[31m-            !new_events.iter().any(|e| !e.rigidity.is_soft() && b.overlaps(e.starts_at, e.ends_at))
(B[m[32m+            !new_events
(B[m[32m+                .iter()
(B[m[32m+                .any(|e| !e.rigidity.is_soft() && b.overlaps(e.starts_at, e.ends_at))
(B[m         });
 
         // Apply overrun: push the affected task's earliest future block.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:347:
 
         let _ = disturbed; // carried forward as an audit hint; not surfaced yet.
 
[31m-        Ok(Schedule { assignments, unplaced, rigidity_cost: rc, generated_at: now })
(B[m[32m+        Ok(Schedule {
(B[m[32m+            assignments,
(B[m[32m+            unplaced,
(B[m[32m+            rigidity_cost: rc,
(B[m[32m+            generated_at: now,
(B[m[32m+        })
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:378:
 
 fn extract_working_hours(task: &Task) -> Option<WorkingHoursSpec> {
     task.constraints.iter().find_map(|c| match c {
[31m-        Constraint::WorkingHours { start, end, days } => {
(B[m[31m-            Some(WorkingHoursSpec { start: *start, end: *end, days: days.clone() })
(B[m[31m-        }
(B[m[32m+        Constraint::WorkingHours { start, end, days } => Some(WorkingHoursSpec {
(B[m[32m+            start: *start,
(B[m[32m+            end: *end,
(B[m[32m+            days: days.clone(),
(B[m[32m+        }),
(B[m         _ => None,
     })
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:387:
 
 enum SlotResult {
[31m-    Found { start: DateTime<Utc>, end: DateTime<Utc> },
(B[m[32m+    Found {
(B[m[32m+        start: DateTime<Utc>,
(B[m[32m+        end: DateTime<Utc>,
(B[m[32m+    },
(B[m     HardBlocked,
     Exhausted,
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:440:
         let prop_end = cursor + want;
 
         // Check placed + task-local blocks for conflict.
[31m-        if placed.iter().chain(task_blocks.iter()).any(|b| b.overlaps(cursor, prop_end)) {
(B[m[32m+        if placed
(B[m[32m+            .iter()
(B[m[32m+            .chain(task_blocks.iter())
(B[m[32m+            .any(|b| b.overlaps(cursor, prop_end))
(B[m[32m+        {
(B[m             // Jump to the end of the offending block.
             let next = placed
                 .iter()
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:463:
                 }
                 Rigidity::Semi(_) => {
                     rc.charge(&ev.rigidity);
[31m-                    return SlotResult::Found { start: cursor, end: prop_end };
(B[m[32m+                    return SlotResult::Found {
(B[m[32m+                        start: cursor,
(B[m[32m+                        end: prop_end,
(B[m[32m+                    };
(B[m                 }
                 Rigidity::Soft => {
                     rc.charge(&ev.rigidity);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:470:
[31m-                    return SlotResult::Found { start: cursor, end: prop_end };
(B[m[32m+                    return SlotResult::Found {
(B[m[32m+                        start: cursor,
(B[m[32m+                        end: prop_end,
(B[m[32m+                    };
(B[m                 }
             }
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:474:
 
[31m-        return SlotResult::Found { start: cursor, end: prop_end };
(B[m[32m+        return SlotResult::Found {
(B[m[32m+            start: cursor,
(B[m[32m+            end: prop_end,
(B[m[32m+        };
(B[m     }
 
     // Distinguish HardBlocked vs Exhausted: if ANY hard event entirely covers
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:479:
     // [cursor, latest] we treat as hard-blocked.
[31m-    if cal.iter().any(|e| e.rigidity.is_hard() && e.starts_at <= cursor && e.ends_at >= latest) {
(B[m[32m+    if cal
(B[m[32m+        .iter()
(B[m[32m+        .any(|e| e.rigidity.is_hard() && e.starts_at <= cursor && e.ends_at >= latest)
(B[m[32m+    {
(B[m         SlotResult::HardBlocked
     } else {
         SlotResult::Exhausted
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:542:
         Task {
             priority: Priority::new(prio),
             chunking: ChunkingPolicy::atomic(),
[31m-            ..Task::new(title, DurationSpec::fixed(Duration::minutes(minutes)), now())
(B[m[32m+            ..Task::new(
(B[m[32m+                title,
(B[m[32m+                DurationSpec::fixed(Duration::minutes(minutes)),
(B[m[32m+                now(),
(B[m[32m+            )
(B[m         }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:566:
     async fn single_task_fits_in_empty_window() {
         let s = scheduler();
         let task = mk_task("write", 60, 0.5);
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
         assert_eq!(sched.assignments[0].task_id, task.id);
         assert!(sched.unplaced.is_empty());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:578:
         let s = scheduler();
         let low = mk_task("low", 60, 0.2);
         let high = mk_task("high", 60, 0.9);
[31m-        let sched =
(B[m[31m-            s.plan(&[low.clone(), high.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[low.clone(), high.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 2);
         // Assignments sorted by start time; high should start earliest.
         assert_eq!(sched.assignments[0].task_id, high.id);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:592:
         let task = mk_task("collide", 120, 0.9);
         // Hard event covers the entire 9–5 window.
         let ev = cal_event("court", 0, 8 * 60, Rigidity::Hard);
[31m-        let sched = s.plan(&[task.clone()], &[ev], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[ev], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         // Either unplaced (HardConflict) or placed after the hard block ends —
         // since hard event covers all working hours today, must be unplaced
         // within 8h horizon.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:609:
     async fn semi_event_costs_but_allows_placement() {
         let s = scheduler();
         let task = mk_task("squeeze", 30, 0.8);
[31m-        let ev = cal_event("standup", 0, 30, Rigidity::Semi(RigidityCost::CreditCost(5)));
(B[m[31m-        let sched = s.plan(&[task.clone()], &[ev], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let ev = cal_event(
(B[m[32m+            "standup",
(B[m[32m+            0,
(B[m[32m+            30,
(B[m[32m+            Rigidity::Semi(RigidityCost::CreditCost(5)),
(B[m[32m+        );
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[ev], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
[31m-        assert_eq!(sched.rigidity_cost.semi_cost_spent.get("credit").copied(), Some(5));
(B[m[32m+        assert_eq!(
(B[m[32m+            sched.rigidity_cost.semi_cost_spent.get("credit").copied(),
(B[m[32m+            Some(5)
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-PLAN-002
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:633:
                 now(),
             )
         };
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         // Task duration p90 = 120 min; max chunk 50 → expect >= 2 chunks.
[31m-        let for_task: Vec<_> = sched.assignments.iter().filter(|b| b.task_id == task.id).collect();
(B[m[32m+        let for_task: Vec<_> = sched
(B[m[32m+            .assignments
(B[m[32m+            .iter()
(B[m[32m+            .filter(|b| b.task_id == task.id)
(B[m[32m+            .collect();
(B[m         assert!(for_task.len() >= 2);
         let total: i64 = for_task.iter().map(|b| b.duration().num_minutes()).sum();
         assert_eq!(total, 120);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:650:
             constraints: vec![Constraint::NoEarlierThan(now() + Duration::hours(5))],
             ..mk_task("afternoon", 30, 0.5)
         };
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::hours(10)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::hours(10))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
         assert!(sched.assignments[0].starts_at >= now() + Duration::hours(5));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:661:
         let s = scheduler();
         // 10h task in a 4h horizon.
         let task = mk_task("too_big", 600, 0.5);
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::hours(4)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::hours(4))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(sched.assignments.is_empty());
         assert_eq!(sched.unplaced.len(), 1);
[31m-        assert!(matches!(sched.unplaced[0].1, UnplacedReason::InsufficientTime));
(B[m[32m+        assert!(matches!(
(B[m[32m+            sched.unplaced[0].1,
(B[m[32m+            UnplacedReason::InsufficientTime
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-PLAN-002
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:673:
         let s = scheduler();
         let id1 = Uuid::new_v4();
         let id2 = Uuid::new_v4();
[31m-        let t1 = Task { id: id1, ..mk_task("a", 60, 0.5) };
(B[m[31m-        let t2 = Task { id: id2, ..mk_task("b", 60, 0.5) };
(B[m[32m+        let t1 = Task {
(B[m[32m+            id: id1,
(B[m[32m+            ..mk_task("a", 60, 0.5)
(B[m[32m+        };
(B[m[32m+        let t2 = Task {
(B[m[32m+            id: id2,
(B[m[32m+            ..mk_task("b", 60, 0.5)
(B[m[32m+        };
(B[m         let tasks = vec![t1, t2];
[31m-        let a = s.plan(&tasks, &[], now(), Duration::hours(8)).await.unwrap();
(B[m[31m-        let b = s.plan(&tasks, &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let a = s
(B[m[32m+            .plan(&tasks, &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        let b = s
(B[m[32m+            .plan(&tasks, &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(a, b);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:687:
         let s = scheduler();
         let t1 = mk_task("keep", 60, 0.5);
         let t2 = mk_task("also_keep", 60, 0.5);
[31m-        let sched =
(B[m[31m-            s.plan(&[t1.clone(), t2.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[t1.clone(), t2.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let starts_before: Vec<_> = sched.assignments.iter().map(|b| b.starts_at).collect();
         let reflow = s.reflow(&sched, &[], now()).await.unwrap();
         let starts_after: Vec<_> = reflow.assignments.iter().map(|b| b.starts_at).collect();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:700:
     async fn reflow_handles_new_task_insertion() {
         let s = scheduler();
         let t1 = mk_task("existing", 60, 0.5);
[31m-        let sched = s.plan(&[t1.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[t1.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let t2 = mk_task("new", 30, 0.9);
[31m-        let reflow = s.reflow(&sched, &[ScheduleChange::NewTask(t2.clone())], now()).await.unwrap();
(B[m[32m+        let reflow = s
(B[m[32m+            .reflow(&sched, &[ScheduleChange::NewTask(t2.clone())], now())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(reflow.assignments.len(), 2);
         assert!(reflow.assignments.iter().any(|b| b.task_id == t1.id));
         assert!(reflow.assignments.iter().any(|b| b.task_id == t2.id));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:713:
     async fn reflow_drops_cancelled_task() {
         let s = scheduler();
         let t1 = mk_task("gone", 60, 0.5);
[31m-        let sched = s.plan(&[t1.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[t1.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
[31m-        let reflow =
(B[m[31m-            s.reflow(&sched, &[ScheduleChange::TaskCancelled(t1.id)], now()).await.unwrap();
(B[m[32m+        let reflow = s
(B[m[32m+            .reflow(&sched, &[ScheduleChange::TaskCancelled(t1.id)], now())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(reflow.assignments.is_empty());
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:724:
     #[tokio::test]
     async fn hard_deadline_bumps_urgency_score() {
         let s = scheduler();
[31m-        let no_dl = Task { priority: Priority::new(0.4), ..mk_task("no_dl", 30, 0.4) };
(B[m[32m+        let no_dl = Task {
(B[m[32m+            priority: Priority::new(0.4),
(B[m[32m+            ..mk_task("no_dl", 30, 0.4)
(B[m[32m+        };
(B[m         let hard_dl = Task {
             priority: Priority::new(0.4),
             deadline: Deadline::hard(now() + Duration::hours(1)),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:731:
             ..mk_task("urgent", 30, 0.4)
         };
         let sched = s
[31m-            .plan(&[no_dl.clone(), hard_dl.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .plan(
(B[m[32m+                &[no_dl.clone(), hard_dl.clone()],
(B[m[32m+                &[],
(B[m[32m+                now(),
(B[m[32m+                Duration::hours(8),
(B[m[32m+            )
(B[m             .await
             .unwrap();
         // Hard-deadline task should be scheduled first.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:777:
     async fn zero_windows_no_placement() {
         let s = scheduler();
         let task = mk_task("nofit", 60, 0.5);
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::zero()).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::zero())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(sched.assignments.is_empty());
         assert_eq!(sched.unplaced.len(), 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:793:
         ta.created_at = now() - Duration::hours(1);
         tb.created_at = now();
         // When priorities tie, the earlier-created task should be scheduled first
[31m-        let sched = s.plan(&[tb.clone(), ta.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[tb.clone(), ta.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         // ta should start first due to older created_at
         assert_eq!(sched.assignments[0].task_id, ta.id);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:805:
         let task = mk_task("override_soft", 60, 0.8);
         // Soft event covers part of working hours
         let ev = cal_event("meeting", 0, 120, Rigidity::Soft);
[31m-        let sched = s.plan(&[task.clone()], &[ev], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[ev], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         // Task should be placed despite soft conflict
         assert_eq!(sched.assignments.len(), 1);
         assert_eq!(sched.rigidity_cost.soft_overrides, 1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:823:
                 max_chunk: Duration::minutes(30),
                 ideal_chunk: Duration::minutes(25),
             },
[31m-            ..Task::new(
(B[m[31m-                "chunked",
(B[m[31m-                DurationSpec::fixed(Duration::minutes(80)),
(B[m[31m-                now(),
(B[m[31m-            )
(B[m[32m+            ..Task::new("chunked", DurationSpec::fixed(Duration::minutes(80)), now())
(B[m         };
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[31m-        let chunks: Vec<_> = sched.assignments.iter().filter(|b| b.task_id == task.id).collect();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        let chunks: Vec<_> = sched
(B[m[32m+            .assignments
(B[m[32m+            .iter()
(B[m[32m+            .filter(|b| b.task_id == task.id)
(B[m[32m+            .collect();
(B[m         assert!(chunks.len() >= 3); // 80 min / 30 max = 3+ chunks
         for chunk in &chunks {
             let dur = chunk.duration();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:854:
             ..mk_task("late", 30, 0.5)
         };
         let sched = s
[31m-            .plan(&[late_dl.clone(), early_dl.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .plan(
(B[m[32m+                &[late_dl.clone(), early_dl.clone()],
(B[m[32m+                &[],
(B[m[32m+                now(),
(B[m[32m+                Duration::hours(8),
(B[m[32m+            )
(B[m             .await
             .unwrap();
         // Earlier deadline should be scheduled first (higher urgency)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:870:
             priority: Priority::new(0.9),
             deadline: Deadline::hard(now() + Duration::hours(8)),
             chunking: ChunkingPolicy::atomic(),
[31m-            ..Task::new(
(B[m[31m-                "atomic",
(B[m[31m-                DurationSpec::fixed(Duration::minutes(90)),
(B[m[31m-                now(),
(B[m[31m-            )
(B[m[32m+            ..Task::new("atomic", DurationSpec::fixed(Duration::minutes(90)), now())
(B[m         };
[31m-        let sched = s.plan(&[task.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[31m-        let task_blocks: Vec<_> = sched.assignments.iter().filter(|b| b.task_id == task.id).collect();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[task.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        let task_blocks: Vec<_> = sched
(B[m[32m+            .assignments
(B[m[32m+            .iter()
(B[m[32m+            .filter(|b| b.task_id == task.id)
(B[m[32m+            .collect();
(B[m         // Since allow_split = false, should be 0 or 1 block, not multiple
         assert!(task_blocks.len() <= 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:889:
         // 10-hour task but only 1-hour working window per day
         let big_task = mk_task("giant", 600, 0.9);
         // Single day with just 1 hour available (9–10)
[31m-        let sched = s.plan(&[big_task.clone()], &[], now(), Duration::hours(1)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[big_task.clone()], &[], now(), Duration::hours(1))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(sched.assignments.is_empty());
         assert_eq!(sched.unplaced.len(), 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:902:
             constraints: vec![Constraint::NoEarlierThan(now() + Duration::hours(6))],
             ..mk_task("afternoon_only", 30, 0.5)
         };
[31m-        let sched = s.plan(&[constrained.clone()], &[], now(), Duration::hours(10)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[constrained.clone()], &[], now(), Duration::hours(10))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
         assert!(sched.assignments[0].starts_at >= now() + Duration::hours(6));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:915:
             constraints: vec![Constraint::NoLaterThan(now() + Duration::hours(4))],
             ..mk_task("early_only", 30, 0.5)
         };
[31m-        let sched = s.plan(&[constrained.clone()], &[], now(), Duration::hours(10)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[constrained.clone()], &[], now(), Duration::hours(10))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
         assert!(sched.assignments[0].ends_at <= now() + Duration::hours(4));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:931:
             constraints: vec![Constraint::NoLaterThan(now() + Duration::hours(2))],
             ..mk_task("impossible", 180, 0.9)
         };
[31m-        let sched = s.plan(&[tight.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[tight.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert!(sched.assignments.is_empty());
         assert_eq!(sched.unplaced.len(), 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:941:
     async fn reflow_applies_block_overrun() {
         let s = scheduler();
         let t1 = mk_task("run_long", 60, 0.5);
[31m-        let sched = s.plan(&[t1.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[t1.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(sched.assignments.len(), 1);
         let original_end = sched.assignments[0].ends_at;
         let new_end = original_end + Duration::minutes(30);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:948:
[31m-        let overrun = ScheduleChange::BlockOverran { task_id: t1.id, new_end };
(B[m[32m+        let overrun = ScheduleChange::BlockOverran {
(B[m[32m+            task_id: t1.id,
(B[m[32m+            new_end,
(B[m[32m+        };
(B[m         let reflow = s.reflow(&sched, &[overrun], now()).await.unwrap();
         // Block should be extended
         assert_eq!(reflow.assignments.len(), 1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:957:
     async fn reflow_handles_new_calendar_event_blocking() {
         let s = scheduler();
         let t1 = mk_task("scheduled", 60, 0.5);
[31m-        let sched = s.plan(&[t1.clone()], &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&[t1.clone()], &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let orig_count = sched.assignments.len();
         // Add a new hard calendar event that overlaps the task
         let hard_event = cal_event("blocking_meeting", 0, 120, Rigidity::Hard);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:964:
[31m-        let reflow =
(B[m[31m-            s.reflow(&sched, &[ScheduleChange::NewCalendarEvent(hard_event)], now()).await.unwrap();
(B[m[32m+        let reflow = s
(B[m[32m+            .reflow(
(B[m[32m+                &sched,
(B[m[32m+                &[ScheduleChange::NewCalendarEvent(hard_event)],
(B[m[32m+                now(),
(B[m[32m+            )
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         // Should have fewer or same assignments (depends on overlap timing)
         assert!(reflow.assignments.len() <= orig_count);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-scheduler/src/lib.rs:971:
     #[tokio::test]
     async fn multiple_tasks_priority_ordering() {
         let s = scheduler();
[31m-        let low1 = Task { priority: Priority::new(0.2), ..mk_task("low1", 30, 0.2) };
(B[m[31m-        let med = Task { priority: Priority::new(0.5), ..mk_task("med", 30, 0.5) };
(B[m[31m-        let high = Task { priority: Priority::new(0.9), ..mk_task("high", 30, 0.9) };
(B[m[32m+        let low1 = Task {
(B[m[32m+            priority: Priority::new(0.2),
(B[m[32m+            ..mk_task("low1", 30, 0.2)
(B[m[32m+        };
(B[m[32m+        let med = Task {
(B[m[32m+            priority: Priority::new(0.5),
(B[m[32m+            ..mk_task("med", 30, 0.5)
(B[m[32m+        };
(B[m[32m+        let high = Task {
(B[m[32m+            priority: Priority::new(0.9),
(B[m[32m+            ..mk_task("high", 30, 0.9)
(B[m[32m+        };
(B[m         let tasks = vec![low1.clone(), high.clone(), med.clone()];
[31m-        let sched = s.plan(&tasks, &[], now(), Duration::hours(8)).await.unwrap();
(B[m[32m+        let sched = s
(B[m[32m+            .plan(&tasks, &[], now(), Duration::hours(8))
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         // Should schedule all three
         assert_eq!(sched.assignments.len(), 3);
         // High priority should start earliest
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/audit_store.rs:31:
 
     /// Construct from an existing adapter, sharing its connection.
     pub fn from_adapter(adapter: &super::SqliteAdapter) -> Self {
[31m-        Self { conn: adapter.conn.clone() }
(B[m[32m+        Self {
(B[m[32m+            conn: adapter.conn.clone(),
(B[m[32m+        }
(B[m     }
 
     // --- async variants -----------------------------------------------------
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/audit_store.rs:83:
             let guard = conn.blocking_lock();
             let s = serde_json::to_string(&new_payload).context("serialize tamper payload")?;
             guard
[31m-                .execute("UPDATE audit_records SET payload = ?1 WHERE seq = ?2", params![s, seq])
(B[m[32m+                .execute(
(B[m[32m+                    "UPDATE audit_records SET payload = ?1 WHERE seq = ?2",
(B[m[32m+                    params![s, seq],
(B[m[32m+                )
(B[m                 .context("tamper update")?;
             Ok(())
         })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/audit_store.rs:107:
 
 fn head_hash_sync(conn: &Connection) -> Result<Option<String>> {
     let row: Option<String> = conn
[31m-        .query_row("SELECT hash FROM audit_records ORDER BY seq DESC LIMIT 1", [], |r| r.get(0))
(B[m[32m+        .query_row(
(B[m[32m+            "SELECT hash FROM audit_records ORDER BY seq DESC LIMIT 1",
(B[m[32m+            [],
(B[m[32m+            |r| r.get(0),
(B[m[32m+        )
(B[m         .optional()
         .context("query head_hash")?;
     Ok(row)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/audit_store.rs:122:
         );
     }
     let next_seq: i64 = conn
[31m-        .query_row("SELECT COALESCE(MAX(seq), 0) + 1 FROM audit_records", [], |r| r.get(0))
(B[m[32m+        .query_row(
(B[m[32m+            "SELECT COALESCE(MAX(seq), 0) + 1 FROM audit_records",
(B[m[32m+            [],
(B[m[32m+            |r| r.get(0),
(B[m[32m+        )
(B[m         .context("compute next audit seq")?;
     let payload = serde_json::to_string(&record.payload).context("serialize audit payload")?;
     conn.execute(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/cursor_store.rs:84:
     async fn save_then_load_roundtrips() {
         let a = adapter().await;
         a.save("canvas", "events", "cur-A").await.unwrap();
[31m-        assert_eq!(a.load("canvas", "events").await.unwrap().as_deref(), Some("cur-A"));
(B[m[32m+        assert_eq!(
(B[m[32m+            a.load("canvas", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("cur-A")
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-EVT-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/cursor_store.rs:93:
         let a = adapter().await;
         a.save("canvas", "events", "cur-A").await.unwrap();
         a.save("canvas", "events", "cur-B").await.unwrap();
[31m-        assert_eq!(a.load("canvas", "events").await.unwrap().as_deref(), Some("cur-B"));
(B[m[32m+        assert_eq!(
(B[m[32m+            a.load("canvas", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("cur-B")
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-EVT-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/cursor_store.rs:103:
         a.save("canvas", "events", "C-E").await.unwrap();
         a.save("canvas", "tasks", "C-T").await.unwrap();
         a.save("google", "events", "G-E").await.unwrap();
[31m-        assert_eq!(a.load("canvas", "events").await.unwrap().as_deref(), Some("C-E"));
(B[m[31m-        assert_eq!(a.load("canvas", "tasks").await.unwrap().as_deref(), Some("C-T"));
(B[m[31m-        assert_eq!(a.load("google", "events").await.unwrap().as_deref(), Some("G-E"));
(B[m[32m+        assert_eq!(
(B[m[32m+            a.load("canvas", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("C-E")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            a.load("canvas", "tasks").await.unwrap().as_deref(),
(B[m[32m+            Some("C-T")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            a.load("google", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("G-E")
(B[m[32m+        );
(B[m         assert_eq!(a.load("google", "tasks").await.unwrap(), None);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/cursor_store.rs:125:
             let _ = Arc::new(a);
         }
         let b = SqliteAdapter::open(&path).unwrap();
[31m-        assert_eq!(b.load("canvas", "events").await.unwrap().as_deref(), Some("persisted"),);
(B[m[32m+        assert_eq!(
(B[m[32m+            b.load("canvas", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("persisted"),
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-EVT-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/cursor_store.rs:135:
         // we treat it as data, not absence.
         let a = adapter().await;
         a.save("canvas", "events", "").await.unwrap();
[31m-        assert_eq!(a.load("canvas", "events").await.unwrap().as_deref(), Some(""));
(B[m[32m+        assert_eq!(
(B[m[32m+            a.load("canvas", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("")
(B[m[32m+        );
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/event_dedup.rs:114:
         assert!(!adapter.is_seen(&key).await.expect("is_seen 1"));
 
         // Mark seen with 30-day TTL
[31m-        adapter
(B[m[31m-            .mark_seen(&key, 2_592_000)
(B[m[31m-            .await
(B[m[31m-            .expect("mark_seen");
(B[m[32m+        adapter.mark_seen(&key, 2_592_000).await.expect("mark_seen");
(B[m 
         // Now seen
         assert!(adapter.is_seen(&key).await.expect("is_seen 2"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/event_dedup.rs:184:
         // Purge entries older than a time in the past (should purge nothing,
         // because our entries were just inserted and are newer than past_time).
         let past_time = Utc::now() - Duration::days(30);
[31m-        let count = adapter
(B[m[31m-            .purge_older_than(past_time)
(B[m[31m-            .await
(B[m[31m-            .expect("purge");
(B[m[31m-        assert_eq!(count, 0, "should not purge recent entries when cutoff is in past");
(B[m[32m+        let count = adapter.purge_older_than(past_time).await.expect("purge");
(B[m[32m+        assert_eq!(
(B[m[32m+            count, 0,
(B[m[32m+            "should not purge recent entries when cutoff is in past"
(B[m[32m+        );
(B[m 
         // Purge entries older than far future (should purge both,
         // because all current entries are older than far_future).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/event_dedup.rs:195:
         let far_future = Utc::now() + Duration::days(30);
[31m-        let count = adapter
(B[m[31m-            .purge_older_than(far_future)
(B[m[31m-            .await
(B[m[31m-            .expect("purge");
(B[m[31m-        assert_eq!(count, 2, "should purge all entries when cutoff is far in future");
(B[m[32m+        let count = adapter.purge_older_than(far_future).await.expect("purge");
(B[m[32m+        assert_eq!(
(B[m[32m+            count, 2,
(B[m[32m+            "should purge all entries when cutoff is far in future"
(B[m[32m+        );
(B[m 
         // Verify they are gone
         assert!(!adapter.is_seen(&key1).await.expect("is_seen 1"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/migrations.rs:167:
         if applied.contains(version) {
             continue;
         }
[31m-        let tx = conn.transaction().with_context(|| format!("begin migration {version}"))?;
(B[m[31m-        tx.execute_batch(sql).with_context(|| format!("apply migration {version}"))?;
(B[m[32m+        let tx = conn
(B[m[32m+            .transaction()
(B[m[32m+            .with_context(|| format!("begin migration {version}"))?;
(B[m[32m+        tx.execute_batch(sql)
(B[m[32m+            .with_context(|| format!("apply migration {version}"))?;
(B[m         tx.execute(
             "INSERT INTO _migrations (version, applied_at) VALUES (?1, ?2)",
             params![*version as i64, chrono::Utc::now().to_rfc3339()],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/migrations.rs:175:
         )
         .with_context(|| format!("record migration {version}"))?;
[31m-        tx.commit().with_context(|| format!("commit migration {version}"))?;
(B[m[32m+        tx.commit()
(B[m[32m+            .with_context(|| format!("commit migration {version}"))?;
(B[m     }
     Ok(())
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/migrations.rs:189:
         let mut conn = Connection::open_in_memory().expect("open");
         run(&mut conn).expect("first run");
         run(&mut conn).expect("second run");
[31m-        let count: i64 =
(B[m[31m-            conn.query_row("SELECT COUNT(*) FROM _migrations", [], |r| r.get(0)).expect("count");
(B[m[32m+        let count: i64 = conn
(B[m[32m+            .query_row("SELECT COUNT(*) FROM _migrations", [], |r| r.get(0))
(B[m[32m+            .expect("count");
(B[m         assert_eq!(count, MIGRATIONS.len() as i64);
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/mod.rs:33:
         let mut conn = Connection::open(path)
             .with_context(|| format!("open sqlite db at {}", path.display()))?;
         migrations::run(&mut conn)?;
[31m-        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
(B[m[32m+        Ok(Self {
(B[m[32m+            conn: Arc::new(Mutex::new(conn)),
(B[m[32m+        })
(B[m     }
 
     /// Open an in-memory SQLite database (tests).
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/mod.rs:40:
     pub fn open_in_memory() -> Result<Self> {
         let mut conn = Connection::open_in_memory().context("open sqlite in-memory")?;
         migrations::run(&mut conn)?;
[31m-        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
(B[m[32m+        Ok(Self {
(B[m[32m+            conn: Arc::new(Mutex::new(conn)),
(B[m[32m+        })
(B[m     }
 
     /// Helper for tests: create adapter from a raw Connection with blocking Mutex.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/penalty_store.rs:55:
             .context("prepare lockouts")?;
         let rows = stmt
             .query_map(params![uid], |row| {
[31m-                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
(B[m[32m+                Ok((
(B[m[32m+                    row.get::<_, String>(0)?,
(B[m[32m+                    row.get::<_, String>(1)?,
(B[m[32m+                    row.get::<_, String>(2)?,
(B[m[32m+                ))
(B[m             })
             .context("query lockouts")?;
         for row in rows {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/penalty_store.rs:99:
     )
     .context("upsert penalty_state")?;
 
[31m-    conn.execute("DELETE FROM lockout_windows WHERE user_id = ?1", params![uid])
(B[m[31m-        .context("clear lockouts")?;
(B[m[32m+    conn.execute(
(B[m[32m+        "DELETE FROM lockout_windows WHERE user_id = ?1",
(B[m[32m+        params![uid],
(B[m[32m+    )
(B[m[32m+    .context("clear lockouts")?;
(B[m     for w in &state.lockout_windows {
         conn.execute(
             "INSERT INTO lockout_windows (user_id, starts_at, ends_at, reason) \
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/rule_store.rs:12:
 use super::SqliteAdapter;
 use crate::ports::RuleStore;
 
[31m-type RuleRow = (String, String, i64, i64, Option<i64>, Option<i64>, String, String, String, String);
(B[m[32m+type RuleRow = (
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    i64,
(B[m[32m+    i64,
(B[m[32m+    Option<i64>,
(B[m[32m+    Option<i64>,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+);
(B[m 
 fn row_to_rule(row: RuleRow) -> Result<Rule> {
     let (id, name, enabled, priority, cs, ds, tmpl, tj, cj, aj) = row;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/task_store.rs:35:
 
     /// Create a new task store from a SqliteAdapter.
     pub fn from_adapter(adapter: &SqliteAdapter) -> Self {
[31m-        Self { conn: adapter.conn.clone() }
(B[m[32m+        Self {
(B[m[32m+            conn: adapter.conn.clone(),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/task_store.rs:51:
     }
 }
 
[31m-type TaskRow =
(B[m[31m-    (String, String, String, String, i64, String, Option<String>, String, String, String, String);
(B[m[32m+type TaskRow = (
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    i64,
(B[m[32m+    String,
(B[m[32m+    Option<String>,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+);
(B[m 
 fn task_to_row(user_id: Uuid, task: &Task) -> Result<TaskRow> {
     let duration_spec = serde_json::to_string(&task.duration).context("serialize duration_spec")?;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/task_store.rs:88:
     ))
 }
 
[31m-type TaskRowRead =
(B[m[31m-    (String, String, String, i64, String, Option<String>, String, String, String, String);
(B[m[32m+type TaskRowRead = (
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    i64,
(B[m[32m+    String,
(B[m[32m+    Option<String>,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+    String,
(B[m[32m+);
(B[m 
 fn row_to_task(row: TaskRowRead) -> Result<Task> {
     let (
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/wallet_store.rs:48:
             let last = parse_rfc3339_opt(last)?;
             streaks.insert(
                 name.clone(),
[31m-                Streak { name, count: count as u32, last_incremented_at: last },
(B[m[32m+                Streak {
(B[m[32m+                    name,
(B[m[32m+                    count: count as u32,
(B[m[32m+                    last_incremented_at: last,
(B[m[32m+                },
(B[m             );
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/wallet_store.rs:101:
     )
     .context("upsert wallet")?;
 
[31m-    conn.execute("DELETE FROM wallet_streaks WHERE user_id = ?1", params![uid])
(B[m[31m-        .context("clear streaks")?;
(B[m[32m+    conn.execute(
(B[m[32m+        "DELETE FROM wallet_streaks WHERE user_id = ?1",
(B[m[32m+        params![uid],
(B[m[32m+    )
(B[m[32m+    .context("clear streaks")?;
(B[m     for (name, s) in &wallet.streaks {
         conn.execute(
             "INSERT INTO wallet_streaks (user_id, name, count, last_incremented_at) \
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/wallet_store.rs:109:
              VALUES (?1,?2,?3,?4)",
[31m-            params![uid, name, s.count as i64, s.last_incremented_at.map(rfc3339)],
(B[m[32m+            params![
(B[m[32m+                uid,
(B[m[32m+                name,
(B[m[32m+                s.count as i64,
(B[m[32m+                s.last_incremented_at.map(rfc3339)
(B[m[32m+            ],
(B[m         )
         .context("insert streak")?;
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/sqlite/wallet_store.rs:114:
 
[31m-    conn.execute("DELETE FROM wallet_unlocks WHERE user_id = ?1", params![uid])
(B[m[31m-        .context("clear unlocks")?;
(B[m[32m+    conn.execute(
(B[m[32m+        "DELETE FROM wallet_unlocks WHERE user_id = ?1",
(B[m[32m+        params![uid],
(B[m[32m+    )
(B[m[32m+    .context("clear unlocks")?;
(B[m     for (k, v) in &wallet.unlock_balances {
         conn.execute(
             "INSERT INTO wallet_unlocks (user_id, key, value) VALUES (?1,?2,?3)",
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:38:
     /// Typically `~/Library/Application Support/FocalPoint/wipe-receipts/` on macOS.
     fn receipt_dir() -> Result<PathBuf> {
         let app_support = if cfg!(target_os = "macos") {
[31m-            let home = std::env::var("HOME")
(B[m[31m-                .context("HOME env var not set")?;
(B[m[31m-            PathBuf::from(home)
(B[m[31m-                .join("Library/Application Support/FocalPoint")
(B[m[32m+            let home = std::env::var("HOME").context("HOME env var not set")?;
(B[m[32m+            PathBuf::from(home).join("Library/Application Support/FocalPoint")
(B[m         } else if cfg!(target_os = "linux") {
[31m-            let home = std::env::var("HOME")
(B[m[31m-                .context("HOME env var not set")?;
(B[m[31m-            PathBuf::from(home)
(B[m[31m-                .join(".config/FocalPoint")
(B[m[32m+            let home = std::env::var("HOME").context("HOME env var not set")?;
(B[m[32m+            PathBuf::from(home).join(".config/FocalPoint")
(B[m         } else {
             anyhow::bail!("unsupported platform for wipe receipts")
         };
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:106:
 
         for table in &tables {
             let count: i64 = conn
[31m-                .query_row(
(B[m[31m-                    &format!("SELECT COUNT(*) FROM {}", table),
(B[m[31m-                    [],
(B[m[31m-                    |row| row.get(0),
(B[m[31m-                )
(B[m[32m+                .query_row(&format!("SELECT COUNT(*) FROM {}", table), [], |row| {
(B[m[32m+                    row.get(0)
(B[m[32m+                })
(B[m                 .unwrap_or(0);
 
             if count > 0 {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:122:
         }
 
         // Vacuum to reclaim space.
[31m-        conn.execute("VACUUM", [])
(B[m[31m-            .context("vacuum database")?;
(B[m[32m+        conn.execute("VACUUM", []).context("vacuum database")?;
(B[m     }
 
     // TODO: Wipe keychain items via SecureSecretStore::wipe_all() once trait is extended.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:150:
     // Traces to: FR-PRIVACY-001
     #[tokio::test]
     async fn wipe_empty_database() {
[31m-        let adapter = crate::sqlite::SqliteAdapter::open_in_memory()
(B[m[31m-            .expect("create adapter");
(B[m[32m+        let adapter = crate::sqlite::SqliteAdapter::open_in_memory().expect("create adapter");
(B[m         let receipt = wipe_all(&adapter).await.expect("wipe");
         assert!(receipt.wiped_at.len() > 20); // ISO 8601 with milliseconds is ~30+ chars
         assert_eq!(receipt.pre_wipe_chain_hash, "none"); // Empty DB has no audit records
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:160:
     // Traces to: FR-PRIVACY-001
     #[tokio::test]
     async fn wipe_with_data() {
[31m-        let adapter = crate::sqlite::SqliteAdapter::open_in_memory()
(B[m[31m-            .expect("create adapter");
(B[m[32m+        let adapter = crate::sqlite::SqliteAdapter::open_in_memory().expect("create adapter");
(B[m 
         // Seed some data.
         {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:171:
                  effective_at, dedupe_key, confidence, payload, raw_ref) \
                  VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                 params![
[31m-                    "evt1", "conn1", "acc1", "login", "2026-04-23T00:00:00Z",
(B[m[31m-                    "2026-04-23T00:00:00Z", "key1", 1.0, "{}", None::<String>
(B[m[32m+                    "evt1",
(B[m[32m+                    "conn1",
(B[m[32m+                    "acc1",
(B[m[32m+                    "login",
(B[m[32m+                    "2026-04-23T00:00:00Z",
(B[m[32m+                    "2026-04-23T00:00:00Z",
(B[m[32m+                    "key1",
(B[m[32m+                    1.0,
(B[m[32m+                    "{}",
(B[m[32m+                    None::<String>
(B[m                 ],
             )
             .expect("insert event");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:197:
     // Traces to: FR-PRIVACY-001
     #[tokio::test]
     async fn receipt_is_valid_json() {
[31m-        let adapter = crate::sqlite::SqliteAdapter::open_in_memory()
(B[m[31m-            .expect("create adapter");
(B[m[32m+        let adapter = crate::sqlite::SqliteAdapter::open_in_memory().expect("create adapter");
(B[m         let receipt = wipe_all(&adapter).await.expect("wipe");
 
         let json = serde_json::to_string(&receipt).expect("serialize");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:205:
[31m-        let _deserialized: WipeReceipt =
(B[m[31m-            serde_json::from_str(&json).expect("deserialize");
(B[m[32m+        let _deserialized: WipeReceipt = serde_json::from_str(&json).expect("deserialize");
(B[m     }
 
     // Traces to: FR-PRIVACY-001
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/src/wipe.rs:210:
     #[tokio::test]
     async fn double_wipe_is_idempotent() {
[31m-        let adapter = crate::sqlite::SqliteAdapter::open_in_memory()
(B[m[31m-            .expect("create adapter");
(B[m[32m+        let adapter = crate::sqlite::SqliteAdapter::open_in_memory().expect("create adapter");
(B[m 
         // First wipe.
         let receipt1 = wipe_all(&adapter).await.expect("first wipe");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:23:
         connector_id: "canvas".into(),
         account_id: Uuid::nil(),
         event_type: et,
[31m-        occurred_at: Utc.with_ymd_and_hms(2026, 1, 1, seed as u32 % 24, 0, 0).unwrap(),
(B[m[31m-        effective_at: Utc.with_ymd_and_hms(2026, 1, 1, seed as u32 % 24, 0, 0).unwrap(),
(B[m[32m+        occurred_at: Utc
(B[m[32m+            .with_ymd_and_hms(2026, 1, 1, seed as u32 % 24, 0, 0)
(B[m[32m+            .unwrap(),
(B[m[32m+        effective_at: Utc
(B[m[32m+            .with_ymd_and_hms(2026, 1, 1, seed as u32 % 24, 0, 0)
(B[m[32m+            .unwrap(),
(B[m         dedupe_key: DedupeKey(dedupe.to_string()),
         confidence: 1.0,
         payload: json!({"seed": seed}),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:46:
     let db_path = dir.join("focus.db");
     {
         let a = SqliteAdapter::open(&db_path).expect("open");
[31m-        let ev = mk_event(1, "ddk-1", EventType::WellKnown(WellKnownEventType::TaskCompleted));
(B[m[32m+        let ev = mk_event(
(B[m[32m+            1,
(B[m[32m+            "ddk-1",
(B[m[32m+            EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+        );
(B[m         a.append(ev).await.expect("append");
     }
     // reopen and confirm the event persists
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:71:
 #[tokio::test]
 async fn event_dedupe_is_no_op() {
     let a = SqliteAdapter::open_in_memory().unwrap();
[31m-    let ev1 = mk_event(1, "ddk-dupe", EventType::WellKnown(WellKnownEventType::TaskCompleted));
(B[m[31m-    let ev2_same_key = NormalizedEvent { event_id: Uuid::from_bytes([2; 16]), ..ev1.clone() };
(B[m[32m+    let ev1 = mk_event(
(B[m[32m+        1,
(B[m[32m+        "ddk-dupe",
(B[m[32m+        EventType::WellKnown(WellKnownEventType::TaskCompleted),
(B[m[32m+    );
(B[m[32m+    let ev2_same_key = NormalizedEvent {
(B[m[32m+        event_id: Uuid::from_bytes([2; 16]),
(B[m[32m+        ..ev1.clone()
(B[m[32m+    };
(B[m     a.append(ev1.clone()).await.unwrap();
     a.append(ev2_same_key).await.unwrap();
     let events = a.since_cursor(None, 10).await.unwrap();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:79:
[31m-    assert_eq!(events.len(), 1, "second event with same dedupe_key must be ignored");
(B[m[32m+    assert_eq!(
(B[m[32m+        events.len(),
(B[m[32m+        1,
(B[m[32m+        "second event with same dedupe_key must be ignored"
(B[m[32m+    );
(B[m     assert_eq!(events[0].event_id, ev1.event_id);
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:105:
 #[tokio::test]
 async fn event_roundtrip_preserves_fields() {
     let a = SqliteAdapter::open_in_memory().unwrap();
[31m-    let ev = mk_event(7, "ddk-rt", EventType::WellKnown(WellKnownEventType::AppSessionStarted));
(B[m[32m+    let ev = mk_event(
(B[m[32m+        7,
(B[m[32m+        "ddk-rt",
(B[m[32m+        EventType::WellKnown(WellKnownEventType::AppSessionStarted),
(B[m[32m+    );
(B[m     a.append(ev.clone()).await.unwrap();
     let fetched = get_by_id(&a, ev.event_id).await.unwrap().expect("present");
     assert_eq!(fetched.event_id, ev.event_id);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:122:
         id: Uuid::new_v4(),
         name: "grant-on-task".into(),
         trigger: Trigger::Event("TaskCompleted".into()),
[31m-        conditions: vec![Condition { kind: "confidence_gte".into(), params: json!({"min": 0.5}) }],
(B[m[32m+        conditions: vec![Condition {
(B[m[32m+            kind: "confidence_gte".into(),
(B[m[32m+            params: json!({"min": 0.5}),
(B[m[32m+        }],
(B[m         actions: vec![Action::GrantCredit { amount: 10 }],
         priority: 5,
         cooldown: Some(Duration::minutes(30)),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:130:
         explanation_template: "{rule_name}".into(),
         enabled: true,
     };
[31m-    let disabled = Rule { id: Uuid::new_v4(), enabled: false, ..enabled.clone() };
(B[m[32m+    let disabled = Rule {
(B[m[32m+        id: Uuid::new_v4(),
(B[m[32m+        enabled: false,
(B[m[32m+        ..enabled.clone()
(B[m[32m+    };
(B[m     upsert_rule(&a, enabled.clone()).await.unwrap();
     upsert_rule(&a, disabled.clone()).await.unwrap();
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:176:
     WalletStore::apply(
         &a,
         uid,
[31m-        WalletMutation::SpendCredit { amount: 40, purpose: "unlock".into() },
(B[m[32m+        WalletMutation::SpendCredit {
(B[m[32m+            amount: 40,
(B[m[32m+            purpose: "unlock".into(),
(B[m[32m+        },
(B[m     )
     .await
     .unwrap();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:183:
[31m-    WalletStore::apply(&a, uid, WalletMutation::StreakIncrement("daily".into())).await.unwrap();
(B[m[32m+    WalletStore::apply(&a, uid, WalletMutation::StreakIncrement("daily".into()))
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m 
     let w = WalletStore::load(&a, uid).await.unwrap();
     assert_eq!(w.earned_credits, 100);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:194:
 async fn wallet_insufficient_credit_rejected() {
     let a = SqliteAdapter::open_in_memory().unwrap();
     let uid = Uuid::new_v4();
[31m-    let err =
(B[m[31m-        WalletStore::apply(&a, uid, WalletMutation::SpendCredit { amount: 5, purpose: "x".into() })
(B[m[31m-            .await
(B[m[31m-            .unwrap_err();
(B[m[32m+    let err = WalletStore::apply(
(B[m[32m+        &a,
(B[m[32m+        uid,
(B[m[32m+        WalletMutation::SpendCredit {
(B[m[32m+            amount: 5,
(B[m[32m+            purpose: "x".into(),
(B[m[32m+        },
(B[m[32m+    )
(B[m[32m+    .await
(B[m[32m+    .unwrap_err();
(B[m     assert!(format!("{err}").contains("wallet mutation"));
     let w = WalletStore::load(&a, uid).await.unwrap();
     assert_eq!(w.balance(), 0);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:209:
     let a = SqliteAdapter::open_in_memory().unwrap();
     let uid = Uuid::new_v4();
 
[31m-    PenaltyStore::apply(&a, uid, PenaltyMutation::Escalate(EscalationTier::Warning)).await.unwrap();
(B[m[31m-    PenaltyStore::apply(&a, uid, PenaltyMutation::GrantBypass(10)).await.unwrap();
(B[m[31m-    PenaltyStore::apply(&a, uid, PenaltyMutation::SpendBypass(3)).await.unwrap();
(B[m[32m+    PenaltyStore::apply(&a, uid, PenaltyMutation::Escalate(EscalationTier::Warning))
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m[32m+    PenaltyStore::apply(&a, uid, PenaltyMutation::GrantBypass(10))
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m[32m+    PenaltyStore::apply(&a, uid, PenaltyMutation::SpendBypass(3))
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m 
     let s = PenaltyStore::load(&a, uid).await.unwrap();
     assert_eq!(s.escalation_tier, EscalationTier::Warning);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_adapter.rs:223:
 async fn penalty_escalation_down_is_rejected() {
     let a = SqliteAdapter::open_in_memory().unwrap();
     let uid = Uuid::new_v4();
[31m-    PenaltyStore::apply(&a, uid, PenaltyMutation::Escalate(EscalationTier::Restricted))
(B[m[31m-        .await
(B[m[31m-        .unwrap();
(B[m[32m+    PenaltyStore::apply(
(B[m[32m+        &a,
(B[m[32m+        uid,
(B[m[32m+        PenaltyMutation::Escalate(EscalationTier::Restricted),
(B[m[32m+    )
(B[m[32m+    .await
(B[m[32m+    .unwrap();
(B[m     let err = PenaltyStore::apply(&a, uid, PenaltyMutation::Escalate(EscalationTier::Warning))
         .await
         .unwrap_err();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_audit_store.rs:17:
 async fn append_and_head_hash_roundtrip() {
     let store = mk_store();
     assert_eq!(store.head_hash_async().await.unwrap(), None);
[31m-    let rec =
(B[m[31m-        append_mutation(&store, "wallet.grant", "user-1", &json!({"v": 1}), chrono::Utc::now())
(B[m[31m-            .expect("append_mutation");
(B[m[32m+    let rec = append_mutation(
(B[m[32m+        &store,
(B[m[32m+        "wallet.grant",
(B[m[32m+        "user-1",
(B[m[32m+        &json!({"v": 1}),
(B[m[32m+        chrono::Utc::now(),
(B[m[32m+    )
(B[m[32m+    .expect("append_mutation");
(B[m     assert_eq!(rec.prev_hash, GENESIS_PREV_HASH);
     assert_eq!(store.head_hash_async().await.unwrap(), Some(rec.hash));
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_audit_store.rs:77:
         .unwrap();
     }
     // Mutate row-2's payload directly, bypassing the chain's hash logic.
[31m-    store.__test_tamper_payload(2, json!({"i": 999})).await.unwrap();
(B[m[31m-    assert!(!store.verify_chain_async().await.unwrap(), "chain verify must fail after row tamper");
(B[m[32m+    store
(B[m[32m+        .__test_tamper_payload(2, json!({"i": 999}))
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m[32m+    assert!(
(B[m[32m+        !store.verify_chain_async().await.unwrap(),
(B[m[32m+        "chain verify must fail after row tamper"
(B[m[32m+    );
(B[m }
 
 // Traces to: FR-STATE-004
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_audit_store.rs:125:
     let hash_after_write = {
         let adapter = SqliteAdapter::open(&path).expect("open1");
         let store = SqliteAuditStore::from_adapter(&adapter);
[31m-        let rec =
(B[m[31m-            append_mutation(&store, "policy.built", "subj", &json!({"x": 1}), chrono::Utc::now())
(B[m[31m-                .unwrap();
(B[m[32m+        let rec = append_mutation(
(B[m[32m+            &store,
(B[m[32m+            "policy.built",
(B[m[32m+            "subj",
(B[m[32m+            &json!({"x": 1}),
(B[m[32m+            chrono::Utc::now(),
(B[m[32m+        )
(B[m[32m+        .unwrap();
(B[m         rec.hash
     };
     let adapter2 = SqliteAdapter::open(&path).expect("open2");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-storage/tests/sqlite_audit_store.rs:134:
     let store2 = SqliteAuditStore::from_adapter(&adapter2);
[31m-    assert_eq!(store2.head_hash_async().await.unwrap(), Some(hash_after_write));
(B[m[32m+    assert_eq!(
(B[m[32m+        store2.head_hash_async().await.unwrap(),
(B[m[32m+        Some(hash_after_write)
(B[m[32m+    );
(B[m     let _ = std::fs::remove_dir_all(&dir);
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/cloudkit_port.rs:43:
     async fn get_last_sync_time(&self) -> Result<Option<std::time::SystemTime>, CloudKitPortError>;
 
     /// Store the timestamp of the last successful sync.
[31m-    async fn set_last_sync_time(&self, time: std::time::SystemTime)
(B[m[31m-        -> Result<(), CloudKitPortError>;
(B[m[32m+    async fn set_last_sync_time(
(B[m[32m+        &self,
(B[m[32m+        time: std::time::SystemTime,
(B[m[32m+    ) -> Result<(), CloudKitPortError>;
(B[m }
 
 /// A record to push to or pull from CloudKit.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/cloudkit_port.rs:130:
     }
 
     async fn pull(&self) -> Result<PullOutcome, CloudKitPortError> {
[31m-        Ok(PullOutcome { pulled: vec![], conflicts: vec![] })
(B[m[32m+        Ok(PullOutcome {
(B[m[32m+            pulled: vec![],
(B[m[32m+            conflicts: vec![],
(B[m[32m+        })
(B[m     }
 
     async fn setup_subscription(&self) -> Result<(), CloudKitPortError> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/cloudkit_port.rs:137:
         Ok(())
     }
 
[31m-    async fn get_last_sync_time(
(B[m[31m-        &self,
(B[m[31m-    ) -> Result<Option<std::time::SystemTime>, CloudKitPortError> {
(B[m[32m+    async fn get_last_sync_time(&self) -> Result<Option<std::time::SystemTime>, CloudKitPortError> {
(B[m         Ok(None)
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/cloudkit_port.rs:168:
     #[tokio::test]
     async fn noop_port_echoes_push_count() {
         let port = NoopCloudKitPort;
[31m-        let records = vec![
(B[m[31m-            CloudKitRecord {
(B[m[31m-                record_id: Uuid::new_v4(),
(B[m[31m-                record_type: "Wallet".into(),
(B[m[31m-                device_id: Uuid::new_v4(),
(B[m[31m-                payload_json: vec![],
(B[m[31m-                device_signature: "sig".into(),
(B[m[31m-                version: 1,
(B[m[31m-                synced_at: std::time::SystemTime::now(),
(B[m[31m-            },
(B[m[31m-        ];
(B[m[32m+        let records = vec![CloudKitRecord {
(B[m[32m+            record_id: Uuid::new_v4(),
(B[m[32m+            record_type: "Wallet".into(),
(B[m[32m+            device_id: Uuid::new_v4(),
(B[m[32m+            payload_json: vec![],
(B[m[32m+            device_signature: "sig".into(),
(B[m[32m+            version: 1,
(B[m[32m+            synced_at: std::time::SystemTime::now(),
(B[m[32m+        }];
(B[m         let count = port.push(records).await.unwrap();
         assert_eq!(count, 1);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/cursor_store.rs:91:
         self.inner
             .lock()
             .map_err(|e| anyhow::anyhow!("poisoned: {e}"))?
[31m-            .insert((connector_id.to_string(), entity_type.to_string()), cursor.to_string());
(B[m[32m+            .insert(
(B[m[32m+                (connector_id.to_string(), entity_type.to_string()),
(B[m[32m+                cursor.to_string(),
(B[m[32m+            );
(B[m         Ok(())
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/cursor_store.rs:117:
         let s = InMemoryCursorStore::new();
         assert_eq!(s.load("c", "events").await.unwrap(), None);
         s.save("c", "events", "cur1").await.unwrap();
[31m-        assert_eq!(s.load("c", "events").await.unwrap().as_deref(), Some("cur1"));
(B[m[32m+        assert_eq!(
(B[m[32m+            s.load("c", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("cur1")
(B[m[32m+        );
(B[m         s.save("c", "events", "cur2").await.unwrap();
[31m-        assert_eq!(s.load("c", "events").await.unwrap().as_deref(), Some("cur2"));
(B[m[32m+        assert_eq!(
(B[m[32m+            s.load("c", "events").await.unwrap().as_deref(),
(B[m[32m+            Some("cur2")
(B[m[32m+        );
(B[m         // Different entity-type is isolated.
         assert_eq!(s.load("c", "tasks").await.unwrap(), None);
         // Different connector is isolated.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/dedup_event_sink.rs:42:
         .map_err(|e| anyhow::anyhow!("hash computation: {e}"))?;
 
         // Check if this event has been seen before
[31m-        let is_duplicate = self.deduplicator.is_seen(&hash).await
(B[m[32m+        let is_duplicate = self
(B[m[32m+            .deduplicator
(B[m[32m+            .is_seen(&hash)
(B[m[32m+            .await
(B[m             .map_err(|e| anyhow::anyhow!("dedup check: {e}"))?;
 
         if is_duplicate {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:7:
 //!
 //! Traces to: FR-CONN-003, FR-EVT-002
 
[32m+pub mod cloudkit_port;
(B[m pub mod cursor_store;
 pub mod dedup_event_sink;
 pub mod event_sink;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:13:
 pub mod retry;
[31m-pub mod cloudkit_port;
(B[m 
[32m+pub use cloudkit_port::{
(B[m[32m+    CloudKitPort, CloudKitPortError, CloudKitRecord, ConflictRecord, ConflictResolution,
(B[m[32m+    NoopCloudKitPort, PullOutcome,
(B[m[32m+};
(B[m pub use cursor_store::{CursorStore, InMemoryCursorStore, NoopCursorStore, EVENTS_ENTITY_TYPE};
 pub use dedup_event_sink::DeduplicatingEventSink;
 pub use event_sink::{EventSink, NoopEventSink};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:19:
 pub use retry::{next_delay, RetryPolicy};
[31m-pub use cloudkit_port::{CloudKitPort, CloudKitRecord, CloudKitPortError, ConflictRecord, ConflictResolution, NoopCloudKitPort, PullOutcome};
(B[m 
 use chrono::{DateTime, Duration as ChronoDuration, Utc};
 use focus_connectors::{Connector, ConnectorError, HealthState};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:170:
     ///
     /// Idempotent; replaces any prior deduplicator. Call this AFTER wiring
     /// the event sink, as the dedup wrapper will replace it with a DeduplicatingEventSink.
[31m-    pub fn with_deduplicator(mut self, dedup: Arc<dyn focus_events::dedup::EventDeduplicator>) -> Self {
(B[m[32m+    pub fn with_deduplicator(
(B[m[32m+        mut self,
(B[m[32m+        dedup: Arc<dyn focus_events::dedup::EventDeduplicator>,
(B[m[32m+    ) -> Self {
(B[m         self.deduplicator = Some(dedup.clone());
         // Wrap the current sink with dedup
         self.event_sink = Arc::new(DeduplicatingEventSink::new(self.event_sink.clone(), dedup));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:336:
                     // without re-ingesting from scratch. Persist errors are
                     // logged but non-fatal -- next successful sync retries.
                     if let Some(cursor) = outcome.next_cursor.as_deref() {
[31m-                        if let Err(e) =
(B[m[31m-                            self.cursor_store.save(&id, EVENTS_ENTITY_TYPE, cursor).await
(B[m[32m+                        if let Err(e) = self
(B[m[32m+                            .cursor_store
(B[m[32m+                            .save(&id, EVENTS_ENTITY_TYPE, cursor)
(B[m[32m+                            .await
(B[m                         {
                             warn!(connector_id = %id, error = %e, "cursor persist failed");
                         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:361:
                     handle.health = HealthState::Degraded(format!("rate_limited:{seconds}"));
                     report.errors.push(SyncErrorEntry {
                         connector_id: id.clone(),
[31m-                        kind: SyncErrorKind::RateLimited { retry_after_s: seconds },
(B[m[32m+                        kind: SyncErrorKind::RateLimited {
(B[m[32m+                            retry_after_s: seconds,
(B[m[32m+                        },
(B[m                         message: format!("rate limited for {seconds}s"),
                     });
                 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:475:
                     version: "test".into(),
                     display_name: id.into(),
                     auth_strategy: AuthStrategy::None,
[31m-                    sync_mode: SyncMode::Polling { cadence_seconds: 60 },
(B[m[32m+                    sync_mode: SyncMode::Polling {
(B[m[32m+                        cadence_seconds: 60,
(B[m[32m+                    },
(B[m                     capabilities: vec![],
                     entity_types: vec![],
                     event_types: vec![],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:507:
             let next = {
                 let mut s = self.script.lock().unwrap();
                 if s.is_empty() {
[31m-                    MockResponse { error: InjectedError::None, event_count: 0, next_cursor: None }
(B[m[32m+                    MockResponse {
(B[m[32m+                        error: InjectedError::None,
(B[m[32m+                        event_count: 0,
(B[m[32m+                        next_cursor: None,
(B[m[32m+                    }
(B[m                 } else {
                     s.remove(0)
                 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:522:
                     let events = (0..next.event_count)
                         .map(|i| synthetic_event(&self.manifest.id, i))
                         .collect();
[31m-                    Ok(SyncOutcome { events, next_cursor: next.next_cursor, partial: false })
(B[m[32m+                    Ok(SyncOutcome {
(B[m[32m+                        events,
(B[m[32m+                        next_cursor: next.next_cursor,
(B[m[32m+                        partial: false,
(B[m[32m+                    })
(B[m                 }
             }
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:557:
     }
 
     fn err(kind: InjectedError) -> MockResponse {
[31m-        MockResponse { error: kind, event_count: 0, next_cursor: None }
(B[m[32m+        MockResponse {
(B[m[32m+            error: kind,
(B[m[32m+            event_count: 0,
(B[m[32m+            next_cursor: None,
(B[m[32m+        }
(B[m     }
 
     // Traces to: FR-CONN-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:565:
     async fn register_schedules_first_sync_at_now_plus_cadence() {
         let conn = MockConnector::new("c1", vec![]);
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("c1", conn, Duration::from_secs(60), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(60), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let h = orch.connector("c1").unwrap();
         assert_eq!(h.next_sync_at, t0() + ChronoDuration::seconds(60));
         assert_eq!(h.last_cursor, None);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:575:
     #[tokio::test]
     async fn register_rejects_duplicate_id() {
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("c1", MockConnector::new("c1", vec![]), Duration::from_secs(60), t0())
(B[m[31m-            .await
(B[m[31m-            .unwrap();
(B[m[32m+        orch.register(
(B[m[32m+            "c1",
(B[m[32m+            MockConnector::new("c1", vec![]),
(B[m[32m+            Duration::from_secs(60),
(B[m[32m+            t0(),
(B[m[32m+        )
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m         let dup = orch
[31m-            .register("c1", MockConnector::new("c1", vec![]), Duration::from_secs(60), t0())
(B[m[32m+            .register(
(B[m[32m+                "c1",
(B[m[32m+                MockConnector::new("c1", vec![]),
(B[m[32m+                Duration::from_secs(60),
(B[m[32m+                t0(),
(B[m[32m+            )
(B[m             .await;
         assert!(matches!(dup, Err(OrchestratorError::AlreadyRegistered(_))));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:590:
         let fast = MockConnector::new("fast", vec![ok(3, Some("A"))]);
         let slow = MockConnector::new("slow", vec![ok(5, Some("B"))]);
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("fast", fast.clone(), Duration::from_secs(10), t0()).await.unwrap();
(B[m[31m-        orch.register("slow", slow.clone(), Duration::from_secs(60), t0()).await.unwrap();
(B[m[32m+        orch.register("fast", fast.clone(), Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        orch.register("slow", slow.clone(), Duration::from_secs(60), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         // t=0 -- neither is due yet (both scheduled for now + cadence).
         let r0 = orch.tick(t0()).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:615:
     async fn cursor_is_passed_back_on_next_sync() {
         let conn = MockConnector::new("c1", vec![ok(1, Some("cursor-A")), ok(2, Some("cursor-B"))]);
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("c1", conn.clone(), Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn.clone(), Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         orch.tick(t0() + ChronoDuration::seconds(10)).await;
[31m-        assert_eq!(orch.connector("c1").unwrap().last_cursor.as_deref(), Some("cursor-A"));
(B[m[32m+        assert_eq!(
(B[m[32m+            orch.connector("c1").unwrap().last_cursor.as_deref(),
(B[m[32m+            Some("cursor-A")
(B[m[32m+        );
(B[m 
         orch.tick(t0() + ChronoDuration::seconds(30)).await;
         let calls = conn.calls();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:625:
         assert_eq!(calls.len(), 2);
         assert_eq!(calls[0], None);
         assert_eq!(calls[1].as_deref(), Some("cursor-A"));
[31m-        assert_eq!(orch.connector("c1").unwrap().last_cursor.as_deref(), Some("cursor-B"));
(B[m[32m+        assert_eq!(
(B[m[32m+            orch.connector("c1").unwrap().last_cursor.as_deref(),
(B[m[32m+            Some("cursor-B")
(B[m[32m+        );
(B[m     }
 
     // Traces to: FR-CONN-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:634:
         let bad = MockConnector::new("bad", vec![err(InjectedError::Auth)]);
         let good = MockConnector::new("good", vec![ok(2, Some("g"))]);
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("bad", bad.clone(), Duration::from_secs(10), t0()).await.unwrap();
(B[m[31m-        orch.register("good", good.clone(), Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("bad", bad.clone(), Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        orch.register("good", good.clone(), Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
         assert_eq!(r.connectors_synced, 1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:643:
         assert_eq!(r.errors.len(), 1);
         assert_eq!(r.errors[0].connector_id, "bad");
         assert!(matches!(r.errors[0].kind, SyncErrorKind::Auth));
[31m-        assert_eq!(orch.connector("bad").unwrap().health, HealthState::Unauthenticated);
(B[m[32m+        assert_eq!(
(B[m[32m+            orch.connector("bad").unwrap().health,
(B[m[32m+            HealthState::Unauthenticated
(B[m[32m+        );
(B[m 
         // Next tick: unauth connector must be skipped entirely.
         let r2 = orch.tick(t0() + ChronoDuration::seconds(60)).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:656:
     async fn rate_limited_pushes_next_sync_by_retry_after() {
         let conn = MockConnector::new("c1", vec![err(InjectedError::RateLimited(60))]);
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("c1", conn, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         let t_sync = t0() + ChronoDuration::seconds(10);
         let r = orch.tick(t_sync).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:663:
         assert_eq!(r.errors.len(), 1);
[31m-        assert!(matches!(r.errors[0].kind, SyncErrorKind::RateLimited { retry_after_s: 60 }));
(B[m[32m+        assert!(matches!(
(B[m[32m+            r.errors[0].kind,
(B[m[32m+            SyncErrorKind::RateLimited { retry_after_s: 60 }
(B[m[32m+        ));
(B[m         let next = orch.connector("c1").unwrap().next_sync_at;
         assert_eq!(next, t_sync + ChronoDuration::seconds(60));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:684:
             ],
         );
         let mut orch = SyncOrchestrator::new(policy);
[31m-        orch.register("c1", conn.clone(), Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn.clone(), Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         // Attempt 1 -> backoff 1s
         let mut now = t0() + ChronoDuration::seconds(10);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:709:
             "got {:?}",
             r3.errors[0].kind
         );
[31m-        assert_eq!(orch.connector("c1").unwrap().failed_attempts, 0, "exhausted resets attempts");
(B[m[31m-        assert!(matches!(orch.connector("c1").unwrap().health, HealthState::Failing(_)));
(B[m[32m+        assert_eq!(
(B[m[32m+            orch.connector("c1").unwrap().failed_attempts,
(B[m[32m+            0,
(B[m[32m+            "exhausted resets attempts"
(B[m[32m+        );
(B[m[32m+        assert!(matches!(
(B[m[32m+            orch.connector("c1").unwrap().health,
(B[m[32m+            HealthState::Failing(_)
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-CONN-003
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:724:
         };
         let conn = MockConnector::new("c1", vec![err(InjectedError::Generic), ok(1, Some("cur"))]);
         let mut orch = SyncOrchestrator::new(policy);
[31m-        orch.register("c1", conn, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         let mut now = t0() + ChronoDuration::seconds(10);
         orch.tick(now).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:749:
             jitter: false,
         };
         let mut orch = SyncOrchestrator::new(policy);
[31m-        orch.register("c1", conn, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
         assert_eq!(r.errors.len(), 1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:759:
     #[tokio::test]
     async fn unregister_removes_handle() {
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("c1", MockConnector::new("c1", vec![]), Duration::from_secs(10), t0())
(B[m[31m-            .await
(B[m[31m-            .unwrap();
(B[m[32m+        orch.register(
(B[m[32m+            "c1",
(B[m[32m+            MockConnector::new("c1", vec![]),
(B[m[32m+            Duration::from_secs(10),
(B[m[32m+            t0(),
(B[m[32m+        )
(B[m[32m+        .await
(B[m[32m+        .unwrap();
(B[m         assert_eq!(orch.len(), 1);
         orch.unregister("c1").unwrap();
         assert!(orch.is_empty());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:768:
[31m-        assert!(matches!(orch.unregister("c1"), Err(OrchestratorError::Unknown(_))));
(B[m[32m+        assert!(matches!(
(B[m[32m+            orch.unregister("c1"),
(B[m[32m+            Err(OrchestratorError::Unknown(_))
(B[m[32m+        ));
(B[m     }
 
     // Traces to: FR-EVT-002
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:774:
         let a = MockConnector::new("a", vec![ok(4, Some("a1"))]);
         let b = MockConnector::new("b", vec![ok(7, Some("b1"))]);
         let mut orch = SyncOrchestrator::with_default_retry();
[31m-        orch.register("a", a, Duration::from_secs(10), t0()).await.unwrap();
(B[m[31m-        orch.register("b", b, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("a", a, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m[32m+        orch.register("b", b, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
         assert_eq!(r.connectors_synced, 2);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:791:
         // Session 1: register, sync, observe cursor saved.
         let conn1 = MockConnector::new("c1", vec![ok(2, Some("saved-cursor"))]);
         let mut orch1 = SyncOrchestrator::with_cursor_store(RetryPolicy::default(), store.clone());
[31m-        orch1.register("c1", conn1, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch1
(B[m[32m+            .register("c1", conn1, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         orch1.tick(t0() + ChronoDuration::seconds(10)).await;
[31m-        assert_eq!(orch1.connector("c1").unwrap().last_cursor.as_deref(), Some("saved-cursor"));
(B[m         assert_eq!(
[31m-            store.load("c1", EVENTS_ENTITY_TYPE).await.unwrap().as_deref(),
(B[m[32m+            orch1.connector("c1").unwrap().last_cursor.as_deref(),
(B[m[32m+            Some("saved-cursor")
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            store
(B[m[32m+                .load("c1", EVENTS_ENTITY_TYPE)
(B[m[32m+                .await
(B[m[32m+                .unwrap()
(B[m[32m+                .as_deref(),
(B[m             Some("saved-cursor"),
         );
         drop(orch1);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:803:
         // Session 2: fresh orchestrator, same store, cursor must hydrate.
         let conn2 = MockConnector::new("c1", vec![ok(1, Some("cursor-after-restart"))]);
         let mut orch2 = SyncOrchestrator::with_cursor_store(RetryPolicy::default(), store.clone());
[31m-        orch2.register("c1", conn2.clone(), Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch2
(B[m[32m+            .register("c1", conn2.clone(), Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         assert_eq!(
             orch2.connector("c1").unwrap().last_cursor.as_deref(),
             Some("saved-cursor"),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:817:
         assert_eq!(calls[0].as_deref(), Some("saved-cursor"));
         // And the new cursor overwrites the old one in the store.
         assert_eq!(
[31m-            store.load("c1", EVENTS_ENTITY_TYPE).await.unwrap().as_deref(),
(B[m[32m+            store
(B[m[32m+                .load("c1", EVENTS_ENTITY_TYPE)
(B[m[32m+                .await
(B[m[32m+                .unwrap()
(B[m[32m+                .as_deref(),
(B[m             Some("cursor-after-restart"),
         );
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:828:
         use focus_events::dedup::InMemoryDeduplicator;
 
         // Same connector polled twice with identical event payload
[31m-        let conn = MockConnector::new("c1", vec![
(B[m[31m-            ok(1, Some("cursor-1")), // First sync: 1 event
(B[m[31m-            ok(1, Some("cursor-2")), // Second sync: same event (duplicate)
(B[m[31m-        ]);
(B[m[32m+        let conn = MockConnector::new(
(B[m[32m+            "c1",
(B[m[32m+            vec![
(B[m[32m+                ok(1, Some("cursor-1")), // First sync: 1 event
(B[m[32m+                ok(1, Some("cursor-2")), // Second sync: same event (duplicate)
(B[m[32m+            ],
(B[m[32m+        );
(B[m 
         let dedup = Arc::new(InMemoryDeduplicator::new());
[31m-        let mut orch = SyncOrchestrator::with_default_retry()
(B[m[31m-            .with_deduplicator(dedup);
(B[m[32m+        let mut orch = SyncOrchestrator::with_default_retry().with_deduplicator(dedup);
(B[m 
[31m-        orch.register("c1", conn, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         // First sync: 1 event appended, dedup records hash
         let r1 = orch.tick(t0() + ChronoDuration::seconds(10)).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:846:
         // Second sync: same event (same connector, same payload -> same hash)
         // But dedup should skip it
         let r2 = orch.tick(t0() + ChronoDuration::seconds(20)).await;
[31m-        assert_eq!(r2.events_pulled, 1, "second sync reports 1 event pulled from connector");
(B[m[32m+        assert_eq!(
(B[m[32m+            r2.events_pulled, 1,
(B[m[32m+            "second sync reports 1 event pulled from connector"
(B[m[32m+        );
(B[m         // But the dedup wrapper should have skipped it, so only 1 unique event persisted
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:856:
         use focus_events::dedup::InMemoryDeduplicator;
 
         // Simulate a connector that returns an event both via webhook and polling
[31m-        let conn = MockConnector::new("c1", vec![
(B[m[31m-            ok(1, Some("cursor-a")), // polling returns the event
(B[m[31m-        ]);
(B[m[32m+        let conn = MockConnector::new(
(B[m[32m+            "c1",
(B[m[32m+            vec![
(B[m[32m+                ok(1, Some("cursor-a")), // polling returns the event
(B[m[32m+            ],
(B[m[32m+        );
(B[m 
         let dedup = Arc::new(InMemoryDeduplicator::new());
[31m-        let mut orch = SyncOrchestrator::with_default_retry()
(B[m[31m-            .with_deduplicator(dedup.clone());
(B[m[32m+        let mut orch = SyncOrchestrator::with_default_retry().with_deduplicator(dedup.clone());
(B[m 
[31m-        orch.register("c1", conn, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m 
         // Simulate polling: gets 1 event
         let r1 = orch.tick(t0() + ChronoDuration::seconds(10)).await;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-sync/src/lib.rs:884:
         let conn = MockConnector::new("c1", vec![ok(2, Some("cursor-1"))]);
         let dedup1 = Arc::new(InMemoryDeduplicator::new());
 
[31m-        let mut orch = SyncOrchestrator::with_default_retry()
(B[m[31m-            .with_deduplicator(dedup1.clone());
(B[m[32m+        let mut orch = SyncOrchestrator::with_default_retry().with_deduplicator(dedup1.clone());
(B[m 
         // Confirm dedup is wired
[31m-        assert!(orch.deduplicator.is_some(), "dedup should be present after with_deduplicator");
(B[m[32m+        assert!(
(B[m[32m+            orch.deduplicator.is_some(),
(B[m[32m+            "dedup should be present after with_deduplicator"
(B[m[32m+        );
(B[m 
[31m-        orch.register("c1", conn, Duration::from_secs(10), t0()).await.unwrap();
(B[m[32m+        orch.register("c1", conn, Duration::from_secs(10), t0())
(B[m[32m+            .await
(B[m[32m+            .unwrap();
(B[m         let r = orch.tick(t0() + ChronoDuration::seconds(10)).await;
         assert_eq!(r.events_pulled, 2);
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/audit.rs:58:
 
     /// Clear old audit records (older than retention_days).
     pub fn cleanup_old(conn: &rusqlite::Connection, retention_days: i32) -> Result<()> {
[31m-        let cutoff = format!(
(B[m[31m-            "datetime('now', '-{} days')",
(B[m[31m-            retention_days
(B[m[31m-        );
(B[m[32m+        let cutoff = format!("datetime('now', '-{} days')", retention_days);
(B[m         conn.execute(
             &format!("DELETE FROM telemetry_audit WHERE flushed_at < {}", cutoff),
             [],
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/lib.rs:318:
         .unwrap();
 
         let props = json!({"feature": "connector.connected"});
[31m-        client
(B[m[31m-            .track("connector.connected", props.clone())
(B[m[31m-            .unwrap();
(B[m[32m+        client.track("connector.connected", props.clone()).unwrap();
(B[m 
         // Verify event is buffered
         let count = client.buffered_event_count().unwrap();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/lib.rs:431:
         // Verify audit table exists and is empty before flush
         let conn = rusqlite::Connection::open(db_file.path()).unwrap();
         let audit_count: usize = conn
[31m-            .query_row(
(B[m[31m-                "SELECT COUNT(*) FROM telemetry_audit",
(B[m[31m-                [],
(B[m[31m-                |row| row.get(0),
(B[m[31m-            )
(B[m[32m+            .query_row("SELECT COUNT(*) FROM telemetry_audit", [], |row| row.get(0))
(B[m             .unwrap_or(0);
 
         assert_eq!(audit_count, 0);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/pii_scrubber.rs:50:
         let mut result = input.to_string();
 
         // Order matters: scrub more specific/longer patterns first to avoid partial matches
[31m-        result = self.uuid_regex.replace_all(&result, "[REDACTED_UUID]").to_string();
(B[m[31m-        result = self.email_regex.replace_all(&result, "[REDACTED_EMAIL]").to_string();
(B[m[31m-        result = self.token_regex.replace_all(&result, "[REDACTED_TOKEN]").to_string();
(B[m[31m-        result = self.phone_regex.replace_all(&result, "[REDACTED_PHONE]").to_string();
(B[m         result = self
[32m+            .uuid_regex
(B[m[32m+            .replace_all(&result, "[REDACTED_UUID]")
(B[m[32m+            .to_string();
(B[m[32m+        result = self
(B[m[32m+            .email_regex
(B[m[32m+            .replace_all(&result, "[REDACTED_EMAIL]")
(B[m[32m+            .to_string();
(B[m[32m+        result = self
(B[m[32m+            .token_regex
(B[m[32m+            .replace_all(&result, "[REDACTED_TOKEN]")
(B[m[32m+            .to_string();
(B[m[32m+        result = self
(B[m[32m+            .phone_regex
(B[m[32m+            .replace_all(&result, "[REDACTED_PHONE]")
(B[m[32m+            .to_string();
(B[m[32m+        result = self
(B[m             .healthkit_regex
             .replace_all(&result, "[REDACTED_HEALTHKIT]")
             .to_string();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/pii_scrubber.rs:72:
 /// Email pattern: user@domain.com
 fn email_pattern() -> &'static Regex {
     static REGEX: OnceLock<Regex> = OnceLock::new();
[31m-    REGEX.get_or_init(|| {
(B[m[31m-        Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap()
(B[m[31m-    })
(B[m[32m+    REGEX.get_or_init(|| Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap())
(B[m }
 
 /// Phone pattern: (555) 555-0123, +1-555-0124, etc.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/pii_scrubber.rs:82:
     static REGEX: OnceLock<Regex> = OnceLock::new();
     REGEX.get_or_init(|| {
         // Match phone patterns like (555) 555-0123, 555-555-0123, +1-555-555-0123
[31m-        Regex::new(
(B[m[31m-            r"(?:\+\d{1,3})?[-.\s]?\(?(?:0\d{1}|[1-9]\d{0,2})\)?[-.\s]?\d{3,4}[-.\s]?\d{4}",
(B[m[31m-        )
(B[m[31m-        .unwrap()
(B[m[32m+        Regex::new(r"(?:\+\d{1,3})?[-.\s]?\(?(?:0\d{1}|[1-9]\d{0,2})\)?[-.\s]?\d{3,4}[-.\s]?\d{4}")
(B[m[32m+            .unwrap()
(B[m     })
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/pii_scrubber.rs:101:
 fn uuid_pattern() -> &'static Regex {
     static REGEX: OnceLock<Regex> = OnceLock::new();
     REGEX.get_or_init(|| {
[31m-        Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
(B[m[31m-            .unwrap()
(B[m[32m+        Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}").unwrap()
(B[m     })
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/pii_scrubber.rs:172:
             Some("[REDACTED_EMAIL]")
         );
         assert_eq!(
[31m-            output.get("nested")
(B[m[32m+            output
(B[m[32m+                .get("nested")
(B[m                 .and_then(|v| v.get("phone"))
                 .and_then(|v| v.as_str()),
             Some("[REDACTED_PHONE]")
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-telemetry/src/pii_scrubber.rs:186:
     #[test]
     fn test_scrub_multiple_patterns_in_one_string() {
         let scrubber = PiiScrubber::new();
[31m-        let input = "Email alice@example.com, phone (555) 555-0123, token Bearer sk_live_abc123def456";
(B[m[32m+        let input =
(B[m[32m+            "Email alice@example.com, phone (555) 555-0123, token Bearer sk_live_abc123def456";
(B[m         let output = scrubber.scrub_string(input);
 
         assert!(output.contains("[REDACTED_EMAIL]"));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:28:
 pub mod signing;
 
 // Re-export signing types for convenience
[31m-pub use signing::{verify_pack, verify_pack_bytes, parse_root_pubkey, PHENOTYPE_ROOT_PUBKEYS};
(B[m[32m+pub use signing::{parse_root_pubkey, verify_pack, verify_pack_bytes, PHENOTYPE_ROOT_PUBKEYS};
(B[m 
 /// Error surface for template-pack operations.
 #[derive(Debug, Error)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:159:
 
 impl From<ConditionDraft> for Condition {
     fn from(c: ConditionDraft) -> Self {
[31m-        Condition { kind: c.kind, params: c.params }
(B[m[32m+        Condition {
(B[m[32m+            kind: c.kind,
(B[m[32m+            params: c.params,
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:170:
 #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
 #[serde(tag = "type", rename_all = "snake_case")]
 pub enum ActionDraft {
[31m-    GrantCredit { amount: i32 },
(B[m[31m-    DeductCredit { amount: i32 },
(B[m[32m+    GrantCredit {
(B[m[32m+        amount: i32,
(B[m[32m+    },
(B[m[32m+    DeductCredit {
(B[m[32m+        amount: i32,
(B[m[32m+    },
(B[m     Block {
         profile: String,
         duration_seconds: i64,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:178:
         #[serde(default = "default_rigidity")]
         rigidity: RigidityDraft,
     },
[31m-    Unblock { profile: String },
(B[m[31m-    StreakIncrement { name: String },
(B[m[31m-    StreakReset { name: String },
(B[m[31m-    Notify { message: String },
(B[m[32m+    Unblock {
(B[m[32m+        profile: String,
(B[m[32m+    },
(B[m[32m+    StreakIncrement {
(B[m[32m+        name: String,
(B[m[32m+    },
(B[m[32m+    StreakReset {
(B[m[32m+        name: String,
(B[m[32m+    },
(B[m[32m+    Notify {
(B[m[32m+        message: String,
(B[m[32m+    },
(B[m }
 
 #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:210:
         match a {
             ActionDraft::GrantCredit { amount } => Action::GrantCredit { amount },
             ActionDraft::DeductCredit { amount } => Action::DeductCredit { amount },
[31m-            ActionDraft::Block { profile, duration_seconds, rigidity } => Action::Block {
(B[m[32m+            ActionDraft::Block {
(B[m                 profile,
[32m+                duration_seconds,
(B[m[32m+                rigidity,
(B[m[32m+            } => Action::Block {
(B[m[32m+                profile,
(B[m                 duration: Duration::seconds(duration_seconds),
                 rigidity: rigidity.into(),
             },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:346:
             }
 
             if !verified {
[31m-                return Err(TemplateError::Verify("no trusted key verified the signature".into()));
(B[m[32m+                return Err(TemplateError::Verify(
(B[m[32m+                    "no trusted key verified the signature".into(),
(B[m[32m+                ));
(B[m             }
         } else if require_signature {
[31m-            return Err(TemplateError::Verify("pack requires signature but none present".into()));
(B[m[32m+            return Err(TemplateError::Verify(
(B[m[32m+                "pack requires signature but none present".into(),
(B[m[32m+            ));
(B[m         }
 
         // Signature verified; apply rules.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:504:
     #[test]
     fn apply_propagates_store_error() {
         let pack = TemplatePack::from_toml_str(SAMPLE_TOML).expect("parse");
[31m-        let mut store = MemStore { fail_at: Some(0), ..Default::default() };
(B[m[32m+        let mut store = MemStore {
(B[m[32m+            fail_at: Some(0),
(B[m[32m+            ..Default::default()
(B[m[32m+        };
(B[m         let err = pack.apply(&mut store).unwrap_err();
         match err {
             TemplateError::Apply(msg) => assert!(msg.contains("boom")),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:543:
 
     #[test]
     fn verify_and_apply_checks_sha256() {
[31m-        
(B[m[31m-        
(B[m[31m-
(B[m         let pack = TemplatePack::from_toml_str(SAMPLE_TOML).expect("parse");
         let digest = signing::digest_pack(&pack).unwrap();
         let mut manifest = TemplatePackManifest {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:552:
             id: pack.id.clone(),
             version: pack.version.clone(),
             author: pack.author.clone(),
[31m-            sha256: "badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadb"
(B[m[31m-                .into(),
(B[m[32m+            sha256: "badbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadbadb".into(),
(B[m             signature: None,
             signed_by: None,
         };
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:560:
         let mut store = MemStore::default();
[31m-        let err = pack.verify_and_apply(&mut store, &manifest, &[], false).unwrap_err();
(B[m[32m+        let err = pack
(B[m[32m+            .verify_and_apply(&mut store, &manifest, &[], false)
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, TemplateError::Verify(_)));
 
         // Correct digest should allow apply
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:565:
         manifest.sha256 = digest;
[31m-        let n = pack.verify_and_apply(&mut store, &manifest, &[], false).unwrap();
(B[m[32m+        let n = pack
(B[m[32m+            .verify_and_apply(&mut store, &manifest, &[], false)
(B[m[32m+            .unwrap();
(B[m         assert_eq!(n, 1);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:580:
             signed_by: None,
         };
         let mut store = MemStore::default();
[31m-        let err = pack.verify_and_apply(&mut store, &manifest, &[], true).unwrap_err();
(B[m[32m+        let err = pack
(B[m[32m+            .verify_and_apply(&mut store, &manifest, &[], true)
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, TemplateError::Verify(_)));
         assert!(err.to_string().contains("requires signature"));
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:609:
         };
 
         let mut store = MemStore::default();
[31m-        let n = pack.verify_and_apply(&mut store, &manifest, &[pubkey_hex], false).unwrap();
(B[m[32m+        let n = pack
(B[m[32m+            .verify_and_apply(&mut store, &manifest, &[pubkey_hex], false)
(B[m[32m+            .unwrap();
(B[m         assert_eq!(n, 1);
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/lib.rs:638:
         };
 
         let mut store = MemStore::default();
[31m-        let err = pack.verify_and_apply(&mut store, &manifest, &[pubkey2_hex], false).unwrap_err();
(B[m[32m+        let err = pack
(B[m[32m+            .verify_and_apply(&mut store, &manifest, &[pubkey2_hex], false)
(B[m[32m+            .unwrap_err();
(B[m         assert!(matches!(err, TemplateError::Verify(_)));
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/src/signing.rs:117:
 /// host app to iterate [`PHENOTYPE_ROOT_PUBKEYS`] at startup.
 pub fn parse_root_pubkey(hex: &str) -> Result<VerifyingKey, TemplateError> {
     if hex.len() != 64 {
[31m-        return Err(TemplateError::Signature(format!("expected 64 hex chars, got {}", hex.len())));
(B[m[32m+        return Err(TemplateError::Signature(format!(
(B[m[32m+            "expected 64 hex chars, got {}",
(B[m[32m+            hex.len()
(B[m[32m+        )));
(B[m     }
     let mut raw = [0u8; 32];
     for i in 0..32 {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/tests/examples_smoke.rs:14:
 
 impl MemStore {
     fn new() -> Self {
[31m-        Self { by_id: HashMap::new() }
(B[m[32m+        Self {
(B[m[32m+            by_id: HashMap::new(),
(B[m[32m+        }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/tests/examples_smoke.rs:51:
             .unwrap_or_else(|e| panic!("parse {}: {e:?}", path.display()));
         count += 1;
     }
[31m-    assert!(count >= 4, "expected at least 4 starter packs; found {count}");
(B[m[32m+    assert!(
(B[m[32m+        count >= 4,
(B[m[32m+        "expected at least 4 starter packs; found {count}"
(B[m[32m+    );
(B[m }
 
 #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-templates/tests/examples_smoke.rs:67:
         let mut store = MemStore::new();
         let n1 = pack.apply(&mut store).expect("first apply");
         let n2 = pack.apply(&mut store).expect("second apply (idempotent)");
[31m-        assert_eq!(n1, n2, "{} upserts differ between first and second apply", path.display());
(B[m[32m+        assert_eq!(
(B[m[32m+            n1,
(B[m[32m+            n2,
(B[m[32m+            "{} upserts differ between first and second apply",
(B[m[32m+            path.display()
(B[m[32m+        );
(B[m         assert_eq!(
             store.by_id.len(),
             n1,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-time/src/lib.rs:20:
 
 impl TestClock {
     pub fn new(initial: DateTime<Utc>) -> Self {
[31m-        Self { fixed: std::sync::Mutex::new(initial) }
(B[m[32m+        Self {
(B[m[32m+            fixed: std::sync::Mutex::new(initial),
(B[m[32m+        }
(B[m     }
 
     pub fn advance(&self, by: chrono::Duration) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:51:
             id: Uuid::parse_str(&rule_ir.id).map_err(|_| anyhow!("Invalid rule ID UUID"))?,
             name: rule_ir.name.clone(),
             trigger: ir_to_trigger(&rule_ir.trigger)?,
[31m-            conditions: rule_ir.conditions.iter().map(ir_to_condition).collect::<Result<_, _>>()?,
(B[m[31m-            actions: rule_ir.actions.iter().map(ir_to_action).collect::<Result<_, _>>()?,
(B[m[32m+            conditions: rule_ir
(B[m[32m+                .conditions
(B[m[32m+                .iter()
(B[m[32m+                .map(ir_to_condition)
(B[m[32m+                .collect::<Result<_, _>>()?,
(B[m[32m+            actions: rule_ir
(B[m[32m+                .actions
(B[m[32m+                .iter()
(B[m[32m+                .map(ir_to_action)
(B[m[32m+                .collect::<Result<_, _>>()?,
(B[m             priority: rule_ir.priority,
             cooldown: rule_ir.cooldown_seconds.map(Duration::seconds),
             duration: rule_ir.duration_seconds.map(Duration::seconds),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:72:
 
 fn trigger_to_ir(trigger: &Trigger) -> TriggerIr {
     match trigger {
[31m-        Trigger::Event(name) => TriggerIr::EventFired { event_name: name.clone() },
(B[m[31m-        Trigger::Schedule(cron) => {
(B[m[31m-            TriggerIr::ScheduleCron { cron_expression: cron.clone(), timezone: "UTC".into() }
(B[m[31m-        }
(B[m[31m-        Trigger::StateChange(state) => {
(B[m[31m-            TriggerIr::UserAction { action_type: "state_change".into(), target: state.clone() }
(B[m[31m-        }
(B[m[32m+        Trigger::Event(name) => TriggerIr::EventFired {
(B[m[32m+            event_name: name.clone(),
(B[m[32m+        },
(B[m[32m+        Trigger::Schedule(cron) => TriggerIr::ScheduleCron {
(B[m[32m+            cron_expression: cron.clone(),
(B[m[32m+            timezone: "UTC".into(),
(B[m[32m+        },
(B[m[32m+        Trigger::StateChange(state) => TriggerIr::UserAction {
(B[m[32m+            action_type: "state_change".into(),
(B[m[32m+            target: state.clone(),
(B[m[32m+        },
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:85:
 fn ir_to_trigger(trigger: &TriggerIr) -> Result<Trigger> {
     match trigger {
         TriggerIr::EventFired { event_name } => Ok(Trigger::Event(event_name.clone())),
[31m-        TriggerIr::ScheduleCron { cron_expression, .. } => {
(B[m[31m-            Ok(Trigger::Schedule(cron_expression.clone()))
(B[m[31m-        }
(B[m[31m-        TriggerIr::UserAction { action_type, target } if action_type == "state_change" => {
(B[m[31m-            Ok(Trigger::StateChange(target.clone()))
(B[m[31m-        }
(B[m[32m+        TriggerIr::ScheduleCron {
(B[m[32m+            cron_expression, ..
(B[m[32m+        } => Ok(Trigger::Schedule(cron_expression.clone())),
(B[m[32m+        TriggerIr::UserAction {
(B[m[32m+            action_type,
(B[m[32m+            target,
(B[m[32m+        } if action_type == "state_change" => Ok(Trigger::StateChange(target.clone())),
(B[m         _ => Err(anyhow!("Unsupported trigger type")),
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:97:
 
 fn condition_to_ir(condition: &Condition) -> ConditionIr {
[31m-    ConditionIr::CustomPredicate { name: condition.kind.clone(), args: condition.params.clone() }
(B[m[32m+    ConditionIr::CustomPredicate {
(B[m[32m+        name: condition.kind.clone(),
(B[m[32m+        args: condition.params.clone(),
(B[m[32m+    }
(B[m }
 
 fn ir_to_condition(ir: &ConditionIr) -> Result<Condition> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:103:
     match ir {
[31m-        ConditionIr::CustomPredicate { name, args } => {
(B[m[31m-            Ok(Condition { kind: name.clone(), params: args.clone() })
(B[m[31m-        }
(B[m[31m-        _ => Err(anyhow!("Complex conditions not yet supported in round-trip")),
(B[m[32m+        ConditionIr::CustomPredicate { name, args } => Ok(Condition {
(B[m[32m+            kind: name.clone(),
(B[m[32m+            params: args.clone(),
(B[m[32m+        }),
(B[m[32m+        _ => Err(anyhow!(
(B[m[32m+            "Complex conditions not yet supported in round-trip"
(B[m[32m+        )),
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:126:
                 m
             },
         },
[31m-        Action::Block { profile, duration, rigidity } => ActionIr::EnforcePolicy {
(B[m[32m+        Action::Block {
(B[m[32m+            profile,
(B[m[32m+            duration,
(B[m[32m+            rigidity,
(B[m[32m+        } => ActionIr::EnforcePolicy {
(B[m             policy_id: "block".into(),
             params: {
                 let mut m = BTreeMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:133:
                 m.insert("profile".into(), serde_json::json!(profile));
[31m-                m.insert("duration_secs".into(), serde_json::json!(duration.num_seconds()));
(B[m[31m-                m.insert("rigidity".into(), serde_json::json!(format!("{:?}", rigidity)));
(B[m[32m+                m.insert(
(B[m[32m+                    "duration_secs".into(),
(B[m[32m+                    serde_json::json!(duration.num_seconds()),
(B[m[32m+                );
(B[m[32m+                m.insert(
(B[m[32m+                    "rigidity".into(),
(B[m[32m+                    serde_json::json!(format!("{:?}", rigidity)),
(B[m[32m+                );
(B[m                 m
             },
         },
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:165:
             text: msg.clone(),
             duration_ms: None,
         },
[31m-        Action::EmergencyExit { profiles, duration, bypass_cost, reason } => {
(B[m[31m-            ActionIr::EnforcePolicy {
(B[m[31m-                policy_id: "emergency_exit".into(),
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert(
(B[m[31m-                        "profiles".into(),
(B[m[31m-                        serde_json::json!(profiles.iter().collect::<Vec<_>>()),
(B[m[31m-                    );
(B[m[31m-                    m.insert("duration_secs".into(), serde_json::json!(duration.num_seconds()));
(B[m[31m-                    m.insert("bypass_cost".into(), serde_json::json!(bypass_cost));
(B[m[31m-                    m.insert("reason".into(), serde_json::json!(reason));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            }
(B[m[31m-        }
(B[m[31m-        Action::Intervention { message, severity: _ } => ActionIr::ShowNotification {
(B[m[32m+        Action::EmergencyExit {
(B[m[32m+            profiles,
(B[m[32m+            duration,
(B[m[32m+            bypass_cost,
(B[m[32m+            reason,
(B[m[32m+        } => ActionIr::EnforcePolicy {
(B[m[32m+            policy_id: "emergency_exit".into(),
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert(
(B[m[32m+                    "profiles".into(),
(B[m[32m+                    serde_json::json!(profiles.iter().collect::<Vec<_>>()),
(B[m[32m+                );
(B[m[32m+                m.insert(
(B[m[32m+                    "duration_secs".into(),
(B[m[32m+                    serde_json::json!(duration.num_seconds()),
(B[m[32m+                );
(B[m[32m+                m.insert("bypass_cost".into(), serde_json::json!(bypass_cost));
(B[m[32m+                m.insert("reason".into(), serde_json::json!(reason));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        },
(B[m[32m+        Action::Intervention {
(B[m[32m+            message,
(B[m[32m+            severity: _,
(B[m[32m+        } => ActionIr::ShowNotification {
(B[m             notification_id: Uuid::new_v4().to_string(),
             text: message.clone(),
             duration_ms: Some(5000),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:188:
         },
[31m-        Action::ScheduledUnlockWindow { profile, starts_at, ends_at, credit_cost } => {
(B[m[31m-            ActionIr::ScheduleTask {
(B[m[31m-                task_id: "unlock_window".into(),
(B[m[31m-                delay_ms: None,
(B[m[31m-                params: {
(B[m[31m-                    let mut m = BTreeMap::new();
(B[m[31m-                    m.insert("profile".into(), serde_json::json!(profile));
(B[m[31m-                    m.insert("starts_at".into(), serde_json::json!(starts_at.to_rfc3339()));
(B[m[31m-                    m.insert("ends_at".into(), serde_json::json!(ends_at.to_rfc3339()));
(B[m[31m-                    m.insert("credit_cost".into(), serde_json::json!(credit_cost));
(B[m[31m-                    m
(B[m[31m-                },
(B[m[31m-            }
(B[m[31m-        }
(B[m[32m+        Action::ScheduledUnlockWindow {
(B[m[32m+            profile,
(B[m[32m+            starts_at,
(B[m[32m+            ends_at,
(B[m[32m+            credit_cost,
(B[m[32m+        } => ActionIr::ScheduleTask {
(B[m[32m+            task_id: "unlock_window".into(),
(B[m[32m+            delay_ms: None,
(B[m[32m+            params: {
(B[m[32m+                let mut m = BTreeMap::new();
(B[m[32m+                m.insert("profile".into(), serde_json::json!(profile));
(B[m[32m+                m.insert(
(B[m[32m+                    "starts_at".into(),
(B[m[32m+                    serde_json::json!(starts_at.to_rfc3339()),
(B[m[32m+                );
(B[m[32m+                m.insert("ends_at".into(), serde_json::json!(ends_at.to_rfc3339()));
(B[m[32m+                m.insert("credit_cost".into(), serde_json::json!(credit_cost));
(B[m[32m+                m
(B[m[32m+            },
(B[m[32m+        },
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:206:
 fn ir_to_action(ir: &ActionIr) -> Result<Action> {
     match ir {
[31m-        ActionIr::EmitEvent { event_type, payload } => match event_type.as_str() {
(B[m[32m+        ActionIr::EmitEvent {
(B[m[32m+            event_type,
(B[m[32m+            payload,
(B[m[32m+        } => match event_type.as_str() {
(B[m             "grant_credit" => {
                 let amount = payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                 Ok(Action::GrantCredit { amount })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:215:
                 Ok(Action::DeductCredit { amount })
             }
             "streak_increment" => {
[31m-                let name =
(B[m[31m-                    payload.get("streak_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
(B[m[32m+                let name = payload
(B[m[32m+                    .get("streak_name")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("")
(B[m[32m+                    .to_string();
(B[m                 Ok(Action::StreakIncrement(name))
             }
             "streak_reset" => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:223:
[31m-                let name =
(B[m[31m-                    payload.get("streak_name").and_then(|v| v.as_str()).unwrap_or("").to_string();
(B[m[32m+                let name = payload
(B[m[32m+                    .get("streak_name")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("")
(B[m[32m+                    .to_string();
(B[m                 Ok(Action::StreakReset(name))
             }
             _ => Err(anyhow!("Unknown event type: {}", event_type)),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:228:
         },
         ActionIr::EnforcePolicy { policy_id, params } => match policy_id.as_str() {
             "block" => {
[31m-                let profile =
(B[m[31m-                    params.get("profile").and_then(|v| v.as_str()).unwrap_or("").to_string();
(B[m[31m-                let duration_secs =
(B[m[31m-                    params.get("duration_secs").and_then(|v| v.as_i64()).unwrap_or(0);
(B[m[32m+                let profile = params
(B[m[32m+                    .get("profile")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("")
(B[m[32m+                    .to_string();
(B[m[32m+                let duration_secs = params
(B[m[32m+                    .get("duration_secs")
(B[m[32m+                    .and_then(|v| v.as_i64())
(B[m[32m+                    .unwrap_or(0);
(B[m                 Ok(Action::Block {
                     profile,
                     duration: Duration::seconds(duration_secs),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:239:
                 })
             }
             "unblock" => {
[31m-                let profile =
(B[m[31m-                    params.get("profile").and_then(|v| v.as_str()).unwrap_or("").to_string();
(B[m[32m+                let profile = params
(B[m[32m+                    .get("profile")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("")
(B[m[32m+                    .to_string();
(B[m                 Ok(Action::Unblock { profile })
             }
             _ => Err(anyhow!("Unsupported policy: {}", policy_id)),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:247:
         },
         ActionIr::ShowNotification { text, .. } => Ok(Action::Notify(text.clone())),
[31m-        _ => Err(anyhow!("Action type not yet supported in IR->Rule conversion")),
(B[m[32m+        _ => Err(anyhow!(
(B[m[32m+            "Action type not yet supported in IR->Rule conversion"
(B[m[32m+        )),
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/focus_rules_transpiler.rs:346:
         let restored = document_to_rule(&doc).expect("From doc");
 
         match &restored.actions[..] {
[31m-            [Action::Block { profile, duration, rigidity: _ }] => {
(B[m[32m+            [Action::Block {
(B[m[32m+                profile,
(B[m[32m+                duration,
(B[m[32m+                rigidity: _,
(B[m[32m+            }] => {
(B[m                 assert_eq!(profile, "social");
                 assert_eq!(duration.num_seconds(), 1800);
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:92:
             .map_err(|e| anyhow!("Invalid trigger data: {}", e))?;
 
         // Extract condition nodes (topologically ordered)
[31m-        let mut condition_nodes: Vec<_> =
(B[m[31m-            graph.nodes.iter().filter(|n| n.node_type == "condition").collect();
(B[m[32m+        let mut condition_nodes: Vec<_> = graph
(B[m[32m+            .nodes
(B[m[32m+            .iter()
(B[m[32m+            .filter(|n| n.node_type == "condition")
(B[m[32m+            .collect();
(B[m         condition_nodes.sort_by_key(|n| n.id.clone());
 
         let conditions: Vec<ConditionIr> = condition_nodes
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:105:
             .collect::<Result<Vec<_>>>()?;
 
         // Extract action nodes
[31m-        let mut action_nodes: Vec<_> = graph.nodes.iter().filter(|n| n.node_type == "action").collect();
(B[m[32m+        let mut action_nodes: Vec<_> = graph
(B[m[32m+            .nodes
(B[m[32m+            .iter()
(B[m[32m+            .filter(|n| n.node_type == "action")
(B[m[32m+            .collect();
(B[m         action_nodes.sort_by_key(|n| n.id.clone());
 
         let actions: Vec<ActionIr> = action_nodes
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:150:
             let cond_node = GraphNode {
                 id: node_id.clone(),
                 node_type: "condition".to_string(),
[31m-                position: XYPosition { x: 0.0, y: 100.0 * (i as f64 + 1.0) },
(B[m[32m+                position: XYPosition {
(B[m[32m+                    x: 0.0,
(B[m[32m+                    y: 100.0 * (i as f64 + 1.0),
(B[m[32m+                },
(B[m                 data: serde_json::to_value(condition)
                     .map_err(|e| anyhow!("Failed to serialize condition: {}", e))?,
             };
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:157:
             nodes.push(cond_node);
 
             // Edge from trigger or previous condition
[31m-            let source =
(B[m[31m-                if i == 0 { "trigger-0".to_string() } else { format!("condition-{}", i - 1) };
(B[m[32m+            let source = if i == 0 {
(B[m[32m+                "trigger-0".to_string()
(B[m[32m+            } else {
(B[m[32m+                format!("condition-{}", i - 1)
(B[m[32m+            };
(B[m 
             edges.push(GraphEdge {
                 id: format!("{}-{}", source, node_id),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:175:
             let action_node = GraphNode {
                 id: node_id.clone(),
                 node_type: "action".to_string(),
[31m-                position: XYPosition { x: 200.0, y: action_start_y + 100.0 * i as f64 },
(B[m[32m+                position: XYPosition {
(B[m[32m+                    x: 200.0,
(B[m[32m+                    y: action_start_y + 100.0 * i as f64,
(B[m[32m+                },
(B[m                 data: serde_json::to_value(action)
                     .map_err(|e| anyhow!("Failed to serialize action: {}", e))?,
             };
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:220:
 
     #[test]
     fn test_graph_to_document_minimal() {
[31m-        let trigger = TriggerIr::EventFired { event_name: "test_event".to_string() };
(B[m[32m+        let trigger = TriggerIr::EventFired {
(B[m[32m+            event_name: "test_event".to_string(),
(B[m[32m+        };
(B[m 
         let graph = GraphJson {
             id: "graph-1".to_string(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:244:
         let rule_ir = RuleIr {
             id: "r1".to_string(),
             name: "Complex".to_string(),
[31m-            trigger: TriggerIr::EventFired { event_name: "evt".to_string() },
(B[m[32m+            trigger: TriggerIr::EventFired {
(B[m[32m+                event_name: "evt".to_string(),
(B[m[32m+            },
(B[m             conditions: vec![
[31m-                ConditionIr::TimeInRange { start_hour: 8, end_hour: 17 },
(B[m[31m-                ConditionIr::DayOfWeek { days: vec!["Monday".to_string()] },
(B[m[32m+                ConditionIr::TimeInRange {
(B[m[32m+                    start_hour: 8,
(B[m[32m+                    end_hour: 17,
(B[m[32m+                },
(B[m[32m+                ConditionIr::DayOfWeek {
(B[m[32m+                    days: vec!["Monday".to_string()],
(B[m[32m+                },
(B[m             ],
             actions: vec![ActionIr::EnforcePolicy {
                 policy_id: "block".to_string(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/graph_transpiler.rs:282:
         let original_rule = RuleIr {
             id: "rt-rule".to_string(),
             name: "Round Trip".to_string(),
[31m-            trigger: TriggerIr::EventFired { event_name: "rt_evt".to_string() },
(B[m[32m+            trigger: TriggerIr::EventFired {
(B[m[32m+                event_name: "rt_evt".to_string(),
(B[m[32m+            },
(B[m             conditions: vec![],
             actions: vec![],
             priority: 1,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/lib.rs:26:
 pub mod wallet_mutation_transpiler;
 
 use anyhow::{anyhow, Result};
[31m-use focus_ir::{Body, DocKind, RuleIr};
(B[m pub use focus_ir::Document;
[32m+use focus_ir::{Body, DocKind, RuleIr};
(B[m 
 /// Trait for transpilers that convert a single domain type to/from a Rule IR Document.
 ///
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/lib.rs:153:
         }
         TargetFormat::Graph => {
             if docs.len() != 1 {
[31m-                return Err(anyhow!("Graph format expects exactly 1 document, got {}", docs.len()));
(B[m[32m+                return Err(anyhow!(
(B[m[32m+                    "Graph format expects exactly 1 document, got {}",
(B[m[32m+                    docs.len()
(B[m[32m+                ));
(B[m             }
             let graph = graph_transpiler::document_to_graph(&docs[0])?;
             serde_json::to_vec(&graph).map_err(|e| anyhow!("Failed to serialize graph: {}", e))?
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:87:
 impl RuleTranspiler<RuleDraft> for RuleDraftTranspiler {
     fn domain_to_ir(draft: &RuleDraft) -> Result<RuleIr> {
         let trigger = draft_trigger_to_ir(&draft.trigger)?;
[31m-        let conditions =
(B[m[31m-            draft.conditions.iter().map(draft_condition_to_ir).collect::<Result<Vec<_>>>()?;
(B[m[31m-        let actions =
(B[m[31m-            draft.actions.iter().map(draft_action_to_ir).collect::<Result<Vec<_>>>()?;
(B[m[32m+        let conditions = draft
(B[m[32m+            .conditions
(B[m[32m+            .iter()
(B[m[32m+            .map(draft_condition_to_ir)
(B[m[32m+            .collect::<Result<Vec<_>>>()?;
(B[m[32m+        let actions = draft
(B[m[32m+            .actions
(B[m[32m+            .iter()
(B[m[32m+            .map(draft_action_to_ir)
(B[m[32m+            .collect::<Result<Vec<_>>>()?;
(B[m 
         Ok(RuleIr {
             id: draft.id.clone(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:108:
 
     fn ir_to_domain(rule_ir: &RuleIr) -> Result<RuleDraft> {
         let trigger = ir_trigger_to_draft(&rule_ir.trigger)?;
[31m-        let conditions =
(B[m[31m-            rule_ir.conditions.iter().map(ir_condition_to_draft).collect::<Result<Vec<_>>>()?;
(B[m[31m-        let actions =
(B[m[31m-            rule_ir.actions.iter().map(ir_action_to_draft).collect::<Result<Vec<_>>>()?;
(B[m[32m+        let conditions = rule_ir
(B[m[32m+            .conditions
(B[m[32m+            .iter()
(B[m[32m+            .map(ir_condition_to_draft)
(B[m[32m+            .collect::<Result<Vec<_>>>()?;
(B[m[32m+        let actions = rule_ir
(B[m[32m+            .actions
(B[m[32m+            .iter()
(B[m[32m+            .map(ir_action_to_draft)
(B[m[32m+            .collect::<Result<Vec<_>>>()?;
(B[m 
         Ok(RuleDraft {
             id: rule_ir.id.clone(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:138:
 
 fn draft_trigger_to_ir(trigger: &TriggerDraft) -> Result<TriggerIr> {
     match trigger {
[31m-        TriggerDraft::Event(value) => Ok(TriggerIr::EventFired { event_name: value.clone() }),
(B[m[32m+        TriggerDraft::Event(value) => Ok(TriggerIr::EventFired {
(B[m[32m+            event_name: value.clone(),
(B[m[32m+        }),
(B[m         TriggerDraft::Schedule(cron_expression) => Ok(TriggerIr::ScheduleCron {
             cron_expression: cron_expression.clone(),
             timezone: "UTC".to_string(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:153:
 fn ir_trigger_to_draft(trigger: &TriggerIr) -> Result<TriggerDraft> {
     match trigger {
         TriggerIr::EventFired { event_name } => Ok(TriggerDraft::Event(event_name.clone())),
[31m-        TriggerIr::ScheduleCron { cron_expression, .. } => {
(B[m[31m-            Ok(TriggerDraft::Schedule(cron_expression.clone()))
(B[m[31m-        }
(B[m[31m-        TriggerIr::UserAction { action_type, target } if action_type == "state_change" => {
(B[m[31m-            Ok(TriggerDraft::StateChange(target.clone()))
(B[m[31m-        }
(B[m[32m+        TriggerIr::ScheduleCron {
(B[m[32m+            cron_expression, ..
(B[m[32m+        } => Ok(TriggerDraft::Schedule(cron_expression.clone())),
(B[m[32m+        TriggerIr::UserAction {
(B[m[32m+            action_type,
(B[m[32m+            target,
(B[m[32m+        } if action_type == "state_change" => Ok(TriggerDraft::StateChange(target.clone())),
(B[m         _ => Err(anyhow!("Unsupported trigger type for TOML conversion")),
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:165:
 
 fn draft_condition_to_ir(cond: &ConditionDraft) -> Result<ConditionIr> {
     // ConditionDraft is generic; map to IR custom predicate
[31m-    Ok(ConditionIr::CustomPredicate { name: cond.kind.clone(), args: cond.params.clone() })
(B[m[32m+    Ok(ConditionIr::CustomPredicate {
(B[m[32m+        name: cond.kind.clone(),
(B[m[32m+        args: cond.params.clone(),
(B[m[32m+    })
(B[m }
 
 fn ir_condition_to_draft(cond: &ConditionIr) -> Result<ConditionDraft> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:172:
     match cond {
[31m-        ConditionIr::CustomPredicate { name, args } => {
(B[m[31m-            Ok(ConditionDraft { kind: name.clone(), params: args.clone() })
(B[m[31m-        }
(B[m[31m-        ConditionIr::TimeInRange { start_hour, end_hour } => Ok(ConditionDraft {
(B[m[32m+        ConditionIr::CustomPredicate { name, args } => Ok(ConditionDraft {
(B[m[32m+            kind: name.clone(),
(B[m[32m+            params: args.clone(),
(B[m[32m+        }),
(B[m[32m+        ConditionIr::TimeInRange {
(B[m[32m+            start_hour,
(B[m[32m+            end_hour,
(B[m[32m+        } => Ok(ConditionDraft {
(B[m             kind: "time_in_range".to_string(),
             params: serde_json::json!({
                 "start_hour": start_hour,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:190:
 
 fn draft_action_to_ir(action: &ActionDraft) -> Result<ActionIr> {
     match action {
[31m-        ActionDraft::Block { profile, duration_seconds, .. } => Ok(ActionIr::EnforcePolicy {
(B[m[32m+        ActionDraft::Block {
(B[m[32m+            profile,
(B[m[32m+            duration_seconds,
(B[m[32m+            ..
(B[m[32m+        } => Ok(ActionIr::EnforcePolicy {
(B[m             policy_id: "block".to_string(),
             params: {
                 let mut m = BTreeMap::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:197:
                 m.insert("profile".to_string(), serde_json::json!(profile));
[31m-                m.insert("duration_secs".to_string(), serde_json::json!(duration_seconds));
(B[m[32m+                m.insert(
(B[m[32m+                    "duration_secs".to_string(),
(B[m[32m+                    serde_json::json!(duration_seconds),
(B[m[32m+                );
(B[m                 m
             },
         }),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:251:
     match action {
         ActionIr::EnforcePolicy { policy_id, params } => {
             if policy_id == "block" {
[31m-                let profile =
(B[m[31m-                    params.get("profile").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
(B[m[31m-                let duration_seconds =
(B[m[31m-                    params.get("duration_secs").and_then(|v| v.as_i64()).unwrap_or(0);
(B[m[32m+                let profile = params
(B[m[32m+                    .get("profile")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("unknown")
(B[m[32m+                    .to_string();
(B[m[32m+                let duration_seconds = params
(B[m[32m+                    .get("duration_secs")
(B[m[32m+                    .and_then(|v| v.as_i64())
(B[m[32m+                    .unwrap_or(0);
(B[m 
                 Ok(ActionDraft::Block {
                     profile,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:262:
                     rigidity: focus_templates::RigidityDraft::Hard,
                 })
             } else if policy_id == "unblock" {
[31m-                let profile =
(B[m[31m-                    params.get("profile").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
(B[m[32m+                let profile = params
(B[m[32m+                    .get("profile")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("unknown")
(B[m[32m+                    .to_string();
(B[m                 Ok(ActionDraft::Unblock { profile })
             } else {
                 Err(anyhow!("Unsupported enforcement policy: {}", policy_id))
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:270:
             }
         }
[31m-        ActionIr::EmitEvent { event_type, payload } => match event_type.as_str() {
(B[m[32m+        ActionIr::EmitEvent {
(B[m[32m+            event_type,
(B[m[32m+            payload,
(B[m[32m+        } => match event_type.as_str() {
(B[m             "grant_credit" => {
                 let amount = payload.get("amount").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                 Ok(ActionDraft::GrantCredit { amount })
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:279:
                 Ok(ActionDraft::DeductCredit { amount })
             }
             "streak_increment" => {
[31m-                let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
(B[m[32m+                let name = payload
(B[m[32m+                    .get("name")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("")
(B[m[32m+                    .to_string();
(B[m                 Ok(ActionDraft::StreakIncrement { name })
             }
             "streak_reset" => {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:286:
[31m-                let name = payload.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
(B[m[32m+                let name = payload
(B[m[32m+                    .get("name")
(B[m[32m+                    .and_then(|v| v.as_str())
(B[m[32m+                    .unwrap_or("")
(B[m[32m+                    .to_string();
(B[m                 Ok(ActionDraft::StreakReset { name })
             }
             _ => Err(anyhow!("Unsupported event type: {}", event_type)),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:290:
         },
[31m-        ActionIr::ShowNotification { text, .. } => {
(B[m[31m-            Ok(ActionDraft::Notify { message: text.clone() })
(B[m[31m-        }
(B[m[32m+        ActionIr::ShowNotification { text, .. } => Ok(ActionDraft::Notify {
(B[m[32m+            message: text.clone(),
(B[m[32m+        }),
(B[m         _ => Err(anyhow!("Unsupported action type for TOML conversion")),
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/toml_transpiler.rs:354:
         let rule_ir = RuleIr {
             id: "test".to_string(),
             name: "test".to_string(),
[31m-            trigger: TriggerIr::EventFired { event_name: "evt".to_string() },
(B[m[32m+            trigger: TriggerIr::EventFired {
(B[m[32m+                event_name: "evt".to_string(),
(B[m[32m+            },
(B[m             conditions: vec![],
             actions: vec![ActionIr::EnforcePolicy {
                 policy_id: "block".to_string(),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/wizard_transpiler.rs:155:
                 .or_else(|| value.get("cron").and_then(|v| v.as_str()))
                 .unwrap_or("0 * * * *")
                 .to_string();
[31m-            Ok(TriggerIr::ScheduleCron { cron_expression: cron, timezone: "UTC".to_string() })
(B[m[32m+            Ok(TriggerIr::ScheduleCron {
(B[m[32m+                cron_expression: cron,
(B[m[32m+                timezone: "UTC".to_string(),
(B[m[32m+            })
(B[m         }
         "user_starts_session" => {
             let session_type = value
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/wizard_transpiler.rs:171:
                 .or_else(|| value.get("target").and_then(|v| v.as_str()))
                 .unwrap_or("unknown")
                 .to_string();
[31m-            Ok(TriggerIr::UserAction { action_type: "state_change".to_string(), target })
(B[m[32m+            Ok(TriggerIr::UserAction {
(B[m[32m+                action_type: "state_change".to_string(),
(B[m[32m+                target,
(B[m[32m+            })
(B[m         }
         _ => Err(anyhow!("Unknown trigger kind: {}", kind)),
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-transpilers/src/wizard_transpiler.rs:179:
 
 fn ir_trigger_to_wizard_fields(trigger: &TriggerIr) -> Result<(String, serde_json::Value)> {
     match trigger {
[31m-        TriggerIr::EventFired { event_name } => {
(B[m[31m-            Ok(("event".to_string(), serde_json::Value::String(event_name.clone())))
(B[m[31m-        }
(B[m[31m-        TriggerIr::ScheduleCron { cron_expression, .. } => {
(B[m[31m-            Ok(("schedule".to_string(), serde_json::Value::String(cron_expression.clone())))
(B[m[31m-        }
(B[m[31m-        TriggerIr::UserStartsSession { session_type } => {
(B[m[31m-            Ok(("user_starts_session".to_string(), serde_json::Value::String(session_type.clone())))
(B[m[31m-        }
(B[m[31m-        TriggerIr::UserAction { target, .. } => {
(B[m[31m-            Ok(("state_change".to_string(), serde_json::Value::String(target.clone())))
(B[m[31m-        }
(B[m[32m+        TriggerIr::EventFired { event_name } => Ok((
(B[m[32m+            "event".to_string(),
(B[m[32m+            serde_json::Value::String(event_name.clone()),
(B[m[32m+        )),
(B[m[32m+        TriggerIr::ScheduleCron {
(B[m[32m+            cron_expression, ..
(B[m[32m+        } => Ok((
(B[m[32m+            "schedule".to_string(),
(B[m[32m+            serde_json::Value::String(cron_expression.clone()),
(B[m[32m+        )),
(B[m[32m+        TriggerIr::UserStartsSession { session_type } => Ok((
(B[m[32m+            "user_starts_session".to_string(),
(B[m[32m+            serde_json::Value::String(session_type.clone()),
(B[m[32m+        )),
(B[m[32m+        TriggerIr::UserAction { target, .. } => Ok((
(B[m[32m+            "state_change".to_string(),
(B[m[32m+            serde_json::Value::String(target.clone()),
(B[m[32m+        )),
(B[m         _ => Err(anyhow!("Unsupported trigger type for wizard conversion")),
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ui/tests/icon_snapshot.rs:15:
     use std::fs;
 
     let sprite_path = workspace_root().join("assets/icons/sprite.svg");
[31m-    assert!(sprite_path.exists(), "Icon sprite must exist at assets/icons/sprite.svg");
(B[m[32m+    assert!(
(B[m[32m+        sprite_path.exists(),
(B[m[32m+        "Icon sprite must exist at assets/icons/sprite.svg"
(B[m[32m+    );
(B[m 
[31m-    let sprite_content = fs::read_to_string(sprite_path)
(B[m[31m-        .expect("Failed to read sprite.svg");
(B[m[32m+    let sprite_content = fs::read_to_string(sprite_path).expect("Failed to read sprite.svg");
(B[m 
     // Verify sprite is valid SVG
[31m-    assert!(sprite_content.contains("<svg"), "Sprite must contain SVG root");
(B[m[31m-    assert!(sprite_content.contains("</svg>"), "Sprite must have closing SVG tag");
(B[m[31m-    assert!(sprite_content.contains("<symbol"), "Sprite must contain symbol elements");
(B[m[32m+    assert!(
(B[m[32m+        sprite_content.contains("<svg"),
(B[m[32m+        "Sprite must contain SVG root"
(B[m[32m+    );
(B[m[32m+    assert!(
(B[m[32m+        sprite_content.contains("</svg>"),
(B[m[32m+        "Sprite must have closing SVG tag"
(B[m[32m+    );
(B[m[32m+    assert!(
(B[m[32m+        sprite_content.contains("<symbol"),
(B[m[32m+        "Sprite must contain symbol elements"
(B[m[32m+    );
(B[m 
     // Count symbols (each icon is a <symbol> element)
     let symbol_count = sprite_content.matches("<symbol").count();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ui/tests/icon_snapshot.rs:31:
 
     // Verify expected icon names exist
     let expected_icons = vec![
[31m-        "nav-home", "nav-focus", "nav-rules", "nav-insights", "nav-connectors", "nav-settings",
(B[m[31m-        "focus-strict", "focus-moderate", "focus-light", "focus-break", "focus-sleep",
(B[m[31m-        "rule-app", "rule-time", "rule-penalty", "rule-reward", "rule-allowlist",
(B[m[31m-        "connector-canvas", "connector-slack", "connector-gmail",
(B[m[31m-        "status-active", "status-blocked", "status-warning",
(B[m[31m-        "achievement-streak", "achievement-milestone",
(B[m[31m-        "action-add", "action-delete", "action-edit",
(B[m[31m-        "mascot-happy", "mascot-thinking", "mascot-celebrating",
(B[m[32m+        "nav-home",
(B[m[32m+        "nav-focus",
(B[m[32m+        "nav-rules",
(B[m[32m+        "nav-insights",
(B[m[32m+        "nav-connectors",
(B[m[32m+        "nav-settings",
(B[m[32m+        "focus-strict",
(B[m[32m+        "focus-moderate",
(B[m[32m+        "focus-light",
(B[m[32m+        "focus-break",
(B[m[32m+        "focus-sleep",
(B[m[32m+        "rule-app",
(B[m[32m+        "rule-time",
(B[m[32m+        "rule-penalty",
(B[m[32m+        "rule-reward",
(B[m[32m+        "rule-allowlist",
(B[m[32m+        "connector-canvas",
(B[m[32m+        "connector-slack",
(B[m[32m+        "connector-gmail",
(B[m[32m+        "status-active",
(B[m[32m+        "status-blocked",
(B[m[32m+        "status-warning",
(B[m[32m+        "achievement-streak",
(B[m[32m+        "achievement-milestone",
(B[m[32m+        "action-add",
(B[m[32m+        "action-delete",
(B[m[32m+        "action-edit",
(B[m[32m+        "mascot-happy",
(B[m[32m+        "mascot-thinking",
(B[m[32m+        "mascot-celebrating",
(B[m     ];
 
     for icon_name in expected_icons {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ui/tests/icon_snapshot.rs:56:
     use std::fs;
 
     let sprite_path = workspace_root().join("assets/icons/sprite.svg");
[31m-    let metadata = fs::metadata(sprite_path)
(B[m[31m-        .expect("Failed to stat sprite.svg");
(B[m[32m+    let metadata = fs::metadata(sprite_path).expect("Failed to stat sprite.svg");
(B[m     let size_bytes = metadata.len();
 
     // Sprite should be reasonably sized (10-25 KB for 63 icons)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-ui/tests/icon_snapshot.rs:78:
         "Icon types must exist at assets/icons/sprite.types.ts"
     );
 
[31m-    let types_content = fs::read_to_string(types_path)
(B[m[31m-        .expect("Failed to read sprite.types.ts");
(B[m[32m+    let types_content = fs::read_to_string(types_path).expect("Failed to read sprite.types.ts");
(B[m 
     // Verify TypeScript exports
     assert!(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/handler.rs:1:
 use async_trait::async_trait;
 use connector_github::webhook::GitHubWebhookHandler;
 use focus_connectors::{
[31m-    signature_verifiers::{CanvasLtiVerifier, GCalChannelVerifier, GitHubHmacVerifier, SignatureVerifier},
(B[m[32m+    signature_verifiers::{
(B[m[32m+        CanvasLtiVerifier, GCalChannelVerifier, GitHubHmacVerifier, SignatureVerifier,
(B[m[32m+    },
(B[m     ConnectorError, Result, WebhookDelivery, WebhookHandler, WebhookRegistry,
 };
 use focus_events::NormalizedEvent;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/handler.rs:243:
             secret: secret.clone(),
         });
 
[31m-        let handler = GitHubHandlerImpl { account_id, verifier };
(B[m[32m+        let handler = GitHubHandlerImpl {
(B[m[32m+            account_id,
(B[m[32m+            verifier,
(B[m[32m+        };
(B[m 
         // Create a valid HMAC signature
         use hmac::Mac;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/handler.rs:275:
     fn test_extract_event_kind_github() {
         let mut headers = HashMap::new();
         headers.insert("x-github-event".to_string(), "pull_request".to_string());
[31m-        assert_eq!(super::extract_event_kind("github", &headers), "pull_request");
(B[m[32m+        assert_eq!(
(B[m[32m+            super::extract_event_kind("github", &headers),
(B[m[32m+            "pull_request"
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/handler.rs:287:
     #[test]
     fn test_extract_event_kind_canvas() {
         let mut headers = HashMap::new();
[31m-        headers.insert("x-canvas-event".to_string(), "assignment_submission".to_string());
(B[m[31m-        assert_eq!(super::extract_event_kind("canvas", &headers), "assignment_submission");
(B[m[32m+        headers.insert(
(B[m[32m+            "x-canvas-event".to_string(),
(B[m[32m+            "assignment_submission".to_string(),
(B[m[32m+        );
(B[m[32m+        assert_eq!(
(B[m[32m+            super::extract_event_kind("canvas", &headers),
(B[m[32m+            "assignment_submission"
(B[m[32m+        );
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:8:
 };
 use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
 use base64::engine::Engine;
[32m+use chrono::{DateTime, Utc};
(B[m use clap::Parser;
 use focus_connectors::WebhookRegistry;
 use focus_plugin_sdk::{PluginRuntime, RuntimeConfig};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:14:
 use serde::{Deserialize, Serialize};
[32m+use std::collections::HashMap;
(B[m use std::net::SocketAddr;
 use std::sync::Arc;
[31m-use std::collections::HashMap;
(B[m[32m+use std::sync::RwLock;
(B[m use tower::ServiceBuilder;
 use tower_http::trace::TraceLayer;
 use tracing::{debug, error, info, warn};
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:21:
[31m-use std::sync::RwLock;
(B[m[31m-use chrono::{DateTime, Utc};
(B[m 
 mod handler;
 mod rate_limit;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:117:
     let app = Router::new()
         .route("/healthz", get(healthz))
         .route("/webhooks/:connector_id", post(webhook_handler))
[31m-        .route("/webhooks/:connector_id/:event_type", post(webhook_handler_with_type))
(B[m[32m+        .route(
(B[m[32m+            "/webhooks/:connector_id/:event_type",
(B[m[32m+            post(webhook_handler_with_type),
(B[m[32m+        )
(B[m         .route("/plugins/:plugin_id/poll", post(plugin_poll_handler))
         .with_state(state)
         .layer(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:172:
     }
 
     // Dispatch to handler
[31m-    let result = handler::handle_webhook(&state.registry, &connector_id, header_map, body.to_vec()).await;
(B[m[32m+    let result =
(B[m[32m+        handler::handle_webhook(&state.registry, &connector_id, header_map, body.to_vec()).await;
(B[m 
     // Update health metrics
     {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:179:
         let mut metrics = state.health_metrics.write().unwrap();
[31m-        let health = metrics.entry(connector_id.clone()).or_insert(ConnectorHealth {
(B[m[31m-            last_received_at: None,
(B[m[31m-            hmac_success_count: 0,
(B[m[31m-            hmac_failure_count: 0,
(B[m[31m-            last_hour_count: 0,
(B[m[31m-        });
(B[m[32m+        let health = metrics
(B[m[32m+            .entry(connector_id.clone())
(B[m[32m+            .or_insert(ConnectorHealth {
(B[m[32m+                last_received_at: None,
(B[m[32m+                hmac_success_count: 0,
(B[m[32m+                hmac_failure_count: 0,
(B[m[32m+                last_hour_count: 0,
(B[m[32m+            });
(B[m         health.last_received_at = Some(Utc::now());
         health.last_hour_count += 1;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:250:
     // Update health metrics
     {
         let mut metrics = state.health_metrics.write().unwrap();
[31m-        let health = metrics.entry(connector_id.clone()).or_insert(ConnectorHealth {
(B[m[31m-            last_received_at: None,
(B[m[31m-            hmac_success_count: 0,
(B[m[31m-            hmac_failure_count: 0,
(B[m[31m-            last_hour_count: 0,
(B[m[31m-        });
(B[m[32m+        let health = metrics
(B[m[32m+            .entry(connector_id.clone())
(B[m[32m+            .or_insert(ConnectorHealth {
(B[m[32m+                last_received_at: None,
(B[m[32m+                hmac_success_count: 0,
(B[m[32m+                hmac_failure_count: 0,
(B[m[32m+                last_hour_count: 0,
(B[m[32m+            });
(B[m         health.last_received_at = Some(Utc::now());
         health.last_hour_count += 1;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:292:
     // GitHub handler
     if let Ok(secret) = std::env::var("FOCALPOINT_GITHUB_WEBHOOK_SECRET") {
         info!("registering github webhook handler");
[31m-        let verifier = Arc::new(
(B[m[31m-            focus_connectors::signature_verifiers::GitHubHmacVerifier {
(B[m[31m-                secret: secrecy::SecretString::new(secret.into_boxed_str()),
(B[m[31m-            },
(B[m[31m-        );
(B[m[32m+        let verifier = Arc::new(focus_connectors::signature_verifiers::GitHubHmacVerifier {
(B[m[32m+            secret: secrecy::SecretString::new(secret.into_boxed_str()),
(B[m[32m+        });
(B[m         let handler = Arc::new(handler::GitHubHandlerImpl {
             account_id: uuid::Uuid::nil(), // TODO: extract from config
             verifier,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:332:
     // GCal handler (stub)
     if let Ok(channel_token) = std::env::var("FOCALPOINT_GCAL_CHANNEL_TOKEN") {
         info!("registering google calendar webhook handler (stub)");
[31m-        let verifier = Arc::new(
(B[m[31m-            focus_connectors::signature_verifiers::GCalChannelVerifier {
(B[m[31m-                channel_token: secrecy::SecretString::new(channel_token.into_boxed_str()),
(B[m[31m-            },
(B[m[31m-        );
(B[m[32m+        let verifier = Arc::new(focus_connectors::signature_verifiers::GCalChannelVerifier {
(B[m[32m+            channel_token: secrecy::SecretString::new(channel_token.into_boxed_str()),
(B[m[32m+        });
(B[m         let handler = Arc::new(handler::GCalHandlerImpl {
             account_id: uuid::Uuid::nil(),
             verifier,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/focus-webhook-server/src/main.rs:467:
 }
 
 impl CleanupGuard {
[31m-    fn new(
(B[m[31m-        status: Arc<RwLock<HashMap<String, PluginExecStatus>>>,
(B[m[31m-        plugin_id: String,
(B[m[31m-    ) -> Self {
(B[m[32m+    fn new(status: Arc<RwLock<HashMap<String, PluginExecStatus>>>, plugin_id: String) -> Self {
(B[m         Self { status, plugin_id }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/pheno-tracing/src/adapters.rs:1:
 //! Adapter implementations for tracing backends
[31m-use crate::port::{TraceOperation, TraceResult, TraceStatus, TracePort};
(B[m[32m+use crate::port::{TraceOperation, TracePort, TraceResult, TraceStatus};
(B[m use std::sync::{Arc, Mutex};
 
 /// In-memory adapter for testing
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/pheno-tracing/src/lib.rs:5:
 pub mod adapters;
 pub mod port;
 
[31m-pub use port::{TracePort, TraceOperation, TraceResult, SpanId, TraceId, SpanKind};
(B[m[32m+pub use port::{SpanId, SpanKind, TraceId, TraceOperation, TracePort, TraceResult};
(B[m 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/pheno-tracing/tests/adapter_tests.rs:1:
[31m-use pheno_tracing::port::{TraceId, SpanId, TraceOperation, SpanKind, TraceStatus};
(B[m use pheno_tracing::adapters::StdoutAdapter;
[31m-use pheno_tracing::TracePort;  // Bring the trait into scope
(B[m[32m+use pheno_tracing::port::{SpanId, SpanKind, TraceId, TraceOperation, TraceStatus};
(B[m[32m+use pheno_tracing::TracePort; // Bring the trait into scope
(B[m use std::collections::HashMap;
 
 #[tokio::test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/pheno-tracing/tests/port_integration.rs:1:
[31m-use pheno_tracing::port::{TraceId, SpanId, TraceOperation, SpanKind, TracePort, TraceStatus};
(B[m use pheno_tracing::adapters::InMemoryAdapter;
[32m+use pheno_tracing::port::{SpanId, SpanKind, TraceId, TraceOperation, TracePort, TraceStatus};
(B[m use std::collections::HashMap;
 
 #[tokio::test]
Warning: can't set `indent_style = Block`, unstable features are only available in nightly channel.
Warning: can't set `group_imports = StdExternalCrate`, unstable features are only available in nightly channel.
Warning: can't set `indent_style = Block`, unstable features are only available in nightly channel.
Warning: can't set `group_imports = StdExternalCrate`, unstable features are only available in nightly channel.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:1:
 //! Format adapters for parsing configuration files.
 
[31m-use crate::domain::{Config, ConfigValue, errors::ConfigError};
(B[m[32m+use crate::domain::{errors::ConfigError, Config, ConfigValue};
(B[m use std::collections::HashMap;
 
 /// TOML format parser.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:8:
 
 impl TomlFormat {
     pub fn parse(&self, content: &str) -> Result<Config, ConfigError> {
[31m-        let value: toml::Value = toml::from_str(content)
(B[m[31m-            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
(B[m[32m+        let value: toml::Value =
(B[m[32m+            toml::from_str(content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
(B[m 
         let values = toml_to_json(value);
         let mut config = Config::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:25:
 
 impl YamlFormat {
     pub fn parse(&self, content: &str) -> Result<Config, ConfigError> {
[31m-        let value: serde_yaml::Value = serde_yaml::from_str(content)
(B[m[31m-            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
(B[m[32m+        let value: serde_yaml::Value =
(B[m[32m+            serde_yaml::from_str(content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
(B[m 
         let values = yaml_to_json(value);
         let mut config = Config::new();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:42:
 
 impl JsonFormat {
     pub fn parse(&self, content: &str) -> Result<Config, ConfigError> {
[31m-        let value: serde_json::Value = serde_json::from_str(content)
(B[m[31m-            .map_err(|e| ConfigError::ParseError(e.to_string()))?;
(B[m[32m+        let value: serde_json::Value =
(B[m[32m+            serde_json::from_str(content).map_err(|e| ConfigError::ParseError(e.to_string()))?;
(B[m 
         let mut config = Config::new();
         for (key, value) in flatten_json(value) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:66:
             serde_json::Value::Array(arr.into_iter().map(toml_to_json).collect())
         }
         toml::Value::Table(map) => {
[31m-            let obj: serde_json::Map<String, serde_json::Value> = map
(B[m[31m-                .into_iter()
(B[m[31m-                .map(|(k, v)| (k, toml_to_json(v)))
(B[m[31m-                .collect();
(B[m[32m+            let obj: serde_json::Map<String, serde_json::Value> =
(B[m[32m+                map.into_iter().map(|(k, v)| (k, toml_to_json(v))).collect();
(B[m             serde_json::Value::Object(obj)
         }
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:97:
         serde_yaml::Value::Mapping(map) => {
             let obj: serde_json::Map<String, serde_json::Value> = map
                 .into_iter()
[31m-                .filter_map(|(k, v)| {
(B[m[31m-                    k.as_str().map(|k| (k.to_string(), yaml_to_json(v)))
(B[m[31m-                })
(B[m[32m+                .filter_map(|(k, v)| k.as_str().map(|k| (k.to_string(), yaml_to_json(v))))
(B[m                 .collect();
             serde_json::Value::Object(obj)
         }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/formats.rs:111:
     match value {
         serde_json::Value::Null => ConfigValue::Null,
         serde_json::Value::Bool(b) => ConfigValue::Bool(*b),
[31m-        serde_json::Value::Number(n) => ConfigValue::Number(
(B[m[31m-            n.as_f64().unwrap_or(0.0)
(B[m[31m-        ),
(B[m[32m+        serde_json::Value::Number(n) => ConfigValue::Number(n.as_f64().unwrap_or(0.0)),
(B[m         serde_json::Value::String(s) => ConfigValue::String(s.clone()),
[31m-        serde_json::Value::Array(arr) => {
(B[m[31m-            ConfigValue::Array(arr.iter().map(parse_value).collect())
(B[m[31m-        }
(B[m[32m+        serde_json::Value::Array(arr) => ConfigValue::Array(arr.iter().map(parse_value).collect()),
(B[m         serde_json::Value::Object(map) => {
[31m-            ConfigValue::Object(
(B[m[31m-                map.iter()
(B[m[31m-                    .map(|(k, v)| (k.clone(), parse_value(v)))
(B[m[31m-                    .collect()
(B[m[31m-            )
(B[m[32m+            ConfigValue::Object(map.iter().map(|(k, v)| (k.clone(), parse_value(v))).collect())
(B[m         }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/mod.rs:1:
 //! Adapters layer.
 
[31m-pub mod sources;
(B[m pub mod formats;
[32m+pub mod sources;
(B[m 
[31m-pub use sources::{FileSource, EnvSource};
(B[m[31m-pub use formats::{TomlFormat, YamlFormat, JsonFormat};
(B[m[32m+pub use formats::{JsonFormat, TomlFormat, YamlFormat};
(B[m[32m+pub use sources::{EnvSource, FileSource};
(B[m 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/sources.rs:1:
 //! Configuration source adapters.
 
[32m+use crate::domain::{errors::ConfigError, sources::Source, Config};
(B[m use async_trait::async_trait;
 use std::collections::HashMap;
 use std::path::Path;
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/sources.rs:6:
[31m-use crate::domain::{
(B[m[31m-    Config, sources::Source,
(B[m[31m-    errors::ConfigError,
(B[m[31m-};
(B[m 
 /// File-based configuration source.
 pub struct FileSource {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/sources.rs:15:
 
 impl FileSource {
     pub fn new(path: impl Into<String>) -> Self {
[31m-        Self {
(B[m[31m-            path: path.into(),
(B[m[31m-        }
(B[m[32m+        Self { path: path.into() }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/sources.rs:34:
     async fn load(&self) -> Result<Config, ConfigError> {
         let content = tokio::fs::read_to_string(&self.path).await?;
 
[31m-        let extension = Path::new(&self.path)
(B[m[31m-            .extension()
(B[m[31m-            .and_then(|e| e.to_str())
(B[m[31m-            .unwrap_or("");
(B[m[32m+        let extension = Path::new(&self.path).extension().and_then(|e| e.to_str()).unwrap_or("");
(B[m 
         let values: serde_json::Value = match extension {
[31m-            "toml" => toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?,
(B[m[31m-            "yaml" | "yml" => serde_yaml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?,
(B[m[31m-            "json" => serde_json::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?,
(B[m[32m+            "toml" => {
(B[m[32m+                toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?
(B[m[32m+            }
(B[m[32m+            "yaml" | "yml" => serde_yaml::from_str(&content)
(B[m[32m+                .map_err(|e| ConfigError::ParseError(e.to_string()))?,
(B[m[32m+            "json" => serde_json::from_str(&content)
(B[m[32m+                .map_err(|e| ConfigError::ParseError(e.to_string()))?,
(B[m             _ => return Err(ConfigError::ParseError(format!("Unknown extension: {}", extension))),
         };
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/adapters/sources.rs:152:
         match value {
             serde_json::Value::Object(map) => {
                 for (key, val) in map {
[31m-                    let path = if prefix.is_empty() {
(B[m[31m-                        key.clone()
(B[m[31m-                    } else {
(B[m[31m-                        format!("{}.{}", prefix, key)
(B[m[31m-                    };
(B[m[32m+                    let path =
(B[m[32m+                        if prefix.is_empty() { key.clone() } else { format!("{}.{}", prefix, key) };
(B[m                     flatten_recursive(val, &path, result);
                 }
             }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/application/builder.rs:1:
 //! Configuration builder.
 
 use crate::domain::{
[31m-    Config, LayerPriority, LayerStack, MergeStrategy,
(B[m[31m-    sources::Source, validation::Validator, errors::ConfigError,
(B[m[32m+    errors::ConfigError, sources::Source, validation::Validator, Config, LayerPriority, LayerStack,
(B[m[32m+    MergeStrategy,
(B[m };
 use std::collections::HashMap;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/application/builder.rs:15:
 impl ConfigBuilder {
     /// Create a new builder.
     pub fn new() -> Self {
[31m-        Self {
(B[m[31m-            stack: LayerStack::new(),
(B[m[31m-            validators: Vec::new(),
(B[m[31m-        }
(B[m[32m+        Self { stack: LayerStack::new(), validators: Vec::new() }
(B[m     }
 
     /// Create with a merge strategy.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/application/builder.rs:25:
     pub fn with_strategy(strategy: MergeStrategy) -> Self {
[31m-        Self {
(B[m[31m-            stack: LayerStack::with_strategy(strategy),
(B[m[31m-            validators: Vec::new(),
(B[m[31m-        }
(B[m[32m+        Self { stack: LayerStack::with_strategy(strategy), validators: Vec::new() }
(B[m     }
 
     /// Add a source with priority.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/application/builder.rs:78:
     }
 
     /// Add a default value layer.
[31m-    pub fn with_default(
(B[m[31m-        mut self,
(B[m[31m-        values: HashMap<String, serde_json::Value>,
(B[m[31m-    ) -> Self {
(B[m[32m+    pub fn with_default(mut self, values: HashMap<String, serde_json::Value>) -> Self {
(B[m         let mut config = Config::new();
         for (key, value) in values {
             config.set(key, crate::domain::ConfigValue::from_json_value(&value));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/config.rs:138:
             serde_json::Value::Array(arr) => {
                 ConfigValue::Array(arr.iter().map(Self::from_json_value).collect())
             }
[31m-            serde_json::Value::Object(map) => {
(B[m[31m-                ConfigValue::Object(
(B[m[31m-                    map.iter()
(B[m[31m-                        .map(|(k, v)| (k.clone(), Self::from_json_value(v)))
(B[m[31m-                        .collect(),
(B[m[31m-                )
(B[m[31m-            }
(B[m[32m+            serde_json::Value::Object(map) => ConfigValue::Object(
(B[m[32m+                map.iter().map(|(k, v)| (k.clone(), Self::from_json_value(v))).collect(),
(B[m[32m+            ),
(B[m         }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/config.rs:151:
 
[31m-
(B[m impl From<bool> for ConfigValue {
     fn from(b: bool) -> Self {
         ConfigValue::Bool(b)
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/config.rs:198:
 impl Config {
     /// Create a new empty configuration.
     pub fn new() -> Self {
[31m-        Self {
(B[m[31m-            values: HashMap::new(),
(B[m[31m-            source: None,
(B[m[31m-        }
(B[m[32m+        Self { values: HashMap::new(), source: None }
(B[m     }
 
     /// Create from a values map.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/config.rs:208:
     pub fn from_values(values: HashMap<String, ConfigValue>) -> Self {
[31m-        Self {
(B[m[31m-            values,
(B[m[31m-            source: None,
(B[m[31m-        }
(B[m[32m+        Self { values, source: None }
(B[m     }
 
     /// Create with a source name.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/errors.rs:7:
     KeyNotFound(String),
 
     #[error("Type mismatch for {key}: expected {expected}, got {actual}")]
[31m-    TypeMismatch {
(B[m[31m-        key: String,
(B[m[31m-        expected: String,
(B[m[31m-        actual: String,
(B[m[31m-    },
(B[m[32m+    TypeMismatch { key: String, expected: String, actual: String },
(B[m 
     #[error("Validation failed: {validator}: {message}")]
[31m-    ValidationFailed {
(B[m[31m-        validator: String,
(B[m[31m-        message: String,
(B[m[31m-    },
(B[m[32m+    ValidationFailed { validator: String, message: String },
(B[m 
     #[error("Parse error: {0}")]
     ParseError(String),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:1:
 //! Configuration layer management.
 
[31m-use serde::{Deserialize, Serialize};
(B[m use super::config::Config;
[32m+use serde::{Deserialize, Serialize};
(B[m 
 /// Layer priority levels.
 #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:23:
     Cli = 100,
 }
 
[31m-
(B[m /// A configuration layer with priority.
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct Layer {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:38:
 impl Layer {
     /// Create a new layer.
     pub fn new(name: impl Into<String>, priority: LayerPriority, config: Config) -> Self {
[31m-        Self {
(B[m[31m-            name: name.into(),
(B[m[31m-            priority,
(B[m[31m-            config,
(B[m[31m-        }
(B[m[32m+        Self { name: name.into(), priority, config }
(B[m     }
 
     /// Create a default layer.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:72:
 }
 
 /// Strategy for merging values from different layers.
[31m-#[derive(Debug, Clone, Copy, PartialEq, Eq)]
(B[m[31m-#[derive(Default)]
(B[m[32m+#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
(B[m pub enum MergeStrategy {
     /// Higher priority overrides lower priority.
     #[default]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:86:
     AppendArrays,
 }
 
[31m-
(B[m /// Layer stack - manages multiple configuration layers.
 #[derive(Debug, Default)]
 pub struct LayerStack {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:102:
 
     /// Create with a merge strategy.
     pub fn with_strategy(strategy: MergeStrategy) -> Self {
[31m-        Self {
(B[m[31m-            layers: Vec::new(),
(B[m[31m-            strategy,
(B[m[31m-        }
(B[m[32m+        Self { layers: Vec::new(), strategy }
(B[m     }
 
     /// Add a layer.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/layers.rs:116:
     }
 
     /// Add a layer from a config with priority.
[31m-    pub fn add(&mut self, name: impl Into<String>, priority: LayerPriority, config: Config) -> &mut Self {
(B[m[32m+    pub fn add(
(B[m[32m+        &mut self,
(B[m[32m+        name: impl Into<String>,
(B[m[32m+        priority: LayerPriority,
(B[m[32m+        config: Config,
(B[m[32m+    ) -> &mut Self {
(B[m         self.add_layer(Layer::new(name, priority, config))
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/mod.rs:1:
 //! Domain layer - pure configuration logic.
 
 pub mod config;
[32m+pub mod errors;
(B[m pub mod layers;
[32m+pub mod ports;
(B[m pub mod sources;
 pub mod validation;
[31m-pub mod ports;
(B[m[31m-pub mod errors;
(B[m 
 // Re-exports
[31m-pub use config::{Config, ConfigValue, ConfigPath};
(B[m[31m-pub use layers::{Layer, LayerPriority, MergeStrategy, LayerStack};
(B[m[32m+pub use config::{Config, ConfigPath, ConfigValue};
(B[m[32m+pub use errors::ConfigError;
(B[m[32m+pub use layers::{Layer, LayerPriority, LayerStack, MergeStrategy};
(B[m pub use sources::Source;
 pub use validation::Validator;
[31m-pub use errors::ConfigError;
(B[m 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/sources.rs:1:
 //! Configuration source definitions.
 
[31m-use async_trait::async_trait;
(B[m use super::config::Config;
 use super::errors::ConfigError;
[32m+use async_trait::async_trait;
(B[m 
 /// Trait for configuration sources.
 #[async_trait]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/validation.rs:1:
 //! Configuration validation.
 
[31m-use super::errors::ConfigError;
(B[m use super::config::{Config, ConfigValue};
[32m+use super::errors::ConfigError;
(B[m 
 /// Validation rule.
 pub trait Validator: Send + Sync {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/validation.rs:49:
 
 impl TypeValidator {
     pub fn new(key: impl Into<String>, expected_type: &'static str) -> Self {
[31m-        Self {
(B[m[31m-            key: key.into(),
(B[m[31m-            expected_type,
(B[m[31m-        }
(B[m[32m+        Self { key: key.into(), expected_type }
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/validation.rs:99:
 
 impl RangeValidator {
     pub fn new(key: impl Into<String>) -> Self {
[31m-        Self {
(B[m[31m-            key: key.into(),
(B[m[31m-            min: None,
(B[m[31m-            max: None,
(B[m[31m-        }
(B[m[32m+        Self { key: key.into(), min: None, max: None }
(B[m     }
 
     pub fn with_min(mut self, min: f64) -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/validation.rs:167:
 
 impl CompositeValidator {
     pub fn new() -> Self {
[31m-        Self {
(B[m[31m-            validators: Vec::new(),
(B[m[31m-        }
(B[m[32m+        Self { validators: Vec::new() }
(B[m     }
 
     /// Add a validator.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/domain/validation.rs:217:
 
     #[test]
     fn test_range_validator() {
[31m-        let validator = RangeValidator::new("port")
(B[m[31m-            .with_min(1.0)
(B[m[31m-            .with_max(65535.0);
(B[m[32m+        let validator = RangeValidator::new("port").with_min(1.0).with_max(65535.0);
(B[m 
         let mut config = Config::new();
         config.set("port", 8080);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/lib.rs:25:
 //!     .unwrap();
 //! ```
 
[31m-pub mod domain;
(B[m[31m-pub mod application;
(B[m pub mod adapters;
[32m+pub mod application;
(B[m[32m+pub mod domain;
(B[m pub mod infrastructure;
 
 // Re-exports
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-config/src/lib.rs:34:
[31m-pub use domain::{Config, ConfigValue, Layer, LayerPriority};
(B[m[31m-pub use domain::errors::ConfigError;
(B[m pub use application::builder::ConfigBuilder;
[32m+pub use domain::errors::ConfigError;
(B[m[32m+pub use domain::{Config, ConfigValue, Layer, LayerPriority};
(B[m pub use infrastructure::error::ConfigKitError;
 
 /// Framework version
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-contracts/src/ports/outbound/mod.rs:23:
 /// Outbound port for caching operations.
 pub trait CachePort: Send + Sync {
     async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, crate::Error>;
[31m-    async fn set(&self, key: &str, value: Vec<u8>, ttl: Option<std::time::Duration>) -> Result<(), crate::Error>;
(B[m[32m+    async fn set(
(B[m[32m+        &self,
(B[m[32m+        key: &str,
(B[m[32m+        value: Vec<u8>,
(B[m[32m+        ttl: Option<std::time::Duration>,
(B[m[32m+    ) -> Result<(), crate::Error>;
(B[m     async fn delete(&self, key: &str) -> Result<(), crate::Error>;
     async fn clear(&self) -> Result<(), crate::Error>;
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-contracts/src/ports/outbound/mod.rs:36:
 
 /// Outbound port for subscribing to domain events.
 pub trait EventSubscriber: Send + Sync {
[31m-    async fn subscribe<E: serde::de::DeserializeOwned>(&self, topic: &str) -> Result<(), crate::Error>;
(B[m[32m+    async fn subscribe<E: serde::de::DeserializeOwned>(
(B[m[32m+        &self,
(B[m[32m+        topic: &str,
(B[m[32m+    ) -> Result<(), crate::Error>;
(B[m }
 
 /// Outbound port for secret management.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-contracts/src/ports/outbound/mod.rs:48:
 
 /// Outbound port for versioned secret management.
 pub trait VersionedSecretPort: SecretPort + Send + Sync {
[31m-    async fn get_secret_version(&self, key: &str, version: &str) -> Result<Option<String>, crate::Error>;
(B[m[32m+    async fn get_secret_version(
(B[m[32m+        &self,
(B[m[32m+        key: &str,
(B[m[32m+        version: &str,
(B[m[32m+    ) -> Result<Option<String>, crate::Error>;
(B[m     async fn list_secret_versions(&self, key: &str) -> Result<Vec<String>, crate::Error>;
     async fn rotate_secret(&self, key: &str, value: &str) -> Result<String, crate::Error>;
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/encryption.rs:6:
 };
 use rand::RngCore;
 
[31m-use crate::{AES256_KEY_SIZE, AES_GCM_NONCE_SIZE, CryptoError, Result};
(B[m[32m+use crate::{CryptoError, Result, AES256_KEY_SIZE, AES_GCM_NONCE_SIZE};
(B[m 
 /// AES-GCM encryption error types.
 #[derive(Debug, Clone, PartialEq, Eq)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/encryption.rs:79:
             });
         }
         let nonce = Nonce::from_slice(nonce);
[31m-        let ciphertext = self.cipher
(B[m[32m+        let ciphertext = self
(B[m[32m+            .cipher
(B[m             .encrypt(nonce, plaintext)
             .map_err(|_| CryptoError::EncryptionFailed("encryption failed".into()))?;
         Ok((ciphertext, nonce.to_vec()))
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hashing.rs:1:
 //! Hashing utilities for SHA-256 and BLAKE3.
 
[31m-use sha2::{Digest, Sha256};
(B[m use blake3::Hasher as Blake3Hasher;
[32m+use sha2::{Digest, Sha256};
(B[m 
 /// Available hash algorithms.
 #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hashing.rs:16:
 pub struct Hash(pub Vec<u8>);
 
 impl Hash {
[31m-    pub fn as_bytes(&self) -> &[u8] { &self.0 }
(B[m[31m-    pub fn as_hex(&self) -> String { hex::encode(&self.0) }
(B[m[31m-    pub fn len(&self) -> usize { self.0.len() }
(B[m[31m-    pub fn is_empty(&self) -> bool { self.0.is_empty() }
(B[m[32m+    pub fn as_bytes(&self) -> &[u8] {
(B[m[32m+        &self.0
(B[m[32m+    }
(B[m[32m+    pub fn as_hex(&self) -> String {
(B[m[32m+        hex::encode(&self.0)
(B[m[32m+    }
(B[m[32m+    pub fn len(&self) -> usize {
(B[m[32m+        self.0.len()
(B[m[32m+    }
(B[m[32m+    pub fn is_empty(&self) -> bool {
(B[m[32m+        self.0.is_empty()
(B[m[32m+    }
(B[m }
 
 impl AsRef<[u8]> for Hash {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hashing.rs:26:
[31m-    fn as_ref(&self) -> &[u8] { &self.0 }
(B[m[32m+    fn as_ref(&self) -> &[u8] {
(B[m[32m+        &self.0
(B[m[32m+    }
(B[m }
 
 impl From<Vec<u8>> for Hash {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hashing.rs:30:
[31m-    fn from(v: Vec<u8>) -> Self { Hash(v) }
(B[m[32m+    fn from(v: Vec<u8>) -> Self {
(B[m[32m+        Hash(v)
(B[m[32m+    }
(B[m }
 
 /// Hash builder for configurable hashing.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hashing.rs:37:
 }
 
 impl Hasher {
[31m-    pub fn new() -> Self { Self { algorithm: HashAlgorithm::default() } }
(B[m[31m-    pub fn sha256() -> Self { Self { algorithm: HashAlgorithm::Sha256 } }
(B[m[31m-    pub fn blake3() -> Self { Self { algorithm: HashAlgorithm::Blake3 } }
(B[m[31m-    pub fn with_algorithm(algorithm: HashAlgorithm) -> Self { Self { algorithm } }
(B[m[32m+    pub fn new() -> Self {
(B[m[32m+        Self {
(B[m[32m+            algorithm: HashAlgorithm::default(),
(B[m[32m+        }
(B[m[32m+    }
(B[m[32m+    pub fn sha256() -> Self {
(B[m[32m+        Self {
(B[m[32m+            algorithm: HashAlgorithm::Sha256,
(B[m[32m+        }
(B[m[32m+    }
(B[m[32m+    pub fn blake3() -> Self {
(B[m[32m+        Self {
(B[m[32m+            algorithm: HashAlgorithm::Blake3,
(B[m[32m+        }
(B[m[32m+    }
(B[m[32m+    pub fn with_algorithm(algorithm: HashAlgorithm) -> Self {
(B[m[32m+        Self { algorithm }
(B[m[32m+    }
(B[m 
     pub fn hash(&self, data: &[u8]) -> Hash {
         match self.algorithm {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hmac.rs:13:
 
 impl HmacSha256 {
     pub fn new(key: &[u8]) -> Self {
[31m-        let mac = HmacSha256Type::new_from_slice(key)
(B[m[31m-            .expect("HMAC can take key of any size");
(B[m[32m+        let mac = HmacSha256Type::new_from_slice(key).expect("HMAC can take key of any size");
(B[m         Self { mac }
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/hmac.rs:27:
     }
 
     pub fn verify(self, signature: &[u8]) -> std::result::Result<(), HmacError> {
[31m-        self.mac
(B[m[31m-            .verify_slice(signature)
(B[m[31m-            .map_err(|_| HmacError)
(B[m[32m+        self.mac.verify_slice(signature).map_err(|_| HmacError)
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/key_derivation.rs:3:
 use pbkdf2::pbkdf2_hmac_array;
 use sha2::Sha256;
 
[31m-use crate::{PBKDF2_DEFAULT_ITERATIONS, PBKDF2_SALT_SIZE, CryptoError, Result};
(B[m[32m+use crate::{CryptoError, Result, PBKDF2_DEFAULT_ITERATIONS, PBKDF2_SALT_SIZE};
(B[m 
 /// PBKDF2 parameters.
 #[derive(Debug, Clone)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/key_derivation.rs:48:
 pub struct Kdf;
 
 impl Kdf {
[31m-    pub fn new() -> Self { Self }
(B[m[32m+    pub fn new() -> Self {
(B[m[32m+        Self
(B[m[32m+    }
(B[m 
     /// Generate a random salt.
     pub fn generate_salt() -> Vec<u8> {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/key_derivation.rs:78:
 }
 
 impl Default for Kdf {
[31m-    fn default() -> Self { Self::new() }
(B[m[32m+    fn default() -> Self {
(B[m[32m+        Self::new()
(B[m[32m+    }
(B[m }
 
 #[cfg(test)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/lib.rs:27:
 use sha2::{Digest, Sha256};
 use thiserror::Error;
 
[31m-pub mod hashing;
(B[m pub mod encryption;
[31m-pub mod key_derivation;
(B[m[32m+pub mod hashing;
(B[m pub mod hmac;
[32m+pub mod key_derivation;
(B[m pub mod signatures;
 
[31m-pub use hashing::{Hasher, Hash, HashAlgorithm};
(B[m pub use encryption::{AesGcmEncryptor, AesGcmError};
[32m+pub use hashing::{Hash, HashAlgorithm, Hasher};
(B[m[32m+pub use hmac::{HmacError, HmacSha256};
(B[m pub use key_derivation::{Kdf, KdfParams, Pbkdf2Error};
[31m-pub use hmac::{HmacSha256, HmacError};
(B[m pub use signatures::{Ed25519Signer, Ed25519Verifier, SignatureError};
 
 /// Result type alias for crypto operations.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/lib.rs:167:
         let input = b"hello world";
         let hash = blake3(input);
         assert_eq!(hash.len(), 32); // BLAKE3 output is 32 bytes
[31m-        // Known BLAKE3 hash of "hello world"
(B[m[32m+                                    // Known BLAKE3 hash of "hello world"
(B[m         assert_eq!(
             hex::encode(&hash),
             "d74981efa70a0c880b8d8c1985d075dbcbf679b99a5f9914e5aaf96b831a9e24"
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/lib.rs:526:
         let signature = signer.sign(&large_message).unwrap();
         let public_key = signer.public_key();
 
[31m-        assert!(verifier.verify(&large_message, &signature, &public_key).is_ok());
(B[m[32m+        assert!(verifier
(B[m[32m+            .verify(&large_message, &signature, &public_key)
(B[m[32m+            .is_ok());
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/lib.rs:551:
         let wrong_public_key = other_verifier.public_key();
 
         // Verification with wrong public key should fail
[31m-        assert!(signer.verify(message, &signature, &wrong_public_key).is_err());
(B[m[32m+        assert!(signer
(B[m[32m+            .verify(message, &signature, &wrong_public_key)
(B[m[32m+            .is_err());
(B[m     }
 
     // -------------------------------------------------------------------------
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/lib.rs:574:
 
     #[test]
     fn test_crypto_error_display() {
[31m-        let err = CryptoError::InvalidKeySize { expected: 32, actual: 16 };
(B[m[32m+        let err = CryptoError::InvalidKeySize {
(B[m[32m+            expected: 32,
(B[m[32m+            actual: 16,
(B[m[32m+        };
(B[m         assert!(err.to_string().contains("Invalid key size"));
 
         let err = CryptoError::EncryptionFailed("test".to_string());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/signatures.rs:73:
 }
 
 impl Default for Ed25519Signer {
[31m-    fn default() -> Self { Self::new() }
(B[m[32m+    fn default() -> Self {
(B[m[32m+        Self::new()
(B[m[32m+    }
(B[m }
 
 /// Ed25519 verifier.
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/signatures.rs:135:
         let message = b"test message";
         let signature = signer.sign(message).unwrap();
 
[31m-        assert!(verifier.verify(message, &signature, &signer.public_key()).is_ok());
(B[m[32m+        assert!(verifier
(B[m[32m+            .verify(message, &signature, &signer.public_key())
(B[m[32m+            .is_ok());
(B[m     }
 
     #[test]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-crypto/src/signatures.rs:144:
         let verifier = Ed25519Verifier::from_public_key(&signer.public_key()).unwrap();
 
         let bad_sig = vec![0u8; 64];
[31m-        assert!(verifier.verify(b"message", &bad_sig, &signer.public_key()).is_err());
(B[m[32m+        assert!(verifier
(B[m[32m+            .verify(b"message", &bad_sig, &signer.public_key())
(B[m[32m+            .is_err());
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-event-sourcing/src/hash.rs:2:
 use sha2::{Digest, Sha256};
 
 /// Compute a hash for an event given its aggregate ID, sequence, payload, and previous hash.
[31m-pub fn compute_hash(aggregate_id: &str, sequence: i64, payload: &str, previous_hash: &str) -> String {
(B[m[32m+pub fn compute_hash(
(B[m[32m+    aggregate_id: &str,
(B[m[32m+    sequence: i64,
(B[m[32m+    payload: &str,
(B[m[32m+    previous_hash: &str,
(B[m[32m+) -> String {
(B[m     let mut hasher = Sha256::new();
     hasher.update(aggregate_id.as_bytes());
     hasher.update(sequence.to_be_bytes());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-event-sourcing/src/memory.rs:64:
         Ok(store.get(aggregate_id).cloned().unwrap_or_default())
     }
 
[31m-    async fn get_events_from(&self, aggregate_id: &str, sequence: i64) -> Result<Vec<Envelope<serde_json::Value>>> {
(B[m[32m+    async fn get_events_from(
(B[m[32m+        &self,
(B[m[32m+        aggregate_id: &str,
(B[m[32m+        sequence: i64,
(B[m[32m+    ) -> Result<Vec<Envelope<serde_json::Value>>> {
(B[m         let events = self.get_events(aggregate_id).await?;
[31m-        Ok(events.into_iter().filter(|e| e.sequence >= sequence).collect())
(B[m[32m+        Ok(events
(B[m[32m+            .into_iter()
(B[m[32m+            .filter(|e| e.sequence >= sequence)
(B[m[32m+            .collect())
(B[m     }
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-event-sourcing/src/store.rs:9:
 pub trait EventStore: Send + Sync {
     async fn append<T: Serialize>(&self, aggregate_id: &str, event: Envelope<T>) -> Result<i64>;
     async fn get_events(&self, aggregate_id: &str) -> Result<Vec<Envelope<serde_json::Value>>>;
[31m-    async fn get_events_from(&self, aggregate_id: &str, sequence: i64) -> Result<Vec<Envelope<serde_json::Value>>>;
(B[m[32m+    async fn get_events_from(
(B[m[32m+        &self,
(B[m[32m+        aggregate_id: &str,
(B[m[32m+        sequence: i64,
(B[m[32m+    ) -> Result<Vec<Envelope<serde_json::Value>>>;
(B[m }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-event-sourcing/src/upcaster.rs:12:
 
 impl EventVersion {
     pub fn new(major: u64, minor: u64, patch: u64) -> Self {
[31m-        Self { major, minor, patch }
(B[m[32m+        Self {
(B[m[32m+            major,
(B[m[32m+            minor,
(B[m[32m+            patch,
(B[m[32m+        }
(B[m     }
 
     pub fn initial() -> Self {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-policy-engine/src/engine.rs:70:
         Ok(PolicyResult::Allow)
     }
 
[31m-    pub fn evaluate_all(&self, context: &EvaluationContext) -> Result<HashMap<String, PolicyResult>> {
(B[m[32m+    pub fn evaluate_all(
(B[m[32m+        &self,
(B[m[32m+        context: &EvaluationContext,
(B[m[32m+    ) -> Result<HashMap<String, PolicyResult>> {
(B[m         let mut results = HashMap::new();
         for (name, policy) in self.policies.iter() {
             if policy.enabled {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-test-utils/src/lib.rs:193:
 
 impl phenotype_contracts::EventPublisher for InMemoryEventPublisher {
     async fn publish<E: serde::Serialize>(&self, event: &E) -> Result<()> {
[31m-        let value = serde_json::to_value(event).map_err(|e| PhenotypeError::serialization(e.to_string()))?;
(B[m[32m+        let value = serde_json::to_value(event)
(B[m[32m+            .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
(B[m         let mut events = self.events.lock().unwrap();
         events.push(value);
         Ok(())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/crates/phenotype-test-utils/src/lib.rs:202:
     async fn publish_batch<E: serde::Serialize>(&self, events: &[E]) -> Result<()> {
         let mut store = self.events.lock().unwrap();
         for event in events {
[31m-            let value = serde_json::to_value(event).map_err(|e| PhenotypeError::serialization(e.to_string()))?;
(B[m[32m+            let value = serde_json::to_value(event)
(B[m[32m+                .map_err(|e| PhenotypeError::serialization(e.to_string()))?;
(B[m             store.push(value);
         }
         Ok(())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/melosviz/desktop/src-tauri/src/commands.rs:408:
     #[test]
     fn test_playback_control_seek() {
         let state = std::sync::Mutex::new(PlaybackState::default());
[31m-        let result = playback_control_inner(
(B[m[31m-            PlaybackAction::Seek { position_ms: 5000 },
(B[m[31m-            &state,
(B[m[31m-        );
(B[m[32m+        let result = playback_control_inner(PlaybackAction::Seek { position_ms: 5000 }, &state);
(B[m         assert!(result.ok);
         let data = result.data.unwrap();
         assert_eq!(data.current_time_ms, 5000);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/melosviz/desktop/src-tauri/src/commands.rs:424:
             "empty": [],
             "missing": null,
         });
[31m-        assert_eq!(
(B[m[31m-            extract_string_array(&json, "note_set"),
(B[m[31m-            vec!["C", "D", "E"]
(B[m[31m-        );
(B[m[32m+        assert_eq!(extract_string_array(&json, "note_set"), vec!["C", "D", "E"]);
(B[m         assert!(extract_string_array(&json, "empty").is_empty());
         assert!(extract_string_array(&json, "missing").is_empty());
         assert!(extract_string_array(&json, "nonexistent").is_empty());
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/melosviz/desktop/src-tauri/src/menu.rs:59:
         ])
         .expect("Failed to append Help items");
 
[31m-    menu.append_items(&[
(B[m[31m-        &file_menu,
(B[m[31m-        &edit_menu,
(B[m[31m-        &view_menu,
(B[m[31m-        &help_menu,
(B[m[31m-    ])
(B[m[31m-    .expect("Failed to append submenus");
(B[m[32m+    menu.append_items(&[&file_menu, &edit_menu, &view_menu, &help_menu])
(B[m[32m+        .expect("Failed to append submenus");
(B[m 
     menu
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/melosviz/desktop/src-tauri/src/main.rs:5:
 
 #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
 
[31m-use melosviz_desktop_lib::PlaybackState;
(B[m use melosviz_desktop_lib::menu::build_menu;
[32m+use melosviz_desktop_lib::PlaybackState;
(B[m use std::sync::Mutex;
 
 fn main() {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e/src/main.rs:146:
     let record = &chain.records[0];
     let assertion4 = record.record_type == "wallet.grant_credit";
     results.record_assertion(assertion4);
[31m-    println!("    Audit record_type == 'wallet.grant_credit': {}", assertion4);
(B[m[32m+    println!(
(B[m[32m+        "    Audit record_type == 'wallet.grant_credit': {}",
(B[m[32m+        assertion4
(B[m[32m+    );
(B[m 
     // Assert chain verifies (no tampering).
     let assertion5 = chain.verify().is_ok();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e/src/main.rs:206:
     let record = &chain.records[0];
     let assertion5 = record.record_type == "wallet.streak_increment";
     results.record_assertion(assertion5);
[31m-    println!("    Audit record_type == 'wallet.streak_increment': {}", assertion5);
(B[m[32m+    println!(
(B[m[32m+        "    Audit record_type == 'wallet.streak_increment': {}",
(B[m[32m+        assertion5
(B[m[32m+    );
(B[m 
     // Assert chain verifies.
     let assertion6 = chain.verify().is_ok();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e/src/main.rs:293:
     results.record_assertion(assertion6);
     println!("    Final wallet balance == 15: {}", assertion6);
 
[31m-    let scenario_passed = assertion1
(B[m[31m-        && assertion2
(B[m[31m-        && assertion3
(B[m[31m-        && assertion4
(B[m[31m-        && assertion5
(B[m[31m-        && assertion6;
(B[m[32m+    let scenario_passed =
(B[m[32m+        assertion1 && assertion2 && assertion3 && assertion4 && assertion5 && assertion6;
(B[m     results.record_scenario("audit_chain_verification", scenario_passed);
 
     Ok(())
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e/src/main.rs:371:
     if let Err(e) = scenario_focus_session_completed(&mut results) {
         eprintln!("Scenario 2 error: {}", e);
         results.scenarios_failed += 1;
[31m-        results.failures.push("focus_session_completed: error".to_string());
(B[m[32m+        results
(B[m[32m+            .failures
(B[m[32m+            .push("focus_session_completed: error".to_string());
(B[m     }
 
     if let Err(e) = scenario_audit_chain_verification(&mut results) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e/src/main.rs:378:
         eprintln!("Scenario 3 error: {}", e);
         results.scenarios_failed += 1;
[31m-        results.failures.push("audit_chain_verification: error".to_string());
(B[m[32m+        results
(B[m[32m+            .failures
(B[m[32m+            .push("audit_chain_verification: error".to_string());
(B[m     }
 
     if let Err(e) = scenario_event_normalization(&mut results) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tests/e2e/src/main.rs:384:
         eprintln!("Scenario 4 error: {}", e);
         results.scenarios_failed += 1;
[31m-        results.failures.push("event_normalization: error".to_string());
(B[m[32m+        results
(B[m[32m+            .failures
(B[m[32m+            .push("event_normalization: error".to_string());
(B[m     }
 
     let elapsed = start.elapsed();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/disk_check.rs:40:
             "DISK BUDGET EXCEEDED: {} GB available, {} GB required (min). \
              Please run 'target-pruner --prune' or manual cleanup.\n\n\
              Command: /repos/FocalPoint/target/release/target-pruner --prune --verbose",
[31m-            available_gb, required_gb
(B[m[32m+            available_gb,
(B[m[32m+            required_gb
(B[m         ));
     }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/disk_check.rs:56:
         cmd.arg("--verbose");
     }
 
[31m-    let status = cmd.status()
(B[m[31m-        .map_err(|e| anyhow!("Failed to run disk-check binary: {}. Ensure it is in PATH.", e))?;
(B[m[32m+    let status = cmd.status().map_err(|e| {
(B[m[32m+        anyhow!(
(B[m[32m+            "Failed to run disk-check binary: {}. Ensure it is in PATH.",
(B[m[32m+            e
(B[m[32m+        )
(B[m[32m+    })?;
(B[m 
     match status.code() {
         Some(0) => Ok(()),
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/disk_check.rs:97:
         // This test will pass on any system with >20GB free
         let result = check_disk_space(1);
         if let Err(e) = result {
[31m-            eprintln!("Disk check warning (expected in constrained environments): {}", e);
(B[m[32m+            eprintln!(
(B[m[32m+                "Disk check warning (expected in constrained environments): {}",
(B[m[32m+                e
(B[m[32m+            );
(B[m         }
     }
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/lib.rs:43:
     pub fn from_file(path: &PathBuf) -> Result<Self> {
         let content = fs::read_to_string(path)
             .map_err(|e| anyhow!("Failed to read orchestration.toml: {}", e))?;
[31m-        toml::from_str(&content)
(B[m[31m-            .map_err(|e| anyhow!("Failed to parse orchestration.toml: {}", e))
(B[m[32m+        toml::from_str(&content).map_err(|e| anyhow!("Failed to parse orchestration.toml: {}", e))
(B[m     }
 
[31m-
(B[m     pub fn validate_non_overlapping(&self) -> Result<()> {
         let mut seen_files: HashMap<String, String> = HashMap::new();
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/lib.rs:58:
 
                 for entry in expanded {
                     let path = entry.map_err(|e| anyhow!("Glob expansion error: {}", e))?;
[31m-                    let path_str = path
(B[m[31m-                        .to_string_lossy()
(B[m[31m-                        .to_string();
(B[m[32m+                    let path_str = path.to_string_lossy().to_string();
(B[m 
                     if let Some(existing_lane) = seen_files.get(&path_str) {
                         return Err(anyhow!(
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/lib.rs:116:
         if !path.exists() {
             return Ok(Self::new());
         }
[31m-        let content = fs::read_to_string(path)
(B[m[31m-            .map_err(|e| anyhow!("Failed to read tracker state: {}", e))?;
(B[m[31m-        serde_json::from_str(&content)
(B[m[31m-            .map_err(|e| anyhow!("Failed to parse tracker state: {}", e))
(B[m[32m+        let content =
(B[m[32m+            fs::read_to_string(path).map_err(|e| anyhow!("Failed to read tracker state: {}", e))?;
(B[m[32m+        serde_json::from_str(&content).map_err(|e| anyhow!("Failed to parse tracker state: {}", e))
(B[m     }
 
     #[allow(dead_code)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/lib.rs:126:
     pub fn update_lane(&mut self, lane_id: String, in_flight: bool) {
[31m-        self.lanes.entry(lane_id.clone())
(B[m[32m+        self.lanes
(B[m[32m+            .entry(lane_id.clone())
(B[m             .or_insert_with(|| LaneTracker {
                 lane_id: lane_id.clone(),
                 last_dispatch: None,
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/lib.rs:141:
             tracker.coverage_count += 1;
         }
     }
[31m-
(B[m }
 
 #[cfg(test)]
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/agent-orchestrator/src/lib.rs:228:
         state.update_lane("lane1".to_string(), true);
 
         let json = serde_json::to_string(&state).expect("Should serialize");
[31m-        let deserialized: TrackerState =
(B[m[31m-            serde_json::from_str(&json).expect("Should deserialize");
(B[m[32m+        let deserialized: TrackerState = serde_json::from_str(&json).expect("Should deserialize");
(B[m 
         assert_eq!(deserialized.lanes.len(), 1);
         assert!(deserialized.lanes["lane1"].in_flight);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/bench-guard/src/main.rs:45:
 struct BenchEntry {
     #[serde(rename = "mean_nanos")]
     mean_nanos: u64,
[31m-    #[serde(rename = "histogram_buckets_nanos", skip_serializing_if = "Option::is_none")]
(B[m[32m+    #[serde(
(B[m[32m+        rename = "histogram_buckets_nanos",
(B[m[32m+        skip_serializing_if = "Option::is_none"
(B[m[32m+    )]
(B[m     histogram_buckets_nanos: Option<Vec<u64>>,
 }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/bench-guard/src/main.rs:124:
         return Ok(BenchBaseline {
             tolerance_percent: 30,
             benches: [
[31m-                ("ir_hash/small".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 1_000_000, // 1ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("ir_hash/large".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 10_000_000, // 10ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("eval_tick".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 5_000_000, // 5ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("audit_verify/1k_tail".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 10_000_000, // 10ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("starlark_compile/small".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 50_000_000, // 50ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("starlark_compile/large".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 500_000_000, // 500ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("scheduler_packing/small".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 240_000, // 240µs
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("scheduler_packing/medium".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 940_000, // 940µs
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[31m-                ("scheduler_packing/large".to_string(), BenchEntry {
(B[m[31m-                    mean_nanos: 1_400_000, // 1.4ms
(B[m[31m-                    histogram_buckets_nanos: None,
(B[m[31m-                }),
(B[m[32m+                (
(B[m[32m+                    "ir_hash/small".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 1_000_000, // 1ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "ir_hash/large".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 10_000_000, // 10ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "eval_tick".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 5_000_000, // 5ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "audit_verify/1k_tail".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 10_000_000, // 10ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "starlark_compile/small".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 50_000_000, // 50ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "starlark_compile/large".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 500_000_000, // 500ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "scheduler_packing/small".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 240_000, // 240µs
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "scheduler_packing/medium".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 940_000, // 940µs
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m[32m+                (
(B[m[32m+                    "scheduler_packing/large".to_string(),
(B[m[32m+                    BenchEntry {
(B[m[32m+                        mean_nanos: 1_400_000, // 1.4ms
(B[m[32m+                        histogram_buckets_nanos: None,
(B[m[32m+                    },
(B[m[32m+                ),
(B[m             ]
             .iter()
             .cloned()
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/bench-guard/src/main.rs:209:
 fn format_text(results: &[BenchResult], _baseline: &BenchBaseline) -> String {
     let mut output = String::from("Benchmark Results:\n");
     for result in results {
[31m-        output.push_str(&format!("  {}: {:.2}ms\n", result.name, result.mean_nanos as f64 / 1_000_000.0));
(B[m[32m+        output.push_str(&format!(
(B[m[32m+            "  {}: {:.2}ms\n",
(B[m[32m+            result.name,
(B[m[32m+            result.mean_nanos as f64 / 1_000_000.0
(B[m[32m+        ));
(B[m     }
     output
 }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/bench-guard/src/main.rs:216:
 
 fn format_markdown(results: &[BenchResult], baseline: &BenchBaseline) -> String {
[31m-    let mut output = String::from("| Benchmark | Baseline (ns) | Current (ns) | Change | Status |\n");
(B[m[32m+    let mut output =
(B[m[32m+        String::from("| Benchmark | Baseline (ns) | Current (ns) | Change | Status |\n");
(B[m     output.push_str("|-----------|--------------|-------------|--------|--------|\n");
 
     let tolerance_factor = 1.0 + (baseline.tolerance_percent as f64 / 100.0);
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/bench-guard/src/main.rs:247:
         for (i, bucket) in result.histogram.iter().enumerate() {
             let bar_width = (*bucket / 100_000).min(50) as usize; // Max 50 chars
             let bar = "█".repeat(bar_width);
[31m-            output.push_str(&format!("  [{}ns-{}ns]  {}\n",
(B[m[31m-                if i == 0 { 0 } else { result.histogram[i-1] },
(B[m[32m+            output.push_str(&format!(
(B[m[32m+                "  [{}ns-{}ns]  {}\n",
(B[m[32m+                if i == 0 { 0 } else { result.histogram[i - 1] },
(B[m                 bucket,
                 bar
             ));
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/executor.rs:146:
         let content = std::fs::read_to_string(&plist_path)?;
 
         // Simple string replacement for version in plist
[31m-        let new_version_str = format!(
(B[m[31m-            "{}.{}.{}",
(B[m[31m-            version.major, version.minor, version.patch
(B[m[31m-        );
(B[m[32m+        let new_version_str = format!("{}.{}.{}", version.major, version.minor, version.patch);
(B[m         let new_content = regex::Regex::new(r"<string>\d+\.\d+\.\d+</string>")?
             .replace_all(&content, format!("<string>{}</string>", new_version_str))
             .to_string();
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/executor.rs:169:
             .output()?;
 
         if !output.status.success() {
[31m-            eprintln!(
(B[m[31m-                "  Warning: release-notes generation failed; proceeding with manual entry"
(B[m[31m-            );
(B[m[32m+            eprintln!("  Warning: release-notes generation failed; proceeding with manual entry");
(B[m             return Ok(());
         }
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/executor.rs:202:
             .output()?;
 
         Command::new("git")
[31m-            .args(["add", "apps/ios/FocalPoint/Sources/FocalPointApp/Info.plist"])
(B[m[32m+            .args([
(B[m[32m+                "add",
(B[m[32m+                "apps/ios/FocalPoint/Sources/FocalPointApp/Info.plist",
(B[m[32m+            ])
(B[m             .current_dir(&self.repo_root)
             .output()?;
 
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/executor.rs:264:
             return Ok(());
         }
 
[31m-        println!("  → Webhook URL: {}...", &webhook_url[..50.min(webhook_url.len())]);
(B[m[32m+        println!(
(B[m[32m+            "  → Webhook URL: {}...",
(B[m[32m+            &webhook_url[..50.min(webhook_url.len())]
(B[m[32m+        );
(B[m 
         Ok(())
     }
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/planner.rs:24:
 
 impl Plan {
     pub fn print(&self) {
[31m-        println!("┌─ Release Plan: {} ─────────────────────────────────┐", self.version);
(B[m[32m+        println!(
(B[m[32m+            "┌─ Release Plan: {} ─────────────────────────────────┐",
(B[m[32m+            self.version
(B[m[32m+        );
(B[m         println!("│");
         println!("│ 1. Git Tag:");
[31m-        println!("│    $ git tag -a {} -m 'FocalPoint {}'", self.git_tag, self.version);
(B[m[32m+        println!(
(B[m[32m+            "│    $ git tag -a {} -m 'FocalPoint {}'",
(B[m[32m+            self.git_tag, self.version
(B[m[32m+        );
(B[m         println!("│    $ git push origin {}", self.git_tag);
         println!("│");
         println!("│ 2. Version Bumps:");
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/planner.rs:114:
         }
 
         // iOS plist version
[31m-        let ios_plist = self.repo_root.join(
(B[m[31m-            "apps/ios/FocalPoint/Sources/FocalPointApp/Info.plist"
(B[m[31m-        );
(B[m[32m+        let ios_plist = self
(B[m[32m+            .repo_root
(B[m[32m+            .join("apps/ios/FocalPoint/Sources/FocalPointApp/Info.plist");
(B[m         if ios_plist.exists() {
             let plist_content = std::fs::read_to_string(&ios_plist)?;
             if let Some(old_plist_version) = extract_plist_version(&plist_content) {
Diff in /Users/kooshapari/CodeProjects/Phenotype/repos/tooling/release-cut/src/planner.rs:123:
                 bumps.push(VersionBump {
                     path: ios_plist.display().to_string(),
                     old_version: old_plist_version,
[31m-                    new_version: format!("{}.{}.{}", new_version.major, new_version.minor, new_version.patch),
(B[m[32m+                    new_version: format!(
(B[m[32m+                        "{}.{}.{}",
(B[m[32m+                        new_version.major, new_version.minor, new_version.patch
(B[m[32m+                    ),
(B[m                 });
             }
         }
EXIT=0
W3-15: cargo fmt fixed
