# Plan: smith chat — Launch Configured LLM Agent

## Goal

Add `smith chat` command that launches a configured coding agent (Claude Code by
default) in the current directory with signal forwarding and TTY inheritance. Unlike
BotMinter's `bm chat <member>`, smith has no teams/members/profiles/daemon — it's
just "you" (the operator) talking to the configured agent.

## Current context

- `smith init` wizard works end-to-end (auth → org → repo → app creation)
- BotMinter's chat infrastructure is already in the crate (commented out in lib.rs):
  - `chat/mod.rs` (1226 lines) — session prep, meta-prompt, launch
  - `chat/spawn.rs` (315 lines) — spawn agent, signal forwarding, TTY inheritance
- `claude` binary is available at `/home/sandbox-test/.local/bin/claude`
- BotMinter's `CodingAgentDef` has: binary, system_prompt_flag, skip_permissions_flag
- BotMinter's chat requires daemon, profiles, team repo, member workspace — all of
  which we don't have or need yet

## What smith chat should do (v1)

```
smith chat                  # launch claude in CWD
smith chat -a               # launch claude with --dangerously-skip-permissions
smith chat --agent codex    # use a different agent binary
```

- Detect the agent binary (default: `claude`)
- Spawn it in the current directory
- Inherit stdin/stdout/stderr (TTY passthrough)
- Forward SIGINT/SIGTERM to the child
- Exit with the agent's exit code

No meta-prompt, no workspace hydration, no session tracking, no daemon. Just spawn
the agent with the right flags. Those layers come later.

## What to port from BotMinter

**Copy verbatim (already in the crate):**
- `chat/spawn.rs` — SpawnConfig, spawn_and_wait, signal forwarding. This is the
  module we need. It's already in `src/chat/spawn.rs`. 100% reusable as-is.

**Not needed (stay commented):**
- `chat/mod.rs` — prepare_chat_session, build_meta_prompt, inject_app_credentials,
  resolve_member_by_role. All of this is team/member/profile machinery we don't need.
- `commands/chat.rs` — BotMinter's command handler that goes through daemon + session
  API. We'll write a much simpler one.

## Default agent config

A `~/.config/loopsmith/config.yml` with:

```yaml
agent:
  binary: claude
  system_prompt_flag: --append-system-prompt-file
  skip_permissions_flag: --dangerously-skip-permissions
```

If no config exists, use hardcoded defaults (claude). This reuses the existing
`config/mod.rs` module which already has config loading infrastructure.

## Step-by-step plan

### Step 1: Enable chat/spawn.rs only

In `lib.rs`, add a `pub mod chat;` that exposes only the `spawn` submodule:
- Create `src/chat/mod.rs` with just `pub mod spawn;` (the existing BotMinter
  chat/mod.rs references deleted modules — comment it out, expose only spawn)

Compile, verify spawn unit tests pass.

### Step 2: Add Chat CLI variant

In `cli.rs`, uncomment or add the Chat command inside the Command enum (before the
`/* Phase 1` comment block). Keep it simple:

```rust
/// Launch a coding agent in the current directory
Chat {
    /// Run in autonomous mode (skip permission prompts)
    #[arg(short = 'a', long)]
    autonomous: bool,

    /// Agent backend binary to use (default: from config or "claude")
    #[arg(short = 'b', long = "agent-backend")]
    agent_backend: Option<String>,
}
```

### Step 3: Add agent config to config/mod.rs

Add to the existing config structures:

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub binary: String,
    pub system_prompt_flag: Option<String>,
    pub skip_permissions_flag: Option<String>,
}
```

Add a `pub fn default_agent_config() -> AgentConfig` that returns claude defaults.
Add `agent: Option<AgentConfig>` to the main config struct.

### Step 4: Write commands/chat.rs

Simple handler:
1. Load config (or use defaults)
2. Resolve agent binary (CLI flag > config > "claude")
3. Build SpawnConfig with current dir
4. If autonomous, add skip_permissions_flag
5. Call spawn_and_wait
6. Exit with agent's exit code

~50 lines. No daemon, no sessions, no workspace hydration.

### Step 5: Wire main.rs

Add the Chat match arm in main.rs.

### Step 6: Smoke test

Run `smith chat` and verify claude launches in the current directory.
Run `smith chat -a` and verify autonomous mode.
Ctrl+C and verify signal forwarding.

### Step 7: rexpect integration test

Spawn `smith chat --agent /usr/bin/echo` (use echo as a dummy agent),
verify it runs and exits 0.

## Files to change

| File | Action |
|---|---|
| `src/lib.rs` | Uncomment/add `pub mod chat;` |
| `src/chat/mod.rs` | New — just `pub mod spawn;` (BotMinter's commented out) |
| `src/chat/spawn.rs` | Already exists — no changes needed |
| `src/cli.rs` | Add Chat variant to Command enum |
| `src/config/mod.rs` | Add AgentConfig struct + defaults |
| `src/commands/mod.rs` | Add `pub mod chat;` |
| `src/commands/chat.rs` | New — ~50 line handler |
| `src/main.rs` | Add Chat match arm |
| `tests/integration.rs` | Add smoke test |

## Tests / validation

1. `cargo build -p smith` — compiles clean
2. `cargo test -p smith` — spawn.rs unit tests pass
3. `smith --help` — shows `chat` command
4. `smith chat --agent /usr/bin/echo` — exits 0
5. `smith chat` — launches claude in CWD (manual)
6. Ctrl+C during `smith chat` — signal forwarded, clean exit

## Risks and tradeoffs

- **Hardcoded claude default**: Fine for now — config override exists for other agents.
  When we support Codex/OpenCode, just change the config.
- **No system prompt**: v1 has no meta-prompt. Agent launches bare. System prompt
  injection comes with workspace/project context later.
- **No config file creation**: `smith init` currently doesn't write a config file.
  `smith chat` should work without one (pure defaults). Config file is optional
  override.
- **spawn.rs depends on libc**: Already a dependency. Linux-only signal forwarding.
  macOS support is a future concern (already has a commented-out formation/local/macos).

## Open questions

- Should `smith chat` require `smith init` to have been run first? Probably not for
  v1 — just launch the agent. Init is for GitHub App setup, chat is independent.
- Should there be a `smith chat --prompt "do X"` for non-interactive one-shot? Claude
  Code supports this. Easy to add later.
