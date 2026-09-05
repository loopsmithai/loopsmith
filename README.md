# Loopsmith

A kit for managing your loops — a value-stream work model abstracted above any harness, runtime, or task tracker.

## Quick Start

```bash
# Build both binaries
cargo build

# Initialize (creates GitHub App + stores credentials)
smith init

# Verify (hits GitHub API with minted token)
smith-agent repo view father-smith
```

## Structure

- `smith` — operator CLI (human-ergonomic, colored output)
- `smith-agent` — agent CLI (machine-readable, verbose, corrective, self-describing)

Both binaries share a single crate at `crates/smith/`.
