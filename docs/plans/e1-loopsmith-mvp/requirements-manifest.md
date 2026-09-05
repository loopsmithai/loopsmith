# Requirements Manifest — Loopsmith MVP (#1)

#1 designs **the kit**: the conformance contracts, the package grammar, the packaging skills, and the
bootstrap-Smith that carries them. The requirements catalogue therefore describes the kit as a generic
capability — what it lets each persona do — not the specific dogfood run that proves it.

## Catalogue

All requirements live in a single journey-organized catalogue file,
[features.md](../../features/features.md), organized in lifecycle order across four personas (P1 Loopsmith
Developer · P2 Family Author · P3 Package Author · P4 End User). Each journey section carries its own
ID series; the series prefix is the category.

| Series | Journey | Count |
|--------|---------|-------|
| [BST](../../features/features.md#bootstrap--the-apprenticeship-4-features) | Bootstrap — the apprenticeship (train Smith, seed the catalogue) | 4 |
| [FAM](../../features/features.md#family-authoring--growing-the-kits-reach-4-features) | Family authoring — new families & blueprints | 4 |
| [PKG](../../features/features.md#packages-9-features) | Packages — provides/requires, capability identity, contracts, lifecycle hooks, meta-skill | 9 |
| [DIS](../../features/features.md#discover--choose-5-features) | Discover & choose — browse blueprints/packages, capability specificity, alternatives, gaps | 5 |
| [CNSLT](../../features/features.md#the-consultation--conversational-discovery-with-smith-5-features) | The consultation — conversational discovery with Smith | 5 |
| [BLD](../../features/features.md#build-a-setup-with-the-builder-agent-9-features) | Build a setup with the builder agent — target → recommend → author → verify | 9 |
| [AWV](../../features/features.md#assemble-wire--verify-9-features) | Assemble, wire & verify — resolve, install, connect sources, conformance check | 9 |
| [CNTXT](../../features/features.md#contexts-sources--connection-rules-5-features) | Contexts, sources & connection rules — membership ⟂ access, home source, unification | 5 |
| [SET](../../features/features.md#what-a-conforming-setup-contains-11-features) | What a conforming setup contains — the house spec | 11 |
| [OPS](../../features/features.md#operate--evolve-a-running-setup-5-features) | Operate & evolve — swap, remove, version/upgrade/rollback, incremental, runtime skills | 5 |
| [USR](../../features/features.md#smith-gets-better-at-serving-you-3-features) | Smith gets better at serving you — per-user friction memory, grounded advice, catalogue growth | 3 |
| [OBS](../../features/features.md#cross-cutting) | Cross-cutting — observability | 1 |

**Total:** 70 features across 12 series, one catalogue file.

## Design decisions parked for design.md

The settled model decisions live in [design-notes.md](design-notes.md) as provisional **PD-01 … PD-17**.
They become **D-NN** (with full rationale + ADRs) when `design.md` is written (epic-mgmt Step 8). The
load-bearing ones:

- **Everything is a package** over one open capability namespace (RPM/dnf lineage) — `provides`/`requires`,
  virtual provides at specificity, gating-by-absence; families/blueprints are metapackages
  ([PD-08](design-notes.md), → PKG-01…04, PKG-16/04/05, PKG-33).
- **The kit ships generators, never instances** — contracts + grammar + skills + bootstrap-Smith; the
  catalogue starts empty and is produced by running the kit; the two MVP runs are the start of the flywheel
  ([PD-11](design-notes.md), → BST-02, LEARN-03).
- **Smith is kit machinery** — the single irreducible seed, credentialed on structure and apprenticing on
  craft, with a build/furnish/maintain repertoire ([PD-09/PD-10](design-notes.md), → BST-01…04, CNSLT-*,
  BLD-*).
- **Agent template ⟂ loop template, composed by a binding** with an equip/train fit-check
  ([PD-13](design-notes.md), → PKG-05, CONF-08/10/11, PKG-28).
- **Port / membership / home-source** rules: membership is declarative and independent of access
  ([PD-05/06/07](design-notes.md), → CNTXT-01…05).
- **Learning mode** — Smith records friction and, under human validation, acts on it by authoring skills
  ([PD-14](design-notes.md), → PKG-29/07, PKG-13, LEARN-01/02).
- **Packaging infrastructure: pixi / rattler / resolvo**, wrapped behind a kit-owned `smith-agent` boundary
  — HOW, not WHAT; the catalogue stays pixi-invisible ([PD-16/PD-17](design-notes.md)).

## Notes

- **Requirements describe the kit as generic capability**, not the specific dogfood run. The operator's
  BotMinter port (run #1, the richest packaging run) and the `simple-assistant` reuse (run #2) are the
  end-to-end **acceptance demo** that proves these features; they live in `design.md` as acceptance criteria
  traced back to these IDs.
- **Priority / MVP cut** is not yet a column in `features.md`. The MVP spine — Smith trained (BST), the
  package grammar and contracts (PKG/AWV), dual-blueprint conformance via the skills, and in-place evolution
  (OPS) — is must-have; the should-/may- softenings are carried in each feature's RFC 2119 keywords
  (e.g. PKG-06/07 MAY, PKG-30 SHOULD, CONF-06 MAY). A formal priority pass is an open item for design.
