# R-04 — Agent event streams, ACP, and two reference projects

Research for #178 (2026-06-16). Question that triggered it: *do Claude Code / Codex / Gemini CLI
support, out of the box today, a structured/attachable agent event stream (turns, tool calls,
file diffs, tokens, permissions) — and what do two named projects
([plum-code-webui](https://github.com/zwaetschge/plum-code-webui),
[agentic-ci](https://opendatahub-io.github.io/agentic-ci/)) actually do?*

Sections 1–4 are **objective findings** (no Loopsmith framing). Section 5 is clearly separated
**interpretation**. Sources at the end.

---

## 1. ACP (Agent Client Protocol) — ecosystem facts

- ACP is an open standard created by **Zed Industries**, launched **2025-08-27**, **Apache-licensed**.
  It standardizes communication between code editors/clients and AI coding agents.
- Transport/encoding: **JSON-RPC 2.0 over stdio** (subprocess).
- It carries structured content: prompts/messages and turns, tool calls and tool progress, file
  diffs/edits, permission requests, streaming deltas, token/usage info.
- **Gemini CLI** was the first integration / the reference ACP implementation (Aug 2025).
- **Claude Code** follows ACP (as a subprocess) since **2025-09-03**.
- **2026-02**: **JetBrains** joined as co-lead maintainer alongside Zed; the two shipped an **ACP
  Agent Registry** (a directory of agents integrated into Zed and JetBrains IDEs).
- Attaching external clients to a *running* session is an active design area with **two parallel
  Claude Code proposals, neither shipped**:
  - **#24365** (`claude serve`): expose **ACP over network transport** (TCP / WebSocket) for
    remote/mobile session attach. *(feature request)*
  - **#65606**: extend **MCP** with bidirectional **session channels** — `session/subscribe`
    (stream messages, tool calls, approval requests), `session/inject` (push prompts),
    `session/approve` (answer approvals); proposed CLI `claude session list|watch|inject|approve`.
    Transport is a **local Unix domain socket** (`~/.claude/sessions/<id>.sock`, MCP JSON-RPC 2.0);
    remote access is the user's SSH port-forward — *"Claude Code does not expose a network port."*
    Supports **multiple subscribers** to one session. *(RFC/proposal; shown "Closed" in UI, body
    labeled "Status: Proposal".)* It explicitly notes it overlaps #24365 on remote access but is
    "local-first and MCP-native."
  - Net: the structured stream + (eventual) local multi-attach is moving first-party; **network
    fan-out to arbitrary surfaces is not provided by either** and remains application-side.
- Adjacent tooling observed: **`openclaw/acpx`** — a headless, scriptable CLI client for **stateful
  ACP sessions** ("curl for agent sessions"); an **AI-SDK ACP community provider** (bridges ACP
  agents to the Vercel AI SDK `LanguageModel` interface); **VS Code** and **Obsidian** ACP client
  extensions; Zed's external-agents feature running Claude/Gemini/Codex side by side.

## 2. Per-CLI out-of-the-box status (today)

| CLI | Structured stream OOTB | Mechanism |
|---|---|---|
| **Gemini CLI** | yes (native) | reference ACP implementation |
| **Claude Code** | yes (native) | ACP subprocess since 2025-09-03; `claude serve` network-attach is an open request (#24365), not shipped |
| **Codex** | yes (native) | `app-server` (JSON-RPC notifications: `item/started`, `item/completed`, `item/agentMessage/delta`, tool progress, `command/exec/outputDelta` with base64 stdout/stderr, account token usage) + `codex exec --json`/JSONL non-interactive with session resume |

## 3. Project: plum-code-webui (objective facts)

Source: <https://github.com/zwaetschge/plum-code-webui>

- **What it is:** a self-hosted **browser interface** for multiple coding-agent CLIs, presenting a
  unified workspace across providers.
- **Providers supported:** Codex (OpenAI, the default), OpenCode (server-backed HTTP/SSE routing
  for 75+ LLMs), Mistral Vibe (Devstral coding models), Claude Code (Anthropic, described as a
  legacy provider option).
- **How it connects to agents:** the backend **spawns each provider as a child process** and
  **bridges its stream over Socket.IO WebSocket events**. Each CLI is run with provider-specific
  arguments (e.g. `codex exec --json`, `opencode run --format json`).
- **Data types it renders:** streaming chat responses with real-time text deltas; tool-execution
  timelines with duration tracking and expandable input/output; token-usage breakdowns
  (input / output / cache-read / cache-write) via context popovers; file diffs and git operations;
  LaTeX/math rendering; interactive permission approvals and choice prompts; compaction-boundary
  cards showing context-compression events.
- **Architecture:** Backend = Express.js + Socket.IO + SQLite, managing process lifecycles via
  **node-pty**. Frontend = React 18 + Vite SPA with Zustand state management. Deployment = a Docker
  container bundling all CLIs, system dependencies, and MCP servers.
- **Session model:** **does not attach to pre-existing agent sessions** — it **spawns a fresh CLI
  instance per turn**.

## 4. Project: agentic-ci (objective facts)

Source: <https://opendatahub-io.github.io/agentic-ci/> (OpenDataHub / Red Hat)

- **What it is:** a framework for executing AI coding agents in **isolated sandbox environments
  within CI pipelines**, with built-in observability.
- **Agent harnesses:** abstracts multiple agent implementations — **currently Claude Code and
  OpenCode** — through a **harness interface** that decouples agent logic from execution
  infrastructure.
- **Isolation backends (two):** **Podman containers** for standard containerized isolation;
  **OpenShell sandboxes** with **network-policy enforcement** for stricter containment.
- **Key components:** (1) **Backend layer** — manages sandbox lifecycle (setup/execution/cleanup),
  abstracts container/sandbox operations; (2) **Harness layer** — bridges agent CLIs with the
  backend, handling protocol translation and session management; (3) **Pipeline framework** —
  "gates" (pre/post-agent validation), skill runners, and verdict handling for composable
  workflows; (4) **Observability** — OpenTelemetry integration tracking token usage, costs, and
  execution metrics; streaming output provides real-time colored logs with tool-call summaries.
- **Workflow:** user provides a prompt and model selection → backend provisions an ephemeral
  sandbox → harness instantiates an agent session → output streams back with parsed logging and
  telemetry. **Sessions are ephemeral** — containers/sandboxes terminate after execution completes.
- **Transport:** the agent communication protocol is **not explicit** in the documentation; the
  abstraction implies agent CLIs run within sandboxes with **captured I/O**.

---

## 5. Interpretation for Loopsmith (separated from the facts above)

- The canonical agent-event stream we need **is ACP** (or Codex's app-server JSON-RPC, same shape).
  Don't invent a stream format; BotMinter's `acp/` is already aligned. → `harness` extensions
  (claude/codex/gemini) are ACP clients, and are genuinely interchangeable because ACP standardizes
  them.
- The **rich stream is OOTB; multi-surface network attach to a running ephemeral session is not**
  (the `claude serve` #24365 and MCP-session #65606 proposals are both unshipped, and #65606 is
  local-socket-only). That gap is where a **session relay / fan-out** sits — net-new, application-
  side infra. If either proposal ships, our relay sits **on top of** the local socket rather than
  being obsoleted.
- **Authority dial validated by #65606's verbs:** `subscribe` / `inject` / `approve` map directly
  onto the kernel port authority dial (observe / send-input / approve-permission). Our relay should
  model authority this way and add the network fan-out both proposals leave out.
- **Disclosure dial validated:** ACP carries diffs/tools/permission-requests that a text channel
  (telegram/whatsapp) cannot render → such a channel must be a **projection**, not an equal surface
  (full-native vs projected = the kernel port disclosure dial).
- **Possible reuse/inspiration (unevaluated):** plum-code-webui's renderer set (tool timelines,
  token popovers, diff views, permission prompts, compaction cards) and its Socket.IO bridging;
  agentic-ci's harness interface, Podman/OpenShell isolation backends, gates/skill-runner pipeline,
  and OTel observability. Both are candidates to reuse / fork / be inspired by — to be assessed,
  not assumed.

## Sources

- Zed ACP: <https://zed.dev/acp> · ACP agents: <https://agentclientprotocol.com/get-started/agents>
- Claude Code ACP (issue #6686): <https://github.com/anthropics/claude-code/issues/6686>
- Claude Code `claude serve` network-transport request (#24365): <https://github.com/anthropics/claude-code/issues/24365>
- Claude Code MCP session-channels RFC (#65606): <https://github.com/anthropics/claude-code/issues/65606>
- Codex App Server: <https://developers.openai.com/codex/app-server>
- openclaw/acpx: <https://github.com/openclaw/acpx>
- plum-code-webui: <https://github.com/zwaetschge/plum-code-webui>
- agentic-ci: <https://opendatahub-io.github.io/agentic-ci/>
