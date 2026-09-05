# ⚠️ Decision needed — design.md / plan.md are out of sync (epic #1)

*Written 2026-06-22 by the autonomous /loop supervisor. No issues created, nothing
deleted. All old content is recoverable from git. Story externalization is BLOCKED
until you resolve this.*

## What happened (verified from git)

| Commit | Date | Effect |
|---|---|---|
| `a14d9c1` | 06-21 | Full design.md — **1164 lines, 47 sections, ~36 ACs**, "revise design after **3-persona review**" |
| `5ce7bf9`/`60fe5e7` | 06-21 | `plan.md` created + "revised after review" against that design |
| `7ac9a11` | 06-21 | **"replace requirements + design with consolidated feature set"** — deleted the 1164-line design **and** replaced the old requirement files (`cat/comp/conf/pkg.md`) with the consolidated `features.md` (70 features) |
| `618fe2d`→`1567e21` | 06-22 | This session rebuilt design.md **§1–§5** on the BPMN/lane/driver footing |

**Current state:** design.md = 690 lines, **0 ACs**, §6/§7/§9–§17 stubbed. `plan.md`
still references **AC-01…AC-36** and sections (`§1.6`, `§5.8`, `§9`) that no longer
exist, and an architecture from before PD-16…PD-20. Epic #1 has **no story
sub-issues** yet.

## Why story creation is blocked
Externalizing stories from the current `plan.md` would bake the **stale** architecture
+ dead AC/section references into GitHub issues that can't be cleanly deleted
(maintainer mandate). Don't create issues until design.md + plan.md are reconciled.

## Options
- **(A) Rewrite-forward** — current §1–§5 is the source of truth; write fresh §6–§17 +
  ACs against `features.md` + BPMN; regenerate `plan.md`. *Cost:* reinvents the
  reviewed §9–§17/ACs from blank.
- **(B) Restore-and-merge** (the fork's pick) — restore `a14d9c1`'s §6–§17/ACs and merge
  forward. *Problem:* its ACs/traceability reference the **deleted** requirement files
  (`cat/comp/conf/pkg`) and predate pixi + BPMN — a heavy, error-prone merge that
  resurrects superseded IDs.
- **(C) Hybrid — RECOMMENDED** — keep the current BPMN/pixi-aware §1–§5; **port
  `a14d9c1`'s §6–§17 + ~36 ACs + §9–§17 forward as a seed**, re-mapping old requirement
  IDs → `features.md` IDs and updating for BPMN/lanes/drivers. Salvages the reviewed
  structure/content without rebuilding from stubs, and lands on the current architecture.

## Also worth your eye
The §1–§5 foundation itself was **actively reworked in this session** (PD-18/19/20,
lanes, driver-artifacts, storage-vs-view) and is **not yet confirmed by you**. Writing
36 ACs + a story breakdown against a model that changed hours ago is premature
regardless of A/B/C. Recommend: **(1) confirm the current §1–§5 direction is the
foundation, (2) pick C, then (3)** drive §6–§17 → ACs → regenerate plan.md → externalize
stories.

## Recover the old reviewed content anytime
```
git show a14d9c1:specs/loopsmith/178-loopsmith-mvp/design.md   # full reviewed design (§1–§17, ~36 ACs)
git show 60fe5e7:specs/loopsmith/178-loopsmith-mvp/plan.md      # matching reviewed story breakdown
```
