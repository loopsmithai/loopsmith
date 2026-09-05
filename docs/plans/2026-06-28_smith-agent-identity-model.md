# Plan: smith-agent Identity Model — Replace team/member with id/org/repo/project

## Goal

Remove all BotMinter vocabulary (team, member) from smith-agent and smith init.
Replace with a clean identity model:

- **Agent ID** (`--id`) — smith's identity, used as the keyring key prefix
- **Org** (`--org`) — the GitHub org where the App is installed
- **Repo** (`--repo`) — the specific repo to operate on
- **Project** (`--project`) — the GitHub Projects V2 board number

## Why

BotMinter had teams (workspaces within an org with multiple members). Loopsmith
has none of that. Smith is the tool, the org is the org, the repo is the repo.
The current `--team` flag maps to org but carries BotMinter semantics. `--member`
is a BotMinter concept (multiple actors per team) that doesn't apply.

## The identity model

| Concept | CLI flag | Default | Required | Source |
|---------|----------|---------|----------|--------|
| Agent ID | `--id` | `smith` | No | CLI flag |
| Org | `--org` | — | Yes | CLI flag |
| Repo | `--repo` | From git remote | No | CLI flag or cwd git remote |
| Project | `--project` | — | Required for board ops | CLI flag or `.smith` config. **Never inferred.** |

### Credential lookup

Keyring service: `loopsmith.{org}.github-app`
Key prefix: `{id}/`

Examples:
- `loopsmith.devguyio-bot-squad.github-app` → `smith/github-app-id`
- `loopsmith.devguyio-bot-squad.github-app` → `smith/github-app-client-id`
- `loopsmith.devguyio-bot-squad.github-app` → `smith/github-app-private-key`
- `loopsmith.devguyio-bot-squad.github-app` → `smith/github-installation-id`

This allows:
- **Multiple agents, same org**: different `--id`, different Apps
- **Same agent, multiple orgs**: different `--org`, different keyring service
- **`--id` + `--org`** together = unique credential lookup

Legacy fallback: `botminter.{org}.github-app` → `smith/*` keys (existing creds
from the init wizard run under the old naming).

### Repo inference

When `--repo` is not provided, extract from git remote in cwd:

```
git remote get-url origin → https://github.com/devguyio-bot-squad/father-smith.git
                          → org=devguyio-bot-squad, repo=father-smith
```

If no git remote or the remote org doesn't match `--org`, error with a hint:
`"Could not infer repo from git remote. Specify --repo explicitly."`

### Project — never inferred

The current `setup.rs:67` calls `gh project list --owner {org}` and takes
`projects[0]["number"]` — this must be removed. Project is either:

1. Specified via `--project <number>` on the CLI
2. Read from `.smith` config file (future)

Operations that require a project board:
- `board`
- `status set`

Operations that work without a project:
- `issue create/view/close/reopen/comment/assign/update/query`
- `pr create/view/list/merge/approve/request-changes/comment/close`
- `sub-issue create/list/status`
- `milestone list/create/assign`
- `fork`
- `auth-status`

If a project-requiring operation is called without `--project` (and no config),
emit an error envelope:
```json
{
  "ok": false,
  "summary": "Project number required for board operations",
  "error": {
    "message": "--project is required for this operation",
    "recovery": [
      "List projects: gh project list --owner {org}",
      "Then: smith-agent github --org {org} --project <number> board"
    ]
  }
}
```

## CLI shape (before and after)

**Before (BotMinter vocabulary):**
```
smith-agent github --team devguyio-bot-squad --member smith board
smith-agent github --team devguyio-bot-squad issue view 42
```

**After:**
```
smith-agent github --org devguyio-bot-squad --project 1 board
smith-agent github --org devguyio-bot-squad issue view 42
smith-agent github --org devguyio-bot-squad --repo father-smith pr list
```

## Changes required

### Code changes

| File | Change |
|------|--------|
| `agent_cli.rs` | Replace `--team`/`--member` with `--id`/`--org`/`--repo`/`--project`. `--id` defaults to `"smith"`, `--repo` is optional, `--project` is optional. |
| `agent_main.rs` | Pass new fields to `run()` |
| `mod.rs` | `run(team, member, command)` → `run(id, org, repo, project, command)`. Route project-requiring ops through project check. |
| `auth.rs` | `setup_github_auth(team_name, member_name)` → `setup_github_auth(org, id)`. Keyring service uses `{org}` not `{team}`. |
| `setup.rs` | Remove `get_team_repo()` (was `gh repo list --owner {team}`). Replace `ProjectSetup` fields: `team_repo` → `repo` (full `org/repo`), `owner` → `org`. Remove project inference (line 67). Accept project number from CLI arg. |
| `gh_runner.rs` | No changes |
| `issue.rs` | Replace all `setup.team_repo` → `setup.repo` (8 functions) |
| `pr.rs` | Replace all `setup.team_repo` → `setup.repo` (8 functions) |
| `status.rs` | Replace `setup.team_repo` |
| `board.rs` | Replace `setup.owner` → `setup.org`, `setup.project_num` from arg |
| `sub_issue.rs` | Replace `setup.team_repo` |
| `milestone.rs` | Replace `setup.team_repo` |
| `fork.rs` | No changes (uses `--source` and `--org` directly) |
| `commands.dot` | All `{team}` → `{org}` in templates. Add `--project` where needed. |
| `command_graph.rs` | Tests: update SAMPLE_GRAPH templates from `{team}` to `{org}` |
| `envelope.rs` | No structural changes, just field names in capabilities |
| `formation/local/linux/mod.rs` | `CredentialDomain::GitHubApp` — `team_name` field → `org`. Legacy fallback stays. |
| `commands/init.rs` | Store creds under `{org}` not `{team_name}`. Key prefix stays `smith/` (the default agent id). |

### smith init changes

`smith init` currently stores credentials with:
- Service: `loopsmith.{team_name}.github-app` (where team_name = org name)
- Keys: `smith/github-app-id`, etc.

After:
- Service: `loopsmith.{org}.github-app`
- Keys: `smith/github-app-id`, etc. (same — `smith` is the default agent id)

The init wizard's prompts change:
- "Select organization" stays (this is the org)
- Remove any "team" language from prompts and output
- The wizard doesn't ask for agent id — it uses `smith` (the default)

### ProjectSetup struct (before and after)

**Before:**
```rust
pub struct ProjectSetup {
    pub team_repo: String,       // owner/repo
    pub owner: String,           // org or user
    pub project_num: String,     // project number
    pub project_id: String,      // project node ID
    pub status_field_id: String, // Status field node ID
}
```

**After:**
```rust
pub struct ProjectSetup {
    pub org: String,             // GitHub org
    pub repo: String,            // org/repo (full path)
    pub project_num: String,     // project number (from CLI, never inferred)
    pub project_id: String,      // project node ID (fetched from project_num)
    pub status_field_id: String, // Status field node ID
}
```

### Grep targets for cleanup

All occurrences of these in active code must be renamed or removed:
- `team_name` → `org`
- `team_repo` → `repo`
- `member_name` → `id` (or hardcoded where internal)
- `--team` → `--org`
- `--member` → `--id`
- `"team"` in JSON output → `"org"`

## Verification

1. `cargo build -p smith` — clean
2. `smith-agent github --org devguyio-bot-squad --project 1 auth-status` — works
3. `smith-agent github --org devguyio-bot-squad --project 1 board` — works
4. `smith-agent github --org devguyio-bot-squad issue view 1` — works (no --project needed)
5. `smith-agent github --org devguyio-bot-squad board` (no --project) — error envelope
6. Legacy keyring lookup still works (botminter.* fallback)
7. `smith-agent github --org x` (no subcommand) → capabilities document uses --org
8. 8 command_graph tests pass with updated templates
9. All `{team}` gone from active code and DOT spec

## Risks

- **Keyring migration**: existing creds are under `botminter.devguyio-bot-squad.github-app`.
  The fallback handles this. No migration script needed — new runs will use
  `loopsmith.{org}`, old creds still found via fallback.
- **`--repo` inference from git remote**: could be surprising if cwd is a different
  repo than intended. The `--repo` override exists for this.
