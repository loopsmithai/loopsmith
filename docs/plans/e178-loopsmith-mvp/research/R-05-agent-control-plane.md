# R-05 — openshift-online/agent-control-plane ("Ambient")

Research for #178 (2026-06-16). Cloned to `projects/agent-control-plane` (shallow) and read from
source. A Red Hat / OpenShift-Online **Kubernetes-native AI automation platform that orchestrates
agentic sessions**. License: MIT.

Sections 1–6 are **objective facts** (from the repo, no Loopsmith framing). Section 7 is clearly
separated **interpretation**. Paths are under `components/` unless noted.

---

## 1. What it is

"AI automation platform for orchestrating agentic sessions on Kubernetes." Teams create **agentic
sessions** — automated tasks that clone repos, run an AI agent, and push results. Sessions are
stored in **PostgreSQL** and reconciled into **Kubernetes pods** via gRPC. Capabilities advertised:
agentic sessions (code review, bug fixes, research, dev); multi-agent workflows (configurable
prompts/models/repos); GitHub + GitLab (SaaS and self-hosted) via credential sidecars; K8s
execution with RBAC, resource limits, namespace isolation; `acpctl` CLI + generated SDKs
(Go/Python/TS). Size: ~406 Go, ~166 Py, ~124 TS files.

## 2. Components

| Component | Tech | Role |
|---|---|---|
| **API Server** (`ambient-api-server`) | Go + rh-trex-ai | REST + **gRPC**, PostgreSQL-backed. Source of truth. |
| **Control Plane** (`ambient-control-plane`) | Go | Watches the API server via **gRPC streams**; reconciles sessions into **K8s pods**. |
| **UI** (`ambient-ui`) | NextJS + Shadcn | Web interface for sessions and agents. |
| **Runner** (`ambient-runner`) | Python (FastAPI) | Executes the AI agent inside a pod; emits **AG-UI** events. |
| **MCP Server** (`ambient-mcp`) | Go | MCP tool definitions, deployed as **credential sidecars**. |
| **CLI** (`ambient-cli`) | Go | `acpctl`. |
| **SDK** (`ambient-sdk`) | generated | Go / Python / TS from the OpenAPI spec. |

Flow (from README): *User creates Session → API server persists to DB → Control Plane creates pod
→ Runner executes AI agent → results stream to API server → UI displays progress.*

## 3. Session data model + streaming (the core)

From `ambient-api-server/proto/ambient/v1/sessions.proto`:

- **`Session`** — `metadata (ObjectReference)`, `name`, spec/status fields.
- **`SessionMessage`** — `{ id, session_id, event_type (string), payload (string) }`. A session is an
  **append-only stream of typed events** (generic `event_type` + `payload` envelope).
- **RPCs:** `Get/Create/Update/UpdateStatus/Delete/List Session`; **`WatchSessions` → `stream
  SessionWatchEvent`**; **`PushSessionMessage`** (push a message *into* a session); **`WatchSessionMessages`
  → `stream SessionMessage`**.
- Sibling protos: `projects.proto` (`WatchProjects`), `inbox.proto` (`WatchInboxMessages`,
  `SessionMessagePush…`), `project_settings.proto`, `users.proto` (`WatchUsers`). RBAC middleware
  present.

So the messaging model is **DB-backed event-sourcing with gRPC server-streaming `Watch` RPCs for
fan-out and `Push` for injection** — multiple clients can watch one session's message stream; agents
and clients push messages in. Hierarchy: `Project → Session → SessionMessage`, plus a cross-cutting
`Inbox`.

## 4. Runner / harness bridging + AG-UI

`runners/ambient-runner` (per its README + `ADR-0006`): a **FastAPI** app exposing **AG-UI
run / interrupt / health** endpoints, layered:

- **Framework-agnostic bridge pattern** — `ambient_runner/bridge.py` defines `PlatformBridge` (ABC),
  `PlatformContext`, `FrameworkCapabilities`; `app.py` exposes `add_ambient_endpoints(app, bridge)`.
- **Bridges:** `bridges/claude.py` (`ClaudeBridge`, Claude Agent SDK), `bridges/langgraph.py`
  (`LangGraphBridge`, "validates the abstraction"); plus `ag_ui_gemini_cli/` (Gemini CLI).
- **AG-UI adapters:** `ag_ui_claude_sdk/adapter.py` — *"wraps the Claude Agent SDK and produces
  **AG-UI protocol** events, enabling Claude-powered agents to work with any AG-UI compatible
  frontend."* Imports `ag_ui.core` (`EventType`, `RunAgentInput`, `RunStartedEvent`,
  `RunFinishedEvent`, `AssistantMessage`, `ToolCall`, `FunctionCall`, …).
- Other endpoints: `/repos/{add,remove,status}`, `/workflow` (runtime workflow switching),
  `/feedback` (Langfuse), `/capabilities`, `/mcp/status`.
- **Observability:** Langfuse + optional **MLflow** tracing middleware wrapping the AG-UI event
  stream; secret sanitization / privacy module.
- The runner streams messages to the API server via a gRPC client (`_grpc_client.py`,
  `_session_messages_api.py`) and reads injected messages (`_inbox_messages_api.py`).

So the **canonical agent-event format here is AG-UI** (CopilotKit's agent↔frontend protocol), NOT
ACP. Agent SDK output is adapted → AG-UI events → carried as `SessionMessage{event_type,payload}`
→ gRPC-streamed to the UI.

## 5. MCP credential sidecars + git providers

`components/credential-sidecars/` ships sidecars: **`github`, `google`, `jira`, `k8s`** (+
`entrypoint`). `ambient-mcp` (Go) provides MCP tool definitions deployed as these credential
sidecars — i.e., per-provider credentialed tool access is delivered as **MCP sidecar containers**
next to the runner pod. Git providers: GitHub + GitLab (SaaS + self-hosted).

## 6. Spec-driven-development / "factory" process layer

The repo builds *itself* with an agentic-SDLC harness:

- **`factory.md`** — "Remote Factory" config (Goal / Scope `Modifiable`+`Read-only` / Guards);
  generates `.factory/config.json` in an "Init mode." **`sdd-manifest.yaml`** — spec-driven-development
  manifest.
- **`skills/`** (align, amber-review, devflow, discover, control-plane, frontend, integrations, …),
  **`workflows/`** (control-plane, sessions, specs, security, integrations), **`specs/`** (numbered:
  `001-coderabbit-integration`, `010-advanced-sdk-options`, `011-session-options-menu`, … + an
  `index.spec.md`).
- **`docs/internal/`**: `adr/`, `agents/`, `architecture/`, `design/`, `proposals/`, `plans/`,
  `superpowers/`, `reference/`, `observability/`, `integrations/`, `feature-flags/`. Astro Starlight
  docs site. `AGENTS.md`, `CLAUDE.md`, `BOOKMARKS.md` at root.

---

## 7. Interpretation for Loopsmith (separated from the facts above)

- **It is, structurally, a production K8s-native build of Loopsmith's factory-family runtime.**
  Mapping: control-plane = our daemon/formation reconciler; `Session` + gRPC `WatchSessionMessages`
  / `PushSessionMessage` = **the session-relay / multi-attach fan-out we said we'd have to build**
  (subscribe = `Watch`, inject = `Push`) — already implemented, DB-backed; runner `PlatformBridge`
  = our `harness` capability; MCP credential sidecars = our `identity` capability (github/jira/google/k8s);
  `ambient-ui` = the console; `Project → Session → SessionMessage` = a clean item/event model.
- **Canonical-stream nuance (revisits R-04).** This project — which is **web-UI-first like
  Loopsmith** — chose **AG-UI**, not ACP, as the agent-event format, and adapts the agent SDK →
  AG-UI. That suggests the cleaner split is: **agent-side source = ACP / Agent SDK; canonical UI
  stream = AG-UI; envelope = a `SessionMessage{event_type,payload}`-style record**. R-04's "ACP is
  the canonical stream" should be refined to "ACP/SDK on the agent side, AG-UI on the
  frontend/console side." (To decide.)
- **The big open question: build-on vs. design-reference vs. reuse-bits.** agent-control-plane could
  be (a) a backend Loopsmith builds on, (b) a strong reference architecture, or (c) a source of
  reusable pieces (the runner bridge pattern, the AG-UI adapters, the gRPC session/watch model, the
  MCP credential-sidecar pattern). It is MIT-licensed, Red Hat-aligned, K8s-native — overlapping
  heavily with the factory/developer blueprint. **This materially affects the "port BotMinter
  backend" assumption** and needs a CTO decision. *To be assessed, not assumed.*
- **Possible reuse/inspiration (unevaluated):** the `SessionMessage` event-envelope + gRPC
  Watch/Push streaming model; the `PlatformBridge` framework-agnostic runner abstraction
  (Claude/Gemini/LangGraph); the AG-UI adapter layer; MCP credential sidecars (github/jira/google/k8s);
  the Langfuse/MLflow observability wrap; the SDD `skills/workflows/specs` + `factory.md` process layer.

## Sources

- Repo (cloned): `projects/agent-control-plane` — <https://github.com/openshift-online/agent-control-plane>
- Docs site: <https://openshift-online.github.io/agent-control-plane/>
- Key paths: `components/ambient-api-server/proto/ambient/v1/sessions.proto`,
  `components/runners/ambient-runner/README.md`,
  `components/runners/ambient-runner/ag_ui_claude_sdk/adapter.py`,
  `components/credential-sidecars/`, `factory.md`, `docs/internal/adr/`.
