# Bound C ABI live effect windows by lane depth (owner question)

**Do not start before the owner answers.** This issue holds owner question Q5 of the #1053 live
updates batch, raised as an FYI by the verdict of *Prepare C ABI plans with live effect lanes*
(#1263; `docs/handoffs/live-updates-1053/1263-attempt1.md`, question 1 and MINOR 2; recorded as
Q5 in that directory's `README.md`). The implementation slice below applies only after a "yes".

## Owner question

**Should a C ABI plan size each effect's live-control span window by the effect lane's depth (16)
instead of by the caller's `maximum_automation_spans_per_block` (S)?**

**Planner recommendation: yes.** On the C ABI, nothing but the live lane stages spans into an
effect today, and one drain of a 16-deep lane stages at most 16 spans per lane. Every span slot
above 16 is memory no block can use. On the nine-track reference session the graph rows would
shrink by 80,640 bytes at eight lanes, and the saving grows linearly with S (about 1.4 MB per
eight-lane bank at S = 4,096). This is a mobile memory question, not a defect.

**What "yes" commits to.**

- On the C ABI, an effect's per-block automation capacity becomes `min(S, 16)`. S keeps its other
  role, the protocol's per-block automation density bound (below).
- When a host first renders stored automation on the C ABI (#1058), that issue must size the
  capacity for those spans too, and pay the window again. This spec's Decisions section records
  that for it.
- The browser is unaffected: it derives its own S (below).

**What "no" means.** Nothing changes. The qualification doc already states the cost and its
formula (`docs/C_ABI_V1_QUALIFICATION.md:357-363`), so a caller can size S small. Close this issue.

## Problem (verified on `main` at `d2fe0555a`)

**Where S goes on the C ABI.**

- `prepare_caps` copies S into the host-core caps unchanged
  (`crates/capi/src/runtime/compile.rs:523`, field at `:528`). Effect preparation makes it each
  effect's `automation_capacity` (`crates/effect-contract/src/lib.rs:2563`).
- S is also the protocol's `per_block_automation_density` (`compile.rs:130-133`), which bounds an
  admitted automation batch (`crates/protocol/src/queue.rs:1100`). That role is not touched here.
- The C header has the bare field and no prose about it
  (`crates/capi/include/miso_engine_v1.h:166`). No header or doc text promises that S covers live
  spans; the qualification doc only describes the cost (`docs/C_ABI_V1_QUALIFICATION.md:344-366`).
- No C ABI path renders stored automation (#1058; the classifier's automation mask holds "only
  while no host renders stored automation", `crates/host-core/src/live_delta.rs:234-236`). In
  `crates/capi/src` the protocol automation queue is only reported (`control.rs:668`), never
  drained into a plan.

**How the window follows S.**

- Each effect lane's ring holds `min(LIVE_QUEUE_DEPTH, automation_capacity)` records
  (`crates/effect-compiler/src/prepare.rs:1400`; `LIVE_QUEUE_DEPTH` = 16,
  `crates/capi/src/runtime/compile.rs:11-13`). Each record stages at most one span, so one drain
  stages at most 16 spans per lane.
- But the banked live-control stage allocates a one-lane staging window and a packed window of
  `automation_capacity` spans per lane (`crates/rack/src/lib.rs:1023`), and refuses any other
  size: the pairing rule of #1012 requires the window to equal the capacity
  (`check_window`, `crates/effect-contract/src/lib.rs:1273-1279`; the bank form at `:1398-1418`).
  The per-node form does the same (`crates/graph/src/runtime.rs:1214`).
- The estimate charges exactly that (`effect_control_resource`,
  `crates/graph-compiler/src/estimate.rs:188`; bank term `:300-340`): per live bank, the stage's
  growth (176), the lane array (72 per lane), the staging window (40 x S), the packed window
  (40 x S x lanes) and the dry shunt.

**The numbers** (span 40 bytes; a zero-latency effect at a 128-frame quantum, so the shunt is
8,192 bytes at eight lanes and 4,096 at four):

| unit | formula | S = 128 | S = 16 (capped) | S = 4,096 |
|---|---|---|---|---|
| eight-lane bank | 8,944 + 360 x S | 55,024 | 14,704 | 1,483,504 |
| four-lane bank | 4,560 + 200 x S | 30,160 | 7,760 | 823,760 |

On the reference session (nine console-slot EQs, 2,104 bytes per member, S = 128):

| width | banks | move today | move capped | graph rows today | graph rows capped (predicted) |
|---|---|---|---|---|---|
| eight lanes | 2 | 128,984 | 48,344 | 382,918 / 185,121 | 302,278 / 104,481 |
| four lanes | 3 | 109,416 (derived) | 42,216 | not yet measured | today's minus 67,200 |

"Graph rows" are `graph_session_plus_plan_bytes` (and `graph_incremental_plan_bytes`) /
`graph_metadata_bytes`. The eight-lane rows today were measured on `d2fe0555a`. The per-member
term does not change: the ring is already 16 deep.

**The browser** derives its S as the larger of its stored automation segments and its live queue
records (`hosts/host-web/src/lib.rs:6574-6582`), so it already pays about 16 spans per lane. It
does not use `prepare_caps` and is out of scope.

## Decisions (apply after "yes")

- **D1. Cap the effect capacity in `prepare_caps`.** `prepare_caps` passes
  `min(limits.maximum_automation_spans_per_block, LIVE_QUEUE_DEPTH)` as the host-core
  `maximum_automation_spans_per_block`. The window then still equals the capacity, so the #1012
  pairing rule, the effect contract, `rack`, `graph` and the estimate are untouched. Do not change
  `compile.rs:130-133` (the protocol density keeps S).
  - *Rejected:* keeping the capacity at S and sizing only the windows at the lane depth. It would
    relax `check_window`'s equality in `effect-contract`, which the browser and the per-node path
    share, for no extra saving.
- **D2. Why this keeps every bit.** A lane stages at most 16 spans per block either way, and every
  launch effect's cut-off is `span_index < automation_capacity`, so no staged span crosses it. The
  capacity is equal on every track, so `EffectProgramKey`s stay equal and no cohort changes. The
  gates prove it.
- **D3. Leave a note for #1058.** One sentence in the `prepare_caps` doc comment and in the
  qualification doc: the cap holds only while the C ABI renders no stored automation; the issue
  that renders it must size the capacity for those spans.
- **D4. Budgets.** The three graph rows fall by a structural move; lower their eight-lane ceilings
  to the new measured values plus 10 %, rounded up to 64, with the reason in the
  `REFERENCE_BUDGETS` doc comment (`crates/capi/tests/resource_lifecycle.rs:1039-1053`). Lower the
  four-lane ceilings the same way from the AArch64 debug job's printed rows (see Dependencies).
- **D5. Docs.** Rewrite the formula bullet of `docs/C_ABI_V1_QUALIFICATION.md:357-363`: the bank
  term is `8,944 + 360 x min(S, 16)` at eight lanes and `4,560 + 200 x min(S, 16)` at four, and
  the reference move is the capped one. Record the owner's answer in
  `docs/handoffs/live-updates-1053/README.md` (Q5).

## Authorized paths (after "yes")

- `crates/capi/src/runtime/compile.rs`: `prepare_caps` and its doc comment only.
- `crates/capi/tests/resource_lifecycle.rs`: `host_caps` (`:524`, which mirrors `prepare_caps`
  field for field), the graph-row ceilings and their doc comment, and the new gate-2 test.
- `docs/C_ABI_V1_QUALIFICATION.md` (the #1263 section), `docs/handoffs/live-updates-1053/README.md`
  (Q5 only).
- This spec.

## Non-goals

- The browser host, the effect contract, `rack`, `graph`, `graph-compiler` and the effects.
- The protocol's automation density, and the C ABI struct or header.
- Rendering stored automation (#1058).
- Any change for S below 16: `min(S, 16)` equals S there.

## Hazards

- **`host_caps` mirrors `prepare_caps`.** The resource oracle in `resource_lifecycle.rs` builds its
  host half from its own copy of the mapping. If only `prepare_caps` changes, the oracle's model
  and graph figures diverge from the plan's, and the exact-charge tests go red for the wrong
  reason. Change both.
- **Live queue depth.** `min(LIVE_QUEUE_DEPTH, capacity)` stays 16 for S >= 16 and stays S below.
  The over-capacity rebuild of #1264 (`an_effect_edit_larger_than_its_queue_rebuilds`) and the
  EQ's 12-target limit must still hold.
- **Tests that set S = 4.** `crates/capi/src/runtime/live_tests.rs:2417-2424` and `:2810` rely on a
  four-deep queue; `min(4, 16)` keeps it.

## Objective gates

1. **No rendered bit moved.** `audit capi` (`cargo build --locked --release -p audit -p capi`, then
   `target/release/audit capi`) at base and head: allocations, deallocations, locks, syscalls and
   `total_violations` 0, and the same `pcm_digest`. One-time PR evidence, not a committed pin.
   `cargo test --locked -p capi` passes, including every `live_tests` case and
   `c_abi_plans_with_live_lanes_render_like_lanes_free_plans`.
2. **The window no longer follows S (new committed test).** Compile the reference session at S =
   128 and at S = 4,096; the three graph rows are equal. At S = 8 they are smaller than at S = 16.
   *Test value: red if a C ABI live window is sized by the caller's S again, which no existing test
   catches because every C ABI resource test runs at S = 128.*
3. **Exact numbers recorded (PR evidence).** The reference session's graph rows at eight lanes
   (predicted 302,278 / 104,481) and at four lanes (from the AArch64 debug log), before and after,
   and every other row unchanged. A difference from the prediction is explained.
4. **Budgets.** `reference_session_retained_rows_stay_within_their_budgets` passes at both widths
   with D4's ceilings.
5. **Allocation-free render.** Gate 1's audit, and the AArch64 release job's `audit capi`.
6. **Policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked -p capi --all-targets --all-features -- -D warnings`
   - `bash scripts/check-capi-abi.sh` (no header change expected)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`

## Evidence

- The owner's answer, quoted with its date.
- Gates 1 and 3.

## Dependencies

- **The owner's answer to this question.**
- *Tighten the four-lane reference graph ceilings from measured AArch64 rows* (#1304), which makes
  the AArch64 debug job print the four-lane rows. Without it, D4's four-lane ceilings cannot be
  measured.

## Standing rules for the implementer

- Do not start before the owner answers. On "no", close the issue with the answer recorded.
- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- No allocation, lock or unbounded work on the render thread.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
