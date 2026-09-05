use std::collections::HashMap;

use clap::Parser;

use smith::agent_cli::{
    AgentCli, AgentCommand, GithubCommand, IssueCommand, MilestoneCommand, PrCommand,
    PrReviewCommand, StatusCommand, SubIssueCommand,
};
use smith::agent_commands::github::command_graph;
use smith::agent_commands::github::envelope::{
    Envelope, EXIT_DISCOVERY, EXIT_FAILURE, EXIT_SUCCESS,
};

fn main() {
    std::env::set_var("NO_COLOR", "1");

    let cli = match AgentCli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            let envelope = Envelope::discovery(
                "Invalid or incomplete command",
                vec![
                    "smith-agent github --org <org> --help".to_string(),
                    "smith-agent github --org <org> auth-status".to_string(),
                    "smith-agent github --org <org> issue --help".to_string(),
                    "smith-agent github --org <org> pr --help".to_string(),
                ],
            );
            envelope.print();
            eprintln!("{}", e);
            std::process::exit(EXIT_DISCOVERY);
        }
    };

    match cli.command {
        AgentCommand::Github {
            id,
            org,
            repo,
            project,
            command,
        } => match command {
            Some(cmd) => {
                // Determine DOT node ID before consuming the command
                let dot_op = dot_node_id(&cmd);

                // Capture stdout from operation
                let tmp = tempfile::NamedTempFile::new().unwrap();
                let tmp_path = tmp.path().to_path_buf();

                let result = {
                    use std::os::unix::io::AsRawFd;
                    let stdout_fd = unsafe { libc::dup(1) };
                    let file_fd = tmp.as_file().as_raw_fd();
                    unsafe { libc::dup2(file_fd, 1) };

                    let r = smith::agent_commands::github::run(
                        &id,
                        &org,
                        repo.as_deref(),
                        project,
                        cmd,
                    );

                    use std::io::Write;
                    std::io::stdout().flush().ok();
                    unsafe {
                        libc::dup2(stdout_fd, 1);
                        libc::close(stdout_fd);
                    }
                    r
                };

                let output = std::fs::read_to_string(&tmp_path).unwrap_or_default();
                drop(tmp);

                // Build template variables for graph traversal
                let mut vars: HashMap<&str, String> = HashMap::new();
                vars.insert("org", org.clone());
                if let Some(ref r) = repo {
                    vars.insert("repo", r.clone());
                }
                if let Some(p) = project {
                    vars.insert("project", p.to_string());
                }

                // Try to extract key fields from the output for template rendering
                if let Ok(result_json) = serde_json::from_str::<serde_json::Value>(output.trim()) {
                    if let Some(n) = result_json["number"].as_u64() {
                        vars.insert("number", n.to_string());
                    }
                    if let Some(n) = result_json["pr"].as_u64().or(result_json["number"].as_u64()) {
                        vars.insert("pr", n.to_string());
                    }
                    if let Some(s) = result_json["state"].as_str() {
                        vars.insert("state", s.to_string());
                    }
                }

                match result {
                    Ok(()) => {
                        let result_value = if output.trim().is_empty() {
                            serde_json::Value::Null
                        } else {
                            serde_json::from_str(output.trim())
                                .unwrap_or(serde_json::Value::String(output.trim().to_string()))
                        };

                        let (next, related) = command_graph::hints_for(&dot_op, &vars);
                        let envelope = Envelope::success(
                            "Operation completed",
                            result_value,
                            next,
                            related,
                        );
                        envelope.print();
                        std::process::exit(EXIT_SUCCESS);
                    }
                    Err(e) => {
                        let recovery = command_graph::recovery_for(&dot_op, &vars);
                        let envelope = Envelope::from_anyhow(&e, recovery);
                        envelope.print();
                        std::process::exit(EXIT_FAILURE);
                    }
                }
            }
            None => {
                let envelope = Envelope::discovery(
                    "GitHub operations",
                    vec![
                        format!("smith-agent github --org {} auth-status", org),
                        format!("smith-agent github --org {} --project <N> board", org),
                        format!(
                            "smith-agent github --org {} issue <create|view|query|close|reopen|comment|assign|update>",
                            org
                        ),
                        format!(
                            "smith-agent github --org {} --project <N> status set <number> --to <status>",
                            org
                        ),
                        format!(
                            "smith-agent github --org {} pr <create|view|list|merge|approve|request-changes|comment|close>",
                            org
                        ),
                        format!(
                            "smith-agent github --org {} sub-issue <create|list|status>",
                            org
                        ),
                        format!(
                            "smith-agent github --org {} milestone <list|create|assign>",
                            org
                        ),
                        format!(
                            "smith-agent github --org {} fork --source <owner/repo> --target-org <org>",
                            org
                        ),
                    ],
                );
                envelope.print();
                std::process::exit(EXIT_DISCOVERY);
            }
        },
    };
}

/// Map a GithubCommand to its DOT node ID in the command graph.
fn dot_node_id(cmd: &GithubCommand) -> String {
    match cmd {
        GithubCommand::AuthStatus => "auth_status".to_string(),
        GithubCommand::MintToken => "mint_token".to_string(),
        GithubCommand::ListProjects => "list_projects".to_string(),
        GithubCommand::Board => "board".to_string(),
        GithubCommand::Issue { command } => match command {
            IssueCommand::Create { .. } => "issue_create".to_string(),
            IssueCommand::View { .. } => "issue_view".to_string(),
            IssueCommand::Query { .. } => "issue_query".to_string(),
            IssueCommand::Close { .. } => "issue_close".to_string(),
            IssueCommand::Reopen { .. } => "issue_reopen".to_string(),
            IssueCommand::Comment { .. } => "issue_comment".to_string(),
            IssueCommand::Assign { .. } => "issue_assign".to_string(),
            IssueCommand::Update { .. } => "issue_update".to_string(),
        },
        GithubCommand::Status { command } => match command {
            StatusCommand::Set { .. } => "status_set".to_string(),
        },
        GithubCommand::Pr { command } => match command {
            PrCommand::Create { .. } => "pr_create".to_string(),
            PrCommand::View { .. } => "pr_view".to_string(),
            PrCommand::List { .. } => "pr_list".to_string(),
            PrCommand::Merge { .. } => "pr_merge".to_string(),
            PrCommand::Approve { .. } => "pr_approve".to_string(),
            PrCommand::RequestChanges { .. } => "pr_request_changes".to_string(),
            PrCommand::Comment { .. } => "pr_comment".to_string(),
            PrCommand::Close { .. } => "pr_close".to_string(),
            PrCommand::Review { command } => match command {
                PrReviewCommand::AddComment { .. } => "pr_review_add_comment".to_string(),
                PrReviewCommand::Submit { .. } => "pr_review_submit".to_string(),
                PrReviewCommand::Show { .. } => "pr_review_show".to_string(),
                PrReviewCommand::Clear { .. } => "pr_review_clear".to_string(),
            },
        },
        GithubCommand::SubIssue { command } => match command {
            SubIssueCommand::Create { .. } => "sub_issue_create".to_string(),
            SubIssueCommand::List { .. } => "sub_issue_list".to_string(),
            SubIssueCommand::Status { .. } => "sub_issue_status".to_string(),
        },
        GithubCommand::Milestone { command } => match command {
            MilestoneCommand::List => "milestone_list".to_string(),
            MilestoneCommand::Create { .. } => "milestone_create".to_string(),
            MilestoneCommand::Assign { .. } => "milestone_assign".to_string(),
        },
        GithubCommand::Fork { .. } => "fork_op".to_string(),
    }
}
