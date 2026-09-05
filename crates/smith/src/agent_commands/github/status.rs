use anyhow::{bail, Context, Result};

use super::gh_runner;
use super::setup::{self, ProjectContext};

/// Transition an issue's project board status with verification.
pub fn set(setup: &ProjectContext, issue_num: u64, to: &str, from: Option<&str>) -> Result<()> {
    // Resolve the target status option ID
    let option_id = setup::resolve_status_option_id(&setup.status_field_id, to)?;

    // Resolve the issue's project item ID
    let items_json = gh_runner::gh(&[
        "project", "item-list", &setup.project_num,
        "--owner", &setup.org, "--format", "json", "--limit", "200",
    ])?;
    let items: serde_json::Value = serde_json::from_str(&items_json)?;
    let item_id = items["items"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .find(|item| item["content"]["number"].as_u64() == Some(issue_num))
        .and_then(|item| item["id"].as_str())
        .context(format!(
            "Issue #{} not found in project #{}. Hint: add it to the project first.",
            issue_num, setup.project_num
        ))?
        .to_string();

    // Update the status
    let mutation = r#"
        mutation($projectId: ID!, $itemId: ID!, $fieldId: ID!, $optionId: String!) {
            updateProjectV2ItemFieldValue(input: {
                projectId: $projectId,
                itemId: $itemId,
                fieldId: $fieldId,
                value: { singleSelectOptionId: $optionId }
            }) {
                projectV2Item { id }
            }
        }
    "#;
    gh_runner::gh_graphql(
        mutation,
        &[
            ("projectId", &setup.project_id),
            ("itemId", &item_id),
            ("fieldId", &setup.status_field_id),
            ("optionId", &option_id),
        ],
    )?;

    // Verify the status actually changed
    let verify_query = r#"
        query($itemId: ID!) {
            node(id: $itemId) {
                ... on ProjectV2Item {
                    fieldValueByName(name: "Status") {
                        ... on ProjectV2ItemFieldSingleSelectValue { name }
                    }
                }
            }
        }
    "#;
    let verify_result = gh_runner::gh_graphql(verify_query, &[("itemId", &item_id)])?;
    let actual_status = verify_result["data"]["node"]["fieldValueByName"]["name"]
        .as_str()
        .unwrap_or("");

    if actual_status != to {
        // Attempt rollback if from was provided
        if let Some(from_status) = from {
            if let Ok(from_id) = setup::resolve_status_option_id(&setup.status_field_id, from_status) {
                let _ = gh_runner::gh_graphql(
                    mutation,
                    &[
                        ("projectId", &setup.project_id),
                        ("itemId", &item_id),
                        ("fieldId", &setup.status_field_id),
                        ("optionId", &from_id),
                    ],
                );
            }
        }
        bail!(
            "Status verification failed for #{}: expected \'{}\', got \'{}\'. \
             Hint: check that the GitHub App has \'organization_projects: admin\' permission.",
            issue_num, to, actual_status
        );
    }

    eprintln!("✓ Status #{}: {} → {}", issue_num, from.unwrap_or("(unknown)"), to);
    let output = serde_json::json!({
        "transitioned": true,
        "number": issue_num,
        "from": from,
        "to": to,
        "verified": true,
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
