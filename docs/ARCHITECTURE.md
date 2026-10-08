# Architecture

Phase 0 status: planning only. This document describes a recommended implementation shape; it does not add production code.

## Recommendation

Use Rust, but start with a small workspace rather than the full five-crate structure. The initial MVP should optimize for correctness, testability, and low abstraction weight.

Recommended v0.1 layout:

```text
stellar-upgrade-guard/
  Cargo.toml
  crates/
    guard-cli/
    guard-core/
  fixtures/
  docs/
  tests/
```

Split later only when pressure appears:

- add `guard-rpc` when deployed contract comparison is implemented;
- add `guard-report` only if output formatting grows beyond simple text/JSON/Markdown modules;
- add `guard-spec` only if spec parsing/normalization becomes large enough to deserve a public crate boundary.

This keeps the MVP honest: one CLI crate, one library crate, real fixtures, and documentation.

## Dependency Boundaries

`guard-cli`:

- owns argument parsing;
- reads files;
- calls `guard-core`;
- maps findings to text/JSON/Markdown output;
- maps results to exit codes.

`guard-core`:

- owns WASM spec extraction wrapper;
- owns normalization;
- owns compatibility analysis;
- owns public data structures and deterministic sorting;
- has no terminal, filesystem, network, or process-exit dependencies except where explicitly passed bytes.

Later `guard-rpc`:

- resolves contract IDs to WASM bytes;
- owns RPC transport, network configuration, and ledger XDR lookup;
- returns the same input shape as local file reads: WASM bytes plus source metadata.

Potential later `guard-report`:

- renders normalized findings;
- remains pure and deterministic;
- does not perform analysis.

## Internal Data Flow

Local v0.1:

```text
old.wasm bytes
  -> extract SCSpecEntry stream
  -> normalize ContractInterface

new.wasm bytes
  -> extract SCSpecEntry stream
  -> normalize ContractInterface

old interface + new interface
  -> compatibility engine
  -> sorted CompatibilityFinding list
  -> selected renderer
  -> exit code policy
```

Later deployed flow:

```text
contract ID + network/RPC
  -> LedgerKey::ContractData(instance)
  -> RPC getLedgerEntries
  -> ScContractInstance.executable
  -> LedgerKey::ContractCode(wasm_hash)
  -> RPC getLedgerEntries
  -> WASM bytes
  -> same extraction and comparison path as local v0.1
```

## Public API Boundaries

The core library should expose byte-oriented APIs first:

```text
parse_contract_interface(wasm: &[u8]) -> Result<ContractInterface, GuardError>
compare_interfaces(old: &ContractInterface, new: &ContractInterface) -> CompatibilityReport
compare_wasm(old: &[u8], new: &[u8]) -> Result<CompatibilityReport, GuardError>
```

Avoid exposing upstream `stellar_xdr` types as the primary public model. They should be accepted internally and normalized into stable project-owned structures. This allows output stability even when upstream generated Rust type names or enum layout details shift across protocol versions.

## Deterministic Analysis Model

Suggested internal concepts:

- `ContractInterface`
- `FunctionSpec`
- `FunctionParam`
- `TypeRef`
- `UserDefinedType`
- `StructField`
- `EnumCase`
- `UnionCase`
- `ErrorCase`
- `EventSpec`
- `ContractMetadata`
- `CompatibilityReport`
- `CompatibilityFinding`
- `CompatibilityImpact`
- `FindingCode`

Rules:

- Normalize before comparing.
- Use maps keyed by stable identifiers for lookups.
- Preserve declaration order where it affects serialized or call-visible behavior.
- Sort all findings by deterministic tuple: impact severity, finding code, subject path, detail key.
- Treat unknown or unsupported spec entry kinds as explicit `Unknown` findings, not silent success.
- Do not use LLM or heuristic semantic judgments in the tool.

## Error Handling Strategy

Use a single top-level error enum in `guard-core`, likely implemented with `thiserror`.

Suggested categories:

- invalid or unreadable WASM;
- missing `contractspecv0`;
- malformed XDR;
- unsupported spec feature;
- duplicate/conflicting spec identity after normalization;
- RPC/network error later;
- output serialization error in CLI.

The CLI should distinguish:

- analysis completed with findings;
- malformed inputs/execution failure;
- unsupported-but-well-formed artifacts.

Unsupported artifacts may produce `Unknown` findings if comparison can continue, or an execution failure if the tool cannot safely parse the interface.

## Exit Code Integration

The report should be independent of exit code policy. The CLI can then implement:

- `0`: no findings at or above fail threshold;
- `1`: findings at or above fail threshold;
- `2`: malformed input, I/O failure, parse failure, RPC failure, or internal execution failure.

Default fail threshold should be `breaking`.

## Why This Is Not Over-Engineered

The MVP needs only two crates because all v0.1 work is local, deterministic, and non-networked. A separate CLI crate protects the core library from presentation concerns, while a separate core crate makes compatibility rules easy to unit test.

The initially proposed `guard-spec`, `guard-rpc`, and `guard-report` crates are valid future extraction points, but adding them before implementation would create interfaces before their actual shape is known. Start smaller; split when test friction or API ownership makes the split pay for itself.

## Dependencies To Evaluate During Implementation

Likely:

- `soroban-spec`
- `stellar-xdr`
- `clap`
- `serde`
- `serde_json`
- `thiserror`
- `wasmparser` only if direct section handling is needed beyond `soroban-spec`

Dev/test:

- `insta` only if snapshot output proves useful;
- `assert_cmd` and `predicates` for CLI tests;
- real fixture contract builds using `stellar contract build`.

Avoid adding RPC/client dependencies in v0.1 unless deployed comparison is pulled into scope.
