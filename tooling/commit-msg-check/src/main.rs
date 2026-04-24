use anyhow::{anyhow, Result};
use clap::Parser;
use regex::Regex;
use std::fs;

#[derive(Parser)]
#[command(name = "commit-msg-check")]
#[command(about = "Validate commit messages for conventional commits + DCO sign-off")]
struct Args {
    /// Path to commit message file (typically COMMIT_EDITMSG from git hook)
    commit_msg_file: String,
}

fn validate_commit_msg(content: &str) -> Result<()> {
    let trimmed = content.trim();

    if trimmed.is_empty() {
        return Err(anyhow!("Commit message is empty"));
    }

    // Split into lines
    let lines: Vec<&str> = trimmed.lines().collect();
    let first_line = lines[0];

    // Validate conventional commits format: <type>(<scope>): <description>
    // or <type>: <description>
    let cc_regex = Regex::new(r"^(feat|fix|docs|chore|test|refactor|perf|ci|build)(\([^)]+\))?: .+$")
        .expect("valid regex");

    if !cc_regex.is_match(first_line) {
        return Err(anyhow!(
            "Invalid commit message format.\nExpected: <type>(<scope>): <description>\n\
             Valid types: feat, fix, docs, chore, test, refactor, perf, ci, build\n\
             Got: {}",
            first_line
        ));
    }

    // Check for DCO sign-off: must have "Signed-off-by: Name <email>" in the message
    let has_signoff = trimmed.lines().any(|line| {
        line.trim_start().starts_with("Signed-off-by:")
    });

    if !has_signoff {
        return Err(anyhow!(
            "Missing DCO sign-off.\nAdd 'Signed-off-by: Your Name <email@example.com>' \
             to the end of your commit message, or use 'git commit -s' to auto-add it."
        ));
    }

    // Validate sign-off format: "Signed-off-by: Name <email>"
    let signoff_regex = Regex::new(r"^Signed-off-by: .+ <.+@.+>$").expect("valid regex");
    if !trimmed
        .lines()
        .filter(|l| l.trim_start().starts_with("Signed-off-by:"))
        .all(|l| signoff_regex.is_match(l.trim_start()))
    {
        return Err(anyhow!(
            "Invalid DCO sign-off format.\nExpected: Signed-off-by: Your Name <email@example.com>"
        ));
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let content = fs::read_to_string(&args.commit_msg_file)
        .map_err(|e| anyhow!("Failed to read {}: {}", args.commit_msg_file, e))?;

    validate_commit_msg(&content)?;

    println!("✓ Commit message is valid");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_conventional_commit_with_dco() {
        let msg = "feat(connector): add new OAuth flow\n\nThis adds support for OAuth 2.0.\n\nSigned-off-by: John Doe <john@example.com>";
        assert!(validate_commit_msg(msg).is_ok());
    }

    #[test]
    fn test_valid_fix_commit_with_dco() {
        let msg = "fix(rules): off-by-one error\n\nSigned-off-by: Jane Smith <jane@example.com>";
        assert!(validate_commit_msg(msg).is_ok());
    }

    #[test]
    fn test_missing_dco() {
        let msg = "feat(connector): add new feature";
        let err = validate_commit_msg(msg);
        assert!(err.is_err());
        assert!(err
            .unwrap_err()
            .to_string()
            .contains("Missing DCO sign-off"));
    }

    #[test]
    fn test_invalid_format() {
        let msg = "add new feature\n\nSigned-off-by: John Doe <john@example.com>";
        let err = validate_commit_msg(msg);
        assert!(err.is_err());
        assert!(err
            .unwrap_err()
            .to_string()
            .contains("Invalid commit message format"));
    }

    #[test]
    fn test_empty_message() {
        let msg = "";
        let err = validate_commit_msg(msg);
        assert!(err.is_err());
    }

    #[test]
    fn test_valid_multiple_signoffs() {
        let msg = "feat(api): new endpoint\n\nSigned-off-by: Alice <alice@example.com>\nSigned-off-by: Bob <bob@example.com>";
        assert!(validate_commit_msg(msg).is_ok());
    }

    #[test]
    fn test_invalid_signoff_format() {
        let msg = "feat(core): change\n\nSigned-off-by: Invalid Format";
        let err = validate_commit_msg(msg);
        assert!(err.is_err());
    }
}
