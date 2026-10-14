# Bound the first live commit of a large session

Successor of #1615, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 9 branch `codex/controls-batch-9k2`. Issue number: assigned at filing
(written here as TBD15). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a
batch review.

## Product outcome

The first live-only transaction after boot on a large session needs about the same peak of heap as
a later one, so a browser page's first fader move does not grow the wasm memory in the
AudioWorklet's message handler on a 64- or 256-track session.

## Context

- #1615 cut the steady live commit of the nine-edit transaction to a 71,252-byte peak, the same at
  1, 64 and 256 tracks (`hosts/host-web/tests/control_command_heap.rs`).
- The first call is not: its peak is 76,011 bytes at 1 track, 196,593 at 64 and 663,179 at 256
  (#1615's decision record). It builds state that later calls reuse.
- On a non-isolated page this runs in the worklet's message handler, where a growth is a
  `memory.grow` on the audio thread (counted and reported; #1562, root ruling A).

## Decisions to take (the implementer proposes, the coordinator rules)

- **Option A (boot).** Build the reused state during boot or preparation, off the audio-thread
  handler's first edit, and charge it in the boot resource report.
- **Option B (lazy, bounded).** Keep it lazy but size it from the session at preparation so the
  first call makes one exact allocation.
- First measure which structures make up the first-call excess at 64 and 256 tracks.

## Objective gates

1. The heap test gains a first-call row at 64 and 256 tracks with a ceiling near the steady peak
   (measured numbers and the reason in the decision record).
2. The boot budget gate (`scripts/check-web-boot-budget.mjs`) still passes at its current pin.
3. `cargo test --locked -p host-web --features host-web/test-support`, `-p control-plane
   --features test-support`, `-p host-core --all-features`.

## Test value

- Gate 1 turns red if the first live commit grows with the session again.

## Authorized paths

- `crates/host-core/src/control_provider.rs`, `crates/control-plane/src/*` (the first-commit state
  only), `hosts/host-web/tests/control_command_heap.rs`, this spec's decision record.

## Dependencies

- #1615.

## Size

Half a day, one sonnet-high implementer.
