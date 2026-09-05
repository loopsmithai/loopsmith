# Agent Envelope Specification

## Purpose

`smith-agent` is an API surface for coding agents, not a human CLI. Every invocation emits exactly
one JSON envelope on stdout describing what happened, the data, what to do next, what else exists,
and — on failure — how to recover. This spec defines the envelope structure, the exit-code semantics,
and the discovery-hint contract that together make `smith-agent` machine-consumable.

## Requirements

### Requirement: Single JSON envelope on stdout

`smith-agent` SHALL print exactly one pretty-printed JSON object to stdout per invocation, with these
fields:

| Field | Type | Presence |
|-------|------|----------|
| `summary` | string | Always — one sentence describing the outcome |
| `result` | object \| null | Exit 0 only (may be JSON `null` when the operation produced no data); omitted on exit 1/2 |
| `next` | array of string | Always (may be empty) |
| `related` | array of string | Always (may be empty) |
| `error` | object `{message, chain, recovery}` | Exit 1 only; `chain` omitted when empty |

There SHALL be no `ok` field — the exit code is the success signal. Color SHALL be disabled
(`NO_COLOR` forced). stderr SHALL carry only human-readable progress and MAY be ignored by agents.

#### Scenario: Successful operation
- GIVEN a valid `smith-agent github` operation that produces data
- WHEN the binary runs
- THEN stdout is one JSON object with `summary`, a non-null `result`, `next`, and `related`
- AND there is no `error` field
- AND the exit code is 0

#### Scenario: Failed operation
- GIVEN an operation that fails (e.g. missing credentials)
- WHEN the binary runs
- THEN stdout is one JSON object with `summary` and `error.message` and `error.recovery`
- AND `next` and `related` are still present (possibly empty arrays)
- AND there is no `result` field
- AND the exit code is 1

### Requirement: Exit-code semantics

`smith-agent` SHALL use exactly three exit codes:

| Code | Meaning | Envelope shape |
|------|---------|----------------|
| 0 | Success — operation executed | `result` present |
| 1 | Failure — operation attempted, failed | `error` present with `recovery` |
| 2 | Discovery — no operation executed | `next` lists available children; no `result` |

#### Scenario: Group node returns discovery
- GIVEN a group invocation with no leaf operation (e.g. `smith-agent github --org X` with no subcommand)
- WHEN the binary runs
- THEN the exit code is 2 (never 0)
- AND the envelope has no `result` field
- AND `next` lists the available child operations

#### Scenario: Unparseable command is discovery, not failure
- GIVEN an incomplete or invalid command (a clap parse error)
- WHEN the binary runs
- THEN it emits a discovery envelope and exits 2 (not 1)
- AND the clap error text is written to stderr

### Requirement: Discovery hints are runnable

Each `next`, `related`, and `error.recovery` entry SHALL be a runnable command string with the real
`--org`, `--project`, and identifier values baked in. Placeholders (`<...>`) SHALL appear only for
values the agent must choose (e.g. `<username>`). A group/discovery envelope's `next` SHALL list that
node's child operations.

#### Scenario: next carries the real created number
- GIVEN `smith-agent github --org acme issue create ...` succeeds and creates issue #42
- WHEN the envelope is produced
- THEN `next` contains a runnable string such as `smith-agent github --org acme issue view 42`
- AND `42` and `acme` are the real values, not placeholders

## Invariants

- A discovery/group invocation exits 2, never 0, and carries no `result`.
- `next` and `related` are always serialized, even when empty (an absent array is a bug).
- The envelope never contains secret material (see `../auth/github-app.md`).

## Limitations & Known Issues

### Known gaps & accepted constraints
- The `Envelope` struct's own doc-comment claims `next`/`related` are exit-0-only; the code emits them
  always. The code behavior (always) is the contract; the comment is stale.
