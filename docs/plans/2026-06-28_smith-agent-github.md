# Plan: smith-agent github — Port github-project Skill to smith-agent CLI

## Goal

Build the `smith-agent` binary with a `github` subcommand that provides all
operations from the BotMinter `github-project` skill, with proper GitHub App
credential support (reading from the system keyring via the existing formation
credential store).

`smith-agent` is the agent-facing CLI — verbose, machine-readable output, corrective
error messages. It's what coding agents (Claude Code, etc.) drive. The github
subcommand replaces the bash scripts in the github-project skill with a proper CLI.

## Architecture (from design.md §3.7, §4.9)

The design calls for two binaries over one engine:
- `smith` — human-facing (concise, colored, interactive)
- `smith-agent` — agent-facing (verbose, machine-readable, corrective)

`smith-agent github` is the Rust equivalent of `bash ${CLAUDE_SKILL_DIR}/scripts/*.sh`.

## What smith-agent github should do (v1)

```
smith-agent github board                     # board view
smith-agent github issue create ...          # create issue
smith-agent github issue view <num>          # single issue query
smith-agent github issue query ...           # query issues
smith-agent github issue close <num>         # close issue
smith-agent github issue reopen <num>        # reopen issue
smith-agent github issue assign <num> <user> # assign
smith-agent github issue comment <num> ...   # add comment
smith-agent github status set <num> <status> # transition status
smith-agent github pr create ...             # PR operations
smith-agent github pr view <num>             # view PR
smith-agent github pr list                   # list PRs
smith-agent github pr merge <num>            # merge PR
smith-agent github pr review ...             # inline review
smith-agent github sub-issue create ...      # sub-issue ops
smith-agent github milestone ...             # milestone ops
```

All output is JSON. All errors include corrective instructions.

## Credential flow

1. Read App credentials from keyring: client_id, private_key, installation_id
2. Generate JWT from client_id + private_key (existing `app_auth::generate_jwt`)
3. Exchange JWT for installation token (existing `app_auth::exchange_for_installation_token`)
4. Write token to a temp `hosts.yml` and set `GH_CONFIG_DIR`
5. All `gh` commands run under the App identity

This reuses the existing credential infrastructure completely:
- `formation::local::create_local_formation(team_name)` → formation
- `formation.credential_store(CredentialDomain::GitHubApp { .. })` → store
- `store.retrieve(&credential_keys::*())` → values
- `app_auth::generate_jwt()` + `app_auth::exchange_for_installation_token()` → token

## Step-by-step plan

### Phase 1: Binary scaffold (smith-agent compiles, prints help)

1. Create `src/agent_main.rs` — entry point for smith-agent binary
2. Create `src/agent_cli.rs` — clap CLI with Github subcommand
3. Uncomment `pub mod agent_cli;` in lib.rs
4. Add `[[bin]]` target to Cargo.toml
5. Verify: `cargo build -p smith` produces both `smith` and `smith-agent` binaries

### Phase 2: GitHub auth subcommand (token from keyring)

1. Create `src/agent_commands/mod.rs`
2. Create `src/agent_commands/github/mod.rs` — the auth/setup layer
3. Implement `get_github_token(team_name, member_name)`:
   - Read from keyring via formation
   - Generate JWT
   - Exchange for installation token
   - Write to temp GH_CONFIG_DIR
4. Wire into agent_main.rs
5. Test: `smith-agent github auth-status` shows token validity

### Phase 3: Port board-view.sh

1. Create `src/agent_commands/github/board.rs`
2. Port the `project_items_json` + grouping logic
3. Output JSON grouped by status
4. Test against real GitHub

### Phase 4: Port core operations (issue, status, comment, assign)

1. `src/agent_commands/github/issue.rs` — create, view, query, close/reopen, comment, assign
2. `src/agent_commands/github/status.rs` — status transition with verification
3. `src/agent_commands/github/sub_issue.rs` — sub-issue create/list/status
4. Port the GraphQL queries from the bash scripts
5. Test each operation

### Phase 5: Port PR operations

1. `src/agent_commands/github/pr.rs` — create, view, list, merge, approve, request-changes
2. `src/agent_commands/github/pr_review.rs` — cached inline review system
3. Test each operation

### Phase 6: Port remaining (milestone, fork)

1. `src/agent_commands/github/milestone.rs`
2. `src/agent_commands/github/fork.rs`

### Phase 7: Integration test

1. Test the full flow: auth → board → create issue → transition → comment

## Files to create/change

| File | Action |
|---|---|
| `Cargo.toml` | Add `[[bin]]` for smith-agent |
| `src/lib.rs` | Uncomment `agent_cli`, add `agent_commands` |
| `src/agent_main.rs` | New — smith-agent entry point |
| `src/agent_cli.rs` | New — clap CLI definition |
| `src/agent_commands/mod.rs` | New — module root |
| `src/agent_commands/github/mod.rs` | New — auth + dispatch |
| `src/agent_commands/github/board.rs` | New — board view |
| `src/agent_commands/github/issue.rs` | New — issue operations |
| `src/agent_commands/github/status.rs` | New — status transitions |
| `src/agent_commands/github/pr.rs` | New — PR operations |
| `src/agent_commands/github/pr_review.rs` | New — cached review |
| `src/agent_commands/github/sub_issue.rs` | New — sub-issue ops |
| `src/agent_commands/github/milestone.rs` | New — milestone ops |
| `src/agent_commands/github/fork.rs` | New — fork repo |

## Key decisions

1. **Use `gh` CLI underneath** — same as the bash scripts. We're wrapping `gh`
   with proper auth, caching, and machine-readable output. Not reimplementing
   the GitHub API in reqwest (that's a future optimization).

2. **JSON output always** — smith-agent is for agents. Human-readable board view
   is smith's job (later).

3. **Corrective errors** — every error message includes what to do about it,
   per design.md §3.7.

4. **Cache layer** — port the setup.sh caching (project ID, status field ID,
   repo metadata) into Rust.

## Implementation approach

Since these are bash scripts being ported to Rust, and they all shell out to
`gh`, the Rust code is essentially:
1. Parse CLI args
2. Authenticate (JWT → installation token → GH_CONFIG_DIR)
3. Build `gh` command with args
4. Run it, capture output
5. Post-process/verify
6. Output JSON

This is a thin Rust wrapper around `gh` — the logic is in arg construction,
caching, and error handling, not HTTP calls.
