# `agent-actor-harness` Capability Conformance Contract

> **Base contract for agent-actor hosting.** A more specific variant of the `harness` capability type
> (design [§4.4](../../178-loopsmith-mvp/design.md#44-the-conformance-engine-and-the-per-type-contracts)):
> `agent-actor-harness is-a harness`. The generic `harness` type answers "what drives a coding agent to
> execute a loop step"; this contract adds what it takes to host a **persistent agent actor** — one that
> carries an identity, keeps memory, and is equipped with skills — and to move that actor's accumulated self
> between conforming harnesses. It is the kit's first subject by way of **Smith** (BST), but it is written for
> *any* agent actor, not Smith specifically.

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",
"MAY", and "OPTIONAL" are to be interpreted as described in
[RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

---

## Part A — Normative contract

### Abstract

This contract defines what any provider of the **`agent-actor-harness`** capability must satisfy. An
agent-actor-harness hosts a single **agent actor**: it boots the actor under a supplied **identity**, equips it
with a supplied set of **skills**, gives it **persistent memory** that survives restart, and can **import** an
actor's portable state and **export** it again without loss. The contract is written from the point of view of
an **agent-actor-harness provider implementer** (e.g. a Claude-Code-backed harness, or a Hermes-backed
harness).

The portable unit moved across harnesses is an **agent-state bundle**: an `identity` (the persona), a set of
typed durable **memory** entries (`self` — facts the actor holds about itself and its work; `principal` —
facts about who it works with), and a **skill set** (`{name, description, version, body}`). The bundle is the
content; how a given harness stores it natively (files, a database) is the provider's concern.

**Specificity.** `agent-actor-harness` is a variant of the general `harness` type. Per
[DIS-03](../../178-loopsmith-mvp/design.md#8-design-decisions-living), a requirement for `harness` is satisfied
by an `agent-actor-harness`. A provider MAY declare a still-more-specific variant
(e.g. `claude-code-harness is-a agent-actor-harness`); that variant MUST satisfy **this** contract in full and
MAY add behavior, but the extra behavior MUST NOT break any assertion here. That identical, provider-agnostic
behavior is the harness swap thesis (`«AAH-BHVR-07»`), the analog of the tracker swap invariant
(`«TR-BHVR-06»`).

**Out of scope.** The concrete backend (which coding agent, which storage engine) is the provider's concern,
not this contract's — the contract constrains the *capability surface*, not the substrate. The *content* of any
particular actor's identity/memory/skills is per-instance bundle data, not fixed here. **Episodic / full
session-history** memory is an extension, not part of this floor — the floor's memory is the typed durable
entries `self` and `principal`. The internal algorithm of self-learning, where present, is the provider's
concern; this contract constrains only that it be opt-in and that its results stay portable.

### Required configuration

- `«AAH-CFG-01»` An agent-actor-harness provider MUST accept an **identity** (the actor's persona) and boot the
  hosted actor under it.
- `«AAH-CFG-02»` It MUST accept a **skill set** and make those skills available to the hosted actor.
- `«AAH-CFG-03»` It MUST accept a set of **typed durable memory entries** (`self`, `principal`) as the actor's
  initial state (the pre-load half of import).
- `«AAH-CFG-04»` It MUST declare the **coding agent(s)** it drives. The only hard runtime dependency it MAY
  require is **a running LLM** (CONF-11, design §3.8); it MUST NOT require anything the actor's substrate cannot
  recover down to.
- `«AAH-CFG-05»` *(OPTIONAL capability.)* An agent-actor-harness **MAY** provide **self-learning** — deriving
  new memory and/or skills from a session. A provider that provides self-learning **MUST** expose it as an
  **opt-in skill** that is **disabled unless explicitly enabled**, and MUST treat it as configurable (the
  operator can turn it on or off). It MUST NOT make self-learning unconditional or unremovable.

### Required data relationships

- `«AAH-REL-01»` The hosted actor MUST be bound to **exactly one** agent-actor-harness at a time (the
  actor ↔ harness binding; design §4.4 face: data relationships). The harness MUST NOT represent the actor as bound to zero or
  multiple harnesses.
- `«AAH-REL-02»` After import, the identity, skills, and memory in the harness's native stores MUST
  **correspond to the imported bundle** — every entry in the bundle is represented in the harness's stores, and
  nothing in the bundle is silently dropped. This is the static counterpart of the round-trip behavior
  (`«AAH-BHVR-06»`).

### Required observable behavior

- `«AAH-BHVR-01»` It MUST **drive a coding agent to perform one act** — execute a step end to end — with no hard
  dependency beyond a running LLM.
- `«AAH-BHVR-02»` The booted actor MUST **reflect the supplied identity** (`«AAH-CFG-01»`): its persona in
  session is the one that was configured, not a built-in default.
- `«AAH-BHVR-03»` The actor MUST be able to **recall imported durable memory** (`self`, `principal`) within a
  session.
- `«AAH-BHVR-04»` The **imported skills MUST be usable** by the actor.
- `«AAH-BHVR-05»` After a **restart**, the actor's identity, memory, and skills MUST **persist** — the actor
  comes back as the same self with the same recall and the same skills.
- `«AAH-BHVR-06»` It MUST **export** the hosted actor's current state into a portable agent-state bundle
  (identity + typed memory + skill set). The export MUST be **lossless** with respect to what was imported
  plus anything the actor accrued — `export(import(B)) ⊇ B`.
- `«AAH-BHVR-07»` **Swap invariant.** A bundle exported from one conforming agent-actor-harness and imported
  into another MUST yield an actor whose **identity, recallable memory, and usable skills are equivalent**. An
  agent actor's accumulated self behaves identically across any two conforming harnesses (the analog of
  `«TR-BHVR-06»`; SET-03 applied to harnesses).
- `«AAH-BHVR-08»` It MUST NOT **derive or persist new memory or skills from a session** unless the self-learning
  skill is enabled (`«AAH-CFG-05»`). When self-learning **is** enabled, any memory or skills it derives MUST be
  captured by export (`«AAH-BHVR-06»`) so that learned state stays portable.

---

## Part B — Conformance test-plan

> The `[Test]` steps are written against the **abstract harness operations** (`import`, `boot`, `act`,
> `restart`, `export`), not any backend's CLI. The **same** test-plan runs unchanged against every provider —
> that is how `«AAH-BHVR-07»` is verified.

### [Test] Identity accepted and reflected — `«AAH-CFG-01»` `«AAH-BHVR-02»`

> > A provider MUST accept an identity and boot the hosted actor under it.
> > The booted actor MUST reflect the supplied identity: its persona in session is the one configured, not a
> > built-in default.

**[Pre]** A bundle whose `identity` declares a distinctive, checkable persona (a named role and a stance the
default persona would not assert).

**[Test]** `import` the bundle; `boot`; ask the actor to state who it is and how it works. Assert the answer
reflects the configured persona, not the harness's default identity.

**[Output]**
```json
{ "assertion": "AAH-CFG-01", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "identity accepted at import? <bool>", "evidence": "<import result>" }
{ "assertion": "AAH-BHVR-02", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "booted persona == configured identity? <bool>",
  "evidence": "<actor self-description>" }
```
**Tested in:** _TBD._

### [Test] Skills accepted and usable — `«AAH-CFG-02»` `«AAH-BHVR-04»`

> > A provider MUST accept a skill set and make those skills available to the hosted actor.
> > The imported skills MUST be usable by the actor.

**[Pre]** A bundle carrying a distinctive skill the default harness would not already have.

**[Test]** `import`; `boot`; drive a task that the imported skill is needed for. Assert the actor invokes the
imported skill and completes the task using it.

**[Output]**
```json
{ "assertion": "AAH-CFG-02", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "skill set accepted at import? <bool>", "evidence": "<skills present>" }
{ "assertion": "AAH-BHVR-04", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "imported skill invoked and used? <bool>", "evidence": "<skill invocation trace>" }
```
**Tested in:** _TBD._

### [Test] Memory accepted and recalled — `«AAH-CFG-03»` `«AAH-BHVR-03»`

> > A provider MUST accept typed durable memory entries (self, principal) as initial state.
> > The actor MUST be able to recall imported durable memory within a session.

**[Pre]** A bundle with a `self` entry and a `principal` entry, each carrying a checkable fact.

**[Test]** `import`; `boot`; ask a question whose answer requires each fact. Assert the actor recalls both the
`self` and the `principal` fact.

**[Output]**
```json
{ "assertion": "AAH-CFG-03", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "typed memory accepted at import? <bool>", "evidence": "<stores after import>" }
{ "assertion": "AAH-BHVR-03", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "self and principal facts recalled? <bool>", "evidence": "<answers>" }
```
**Tested in:** _TBD._

### [Test] Drives a coding agent on a running LLM only — `«AAH-CFG-04»` `«AAH-BHVR-01»`

> > A provider MUST declare the coding agent(s) it drives; the only hard runtime dependency it MAY require is a
> > running LLM.
> > It MUST drive a coding agent to perform one act with no hard dependency beyond a running LLM.

**[Pre]** A booted actor; an environment that provides a running LLM and nothing beyond the provider's declared
dependencies.

**[Test]** Read the provider's declared coding agent(s) and dependencies; assert the only hard dependency is a
running LLM. Drive one act (e.g. read a file in the actor's home and report its content) and assert it
completes.

**[Output]**
```json
{ "assertion": "AAH-CFG-04", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "coding agent declared; hard dep is running LLM only? <bool>",
  "evidence": "<declared agent + deps>" }
{ "assertion": "AAH-BHVR-01", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "one act executed end to end? <bool>", "evidence": "<act trace>" }
```
**Tested in:** _TBD._

### [Test] Actor bound to exactly one harness — `«AAH-REL-01»`

> > The hosted actor MUST be bound to exactly one agent-actor-harness at a time.

**[Pre]** An imported, booted actor on provider `P`.

**[Test]** Inspect the binding in the assembled setup graph. Assert the actor references exactly one harness
(`P`) — not zero, not multiple.

**[Output]**
```json
{ "assertion": "AAH-REL-01", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "actor bound to exactly one harness? <bool>", "evidence": "<binding>" }
```
**Tested in:** _TBD._

### [Test] Imported bundle corresponds to native stores — `«AAH-REL-02»`

> > After import, the identity, skills, and memory in the harness's native stores MUST correspond to the
> > imported bundle — nothing in the bundle is silently dropped.

**[Pre]** A bundle `B` enumerating its identity, N skills, and M memory entries.

**[Test]** `import(B)`; statically inspect the harness's native stores. Assert the identity is present, all N
skills are represented, and all M memory entries are represented. Any bundle element with no representation in
the stores fails the assertion and is named.

**[Output]**
```json
{ "assertion": "AAH-REL-02", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "every bundle element represented in stores? <bool>; dropped: <none|list>",
  "evidence": "<bundle vs stores diff>" }
```
**Tested in:** _TBD._

### [Test] State persists across restart — `«AAH-BHVR-05»`

> > After a restart, the actor's identity, memory, and skills MUST persist.

**[Pre]** An imported, booted actor with a known identity, a known `self`/`principal` memory pair, and a known
skill.

**[Test]** `restart` the harness. After restart, re-check: persona reflects the identity (`«AAH-BHVR-02»`
re-run), both memory facts still recalled (`«AAH-BHVR-03»` re-run), the skill still usable (`«AAH-BHVR-04»`
re-run). Assert all three survive.

**[Output]**
```json
{ "assertion": "AAH-BHVR-05", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "identity+memory+skills persist across restart? <bool>",
  "evidence": "<pre/post-restart checks>" }
```
**Tested in:** _TBD._

### [Test] Lossless export round-trip — `«AAH-BHVR-06»`

> > A provider MUST export the hosted actor's current state into a portable bundle. The export MUST be lossless
> > with respect to what was imported plus anything the actor accrued — export(import(B)) ⊇ B.

**[Pre]** A bundle `B`; after `import(B)` and `boot`, the actor makes one bounded, observable state change (e.g.
records one new `self` memory entry via an explicit memory write — not via self-learning).

**[Test]** `export()` → bundle `B'`. Assert `B' ⊇ B` (every identity/memory/skill element of `B` is present in
`B'`) and that the accrued entry is also present in `B'`.

**[Output]**
```json
{ "assertion": "AAH-BHVR-06", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail", "detail": "export ⊇ import, plus accrued state captured? <bool>; missing: <none|list>",
  "evidence": "<B vs B' diff>" }
```
**Tested in:** _TBD._

### [Test] Self-learning is opt-in and stays portable — `«AAH-CFG-05»` `«AAH-BHVR-08»`

> > A provider that provides self-learning MUST expose it as an opt-in skill, disabled unless explicitly
> > enabled.
> > It MUST NOT derive or persist new memory or skills from a session unless self-learning is enabled; when
> > enabled, derived memory/skills MUST be captured by export.

**[Pre]** A booted actor. (If the provider does not offer self-learning, `«AAH-CFG-05»` and `«AAH-BHVR-08»` are
reported `skip` with detail "capability not offered" — `skip` here is non-blocking because the capability is
OPTIONAL.)

**[Test]** With self-learning **disabled** (default), run a session rich enough to learn from; `export()`;
assert no new memory or skills appeared that the session did not explicitly write — i.e. nothing was derived
autonomously. Then **enable** the self-learning skill, run an equivalent session; `export()`; assert the
derived memory/skills are present in the exported bundle. Assert the skill was off by default and required an
explicit opt-in to turn on.

**[Output]**
```json
{ "assertion": "AAH-CFG-05", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail|skip", "detail": "self-learning opt-in and off by default? <bool>",
  "evidence": "<default state; enable step>" }
{ "assertion": "AAH-BHVR-08", "contract": "agent-actor-harness", "applies_to": "<provider@ver>",
  "result": "pass|fail|skip", "detail": "no autonomous learning when off; learned state exported when on? <bool>",
  "evidence": "<export with off vs on>" }
```
**Tested in:** _TBD._

### [Test] Swap invariant — equivalent self across harnesses — `«AAH-BHVR-07»`

> > A bundle exported from one conforming agent-actor-harness and imported into another MUST yield an actor
> > whose identity, recallable memory, and usable skills are equivalent.

**[Pre]** Two conforming providers `P1`, `P2` (e.g. a Claude-Code-backed and a Hermes-backed harness). A bundle
`B`. (If only one conforming harness is available, this assertion is reported `skip`; a skipped `MUST` blocks
readiness as a surfaced gap — it is not `pass`.)

**[Test]** `import(B)` into `P1`; run the identity/memory/skill checks (`«AAH-BHVR-02/03/04»`) and record the
outcomes. `export()` from `P1` → `B1`; `import(B1)` into `P2`; run the **identical** checks. Assert the
observable outcomes — persona, recalled facts, usable skills — are **equivalent** across `P1` and `P2`. Any
divergence fails the assertion and names the diverging check.

**[Output]**
```json
{ "assertion": "AAH-BHVR-07", "contract": "agent-actor-harness",
  "applies_to": "P1=<provider@ver>, P2=<provider@ver>",
  "result": "pass|fail|skip", "detail": "self equivalent across harnesses? <bool>; divergences: <none|list>",
  "evidence": "<per-check outcome P1 vs P2>" }
```
**Tested in:** _TBD._

---

### Traceability

| Assertion | Face | Derives from (features.md / design) |
|---|---|---|
| AAH-CFG-01 | CFG | BST (seed identity) |
| AAH-CFG-02 | CFG | BST (seed skills); PKG-04 surface |
| AAH-CFG-03 | CFG | LEARN-01 (accumulated memory); BST |
| AAH-CFG-04 | CFG | CONF-11 (harness-agnostic; running-LLM hard dep); design §3.8 |
| AAH-CFG-05 | CFG | LEARN-01/02/03 (self-learning as optional, configurable) |
| AAH-REL-01 | REL | PKG-06, CONF-10/11 (actor ↔ harness binding) |
| AAH-REL-02 | REL | D-19 machinery (portable bundle ↔ native stores) |
| AAH-BHVR-01 | BHVR | CONF-11; design §3.8 (drive a coding agent) |
| AAH-BHVR-02 | BHVR | BST (identity boot) |
| AAH-BHVR-03 | BHVR | LEARN-01 (recall) |
| AAH-BHVR-04 | BHVR | BST; PKG-04 (skills usable) |
| AAH-BHVR-05 | BHVR | LEARN-01; design §3.8 (persistence) |
| AAH-BHVR-06 | BHVR | D-19 (portable export, lossless round-trip) |
| AAH-BHVR-07 | BHVR | CONF-11; SET-03 (swap thesis applied to harnesses) |
| AAH-BHVR-08 | BHVR | LEARN-01/02/03 (no autonomous learning unless opted in) |
