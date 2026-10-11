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
