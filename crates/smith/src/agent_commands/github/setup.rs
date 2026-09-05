use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::gh_runner;

/// Repo-level context — sufficient for issue/PR/milestone/fork operations.
#[derive(Debug, Clone)]
pub struct RepoContext {
    pub org: String,  // GitHub org
    pub repo: String, // org/repo (full path)
}

impl RepoContext {
    pub fn owner(&self) -> &str {
        &self.org
    }

    pub fn repo_name(&self) -> &str {
        self.repo.split('/').nth(1).unwrap_or(&self.repo)
    }
}

/// Project-level context — extends RepoContext with project board data.
/// Required for board view and status transitions.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectContext {
    pub org: String,             // GitHub org
    pub repo: String,            // org/repo
    pub project_num: String,     // project number (from CLI, never inferred)
    pub project_id: String,      // project node ID
    pub status_field_id: String, // Status field node ID
}

impl ProjectContext {
    pub fn as_repo_ctx(&self) -> RepoContext {
        RepoContext {
            org: self.org.clone(),
            repo: self.repo.clone(),
        }
    }
}

/// Cached repository metadata.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoMeta {
    pub repo_id: String,
    pub issue_type_ids: serde_json::Value,
}

fn cache_dir() -> PathBuf {
    let dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("smith-agent")
        .join("github");
    std::fs::create_dir_all(&dir).ok();
    dir
}

fn project_cache_path(org: &str, project_num: u64) -> PathBuf {
    cache_dir().join(format!("project-{}-{}.json", org, project_num))
}

fn repo_meta_cache_path(owner: &str, repo: &str) -> PathBuf {
    cache_dir().join(format!("repo-meta-{}-{}.json", owner, repo))
}

/// Infer the repo from the git remote in the current directory.
/// Returns "org/repo" format. Validates that the remote org matches --org.
pub fn infer_repo_from_git(org: &str) -> Result<String> {
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .context("Failed to run 'git remote get-url origin'. Are you in a git repo?")?;

    if !output.status.success() {
        bail!(
            "Could not infer repo from git remote. Specify --repo explicitly.\n\
             Hint: smith-agent github --org {} --repo <repo-name> ...",
            org
        );
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse org/repo from various URL formats
    let repo_path = if url.starts_with("git@") {
        // git@github.com:org/repo.git
        url.split(':')
            .nth(1)
            .unwrap_or("")
            .trim_end_matches(".git")
            .to_string()
    } else {
        // https://github.com/org/repo.git
        let path = url
            .trim_end_matches(".git")
            .split('/')
            .collect::<Vec<_>>();
        if path.len() >= 2 {
            format!("{}/{}", path[path.len() - 2], path[path.len() - 1])
        } else {
            bail!(
                "Could not parse git remote URL: {}. Specify --repo explicitly.",
                url
            );
        }
    };

    let parts: Vec<&str> = repo_path.split('/').collect();
    if parts.len() != 2 {
        bail!(
            "Git remote '{}' doesn't look like org/repo. Specify --repo explicitly.",
            repo_path
        );
    }

    let remote_org = parts[0];
    if remote_org != org {
        bail!(
            "Git remote org '{}' doesn't match --org '{}'. Specify --repo explicitly.\n\
             Hint: smith-agent github --org {} --repo {} ...",
            remote_org,
            org,
            org,
            parts[1]
        );
    }

    Ok(repo_path)
}

/// Build a RepoContext from CLI args, inferring repo from git if needed.
pub fn build_repo_context(org: &str, repo: Option<&str>) -> Result<RepoContext> {
    let full_repo = match repo {
        Some(r) => {
            if r.contains('/') {
                r.to_string()
            } else {
                format!("{}/{}", org, r)
            }
        }
        None => infer_repo_from_git(org)?,
    };

    Ok(RepoContext {
        org: org.to_string(),
        repo: full_repo,
    })
}

/// Load or fetch project context. Requires an explicit project number — never inferred.
pub fn load_or_fetch_project(
    repo_ctx: &RepoContext,
    project_num: u64,
) -> Result<ProjectContext> {
    let cache_path = project_cache_path(&repo_ctx.org, project_num);
    let project_num_str = project_num.to_string();

    // Try cache first
    if let Ok(data) = std::fs::read_to_string(&cache_path) {
        if let Ok(ctx) = serde_json::from_str::<ProjectContext>(&data) {
            eprintln!(
                "✓ Project (cached): {}, project #{}",
                ctx.repo, ctx.project_num
            );
            return Ok(ctx);
        }
    }

    eprintln!(
        "→ Fetching project setup for {} #{}...",
        repo_ctx.org, project_num
    );

    // Fetch project ID
    let project_view = gh_runner::gh(&[
        "project",
        "view",
        &project_num_str,
        "--owner",
        &repo_ctx.org,
        "--format",
        "json",
    ])?;
    let project_data: serde_json::Value = serde_json::from_str(&project_view)?;
    let project_id = project_data["id"]
        .as_str()
        .context("Could not get project ID")?
        .to_string();

    // Fetch Status field ID
    let fields_json = gh_runner::gh(&[
        "project",
        "field-list",
        &project_num_str,
        "--owner",
        &repo_ctx.org,
        "--format",
        "json",
    ])?;
    let fields: serde_json::Value = serde_json::from_str(&fields_json)?;
    let status_field_id = fields["fields"]
        .as_array()
        .context("No fields in project")?
        .iter()
        .find(|f| f["name"].as_str() == Some("Status"))
        .context(
            "No 'Status' field found in project. \
             Hint: add a Status field to your GitHub Project.",
        )?["id"]
        .as_str()
        .context("Status field has no ID")?
        .to_string();

    let ctx = ProjectContext {
        org: repo_ctx.org.clone(),
        repo: repo_ctx.repo.clone(),
        project_num: project_num_str,
        project_id,
        status_field_id,
    };

    // Cache it
    let json = serde_json::to_string_pretty(&ctx)?;
    std::fs::write(&cache_path, &json).ok();

    eprintln!(
        "✓ Project setup: {}, project #{}",
        ctx.repo, ctx.project_num
    );
    Ok(ctx)
}

/// Load status options for the project's Status field.
pub fn load_status_options(status_field_id: &str) -> Result<Vec<StatusOption>> {
    let query = r#"
        query($fieldId: ID!) {
            node(id: $fieldId) {
                ... on ProjectV2SingleSelectField {
                    options { id name }
                }
            }
        }
    "#;

    let result = gh_runner::gh_graphql(query, &[("fieldId", status_field_id)])?;
    let options = result["data"]["node"]["options"]
        .as_array()
        .context("Could not fetch status options")?;

    let mut status_options = Vec::new();
    for opt in options {
        status_options.push(StatusOption {
            id: opt["id"].as_str().unwrap_or("").to_string(),
            name: opt["name"].as_str().unwrap_or("").to_string(),
        });
    }
    Ok(status_options)
}

#[derive(Debug, Clone)]
pub struct StatusOption {
    pub id: String,
    pub name: String,
}

/// Resolve a status name to its option ID.
pub fn resolve_status_option_id(status_field_id: &str, status_name: &str) -> Result<String> {
    let options = load_status_options(status_field_id)?;
    for opt in &options {
        if opt.name == status_name {
            return Ok(opt.id.clone());
        }
    }
    let available: Vec<&str> = options.iter().map(|o| o.name.as_str()).collect();
    bail!(
        "Status '{}' not found in project. Available statuses: {:?}. \
         Hint: use one of the listed status names.",
        status_name,
        available
    );
}

/// Load repository metadata (repo ID and issue type IDs).
pub fn load_repo_meta(owner: &str, repo: &str) -> Result<RepoMeta> {
    let cache_path = repo_meta_cache_path(owner, repo);

    // Try cache
    if let Ok(data) = std::fs::read_to_string(&cache_path) {
        if let Ok(meta) = serde_json::from_str::<RepoMeta>(&data) {
            eprintln!("✓ Repo metadata (cached)");
            return Ok(meta);
        }
    }

    eprintln!("→ Fetching repository metadata...");

    let query = r#"
        query($owner: String!, $repo: String!) {
            repository(owner: $owner, name: $repo) { id }
        }
    "#;
    let result = gh_runner::gh_graphql(query, &[("owner", owner), ("repo", repo)])?;
    let repo_id = result["data"]["repository"]["id"]
        .as_str()
        .context("Could not fetch repository ID")?
        .to_string();

    // Fetch issue type IDs
    let type_query = r#"
        query($owner: String!, $repo: String!) {
            repository(owner: $owner, name: $repo) {
                issueTypes(first: 20) { nodes { id name } }
            }
        }
    "#;
    let type_result = gh_runner::gh_graphql_with_headers(
        type_query,
        &[("owner", owner), ("repo", repo)],
        &[],
        &["GraphQL-Features: issue_types"],
    )?;

    let issue_type_ids = type_result["data"]["repository"]["issueTypes"]["nodes"].clone();

    let meta = RepoMeta {
        repo_id,
        issue_type_ids,
    };

    let json = serde_json::to_string_pretty(&meta)?;
    std::fs::write(&cache_path, &json).ok();

    Ok(meta)
}

/// Resolve an issue type name to its ID.
pub fn resolve_issue_type_id(
    issue_type_ids: &serde_json::Value,
    type_name: &str,
) -> Result<String> {
    let lower = type_name.to_lowercase();
    let normalized = match lower.as_str() {
        "epic" => "Epic",
        "story" => "Story",
        "task" => "Task",
        "bug" => "Bug",
        other => other,
    };

    if let Some(types) = issue_type_ids.as_array() {
        for t in types {
            if t["name"].as_str() == Some(normalized) {
                return t["id"]
                    .as_str()
                    .map(|s| s.to_string())
                    .context("Issue type has no ID");
            }
        }
    }

    bail!(
        "Issue type '{}' not found. Available types: {:?}. \
         Hint: use one of: epic, story, task, bug.",
        type_name,
        issue_type_ids
    );
}
