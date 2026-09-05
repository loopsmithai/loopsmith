# Concept Reconciliation — BotMinter → Loopsmith (#178)

Loopsmith is the **generalization of BotMinter**, not a 1:1 UX port. This document runs every
significant BotMinter subsystem through a **product-first lens** and records a verdict that feeds
§4 (Components) and the §8 design decisions (D-NN) of [design.md](../design.md).

## The lens (three questions per concept)

For each BotMinter concept/subsystem:

1. If we **port it as-is**, does it give the **UX we want**?
2. Does porting as-is **lock in real technical debt** (a one-way door — fixing it later means
   re-architecture)?
3. If we **build it from scratch today**, do we get **better UX / ergonomics / story**?

The default is *not* "preserve, change only when forced." It is "build the product we want;
BotMinter's implementation is a **candidate to reuse**, and the bar is UX + avoiding debt." The
lock-in risk is almost always the **slot/capability**, not the Nth provider. **Now is the only
refactor window**, so model debt is resolved here, not deferred. Evidence below is grounded in the
`squash/ct03` code (`crates/bm/src/…`).

## Tier A — Reuse as-is

Porting gives the UX we want, carries no debt, and a from-scratch rebuild would not be better.
These are invisible plumbing.

| Concept | Today (evidence) | Why reuse |
|---|---|---|
| daemon / event-bus | `daemon/` — scheduler + GitHub webhook/poll + lifecycle | solid plumbing, invisible to UX |
| session execution | `session/` drives the `ralph` binary; records / history / retention / work-item locks | `ralph` is the proven loop engine; the UX sits above it |
| runtime placement | `topology/` + `formation/lima.rs` — local process / k8s / lima VM | becomes `runtime` extensions unchanged |
| GitHub operations | `git/` — repo / board / label / fork / app-auth | the impl behind `github` tracker + `github-repo` source + `github-app` identity |
| credential storage | `config/` + keyring | sits beneath the `identity` capability |
| workspace hydration | `workspace/` (12.6k lines) — assembles a member's files (CLAUDE.md, skills, knowledge, invariants) from the team repo + credential relay | this *is* the `context` / `home-source` materialization mechanism |
| agent driving | `acp/` (Agent Client Protocol), `agent_tags/` | how a step drives a coding agent; sits beneath `harness` |

## Tier B — Reuse the engine, reframe the concept

The code is fine; the **concept boundary** carries the debt. Keep the mechanisms, change the
framing.

- **brain → chief-of-staff loop + interface.** `brain/` (7.8k lines: `event_watcher`,
  `heartbeat`, `multiplexer`, `queue`, `inbox`, `bridge_adapter`, `prompt_template`) is a real,
  working digest engine — but it is **special-cased as a member** (`formation::is_brain_member`,
  `launch_brain`, `BrainLaunchConfig`, separate from `launch_ralph`) and **hardwired to bridge
  delivery** (`bridge_adapter`). Keep the mechanisms; kill the "special brain" framing → the
  digest becomes a visible, editable **loop** (the chief-of-staff template) delivering via the
  `interface` capability. **Consequence:** `digest` does **not** need to be a capability *type*
  (R-03 kept it coarse "type later") — it is a **template** using `interface` + `tracker`. This
  drops one type-driver from the catalog. *(See the detailed write-up below.)*
- **profile bundle → family / blueprint / template / capability.**
  `profiles/agentic-sdlc-planning/` is one flat directory (botminter.yml + roles + workflows +
  brain + bridges + coding-agent + knowledge/invariants). Reuse the content; decompose the
  structure (Δ-2). No new behavior, better model.
- **status graph + labels → `tracker` config.** Reuse as the `github` extension's configuration;
  generalize the slot so `files-checklist` and `jira` fill it.

## Tier C — Build fresh now

Port-as-is fails the UX bar or locks in debt, and from-scratch is materially better. Each still
**reuses proven ops underneath** — this is not "rebuild everything."

- **interface (unify).** Today there are **three fragmented paths**: `web/` is a **read-only
  dashboard** (`/api/teams`, `…/overview`, `…/members`, `…/files`, `…/sessions`, + `sync` —
  no chat, no design, no onboarding), `bridge/` is a separate external-process subsystem
  (manifests, recipes, rooms, identities, lifecycle), and `chat/` is a local CLI agent spawn
  (assembles a meta-prompt, launches via `acp/`). The product wants **one interactive primary
  surface** (chat + design + digest) with channels (cli / telegram / matrix) as **extensions
  bound to loops/actors**. → build the `interface` capability + interactive console fresh; reuse
  the read endpoints as digest/view feeds.
- **loop-as-structured-data + Loop Studio.** Today a loop is `ralph.yml` — a structured
  event-graph (`hats` / `triggers` / `publishes`) wrapped around large **prose** prompt blocks.
  Hand-editing YAML is not the UX, and the structured model is the **forcing function**. → build
  the kernel loop model + Loop Studio fresh; the triggers/publishes semantics carry over as the
  event wiring.
- **bootstrap / onboarding.** Today `bm init` / `hire` / `start` CLI choreography. BOOT-02/03/04
  want Smith-driven console onboarding + reconcile + dual-verify. → build the onboarding loop
  fresh; reuse create-repo/board + hire+identity ops beneath it.
- **port + membership.** Today `ProjectDef { name, fork_url }` — coarse. COMP-03/13/14 want
  authority + disclosure dials + declared membership. → build the port/membership model fresh;
  reuse fork/PR ops underneath.
- **reconcile / verify engine (generalize).** Reconcile exists **three times, disconnected**
  today — the daemon (desired-vs-actual member placement), the shepherd hat (story-gap
  reconciliation), and onboarding. → unify as one loop pattern (BOOT-07/08); the daemon's
  reconcile loop is the seed.

## Open forks (need CTO sign-off)

1. **Brain:** dissolve the *concept* entirely (reuse engine as the CoS loop, drop `digest` as a
   capability type)? *Lead recommendation: yes.*
2. **ralph** stays the loop-execution engine under the new structured model (reuse), not a
   from-scratch rewrite? *Lead recommendation: reuse.*
3. **base family / simple-assistant** no-daemon runtime (loop-as-`/ping`, no session/ralph) — build
   that path now as the D-i proof, not defer? *Lead recommendation: build now.*
