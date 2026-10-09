# v0.1.0 Release Readiness

## Summary

Stellar Upgrade Guard is close to a `v0.1.0` source release, but the project should complete final release process checks before tagging.

## Assessment

| Area | Status | Notes |
| --- | --- | --- |
| Functionality | Ready for v0.1 scope | Local and deployed interface comparison exist. |
| Tests | Strong for current scope | Parser, compatibility, CLI, RPC mock, and fixture tests exist. |
| CI | Ready | Repository CI and action-test workflows exist. |
| Documentation | Mostly ready | OSS docs, action docs, and release docs are present. |
| Security | Reasonable initial posture | Security policy and secret handling guidance exist. |
| Packaging | Partial | Source and Git install work; crates.io publishing is not configured as a v0.1 requirement. |
| Installation | Source/Git-based | `cargo install --path crates/guard-cli` and Git install are documented and verified. |
| API stability | Pre-1.0 | Internal APIs may change. |
| Action stability | Initial | Composite action is tested against local fixtures. |
| Known limitations | Documented | No storage, runtime, authorization, or migration safety proof. |

## Remaining Blockers Before v0.1.0

- Decide whether to publish crates or ship source-only for the first release. Current recommendation is GitHub source release first.
- Update `CHANGELOG.md` with the actual release date.
- Confirm crate versions and metadata one final time.
- Verify the MSRV claim with `cargo +1.91.0 check --workspace` and `cargo +1.91.0 test --workspace`.
- Run dependency audit, including `cargo audit` if available.
- Run final secret scan.
- Confirm branch protection and required checks in GitHub settings.
- Enable GitHub private vulnerability reporting or document another private security contact.
- Verify README examples after choosing the release reference.
- Decide whether the action should be referenced by tag `v0`, `v0.1.0`, or commit SHA for the first release.

## Recommendation

Proceed toward `v0.1.0` after the blockers above are resolved. Do not claim production safety; position the release as an early deterministic interface guard for Soroban contract upgrades.
