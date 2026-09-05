# Specs Index

The desired-state specification layer — what the loopsmith system **is**, in technical terms. These
are the setpoints an implementation agent reconciles code toward. Seeded from the codebase on
2026-06-29 (see `drift/2026-06-29-drift.md`).

## Domains

| Domain | Covers |
|--------|--------|
| [cli/](cli/index.md) | The `smith` operator binary and the `smith-agent` machine binary — command surfaces |
| [api/](api/index.md) | The `smith-agent` response envelope and exit-code contract |
| [auth/](auth/index.md) | GitHub App authentication and credential storage |
| [conformance/](conformance/index.md) | Conformance contract format, verification engine, and per-type contracts (base-setup, work-tracker, agent-actor-harness) |

## Drift reports

Transient `code → spec` reconciliation reports live in [`drift/`](drift/). They are not specs.
