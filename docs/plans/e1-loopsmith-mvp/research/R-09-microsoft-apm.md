# R-09 — Microsoft APM (Agent Package Manager)

**Question:** How does Microsoft's APM relate to the Loopsmith kit? Is it prior art, a competitor, or complementary infrastructure?

**Why it matters:** APM (https://github.com/microsoft/apm) is an open-source Agent Package Manager from Microsoft that packages AI agent configurations — prompts, skills, instructions, hooks, MCP servers, plugins — with transitive dependency resolution, lockfiles, multi-harness deployment, and governance. It's the closest existing system to what we're building.

## What APM Is

APM is a **deterministic package resolver and multi-harness deployer for AI agent configuration.** It packages and distributes agent setup files across multiple LLM coding harnesses.

- **Version:** 0.21.0 (released 2026-06-19), MIT license, Python 3.10+
- **Spec:** OpenAPM v0.1 (Working Draft, 90 normative statements, vendor-neutral)
- **Maintainers:** Microsoft (Daniel Meppiel), EPAM (Sergio Sisternes), JFrog (Nadav Yogev)
- **Dogfooded:** Microsoft uses it on the `microsoft/apm` repo itself (2 direct deps → 5 transitive skills deployed)

## Architecture (from source code)

### Three-Layer Stack

```
apm CLI (commands: install, audit, compile, init, run, pack, ...)
  ├── Resolver (deps/apm_resolver.py — 1141 lines)
  ├── Compiler (compilation/ — 200K+ lines: agents, distributed, context, links)
  ├── Installer (install/ — pipeline + sources + drift + validation)
  ├── Policy engine (policy/ — discovery + checks + inheritance)
  └── Models (DependencyReference, APMPackage, PackageType, etc.)
```

### Resolver: Level-Batched Parallel BFS

The resolver is **URL-based and monorepo-aware**, not capability-based:

1. Parse dependency identifiers → `DependencyReference` objects (2099-line dataclass supporting 6 source types: GitHub shorthand, explicit URL, local path, monorepo `git: parent`, registry, marketplace)
2. BFS expansion with 4 parallel workers (configurable via `APM_RESOLVE_PARALLEL`), downloading + parsing transitive `apm.yml` files at each level
3. "First wins" flattening — deterministic, level-by-level, matches npm's hoisting model
4. Circular dependency detection via DFS with O(1) membership test

**Key techniques:**
- **Monorepo virtual packages** — child dep declares `{ git: parent, path: agents/foo }`, expands using parent's coordinates. Enables single-repo multi-package patterns.
- **Security: fail-closed on path confusion** — remote packages declaring `local_path` are rejected (#940). Absolute paths inside remote packages rejected. `..` traversal beyond project root rejected. Symlinks in destinations rejected.
- **git-semver opt-in** — semver resolution only when a range is supplied (`^1.2.0`); non-semver refs treated as literal branch/tag/SHA.

### Package Model: Polymorphic Detection

Package type is determined by **on-disk layout**, not manifest declaration:

| Type | Detection |
|---|---|
| `apm_package` | Has `apm.yml` |
| `claude_skill` | Has `SKILL.md`, no `apm.yml` |
| `hook_package` | Has `hooks/hooks.json` |
| `hybrid` | Both `apm.yml` and `SKILL.md` at root |
| `marketplace_plugin` | Has `plugin.json` or `.claude-plugin/` |
| `skill_bundle` | Has `skills/<name>/SKILL.md` (nested) |

The `type:` field in `apm.yml` (`instructions | skill | hybrid | prompts`) is **advisory in v0.1** — behavior is driven by what files exist, not what the manifest says.

### Eight Primitive Types

| Primitive | Format |
|---|---|
| Instructions | AGENTS.md, rules/ |
| Skills | SKILL.md |
| Prompts | Markdown with frontmatter |
| Agents | AGENTS.md persona definitions |
| Hooks | hooks.json (lifecycle events) |
| Plugins | plugin.json / .claude-plugin/ |
| MCP Servers | MCP transport config |
| LSP Servers | LSP transport config |

### Multi-Harness Compilation

APM's most distinctive feature. `apm compile` transforms generic packages into target-specific output:

| Target | Output |
|---|---|
| Copilot | `.github/prompts/AGENTS.md` + Workflow URIs + execution policies |
| Claude Code | `.claude/CLAUDE.md` + `.claude/rules/` + MCP configs |
| Cursor | Cursor-specific instruction format |
| Windsurf | Windsurf directives |
| Codex, Gemini, OpenCode, Kiro | Per-harness adapters |

Implementation: hand-tuned adapter per harness, no universal format. Deterministic output — timestamp removed, build ID computed after full content assembly. Supports `distributed` strategy (split instructions across multiple files per "Minimal Context Principle") and `managed_section` mode (update only APM block in existing AGENTS.md).

### Lockfile: Integrity Anchor

`apm.lock.yaml` records per-entry:
- `resolved_commit` — git SHA
- `tree_sha256` — git tree hash (guards against SHA-1 collisions)
- `deployed_files` — list of on-disk paths
- `deployed_file_hashes` — per-file SHA256
- `resolved_hash` — for registry-sourced entries
- `content_hash` — for local package sources

Frozen-install mode (`apm install --frozen`) re-verifies every hash. Drift detection rebuilds from scratch and diffs against working tree.

### Governance: Policy as First-Class

`apm-policy.yml` with pluggable discovery (per-host conventions), cascading inheritance (max 5 layers, cycle detection), and three merge semantics:

| Field | Merge rule |
|---|---|
| Enforcement (allow/warn/deny) | Stricter wins |
| `*.allow` patterns | Set intersection |
| `*.deny` patterns | Union |
| `*.require` | Union |
| `max_depth` | min(parent, child) |
| `require_hashes` | Logical OR |

Policy discovery is pluggable — different conventions per git host (`.github/`, `.apm/`, `_apm/` for ADO).

### OpenAPM Spec (90 Normative Statements)

Location: `docs/src/content/docs/specs/openapm-v0.1.md` (2840 lines)

Key aspects:
- 4 conformance classes: Producer, Consumer, Registry (v0.2), Governance
- YAML 1.2 safe subset only (no anchors/aliases/custom tags)
- Vendor-extension namespace: `x-[a-z][a-z0-9-]*` at every mapping level
- Lockfile version monotonic (once `"2"`, never demote to `"1"`)
- No registry wire contract yet (v0.1 = on-disk formats only; HTTP API = v0.2)

### Design Principles (PRINCIPLES.md)

- **P1:** No invented primitive frontmatter — emit to canonical schemas only
- **P2:** Multi-harness with traction gating — ship to harnesses with user traction only
- **P3:** Vendor neutral by construction — no preferred LLM or runtime baked in
- **P4:** UX is the floor — never degraded for fixes
- **P5:** Portability over vendor lock-in — write once, run anywhere
- **P6:** Reliability over magic — predictable, auditable, explainable
- **P7:** Community over feature count — external PRs first

## What APM Does NOT Have

| Feature | APM | Loopsmith Kit |
|---|---|---|
| **Virtual provides/requires** | No — URL/name-based resolution only | Yes — capabilities as virtual packages, any-provider resolution |
| **Conformance contracts** | No | Yes — three faces (structural, data, behavioral) per capability |
| **Setup composition model** | No — packages are flat config file trees | Yes — kernel nouns (context, loop, item, actor, port) |
| **Capability-based resolution** | No — "give me `owner/repo`" not "give me any tracker" | Yes — `__tracker` resolved to any provider |
| **Builder agent** | No — human selects packages | Yes — Smith discovers needs, recommends, authors, wires |
| **Curated sets / blueprints** | No — flat dependency lists | Yes — pixi features composed into environments |
| **Domain model** | Generic (any config files) | Specific (agentic SDLC: agents, loops, process, skills, ports) |
| **SAT-based resolution** | No — BFS + first-wins flattening | Yes — resolvo CDCL SAT solver |

## What APM Does That We Should Adopt or Study

### 1. Multi-Harness Compilation (ADOPT)
The single most important APM feature for us. Our `__harness` capability provider must emit harness-specific output (Claude Code CLAUDE.md, Copilot instructions, etc.). APM's adapter-per-harness pattern with deterministic compilation is the right model. We should either:
- Use APM's compilation pipeline directly (our packages produce APM-compatible primitives, APM compiles them to targets), or
- Implement the same adapter pattern in our post-install wiring

### 2. Governance / Policy Model (ADOPT)
`apm-policy.yml` with tighten-only inheritance is production-ready for enterprise adoption. We need the same: which capabilities are allowed, which channels are trusted, which packages are approved. The three merge semantics (stricter wins, intersection, union) are well-thought-out.

### 3. Security Posture (ADOPT)
- Content scanning for hidden Unicode (prompt injection via invisible characters)
- Per-file SHA256 lockfile integrity
- Drift detection (lockfile vs disk)
- Fail-closed on path confusion (remote packages declaring local paths)
- SBOM export (CycloneDX, SPDX)

### 4. OpenAPM Spec Structure (STUDY)
90 normative statements with stable IDs, 4 conformance classes, vendor-extension namespace. A model for how to write our conformance contract spec.

### 5. Primitive Type Taxonomy (STUDY)
APM's 8 primitives (instructions, skills, prompts, agents, hooks, plugins, MCP, LSP) describe what goes INSIDE agent packages. Our kernel nouns (context, loop, item, actor, port) describe how packages COMPOSE. These are orthogonal decompositions — our packages could contain APM primitives.

### 6. Git-Native Distribution (CONSIDER)
APM distributes from any git host — no central registry required. Simpler than hosting a conda channel for early adoption. Could complement our pixi channel.

## Relationship to Loopsmith

**Overlapping but different models:**

- **APM** = resolve by name → install files → compile to harness. "Install this specific skill for Claude Code."
- **Loopsmith** = resolve by capability → install packages → wire to infrastructure → verify conformance → running setup. "I need a tracker — find a compatible one, install it, wire it to this GitHub repo, verify the setup works."

Loopsmith owns the full lifecycle: a blueprint isn't done until the setup is running. Resolution picks the packages; installation puts them in place; wiring connects them to concrete infrastructure (this repo, these credentials, this board); verification checks that the assembled setup actually conforms. APM stops at "files on disk, compiled for your harness."

Where they could intersect: APM's multi-harness compilation could be a technique Loopsmith uses *during its wiring phase* — when a Loopsmith package provides `__harness` and targets Claude Code, the wiring step could invoke APM-style compilation to emit `.claude/CLAUDE.md` and MCP configs. The primitives inside Loopsmith packages (skills, instructions, agent configs) could follow APM/OpenAPM formats for interoperability, even though the resolution and lifecycle are Loopsmith's.

## Open Questions

1. Should Loopsmith packages contain APM-compatible primitives (`SKILL.md`, `AGENTS.md`, hooks)?
2. Should post-install wiring invoke `apm compile` to generate harness-specific output?
3. Should the kit adopt APM's governance model (`apm-policy.yml`) alongside or instead of building its own?
4. Is the OpenAPM spec a natural standard for what our packages *contain*, while pixi is the standard for how they *compose*?
5. Should we engage with the APM community (Microsoft, EPAM, JFrog) on interoperability?
