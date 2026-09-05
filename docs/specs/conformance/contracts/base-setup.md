# Base Setup Conformance Contract

> **Reference contract** demonstrating the Loopsmith contract format ([conformance/README.md](../README.md)).
> This is the one **always-applicable** contract (AWV-06): every setup must satisfy it regardless of which
> capabilities are present. Authored now as a worked example; the structural floor it encodes is real and
> ships at MVP.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",
"MAY", and "OPTIONAL" are to be interpreted as described in
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

---

## Part A — Normative contract

### Abstract

This contract defines the **structural floor** a conforming Loopsmith setup must satisfy to be marked *ready*.
It is written over the five kernel nouns — **context · loop · item · actor · port** — and is verified before a
setup goes live, in addition to one per-capability-type contract for each capability the setup uses.

Requirements are written from the point of view of someone **assembling a setup**. A setup that violates any
`MUST` here is non-conforming and MUST NOT be marked ready, even if every per-type contract passes.

**Out of scope.** This contract does not define what any *capability* must do — that is the per-type contracts
(e.g. [work-tracker](work-tracker.md)). It also does not assert **behavioral fidelity of step prose**: that a
step which declares it alters status *actually* performs that transition at runtime is not statically
verifiable and is a known residual (PD-18) — this contract checks the *structural* half only.

### Required configuration

- `«BASE-CFG-01»` Every loop in the setup MUST have a **workflow definition** — a set of statuses, the allowed
  transitions between them, and any gates (SET-04). A loop without a workflow is non-conforming.
- `«BASE-CFG-02»` Every context MUST be configured with a **home source** (CNTXT-04) — a context with none is
  non-conforming. (Cardinality and uniqueness are `«BASE-REL-03»`.)
- `«BASE-CFG-03»` The setup MUST route operator↔actor communication through a **configured interface**
  (SET-07). At least one interface MUST be configured if any loop contains a human touchpoint or HITL gate
  (SET-06).

### Required data relationships

- `«BASE-REL-01»` A conforming setup MUST contain **at least one context** and MAY contain more.
- `«BASE-REL-02»` A conforming setup MUST contain **at least one loop** and MAY contain more (SET-02).
- `«BASE-REL-03»` Each context MUST have **exactly one** home source, and two contexts MUST NOT share a home
  source (CNTXT-04) — home sources are setup-unique.
- `«BASE-REL-04»` Every source MUST be declared into **exactly one** context (CNTXT-03); sources declared into
  the same context are treated as one unified context. A source's context membership MUST be independent of
  its access (CNTXT-01).
- `«BASE-REL-05»` Every item MUST carry **exactly one current status**, and that status MUST be a member of
  its loop's workflow status set (SET-04). An item whose status is not in the workflow is non-conforming.
- `«BASE-REL-06»` Every loop **step** MUST have an **assigned actor** — `human`, `agent`, or `automated`
  (SET-08, [D-06](../../../plans/e1-loopsmith-mvp/design.md#8-design-decisions-living)). A step with no actor is
  non-conforming.
- `«BASE-REL-07»` Each actor assigned into the setup MUST belong to **at least one context** (SET-09); a
  single actor MAY belong to more than one.
- `«BASE-REL-08»` For every step the setup declares as **status-altering**, that step MUST declare at least
  one outcome carrying a `status-out`, and **every** declared `status-out` MUST be a valid transition in the
  owning loop's workflow (the structural half of the re-record guard). A status-out that is not a declared
  transition is non-conforming.
- `«BASE-REL-09»` A conforming setup MUST contain **at least one actor**, and **at least one** of those actors
  MUST be an **agent** (SET-01); it MAY contain more actors of any kind. A setup with no actor, or with no
  agent actor, is non-conforming.

### Required observable behavior

- `«BASE-BHVR-01»` Every declared source MUST be **connected and live** before the setup is marked ready: its
  credentials MUST validate and a liveness check MUST confirm it is reachable (AWV-05). A declared-but-
  unreachable source blocks readiness.
- `«BASE-BHVR-02»` Every loop in the setup MUST be **enactable**: its loop driver MUST be able to load the
  loop's generated runtime artifact and start it without error (§3.10). A loop that fails to load is
  non-conforming.

---

## Part B — Conformance test-plan

### [Test] At least one context and one loop — `«BASE-REL-01»` `«BASE-REL-02»`

> > A conforming setup MUST contain at least one context and MAY contain more.
> > A conforming setup MUST contain at least one loop and MAY contain more.

**[Pre]** An assembled, wired candidate setup, prior to readiness.

**[Test]** Query the setup graph for contexts and loops; assert each count ≥ 1.

**[Output]**
```json
{ "assertion": "BASE-REL-01", "contract": "base-setup", "result": "pass|fail",
  "detail": "context count = <n>", "evidence": "<context ids>" }
{ "assertion": "BASE-REL-02", "contract": "base-setup", "result": "pass|fail",
  "detail": "loop count = <n>", "evidence": "<loop ids>" }
```
**Tested in:** _TBD (implementation)._

### [Test] Every loop has a workflow — `«BASE-CFG-01»`

> > Every loop in the setup MUST have a workflow definition — statuses, transitions, and any gates.

**[Pre]** The candidate setup graph.

**[Test]** For each loop, assert a workflow is defined with a non-empty status set and a transition graph over
those statuses.

**[Output]**
```json
{ "assertion": "BASE-CFG-01", "contract": "base-setup", "result": "pass|fail",
  "detail": "loop <id> workflow present (statuses=<n>, transitions=<n>)? <bool>", "evidence": "<workflow>" }
```
**Tested in:** _TBD._

### [Test] Home source — designated, singular, unique — `«BASE-CFG-02»` `«BASE-REL-03»`

> > Every context MUST be configured with a home source.
> > Each context MUST have exactly one home source, and two contexts MUST NOT share a home source.

**[Pre]** The candidate setup graph.

**[Test]** For each context, assert a home source is designated and that it has exactly one. Then assert the
multiset of home sources across all contexts has no duplicates.

**[Output]**
```json
{ "assertion": "BASE-CFG-02", "contract": "base-setup", "result": "pass|fail",
  "detail": "<context> home source designated? <bool>", "evidence": "<source id>" }
{ "assertion": "BASE-REL-03", "contract": "base-setup", "result": "pass|fail",
  "detail": "<context> home-source count = <n>; duplicate home sources: <none | list>",
  "evidence": "<map context→home-source>" }
```
**Tested in:** _TBD._

### [Test] Source ↔ context membership and access independence — `«BASE-REL-04»`

> > Every source MUST be declared into exactly one context. A source's context membership MUST be independent
> > of its access.

**[Pre]** The candidate setup graph.

**[Test]** For each source, assert it is declared into exactly one context. Then assert membership and access
are separately represented — changing one in the model leaves the other unchanged (no implied coupling).

**[Output]**
```json
{ "assertion": "BASE-REL-04", "contract": "base-setup", "result": "pass|fail",
  "detail": "source <id> in exactly one context? <bool>; membership⟂access? <bool>",
  "evidence": "<source→context; membership vs access fields>" }
```
**Tested in:** _TBD._

### [Test] Item status validity — `«BASE-REL-05»`

> > Every item MUST carry exactly one current status, and that status MUST be a member of its loop's workflow
> > status set.

**[Pre]** The candidate setup with at least one tracked item per loop.

**[Test]** For each item, resolve its owning loop's workflow status set; assert the item's current status is a
single value drawn from that set.

**[Output]**
```json
{ "assertion": "BASE-REL-05", "contract": "base-setup", "result": "pass|fail",
  "detail": "item <id> status '<s>' ∈ workflow? <bool>", "evidence": "<workflow status set>" }
```
**Tested in:** _TBD._

### [Test] Every step has an actor — `«BASE-REL-06»`

> > Every loop step MUST have an assigned actor — human, agent, or automated.

**[Pre]** The candidate setup graph.

**[Test]** Enumerate every step of every loop; assert each has an `actor-ref` resolving to an actor of kind
`human | agent | automated`.

**[Output]**
```json
{ "assertion": "BASE-REL-06", "contract": "base-setup", "result": "pass|fail",
  "detail": "unassigned steps: <none | list>", "evidence": "<map step→actor.kind>" }
```
**Tested in:** _TBD._

### [Test] Actors belong to a context — `«BASE-REL-07»`

> > Each actor assigned into the setup MUST belong to at least one context.

**[Pre]** The candidate setup graph.

**[Test]** For each actor, assert it is a member of ≥1 context.

**[Output]**
```json
{ "assertion": "BASE-REL-07", "contract": "base-setup", "result": "pass|fail",
  "detail": "actors with no context: <none | list>", "evidence": "<map actor→contexts>" }
```
**Tested in:** _TBD._

### [Test] At least one agent — `«BASE-REL-09»`

> > A conforming setup MUST contain at least one actor, and at least one of those actors MUST be an agent.

**[Pre]** The candidate setup graph.

**[Test]** Assert actor count ≥ 1 and that ≥1 actor has kind `agent`.

**[Output]**
```json
{ "assertion": "BASE-REL-09", "contract": "base-setup", "result": "pass|fail",
  "detail": "actor count = <n>; agent count = <n>", "evidence": "<map actor→kind>" }
```
**Tested in:** _TBD._

### [Test] Status-out well-formedness — `«BASE-REL-08»`

> > For every step the setup declares as status-altering, that step MUST declare at least one outcome carrying
> > a status-out, and every declared status-out MUST be a valid transition in the owning loop's workflow.

**[Pre]** The candidate setup graph with workflow definitions.

**[Test]** For each step flagged `status-altering`: assert ≥1 outcome has a non-empty `status-out`; then for
every `status-out` across the step's outcomes, assert `(status-in → status-out)` is an edge in the workflow's
transition graph.

**[Output]**
```json
{ "assertion": "BASE-REL-08", "contract": "base-setup", "result": "pass|fail",
  "detail": "step <id>: declared status-out '<x>' is a workflow transition? <bool>",
  "evidence": "<workflow transition edges>" }
```
**Tested in:** _TBD._

### [Test] Interface configured for human touchpoints — `«BASE-CFG-03»`

> > The setup MUST route operator↔actor communication through a configured interface; at least one interface
> > MUST be configured if any loop contains a human touchpoint or HITL gate.

**[Pre]** The candidate setup graph.

**[Test]** If any loop has a human-actor step or a HITL gate, assert ≥1 interface is configured and that
operator↔actor communication is bound to it.

**[Output]**
```json
{ "assertion": "BASE-CFG-03", "contract": "base-setup", "result": "pass|fail|skip",
  "detail": "human touchpoint/gate present? <bool>; interface configured? <bool>",
  "evidence": "<interfaces; touchpoints/gates>" }
```
**Tested in:** _TBD._

### [Test] Sources are live — `«BASE-BHVR-01»`

> > Every declared source MUST be connected and live before the setup is marked ready: its credentials MUST
> > validate and a liveness check MUST confirm it is reachable.

**[Pre]** The candidate setup with sources configured (connection values supplied, credentials bound).

**[Test]** For each declared source, run its connector's liveness check (validate credentials → confirm
reachable). A source that fails either step fails the assertion.

**[Output]**
```json
{ "assertion": "BASE-BHVR-01", "contract": "base-setup", "result": "pass|fail|skip",
  "detail": "source <id> live? <bool> (creds=<ok|bad>, reachable=<ok|no>)",
  "evidence": "<connector liveness response>" }
```
**Tested in:** _TBD._

### [Test] Loops are enactable — `«BASE-BHVR-02»`

> > Every loop MUST be enactable: its loop driver MUST be able to load the loop's generated runtime artifact
> > and start it without error.

**[Pre]** The candidate setup, with each loop transpiled to its driver-native runtime artifact (§3.10).

**[Test]** For each loop, invoke the loop driver's load/dry-start on the generated artifact; assert it loads
and the driver reports the loop startable without error.

**[Output]**
```json
{ "assertion": "BASE-BHVR-02", "contract": "base-setup", "result": "pass|fail",
  "detail": "loop <id> loaded by <driver>? <bool>", "evidence": "<driver load diagnostics>" }
```
**Tested in:** _TBD._

---

### Traceability

| Assertion | Face | Derives from (features.md) |
|---|---|---|
| BASE-CFG-01 | CFG | SET-04 |
| BASE-CFG-02 | CFG | CNTXT-04 (home source designated) |
| BASE-CFG-03 | CFG | SET-06, SET-07 |
| BASE-REL-01 | REL | (kernel: context) |
| BASE-REL-02 | REL | SET-02 |
| BASE-REL-03 | REL | CNTXT-04 (cardinality + uniqueness) |
| BASE-REL-04 | REL | CNTXT-01, CNTXT-03 |
| BASE-REL-05 | REL | SET-04 |
| BASE-REL-06 | REL | SET-08 |
| BASE-REL-07 | REL | SET-09 |
| BASE-REL-08 | REL | SET-04 (status-out well-formedness; PD-18) |
| BASE-REL-09 | REL | SET-01 |
| BASE-BHVR-01 | BHVR | AWV-05 |
| BASE-BHVR-02 | BHVR | SET-03, §3.10 |
