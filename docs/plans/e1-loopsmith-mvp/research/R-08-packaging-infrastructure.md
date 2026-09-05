# R-08 — Packaging Infrastructure for the Kit

**Question:** What existing packaging infrastructure should the kit build on, rather than inventing its own package format, resolver, and tooling?

**Why it matters:** The kit's core model is RPM-inspired — packages declare `provides: [capabilities]` and `requires: [capabilities]` over an open namespace, resolved by a SAT solver. Building a custom package manager from scratch (format + resolver + installer + database + lock files + repo hosting + discovery) is thousands of lines of infrastructure that already exists in mature ecosystems. Choosing the right foundation determines how much of the 47-feature surface (see fresh-features experiment below) the kit gets for free vs must build.

## Method

Evaluated seven candidates against the kit's requirements: virtual provides/requires capability resolution, cross-platform support, embeddable as a library, package creation, lock files, and composability.

## Candidates Evaluated

### 1. rpm-rs (RPM format library)

- **URL:** https://github.com/rpm-rs/rpm-rs
- **What it is:** Pure Rust library for creating and reading RPM packages. Not a package manager — a format library.
- **Provides/requires:** Full support — `PackageBuilder::provides()`, `requires()`, `conflicts()`, `obsoletes()`, plus weak deps. Both read and write.
- **Scriptlets:** Full pre/post install/uninstall/trans support with custom interpreters.
- **Signing:** PGP v4/v6, RSA, Ed25519, remote HSM signing.
- **Python bindings:** Yes (PyPI as `rpm-rs`).
- **Cross-platform:** Yes (pure Rust).
- **What's missing:** No resolver, no installer, no rpmdb write, no repo creation, no lock file. It's a format-only library.
- **Maintainer:** Daniel Alley (Red Hat), actively maintained, 40+ contributors.

### 2. resolvo + resolvo-rpm (SAT resolver)

- **URL:** https://github.com/prefix-dev/resolvo, https://github.com/prefix-dev/resolvo-rpm
- **What it is:** resolvo is a generic CDCL SAT-based dependency resolver (from the mamba/pixi team). resolvo-rpm is a ~450-line proof-of-concept bridging RPM repo metadata to resolvo.
- **Virtual provides:** Yes — resolvo-rpm maps RPM `Provides:` to a `provides_to_package` HashMap; any package providing a capability satisfies a `Requires:` for it.
- **Performance:** Faster than libsolv for complex problems (the C solver used by dnf).
- **Cross-platform:** Yes (pure Rust).
- **What's missing:** resolvo-rpm is a PoC — conflicts/obsoletes not wired yet (resolvo supports them). No installer, no database.
- **Relationship:** rpm-rs maintainer (dralley) and resolvo maintainer (wolfv) actively collaborate — see rpm-rs discussion #330.

### 3. rpmdb-rs (RPM database reader)

- **URL:** https://github.com/yybit/rpmdb-rs
- **What it is:** Pure Rust library for reading RPM package databases (BDB, NDB, SQLite3 formats).
- **Read-only:** Can extract name, version, provides, requires, file lists from any rpmdb. Cannot write.
- **Cross-platform:** Mostly (has `nix` crate dependency for file locking).
- **Verdict:** Fills a niche but the read-only limitation means you'd still write the DB yourself.

### 4. archlinux/alpm (Pure Rust ALPM)

- **URL:** https://github.com/archlinux/alpm
- **What it is:** Arch Linux's official pure-Rust rewrite of ALPM — 19 crates, funded by Sovereign Tech Agency. Schema-first design.
- **Provides/requires:** Full support — `provides`, `requires`, `conflicts`, `replaces` in both installed DB and repo DB schemas. Includes soname-based virtual packages.
- **Database:** Read AND write for both installed packages (alpm-db) and repo metadata (alpm-repo-db desc files). Pure Rust, no platform dependencies in core type/DB crates.
- **Package format:** `.pkg.tar.zst` — create and read via `alpm-package` crate.
- **Groups:** Simple package group membership in the data model.
- **What's missing:** No resolver (data model ready, stub crates for future work). No lock file.
- **Cross-platform:** Core crates (types, DB, repo-db) are fully platform-agnostic. Only filesystem operations (symlink extraction in alpm-package, mtree validation) use `os::unix`.

### 5. uv (Python package manager)

- **URL:** https://github.com/astral-sh/uv
- **What it is:** Extremely fast Python package manager (10-100x faster than pip). 70 Rust crates, PubGrub resolver.
- **Provides/requires:** No. Python packaging resolves on package names + version ranges, not virtual capabilities.
- **Architecture:** Excellent modular design with trait-based `ResolverProvider`. But everything below the resolver is Python-specific (wheel format, PEP 508 markers, platform tags, venvs).
- **Verdict:** Wrong dependency model for our use case. Good patterns to study (batch prefetching, lock file design) but not reusable infrastructure.

### 6. pixi / rattler / resolvo (conda ecosystem in Rust)

- **URL:** https://github.com/prefix-dev/pixi, https://github.com/conda/rattler, https://github.com/prefix-dev/resolvo
- **What it is:** Three-layer stack. Pixi = CLI + manifest + environments. Rattler = engine (28 Rust crates: types, solver, installer, channels, lock). Resolvo = generic SAT solver.
- **Virtual capabilities:** Yes — arbitrary virtual package names. The solver (`rattler_solve`) accepts `GenericVirtualPackage` with any name (not a closed enum). Pixi's manifest accepts arbitrary `__name = "version"` entries via a raw escape hatch. The typed `VirtualPackage` enum is only for host-detection (irrelevant to our use case). Resolution is fully open.
- **Evidence:** `rattler_solve/src/resolvo/mod.rs` lines 341-346 — virtual packages are interned into the solver pool by name, no filtering. Any `GenericVirtualPackage { name: "__tracker", version: "1.0" }` resolves correctly.
- **Lock file:** `pixi.lock` (YAML, per-environment/platform) — done.
- **Features/environments:** Pixi features = named dependency sets (our "families"). Pixi environments = feature compositions (our "blueprints"). Solve groups = joint resolution across environments.
- **Task runner:** Graph-based with templating, caching, platform-specific tasks.
- **Package creation:** rattler-build / pixi-build.
- **Channel hosting:** Directory + `repodata.json`, or prefix.dev hosted.
- **Cross-platform:** Linux, macOS, Windows.
- **Python bindings:** Yes.
- **Maturity:** Production-grade. Pixi is prefix.dev's flagship product.

### 7. Nix

- **What it is:** Functional package manager with declarative configurations, reproducible builds.
- **Capability model:** NixOS module system — modules declare `options` (what they need) and `config` (what they provide). Type-checked composition, fails early on unmet requirements. Overlays for swapping providers.
- **Flakes:** `inputs`/`outputs` + `flake.lock`. Composable, reproducible.
- **Arbitrary payloads:** Yes — derivations can output config files, not just compiled software.
- **Reproducibility:** Best in class (content-addressed store).
- **What's missing:** Not embeddable (requires Nix daemon infrastructure). Steep learning curve (own language). tvix (Rust reimplementation) incomplete.
- **Cross-platform:** Linux + macOS. Windows via WSL only.
- **Precedent:** devbox, devenv prove Nix works for non-OS packaging.

### 8. Homebrew

- **What it is:** macOS/Linux package manager (Ruby).
- **Provides/requires:** No. Explicitly moved away from flexibility — dependencies hardcoded to specific formula names. Open feature request (#11312) for equivalent dependencies, unimplemented.
- **Verdict:** Wrong model, wrong language, not embeddable.

### 9. Cargo (Rust crates)

- **What it is:** Rust's package manager.
- **Provides/requires:** No native capability resolution. Resolves on crate names + semver.
- **Extensibility:** `[package.metadata]` / `[workspace.metadata]` for custom tool data. `[patch]` for swapping sources. Virtual workspaces for organizing members. `Cargo.lock` for reproducibility. `build.rs` for post-install logic. Private registries.
- **Verdict:** Rich infrastructure, but you'd build a capability resolution layer on top. The resolver doesn't understand "any crate providing tracker."

### 10. Aura (Arch Linux AUR helper)

- **URL:** https://github.com/fosskers/aura
- **What it is:** Rust AUR helper wrapping pacman/libalpm. Has `aura-core` library with `DbLike` trait and `PkgGraph` (petgraph DAG).
- **Provides/requires:** Yes, via libalpm (C bindings, not pure Rust). Arch-only.
- **Verdict:** Good design patterns (`DbLike` trait) but not portable — tied to libalpm.

### 11. rpm-ostree (atomic OS composition)

- **URL:** https://github.com/coreos/rpm-ostree
- **What it is:** Composes immutable OS images from RPM packages + OSTree. Powers Fedora CoreOS, Silverblue.
- **Relevant pattern:** Declarative manifest (treefile) → resolve full closure → compose atomically → verify. Not "install one-by-one" but "build the whole tree at once." Deterministic — no state drift from install order.
- **Verdict:** Not usable infrastructure (full OS deployment system), but the atomic-compose-from-manifest pattern is the right mental model for setup realization.

## Decision

**Pixi (rattler + resolvo).**

The kit adopts pixi as its packaging infrastructure. Packages are conda packages (`.conda`) hosted on a custom channel. Capabilities are conda virtual packages (`__tracker`, `__harness`, etc.) with arbitrary names — the solver is fully open. Features, environments, lock files, task runner, package creation, and channel hosting come out of the box.

**Rationale:** Pixi gives the most complete stack with the least to build. The RPM and ALPM stacks provide format libraries but require building the engine (resolver bridge, installer, database, lock file, repo creation, discovery). Nix has a strong model but isn't embeddable. Cargo lacks native capability resolution. Pixi/rattler/resolvo is embeddable Rust crates with the virtual capability model already working end-to-end.

**Mapping to kit concepts:**

| Kit concept | Pixi equivalent |
|---|---|
| Package | conda package (`.conda`) |
| Capability | Virtual package (`__tracker`, `__harness`, etc.) |
| Blueprint | Pixi environment (composition of features) |
| Family | Pixi feature (named dependency set) |
| Resolver | resolvo SAT solver |
| Lock file | `pixi.lock` |
| Post-install wiring | Conda package scripts |
| Task runner | Pixi tasks |
| Channel / catalogue | Directory + `repodata.json` |

**What the kit still builds on top of pixi:**
- Conformance contracts (three faces per capability)
- Verification (check a setup against contracts)
- Smith's packaging skills (discover, recommend, author, wire, verify)
- The domain model (what goes in these packages — agents, loops, process, skills, ports)
- Wiring to concrete infrastructure

## Fresh-Features Experiment

A clean-room agent (no access to our existing specs) was given only the system description and asked to write features from scratch. It produced 47 features across 8 domains. Of these, **22 are covered by pixi OOTB** (packages, capabilities, resolution, curated sets, most of setup lifecycle and discovery). The remaining 25 are the kit's actual value-add: conformance contracts (10), Smith's skills (8), and the domain model / setup composition (8). See `/tmp/loopsmith-features-experiment/features-classified.md`.

## Related Research

- R-03: BotMinter on Loopsmith (the concrete system the kit must package)
- PD-08: Everything is a package (the RPM-inspired model — now realized via pixi/conda)
- PD-16: Packaging infrastructure decision rationale (design-notes.md)
