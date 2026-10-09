# Contributor Backlog Proposal

This is a proposal for future issues. No GitHub issues have been created from this list.

## 1. Improve Type Rendering For Nested Types

Problem: Some normalized type strings are compact but not always easy to scan.

Scope: Improve display formatting without changing semantic comparisons.

Likely files: `crates/guard-core/src/normalize.rs`, parser tests, README examples.

Acceptance criteria: Existing findings stay deterministic; nested types render consistently; tests cover at least two nested shapes.

Test expectations: Unit tests for normalized output and compare messages.

Complexity: Medium.

First-time contributor: Yes, if scoped to rendering and tests.

## 2. Add Markdown Report Output

Problem: CI users may want readable PR comments or job summaries.

Scope: Add a Markdown output mode or helper renderer without changing JSON schema.

Likely files: `crates/guard-cli/src/main.rs`, CLI tests, action docs.

Acceptance criteria: Markdown includes result, counts, and findings; JSON output remains unchanged.

Test expectations: Snapshot-like CLI assertions for stable Markdown.

Complexity: Medium.

First-time contributor: Yes, with guidance.

## 3. Evaluate SARIF Output

Problem: Code scanning integrations may benefit from SARIF.

Scope: Research mapping findings to SARIF and prototype only if the mapping is meaningful.

Likely files: docs first, then CLI output code if accepted.

Acceptance criteria: Documented mapping decisions and limitations.

Test expectations: JSON/SARIF validation if implemented.

Complexity: High.

First-time contributor: No.

## 4. Add RPC Provider Configuration Examples

Problem: Users may need provider-specific RPC configuration without exposing secrets.

Scope: Document safe patterns for `--rpc-url`, GitHub secrets, and private providers.

Likely files: README, `action/README.md`, `docs/RPC_ARCHITECTURE.md`.

Acceptance criteria: Examples do not print credentials and avoid provider lock-in.

Test expectations: Documentation-only review.

Complexity: Trivial.

First-time contributor: Yes.

## 5. Add More Real WASM Compatibility Fixtures

Problem: Rule confidence improves with real Soroban fixtures.

Scope: Add focused old/new fixture pairs for uncovered edge cases.

Likely files: `fixtures/phase2`, `crates/guard-core/tests/compare.rs`.

Acceptance criteria: New fixture pair is small, committed as canonical `old.wasm`/`new.wasm`, and covered by tests.

Test expectations: Compare tests using the fixture pair.

Complexity: Medium.

First-time contributor: Yes, if the case is already classified.

## 6. Event Semantics Research

Problem: Some event changes are conservatively unknown.

Scope: Research Stellar/Soroban event compatibility semantics and propose rule changes only with evidence.

Likely files: `docs/COMPATIBILITY_RULES.md`, issue discussion, maybe `crates/guard-core/src/compare.rs`.

Acceptance criteria: Evidence-backed recommendation and fixtures if implementation follows.

Test expectations: Tests required for any code change.

Complexity: High.

First-time contributor: No.

## 7. Union Semantics Research

Problem: Union compatibility can be subtle and currently conservative.

Scope: Research XDR/Soroban union behavior and propose rule adjustments.

Likely files: docs, compatibility rule tests, fixtures.

Acceptance criteria: Evidence-backed proposal; no safety claims without proof.

Test expectations: Tests required for any implementation.

Complexity: High.

First-time contributor: No.

## 8. GitHub Action Distribution Improvements

Problem: The action currently builds the CLI from source during the workflow.

Scope: Evaluate release artifact download or cache strategies after release artifacts exist.

Likely files: `action/action.yml`, `action/README.md`, release docs.

Acceptance criteria: Secure source, no `curl | bash`, pinned references, no secret exposure.

Test expectations: Action-test workflow remains offline.

Complexity: Medium.

First-time contributor: No until release artifacts exist.

## 9. Cross-Platform Verification

Problem: CI currently targets Ubuntu-hosted runners first.

Scope: Evaluate Windows and macOS workflow coverage for CLI tests.

Likely files: `.github/workflows/ci.yml`, docs.

Acceptance criteria: Matrix does not add excessive cost or flaky dependencies.

Test expectations: CI passes on selected platforms.

Complexity: Medium.

First-time contributor: Yes, if limited to documentation or one platform experiment.

## 10. Add More Documentation Examples

Problem: New users benefit from realistic local and deployed examples.

Scope: Add concise examples using existing fixtures and sanitized contract IDs.

Likely files: README, docs.

Acceptance criteria: Examples are runnable or clearly marked as illustrative; no secrets.

Test expectations: Documentation review.

Complexity: Trivial.

First-time contributor: Yes.

## Good First Issue Candidates

At most three tasks from this backlog are good first issue candidates:

- Add RPC provider configuration examples: documentation-only, security-focused, small scope.
- Add more documentation examples: low-risk and easy to review.
- Improve type rendering for a narrow nested type case: code plus tests, but no semantic rule change.

Compatibility semantics research is not a good first issue because it requires deep Soroban evidence and careful classification.
