---
title: Verification Criteria
description: Security and functional requirements for published connectors and rule packs.
---

# Verification Criteria

All connectors and rule packs published in the FocalPoint marketplace must meet verification criteria.

## Connector Verification

### Security Requirements

- [ ] No hardcoded credentials in source code
- [ ] Credentials stored in Keychain/Keystore (not plaintext)
- [ ] HTTPS enforced for all API calls
- [ ] OAuth 2.0 or token-based auth (no basic auth over HTTP)
- [ ] No data exfiltration; user data stays local
- [ ] Dependency audit: `cargo audit` passes
- [ ] No unsafe Rust blocks without justification

### Functional Requirements

- [ ] Manifest file is valid (see [Manifest Format](../connector-sdk/manifest))
- [ ] All declared events have schema definitions
- [ ] Events are emitted correctly (JSON schema validation)
- [ ] OAuth flow tested with real service
- [ ] Token refresh works correctly
- [ ] Error handling for network failures
- [ ] Rate limiting respected (if applicable)

### Test Requirements

- [ ] >=70% code coverage
- [ ] All critical paths covered (100%)
- [ ] Unit tests included
- [ ] Integration tests with mocked service
- [ ] E2E test with real auth flow

### Documentation Requirements

- [ ] README with setup instructions
- [ ] Event schema documented
- [ ] Example rule provided
- [ ] Troubleshooting section
- [ ] Privacy statement (if accessing personal data)

### Code Quality

- [ ] `cargo fmt` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No compiler warnings
- [ ] Code comments for complex logic
- [ ] Follows Rust API guidelines

## Rule Pack Verification

### Functional Requirements

- [ ] All rules are syntactically valid
- [ ] All event types exist (in shipped connectors)
- [ ] All actions are valid (see [Action Catalogue](../rules/actions))
- [ ] Rules are tested with real events
- [ ] Conditions make logical sense

### Documentation Requirements

- [ ] Pack description (1–3 sentences)
- [ ] Use case example
- [ ] Instructions for import/installation
- [ ] Troubleshooting section
- [ ] Author/license information

### Quality Requirements

- [ ] Rules are ordered logically
- [ ] No conflicting rules
- [ ] Naming is consistent
- [ ] Each rule has a clear purpose
- [ ] No more than 50 rules per pack

## Verification Tiers

### Trusted

Created by FocalPoint core. Pinned in app UI.

- **Requirements**: All above + design review + accessibility audit
- **Release**: Bundled with app updates

### Verified

Community submission that passed full audit.

- **Requirements**: All above
- **Process**: 1–2 week audit by maintainers
- **Release**: Listed in marketplace

### Community

Published without formal audit.

- **Requirements**: Minimal (malware scan only)
- **Disclaimer**: "User assumes risk"
- **Release**: Immediate (upon review)

## Audit Process

### For Connectors

1. Submit GitHub PR with manifest, code, tests, docs
2. Maintainers review for security, functionality, quality
3. Open PR feedback; author addresses
4. Approval and merge to `connectors/` directory
5. Connector available in marketplace

Timeline: **1–2 weeks**

### For Rule Packs

1. Submit via marketplace interface (simple form)
2. Automated validation (syntax, event types)
3. Quick review for quality & conflicts
4. Approval and listing

Timeline: **2–3 days**

## Security Audit

All connectors undergo security review:

- Code review by maintainers
- Dependency vulnerability scan (`cargo audit`)
- OAuth flow validation
- Data handling assessment
- Permission scope audit

## Appeal Process

If your submission is rejected, you can:

1. Request detailed feedback
2. Make improvements
3. Resubmit (no additional fee)

See [GitHub Issues](https://github.com/KooshaPari/FocalPoint/issues) for appeals.

## Ongoing Requirements

After publication:

- **Security updates**: Prompt response to CVEs
- **Compatibility**: Test with new FocalPoint versions
- **Maintenance**: Keep code updated (no >6 months without update)
- **User support**: Respond to issues within 1 week

Failure to maintain may result in delisting.

## Questions?

- Email: verify@focalpoint.local
- GitHub: [Security Issues](https://github.com/KooshaPari/FocalPoint/security/advisories)
