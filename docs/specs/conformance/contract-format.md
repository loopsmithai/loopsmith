# Contract Format Specification

## Purpose

Defines the structure every conformance contract follows — three faces, stable assertion IDs, and a
two-part document convention (normative contract + test-plan). Any contract authored for a new
capability type uses this format, so all contracts are verifiable the same way.

## Requirements

### Requirement: Three-face structure

Every conformance contract SHALL be organized into three faces:

| Face | Question | Verification method |
|------|----------|-------------------|
| Required configuration | What config must a provider accept/expose? | Static inspection of the assembled setup |
| Required data relationships | What structural relationships must hold? | Static inspection of the setup graph |
| Required observable behavior | What must the provider do? | Behavioral exercise against the provider |

### Requirement: Assertion IDs

Every normative clause in a contract SHALL carry a stable assertion ID as an inline prefix, using the
scheme `«<SCOPE>-<FACE>-<NN>»`:

| Component | Values |
|-----------|--------|
| SCOPE | `BASE` (base contract) or a capability-type abbreviation (`TR`, `SRC`, `AAH`, etc.) |
| FACE | `CFG` (configuration), `REL` (data relationship), `BHVR` (observable behavior) |
| NN | Zero-padded sequence within scope+face |

Assertion IDs SHALL be stable once published — append only, never renumber.

#### Scenario: Assertion ID traces across layers
- GIVEN a contract with assertion `«TR-BHVR-04»`
- WHEN the verification engine runs
- THEN the `[Output]` record cites `TR-BHVR-04` as the `assertion` field
- AND a failure report names the specific violated assertion IDs

### Requirement: Two-part document structure

Each contract document SHALL have two parts:

- **Part A — Normative contract:** RFC 2119 prose from the implementer's point of view, with an
  Abstract, three face subsections, and inline assertion IDs.
- **Part B — Conformance test-plan:** One subsection per assertion (or cluster), each with: the
  block-quoted normative clause, `[Pre]` fixture, `[Test]` runnable check, `[Output]` envelope,
  and `Tested in:` back-link.

### Requirement: Two contract kinds

The kit SHALL define exactly two kinds of contract:

- **Base contract** — the always-applicable structural floor every setup satisfies. Exactly one
  exists. Written over the five kernel nouns (context, loop, item, actor, port).
- **Per-capability-type contract** — one per capability type. Defines what any provider of that type
  must satisfy so providers are interchangeable.
