use clap::{Parser, Subcommand};

fn build_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Loopsmith agent CLI — machine-readable interface for coding agents
#[derive(Parser)]
#[command(name = "smith-agent", version = build_version(), about, next_display_order = None)]
pub struct AgentCli {
    #[command(subcommand)]
    pub command: AgentCommand,
}

#[derive(Subcommand)]
pub enum AgentCommand {
    /// GitHub operations — issues, PRs, board, status transitions
    Github {
        /// Agent identity (for credential lookup, default: "smith")
        #[arg(long, default_value = "smith")]
        id: String,

        /// GitHub organization
        #[arg(long)]
        org: String,

        /// Repository name (default: inferred from git remote in cwd)
        #[arg(long)]
        repo: Option<String>,

        /// GitHub Projects V2 board number (required for board/status operations)
        #[arg(long)]
        project: Option<u64>,

        #[command(subcommand)]
        command: Option<GithubCommand>,
    },
}

#[derive(Subcommand)]
pub enum GithubCommand {
    /// Show authentication status and token validity
    AuthStatus,

    /// Mint a token and write a persistent GH_CONFIG_DIR for git/gh use
    MintToken,

    /// List GitHub Projects V2 for the org
    ListProjects,

    /// Show project board grouped by status (requires --project)
    Board,

    /// Issue operations
    Issue {
        #[command(subcommand)]
        command: IssueCommand,
    },

    /// Status transitions on project board (requires --project)
    Status {
        #[command(subcommand)]
        command: StatusCommand,
    },

    /// Pull request operations
    Pr {
        #[command(subcommand)]
        command: PrCommand,
    },

    /// Sub-issue operations (GitHub native sub-issues)
    SubIssue {
        #[command(subcommand)]
        command: SubIssueCommand,
    },

    /// Milestone management
    Milestone {
        #[command(subcommand)]
        command: MilestoneCommand,
    },

    /// Fork a repository into an organization
    Fork {
        /// Source repository (e.g. "openshift/enhancements")
        #[arg(long)]
        source: String,

        /// Target organization to fork into
        #[arg(long = "target-org")]
        target_org: String,
    },
}

#[derive(Subcommand)]
pub enum IssueCommand {
    /// Create a new issue (epic, story, task, or bug)
    Create {
        /// Issue title
        #[arg(long)]
        title: String,

        /// Issue body/description
        #[arg(long)]
        body: String,

        /// Issue kind: epic, story, task, or bug
        #[arg(long)]
        kind: String,

        /// Parent issue number (for sub-issues)
        #[arg(long)]
        parent: Option<u64>,

        /// Milestone to assign
        #[arg(long)]
        milestone: Option<String>,

        /// Assignee username
        #[arg(long)]
        assignee: Option<String>,

        /// Initial status on the project board
        #[arg(long, default_value = "Backlog")]
        initial_status: String,
    },

    /// View a single issue with full details
    View {
        /// Issue number
        issue: u64,

        /// Include last 20 comments
        #[arg(long, default_value_t = false)]
        comments: bool,
    },

    /// Query issues by various filters
    Query {
        /// Query type: label, status, milestone, assignee, project-status, issue-type
        #[arg(long)]
        by: String,

        /// Label or issue-type name
        #[arg(long)]
        label: Option<String>,

        /// Status name (for project status queries)
        #[arg(long)]
        status: Option<String>,

        /// Milestone title
        #[arg(long)]
        milestone: Option<String>,

        /// Assignee username
        #[arg(long)]
        assignee: Option<String>,

        /// Issue number (for single or project-status queries)
        #[arg(long)]
        issue: Option<u64>,
    },

    /// Close an issue
    Close {
        /// Issue number
        issue: u64,
    },

    /// Reopen an issue
    Reopen {
        /// Issue number
        issue: u64,
    },

    /// Add a comment to an issue
    Comment {
        /// Issue number
        issue: u64,

        /// Comment body
        #[arg(long)]
        body: String,
    },

    /// Assign or unassign a user
    Assign {
        /// Issue number
        issue: u64,

        /// Username
        #[arg(long)]
        user: String,

        /// Action: "assign" or "unassign"
        #[arg(long, default_value = "assign")]
        action: String,
    },

    /// Update issue title and/or body
    Update {
        /// Issue number
        issue: u64,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New body
        #[arg(long)]
        body: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum StatusCommand {
    /// Set an issue's status on the project board (requires --project)
    Set {
        /// Issue number
        issue: u64,

        /// Target status name
        #[arg(long)]
        to: String,

        /// Previous status (for rollback on failure)
        #[arg(long)]
        from: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum PrCommand {
    /// Create a pull request
    Create {
        /// PR title
        #[arg(long)]
        title: String,

        /// PR body
        #[arg(long)]
        body: String,

        /// Head branch
        #[arg(long)]
        branch: String,

        /// Base branch
        #[arg(long, default_value = "main")]
        base: String,

        /// Create as draft
        #[arg(long, default_value_t = false)]
        draft: bool,
    },

    /// View a pull request
    View {
        /// PR number
        pr: u64,
    },

    /// List open pull requests
    List {
        /// Optional search filter
        #[arg(long)]
        search: Option<String>,
    },

    /// Merge a pull request
    Merge {
        /// PR number
        pr: u64,

        /// Merge method: merge, squash, or rebase
        #[arg(long, default_value = "squash")]
        method: String,
    },

    /// Approve a pull request
    Approve {
        /// PR number
        pr: u64,

        /// Optional review body
        #[arg(long)]
        body: Option<String>,
    },

    /// Request changes on a pull request
    RequestChanges {
        /// PR number
        pr: u64,

        /// Review body (required)
        #[arg(long)]
        body: String,
    },

    /// Comment on a pull request
    Comment {
        /// PR number
        pr: u64,

        /// Comment body
        #[arg(long)]
        body: String,
    },

    /// Close a pull request
    Close {
        /// PR number
        pr: u64,
    },

    /// Inline review operations (cached batch reviews)
    Review {
        #[command(subcommand)]
        command: PrReviewCommand,
    },
}

#[derive(Subcommand)]
pub enum PrReviewCommand {
    /// Cache an inline comment for later submission
    AddComment {
        /// PR number
        pr: u64,

        /// File path in the diff
        #[arg(long)]
        path: String,

        /// Line number (end line for ranges)
        #[arg(long)]
        line: u64,

        /// Start line (for multi-line comments)
        #[arg(long)]
        start_line: Option<u64>,

        /// Comment body
        #[arg(long)]
        body: String,

        /// Subject type: "line" or "file"
        #[arg(long, default_value = "line")]
        subject_type: String,
    },

    /// Submit all cached comments as a single review
    Submit {
        /// PR number
        pr: u64,

        /// Review event: APPROVE, REQUEST_CHANGES, or COMMENT
        #[arg(long, default_value = "COMMENT")]
        event: String,

        /// Optional review body
        #[arg(long)]
        body: Option<String>,
    },

    /// Show cached comments (no API calls)
    Show {
        /// PR number
        pr: u64,
    },

    /// Clear cached comments
    Clear {
        /// PR number
        pr: u64,
    },
}

#[derive(Subcommand)]
pub enum SubIssueCommand {
    /// Create a sub-issue under a parent
    Create {
        /// Parent issue number
        #[arg(long)]
        parent: u64,

        /// Sub-issue title
        #[arg(long)]
        title: String,

        /// Sub-issue body
        #[arg(long)]
        body: Option<String>,

        /// Issue type (default: Story)
        #[arg(long, default_value = "Story")]
        issue_type: String,
    },

    /// List sub-issues of a parent
    List {
        /// Parent issue number
        #[arg(long)]
        parent: u64,
    },

    /// Show completion status of sub-issues
    Status {
        /// Parent issue number
        #[arg(long)]
        parent: u64,
    },
}

#[derive(Subcommand)]
pub enum MilestoneCommand {
    /// List milestones
    List,

    /// Create a milestone
    Create {
        /// Milestone title
        #[arg(long)]
        title: String,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_date: Option<String>,
    },

    /// Assign an issue to a milestone
    Assign {
        /// Issue number
        #[arg(long)]
        issue: u64,

        /// Milestone title
        #[arg(long)]
        title: String,
    },
}
