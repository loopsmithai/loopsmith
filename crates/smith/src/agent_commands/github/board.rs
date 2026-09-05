use anyhow::{Context, Result};

use super::gh_runner;
use super::setup::ProjectContext;

/// Fetch and print the project board grouped by status.
pub fn board_view(setup: &ProjectContext) -> Result<()> {
    // Fetch total count first, then all items
    let count_json = gh_runner::gh(&[
        "project", "item-list", &setup.project_num,
        "--owner", &setup.org, "--format", "json", "--limit", "1",
    ])?;
    let count_data: serde_json::Value = serde_json::from_str(&count_json)?;
    let total = count_data["totalCount"].as_u64().unwrap_or(100);

    let items_json = gh_runner::gh(&[
        "project", "item-list", &setup.project_num,
        "--owner", &setup.org, "--format", "json",
        "--limit", &total.to_string(),
    ])?;

    let items: serde_json::Value = serde_json::from_str(&items_json)
        .context("Failed to parse project items")?;

    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}
