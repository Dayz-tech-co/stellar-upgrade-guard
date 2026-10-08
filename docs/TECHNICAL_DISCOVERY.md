# Technical Discovery

Phase 0 status: discovery only. No production implementation is implied by this document.

## Summary

Stellar smart contracts built with `soroban-sdk` embed their callable interface in compiled WASM, not in source-only metadata. The primary artifact for local comparison is the WASM custom section named `contractspecv0`, whose payload is a raw stream of XDR-encoded `SCSpecEntry` values. This is the right foundation for Stellar Upgrade Guard v0.1 because it is official, machine-readable, stored with uploaded contract WASM, and already consumed by Stellar CLI and SDK tooling.

The tool should initially compare only facts present in official WASM spec and metadata sections. It should not claim storage migration safety, behavioral equivalence, authorization safety, or semantic safety.

## Official Mechanisms

The current official contract interface mechanism is SEP-48, Contract Interface Spec. SEP-48 defines:

- the `contractspecv0` WASM custom section;
- a payload that is a binary XDR stream of `SCSpecEntry` values;
- spec entries for exported contract functions, user-defined structs, unions, enums, error enums, and events.

Reference: [SEP-48 Contract Interface Spec](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0048.md).

The compiled WASM may also include:

- `contractmetav0`: a stream of `SCMetaEntry` values for contract metadata;
- `contractenvmetav0`: a stream of `SCEnvMetaEntry` values for environment/interface-version requirements.

References: [Stellar CLI contract info docs](https://developers.stellar.org/docs/tools/cli/stellar-cli), [SEP-58 reproducible build metadata](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0058.md), [CAP-46 runtime environment](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0046-01.md).

## WASM Spec Extraction Path

The official Rust extraction path is already present in the `soroban-spec` crate from `stellar/rs-soroban-sdk`:

1. Parse the WASM with `wasmparser::Parser`.
2. Locate a `Payload::CustomSection`.
3. Select the custom section whose name is `contractspecv0`.
4. Decode the section bytes as a stream of `stellar_xdr::ScSpecEntry` using `ScSpecEntry::read_xdr_iter`.

The current upstream implementation does exactly this in `soroban-spec/src/read.rs`.

Reference: [soroban-spec read.rs](https://raw.githubusercontent.com/stellar/rs-soroban-sdk/main/soroban-spec/src/read.rs).

Important implications:

- There is no JSON envelope, length prefix, or custom framing around the spec stream.
- Missing `contractspecv0` should be treated as "not a Soroban interface-bearing WASM" or malformed input, not as an empty compatible interface.
- The parser should preserve all spec entry kinds even if v0.1 does not fully analyze all of them.

## Relevant Rust Crates

Recommended official crates for MVP:

- `soroban-spec`: official utility crate that can parse `contractspecv0` from WASM with `read::from_wasm`.
- `stellar-xdr`: official generated Rust XDR crate containing `ScSpecEntry`, `ScSpecFunctionV0`, `ScSpecTypeDef`, `ScMetaEntry`, `ScEnvMetaEntry`, ledger keys, ledger entries, and RPC-relevant XDR values.
- `wasmparser`: used by official Stellar tooling to walk custom sections. It can remain a transitive implementation detail if `soroban-spec` is sufficient.

Optional/later:

- `soroban-spec-rust`: useful for rendering Rust-like interfaces, but not needed for deterministic compatibility analysis.
- `soroban-spec-tools`: used by Stellar CLI for higher-level contract info operations, but it may be more CLI-oriented than the MVP needs.
- `stellar-strkey`: likely needed later for decoding contract IDs and building ledger keys for deployed lookup.

References: [soroban-spec crate page](https://docs.rs/crate/soroban-spec/28.0.0), [stellar-xdr crate docs](https://docs.rs/stellar-xdr/latest/stellar_xdr/), [Stellar CLI interface source](https://raw.githubusercontent.com/stellar/stellar-cli/main/cmd/soroban-cli/src/commands/contract/info/interface.rs).

## XDR Types That Matter

Primary spec entry union:

- `ScSpecEntry`
- `ScSpecEntryKind`

Entry variants:

- `ScSpecFunctionV0`
- `ScSpecUdtStructV0`
- `ScSpecUdtUnionV0`
- `ScSpecUdtEnumV0`
- `ScSpecUdtErrorEnumV0`
- `ScSpecEventV0`

Function types:

- `ScSpecFunctionV0`
- `ScSpecFunctionInputV0`
- `ScSpecTypeDef`

User-defined type types:

- `ScSpecUdtStructFieldV0`
- `ScSpecUdtUnionCaseV0`
- `ScSpecUdtUnionCaseVoidV0`
- `ScSpecUdtUnionCaseTupleV0`
- `ScSpecUdtEnumCaseV0`
- `ScSpecUdtErrorEnumCaseV0`

Event types:

- `ScSpecEventV0`
- `ScSpecEventParamV0`
- `ScSpecEventParamLocationV0`
- `ScSpecEventDataFormat`

Metadata/environment:

- `ScMetaEntry`
- `ScMetaV0`
- `ScEnvMetaEntry`
- `ScEnvMetaEntryInterfaceVersion`

Deployed contract resolution:

- `LedgerKey`
- `LedgerKeyContractData`
- `LedgerKeyContractCode`
- `LedgerEntryData`
- `ContractDataEntry`
- `ContractCodeEntry`
- `ScVal::ContractInstance`
- `ScContractInstance`
- `ContractExecutable`
- `Hash`

References: [Stellar-contract-spec.x](https://github.com/stellar/stellar-xdr/blob/main/Stellar-contract-spec.x), [Stellar-contract.x](https://github.com/stellar/stellar-xdr/blob/main/Stellar-contract.x), [stellar-xdr Rust docs](https://docs.rs/stellar-xdr/latest/stellar_xdr/).

## Stellar CLI Behavior

Modern Stellar CLI exposes `stellar contract info`:

- `stellar contract info interface --wasm <WASM>` outputs the interface.
- `stellar contract info interface --wasm-hash <HASH>` fetches by WASM hash.
- `stellar contract info interface --contract-id <ID>` fetches by deployed contract ID.
- output formats include `rust`, `xdr-base64`, `json`, and `json-formatted`.

`stellar contract inspect` is documented as deprecated in favor of `contract info`.

CLI source shows interface extraction uses `soroban_spec_tools::contract::Spec`, renders Rust through `soroban_spec_rust`, and handles Stellar Asset Contracts with the embedded `stellar_asset_spec`.

References: [Stellar CLI manual](https://developers.stellar.org/docs/tools/cli/stellar-cli), [Stellar CLI interface command source](https://raw.githubusercontent.com/stellar/stellar-cli/main/cmd/soroban-cli/src/commands/contract/info/interface.rs).

## Deployed Contract and WASM Resolution

For a normal Wasm-backed deployed contract:

1. Build a `LedgerKey::ContractData` for the contract instance:
   - contract address from the contract ID;
   - key `ScVal::LedgerKeyContractInstance`;
   - durability `Persistent`.
2. Call RPC `getLedgerEntries` with that key.
3. Decode the returned `LedgerEntryData::ContractData`.
4. Read `ContractDataEntry.val`, which should contain `ScVal::ContractInstance`.
5. Read `ScContractInstance.executable`.
6. If `ContractExecutable::Wasm`, extract `wasm_hash`.
7. Build `LedgerKey::ContractCode { hash: wasm_hash }`.
8. Call RPC `getLedgerEntries` again.
9. Decode `LedgerEntryData::ContractCode` and read `ContractCodeEntry.code`.
10. Parse `contractspecv0` from the returned WASM bytes.

Stellar Asset Contracts are a special case. They use `ContractExecutable::StellarAsset` and have no uploaded WASM blob. Stellar CLI handles them by using the embedded Stellar Asset Contract spec. v0.1 can exclude deployed SAC comparison; any later RPC mode must make this behavior explicit.

Current XDR also includes `ContractExecutable::ExternalRef` behind CAP-85-era functionality. This should be treated as unsupported or unresolved in early releases until the active-network behavior and RPC/tooling expectations are validated.

References: [RPC getLedgerEntries docs](https://developers.stellar.org/docs/data/apis/rpc/api-reference/methods/getLedgerEntries), [JS Stellar SDK RPC reference](https://stellar.github.io/js-stellar-sdk/reference/network-rpc/), [Stellar-contract.x](https://github.com/stellar/stellar-xdr/blob/main/Stellar-contract.x), [CAP-85](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0085.md).

## RPC Methods Required

Minimum deployed lookup requires:

- `getLedgerEntries` for the contract instance data entry;
- `getLedgerEntries` for the contract code entry.

Useful later:

- `getNetwork` to validate RPC network passphrase and avoid comparing against the wrong network;
- `getLatestLedger` for diagnostics and archival/TTL messaging;
- restore/TTL workflows only if the tool later helps users resolve archived entries.

## Network Differences

The local comparison MVP avoids network differences completely.

For later deployed comparison:

- Mainnet/pubnet requires a real RPC provider; Stellar CLI documentation notes the built-in mainnet config may be a placeholder until the user configures an RPC URL.
- Testnet has public SDF RPC availability and is the best first network for RPC tests.
- Futurenet may expose protocol features before testnet/mainnet and must not be assumed to match mainnet XDR behavior.
- Local Quickstart is useful for deterministic integration tests and fixture deployment.
- Network passphrase must be paired with the RPC endpoint for transaction construction and network validation, even if read-only calls are the first deployed feature.

References: [Stellar networks docs](https://developers.stellar.org/docs/networks), [Stellar CLI troubleshooting](https://developers.stellar.org/docs/tools/cli/agent-cli/reference/troubleshooting), [Quickstart network modes](https://developers.stellar.org/docs/tools/quickstart/network-modes).

## Existing Tool Overlap Audit

Official overlap:

- Stellar CLI can extract and render contract interfaces from local WASM, WASM hash, or contract ID.
- Stellar CLI can output spec XDR and JSON.
- Stellar CLI bindings generation and SDK clients consume the same interface spec.
- Stellar Lab Contract Explorer can show contract spec, meta, env-meta, source/build information, and download WASM.
- `stellar-xdr` CLI can decode XDR streams such as `ScSpecEntry`.

Community/adjacent overlap:

- Soroban Contract Explorer fetches deployed contracts, resolves WASM, extracts `contractspecv0`, and renders function forms.
- `soroban-decompiler` wraps `soroban_spec::read::from_wasm` as part of decompilation.
- `soroban-drift-core` appears to perform partial spec extraction and storage-oriented analysis, but its docs describe incomplete XDR parsing for complex types.
- Security/static-analysis tools overlap with contract quality but not deterministic interface compatibility.

No mature, official, dedicated Soroban ABI/interface compatibility diff tool was found in this discovery pass. Stellar Upgrade Guard is therefore not redundant if it stays focused on deterministic diffing and CI behavior rather than duplicating CLI rendering or explorer UI features.

References: [Stellar CLI manual](https://developers.stellar.org/docs/tools/cli/stellar-cli), [Contract Explorer docs](https://developers.stellar.org/docs/tools/lab/smart-contracts/contract-explorer), [soroban-decompiler docs](https://docs.rs/soroban-decompiler/latest/soroban_decompiler/spec_extract/index.html), [soroban-drift-core docs](https://docs.rs/soroban-drift-core/latest/soroban_drift_core/spec_extractor/fn.extract_spec.html).

## Unresolved Questions

- Confirm the exact released crate versions to pin when implementation begins. As of this discovery, `soroban-spec` and `stellar-xdr` have v28 releases/RCs, and compatibility with Stellar CLI versions must be checked at implementation time.
- Validate event spec emission across currently supported `soroban-sdk` versions and whether all relevant contracts include `ScSpecEventV0` entries in practice.
- Confirm active-network support and expected UX for `ContractExecutable::ExternalRef`.
- Decide whether `ScMetaEntry` differences are warnings only or whether some keys should become policy-configurable.
- Verify how generated specs behave for tuple structs, renamed Rust fields, optional fields, and CAP-86 struct decoding changes using real compiled fixtures.
- Validate whether type names in `lib` fields are stable enough to include in compatibility identity or should be normalized primarily by referenced UDT name.
