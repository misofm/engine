# Give the multiband compressor a mono-collapse body

Filed by root after #973 (backlog sprint, 2026-10-24). A handoff promised these mono-body siblings, but they were never filed.

## Product outcome

A mono track whose chain holds `miso.multiband-compressor` can mono-collapse like the compressor, parametric EQ, true-peak limiter and (since #973) gate-expander. Today `multiband-compressor` keeps every default of the mono trio, so its bank never collapses and a mono strip with it pools Stereo.

## Smallest closable slice (follow #973 as the template)

- One kernel body with a const MONO parameter (one implementation shape, the same per-lane arithmetic): the collapsed form reads and writes only the left plane.
- `supports_mono_collapse`, `process_bank_mono` (sharing one inlined body with `process_bank`), `desymmetrize_channels` (copy every word the kernel reads), `channels_agree`, `lane_channel_symmetry`, and the scalar `channel_symmetry` on the same designed-words witness.
- D7 in a collapsed block re-seeds a failed lane on both channels.
- Wasm KERNEL_ROSTER: split the effect's f32x4 row into dual and collapsed rows if a second kernel appears (each pattern matches exactly one kernel; never loosen).
- console-workload `tests/paired_spans.rs`: add the effect's A3 per-effect obligation and its id to `expected`.

## Gates

- `crates/multiband-compressor/tests/mono_collapse.rs` (new): a collapse then a disengage equals a never-collapsed bank, bit for bit; the witness declines on exactly the asymmetric lane; and a test that the `channels_agree` word list is complete.
- The randomized differential with `witness: true`; `cargo test --locked --release -p console-workload --test paired_spans`; check-web-audioworklet.sh; check-cross-targets.sh.
- Class A: a one-time digest of dual outputs before and after (PR evidence).

## Owner rule

Only mono source files get dual-mono processing; no content detection.

## Decision and evidence record (attempt 1, 2026-10-24)

* Scope note `scope/1871.md` accepted by root; its path list replaces the spec's. Exact in all
  three link modes: the collapsed body calls `link_levels::<LINK>(own, own)` in the dual order.
* Kernel: one body, `run_segment::<.., ARMABLE, MONO>` / `process_block::<.., MONO>` /
  `render_body::<.., MONO>` (`render` is its dual instantiation, so `corpus.rs` and `split.rs` are
  unchanged). `MONO` plans, snaps, arms, counts silence and writes ramps back on side 0 only; the
  right plane and side 1 are never read or written.
* D7 collapsed (`finish_mono`): the left plane decides; a failed lane is zeroed on the left and
  reset on both sides, and counts one block on each channel.
* `process_bank_inner::<MONO>` (`#[inline(always)]`) serves `process_bank` and
  `process_bank_mono`. #893's silent path: `quiet`, the fixed-point skip's silence advance and the
  re-earned claim read the left plane and side 0. A claim earned collapsed holds on the next dual
  block because the disengage copies the resting left side first.
* `desymmetrize_channels` copies `Side.{filter, silence, gain_db, ramps}`; `channels_agree`
  compares those words bit for bit. `lane_channel_symmetry` and the scalar `channel_symmetry`
  compare the 17 ramps' four words. The randomized differential runs with `witness: true`.
* Class A (PR evidence, not committed): every console `*output_sha256` (42 digest fields of one
  untimed `bench console` warmup run) is identical on base `e68454874` and on this branch; the
  committed corpus digests (`cross_target_digest`) are unchanged.
* Wasm: shipped module 4,569,236 B on base, 4,603,306 B on the branch (+34,070 B, +0.75 %; the
  `MONO` monomorphs). `process_bank` keeps 2,527 `f32x4` operations (scalar 20 on base, 0 on the
  branch, codegen); `process_bank_mono` is 1,389 / 0, a new pinned roster row. Every
  browser expected-resources row is unchanged against base, so nothing is re-pinned.
