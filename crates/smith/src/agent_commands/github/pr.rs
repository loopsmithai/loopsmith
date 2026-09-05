use anyhow::{bail, Context, Result};

use super::gh_runner;
use super::setup::RepoContext;

pub fn create(
    setup: &RepoContext,
    title: &str,
    body: &str,
    branch: &str,
    base: &str,
    draft: bool,
) -> Result<()> {
    let mut args = vec![
        "pr", "create",
        "--repo", &setup.repo,
        "--title", title,
        "--body", body,
        "--head", branch,
        "--base", base,
    ];
    if draft {
        args.push("--draft");
    }
    let output = gh_runner::gh(&args)?;
    println!("{}", output.trim());
    Ok(())
}

pub fn view(setup: &RepoContext, pr_num: u64) -> Result<()> {
    let output = gh_runner::gh(&[
        "pr", "view", &pr_num.to_string(),
        "--repo", &setup.repo,
        "--json", "number,title,state,body,author,baseRefName,headRefName,url,reviewDecision,comments,reviews",
    ])?;
    println!("{}", output);
    Ok(())
}

pub fn list(setup: &RepoContext, search: Option<&str>) -> Result<()> {
    let mut args = vec![
        "pr", "list",
        "--repo", &setup.repo,
        "--json", "number,title,state,author,headRefName,url",
    ];
    let search_owned;
    if let Some(s) = search {
        search_owned = s.to_string();
        args.push("--search");
        args.push(&search_owned);
    }
    let args_str: Vec<&str> = args.iter().map(|s| *s).collect();
    let output = gh_runner::gh(&args_str)?;
    println!("{}", output);
    Ok(())
}

pub fn merge(setup: &RepoContext, pr_num: u64, method: &str) -> Result<()> {
    let method_flag = match method {
        "merge" => "--merge",
        "squash" => "--squash",
        "rebase" => "--rebase",
        other => bail!("Unknown merge method \'{}\'. Use: merge, squash, or rebase.", other),
    };
    gh_runner::gh(&[
        "pr", "merge", &pr_num.to_string(),
        "--repo", &setup.repo,
        method_flag, "--yes",
    ])?;
    let output = serde_json::json!({"merged": true, "number": pr_num, "method": method});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

pub fn approve(setup: &RepoContext, pr_num: u64, body: Option<&str>) -> Result<()> {
    let pr_str = pr_num.to_string();
    let mut args = vec![
        "pr", "review", &pr_str,
        "--repo", &setup.repo,
        "--approve",
    ];
    let body_owned;
    if let Some(b) = body {
        body_owned = b.to_string();
        args.push("--body");
        args.push(&body_owned);
    }
    let args_str: Vec<&str> = args.iter().map(|s| *s).collect();
    gh_runner::gh(&args_str)?;
    let output = serde_json::json!({"approved": true, "number": pr_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

pub fn request_changes(setup: &RepoContext, pr_num: u64, body: &str) -> Result<()> {
    let pr_str = pr_num.to_string();
    gh_runner::gh(&[
        "pr", "review", &pr_str,
        "--repo", &setup.repo,
        "--request-changes", "--body", body,
    ])?;
    let output = serde_json::json!({"requested_changes": true, "number": pr_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

pub fn comment(setup: &RepoContext, pr_num: u64, body: &str) -> Result<()> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let attributed = format!("### 🤖 smith — {}\n\n{}", timestamp, body);
    gh_runner::gh(&[
        "pr", "comment", &pr_num.to_string(),
        "--repo", &setup.repo,
        "--body", &attributed,
    ])?;
    let output = serde_json::json!({"commented": true, "number": pr_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

pub fn close(setup: &RepoContext, pr_num: u64) -> Result<()> {
    gh_runner::gh(&[
        "pr", "close", &pr_num.to_string(),
        "--repo", &setup.repo,
    ])?;
    let output = serde_json::json!({"closed": true, "number": pr_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
