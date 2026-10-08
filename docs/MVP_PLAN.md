# MVP Plan

Phase 0 status: implementation plan only. Do not begin Phase 1 from this document without an explicit follow-up decision.

## v0.1 Scope

v0.1 should support:

- comparing two local compiled Soroban WASM files;
- extracting official `contractspecv0` entries;
- normalizing functions, function inputs/outputs, UDT structs, UDT enums, UDT unions, UDT error enums, events where present, metadata where useful;
- producing deterministic compatibility findings;
- producing human-readable text output;
- producing JSON output;
- returning CI-friendly exit codes;
- demonstrating behavior with real compiled Soroban fixture contracts.

Recommended first command:

```text
stellar-upgrade-guard compare --old ./contract-v1.wasm --new ./contract-v2.wasm
```

Recommended v0.1 options:

```text
--format text|json
--fail-on warning|breaking
```

Default:

- `--format text`
- `--fail-on breaking`

Markdown output can wait unless documentation automation needs it immediately.

## v0.1 Exclusions

Exclude from v0.1:

- deployed contract lookup by contract ID;
- RPC configuration;
- WASM hash resolution;
- Stellar Asset Contract special-case comparison;
- storage migration safety;
- source-code analysis;
- bytecode semantic analysis;
- security analysis;
- GitHub Actions templates;
- release automation;
- broad open-source governance boilerplate.

These are useful later, but they are not required to prove the core value.

## Milestones

1. Create Rust workspace with `guard-cli` and `guard-core`.
2. Add `soroban-spec` and `stellar-xdr` parsing path.
3. Implement normalized internal model.
4. Implement deterministic diff engine for certain v0.1 rules.
5. Add text and JSON renderers in CLI or a small reporting module.
6. Add exit-code policy.
7. Add minimal real fixture contracts and compiled WASM fixtures.
8. Add integration tests for fixture comparisons.
9. Add malformed WASM and missing-spec tests.
10. Refresh README with accurate MVP usage only after the tool works.

## Exact Build Order

1. Establish workspace and crate skeleton.
2. Implement `guard-core::parse_contract_interface(wasm: &[u8])`.
3. Add parser tests using one real compiled fixture and one malformed WASM.
4. Define normalized `ContractInterface` with deterministic ordering.
5. Add normalization tests from raw `ScSpecEntry` fixtures.
6. Implement function-level compatibility rules.
7. Add UDT enum/error/union/struct rules that are certain.
8. Add event and metadata warning-only reporting.
9. Implement CLI local file comparison.
10. Implement text output.
11. Implement JSON output.
12. Implement exit code policy.
13. Add end-to-end CLI tests.
14. Add README usage only after tests pass.

## Test Strategy

Use tests to validate behavior, not to duplicate implementation logic.

Unit tests:

- parser error taxonomy;
- normalization ordering;
- type identity comparison;
- compatibility rule classification;
- finding sorting.

Integration tests:

- real compiled fixture: function added;
- real compiled fixture: function removed;
- real compiled fixture: argument type changed;
- real compiled fixture: return type changed;
- real compiled fixture: enum value changed;
- real compiled fixture: error code changed;
- real compiled fixture: event changed if current SDK emits event specs;
- malformed WASM;
- valid WASM without `contractspecv0`.

CLI tests:

- compatible comparison exits `0`;
- breaking comparison exits `1`;
- malformed input exits `2`;
- `--fail-on warning` exits `1` when only warnings exist;
- JSON output is stable and parseable.

Golden/snapshot tests:

- Use sparingly for text output once format stabilizes.
- Prefer structural JSON assertions for machine output.

## Fixture Strategy

Fixtures should be real Soroban contracts compiled with `stellar contract build`, not hand-written XDR only.

Suggested layout:

```text
fixtures/
  function-added/
    old/
    new/
  function-removed/
    old/
    new/
  argument-type-changed/
    old/
    new/
  return-type-changed/
    old/
    new/
  enum-value-changed/
    old/
    new/
  error-code-changed/
    old/
    new/
```

Store enough source to rebuild fixtures. Decide later whether compiled WASM is committed; if committed, document the exact Stellar CLI and Rust SDK versions used.

## Demo Strategy

The first public demo should be boring and trustworthy:

1. Build two small fixture contracts.
2. Run local compare.
3. Show breaking findings with function/type paths.
4. Show JSON output in CI-like form.
5. Show exit code behavior.

Avoid deployed comparison in the first demo unless RPC support has already been tested against local Quickstart and testnet.

## Release Success Criteria

v0.1 is successful if:

- it compares two local compiled Soroban WASM files;
- it extracts official specs through official crates/types;
- it detects high-confidence breaking interface changes;
- output is deterministic;
- JSON output is documented enough for CI users;
- exit codes are CI-friendly;
- tests include real compiled fixtures;
- README claims only what the tool actually does;
- unsupported areas are explicit.

## Later v0.2 Candidates

- deployed contract comparison: `--contract <ID> --candidate <WASM> --network testnet`;
- `--rpc-url` and `--network-passphrase`;
- contract ID to WASM resolution through `getLedgerEntries`;
- Markdown output;
- policy config for warnings and metadata keys;
- richer event compatibility modes;
- Stellar Asset Contract handling;
- Quickstart-based RPC integration tests.
