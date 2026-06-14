W6-01: Run actual cargo test (non-failing)
[1m[92m   Compiling[0m melosviz-desktop v0.0.12 (/Users/kooshapari/CodeProjects/Phenotype/repos/melosviz/desktop/src-tauri)
[1m[33mwarning[0m[1m: unused import: `json`[0m
 [1m[94m--> [0mcrates/focus-cli/tests/json_output_test.rs:5:18
  [1m[94m|[0m
[1m[94m5[0m [1m[94m|[0m use serde_json::{json, Value};
  [1m[94m|[0m                  [1m[33m^^^^[0m
  [1m[94m|[0m
  [1m[94m= [0m[1mnote[0m: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m[1m: unused import: `std::fs`[0m
 [1m[94m--> [0mcrates/focus-cli/tests/json_output_test.rs:6:5
  [1m[94m|[0m
[1m[94m6[0m [1m[94m|[0m use std::fs;
  [1m[94m|[0m     [1m[33m^^^^^^^[0m

[1m[33mwarning[0m: `focus-cli` (test "json_output_test") generated 2 warnings (run `cargo fix --test "json_output_test" -p focus-cli` to apply 2 suggestions)
[1m[33mwarning[0m[1m: unused variable: `client`[0m
   [1m[94m--> [0mcrates/connector-strava/src/api.rs:204:13
    [1m[94m|[0m
[1m[94m204[0m [1m[94m|[0m         let client = StravaClien[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^^[0m [1m[33mhelp: if this is intentional, prefix it with an underscore: `_client`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m: `connector-strava` (lib test) generated 1 warning (run `cargo fix --lib -p connector-strava --tests` to apply 1 suggestion)
[1m[33mwarning[0m[1m: unused variable: `pages`[0m
   [1m[94m--> [0mcrates/connector-notion/src/api.rs:276:13
    [1m[94m|[0m
[1m[94m276[0m [1m[94m|[0m         let pages = NotionPage::[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^[0m [1m[33mhelp: if this is intentional, prefix it with an underscore: `_pages`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m: `connector-notion` (lib test) generated 1 warning (run `cargo fix --lib -p connector-notion --tests` to apply 1 suggestion)
[1m[33mwarning[0m[1m: function `mock_github_pr_closed_event` is never used[0m
   [1m[94m--> [0mcrates/focus-rule-suggester/src/lib.rs:595:8
    [1m[94m|[0m
[1m[94m595[0m [1m[94m|[0m [1m[94m...[0mfn mock_github_pr_closed_event(dt[1m[94m...[0m
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m[1m: struct `MockAuditStore` is never constructed[0m
   [1m[94m--> [0mcrates/focus-rule-suggester/src/lib.rs:623:12
    [1m[94m|[0m
[1m[94m623[0m [1m[94m|[0m     struct MockAuditStore {
    [1m[94m|[0m            [1m[33m^^^^^^^^^^^^^^[0m

[1m[33mwarning[0m[1m: associated items `new`, `add_checkin`, and `add_grant` are never used[0m
   [1m[94m--> [0mcrates/focus-rule-suggester/src/lib.rs:628:12
    [1m[94m|[0m
[1m[94m627[0m [1m[94m|[0m     impl MockAuditStore {
    [1m[94m|[0m     [1m[94m-------------------[0m [1m[94massociated items in this implementation[0m
[1m[94m628[0m [1m[94m|[0m         fn new() -> Self {
    [1m[94m|[0m            [1m[33m^^^[0m
[1m[94m...[0m
[1m[94m634[0m [1m[94m|[0m         fn add_checkin(&mut self[1m[94m...[0m
    [1m[94m|[0m            [1m[33m^^^^^^^^^^^[0m
[1m[94m...[0m
[1m[94m647[0m [1m[94m|[0m         fn add_grant(&mut self, [1m[94m...[0m
    [1m[94m|[0m            [1m[33m^^^^^^^^^[0m

[1m[33mwarning[0m[1m: function `missing_celebrations_heuristic_detects_unCelebrated_tasks` should have a snake case name[0m
   [1m[94m--> [0mcrates/focus-rule-suggester/src/lib.rs:547:8
    [1m[94m|[0m
[1m[94m547[0m [1m[94m|[0m [1m[94m...[0mfn missing_celebrations_heuristic_detects_unCelebrated_tasks() {
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^[0m [1m[33mhelp: convert the identifier to snake case: `missing_celebrations_heuristic_detects_un_celebrated_tasks`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

[1m[33mwarning[0m[1m: unused variable: `task_store`[0m
   [1m[94m--> [0mcrates/focus-demo-seed/src/lib.rs:480:13
    [1m[94m|[0m
[1m[94m480[0m [1m[94m|[0m [1m[94m...[0m   let task_store = SqliteTas[1m[94m...[0m
    [1m[94m|[0m           [1m[33m^^^^^^^^^^[0m [1m[33mhelp: if this is intentional, prefix it with an underscore: `_task_store`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m[1m: variable `demo_task_count` is assigned to, but never used[0m
   [1m[94m--> [0mcrates/focus-demo-seed/src/lib.rs:484:13
    [1m[94m|[0m
[1m[94m484[0m [1m[94m|[0m [1m[94m...[0m   let mut demo_task_count = 0;
    [1m[94m|[0m           [1m[33m^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: consider using `_demo_task_count` instead

[1m[33mwarning[0m[1m: unused variable: `uuid`[0m
   [1m[94m--> [0mcrates/focus-demo-seed/src/lib.rs:489:31
    [1m[94m|[0m
[1m[94m489[0m [1m[94m|[0m [1m[94m...[0mf let Ok(uuid) = uuid::Uuid::[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^[0m [1m[33mhelp: if this is intentional, prefix it with an underscore: `_uuid`[0m

[1m[33mwarning[0m[1m: value assigned to `demo_task_count` is never read[0m
   [1m[94m--> [0mcrates/focus-demo-seed/src/lib.rs:491:25
    [1m[94m|[0m
[1m[94m491[0m [1m[94m|[0m [1m[94m...[0m   demo_task_count += 1;
    [1m[94m|[0m       [1m[33m^^^^^^^^^^^^^^^^^^^^[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mhelp[0m: maybe it is overwritten before being read?
    [1m[94m= [0m[1mnote[0m: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m: `focus-rule-suggester` (lib test) generated 4 warnings
[1m[33mwarning[0m: `focus-demo-seed` (lib test) generated 4 warnings (run `cargo fix --lib -p focus-demo-seed --tests` to apply 2 suggestions)
[1m[33mwarning[0m[1m: unused variable: `rules`[0m
   [1m[94m--> [0mcrates/focus-replay/src/lib.rs:386:13
    [1m[94m|[0m
[1m[94m386[0m [1m[94m|[0m         let rules: Vec<Rule> = v[1m[94m...[0m
    [1m[94m|[0m             [1m[33m^^^^^[0m [1m[33mhelp: if this is intentional, prefix it with an underscore: `_rules`[0m
    [1m[94m|[0m
    [1m[94m= [0m[1mnote[0m: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

[1m[33mwarning[0m: `focus-replay` (lib test) generated 1 warning (run `cargo fix --lib -p focus-replay --tests` to apply 1 suggestion)
[1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`test` profile [unoptimized + debuginfo]]8;;\ target(s) in 5.70s
[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/agent_orchestrator-e4d4707a8a76fd39)

running 5 tests
test tests::test_lane_parsing ... ok
test tests::test_non_overlap_detection ... ok
test tests::test_tracker_state_roundtrip ... ok
test tests::test_tracker_json_serialization ... ok
test disk_check::tests::test_disk_space_check ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

[1m[92m     Running[0m unittests src/main.rs (target/debug/deps/agent_orchestrator-b7814892c6d9eca5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/main.rs (target/debug/deps/bench_guard-9d265506354dbab5)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_canvas-aa83c4a3fdbdf948)

running 41 tests
test api::tests::parse_next_link_absent ... ok
test api::tests::parse_next_link_finds_next ... ok
test auth::tests::in_memory_token_store_roundtrip ... ok
test auth::tests::keychain_store_roundtrips_via_in_memory_secret_store ... ok
test auth::tests::keychain_store_surfaces_backend_errors_as_auth_errors ... ok
test auth::tests::builds_authorize_url ... ok
test auth::tests::token_is_expired_respects_skew ... ok
test auth::tests::token_legacy_json_without_issued_at_deserializes ... ok
test auth::tests::token_without_expiry_refreshes_after_one_hour_if_refresh_token_present ... ok
test events::tests::dedupe_keys_are_distinct_per_entity ... ok
test events::tests::due_soon_fires_within_window ... ok
test events::tests::due_soon_skips_when_outside_window ... ok
test events::tests::grade_posted_fires_only_when_scored_and_graded ... ok
test events::tests::map_assignment_falls_back_to_field_if_no_hint ... ok
test events::tests::map_assignment_prefers_course_id_hint_over_field ... ok
test events::tests::maps_announcement_posted ... ok
test events::tests::maps_assignment_with_due_date ... ok
test events::tests::maps_assignment_without_due_date_uses_now ... ok
test events::tests::maps_course_enrolled ... ok
test events::tests::maps_submission ... ok
test events::tests::overdue_fires_when_past_due_and_no_submission ... ok
test events::tests::overdue_skips_when_submission_exists_or_not_past_due ... ok
test events::tests::traces_reference_canvas_ids ... ok
test models::tests::parses_announcement_json ... ok
test models::tests::parses_announcement_minimal ... ok
test models::tests::parses_assignment_missing_optional ... ok
test models::tests::parses_assignment_with_explicit_course_id ... ok
test models::tests::parses_course_json ... ok
test models::tests::parses_submission ... ok
test tests::builder_scopes_override_applies ... ok
test tests::default_manifest_scopes_are_empty ... ok
test tests::manifest_declares_new_event_types ... ok
test api::tests::too_many_requests_defaults_when_retry_after_missing ... ok
test auth::tests::exchanges_code_against_mock ... ok
test api::tests::unauthorized_maps_to_auth_error ... ok
test api::tests::too_many_requests_honors_retry_after ... ok
test api::tests::list_announcements_hits_expected_endpoint ... ok
test auth::tests::refresh_preserves_refresh_token_when_missing ... ok
test api::tests::forbidden_with_rate_limit_body_maps_to_rate_limit ... ok
test api::tests::forbidden_without_rate_limit_body_maps_to_auth ... ok
test api::tests::lists_courses_and_follows_pagination ... ok

test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s

[1m[92m     Running[0m tests/integration.rs (target/debug/deps/integration-61bfe7652b9db102)

running 39 tests
test get_user_grades_401_unauthorized ... ok
test get_user_profile_401_unauthorized ... ok
test get_assignment_404_not_found ... ok
test get_conversation_happy_path ... ok
test get_quiz_submissions_happy_path ... ok
test get_course_progress_happy_path ... ok
test get_user_grades_happy_path ... ok
test get_course_progress_403_permission_denied ... ok
test get_assignment_single_detail ... ok
test health_unauthenticated_when_no_token ... ok
test health_healthy_when_self_returns_200 ... ok
test get_user_profile_happy_path ... ok
test list_discussion_topics_403_forbidden ... ok
test list_discussion_topics_happy_path ... ok
test list_discussion_entries_happy_path ... ok
test list_conversations_happy_path ... ok
test full_sync_emits_course_assignment_submission_events ... ok
test list_calendar_events_401_unauthorized ... ok
test list_calendar_events_happy_path ... ok
test list_group_memberships_403_requires_teacher ... ok
test list_files_happy_path ... ok
test list_group_memberships_happy_path ... ok
test list_module_items_happy_path ... ok
test list_modules_happy_path ... ok
test list_outcome_results_happy_path ... ok
test list_groups_happy_path ... ok
test list_outcomes_happy_path ... ok
test list_quizzes_happy_path ... ok
test list_rubric_assessments_happy_path ... ok
test list_rubrics_403_requires_teacher ... ok
test list_rubrics_happy_path ... ok
test list_planner_items_happy_path ... ok
test list_pages_happy_path ... ok
test list_planner_notes_happy_path ... ok
test pagination_cursor_is_surfaced ... ok
test list_students_403_requires_teacher_permission ... ok
test list_todo_happy_path ... ok
test sync_refreshes_on_401_then_succeeds ... ok
test list_students_happy_path ... ok

test result: ok. 39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

[1m[92m     Running[0m tests/integration_live.rs (target/debug/deps/integration_live-0e01d71846a3228b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_fitbit-8b010586e9850d03)

running 11 tests
test events::tests::map_heart_rate_to_events ... ok
test events::tests::map_steps_milestone ... ok
test models::tests::activity_from_json ... ok
test models::tests::heart_rate_from_json ... ok
test auth::tests::token_expiration_check ... ok
test events::tests::map_activities_to_events ... ok
test events::tests::map_sleep_to_events ... ok
test models::tests::sleep_from_json ... ok
test auth::tests::in_memory_token_store ... ok
test auth::tests::oauth2_auth_url ... ok
test api::tests::fitbit_client_construction ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_gcal-128d76d3c4c8a78d)

running 42 tests
test api::tests::batch_get_events_empty_ids ... ok
test api::tests::get_event_unauthorized ... ok
test api::tests::forbidden_with_rate_limit_body_maps_to_rate_limit ... ok
test api::tests::forbidden_permission_denied_maps_to_auth ... ok
test api::tests::lists_events_with_single_events_and_order ... ok
test api::tests::batch_get_events_forbidden ... ok
test api::tests::get_event_single_detail ... ok
test api::tests::expand_recurring_events_queries_instances ... ok
test api::tests::lists_calendar_list ... ok
test api::tests::batch_get_events_multiple ... ok
test api::tests::urlencode_escapes_nonalpha ... ok
test auth::tests::in_memory_token_store_roundtrip ... ok
test auth::tests::authorize_url_accepts_custom_scopes ... ok
test auth::tests::builds_authorize_url_with_offline_and_default_scope ... ok
test auth::tests::token_is_expired_respects_skew ... ok
test auth::tests::keychain_store_roundtrips_via_in_memory_secret_store ... ok
test auth::tests::token_json_roundtrip ... ok
test auth::tests::token_legacy_json_without_issued_at_deserializes ... ok
test api::tests::watch_channel_create_missing_env_returns_auth_error ... ok
test auth::tests::token_without_expiry_becomes_stale_after_one_hour_if_refreshable ... ok
test api::tests::too_many_requests_honors_retry_after ... ok
test api::tests::too_many_requests_defaults_when_retry_after_missing ... ok
test events::tests::all_day_event_marks_all_day_and_parses_midnight_utc ... ok
test events::tests::event_ended_none_when_no_end ... ok
test events::tests::dedupe_keys_are_distinct_per_entity ... ok
test events::tests::maps_calendar_subscribed ... ok
test api::tests::unauthorized_maps_to_auth_error ... ok
test events::tests::maps_event_ended_when_end_present ... ok
test events::tests::maps_timed_event_to_event_started ... ok
test models::tests::parses_calendar_list_minimal ... ok
test models::tests::parses_event_timed ... ok
test events::tests::trace_ref_points_at_event_id ... ok
test models::tests::parses_event_list_with_paging ... ok
test models::tests::parses_event_all_day ... ok
test auth::tests::exchanges_code_against_mock ... ok
test tests::manifest_declares_event_types ... ok
test tests::default_manifest_has_calendar_readonly_scope ... ok
test tests::manifest_entity_types_include_calendar_and_event ... ok
test tests::builder_scopes_override_applies ... ok
test api::tests::watch_channel_stop_succeeds ... ok
test auth::tests::refresh_preserves_refresh_token_when_google_omits_it ... ok
test api::tests::watch_channel_create_succeeds ... ok

test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_github-6c3cf87481285512)

running 32 tests
test api::tests::parse_next_link_absent ... ok
test api::tests::parse_next_link_finds_next ... ok
test events::tests::dedupe_key_is_stable_and_namespaced ... ok
test events::tests::drops_unsupported_event_types ... ok
test api::tests::rate_limit_headers_parse ... ok
test events::tests::drops_unsupported_pr_review_actions ... ok
test events::tests::maps_issue_closed ... ok
test events::tests::maps_issue_comment_and_create ... ok
test events::tests::maps_pr_merged_vs_closed ... ok
test events::tests::maps_pr_opened ... ok
test events::tests::maps_pr_review_requested ... ok
test events::tests::maps_pr_review_submitted ... ok
test events::tests::maps_push_event ... ok
test events::tests::trace_ref_points_at_github ... ok
test tests::health_rate_limited_when_primary_quota_exhausted ... ok
test tests::health_unauthenticated_on_401 ... ok
test api::tests::get_pull_request_succeeds ... ok
test api::tests::list_check_runs_succeeds ... ok
test api::tests::graphql_succeeds ... ok
test api::tests::list_workflow_runs_succeeds ... ok
test api::tests::graphql_unauthorized ... ok
test api::tests::list_user_repos_succeeds ... ok
test api::tests::list_my_issues_succeeds ... ok
test tests::manifest_declares_contribution_event_types ... ok
test api::tests::get_pull_request_forbidden ... ok
test tests::token_serde_roundtrip_preserves_secret ... ok
test tests::token_debug_redacts_secret ... ok
test webhook::tests::test_github_webhook_push_event ... ok
test webhook::tests::test_github_webhook_pull_request_merged ... ok
test webhook::tests::test_github_webhook_invalid_json ... ok
test tests::sync_unauthorized_when_no_token ... ok
test tests::sync_happy_path_maps_events_and_follows_pagination ... ok

test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_linear-e60e459ffcffd6b0)

running 26 tests
test api::tests::linear_api_base_url ... ok
test api::tests::parse_issue_with_different_states ... ok
test api::tests::parse_multiple_issues ... ok
test api::tests::parse_empty_issue_list ... ok
test api::tests::parse_issue_schema_validation ... ok
test api::tests::parse_issue_identifier_formats ... ok
test api::tests::parse_issue_with_cursor_metadata ... ok
test auth::tests::linear_auth_bearer_header ... ok
test auth::tests::linear_auth_long_token_string ... ok
test auth::tests::linear_auth_with_oauth_token ... ok
test auth::tests::linear_auth_with_pat_token ... ok
test auth::tests::in_memory_token_store_set_get ... ok
test auth::tests::token_store_initial_none ... ok
test auth::tests::token_store_multiple_keys_sequential ... ok
test auth::tests::token_store_replace_token ... ok
test events::tests::map_issues_closed ... ok
test events::tests::map_issues_created ... ok
test models::tests::issue_from_json ... ok
test auth::tests::token_store_arc_clone ... ok
test tests::linear_manifest_has_events ... ok
test tests::test_auth_strategy_is_apikey ... ok
test tests::test_manifest_event_types ... ok
test tests::test_manifest_entity_types ... ok
test tests::test_manifest_metadata ... ok
test api::tests::linear_client_construction ... ok
test tests::linear_builder_constructs ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_notion-7cbd6d2a0133c8fd)

running 26 tests
test api::tests::notion_api_base_url ... ok
test api::tests::parse_empty_page_list ... ok
test api::tests::parse_page_response ... ok
test api::tests::parse_paginated_response ... ok
test api::tests::parse_task_with_different_statuses ... ok
test auth::tests::in_memory_token_store_initial_empty ... ok
test api::tests::parse_multiple_pages ... ok
test api::tests::parse_task_response ... ok
test auth::tests::in_memory_token_store_overwrite ... ok
test auth::tests::in_memory_token_store_set_get ... ok
test auth::tests::notion_auth_bearer_header ... ok
test auth::tests::notion_auth_bearer_header_escaping ... ok
test auth::tests::notion_auth_empty_token ... ok
test auth::tests::notion_auth_long_token ... ok
test auth::tests::token_store_multiple_sequences ... ok
test models::tests::page_from_json ... ok
test tests::test_notion_auth_strategy_is_apikey ... ok
test events::tests::map_tasks_to_events_only_completed ... ok
test tests::notion_manifest_has_events ... ok
test tests::test_notion_entity_types ... ok
test models::tests::task_from_json ... ok
test events::tests::map_pages_to_events ... ok
test tests::test_notion_manifest_metadata ... ok
test auth::tests::token_store_concurrent_access ... ok
test tests::notion_builder_constructs ... ok
test api::tests::notion_client_construction ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_readwise-64ef46a7e62cf5c2)

running 26 tests
test api::tests::readwise_api_base_url ... ok
test api::tests::parse_empty_highlights_list ... ok
test api::tests::parse_highlights_with_different_colors ... ok
test api::tests::parse_article_reading_progress ... ok
test api::tests::parse_article_response ... ok
test auth::tests::readwise_auth_bearer_format ... ok
test auth::tests::readwise_auth_bearer_header ... ok
test api::tests::parse_highlight_response ... ok
test auth::tests::readwise_auth_special_chars ... ok
test api::tests::parse_multiple_articles ... ok
test auth::tests::readwise_auth_numeric_token ... ok
test models::tests::article_from_json ... ok
test models::tests::highlight_from_json ... ok
test events::tests::map_articles_to_events ... ok
test events::tests::map_highlights_to_events ... ok
test tests::test_readwise_auth_strategy_is_apikey ... ok
test tests::test_readwise_manifest_metadata ... ok
test tests::readwise_manifest_has_events ... ok
test tests::test_readwise_entity_types ... ok
test auth::tests::token_store_empty_initial ... ok
test auth::tests::token_store_sequential_updates ... ok
test auth::tests::in_memory_token_store_set_get ... ok
test auth::tests::token_store_update_token ... ok
test auth::tests::token_store_shared_access ... ok
test tests::readwise_builder_constructs ... ok
test api::tests::readwise_client_construction ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_strava-1bbe0cba152d95cd)

running 26 tests
test api::tests::parse_activity_with_elevation ... ok
test api::tests::parse_minimal_activity ... ok
test api::tests::parse_multiple_activities ... ok
test api::tests::strava_api_base_constant ... ok
test api::tests::activity_different_sports ... ok
test api::tests::parse_activity_response ... ok
test auth::tests::oauth_authorize_url_escaping ... ok
test auth::tests::oauth_authorize_url_generation ... ok
test auth::tests::token_expiration_check ... ok
test auth::tests::keychain_store_stub_behavior ... ok
test auth::tests::in_memory_token_store ... ok
test auth::tests::token_expiry_buffer ... ok
test auth::tests::token_not_expired_recent ... ok
test auth::tests::token_store_delete ... ok
test auth::tests::token_store_overwrite ... ok
test events::tests::dedupe_key_generation ... ok
test events::tests::map_activities_to_events ... ok
test events::tests::map_pr_earned_to_events ... ok
test events::tests::multiple_prs_single_activity ... ok
test models::tests::parse_activity_from_json ... ok
test tests::test_in_memory_token_store ... ok
test tests::test_manifest_validation ... ok
test tests::test_oauth2_scopes ... ok
test tests::test_token_expiration ... ok
test api::tests::client_constructor ... ok
test tests::test_builder_creates_connector ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/connector_testkit-57aebffde41831fe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m tests/dedupe_contract.rs (target/debug/deps/dedupe_contract-b19d58fb0650cd8a)

running 1 test
test connector_dedupe_contract_in_memory ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_always_on-c2cc8112a0d01dfe)

running 6 tests
test tests::test_sleep_hour_suppression ... ok
test tests::test_determinism_with_fixed_clock ... ok
test tests::test_cross_hour_bucketing ... ok
test tests::test_productive_hours_filtering ... ok
test tests::test_rolling_average_calculation ... ok
test tests::test_new_predictor_no_history ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_asset_fetcher-0d8debd0faf0a303)

running 10 tests
test tests::test_parse_asset_line_empty ... ok
test tests::test_parse_asset_line_invalid_url ... ok
test tests::test_parse_asset_line_missing_value ... ok
test tests::test_parse_asset_line_with_flags ... ok
test tests::test_ffmpeg_command_minimal ... ok
test tests::test_parse_asset_line_comment ... ok
test tests::test_parse_sound_sources_ignores_comments_and_headers ... ok
test tests::test_ffmpeg_command_with_trim_and_gain ... ok
test tests::test_cache_hit_no_cache ... ok
test tests::test_parse_asset_line_minimal ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/main.rs (target/debug/deps/focalpoint_fetch_assets-4574487a1f80f1ac)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_audit-4a1f796b43ac873f)

running 20 tests
test canonical::tests::canonicalize_preserves_array_order ... ok
test canonical::tests::canonicalize_sorts_top_level_keys ... ok
test tests::append_mutation_builds_record_from_payload ... ok
test tests::audit_sink_in_memory_appends ... ok
test tests::canonicalization_makes_hash_key_order_independent ... ok
test tests::capturing_sink_captures_record ... ok
test canonical::tests::canonicalize_handles_primitives_and_null ... ok
test canonical::tests::canonicalize_sorts_nested_keys ... ok
test tests::compute_hash_is_deterministic_across_calls ... ok
test tests::append_mutation_chains_prev_hash ... ok
test tests::empty_chain_verify_returns_empty ... ok
test tests::noop_audit_sink_does_nothing_but_succeeds ... ok
test tests::head_hash_advances_with_each_append ... ok
test tests::single_record_chain_verifies ... ok
test tests::penalty_mutations_are_append_only ... ok
test tests::wallet_mutations_are_append_only ... ok
test tests::in_memory_store_append_and_head_hash ... ok
test tests::prev_hash_break_detected ... ok
test tests::tamper_detection_via_payload_mutation ... ok
test tests::hundred_record_chain_builds_and_verifies ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_backup-1519eaee207e4a69)

running 10 tests
test tests::test_restore_config_merge_mode ... ok
test tests::test_restore_report_total ... ok
test tests::test_backup_config_default ... ok
test tests::test_version_mismatch_detection ... ok
test manifest::tests::test_manifest_new ... ok
test tests::test_backup_error_display ... ok
test tests::test_backup_manifest_serialization ... ok
test manifest::tests::test_content_section_serialization ... ok
test tar_builder::tests::test_tar_empty_manifest ... ok
test tar_builder::tests::test_tar_round_trip ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_calendar-dc793fd7d07aab1e)

running 4 tests
test tests::deletion_clears_event ... ok
test tests::in_memory_roundtrip_create_and_list ... ok
test tests::overlapping_events_returned_sorted ... ok
test tests::list_filters_by_range ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_ci_watcher-a1cc5c54a02c9389)

running 8 tests
test tests::test_format_ci_result_failure ... ok
test tests::test_format_ci_result_output_truncation ... ok
test tests::test_format_ci_result_success ... ok
test tests::test_parse_git_sha ... ok
test tests::test_parse_git_sha_empty ... ok
test tests::test_parse_git_sha_multiline ... ok
test tests::test_poll_result_struct ... ok
test tests::test_sandbox_creation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/main.rs (target/debug/deps/focus_ci_watcher-95b4135d0409d158)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_cli-8d76beefd82a41eb)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/main.rs (target/debug/deps/focus-2a47fc5b4228114b)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m tests/json_output_test.rs (target/debug/deps/json_output_test-24fa506ea33b3dc5)

running 16 tests
test test_audit_head_json ... ok
test test_audit_tail_json ... ok
test test_audit_verify_json ... ok
test test_focus_complete_json ... ok
test test_focus_start_json ... ok
test test_release_notes_json ... ignored, TBD: see test fixture in tests/fixtures/release-notes/
test test_templates_list_json ... ignored, TBD: see test fixture in tests/fixtures/templates/
test test_json_flag_short_form ... ok
test test_json_output_not_default ... ok
test test_penalty_show_json ... ok
test test_tasks_add_json ... ok
test test_rules_list_json ... ok
test test_tasks_list_json ... ok
test test_wallet_spend_json ... ok
test test_wallet_balance_json ... ok
test test_wallet_grant_json ... ok

test result: ok. 14 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.01s

[1m[92m     Running[0m tests/release_notes_llm.rs (target/debug/deps/release_notes_llm-74dee375004296b2)

running 6 tests
test tests::test_release_notes_http_error_codes ... ok
test tests::test_release_notes_llm_malformed_response ... ok
test tests::test_release_notes_llm_response_parsing ... ok
test tests::test_release_notes_llm_prompt_construction ... ok
test tests::test_release_notes_synthesize_fallback_missing_env ... ok
test tests::test_release_notes_synthesize_with_env_var ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m tests/template_marketplace.rs (target/debug/deps/template_marketplace-14e3e86eed86b3f5)

running 11 tests
test tests::test_local_fallback_catalog_completeness ... ok
test tests::test_template_auth_token_env_var ... ok
test tests::test_template_pack_detail_structure ... ok
test tests::test_template_rate_invalid_rating ... ok
test tests::test_template_rate_offline_graceful ... ok
test tests::test_template_rating_request_structure ... ok
test tests::test_template_registry_url_env_var ... ok
test tests::test_template_rating_response_structure ... ok
test tests::test_template_search_local_fallback ... ok
test tests::test_template_search_result_structure ... ok
test tests::test_template_show_local_fallback ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_coaching-29e65103776e284d)

running 12 tests
test tests::bubble_prompt_mentions_voice_constraints ... ok
test tests::test_fr_ux_001_rule_firing_explanation ... ignored, TBD: feature spec not yet implemented
test tests::test_fr_ux_002_connector_auth_platform_native ... ignored, TBD: feature spec not yet implemented
test tests::test_fr_ux_003_penalty_escalation_visibility ... ignored, TBD: feature spec not yet implemented
test tests::test_fr_ux_004_streak_state_home_surface ... ignored, TBD: feature spec not yet implemented
test tests::rule_authoring_prompt_substitutes_schema ... ok
test tests::stub_returns_canned_then_wraps ... ok
test tests::guard_passes_through_when_unset ... ok
test tests::noop_provider_returns_none ... ok
test tests::rate_limit_caps_calls ... ok
test tests::kill_switch_forces_none_via_guard ... ok
test tests::stub_single_helper ... ok

test result: ok. 8 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_connectors-760cb13786a219be)

running 29 tests
test connector_trait_tests::test_fr_conn_004_canvas_oauth2_cursor_sync ... ok
test connector_trait_tests::manifest_schema_declares_auth_sync_capabilities_correctly ... ok
test connector_trait_tests::connector_manifest_declares_required_fields ... ok
test mcp_bridge::tests::manifest_shape_exposes_tier_and_health_indicators ... ok
test registry_tests::catalog_by_tier_filters ... ok
test registry_tests::mark_installed_flips_listing_flag ... ok
test mcp_bridge::tests::sync_returns_not_wired_error ... ok
test connector_trait_tests::connector_health_state_transitions_observable ... ok
test derived::tests::derived_transforms_combined_base_events_and_picks_max_cursor ... ok
test registry_tests::catalog_orders_by_tier_then_display_order ... ok
test mcp_bridge::tests::dedupe_prefix_uses_endpoint_and_field ... ok
test mcp_bridge::tests::tier_defaults_to_verified_when_deserializing_legacy_manifest ... ok
test registry_tests::register_with_same_id_replaces_listing ... ok
test signature_verifiers::tests::test_canvas_lti_expired_jwt ... ok
test signature_verifiers::tests::test_canvas_lti_aud_mismatch ... ok
test signature_verifiers::tests::test_canvas_lti_future_issued_jwt ... ok
test signature_verifiers::tests::test_canvas_lti_invalid_jwt_format ... ok
test signature_verifiers::tests::test_canvas_lti_missing_header ... ok
test signature_verifiers::tests::test_canvas_lti_iss_mismatch ... ok
test signature_verifiers::tests::test_gcal_channel_missing_header ... ok
test signature_verifiers::tests::test_gcal_channel_tampered_token ... ok
test signature_verifiers::tests::test_gcal_channel_valid_token ... ok
test signature_verifiers::tests::test_github_hmac_missing_header ... ok
test signature_verifiers::tests::test_github_hmac_tampered_body ... ok
test signature_verifiers::tests::test_github_hmac_valid_signature ... ok
test webhook_tests::handler_rejects_empty_body ... ok
test webhook_tests::registry_dispatches_to_matching_handler ... ok
test webhook_tests::registry_errors_when_no_handler ... ok
test webhook_tests::registry_is_reentrant_for_reads ... ok

test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_connectors_mock_familycontrols-5ed01ea20b16bd2b)

running 20 tests
test synthetic_events::tests::invalid_scenario_errors ... ok
test synthetic_events::tests::schedule_from_intervention_flow_scenario ... ok
test synthetic_events::tests::schedule_from_emergency_exit_scenario ... ok
test synthetic_events::tests::schedule_from_standard_day_scenario ... ok
test synthetic_events::tests::schedule_peek_and_dequeue ... ok
test synthetic_events::tests::synthetic_event_kind_names_correct ... ok
test tests::enqueue_and_peek_events ... ok
test tests::health_always_healthy ... ok
test synthetic_events::tests::emergency_exit_payload_correct ... ok
test tests::emergency_exit_event_generates_correctly ... ok
test tests::load_invalid_scenario_errors ... ok
test tests::mock_connector_manifest_correct ... ok
test time_source::tests::deterministic_downcast ... ok
test tests::intervention_triggered_event_includes_payload ... ok
test tests::mock_connector_uses_deterministic_time ... ok
test tests::multiple_syncs_drain_queue ... ok
test time_source::tests::real_time_source_returns_current_time ... ok
test time_source::tests::deterministic_time_source_advance ... ok
test tests::sync_generates_events_from_schedule ... ok
test time_source::tests::deterministic_time_source_manual_set ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_crypto-3fcaf88238540e83)

running 6 tests
test keychain::tests::keychain_live_roundtrip ... ignored, hits the real macOS/iOS keychain; run manually on a dev Mac
test keychain::tests::default_secure_store_returns_some_box ... ok
test keychain::tests::in_memory_delete_missing_is_ok ... ok
test keychain::tests::in_memory_isolates_keys ... ok
test keychain::tests::in_memory_roundtrip ... ok
test keychain::tests::null_store_errors_loudly ... ok

test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_demo_seed-72bd04ccde11f5bb)

running 7 tests
test tests::test_seed_demo_rituals ... ok
test tests::test_seed_demo_connectors ... ok
test tests::test_seed_demo_rules ... ok
test tests::test_seed_demo_tasks ... ok
test tests::test_seed_demo_wallet_audit ... ok
test tests::test_seed_demo_data ... ok
test tests::test_reset_demo_data ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_domain-857b3ffe4706a2e3)

running 7 tests
test rigidity_tests::default_is_hard ... ok
test rigidity_tests::hard_is_hard ... ok
test rigidity_tests::semi_credit_cost_extraction ... ok
test rigidity_tests::semi_friction_delay_and_ping ... ok
test rigidity_tests::rigidity_roundtrips_serde ... ok
test rigidity_tests::semi_tier_bump_and_streak_risk ... ok
test rigidity_tests::soft_is_soft ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_entitlements-aaf4fb7ca458e110)

running 4 tests
test tests::tier_matrix_tests::test_feature_matrix_all_combinations ... ok
test tests::tier_matrix_tests::test_focus_break_duration_boundaries ... ok
test tests::tier_matrix_tests::test_rule_task_boundaries ... ok
test tests::tier_matrix_tests::test_subscription_expiry_behavior ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_errors-7b9a0e4b2befd3b0)

running 7 tests
test tests::test_config_error ... ok
test tests::test_connector_error ... ok
test tests::test_crypto_error ... ok
test tests::test_event_error ... ok
test tests::test_focus_error_alias ... ok
test tests::test_result_alias ... ok
test tests::test_transpilation_error ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_eval-6806df2fc2ecae8e)

running 23 tests
test batched::tests::zero_event_batch_returns_empty_report ... ok
test tests::audit_records_rule_fired_entries ... ok
test tests::event_matching_rule_produces_wallet_grant ... ok
test batched::tests::determinism_property_cursor_idempotence ... ok
test tests::intervention_action_emits_audit_with_severity ... ok
test tests::intervention_urgent_maps_to_rule_fired_priority ... ok
test tests::notify_action_emits_notify_dispatched_audit_line ... ok
test tests::cooldown_suppresses_second_fire_within_window ... ok
test tests::emergency_exit_rate_limit_blocks_second_fire_within_hour ... ok
test tests::dedupe_prevents_double_grant_for_same_event ... ok
test tests::emergency_exit_action_emits_session_completed ... ok
test tests::decision_sink_receives_fired_decisions ... ok
test tests::scheduled_unlock_window_action_emits_activation ... ok
test tests::session_double_start_deduped ... ok
test tests::session_concurrent_start_evaluated ... ok
test tests::cursor_persists_across_pipeline_instances ... ok
test tests::session_pause_idempotent ... ok
test tests::session_cancel_after_complete_noop ... ok
test tests::session_resumption_after_crash ... ok
test tests::session_timer_drift_duration_invariant ... ok
test tests::session_zero_duration_audited ... ok
test tests::session_very_long_no_overflow ... ok
test batched::tests::equivalence_property_sequential_vs_parallel ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_events-b9fb3d7c44d3f95f)

running 22 tests
test dedup::tests::normalize_json_keys_nested_object ... ok
test dedup::tests::normalize_json_keys_array_preserved ... ok
test dedup::tests::normalize_json_keys_simple_object ... ok
test tests::display_renders_canonical_and_custom ... ok
test tests::event_type_roundtrip_serde_custom ... ok
test dedup::tests::canonical_hash_deterministic_same_payload ... ok
test dedup::tests::canonical_hash_different_key_order_same_hash ... ok
test tests::event_type_roundtrip_serde_well_known ... ok
test dedup::tests::canonical_hash_different_payload_different_hash ... ok
test dedup::tests::canonical_hash_different_event_type_different_hash ... ok
test dedup::tests::canonical_hash_different_connector_different_hash ... ok
test tests::from_manifest_string_canonical_yields_well_known ... ok
test tests::from_manifest_string_unknown_becomes_custom_prefixed ... ok
test tests::validate_happy_path ... ok
test tests::validate_rejects_empty_connector_id ... ok
test tests::validate_rejects_empty_dedupe_key ... ok
test tests::validate_rejects_out_of_range_confidence ... ok
test tests::validate_rejects_time_order ... ok
test tests::well_known_from_canonical_rejects_garbage ... ok
test dedup::tests::noop_deduplicator_always_unseen ... ok
test dedup::tests::in_memory_deduplicator_independent_keys ... ok
test dedup::tests::in_memory_deduplicator_tracks_seen ... ok

test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_events_core-7199e79dd94808e0)

running 14 tests
test tests::test_clear_history ... ok
test tests::test_bus_event_from_normalized ... ok
test tests::test_deduplication ... ok
test tests::test_different_topics ... ok
test tests::test_filtered_subscription ... ok
test tests::test_history_backlog ... ok
test tests::test_is_duplicate_disabled ... ok
test tests::test_multiple_subscribers ... ok
test tests::test_publish_and_subscribe ... ok
test tests::test_publish_invalid_normalized ... ok
test tests::test_publish_normalized ... ok
test tests::test_publish_no_subscribers ... ok
test tests::test_subscription_id_unique ... ok
test tests::test_topic_count ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

[1m[92m     Running[0m unittests src/lib.rs (target/debug/deps/focus_ffi-6b5d3bd9490f5c88)

running 26 tests
test tests::host_backed_calendar_port_lists_and_parses_events ... ok
test tests::connect_canvas_rejects_bogus_instance_url ... ok
test tests::generate_bubble_none_when_no_provider ... ok
test tests::host_event_emit_rejects_bad_confidence ... ok
test tests::connect_github_rejects_empty_pat ... ok
test tests::generate_bubble_uses_injected_provider ... ok
test tests::mascot_surface_still_works ... ok
test tests::connect_canvas_errors_without_env_client_id ... ok
test tests::host_event_emit_rejects_malformed_payload_json ... ok
test tests::host_event_emit_rejects_empty_event_type ... ok
test tests::host_event_emit_happy_path_appends_and_audits ... ok
test tests::penalty_escalate_quote_and_audit_chain_grows ... ok
test tests::propose_rule_errors_when_no_provider ... ok
test tests::policy_empty_when_no_decisions_then_reflects_seeded_block ... ok
test tests::propose_rule_from_nl_via_ffi_returns_summary ... ok
test tests::set_calendar_host_swaps_port_atomically ... ok
test tests::rule_upsert_then_list_enabled ... ok
test tests::task_add_rejects_bad_inputs ... ok
test tests::sync_tick_with_no_connectors_is_noop ... ok
test tests::task_add_persists_deadline_and_priority ... ok
test tests::task_add_list_remove_round_trip ... ok
test tests::task_mark_done_transitions_status ... ok
test tests::templates_install_unknown_id_errors ... ok
test tests::templates_install_known_id_persists_rules ... ok
test tests::wallet_grant_then_spend_through_ffi ... ok
test tests::templates_list_bundled_returns_all_starter_packs ... FAILED

failures:

---- tests::templates_list_bundled_returns_all_starter_packs stdout ----

thread 'tests::templates_list_bundled_returns_all_starter_packs' (21921378) panicked at crates/focus-ffi/src/lib.rs:3344:9:
expected ≥4 bundled packs, got 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::templates_list_bundled_returns_all_starter_packs

test result: FAILED. 25 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

[1m[91merror[0m: test failed, to rerun pass `-p focus-ffi --lib`
EXIT=0
