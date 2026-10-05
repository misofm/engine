# Snapshot a running plan into a returned successor at a block

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 step 2).
Slice of *Pre-roll a successor whose latency grows* (#1287), W2. Code anchors verified on `main` at
`6fb211594`.

## Product outcome

The control thread can hand render a prepared warm successor and get it back, holding the
running plan's state as of one block boundary B. Render's cost is one bounded copy. The running
plan's output does not move a bit, and every carried ring is armed so a catch-up can read from
frame B.

## Context

- `RealtimePlanOwner::enter_block` (`crates/engine/src/realtime/plan_exchange.rs:375-441`) has one
  outcome for a candidate today: adopt it.
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311) adds
  `PlanAdoption::{Next, NoEarlierThan, ExactlyAt}` (D1), a fourth cell state `Returned` with a
  `ReturnReason` field in #1343's state word (D3; it leaves room for this issue's reasons), a
  return path that never drops and needs no push (D4), and `Withdrawal::Returned { candidate,
  reason }` from `PlanPublisher::withdraw()` (D6). There is no return queue.
- *Carry plan state by copy as well as by move* (#1322) adds
  `PreparedPlanExecutor::copy_from_predecessor`. It leaves the predecessor's bits unchanged (D2a),
  writes all or nothing (D5), sets the successor's clock to the predecessor's next sample (D1), and
  moves no source consumer (D2c). Its byte budget is `graph::carry_program_copy_bytes` (D6).
- `GraphExecutor::adopt_predecessor` (`crates/graph/src/lib.rs:3139`) and `adopt_sources`
  (`crates/source/src/lib.rs:1851`) are the move-mode source section.
- #1320 adds `PcmSourceConsumer::arm_peek` (D4), which is render-side and store-only.

## Decisions frozen for this slice

- **D1. A fourth adoption mode.** `PlanAdoption::CopyAndReturn { not_before: u64 }` is added to
  #1311's enum.
  1. Render claims such a candidate at the first block it sees it whose first sample is at or after
     `not_before` (0: any block). That block is B. `not_before` lets an edit that also ducks strips
     copy only once the duck has settled (#1287 W10).
  2. Its one compare-and-swap marks the cell `Returned` with `ReturnReason::Copying` (#1311 D3).
     Control never moves a candidate out of a `Copying` cell: `withdraw()` on it returns
     `Withdrawal::InFlight` (#1311 D6), and the control thread looks again at its next call; it
     never waits on render. A `Copying` cell lasts at most the rest of one render callback.
  3. Render runs `copy_from_predecessor(active -> candidate)`, then stores the reason `Copied`, or
     `CopyRefused` on `PredecessorMismatch` (the candidate untouched, counted in
     `carry_mismatch_count`). That store cannot race: control changes no `Copying` cell. The plan,
     its retirement credit, its revision word and its epoch stay in the cell.
  4. The active plan then renders block B as usual.

  `RealtimePlanOwner::copied_count` counts copies, saturating. The control thread that published
  the candidate takes it with `withdraw()` (`Withdrawal::Returned { reason: Copied | CopyRefused }`),
  so a refused copy can never be taken for a copied one.
- **D2. Arming in the copy program.** The graph's copy-mode source section calls `arm_peek` on
  each predecessor consumer the program names. It checks first that every one of them is unarmed,
  so a refusal arms nothing (#1322 D5). It records each ring's arm frame in the successor's carry
  state, for #1355.
- **D3. Host-core warm preparation.** `SuccessorBase` gains `warm: Option<WarmLead>`.
  - **The lead is computed here.** `host_core::warm_lead(predecessor, successor_model)
    -> Result<Option<WarmLead>, PrepareError>` compiles the successor with #1285's floors and no
    lead, and takes `Δ`, the largest growth of arrival over the predecessor at any surviving node
    (same stable node ID in both plans), the output and every submix included. With `Δ <= 0` it
    returns `None` (an ordinary rebuild). Otherwise `lead_samples = ceil(Δ / quantum) * quantum`.
    It is the only place `P` is computed; the control plane calls it at classification (#1360 D1).
  - `WarmLead { lead_samples: u64 }` asks for every surviving node to be floored at predecessor +
    `lead_samples`, as #1287's first slice defines, and for the program to be installed in copy
    mode.
  - Preparation refuses `lead_samples` that is not a positive multiple of the quantum, with a typed
    error.
  - The returned prepared host exposes `lead_samples()`.
  - **Copy budget.** The copy at B runs inside one render callback, so its cost is bounded before
    publication. `COPY_BYTES_MAX`, a host-core constant with the value #1286 D3 item 5 derives (no formula
    is restated here; its comment names the record row), caps `carry_program_copy_bytes` (the warm configuration carries the ceiling, `COPY_BYTES_MAX` by default, so a gate can
    lower it). A successor over the cap is not refused: warm preparation returns
    `WarmUnavailable::CopyBudget`, a typed value distinct from a preparation error, and the edit
    takes the transition (#1287 W7). #1355 adds the other `WarmUnavailable` reasons.
- **D4. Observers.** The copy-mode program copies each carried meter's and observation tap's
  window state into the successor under #1327 D1's keys (#1327 D4), and each spectrum capture's
  state per *Carry spectrum capture state across a plan swap* (#1395). A successor
  copied from B publishes nothing until #1355 adopts it.
- **D5. Acked-batch question.** A returned candidate stays whole in its cell until its publisher
  takes it (#1311 D4), and a refused copy returns it untouched under its own reason. A warm successor is never adopted in this slice. An ack can never
  precede a drop.

## Deliverables

1. D1 in `crates/engine/src/realtime/plan_exchange.rs`.
2. D2 and D4 in the graph copy program, plus the observer copy sections in rack and builtins
   where #1327 put the move sections.
3. D3 in `crates/host-core/src/prepare.rs`.
4. `crates/host-core/tests/warm_successor.rs` (new) with gates 1-4.

## Authorized paths

- `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/mod.rs`
- `crates/graph/src/lib.rs` (copy-mode source and observer sections), `crates/source/src/lib.rs`
  (the driver's arm call)
- The observer copy sections beside #1327's move sections in `crates/rack/src/` and
  `crates/builtins/src/`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/warm_successor.rs` (new)

## Non-goals

- No catch-up, no peek reading and no adoption of a warm successor (#1355).
- No control-plane or C ABI wiring (#1360).

## Objective gates

1. **Predecessor untouched.** Two identical running plans, P and its twin. The control thread
   prepares a warm successor W of P (a muted limiter track added, lead `4 * 128` at 48 kHz) and
   publishes W `CopyAndReturn`. P's output from B on is bit-identical to the twin's for 64 blocks.
   `withdraw()` returns `Withdrawal::Returned { reason: Copied }` with W, and
   `next_absolute_sample() == B`. While a `test-support` hook holds render inside the copy, a
   `withdraw()` from the control thread returns `InFlight` and leaves the cell `Copying`. With an envelope mismatch the reason is `CopyRefused`
   and W's state equals its unpublished state.
2. **Copy equals move.** Take the returned W and adopt it in move mode at B on a fresh copy of the
   timeline, with only the source section running. Its next 8 blocks equal those of a successor
   adopted in move mode at B with no copy.
3. **Armed at B.** After the return, each carried ring's peek reports its arm frame. That frame is
   the frame P's consumer played in block B.
4. **Realtime.** The claim block makes zero allocations and frees on the render thread. Measure
   with `bench_support::alloc` current-thread counters. Its copied bytes equal
   `carry_program_copy_bytes`.
5. **Lead.** `warm_lead` for gate 1's edit returns `ceil(L / 128) * 128`, with `L` the limiter's
   latency at 48 kHz. For a limiter added inside a submix whose arrival grows while the output's
   does not, it returns the submix's growth. For an edit that grows nothing it returns `None`.
6. **Copy budget and `not_before`.** With a test ceiling one byte below gate 1's
   `carry_program_copy_bytes`, warm preparation returns `WarmUnavailable::CopyBudget`; at the exact
   value it succeeds. A `CopyAndReturn { not_before: B + 3 * 128 }` published before `B` is claimed
   exactly at `B + 3 * 128`.
7. Commands:
   - `cargo test --locked -p engine --features engine/realtime-audit`
   - `cargo test --locked -p graph --features graph/test-support`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: a claim that adopts instead of returning, or a copy that swaps the predecessor's storage
  in place of copying it, moves P's bits; a withdrawal that takes a `Copying` cell, or one reason
  for both outcomes, fails the reason checks. Red.
- Gate 2: a copy section that skips an owner the move section carries starts that owner at rest.
  Red.
- Gate 3: arming at the block after B, or arming only some rings, makes the arm frame differ. Red.
- Gate 4: a copy that clones a `Vec`, or arms through an `Arc` clone, counts an allocation. Red.
- Gate 5: a lead sized from the output's arrival alone, or not rounded up to whole quanta, gives the
  wrong `P` for the submix case. Red.
- Gate 6: a ceiling compared with `<` instead of `<=`, or a claim at the first block regardless of
  `not_before`, fails the exact cases. Red.

## Dependencies

- *Pre-roll a successor whose latency grows* (#1287), first slice: floors at predecessor + P.
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311).
- *Carry plan state by copy as well as by move* (#1322), and the copy modes of every carry slice it
  names, through *Carry strip delay lines and live send ramps across a plan swap* (#1284).
- *Carry meter and effect observation state across a plan swap* (#1327).
- *Carry spectrum capture state across a plan swap* (#1395).
- *Give the source ring a read-only peek cursor that gates release* (#1320).
- *Keep every node's latency from dropping during playback* (#1285): the floors `warm_lead`
  compiles with.
- *Record the swap block's cost on the 64-track console* (#1286), D3: `COPY_BYTES_MAX`.
