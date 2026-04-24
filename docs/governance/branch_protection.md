# Branch Protection Policy

FocalPoint enforces strict branch protection on the `main` branch to ensure code quality, legal compliance, and audit integrity.

## Settings

### Require signed commits
- **Enabled**: Yes
- **Enforcement**: All commits to `main` must be signed with GPG or SSH. GitHub displays a "Verified" badge on signed commits.
- **Rationale**: Proves commit authorship and prevents unauthorized history modification.

### Require a pull request before merging
- **Enabled**: Yes
- **Dismiss stale PR approvals when new commits are pushed**: Yes
- **Require code review from codeowners**: Yes
- **Minimum approvals**: 1

### Require status checks to pass
- **Enforce for admins**: Yes
- **Required status checks**:
  - `dco` — Developer Certificate of Origin check (via `tim-actions/dco@v1.1.0`)
  - `commit-msg-check` — Local commit message validator (conventional commits + Signed-off-by)
  - Other project checks: `clippy`, `cargo-test`, `docs-build` (if configured)

### Restrict who can push to matching branches
- **Enforce**: Yes
- **Allow force pushes**: No
- **Allow deletions**: No

## Rationale

1. **Signed commits** prove authorship and prevent commit history tampering, which is critical for audit integrity.
2. **DCO enforcement** certifies that contributors have legal rights to their code (Developer Certificate of Origin).
3. **Conventional commits** enforce readable, traceability-friendly commit messages across the team.
4. **Code review** ensures at least one other pair of eyes reviews all changes.
5. **No force-push** preserves audit trail and prevents accidental history rewriting.

## How to comply

### Set up Git signing locally

**Option 1: GPG**
```bash
# Generate or list existing keys
gpg --list-secret-keys --keyid-format long

# Configure Git to use your key
git config --global user.signingkey <YOUR-KEY-ID>
git config --global commit.gpgSign true
```

**Option 2: SSH**
```bash
# Configure Git to use your SSH key
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global commit.gpgSign true
```

### Use the commit template

Set up the `.gitmessage` template to guide conventional commit format:

```bash
git config core.hooksPath .githooks
git config commit.template .gitmessage
```

### Sign and DCO all commits

```bash
# Commits are auto-signed (commit.gpgSign = true)
# Add DCO manually or use -s flag:
git commit -s -m "feat(scope): description"

# Or edit the template and add the Signed-off-by line before committing.
```

## For maintainers

To enforce these settings on GitHub:

1. Navigate to **Settings → Branches → main → Branch protection rules**.
2. Enable:
   - Require signed commits
   - Require a pull request before merging
   - Require status checks to pass (with `dco` and `commit-msg-check`)
   - Restrict who can push (no force-push, no deletions)
3. Save.

For more details on DCO setup, see [`docs/governance/dco_setup.md`](./dco_setup.md).
