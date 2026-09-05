use anyhow::{Context, Result};

use super::gh_runner;
use super::setup::{self, RepoContext, RepoMeta};

pub fn create(
    setup: &RepoContext,
    repo_meta: &RepoMeta,
    parent_num: u64,
    title: &str,
    body: Option<&str>,
    issue_type: &str,
) -> Result<()> {
    let parts: Vec<&str> = setup.repo.split('/').collect();
    let owner = parts[0];
    let repo = parts[1];

    let type_id = setup::resolve_issue_type_id(&repo_meta.issue_type_ids, issue_type)?;

    // Get parent issue node ID
    let parent_query = format!(
        r#"query {{ repository(owner: "{}", name: "{}") {{ issue(number: {}) {{ id }} }} }}"#,
        owner, repo, parent_num
    );
    let parent_result = gh_runner::gh_graphql_with_headers(
        &parent_query, &[], &[],
        &["GraphQL-Features: sub_issues,issue_types"],
    )?;
    let parent_id = parent_result["data"]["repository"]["issue"]["id"]
        .as_str()
        .context(format!("Parent issue #{} not found", parent_num))?;

    let body_str = body.unwrap_or("");
    let mutation = r#"
        mutation($repoId: ID!, $title: String!, $body: String!, $issueTypeId: ID!, $parentIssueId: ID!) {
            createIssue(input: {
                repositoryId: $repoId,
                title: $title,
                body: $body,
                issueTypeId: $issueTypeId,
                parentIssueId: $parentIssueId
            }) {
                issue { number title url }
            }
        }
    "#;
    let result = gh_runner::gh_graphql_with_headers(
        mutation,
        &[
            ("repoId", &repo_meta.repo_id),
            ("title", title),
            ("body", body_str),
            ("issueTypeId", &type_id),
            ("parentIssueId", parent_id),
        ],
        &[],
        &["GraphQL-Features: sub_issues,issue_types"],
    )?;

    let issue = &result["data"]["createIssue"]["issue"];
    println!("{}", serde_json::to_string_pretty(issue)?);
    Ok(())
}

pub fn list(setup: &RepoContext, parent_num: u64) -> Result<()> {
    let parts: Vec<&str> = setup.repo.split('/').collect();
    let query = r#"
        query($owner: String!, $repo: String!, $number: Int!) {
            repository(owner: $owner, name: $repo) {
                issue(number: $number) {
                    subIssues(first: 50) {
                        nodes {
                            number title state
                            issueType { name }
                            assignees(first: 5) { nodes { login } }
                        }
                    }
                }
            }
        }
    "#;
    let result = gh_runner::gh_graphql_with_headers(
        query,
        &[("owner", parts[0]), ("repo", parts[1])],
        &[("number", &parent_num.to_string())],
        &["GraphQL-Features: sub_issues,issue_types"],
    )?;
    let sub_issues = &result["data"]["repository"]["issue"]["subIssues"]["nodes"];
    println!("{}", serde_json::to_string_pretty(sub_issues)?);
    Ok(())
}

pub fn status(setup: &RepoContext, parent_num: u64) -> Result<()> {
    let parts: Vec<&str> = setup.repo.split('/').collect();
    let query = r#"
        query($owner: String!, $repo: String!, $number: Int!) {
            repository(owner: $owner, name: $repo) {
                issue(number: $number) {
                    title
                    subIssues(first: 50) {
                        nodes { number title state }
                    }
                }
            }
        }
    "#;
    let result = gh_runner::gh_graphql_with_headers(
        query,
        &[("owner", parts[0]), ("repo", parts[1])],
        &[("number", &parent_num.to_string())],
        &["GraphQL-Features: sub_issues"],
    )?;
    let issue = &result["data"]["repository"]["issue"];
    let sub_issues = issue["subIssues"]["nodes"].as_array();
    let total = sub_issues.map(|s| s.len()).unwrap_or(0);
    let closed = sub_issues
        .map(|s| s.iter().filter(|i| i["state"].as_str() == Some("CLOSED")).count())
        .unwrap_or(0);
    let output = serde_json::json!({
        "parent": parent_num,
        "title": issue["title"],
        "total": total,
        "closed": closed,
        "open": total - closed,
        "complete": total > 0 && closed == total,
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
