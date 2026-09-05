# `tracker` Capability Conformance Contract

> **Reference contract** demonstrating a **per-capability-type** contract (PKG-04), the exemplar for the
> Loopsmith contract format ([conformance/README.md](../README.md)). It is the spine of a loop and the kit's
> clearest swap story: any conforming `tracker` is interchangeable under a loop (SET-03), and a requirement
> for the general `tracker` is satisfied by any more specific variant (DIS-03).

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",
"MAY", and "OPTIONAL" are to be interpreted as described in
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

---

## Part A — Normative contract

### Abstract

This contract defines what any provider of the **`tracker`** capability must satisfy. A tracker holds **items**
(units of work), each at one **status** within a **workflow**, and lets a loop list, read, mutate, and
transition them. The contract is written from the point of view of a **tracker provider implementer** (e.g. a
`github-projects-tracker` or a `files-checklist-tracker`).

The contract is provider-agnostic on purpose: it is verified **identically** against every provider, and that
identical behavior is the swap thesis (SET-03) — a loop redefining nothing works the same whichever tracker
backs it.

**Specificity.** `tracker` is the general type. A provider MAY declare a more specific variant (e.g.
`github-projects-tracker is-a tracker`); per DIS-03 a requirement for `tracker` is then satisfiable by that
variant. A variant provider MUST satisfy **this** contract in full and MAY add behavior beyond it; the extra
behavior MUST NOT break any assertion here.

**Out of scope.** Backend-specific configuration (a GitHub project number, a directory path) is the
provider's concern, not this contract's — the contract constrains the *capability surface*, not the backend.
Workflow *content* (which statuses a particular loop defines) is per-setup configuration (SET-04), not fixed
here.

### Required configuration

- `«TR-CFG-01»` A tracker provider MUST accept the **connection configuration** required to bind to its
  backing system, and MUST validate it at connect time (the home-source liveness check, AWV-05, runs through
  the base contract).
- `«TR-CFG-02»` A tracker provider MUST accept a **workflow definition** — a status set, an allowed-transition
  graph over that set, and any gates (SET-04) — and MUST operate against it. It MUST NOT hard-code a status
  set of its own.

### Required data relationships

- `«TR-REL-01»` Every item the tracker holds MUST carry **exactly one current status**, drawn from the
  configured workflow's status set. The tracker MUST NOT represent an item as having zero or multiple current
  statuses. (This is the provider-surface counterpart of the setup-graph check `«BASE-REL-05»`.)
- `«TR-REL-02»` The set of transitions the tracker permits MUST be a **subset** of the configured workflow's
  transition graph. The tracker MUST NOT permit a transition the workflow does not declare.
- `«TR-REL-03»` Every item MUST be associated with its **owning loop** and MAY be associated with an
  **assigned actor**; these associations MUST be readable through the tracker surface.

### Required observable behavior

- `«TR-BHVR-01»` A tracker MUST support **listing/querying items by status** — given a status, return exactly
  the items currently at that status.
- `«TR-BHVR-02»` A tracker MUST support **create, read, and update** of an item (its fields and associations).
- `«TR-BHVR-03»` A tracker MUST **transition** an item from its current status to a target status **when that
  transition is declared** in the workflow, and the item's current status MUST reflect the target afterward.
- `«TR-BHVR-04»` A tracker MUST **reject an illegal transition** — a target not reachable from the current
  status in the workflow graph — and MUST NOT mutate the item's status when it does (SET-04). The rejection
  MUST be observable to the caller.
- `«TR-BHVR-05»` When a transition crosses a **gate** (SET-06), the tracker MUST surface the gate and MUST NOT
  complete the transition until the gate is satisfied.
- `«TR-BHVR-06»` A tracker's observable behavior under `«TR-BHVR-01..05»` MUST be **identical regardless of
  backing system** (SET-03): the same workflow and the same operation sequence MUST yield the same item
  lifecycle across any two conforming providers. This is the swap invariant.

---

## Part B — Conformance test-plan

> The `[Test]` steps are written against the **abstract tracker operations** (`list-by-status`, `create`,
> `read`, `update`, `transition`), not any backend's CLI. The **same** test-plan runs unchanged against every
> provider — that is how `«TR-BHVR-06»` is verified.

### [Test] Connection accepted and validated — `«TR-CFG-01»`

> > A tracker provider MUST accept the connection configuration required to bind to its backing system, and
> > MUST validate it at connect time.

**[Pre]** A valid connection config and a deliberately-bad variant.

**[Test]** `connect` with valid config → assert success. `connect` with bad config → assert it is rejected at
connect time. (Home-source *liveness* is asserted by the base contract, `«BASE-BHVR-01»`; this entry covers
connect-time **validation**.)

**[Output]**
```json
{ "assertion": "TR-CFG-01", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "valid config accepted, bad config rejected at connect? <bool>",
  "evidence": "<connect results>" }
```
**Tested in:** _TBD._

### [Test] Workflow is honored, not hard-coded — `«TR-CFG-02»` `«TR-REL-01»`

> > A tracker provider MUST accept a workflow definition … and MUST NOT hard-code a status set of its own.
> > Every item the tracker holds MUST carry exactly one current status, drawn from the configured workflow's
> > status set.

**[Pre]** Connect the provider; configure it with a test workflow `W = { statuses: [todo, doing, done],
transitions: [todo→doing, doing→done] }`. Create one item.

**[Test]** Read the item; assert its status is a single value ∈ `W.statuses`. Then reconfigure with a
different workflow `W' = { backlog, active, shipped }` and assert the provider operates against `W'` (an item
created under `W'` carries a status ∈ `W'.statuses`), proving the status set is configured, not built in.

**[Output]**
```json
{ "assertion": "TR-CFG-02", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "operated against configured workflow? <bool>",
  "evidence": "<status set observed under W and W'>" }
{ "assertion": "TR-REL-01", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "item carries exactly one status ∈ workflow? <bool>",
  "evidence": "<item status>" }
```
**Tested in:** _TBD._

### [Test] List/query by status — `«TR-BHVR-01»`

> > A tracker MUST support listing/querying items by status — given a status, return exactly the items
> > currently at that status.

**[Pre]** Workflow `W`; create items `i1, i2` at `todo` and `i3` at `doing`.

**[Test]** `list-by-status(todo)` → assert the result set is exactly `{i1, i2}`. `list-by-status(doing)` →
exactly `{i3}`. `list-by-status(done)` → empty.

**[Output]**
```json
{ "assertion": "TR-BHVR-01", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "list-by-status returns exact membership? <bool>",
  "evidence": "<status→item-ids returned>" }
```
**Tested in:** _TBD._

### [Test] Create / read / update — `«TR-BHVR-02»` `«TR-REL-03»`

> > A tracker MUST support create, read, and update of an item.
> > Every item MUST be associated with its owning loop and MAY be associated with an assigned actor; these
> > associations MUST be readable through the tracker surface.

**[Pre]** Workflow `W`; an owning loop `L`.

**[Test]** `create(item, owning-loop=L)` → read back: assert fields and `owning-loop = L`. `update(item,
assigned-actor=A)` → read back: assert `assigned-actor = A`.

**[Output]**
```json
{ "assertion": "TR-BHVR-02", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "create/read/update round-trips? <bool>", "evidence": "<item before/after>" }
{ "assertion": "TR-REL-03", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "owning-loop and assigned-actor readable? <bool>",
  "evidence": "<associations>" }
```
**Tested in:** _TBD._

### [Test] Legal transition succeeds — `«TR-BHVR-03»`

> > A tracker MUST transition an item … when that transition is declared in the workflow, and the item's
> > current status MUST reflect the target afterward.

**[Pre]** Workflow `W`; item `i1` at `todo`.

**[Test]** `transition(i1, doing)` (declared: `todo→doing`) → assert it succeeds; read `i1` → assert status =
`doing`.

**[Output]**
```json
{ "assertion": "TR-BHVR-03", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "legal transition applied, status now '<s>'", "evidence": "<i1 status>" }
```
**Tested in:** _TBD._

### [Test] Illegal transition rejected, status unchanged — `«TR-BHVR-04»` `«TR-REL-02»`

> > A tracker MUST reject an illegal transition … and MUST NOT mutate the item's status when it does. The
> > rejection MUST be observable to the caller.
> > The set of transitions the tracker permits MUST be a subset of the configured workflow's transition graph.

**[Pre]** Workflow `W`; item `i1` at `todo` (note `todo→done` is **not** declared).

**[Test]** `transition(i1, done)` → assert it is **rejected** with an observable error; read `i1` → assert
status is still `todo` (unchanged).

**[Output]**
```json
{ "assertion": "TR-BHVR-04", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "illegal transition rejected and status unchanged? <bool>",
  "evidence": "<error returned; i1 status after>" }
{ "assertion": "TR-REL-02", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "permitted transitions ⊆ workflow graph? <bool>",
  "evidence": "<attempted vs declared transitions>" }
```
**Tested in:** _TBD._

### [Test] Gates are surfaced, not bypassed — `«TR-BHVR-05»`

> > When a transition crosses a gate, the tracker MUST surface the gate and MUST NOT complete the transition
> > until the gate is satisfied.

**[Pre]** Workflow `W` with a gate on `doing→done`; item `i1` at `doing`.

**[Test]** `transition(i1, done)` → assert the transition is **held** and the gate is surfaced (not silently
applied, not silently dropped); read `i1` → status still `doing`. Satisfy the gate, retry → assert it now
completes.

**[Output]**
```json
{ "assertion": "TR-BHVR-05", "contract": "work-tracker", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "gate surfaced and transition held until satisfied? <bool>",
  "evidence": "<gate surfaced; status before/after satisfy>" }
```
**Tested in:** _TBD._

### [Test] Swap invariant — identical behavior across providers — `«TR-BHVR-06»`

> > A tracker's observable behavior … MUST be identical regardless of backing system: the same workflow and
> > the same operation sequence MUST yield the same item lifecycle across any two conforming providers.

**[Pre]** Two conforming providers `P1`, `P2` (e.g. `github-projects-tracker`, `files-checklist-tracker`),
each configured with the **same** workflow `W`.

**[Test]** Run the **identical** operation sequence from `«TR-BHVR-01..05»` against `P1` and against `P2`.
Assert that for each step, the observable outcome (returned membership, resulting status, rejection,
gate-surfacing) is **equal** across `P1` and `P2`. Any divergence fails the assertion and names the diverging
step.

**[Output]**
```json
{ "assertion": "TR-BHVR-06", "contract": "work-tracker", "applies_to": "P1=<provider@ver>, P2=<provider@ver>",
  "result": "pass|fail", "detail": "behavior identical across providers? <bool>; divergences: <none|list>",
  "evidence": "<per-step outcome P1 vs P2>" }
```
**Tested in:** _TBD._

---

### Traceability

| Assertion | Face | Derives from (features.md) |
|---|---|---|
| TR-CFG-01 | CFG | AWV-05 (connect/validate) |
| TR-CFG-02 | CFG | SET-04 (workflow), SET-03 (no hard-coded statuses) |
| TR-REL-01 | REL | SET-04 |
| TR-REL-02 | REL | SET-04 |
| TR-REL-03 | REL | SET-08 (assigned actor), SET-02 (owning loop) |
| TR-BHVR-01..03 | BHVR | PKG-04 surface; SET-02 |
| TR-BHVR-04 | BHVR | SET-04 (illegal-transition rejection) |
| TR-BHVR-05 | BHVR | SET-06 (gates) |
| TR-BHVR-06 | BHVR | SET-03 (the swap thesis) |
