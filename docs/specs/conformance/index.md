# Conformance

The conformance domain defines how the kit verifies that a setup, a provider, or an actor-hosting
harness satisfies its contract. Two parts:

- **Contract format** — how contracts are written (three faces, assertion IDs, two-part structure)
- **Verification engine** — how contracts are checked (discover, load, execute, gate readiness)

The contracts themselves are content — authored per capability type as the catalogue grows.

## Specs

| Spec | Covers |
|------|--------|
| [Contract Format](contract-format.md) | Three-face structure, assertion ID scheme, two-part document convention, contract kinds |
| [Verification Engine](verification-engine.md) | Discover/load contracts, run assertions, emit `[Output]` records, gate readiness |

## Contracts

| Contract | Scope |
|----------|-------|
| [Base Setup](contracts/base-setup.md) | Always — structural floor every setup satisfies |
| [Work Tracker](contracts/work-tracker.md) | Per-type — `tracker` capability |
| [Agent-Actor Harness](contracts/agent-actor-harness.md) | Per-type — `agent-actor-harness` capability |
