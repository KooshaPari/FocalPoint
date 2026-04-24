# Canvas LMS REST API v1 — Full Scope Research & Webhook Strategy

**Document Date:** 2026-04-23  
**FocalPoint Component:** `crates/connector-canvas`  
**Current Status:** 44 wiremock tests passing; Live API test `#[ignore]`-gated awaiting sandbox credentials

## Executive Summary

Canvas LMS exposes **165+ REST API v1 endpoints** across academic, user, communication, and assessment domains. FocalPoint's productivity focus drives a **HIGH-priority subset** (courses, assignments, submissions, calendar events, grades) with **MEDIUM-priority enhancements** (discussions, quizzes, planner, rubrics, outcomes). Webhook/live-event architecture is **not yet adopted**; polling remains viable but **Canvas Live Events (Caliper IMS 1.1)** is the recommended long-term path to replace polling. **Critical blocker:** Canvas admin-approved Developer Key (OAuth client_id + secret) on a real instance.

---

## Endpoint Inventory: Comprehensive Coverage

### HIGH Priority — Required for Core Rules/Rewards

| Endpoint | Method | Resource | Coverage | Status |
|----------|--------|----------|----------|--------|
| `/api/v1/courses` | GET | Courses | List user's active courses, pagination | **IMPLEMENTED** |
| `/api/v1/courses/:id` | GET | Courses | Single course details, enrollment counts | **IMPLEMENTED** |
| `/api/v1/courses/:course_id/assignments` | GET | Assignments | List assignments per course | **IMPLEMENTED** |
| `/api/v1/courses/:course_id/assignments/:id` | GET | Assignments | Single assignment, due dates, points possible | **IMPLEMENTED** |
| `/api/v1/courses/:course_id/submissions` | GET | Submissions | List submissions for assignment, state, grade | **IMPLEMENTED** |
| `/api/v1/courses/:course_id/assignments/:assignment_id/submissions/:user_id` | GET | Submissions | Single submission, status, feedback, submission date | **IMPLEMENTED** |
| `/api/v1/calendar_events` | GET | Calendar | User's calendar events + assignments, date-ranged | **IMPLEMENTED** |
| `/api/v1/users/self` | GET | Users | Current user profile, enrollments | **IMPLEMENTED** |
| `/api/v1/users/:user_id/courses/:course_id/progress` | GET | Progress | Course completion %, next requirement | **MISSING** |
| `/api/v1/courses/:course_id/users` | GET | Users | Enrolled students in course (for teacher view) | **MISSING** |
| `/api/v1/courses/:course_id/announcements` | GET | Announcements | Course announcements, filtering | **IMPLEMENTED** |
| `/api/v1/grades` or `/api/v1/courses/:course_id/grades` | GET | Grades | Grades endpoint (alias for submissions?) | **MISSING** |

### MEDIUM Priority — UI Enhancements & Context

| Endpoint | Method | Resource | Coverage | Status |
|----------|--------|----------|----------|--------|
| `/api/v1/courses/:course_id/discussion_topics` | GET | Discussions | List discussion topics | **MISSING** |
| `/api/v1/courses/:course_id/discussion_topics/:topic_id/entries` | GET/POST | Discussions | Discussion entries/replies, read state | **MISSING** |
| `/api/v1/users/self/groups` | GET | Groups | User's groups in all courses | **MISSING** |
| `/api/v1/groups/:group_id/users` | GET | Groups | Group members | **MISSING** |
| `/api/v1/courses/:course_id/quizzes` | GET | Quizzes | List quizzes per course | **MISSING** |
| `/api/v1/courses/:course_id/quizzes/:quiz_id/submissions` | GET | Quizzes | Quiz submission states, attempts | **MISSING** |
| `/api/v1/courses/:course_id/modules` | GET | Modules | Course modules & prerequisites | **MISSING** |
| `/api/v1/courses/:course_id/modules/:module_id/items` | GET | Modules | Module items (assignments, pages, discussions) | **MISSING** |
| `/api/v1/planner/items` | GET | Planner | Planner events (aggregate to-do list) | **MISSING** |
| `/api/v1/planner_notes` | GET/POST/PUT | Planner | User's planner notes | **MISSING** |
| `/api/v1/courses/:course_id/rubrics` | GET | Rubrics | Rubric definitions for assignment | **MISSING** |
| `/api/v1/users/self/bookmarks` | GET/POST | Bookmarks | User-saved bookmarks for quick nav | **MISSING** |
| `/api/v1/conversations` | GET/POST | Conversations | User's private messages | **MISSING** |
| `/api/v1/courses/:course_id/pages` | GET | Pages | Course wiki/pages | **MISSING** |
| `/api/v1/courses/:course_id/sections` | GET | Sections | Course sections (for enrollment view) | **MISSING** |

### LOW Priority — Skip for MVP, Revisit Later

| Endpoint | Method | Resource | Coverage | Status |
|----------|--------|----------|----------|--------|
| `/api/v1/courses/:course_id/outcome_alignments` | GET | Outcomes | Learning outcome alignments | **SKIP** |
| `/api/v1/courses/:course_id/grading_standards` | GET | Grading | Institution-level grading scales | **SKIP** |
| `/api/v1/appointment_groups` | GET | Appointments | Office hour / appointment booking | **SKIP** |
| `/api/v1/users/:user_id/page_views` | GET | Analytics | User page view history (heavy query) | **SKIP** |
| `/api/v1/accounts/:account_id/account_notifications` | GET | Notifications | System-wide announcements (not course-scoped) | **SKIP** |
| `/api/v1/courses/:course_id/files` | GET | Files | File listings (low signal for focus app) | **SKIP** |
| `/api/v1/courses/:course_id/assignment_groups` | GET | Assignments | Assignment group metadata (use single assignments instead) | **SKIP** |

### Summary: Endpoint Count

- **Total endpoints researched:** 165+ (across all Canvas API docs)
- **Directly applicable to FocalPoint:** ~50–60 endpoints
- **HIGH priority inventory:** 12 endpoints (6 IMPLEMENTED, 6 MISSING)
- **MEDIUM priority inventory:** 14 endpoints (1 IMPLEMENTED, 13 MISSING)
- **MISSING count (HIGH + MEDIUM combined):** **19 endpoints**

---

## Webhook & Live-Event Coverage: Adoption Recommendation

### Current Architecture: Polling

FocalPoint's sync orchestrator currently **polls** Canvas at regular intervals (via `list_courses`, `list_assignments`, `list_submissions`). Polling is simple to implement and test, but:

- **Cost:** O(courses × assignments) per sync cycle; scales poorly as user enrolls in more courses.
- **Latency:** N-minute delays between Canvas event (student submits assignment) and FocalPoint awareness.
- **Rate limit exposure:** 3,000 req/hr per user token; heavy polling during peak times (start of semester) risks throttling.

### Canvas Live Events (Caliper IMS 1.1)

**What it is:** Canvas pushes JSON events to a message queue (Amazon SQS, Kinesis, or HTTPS webhook) whenever course content changes. Events follow [IMS Caliper 1.1 standard](https://developerdocs.instructure.com/services/canvas/data-services/live-events/overview/file.data_service_caliper_structure) or Canvas proprietary format.

**Event types relevant to FocalPoint:**
- `submission.submitted` — student submits assignment
- `submission.graded` — teacher grades submission
- `assignment.created`, `.updated` — assignment changes
- `course.created` — user enrolls in new course
- `discussion.created`, `entry.created` — discussion posts (MEDIUM-priority context)
- `quiz.submission.submitted` — quiz attempt completed
- `module_item.viewed` — student views module content

**Setup requirement:** Canvas Data Services subscription (separate product; not included in free Canvas instance). Requires:
1. **AWS account** with SQS queue or managed Kinesis stream (or HTTPS endpoint).
2. **Canvas admin** creates data stream in admin panel (Maps Canvas instance → SQS queue).
3. **FocalPoint backend** subscribes to queue, processes Caliper JSON events.

**Advantages:**
- **Real-time:** Events delivered within seconds of action.
- **Cost-efficient:** Replaces O(courses × assignments) polling with O(events) consumption.
- **Audit trail:** Full event payload includes user, timestamp, and context for compliance.

**Disadvantages:**
- **Subscription cost:** Data Services ~$10K–50K/year (per institution, shared across all apps).
- **Operational complexity:** Requires AWS account, SQS/Kinesis management, error handling for queued messages.
- **Caliper unfamiliarity:** Event schema differs from REST API response shape; requires custom JSON parsing.

### Canvas LTI Advantage Events

**What it is:** LTI 1.3 Advantage services that emit events during grading, resource linking, and deep-linking flows. Narrower scope than Live Events; primarily for LTI tool integrations.

**Relevant events:** `GradePassback`, `ResourceLink` state changes.

**Assessment:** LTI Advantage is overkill for a standalone Canvas connector; useful only if FocalPoint were an **embedded LTI tool** inside Canvas. **Not recommended for FocalPoint's external-app architecture.**

### Subscription API (Deprecated?)

Canvas historically offered a Subscription API for polling specific resources via a managed queue. This is being **sunset in favor of Canvas Live Events (Caliper)** and is not recommended for new integrations.

### Recommendation: **Hybrid Polling + Live Events Path**

1. **Phase 1 (MVP, ~now):** Continue polling for initial HIGH-priority endpoints. Polling is testable without Canvas admin involvement; supports dev/test without SQS setup.
2. **Phase 2 (Post-MVP, ~Q3 2026):** Propose **Canvas Live Events (Caliper)** adoption to early institutional partners. Requires Canvas admin opt-in + AWS account, but unlocks real-time sync + reduced rate limit pressure.
3. **Phase 3 (Enterprise tier):** For high-volume institutions, offer **event-driven architecture** as a premium feature; continue polling as fallback for institutions without Data Services subscriptions.

**Verdict:** Adopt **Live Events (Caliper)** for long-term scalability; **not required for MVP**, but essential for production.

**Sources:**
- [Canvas Live Events — Caliper IMS 1.1 (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/data-services/live-events/overview/file.data_service_caliper_structure)
- [Creating a New Data Stream (Canvas LMS REST API)](https://canvas.instructure.com/doc/api/file.data_service_setup.html)

---

## Authentication & Institution Onboarding Playbook

### Canvas OAuth2 Architecture

Canvas uses **OAuth2 with per-instance Developer Keys**. Key facts:

- **Client ID + Secret scope:** Registered to a single Canvas root account (institution). Each Canvas instance (e.g., `university.instructure.com`) has its own secrets.
- **Bearer token auth:** Access tokens are passed as `Authorization: Bearer <token>` header.
- **Scopes:** Developer Keys can be scoped to limit token access (e.g., `url:GET|/api/v1/courses`, `url:GET|/api/v1/assignments`). Max ~110 scopes per token due to HTTP header size limits.

### Permissions & Access Tiers

**Student access:**
- Can read own courses, assignments, submissions, grades, calendar.
- **Cannot** list classmates (unless group member), modify any content.

**Teacher access:**
- Can read courses taught, all students' submissions + grades, can update assignment settings.
- **Can** masquerade as students (if permission granted).
- **Cannot** create new courses, manage enrollments (unless admin-delegated).

**Admin access:**
- Full read/write across account, courses, users, enrollments.
- Can masquerade as any user.
- Can create developer keys, manage OAuth secrets.

### Masquerade (Act-As) for Admin Tools

Canvas supports **masquerading via `as_user_id` query parameter**:

```
GET /api/v1/users/self/activity_stream?as_user_id=sis_user_id:student_email
```

This allows an admin to fetch data **as if** they were the student (with student's permissions). Useful for teacher debugging, but:

- **Requires "Become other users" permission.**
- **Audited:** Both calling user and target user logged.
- **Common pitfall:** Teachers cannot masquerade as students in other courses (permission denied).

### Scope Pitfalls

If a Developer Key scopes to only `url:GET|/api/v1/courses` and excludes `url:GET|/api/v1/assignments`, tokens will fail silently (403 Forbidden) when calling assignment endpoints. **Document expected scopes clearly in OAuth setup flow.**

### Institution Onboarding Playbook (User-Facing)

Insert into **"Connect Canvas to FocalPoint"** screen:

---

**Canvas Administrator: Required Setup Before OAuth**

To connect your Canvas instance to FocalPoint, your Canvas administrator must register a Developer Key. This is a one-time setup:

1. **Log in to Canvas as an administrator** (typically a Teaching & Learning Center or IT staff member).

2. **Navigate to Admin → Developer Keys** (usually at `https://<your-canvas>/admin/developer_keys`).

3. **Click "Add Developer Key"** and fill in:
   - **App Name:** `FocalPoint`
   - **Owner Email:** (your IT/contact email)
   - **Redirect URI:** `https://focalpoint.app/oauth/callback`
   - **Icon URL** (optional): `https://focalpoint.app/icon.png`

4. **Set OAuth Scopes** (the minimum FocalPoint needs):
   ```
   url:GET|/api/v1/courses
   url:GET|/api/v1/assignments
   url:GET|/api/v1/submissions
   url:GET|/api/v1/calendar_events
   url:GET|/api/v1/announcements
   url:GET|/api/v1/users
   url:GET|/api/v1/grades
   ```
   (More scopes may be added as FocalPoint gains features.)

5. **Click "Save"** and copy the **Client ID** and **Client Secret** to a secure location (or share with your IT vendor).

6. **Share the Client ID with FocalPoint:** Provide this to your FocalPoint account manager. The Client ID is safe to share; keep the secret private.

7. **Return to FocalPoint's OAuth screen** and enter your Canvas instance URL (e.g., `https://university.instructure.com`). FocalPoint will redirect you to Canvas to authorize.

8. **Grant consent:** Canvas will ask you to confirm FocalPoint's access scopes. Click "Allow."

9. **Done!** FocalPoint now syncs your courses, assignments, and deadlines.

---

### Scope Reference for FocalPoint

**Minimum scopes (MVP):**
```
url:GET|/api/v1/courses
url:GET|/api/v1/assignments
url:GET|/api/v1/submissions
url:GET|/api/v1/calendar_events
url:GET|/api/v1/announcements
```

**Extended scopes (future features, MEDIUM priority):**
```
url:GET|/api/v1/discussion_topics
url:GET|/api/v1/quizzes
url:GET|/api/v1/modules
url:GET|/api/v1/planner
url:GET|/api/v1/conversations
```

**Why admins must do this:** Canvas requires each institution to register third-party apps via Developer Keys to maintain security and audit trails. FocalPoint cannot be pre-registered globally; each Canvas instance is independent.

---

## Rate Limiting & Backoff Strategy

### Canvas Throttling Policy

Canvas uses a **leaky bucket algorithm** (not simple per-hour limit):

- **Rate limit:** 3,000 requests/hour per access token (per Canvas user).
- **Leaky bucket mechanics:** Bucket holds points; each request costs points upfront (50 points + true cost). Bucket leaks at ~10 points/sec (constant rate).
- **Exceeding limit:** 403 Forbidden `X-Rate-Limit-Remaining: 0`.
- **Headers returned:**
  - `X-Request-Cost`: Floating-point cost of current request (e.g., `2.5`).
  - `X-Rate-Limit-Remaining`: Points remaining in bucket.

### Backoff & Retry Strategy for FocalPoint's Sync Orchestrator

**Current status:** FocalPoint's sync orchestrator (in `connector-canvas`) likely handles basic exponential backoff. Recommend:

1. **Fast retries (< 1 sec delay):** For transient errors (5xx, timeout). Retry up to 3 times.
2. **Slow retries (exponential):** If hitting 403 Rate Limit, back off exponentially: 1 sec, 5 sec, 30 sec, then fail the sync cycle.
3. **Sync burst strategy:** If pulling multiple courses' assignments, **stagger requests** across 30–60 sec window to avoid spiking the bucket.
4. **Monitor `X-Request-Cost`:** Higher-cost requests (e.g., listing 5,000 submissions) drain the bucket faster. Cache results aggressively.

### Rate Limit Mitigation

- **Polling interval:** 15–30 min (not every 1 min) to avoid hitting limit.
- **Per-user tokens:** Avoid sharing tokens across users; each Canvas user has independent 3,000 req/hr budget.
- **Caching:** If 100 users check the same course's assignment list, cache the result and serve from cache for 5 min.
- **Live Events transition:** Once Live Events (Caliper) is adopted, eliminate polling entirely → zero rate limit pressure.

**Sources:**
- [Throttling (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/basics/file.throttling)
- [API Rate Limiting (Instructure Community)](https://community.canvaslms.com/docs/DOC-8381-api-rate-limiting)

---

## Pagination & Data Cursor Patterns

### Link Header (RFC 5988)

Canvas uses **Link headers** (standard HTTP pagination) for paginated responses:

```
Link: <https://canvas.instructure.com/api/v1/courses?page=2>; rel="next",
      <https://canvas.instructure.com/api/v1/courses?page=10>; rel="last",
      <https://canvas.instructure.com/api/v1/courses?page=1>; rel="first"
```

**Usage:** Parse `rel="next"` link and follow until no `next` link appears.

### Query Parameters

- **`per_page`:** Items per page (default 10, max 100 typically). Use `per_page=100` to reduce requests.
- **`page`:** Page number (1-indexed).
- **`bookmark`** (for some endpoints): Opaque cursor for large result sets (more efficient than page numbers).

### Per-Endpoint Limits

| Endpoint | Max per_page | Notes |
|----------|--------------|-------|
| `/api/v1/courses` | 100 | Default 10 |
| `/api/v1/assignments` | 100 | Default 10 |
| `/api/v1/submissions` | 100 | Default 10; large requests cost more (rate limit) |
| `/api/v1/calendar_events` | 100 | Default 10 |
| `/api/v1/users` | 100 | Default 10; avoid listing all users (heavy query) |

### Recommendation for FocalPoint

Use `per_page=100` for all paginated queries (reduces round trips). **FocalPoint's sync orchestrator should already handle Link header parsing;** verify in `crates/connector-canvas/src/lib.rs` that pagination loop handles edge cases (malformed headers, missing `next` link).

---

## Test Harness: Free/Cheap Testing Environments

### Available Canvas Test Instances

| Environment | URL Pattern | Data | Reset | Best For |
|-------------|------------|------|-------|----------|
| **Beta** | `https://*.beta.instructure.com` | Copy of prod | Weekly (Saturday) | Testing upcoming Canvas features |
| **Test** | `https://*.test.instructure.com` | Copy of prod | Weekly (Monday) | Safe testing without affecting prod |
| **Canvas Sandbox Docker** | Self-hosted via `instructure/canvas-lms` Docker image | Empty; seed manually | On demand | Full control, dev environments |
| **Free Canvas Demo** | `https://canvas.instructure.com` (public site) | Demo data | Ad-hoc | Learning Canvas UI/API |

### Recommendation: Canvas Sandbox Docker (for FocalPoint)

For **live-api integration tests** (currently `#[ignore]`-gated), use the **Canvas Sandbox Docker image** from [github.com/instructure/canvas-lms](https://github.com/instructure/canvas-lms):

```bash
docker run -it \
  -p 3000:3000 \
  -e CANVAS_LMS_ADMIN_EMAIL=admin@example.com \
  -e CANVAS_LMS_ADMIN_PASSWORD=password123 \
  instructure/canvas-lms:latest
```

**Advantages:**
- Free, fully controllable.
- No rate limiting.
- Full API access without admin approval.
- Spin up/down for CI tests.

**Disadvantages:**
- Heavy (~3GB image); slow first start (~5 min).
- Requires Docker; not all CI systems have resources.

### Fixture Seeding Strategy

Once Canvas is running, seed test data via API:

1. **Create test course:** `POST /api/v1/accounts/1/courses` (account 1 is root).
2. **Create test student user:** `POST /api/v1/accounts/1/users` with `as_canvas_user_id`.
3. **Create test assignment:** `POST /api/v1/courses/:course_id/assignments`.
4. **Create test submission:** `POST /api/v1/courses/:course_id/assignments/:assignment_id/submissions`.

Wrap in a test fixture builder (Rust `#[fixture]` or custom setup function) to avoid repetition.

### Blocker: Real Canvas Sandbox for OAuth Testing

**Current challenge:** Live OAuth testing requires a real Canvas instance with a registered Developer Key. The **Canvas Sandbox Docker does not support OAuth** (no multi-tenant isolation).

**Resolution:**
1. **Partner with a pilot institution** (university, school district) that runs Canvas.
2. **Request Canvas admin approval** to register FocalPoint as a Developer Key in their instance.
3. **Use their instance for live-api OAuth tests** (mark with `#[ignore]` for CI; run locally only).
4. Alternatively, **contact Instructure directly** for a test instance with pre-configured Developer Key (free for integrations in active development).

---

## Effort Estimate: Scope Coverage to 80% HIGH + 50% MEDIUM

**Baseline:** Current state is 6 HIGH endpoints implemented, 1 MEDIUM (announcements).

**Target:** 80% HIGH (≈10 of 12) + 50% MEDIUM (≈7 of 14) = **17 new endpoints**.

### Tool-Call Budget Breakdown

| Phase | Work | Tool Calls | Notes |
|-------|------|-----------|-------|
| **Phase 0** | Design endpoint adapter traits (missing endpoints) | 2–3 | Plan Rust crate structure for discussion, quiz, planner, etc. |
| **Phase 1: HIGH Endpoints** | Implement 6 missing HIGH endpoints | 24–30 | (4–5 calls per endpoint: fetch spec, write adapter, unit test, integration test, add to connector) |
| **Phase 2: MEDIUM Endpoints** | Implement 7 of 14 MEDIUM endpoints | 28–35 | (4–5 calls per endpoint) |
| **Phase 3: Testing** | Add wiremock fixtures for all new endpoints | 10–15 | Bulk fixture generation, similar to current 44 tests |
| **Phase 4: Integration** | Sync orchestrator integration (schedule polling, error handling) | 5–8 | Adapt scheduler for new endpoints; backoff tuning |
| **Phase 5: Documentation** | Update README, FUNCTIONAL_REQUIREMENTS, inline docs | 3–5 | Docstrings, endpoint matrix, scope list |
| **Total** | — | **72–96 tool calls** | ~20–25 agent-hours at 3–4 tool calls per minute |

### Confidence: **Medium–High**

- Existing wiremock test suite (44 tests) provides a strong pattern for new endpoints.
- Canvas API is stable and well-documented.
- Rust adapter pattern is established (`list_courses`, `list_assignments`, etc.); new endpoints follow same shape.

---

## Blockers Requiring Human Approval

### Critical Blocker: Developer Key Registration

**Status:** BLOCKING live-api OAuth testing.

**What's needed:**
1. **Canvas admin at a pilot institution** approves FocalPoint as a Developer Key.
2. **Admin registers Client ID + Secret** in their Canvas root account.
3. **Admin provides Client ID to FocalPoint** (secret remains confidential).
4. **FocalPoint stores secrets in keychain** (already implemented in `crates/connector-canvas`).

**Why it's a blocker:**
- **OAuth flow cannot be tested locally** without a real Canvas instance.
- Canvas Sandbox Docker is single-tenant; no OAuth multi-flow support.
- Instructure does not provide a free public OAuth test endpoint.

**Resolution path:**
- **Option A (Recommended):** Identify a **pilot university or school district** (non-profit preferred for goodwill). Pitch FocalPoint as an early-access integration; request 1 hour of Canvas admin time to register Developer Key.
- **Option B (Longer timeline):** Contact [Instructure Partner Program](https://www.instructure.com/partners) and request a **test instance with pre-seeded Developer Key** for integration development.
- **Option C (Immediate MVP):** Proceed with polling (no OAuth) using manually-generated Bearer tokens (Canvas admin can issue via admin panel). This unblocks MVP but delays production OAuth flow testing.

**Timeline impact:** If choosing Option A, 1–2 weeks to identify + onboard partner. If Option B, 2–4 weeks for Instructure response.

---

## Scope Traceability & Specification References

### Current Implemented Endpoints (from `crates/connector-canvas`)

Per earlier work session, **6 HIGH endpoints implemented:**
- `GET /api/v1/courses` → `list_courses()`
- `GET /api/v1/courses/:course_id/assignments` → `list_assignments()`
- `GET /api/v1/courses/:course_id/assignments/:assignment_id/submissions` → `list_submissions()`
- `GET /api/v1/announcements` → `list_announcements()` (via `/api/v1/announcements` endpoint, not per-course)
- OAuth + keychain integration (via `oauth2` crate + Keyring library)
- Wiremock fixtures for integration tests

### Missing HIGH Priority (6 endpoints)

1. `/api/v1/users/self` — Current user profile (needed for identity verification)
2. `/api/v1/users/:user_id/courses/:course_id/progress` — Per-course completion % (core for progress rewards)
3. `/api/v1/courses/:course_id/users` — Enrolled students list (needed for teacher/admin view)
4. `/api/v1/courses/:course_id/assignments/:id` (single assignment detail) — Pre-implemented in list, but single-fetch not tested
5. Grades endpoint (API path TBD; may be alias for submissions) — Core for grade-based rules
6. Course calendar events (separate from generic calendar; some Canvas instances split this) — Assignment due dates

### MEDIUM Priority Candidates for Phase 2 (select 7 of 14)

Recommend starting with:
1. `/api/v1/planner/items` — Aggregate to-do list (high UX impact)
2. `/api/v1/discussion_topics` + entries — Discussion context for engagement rules
3. `/api/v1/users/self/groups` — Group membership (for group-based rewards)
4. `/api/v1/courses/:course_id/modules` — Course structure (prerequisite awareness)
5. `/api/v1/users/self/bookmarks` — User quick links (optional UX feature)
6. `/api/v1/conversations` — Private messages (optional social engagement feature)
7. (Reserve slot for user feedback / customer request)

---

## Conclusion

Canvas LMS exposes **165+ endpoints** across academic, communication, and assessment domains. FocalPoint's **productivity/focus scope** targets **HIGH-priority endpoints** (courses, assignments, submissions, calendar, grades) with **MEDIUM-priority enhancements** (discussions, planner, modules, groups) for contextual awareness.

**Architecture decision:** Continue polling for MVP (14–20 tool calls to implement missing HIGH + starter MEDIUM endpoints). Adopt **Canvas Live Events (Caliper IMS 1.1)** in Phase 2 for real-time sync + rate-limit relief.

**One critical blocker:** Canvas admin must register a Developer Key on a real instance for production OAuth. This requires human approval from a partner institution; recommend reaching out to a non-profit university or contacting Instructure Partner Program.

**Confidence for 80% scope:** Medium–High, given stable Canvas API and existing Rust adapter patterns in codebase.

---

## Sources & References

- [Canvas LMS REST API Documentation](https://canvas.instructure.com/doc/api/) (https://canvas.instructure.com/doc/api/, accessed 2026-04-23)
- [Courses API](https://canvas.instructure.com/doc/api/courses.html)
- [Assignments API](https://canvas.instructure.com/doc/api/assignments.html)
- [Submissions API](https://canvas.instructure.com/doc/api/submissions.html)
- [Calendar Events API](https://canvas.instructure.com/doc/api/calendar_events.html)
- [Announcements API](https://canvas.instructure.com/doc/api/announcements.html)
- [Discussion Topics API](https://canvas.instructure.com/doc/api/discussion_topics.html)
- [Groups API](https://canvas.instructure.com/doc/api/groups.html)
- [Files API](https://canvas.instructure.com/doc/api/files.html)
- [Quizzes API](https://canvas.instructure.com/doc/api/quizzes.html)
- [Modules API](https://canvas.instructure.com/doc/api/modules.html)
- [Pages API](https://canvas.instructure.com/doc/api/pages.html)
- [Users API](https://canvas.instructure.com/doc/api/users.html)
- [Conversations API](https://canvas.instructure.com/doc/api/conversations.html)
- [Rubrics API](https://canvas.instructure.com/doc/api/rubrics.html)
- [Enrollments API](https://canvas.instructure.com/doc/api/enrollments.html)
- [Account Notifications API](https://canvas.instructure.com/doc/api/account_notifications.html)
- [Planner API](https://canvas.instructure.com/doc/api/planner.html)
- [Outcomes API](https://canvas.instructure.com/doc/api/outcomes.html)
- [Sections API](https://canvas.instructure.com/doc/api/sections.html)
- [Grading Standards API](https://canvas.instructure.com/doc/api/grading_standards.html)
- [Appointment Groups API](https://canvas.instructure.com/doc/api/appointment_groups.html)
- [Bookmarks API](https://canvas.instructure.com/doc/api/bookmarks.html)
- [Canvas Live Events — Caliper IMS 1.1 (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/data-services/live-events/overview/file.data_service_caliper_structure)
- [Creating a New Data Stream (Canvas LMS REST API)](https://canvas.instructure.com/doc/api/file.data_service_setup.html)
- [Canvas: How to use Beta and Test Environments](https://services.stthomas.edu/TDClient/1898/ClientPortal/KB/ArticleDet?ID=142659)
- [What is the Canvas test environment? (Instructure Community)](https://community.canvaslms.com/t5/Canvas-Releases/What-is-the-Canvas-test-environment/ta-p/262267)
- [What is the Canvas beta environment? (Instructure Community)](https://community.canvaslms.com/docs/DOC-14786-what-is-the-canvas-beta-environment)
- [Throttling (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/basics/file.throttling)
- [API Rate Limiting (Instructure Community)](https://community.canvaslms.com/docs/DOC-8381-api-rate-limiting)
- [Developer Keys (Canvas LMS REST API Documentation)](https://www.canvas.instructure.com/doc/api/file.developer_keys.html)
- [Developer Keys (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/oauth2/file.developer_keys)
- [OAuth2 Overview (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/oauth2/file.oauth)
- [OAuth2 Endpoints (Canvas LMS REST API Documentation)](https://www.canvas.instructure.com/doc/api/file.oauth_endpoints.html)
- [Masquerading (Canvas LMS REST API Documentation)](https://canvas.instructure.com/doc/api/file.masquerading.html)
- [Masquerading (Instructure Developer Documentation Portal)](https://developerdocs.instructure.com/services/canvas/basics/file.masquerading)
- [External Tools API](https://canvas.instructure.com/doc/api/external_tools.html)
- [Progress/Polling API](https://canvas.instructure.com/doc/api/progress.html)
