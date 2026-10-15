# Build the effect control owner's candidate at begin

Successor of #1611 (its decision record), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as #1624). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

An effect control owner (the EQ's target owner) keeps no candidate copy and dirty array between
transactions, so the browser's boot and retained memory shrink by about 1.15 MB on the 1 MiB worst
document.

## Context

- `EffectControlOwner` keeps a `candidate` copy and a dirty array per EQ, about 780 bytes each,
  about 600 KB to 1.15 MB on the worst document (#1611's decision record).
- Both are used only while a transaction stages a value.

## Decisions to take (the implementer proposes, the coordinator rules)

- **Option A.** Build the candidate and the dirty array in `begin` from one control-thread scratch
  the control plane owns (one per session, sized to the largest owner), and drop them at commit.
- **Option B.** Keep them per owner but allocate on first use and charge them.
- Recommendation: A. Render is not affected; the owner is control-thread state.

## Objective gates

1. Boot growth (`scripts/check-web-boot-budget.mjs`) and native retained bytes fall by the
   measured amount, recorded; the multiplier 20 holds.
2. The EQ live-edit tests (C ABI and browser parity) pass unchanged.
3. `cargo test --locked -p effect-contract -p control-plane --features test-support -p capi -p
   host-web --features host-web/test-support`.

## Authorized paths

- `crates/effect-contract/src/live.rs` (the owner), `crates/control-plane/src/control.rs`, their
  tests, resource ceilings with measured numbers, this spec's record.

## Dependencies

- #1611.

## Size

Half a day.

## Decision record: #1624 (batch 14b)

**Stopped once before coding.** The spec's Option A (one scratch sized to the largest owner) cannot
hold: `commit_live` step 3 begins every touched EQ owner before any commits, so the open candidates
overlap (up to every EQ, 768 on the worst document). The owner is in
`crates/effect-compiler/src/control.rs`, not in the spec's paths. The coordinator ruled option B'
and approved the added paths (`effect-compiler/src/control.rs`, `prepare.rs`,
`host-core/tests/prepare.rs`, the snapshot doc in `control-plane/src/control.rs`).

**Shipped (B').** `EffectControlOwner` holds `candidate: Option<OpenCandidate>` (rows + touched mask).
`begin` builds it from the committed rows; `commit`, `discard` and a publication fault drop it, and
`OpenOwners::drop` already discards every begun owner on a refusal. `candidate()` and `dirty()`
return an empty slice while no candidate is open. `resource_facts` drops the two terms (an idle
owner retains the box and its committed rows only). `new` keeps a transient all-false mask for its
validation call.

**Departure.** The coordinator asked for the rows and mask in one allocation. They are two exact
`Box<[T]>` (720 and 60 bytes per EQ): `EffectTargetRequest` takes `values: &[InitialParameterValue]`
and `changed: &[bool]` as separate slices, and one allocation would need `unsafe` or a contract
change. Cost: two allocations per begun owner per transaction.

**Measured (x86-64, eight lanes).**

| Figure | Before | After |
|---|---|---|
| Wasm boot growth, 1 MiB worst document (`check-web-boot-budget.mjs`, shipped simd128 module) | 20,512,768 (313 pages) | 19,857,408 (303 pages), -655,360; the multiplier 20 holds (pin plus one page is 21,037,056) |
| `capi_retained_bytes`, 9-EQ reference session | 166,374 | 159,354 (-7,020 = 9 x 780) |
| effect-control resources (producer table + owner payload) | 19,377 | 12,357 |
| routed-submix `capi` observed (one EQ) | 136,634 | 135,854 |
| Native retained, worst document | not run | 768 EQs x 780 = 599,040 bytes by arithmetic |
| `control_command_heap` steady, 1 track (seven EQ edits on one EQ): allocations / requested / peak | 756 / 103,138 / 72,841 | 758 / 103,918 / 72,841 |
| `control_command_heap` first call, 1 track: allocations / requested / peak | 837 / 129,514 / 77,600 | 839 / 130,294 / 77,600 |

The peak is unmoved (it is reached elsewhere in the call than the open owner); the steady requested
bytes are 82 under their 104,000 ceiling, so a second EQ in that transaction would break it (the
ceiling is the test's own and was not raised).

**Re-pins ("#1624", ceilings only fall).** `resource_lifecycle.rs`: `capi_retained_bytes` ceiling
182,400 -> 175,296 at both widths (159,354 + 10 %, up to 64); `REFERENCE_EFFECT_CONTROL_BUDGET`
21,376 -> 13,632 (12,357 + 10 %, up to 64). The four-lane value is the same (lane-independent
rows; CI prints confirm).

**Gates.** `cargo test --locked` green for effect-compiler, effect-contract, control-plane
(`--all-features`), host-core (`--all-features`), host-web (`--all-features`; includes
`render_locked_*` and `retained_ceilings`, unchanged), capi (including `reference_session_retained_rows_stay_within_their_budgets`
and `capi_retained_bytes_charge_every_byte_the_compile_retains`), and
`parameter-metadata --test control_command_heap`. The EQ live-edit tests (C ABI and browser parity)
pass unchanged. New test (effect-compiler):
`the_candidate_lives_only_between_begin_and_commit_or_discard`, red if `begin` leaves nothing or
`discard` / `commit` keep the candidate (it asserts empty slices when idle and a fresh committed
copy after a discard). `revision_overflow_is_refused_before_candidate_mutation` now asserts no
candidate is built on a refused begin. Updated arithmetic: `prepare.rs` resource tests and
`host-core/tests/prepare.rs` drop the candidate and mask terms. `realtime-policy` ok; clippy
`-D warnings` and fmt clean on the touched crates.
