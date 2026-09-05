# R-07 — First-Product Requirements (archived)

**Status:** archived. These are the requirements of the firm's **first product** — the
developer-blueprint runtime (daemon, formation, console, Smith) whose architecture is archived in
[R-06](R-06-first-product-developer-blueprint.md). They describe the **end user ↔ Smith/console**
experience, not the firm.

They were moved here when #1 re-centered on **the firm** (the packaging consultancy). #1 designs
the firm — its contract library, grammar engine, catalogue, practice, and conformance verifier
([design.md §3–§5](../design.md)). It does **not** design this product; the firm *packages* it as its
first engagement.

These requirements are **not deleted** — they remain the **acceptance target of that first
engagement**: the developer blueprint the firm produces must satisfy them. Firm requirements proper
now live in `team/specs/loopsmith/requirements/{prac,conf,cat,comp}.md`. IDs (`BOOT-`, `CONS-`) are
preserved for traceability.

---

## BOOT — Bootstrapping & Onboarding (first product)

Observable behavior of getting from install to running, verified loops — driven conversationally by
Smith in the console.

| ID | Requirement | Priority | Source |
|----|-------------|----------|--------|
| BOOT-01 | The system MUST be installable via a single copy-paste CLI command. | must-have | #1/Q-19 |
| BOOT-02 | After installation, the CLI MUST launch the console, and conceptual onboarding MUST occur in the console. | must-have | #1/Q-14 |
| BOOT-03 | On first run, Smith MUST onboard the user conversationally, introducing the context-based mental model. | must-have | #1/Q-14, #1/Q-02 |
| BOOT-04 | Smith MUST drive setup as an emergent sequence of phases — reconciling current vs desired state and prompting the user for the next phase — rather than a fixed wizard. | must-have | #1/Q-15 |
| BOOT-05 | A persisted context MUST have exactly one home source set, which anchors and identifies it. Setting the home source is the operation that establishes this. | must-have | #1/Q-05, #1/Q-08 |
| BOOT-06 | The user MUST be able to visually design and edit a loop in Loop Studio and Save it as the desired state. | must-have | #1/Q-15, #1/Q-19 |
| BOOT-07 | The system MUST reconcile a saved desired-state into a running, healthy loop. | must-have | #1/Q-15 |
| BOOT-08 | After reconciliation, Smith MUST verify the loop is in place, healthy, and running, and MUST ask the user to verify as well (dual verification). | must-have | #1/Q-15 |
| BOOT-09 | Loop Studio MUST highlight unconfigured items and let the user open a guided creation flow for each. | should-have | #1/Q-15 |
| BOOT-10 | The system MUST provide guided source-setup flows for connecting a source to a context (e.g. GitHub app creation + installation). | must-have | #1/Q-15, #1/Q-19 |
| BOOT-11 | The system MUST support setting a git context source (a local or remote git repository) as a context's home source. | must-have | #1/Q-08 |

## CONS — Console Daily Operations (first product)

Observable behavior of the daily console experience: Smith aggregates and shepherds work across the
user's contexts.

| ID | Requirement | Priority | Source |
|----|-------------|----------|--------|
| CONS-01 | The user MUST be able to chat with Smith in the console. | must-have | #1/Q-12 |
| CONS-02 | Smith MUST present a single unified digest that aggregates flagged items across all of the user's contexts. | must-have | #1/Q-12 |
| CONS-03 | The unified digest MUST be available on demand. | should-have | #1/Q-05, #1/Q-12 |
| CONS-04 | The digest MUST flag items per context, attributing each flagged item to the context it belongs to. | must-have | #1/Q-12 |
| CONS-05 | The digest MUST report agentic-work status, including work awaiting acceptance and breaches of the user's work policy, with a recommended action for each. | must-have | #1/Q-12 |
| CONS-06 | The digest MUST include a summary of completed agentic work. | should-have | #1/Q-12 |
| CONS-07 | The user MUST be able to configure work-management policy (e.g. work-in-progress limits). | must-have | #1/Q-12 |
| CONS-08 | Smith MUST detect a stuck loop, root-cause it to a context/authority gap it cannot self-fix, and escalate to the operator with the finding and a recommended action. | must-have | #1/Q-13 |
