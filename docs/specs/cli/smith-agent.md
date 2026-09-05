# smith-agent CLI Specification

## Purpose

`smith-agent` is the machine-facing binary that coding agents use to drive GitHub work — issues,
pull requests, the project board, status transitions, sub-issues, milestones, and forks. This spec
defines its identity model (how it resolves which org/repo/project/credentials to act on) and its
command surface. Its output contract is defined separately in [agent-envelope](../api/agent-envelope.md).

## Requirements

### Requirement: Identity model and global flags

The `github` command SHALL accept these global flags:

| Flag | Type | Default | Required | Source |
|------|------|---------|----------|--------|
| `--id` | string | `smith` | No | Agent identity; keyring key prefix |
| `--org` | string | — | Yes | GitHub org |
| `--repo` | string | inferred | No | From cwd git `origin` remote when omitted |
| `--project` | u64 | — | Required for board/status ops | CLI only — **never inferred** |

#### Scenario: Repo inferred from git remote
- GIVEN the cwd is a clone whose `origin` remote is `https://github.com/acme/father-smith.git`
- WHEN `smith-agent github --org acme issue view 1` runs without `--repo`
- THEN the repo resolves to `acme/father-smith`

#### Scenario: Repo cannot be inferred
- GIVEN the cwd has no usable git remote
- WHEN a repo-scoped operation runs without `--repo`
- THEN it fails (exit 1) with recovery text "Could not infer repo from git remote. Specify --repo explicitly."

#### Scenario: Project is required, never inferred
- GIVEN no `--project` is supplied
- WHEN a board or `status set` operation runs
- THEN it fails with recovery hinting to pass `--project <number>`
- AND the tool SHALL NOT auto-select a project (e.g. the org's first project)

### Requirement: github command surface

`smith-agent github` SHALL expose these operations:

| Group | Operations |
|-------|-----------:|
| (top) | `auth-status`, `mint-token`, `list-projects`, `board`, `fork` |
| `issue` | `create`, `view`, `query`, `close`, `reopen`, `comment`, `assign`, `update` |
| `status` | `set` |
| `pr` | `create`, `view`, `list`, `merge`, `approve`, `request-changes`, `comment`, `close` |
| `sub-issue` | `create`, `list`, `status` |
| `milestone` | `list`, `create`, `assign` |

Notable flag defaults SHALL be: `issue create --initial-status` = `Backlog`;
`pr create --base` = `main`, `--draft` = false; `pr merge --method` = `squash`;
`sub-issue create --issue-type` = `Story`. `issue query --by` SHALL accept one of
`label | status | milestone | assignee | project-status | issue-type`.

#### Scenario: Board requires a project
- GIVEN `--org acme --project 1`
- WHEN `smith-agent github --org acme --project 1 board` runs
- THEN the board is returned grouped by status in the envelope `result`

### Requirement: mint-token writes a persistent GH_CONFIG_DIR

`smith-agent github --org <org> mint-token` SHALL mint (or use a cached) installation token
and write a `hosts.yml` file to `~/.config/smith-agent/gh-config/{org}/`. The envelope `result`
SHALL contain `gh_config_dir` (the absolute path) and `hint` (the export command). The caller
uses `export GH_CONFIG_DIR=<path>` + `gh auth setup-git` to enable `git` operations.

#### Scenario: Mint token for git access
- GIVEN valid App credentials for `--org acme`
- WHEN `smith-agent github --org acme mint-token` runs
- THEN exit code is 0
- AND `result.gh_config_dir` is an absolute path to a directory containing `hosts.yml`
- AND `hosts.yml` contains a valid installation token under `github.com`
- AND `export GH_CONFIG_DIR=<result.gh_config_dir> && gh auth setup-git && git ls-remote` succeeds

#### Scenario: Mint token with no credentials
- GIVEN no App credentials in the keyring
- WHEN `smith-agent github --org acme mint-token` runs
- THEN exit code is 1
- AND `error.recovery` hints at `smith init`

## Limitations & Known Issues

### Boundaries
- `smith-agent` operates on a single org/repo per invocation; multi-repo fan-out is out of scope.

### Known gaps & accepted constraints
- `pr review {add-comment, submit, show, clear}` (the cached inline-review system) is **not
  implemented** — it is exposed in the CLI but each subcommand returns a "not yet implemented" error
  envelope. It is deferred, not a current contract.
