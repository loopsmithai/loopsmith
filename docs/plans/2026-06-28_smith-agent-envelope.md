# Plan: smith-agent Envelope Format & LLM-Native CLI

## Goal

Every smith-agent response is one JSON envelope on stdout. No bare JSON, no
unstructured text. The envelope carries what happened, the data, what to do next,
what else exists, and on failure, how to recover. smith-agent is an API surface
for coding agents, not a human CLI.

## The envelope contract

```json
{
  "summary": "One sentence: what happened",
  "result": { ... },
  "next": ["exact runnable commands for logical next steps"],
  "related": ["commands the agent might not know about"],
  "error": {                          // only on exit 1
    "message": "what went wrong",
    "chain": ["cause chain"],
    "recovery": ["exact commands to fix it"]
  }
}
```

Exit codes:
- **0** — operation completed successfully. `result` is present.
- **1** — operation failed. `error` is present with `recovery` hints.
- **2** — discovery, no operation executed. `next` lists available subcommands.

Rules:
- `summary` is always present, always one sentence, always describes the outcome
- `result` is the operation-specific data — only on exit 0
- `error` with `recovery` — only on exit 1
- `next` contains **runnable commands** with the real `--org` value baked in — never
  `<placeholder>` except for values the agent must choose (like `<username>`)
- `related` is discovery — operations the agent probably doesn't know about
- `error.recovery` is specific: "run this command" not "check permissions"
- stderr is progress only (`→ Generating JWT...`, `✓ Authenticated...`) — agents can
  ignore it safely

## Per-operation specification

All commands below assume `T` = the `--team` value (threaded through at runtime).

### auth-status

```
summary: "Authenticated as GitHub App {app_id} (installation {inst_id}) for team {T}"
result: { team, member, app_id, client_id, installation_id, has_private_key }
next:
  - "View the project board: smith-agent github --team {T} board"
  - "List issues by type: smith-agent github --team {T} issue query --by issue-type --label Epic"
related:
  - "Create an issue: smith-agent github --team {T} issue create --title <title> --body <body> --kind <epic|story|task|bug>"
```

On error (no credentials):
```
recovery:
  - "Register a GitHub App: smith init"
  - "If the App was uninstalled: re-run smith init for this team"
```

### board

```
summary: "Board for {team_repo} project #{project_num}: {n} items across {k} statuses"
result: { items: [...], totalCount: N }
next:
  - "Query by status: smith-agent github --team {T} issue query --by status --status <status_name>"
  - "View a specific issue: smith-agent github --team {T} issue view <number>"
related:
  - "Create an issue: smith-agent github --team {T} issue create --title <title> --body <body> --kind <epic|story|task|bug>"
  - "Transition status: smith-agent github --team {T} status set <number> --to <status>"
  - "Query by assignee: smith-agent github --team {T} issue query --by assignee --assignee <username>"
```

### issue create

```
summary: "Created {kind} #{number} '{title}' in {team_repo}, status: {initial_status}"
result: { created: true, number, title, kind, status, url, parent?, assignee?, milestone? }
next:
  - "View it: smith-agent github --team {T} issue view {number}"
  - "Transition status: smith-agent github --team {T} status set {number} --to <status> --from {initial_status}"
  - "Assign someone: smith-agent github --team {T} issue assign {number} --user <username>"
  - (if kind=epic) "Add a sub-issue: smith-agent github --team {T} sub-issue create --parent {number} --title <title> --issue-type Story"
related:
  - "View the board: smith-agent github --team {T} board"
  - "Query epics: smith-agent github --team {T} issue query --by issue-type --label Epic"
```

On error (issue type not found):
```
recovery:
  - "Available kinds: epic, story, task, bug"
  - "Check available issue types: smith-agent github --team {T} issue query --by issue-type --label Epic"
```

On error (parent not found):
```
recovery:
  - "Verify the parent issue exists: smith-agent github --team {T} issue view {parent_num}"
```

### issue view

```
summary: "Issue #{number} '{title}' ({state}, type: {issueType}, status: {boardStatus})"
result: { number, title, state, body, issueType, labels, assignees, milestone, parent, subIssues, projectItems, comments? }
next:
  - "Transition status: smith-agent github --team {T} status set {number} --to <status> --from {current_status}"
  - "Add a comment: smith-agent github --team {T} issue comment {number} --body <text>"
  - (if has sub-issues) "Check sub-issue completion: smith-agent github --team {T} sub-issue status --parent {number}"
  - (if state=OPEN) "Close it: smith-agent github --team {T} issue close {number}"
  - (if state=CLOSED) "Reopen it: smith-agent github --team {T} issue reopen {number}"
related:
  - "Update title/body: smith-agent github --team {T} issue update {number} --title <new_title>"
  - "Assign/unassign: smith-agent github --team {T} issue assign {number} --user <username>"
  - (if no sub-issues and type=Epic) "Add sub-issues: smith-agent github --team {T} sub-issue create --parent {number} --title <title>"
```

### issue query

```
summary: "Found {n} issues matching {query_type}={value}"
result: [ ...issues... ]
next:
  - "View details: smith-agent github --team {T} issue view <number>"
  - (if query_type=status) "Transition one: smith-agent github --team {T} status set <number> --to <next_status>"
related:
  - "Other query types: --by label|status|milestone|assignee|issue-type"
  - "Full board view: smith-agent github --team {T} board"
```

### issue close

```
summary: "Closed issue #{number}"
result: { closed: true, number }
next:
  - "Reopen it: smith-agent github --team {T} issue reopen {number}"
  - "View it: smith-agent github --team {T} issue view {number}"
related:
  - "View the board: smith-agent github --team {T} board"
```

### issue reopen

```
summary: "Reopened issue #{number}"
result: { reopened: true, number }
next:
  - "Transition status: smith-agent github --team {T} status set {number} --to <status>"
  - "View it: smith-agent github --team {T} issue view {number}"
related:
  - "Close it again: smith-agent github --team {T} issue close {number}"
```

### issue comment

```
summary: "Added comment to issue #{number}"
result: { commented: true, number }
next:
  - "View the issue (with comments): smith-agent github --team {T} issue view {number} --comments"
  - "Transition status: smith-agent github --team {T} status set {number} --to <status>"
related:
  - "Assign someone: smith-agent github --team {T} issue assign {number} --user <username>"
```

### issue assign

```
summary: "{action} {user} on issue #{number}"  // "Assigned devguyio on issue #42" or "Unassigned..."
result: { action, number, user }
next:
  - "View the issue: smith-agent github --team {T} issue view {number}"
  - (if action=assign) "Unassign: smith-agent github --team {T} issue assign {number} --user {user} --action unassign"
  - (if action=unassign) "Re-assign: smith-agent github --team {T} issue assign {number} --user <username>"
related:
  - "Query by assignee: smith-agent github --team {T} issue query --by assignee --assignee {user}"
```

### issue update

```
summary: "Updated issue #{number}: {changed_fields}"  // "title" or "body" or "title and body"
result: { updated: true, number }
next:
  - "View it: smith-agent github --team {T} issue view {number}"
related:
  - "Add a comment about the update: smith-agent github --team {T} issue comment {number} --body <text>"
```

### status set

```
summary: "Transitioned #{number}: {from} → {to} (verified)"
result: { transitioned: true, number, from, to, verified: true }
next:
  - "View the issue: smith-agent github --team {T} issue view {number}"
  - "Comment on the transition: smith-agent github --team {T} issue comment {number} --body <text>"
  - (workflow-aware, if known) "Next status: smith-agent github --team {T} status set {number} --to <next_in_workflow> --from {to}"
related:
  - "View the board: smith-agent github --team {T} board"
```

On error (status not found):
```
recovery:
  - "View available statuses on the board: smith-agent github --team {T} board"
  - "Common statuses in this project: {list from cached status options}"
```

On error (verification failed):
```
recovery:
  - "The status field may not have updated. Retry: smith-agent github --team {T} status set {number} --to {to}"
  - "Check App permissions: the GitHub App needs 'organization_projects: admin'"
```

On error (issue not in project):
```
recovery:
  - "The issue may not be added to the project board. View it first: smith-agent github --team {T} issue view {number}"
  - "Check if it exists: the issue may have been deleted or moved"
```

### pr create

```
summary: "Created PR from {branch} → {base} in {team_repo}"
result: { url, number? (if parseable from gh output) }
next:
  - "View the PR: smith-agent github --team {T} pr view <number>"
  - "List PRs: smith-agent github --team {T} pr list"
related:
  - "Create a draft PR: add --draft flag"
```

### pr view

```
summary: "PR #{number} '{title}' ({state}, review: {reviewDecision})"
result: { full PR JSON from gh }
next:
  - (if state=OPEN) "Approve: smith-agent github --team {T} pr approve {number} --body <text>"
  - (if state=OPEN) "Request changes: smith-agent github --team {T} pr request-changes {number} --body <text>"
  - (if state=OPEN, review=APPROVED) "Merge: smith-agent github --team {T} pr merge {number} --method squash"
  - (if state=OPEN) "Comment: smith-agent github --team {T} pr comment {number} --body <text>"
  - (if state=OPEN) "Close: smith-agent github --team {T} pr close {number}"
related:
  - "List all PRs: smith-agent github --team {T} pr list"
```

### pr list

```
summary: "Found {n} open PRs in {team_repo}"
result: [ ...PRs... ]
next:
  - "View a PR: smith-agent github --team {T} pr view <number>"
  - "Create a new PR: smith-agent github --team {T} pr create --title <t> --body <b> --branch <head> --base main"
related:
  - "Search PRs: smith-agent github --team {T} pr list --search 'author:username is:open'"
```

### pr merge

```
summary: "Merged PR #{number} via {method}"
result: { merged: true, number, method }
next:
  - "View the merged PR: smith-agent github --team {T} pr view {number}"
  - "View the board: smith-agent github --team {T} board"
related:
  - "Close related issues: smith-agent github --team {T} issue close <number>"
```

### pr approve / request-changes / comment

```
summary: "Approved PR #{number}" / "Requested changes on PR #{number}" / "Commented on PR #{number}"
result: { approved/requested_changes/commented: true, number }
next:
  - "View the PR: smith-agent github --team {T} pr view {number}"
  - (if approved) "Merge: smith-agent github --team {T} pr merge {number} --method squash"
related:
  - "List PRs: smith-agent github --team {T} pr list"
```

### pr close

```
summary: "Closed PR #{number}"
result: { closed: true, number }
next:
  - "View the board: smith-agent github --team {T} board"
related:
  - "List remaining PRs: smith-agent github --team {T} pr list"
```

### sub-issue create

```
summary: "Created {issue_type} #{number} as sub-issue of #{parent}"
result: { number, title, url, parent }
next:
  - "View parent: smith-agent github --team {T} issue view {parent}"
  - "Check completion: smith-agent github --team {T} sub-issue status --parent {parent}"
  - "Create another: smith-agent github --team {T} sub-issue create --parent {parent} --title <title>"
related:
  - "List all sub-issues: smith-agent github --team {T} sub-issue list --parent {parent}"
```

### sub-issue list

```
summary: "{n} sub-issues under #{parent}"
result: [ ...sub-issues... ]
next:
  - "Check completion: smith-agent github --team {T} sub-issue status --parent {parent}"
  - "Add another: smith-agent github --team {T} sub-issue create --parent {parent} --title <title>"
  - "View a sub-issue: smith-agent github --team {T} issue view <number>"
related:
  - "View parent: smith-agent github --team {T} issue view {parent}"
```

### sub-issue status

```
summary: "#{parent}: {closed}/{total} sub-issues complete ({open} remaining)"
result: { parent, title, total, closed, open, complete }
next:
  - (if not complete) "View open sub-issues: smith-agent github --team {T} sub-issue list --parent {parent}"
  - (if complete) "Close the parent: smith-agent github --team {T} issue close {parent}"
  - (if complete) "Transition parent status: smith-agent github --team {T} status set {parent} --to <done_status>"
related:
  - "View parent: smith-agent github --team {T} issue view {parent}"
```

### milestone list

```
summary: "{n} milestones in {team_repo}"
result: [ ...milestones... ]
next:
  - "Create a milestone: smith-agent github --team {T} milestone create --title <title> --due-date YYYY-MM-DD"
  - "Assign an issue: smith-agent github --team {T} milestone assign --issue <N> --title <milestone>"
related:
  - "Query by milestone: smith-agent github --team {T} issue query --by milestone --milestone <title>"
```

### milestone create

```
summary: "Created milestone '{title}'"
result: { ...milestone JSON from API... }
next:
  - "Assign an issue: smith-agent github --team {T} milestone assign --issue <N> --title {title}"
  - "List milestones: smith-agent github --team {T} milestone list"
related:
  - "Query issues in this milestone: smith-agent github --team {T} issue query --by milestone --milestone {title}"
```

### milestone assign

```
summary: "Assigned issue #{number} to milestone '{title}'"
result: { assigned: true, number, milestone }
next:
  - "View the issue: smith-agent github --team {T} issue view {number}"
  - "Query all issues in this milestone: smith-agent github --team {T} issue query --by milestone --milestone {title}"
related:
  - "List all milestones: smith-agent github --team {T} milestone list"
```

### fork

```
summary: "Forked {source} into {org}/{repo_name}"
result: { fullName, url, sshUrl }
next:
  - "Clone it: git clone {sshUrl}"
  - "View the fork: smith-agent github --team {T} issue query --by label --label <label>"
related:
  - "Create a PR from the fork: smith-agent github --team {T} pr create --title <t> --body <b> --branch <head> --base main"
```

## Capabilities documents (no-subcommand case)

When `smith-agent github --team x` is run with no subcommand, or
`smith-agent github --team x issue` with no issue subcommand, emit a capabilities
envelope listing all available operations with usage examples.

This replaces clap's help text for the no-subcommand path. `--help` still works
for humans (clap's output), but the default "missing subcommand" path emits JSON.

## Implementation steps

### Step 1: Thread team name through operations

Currently operations receive `&ProjectSetup` which has `owner` but not the team
name as passed on the CLI. The `next`/`related` hints need the exact `--team`
value. Either:
- Add `team_name: String` to `ProjectSetup`, or
- Pass `team` as a separate arg to each operation

Decision: add to `ProjectSetup` — it's already the context bag.

### Step 2: Change all operation signatures to return `Envelope`

Every `pub fn` in issue.rs, status.rs, etc. changes from `-> Result<()>` to
`-> Result<Envelope>`. Remove all `println!()`. Build envelope with the hints
specified above.

### Step 3: Wire dispatch in mod.rs

`run()` returns `Result<Envelope>`. `agent_main.rs` calls `envelope.print()`.
On `Err`, wraps in `Envelope::error()`.

### Step 4: Catch clap errors in agent_main.rs

Override clap's error rendering so missing-subcommand and unknown-arg errors
also emit envelope JSON. Use `AgentCli::try_parse()` instead of `parse()`.

### Step 5: Disable color in clap

`#[command(disable_colored_help = true, color = clap::ColorChoice::Never)]`

### Step 6: Test every operation

Re-run the 19-operation smoke test, verify every response has the envelope
format with appropriate `next` and `related` arrays.

## Files to change

| File | Change |
|---|---|
| `envelope.rs` | Refine, add per-error recovery helpers |
| `agent_main.rs` | try_parse, print envelope, catch clap errors |
| `agent_cli.rs` | Disable color, optional subcommands |
| `mod.rs` | Return `Result<Envelope>`, capabilities dispatch |
| `setup.rs` | Add `team_name` to `ProjectSetup` |
| `issue.rs` | Return `Envelope` with per-operation hints (8 functions) |
| `status.rs` | Return `Envelope` with workflow hints |
| `board.rs` | Return `Envelope` with discovery hints |
| `pr.rs` | Return `Envelope` with PR lifecycle hints (8 functions) |
| `sub_issue.rs` | Return `Envelope` (3 functions) |
| `milestone.rs` | Return `Envelope` (3 functions) |
| `fork.rs` | Return `Envelope` |
| `auth.rs` | Return `Envelope` |

## Reviewer findings addressed

### #2 — pr review subcommands (AddComment, Submit, Show, Clear)

Currently `bail!()`. Under envelope, these emit:
```
ok: false, summary: "pr review add-comment is not yet implemented"
error.recovery: ["This operation is planned but not yet built. Use the gh CLI directly: gh api graphql ..."]
```
Four explicit error envelopes, not bare bail strings.

### #3 — fork can't access team name

Pass `team: &str` as a separate arg to `fork::fork_repo()`. Fork doesn't use
ProjectSetup at all, so threading it through the bag doesn't make sense.
Signature: `pub fn fork_repo(source: &str, org: &str, team: &str) -> Result<Envelope>`.

### #4 — fork related hint is wrong

Replace with:
```
related:
  - "View the forked repo: gh repo view {org}/{repo_name}"
```
Not a smith-agent command — fork is a one-off, there's no smith-agent view-repo.

### #5 — team_name vs owner

**Superseded by identity model plan** (`docs/plans/2026-06-28_smith-agent-identity-model.md`).
All team/member vocabulary replaced with id/org/repo/project. The envelope plan
uses `{org}` not `{team}` in templates. Execute the identity model plan first,
then the envelope plan.

### #6 — --member missing from hints

Since `--member` defaults to `"smith"` and is never changed today, omit it from
hints to keep them short. If we later support non-default members, add it then.
Document this in envelope.rs as a conscious decision.

### #7 — error channel inversion

Errors go on stdout as envelope JSON. Exit codes:
- 0 = operation succeeded, `result` present
- 1 = operation failed, `error` present
- 2 = discovery, no operation executed, `next` lists children

### #9 — pr create outputs URL not JSON

After `gh pr create`, parse the URL for the PR number, then call `gh pr view
<number> --json number,title,url,headRefName` to get structured data. If URL
parsing fails, return the raw URL in `result.url` with `result.number: null`
and a `related` hint to view it manually.

### #10 — milestone create has a pre-existing bug

Fix it during this work. Replace `--input -` with `--raw-field` args:
```
gh api repos/{repo}/milestones --method POST \
  -f title={title} -f description={desc} -f due_on={date}
```
No stdin piping needed.

### #11 — skip_serializing_if contract

Remove `skip_serializing_if` from `next` and `related`. Always emit them, even
as empty arrays. The contract is: these fields are always present. Agents don't
need to handle missing vs empty.

### #13 — optional subcommands underspecified

Make subcommands optional with `#[command(subcommand)] command: Option<GithubCommand>`.
In `mod.rs::run()`:
```rust
match command {
    None => { envelope::capabilities("github", team).print(); Ok(()) }
    Some(cmd) => { /* existing dispatch */ }
}
```
Same pattern for nested: `Option<IssueCommand>`, `Option<PrCommand>`, etc.
Each `None` branch emits the group-level capabilities document.

This changes clap's behavior: missing subcommand is no longer an error, it's
a capabilities response. `--help` still works via clap for humans.

### #17 — per-error recovery mechanism

Don't pattern-match on error strings. Instead, each operation function catches
its own errors and maps them:

```rust
pub fn set(setup: &ProjectSetup, team: &str, ...) -> Result<Envelope> {
    let option_id = match setup::resolve_status_option_id(&setup.status_field_id, to) {
        Ok(id) => id,
        Err(_) => return Ok(Envelope::error_with_recovery(
            format!("Status '{}' not found", to),
            vec![
                format!("View available statuses: smith-agent github --team {} board", team),
            ],
        )),
    };
    // ...
}
```

Each function owns its error mapping. No central error-type registry. Add
`Envelope::error_with_recovery(message, recovery)` as a convenience constructor.

### #18 — capabilities function signature

Change `capabilities(group: &str)` → `capabilities(group: &str, team: &str)`.
Hints use the real team value.

### #19 — gh_runner.rs stderr

Add `gh_runner.rs` to the files-to-change table. Change `gh()` to capture stderr
and only forward it if the command succeeds (as progress). On failure, include
gh's stderr in the error envelope's `chain` array.

### #22 — backward compatibility

smith-agent has zero consumers today — it was built this session. No migration
needed. Note this in the plan as the reason no compatibility flag exists.

### #8 — gh stderr leaks

Covered by #19 above.

### #12 — summaries need uncomputed data

Each operation already has the data in scope — it just needs to extract it before
building the envelope instead of dumping raw JSON. For board: parse the items
array length and count distinct statuses. For issue view: extract from the
GraphQL result. Not new API calls, just a few lines of JSON navigation per function.

### #14 — keyring_service in auth-status

Keep it in the result. Add to the plan's result spec.

### #15 — issue query --by issue-type --label

Known wart. Document in the capabilities hint for `issue query`:
`"Note: for --by issue-type, the --label flag takes the type name (Epic, Task, Bug)"`

### Command graph — single source of truth for all relationships

The operations and their relationships live in one data structure — a command
graph. Each operation is a node. Edges define what comes `next` and what's
`related`. The envelope is generated by traversing the graph, not by each
function hardcoding hint strings.

**Architecture (using petgraph):**

petgraph `DiGraph` — nodes are operations, edges are relationships (next,
related, recovery). Plugins register new nodes and edges at runtime.

```rust
// command_graph.rs — the single source of truth

use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub struct CommandGraph {
    graph: DiGraph<CommandNode, EdgeKind>,
    index: HashMap<Op, NodeIndex>,
}

pub struct CommandNode {
    pub op: Op,
    /// Human summary template: "Created {kind} #{number} in {team_repo}"
    pub summary_template: &'static str,
}

/// Edge weight — what kind of relationship this edge represents
pub enum EdgeKind {
    /// "What to do next" — directional, contextual
    Next {
        template: String,
        condition: Option<EdgeCondition>,
    },
    /// "What else exists" — discovery, non-directional
    Related {
        template: String,
        condition: Option<EdgeCondition>,
    },
    /// Error recovery — keyed by failure class
    Recovery {
        failure_class: String,
        template: String,
    },
}

pub enum EdgeCondition {
    KindEquals(&'static str),   // e.g., "epic"
    StateEquals(&'static str),  // e.g., "OPEN", "CLOSED"
    HasSubIssues,
    SubIssuesComplete,
    ResultFieldPresent(&'static str),
}

/// All 26 operations
pub enum Op {
    AuthStatus,
    Board,
    IssueCreate, IssueView, IssueQuery, IssueClose,
    IssueReopen, IssueComment, IssueAssign, IssueUpdate,
    StatusSet,
    PrCreate, PrView, PrList, PrMerge, PrApprove,
    PrRequestChanges, PrComment, PrClose,
    PrReviewAddComment, PrReviewSubmit, PrReviewShow, PrReviewClear,
    SubIssueCreate, SubIssueList, SubIssueStatus,
    MilestoneList, MilestoneCreate, MilestoneAssign,
    Fork,
}
```

**How it works at runtime:**

1. Operation function runs, produces `result: serde_json::Value`
2. Calls `graph.build_envelope(Op::IssueCreate, team, &result)`
3. `build_envelope` looks up the node, evaluates each edge's condition against
   the result data, renders matching templates with `team` + result fields,
   returns a complete `Envelope`
4. The operation function only provides the `result` — the graph owns all
   relationship logic

**Template rendering:**

Templates use `{field_name}` placeholders resolved from:
- `{team}` — the `--team` CLI value
- `{number}`, `{title}`, `{kind}`, `{status}`, etc. — extracted from `result`

```rust
impl CommandGraph {
    pub fn build_envelope(&self, op: Op, team: &str, result: &serde_json::Value) -> Envelope {
        let node_idx = self.index[&op];
        let node = &self.graph[node_idx];
        let summary = render_template(node.summary_template, team, result);

        let mut next = Vec::new();
        let mut related = Vec::new();

        // Walk outgoing edges from this node
        for edge in self.graph.edges(node_idx) {
            match edge.weight() {
                EdgeKind::Next { template, condition } => {
                    if condition.as_ref().map_or(true, |c| c.met(result)) {
                        next.push(render_template(template, team, result));
                    }
                }
                EdgeKind::Related { template, condition } => {
                    if condition.as_ref().map_or(true, |c| c.met(result)) {
                        related.push(render_template(template, team, result));
                    }
                }
                EdgeKind::Recovery { .. } => {} // recovery is pulled separately on error
            }
        }

        Envelope::success(summary, result.clone(), next, related)
    }

    pub fn build_error_envelope(
        &self, op: Op, team: &str, failure_class: &str, message: &str,
    ) -> Envelope {
        let node_idx = self.index[&op];
        let recovery: Vec<String> = self.graph.edges(node_idx)
            .filter_map(|e| match e.weight() {
                EdgeKind::Recovery { failure_class: fc, template }
                    if fc == failure_class => Some(render_template(template, team, &serde_json::Value::Null)),
                _ => None,
            })
            .collect();
        Envelope::error_with_recovery(message, recovery)
    }

    /// List all operations in a group (for capabilities documents).
    /// Walks the graph to find all nodes — plugins included.
    pub fn capabilities(&self, group: &str, team: &str) -> Envelope {
        let ops: Vec<_> = self.graph.node_weights()
            .filter(|n| n.op.group() == group)
            .map(|n| format!("{}: {}", n.op.command_path(team), n.summary_template))
            .collect();
        Envelope::success(
            format!("{} operations available", group),
            serde_json::json!({"operations": ops}),
            vec![],
            vec![],
        )
    }

    /// Register a plugin-provided operation at runtime.
    pub fn register(&mut self, node: CommandNode) -> NodeIndex {
        let idx = self.graph.add_node(node);
        self.index.insert(self.graph[idx].op.clone(), idx);
        idx
    }

    /// Add an edge between two registered operations.
    pub fn add_edge(&mut self, from: Op, to: Op, kind: EdgeKind) {
        let from_idx = self.index[&from];
        let to_idx = self.index[&to];
        self.graph.add_edge(from_idx, to_idx, kind);
    }
}
```

**Why petgraph:**

- Plugins register new nodes and edges at runtime via `register()` + `add_edge()`
- `capabilities()` walks all nodes in the graph — plugin-provided ops show up
  automatically in the capabilities document
- Future: DOT export for visualizing the command tree
- Future: reachability queries ("what operations can I reach from here?")
- Standard crate (412M downloads), no maintenance burden

**Compile-time DOT spec:**

The command graph is declared in `commands.dot` — a DOT file parsed at compile
time via `petgraph::graph_from_file!`. The graph is baked into the binary. No
runtime file dependency. Invalid DOT syntax = build failure.

`commands.dot` already exists with all 26 operations, edge types (next, related,
recovery), conditional edges (kind_equals_epic, state_equals_OPEN, etc.), and
failure classes (status_not_found, issue_not_in_project, etc.). 8 tests pass.

**The spec (`crates/smith/src/agent_commands/github/commands.dot`):**

33 nodes, 124 edges. The graph is the full command tree — not just leaf operations.

**Hierarchy** — 7 group nodes (root, github, issue, pr, status, sub-issue,
milestone) + 26 leaf operation nodes. Group → child edges (`kind="child"`)
define the command tree. Running a group node returns an envelope whose `next`
lists its children.

**Workflow** — Leaf → leaf edges define what follows each operation:
- 61 `next` edges (12 conditional: kind_equals_epic, state_equals_OPEN, etc.)
- 21 `related` edges (discovery)
- 11 `recovery` edges (status_not_found, issue_not_in_project, etc.)

Every node has a `summary` template. Every edge has a `template` with a runnable
command using `{org}`, `{number}`, `{project}`, etc. placeholders.

**Uniform envelope**: every node — group or leaf — returns the same envelope shape.
The only differences:
- **Leaf nodes** (exit 0): `result` is present with operation data
- **Leaf nodes** (exit 1): `error` is present with recovery hints
- **Group nodes** (exit 2): no `result`, no `error`. `summary` is the group
  description. `next` MUST be dynamically built by walking all outgoing
  `kind="child"` edges from this node in the graph and rendering each edge's
  `template` attribute. This is not optional — an empty `next` on a group
  node is a bug. The implementation MUST call `graph.edges(node_idx)`,
  filter to `kind="child"`, render templates, and populate `next`.
  Adding a child operation to the DOT file automatically updates the
  group's envelope with no code changes.

The file is the authoritative spec. All templates use `{org}` (not `{team}`).
Operations that need a project board include `--project {project}` in their templates.

8 compile-time tests validate the file.

Dependencies: `petgraph` (with `dot_parser` feature), `dot-parser`, `dot-parser-macros`.

**Structural enforcement:**

Adding a new operation means adding a node to `commands.dot`. If the graph
doesn't compile (malformed DOT), the build fails. The `compile_time_dot_file_import`
test asserts the expected node and edge counts — adding a node without edges
causes a test failure, not just a missing hint.

### #16 — conditional hints

Handled by `EdgeCondition` in the graph. Each edge has an optional condition
evaluated against the result JSON. No per-function `if` blocks for hints.

### #20 — verification

The smoke test is a bash script that runs each operation and validates:
1. stdout parses as JSON
2. `.ok` field exists
3. `.summary` is a non-empty string
4. `.next` is an array (possibly empty)
5. `.result` exists when `ok: true`
6. `.error` exists when `ok: false`

Not validating that commands in `next` are "runnable" — just that they're strings
containing `smith-agent`. Full runnable validation would require executing them,
which is destructive.

### #21 — sub-issue create --issue-type redundancy

Omit `--issue-type Story` from the hint since it's the default. Show it as:
`"Create another: smith-agent github --team {T} sub-issue create --parent {parent} --title <title>"`

## Files to change (updated)

| File | Change |
|---|---|
| `command_graph.rs` | Existing — wire `graph_from_file!("commands.dot")` into runtime `CommandGraph`, add `build_envelope()` / `build_error_envelope()` / `capabilities()` |
| `commands.dot` | Existing — the compile-time spec, all 26 ops with edges (already done, 8 tests pass) |
| `envelope.rs` | Remove `ok` field, remove skip_serializing_if, simplify constructors (graph builds envelopes now) |
| `agent_main.rs` | try_parse, print envelope on stdout, exit 0/1/2 based on outcome |
| `agent_cli.rs` | Disable color, make subcommands Optional for capabilities |
| `mod.rs` | Return Result<Envelope>, use graph.build_envelope(), capabilities from graph traversal |
| `issue.rs` | Return result data only — graph builds the envelope (8 functions) |
| `status.rs` | Return result data, catch-and-map errors to graph recovery keys |
| `board.rs` | Return result data, compute counts for summary template |
| `pr.rs` | Return result data, fix pr create to parse URL→number (8 functions) |
| `sub_issue.rs` | Return result data (3 functions) |
| `milestone.rs` | Return result data, fix create bug (3 functions) |
| `fork.rs` | Return result data |
| `auth.rs` | Return result data, keep keyring_service |
| `gh_runner.rs` | Capture stderr, include in error envelope on failure |

## Verification (updated)

1. `cargo build -p smith` — clean
2. Envelope schema validation script: every operation's stdout has ok/summary/result/next/related
3. Error cases return ok=false with error.recovery array
4. `smith-agent github --team x` (no subcommand) → capabilities JSON on stdout
5. `smith-agent github --team x issue` (no subcommand) → issue capabilities JSON
6. No ANSI colors anywhere (pipe through `cat -v`, grep for escape codes)
7. Exit code 0 on success, 1 on failure, 2 on discovery (group nodes)
8. Full 26-operation smoke test against real GitHub
9. pr review stubs return proper error envelopes, not panic/bare text
