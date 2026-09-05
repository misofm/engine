# Astra #430 browser fixture resource accounting

Source-only derivation for immutable runtime `7951736605fa64870bc1d91342d00d5fdb6417c5`, current packaging `00e21a94`, `/home/bl/misofm/engine-live-pair-proof`. No build, test, oracle execution or repository edit was performed by this review.

**The justified Wasm resource correction is exactly +8 bytes to each of graphSessionPlusPlanBytes and graphIncrementalPlanBytes: 29286 -> 29294.** The matching native one-track fixture receives +16 bytes to each corresponding graph total relative to its previous native value. Neither is a change to builtinRetainedBytes or bridgeRetainedBytes. The two displayed graph totals overlap in ownership; do not sum their deltas into a16-byte Wasm allocation.

## Independent derivation

- `hosts/host-web/tests/browser-v1/session.json` has exactly one track (`track`), empty three racks and one PostMatrix output route. It is not the nine-track CAPI resource fixture. `hosts/host-web/examples/browser_fixture_resources.rs:19` includes this identical session and boots the same facade with48000Hz/128frames and128 source-ring frames.
- `crates/host-core/src/prepare.rs:769` selects Backend::current. The shipped wasm32 simd128 configuration is W4; native x86-64-v3 is W8 (`crates/lane/src/backend.rs:31`). `planned_strip_banks` and `planned_builtin_bank_members` (`crates/builtins-compiler/src/lib.rs:1038` onward) group each of the three fixed stages independently, retain partial groups and pad absent lanes. One track therefore yields ONE fader bank on either target, not4/8 banks and not zero because the cohort is partial.
- `FaderMatrixBankProcessor` (`lib.rs:661`) contains exactly two typed Box fields. They are thin pointers to concrete sized owners, not trait-object fat pointers. The outer layout is2*4=8 bytes on wasm32,2*8=16 on native64. The original pointed-to fader and matrix allocations/consumer arrays remain charged separately.
- The ONLY changed per-bank resource term is PostFader's `strip_processor_bytes` (`lib.rs:986-1001`): original inline owner + original lane consumer array + sizeof(FaderMatrixBankProcessor). `graph_builtin_bank_resource` (`lib.rs:1930`) multiplies this per-group term through `builtin_bank_resource` (`lib.rs:1148-1206`). The delta is ONE group times8/16; member IDs, strings, descriptor count, scratch and stage counts do not change.
- This is a conservative preparation allowance for each potentially pairable fader bank; it is not conditional on the later concrete graph factory succeeding or on Concurrent versus serialized runtime dispatch. In particular the browser fixture's route/output ownership can prevent actual pairing without removing the admitted allowance. No claim of this one-track fixture executing the composite is needed for its resource calculation.
- `GraphResourceEstimate::checked_add_builtin_banks` (`crates/graph/src/lib.rs:215-242`) adds the payload delta to builtin_bank_bytes and once to EACH incremental_plan_bytes and session_plus_plan_bytes. It does not add it to graph_metadata_bytes. `crates/host-core/src/prepare.rs:906-922` keeps graph totals and original scalar/prepared builtin payload fields distinct; `hosts/host-web/src/lib.rs:2663-2668` copies those separate fields to the browser report.

## Cross-check with the independent CAPI mirror

`crates/capi/tests/resource_lifecycle.rs:1100-1108` independently spells the outer structure as two typed Box pointers. The fader processor owner row at1727-1732 adds that mirror size once per builtin bank while retaining both originals. Its nine-track native fixture has ceil(9/8)=2 fader banks, so its total increase is2*16=32 bytes, as the explicit frozen report comment at512 states. That +32 is not the browser one-track fixture delta. Its exact/one-below cap tests retain the independent resource consequences.

## Permitted current pin correction and limits

In `hosts/host-web/tests/browser-v1/expected.json`, change only:

- `directOracle.simd128.resources.graphSessionPlusPlanBytes`: `"29286"` -> `"29294"`.
- `directOracle.simd128.resources.graphIncrementalPlanBytes`: `"29286"` -> `"29294"`.

The allocation allowance is graph payload, so builtinRetainedBytes985, graphMetadataBytes3455, bridge metadata/retained rows, source/effect/observation rows and ABI structure sizes have no corresponding accounting delta. The named maximum is dominated by the existing16384-byte diagnostic allocation; this8-byte allowance does not justify a maximum-row repin. WebAssembly memory page counts are actual artifact/runtime observations, not a linear byte-accounting formula; preserve their old values unless independent execution shows and explains a separate change.

Root reports the actual resource gate failed only on these two +8 differences, while its26 red controls passed, and the direct oracle independently confirms all other fields and PCM unchanged. That report is consistent with the source derivation above; it is not used to invent the allowance. Preserve the original failure/oracle and rerun the existing resource comparator after the narrowly justified two-row correction. No PCM/digest/timeline/status pin, numeric tolerance, comparator partition, historical resource record or production algorithm change is authorized by this accounting note.
