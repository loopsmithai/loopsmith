---
epic_issue: 1
epic_name: loopsmith-mvp
project: loopsmith
spine: kit + two reference implementations
date: 2026-06-14
---

# Loopsmith MVP — Rough Idea

> Captured verbatim-in-spirit from the operator's framing (2026-06-13/14). This is the
> anchor document; idea-honing (Q-NN) refines it. Subject to the operator's correction.

## One-liner

**Loopsmith is a kit for managing your loops** — the value-stream work model abstracted
above any harness, runtime, or task tracker. The MVP = **the abstraction/spec (the kit)**
+ **two conforming reference implementations**.

## Origin & relationship to BotMinter

- BotMinter is the **"implementation-first, extract-the-abstraction"** path, already done.
  Loopsmith extracts and refines that abstraction.
- The abstraction is only *real* if a **second, deliberately non-BotMinter implementation**
  also conforms. Hence **kit + 2 impls** (the operator's chosen spine, with the explicit
  reasoning that one impl = wishful thinking, two conforming impls = proof).
- **Reference impl #1 = BotMinter-next**: slick console + bootstrapping; built as a
  **new repo**; **ports BotMinter's proven backend** (not a toy backend).
- **Reference impl #2 = deliberately non-BotMinter** (proves generalization). Strong
  candidate: **Wazeer-style** (files + coding agent + skill `/loop /ping`, no Ralph) —
  maximally different from BotMinter. (To confirm in honing.)
- The journal at [`team/vision/JOURNAL.md`](../../../vision/JOURNAL.md) (§1, §10–14) is the
  **seed of the kit's abstraction** (context / loop / item / actor, ports, seams) and a
  current-state map of BotMinter. Loopsmith formalizes that into the kit.

## The kit (abstraction) — what it manages

- A **loop** = a value-stream work model. A loop can be as simple as *a directory + a
  coding agent* (Wazeer), or BotMinter, or Gastown, or autonomous agents
  (Hermes / ZeroClaw / OpenClaw), or a mix. Loopsmith does not care — the abstractions
  transcend them.
- Working-draft kernel (from the journal, subject to refinement): **context · loop · item ·
  actor (human|agent)**; member = actor↔context binding; **ports** (driver = home link,
  port = foreign link; two dials: authority read/write, disclosure full/projected); privacy
  = projection at the port (least disclosure); mental model: *autonomy is bounded by visible
  context; touchpoints are context gaps; you widen autonomy by connecting context.*

## The MVP UX — cornerstone is bootstrapping

1. **Install** = a script that installs a CLI. That is the entire install story.
2. **Onboarding** through *some* interface (TUI / CLI / inside-the-coding-agent / desktop /
   web — assume all deliver the same experience for now; the surface is an open question).
   A polished first-run (Apple-Vision-/Meta-Quest-style): teach *"you live in a world of
   contexts; Loopsmith helps you stay on top of them; here's what a context is."*
3. A **persona** ("Smith" / "Minty" — name TBD) runs behind it as an AI session; **how it
   runs is abstracted** (a brain + skills + chat UI, or a chat session whose LLM summons
   fancy UI via tools/MCP — it depends).
4. **Add your first context** (maybe drag-and-drop). The first context ships with a
   **default minimal loop = the chief-of-staff loop** (small, between you and an AI). The
   persona offers: focus on personal life, professional life, or build both together.
5. This is the equivalent of BotMinter standing up *first agent + first project board +
   first ralph.yml* — but **none of it is hardcoded.**
6. As onboarding continues, the persona sets things up (sometimes itself, sometimes
   instructing you): curate the GitHub org, curate the project board, deploy autonomous
   agent(s) — the runtime equivalent of today's BotMinter — **and** onboard the Wazeer half
   (career, objectives, team, email, open PRs).

## Differentiator: person-first & connected

- Start from **you**; extend personal → career → team → daily coding tasks. Everything
  branches from your account, so it is **naturally connected** (vs. BotMinter feeling
  disconnected from the rest of your world).
- Concrete friction removed: today a PR an agent opens in the bot-squad hypershift fork
  requires a *separate* personal Claude Code session (personal PAT) to mirror it into
  `openshift/hypershift`. Loopsmith's connected contexts + ports natively understand how the
  agent's PRs flow into your team.

## Console-first + the dual-UX invariant (architectural correctness test)

- The **console/dashboard is the primary daily UX**: chats, work-item state, contexts
  (view / tweak / state), knowledge, invariants.
- A files-+-GitHub-only UX is **not** the MVP (already proven in BotMinter). The MVP builds
  the **slick console**.
- **But** the file / GitHub / CLI UX must remain possible over the **same core**. *"The MVP
  can drive both UXes"* is the test that the abstractions and architecture are right — this
  is a **project invariant**, not a nice-to-have.
- **Backend:** port BotMinter's proven backend (GitHub interaction, workflow, daemon,
  sessions); do **not** toy-build. Build the abstraction + console fresh.

## Terminology

- **Wazeer ≈ CoS** (used interchangeably). Lineage: the operator used Wazeer (project +
  persona) regularly → realized he needed a "Wazeer" inside BotMinter → role started as
  "team manager" → evolved into Chief of Staff. **"Wazeer-style"** = the implementation
  detail of the github.com/devguyio/wazeer project: everything is files, no Ralph, the loop
  *is* a skill (`/loop /ping`).

## Open questions (for idea-honing)

- Persona name: **Smith** vs **Minty**.
- Delivery surface(s) for the MVP: TUI / CLI / inside-coding-agent / desktop / web — and
  which is built first.
- Where MVP bootstrapping **stops**: first context + minimal CoS loop live, vs. all the way
  to deployed autonomous agents + curated org/board.
- The non-BotMinter impl #2: Wazeer-style, or something else?
- BotMinter-next: confirm new repo; what exactly is ported vs. rebuilt.
- Loopsmith **org/repo structure** for the kit.
- **TBD:** does BotMinter survive as BotMinter, or fold into Loopsmith and retire?
