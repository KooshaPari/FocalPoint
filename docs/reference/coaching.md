# Coaching — LLM-backed Coachy copy & rule authoring

**Status:** partial (2026-04-23). Provider contract + Rust wiring + FFI
surface landed; no Swift integration yet.

FocalPoint routes Coachy's bubble copy, rule explanations, and
natural-language rule authoring through a cheap LLM (Minimax M2.7 / Kimi
K2.5 Turbo) behind a single trait so mascot/rules code never imports
`reqwest` directly.

## Provider contract

```rust
// crates/focus-coaching/src/lib.rs
#[async_trait]
pub trait CoachingProvider: Send + Sync {
    async fn complete(
        &self,
        prompt: &str,
        system: Option<&str>,
        max_tokens: u32,
    ) -> anyhow::Result<Option<String>>;
}
```

`Ok(None)` means "coaching is disabled or best-effort failed — fall back to
static copy." Callers never panic on missing coaching.

### Impls

| Impl | Purpose |
|------|---------|
| `HttpCoachingProvider` | POST `{endpoint}/chat/completions` (OpenAI-compat), Bearer auth via `SecretString`. Shape mirrors `cheap-llm-mcp/providers/openai_compat.py`. |
| `NoopCoachingProvider` | Always `Ok(None)`. Offline / no-API-key mode. |
| `StubCoachingProvider` | Test double; returns canned responses in order (wraps around). |
| `RateLimitedProvider`  | Token-bucket wrapper — 10 calls / 60s default. Excess → `Ok(None)` + `coaching.fallback` tracing event. |

`cheap-llm-mcp` itself is a stdio MCP, not HTTP — so FocalPoint calls the
underlying Minimax / Kimi / Fireworks OpenAI-compatible endpoints directly
rather than shelling out to the MCP.

## Prompt templates

Located in `crates/focus-coaching/src/prompts.rs`:

- `BUBBLE_SYSTEM_PROMPT` — Duolingo-Coachy voice brief. 1-line max, flame
  emoji only on Celebratory poses.
- `RULE_EXPLANATION_SYSTEM_PROMPT` — rewrite a rule's static explanation
  template as a human sentence grounded in the event payload.
- `RULE_AUTHORING_SYSTEM_PROMPT` — NL → JSON Rule. `rule_authoring_prompt()`
  substitutes the canonical Rule schema literal.

## Kill switch + rate limit

| Scenario | Behavior |
|----------|----------|
| `FOCALPOINT_DISABLE_COACHING=1` set | All provider calls → `Ok(None)` via `complete_guarded`. Bubble/explanation fall back to static template; NL authoring returns a "no content" error. |
| No provider wired (`set_coaching(None)`) | Same as kill switch for bubble/explanation paths. NL authoring returns `InvalidArgument("no coaching provider wired")`. |
| Rate-limited (>10/60s) | Call returns `Ok(None)`, logs `coaching.fallback{reason="rate_limit"}`, static copy used. |
| HTTP non-2xx | Returns `Ok(None)`, logs `coaching.fallback{reason="http_status"}`. |
| Empty LLM content | Returns `Ok(None)`, logs `coaching.fallback{reason="empty_content"}`. |
| NL rule parse failure | `propose_rule_from_nl` returns Err with raw LLM output attached. |

## Telemetry

All calls emit structured `tracing` events at INFO (request/response) or
WARN (fallback) level:

- `coaching.request` — model, prompt_chars, max_tokens
- `coaching.response` — response chars
- `coaching.fallback` — reason={kill_switch | rate_limit | http_status | empty_content}

API keys are stored as `secrecy::SecretString` and never logged.

## FFI surface

```
interface CoachingConfig {
    constructor(string endpoint, string api_key, string model);
};

interface FocalPointCore {
    void set_coaching(CoachingConfig? config);
    string? generate_bubble(MascotEvent event);
    [Throws=FfiError]
    RuleSummary propose_rule_from_nl(string nl_spec);
};
```

Swift onboarding should call `set_coaching(CoachingConfig(...))` after the
user opts into LLM-backed copy; passing `null` (or never calling it) keeps
everything static.

## References

- `crates/focus-coaching/` — trait, impls, prompts, 8 tests.
- `crates/focus-mascot/` — `on_event_with_bubble` async path, 3 LLM tests.
- `crates/focus-rules/` — `propose_rule_from_nl`, `render_llm_explanation`, 6 LLM tests.
- `crates/focus-ffi/` — `CoachingConfig`, `set_coaching`, `generate_bubble`, `propose_rule_from_nl`, 4 integration tests.
