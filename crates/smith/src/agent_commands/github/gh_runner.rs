use std::process::Command;
use anyhow::{bail, Context, Result};

/// Run a `gh` CLI command and return its stdout as a string.
/// Stderr is inherited (shown to the user). Fails if exit code != 0.
pub fn gh(args: &[&str]) -> Result<String> {
    let output = Command::new("gh")
        .args(args)
        .stderr(std::process::Stdio::inherit())
        .output()
        .context("Failed to execute `gh` CLI. Hint: install GitHub CLI (https://cli.github.com/)")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "gh {} failed (exit {}). stdout: {} stderr: {}",
            args.join(" "),
            output.status.code().unwrap_or(-1),
            stdout.trim(),
            stderr.trim()
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Run a `gh` CLI command, capture stderr too (don't inherit).
/// Returns (stdout, stderr). Fails if exit code != 0.
pub fn gh_quiet(args: &[&str]) -> Result<(String, String)> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .context("Failed to execute `gh` CLI.")?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        bail!(
            "gh {} failed (exit {}). stdout: {} stderr: {}",
            args.join(" "),
            output.status.code().unwrap_or(-1),
            stdout.trim(),
            stderr.trim()
        );
    }

    Ok((stdout, stderr))
}

/// Run a `gh api graphql` query and return the parsed JSON.
pub fn gh_graphql(query: &str, variables: &[(&str, &str)]) -> Result<serde_json::Value> {
    let query_arg = format!("query={}", query);
    let mut args = vec!["api", "graphql", "-f", &query_arg];
    let formatted_vars: Vec<String> = variables
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    for var in &formatted_vars {
        args.push("-f");
        args.push(var);
    }

    let output = gh(&args)?;
    let value: serde_json::Value =
        serde_json::from_str(&output).context("Failed to parse GraphQL response as JSON")?;

    // Check for GraphQL errors
    if let Some(errors) = value.get("errors") {
        bail!("GraphQL errors: {}", errors);
    }

    Ok(value)
}

/// Run a `gh api graphql` query with custom headers and -F (typed) variables.
pub fn gh_graphql_with_headers(
    query: &str,
    f_variables: &[(&str, &str)],  // -f string vars
    typed_variables: &[(&str, &str)],  // -F typed vars
    headers: &[&str],
) -> Result<serde_json::Value> {
    let mut cmd_args: Vec<String> = vec![
        "api".to_string(),
        "graphql".to_string(),
    ];

    for header in headers {
        cmd_args.push("-H".to_string());
        cmd_args.push(header.to_string());
    }

    cmd_args.push("-f".to_string());
    cmd_args.push(format!("query={}", query));

    for (k, v) in f_variables {
        cmd_args.push("-f".to_string());
        cmd_args.push(format!("{}={}", k, v));
    }

    for (k, v) in typed_variables {
        cmd_args.push("-F".to_string());
        cmd_args.push(format!("{}={}", k, v));
    }

    let args_refs: Vec<&str> = cmd_args.iter().map(|s| s.as_str()).collect();
    let output = gh(&args_refs)?;
    let value: serde_json::Value =
        serde_json::from_str(&output).context("Failed to parse GraphQL response as JSON")?;

    if let Some(errors) = value.get("errors") {
        bail!("GraphQL errors: {}", errors);
    }

    Ok(value)
}
