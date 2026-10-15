# Carry, then retarget a strip input section across a plan swap

Successor found by the batch-10 carry-chain check, decision 17
(`docs/rulings/one-control-abstraction-2026-10-13.md`). Code anchors verified on `12d4a4f9d`.
Issue number: assigned at filing (written here as #1630). Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review.

## Product outcome

A structural edit that also changes a strip's live input value (trim, polarity, an input filter)
keeps the strip's input section running: the successor carries the predecessor's input state and
then ramps to the new value, on both hosts, instead of restarting the section.

## Context

- The input join requires every input value to be equal: `same_input_section`
  (`crates/host-core/src/prepare.rs:2726`, used at `:1803`). Any live input difference in the same
  transaction as a structural edit restarts the input section (its filter state and ramps).
- The effect carry follows the same pattern: #1280a carries, #1280c retargets a live difference
  after the carry (D4).
- The C ABI behaves the same today, so this does not block #1562b.

## Decisions frozen for this slice

- **D1.** The input join keys on the section's structure (which builtins exist, their designs'
  shape), not on live values; a live difference carries the state and retargets.
- **D2.** The retarget writes through the successor's input producer, stamped on its gate, in the
  block that writes the strip retargets, with the session's ramps; filter retargets use the
  existing live input-filter path (#1556).
- **D3.** The carry is allocation-, lock- and syscall-free on the swap block.

## Objective gates

1. A host-core successor-swap test: a structural edit plus a trim change on a surviving strip; the
   input section's state carries (no restart transient) and the trim ramps to the new value,
   bit-equal to a plan that made the same edit live.
2. The same for an input filter cutoff change and a polarity change.
3. Zero allocations and frees on the swap block; `cargo test --locked -p host-core --all-features
   -p capi -p host-web --features host-web/test-support`; clippy.

## Authorized paths

- `crates/host-core/src/prepare.rs` (the input join and retarget), `crates/host-core/tests/successor_swap.rs`,
  the builtins input stage's carry entry if it lacks one, this spec's decision record.

## Dependencies

- #1280c (the retarget pattern). Lane K.

## Size

Half a day to a day.

## Decision record: #1630 (batch 14b)

**Shipped.** The input join in `crates/host-core/src/prepare.rs` (the `carried_inputs` block of
the successor's preparation) now keys on the section's structure. Every strip's input section has
the same stages (trim, polarity, two filter sections per channel), so the structure is the control
kind (`row.live == live_input`, as before) plus the rate the designs are for. On a plan with input
cells, a surviving strip of the same kind always carries, and the records the live path gives the
same edit (`input_records`, #1556) become its retarget (`InputJoin::retarget`). They are written
through the successor's own `InputCells`, stamped on its held gate in the same block as the strip,
effect and route retargets, with the session's ramps or the transaction's edit ramp for
`(Strip, Input)` (#1394 D6). A plan without input cells cannot retarget, so it still carries only
bit-equal sections (`same_input_section`, unchanged #1276 behaviour).

**Departures and code truth.**

- `input_records` and `filter_edits` moved from `live_delta.rs` (built only with
  `control-provider`) to `prepare.rs` (every build), as the strip-retarget helpers did for #1277
  D5; the classifier imports them and maps `OutOfDomain` to `LiveRebuild::Domain`. One function,
  so a retarget and a live edit cannot disagree. This edits `crates/host-core/src/live_delta.rs`,
  outside the authorized paths, mechanically (move, imports, one doc link).
- The join adds a sample-rate check (`base.inventory.sample_rate_hz == compiled rate`): a carried
  lane's coefficients and filter state are designed for the predecessor's rate, so a rate change
  is a structural difference (D1, "their designs' shape"). The check is defensive: the control
  plane and `adopt_predecessor_plan` already refuse a successor at another rate, so it cannot
  change any output today.
- A value the live path refuses (out of domain) restarts the section instead of carrying; a
  compiled model does not reach it, and a debug build asserts so (review fold).
- No builtins or `crates/engine/src/realtime/` edit was needed: the input stage's carry entry
  (#1276 `export_lane`/`import_lane`) and the successor's cell drain already order carry before
  retarget on the swap block. No new realtime region.

**Superseded tests.** `a_restarting_input_lane_does_not_count_its_replaced_writes` and
`a_pending_trim_record_in_a_later_bank_survives_the_swap` restarted one strip by a trim difference
between committed and B, which now carries. They now restart it with a committed model that lacks
the strip (`without_track`), so they keep their discrimination. Gate 4 of #1276
(`a_strip_whose_input_section_changed_starts_at_rest`) runs on plans without input cells and stays.

**Gates** (`crates/host-core/tests/successor_swap.rs`, `control-provider`, both widths each):

1. `a_trim_change_carries_and_retargets_the_input_section`
2. `a_filter_cutoff_change_carries_and_retargets_the_input_section` (HPF on both lanes, LPF one
   lane), `a_polarity_change_carries_and_retargets_the_input_section`.

Each: the carry-then-retarget run equals, in every block, a run that wrote the classifier's records
for the same edit to the predecessor before the swap; it differs at the swap block from a run whose
section restarts and from a run without the edit. Red on the old join (first difference at block 6)
and red when the retarget write is dropped.
3. The same runs: swap block 0 allocations, reallocations and frees (`bench_support::alloc` thread
   counters after `audit::warm_up`) and 0 audited allocations. `cargo test --locked -p host-core
   --all-features`, `-p capi`, `-p host-web --features host-web/test-support`: pass. Clippy
   `-p host-core --all-targets` with and without `--all-features`: clean. `audit capi`:
   `pcm_digest` `bf7dcedd1d92621f`, 0 violations. `realtime_policy`: ok, 149 regions (unchanged).

**Review fold (batch 14b).** (f) Root ruling: D15-2's `live_values_superseded` is exact, so a
carried value the successor's retarget replaces in the adopting block is counted, as an effect
lane's carried write is. The input and strip carry drains apply a predecessor's unread value to lane
state at the swap; until now, when the successor's own cell (the retarget) applied in the adopting
block, before any sample, that carried value was replaced unheard and not counted. Each render-side
cell reader (`InputReader`, `FaderReader`, `MatrixReader`, `crates/builtins-compiler/src/lib.rs`)
now keeps `CarryMarks`: the cells its last drain applied, and the cells a swap carried in. The
adopting lane takes the predecessor reader's applied cells (`adopt_input_lane`, the fader and matrix
banks' `adopt_lane`, which the fused bank shares); the successor's first drain counts one
superseded value per carried cell it applies (`LiveValuesSuperseded::add_carried`) and clears the
marks, so from its second block a carried value counts as heard. No `crates/engine/src/realtime`
or `graph` edit; the new code sits in the existing realtime-policy regions plus one new region (the
`CarryMarks` methods and `adopt_carry_marks`): `realtime_policy` ok at 150 regions, and its
floor (`tools/realtime-policy/src/policy.rs`) rises 149 -> 150 in this change. The marks add four bytes per
reader: `crates/capi/tests/resource_lifecycle.rs`'s eight-lane `builtin_bank_bytes` measured 15,321
-> 15,705 (+384), over its 15,680 ceiling, re-pinned by that file's rule (plus 10 %, up to 64) to
17,280; the four-lane ceiling was not re-measured here. Tests (`successor_swap.rs`, both widths, both strip
forms): `a_carried_input_value_the_retarget_replaces_is_counted_on_every_width` and
`a_carried_strip_value_the_retarget_replaces_is_counted_on_every_width_and_form`: a one-lane write
pending at the swap and committed in the base, then moved again by B, counts exactly 1 on the
successor (0 on the predecessor), is bit-identical in every block to a run whose base never held
the write, and counts 0 when B leaves the value (no retarget). Red with the count removed:
`(0, 0)` against `(0, 1)` in both. The harness reads the successor's own count
(`successor_superseded`), since the test seam gives each plan its own counter. (g)
`an_input_difference_carries_and_retargets` also runs every case with a transaction that gave the
strip's `(Strip, Input)` row a 37-sample ramp, through the successor path (`direct_run_ramped`,
`SuccessorBase::edit_ramps`): it equals the run that writes the classifier's records for the
ramped edit, and for trim and polarity (whose records the row changes) differs at the swap block
from the session-ramp retarget. Red when the join ignores the edit ramps (`StripEditRamps::default()`
at the `input_records` call): trim and polarity fail, first difference at block 6. (h) The
`input_records(..)` call in the join is now `debug_assert!`ed `Ok` before `.ok()?`: an internal
refusal would silently restart the section.
