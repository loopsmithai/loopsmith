# Session Handoff — Loopsmith loop model / BPMN architecture

*Checkpoint dump so the conversation can be compacted and resumed cleanly. Date: 2026-06-22.
Epic #178 "Loopsmith MVP", design phase. Nothing in this session is committed.*

---

## 0. Where we are / the job

We are in the **design phase of epic #178**. The thing we had to nail before writing
`design.md` is: **what is a "loop," precisely, and how is it represented?** This session
converged on the answer below and validated it against a real system (BotMinter) and a working
transpile simulation. The core design bet is **de-risked**. The next move is to **write it down**
and resume `design.md §3`.

---

## 1. THE CONVERGED MODEL  (at-risk — lives only in chat + here; not yet a PD)

### One model, two faces, plus a runtime
- **Face A — agentic-friendly format** (loops, hats, actors, trackers). The UI layer ("LoopStudio").
  The high-level language.
- **Face B — BPMN 2.0** = our **internal canonical representation**. Adopted as a **standard** for
  data-format, validation, interchange, and **library reuse** (esp. `bpmn-js` as the editor).
  BPMN is **the substrate we build on, not a foreign format we transpile *from*.** Analogy the
  operator used: *"we're building Java + bytecode; BPMN is the assembly + kernel."*
- **Runtime targets** — `ralph.yml` + `PROMPT.md` + `.claude/` are **generated from the model** and
  run by **Ralph** (one backend; others possible later). We do **not** execute BPMN.

### The reduction (Face A ⇄ Face B is reversible)
- **`hat  ⇄  task + (virtual) gateway`**. The UI shows hats + arrows; underneath it's standard BPMN.
- **Virtual gateway**: the user never draws a gateway. Its **branches = the events in `publishes`**;
  connecting `hat1 → hat2` **auto-adds a branch**. Underneath it's a real `task + exclusiveGateway`.
  This is exactly why we can **reuse `bpmn-js` and add a thin reduction layer** on top.
- BPMN **switch/case/default = Exclusive Gateway + a default flow.** (Inclusive GW = OR/multi-match;
  Event-based GW = race on incoming events.)

### Three faces of a behavioral unit (how prose relates to structure)
- **prose block** = the content / source of truth (instructions an LLM runs).
- **BPMN representation** = the **wiring** (entry status, triggers, branches, `publishes`, status-out).
- **agentic representation** = the runtime **hat** (an LLM session + the prose as prompt).
- The prose is **pure content**; all wiring lives in the **graph**. The **only coupling** is the
  **outcome vocabulary** — the prose declares named outcomes (e.g. PASS/FAIL or the events it emits),
  and the gateway branches on those names.

### Transformers (how prose is assembled)
- A **transformer** takes prose A + prose B → coherent prose C. It is **deterministic OR an LLM session
  + a transformation skill**. (Same `automatic | agent` performer duality as runtime; **Smith is the
  agentic transformer.** The build pipeline uses the same model it builds.)
- In the transpile, two steps are transformers — render **graph wiring → prose**, then **compose**
  header + wiring-prose + content-prose → the hat's `instructions`. Everything else is field-mapping.

### Templates → instances
- A hat's prose comes from a **hat template / loop template / skill**. The user **instantiates**, edits
  the instance, and can **export it as a new template** or **push changes back to the template**.
- = BotMinter's **role `ralph.yml` (template) vs member `ralph.yml` (instance)**. Catalogue = templates;
  a setup = instances.

---

## 2. VALIDATION DONE this session

- **`loopsim.py`** transpiles one hat ("record merge") → a **valid `ralph.yml`**; **all 10 checks pass**.
  The transpile is a **9-rule deterministic ruleset** (7 field-maps + 2 transformers). The generated
  hat's key-set (`name, description, triggers, publishes, instructions`) **== the real `pr_gate`'s**.
- **BotMinter expressibility:** the sentinel = **one loop, two activities** (`pr_gate` @ `snt:gate:merge`,
  `pr_triage`) over the GitHub tracker, claude actor, matrix interface. The engineer↔sentinel relationship
  is **steps of a workflow assigned to different actors**, *not* separate choreographed loops. ~85%
  expressible as-is.
- **BPMN constructs we rely on are real BPMN 2.0** (verified vs the OMG spec PDF + the official example
  models): exclusive gateway + default (switch/case), call activity (reuse), conditional/timer/message
  start events (triggers), `ioMapping`, correlation key (item routing), `extensionElements` (our
  `loopsmith:` namespace). The toy `bpmn-engine-ts` clone is **incomplete (stubbed)** — do **not** cite it
  as proof of executability; irrelevant since we don't execute BPMN.

---

## 3. DECISIONS / NOTES settled

- **PD-18 logged** in `design-notes.md`: *structure is machine-verifiable, prose is not.* `publishes` is a
  **declared contract, not a guarantee** — structural validity never implies behavioral correctness; a
  **natural constraint** of prose+LLM systems. Defense 1 (generate wiring-prose from the graph) = **future,
  not MVP**. Defense 2 (runtime enforcement of `publishes`) = **already have it, by default in
  ralph-orchestrator**. Residual caught by behavioral tests + the BotMinter zero-trust shepherd.
- **"All packages are BPMN documents"** — mostly true (loop→process, actor→resource, skill→referenced task,
  source→interface+operations), **but breaks for**: capability-type **contracts** (a schema, not a flow) and
  **pure-knowledge skills** (prose, not an activity) — these are package **payloads**, not BPMN processes.
  The **resolver / Smith / `smith-agent` binary are machinery, not packages.**
- **Earlier "BPMN executes / runs on any engine" framing was wrong** and is retired: BPMN = source/IR;
  Ralph = runtime; transpiler = the bridge. Generation from the model is an **assembler**, low-risk.

---

## 4. OPEN THREADS (pick up here)

- **A. Capture the §1 model as a PD** (the main decision — currently only in chat + this file). **#1 priority.**
- **B. status→event dispatch gap** — who owns "board status X → which hat fires" (the board-scanner's
  dispatch map). Surfaced twice in the sim; unresolved.
- **C. Tracker-agnosticism binding** — a loop claims `trackerAgnostic` but ops hardcode `github:`. Needs:
  ops reference an **abstract tracker capability**, bound to a concrete provider **at transpile**. (SET-03.)
- **D. The deliverable — `design.md §3 Architecture`** — writable now on the BPMN footing. The actual job.
- **E. Housekeeping — nothing committed** this session (see §5). Commit only on operator approval.

**Recommended order:** A → D (B and C resolve inside writing §3).

Minor, already understood (no action needed unless surfacing):
- **Re-record bug:** a hat with `status-out: None` never leaves its trigger status → re-fires forever.
  Fix is a *source-graph* change (add a `status-out`). `status-out` is load-bearing for progress, not decoration.

---

## 5. ARTIFACTS created this session (uncommitted)

| File | What |
|---|---|
| `exercise-loopstudio-to-bpmn.md` | Full sentinel storyboard, 3 layers (UI / brick model / BPMN). |
| `loopsim.py` | Working transpile simulation — animates 9 rules, emits + validates `ralph.yml`. Run: `python3 loopsim.py` (interactive) or `--auto`. |
| `generated-ralph.yml` | Output of the sim (the validated single-hat `ralph.yml`). |
| `design-notes.md` | **PD-18 appended.** |
| `session-handoff.md` | This file. |

Research corpus in `research/`:
- `formal-13-12-09.pdf` — BPMN 2.0.2 normative spec (532 pp).
- `dtc-10-06-02.pdf` — "BPMN 2.0 by Example" (47 pp, **read in full** — Incident Mgmt ch.6 is the
  one-process-many-views demo; Correlation ch.11 = item routing; §7 = call activity / reuse).
- `dtc-10-06-03.zip` + `2010-06-03/` — 25 official `.bpmn` example models (no prose).

Reference clones in `/tmp` (not in repo): `camunda-modeler` (Electron app), `bpmn-engine-ts`
(Rust BPMN engine — **incomplete/stubbed**, reference only).

---

## 6. GROUND TRUTH (what the model must reproduce)

- Sentinel: `/home/sandbox-test/.botminter/workspaces/may-team/sentinel-heimdel/ralph.yml`
- Engineer: `/home/sandbox-test/.botminter/workspaces/may-team/engineer-bob/ralph.yml`
- Status lifecycle / conventions: `team/PROCESS.md`
- Requirements: `team/specs/loopsmith/requirements/features.md` (70 features) + `requirements-manifest.md`
- Prior decisions: `design-notes.md` (PD-01…PD-18)

---

## 7. RESUME PROMPT (paste after compaction)

> We finished de-risking the Loopsmith loop model. Read `session-handoff.md` for full state.
> Next: **thread A** — write the converged BPMN two-faces architecture (§1 of the handoff) as a new
> `PD-19` in `design-notes.md`, then resume `design.md §3 Architecture`. Resolve threads B (status→event
> dispatch) and C (abstract tracker binding) inside §3. Nothing is committed; do not commit without my OK.
