# Verification Engine Specification

## Purpose

The verification engine runs conformance contracts against a candidate setup and gates readiness.
It discovers which per-type contracts apply, executes every assertion, emits a machine-readable
`[Output]` record per assertion, and refuses to mark a setup ready until all `MUST` assertions pass.

## Requirements

### Requirement: Discover and load contracts by capability type

The verification engine SHALL discover per-type contracts by the capability types present in a
candidate setup. A type's contract ships in its package. The set of authored per-type contracts is
deliberately open — a setup using a type whose contract is not yet authored is verified against the
base plus whatever per-type contracts exist.

#### Scenario: Setup with an un-authored capability type
- GIVEN a setup using capability type `calendar` with no authored contract
- WHEN verification runs
- THEN the base contract is checked
- AND no `calendar` per-type contract is loaded (none exists)
- AND verification does not fail due to the missing contract

### Requirement: Run base + per-type contracts and emit per-assertion output

The verification engine SHALL run the base contract plus every applicable per-type contract against
a candidate setup. It SHALL emit one `[Output]` record per assertion:

```json
{
  "assertion": "<SCOPE-FACE-NN>",
  "contract": "<contract-name>",
  "applies_to": "<provider@version>",
  "result": "pass | fail | skip",
  "detail": "<human-readable; on fail, the specific violation>",
  "evidence": "<query result, command output, or operation trace>"
}
```

#### Scenario: Behavioral assertion cannot be exercised
- GIVEN a `MUST` assertion that requires a running provider not available in this environment
- WHEN verification runs
- THEN the assertion result is `skip`
- AND a skipped `MUST` blocks readiness as a surfaced gap (it is NOT treated as `pass`)

#### Scenario: SHOULD violation does not block readiness
- GIVEN a `SHOULD` assertion that fails
- WHEN verification runs
- THEN the assertion result is `fail`
- AND the failure is reported as a warning
- AND readiness is NOT blocked

### Requirement: Gate readiness on all MUST assertions passing

A setup SHALL be marked ready if and only if:
1. The base contract passes (every `MUST` assertion is `pass`), AND
2. Every applicable per-type contract passes.

The verification report SHALL list per-contract pass/fail plus the specific violated assertion IDs.
A non-conforming setup SHALL NOT be marked ready.

#### Scenario: One MUST assertion fails
- GIVEN a candidate setup where `«BASE-REL-02»` fails
- WHEN verification completes
- THEN the setup is NOT marked ready
- AND the report cites `BASE-REL-02` as the violated assertion

## Limitations & Known Issues

### Boundaries
- The per-type contracts are authored incrementally as the catalogue grows — they are content, not
  machinery.
- `MAY` clauses are never assertions and produce no `[Output]` records.

### Known gaps & accepted constraints
- No runnable conformance tests exist yet — every contract's `Tested in:` says `_TBD._`. STORY-01
  targets the agent-actor-harness contract as the first exercised test.
