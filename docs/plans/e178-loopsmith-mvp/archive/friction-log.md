# Friction Log — the kit's learning instrument

The kit's **standing learning instrument**, fed by **every packaging run**. Each time the packaging
skills turn a setup into a conforming one, every point where the real setup *resists* the kernel, a
capability-type contract, or the grammar is recorded here **as it is hit** — because that resistance is
the evidence that tells us whether the kit's contracts/grammar/skills are right. Per
[design.md §4.7](../design.md#47-friction-log) and [PKG-11](../requirements/pkg.md), appending friction is
a first-class behavior of the packaging skills, not an afterthought.

The **first and richest** packaging run is the BotMinter port (the `developer` blueprint — R-06/R-07);
later runs (a Gascity setup, a single OpenClaw agent, a Claude-Code-plus-plugins setup) append to the
same log.

## How to use this log

- One row per friction point, added **in the moment** it is encountered (not reconstructed later).
- Keep entries blunt and specific — name the exact **run**, the exact **concept** (kernel noun,
  capability type, or grammar shape), and the exact resistance.
- **Disposition** is one of (aligned with §4.7):
  - `change-contract/grammar` — the kit's methodology is wrong/incomplete; change a kernel/capability
    contract or the grammar (link the §8 D-NN it feeds).
  - `grow-catalogue` — the model is right; mint a family/blueprint/module so this is **reuse** next
    time (note what was minted).
  - `defer` — real friction, but out of MVP scope; park it (note where).
- This log feeds the §8 design decisions and the methodology-evolution process. A cluster of
  `change-contract/grammar` rows around one concept is a signal the kit needs rework there.

## Log

| # | Run | Concept (noun / type / grammar) | Friction (what resisted, and why) | Disposition | Notes / link |
|---|-----|----------------------------------|-----------------------------------|-------------|--------------|
| _(populated during each packaging run; BotMinter port is the first)_ | | | | | |
