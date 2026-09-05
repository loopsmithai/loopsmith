use anyhow::{bail, Context, Result};

use super::gh_runner;
use super::setup::{self, ProjectContext, RepoContext, RepoMeta};

/// Create an issue with the given parameters.
pub fn create(
    setup: &ProjectContext,
    repo_meta: &RepoMeta,
    title: &str,
    body: &str,
    kind: &str,
    parent: Option<u64>,
    milestone: Option<&str>,
    assignee: Option<&str>,
    initial_status: &str,
) -> Result<()> {
    let parts: Vec<&str> = setup.repo.split('/').collect();
    if parts.len() != 2 {
        bail!("Invalid repo format: {}", setup.repo);
    }
    let owner = parts[0];
    let repo = parts[1];

    // Resolve issue type ID
    let type_id = setup::resolve_issue_type_id(&repo_meta.issue_type_ids, kind)?;

    // Build the GraphQL mutation
    let mutation;
    let mut f_vars: Vec<(&str, String)>;
    let mut typed_vars: Vec<(&str, String)>;

    if let Some(parent_num) = parent {
        // Create as sub-issue with parent
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

        mutation = r#"
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
        "#.to_string();
        f_vars = vec![
            ("repoId", repo_meta.repo_id.clone()),
            ("title", title.to_string()),
            ("body", body.to_string()),
            ("issueTypeId", type_id),
            ("parentIssueId", parent_id.to_string()),
        ];
        typed_vars = vec![];
    } else {
        mutation = r#"
            mutation($repoId: ID!, $title: String!, $body: String!, $issueTypeId: ID!) {
                createIssue(input: {
                    repositoryId: $repoId,
                    title: $title,
                    body: $body,
                    issueTypeId: $issueTypeId
                }) {
                    issue { number title url }
                }
            }
        "#.to_string();
        f_vars = vec![
            ("repoId", repo_meta.repo_id.clone()),
            ("title", title.to_string()),
            ("body", body.to_string()),
            ("issueTypeId", type_id),
        ];
        typed_vars = vec![];
    }

    let f_refs: Vec<(&str, &str)> = f_vars.iter().map(|(k, v)| (*k, v.as_str())).collect();
    let result = gh_runner::gh_graphql_with_headers(
        &mutation,
        &f_refs,
        &[],
        &["GraphQL-Features: sub_issues,issue_types"],
    )?;

    let issue = &result["data"]["createIssue"]["issue"];
    let issue_num = issue["number"].as_u64().context("No issue number in response")?;
    eprintln!("✓ Issue #{} created: {}", issue_num, issue["title"]);

    // Add to project
    let add_mutation = r#"
        mutation($projectId: ID!, $contentId: ID!) {
            addProjectV2ItemById(input: { projectId: $projectId, contentId: $contentId }) {
                item { id }
            }
        }
    "#;
    // Need the issue node ID
    let issue_id_query = format!(
        r#"query {{ repository(owner: "{}", name: "{}") {{ issue(number: {}) {{ id }} }} }}"#,
        owner, repo, issue_num
    );
    let issue_id_result = gh_runner::gh_graphql(&issue_id_query, &[])?;
    let issue_node_id = issue_id_result["data"]["repository"]["issue"]["id"]
        .as_str()
        .context("Could not get issue node ID")?;

    let add_result = gh_runner::gh_graphql(
        add_mutation,
        &[("projectId", &setup.project_id), ("contentId", issue_node_id)],
    )?;
    let item_id = add_result["data"]["addProjectV2ItemById"]["item"]["id"]
        .as_str()
        .context("Failed to add issue to project")?;
    eprintln!("✓ Added to project #{}", setup.project_num);

    // Set initial status
    let status_option_id = setup::resolve_status_option_id(&setup.status_field_id, initial_status)?;
    let status_mutation = r#"
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
        status_mutation,
        &[
            ("projectId", &setup.project_id),
            ("itemId", item_id),
            ("fieldId", &setup.status_field_id),
            ("optionId", &status_option_id),
        ],
    )?;
    eprintln!("✓ Status set to: {}", initial_status);

    // Set assignee if provided
    if let Some(user) = assignee {
        gh_runner::gh(&[
            "issue", "edit", &issue_num.to_string(),
            "--repo", &setup.repo,
            "--add-assignee", user,
        ])?;
        eprintln!("✓ Assigned to: {}", user);
    }

    // Set milestone if provided
    if let Some(ms) = milestone {
        gh_runner::gh(&[
            "issue", "edit", &issue_num.to_string(),
            "--repo", &setup.repo,
            "--milestone", ms,
        ])?;
        eprintln!("✓ Milestone: {}", ms);
    }

    // Output
    let output = serde_json::json!({
        "created": true,
        "number": issue_num,
        "title": title,
        "kind": kind,
        "status": initial_status,
        "url": issue["url"],
    });
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// View a single issue with full details.
pub fn view(setup: &RepoContext, issue_num: u64, include_comments: bool) -> Result<()> {
    let parts: Vec<&str> = setup.repo.split('/').collect();
    let owner = parts[0];
    let repo = parts[1];

    let comments_fragment = if include_comments {
        "comments(last: 20) { nodes { author { login } body createdAt } }"
    } else {
        ""
    };

    let query = format!(r#"
        query($owner: String!, $repo: String!, $number: Int!) {{
            repository(owner: $owner, name: $repo) {{
                issue(number: $number) {{
                    number title state body url createdAt updatedAt
                    issueType {{ name }}
                    labels(first: 20) {{ nodes {{ name }} }}
                    assignees(first: 10) {{ nodes {{ login }} }}
                    milestone {{ title }}
                    parent {{ number title }}
                    subIssues(first: 50) {{ nodes {{ number title state }} }}
                    projectItems(first: 5) {{
                        nodes {{
                            fieldValueByName(name: "Status") {{
                                ... on ProjectV2ItemFieldSingleSelectValue {{ name }}
                            }}
                        }}
                    }}
                    {}
                }}
            }}
        }}
    "#, comments_fragment);

    let result = gh_runner::gh_graphql_with_headers(
        &query,
        &[("owner", owner), ("repo", repo)],
        &[("number", &issue_num.to_string())],
        &["GraphQL-Features: sub_issues,issue_types"],
    )?;

    let issue = &result["data"]["repository"]["issue"];
    println!("{}", serde_json::to_string_pretty(issue)?);
    Ok(())
}

/// Query issues by various filters.
pub fn query(
    setup: &RepoContext,
    query_type: &str,
    label: Option<&str>,
    _status: Option<&str>,
    milestone: Option<&str>,
    assignee: Option<&str>,
    issue: Option<u64>,
) -> Result<()> {
    match query_type {
        "single" => {
            let num = issue.context("--issue required for single query")?;
            view(setup, num, false)
        }
        "label" => {
            let lbl = label.context("--label required for label query")?;
            let output = gh_runner::gh(&[
                "issue", "list", "--repo", &setup.repo,
                "--label", lbl, "--json", "number,title,state,assignees,labels",
                "--limit", "100",
            ])?;
            println!("{}", output);
            Ok(())
        }
        "assignee" => {
            let user = assignee.context("--assignee required for assignee query")?;
            let output = gh_runner::gh(&[
                "issue", "list", "--repo", &setup.repo,
                "--assignee", user, "--json", "number,title,state,assignees,labels",
                "--limit", "100",
            ])?;
            println!("{}", output);
            Ok(())
        }
        "milestone" => {
            let ms = milestone.context("--milestone required for milestone query")?;
            let output = gh_runner::gh(&[
                "issue", "list", "--repo", &setup.repo,
                "--milestone", ms, "--json", "number,title,state,assignees,labels",
                "--limit", "100",
            ])?;
            println!("{}", output);
            Ok(())
        }
        "project-status" | "status" => {
            bail!(
                "Status/project-status queries require --project. \
                 Use query_with_project path instead."
            )
        }
        "issue-type" => {
            let type_name = label.context("--label required for issue-type query (pass the type name)")?;
            let parts: Vec<&str> = setup.repo.split('/').collect();
            let query = format!(r#"
                query($owner: String!, $repo: String!) {{
                    repository(owner: $owner, name: $repo) {{
                        issues(first: 100, states: OPEN) {{
                            nodes {{
                                number title state
                                issueType {{ name }}
                                assignees(first: 5) {{ nodes {{ login }} }}
                            }}
                        }}
                    }}
                }}
            "#);
            let result = gh_runner::gh_graphql_with_headers(
                &query,
                &[("owner", parts[0]), ("repo", parts[1])],
                &[],
                &["GraphQL-Features: issue_types"],
            )?;
            let issues = &result["data"]["repository"]["issues"]["nodes"];
            let empty2 = Vec::new();
            let filtered: Vec<&serde_json::Value> = issues
                .as_array()
                .unwrap_or(&empty2)
                .iter()
                .filter(|i| i["issueType"]["name"].as_str() == Some(type_name))
                .collect();
            println!("{}", serde_json::to_string_pretty(&filtered)?);
            Ok(())
        }
        other => bail!(
            "Unknown query type '{}'. Available: single, label, status, milestone, assignee, project-status, issue-type.",
            other
        ),
    }
}

/// Close an issue.
pub fn close(setup: &RepoContext, issue_num: u64) -> Result<()> {
    gh_runner::gh(&[
        "issue", "close", &issue_num.to_string(),
        "--repo", &setup.repo,
    ])?;
    let output = serde_json::json!({"closed": true, "number": issue_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Reopen an issue.
pub fn reopen(setup: &RepoContext, issue_num: u64) -> Result<()> {
    gh_runner::gh(&[
        "issue", "reopen", &issue_num.to_string(),
        "--repo", &setup.repo,
    ])?;
    let output = serde_json::json!({"reopened": true, "number": issue_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Add a comment to an issue.
pub fn comment(setup: &RepoContext, issue_num: u64, body: &str) -> Result<()> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let attributed_body = format!("### 🤖 smith — {}\n\n{}", timestamp, body);
    gh_runner::gh(&[
        "issue", "comment", &issue_num.to_string(),
        "--repo", &setup.repo,
        "--body", &attributed_body,
    ])?;
    let output = serde_json::json!({"commented": true, "number": issue_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Assign or unassign a user.
pub fn assign(setup: &RepoContext, issue_num: u64, user: &str, action: &str) -> Result<()> {
    let flag = match action {
        "assign" => "--add-assignee",
        "unassign" => "--remove-assignee",
        other => bail!("Unknown action '{}'. Use 'assign' or 'unassign'.", other),
    };
    gh_runner::gh(&[
        "issue", "edit", &issue_num.to_string(),
        "--repo", &setup.repo,
        flag, user,
    ])?;
    let output = serde_json::json!({"action": action, "number": issue_num, "user": user});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Update an issue's title or body.
pub fn update(
    setup: &RepoContext,
    issue_num: u64,
    title: Option<&str>,
    body: Option<&str>,
) -> Result<()> {
    let issue_str = issue_num.to_string();
    let mut args = vec![
        "issue", "edit", &issue_str,
        "--repo", &setup.repo,
    ];
    let title_owned;
    let body_owned;
    if let Some(t) = title {
        title_owned = t.to_string();
        args.push("--title");
        args.push(&title_owned);
    }
    if let Some(b) = body {
        body_owned = b.to_string();
        args.push("--body");
        args.push(&body_owned);
    }
    if title.is_none() && body.is_none() {
        bail!("At least one of --title or --body is required.");
    }
    let args_str: Vec<&str> = args.iter().map(|s| *s).collect();
    gh_runner::gh(&args_str)?;
    let output = serde_json::json!({"updated": true, "number": issue_num});
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

/// Query issues that require project context (status/project-status queries).
pub fn query_with_project(
    setup: &ProjectContext,
    query_type: &str,
    _label: Option<&str>,
    status: Option<&str>,
    _milestone: Option<&str>,
    _assignee: Option<&str>,
    issue: Option<u64>,
) -> Result<()> {
    match query_type {
        "project-status" => {
            let num = issue.context("--issue required for project-status query")?;
            let repo_ctx = setup.as_repo_ctx();
            let parts: Vec<&str> = repo_ctx.repo.split('/').collect();
            let result = gh_runner::gh_graphql_with_headers(
                r#"query($owner: String!, $repo: String!, $number: Int!) {
                    repository(owner: $owner, name: $repo) {
                        issue(number: $number) {
                            number title
                            issueType { name }
                            projectItems(first: 5) {
                                nodes {
                                    fieldValueByName(name: "Status") {
                                        ... on ProjectV2ItemFieldSingleSelectValue { name }
                                    }
                                }
                            }
                        }
                    }
                }"#,
                &[("owner", parts[0]), ("repo", parts[1])],
                &[("number", &num.to_string())],
                &["GraphQL-Features: issue_types"],
            )?;
            let issue_data = &result["data"]["repository"]["issue"];
            let output = serde_json::json!({
                "number": issue_data["number"],
                "title": issue_data["title"],
                "type": issue_data["issueType"]["name"],
                "status": issue_data["projectItems"]["nodes"][0]["fieldValueByName"]["name"],
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
            Ok(())
        }
        "status" => {
            let status_name = status.context("--status required for status query")?;
            let items_json = gh_runner::gh(&[
                "project",
                "item-list",
                &setup.project_num,
                "--owner",
                &setup.org,
                "--format",
                "json",
                "--limit",
                "200",
            ])?;
            let items: serde_json::Value = serde_json::from_str(&items_json)?;
            let empty = Vec::new();
            let filtered: Vec<&serde_json::Value> = items["items"]
                .as_array()
                .unwrap_or(&empty)
                .iter()
                .filter(|item| item["status"].as_str() == Some(status_name))
                .collect();
            println!("{}", serde_json::to_string_pretty(&filtered)?);
            Ok(())
        }
        other => bail!(
            "Unknown project query type '{}'. Available: status, project-status.",
            other
        ),
    }
}
