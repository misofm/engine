# Publish spectrum records by reserve and commit

## Product outcome

Each spectrum publication reconstructs a 2 048-frame window from the ring with a scalar per-frame wrap branch into a staging buffer, copies it into a by-value `SpectrumWindow` on the stack, then moves the ~16 KB record into the queue slot: about 48 KB of memmove and a 16 KB stack frame inside the audio callback per hop. Write once, directly into the queue slot. Class A.

## Root evidence

- `crates/host-core/src/spectrum.rs:2581` `continuous_publish`; `SpectrumWindow` (`:438`, two `[f32; 2048]`); the captured record type near `:465-471`; `try_push` on the bounded SPSC (`crates/engine/src/realtime/spsc.rs`).
- A reserve/commit producer pattern already exists for the retirement queue in `crates/engine/src/realtime/plan_exchange.rs`.

## Smallest closable slice

Authorized paths: `crates/engine/src/realtime/spsc.rs` (a `try_reserve`/`commit` or `try_push_with(|slot| ...)` API, documented safety unchanged), `crates/host-core/src/spectrum.rs`, their tests, and this spec.

Reconstruct the window with two `copy_from_slice` per plane straight into the reserved slot; commit; no stack temporary.

## Non-goals

No change to record contents, queue depth, drop accounting or the consumer side.

## Objective gates

1. New test: published records bit-identical to the current implementation for random ring states including every wrap offset.
2. New SPSC test: reserve/commit preserves the existing ordering guarantees and drop counting; a reserve without commit leaves the queue unchanged.
3. `scripts/check-host-core-policy.sh`, `tools/realtime-policy`, `scripts/check-realtime-audit-leak.sh`; `cargo test -p engine -p host-core` green.

## Dependencies

None; may follow "Capture continuous spectrum history with block copies".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Decision and evidence record (2026-10-24, attempt 1)

- Engine API (root R1-R3): `bounded_spsc_prefilled<T: Copy>(capacity, fill, generation)` writes `fill` into every physical slot at construction and returns an `InPlaceProducer<T>`. `try_push_in_place(|&mut T|)` reserves, gives the writer the initialized slot, then publishes with `commit`'s `Release` store and counters. A full queue runs no writer. The type is the guarantee: a `Copy` pop leaves the bytes in place, so a slot is never uninitialized. `try_reserve` and `PushPermit` stay `pub(crate)`. Root authorized the re-export in `crates/engine/src/realtime/mod.rs` (two names), the one path outside this spec's list.
- host-core: the observer's queue is prefilled with `SpectrumCapturedRecord::EMPTY` (zero planes). `continuous_publish` folds `source_underrun` over the full ring first. It then copies the window from the ring straight into the slot: two `copy_from_slice` per selected plane. The staging copy and the ~16 KB stack record are gone. An unselected plane is never written, so it reads 0. The one-shot path and drop accounting are unchanged.
- Probes now count only work that happens: on a continuous publish, `owned_record_sample_copies` and `queue_payload_transfer_samples` are 0. `chronological_reconstruction_writes` and `payload_constructions` count only pushes that found a slot. The count tests were updated (q128 total 12 544 -> 4 352).
- Gate 1 is a one-time PR comparison, not a committed test. Direct `continuous_publish` over every `history_write_index` 0..2047, all three masks, random ring contents and validity, offsets 0..63, and random skipped pops (drops). It hashes every popped record and the drop, failure and sequence counters. Base and branch: `002fba7107ef52e9` (4 649 records), identical. The #896 block digest is also unchanged: `d866542cd9608452`.
- Gate 2: `in_place_push_writes_the_initialized_slot_and_a_full_queue_runs_no_writer`.
