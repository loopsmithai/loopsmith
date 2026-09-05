mod auth;
mod board;
pub mod command_graph;
pub mod envelope;
mod fork;
mod gh_runner;
mod issue;
mod milestone;
mod pr;
mod setup;
mod status;
mod sub_issue;

use crate::agent_cli::{
    GithubCommand, IssueCommand, MilestoneCommand, PrCommand, PrReviewCommand, StatusCommand,
    SubIssueCommand,
};
use anyhow::{bail, Result};

/// Run a github subcommand.
///
/// - `id`: agent identity (keyring key prefix, default "smith")
/// - `org`: GitHub organization
/// - `repo`: optional repo name (inferred from git remote if absent)
/// - `project`: optional project number (required for board/status ops)
pub fn run(
    id: &str,
    org: &str,
    repo: Option<&str>,
    project: Option<u64>,
    command: GithubCommand,
) -> Result<()> {
    // Authenticate first — get a fresh installation token
    let _token_guard = auth::setup_github_auth(org, id)?;

    match command {
        GithubCommand::AuthStatus => auth::print_auth_status(org, id),
        GithubCommand::MintToken => auth::mint_token_to_config_dir(org, id),

        GithubCommand::ListProjects => {
            let json = gh_runner::gh(&[
                "project", "list", "--owner", org, "--format", "json",
            ])?;
            let projects: serde_json::Value = serde_json::from_str(&json)?;
            println!("{}", serde_json::to_string_pretty(&projects)?);
            Ok(())
        }

        GithubCommand::Board => {
            let project_num = require_project(project)?;
            // Board operates on the project (org-level), not a specific repo.
            let org_ctx = setup::RepoContext {
                org: org.to_string(),
                repo: format!("{}/.", org),
            };
            let project_ctx = setup::load_or_fetch_project(&org_ctx, project_num)?;
            board::board_view(&project_ctx)
        }

        GithubCommand::Issue { command } => {
            let repo_ctx = setup::build_repo_context(org, repo)?;
            match command {
                IssueCommand::Create {
                    title,
                    body,
                    kind,
                    parent,
                    milestone,
                    assignee,
                    initial_status,
                } => {
                    // Create needs a project for status assignment
                    let project_num = require_project(project)?;
                    let project_ctx =
                        setup::load_or_fetch_project(&repo_ctx, project_num)?;
                    let repo_meta =
                        setup::load_repo_meta(repo_ctx.owner(), repo_ctx.repo_name())?;
                    issue::create(
                        &project_ctx,
                        &repo_meta,
                        &title,
                        &body,
                        &kind,
                        parent,
                        milestone.as_deref(),
                        assignee.as_deref(),
                        &initial_status,
                    )
                }
                IssueCommand::View { issue, comments } => {
                    issue::view(&repo_ctx, issue, comments)
                }
                IssueCommand::Query {
                    by,
                    label,
                    status,
                    milestone,
                    assignee,
                    issue: issue_num,
                } => {
                    // Status query needs project context
                    if by == "status" || by == "project-status" {
                        let project_num = require_project(project)?;
                        let project_ctx =
                            setup::load_or_fetch_project(&repo_ctx, project_num)?;
                        issue::query_with_project(
                            &project_ctx,
                            &by,
                            label.as_deref(),
                            status.as_deref(),
                            milestone.as_deref(),
                            assignee.as_deref(),
                            issue_num,
                        )
                    } else {
                        issue::query(
                            &repo_ctx,
                            &by,
                            label.as_deref(),
                            status.as_deref(),
                            milestone.as_deref(),
                            assignee.as_deref(),
                            issue_num,
                        )
                    }
                }
                IssueCommand::Close { issue } => issue::close(&repo_ctx, issue),
                IssueCommand::Reopen { issue } => issue::reopen(&repo_ctx, issue),
                IssueCommand::Comment { issue, body } => {
                    issue::comment(&repo_ctx, issue, &body)
                }
                IssueCommand::Assign {
                    issue,
                    user,
                    action,
                } => issue::assign(&repo_ctx, issue, &user, &action),
                IssueCommand::Update { issue, title, body } => {
                    issue::update(&repo_ctx, issue, title.as_deref(), body.as_deref())
                }
            }
        }

        GithubCommand::Status { command } => {
            let project_num = require_project(project)?;
            let repo_ctx = setup::build_repo_context(org, repo)?;
            let project_ctx = setup::load_or_fetch_project(&repo_ctx, project_num)?;
            match command {
                StatusCommand::Set { issue, to, from } => {
                    status::set(&project_ctx, issue, &to, from.as_deref())
                }
            }
        }

        GithubCommand::Pr { command } => {
            let repo_ctx = setup::build_repo_context(org, repo)?;
            match command {
                PrCommand::Create {
                    title,
                    body,
                    branch,
                    base,
                    draft,
                } => pr::create(&repo_ctx, &title, &body, &branch, &base, draft),
                PrCommand::View { pr: pr_num } => pr::view(&repo_ctx, pr_num),
                PrCommand::List { search } => pr::list(&repo_ctx, search.as_deref()),
                PrCommand::Merge { pr: pr_num, method } => {
                    pr::merge(&repo_ctx, pr_num, &method)
                }
                PrCommand::Approve { pr: pr_num, body } => {
                    pr::approve(&repo_ctx, pr_num, body.as_deref())
                }
                PrCommand::RequestChanges { pr: pr_num, body } => {
                    pr::request_changes(&repo_ctx, pr_num, &body)
                }
                PrCommand::Comment { pr: pr_num, body } => {
                    pr::comment(&repo_ctx, pr_num, &body)
                }
                PrCommand::Close { pr: pr_num } => pr::close(&repo_ctx, pr_num),
                PrCommand::Review { command } => match command {
                    PrReviewCommand::AddComment { .. } => {
                        bail!(
                            "pr review add-comment: not yet implemented. \
                             Hint: this is the cached inline review system — coming soon."
                        )
                    }
                    PrReviewCommand::Submit { .. } => {
                        bail!("pr review submit: not yet implemented.")
                    }
                    PrReviewCommand::Show { .. } => {
                        bail!("pr review show: not yet implemented.")
                    }
                    PrReviewCommand::Clear { .. } => {
                        bail!("pr review clear: not yet implemented.")
                    }
                },
            }
        }

        GithubCommand::SubIssue { command } => {
            let repo_ctx = setup::build_repo_context(org, repo)?;
            let repo_meta = setup::load_repo_meta(repo_ctx.owner(), repo_ctx.repo_name())?;
            match command {
                SubIssueCommand::Create {
                    parent,
                    title,
                    body,
                    issue_type,
                } => sub_issue::create(&repo_ctx, &repo_meta, parent, &title, body.as_deref(), &issue_type),
                SubIssueCommand::List { parent } => sub_issue::list(&repo_ctx, parent),
                SubIssueCommand::Status { parent } => sub_issue::status(&repo_ctx, parent),
            }
        }

        GithubCommand::Milestone { command } => {
            let repo_ctx = setup::build_repo_context(org, repo)?;
            match command {
                MilestoneCommand::List => milestone::list(&repo_ctx),
                MilestoneCommand::Create {
                    title,
                    description,
                    due_date,
                } => milestone::create(
                    &repo_ctx,
                    &title,
                    description.as_deref(),
                    due_date.as_deref(),
                ),
                MilestoneCommand::Assign { issue, title } => {
                    milestone::assign(&repo_ctx, issue, &title)
                }
            }
        }

        GithubCommand::Fork { source, target_org } => fork::fork_repo(&source, &target_org),
    }
}

/// Require --project flag, returning an error envelope hint if missing.
fn require_project(project: Option<u64>) -> Result<u64> {
    project.ok_or_else(|| {
        anyhow::anyhow!(
            "--project is required for this operation. \
             Hint: list projects with 'gh project list --owner <org>', \
             then pass --project <number>."
        )
    })
}
