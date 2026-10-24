# Tiled gather and scatter for partial banks

## Product outcome

Whenever a bank's active lane count is not the full width (a single-track session, or any cohort whose size is not a multiple of W), `BankChain` gathers and scatters with a scalar strided loop, one 4-byte move per lane-sample per plane, while the kernel still processes all W lanes. Run the tiled register transpose for partial banks too. Class A: transposes are exact permutations.

## Root evidence

- `crates/rack/src/lib.rs`: `full_bank` computed at `:1749`; `gather` branches on it at `:2535` and `scatter` at `:2600`; the scalar fallbacks are `gather_lane` (`:180`) and `scatter_lane` (`:222`); the tiled path is `gather_tiled` (`:2553`) and `scatter_tiled` (near `:2666`), built on `tile_gather`/`tile_scatter` (`:252-337`) and `transpose_tile_4/8` in `crates/effect-contract/src/lib.rs` near line 289.
- Inactive lanes can be fed from `ARENA_SILENCE_BUFFER` (`crates/engine/src/realtime/disjoint.rs:33`); `write_stereo_many` (`disjoint.rs:262`) requires distinct writable buffers, so an inactive lane needs a chain-owned dump buffer reserved at bind, never a render-time allocation.
- Existing gate: `full_bank_gather_scatter_round_trip_is_bit_exact` (`crates/rack/src/lib.rs:4408`).

## Smallest closable slice

Authorized paths: `crates/rack/src/lib.rs`, `crates/graph/src/runtime.rs` (only to reserve one dump buffer per chain at bind if the arena must own it), their tests, and this spec.

When `full_bank` is false: gather with the tiled path using the silence buffer for inactive lanes; scatter with the tiled path into the member buffers for active lanes and into the dump buffer for inactive lanes (or into staging and copy only active lanes out, whichever keeps `write_stereo_many` satisfied). Keep the ragged frame tail on the existing per-lane code. Do not change kernels.

## Non-goals

No change to which cohorts get banked (that is the identity-lane issue), no kernel change, no fold change.

## Objective gates

1. New test: for every active count 1..W-1 and random active-lane masks, the gathered scratch and the scattered outputs are bit-identical to the current per-lane path on random input (extend `full_bank_gather_scatter_round_trip_is_bit_exact`).
2. Allocation test: preparing and rendering a partial bank allocates nothing on render (use the crate's existing allocation-counter test pattern).
3. `cargo test -p rack` green; `scripts/check-rack-policy.sh`, `tools/realtime-policy` pass.
4. One descriptive `scripts/run-rack-benchmark.sh` run before/after, attached.

## Dependencies

None. Unblocks the identity-lane issues.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`tools/realtime-policy` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Amendment (2026-09-29, console strip, owner decision 12)

Decision 12 (`docs/rulings/engine-footprint-2026-09-29.md`) makes console slots always bank, with
partial groups padded with inactive lanes. The umbrella is *Console strip: session-level console
effects with per-track inserts*. What changes here:

- **Weight, not scope.** After *Bind every console slot banked for every track count* (S2), every
  console remainder is a padded partial bank. The scalar strided gather and scatter then runs on
  every session whose track count is not a multiple of W. This stays a class-A performance issue.
- **Dependencies.** *Add the console-strip benchmark rows* (#1085), whose N = 10 and 13 rows the
  replaced gate 4 reads. There is no functional dependency. It no longer unblocks the identity-lane
  issues: #888 and #889 are narrowed (see their amendments). The padding slices do not depend on it
  either, because the rack already runs partial chains correctly (Sol's M1 in
  `docs/handoffs/console-strip-2026-09-29/VERIFY.md`).
- **Inactive lanes.** The padding contract feeds a padded lane `+0.0` and discards its output (*Let
  an effect bank bind a partial group with inactive lanes*, P2a). The tiled path must keep both:
  gather inactive lanes from `ARENA_SILENCE_BUFFER`, and never write their output to a member
  buffer.
- **Gate 4 is replaced.** `scripts/run-rack-benchmark.sh` no longer exists: #1026 retired the rack
  benchmark. Read the motion on the console benchmark's remainder rows (N = 9, 10 and 13, added by
  *Add the console-strip benchmark rows*) with one `--step` run, one warmup and two measured rounds.

## Decision record (2026-10-24, attempt 1)

Scope note `scope/887.md` and root's decisions supersede the slice above where they differ:

- **No dump buffer and no runtime change.** A padded lane's `bank_inputs` entry is already
  `ARENA_SILENCE_BUFFER`, and `distinct_planes_mut` declines for a partial bank, so the staged
  scatter is the partial bank's path. `crates/graph/src/runtime.rs` is not touched.
- **A chain-owned `+0.0` plane.** A partial chain allocates one `quantum`-word zero plane at
  construction (`4 * quantum` bytes; 512 B at quantum 128); a full chain allocates none. The tiled
  gather reads it for every inactive lane, so the rack never asks a provider for an inactive
  lane's plane, and an inactive scratch lane is `+0.0` at the start of every block (the scalar
  path left it stale).
- **Staging for every chain.** `staging_left/right` (`2 * W * quantum * 4` bytes) are now
  allocated for a partial chain too.
- **Every retained byte is charged** (root, 2026-10-24, authorizing the paths outside the slice).
  `effect_bank_resource` already charged staging and now adds a padded bank's zero plane to
  `effect_bank_runtime_buffer_bytes`; `builtin_bank_resource` now charges every builtin bank's
  staging (never charged before, full banks included) and a padded bank's zero plane in
  `builtin_bank_scratch_bytes`. Pinned by `a_padded_bank_charges_member_metadata_per_active_member`
  and `builtin_bank_resource_charges_two_planes_and_actual_members`; the exact construction
  allocations are `allocation_tracker.rs`'s. Re-pinned by the +10 % rule: the browser identity
  fixture's two graph plan rows (35,648 -> 49,280 / 49,536), capi's reference-session graph and
  builtin-scratch rows (reasons in `resource_lifecycle.rs`), and the `direct-route` graph fixture.
- **Scatter.** The direct-plane scatter now also requires a full bank in the chain itself, not
  only in the provider. The staged path copies out, or folds, only active lanes. Resident
  acquisition copies the whole block on every bank.
- **The scalar move is test-only.** `scatter_lane` and the per-lane gather/scatter loops are the
  `#[cfg(test)]` oracle behind `force_scalar_transpose`; `gather_lane`/`gather_lane_left` remain
  production only for the ragged frame tail.
- **Gate 1** is `tiled_transpose_matches_the_scalar_path_bit_for_bit`, extended over every active
  mask `1..2^W` at both widths and every frame shape, and asserting that inactive scratch lanes
  are fed `+0.0` each block. Recorded red on the pre-change code (`tiled, 4 lanes, mask 0x1,
  3 frames`: the stale inactive word). Gate 2's allocation evidence is the existing graph/console
  render allocation gates (the zero plane and staging are made in `BankChain::new`). Gate 4 is
  read by the coordinator after the batch.
