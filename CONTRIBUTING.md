# Contributing

Stellar Upgrade Guard is intentionally conservative infrastructure. It inspects Soroban contract interface metadata and reports deterministic compatibility findings. It must not imply that a contract upgrade is safe unless that claim is backed by explicit, implemented checks.

## Project Philosophy

- Prefer deterministic output over clever inference.
- Prefer conservative compatibility classifications when Soroban semantics are uncertain.
- Keep parser and comparison behavior evidence-based.
- Do not add storage migration, authorization, or runtime safety claims without a separately designed feature.
- Treat compatibility-rule changes as product behavior changes, not cosmetic refactors.

## Welcome Contributions

Useful contributions include:

- bug fixes with small reproductions;
- additional Soroban fixture pairs;
- parser and normalization improvements;
- compatibility rules backed by Stellar/Soroban evidence;
- CLI, JSON, Markdown, or GitHub Action reporting improvements;
- documentation that clarifies real behavior and limitations;
- CI, packaging, and release-process hardening.

Large semantic changes should start with an issue or design discussion before implementation.

## Setup

Required:

- Rust 1.91.0 or newer with Cargo.
- The workspace toolchain used by CI.

Optional:

- Stellar CLI v25.2.0 or newer for rebuilding real Soroban fixture WASM.
- `wasm32v1-none` target for fixture builds:

```text
rustup target add wasm32v1-none
```

## Local Checks

Run these before opening a PR:

```text
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

If you touch fixture source outside the workspace, also format that fixture directly:

```text
cargo fmt --check --manifest-path fixtures/phase3/live-breaking-candidate/Cargo.toml
```

## Fixtures

Committed real-WASM compatibility fixtures live under `fixtures/phase2/<scenario>/old.wasm` and `fixtures/phase2/<scenario>/new.wasm`.

Fixture source crates are excluded from the production workspace. Build output under `target/` or `.build-target/` must not be committed unless a future design explicitly requires a tracked artifact.

When adding or changing a compatibility rule, include focused tests and, where practical, a real Soroban fixture that demonstrates the old and new interface.

## Compatibility Rules

Compatibility rules must be backed by evidence from Soroban/Stellar semantics, XDR structures, SDK behavior, or documented contract interface behavior. A rule proposal should explain:

- the old and new interface shapes;
- why the change is compatible, warning, breaking, or unknown;
- relevant spec, SDK, or XDR references;
- the expected finding code and message;
- tests or fixtures that reproduce the case.

If evidence is incomplete, classify conservatively as `unknown` rather than claiming safety.

## Pull Requests

PRs should be narrow and reviewable. Include:

- what changed and why;
- tests added or updated;
- docs updated where behavior changed;
- confirmation that local quality gates pass;
- notes about any skipped checks.

Do not include secrets, local Stellar identities, target directories, dependency caches, logs, or unrelated generated files.
