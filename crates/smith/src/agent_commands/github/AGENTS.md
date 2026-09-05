# AGENTS.md — `agent_commands/github`

The `github` command subsystem behind `smith-agent`. It executes GitHub operations and wraps every
result in the standard envelope. Up: [repo root](../../../../../AGENTS.md).

## Contract

This subsystem's observable contract is specified in `docs/specs/`:
- Envelope + exit codes + discovery hints — [api/agent-envelope.md](../../../../../docs/specs/api/agent-envelope.md)
- CLI surface + identity model — [cli/smith-agent.md](../../../../../docs/specs/cli/smith-agent.md)

Change behavior here ⇒ update those specs (or run the drift detector).

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | Dispatch — routes a parsed command to its operation, returns a result |
| `envelope.rs` | The `Envelope` struct, exit-code constants, success/error/discovery constructors |
| `command_graph.rs` + `commands.dot` | Compile-time command graph: renders `next`/`related`/`recovery` hints. The DOT file is the source of truth; `command_graph.rs` loads and traverses it |
| `setup.rs` | Resolves repo (git-remote inference) and project context; **project is never inferred** |
| `gh_runner.rs` | Runs the `gh` CLI; captures stderr, surfaces it in the error envelope on failure |
| `auth.rs` | `auth-status` operation |
| `issue.rs` / `pr.rs` / `status.rs` / `board.rs` / `sub_issue.rs` / `milestone.rs` / `fork.rs` | Operation implementations |

## Conventions

- Operations return result data; the envelope (summary, next, related) is built by traversing the
  command graph — do not hardcode hint strings in operation functions.
- Hints must be runnable commands with real `--org`/`--project`/number values baked in.
- `pr review {add-comment,submit,show,clear}` is **not implemented** — it returns a "not yet
  implemented" error envelope (deferred; see the spec's Known Issues).
