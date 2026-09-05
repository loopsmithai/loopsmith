use anyhow::{Context, Result};

use super::gh_runner;
use super::setup::RepoContext;

pub fn list(setup: &RepoContext) -> Result<()> {
    let json_output = gh_runner::gh(&[
        "api", &format!("repos/{}/milestones", setup.repo),
    ])?;
    println!("{}", json_output);
    Ok(())
}

pub fn create(setup: &RepoContext, title: &str, description: Option<&str>, due_date: Option<&str>) -> Result<()> {
    let mut body = serde_json::json!({"title": title});
    if let Some(desc) = description {
        body["description"] = serde_json::Value::String(desc.to_string());
    }
    if let Some(date) = due_date {
        body["due_on"] = serde_json::Value::String(format!("{}T00:00:00Z", date));
    }

    let body_str = serde_json::to_string(&body)?;
    let output = gh_runner::gh(&[
        "api", &format!("repos/{}/milestones", setup.repo),
        "--method", "POST",
        "--input", "-",
    ])?;
    println!("{}", output);
    Ok(())
}

pub fn assign(setup: &RepoContext, issue_num: u64, title: &str) -> Result<()> {
    // Find milestone number by title
    let milestones_json = gh_runner::gh(&[
        "api", &format!("repos/{}/milestones", setup.repo),
    ])?;
    let milestones: Vec<serde_json::Value> = serde_json::from_str(&milestones_json)?;
    let _ms_num = milestones
        .iter()
        .find(|m| m["title"].as_str() == Some(title))
        .and_then(|m| m["number"].as_u64())
        .context(format!("Milestone '{}' not found", title))?;

    gh_runner::gh(&[
        "issue", "edit", &issue_num.to_string(),
        "--repo", &setup.repo,
        "--milestone", title,
    ])?;
    let output = serde_json::json!({"assigned": true, "number": issue_num, "milestone": title});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
