# Design Notes — Pending Decisions (parked for design.md)

Provisional `PD-NN` labels — these become `D-NN` (with full rationale + ADRs) when `design.md` is
written (epic-mgmt Step 8). Settled with the operator during idea-honing/model reconciliation; recorded
here so they survive context compaction. Authoritative model lives in
[research/R-03](research/R-03-botminter-on-loopsmith.md) and [idea-honing Q-20](idea-honing.md).

## PD-01 — Capability dependency-graph model
> **⚠ SUPERSEDED by PD-08 (package model).** Kept for history. The `driver`/`extension` framing,
> `requires`=concrete-modules, and `provides`∈{type,thing-of-type,infra} are replaced by **package** +
> an open **capability** namespace. Gating-by-absence survives (now: no family package provides the
> capability). The kernel→family→blueprint→template→loop layering survives; family/blueprint become
> metapackages.

kernel → family → blueprint → template → loop. **capability** = an open-set type/slot (a contract),
introduced by the kernel *or* by a driver/extension — **not** a `port`. **driver vs extension =
provenance/layer** (driver bundled by a family; extension enabled on a blueprint/loop), not
what-it-provides. A **type-introducing driver** introduces only the slot (requires nothing, ships no
impl); **substrate drivers** provide infra/a constant. **`requires` = concrete modules**;
**`provides` ∈ {new type, thing-of-type, infra}**. **Gating = `requires` + driver ABSENCE** (no
`excludes`/conflicts). Family = a named, curated, pick-first driver-set (coherence/UX, not a distinct
mechanism). Headline: make `tracker` a type like `runtime`/`interface`/`harness`; then github→jira swap =
`{+ identity:jira-cred, + jira (tracker), re-point binding}`. Ref R-03 (15 rounds).

## PD-02 — Self-hosting bootstrap via a built-in system context
> **⚠ SUPERSEDED in part by D-23.** The self-describing system-loop idea survives (D-07), but
> "system context = daemon + event-bus" is **retired**: the kit ships the **loop** (control plane); the
> **driver** (daemon, event-bus, etc.) is a **runtime decision**, not shipped. "Onboarding = bootstrap
> phase pivoting into a digest/shepherding steady state" is reframed as **user mode (P4)** — furnish +
> maintain (§3.6, §3.8).

Loopsmith ships a built-in **system/management context** (the control plane = `factory-core`: daemon +
event-bus) that hosts the **system loop** — bare, immutable, always-present firmware (onboarding +
repair), depending on nothing the user can break. Onboarding is the system context's **bootstrap phase**
— a loop whose first item is "create the first context" — which **pivots** into steady-state (digest /
shepherding). Hypershift-style *hosted*, not standalone-OCP *throwaway*. "Loops all the way down"
(R-03 Round 8) includes the control plane.

## PD-03 — Persona ⊥ loop ⊥ harness; "Smith" is a movable binding
> **⚠ UPDATED by PD-09/PD-10.** Persona ⊥ loop ⊥ harness still holds, but Smith is now **kit machinery**
> (the consultant agent that *runs* the packaging skills), not a product-only persona — and he spans a
> build/furnish/maintain repertoire.

**Smith is a persona + a pointer** ("the home loop designation"), not a fixed loop. Three layers:
**system loop** (firmware, always exists, not customizable) · **home loop** (the loop currently
designated as the PA — optional, customizable, user-configured) · **persona** (Smith — portable skills/
knowledge/prompt, *bound to* a loop at runtime). Onboarding re-points Smith from the system loop to a
real user loop (the pivot). **Emergency mode** rebinds the system persona onto a bare loop + a **core
harness**; the only hard dependency is *a running LLM*. (Full recovery likely post-MVP; architecture must
not preclude it — the floor-depends-on-nothing rule.)

## PD-04 — Steady-state Smith hosted in personal context, *member of* others
The cross-context digest works by **membership**, not by elevation: steady-state Smith is hosted in the
user's **personal context** and is a *member of* the others (e.g. professional). **Q-05 reconciliation:**
the CoS/Smith actor is "a *member of*" the professional context, not "hosted in" it. The system context
is reserved for the firmware floor only.

## PD-05 — Port is the sole access mechanism; membership is orthogonal
**Port = HOW you access** (the mechanism; carries authority + disclosure dials) — always, for any
cross-boundary data. **Membership = WHETHER a connected context is internal/external**, which *tunes the
port's disclosure* (member-grade vs projected) — not a second access path. (= Q-08: "local vs external =
membership + dials, not a different noun.") **Connected systems are modeled as ports on steps, NOT as
nested subcontexts** in the loop diagram — no `my-context → GitHub subcontext` explosion.

## PD-06 — A context = the merge of same-"color" fragments (realizes D-iv)
A **source** holds a **context fragment**. A **context** is not a place; it is a **color/identity**,
materialized as the **merge of all same-color fragments across sources** (additive light: same-color
fragments fuse into one contiguous spot; different-color stays distinct/external). `context fragment`
refines `context` — *not* a 6th kernel noun; the home source is the anchor fragment (Q-05 1:1).
This is the concrete mechanism of **D-iv** (Q-02): a datum's context is fixed by membership, not by access
— you can hold a port to a different-color fragment and it still won't merge.

## PD-07 — Membership is DECLARATIVE (not computed)
A fragment's "color" is **written on it at connect/config time** (the user *declares* "this source is
professional"), trusted **by fiat** — there is **no validating predicate/resolver**; a mislabel is user
error. (Lamp metaphor: you write the color on the lamp when you place it; same-color lamps project as one
spot; you *can* mislabel.) **Access is an independent port authority** (rw/ro/none): a declared-member
source with no access is **permitted but inert** (UX may flag it). Membership ⟂ access — neither implies
the other. → requirements [COMP-13, COMP-14](../requirements/comp.md); supersedes any "computed
membership resolver" idea (there is none).

## PD-08 — Everything is a package (supersedes PD-01)
One unit: the **package** (RPM/dnf lineage). A package declares **`requires: [capabilities]`** +
**`provides: [capabilities]`** over one **open capability namespace** — nothing else. Capabilities: the
well-known swappable *slots* (`tracker`/`source`/`runtime`/`harness`/`interface`/`identity`/`planning`)
**plus** templates, agent templates, skills, infra — an **open set**. `provides` is the indirection that
yields **alternatives + swap (CAT-04) + gating-by-absence**; without it a loop hardcodes a package and
**D-i dies**. RPM patterns inherited: **implicit self-provide** (`github` package provides `github`);
**multi/virtual provides at specificity** (`provides: tracker, github-tracker` → a dependent can
`require: tracker` *or* `github-tracker`); abstract (many providers) vs concrete (one) is **emergent, not
declared**. **family / blueprint = metapackages** (curated package sets: substrate / on-top). **Gating-by-
absence** = the family includes no package providing the capability (base bundles no daemon package →
no `runtime` → `k8s` unsatisfiable). **"kernel module" = a role/placement** (a family-included package
providing a *substrate* capability; OS-driver analogy) vs **"app"** (blueprint-pulled integration) —
decided by *which metapackage includes it*, not intrinsic; coarse packages can straddle. **Granularity** =
RPM subpackaging: model permits all-in-one / mix-and-match / metapackage-of-subpackages; **principle: start
coarse, slice finer as reuse demands** — deliberately NOT frozen. **Resolver** = dnf-style
`requires`-closure. Retires `driver`/`extension`/`capability-module`/`module`-as-unit.

## PD-09 — Smith is kit machinery; the single irreducible seed (updates PD-03)
Smith = the **consultant/architect agent that runs the packaging skills** (he *does*
discover→recommend→co-design). **Kit machinery, not a product-only persona.** The **single irreducible
bootstrap seed** — everything else is produced by running the kit. Ships **credentialed on the structural
model** (kernel + grammar + contracts) and **apprenticing on agentic best practices**; grows the craft by
**authoring skills** (skill-packages) with the human — a *modest, off-the-shelf* self-learning ability
("which skill to improve" + "how to author a skill," scaffolded by concrete **skill-building skills**; cf.
Hermes, vanilla Claude Code), **not** autonomous self-rewriting. **Re-packaged as a conforming agent
template**; the developer blueprint ships **Smith + a bootstrap/onboarding loop OOTB**, so at n-0 the same
Smith onboards the end user.

## PD-10 — Smith's repertoire: build / furnish / maintain (build re-entrant)
One agent, one skill set, a repertoire of **sibling loops** (the contractor analogy):
- **build** — author families/blueprints/templates/packages (heavy authoring; empty catalogue);
- **furnish** — onboard: fit a built blueprint to a user's day-to-day (reuse-heavy; Q-14/Q-15);
- **maintain** — troubleshoot a running setup (shepherding/escalation; Q-13).

**Build is re-entrant** from furnish/maintain: hit a missing piece → drop into author → produce it properly
(no hand-hack) → return. Onboarding is **NOT** "a packaging run one level down" — it's a sibling loop
unified by the agent + the skills.

## PD-11 — Generators not instances; empty catalogue; the flywheel
The kit **ships generators, never instances** (no one-offs): contracts + grammar (package *shapes*) +
packaging skills + bootstrap-Smith. **All** concrete artifacts (families, blueprints, agent/loop templates,
packages) are **produced by running the kit**, first during **run #1 (Smith packaging BotMinter)**.
**Catalogue = a repo; starts empty;** grows per run. **Flywheel:** authoring front-loaded, decays as the
catalogue matures. The "seed catalogue" idea dissolves. The two MVP runs are the **start of the curve**:
run #2 (`simple-assistant`) **reuses** run #1 (`developer`), authoring only the **localized delta**
(files-checklist tracker + base/no-daemon substrate) — a small localized delta **strengthens the D-i proof**
(the abstraction is real iff the difference between two real setups collapses to a few packages).

**Meta-level stack (ostree analogy):** n-3 **kit** (≈ rpm-ostree tooling) → n-2 **factory family** (≈ a
compose base) → n-1 **developer blueprint** (≈ Fedora Silverblue) → n-0 **the running setup** (≈ the booted
OS). Run #1 builds *down* the stack.

## PD-12 — Packaging = discover → recommend → co-design → realize → verify (+ learn)
Not transcription. The skill **elicits** the user's needs (an *intent*, not just an existing setup — "what
do you do?"), **recommends** an opinionated best-fit architecture (proposing grammar the user never named,
applying best practices — e.g. arriving at the factory family unprompted), and where the catalogue doesn't
fit, **co-designs** the authored grammar with the user (refining requirements — UX drill-downs — and
**evaluating existing assets** like BotMinter for **port-as-is / tweak / rebuild**). Maps the dropped
consultancy metaphor's discovery→fit&recommendation→build→provision→verify onto
**represent→synthesize→author→realize→verify**. Reuse-first throughout.

## PD-13 — Agent template ⊥ loop template, composed by a binding (fit-check)
A BotMinter "role" **dissolves** into three orthogonal pieces:
- **agent template** — *who*: persona + skills + subagents + **supported harness** (e.g. Ahmed-the-
  engineer). Instantiated → an **actor**.
- **loop template** — *process*: steps · events · gates, agnostic of who runs it (the `ralph.yml`-
  equivalent). Instantiated → a **loop**.
- **binding** — instance-time: **hire** an actor into a context + **assign** it to a loop, with an
  **equip/train fit-check** (is the actor equipped for the loop's steps; if not, **train** by adding skills
  where closable, else surface a **touchpoint**).

Both templates are first-class **capabilities a package provides**; **harness is declared by the agent
template**, concrete harness resolved at bind. At n-0: "add agent" → pick an agent template; "add loop" →
pick a loop template.

## PD-14 — Learning mode = method for MVP (autonomous self-improvement = post-MVP)
Self-learning is **mundane**, not research (Hermes; vanilla Claude Code authors skills; scaffolded by
concrete **skill-building skills**). **MVP:** learning mode is the **method** by which Smith + the human
co-develop & validate the spec via **test runs** (the R-03 loop, continued) — Smith authors/improves skills
**under human guidance**, captured. Deliverable = the **crystallized kit** + an upgraded **`learn` loop**
(propose → human-validate → capture). This **softens** the earlier "friction record-only in MVP" (Smith
*does* act on learning by authoring skills, human-guided). **Fully autonomous self-rewriting Smith =
post-MVP North Star** (the loops-as-product vision).

## PD-15 — Smith's knowledge = contracts + grammar + methodology (derived; gated on locking the house spec)
"What does Smith know?" is **not** a separate artifact to invent — it's the kit's **contracts** (the house
spec) + **grammar** + **packaging methodology**, viewed as a knowledge base. **Sequencing law:** can't
specify the builder's knowledge until the **house spec is locked & agreed**. The **structural** spec is
mostly locked (kernel nouns; capability set; port/membership/home-source rules; the package model) and is
**what Smith ships with**; the **agentic best practices** (craft) are **what learning mode grows**. Smith's
explicit knowledge manifest = **derived, deferred** until the house spec is locked.
**House-spec OPEN items (the design's real work):** the **per-capability-type contracts** (the three faces
for each of `tracker`/`source`/`runtime`/`harness`/`interface`/`identity`/`planning` — "the room designs");
the **agent-template / loop-template / package** shapes; the **binding fit-check** semantics.

## PD-16 — Packaging infrastructure: pixi / rattler / resolvo (conda ecosystem)

After evaluating seven candidates (rpm-rs, archlinux/alpm, uv/PubGrub, Cargo, Nix, Homebrew, pixi/rattler),
the kit adopts **pixi** as its packaging infrastructure. Pixi is built on **rattler** (Rust engine) and
**resolvo** (CDCL SAT solver from prefix.dev).

**Why pixi wins:** most complete stack — package format, resolver with virtual capability support, lock files,
feature/environment composition, task runner, cross-platform, embeddable Rust crates. No fork needed.

**Key finding:** resolvo's solver accepts **arbitrary** `GenericVirtualPackage` names (`__tracker`,
`__harness`, `__identity`, etc.) — the capability namespace is open. Pixi's manifest also accepts arbitrary
`__name = "version"` entries via a raw escape hatch. The typed `VirtualPackage` enum in pixi is only for
host-detection ("does this machine have CUDA?"), irrelevant to our use case. Resolution is fully open.

**Mapping to kit concepts:**
- **Package** = conda package (`.conda` archive) on a custom channel
- **Capability** = conda virtual package (`__tracker`, `__harness`, `__identity`, etc.)
- **Blueprint** = pixi environment (composition of features)
- **Family** = pixi feature (named dependency set)
- **Resolver** = resolvo SAT solver (already proven with RPM provides/requires via resolvo-rpm PoC)
- **Lock file** = `pixi.lock`
- **Post-install wiring** = conda package scripts
- **Task runner** = pixi tasks (graph-based, templated)
- **Smith authors packages** via rattler-build / pixi-build
- **Channel** = directory + `repodata.json`, or prefix.dev hosted

**Alternatives and why they lost:**
- **rpm-rs:** solid format library, but no resolver/DB/lock file/repo creation — build everything yourself
- **archlinux/alpm:** pure Rust types + DB (read+write), but no resolver, newer, less mature
- **uv (PubGrub):** wrong model — Python packaging has no virtual provides/requires
- **Cargo:** no native capability resolution; `package.metadata` escape hatch exists but you'd build a
  custom resolver layer on top
- **Nix:** strong model (module system = genuine provides/requires), but not embeddable (requires daemon),
  steep learning curve
- **Homebrew:** no virtual provides, Ruby-based, explicitly moved away from flexibility (hardcoded defaults)

**Stack:** `pixi` (CLI + manifest + environments) → `rattler` (engine: types, solve, install, channels,
lock) → `resolvo` (SAT solver, generic). Three layers, not three peers.

## PD-17 — Smith talks to packaging only through `smith-agent` (pixi is wrapped)

Smith never invokes `pixi` (or rattler) directly. It calls a single kit-owned binary, **`smith-agent`**,
which is the stable interface for every packaging operation (resolve, install, wire, verify, query, …).
Behind that boundary `smith-agent` uses pixi **programmatically where pixi exposes a usable Rust entry
point, and shells out to the `pixi` binary for the MVP where it does not** — and the backing implementation
can evolve (shell-out → programmatic → fork) without changing the surface Smith sees.

**Why:** decouples Smith from pixi's CLI surface; gives one auditable, swappable seam; lets us adopt pixi
incrementally without betting the agent's contract on pixi's API stability. **Established pattern:** this is
exactly how BotMinter already works — the agent calls the `github-project` skill, never `gh` directly
(`smith-agent : pixi :: github-project : gh`).

**Requirements impact:** none — this is HOW, not WHAT. `features.md` stays capability-level and
pixi-invisible ("the kit can resolve / install / verify / wire"). "Does pixi expose a programmatic entry
point for operation X" becomes a per-operation design question for `smith-agent`, not a feature-scoping
question.

## PD-18 — Structure is machine-verifiable; prose is not (a natural constraint, not a defect)

A hat (and the loop graph) has two faces: a **machine-verifiable structure** — the workflow graph
(triggers, `publishes`, branches, status transitions, connectivity), whose internal form is **BPMN** —
and an **unverifiable prose body**, the instructions an **LLM** executes. `publishes: [x, y]` is a
**declared contract, not a guarantee**: if a hat's prose never actually emits `x`/`y` under the right
conditions, the model is **structurally valid but behaviorally wrong**. **Structural correctness never
implies behavioral correctness.** This is a natural constraint of any system whose work is done by prose +
a non-deterministic LLM — not a defect of the design.

Two defenses bound it:
- **Generate the wiring-prose from the graph** — the editor injects the "On X → emit `x`" lines from the
  drawn arrows, so the prose cannot drift from `publishes` at authoring time; the human writes only the
  judgment content. **Future feature, NOT MVP.** (Closes *authoring* drift; does not touch runtime fidelity.)
- **Runtime contract enforcement** — the **ralph-orchestrator** event bus already treats `publishes` as the
  allowed event set **by default**; undeclared or missing emissions surface at runtime. **This is the MVP
  mechanism — we already have it, inherited from the harness.**

**Residual (irreducible):** even with both, an LLM can still judge or emit wrong at runtime — caught only by
**behavioral tests + runtime reconciliation** (exactly why BotMinter runs the zero-trust shepherd).

**Status alteration is the weakest case (refinement).** Whether a step changes an item's status is a
**declared property** (package-author-set, carried on hats and on skills; a driver MAY also provide built-in
status-altering steps — post-MVP, since the two MVP drivers are ralph-orch + claude). A status-altering
outcome expands to a BPMN branch carrying the `status-out`; a non-altering one does not. The **declaration is
structural and verifiable**; the **behavior is not** — and it is **weaker than `publishes`**: events flow
through the ralph-orch **bus** (which whitelists the declared set), but status is mutated **tracker-side, off
ralph-orch's path**, so **neither** a missing transition **nor** a move to an *undeclared* status is caught at
runtime. **Future closures to explore (post-MVP):** a board-scanner that **re-reads status after each status-altering
step** (detection parity, not prevention), or a **harness/driver hook** intercepting the tracker mutation —
e.g. a post-step / pre-tool hook asserting the landing status is in the declared out-set (**prevention**).
**Deferred** — **MVP accepts this as residual** (caught by behavioral tests + the zero-trust shepherd).

**Design/requirements impact:** conformance (PKG-38/07) verifies the **graph**; **behavior** is verified by
testing/observation, not statically. The design MUST state this boundary as a first-class limitation. No new
MVP feature — defense 1 is post-MVP, defense 2 is inherited from the harness.

## PD-19 — One model, two faces, one runtime: BPMN 2.0 is the internal representation (firms up PD-18)

A loop is **not a new invention** — it is a **business process**, and the BPMN/workflow-net literature
(van der Aalst soundness, choreography vs orchestration, call activity = reuse, correlation = item routing)
has already solved its modelling. So Loopsmith stops inventing a graph language and **adopts BPMN 2.0 as
the internal canonical representation**. The model has **two faces and one runtime**:

- **Face A — the agentic-friendly format** (loops · hats · actors · trackers · sources). The **high-level
  language** and the only thing users ever see — the editor ("LoopStudio") and the package grammar
  (PD-08) speak Face A. Users **never see BPMN.**
- **Face B — BPMN 2.0** — the **internal canonical representation**: the on-disk/interchange data format,
  the validation substrate, and — decisively — the **library-reuse substrate** (`bpmn-js` as the editor
  core; standard sound-net validators). BPMN is **the substrate we build on, not a foreign format we
  transpile *from*.** Operator's framing: *"we're building Java + bytecode; BPMN is the assembly + kernel."*
- **Runtime — Ralph.** `ralph.yml` + `PROMPT.md` + `.claude/` are **generated from the model** and executed
  by **Ralph** (one backend today; the model is backend-neutral). **We do NOT execute BPMN** — the earlier
  "BPMN runs on any engine" framing is **retired**. BPMN = source/IR; the transpiler = the bridge;
  Ralph = runtime. Generation is an **assembler** (deterministic field-mapping + two prose transformers),
  hence **low-risk** — validated by `loopsim.py` (one hat → valid `ralph.yml`, key-set identical to the
  real `pr_gate`; all checks pass).

**The reduction (Face A ⇄ Face B is reversible and total):**
- **`hat ⇄ task + (virtual) gateway`.** The UI shows hats joined by arrows; underneath each hat is a
  standard BPMN **task** followed by an **exclusive gateway**.
- **Virtual gateway** — the user **never draws a gateway**. Its branches **= the events in the hat's
  `publishes`**; wiring `hat₁ → hat₂` **auto-adds the branch**. The reduction layer is **thin** precisely
  because BPMN already carries the structure — we reuse `bpmn-js` and hide the gateways.
- **switch / case / default = Exclusive Gateway + default flow.** (Inclusive GW = OR/multi-match;
  Event-based GW = race on incoming events.) Everything the loop model needs is **stock BPMN 2.0**,
  verified against the OMG spec + the official example models.

**Three faces of a behavioral unit** (firms up PD-18's "two faces"):
- **prose block** — pure **content**, the instructions an LLM runs; carries **no wiring**.
- **BPMN representation** — the **wiring**: entry condition, triggers, branches, `publishes`, status-out.
- **agentic representation** — the runtime **hat** = an **LLM session + the prose as its prompt**.
The **only coupling** between prose and graph is the **outcome vocabulary**: the prose declares named
outcomes (PASS/FAIL, or the events it emits) and the gateway branches on those names. This is exactly the
seam PD-18 calls unverifiable — `publishes` names the branch labels structurally; whether the prose
*reaches* them is behavioral.

**Transformers** (how prose is assembled at build time): a **transformer** takes prose A + prose B → a
coherent prose C, and is **deterministic OR an LLM session + a transformation skill** — the **same
`automatic | agent` performer duality** that exists at runtime (PD-13). **Smith is the agentic
transformer** (PD-09): the build pipeline runs on the same agentic substrate it builds. In the transpile,
exactly **two** steps are transformers (render graph-wiring → prose; compose header + wiring-prose +
content-prose → the hat's `instructions`); **all other rules are field-mapping.**

**Templates → instances** (this is PD-13 + the catalogue, restated through the two faces): a hat's prose
originates in a **hat/loop template or skill**; the user **instantiates** it, edits the instance, and may
**export a new template** or **push the delta back**. This **is** BotMinter's **role `ralph.yml`
(template) vs member `ralph.yml` (instance)** — **catalogue = templates; a setup = instances.**

**Scope of "all packages are BPMN documents"** (bounds PD-08): mostly true — loop→process, actor→resource,
skill→referenced task, source→interface+operations — **but two payloads are NOT processes**:
capability-type **contracts** (a schema, not a flow) and **pure-knowledge skills** (prose, not an
activity); these ride **inside** packages as payload. The **resolver / Smith / `smith-agent` binary are
machinery, not packages.**

**Design/requirements impact:** Face A is the user-facing grammar (PD-08) and the editor; Face B (BPMN) is
an **internal representation decision** — surfaces in design.md §3 (architecture: the model + the
transpiler/assembler), §5 (data models: the Face-A schema + its BPMN mapping), and as a **D-NN + ADR**
("adopt BPMN 2.0 as the internal representation; do not execute it"). The hat⇄task+gateway reduction and
the `publishes`→branch identity are **first-class design facts**, not implementation detail. Two design
threads to resolve in §3: **(B)** the **status→event dispatch** owner (board-scanner maps board status →
which hat fires) and **(C)** **abstract-tracker binding** (ops reference a tracker *capability*, bound to a
concrete provider at transpile — CONF-03 / PD-08 `requires: tracker`).

## PD-20 — MVP: swimlane layout is a hardcoded per-driver convention (dynamic views deferred)

The pool/lane/sublane question has **no default semantics in BPMN** — it is a modeling choice for a purpose
(per the OMG "by Example" doc: dept/team/worker interaction may be modeled as separate pools *or* as one pool
with lanes; "it is totally up to the purpose of the model"). Rather than build a configurable view engine for
MVP, **each loop driver ships one fixed swimlane convention**, bound to the driver (the family just selects
the driver). MVP ships two:

| Driver (family) | Tracker | Lane | Sublane |
|---|---|---|---|
| **ralph-orchestrator** (factory / BotMinter) | GitHub / Jira | per **issue type** | per **hat prefix** (role) |
| **simple** (simple family) | files | per **file** | — |

**Load-bearing facts that stay** (cheap, true, and they keep the door open):
- **A loop = one driver-config artifact** — `ralph.yml` for ralph (one `event_loop` ⇒ one loop; many hats
  inside), a **skill file** for the claude driver (one agent may carry **many** skill-loops). Two artifacts =
  two loops, unconditionally. The boundary is the **artifact**, not the driver process (ralph 1:1, claude
  1:N). A view may render two artifacts side-by-side but **cannot fuse** them.
- **Lanes/sublanes carry no execution semantics** (BPMN: work is assigned via the step's performer, not the
  lane) — which is exactly why hardcoding them per driver is safe and reversible later.

**Deferred to post-MVP** (explicitly, to stop circling): configurable/dynamic views; the general
loop/context **boundary rules** (different context → different pool/port=message-flow; different item +
lifecycle; independent enactment; reuse/ownership; cohesion); the **hub-and-spoke** framing (BPMN as the
canonical *semantic/interchange* hub vs. driver-native artifacts as the persisted spokes) and cross-driver
portability/round-trip. None of these block MVP; the two hardcoded conventions above are sufficient.

**Design/requirements impact:** design-only for MVP (a per-driver layout convention, not a feature). The
boundary fact (loop = driver-config artifact) belongs in §3/§5; the deferred items are a §8 "post-MVP" note,
not requirements.

---

## PD-21 — Normative contracts are *content*, authored at bootstrap via the contract machinery; the design pins the machinery and ships two reference contracts

**The split.** A conformance contract is **content the kit produces**, not part of the kit's machinery.
This is the same structure/craft line BST-01/BST-03 already draw: Smith ships credentialed on the
**structural** model (kernel nouns, package grammar, *contract format*) and apprentices on **craft** — and
the catalogue's actual contracts are part of that craft, authored during the apprenticeship (BST-02: "the
output of the run IS the first catalogue content"). Contracts ride the same lifecycle as packages
([[package-is-delivery-unit]]): authored, versioned, published, discovered — never baked into the design.

**What this changes.** The §4.4 seven-row per-type table (`tracker · source · identity · harness · runtime ·
interface · planning`) was pre-specifying seven contracts at shallow depth. That is the wrong altitude and
the wrong *time*: those contracts get authored **incrementally as MVP stories** (some Smith-assisted at
bootstrap) and land as **catalogue content** — they ship, just not designed in this doc. The design's job is to pin the **machinery** that makes a contract *expressible, storable,
referenceable, and verifiable* — not to enumerate the contracts.

**What the design pins now (the machinery surface):**
1. **Contract format / convention** — the knative split: a normative **contract** layer (RFC 2119 prose over
   the three faces — required configuration · required data relationships · required observable behavior,
   PKG-04) + a **test-plan** layer (block-quote the normative clause → runnable check → machine-readable
   `[Output]` envelope). Each normative clause carries a stable **assertion ID** the test-plan and the
   verification report both reference. Authored to `conformance/` (sibling of `requirements/`).
2. **Verification harness** (§4.4) — runs a candidate setup against the base contract + every applicable
   per-type contract; pass/fail per contract with the **specific violated assertion IDs** (PKG-31); gates
   readiness (PKG-39).
3. The capability namespace (§3.3), package grammar (§3.5/§5.1), resolver, kernel + BPMN representation
   (§3.10/§5.7), catalogue/introspection (PKG-32), and conventions/tooling — already substantially in §3/§5.

**Reference contracts (worked examples, authored now to prove the format):**
- **`conformance/contracts/base-setup.md`** — the base contract (PKG-38); the one non-optional,
  non-capability contract; structural assertions over the five nouns. Must exist at MVP regardless.
- **`conformance/contracts/work-tracker.md`** — the per-capability exemplar (PKG-04). Chosen because it is
  the spine of a loop and exercises the **general→specific** match (`tracker` → `github-projects-tracker` /
  `files-checklist-tracker`, PKG-16, [[tracker-not-files-is-the-axis]]) and **tracker-agnosticism** (CONF-03)
  — the most concrete variant story in the kit.

The other five capability types are **named** in §4.4 as a non-normative map of *which types MVP recognizes*
(so the resolver and the namespace are grounded), with their full contracts **deferred to bootstrap**,
authored through this same machinery.

**Design/requirements impact:** §3.4 keeps the apparatus (base + per-type, three faces); §4.4 demotes the
seven-row table from "specification" to "recognized-types map" and points at the machinery + the two
references; §5.2 keeps the contract-as-assertion-set data model and adds the assertion-ID + test-plan
envelope. A §8 decision records the defer-to-bootstrap stance. No requirements change — `features.md` already
holds the product features; contracts are content those features produce.

---

## PD-22 — The three machinery threads resolved (resolver · catalogue/introspection · boundary/meta-skill/friction)

Resolution of the open machinery after PD-21. **Forest:** MVP is a thin kit-owned layer over **pixi/resolvo**
(packaging — resolve/install/lock/channels) + **ralph/BotMinter** (loop runtime) + **Smith** (skills);
net-new kit code is just the `smith`/`smith-agent` boundary, the conformance engine, the capability↔pixi
mapping + placement encoding, and Smith's skills.

- **Resolver (D-20):** PKG-41 placement = `requires` over the disjunction of present `harness:<agent>`,
  resolved in-solver; per-step actor↔harness = bind-time fit-check (§4.5). `resolve` emits sufficient-set +
  lock + rationale (mechanical, default-pick never block); Smith presents with judgment; **no surfacing
  policy** — what to surface is craft (BST-03), not a build-time rule. [[feedback_dont-design-agent-judgment-as-policy]]
- **Catalogue/introspection (D-21):** repertoire is runtime catalogue data (PKG-32), not baked; a type's
  contract ships in its package; publish consent-gated, public/private = channel. **Flywheel thinned:**
  cross-run friction memory/citation (LEARN-01/02) + cross-author aggregation (PKG-13) → post-MVP; MVP keeps
  introspection + publish.
- **Boundary/meta-skill/friction (D-22):** `smith-agent` verb surface
  (search·browse·query·resolve·install·wire·verify·publish·remove·upgrade·rollback·inventory) mirroring
  github-project; meta-skill = judgment layer loaded around deterministic hooks; friction disposition state
  machine.

**Design impact:** §4.3 (resolver split), §4.9 (boundary surface), §4.10 (meta-skill), §4.11
(catalogue/publish), §5.6 (friction states), §8 D-20/21/22. No requirements change.

---

**Requirement traceability (this session):** PD-05 → COMP-03; PD-06/PD-07 → COMP-07, COMP-08, and the
membership-not-access face of D-iv. PD-01 stays design-only (model, not requirements). **New (package-model
session):** PD-08 supersedes PD-01 (design-only model); PD-09/10/12/13 → packaging-skill + Smith behavior
(PKG-*, and a new binding/fit-check behavior); PD-11 → catalogue/flywheel + a CAT reframe (CAT-03/06 shift
from "kit ships" to "produced by runs"); PD-14 → the `learn` loop (revises friction-record-only); PD-15 → a
deferred D-NN. Carry all into design.md §3.4 (grammar = packages), §4.5 (packaging skills), §5.7/5.8
(package + template shapes), §8 (D-NN + ADRs). **New (BPMN-model session):** PD-18 → PKG-38/07 scope
(graph-only) + the verifiability-boundary limitation; PD-19 → §3 (model + transpiler/assembler), §5
(Face-A schema + BPMN mapping), §8 (D-NN + ADR: "adopt BPMN 2.0 as internal representation, do not
execute it"). Threads B (status→event dispatch) and C (abstract-tracker binding) resolve inside §3.
PD-20 → §3/§5 (loop = driver-config artifact = the boundary; per-driver hardcoded swimlane conventions) +
a §8 post-MVP note (dynamic views, boundary rules, hub/spoke all deferred).
