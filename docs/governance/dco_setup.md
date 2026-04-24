# DCO Setup Guide

Developer Certificate of Origin (DCO) is a lightweight legal commitment certifying that you have the right to contribute code under FocalPoint's MIT OR Apache-2.0 license.

## What is DCO?

The DCO certifies that:
- You authored the code (or obtained it under a compatible license).
- You have the legal right to contribute it under FocalPoint's dual license.
- You understand the implications of your contribution.

It is **not** a Contributor License Agreement (CLA); it is a statement of honesty and responsibility.

## GitHub DCO Check

FocalPoint runs a DCO check on all pull requests via the GitHub Actions workflow (`.github/workflows/dco.yml`). This check verifies that every commit in your PR includes a `Signed-off-by` trailer with your name and email.

**Example of a valid commit:**

```
feat(connector-canvas): handle 429 with Retry-After backoff

Signed-off-by: Alice Wonder <alice@example.com>
```

## Local Setup (Optional but Recommended)

For convenience, install a local commit message validator that runs before each commit. This catches DCO violations early.

### Prerequisites

- **Rust 1.82+** (installed via `rustup`)
- **Git** with hook support

### Installation

1. **Build and install the validator:**

```bash
cargo install --path tooling/commit-msg-check
```

This installs `commit-msg-check` to `~/.cargo/bin/` (add to PATH if needed).

2. **Create the `.githooks` directory:**

```bash
mkdir -p .githooks
chmod 755 .githooks
```

3. **Write the commit-msg hook:**

Create `.githooks/commit-msg` with the following content and make it executable:

```bash
#!/bin/bash
# FocalPoint commit message validator
# Checks: conventional commits format + DCO sign-off
# Exit non-zero to block the commit.

exec commit-msg-check "$1"
```

Then:

```bash
chmod +x .githooks/commit-msg
```

4. **Configure Git to use the hooks:**

```bash
git config core.hooksPath .githooks
```

5. **Optional: Use the commit template:**

To get reminders about the commit format, configure the template:

```bash
git config commit.template .gitmessage
```

Then edit `~/.gitconfig` to always use it (or set it per-repo as above).

### Verifying the Setup

Try committing with an invalid message:

```bash
git commit -m "oops: forgot DCO"
```

The hook should reject it and print an error:

```
Missing DCO sign-off.
Add to your commit: Signed-off-by: Your Name <your.email@example.com>
Or use: git commit -s -m "..."
```

### Making a valid commit

Use the `-s` flag to auto-add the sign-off:

```bash
git commit -s -m "feat(connector-canvas): handle 429 with Retry-After backoff"
```

Or manually add the `Signed-off-by` line in your editor.

### Removing the local setup

If you no longer want the local validator:

```bash
rm .githooks/commit-msg
cargo uninstall commit-msg-check
```

## Validator Rules

The local `commit-msg-check` validator enforces:

1. **Conventional Commit Format** on the first line:
   - Format: `<type>(<scope>): <description>` or `<type>: <description>`
   - Types: `feat`, `fix`, `docs`, `chore`, `test`, `refactor`, `perf`, `ci`
   - Scope (optional): domain or crate name (e.g., `connector-canvas`, `focus-rules`)
   - Description: imperative, lowercase, no period
   - Merge commits are allowed (they bypass this check)

2. **DCO Sign-Off**:
   - Must include a line matching: `Signed-off-by: Your Name <your.email@example.com>`
   - Can be anywhere in the message body or footer
   - Email must be enclosed in angle brackets

### Example valid commits:

```
feat(connector-canvas): handle 429 with Retry-After backoff

Signed-off-by: Alice Wonder <alice@example.com>
```

```
fix(focus-rules): cooldown timer off-by-one at DST boundary

This fixes the edge case where rules incorrectly fire at DST transition.

Signed-off-by: Bob Builder <bob@example.com>
```

```
docs(rules): add sleep-debt sample rule
Signed-off-by: Carol King <carol@example.com>
```

## FAQ

**Q: Can I use a different email?**
A: Yes, but it should match your GitHub email for the PR to show as authored by you.

**Q: What if I forgot to sign a commit?**
A: If it's the last commit, use `git commit --amend -s` to add the sign-off. For older commits, ask a maintainer for guidance.

**Q: What if I'm fixing a bug that someone else reported?**
A: You still need your own `Signed-off-by`. You can mention the original reporter in the commit body or PR description, but the sign-off is your legal commitment.

**Q: Does DCO conflict with GPG signing?**
A: No. You can (and should) use both. GPG proves you made the commit; DCO certifies you have the right to contribute it.

## Further Reading

- [Developer Certificate of Origin](https://developercertificate.org/)
- [FocalPoint CONTRIBUTING.md](../../CONTRIBUTING.md)
- [Branch Protection Policy](./branch_protection.md)
