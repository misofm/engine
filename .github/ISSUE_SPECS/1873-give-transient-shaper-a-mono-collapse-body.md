# Give the transient shaper a mono-collapse body

Filed by root after #973 (backlog sprint, 2026-10-24). A handoff promised these mono-body siblings, but they were never filed.

## Product outcome

A mono track whose chain holds `miso.transient-shaper` can mono-collapse like the compressor, parametric EQ, true-peak limiter and (since #973) gate-expander. Today `transient-shaper` keeps every default of the mono trio, so its bank never collapses and a mono strip with it pools Stereo.

## Smallest closable slice (follow #973 as the template)

- One kernel body with a const MONO parameter (one implementation shape, the same per-lane arithmetic): the collapsed form reads and writes only the left plane.
- `supports_mono_collapse`, `process_bank_mono` (sharing one inlined body with `process_bank`), `desymmetrize_channels` (copy every word the kernel reads), `channels_agree`, `lane_channel_symmetry`, and the scalar `channel_symmetry` on the same designed-words witness.
- D7 in a collapsed block re-seeds a failed lane on both channels.
- Wasm KERNEL_ROSTER: split the effect's f32x4 row into dual and collapsed rows if a second kernel appears (each pattern matches exactly one kernel; never loosen).
- console-workload `tests/paired_spans.rs`: add the effect's A3 per-effect obligation and its id to `expected`.

## Gates

- `crates/transient-shaper/tests/mono_collapse.rs` (new): a collapse then a disengage equals a never-collapsed bank, bit for bit; the witness declines on exactly the asymmetric lane; and a test that the `channels_agree` word list is complete.
- The randomized differential with `witness: true`; `cargo test --locked --release -p console-workload --test paired_spans`; check-web-audioworklet.sh; check-cross-targets.sh.
- Class A: a one-time digest of dual outputs before and after (PR evidence).

## Owner rule

Only mono source files get dual-mono processing; no content detection.

## Decision and evidence record (attempt 1, 2026-10-24)

* Scope note re-checked against main `9ac7d0d6b` (after #1368b, PR #1902): the per-lane link
  (`LinkLanes`, `LINK_MIXED`, `set_lane_link`) is in, so the slice branches from it as planned.
  Changed from the note: the wasm roster row already pinned `13process_block` (#1368b), so the
  block body is now `#[inline(always)]` and the two rows pin the bank's `12process_bank` and
  `17process_bank_mono`; `src/corpus.rs` calls `process_block::<false>`.
* `step`/`run`/`process_block` take `const MONO: bool`. Collapsed: the left plane, envelopes and
  ramps only; the ramp prefix is the left ramps' (partition-invariant, and the dual one on
  `L == R`); the silent path reads the left plane and left rest; a D7 failure on the left plane
  is recovered on both channels and charged on both. `process_bank_inner::<MONO>` is shared.
* **A collapsed lane's link partner is its own word**: every arm computes `link(x, x, lanes)`,
  the dual expression on `L == R` in the same order; the mixed arm selects per lane its own
  mode's law of `(x, x)`. No mode declines. Proved bit for bit by the transition test on each
  uniform mode and on a mixed bank (lanes cycle DualMono, Maximum, Average).
* `desymmetrize_channels` copies `right_env = left_env` and `right.ramps = left.ramps`;
  `channels_agree` compares the same 14 words per lane bit for bit. `DESIGNED` witness (bank and
  scalar): the lane's three ramps, four fields each. Excluded: envelopes (running state),
  `Ramps::defaults` (never read), links/coefficients/bypass (one word for both channels).
* Tests: `tests/mono_collapse.rs` (transition against a never-collapsed bank with a D7 recovery
  and ramps in flight at the disengage; the word lists and witness via restores); randomized
  differential `witness: true`; paired_spans A3 rows for the three modes.
* Class A (PR evidence, not committed): all 42 console `*output_sha256` fields of one untimed
  `bench console` warmup are identical on base `9ac7d0d6b` and the branch (the console fixtures
  hold no shaper, so this proves no other effect moved); the corpus digests (`tests/cross_target.rs`)
  are unchanged and green.
* Wasm: shipped module 4,627,981 B on base, 4,641,920 B on the branch (+13,939 B, the `MONO`
  monomorph). Dual `process_bank` 700 / 0 (the base `process_block` was 700 / 0), collapsed
  `process_bank_mono` 352 / 0; both ceilings 0.10 (the old 0.38 row was 4 x the M2 786 / 72
  body). Expected-resources rows unchanged. `--kernel-min` stays 13 (29 kernels now), as #973,
  #1871 and #1872 left it.
