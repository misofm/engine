# Carry plan state by copy as well as by move

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Code anchors verified on `main` at `6fb211594`.

**Order in stream A.** This issue, then *Let soft-clip restore its own non-finite history* (#1300),
then #1277 and the other carry slices. The edge with #1300 is order only: neither touches the
other's code. Every later DSP carry slice (#1277, #1279-#1284) implements both modes for its own
state family against the rules frozen here. Landing copy mode after them would reopen six merged slices. Observers (#1327)
are the exception: their copy-mode rule depends on the warm successor's clock, so #1287 owns it.

## Product outcome

A successor plan can take a copy of the running plan's state at a block boundary while the running
plan keeps rendering, and its output does not change by one bit. This is step 2 of D15-8's warm
successor: render pays one bounded copy, and the catch-up runs off the render thread (*Pre-roll a
successor whose latency grows*, #1287). After this slice, the two families that carry today follow
the copy-mode rules: source consumers and strip input sections.

## Context

- The hand-over is one executor hook. `PreparedPlanExecutor::adopt_predecessor`
  (`crates/engine/src/realtime/plan.rs:296-313`) runs in the swap block, and its doc says the
  predecessor "never renders again, so state may be moved out of it" (`:307-308`).
  `CarryOutcome` has three values (`:486-494`). `PreparedRenderPlan::adopt_predecessor_plan`
  (`:899-905`) is the synchronous wrapper: it checks the envelope, adopts the clock, and runs
  `carry_from` (`:916-925`) inside a render scope. The exchange calls `carry_from` from
  `RealtimePlanOwner::enter_block` (`crates/engine/src/realtime/plan_exchange.rs:419-429`) and then
  retires the predecessor.
- The graph executor holds the installed program: `carry` and `carry_inputs`
  (`crates/graph/src/lib.rs:2735-2739`), `GraphCarryProgram` (`:2757`), and
  `install_builtin_input_carry` (`:2837`). Its `adopt_predecessor` (`:3139-3177`) checks identity,
  checks that every input lane resolves (all or nothing), moves the sources
  (`driver.adopt_sources`, `:2225`), and copies the input lanes.
- The input-lane section (`Runtime::carry_input_lanes`, `crates/graph/src/runtime.rs:4174`) touches
  the predecessor in two ways. It calls `disengage_for_carry` on a collapsed chain
  (`crates/rack/src/lib.rs:2833`, whose doc says "The chain renders no more"). It also calls the
  bounded `drain_for_carry` (`crates/graph/src/lib.rs:1336`), which applies records to the lane
  state.
- The bank payload calls are render-safe and exact mid-ramp (#1278).
  `snapshot_track_state_payload` takes `&self` (`crates/effect-contract/src/lib.rs:2070`).

## Decisions frozen for this slice

- **D1. Two modes, one program.** *Move* is today's hook. It runs at the swap block, and the
  predecessor then retires. *Copy* is a new hook, `PreparedPlanExecutor::copy_from_predecessor(&mut
  self, predecessor: &mut dyn PreparedPlanExecutor) -> CarryOutcome`. Its default returns
  `NotRequested`. Its synchronous wrapper is `PreparedRenderPlan::copy_predecessor_state(&mut self,
  predecessor: &mut Self) -> CarryOutcome`. The wrapper uses the same envelope check as
  `adopt_predecessor_plan`. It sets the successor's clock to the predecessor's
  `next_absolute_sample`, and it runs inside a render scope. Both modes use the same installed
  program and the same identity check.
- **D2. Copy-mode guarantees.**
  - (a) From the copy on, the predecessor's output is bit-identical to a twin that was never copied
    from. Copy mode may touch the predecessor only in ways that move none of its bits:
    - disengage a collapsed chain (collapse saves cost and never changes bits);
    - desymmetrize a bank before a payload snapshot;
    - apply builtin records at the block boundary where the predecessor's next block would apply
      them anyway.
  - (b) For every section it copies, the successor's state equals what move mode would give at the
    same boundary.
  - (c) Copy mode moves no source consumer. Each carried source stays vacant in the successor until
    a move-mode adoption (D3).
- **D3. Copy, then adopt.** When the copy hook returns `Carried`, the successor records it. Its
  later move-mode `adopt_predecessor`, against the same predecessor identity, runs only the source
  section and copies no state again. In #1287 that adoption happens at sample `S`, and #1287 and
  *Give the source ring a read-only peek cursor that gates release* (#1320) place the read index.
  A successor that was never copied runs every section, exactly as today.
- **D4. The input-lane section in copy mode.** It runs the same drain-then-copy as move mode.
  `rack::BankChain::disengage_for_carry`'s doc changes from "renders no more" to "renders on dual
  in copy mode, and its bits are unchanged".
- **D5. All or nothing.** Copy mode checks that every section resolves before it writes anything,
  as move mode does. A refused copy writes nothing to either plan and reports
  `PredecessorMismatch`.
- **D6. Bounded and known.** A new function `graph::carry_program_copy_bytes(plan) -> u64` returns
  the bytes a copy writes. It is computed when the program is installed: the input-lane state sizes
  in this slice, and each later slice adds its own family. #1286 reports it, and #1287 budgets with
  it. No section loops on anything but the program length.
- **D7. The rule for every later family.** Each carry slice states its copy-mode mechanism and adds
  a copy-mode gate. The mechanism is a copy into the successor's preallocated storage. It never
  swaps owned storage, and it never moves a queue consumer, because the predecessor keeps both. A
  family whose pending live values cannot be applied to lane state at the boundary (an effect
  lane, which only stages them) copies them unread instead: *Carry live-controlled effect lanes
  across a plan swap* (#1280) lands after the effect lanes become latest-target cells (#1345) and
  copies each unread cell value into the successor without consuming it. No family refuses copy
  mode for a pending live value.
- **D8. Not here.** The exchange's copy request, the return queue and exact-sample adoption are
  *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311) and
  #1287. This slice adds only the hook, the wrapper and the graph sections. Tests call the
  wrapper directly.

## Deliverables

1. D1 in `crates/engine/src/realtime/plan.rs`: the trait method with its render-thread doc, and
   the wrapper. Engine unit tests for the envelope check, the clock, and an executor-less plan.
2. D2-D6 in `crates/graph` (copy hook, copied flag, D3 skip, `carry_program_copy_bytes`) and the
   D4 doc in `crates/rack`.
3. Copy-mode tests in `crates/host-core/tests/successor_swap.rs`, with a copy entry beside
   `direct_run` in `crates/host-core/tests/support/successor.rs` or in the test file.

## Authorized paths

- `crates/engine/src/realtime/plan.rs` and `crates/engine/src/realtime/mod.rs` (tests module only).
  These are stream B's files. The change is one trait method and one wrapper, and root sequences
  it with B(2).
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/rack/src/lib.rs` (doc only)
- `crates/host-core/tests/successor_swap.rs`, `crates/host-core/tests/support/successor.rs`

## Non-goals

- No catch-up, no return queue, no scheduled adoption (#1311, #1321, #1287).
- No new state family. #1277 and the later slices add theirs in both modes.
- No change to move mode's behaviour.

## Objective gates

1. **The predecessor is untouched, both widths.** At `Backend::Simd8` and `Backend::Simd4`, use
   the `filtered_session` shape in two variants: dual, with an HPF ramp in flight and a pending trim
   record at block 6; and mono-collapsed (the `collapsed_strips_keep_collapsing` shape). Copy A
   into its successor B (A plus a muted track whose ID sorts first) after block 6, then keep
   rendering A to `BLOCKS`. Every block equals a twin A that was never copied.
2. **Copy, then adopt, equals move.** Copy after block 6, then immediately run
   `adopt_predecessor_plan` (sources only, D3), then render B on. Every block equals the
   move-mode `direct_run` of the same edit and the fresh B reference. The swap block reports
   `Carried` twice: once for the copy, once for the adoption.
3. **No second copy.** Copy after block 6, render A for three more blocks, then adopt. A
   test-support count of copied input lanes is the same after the adoption as after the copy.
4. **Refusal writes nothing.** An unresolvable input lane, or another predecessor identity, reports
   `PredecessorMismatch`. The successor's lanes stay at rest, and gate 1's twin comparison still
   holds.
5. **Realtime.** The copy call makes zero allocations and frees, measured with the
   `bench_support::alloc` thread counters and the engine render audit.
   `carry_program_copy_bytes` is 0 for a program with no input moves, and nonzero with them.
6. Commands:
   - `cargo test --locked -p engine -p graph -p rack -p host-core --features graph/test-support,host-core/test-support`
   - `cargo test --locked -p engine --features realtime-audit`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh` (the new
     render code sits in `REALTIME_POLICY` regions)
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`

## Test value

- Gate 1: a copy that drains effect-style records, or disengages without restoring both channels,
  moves the predecessor's bits. It turns red, and no move-mode test can see it, because there the
  predecessor never renders again.
- Gate 2: a copy that misses a derived word (the input chain's agreement flag, a ramp's
  `remaining`), or that moves a source consumer, diverges from the move run. It turns red.
- Gate 3: an adoption that copies state again overwrites the successor's own catch-up state. It
  turns red.
- Gate 4: a copy that writes part of the program before it fails a later check turns it red.

## Dependencies

- None open. It builds on #1270-#1276 and #1278, all on `main`.
- Stream order only: before *Let soft-clip restore its own non-finite history* (#1300), which
  lands before the effect-lane carries (#1279-#1282).
