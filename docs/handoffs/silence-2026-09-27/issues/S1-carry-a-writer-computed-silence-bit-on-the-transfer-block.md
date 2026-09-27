# Carry a writer-computed silence bit on the transfer block

Draft, slice S1 of the silence architecture issue (A0). Class A: adds a fact, moves no rendered bit.
Evidence: `docs/handoffs/silence-2026-09-27/DESIGN.md` sections 3 and 4.2. Base: the batch branch at
`85019581` or later; re-read every cited function first.

## Product outcome

A played block that is digital silence (62.4 % of the dogfood session's track-blocks) is known to be
silent without a render-thread scan. The two writers already touch every word off the render thread:
the host submit path `submit_planes` (`crates/source/src/lib.rs:867-903`, a `copy_from_slice` per
channel) and the native worker's decode into the reserved block, committed by `commit_block`
(`crates/source/src/lib.rs:961-990`). Each computes, per channel, whether every word it wrote is
`+0.0` by bits, and the graph can ask for a claim's answer. Unplayed claims (`None`) stay the free
fact they are. The consumer is S4; this slice only produces and exposes the fact.

## Interface contract

* `TransferBlock` (`crates/source/src/lib.rs:489-496`) gains `silent_channels: u64`: bit `c` is set
  iff channel `c`'s `frames` valid words are all `+0.0` (bit pattern 0). `reset_metadata` and
  `try_new` set it to 0 ("not known silent"). A channel index of 64 or more is never marked (its bit
  stays 0). `play()`'s zero-fill of a short block's tail (`:1557-1575`) keeps a set bit true for the
  whole quantum.
* `submit_planes`: OR the bits of each channel's copied words (after the `copy_from_slice`, same
  loop) and set the bit when the OR is 0. Native path: the contract point is `commit_block`
  (`crates/source/src/lib.rs:961`), which ORs each channel's `frames` words of the decoded block
  before publishing; fusing the OR into the decoder's `convert_frames`
  (`crates/source/src/native_wave.rs:771`) is an allowed optimisation if it yields the same bits.
  Both run on the worker, off the render thread; no allocation.
* `graph::GraphPreparedSourceSetDriver` (`crates/graph/src/lib.rs:1851`) gains a provided method
  `fn played_silent(&self, claim_index: usize) -> bool { false }`: true iff the claim played no block
  this quantum, or both of its mapped channels' bits are set. `graph::GraphSourcePlanes` gets the same
  method after the set's claim checks. Realtime: no allocation, lock or syscall.
* `source::SourceGraphSourceSetDriver` (`crates/source/src/lib.rs:1786`) implements it from the
  played block of the claim's source and its `(left_channel, right_channel)` mapping.
* `console_workload::FrozenSourceDriver` (`tools/console-workload/src/lib.rs:1916-1964`) implements it
  from its frozen blocks, computed once at construction.

## Authorized paths

`crates/source/src/lib.rs`, `crates/source/src/native_wave.rs` (only if the OR is fused), `crates/graph/src/lib.rs` (the two provided methods only),
`tools/console-workload/src/lib.rs` (`FrozenSourceDriver` only), new tests under
`crates/source/tests/` and `crates/graph/tests/`, `crates/source/tests/MUTATIONS.md`, this spec.

## Non-goals

No consumer (S4 reads it). No render-thread scan. No change to what `played_planes` or
`copy_track_input` return. No change to any digest.

## Objective gates

1. **Bit correctness.** A property test over at least 10,000 seeded submitted blocks: channel counts
   1, 2 and 6; frame counts 1, 17, 127 and 128 (short blocks included); each channel either all
   `+0.0` or `+0.0` except one word at a random position holding `-0.0`, the smallest positive
   subnormal, the smallest negative subnormal, a quiet NaN or 1.0. `silent_channels` must equal a
   naive per-channel reference (`iter().all(|v| v.to_bits() == 0)` over the valid frames of the words
   the writer stored, i.e. after the decoder's sanitisation: `sanitize_f32`,
   `crates/source/src/native_wave.rs:883-892`, maps a non-finite or subnormal float to `+0.0` and keeps
   `-0.0`) on every
   block, through both writers (`submit_planes` and the native decode/commit path with 16- and 24-bit
   integer PCM and 32-bit float WAV, including a float file carrying `-0.0` words).
2. **Driver answers.** `played_silent` is true on an unplayed claim, true for a claim mapped to two
   silent channels, false if either mapped channel is not silent, for the mappings `(0,1)`, `(0,0)`
   and `(1,0)`, and after a seek and a stale-generation discard.
3. **Red mutations** (record in `crates/source/tests/MUTATIONS.md`): mark by `== 0.0` instead of bits
   (the `-0.0` case turns gate 1 red); OR over the whole quantum instead of the valid frames with a
   stale tail (a short block after a loud one turns it red); forget to clear the bit in
   `reset_metadata`.
4. **No render change.** Every console row's digest and `test_only_source_plane_counts` pins are
   unchanged; the allocation-free render tests of `source` and `graph` pass;
   `scripts/check-realtime-policy.sh` passes.
5. **Wasm.** `host-web` builds for `wasm32-unknown-unknown`; the AudioWorklet callgraph gate stays
   green (the submit path is not on the render path, but it ships in the same artifact).
6. fmt, workspace clippy with `-D warnings`, `cargo test -p source -p graph -p console-workload`.

## Console benchmark rows

None move. Every digest is unchanged.

## Dependencies

A0 ruled. Independent of S2 and S3.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every gate that says "unchanged" or "equal" is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving.
