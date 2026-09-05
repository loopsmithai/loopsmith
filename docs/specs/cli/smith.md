# smith Operator CLI Specification

## Purpose

`smith` is the human-facing operator binary. It registers a GitHub App for an org and stores its
credentials, so that `smith-agent` can later mint installation tokens. This spec defines the operator
command surface.

## Requirements

### Requirement: `smith init`

`smith init` SHALL run an interactive wizard that authenticates with a GitHub PAT, selects an org,
selects a repo, runs the GitHub App manifest flow, and stores the resulting App credentials in the
credential store (see [credential-store](../auth/credential-store.md)).

`smith init` SHALL also support a non-interactive path (`--non-interactive`) requiring the inputs:
profile, the team/workspace name flag, org, repo, and the project-board title. It SHALL additionally
accept optional inputs: project fork URL, bridge, workzone override, and a credentials file (for
machine migration), plus a hidden test-only flag to skip GitHub API calls.

#### Scenario: Interactive registration
- GIVEN a valid GitHub PAT with org access
- WHEN the operator runs `smith init` and completes the wizard
- THEN a GitHub App is registered and its credentials are stored in the keyring for that org

#### Scenario: Non-interactive registration
- GIVEN all required inputs are supplied as flags
- WHEN `smith init --non-interactive ...` runs
- THEN the wizard prompts are skipped and registration proceeds from the flag values

### Requirement: `smith install`

`smith install` SHALL install an already-registered GitHub App onto a new org. The operator
picks the org in the GitHub browser UI; the CLI discovers which org was chosen by diffing
installations before and after.

| Flag | Type | Default | Required |
|------|------|---------|----------|
| `--id` | string | `smith` | No |

#### Scenario: Install existing App on a new org
- GIVEN an App already registered under identity `smith`
- WHEN `smith install` runs and the operator installs on `neworg` in the browser
- THEN the CLI detects the new installation on `neworg` and stores the installation_id
- AND `smith-agent github --org neworg auth-status` succeeds

## Limitations & Known Issues

### Boundaries
- Only `init` and `install` are part of the current operator surface. No other operator subcommands
  are compiled.

### Known gaps & accepted constraints
- The non-interactive team/workspace-name input is still the literal flag `--team-name`, carrying
  legacy "team" vocabulary. This is the current flag name; an org-aligned rename is pending and the
  "team" concept is not desired domain vocabulary.
- A large body of copied-but-uncompiled modules (the original BotMinter source) remains on disk,
  commented out in `lib.rs`. It is dead code, not part of any contract, and is pending deletion.
