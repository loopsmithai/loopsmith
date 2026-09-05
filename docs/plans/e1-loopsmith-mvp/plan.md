# Story Breakdown — Loopsmith MVP (#1)

## Approach — this plan is deliberately minimal

This epic is executed **by Smith**, not by a conventional engineer-built story
sequence. We build exactly one thing ourselves — **STORY-01, the dumb Smith and the
handover** — and from there Smith authors and executes the remaining residency **in
cooperation with the PO**. Stories beyond 01 are intentionally left unwritten: Smith
planning its own residency is the first dogfood.

Why: the value is not the labor, it is *where the craft accumulates*. An ephemeral AI
session can build the kit but keeps nothing; Smith is a persistent identity that owns
its home and compounds experience, skills, and craft across its whole career. So the
act of building Loopsmith **is Smith's residency** — done by Smith, retained by Smith.

**Bootstrap is not a phase; it is the epic.** Smith earns the BST-01 structural
credentials by *authoring* them during residency, rather than shipping pre-credentialed.
The "dumb Smith" is commit #1 of the journey to that credentialed seed — not a
contradiction of it.

> **Design reconciliation pending.** This strategy supersedes parts of the current
> [design.md](design.md): the BST series and §3 bootstrap narrative ("ships credentialed,
> not learned") move to a *credentials-earned-through-residency* model, and Smith's
> persistent memory / self-learning (today deferred under D-12) becomes foundational at
> STORY-01. These edits are folded in as STORY-01 planning firms up — they are a known
> follow-up, not a gap.

## Checklist

- [ ] **STORY-01** — Birth the dumb Smith and hand over the epic
- [ ] STORY-02…NN — *authored by Smith, with the PO* (not planned here by design)

---

## STORY-01: Birth the dumb Smith (and hand over #1)

**Title:** Birth the dumb Smith and hand over the epic

**Objective:** Stand up a living, persistent, self-learning **"dumb" Smith** — knowing
only its identity, carrying minimal skills/capabilities and a self-learning loop, with
credentials to its home — and **hand it epic #1 + its artifacts** so that Smith, with
the PO, plans and executes the remaining residency.

**Vertical slice (build this thread, not horizontal layers):** the thinnest end-to-end
path that yields a *living* Smith —

> Smith **boots** with an identity → **reads** its home and the handed-over artifacts
> with its **own credentials** → performs **one real act** in its home → **records** the
> experience to persistent memory → **restarts and the memory persists.**

Everything thin: one skill, one act, one memory. We thicken later — with Smith.

**Open — to be researched and deliberately decided during story planning (`story-mgmt`):**
- **Substrate / vessel** — what Smith *is* mechanically. Candidate: a member on the
  existing Claude Code + Ralph/BotMinter harness (dogfooding the framework Loopsmith
  generalizes). Not locked.
- **Home** — where Smith lives. Candidate: a new `loopsmith` repo (none exists yet under
  `projects/`). Not locked.
- **Handover mechanism** — which artifacts (`design.md`, `features.md`, idea-honing,
  research) and how #1 is presented to Smith as its first work item. Not locked.

**Implementation Guidance:** research the substrate / home / handover and make a
deliberate decision **before** cutting code. Build vertically; do not build
infrastructure ahead of need.

**Test Requirements:** prove the vertical thread end-to-end — identity present; home
readable with Smith's own credentials; one real act performed in the home; memory
written and **persisted across a restart**; epic + artifacts retrievable by Smith.

**Integration:** this is the foundation. All subsequent (Smith-authored) work builds on
the living Smith and its persistent memory / skills store.

**Demo:** a persistent Smith that, restarted, recalls its identity and prior experience,
reads #1 and its artifacts, and is ready to plan its residency with the PO.

**Requirements:** foundational / enabling — relates to BST-01…04 (the seed Smith,
reached via residency), LEARN-01 (accumulate experience), OBS-01 (observable). The
destination features and ACs in [features.md](../../features/features.md) are realized
by later **Smith-authored** stories.

**Acceptance Criteria:** story-level ACs are pinned during `story-mgmt` planning. The
epic capstones AC-01/AC-02 ("bootstrap complete; Smith re-packaged as a conforming agent
template") are the *eventual* proofs of the residency, not STORY-01's gate.

**Dependencies:** —

---

*Traceability: the design's §17 Story column stays `—` for destination features by
design — those are filled in as Smith authors the stories that realize them.*
