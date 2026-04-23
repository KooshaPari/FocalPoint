---
name: Bug report
about: Something broke. Help us reproduce and fix it.
title: "bug: <short summary>"
labels: ["bug", "triage"]
assignees: []
---

## Summary

<One sentence: what happened?>

## Expected

<What should have happened?>

## Actual

<What actually happened? Include error messages verbatim.>

## Steps to reproduce

1. ...
2. ...
3. ...

## Environment

- FocalPoint version / commit: `<commit sha or tag>`
- Platform: [ ] iOS <version>  [ ] CLI  [ ] docs-site  [ ] other
- Connector(s) involved: `<e.g. connector-canvas 0.0.1>`
- Rust toolchain: `rustc --version`
- Xcode (if iOS): `xcodebuild -version`

## Rule DSL (if relevant)

```toml
# paste the rule that fired, or the rule that failed to fire
```

## Event stream (if relevant)

```json
{ "paste": "minimal event payload that reproduces" }
```

## Audit chain state

- [ ] Audit chain verifies clean on launch
- [ ] Audit chain reports a break (paste the error)
- [ ] N/A

## Logs

<Paste relevant excerpts. Redact secrets.>

## Notes

<Anything else — workaround, guesses, related issues.>
