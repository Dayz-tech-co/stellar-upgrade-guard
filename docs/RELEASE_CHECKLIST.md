# Release Checklist

Use this checklist for the future `v0.1.0` release. Do not create releases from a dirty worktree.

## Before Tagging

- Start from clean `main`.
- Confirm required GitHub workflows are green:
  - CI / Rust
  - Action Test / Composite Action
- Run local checks:

```text
cargo fmt --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

- Run dependency review:

```text
cargo tree
cargo tree -d
```

- Run `cargo audit` if available.
- Run a secret scan.
- Confirm GitHub private vulnerability reporting is enabled or `SECURITY.md` documents another private reporting path.
- Confirm `CHANGELOG.md` has the release date and entries.
- Confirm crate versions intended for release are updated.
- Confirm README installation and action references are accurate.
- Confirm no `target/`, `.build-target/`, logs, local identities, or generated release artifacts are tracked.
- Perform a Testnet smoke test if deployed comparison changed.

## Release

- Create the release commit.
- Create the tag only after review approval.
- Draft GitHub release notes from `CHANGELOG.md`.
- Attach release artifacts only if the distribution process has been defined.
- Update action examples or tags if an action tag is part of the release.

## After Release

- Verify installation instructions.
- Verify the GitHub Action reference.
- Open follow-up issues for deferred release blockers.
- Move `CHANGELOG.md` back to an `[Unreleased]` development state.
