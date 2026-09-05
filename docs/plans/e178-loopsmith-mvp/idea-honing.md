# Idea Honing — Loopsmith MVP (#178)

This document records the **settled decisions** for the Loopsmith MVP, in the **kit frame** that #178
landed on. #178 builds **the kit** — *conformance contracts* + *packaging grammar* + *packaging skills* —
an implementation-decoupled definition of what an agentic-LLM setup must be to **conform**, plus the
skills that package any setup into a conforming one (run by **Smith**, the kit's consultant agent). **The
kit ships no runtime.** The runtime people picture (console, Loop Studio, daemon, the cross-context digest)
is the **first product the kit packages** — the proving ground, archived in
[research/R-06](research/R-06-first-product-developer-blueprint.md) and
[research/R-07](research/R-07-first-product-requirements.md) — **not** what #178 designs. (Smith himself is
*kit* machinery, shipped OOTB in that product — see Q-11.)

Each answer is self-contained and uses the canonical vocabulary below. The packaging/extensibility model
is honed to convergence in **Q-20/Q-21/Q-22**, stress-tested against real BotMinter code in
[research/R-03](research/R-03-botminter-on-loopsmith.md). The first product (the developer-blueprint
runtime) is archived in [research/R-06](research/R-06-first-product-developer-blueprint.md) and
[research/R-07](research/R-07-first-product-requirements.md).

> **Note on Q numbering:** Q-NN identifiers are stable — do not renumber (downstream artifacts reference
> them). This rewrite re-frames each answer to the kit; it does **not** renumber. Q-21/Q-22 are the later
> honing (the kit/product split and the packaging skills) that the original Q-01…Q-20 predated.

## Canonical vocabulary (used consistently throughout)

- **kit** — what #178 builds: **conformance contracts** + **packaging grammar** + **packaging skills**,
  carried by **Smith** (the agent that runs them). Implementation-decoupled; it ships **no runtime**, and the
  catalogue of packages is **produced by running the kit** (it starts empty).
- **conformance contract** — what a setup's parts must *be and do* to conform. Two kinds: the **kernel
  contract** (work expressible as the five nouns) and a **capability contract** per capability. Each has
  three faces: **structural** (shape) · **data** (schemas + invariants) · **behavioral** (operations +
  guarantees). A setup **satisfies a contract**; it never "satisfies" a blueprint (a blueprint is a
  *package* the kit emits).
- **package** — the unit of reuse the kit composes a setup from. A package declares the **capabilities it
  needs** and the **capabilities it provides**. Everything reusable — a tracker, a skill, a template, a
  substrate — is a package.
- **capability** — the open vocabulary of what a setup needs or offers: `tracker`, `source`, `runtime`,
  `harness`, `interface`, `identity`, `planning`, and also skills, templates, and more. Open-ended, not a
  fixed list. Packages are described by the capabilities they need and provide.
- **family** (`factory` / `base`) — a curated **substrate**: the base set of packages a setup is built on,
  which determines what's available. **blueprint** (`developer` / `simple-assistant`) — a curated **starting
  set** on a family: its intent + the templates it offers. ("distro" / "distribution" = **blueprint**.) A
  **loop** is the running instance.
- **agent template** — a capability a package provides: a **who-seed** — a persona + its skills + subagents
  + supported harness (e.g. `engineer`, `sentinel`, `Smith`). Instantiated into an **actor**.
- **loop template** — a capability a package provides: a **process-seed** — steps · events · gates,
  agnostic of who runs it (the `ralph.yml`-equivalent). Instantiated into a **loop**.
- **binding** — instance-time composition: **bind** an actor (from an agent template) into a context and
  **assign** it to a loop (from a loop template), with an **equip/train fit-check** (is the actor equipped
  for the loop's steps; if not, train by adding skills). agent template ⊥ loop template; the binding joins
  them.
- **packaging skills** — the flow of sub-skills **Smith** runs to turn a *target* (an existing setup, or a
  stated intent/need) into a conforming setup: **discover → recommend → co-design → realize → verify** (+ a
  cross-cutting **learn**). Reuse-first; they ship no runtime. Smith runs them across a repertoire of loops —
  **build / furnish / maintain**.
- **Smith** — **kit machinery**: the consultant/architect agent that *runs* the packaging skills (he *does*
  discover→recommend→co-design). The **single irreducible seed** (everything else is produced by running the
  kit). Ships credentialed on the structural model and apprenticing on agentic best practices, which he
  grows by authoring skills (skill-packages) with the human. Re-packaged as an agent template; the developer
  blueprint ships **Smith + a bootstrap loop OOTB**.
- **catalogue** — the body of reusable packages the skills draw from and grow. Starts **empty** and grows
  per run — the **flywheel**: authoring is front-loaded and decays as the catalogue matures.
- **Kernel nouns — exactly five:** `context` · `loop` · `item` · `actor` · `port`. Nothing else is a
  kernel noun.
- **port** — the kernel's connection/access mechanism to another context or system; carries an
  **authority** dial (read / write / none) and a **disclosure** dial (full / projected). Port is the
  *only* access mechanism.
- **membership** — a *declared* label on a connection (which context a fragment belongs to), trusted by
  fiat; **independent of access** (the port's authority). A context = the merge of all same-membership
  ("same-color") fragments across sources.
- **home source** — the single context source a local context designates as its anchor/identity (1:1 with
  the context). A *role* filled by a context source — not a hardcoded constant. The MVP type is a **git
  context source** (local or remote git repo); it holds the context's files (CLAUDE.md, settings, skills,
  knowledge, invariants). The operation is **setting the home source**.
- **first product** — the developer-blueprint **runtime** (BotMinter-equivalent: daemon, console, Loop
  Studio, the cross-context digest). The **first setup the kit packages** and the source of the dogfood
  demo. Designed in R-06; its requirements archived in R-07. **Not the kit** — and **Smith is *kit*
  machinery, shipped OOTB in this blueprint**, not a product-only persona.

---

## Q-01: What does the MVP build, and where does it stop?

**Answer:** The MVP builds **the kit** — conformance contracts + packaging grammar + packaging skills,
carried by **Smith** (the consultant/architect agent who *runs* the skills; the single irreducible
machinery seed) — **not a product**. The kit ships **generators, never instances** (no one-offs): the
**catalogue starts empty**. It is a **proof of the kit**: the boundary is the **smallest slice that defines
the full kernel contract and is satisfied by two deliberately-different conforming setups *that the skills
produce*** (one BotMinter-style `developer`, one non-BotMinter `simple-assistant`) — produced, not
hand-shipped. The success metric is **kernel-contract coverage + dual-setup conformance via the skills**,
not "a usable Jarvis." The product *runtime* (console, the onboarding surface) is the **first packaged
product** (R-06), the proving ground — not the deliverable; **Smith himself is kit machinery**, later
re-packaged as a conforming agent template that the product ships.

## Q-02: Which of the kernel's *distinctive* claims must conformance make falsifiable?

The kernel contract's distinctive, falsifiable claims — its behavioral face — that two deliberately-
different conforming setups validate:

| Distinctive claim | What "both setups conform" proves |
|---|---|
| **D-i. Tracker-agnostic loop** — same loop definition runs over a github-tracker setup AND a files-checklist-tracker setup | the loop/tracker split is real |
| **D-ii. Context + port** — connecting a second context changes what a loop can do (autonomy via context) | the context/port model is real, not config |
| **D-iii. Actor uniformity / touchpoint** — a step's performer is swappable human↔agent via the same kernel op | the actor model + the touchpoint work |
| **D-iv. Membership ≠ access** — a datum's context is fixed by **declared membership**, not by access/transport | the membership rule holds |

**Answer:** **All of D-i…D-iv are required** — every kernel concept must be present or the contract isn't
viable. The minimization axis is **cardinality, not concepts**: the *few* packages/ports/contexts that
still constitute a real conforming setup, not tens.

## Q-03: What are the two reference setups the kit packages?

**Answer:**
- **Both setups use files for LLM-context** — GitHub *is* files. The distinguishing axis is the
  **tracker**, not "files vs no files."
- **`developer`** (BotMinter lineage): tracker = `github`; console-primary surface; daemon/sessions
  backend. This is the **first product** the kit packages (R-06).
- **`simple-assistant`** (Wazeer lineage): tracker = `files-checklist` (markdown checklists in the home
  source, visualized via Obsidian / a neovim plugin); CLI surface; loop-as-skill (`/ping`), no daemon.
  The deliberately-different second setup that keeps the contracts honest.
- **D-i is precisely tracker-agnosticism**; the swap seam = the **tracker capability** (packages:
  `github` / `files-checklist`).
- **Dual-setup + dual-UX collapse into one proof:** the two setups *are* the two surfaces. Dual-UX
  *within* a single setup is post-MVP.

## Q-04: MVP scope = contracts + skills + Smith; the catalogue is *produced by running the kit*

**Answer:**
- **No one-offs — the kit ships generators, never instances.** What ships at the kit level: the contracts,
  the grammar (the *shapes*), the packaging skills (including the skill to author each grammar kind), and
  **bootstrap-Smith**. Every concrete artifact — families, blueprints, agent/loop templates, packages — is
  **produced by *running* the kit**, first during run #1 (Smith packaging BotMinter). The **catalogue starts
  empty** and grows per run: authoring is **front-loaded and decays** as families, blueprints, and packages
  accumulate (the flywheel, Q-20).
- **Two cardinalities, kept separate:** *user-instance* cardinality (contexts/ports/loops a packaged setup
  creates) is **dynamic and unbounded**; the **"the few, not tens"** discipline now governs **what run #1
  authors**, not a shipped seed.
- **Kernel verbs the contract must cover** (none optional): create/edit a **context**; compose/edit a
  **loop** (steps + events + gates); connect a **port** (authority + disclosure dials); add an **actor**
  (human/agent) and (re)assign it to a step; **items** flow with **touchpoints**; run over a chosen
  **tracker**.
- **What the two runs *produce* (the few):** 2 families × 1 blueprint (`factory`→`developer`,
  `base`→`simple-assistant`); the `github` + `files-checklist` trackers (+ `jira` for the swap); the agent
  templates + loop templates a blueprint offers; the few source/identity packages — **all authored by the
  skills, none hand-shipped.**
- **The structured model is forced by authoring, not by a GUI.** Any packaging — visual designer or
  conversational — can only manipulate a **well-defined structured model**, so the grammar forces the
  kernel to be **real structured data, not prose**. (In the first product, that designer is Loop Studio;
  but the structured-model requirement is the kit's, independent of any UI.)

## Q-05: The two contexts and their two loops — the kernel facts (and the dogfood demo)

This is the operator's dogfood setup — the **acceptance demo** of the first product, and the source of the
kernel-contract facts below.

- **Context #1 — Personal.** Hosts the **Smith** home loop that sweeps work-streams and surfaces a digest.
  Connections: a GitHub project board + a personal home source (git).
- **Context #2 — Professional.** **Smith is a member here** (one actor, two contexts). Reuses the same
  board but a **different home source** (two contexts must not share a home source). Adds a second actor —
  **engineer** — with its own loop.

**Derived kernel-contract facts (carried into the contracts):**
1. **Context ⇔ home source is 1:1.** The home source is the context's anchor fragment / identity. Two
   contexts MUST NOT share a home source.
2. **The board is a shared tracker surface, not the context boundary.** One board can host items from
   multiple contexts; board-visibility ≠ membership.
3. **An actor binds to N contexts; membership — not elevation — enables the digest.** Membership sets a
   port's disclosure to member-grade, so an actor reaches every context it belongs to with full,
   in-session access. (Access is via a **port**; membership only sets the dial — see Q-08.)

## Q-06: What is a "project" in this model?

**Answer:** A "project" (as BotMinter uses it across botminter/hypershift/ralph-orchestrator) is **not a
kernel noun.** A project is a connection to an **external context** from the `github-repo` source — a
different-membership ("different-color") context reached via a **port** with an authority dial (write via
fork→PR, not push). An item's `project/<name>` is simply which external context/port it targets. If
instead you *declare* the repo into your own context, it **merges** (it becomes internal) rather than
being a "project."

## Q-07: External contexts are *typed*; the type carries type-specific operations (e.g., fork)

**Answer:** External contexts have **types** — modeled by the **`source` capability**. The first source
package is **`github-repo`**, which exposes type-specific verbs (`fork`, `branch`, `PR`, `issues`) a bare
port can't.

- A context has an **identity / namespace** (e.g. personal = `devguyio`; coding = the `devguyio-bot-squad`
  org); when the source materializes resources it acts **as that identity**.
- **`fork` mints a new same-type external context**, recording an upstream→fork edge. Connecting the same
  upstream from two home contexts builds a **fork chain**:
  `openshift/hypershift` → `devguyio/hypershift` → `devguyio-bot-squad/hypershift`.
- **The fork chain is the native fix for cross-identity PR friction:** an agent's PR in the bot-squad fork
  has a known path up the chain — no separate personal-PAT session to mirror it.
- **`fork` is gated by the authority dial** — a read-only external context does not fork; flipping
  write-authority ("I intend to contribute") is what enables it.
- The **`source` capability is a separate seam from the `tracker` capability** (confirmed in Q-08).

## Q-08: Context (local/external), the home source, and the tracker

**Answer:**
- **Context** = the world/instance (members, loop, items, identity). **Local** (yours; you run its loop)
  or **external** (connected; dialed authority/disclosure). Local vs external = **membership + dials, not
  a different noun.**
- **Home source** — every local context designates exactly one (1:1 with the context, per Q-05); it holds
  the context's files. The home source is a **designated context source** (a *role*, filled via the
  `source` capability), **not** a hardcoded constant; the MVP type is a **git context source**. The
  operation is **setting the home source**.
- **`tracker` capability** — where the loop's items live; the **only** axis distinguishing the two
  reference setups (D-i). Modules: `github` | `files-checklist`. (Files-checklist lives in the home
  source; "files vs no files" is not the axis — GitHub is files too.)
- **`source` capability** — how you *reach* a context (especially external ones). Module: `github-repo`
  (fork/PR/issues); `calendar`/`email` post-MVP. Separate seam from `tracker`.

**Two seams for the contracts:** the `tracker` capability and the `source` capability. The **home source**
is itself a context source: every local context designates exactly one source as its home source.

## Q-09: Repo/org structure for the kit and the two reference setups

**Answer:** A **monorepo** `loopsmith/loopsmith` — so a contract change + both setups' packaging + the
conformance proof land in one PR while the abstraction is still settling (split into polyrepo once it
stabilizes; keeps the experiment net-new, no touching BotMinter's repo):

```
loopsmith/loopsmith
├── kit/                    # contracts + packaging grammar + packaging skills (no runtime)
└── distributions/         # the two packaged setups (opinionated blueprints on a family)
    ├── developer/         # packages the first product (R-06) — github tracker; its console/daemon
    │                      #   runtime is the *product's*, not the kit's
    └── simple-assistant/  # files-checklist tracker, notes, CLI
```

A directory under `distributions/` ships **one blueprint** (Linux analogy: kit = the spec + tooling;
blueprint ≈ Ubuntu/Fedora; the runtime under it = a product).

## Q-10: Names for the two reference blueprints

**Answer:** **`developer`** and **`simple-assistant`** — named by audience, not by tracker plumbing:
- `developer` — SWE-facing (github tracker, console, daemon); the first product (R-06).
- `simple-assistant` — for anyone wanting an AI PA/coach (files-checklist tracker, notes/Obsidian, CLI);
  the `simple-` qualifier signals the lighter, no-GitHub-required blueprint.

## Q-11: Smith — the consultant agent (kit machinery)

**Answer:** **`Smith` is kit machinery** — the consultant/architect agent that *runs the packaging skills*.
He is what *does* discover → recommend → co-design (Q-22): the agent who builds the factory family (n-2)
and the developer blueprint (n-1) by packaging BotMinter. He is the **single irreducible seed** — the one
hand-assembled artifact; everything else is **produced** by running the skills (Q-04).

Smith operates in three persona-driven modes:

**Training mode (P1 — Loopsmith Developer).** Smith ships credentialed on the kit's structure and
apprentices on craft through supervised runs with the developer. Training completes when Smith is
**re-packaged as a conforming agent template** — Smith becomes a package that a blueprint ships.

**Building mode (P2 Family Author, P3 Package Author).** P2 and P3 use Smith's **build** loop to author
families, blueprints, templates, and packages (empty-catalogue, heavy authoring).

**User mode (P4 — End User).** P4 uses Smith for two activities:
- **furnish** — onboard: fit a built blueprint to a user's day-to-day (add contexts, pick agent + loop
  templates, wire them — Q-14/Q-15); reuse-heavy;
- **maintain** — troubleshoot a running setup (shepherding/escalation — Q-13).

The **build loop is re-entrant** across modes: hit a missing piece in user mode and Smith drops into
**building mode**, produces it properly (no hand-hack — Q-04), and returns. The developer blueprint ships
**Smith OOTB**, so at n-0 the same Smith that P1 trained onboards and serves P4. (A persona is run by a
harness and bound to a loop; users may rename their own instance. The *console* that fronts Smith in the
product is R-06; **Smith himself is the kit's.**)

## Q-12: The concrete observable scene(s) that mean the first product works — the demo

**Answer:** Two demo journeys of the **first product** (the **acceptance demo** — illustrative; the
generic, user-observable *features* are honed in Q-18/Q-19):

- **Day 1 — Bootstrapping & design.** Install → onboarding → set up two contexts and their loops (the Loop
  Studio design experience). Detailed in Q-14/Q-15.
- **Day 2 — The console digest.** The operator opens the console; **Smith** presents one unified digest
  mixing **personal-context** flagged items (e.g. an email reminder), **professional-context** flagged
  items (e.g. a teammate's PR awaiting review), **agentic-work status** (e.g. work awaiting acceptance;
  backlog below the configured WIP minimum → recommend a triage + planning meeting), and an
  **agentic-work summary** of finished work.

*(Concrete examples are demo data, not features — they are the first product's acceptance demo; the
generic, user-observable features are honed in Q-18/Q-19.)*

## Q-13: The day-2 failure/escalation (shepherding) scene

**Answer:** The first-product demo includes **one escalation archetype**: Smith detects a stuck loop
(repeatedly failing PRs), root-causes it to a credential/authority gap it **cannot self-fix** (e.g.
invalid e2e test-app creds), and **escalates to the operator** with the finding and a recommended action.
This is the concrete proof of *shepherding* (a touchpoint = a context gap), not mere aggregation. It is
**not** a general anomaly-detection engine. (A first-product behavior, resting on the kernel's
touchpoint — the actor/authority gap of Q-04/Q-05.)

## Q-14: Day-1 entry — where onboarding happens (first product)

**Answer:** **Console-first.** The first product's CLI only installs and launches the console. Onboarding
happens **in the console**, driven conversationally by Smith ("you live in a world of contexts"); the
first thing the user does is **add their first context.** This is the **product embodiment** of the kit's
packaging skills (Q-22) — onboarding *is* a packaging run with a console front-end.

## Q-15: The day-1 design journey — the product embodiment of the packaging skills

**Answer:** Day-1 is a **design→reconcile→verify** journey, itself run as a Smith-driven loop (the system
loop's bootstrap phase). It is the **first product's embodiment of the packaging skills** (Q-22 — the
same represent → synthesize → author → realize → verify flow, here with a console front-end):

1. The user works in the **designer ("Loop Studio")** — drag-and-drop, edit — then clicks **Save**.
2. The user **tells Smith they finished.**
3. Smith reads the **desired state** (the saved artifact) and **reconciles** it: asks clarifying
   questions, double-checks, suggests improvements, and translates the structured artifact into the real
   loop (home source, connected sources, wiring). *(This is realize.)*
4. Smith **verifies** the loop is in place, healthy, and running, then **asks the user to verify too**
   (dual verification). *(This is verify.)*

**Phases are emergent, not a fixed wizard:** Smith reconciles desired-vs-current and pings the human at
each touchpoint — e.g. **Phase 1 Foundation** (stand up the first loop), **Phase 2 Configuration** (Loop
Studio highlights unconfigured items, e.g. GitHub; the user goes through a guided GitHub App
creation+installation flow parallel to `bm init`).

Notes:
- The designer is **"Loop Studio"** (a *product* surface; the kit's equivalent is authoring a template —
  Q-22).
- A context's **home source** must be set for it to persist; the working loop comes from this
  design→reconcile flow — **no auto-seeded per-context loop.**
- The dogfood ordering (personal context first) is a **demo/UX choice, not a requirement.**

## Q-16: Delivery surface (first product)

**Answer:** **Web UI (browser)** for the first product. Desktop packaging (Tauri vs Electron) is a cheap
post-MVP wrapper — see [R-01](research/R-01-console-desktop-packaging.md) — intentionally not detailed.
(Product concern; the kit treats `interface` as a capability.)

## Q-17: Loop Studio canvas — PARKED

**Answer:** Concrete Loop Studio canvas/primitives are deferred to a dedicated post-idea-honing UX
research phase (avoid over-specifying product UX now). Seed work exists in BotMinter's ralph-designer
(#146: `WorkflowNode` / `WorkflowEdge` / `WorkflowGraph`).

## Q-18: What building blocks does the MVP need?

**Answer:** The inventory falls out of the kit (contracts + grammar + skills) and — separately — the first
product:

**1. Kernel nouns** (all required, the kernel contract): `context` (local/external) · `loop` · `item` ·
`actor` (human|agent) · `port` (authority + disclosure dials).

**2. The kit:**
- **Conformance contracts** — the kernel contract + a capability contract per capability, each in three
  faces (Q-21).
- **Packaging grammar** — packages (what a setup is composed from) + families / blueprints / agent
  templates / loop templates (Q-20).
- **Packaging skills** — discover → recommend → co-design → realize → verify (+ learn) (Q-22); **no
  runtime**.
- **Smith** — the consultant agent that runs the skills (Q-11) — and the **friction log** (the learning
  instrument).

**3. The first product (the proving ground, R-06 — NOT the kit):** the reconcile/verify runtime, the loop
runtime, the console + Loop Studio surfaces, the cross-context digest. (Smith is *kit* machinery, shipped
here OOTB — Q-11.)

**4. What the first runs *produce* (the catalogue starts empty — no one-offs):**
- **Two families × one blueprint:** `factory`→`developer`, `base`→`simple-assistant`.
- **Trackers:** `github`, `files-checklist` (+ `jira` for the swap).
- **Source:** `github-repo` (+ guided GitHub App setup). (`email`, `calendar` post-MVP.)
- **Agent + loop templates** the blueprints offer (e.g. engineer, sentinel).

All **produced by the skills**, none hand-shipped.

**Distro vs template:** "distro/distribution" = **blueprint**. A **loop template** is a generic process seed
("what do you do?"); an **agent template** is a who seed (persona + skills). A *product-level* template-
management UX (browse/import/export in Loop Studio) is post-MVP and **product-scoped**.

## Q-19: What features does the MVP need?

The MVP's features split by where they live:

**Kit features (what #178 builds):**
- **Packaging skills** — discover the target, recommend an opinionated best-fit reuse-first, co-design
  (author) what's missing, realize, verify, and a cross-cutting learn.
- **Conformance** — the kernel contract + a capability contract per capability (three faces each); verify a
  candidate setup and gate on conformance.
- **Packages & grammar** — compose a setup from packages (Q-20); swap a capability provider in place (the
  tracker swap); a family determines what's available; author at every level; the catalogue grows per run.
- **Kernel & port behavior** — tracker-agnostic loop, dialed ports, typed sources, actor uniformity,
  multi-context membership, membership≠access, and one home source per context.

**First-product features (the proving ground — archived in research/R-06 & R-07, not the kit):** CLI
install; console-first Smith onboarding; phased reconcile setup; Loop Studio design→reconcile→verify;
guided source setup; chat with Smith; the cross-context digest; per-context flagging; agentic-work
status/summary; a configurable work policy; shepherding/escalation. These remain the **acceptance target**
of packaging the first product.

## Q-20: The packaging grammar — packages, levels, and templates

> The model, stress-tested to convergence against real BotMinter code in
> [R-03](research/R-03-botminter-on-loopsmith.md) (15 rounds). *(How packages declare and resolve their
> needs is design-level mechanism — not honed here.)*

**Everything reusable is a package.** A setup is composed from packages, each described by the capabilities
it **needs** and **provides** (Q-08). The kit picks **reuse-first** from the catalogue and authors a new
package only for what's missing; authored packages grow the catalogue (the **flywheel** — authoring is
heaviest on the first run and lightens as the catalogue matures).

**The levels (invariant core → running instance):**

| Level | What it is | Examples |
|---|---|---|
| **kernel** | the invariant core — never varies; everything conforms | context · loop · item · actor · port |
| **family** | a curated **substrate**: the base package-set a setup is built on, which determines what's available | `factory` / `base` |
| **blueprint** | a curated **starting set** on a family: its intent + the templates it offers | `developer` / `simple-assistant` |
| **agent template** | a reusable **who-seed** — a persona + its skills + supported harness | `engineer` · `sentinel` · `Smith` |
| **loop template** | a reusable **process-seed** — steps · events · gates, agnostic of who runs it | the engineer process · the sentinel process |
| **actor / loop** | the running instances; a **binding** hires an actor (from an agent template) into a context and assigns it to a loop (from a loop template) | the engineer actor on the engineer loop |

**Two reusable seeds, composed at use:** a BotMinter "role" is **not** one thing — it is an **agent
template** (who) plus a **loop template** (process), joined by a **binding** that also checks the actor is
equipped for the loop's steps (and trains it — adds skills — if not). "add an agent" picks an agent template;
"add a loop" picks a loop template.

**What the model supports** (mechanism in design): a capability can be **swapped in place** — change the
package providing `tracker` from github to jira and the loop is untouched (the headline MVP swap, CAT-04);
and a **family determines what's available** — a substrate that doesn't include a capability forecloses it
(base has no daemon, so no `runtime`). The kit can **author at every level** — a package, an agent or loop
template, a blueprint, or a family — so any gap can be filled, not only the levels that happened to exist.

## Q-21: The conformance contracts — what a setup must satisfy

**Answer:** Conforming is defined by **contracts**, not by resemblance to BotMinter. Two kinds:

- **The kernel contract** — a setup's work must be expressible as the five nouns (`context · loop · item ·
  actor · port`), with the kernel data invariants (e.g. one home source per context; membership ⊥ access)
  and behavioral guarantees (D-i…D-iv, Q-02).
- **A capability contract per capability** — each capability (`tracker`, `source`, `runtime`, `harness`,
  `interface`, `identity`, `planning`) has a data contract (what it exposes) and a behavior contract (what
  it does). A `files-checklist` tracker and a `github` tracker both **satisfy the `tracker` contract**.

Each contract has **three faces**: **structural** (shape), **data** (schemas + invariants), **behavioral**
(operations + guarantees). The dividing line that took the most work to get right: a setup **satisfies a
capability *type* contract**; it never "satisfies" a `blueprint` — a blueprint is *grammar the kit emits*
(Q-20), not a thing to satisfy. The kit needs **no runtime** precisely because the behavioral face
specifies what a conforming setup must *do* without shipping the thing that does it — conformance is
testable against the contract.

## Q-22: The packaging skills — how a target becomes a conforming setup

**Answer:** The kit's active ingredient is a **flow of sub-skills**, run by **Smith** (Q-11), that takes a
*target* — an existing agentic setup **or a stated intent/need** — and produces a **conforming setup**. It
is **not transcription**; the skills:

- **discover** the user's needs (often just an intent — "what do you do?"): eliciting and refining, not
  reading off an existing system;
- **recommend** an opinionated best-fit setup, reuse-first from the catalogue — proposing structure the user
  never named and applying best practices (e.g. arriving at a factory-family developer setup unprompted);
- **co-design** whatever's missing with the user — authoring new packages/templates, and where existing
  material (like BotMinter) is in play, deciding together what to reuse, tweak, or rebuild;
- **realize** against the user's concretes (repo, credentials, tracker, harness);
- **verify** conformance and gate completion — with a cross-cutting **learn** that records friction.

> **You can think of it as if you walked into an *agentic-SDLC consultancy*:** you arrive wanting a local
> agentic setup (maybe you like Gascity, run Claude Code with a few plugins, want a single OpenClaw agent,
> or a full team); the firm interviews you about what you have and want (*discover*), recommends and reuses
> what fits (*recommend*), custom-builds whatever's missing (*co-design*), provisions and wires it up
> (*realize*), and signs off only once it all conforms (*verify*) — learning from every engagement. That
> consultancy is just an **analogy** for conveying the flow; the deliverable is the **skills** (run by
> Smith), not a company, and there are no "departments."

**Smith runs these skills across three persona-driven modes** — **building mode** (P2/P3: construct families,
blueprints, templates, packages via the **build** loop), **user mode** (P4: **furnish** — onboard, fit a
built blueprint to a user — Q-14/Q-15; **maintain** — troubleshoot a running setup — Q-13) — *the same agent
and skills*, because the one who built it can fix it. The **build loop is re-entrant** across modes: hit a
missing piece in user mode and Smith drops into building mode, produces it properly (no one-off hack — Q-04),
and returns.

Two properties keep it honest: **reuse-first** (authoring only fills gaps; what's authored grows the
catalogue — the flywheel, so authoring lightens over time), and **no runtime** (the skills emit a conforming
setup; running it is a *product's* concern — R-06). The catalogue **starts empty** and is produced by running
the kit (Q-04). The first product's onboarding (Q-14/Q-15) is **user mode** — the **furnish loop** with a
console front-end, not a separate mechanism.
