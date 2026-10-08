# Compatibility Engine Audit

Phase 2.5 audit status: local deterministic compatibility engine only. No RPC, deployed lookup, storage migration analysis, or semantic/runtime analysis is included.

## Matching Strategy

Comparison direction is `old -> new`, representing the intended upgrade direction.

Top-level functions, structs, enums, error enums, unions, and events are matched by normalized name. Function arguments are matched by name unless the old and new functions have the same argument count and all positional types are unchanged; that case is classified as argument rename warnings. Exact same argument set in a different order is classified as argument reordered.

This means `foo(a: u32) -> foo(b: u64)` intentionally reports argument `a` removed and argument `b` added, instead of guessing that a rename and type change occurred. The engine avoids position-based cascades unless the evidence is unambiguous.

Findings are sorted by severity, finding code, path, and message. Severity order is `Breaking`, `Unknown`, `Warning`, then `Compatible`.

Overall result policy:

- any `Breaking` finding => `Breaking`;
- else any `Unknown` finding => `Unknown`;
- else any `Warning` finding => `Warning`;
- else `Compatible`.

## Rule Matrix

| Finding code | Old state | New state | Classification | Reason | Test/fixture evidence | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| `FunctionAdded` | Function absent | Function present | Compatible | Existing callers can continue using old functions. | `function_addition_is_compatible`, `real_wasm_function_addition_is_compatible`, `compatible-function-added` | High |
| `FunctionRemoved` | Function present | Function absent | Breaking | Old callers cannot invoke the removed function. | `function_removal_is_breaking`, `real_wasm_function_removal_is_breaking`, `breaking-function-removed` | High |
| `ArgumentAdded` | Argument absent | Argument present | Breaking | Old callers will not provide the new positional argument. | `argument_addition_and_removal_are_breaking` | High |
| `ArgumentRemoved` | Argument present | Argument absent | Breaking | Old callers will provide an argument no longer accepted. | `argument_addition_and_removal_are_breaking` | High |
| `ArgumentRenamed` | Same position/type, old name | Same position/type, new name | Warning | Low-level invocation is positional, but generated clients and humans use names. | `argument_rename_is_warning` | Medium |
| `ArgumentReordered` | Same argument set in one order | Same argument set in another order | Breaking | Position maps values to different arguments. | `argument_reorder_is_breaking` | High |
| `ArgumentTypeChanged` | Argument has old type | Same named argument has new type | Breaking | `ScSpecTypeDef` changed for a callable input. | `argument_changes_are_detected`, `nested_composite_type_change_is_detected`, `real_wasm_argument_type_change_is_breaking`, `breaking-argument-type-changed` | High |
| `ReturnTypeChanged` | Old output type list | New output type list | Breaking | Clients decode returned values according to old output spec. | `return_type_change_is_breaking`, `multiple_return_value_shape_is_compared_if_present`, `real_wasm_return_type_change_is_breaking`, `breaking-return-type-changed` | High |
| `StructAdded` | Struct absent | Struct present | Warning | Additive UDT may affect generated clients/docs but does not remove old interface. | `struct_add_remove_and_field_removal_are_classified_conservatively` | Medium |
| `StructRemoved` | Struct present | Struct absent | Breaking | Existing functions/clients may reference the removed UDT. | `struct_add_remove_and_field_removal_are_classified_conservatively` | Medium |
| `StructFieldAdded` | Field absent | Field present | Unknown | Soroban struct shape tolerance needs more validation. | `struct_field_addition_is_unknown` | Medium |
| `StructFieldRemoved` | Field present | Field absent | Unknown | Soroban struct shape tolerance needs more validation. | `struct_add_remove_and_field_removal_are_classified_conservatively` | Medium |
| `StructFieldRenamed` | Same position/type, old name | Same position/type, new name | Unknown | Rename may affect map keys/generated clients; semantics need more evidence. | `struct_field_rename_and_reorder_are_unknown` | Medium |
| `StructFieldReordered` | Same fields in one order | Same fields in another order | Unknown | Named field serialization may be order-insensitive, but not promoted without evidence. | `struct_field_rename_and_reorder_are_unknown` | Medium |
| `StructFieldTypeChanged` | Field has old type | Same named field has new type | Breaking | Field `ScSpecTypeDef` changed. | `struct_field_type_change_is_breaking`, `real_wasm_struct_field_change_is_breaking`, `struct-field-changed` | High |
| `EnumAdded` | Enum absent | Enum present | Warning | Additive UDT may affect generated clients/docs. | Covered by named-item engine; no real fixture yet | Medium |
| `EnumRemoved` | Enum present | Enum absent | Breaking | Existing functions/clients may reference removed enum. | Covered by named-item engine; no real fixture yet | Medium |
| `EnumCaseAdded` | Case absent | Case present | Unknown | Generated exhaustive client behavior needs more validation. | `enum_order_only_produces_no_findings_and_case_addition_is_unknown` | Medium |
| `EnumCaseRemoved` | Case present | Case absent | Breaking | Old values using the case may no longer be valid. | Covered by enum case engine; no real fixture yet | High |
| `EnumCaseValueChanged` | Case numeric value old | Case numeric value new | Breaking | `SCV_U32` meaning changes. | `enum_and_error_changes_are_detected`, `multiple_changes_report_counts_and_order_are_stable` | High |
| `ErrorAdded` | Error enum absent | Error enum present | Warning | Exposed error surface changed but not callable input shape. | Covered by named-item engine; no real fixture yet | Medium |
| `ErrorRemoved` | Error enum present | Error enum absent | Warning | Exposed error surface changed; not always call-breaking. | Covered by named-item engine; no real fixture yet | Medium |
| `ErrorCaseAdded` | Error case absent | Error case present | Warning | New possible error code may require client updates. | Covered by error case engine; no real fixture yet | Medium |
| `ErrorCaseRemoved` | Error case present | Error case absent | Unknown | Removing exposed error code semantics need validation. | Covered by error case engine; no real fixture yet | Medium |
| `ErrorCodeChanged` | Error code old | Error code new | Breaking | Machine interpretation of contract error changes. | `enum_and_error_changes_are_detected` | High |
| `UnionAdded` | Union absent | Union present | Warning | Additive UDT may affect generated clients/docs. | Covered by named-item engine; no real fixture yet | Medium |
| `UnionRemoved` | Union present | Union absent | Breaking | Existing functions/clients may reference removed union. | Covered by named-item engine; no real fixture yet | Medium |
| `UnionCaseAdded` | Union case absent | Union case present | Unknown | Union compatibility semantics need more evidence. | `union_and_event_uncertain_changes_are_unknown` | Medium |
| `UnionCaseRemoved` | Union case present | Union case absent | Breaking | Old values using removed case may no longer be valid. | Covered by union case engine; no real fixture yet | High |
| `UnionCasePayloadChanged` | Case payload old | Case payload new | Breaking | Associated payload types changed. | Covered by union case engine; no real fixture yet | High |
| `EventAdded` | Event absent | Event present | Unknown | Event consumer/indexer compatibility semantics need more evidence. | Covered by named-item engine; no real fixture yet | Medium |
| `EventRemoved` | Event present | Event absent | Unknown | Event consumers may rely on removed event shape. | Covered by named-item engine; no real fixture yet | Medium |
| `EventParametersChanged` | Event params old | Event params new | Unknown | Event decoding/indexing shape changed. | `union_and_event_uncertain_changes_are_unknown` | Medium |
| `EventDataFormatChanged` | Event data format old | Event data format new | Unknown | Event data decoding changed. | `union_and_event_uncertain_changes_are_unknown` | Medium |

## Composite Type Audit

Parser tests verify normalized rendering for:

- `Option<T>`
- `Vec<T>`
- `Map<K, V>`
- tuple values
- `Result<Ok, Error>`
- `Bytes`
- `BytesN<N>`
- `Address`
- `Symbol`
- `String`
- signed and unsigned integers through existing function/type tests
- custom UDT references

Nested type changes such as `Option<u32> -> Option<u64>` are detected as `ArgumentTypeChanged` because normalized type strings preserve the nested type structure.

## Determinism And Directionality

Tests verify repeated comparisons produce identical `CompatibilityReport` values and identical JSON serialization. Directionality is tested with function removal in `old -> new` and function addition in `new -> old`.

Declaration order changes for functions and enum cases do not create findings because normalized comparisons use stable name-based maps where order is not semantically meaningful.

## Fixture Binary Policy

Phase 2 fixture WASM files are small and useful for offline deterministic integration tests. Each committed WASM has matching source and a `Cargo.toml` under the same fixture pair.

| Fixture WASM | Size |
| --- | ---: |
| `fixtures/phase2/breaking-argument-type-changed/old.wasm` | 545 bytes |
| `fixtures/phase2/breaking-argument-type-changed/new.wasm` | 600 bytes |
| `fixtures/phase2/breaking-function-removed/old.wasm` | 624 bytes |
| `fixtures/phase2/breaking-function-removed/new.wasm` | 569 bytes |
| `fixtures/phase2/breaking-return-type-changed/old.wasm` | 525 bytes |
| `fixtures/phase2/breaking-return-type-changed/new.wasm` | 522 bytes |
| `fixtures/phase2/compatible-function-added/old.wasm` | 569 bytes |
| `fixtures/phase2/compatible-function-added/new.wasm` | 624 bytes |
| `fixtures/phase2/struct-field-changed/old.wasm` | 754 bytes |
| `fixtures/phase2/struct-field-changed/new.wasm` | 820 bytes |

The binaries were generated with `stellar contract build --manifest-path <fixture>/Cargo.toml` using Stellar CLI 28.1.0 as reported by the build environment. Keeping these small binaries is justified because real-WASM integration tests can run without rebuilding contracts on every machine.

Do not commit full `target/` or `.build-target/` directories.

## Stellar CLI Cross-Check

Stellar CLI `contract info interface --wasm` confirms the real fixture interfaces:

- `compatible-function-added`: `new.wasm` adds `goodbye() -> Symbol`.
- `breaking-function-removed`: `old.wasm` has `removed() -> Symbol`; `new.wasm` does not.
- `breaking-argument-type-changed`: `set(value: u32)` changes to `set(value: u64)`.
- `breaking-return-type-changed`: `value() -> u32` changes to `value() -> u64`.
- `struct-field-changed`: `Account.id: u32` and function return `u32` change to `Account.id: u64` and function return `u64`.

The CLI emits informational text while loading specs; JSON output from `stellar-upgrade-guard --format json` remains clean stdout with errors on stderr.
