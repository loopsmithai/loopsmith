# Exercise — LoopStudio → BPMN, three layers deep

*A thesis + storyboard. We build the exact equivalent of BotMinter's **sentinel** agent
(GitHub + Ralph orchestrator + skills + `ralph.yml`) by dragging bricks in LoopStudio,
and watch the intermediate model and the final BPMN materialize underneath.*

---

## Thesis

1. **BPMN 2.0 is our internal model.** Every Loopsmith **package is a BPMN `definitions`
   document** under the hood — a `process` for a loop, a `resource` for an actor, an
   `interface`+`operation`s for a source, a `globalTask` for a skill, `itemDefinition`s for
   data. Our specifics (`performer`, tracker refs, capability `provides`/`requires`,
   coding-agent binding) ride in a `loopsmith:` **extension namespace** on that document.

2. **Users never see BPMN.** They compose in agentic-SDLC vocabulary — **loops, actors,
   trackers, gates, skills, drivers** — in a skinned web designer (**LoopStudio**, a
   re-themed `bpmn-js`). If they happen to know BPMN they'll recognize it; if they don't,
   they never need to.

3. **Plugins (packages) contribute brick shapes.** A fresh blueprint exposes only the
   **core bricks**. Installing a plugin **extends the palette** with the bricks that plugin
   provides — and each brick knows how it expands into BPMN.

4. **The named bricks are aggregates over BPMN primitives.** A "Human Gate" brick is one
   thing to the user and ~4 BPMN elements underneath. That macro library *is* Loopsmith's
   opinion — the thing that makes it Loopsmith and not a raw BPMN editor.

5. **Machinery is not a package.** Smith, the resolver, and the Ralph engine *produce and
   run* these documents. They deliver nothing into the catalogue, so they aren't packages
   and aren't BPMN.

6. **Validation:** composing the right bricks must reproduce a real system. This exercise's
   acceptance test is that the finished canvas is behaviorally equal to the sentinel
   `ralph.yml`.

---

## The three layers (read every scene top-to-bottom)

| Layer | What it is | Who sees it |
|---|---|---|
| **① LoopStudio (the console)** | The drag-drop canvas the user works in. Each element is annotated with the **plugin** it came from. | the user |
| **② Brick model (transient IR)** | The intermediate agentic-domain Lego model. Composed only from **brick shapes in the current legend** (which grows as plugins install). | nobody (internal) |
| **③ BPMN (final)** | Standard BPMN 2.0 `definitions` + `loopsmith:` extensions. Runs on any BPMN engine. | nobody (the artifact) |

> ②→③ is a pure expansion (each brick is a BPMN macro). ①→② is a pure re-skin (each
> canvas element is one brick).

---

## Legend — brick shapes for Layer ②

Bricks marked **[core]** exist on a bare blueprint. The rest appear only after their plugin
is installed (Scene 1).

| Brick | Glyph | Provided by | Expands to in BPMN (Layer ③) |
|---|---|---|---|
| Loop | `⊚` | **[core]** | `<process>` |
| Step | `▭` | **[core]** | `<task>` (neutral) + `loopsmith:performer` |
| Transition | `→` | **[core]** | `<sequenceFlow>` (+ `<conditionExpression>`) |
| Gate (decision) | `◇` | **[core]** | `<exclusiveGateway>` + branch flows |
| Human Gate | `▤` | **[core]** | `<userTask>` + gateway + `<messageFlow>` (macro) |
| Actor | `☻` | **[core]** | `<resource>` + `loopsmith:performer` binding |
| Trigger | `⊙` | **[core]** | `<startEvent>` (typed) |
| End | `⊗` | **[core]** | `<endEvent>` |
| GitHub Tracker | `⛁` | **[github]** | `<dataStore>` + `<interface>` + correlation key |
| GitHub Op | `⚙` | **[github]** | `<serviceTask>` → `<operation>` (find/merge/approve/comment/setStatus) |
| Issue (item) | `▤?` | **[github]** | `<itemDefinition>` (work item) |
| Ralph Driver | `⟳` | **[ralph]** | `<startEvent kind=conditional>` (poll) + dispatch-by-status (engine) |
| Claude Actor | `☻c` | **[claude]** | `<resource>` + `loopsmith:codingAgent="claude"` |
| Skill | `✦` | **[skills]** | `<globalTask>` / referenced task content |
| Matrix Interface | `✉` | **[matrix]** | human `<participant>` + `<messageFlow>` channel |

---

# STORYBOARD

Target: **the sentinel agent** — one loop over the GitHub board with two activities
(`pr_gate` at `snt:gate:merge`, `pr_triage` scanning orphan PRs), driven by Ralph, voiced
through Matrix, run by Claude.

---

## Scene 0 — Fresh bare blueprint

**① LoopStudio**
```
┌─ LoopStudio ───────────────────────────────── blueprint: "blank" ─┐
│  palette:  [⊚ Loop] [▭ Step] [→] [◇ Gate] [▤ Human Gate]          │
│            [☻ Actor] [⊙ Trigger] [⊗ End]                          │
│                                                                    │
│   canvas:                                                          │
│            ( empty )                                               │
└────────────────────────────────────────────────────────────────────┘
```

**② Brick model**
```
∅   (nothing composed yet)
```
*Legend available:* core bricks only — `⊚ ▭ → ◇ ▤ ☻ ⊙ ⊗`.

**③ BPMN**
```xml
<definitions xmlns="...BPMN/20100524/MODEL"
             xmlns:loopsmith="...loopsmith/ext">
  <!-- empty -->
</definitions>
```

---

## Scene 1 — Install plugins (the palette grows)

**① LoopStudio**
```
┌─ LoopStudio · Plugins ─────────────────────────────────────────────┐
│  installing…                                                       │
│   ✔ github            provides: work-tracker source, gh operations │
│   ✔ ralph-orchestrator provides: persistent poll driver           │
│   ✔ coding-agent/claude provides: claude actor binding            │
│   ✔ sentinel-skills   provides: merge-gate, github-project,        │
│                                  status-workflow skills            │
│   ✔ matrix-interface  provides: operator comms channel            │
│                                                                    │
│  palette (NEW shapes appended):                                    │
│    [⛁ GitHub Tracker] [⚙ GitHub Op] [⟳ Ralph Driver]              │
│    [☻c Claude Actor]  [✦ Skill]     [✉ Matrix Interface]          │
└────────────────────────────────────────────────────────────────────┘
```

**② Brick model**
```
∅   (still nothing composed — but the legend just expanded)
```
*Legend available NOW:* core **+** `⛁ ⚙ ⟳ ☻c ✦ ✉`
(each tagged with its plugin: `⛁⚙`→github, `⟳`→ralph, `☻c`→claude, `✦`→skills, `✉`→matrix).

**③ BPMN** — installing registers each plugin's reusable BPMN fragments into a referencable
import scope (nothing is in *our* process yet):
```xml
<definitions ...>
  <import name="github"  loopsmith:provides="tracker:work-tracker, op:gh.*"/>
  <import name="claude"  loopsmith:provides="coding-agent:claude"/>
  <import name="skills"  loopsmith:provides="skill:merge-gate, skill:github-project,
                                              skill:status-workflow"/>
  <import name="matrix"  loopsmith:provides="interface:matrix"/>
  <!-- ralph is engine/driver config, referenced later -->
</definitions>
```

---

## Scene 2 — Drop the Loop, attach the GitHub Tracker

**① LoopStudio**
```
┌─ LoopStudio ───────────────────────────────────────────────────────┐
│   ┌───────────────────────────── ⊚ "sentinel-merge-loop" ──────┐   │
│   │                                                             │   │
│   │   ⛁ GitHub Tracker  ◄── attached            [github]        │   │
│   │                                                             │   │
│   └─────────────────────────────────────────────────[core]─────┘   │
└────────────────────────────────────────────────────────────────────┘
```

**② Brick model**
```
⊚ Loop "sentinel-merge-loop"        [core]
 └─ runs-over → ⛁ GitHub Tracker     [github]   (items = Issues; key = issue#)
```

**③ BPMN**
```xml
<process id="sentinel-merge-loop" isExecutable="true">
  <extensionElements>
    <loopsmith:tracker ref="github:work-tracker" trackerAgnostic="true"/>
  </extensionElements>
</process>
<dataStore id="board" loopsmith:source="github:work-tracker"/>
<itemDefinition id="Issue" loopsmith:kind="work-item"/>
<correlationProperty id="issueNo">          <!-- handoff finds its item -->
  <retrievalExpression messageRef="boardEvent"><messagePath>issue.number</messagePath></retrievalExpression>
</correlationProperty>
```

---

## Scene 3 — Add the `pr_gate` step (actor + skill bound)

**① LoopStudio**
```
┌─ ⊚ sentinel-merge-loop ────────────────────────────────────────────┐
│                                                                    │
│   ⊙ on Issue@snt:gate:merge ─→ ▭ "PR Gate"                         │
│        [ralph]                    actor: ☻c Claude   [claude]       │
│                                   skill: ✦ merge-gate [skills]      │
│                                   uses:  ⚙ find-PR    [github]      │
└────────────────────────────────────────────────────────────────────┘
```

**② Brick model**
```
⊚ Loop
 ├─ ⊙ Trigger  on status = snt:gate:merge        [ralph]
 └─ ▭ Step "PR Gate"                              [core]
      ├─ performer → ☻c Claude Actor              [claude]
      ├─ skill     → ✦ merge-gate                 [skills]
      └─ op        → ⚙ github.findLinkedPR         [github]
```

**③ BPMN**
```xml
<startEvent id="onMergeReady">
  <conditionalEventDefinition loopsmith:status="snt:gate:merge"/>   <!-- ralph poll -->
</startEvent>
<sequenceFlow sourceRef="onMergeReady" targetRef="prGate"/>

<task id="prGate" name="PR Gate">
  <extensionElements>
    <loopsmith:performer kind="agent" resourceRef="claudeActor"/>
    <loopsmith:skill ref="skills:merge-gate"/>
  </extensionElements>
</task>
<serviceTask id="findPR" name="Find linked PR" operationRef="github:findLinkedPR"/>
<sequenceFlow sourceRef="prGate" targetRef="findPR"/>

<resource id="claudeActor" loopsmith:codingAgent="claude"/>
<globalTask id="merge-gate" loopsmith:skill="true"/>
```

---

## Scene 4 — The pass/fail gate and both paths

`pr_gate`: run merge-gate tests → **pass:** approve + merge + `human:po:accept`;
**fail:** request-changes + hand back to `eng:dev:implement`.

**① LoopStudio**
```
┌─ ⊚ sentinel-merge-loop ────────────────────────────────────────────┐
│  …▭ PR Gate ─→ ⚙ run merge-gate tests ─→ ◇ pass? ─────────────┐    │
│                          [github+skills]      │ yes            │    │
│                                               ▼                │    │
│        ⚙ approve ─→ ⚙ merge ─→ ▭ set snt:gate.merged           │    │
│                                  set status → human:po:accept  │    │
│                                                    ⊗ merged    │    │
│                                               │ no             │    │
│                                               ▼                │    │
│        ⚙ request-changes ─→ ▭ set status → eng:dev:implement   │    │
│                                                    ⊗ rejected  │    │
│                                            (handoff → engineer)│    │
└──────────────────────────────────────────────────────[github]─┘    │
```

**② Brick model**
```
▭ PR Gate
 └─ ⚙ github.runMergeGate                          [github+skills]
     └─ ◇ Gate "tests pass?"                        [core]
         ├─ →[yes] ⚙ approve → ⚙ merge → ▭ setStatus(human:po:accept) → ⊗ merged
         └─ →[no]  ⚙ requestChanges → ▭ setStatus(eng:dev:implement)   → ⊗ rejected
                                              ↑ handoff: status in another loop's domain
```

**③ BPMN**
```xml
<serviceTask id="runGate" operationRef="github:checkoutAndTest"
             loopsmith:skill="skills:merge-gate"/>
<exclusiveGateway id="passQ" default="failFlow"/>
<sequenceFlow sourceRef="runGate" targetRef="passQ"/>

<sequenceFlow id="passFlow" sourceRef="passQ" targetRef="approve">
  <conditionExpression>${testsPassed}</conditionExpression></sequenceFlow>
<serviceTask id="approve" operationRef="github:approvePR"/>
<serviceTask id="merge"   operationRef="github:mergePR"/>
<serviceTask id="setAccept" operationRef="github:setStatus"
             loopsmith:status="human:po:accept" loopsmith:skill="skills:status-workflow"/>
<endEvent id="merged"><loopsmith:publish event="snt.gate.merged"/></endEvent>

<sequenceFlow id="failFlow" sourceRef="passQ" targetRef="reqChanges"/>
<serviceTask id="reqChanges" operationRef="github:requestChanges"/>
<serviceTask id="setImpl" operationRef="github:setStatus"
             loopsmith:status="eng:dev:implement"      <!-- writes into engineer's domain -->
             loopsmith:handoff="cross-loop"/>
<endEvent id="rejected"><loopsmith:publish event="snt.gate.rejected"/></endEvent>
```

---

## Scene 5 — Second activity: `pr_triage` (orphan-PR scan)

A separate trigger path: list open PRs on forks → any with no linked board issue → create a
`human:po:triage` issue + comment on the PR.

**① LoopStudio**
```
┌─ ⊚ sentinel-merge-loop ────────────────────────────────────────────┐
│   ⊙ on triage tick ─→ ⚙ list open PRs ─→ ◇ has linked issue?       │
│       [ralph]            [github]            │ no                   │
│                                             ▼                       │
│             ⚙ create issue @human:po:triage ─→ ⚙ comment on PR      │
│                  + project label                  ⊗ triaged         │
│                                             │ yes → ⊗ (skip)        │
└──────────────────────────────────────────────────────[github]──────┘
```

**② Brick model**
```
⊚ Loop
 └─ ⊙ Trigger "triage tick"                        [ralph]
     └─ ⚙ github.listForkPRs                        [github]
         └─ ◇ Gate "linked issue?"                  [core]
             ├─ →[no]  ⚙ github.createIssue(human:po:triage,+project) → ⚙ github.commentPR → ⊗ triaged
             └─ →[yes] ⊗ skip
```

**③ BPMN**
```xml
<startEvent id="triageTick"><timerEventDefinition loopsmith:driver="ralph:poll"/></startEvent>
<serviceTask id="listPRs" operationRef="github:listForkPRs"/>
<exclusiveGateway id="linkedQ" default="orphanFlow"/>
<sequenceFlow id="orphanFlow" sourceRef="linkedQ" targetRef="mkIssue"/>
<serviceTask id="mkIssue" operationRef="github:createIssue"
             loopsmith:status="human:po:triage"/>
<serviceTask id="commentPR" operationRef="github:commentPR"/>
<endEvent id="triaged"><loopsmith:publish event="snt.triage.done"/></endEvent>
```

---

## Scene 6 — Attach the Ralph Driver + the Matrix interface

The loop's **driver** (what wakes it) and the **channel** for any human-facing comms.

**① LoopStudio**
```
┌─ ⊚ sentinel-merge-loop · properties ───────────────────────────────┐
│   driver:    ⟳ Ralph (persistent poll, dispatch-by-status) [ralph]  │
│   interface: ✉ Matrix  (operator: @bmadmin)                [matrix] │
│   guardrails: invariants/*  ·  never merge w/o merge-gate tests     │
└────────────────────────────────────────────────────────────────────┘
```

**② Brick model**
```
⊚ Loop "sentinel-merge-loop"
 ├─ driver    → ⟳ Ralph                            [ralph]
 ├─ interface → ✉ Matrix                            [matrix]
 └─ guardrails: [invariants, merge-gate-required]
```

**③ BPMN**
```xml
<process id="sentinel-merge-loop" ...>
  <extensionElements>
    <loopsmith:driver ref="ralph" mode="persistent-poll" dispatch="by-status"/>
    <loopsmith:interface ref="matrix:operator" id="@bmadmin:localhost"/>
    <loopsmith:guardrail ref="invariants/*"/>
    <loopsmith:guardrail rule="merge requires merge-gate pass"/>
  </extensionElements>
</process>
<participant id="operator" loopsmith:human="true" loopsmith:channel="matrix"/>
```

---

## Scene 7 — Done: the full sentinel, and the round-trip check

**① LoopStudio (final canvas)**
```
┌─ ⊚ sentinel-merge-loop ─── driver:⟳Ralph · ☻c Claude · ✉Matrix ────┐
│                                                                    │
│  ⊙ snt:gate:merge ─▭PR Gate─⚙test─◇pass? ─yes→ ⚙approve→⚙merge→     │
│       [ralph]       [claude] [skills]      └─no→ ⚙req-changes→      │
│                                                  →eng:dev:implement │
│                                                  → human:po:accept  │
│  ⊙ triage tick ─⚙list PRs─◇linked? ─no→ ⚙create triage→⚙comment    │
│       [ralph]    [github]                                          │
└────────────────────────────────────────────────────────────────────┘
```

**② Brick model (complete)**
```
⊚ sentinel-merge-loop  · driver:⟳Ralph · interface:✉Matrix · over:⛁GitHub
 ├─ activity pr_gate:
 │    ⊙snt:gate:merge → ▭PR Gate(☻c, ✦merge-gate) → ⚙test → ◇pass?
 │       ├ yes → ⚙approve → ⚙merge → setStatus:human:po:accept → ⊗merged
 │       └ no  → ⚙request-changes → setStatus:eng:dev:implement → ⊗rejected
 └─ activity pr_triage:
      ⊙triage tick → ⚙listForkPRs → ◇linked? └ no → ⚙createIssue:human:po:triage → ⚙commentPR → ⊗triaged
```

**③ BPMN** — the assembled `<definitions>` from Scenes 2–6 = one runnable document.

### Round-trip check: `ralph.yml` ⇄ this BPMN

| sentinel `ralph.yml` | Layer ② brick | Layer ③ BPMN |
|---|---|---|
| `event_loop` (persistent) | `⟳ Ralph Driver` | `loopsmith:driver mode=persistent-poll` |
| `board-scanner: auto_inject` | (Ralph dispatch) | `conditionalEventDefinition loopsmith:status=…` + correlation |
| `cli.backend: claude` | `☻c Claude Actor` | `<resource loopsmith:codingAgent="claude">` |
| hat `pr_gate` (`snt.gate`) | activity `pr_gate` | startEvent`@snt:gate:merge` → tasks → gateway → ends |
| hat `pr_triage` (`snt.triage`) | activity `pr_triage` | timer startEvent → list/gateway/create/comment |
| skill `merge-gate` | `✦ merge-gate` | `<globalTask>` + `serviceTask github:checkoutAndTest` |
| skill `github-project` | `⚙ GitHub Ops` | `serviceTask operationRef="github:*"` |
| skill `status-workflow` | (on setStatus ops) | `loopsmith:skill="skills:status-workflow"` |
| statuses (`snt:gate:merge`,`human:po:accept`,`eng:dev:implement`) | tracker statuses | `loopsmith:status` on events/flows |
| `RObot.matrix` | `✉ Matrix Interface` | `<participant loopsmith:channel="matrix">` |
| guardrails | loop guardrails | `<loopsmith:guardrail>` |
| publishes `snt.gate.merged/.rejected/.done` | `⊗` end bricks | `<loopsmith:publish event=…>` |

**Acceptance:** the BPMN, run by the engine, scans the board, gates merges exactly as
`pr_gate` does, and triages orphan PRs exactly as `pr_triage` does. The user never typed an
angle bracket — they dragged eight brick types, six of which arrived by installing five
plugins.

---

## What the exercise demonstrates

- **Layer ② is the whole product surface.** The agentic vocabulary (loop/actor/tracker/
  gate/skill/driver/op) is sufficient to express a real agent; BPMN never surfaces.
- **Plugins = packages = palette.** Capability arrives as installable bricks. No github
  plugin → no `⚙` ops → you literally cannot draw a GitHub step. Gating-by-absence, visibly.
- **Every brick is a BPMN macro.** ②→③ is mechanical. The "opinion" is which macros exist.
- **The two known frictions show up exactly where predicted:** actor-kind is a
  `loopsmith:performer` attribute on a neutral `<task>` (not UserTask-vs-ServiceTask), and
  the tracker is an attached `<dataStore>`/source, not a native BPMN element.

## Open questions surfaced (not answered here)

- Is a "loop" one `<process>` with multiple trigger paths (as drawn), or one process per
  activity? (We drew one; sentinel `ralph.yml` has two hats in one member — consistent.)
- The `loopsmith:status` ↔ correlation-key relationship: is "status" a property of the item
  in the tracker, or a token state? (We treated it as item property the driver polls.)
- Cross-loop handoff (`setStatus eng:dev:implement`) writes into another loop's status
  domain — by what rule is that write allowed? (Left as a connection-rule question.)
