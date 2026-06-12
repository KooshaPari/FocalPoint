# Changelog

All notable changes to `pheno-fastapi-base` are documented here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] - 2026-06-12

### Added
- Initial scaffold (L3-51 cherry-pick)
- 3 unit + 3 integration + 2 property tests = 8/8 passing
- `create_app(config: AppConfig) -> FastAPI` factory
- `AppConfig` (Pydantic v2) for declarative config
- Standard middleware: CORS, RequestId (UUID v7), OTEL
- `pheno_fastapi_base.errors.AppError` base + `NotFoundError`, `ValidationError`
- RFC 7807 problem+json error responses
- AI-DD crutches: `AGENTS.md`, `llms.txt`, `WORKLOG.md` (V2 schema), `LICENSE-MIT`
- This `CHANGELOG.md`

### Notes
- Standalone Python package (monorepo member)
- Depends on `fastapi`, `pydantic>=2`, `structlog`, `opentelemetry-instrumentation-fastapi`
- Python 3.10+ (uses `match` and PEP 604 unions)
- L4 hexagonal: `AppConfig` mirrors the pheno-config crate schema
