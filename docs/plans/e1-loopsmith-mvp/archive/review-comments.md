# Design Review Comments — PO Session

Tracking comments from the PO during the design walkthrough. Each comment is investigated against the source artifacts (design.md, idea-honing.md, requirements-manifest.md, concept-reconciliation.md, features.md) to find root causes and reconcile.

---

## Comment 1: "Does that mean once a setup is out, I can no longer add a package or change it?"

**Source:** Slide 5 caption — "The kit ships generators, never instances" framing implied setups are produced-and-done.

**Investigation:**
- design.md §4.3 (OPS verbs): `remove`, `upgrade`, `rollback` are explicit operations on the packaging boundary.
- design.md §2 (OPS series): PKG-42 through PKG-46 — evolve a running setup with re-verification; remove packages; version/upgrade/rollback; incremental builder operation; runtime skill-package install.
- design.md AC-24: "Given a running setup, when a package is added or removed, then the setup is re-verified."
- design.md AC-25: "a skill-package can be installed into a running setup."
- design.md §3.6: "Build is re-entrant from furnish/maintain."

**Root cause:** The presentation's caption on slide 5 omitted the OPS lifecycle entirely. The phrase "ships generators, never instances" is about the kit's DELIVERABLE (machinery, not pre-built content) — not about whether a produced setup is frozen. The design doc is clear that setups are evolvable.

**Resolution:** Caption rewritten to explicitly state setups are evolvable in place with re-verification. Fixed.

---

## Comment 2: "How can it say its users are not end users, yet in-place evolution is allowed — won't that be done by those end users?"

**Source:** Slide 3 caption — "Its users are the people who build and grow agentic setups, not the end users who live in them."

**Investigation:**
- design.md §1 (line 56-58): "P4 End User stands up, wires, runs, and evolves a setup — the primary target and the bulk of the feature set."
- design.md §2: The OPS, AWV, DIS, CNSLT, and BLD series are all P4-facing. P4 is not a passive consumer.
- idea-honing.md Q-19: "Kit features" include packaging skills and conformance — P4 uses these through Smith.
- idea-honing.md Q-22: The packaging skills (discover/recommend/co-design/realize/verify) are what P4 exercises during furnish and maintain loops.
- requirements-manifest.md: features.md is "organized in lifecycle order across four personas (P1...P4)." P4 is a first-class persona with direct feature coverage.

**Root cause:** The presentation injected a framing ("not the end users who live in them") that contradicts the design doc. The design doc says P4 is "the primary target and the bulk of the feature set" — P4 is a direct user of the kit's packaging operations (via Smith's furnish/maintain loops), not just a beneficiary.

**Deeper root cause in the design doc itself?** Partially. The design doc §1 says "Its direct users are the P1-P3 personas... the P4 end user is the ultimate beneficiary." But then §1 also says P4 "stands up, wires, runs, and evolves a setup" — which IS direct use of the kit. And P4 is "the primary target and the bulk of the feature set." There is tension in the design doc between "P4 is the beneficiary" (line 26) and "P4 is the primary target" (line 58). The feature set resolves this tension in P4's favor — P4 exercises the kit through Smith.

**Design doc tension identified:**
- Line 24-26: "Its direct users are the P1–P3 personas... the P4 end user is the ultimate beneficiary, who lives in the setups it produces."
- Line 56-58: "P4 End User stands up, wires, runs, and evolves a setup — the primary target and the bulk of the feature set."

These two statements are in tension. Line 24-26 positions P4 as indirect ("beneficiary who lives in setups"); line 56-58 positions P4 as direct and primary. The feature set (70 features) confirms P4 is direct — the OPS, AWV, DIS, CNSLT series are all P4 operations.

**Resolution:** Presentation caption fixed to state all four personas are direct users.

### Full Artifact Audit — P4 Characterization

Audited all spec artifacts for every mention of P4/end-user. Results:

**INDIRECT characterizations (P4 as passive beneficiary):**
- design.md line 24-26: "Its direct users are the P1-P3 personas... the P4 end user is the ultimate beneficiary, who lives in the setups it produces."
- design.md line 30: "the running setup (n-0) the end user lives in"

**DIRECT characterizations (P4 as active user):**
- design.md line 57-58: "P4 End User stands up, wires, runs, and evolves a setup — the primary target and the bulk of the feature set."
- design.md line 86, 292, 916: "catalogue grows from end-user runs" / "end-user runs feed the catalogue"
- design.md AC-29: "a second user stands up a different setup by reusing existing packages"
- features.md line 9: "The primary target is the End User — Loopsmith exists first to be its author's own Jarvis."
- features.md line 14-16: "Stands up, wires, runs, and evolves their own setup, then lives in it."
- features.md: 55+ features (DIS, CNSLT, BLD, AWV, CNTXT, SET, OPS, USR series) written with P4 as the acting user ("As a user...")
- requirements-manifest.md: P4 is a first-class persona with direct feature coverage

**AMBIGUOUS:**
- design.md line 53, 251; idea-honing.md line 257: "Smith onboards the end user" (P4 receives onboarding — implies active use)

**Zero mentions** in: concept-reconciliation.md, friction-log.md, summary.md.

### Verdict

The tension is **isolated to one paragraph** in design.md (lines 24-26) plus a minor echo at line 30. The overwhelming weight — features.md, the persona table, the ACs, every feature series — positions P4 as a direct, primary user who exercises the kit through Smith.

### Recommended design.md fix

Lines 24-26 should be reworded. The current text draws a P1-P3 vs P4 line that the rest of the document contradicts. Suggested:

> Its users span four personas (P1-P4); the **P4 end user** — who stands up, wires, runs, and evolves setups — is the primary target and the bulk of the feature set. P1-P3 build and grow what's available in the catalogue.

The word "beneficiary" should be dropped. "Lives in the setups it produces" (line 30) is fine as long as it's additive to "stands up, wires, runs, and evolves" rather than the sole characterization.

**Status:** Presentation fixed. Design doc lines 24-26 flagged for author — isolated wording issue, not a systemic problem.

---

## Comment 3: "What is 'runtime server'?"

**Source:** Presentation caption on slide 3 and slide 5 used the phrase "hosted runtime server" — a term not in the design doc.

**Investigation:**
- design.md §3.1 (line 100): "The kit's runtime is not a server that hosts setups; it is the machinery that produces and verifies them."
- design.md §4.4: The `runtime` capability type is about placement (local process / k8s pod / VM) — a recognized capability, not a kit property.
- design.md §3.8: The system context ships a daemon + event-bus as firmware.

**Evidence:** `grep -rni "runtime server"` across all spec artifacts: **zero hits**. The phrase does not exist in any artifact. The only related text is design.md line 100: "The kit's runtime is not a server that hosts setups; it is the machinery that produces and verifies them."

**Root cause:** The presentation invented the phrase "hosted runtime server" which conflates three distinct design doc concepts: (1) "not a server that hosts setups" (what the kit is not), (2) the `runtime` capability type (placement), and (3) the system context firmware (daemon + event-bus). The design doc uses none of these together as one term. This is a presentation-only fabrication — no spec artifact contributed to it.

**Resolution:** All instances replaced with the design doc's own language: "not a server that hosts setups" and "ships a system context with firmware (daemon + event-bus)."

**Additional finding (follow-up from PO):** `grep -rni "\bserver\b"` across main spec artifacts: exactly **one hit** — design.md line 100, and only to say what the kit is NOT ("not a server that hosts setups"). The qualifier "that hosts setups" matters — the presentation's slide 11 dropped it, saying "NOT a server" flat, which overstates the negation. Fixed to match the design doc's exact phrasing.

**Status:** Presentation fixed.

---

## Comment 4: "What do you mean 'no product UI'?"

**Source:** Slide 5 — "No product UI (that's the 'first product,' designed separately — see idea-honing Q-14/Q-15/Q-16)"

**PO concern:** The phrase "no product UI" is unclear — what does it mean? Is the kit headless? Does Smith have no interface?

**Investigation:**

`grep -rni "no product UI\|no.*product.*UI\|no UI"` across all spec artifacts: **zero hits**. The phrase "no product UI" does not exist in any artifact. This is another presentation invention.

What the design doc ACTUALLY says about its user-facing surface:
- design.md §3.7 (line 34): the kit ships "two command-line binaries over the packaging engine — `smith` for humans and `smith-agent` for agents"
- design.md §4.6: Smith conducts "adaptive conversation" (PKG-19), the "consultation surface"
- design.md §4.8: "adjustable verbosity/log levels and a stream" via "the configured interface"
- design.md §4.4 (recognized types): `interface` is a capability type — "Channel(s) (console, telegram, matrix...), bound to loops/actors"

What the idea-honing says about the "first product":
- idea-honing.md Q-14: "Console-first. The first product's CLI only installs and launches the console."
- idea-honing.md Q-16: "Web UI (browser) for the first product."
- idea-honing.md canonical vocabulary (line 74-77): "first product — the developer-blueprint runtime (BotMinter-equivalent: daemon, console, Loop Studio). Not the kit."

The kit/product split is: the kit ships `smith` + `smith-agent` (two CLIs) and the `interface` capability type. The "first product" (R-06/R-07, idea-honing Q-14-Q-16) adds the console, Loop Studio, and web UI on top. But the kit is NOT headless — it ships two CLIs and Smith conducts consultations through the configured interface.

**Root cause:** The presentation said "No product UI" as a bullet point under "What the kit does NOT ship." This conflates two things: (1) the kit doesn't ship the console/Loop Studio/web UI (true — those are the "first product"), and (2) the kit has no user interface (false — it ships `smith` CLI and Smith's consultation surface). The bullet implies the kit is headless, which it isn't.

Additionally, "product UI" is the presentation's own term. The design doc distinguishes between the kit's CLIs/interface capability and the "first product's" console/Loop Studio — but it never uses the phrase "product UI."

**Resolution:** Remove the "No product UI" bullet. Replace with language that distinguishes the kit's interface (smith CLI + consultation surface + interface capability) from the first product's additional surfaces (console, Loop Studio, web UI) without implying the kit is headless.

**Status:** Presentation fixed. "No product UI" bullet replaced with accurate distinction: the kit ships smith/smith-agent CLIs + interface capability; the first product adds console/Loop Studio/web UI on top.

---

## Comment 5: "I am so confused by 'how the MVP proves this — two runs, run 1, run 2' building two real families and blueprints, then the very next slide asks 'is it clear that #1 builds the kit, not the product?'"

**Source:** Slide 7 (MVP Thesis) describes two runs that produce real families and blueprints. Slide 8 (Review prompt) asks "Is it clear that #1 builds the kit, not a product?"

**PO concern:** If the MVP's proof IS producing two real setups (families, blueprints, packages), how can the design simultaneously claim it's "not a product"? The two runs ARE the product output — they produce real, usable things. The kit/product distinction collapses when the proof of the kit is building products.

**Investigation:**

`grep -n "kit.*not.*product\|not.*product\|not a product"` across design.md: **zero hits** for "not a product." The design doc never says "not a product." In fact, line 22 says: "Loopsmith is a **kit** — a product in the sense a compiler toolchain or an IDE is a product."

The "not a product" language comes from:
- idea-honing.md Q-01 (line 85): "The MVP builds the kit... **not a product**."
- idea-honing.md canonical vocabulary (line 76-77): "**Not the kit** — and Smith is kit machinery, shipped OOTB in this blueprint, not a product-only persona."

The idea-honing uses "not a product" to mean: what #1 delivers is the machinery (contracts + grammar + skills), not the "first product" (the developer-blueprint runtime with console/Loop Studio/web UI, designed in R-06/R-07). The "first product" is a separate deliverable that USES the kit.

BUT the two MVP runs (design.md lines 62-65) produce real families, blueprints, and packages — `factory` family, `developer` blueprint, `simple-assistant` blueprint. These ARE concrete, usable outputs. The idea-honing acknowledges this (Q-01 line 88): "produced, not hand-shipped." The kit's proof IS producing things.

**The confusion:** The presentation juxtaposes:
1. "The MVP proves this with two real runs that produce families and blueprints" (slide 7)
2. "Is it clear that #1 builds the kit, not a product?" (slide 8 review prompt)

A reader sees the kit producing real, concrete, usable artifacts and is then asked to agree it's "not a product." This is incoherent because:

- The design doc itself says the kit IS "a product in the sense a compiler toolchain is a product" (line 22)
- The two runs produce real content (families, blueprints, packages) that users use
- "Not a product" is idea-honing's shorthand for "not the first product (R-06)" — a distinction between the kit and one specific product built on it

**Root cause analysis — is this a presentation problem or a design doc problem?**

The design doc is internally consistent on this: the kit is a product (line 22), it produces content (the two runs), and the "first product" (R-06) is a separate thing built using the kit. The kit/first-product split is clear in the design doc.

The confusion has two sources:
1. **The presentation's review prompt** (slide 8) asked "is it clear that #1 builds the kit, not a product?" — importing the idea-honing's "not a product" shorthand, which contradicts the design doc's own line 22 ("a product in the sense a compiler toolchain is a product").
2. **The idea-honing's "not a product" shorthand** (Q-01 line 85) is confusing when read without context. It means "not the first product (R-06)" but reads as "not a product at all."

**Resolution needed:**
1. Presentation: remove the "kit, not a product" review prompt — it contradicts the design doc and confuses readers who just saw the kit producing real things.
2. Flagged for design author: the idea-honing's "not a product" shorthand (Q-01) may need qualification. The design doc's framing ("a product in the sense a compiler toolchain is a product") is clearer. The kit IS a product; what it is NOT is the "first product" (the developer-blueprint runtime with console/Loop Studio).

**Status:** Presentation fixed. Review prompt rewritten to use the design doc's own framing (the kit IS a product; the question is whether the kit vs "first product" split is clear). Flagged for design author: idea-honing Q-01's "not a product" shorthand may need qualification to align with design.md line 22.

---

## Comment 6: "If I'm building a compiler, and in the MVP I compile real software and test it, I can't say 'the MVP never produces any software.' Maybe it never deploys it in production, but it DOES produce software."

**Source:** Follows from Comment 5. The design's framing around "ships generators, never instances" and the idea-honing's "not a product" imply the kit produces nothing concrete. But the MVP's two runs DO produce concrete, usable artifacts — families, blueprints, packages, a transpiled ralph.yml that enacts and transitions items through a workflow (AC-28).

**PO's analogy:** A compiler MVP that compiles and tests real software against compiler bugs can't claim it "never produces software." It produces software — it just doesn't deploy it in production. Similarly, the kit produces real setups — it just doesn't host/deploy them.

**Investigation:**

The design doc says both things simultaneously:

**"Ships generators, never instances" / "catalogue starts empty":**
- design.md D-02 (line 979): "The kit ships generators, never instances."
- design.md line 106: "The kit emits generators, never instances."
- design.md line 119 (mermaid): "Catalogue (starts empty, grows per run)"
- idea-honing Q-01 (line 85): "The kit ships generators, never instances (no one-offs): the catalogue starts empty."
- idea-honing Q-04 (line 128): "No one-offs — the kit ships generators, never instances."

**But the kit DOES produce concrete outputs:**
- design.md line 48: "The catalogue — families, blueprints, packages, templates — is produced by running the kit."
- design.md line 288: "run #1 (Smith packaging BotMinter) produces the `developer` blueprint on a `factory` family"
- design.md AC-28 (line 929-934): the port run produces a setup that "passes base + every per-type contract," "transpiles to a runnable ralph.yml," and "≥1 item transitions through its workflow"
- design.md AC-01 (line 785): "it produces conforming packages"
- design.md line 26: P4 "lives in the setups it produces"
- idea-honing Q-04 (line 353): "What the first runs *produce* (the catalogue starts empty — no one-offs)"

**The contradiction:** "Never instances" is stated as a principle, but the MVP's own proof is producing instances (families, blueprints, packages, a transpiled runnable setup). The design doc acknowledges this — line 48 says "produced by running the kit" — but frames it under "generators, never instances" which reads as "the kit produces nothing."

**What "generators, never instances" actually means in D-02:** The rejected alternative was "ship a seed catalogue of pre-built families/blueprints." The rationale (line 980-982): "avoids one-offs, makes the two MVP runs the proof of the abstraction, and keeps the kit/instance boundary clean."

So the decision is: the kit does NOT ship with pre-built content (no seed catalogue). The content is produced by running the kit. This is NOT the same as "the kit never produces content." Using the PO's compiler analogy:
- "Ships generators, never instances" = "we ship the compiler, not pre-compiled binaries"
- But the compiler DOES compile real software during its MVP validation
- The compiled software EXISTS and is usable — it's just produced, not pre-shipped

**Root cause:** The phrase "generators, never instances" is overloaded. D-02's actual decision is narrow: don't PRE-SHIP content, produce it by running the kit. But the phrase reads as a broader claim: the kit never produces concrete things. The design doc itself contradicts this broader reading on lines 48, 288, 785, and in AC-28.

This is a **design doc wording issue**, not just a presentation issue. The phrase "never instances" should be qualified to mean "never pre-ships instances" or "instances are produced, not pre-shipped." The current phrasing misleads both the presentation author (who wrote "no product UI," "not a product") and the PO reviewer.

**Recommended design.md fix for D-02:**

Current: "The kit ships generators, never instances."
Suggested: "The kit ships the machinery, not pre-built content — the catalogue starts empty and is produced by running the kit."

This preserves D-02's actual decision (no seed catalogue) without implying the kit produces nothing.

**Status:** Presentation fixed — D-02 slide retitled "Ships Machinery, Not Pre-Built Content," caption added explaining the distinction. All "generators, never instances" references qualified. Design doc D-02 phrasing still flagged for author.

---

## Comment 7: "FAM and PKG should probably be the same category called PKG about packaging — a family and a blueprint are all packaging story"

**Source:** Slide 9 (Requirements Landscape) lists FAM (4 features) and PKG (9 features) as separate series.

**PO's point:** Families and blueprints are metapackages (the design doc says so explicitly in §3.3 and §3.5). If a family is a metapackage, its authoring features belong in the packaging series, not a separate "family authoring" series. The separation implies families are a distinct concept from packages, when the design doc's own D-01 says everything is a package.

**Investigation:**

What D-01 says (design.md line 973-977): "Everything is a package over one open capability namespace... families/blueprints are metapackages."

What design.md §3.3 says (line 159): "Families and blueprints are metapackages — curated package-sets."

What design.md §3.5 says (line 231): "family / blueprint — metapackages (§3.3)."

What the FAM features actually cover (features.md lines 35-38):
- PKG-10: Author a new family (a curated substrate package-set) with Smith
- PKG-11: Author a new blueprint on a family (adding loop templates, actor configs, capability-providing packages)
- PKG-12: Publish packages to a catalogue (public or private)
- PKG-13: Feed structural friction back to the kit

What the PKG features cover (features.md lines 46-54):
- PKG-01..03: Declares provides/requires/identity
- PKG-04: Per-type conformance contract
- PKG-05: Agent template / loop template composability
- PKG-06..09: Coding-agent support, lifecycle hooks, meta-skill

**Analysis:** The FAM features are about AUTHORING packages (specifically metapackages — families and blueprints) and PUBLISHING them. The PKG features are about the SHAPE and CONTRACT of packages (what they declare, how they compose, what they carry). Both are packaging concerns.

Moreover:
- PKG-12 (publish to catalogue) is a packaging operation — it appears in the `smith-agent` verb table (design.md line 527) alongside other packaging operations.
- PKG-10/02 (author a family/blueprint) is Smith running the packaging skills (build loop) to produce metapackages — the same skills that produce any package.
- PKG-13 (friction feedback) is the cross-cutting "learn" step of the packaging skills.

The PO's observation is correct: under D-01 (everything is a package), family/blueprint authoring IS packaging. The separation into a distinct FAM series creates the impression that families are a different kind of thing, when the design's own model says they are metapackages.

**Why the separation might exist:** The features.md organizes by PERSONA JOURNEY, not by concept. FAM is the "Family Author (P2)" journey. PKG is about package shape/contracts (primarily P3 Package Author). The separation is a journey/persona cut, not a conceptual cut. But this is not obvious — a reviewer reading "Family Authoring" and "Packages" as separate series naturally assumes they are separate concepts.

**Root cause:** The requirements organization by persona journey creates a false conceptual split. Under D-01, families and blueprints are metapackages. The FAM series features (author, publish, friction feedback) are packaging operations on metapackages. A single PKG series covering both package shape/contracts AND metapackage authoring/publishing would be more consistent with D-01.

**PO follow-up:** "I thought the split is also by journeys — so FAM and PKG are two different journeys?"

Yes — the design.md §2 table header is "Journey." FAM = "Family authoring" journey, PKG = "Packages" journey. But "Packages" is not a journey — it's a shape description. The actual journeys are:

| Series | Journey name in design.md §2 | What the journey IS |
|--------|------------------------------|---------------------|
| FAM | "Family authoring" | Author metapackages (families/blueprints), publish, friction feedback |
| PKG | "Packages" | Declare provides/requires, contracts, composability, hooks, meta-skill |

FAM is a journey (author → publish → feedback). PKG is a set of shape/contract requirements — not a user journey. The design.md §2 table calls PKG a "journey" but its features are structural declarations (PKG-01: "A package MUST declare..."), not journey steps. If both were truly journeys, PKG would be "Package authoring" and would overlap heavily with FAM (which IS about authoring packages — specifically metapackages).

This reinforces the Comment 7 finding: the FAM/PKG split is inconsistent. FAM is a journey; PKG is a grammar. Under D-01 they belong together — FAM's journey features (author, publish, feedback) could sit alongside PKG's shape features in one "Packaging" series, or PKG could be reframed as a journey ("Package authoring & shape").

**Flagged for design author:** Consider either (a) merging FAM into a broader "Packaging" series, or (b) reframing PKG as a journey to match FAM. The current split treats one as a journey and the other as a grammar, which is inconsistent. The persona distinction (P2 vs P3) can live in the user stories.

**PO follow-up — three invariants for feature prefix organization:**

1. **Feature journeys should stay** — they help readability. The journey grouping is good.
2. **The set of prefixes must be finite and slow-growing** — features will keep growing, so prefixes should be stable categories, not a prefix per persona or per journey.
3. **Prefixes stem from the DOMAIN CATEGORY, not from the persona or journey.** If two features in different journeys are about the same domain (e.g., "as an admin, update a user password" and "as a user, change my username"), they share one prefix (USRMGMT), not two (ADMN, USR). The prefix names the category of capability, not who does it.

**Applying the invariants to FAM/PKG:**

Under invariant 3, PKG-10 ("as a family author, author a new family") and PKG-01 ("as a package author, declare what capabilities it provides") are both about packaging — different journeys, same domain category. The prefix should stem from the domain: **PKG** (or PACKAGING). The journey distinction lives in the user stories and the journey grouping (invariant 1), not in the prefix.

Current state violates invariant 3: FAM is prefixed by persona (family author), PKG by domain (packages). Under the invariant, both should be PKG (the domain) with different journeys within it.

Also flagging other series against invariant 3:

| Current prefix | Journey | Domain category | Consistent? |
|---------------|---------|----------------|-------------|
| BST | Bootstrap | Kit bootstrapping | Yes — domain is bootstrap |
| FAM | Family authoring | Packaging | **No — should be PKG** |
| PKG | Packages | Packaging | Yes |
| DIS | Discover & choose | Packaging (discovery phase) | Borderline — could be PKG |
| CNSLT | Consultation | Packaging (consultation phase) | Borderline — could be PKG |
| BLD | Build | Packaging (build phase) | Borderline — could be PKG |
| AWV | Assemble, wire & verify | Packaging (assembly phase) | Borderline — could be PKG |
| CNTXT | Contexts & sources | Kernel model | Yes — domain is contexts |
| SET | The house spec | Conformance | Yes — domain is setup contracts |
| OPS | Operate & evolve | Lifecycle | Yes — domain is operations |
| USR | Per-user growth | User memory | Yes — domain is per-user |
| OBS | Cross-cutting | Observability | Yes — domain is observability |

The FAM violation is clear. DIS/CNSLT/BLD/AWV are borderline — they're all phases of the packaging journey. Under invariant 2 (slow-growing, finite), having one prefix per packaging phase may be too many. But they do name different journey phases (discover vs consult vs build vs assemble), not different personas, so they don't violate invariant 3.

**PO correction on invariant 3 — the prefix is a product-level FEATURE CATEGORY, not a journey phase:**

The analogy was: "user management" is one umbrella — it covers admin resetting passwords AND users changing usernames, across different personas and journeys. You'd never split "discover users" into a DISC prefix separate from USRMGMT. Similarly, "package management" is one umbrella — discover, author, publish, resolve, install, verify, evolve are all phases of the same feature category. You'd never hear of a "package management" product that puts "discover packages" under a separate "discoverability" category.

**Re-applying invariant 3 correctly:**

The prefix names a product-level feature category — a cross-persona, cross-journey umbrella. Packaging is one such umbrella. Everything that is about packages (authoring, discovering, consulting about, building, assembling, wiring, verifying, publishing, evolving) belongs under one prefix.

| Current prefix | What it covers | Product feature category |
|---------------|---------------|------------------------|
| FAM | Author metapackages, publish, friction | **Packaging** |
| PKG | Package shape, contracts, composability, hooks | **Packaging** |
| DIS | Browse, discover, specificity, alternatives | **Packaging** (discover phase) |
| CNSLT | Conversational discovery with Smith | **Packaging** (consultation phase) |
| BLD | Target, recommend, author, verify | **Packaging** (build phase) |
| AWV | Resolve, install, connect, verify | **Packaging** (assembly phase) |
| OPS | Evolve, remove, upgrade, rollback | **Packaging** (lifecycle phase) |
| BST | Train Smith, seed catalogue | **Bootstrap** |
| CNTXT | Contexts, sources, membership, access | **Contexts & access** |
| SET | What a conforming setup contains | **Conformance** |
| USR | Per-user friction memory, grounded advice | **Per-user growth** |
| OBS | Observability | **Observability** |

Under this reading, FAM + PKG + DIS + CNSLT + BLD + AWV + OPS are ALL packaging — 7 prefixes for one feature category. That's 50 features (out of 70) under one umbrella, split into 7 prefixes. The journeys within that umbrella are valuable for readability (invariant 1), but the PREFIX should be one: PKG (or PACKAGING).

The remaining categories — BST (4), CNTXT (5), SET (11), USR (3), OBS (1) — are genuinely different feature categories.

**Revised assessment:**

| Proposed prefix | Features | Journeys within |
|----------------|----------|-----------------|
| **PKG** | 50 | Family authoring, package shape/contracts, discover, consult, build, assemble/wire/verify, operate/evolve |
| **BST** | 4 | Bootstrap |
| **CNTXT** | 5 | Contexts & sources |
| **SET** | 11 | Conformance / house spec |
| **USR** | 3 | Per-user growth |
| **OBS** | 1 | Observability |

6 prefixes instead of 12. Each is a genuine product-level feature category. Journeys still exist within each for readability (invariant 1). The prefix count is finite and stable (invariant 2). The prefix stems from the domain category, not the persona or journey phase (invariant 3).

**Status:** Presentation fixed — requirements landscape slide now notes that FAM through OPS are all packaging under D-01, and prefixes may consolidate. Design author flagged for prefix reorganization.

---

## Comment 9: "What does SET stand for? And what does USR stand for?"

**Source:** Slide 9 (Requirements Landscape) — the series table lists SET and USR as prefixes.

**PO concern:** The prefix abbreviations are unclear.

**Investigation:**

- **SET** = "Setup" — from the journey name "What a conforming setup contains" (the "house spec"). features.md section heading: "These features define the structural rules that every conforming setup must satisfy." CONF-01 through CONF-11 define the invariants of a conforming setup (at least 1 agent, at least 1 loop, tracker-agnostic, workflow per loop, HITL gates, actor parity, etc.).

- **USR** = "User" — from the journey name "Smith gets better at serving you." features.md: LEARN-01 through LEARN-03 cover per-user friction memory, grounded advice citing prior runs, and catalogue growth from end-user runs.

Neither abbreviation is self-evident from the prefix alone. SET could be mistaken for "settings." USR could be mistaken for "user management."

**Applying Comment 7's invariant 3 (prefix = product-level feature category):**

SET (11 features about what a conforming setup must contain) is really about **conformance** — the house spec IS the base contract. These features define the contract's structural assertions. Under invariant 3, this could be CONF (conformance) rather than SET (setup).

USR (3 features about per-user friction memory and catalogue growth) — this is genuinely its own category. But "USR" is ambiguous. Something like LEARN or MEMORY would be clearer, since the domain is Smith's learning/memory across runs, not user management.

**Status:** Presentation fixed — SET and USR spelled out in the requirements table ("setup conformance" and "user/Smith memory"). Design author flagged for clearer prefix names. Now renamed: SET→CONF, USR→LEARN across all spec artifacts and presentation.

---

## Comment 10: "The design pins the machinery, not the catalogue: The verification engine — is this accurate? Do we pin the engine in this doc?"

**Source:** Presentation slide "3.4 (continued) — Machinery vs Content (D-19)" — states the verification engine is pinned.

**PO concern:** Does design.md actually pin the verification engine, or does it just say it will be pinned?

**Investigation:**

What design.md says about what's pinned:

- §4.4 (line 454-463): "What the design pins is the **format and the engine** so any contract authored later is verifiable the same way." Then it describes what the engine does: "it runs the base contract + each applicable per-type contract, emits one `[Output]` record per assertion, and gates readiness on every `MUST` passing — citing the specific violated assertion IDs on failure."
- §3.4 (line 197): "what must be pinned now... is the **discover→load→execute-by-type path** (§4.4) and the format — both fixed here."
- D-19 (line 1109): "What it does pin... is the engine's **discover→load→execute-by-type** path and the contract format."

**What "pinned" means here:** The design doc describes the engine's RESPONSIBILITY (§4.4 line 445: "verify a candidate setup against the base contract + every applicable per-type contract, report pass/fail per contract with specific violations, and gate readiness") and its BEHAVIOR (discover contracts by type → load → run → emit `[Output]` per assertion → gate readiness). It also pins the engine's **path** (discover→load→execute-by-type).

**What is NOT pinned:** The engine's implementation — how it discovers contracts, what data structures it uses, what language it's written in. The design fixes the contract (what the engine does and the path it follows) but not the implementation.

**Verdict:** The presentation's claim is accurate. The design doc does pin the verification engine — not its implementation, but its responsibility, behavior, and the discover→load→execute-by-type path. The word "pins" means "fixes the contract the engine must satisfy" — the same discipline the design applies to everything else (machinery vs content). The engine is described concretely enough in §4.4 that a story can build against it.

**Status:** No fix needed — the presentation accurately reflects the design doc.

---

## Comment 11: "'Hire an actor into a context' is inaccurate — hire is from BotMinter, it shouldn't be used in Loopsmith"

**Source:** Presentation uses "hire" in multiple slides (binding definition, component overview, glossary). The design doc also uses "hire."

**PO concern:** "Hire" is BotMinter vocabulary (`bm hire`). Loopsmith should have its own terminology.

**Investigation:**

`grep -rn "\bhire\b"` across design.md, idea-honing.md, features.md: 3 hits, all in the binding definition context:
- design.md line 226: "binding — instance-time: **hire** an actor into a context"
- design.md line 488: "at instance time, hire an actor into a context"
- idea-honing.md line 47: "binding — instance-time composition: **hire** an actor"

features.md: zero hits — the features never use "hire."

**Root cause:** "Hire" is BotMinter's `bm hire` command vocabulary. It leaked into the design doc's binding definition. The concept is already called a "binding" — the verb should be "bind," not "hire."

**Resolution:** Replaced "hire" → "bind" across design.md (2 instances), idea-honing.md (1 instance), and presentation (2 instances). Zero "hire" remaining.

**Status:** Fixed across all artifacts.

---

## Comment 12: "The Smith slide doesn't distinguish between the learning/apprenticing phase (bootstrap) and the family authoring phase. Also slide 23 is overflowing."

**Source:** Presentation slide 24 (3.6 — Smith and the Packaging Skills) mixes bootstrap (BST — Smith learning craft) with steady-state operation (build/furnish/maintain loops, family authoring) without making the phase boundary clear. Also the slide overflows.

**PO concern:** The slide reads as one flat description. A reader can't tell when Smith is being trained vs when Smith is doing the work. The bootstrap-to-steady-state transition is a key lifecycle moment that the slide flattens.

**Investigation:**

design.md §3.6 (lines 239-251) describes Smith in two phases but runs them together in one paragraph:
- Lines 239-241: "ships credentialed on the structural model and apprenticing on craft, growing the craft by authoring skill-packages with the human" — this is the BOOTSTRAP phase (BST-01 through BST-04)
- Lines 246-251: "Smith's repertoire is three peer loops... build, furnish, maintain" — this is STEADY-STATE operation after bootstrap completes

The design doc itself doesn't make the phase boundary explicit within §3.6 — it flows from one to the other. But the BST series (§2) clearly separates bootstrap as a distinct lifecycle phase that COMPLETES (BST-04: "re-packaged as a conforming agent template").

**Root cause:** The presentation mirrored the design doc's §3.6 structure, which blends the two phases. The design doc IS clear about the phases (BST series + §3.9 describes the bootstrap-to-steady-state transition), but §3.6 itself presents them as one continuous description.

**Resolution:** Presentation split into two slides: "Smith: Bootstrap Phase (BST)" covering the apprenticeship and re-packaging, and "Smith: Steady-State Operation" covering the three loops and re-entrant build. The packaging boundary slide (page 25) also split to fix overflow. Rebuilt.

**Status:** Presentation fixed. No design doc change needed — the phase distinction exists in the design doc (BST series), just not within §3.6's prose.

---

## Comment 13: "Does this confusion between apprenticeship and steady state exist also in the spec artifacts?"

**Source:** Follows from Comment 12. PO wants to know if design.md §3.6 blends the two phases.

**Investigation:**

Searched all spec artifacts for how they present the bootstrap-to-steady-state boundary.

**design.md — the blending happens in two places:**

1. **§1 (lines 50-53):** "Smith is the single irreducible seed: credentialed on the kit's structure from day one, apprenticing on craft over time, and ultimately re-packaged as a conforming agent template so that the same Smith that bootstraps the kit also onboards end users." — This is one sentence that spans from bootstrap (seed, credentialed, apprenticing) through the transition (re-packaged) to steady state (onboards end users). No phase break.

2. **§3.6 (lines 238-251):** One continuous paragraph: "ships credentialed on the structural model and apprenticing on craft" (bootstrap) → the packaging method → "repertoire is three peer loops" (steady state) → "At the end of bootstrap, Smith is re-packaged" (transition). The transition sentence comes AFTER the steady-state description, which is backwards chronologically.

**design.md — where the phases ARE clearly separated:**

3. **§2 (line 76):** The BST series is clearly labeled "Bootstrap" with its own summary.
4. **§7 (lines 782-792):** AC-01 and AC-02 are labeled "Bootstrap — the apprenticeship" as a distinct AC group.
5. **§3.9 (line 287-291):** "run #1... heavy authoring against an empty catalogue; run #2... reuses run #1" — describes the bootstrap-to-steady-state transition through the flywheel.

**features.md — clear separation:**

6. **Line 18-27:** "Bootstrap — The Apprenticeship (4 features)" is its own section with a clear narrative: "Before any catalogue, family, or blueprint exists, the developer runs Smith against real systems." BST-04 is explicitly the transition: "the bootstrap completes when Smith becomes a package."

**idea-honing.md — blended in Q-11:**

7. **Lines 247-258:** Same pattern as design.md §3.6 — Smith's repertoire (build/furnish/maintain) is described alongside re-packaging ("later re-packaged as a conforming agent template") without a clear phase break.

**Verdict:** The confusion exists in the design doc in two places (§1 and §3.6) and in idea-honing (Q-11). In all three cases, the apprenticeship and steady-state are described in one continuous paragraph with the transition buried mid-sentence. The features.md and the ACs (§7) have the clean separation.

The root cause is that §3.6 describes Smith's CAPABILITIES (what it ships with, what it can do) rather than Smith's LIFECYCLE (what happens first, what happens after). The capabilities span both phases, so the description naturally blends them.

**Recommended design.md fix:** Split §3.6 into two paragraphs with an explicit phase boundary. Something like:

> **Bootstrap (BST).** Smith ships credentialed on the structural model and apprenticing on craft. The developer runs Smith against real systems with an empty catalogue, producing the first content through supervised packaging runs (BST-01/02/03). At the end of bootstrap, Smith is re-packaged as a conforming agent template (BST-04).
>
> **Steady state.** After bootstrap, Smith runs the packaging skills across three peer loops...

**Status:** Fixed across all artifacts. design.md §1 (lines 50-53), §3.6 (lines 238-251), and idea-honing.md Q-11 (lines 242-259) all split into explicit **Bootstrap** and **Steady state** paragraphs with a clear phase boundary. Presentation was already fixed in Comment 12.

---

## Comment 14: "Onboarding is the system context's bootstrap phase" is confusing — this is steady-state operation

**Source:** Presentation slide 27 (3.8 — Self-Hosting: The System Context) — bullet says "Onboarding is the system context's bootstrap phase — a loop whose first item is 'create the first context'"

**PO concern:** After establishing the bootstrap (BST) vs steady-state distinction in Comments 12-13, calling the system context's onboarding a "bootstrap phase" is confusing. The onboarding loop (creating a user's first context, fitting a blueprint to a user) is part of steady-state operation — Smith's furnish loop — not part of the BST apprenticeship.

**Investigation:**

`grep -n -i "onboard"` across design.md and idea-honing.md reveals two uses of "onboarding":

1. **System context onboarding (§3.8, line 283):** "Onboarding is the system context's bootstrap phase — a loop whose first item is 'create the first context' — which pivots into steady state"
2. **Smith's furnish loop (§3.6, line 251):** "furnish (onboard: fit a built blueprint to a user)"

And the definitive statement from **idea-honing.md line 471:** "The first product's onboarding (Q-14/Q-15) is the **furnish loop** with a console front-end — not a separate mechanism."

This is unambiguous: onboarding IS the furnish loop, which IS steady-state operation. The system context's firmware runs onboarding as part of Smith's steady-state repertoire (build/furnish/maintain), not as part of the BST apprenticeship.

**Root cause:** design.md §3.8 uses "bootstrap phase" to describe the system context's initial onboarding loop. This conflates two meanings of "bootstrap": (1) BST — Smith learning craft through supervised runs, and (2) "initial setup" in the colloquial sense. After Comments 12-13 established the BST/steady-state boundary, calling ANY non-BST activity a "bootstrap phase" is confusing. The system context's onboarding loop is the furnish loop's firmware — it creates the first context during steady state, then pivots Smith onto the user's loop.

**Resolution:** Fix design.md §3.8 and the presentation to remove "bootstrap phase" from the system context's onboarding description. The onboarding loop is firmware that runs during steady state — it's the furnish loop's system-level entry point.

**Status:** Fixed — but superseded by Comment 15 (deeper structural fix).

---

## Comment 15: Smith's modes must be organized by persona, not just lifecycle phase

**Source:** Follows from Comments 12-14. The root confusion is that Smith's modes are never tied to the personas who use them.

**PO's model — four personas, three modes:**

| Persona | Smith mode | What happens |
|---------|-----------|-------------|
| **P1** (Loopsmith Developer) | **Training** | BST — the apprenticeship. Smith learns craft through supervised runs. |
| **P2** (Family Author) + **P3** (Package Author) | **Building** | The build loop — author families, blueprints, templates, packages. |
| **P4** (End User) | **User** | Furnish (onboarding — fit a blueprint to a user) + Maintain (day-to-day steady state when needed). |

The current design splits Smith into "Bootstrap" and "Steady state" — a temporal axis. But this lumps the build loop (P2/P3) with furnish/maintain (P4) under one "steady state" umbrella, obscuring who does what. The persona is the missing organizing axis.

**PO directive:** These distinctions must be VERY clear in the design. Any confusion must be refactored and fixed now.

**Investigation:**

Searched all spec artifacts for how Smith's modes are described. Found the confusion exists in six locations:

1. **design.md §1 (lines 50-54):** "during bootstrap... After bootstrap, the same Smith runs the packaging skills in steady state" — temporal axis only, no persona context.
2. **design.md §3.6 (lines 239-256):** Split into "Bootstrap (BST)" and "Steady state" — lumps build (P2/P3) with furnish/maintain (P4) under one "steady state" umbrella.
3. **design.md §3.8 (line 283):** Called onboarding "the system context's bootstrap phase" — confusing because onboarding is P4 user mode, not BST.
4. **design.md §4.7 (lines 508-511):** System context component says "pivot into steady state" without persona context.
5. **idea-honing.md Q-11 (lines 247-262):** Same "Bootstrap"/"Steady state" structure with all three loops lumped.
6. **idea-honing.md Q-22 (lines 462-472):** Describes loops as "build/furnish/maintain" without persona context.

**Root cause:** The design organized Smith's phases on a **temporal axis** (bootstrap → steady state) instead of a **persona axis** (who uses Smith and for what). The temporal axis lumps P2/P3's build loop with P4's furnish/maintain under one umbrella, making it impossible to tell that onboarding is P4's activity, not BST's.

**Resolution:** Refactored all six locations plus the presentation to use three persona-driven modes:

| Persona | Smith mode | Loops |
|---------|-----------|-------|
| P1 (Loopsmith Developer) | **Training** | BST — the apprenticeship |
| P2 (Family Author) + P3 (Package Author) | **Building** | build loop |
| P4 (End User) | **User** | furnish (onboarding) + maintain (day-to-day) |

The temporal axis is implied: training comes first (P1), then building and user modes run as the catalogue grows. Build is re-entrant across modes: P4 hits a gap → Smith drops into building mode → produces the missing piece → returns to user mode.

**Files changed:**
- design.md §1 (overview paragraph)
- design.md §3.6 (three mode paragraphs replace two phase paragraphs)
- design.md §3.8 ("Onboarding is user mode (P4)")
- design.md §4.7 (system context component)
- idea-honing.md Q-11 (three mode paragraphs)
- idea-honing.md Q-22 (persona-driven mode framing)
- Presentation: Smith slides restructured into three-mode layout with persona table
- Presentation: D-04 decision updated to reflect persona-driven modes

**Status:** Fixed across all artifacts. PDF rebuilt.

---

## Comment 16: Self-hosting can't "do" onboarding — the kit doesn't run anything

**Source:** Presentation slide 27 (3.8 — Self-Hosting) and design.md §3.8.

**PO concern:** The kit is a generator — it produces setups but doesn't host or run them. So how can the system context "do" onboarding? Onboarding is a runtime activity that happens AFTER a family and blueprint exist. The onboarding described in §3.8 is actually user mode (P4) that any conforming blueprint can offer through Smith. The system context provides the infrastructure floor; the furnish loop (shipped by the blueprint via Smith OOTB) does the actual onboarding.

**Investigation:**

**PO correction on Comment 14/15's fix:** The system context is NOT a blueprint-level capability. The kit DOES ship Smith's user mode and the system context firmware. The analogy is Anaconda in Fedora: Anaconda is shipped by the distribution (the kit), works across all families (Workstation, Server, CoreOS), and adapts to whatever the family provides (GUI or text mode). Similarly, Smith (user mode) is kit-level machinery that any conforming blueprint inherits and that adapts to the blueprint's capabilities.

**What this means:**
- The kit ships: contracts + grammar + skills + Smith (including user mode = furnish/maintain + system context)
- Any conforming blueprint inherits Smith's user mode from the kit
- Onboarding RUNS once a family and blueprint exist (P4 picks a blueprint), but the onboarding MACHINERY comes from the kit
- Smith adapts to the blueprint's capabilities (like Anaconda adapts to the family)

**Root cause of my error:** I conflated "the kit doesn't run anything" with "the kit doesn't ship the onboarding machinery." The kit is a generator — it doesn't run setups. But it DOES ship the installer/onboarder (Smith user mode + system context firmware). The machinery ships with the kit; the activation happens at the blueprint level.

**Resolution:** Revert "blueprint-level capability" language. The system context is kit-level infrastructure that any conforming blueprint inherits. The onboarding loop is Smith's furnish entry point — kit machinery that runs when P4 picks a blueprint.

**Status:** Fixed — then corrected further by Comment 17.

---

## Comment 17: System context conflates the loop (control plane) with the driver (daemon/event-bus)

**Source:** design.md §3.8 says the system context is "the control plane — daemon + event-bus."

**PO correction:** The loop IS the control plane (steps, events, gates). The daemon and event-bus are DRIVER concerns — they are not shipped by the kit. Smith can bootstrap as a skill in any LLM/coding agent, then evaluate what driver is available (Claude Code? something else?) and adapt. The kit ships the system loop (onboarding + repair steps); the driver that executes it is determined at runtime.

Don't lock in daemon/event-bus decisions at design time. The decision is: the kit ships the loop. The driver is a runtime concern.

**Investigation:**

`grep -n "daemon\|event.bus"` across design.md: multiple hits tying "daemon + event-bus" to the system context as if they're kit-level decisions. These are driver-level concerns that should not be locked in.

**Resolution:** Remove "daemon + event-bus" from the system context description. The system context is a control-plane loop (steps for onboarding + repair). The driver is determined at runtime by Smith based on what's available.

**Status:** Fixed.

---

## Comment 18: "Self-hosting" is the wrong term — the kit doesn't host anything

**Source:** design.md §3.8 heading "Self-hosting: the system context and the floor," plus D-07, AC-02, appendix.

**PO concern:** If the kit ships loops but not drivers, "self-hosting" implies the kit runs/hosts itself, which it doesn't. The actual property is that the model is **self-describing** — its own operations (onboarding, repair, formation) and its own creator (Smith) are expressible within the five-noun model. No mechanism outside the model is needed to describe them.

**Investigation:**

Seven hits for "self-hosting" in design.md:
1. §3.8 heading (line 286): "Self-hosting: the system context and the floor"
2. §3.8 body (line 288): "The kit is self-hosting"
3. AC-02 (line 811): "Smith is re-expressed as an agent template (self-hosting)"
4. D-07 title (line 1028): "the kit is self-hosting"
5. D-07 rejected alt (line 1031): "a non-self-hosting bootstrap path"
6. D-07 rationale (line 1033): "self-hosting keeps the floor inside the model"
7. Appendix (line 1310): "Self-hosting (reconcile/onboarding/formation all expressible as loops)"

Six hits in the presentation. Zero in idea-honing or other spec artifacts.

All seven instances describe the same property: the model is closed under its own operations and entities. "Self-describing" captures this without implying the kit runs itself.

**Resolution:** Renamed "self-hosting" → "self-describing" across all instances in design.md and the presentation.

**Status:** Fixed.

---

## Comment 19: The jump from "how the catalogue comes to exist" to "the transpiler" is confusing — when does the transpiler run? Is it a P4 feature?

**Source:** Presentation slides 3.9 → 3.10. After explaining how the catalogue grows (the flywheel), the next section jumps straight into BPMN and the transpiler with no bridge explaining WHERE in the lifecycle the transpiler fits and WHO triggers it.

**PO concern:** "Made me confused af." The transpiler appears out of nowhere. When does it run? Is it an n-0 thing? Is it a P4 feature? What's the relationship between the catalogue (which the previous section just explained) and the transpiler?

**Investigation:**

Where the transpiler fits in the lifecycle:

1. **Building mode (P2/P3):** Author loop templates, packages, etc. Catalogue grows.
2. **User mode (P4) — furnish:** Pick a blueprint → resolve → install → **assemble** → wire → verify.
3. **The transpiler runs during assembly** (step 2): it takes the loop model (tracker-agnostic, expressed in the authoring face) and materializes it with concrete provider bindings into a driver-native artifact (e.g., `ralph.yml`).
4. **The driver then executes** the transpiled artifact at n-0.

Evidence:
- design.md line 409: "bound at transpile" — the concrete tracker provider is resolved and bound during transpile
- AC-28 (line 956): "transpiles to a runnable ralph.yml" — happens as part of assembly
- design.md line 328: "the transpiler is the bridge" between the BPMN IR and the driver

The transpiler is **kit machinery** — it runs behind the `smith-agent` boundary during assembly. P4 never sees BPMN or the transpiler. They work with the authoring face (five nouns + grammar); the transpiler is internal plumbing.

**Root cause of the presentation confusion:** §3.9 ends with the flywheel (catalogue growth), and §3.10 jumps into "BPMN 2.0 is the internal loop representation" with no bridge explaining:
- The catalogue produces loop templates (among other packages)
- When a setup is assembled, those templates need to become runnable artifacts
- The transpiler does this during assembly, behind `smith-agent`
- This is kit machinery, not a user-facing concept

The design doc has a partial bridge (§3.10 opens with "Everything above is the authoring face"), but the presentation distills it into bullets and loses the connection.

**Resolution:** Added a bridge slide between §3.9 and §3.10 in the presentation that positions the transpiler in the lifecycle: it runs during assembly (user mode / building mode), behind `smith-agent`, materializing loop templates into driver-native artifacts. P4 never touches it.

**Status:** Fixed — then superseded by Comment 20 (deeper structural issue).

---

## Comment 20: The transpiler section conflates kit-level concepts with ralph-specific implementation

**Source:** Presentation slide 33 (3.10 continued — The Transpiler). Also design.md §3.10.

**PO concern:** The slide presents ralph-specific internals (ralph.yml, prose transformers, board-scanner) as if they're kit-level concepts. Not every blueprint has a board-scanner. The transpiler as described is too tightly coupled to the ralph driver. "This whole transpiler is messing things up in the design."

**Investigation:**

The conflation runs deep. §3.10 presented the ralph-specific chain (ralph.yml → prose transformers → board-scanner) as the universal kit-level process. "Board-scanner" appeared 8 times in design.md, including D-17's title.

**What's kit-level vs ralph-specific:**

| Kit-level concept | Ralph-specific implementation |
|---|---|
| Materialization: loop model → driver-native artifact | field-mapping + two prose transformers → ralph.yml |
| Dispatcher: uses status→step map to fire steps | Board-scanner: poll-based status reader |
| Driver determined at runtime by Smith (D-23) | ralph-orchestrator is MVP primary |

**Changes made:**

1. **design.md §3.10:** Restructured into three layers:
   - "Materialization" — kit-level: assembler-shaped, per-driver backend
   - "Status→event dispatch" — kit-level: workflow declares map, driver's dispatcher uses it
   - "Ralph driver backend (MVP primary)" — clearly labeled ralph-specific section

2. **D-17:** Title changed from "board-scanner" to "dispatcher." Body uses "dispatcher" throughout.

3. **"board-scanner" audit:** Replaced with "dispatcher" in 7 of 8 occurrences. The one remaining is in the ralph-specific section, correctly labeled: "Ralph's dispatcher is its board-scanner."

4. **Other body references** (§5.5, AC-31, testing, security): all "board-scanner" → "dispatcher."

5. **Presentation:** Split old slide 33 into two slides:
   - "Materialization and Dispatch (Kit-Level)" — generic concepts
   - "Ralph Driver Backend (MVP Primary)" — ralph-specific details
   - D-17 residual slide uses "dispatcher" instead of "board-scanner"

6. **Mermaid diagram:** Changed from "Runtime (Ralph)" to "Runtime (driver-determined, D-23)" and "transpiler: field-map" to "materialization (per-driver backend)."

**Status:** Fixed.

---

## Comment 21: "The reduction (authoring to BPMN) is reversible and total" — no bridge, jargon dropped without context

**Source:** Presentation slide 32 (3.10 continued — The BPMN Reduction). Also design.md §3.10 and §5.7.

**PO concern:** The slide drops the term "reduction" with no explanation of what it means or why authoring → BPMN is called a "reduction." There's no bridge from the previous slide.

**Investigation:**

`grep -n "reduction" design.md`:

- Line 345: "The reduction (authoring ⇄ BPMN is reversible and total)"
- Line 656: §5.7 heading "Reduction (authoring ⇄ BPMN), per element"
- Line 669: "The mapping is total and reversible"

The design doc uses "reduction" as a technical term for "the mapping between the authoring face and the BPMN representation." But it never defines what "reduction" means in this context. A reader encountering it cold has no idea that it means "two representations of the same data, mechanically convertible in both directions."

**Root cause:** The term "reduction" is borrowed from formal language theory (a reduction maps one representation to another). The design doc uses it without definition. The presentation copies it without bridging.

**Is this a design doc flaw?** Borderline. The design doc's audience (the architect) likely understands "reduction" in context. But:
1. §3.10 uses "reduction" without defining it on first use (line 345)
2. §5.7 uses it as a section heading (line 656) — clearer because it's followed by a per-element mapping table
3. The concept is simple (same data, two representations, reversible mapping) — calling it a "reduction" obscures rather than clarifies

**Resolution:**
- **Presentation:** Rewrote slide to explain the concept in plain terms: "same data in two representations, mechanical and reversible mapping." Dropped the term "reduction." Added a table showing the mapping.
- **Design doc:** The term "reduction" in §3.10 and §5.7 should be clarified. Adding a brief definition on first use.

**Status:** Presentation fixed. Design doc needs the PD-19 framing embedded — see Comment 22.

---

## Comment 22 (revised): PD-19's clear framing lives in transient design-notes, not in the authoritative design.md

**Source:** design.md §3.10 uses "two faces," "reduction," "authoring face," "representation face" — all borrowed from PD-19 (design-notes.md lines 266-295) without carrying over the definitions.

**PO concern:** design-notes.md is transient. The authoritative design.md must be self-contained. A reader shouldn't need to find PD-19 to understand what "two faces" means or why "a step reduces to a task + gateway."

**Investigation:**

PD-19 defines the architecture clearly:
- **Face A** = "the agentic-friendly format (loops · hats · actors · trackers · sources). The high-level language and the only thing users ever see."
- **Face B** = "BPMN 2.0 — the internal canonical representation: the on-disk/interchange data format, the validation substrate, the library-reuse substrate."
- **The analogy** (line 279): "we're building Java + bytecode; BPMN is the assembly + kernel."
- **The reduction** (line 287): "Face A ⇄ Face B is reversible and total" — hat ⇄ task + virtual gateway.

design.md §3.10 has NONE of this context. It says "two faces and one runtime" and "the reduction is reversible and total" — jargon without the definitions, the analogy, or the rationale that make PD-19 clear.

**Resolution:** Embed PD-19's framing into design.md §3.10: the Face A / Face B definitions, the Java/bytecode analogy, and the explicit reduction rationale. The authoritative doc must stand alone.

**Status:** Fixed. PD-19's framing (Java/bytecode analogy, Face A = authoring vocabulary / Face B = BPMN on disk, reversible mapping) embedded into design.md §3.10, §5.7, and D-16. Jargon replaced: "authoring face" → "authoring vocabulary," "representation face" → "BPMN (the canonical form)," "reduction" → "mapping." D-16 now carries the Java/bytecode analogy and the Face A/Face B definitions from PD-19, not just a PD-19 tag.

---

## Comment 22: "Authoring face" is a UX vocabulary, not a file format — the jargon is creating confusion

**Source:** design.md §3.10 says "two representations of the same data — not two stored copies." §5.7 says "the authoring face is the edited form, BPMN is its canonical serialization."

**PO concern:** "Authoring face" has been used throughout the design to mean "the UX/vocabulary users work with" (five nouns + grammar). Saying "not two stored copies" implies the authoring face is ALSO a file format that COULD be stored. It isn't. There's one UX vocabulary (five nouns), one storage format (BPMN on disk), and one runtime artifact (driver-native). The "faces" jargon obscures this simple architecture.

**Investigation:** Verified against PD-19 in design-notes.md. The PO's instinct that "authoring face = UX vocabulary, not a stored format" was checked against the canonical model — and the design is actually consistent (Face A = authoring vocabulary / Face B = BPMN-on-disk; only BPMN is stored; the authoring vocabulary is a reversible projection over it, per the Java/bytecode framing). The real defect was that PD-19's substance lived only in the transient design-notes.md and was not embedded in design.md.

**Resolution:** Fixed. design.md §3.10/§5.7/D-16 now carry the Java/bytecode framing explicitly: the authoring vocabulary (five nouns + grammar) is the high-level language; **only BPMN 2.0 is stored on disk**; the authoring vocabulary is a reversible projection over it. The "two stored copies" phrasing was removed. The mapping between the two is mechanical, reversible, and total.

---

## Comment 23: Reconciliation audit — find anything dropped, conflicting, or silently superseded across 10 days of planning

**Source:** PO directive — "this planning has been going for 10 days, a lot of ideas flew out there; I may have forgotten earlier ideas I wanted in the MVP, later discussion didn't mention them, so you assumed superseded. Find anything conflicting / left out / superseded by design.md and revisit."

**Investigation:** Swept design-notes.md (PD-01→PD-22), the deleted reviewed design `a14d9c1` (1164 lines, ~36 ACs — replaced in the BPMN/pixi rebuild), idea-honing.md (Q-01→Q-22), and concept-reconciliation.md against the current design.md. Key structural fact: the current design.md is a **rebuild** (RECONCILIATION-NEEDED.md), which is exactly where earlier ideas could silently drop.

**Master tension (resolved by PO):** idea-honing draws a hard line — #1 builds *the kit*; the kit ships *no runtime*; the console/Loop Studio/daemon/cross-context digest are the **first product** (R-06/R-07), "not what #1 designs." design.md never states this boundary (no mention of `first product`, `R-06/R-07`, `no runtime`). **PO resolution:** (1) Kit ≠ product — the kit ships no runtime, like an IDE doesn't ship the apps it builds. (2) Building/testing ≠ shipping — #1 **will build and test two families** (a `developer` blueprint onboarded via **web console**; a `simple-assistant` via **CLI**) specifically to validate the kit's central feature, **Smith**, against two genuinely different onboarding surfaces. The runtimes produced are the acceptance-demo *target*, not the kit's deliverable.

**Findings ledger:**

| # | Item | design.md now | Earlier intent | Verdict |
|---|------|---------------|----------------|---------|
| A | Kit ↔ first-product boundary | Not stated; no R-06/R-07 | idea-honing Q-01/19 | Conflict/omission |
| B | Four falsifiable claims D-i…D-iv | Only D-i (tracker swap) in §1 | Q-02: "all of D-i…D-iv required"; old §1.4 | Dropped |
| C | "MVP trims cardinality, never concepts" | Absent | Q-02/Q-04; old §1.5 | Dropped |
| D | PD-04 personal/professional/digest hosting | Generic membership only (CONF-09, D-08, AC-19/23) | Q-05; PO: "04 is in" | Gap (hangs off A) |
| E | PD-19 three-faces-of-a-behavioral-unit + "two payloads are NOT processes" | Absent (the "three faces" hits = contract faces) | PD-19 | Gap |
| F | Fork chain / cross-identity PR fix (Q-07) | Absent | Q-07 | Gap (confirm scope) |
| G | PD-02 steady-state = "digest + shepherding" | daemon/event-bus retired (D-23); digest/shepherd unaddressed | PD-02 | Silently reworked → folds into A |

**Already covered (no action):** D-i swap → AC-29; D-iii touchpoint/actor parity → §3.2 + AC-22; D-iv membership≠access → AC-19; friction/learn → AC-13/26; PD-03 firmware floor → §3.8; port authority → AC-19; home source → AC-20.

**Resolution:** IN PROGRESS.
- **E — DROPPED (non-gap).** PO Socratic check caught an over-flag: design.md scopes BPMN to *loops* only (§3.10/§5.7/D-16), never claims "all packages are workflows" — so PD-19's "two payloads are NOT processes" bounds an overclaim that doesn't exist. And the "three faces of a behavioral unit" substance is already in §3.10 (prose = content / BPMN = wiring / hat = LLM session), sensibly *not* reusing the "three faces" label (taken by the contract). No edit needed.
- **F — DONE.** §3.3: `github-repo` source type-specific ops (`fork`/`branch`/`PR`/`issues`) + `upstream → fork` chain for cross-identity contribution; concrete operation set + fork-chain mechanics deferred to a story-level package-building task (run #1 authors `github-repo`).
- **G — DONE.** Shepherding added to §3.6 as a "rough idea — to be sharpened in a story" note (touchpoint-based detect→root-cause→escalate; not anomaly-detection; mechanism story-deferred). Stale PD-02 citations dropped from §3.8 + D-07; PD-02 marked superseded-in-part in design-notes (daemon/event-bus retired by D-23).
- **A — DONE.** §1: "Kit, not product — and building is not shipping" — kit ships no runtime, but #1 builds+tests two products (developer/web-console, simple-assistant/CLI) to validate Smith; developer product named as the personal/professional cross-context digest setup; R-06/R-07 linked.
- **B — DONE.** §1: four falsifiable claims table (D-i…D-iv), restoring D-ii ("autonomy comes from context, not config") which was absent; + naming note distinguishing D-i…D-iv (hypotheses) from D-01…D-23 (decisions).
- **C — DONE.** §2: "MVP trims cardinality, never concepts" restored.
- **D — DONE.** §3.2: membership-not-elevation enables cross-context reach (one Smith spanning personal+professional); "tracker board is a shared surface, not the context boundary" (board-visibility ≠ membership). The personal/professional instantiation also landed in the §1 A-paragraph.
- **Presentation + PDF — DONE.** Synced 5 slides: "What Is Loopsmith?" (no-runtime line), "MVP Thesis" (build≠ship + web-console/CLI + personal/professional), NEW "Four Falsifiable Claims" slide, "MVP Cut" (cardinality), "Kernel Invariants" (membership-not-elevation + board≠boundary). Also fixed the stale Section-1 review-box bullet that still asserted "the kit IS a product" → now states the resolved kit≠product / build≠ship framing. PDF (101pp) + HTML rebuilt; pages 5, 8, 9, 10, 14, 19 verified against the actual render — no overflow.

**Comment 23 status: RESOLVED.**

---

## Comment 24: Status→step dispatch is an example of how a driver maps its constructs across the two formats, not a universal mechanism

**Source:** Slide 34 (3.10 — Materialization and Dispatch) presents status→step dispatch as a standalone kit-level mechanism. Also §3.10 in design.md.

**PO concern:** The status→step dispatch section reads as though it's a universal kit-level concept. It's actually an **example** of how a specific driver (Ralph) defines a construct in the five-noun format and its equivalent in the BPMN format. In Ralph, a hat is a step (task); but if the hat's prose changes a GitHub status, it becomes a "status-changing step" — which internally maps to a task + gateway. This is just one illustration of how a driver can define its own constructs and their BPMN equivalents. It should be introduced as an example, not a universal dispatch mechanism.

**Investigation:** Confirmed. design.md §3.10 presented status→step dispatch as a standalone kit-level mechanism. It should be an example of the general pattern: each driver defines constructs in the five-noun format and specifies their BPMN equivalents (e.g. Ralph's hat = a step/task; a status-changing hat = task + gateway).

**Resolution:** Fixed in both design.md §3.10 and the presentation (page 34). The section now leads with "How a driver defines constructs across the two formats (example)" using the Ralph hat → status-changing step → task+gateway decomposition, then derives status→step dispatch as a consequence of that mapping. The general principle (driver-level construct decomposes into stock BPMN elements) is stated first; Ralph is the concrete illustration.

**Status:** RESOLVED.

---

## Comment 25: Slide 36 ("The Known Residual D-17") is random, jargon-heavy, and cryptic

**Source:** Slide 36 / page 36 of the presentation.

**PO concern:** The slide drops in mid-flow with terms like "status-altering," "blast radius," "driver-portable dispatcher status re-read," "zero-trust shepherd" — none of which have been introduced on the preceding slides. A reader hitting this cold has no idea what problem is being described or why it matters.

**Investigation:** The slide was a compressed dump of design.md §3.10's D-17 residual section — terms like "blast radius," "driver-portable dispatcher status re-read," and "zero-trust shepherd" dropped without context.

**Resolution:** Rewrote the slide in plain language: states the problem (steps declare a status change but the actual change happens on the tracker, outside the driver's control), what can go wrong (items get stuck or drift to unexpected statuses), what the MVP does (re-read and check after the fact — detection, not prevention), and what post-MVP adds (intercept the change itself). No jargon; a reader hitting this cold can follow it.

**Status:** RESOLVED.

---

## Comment 26: Package manifest fields (page 45) — are we inventing these or does pixi support them?

**Source:** Slide 45, "Data Models — Key Shapes," lists package manifest fields: `provides`/`requires`, `supported-coding-agents`, `lifecycle-hooks`, `meta-skill`, `version`, `content`.

**PO concern:** Are these fields that pixi/conda natively supports, or are we inventing a custom manifest format? If invented, the slide should say so. If pixi supports them, the slide should ground them.

**Investigation:** Checked PD-16 (pixi/conda mapping) and design.md §5.1. It's a mix: `provides`/`requires` map to conda virtual packages, `version` is native, `lifecycle-hooks` map to conda package scripts, `content` is archive contents. But `supported-coding-agents` is kit-specific (encoded as `requires: harness:<agent>` virtual packages) and `meta-skill` is kit-specific (prose sections riding as content).

**Resolution:** Fixed in both design.md §5.1 (each field now annotated with its conda/pixi mapping or "kit-specific" label) and the slide (replaced bullet list with a three-column table: Field / What it does / Conda/pixi?). A reader can immediately see what's native vs. invented.

**Status:** RESOLVED.
