# Size each effect's automation span window from the producers its plan has

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-5).
Code anchors verified on `main` at `6fb211594`.

This issue started as owner question Q5 of the #1053 batch (`docs/handoffs/live-updates-1053/README.md:42`):
should a C ABI plan size each effect's span window by the lane depth (16), rather than by the
caller's `maximum_automation_spans_per_block` (S)? **Decision 15, D15-5 answers it, so the owner
question is closed.** The answer is to size the window once, from the span producers the plan
really has: the live parameter cells of *Hold effect parameter, bypass and EQ-target values in
latest-target cells* (#1345), plus the stored-automation spans per block that preparation computes. The caller's S then bounds only `AUTOMATION_ENQUEUE` density. The old draft's
`min(S, 16)` cap is superseded by D15-5. That cap would have needed a second resize when stored
automation renders, which is the kind of later fix the owner principle forbids. The issue was
retitled for decision 15 (formerly *Bound C ABI live effect windows by lane depth (owner
question)*).

**Not ready for implementation** until *Research: render stored session automation in the engine,
identically on every platform* (#1058) records its answers A7 (the stored span bound per effect
instance) and A8 (whether `AUTOMATION_ENQUEUE` stages spans into effect windows, and its bound).
D1's `stored` term and its `AUTOMATION_ENQUEUE` term are those answers, unchanged.

## Product outcome

Each prepared effect's per-block automation span window is sized from its plan's own span
producers, on both hosts. The caller's S stops costing memory: on the nine-track reference session
the C ABI graph rows fall by about 60 KB at S = 128, and they no longer grow with S (about 1.4 MB
per eight-lane bank at S = 4,096 today).

## Context

**Where S goes today.**
- capi copies S into the host-core caps unchanged (`prepare_caps`, which #1309 moved to
  `crates/control-plane/src/compile.rs:531`, field at `:536`). The field is
  `HostPrepareCaps::maximum_automation_spans_per_block` (`crates/host-core/src/prepare.rs:110`).
  Host-core passes it on as `EffectCompileCaps::maximum_automation_spans_per_block`
  (`prepare.rs:1350`), and effect preparation makes it each effect's `automation_capacity`
  (`crates/effect-contract/src/lib.rs:2563`).
- S is also the protocol's `per_block_automation_density` (`compile.rs:158-161`). That role stays.
- The browser derives its own S: its stored segment count, or its live queue records, whichever is
  larger (`hosts/host-web/src/lib.rs:6574-6583`, passed at `:6591`). No host renders stored
  automation yet (#1058), so the segment term is a producer that does not exist.

**How the window follows the capacity.**
- Today an effect lane's ring holds `min(depth, automation_capacity)` records
  (`crates/effect-compiler/src/prepare.rs:1360-1365` and `:1399-1400`; the C ABI depth is
  `LIVE_QUEUE_DEPTH` = 16, `compile.rs:13-17`). Each record stages at most one span. #1345
  replaces that ring with one latest-target cell per `(parameter_index, channel)` of every
  `Block`-rate parameter (#1345 D1), and its render stages at most `automation_capacity` parameter
  spans per block, deferring any excess dirty cell (#1345 D4). So the live producer of spans is
  the cell count, not a queue depth.
- The parametric EQ has 22 `Block`-rate parameters, all `PerLane`: four automatable fields of
  each of its four bands and its six cut parameters (`crates/parametric-eq/src/lib.rs:491-528`,
  `:530-561`, table at `:564`). It therefore has 44 live parameter cells.
- The banked live-control stage allocates a staging window and a packed window of
  `automation_capacity` spans per lane (`crates/rack/src/lib.rs:1023`). The #1012 pairing rule
  requires the window to equal the capacity (`check_window`, `crates/effect-contract/src/lib.rs:1273-1279`;
  the bank form is at `:1398-1418`). The per-node form does the same (`crates/graph/src/runtime.rs:1214`).
- The graph estimate charges exactly that (`effect_control_resource`,
  `crates/graph-compiler/src/estimate.rs:188`; the bank term is at `:296-340`): per eight-lane bank
  `8,944 + 360 x S` bytes, and per four-lane bank `4,560 + 200 x S`, at a 128-frame quantum.
- `automation_capacity` is part of `EffectProgramKey` (`effect-contract/src/lib.rs:1171`, field at
  `:1185`), so two instances bank together only when their capacities are equal.
- The effect contract refuses a capacity of 0 (`effect.prepare.capacity`,
  `effect-contract/src/lib.rs:2466-2475`).
- Effect preparation takes one session-wide value: `EffectCompileCaps::maximum_automation_spans_per_block`
  (`crates/effect-compiler/src/prepare.rs:22-27`). `prepare_with_console_eligibility` refuses it
  at 0 (`:309-317`) and copies it into every instance's `PrepareEffectLimits` (`:459-463`).
  `EffectCompileCaps` is `Copy`, and more than 50 test and tool literals build it.

**Reference numbers** (`crates/capi/tests/resource_lifecycle.rs:1039-1053`; nine console-slot EQs,
S = 128, eight lanes): at `6fb211594`, `graph_session_plus_plan_bytes` and
`graph_incremental_plan_bytes` are 382,918, and `graph_metadata_bytes` is 185,121. #1345 moves
these rows (cells replace the queue and the target staging), so this issue's base is the rows
measured after #1345. A capacity of 44 instead of 128 makes the two eight-lane banks
2 x 360 x 84 = 60,480 bytes smaller than that base, if #1345 keeps the bank's 360-byte per-span
term.

## Decisions frozen for this slice

- **D1. One sizing function, in host-core preparation.** For each prepared effect instance:
  `capacity = live + stored`. Its terms:
  - `live` is the instance's live parameter cell count when it has a live lane, and 0 when it
    has none. The count is #1345 D1's: one cell for each `Block`-rate `Shared` parameter and two
    for each `Block`-rate `PerLane` one (44 for the parametric EQ). Prepared-target cells and the
    bypass cell stage no span and do not count. With this term #1345 D4's deferral never fires
    for a live edit, because the window holds every live cell;
  - `stored` is #1058's answer A7: the per-block span bound for the instance's stored
    automation, evaluated at preparation;
  - if #1058's answer A8 routes `AUTOMATION_ENQUEUE` batches into effect span windows, their
    per-block bound from A8, at most S, is a third term; if A8 says they do not, there is none;
  - the result is at least 1, the contract's minimum.

  This issue owns the function. The slice that renders stored effect automation (#1058's answer
  A11 names it) evaluates A7 as a summand of this function and adds no second sizing.
- **D2. Cohorts are kept.** Every instance of the same native effect identity and quality in one
  session gets the maximum of their D1 values, so no bank cohort splits. A console slot's
  instances are covered by the same rule.
- **D3. S leaves the preparation caps.** Remove `HostPrepareCaps::maximum_automation_spans_per_block`.
  The parts that change:
  - effect preparation takes a per-instance capacity. `prepare_with_console_eligibility`
    (`crates/effect-compiler/src/prepare.rs:301`) gains a capacity function of the instance's
    native effect identity and quality, and puts its value in each `PrepareEffectLimits`
    (`:459-463`). `EffectCompileCaps` keeps its shape and `Copy`, so no literal changes: its
    `maximum_automation_spans_per_block` becomes a ceiling, and a capacity of 0 or above it is
    refused with `effect.resource.limit` (`:309-317`). `prepare_native_session_effects` (`:270`)
    passes the ceiling as every instance's capacity, as today. A new
    `prepare_native_session_effects_with_automation_capacity` takes the function;
  - host-core (`crates/host-core/src/prepare.rs:1344-1352`) calls that entry with D2's function
    and a ceiling of `u32::MAX`: the graph estimate already charges each window's bytes;
  - capi keeps S only for `per_block_automation_density` (`compile.rs:158-161`);
  - the browser's derivation at `hosts/host-web/src/lib.rs:6574-6583` is deleted;
  - every `HostPrepareCaps` literal loses the field's line.

  `ResponsePreviewLimits` (`crates/host-core/src/response.rs:64-73`) is the EQ response preview,
  not a render plan, and is not touched.
- **D4. Why no bit moves.** A block stages at most one span per dirty live cell. Before this
  issue the window is S = 128 (at least the 44 cells), and after it the window is the cell count
  itself, so in both cases no dirty cell is deferred and the same spans are staged in the same
  order. D2 keeps every `EffectProgramKey` equal across a cohort. The gates prove it.
- **D5. No small-queue test lever.** Two tests reached a small effect queue through S = 4
  (`crates/capi/src/runtime/live_tests.rs:2419`, `an_eq_edit_designing_more_targets_than_its_queue_rebuilds`;
  `:2806`, `an_eq_bypass_beside_targets_filling_its_queue_rebuilds`). Both assert the
  queue-capacity rebuild that #1345 D5 removes, so neither survives #1345, and this issue needs no
  test lever. (#1345 gate 6 names the first; the second asserts the same removed rebuild.)
- **D6. Budgets and docs.**
  - Lower the eight-lane graph ceilings to the measured values plus 10 %, rounded up to 64. Give
    the reason in the `REFERENCE_BUDGETS` doc comment. Lower the four-lane ceilings the same way,
    from the rows that the AArch64 debug job prints.
  - Rewrite the formula bullet of `docs/C_ABI_V1_QUALIFICATION.md:357-363`: the bank term uses the
    D1 capacity, not S.
  - Record the answer as Q5 in `docs/handoffs/live-updates-1053/README.md`.

## Deliverables

D1-D6.

## Authorized paths

- `crates/host-core/src/prepare.rs`: the caps field and the effect-caps call (D1-D3).
- `crates/effect-compiler/src/prepare.rs` and `crates/effect-compiler/tests/native_session.rs`:
  the per-instance capacity entry, the ceiling check and gate 2's effect-compiler test (D3). Stream B also edits this file (#1315, #1345), so root sequences the merge.
- `crates/control-plane/src/compile.rs`: `prepare_caps` and `prepare_runtime` (moved there from
  `crates/capi/src/runtime/compile.rs` by #1309; `prepare_caps` then takes `ControlLimits`).
- `hosts/host-web/src/lib.rs`: the caps derivation at `:6574-6591` only. This is stream H's file,
  so root sequences the merge.
- One line in each `HostPrepareCaps` literal: in `crates/host-core/src/` and `crates/host-core/tests/`,
  and in `crates/capi/tests/resource_lifecycle.rs` (where `host_caps` at `:524` mirrors
  `prepare_caps`).
- `crates/capi/tests/resource_lifecycle.rs`: the budgets and their comment, and gate 2's test.
- `docs/C_ABI_V1_QUALIFICATION.md` (the #1263 section); `docs/handoffs/live-updates-1053/README.md`
  (Q5 only); this spec.

## Non-goals

- The effect contract, `rack`, `graph`, `graph-compiler` and the effects: the pairing rule stays.
- The protocol's automation density.
- The C header and the struct layout.
- Rendering stored automation (#1058's slices).
- `ResponsePreviewLimits`.

## Hazards

- **`host_caps` mirrors `prepare_caps`.** If only one of them changes, the exact-charge oracles go
  red for the wrong reason. Change both.
- **The EQ's 12-target limit** must keep refusing as it does after #1345.
- **A cohort split** would change cost, not bits, but it would move the budgets. D2 forbids it.

## Objective gates

1. **No rendered bit moved (PR evidence).** Run `cargo build --locked --release -p audit -p bench -p capi -p session-validator`,
   then `./target/release/audit capi`, at base (after #1345) and at head. Both runs show allocations,
   deallocations, locks, syscalls and `total_violations` of 0, and the same `pcm_digest`. Also:
   - `cargo test --locked -p capi` passes, including every `live_tests` case and
     `c_abi_plans_with_live_lanes_render_like_lanes_free_plans`;
   - the browser legs of the `browser` job in `.github/workflows/qualification.yml` pass, with
     unchanged digests.
2. **The window no longer follows S** (new committed test in `resource_lifecycle.rs`). Compile the
   reference session at S = 128 and at S = 4,096: the three graph rows are equal.
   A host-core unit test of D1's function: an EQ instance with a live lane gets 44, the same
   instance without one gets 1, and a cohort of the two gets 44 (D2).
   An effect-compiler unit test: the capacity function's value reaches each instance's prepared
   `automation_capacity`, and a value of 0 or above the ceiling is refused with
   `effect.resource.limit`.
3. **Exact numbers (PR evidence).** Record the reference graph rows at eight lanes (predicted: the
   post-#1345 base minus 60,480) and at four lanes (from the AArch64 debug log), before and after. Every other
   row must be unchanged, and any difference from the prediction explained.
4. **Budgets.** `reference_session_retained_rows_stay_within_their_budgets` passes at both widths
   with the D6 ceilings.
5. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/check-capi-abi.sh` (no header change expected)
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-web-audioworklet.sh`
   - the workspace test step of `qualification.yml`

## Test value

- Gate 2 turns red if an effect window is sized by the caller's S again. No existing test catches
  that, because every C ABI resource test runs at S = 128.
- The D1 unit test turns red if the live term counts a queue depth, the bypass or target cells,
  or a channel too few, or if a cohort splits.
- The effect-compiler test turns red if preparation keeps one session-wide capacity, or admits a
  capacity the contract or the ceiling refuses.

## Dependencies

- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): its cells
  are D1's live term.
- *Research: render stored session automation in the engine, identically on every platform* (#1058):
  its answer A7 is D1's stored term, and its answer A8 says whether `AUTOMATION_ENQUEUE` feeds
  effect windows and with what bound. This issue lands in the same batch as the slice that
  renders stored effect automation, which #1058's answer A11 names.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Tighten the four-lane reference graph ceilings from measured AArch64 rows* (#1304), for the
  four-lane rows in D6

## Amendment (controls coordinator, batch 10, 2026-10-14)

Root's ruling on finding F23 of the stored-automation check (`docs/handoffs/stored-automation-1058/README.md`,
"Root's rulings (2026-10-14)"). Code anchors read on `2e6343a99`. **This issue is now its D3 only.**

- **The window term is done elsewhere.** #1611 sizes the live bank stage's span window by the widest
  lane's parameter cells, and #1621 sizes the per-node window the same way (its D1); every effect
  has its live lane on every plan (decision 17 C6). Effect preparation already gives each instance
  a capacity of at least its descriptor's live parameter cells
  (`crates/effect-compiler/src/prepare.rs:510-526`), and the lane attach checks it (`:1659-1695`).
  #1058 answered A7 and A8: a window holds one span per cell, automated or not, and
  `AUTOMATION_ENQUEUE` stages no span (it is retired by #1627-#1629). So D1, D2, D4 and D5, the
  "Not ready" paragraph, the A7 and A8 terms, gate 2's D1 unit test and gate 3's 60,480-byte
  prediction are void.
- **What remains: D3, S leaves the preparation caps.**
  - Remove `HostPrepareCaps::maximum_automation_spans_per_block` (`crates/host-core/src/prepare.rs:129`)
    and its copy into `EffectCompileCaps` (`:1903`); every `HostPrepareCaps` literal loses the line.
  - Each instance's `automation_capacity` is `max(1, EffectLaneShape::live_parameter_cells(descriptor))`:
    the `caps.maximum_automation_spans_per_block.max(..)` term at
    `crates/effect-compiler/src/prepare.rs:521-526` loses its S operand. It depends on the
    descriptor alone, so a cohort never splits (old D2 holds by construction).
    `EffectCompileCaps` keeps its shape and `Copy`; its field becomes a ceiling, a capacity above it
    or a ceiling of 0 is refused `effect.resource.limit` (`:358`), and host-core passes `u32::MAX`.
  - capi: `prepare_caps` (`crates/control-plane/src/compile.rs:601`) drops the field. Its other read,
    the protocol's `per_block_automation_density` (`:202`), is deleted by #1629 in batch P1; #1668
    of the #1058 program then reserves the C field.
  - Browser: the derivation from the stored segment count (`hosts/host-web/src/lib.rs:4447-4455`) and
    its two uses (`:4463`, `:4494`) go.
- **Gates now.** (1) No rendered bit moved (PR evidence): `audit capi` gives the same `pcm_digest`
  at base and head, and the browser legs pass with unchanged digests. (2) A committed test in
  `crates/capi/tests/resource_lifecycle.rs`: the reference session compiled at S = 128 and at
  S = 4,096 gives equal graph rows and equal prepared `automation_capacity` per instance. (3) An
  effect-compiler unit test: an instance's capacity is its live parameter cells (at least 1), and a
  ceiling of 0 is refused. (4) Any resource ceiling that moves is re-pinned from measured numbers,
  recorded. (5) Gate 5's policy commands, unchanged.
- **Test value.** Gate 2 turns red if any capacity or window follows the caller's S again; gate 3
  turns red if preparation keeps S as a capacity term.
- **Authorized paths** are old D3's: `crates/host-core/src/prepare.rs` and its literals,
  `crates/effect-compiler/src/prepare.rs` and its tests, `crates/control-plane/src/compile.rs`
  (`prepare_caps`), `hosts/host-web/src/lib.rs` (the derivation and its uses),
  `crates/capi/tests/resource_lifecycle.rs`, `docs/C_ABI_V1_QUALIFICATION.md` (the formula bullet,
  if #1611 and #1621 have not rewritten it), this spec.
- **Dependencies.** #1611 (landed) and #1621 (the per-node window; this issue lands after it). #1629
  (batch P1) removes S's protocol read; this issue and #1668 form batch P2, after P1. The #1058
  rendering slices no longer depend on this issue.

## Current-source implementation record (2026-10-26, attempt 1)

Root approved the amended D3 slice on `8f32d1673`; the user requested Astra
for implementation and root retains independent adversarial review. #1629 is
closed; #1306 and #1668 are delivered as sequential reviewed checkpoints in one PR.
Both host-core preparation paths (main preparation and the arrivals-only compile)
now pass the independent effect ceiling `u32::MAX`. The effect compiler derives
`max(1, descriptor live parameter cells)` and refuses an insufficient ceiling with
`effect.resource.limit`. Response-preview limits retain their existing purpose.

All `HostPrepareCaps` literals lose S, including the current-source literals in
`crates/capi/tests/response_strip.rs`, `tools/bench/src/swap.rs` and host-core's
response fixture (root-approved inventory additions). The browser no longer counts
stored segments; its obsolete, now unread `ControlLimits` S field temporarily
receives zero until #1668 removes the field in the next checkpoint.

Root approved the test-ownership refinement: the temporary CAPI parity test
compares actual FFI graph rows at S = 128 and 4,096. PreparedHost intentionally
exposes no prepared-effect metadata inspector, so per-instance capacity is tested
only by the existing effect-compiler owner test, rewritten in place. No production
inspection API or duplicate preparation test is added. That owner test covers all
ten launch descriptors, exact versus generous ceilings, zero and cells-minus-one
refusals, every-cell staging, and a no-live-cell descriptor's minimum capacity 1.
The metadata-forgery and native-session fixtures use sufficient ceilings, preserving
their original diagnostic claims.

Test value: the owner test fails if caller capacity inflates descriptor capacity,
if insufficient ceilings are accepted, if the minimum becomes zero, or if a dirty
cell cannot fit in its lane's window; the temporary CAPI row fails if caller S
changes host graph resources. #1668 deletes the temporary parity test when S is
reserved. No rendered-output digest is committed as a test.

Baseline `audit capi` (`8f32d1673`): `pcm_digest=bffc21e857b5e134`, 100,000
render calls, zero errors and zero allocations, deallocations, locks, syscalls or
other realtime violations. Local evidence is `/tmp/issue1668-base-audit.json`.
The historical policy command names `scripts/check-realtime-policy.sh` and
`test-realtime-policy.sh`, which no longer exist; this is recorded rather than
creating substitute scripts. The current replacement is `cargo run --locked
--release -p realtime-policy` plus `cargo test --locked -p realtime-policy`
(running at this checkpoint). Host-core and workspace policy checks and mutation
suites pass.

Focused validation passed: capi's complete test suite; effect-compiler and
host-core complete suites (including doctests); the rewritten owner test;
`cargo fmt --all -- --check`; and `git diff --check`. The initial effect-compiler
run exposed old fixture ceilings below EQ's current live-cell requirement; correcting only those
fixtures restored their intended metadata/diagnostic tests.
Head `audit capi` matches baseline `bffc21e857b5e134`, all violation counters zero.
Eight-lane reference graph rows: session-plus-plan 374,654; incremental 374,414;
metadata 114,049. Existing resource ceilings pass unchanged. Broader portability,
browser and workspace qualification run on the coherent #1306/#1668 boundary.

Root adversarial review of `89b6208bc`: **PASS, attempt 1**, conditional on the
shared final browser/portability/workspace qualification. Independent owner-test
mutations fail when caller ceiling replaces descriptor capacity and when the
ceiling check is deleted; the restored owner test passes. Test value: the owner
test catches capacity inflation, insufficient-ceiling acceptance, missing minimum
1 and dropped dirty cells; the temporary CAPI test catches host resource coupling
to S and is deleted by #1668. Review evidence is preserved under
`/home/bl/misofm/engine-work-evidence/1668/root-review/1306-*.log`.
The current realtime-policy binary passes (233 marked regions, 43 files), with
2 unit and 240 policy tests passing.

## Final integration correction (attempt 2, root-approved 2026-10-26)

The provisional component PASS is superseded by **attempt 1 final integration
FAIL**: the full debug-a corpus reached 43 graph-compiler test refusals where
existing launch-effect fixtures passed a ceiling of 32. Current generated
descriptors have 60 live cells for parametric EQ and 96 for dynamic EQ (the old
brief's 44-cell EQ count is historical); every other launch descriptor has at most
22. The bounded correction sets formerly generous launch-fixture ceilings to 128
in graph-compiler's test helpers, retaining custom-registry, delay-only,
multiband-only and third-party-refusal ceilings that were already sufficient.
This changes no named graph, memory or release budget and no negative ceiling gate.
The authorized paths include `crates/graph-compiler/src/lib.rs` (test module),
`src/tests/{bank_padding,console_banking}.rs`, and its existing integration fixtures
`bank_levels`, `bypass_cohorts`, `console_banking`, `console_link_override`, `scale`,
`silent_bank_skip`, `tail_composition` and `bypass_resources`.

One production **tool** caller, `tools/console-workload/src/lib.rs::effect_caps`,
also prepares all launch descriptors; its ceiling becomes 128 for the same reason.
Its functional tests run without timing. Direct audit requests such as dynamic
EQ's 16 and parametric EQ's 48 use `PrepareEffectLimits` through native factories,
not the changed session-compiler ceiling, and remain untouched.

Two bypass_resources tests are deleted as superseded: the per-node 128/4096
capacity comparison and the MAX-capacity/OOM reproducer. Their compiler ceilings
no longer change prepared capacity, so their stated window mutations no longer
turn them red. Measured allocator-accounting and render-allocation tests remain;
direct graph/rack owner tests that vary actual prepared capacity retain the
window-sizing claim, and the effect-compiler owner retains descriptor sizing and
ceiling-refusal claims. No replacement harness or new test is added.

Attempt 2 focused validation: graph-compiler's complete all-targets
`--no-fail-fast` suite passed every target except one indirect `bind(...,32,None)`
ceiling in the retained bypass allocation test. That argument now uses 128; its
four-test target then passes with allocation assertions unchanged. This was
within the same bounded correction pass, before review. The production-tool
console-workload release functional suite passes (56 tests, no timing workload).
The exact workspace debug-a doctest leg also passes. Logs, including the command
preflight errors and the initial indirect-ceiling failure, remain in the external
evidence directory. The final full debug-a run uses `--no-fail-fast` so every
required target runs before a verdict.

## Final aggregate qualification (2026-10-26)

Root's independent final bounded reviews: **#1306 attempt 2 PASS** at
`1f6e12dcf`; **#1668 attempt 2 PASS** at `8a10367ca`. These supersede the
provisional attempt-1 component passes and their recorded integration failures.
The final full `qualification.yml` debug-a workspace/all-target corpus, using its
exact exclusions/features with `--no-fail-fast`, passes on `1f6e12dcf` (exit 0).
Its separate workspace doctest step with the same exclusions/features also passes.

Other completed gates: workspace/all-target/all-feature clippy with warnings
denied; formatting and diff checks; host-core and workspace policies and their
mutation suites; current realtime-policy binary plus 242 tests; C11/C++17 ABI
checker and its mutation suite, including shared/static consumers; and the full
cross-target matrix (iOS/Android checks and lint, iOS memset ratchet, Wasm SIMD,
32-bit/scalar-Wasm refusal rows). The capi/control-plane focused suites pass.
Console-workload's affected production-tool preparation passes its 56 release
functional tests; no timed benchmark was run. The later changes are test fixtures
and that tool helper, outside the engine/browser closure, so the already-passed
product qualification remains applicable as independently reviewed by root.

Production baseline `8f32d1673` and integrated production `1508be064` both give
`audit capi` PCM digest `bffc21e857b5e134` over 100,000 calls, with zero render errors
and zero allocations, frees, locks, syscalls or other realtime violations.
The source-built browser module SHA-256 is
`408bd79a0570609afa12f7578b2149449477c432a9d6908b98ea9332cc9e63af`.
Static/object/closure gates, native/SIMD digest and resource parity, hermetic
host/worklet tests, and current supported Chromium 151.0.7922.34/WebKit 26.5
qualification with matrix checks and red mutations all pass. Expected PCM
digests are unchanged. The checked release matrix retains its own lineage;
`final-browser-matrix.log` is the current-run evidence, linked to the artifact
and source commit by `final-browser-run-receipt.json`.

All logs (including initial failures), browser artifacts and the run receipt
are preserved outside the worktree at
`/home/bl/misofm/engine-work-evidence/1668/astra/`; independent reviewer mutation
evidence is under `../root-review/`. Final full-corpus/clippy logs are
`final-debug-a-complete.log`, `final-debug-a-doc.log` and
`final-clippy-complete.log`. This evidence checkpoint changes only the two issue
specs; root owns PR qualification, merged-body synchronization and explicit
GitHub closure.
