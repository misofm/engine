# Prove under a racing render that a live transaction lands in one block and the watermark names it

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Split out of *Hold live values in latest-target cells on both hosts* (#1312) by root on 2026-10-05
(Amendment 2 of #1312; review finding m6). Code anchors verified on `codex/d15-stream-b` at
`13f335d79`.

## Product outcome

A C ABI test proves, against a real concurrently running render thread, two things.

- A live transaction that changes two strips lands in one block: no rendered block holds one
  strip at the new value and the other at the old.
- The applied-revision watermark names exactly that block's first sample. It is never early and
  never late.

This is the converse that #1314's race test could not assert before cells, because each strip then
drained at its own node.

## Context

- `crates/capi/tests/plan_swap_race.rs:918-936`:
  `a_watermark_advance_names_a_block_that_applied_the_edit` is the one-way form.
  - A control thread under `bench_support::producer::render_while_producing`
    (`tools/bench-support/src/producer.rs:55`) commits 320 one-strip fader edits on `eq0`.
  - A `RenderGate` handshake sweeps the revision publications across render calls.
  - The test asserts only that an advance to `r` names a block that applied `r`.
  - Its doc records why the converse was not asked: a record pushed after render's revision load
    and before its strip's drain rendered one block before the watermark reported it.
- After #1312, every strip drain reads its cells under the block's snapshot `S`, so the converse
  holds.
  - #1312 gate 8 proves it deterministically, single-threaded.
  - #1432's loom models L1-L4 prove the primitive under every interleaving loom reaches.
  - This issue adds the end-to-end race under the real C ABI and render thread.
- #1314's attempt-1 author reported that the first draft of exactly this comparison went red on
  unmodified code because of harness subtleties. That is why root split it from #1312 (review
  m6).

## Decisions frozen for this slice

- **D1. The scenario.** The same harness as the one-way test, with two strips (`eq0` and a second
  strip).
  - Each commit is one transaction that sets the two strips' faders to opposite levels: `eq0` at
    -6 dB and the other at 0 dB, then the reverse, alternating.
  - The fader ramps end within one block, measured single-threaded first, as the one-way test
    measures `live_levels`.
- **D2. The assertions, per rendered block.**
  - Both strips' last samples are at their old steady levels, or both at their new ones, never a
    mix.
  - When the watermark advanced to `r` at this block, both strips' last samples are `r`'s steady
    levels.
  - When the block's last samples are a revision's steady levels that differ from the previous
    block's, the watermark advanced to that revision at this block's first sample.
- **D3. The guard.** As in the one-way test: more than a tenth of the publications land inside a
  render call, in either build profile, or the run is refused as not racing.

## Deliverables

1. The new test in `crates/capi/tests/plan_swap_race.rs`, and a two-strip document helper beside
   `live_document`.

## Authorized paths

- `crates/capi/tests/plan_swap_race.rs`.
- `tools/bench-support/src/producer.rs`, only if the harness needs a hook it lacks.

## Non-goals

- Any engine or control-plane change: this issue only proves behaviour that #1312 delivers.

## Objective gates

1. **The race (new test, debug and release).** Ten runs in each profile, all green.
   - Mutation (PR evidence): the strip drains read under `LiveSnapshot::ALL` instead of the
     block's `S`. This turns it red: a mixed block, or a watermark one block late.
   - Mutation (PR evidence): the snapshot taken after `render_inner`. This turns it red too.
2. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked --release -p capi --test
   plan_swap_race`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a transaction split across two blocks, or a watermark that names a block other than
  the first one wholly in effect, under a real race. No single-threaded test can place a commit
  between two strips' drains.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b3`, one test-only commit)

**What landed.** Only `crates/capi/tests/plan_swap_race.rs`; no engine, control-plane or harness change.

- `pair_document` (beside `live_document`): the stateless fixture cut to `eq0` and `eq1`, the two
  routes' matrices set to one diagonal entry each (`eq0` to the left plane only, `eq1` to the right
  plane only), so one block's last sample on each plane shows one strip. `eq0` starts at 0 dB and
  `eq1` at -6 dB.
- `pair_command`: one transaction with two `SetTrackFader` edits: odd edits set `eq0` -6 dB and
  `eq1` 0 dB, even edits the reverse (D1). `pair_levels` measures both steady states single-threaded
  (the ramps end within the 1024-frame block, the second block repeats the first, edit 2 restores the
  first state bit for bit).
- `a_two_strip_transaction_lands_in_one_block_and_the_watermark_names_it`, in the one-way test's
  harness (`RenderGate`, `LiveControl`, `render_while_producing`), 640 edits. Per block: both planes
  show one of the two steady states (D2 first bullet); an advance to `r` names the block's first
  sample and the block shows `r`'s state (second bullet); a block that shows a state different from
  the previous block's is the block the watermark advanced at (third bullet).
  The D3 guard is the one-way test's: more than a tenth of the revision stores inside a render call.
- `LiveControl` gained two fields: `command` (the transaction encoder) and `windows` (the sweeps of
  the revision store across the render call). The one-way test passes `fader_command` and the one
  sweep it always had (`(-16, 64)` in 64ths of a call: its formula is unchanged). Its doc no longer
  says the converse does not hold.

**Harness finding (not a product defect).** The test is green on the unmodified tree. The first form
of the sweep (the one-way test's, over the whole render call) did not catch mutation M1 in the debug
profile: in debug a render call is about 2.2 ms and a strip drains within a few microseconds of the
block's snapshot, so only a store inside that sliver of a sweep that spans 1.25 calls can hit it
(1 red in 6 runs). The new test therefore alternates two sweeps: the whole call, and
`(-2, 6)` 64ths, the first few percent of the call, where a store lands after the snapshot and
before the drains. With that, M1 is red in 8 of 8 runs in each profile.

**Gate 1, clean tree.** Ten runs of `plan_swap_race` in each profile, all 4 tests green
(debug 3.5 to 4.8 s, release 0.35 to 0.52 s). D3 measured on 10 debug and 10 release runs of the new
test: 450 to 472 of 640 revision stores inside a render call in debug (70 to 74 percent), 392 to 438
in release (61 to 68 percent). Debug: commit about 0.45 ms, render call about 2.2 ms. Release: commit
about 31 us, render call about 13 us.

**Mutations (each applied to unmodified product code, then reverted with `git checkout`).**

| Mutation | New test, debug | New test, release | One-way test (unchanged) |
| --- | --- | --- | --- |
| M1: `GraphExecutor::begin_live_block` stores `LiveSnapshot::ALL` (drains read the latest cells, the watermark still names `S`) | red 8/8 ("block N shows a new state, but the watermark did not advance at it") | red 8/8 | green, both profiles |
| M2: snapshot taken after `render_block` (drains under `ALL`) | red ("advanced to edit 1 in a block that does not show it") | red | red, both profiles |
| M3: report taken from a second `gate.snapshot()` after the block rendered | red | red | red, both profiles |
| M4: watermark advances to `S - 1` | red | red | red (the edits never complete) |
| M5: watermark advances to the previous block's `S` (one block late) | red (3/3) | red (3/3) | red in debug (3/3), green in release (3/3) |

M1 is the defect only the new test catches in both profiles, and M5 in release: a revision whose
cell stores land after a block's snapshot and before its drains is applied a block early, and a
watermark that is a block late both pass the one-way test's "an advance names a block that applied
the edit". M2 to M4 are also caught by the one-way test, which the spec's second mutation already
implies; they are recorded as the new test's own red runs, not as unique catches. The first
assertion (mixed planes) did not fire in any mutation run, because under M1 the drains read both
strips' cells in the same sliver and the "new state without an advance" assertion fires first; it
defends a transaction whose two strips straddle a block boundary, which no available mutation
produces without a hook in the product, and the spec's D2 names it.

**Gates run.** `cargo fmt --all -- --check` ok. `cargo clippy --locked -p capi --all-targets
--all-features -- -D warnings` clean. `bash scripts/check-workspace-policy.sh` ok. `cargo test
--locked -p capi`: all suites green (4 in `plan_swap_race`). `cargo test --locked --release -p capi
--test plan_swap_race` green, 10 runs. The workspace-wide clippy of gate 2 was not run (the change is
one test file in `capi`).
