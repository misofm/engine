# Classify and carry effect automation edits

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.8), A2 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R3.

## Product outcome

A host edits a stored effect automation curve while the session plays, on the C ABI or through
the shared commit, and hears the new curve from the swap block on, with no gap and no restart:
every effect keeps its state, every unchanged curve keeps its place, a changed curve takes one
jump at the swap block, and a removed curve ramps to the static value. The revision completes
`exact` at adoption. A static value edit on an automated effect parameter is stored and not heard,
and commits as `model_only`, on both hosts: a browser live effect parameter edit on an
automated cell reaches that rule through the Worker's apply (#1382) and replies `model_only`. The
EQ's rows stay masked until draft 20.

## Context

**The classifier** (`crates/host-core/src/live_delta.rs`).
- `classify_live_delta` (`:211`) masks the whole automation table (`:236`); its rustdoc says the
  first slice that renders automation must remove that line (`:181-185`). Draft 10 makes the mask
  per target row, adds `LiveRebuild::Automation` and one predicate `automated_cell(model,
  address)`, and removes the fader row.
- The numbered rules are `:149-179`; the rebuild reasons are `LiveRebuild` (`:117-141`).
- An effect `params` change becomes one `EffectControlRecord::Parameter` per changed
  `(parameter_index, channel)` (`parameter_records`, `:411-449`). The EQ's records become
  prepared targets (`:450-458`).

**The swap carry.**
- A successor runs its hand-over in `adopt_predecessor` (`crates/graph/src/lib.rs:3139-3177`):
  every move must resolve before anything moves (`:3153-3160`), then sources and builtin input
  lanes move.
- Host-core builds the program from the predecessor's inventory and joins by strip ID
  (`GraphCarryProgram`, `crates/host-core/src/prepare.rs:1292-1302`; installed at `:1790-1794`;
  `GraphLaneMove`, `crates/graph/src/lib.rs:2810-2815`).
- *Carry console effect lanes across a plan swap* (#1279) and *Carry live-controlled effect lanes
  across a plan swap* (#1280) add the effect lanes. #1279 D1 counts "for an owner with no live
  control lane, every parameter value and its bypass" as prepared values. #1280 D1 adds "live
  controls attached in both plans or in neither".

**The browser.** After *Admit browser live edits in the Worker through the committed model*
(#1382), a browser live effect parameter edit (today `COMMAND_EFFECT_PARAM`,
`hosts/host-web/src/lib.rs:835`) reaches the shared commit through the Worker's apply and meets the
shared classifier, as a C ABI transaction does.

**What drafts 18a and 18b give.** Each automated cell has a stable address `(strip ID, rack, effect ID,
parameter_index, channel)`, a compiled program and an event state (cursor, current target, ramp
end, held grid, last seek boundary). An automated instance with no live lane has a channel-less
lane and a window of `stored(i)` spans.

## Decisions frozen for this slice

- **D1. The mask.** Effect automation rows leave the mask, except rows whose target is an
  instance with target preparation (the EQ), which draft 20 removes. A delta whose effect
  automation entries differ returns `LiveRebuild::Automation`. The numbered rules gain the step.
  An automation-only edit is then a rebuild on both hosts' shared commit (A10).
- **D2. Static values on automated cells.** `parameter_records` emits no record for a
  `(parameter_index, channel)` of a span-driven effect that `automated_cell(next, address)`
  reports. The EQ's static values still render until draft 20, so they keep today's
  records. A `both` entry covers
  both channels; a `left` entry covers only the left. A delta whose only change is such a value
  has no record and commits as `model_only` (#1313). A transaction that also changes automation is
  a rebuild (D1).
- **D3. Carried cells.** At the swap block, each cell present in both programs carries its whole
  event state by stable address. Host-core joins the successor's cells with the predecessor's
  inventory by address, off render, and installs one move table in the carry program. In
  `adopt_predecessor`, the moves run with the other moves, all or nothing. Each carried cell then
  repositions its cursor in the new table (draft 07). If the new curve's value at the adoption
  block differs from the carried target, the cell's first event is a jump at that block's first
  sample (A1.8).
- **D4. Released cells.** A cell present in the predecessor and absent from the successor is
  released: the successor stages one `Point` with the instance's prepared value (the static value)
  at the adoption block's first sample, over the parameter's smoothing (D15-7: carry, then
  retarget). Host-core lists released cells in the same move table; nothing in the successor's
  program outlives that block.
- **D5. New cells.** A cell absent from the predecessor, on an instance whose lane carries,
  starts with the predecessor model's resolved value for that `(parameter_index, channel)` as its
  current target: that is the value the carried effect state holds, because every live edit is in
  the committed model. Its first event is a jump at the adoption block if the curve's value
  differs from it, to the curve's value at `+ L - 1`. Draft 18a D6's rule (current target = the
  prepared value) applies only to an instance that does not carry. Host-core writes the seed into
  the move table.
- **D6. The stage carries.** This slice extends the carry predicates that #1279 and #1280 build
  (both land first); neither spec is amended.
  - #1279 D1's prepared-value test: the automation program, the prepared values it writes (draft 18a D3) and the window
    capacity it sets are not prepared values; nor is the static value of a cell the successor
    automates. An owner that gains, loses or changes stored automation carries.
  - #1279 D2's rule: an owner whose stored automation gives it a channel-less lane carries like one
    without.
  - #1280 D1's rule "live controls attached in both plans or in neither" ignores a channel-less lane
    that exists only for stored automation.
  - The lane state itself moves by #1279's snapshot and restore, whatever stage type holds it.
- **D7. One rule, both hosts.** D2's static-edit rule lives in the shared classifier, so the
  browser meets it through #1382's Worker commit; no host has its own admission for automated
  cells.
- **D8. The acked-batch question: can an ack ever precede a drop? No.** A static edit on an
  automated cell is committed to the model and stored, which is its whole meaning (A2), and the
  response says `model_only`. An automation edit is acked at adoption, after its moves ran.

## Deliverables

1. D1-D2 in `crates/host-core/src/live_delta.rs` and its rustdoc rules.
2. D3-D5: the inventory rows and the join in `crates/host-core/src/prepare.rs`, the move table in
   `crates/graph/src/lib.rs`, the cell moves in the stages (`crates/rack/src/lib.rs`,
   `crates/graph/src/runtime.rs`).
3. D6's predicate changes in host-core's join.
4. The tests below.

The slice has two parts: classification (D1, D2, D7) and carry (D3-D6). They may be two commits,
but they merge together, in one push: classification without carry makes an automation edit a
rebuild whose cells restart, and carry without classification is never reached.

## Authorized paths

- `crates/host-core/src/{live_delta.rs,prepare.rs}`, `crates/host-core/tests/live_delta.rs`,
  `crates/host-core/tests/successor_swap.rs`.
- `crates/graph/src/lib.rs` (the carry program and `adopt_predecessor`),
  `crates/graph/src/runtime.rs` (the carry path), `crates/rack/src/lib.rs` (the carry path),
  `crates/graph/tests/rt11_swap_carry_alloc.rs`.
- `crates/capi/src/runtime/live_tests.rs` (tests).
- `hosts/host-web/src/tests.rs` (gate 6).

## Non-goals

- The EQ's rows and its group cell (draft 20).
- OQ1: what a live move does to an automated control. Option A ships (A2).
- The browser's commit in the Worker (#1382, which lands before draft 09a).
- Carrying the effect lane state itself (#1279, #1280).

## Hazards

- **D6 is required.** Without it, an automation edit puts the strip in the restart set and the
  edit is heard as a duck-swap, not a jump. Gate 4 holds it.
- **Restart set.** A strip in the restart set for another reason (D15-9) restarts its effects; its
  cells then start fresh (D5), not carried.
- **Cohort capacity.** Adding automation to one instance of a cohort can raise the cohort's
  capacity (#1306 D2), which changes `EffectProgramKey` for its bank-mates. D6 makes that a carry,
  not a restart; gate 4 holds it.
- **Hot files.** `crates/host-core/src/live_delta.rs` (stream B), `crates/graph/src/lib.rs` and
  the stages (stream A), `hosts/host-web/src/tests.rs` (stream H). Root sequences the merge.

## Objective gates

1. **Classifier** (`crates/host-core/tests/live_delta.rs`, new cases): a delta that adds, changes
   or removes a compressor automation entry gives `Err(LiveRebuild::Automation)`; an EQ automation
   change stays live with no records; a static `params` change on an automated cell gives no
   record and, alone, a live delta with no records; on a `left`-only entry, a `both`-equal static
   change gives one `Right` record.
2. **Future edit, exact** (`crates/capi/src/runtime/live_tests.rs`, new). On a playing engine,
   add a segment that starts after the adoption block to a compressor threshold curve. The
   transaction takes the rebuild path and completes `exact`; from the adoption block on, the
   output equals, bit for bit, a twin engine that played the new session from the start.
3. **Changed value, one jump.** An edit that changes the curve's value at the adoption block
   stages one `Point` at that block's first sample with the new curve's value at `+ L - 1`; from
   the next grid sample on, the stored `Point`s equal the twin's (test-support trace of staged
   stored `Point`s). An edit that keeps the curve's value at adoption stages no jump.
4. **Gain, lose, capacity.** An instance with no live lane gains automation (its stage type and its
   cohort's capacity change): its lane and its bank-mates carry (output equals the twin from the
   adoption block when the new curve's first value equals the static value and starts after
   adoption). Removing the entry stages one release `Point` with the static value.
5. **Static edit.** A static `params` edit on an automated cell commits `model_only`, the plan is
   not replaced, and the next blocks equal an engine that never received it.
6. **Browser** (`hosts/host-web/src/tests.rs`, new): a live effect parameter edit through the Worker's
   apply on an automated cell replies `model_only` and moves no bit; the same edit on a
   non-automated cell of the same instance
   replies `live`.
7. **Allocation.** `crates/graph/tests/rt11_swap_carry_alloc.rs` gains a swap with carried and
   released cells: 0 allocations and 0 frees in the swap block and the 1,000 blocks after it
   (`bench_support::alloc::current_thread_delta_since`).
8. **No rendered bit moves** for a session with no stored automation (PR evidence):
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
   `./target/release/audit capi` gives the same `pcm_digest` at base and head; the browser legs of
   `qualification.yml`'s `browser` job pass with unchanged digests.
9. **Workspace.**
   - `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p graph --features test-support`,
     `cargo test --locked -p host-web --features test-support`
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi` (every
     violation count 0)
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red if effect automation stays masked (an edit commits and is never heard), if the
  EQ leaves the mask early, or if a static edit on an automated cell still pushes a record.
- Gate 2 turns red if any carried owner restarts, or if a cell's cursor is not repositioned.
- Gate 3 turns red if a carried cell keeps its old target, or jumps when the value did not change.
- Gate 4 turns red if a stage-type or capacity change restarts a lane, or if a removed curve
  leaves the parameter at the curve's last value.
- Gate 5 turns red if a static edit on an automated cell rebuilds or moves a bit.
- Gate 6 turns red if the browser has a path that applies a live value over automation.
- Gate 7 turns red if the moves allocate on the render thread.

## Dependencies

Batch R3, in one push with drafts 18a and 18b and #1306 (README "Must-land-together groups"):
amended #1306 sizes each window without an automated cell's live term, and this slice's D2 stops
the live record that would otherwise overflow it. Direct dependencies:

- Draft 18b *Render stored effect parameter automation across seeks and on both hosts*.
- *Carry console effect lanes across a plan swap* (#1279): the carry D6 extends.
- *Carry live-controlled effect lanes across a plan swap* (#1280): the carry D6 extends.

Draft 18a, draft 10 (the mask per row, `LiveRebuild::Automation`, `automated_cell`), draft 07 (the cursor reposition) and *Report each transaction's edit path in
its response* (#1313) arrive through draft 18b.
