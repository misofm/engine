# Drop the per-frame identity select from settled 2x2 matrices that have no identity lane

Gain/pan stage, slice 1 (research 2026-09-26, base `main` at `14f2917b`; every `file:line` below
was read on that tree). The research question was whether the gain/pan stage pays for parameter
smoothing per track in scalar code, or per sample while a parameter is static. It does neither
("Research findings", Q2 and Q3). This issue is the smallest closable slice of the cost the
measurement found instead; the rest are named successors. The in-process harnesses the findings
cite are preserved as `docs/handoffs/gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch`.

## Product outcome

`sixty_four_track_gain_pan_only` spends a quarter of its block in the settled pan-matrix pass, and
three quarters of that on one instruction. The pan matrix of every bank renders through
`matrix2x2_block` (`crates/lane/src/kernels/builtins.rs:258`),
whose left output is `store(left, select(identity, load(left), ll*l + lr*r))`. On `x86-64-v3`,
LLVM 22 folds that store-of-a-select-of-the-same-load into one `vmaskmovps` masked store with
mask `!identity` (disassembly of the release `bench` binary, quoted under Research findings). On
the measurement host (AMD EPYC 7313P, Zen 3) that store costs about 10 extra cycles per frame. The
matrix slot measures 1,810 cycles per bank per block in the real plan, against 470 in a replica of
the same arithmetic written without the select. Over the row's 8 banks that is about 10,000 of the
block's 57,000 cycles.

On the standing fixture no lane is ever the identity: the pan law cannot produce one, because
`cos(pi/2)` rounds to `6.1e-17` in `f32`. So `identity` is all-false and the select always
returns its second arm. Render the select-free arm when no lane of the bank's identity mask is
set, and the existing kernel otherwise. Class A: `select(no lane, a, b)` is `b` bit for bit.

A throwaway prototype of exactly this change, measured in-process and pinned to one core,
rendered all 16 standing console workloads to the base digests over 64 blocks. It moved the row
from 15.23-15.52 us to 12.63-12.66 us per block (two interleaved rounds). That figure is
descriptive only; the paired console benchmark is run later by someone else.

## Amendments (adversarial verification, 2026-09-26; these override any conflicting text below)

The verification reproduced the root cause (the settled loop's `vmaskmovps` folded from the
select; the new kernel emits two `vmovups`, and the mixed arm keeps its masked store), the cost
(about 9.7 cycles per frame on Zen 3; uops.info gives 12 on Zen 2-4, 0.5 on Zen 5 and 1 on Intel;
NEON and wasm drop a bitselect, so the new kernel is never slower on any target), the saving
(about -2.8 us per block on the gain/pan row in process), all 16 console digests, the gate-4 pins,
mutations M1 and M3, and that `:2988` and `:3017` are the only callers. Evidence:
`docs/handoffs/gain-pan-2026-09-26/GAIN-PAN-VERIFY.md`.

1. **NaN payloads (gate 1, class statement).** In the release build CI runs (`--release -p lane`,
   `qualification.yml:526`) LLVM commutes the two products it adds: today's kernel computes
   `rr*r + rl*l`, the new one `rl*l + rr*r`. When both terms are NaN with different payloads, x86
   keeps the first operand's, so the words differ (`new=7fc00000 old=7fc01234`). Real renders cannot
   see this: the input stage sanitises NaN before the matrix, and no digest moves. Class A is
   therefore stated as "every non-NaN word is unchanged, and a NaN word stays a NaN". Gate 1 compares
   NaN words as "both NaN" and every other word by bits, and runs in both dev and release.
2. **Gate 1 negative control.** At `f32` with `frames = 1` the two kernels cannot differ; assert the
   control over the union of frame counts, not per frame count.
3. **Gate 3 must catch M1 by bits.** Include an identity-lane frame with `l = -0.0, r = +0.0`, so
   mutation M1 is caught by the output words and not only by the `debug_assert`.
4. **Gate 7 (wasm).** `check-web-audioworklet.sh` stops at the artifact pin before a repin, and rule 3
   never applies here (`BuiltinMatrixBank::process` does not match `4wide6f32x[48]`). Instead: build
   the AudioWorklet wasm with the build script's own cargo line, pipe `wasm-objdump -d` into the two
   callgraph checks, and record that the kernel count stays 15 and the render closure is unchanged.
   The artifact pin itself is repinned once at the batch boundary.
5. **Stale docs.** `tools/bench/src/floor.rs:236-240` and the `GainPan` doc
   (`tools/console-workload/src/lib.rs:271-283`) say the matrix has no data-dependent path; correct
   both. Ruling 4's `dispatch_only` doc fix (`tools/console-workload/src/lib.rs:150-179`,
   `:682-694`) is done here too. Those three places are authorized, docs only.
6. **Class B successor S-E, one more hazard.** A mute folded in as a zero column is not a hard mute:
   `0 * inf` and `0 * NaN` give NaN, while today's `andnot` gives exactly `+0.0` for any input. When
   S-E is briefed it must keep mute as a mask, or decline composition while any lane is muted.

## Invariants

- **Class A.** Every rendered word is unchanged. Only the instruction sequence of an
  identity-free settled matrix block changes.
- The select-free arm is taken only when `L::mask_any(coef.identity)` is false over **all**
  `L::WIDTH` lanes, including padding lanes. That makes it exact for every word of the resident
  block, padding included. Partial banks carry identity padding lanes, so they keep today's kernel
  (see successor S-F).
- The mask is tested once per call, per bank per block, never per frame. No new field is added to
  `MatrixStage`. Its `size_of` feeds sealed resource accounting, as `InputStage::symmetry`'s doc
  (`crates/builtins/src/lib.rs:1042-1049`) records for its sibling.
- Render stays allocation-free, lock-free and syscall-free. `crates/builtins` and `crates/lane`
  stay free of `unsafe`. Only `crates/lane` names `wide`; the new kernel uses the `Lane` trait
  alone.
- Nothing else moves: not the ramp arm, not the fused serialized pair (`fader_matrix_block`), not
  the input stage, the fold, graph, rack or builtins-compiler.

## Interface contract

1. `crates/lane/src/kernels/builtins.rs`, new, beside `matrix2x2_block`:

   ```rust
   /// [`matrix2x2_block`] for a coefficient set with **no** identity lane: its second arm alone.
   #[inline(always)]
   pub fn matrix2x2_block_without_identity<L: Lane>(
       left: &mut [f32],
       right: &mut [f32],
       frames: usize,
       c: &Matrix2x2Coef<L>,
   )
   ```

   Precondition: `!L::mask_any(c.identity)`, checked with a `debug_assert!`.

   Frozen operation order, per frame:
   1. `l = load(left)`, `r = load(right)`
   2. `yl = c.ll.mul(l).add(c.lr.mul(r))`
   3. `yr = c.rl.mul(l).add(c.rr.mul(r))`
   4. `store(left, yl)`, `store(right, yr)`

   That is `matrix2x2_block`'s second arm verbatim, with no `L::select` anywhere in the body. A
   select, even one on a constant mask, is exactly what LLVM turns into the masked store.

   `matrix2x2_block` itself is **not** edited. It remains the oracle in
   `crates/lane/tests/fader_matrix.rs:66`.
2. `crates/builtins/src/lib.rs`, `MatrixStage<L>`: add one private method inside the existing
   `REALTIME_POLICY` region (`:2969-3025`):

   ```rust
   fn settled_block(&self, left: &mut [f32], right: &mut [f32], frames: usize) {
       if L::mask_any(self.coef.identity) {
           matrix2x2_block::<L>(left, right, frames, &self.coef);
       } else {
           matrix2x2_block_without_identity::<L>(left, right, frames, &self.coef);
       }
   }
   ```

   Both settled call sites use it: the `maximum == 0` arm (`:2988`), and the post-ramp tail
   (`:3017`), which runs after `sync_settled()` (`:3015`) has recomputed the mask. The scalar
   per-track stage `MatrixStage<f32>` takes the same method. There, `mask_any` is the lane's own
   flag.
3. No public API of `builtins`, `builtins-compiler`, `graph` or `rack` changes.

## Smallest closable slice

Authorized paths:

- `crates/lane/src/kernels/builtins.rs`: the new kernel and its doc.
- `crates/lane/tests/fader_matrix.rs`: gate 1.
- `crates/lane/tests/MUTATIONS.md`
- `crates/builtins/src/lib.rs`: `MatrixStage` only (the method, the two call sites, and one
  `#[cfg(test)]` thread-local witness beside the existing ones at `:871-878`).
- `crates/builtins/src/tests.rs`: gate 2's witness test.
- `crates/builtins/tests/matrix.rs`: gate 3.
- `crates/builtins/tests/MUTATIONS.md`
- `tools/console-workload/tests/chain_shape.rs`: gate 4.
- This spec.

Steps:

1. **First, on the unmodified base commit:** write gate 3's scenario test, which uses the public
   `builtins` API only. Run it with the expected digest left empty, and record the printed digest
   in this spec's evidence. Then pin it in the test.
2. Add `matrix2x2_block_without_identity` (contract 1). Its doc states why it exists, the
   precondition, and why it is class A.
3. Add `MatrixStage::settled_block` and route both settled call sites through it (contract 2).
   Add a `#[cfg(test)]` counter `MATRIX_SELECT_FREE_BLOCKS` that the select-free arm increments.
   Follow the pattern of `FILTER_PREFIX_KERNEL_FRAMES` (`:877`).
4. Gates below, then the evidence record.

## Non-goals

- The partial-bank case: a mask set only on padding lanes (S-F).
- An all-identity skip, and a unity-fader skip (S-G). Both change the floor accounting.
- Pairing fader and matrix under `Concurrent` delivery (S-C).
- The fold's tile copy (S-A).
- The other masked-store sites in the workspace (S-B).
- The ramping pass (#890).
- Any class-B composition (S-E).
- No change to `fader_matrix_block`, `matrix2x2_ramp_block`, or `docs/rulings/`: the identity
  inventory's sentence about a settled identity matrix (`effect-floor-accounting.md:477-480`)
  stays true.
- No floor recount (see "Rulings", item 2).

## Objective gates

1. **Kernel identity (lane).** Add `select_free_matrix_matches_the_select_form_when_no_lane_is_identity`
   to `crates/lane/tests/fader_matrix.rs`:
   - Reuse `compare_case`'s four hostile families (finite, signed zero, subnormal, non-finite
     with a NaN payload), the frame counts `[1, 3, 8, 9, 128]`, the guard words at both ends, and
     `L` in `f32`, `Simd4`, `Simd8`, under `CanonicalFpEnv::enter()`.
   - Assert that `matrix2x2_block_without_identity` equals `matrix2x2_block` with an all-zero
     identity mask, word for word, guard words included.
   - Negative control, in the same test: with the mixed mask
     `[1, 0, 1, 0, 0, 1, 0, 1]` on the signed-zero family, the two kernels differ in at least one
     word. This is why the arm is keyed on the mask.
2. **Dispatch witness (builtins).** Add a unit test in `crates/builtins/src/tests.rs` that builds
   `BuiltinMatrixBank::new` banks at `(Backend::Simd8, BankWidth::Eight)` and
   `(Backend::Simd4, BankWidth::Four)`, plus one scalar `MatrixBuiltins`:
   - Full bank, every lane non-identity: every settled block increments
     `MATRIX_SELECT_FREE_BLOCKS` exactly once.
   - Any lane identity, including padding lanes in a partial bank: no increment.
   - A ramp that ends mid-block toward a non-identity target: one increment for the tail.
   - A ramp that ends mid-block toward `Matrix2x2::IDENTITY` on one lane: no increment for that
     tail.
3. **Bits (builtins).** Add `settled_matrix_shapes_render_the_base_bits` to
   `crates/builtins/tests/matrix.rs`. It renders one deterministic scenario, 24 blocks of 128
   frames, through `BuiltinMatrixBank` at W4 and W8, with members `{1, W-1, W}`. The input is
   hostile, and it must include `-0.0` on every lane and both planes, subnormals, and magnitudes
   `2^-24..2^25`. The scenario covers:
   - (a) every lane non-identity;
   - (b) one member exactly `Matrix2x2::IDENTITY`;
   - (c) a retarget at block 3 with a 200-sample window, so the ramp ends mid-block and the tail
     is settled;
   - (d) a retarget to `IDENTITY` on one lane, ending mid-block.

   Fold every output word, and every block's `test_support::matrix_bank_lane_words`, into one
   SHA-256. Assert it equals the digest recorded on the base commit in step 1 of the slice.

   **Ramp endpoint.** Also assert directly, after the block in which each (c) and (d) ramp ends,
   the retargeted lane's `matrix_bank_lane_words` (`crates/builtins/src/lib.rs:5209`):
   - words `0..4` (current) equal words `4..8` (target), bitwise;
   - word 13 (the `u32` countdown) is `0`.

   The existing tests stay green unchanged:
   - `settled_identity_matrix_preserves_signed_zero` (`matrix.rs:152`)
   - `matrix_ramp_reaches_target` (`:185`)
   - `matrix_ramp_matches_reference_d11_law` (`:37`)
   - `banked_fader_and_matrix_are_bit_identical_to_the_per_track_sections`
     (`crates/builtins/tests/stage.rs:1067`)
4. **Rows (console-workload).** Add `the_select_free_matrix_arm_renders_the_base_bits` to
   `tools/console-workload/tests/chain_shape.rs`. It uses the file's `render` helper (`:30`) over
   `BLOCKS = 64` and pins these base digests (measured on `14f2917b`):
   - `sixty_four_track_gain_pan_only` `01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4`
   - `sixty_four_track_dispatch_only` `15688888612d161e507bc400b9eed356fc1776797c8c66ca52d1e7c9114d3a2d`
   - `sixty_four_track_builtins_only` `b63eccd09c19eb7a6e0608144024ac5b14c7d5f7d1c56012cbbd49d6aad8f7f0`
   - `sixty_four_track_console` `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de`

   The plumbing pin (`chain_shape.rs:891`, `57535244...`) must not move. Every other console
   workload's 64-block digest must not move either: check all of them before and after (the
   `digests` harness in `docs/handoffs/gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch`
   does this; apply it in a scratch copy, do not commit it) and attach both outputs.
5. **Mutations**, each applied alone and recorded red in the crate's `MUTATIONS.md`:
   - M1: `settled_block` always takes the select-free arm. Gates 2 and 3, and
     `settled_identity_matrix_preserves_signed_zero`, go red.
   - M2: `settled_block` never takes it. Gate 2 goes red. This is the only gate that sees a
     performance-only regression.
   - M3: swap `c.lr` and `c.rl` in the new kernel. Gates 1, 3 and 4 go red.
   - M4: the tail call site (`:3017`) keeps calling `matrix2x2_block` unconditionally. Gate 2's
     ramp-tail case goes red.
6. **Allocation, realtime and policy.**
   - `cargo test -p builtins-compiler --features test-support --test allocation_tracker`: its
     fallback paths render the unpaired `MatrixBankProcessor`.
   - `bash scripts/check-realtime-policy.sh`, `check-builtins-policy.sh`, `check-lane-policy.sh`.
7. **Browser artifact.** Run `bash scripts/check-web-audioworklet.sh`. It rebuilds the
   AudioWorklet artifact and runs `check-web-audioworklet-callgraph.py --kernel-shape
   --kernel-pattern '4wide6f32x[48]' --kernel-min 11`. Rule 3 (`:352-359`) requires every
   matching function that carries `f32x4` arithmetic to have more vector than scalar ops. The new
   kernel has no scalar tail, since an AoSoA block is whole frames, so it adds vector ops only.
   The same script also runs `--callgraph miso_engine_web_v1_render`: no allocator or trap may be
   added. The artifact pin and browser qualification are repinned once at the batch boundary.
8. **Toolchain and tests.**
   - `cargo fmt --all --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `cargo test -p lane`, `-p builtins` (with and without `--features test-support`),
     `-p builtins-compiler --features test-support`, `-p console-workload`, `-p host-core`
   - `bash scripts/check-builtins-fixtures.sh`
9. **Codegen evidence** (recorded, not asserted). In `objdump -d --no-show-raw-insn -C
   target/release/bench`, the settled identity-free loop of `BuiltinMatrixBank>::process`
   contains two `vmovups` stores and no `vmaskmovps`. The mixed arm keeps its masked store. Quote
   the loop.

## Console benchmark rows

- **Can move:** every row that prepares builtins, by about the same absolute amount (8 banks per
  64 tracks). `console-workload` prepares through `prepare_session_builtins`
  (`tools/console-workload/src/lib.rs:1042`), which is `Concurrent` delivery
  (`crates/builtins-compiler/src/lib.rs:3181-3212`). Under `Concurrent` delivery the fader and the
  matrix are never paired (`:1023-1024`), so every such row runs `MatrixStage::process`. The rows:
  - Primary: `sixty_four_track_gain_pan_only`.
  - Also `sixty_four_track_dispatch_only`, `sixty_four_track_builtins_only`, and the console,
    effect and mono rows.
- **Must not move, bit or unit:** `sixty_four_track_plumbing_only` and
  `sixty_four_track_plumbing_ring`.
- **No new row.** The ramp arm is untouched, and the post-ramp tail is covered by gates 2 and 3.
  A ramping row is successor S-D.

## Dependencies

None. The slice is independent of S-A (graph, disjoint files) and of #890 (the ramping pass; it
touches `FaderMatrixBankProcessor`, not `MatrixStage`'s settled arm).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes,
  loads, stores, copies or branches. Every "bit-identical" gate above is a hard stop, never a
  tolerance.
- The owner's copy rule: a block-sized copy on the render path exists only with a written
  justification that no in-place or direct-write form exists. This slice adds none.
- Render paths stay allocation-free, lock-free and syscall-free (`check-realtime-policy.sh` is
  mandatory).
- Run fmt, clippy and the focused tests before every checkpoint. Commit on a
  `codex/<issue>-<slug>` branch from synchronized `main`, and touch no path outside the list.
- Do not quote a projected saving. The paired console benchmark runs once at the batch boundary;
  `scripts/run-console-benchmark.sh` is not run by the implementer.

## What the implementer will hit

- **The fold is fragile, so check the emitted code.** Any `L::select` in the new body lets LLVM
  rebuild the masked store. That includes `select(no_lanes(), ...)` and a helper that "keeps the
  select but hoists the mask". Check the release `bench` binary, not a debug build: the workspace
  release profile is fat LTO with one codegen unit (`Cargo.toml` `[profile.release]`), so
  inlining differs from `dev`.
- **Swapping the stores does not help.** Storing right before left was measured and is just as
  slow (replica: 2,080 cycles per bank against 2,084).
- **Do not "fix" `matrix2x2_block` in place.** It is the oracle of
  `settled_fader_matrix_matches_the_two_primitive_oracle` (`crates/lane/tests/fader_matrix.rs:107`)
  and of gate 1. It is also still the correct kernel for mixed and identity banks.
- **Partial banks are not the target.** `MatrixStage::new` fills every lane past the members with
  `Matrix2x2::IDENTITY` and a zero window (`crates/builtins/src/lib.rs:2823-2853`), so a
  60-track session's last bank has identity lanes and keeps the masked store. That is S-F, not a
  bug in this slice.
- **Do not cache the mask test.** Keep it a per-call `mask_any`. A cached `bool` would be a new
  `MatrixStage` field, with the sealed-size and stale-cache hazards `InputStage::symmetry`'s
  doc (`:1042-1049`) describes.
- **The pan law never yields the identity.** `pan_matrix` (`:4253`) computes `cos` and `sin` in
  `f64` and rounds, and `cos(pi/2)` is `6.1e-17`, not `0.0`. A gate-3 case that needs an identity
  lane must use `Matrix2x2::IDENTITY` directly.
- **`sixty_four_track_dispatch_only` does not exercise an identity matrix.** Its "hard identity"
  pan is `Pan { left: 1.0, right: 1.0 }` (`tools/console-workload/src/lib.rs:689-693`), which
  routes both inputs hard right. So it moves with this slice exactly as `gain_pan_only` does. See
  "Harness finding" below; do not edit the row here.

## Research findings

### Host, instrument and method

- **Host.** AMD EPYC 7313P (Zen 3), Linux 6.8, rustc 1.97.1 / LLVM 22.1.6, `x86-64-v3`, cpu 31
  pinned, loadavg 1.1-5.7 (other agents building; every comparison below was run interleaved or
  back to back). `perf` is unavailable, so cycles are **derived**: nanoseconds times a core clock
  calibrated in-process from a dependent `f32` add chain at 3 cycles per add, 3.697-3.698 GHz on
  every run. This is the plumbing cycle's method
  (`docs/handoffs/plumbing-floor-2026-09-26/PLAN.md`, Part 2).
- **Graph phases.** `graph::test_only_phase_profile`, through the harness
  `gain_pan_profile.rs` in `docs/handoffs/gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch` (`phase_profile_rows`, `digests`).
- **Bank sub-phases.** A throwaway `Instant` lap probe inside `rack::BankChain::run_with_input`
  (`crates/rack/src/lib.rs:2234`), one lap per sub-phase. It is not committed. Each lap costs
  about 100 cycles per bank, measured as the probes-on minus probes-off difference of the graph
  `BANK` phase over 88 laps per block, and that is subtracted below.
- **Kernel replicas.** The harness `gain_pan_replicas.rs` in the same patch
  (`replicas`, `ramps`) runs the row's banked working set through the real `builtins` bank API, the
  raw `lane` kernels, and a verbatim replica of `fold_resident_tiles`. It asserts every candidate
  bit-identical to today's chain before timing it.

### Q1. Where the row's block goes

`sixty_four_track_gain_pan_only`, base `14f2917b`, probes-off p50 **15,420 ns = 57,027 cycles**,
against `sixty_four_track_plumbing_only` at 3,156 ns = 11,672 cycles. Bench records on the same
host (`artifacts/plumbing-floor/console-benchmark.accepted.jsonl`, uncontrolled) give 15.24 /
15.09 us and 3.34 / 3.38 us.

The unit schedule is 64 bound, 8 bank, 1 output (73 units), with bank shape `[8, 24]`: eight
chains of three runtime slots each, input, fader and matrix. Fader and matrix are **not** paired
on this row; see Q4.

| stage | code | cycles/block | us | per bank | vs its own floor |
|---|---|---:|---:|---:|---|
| 64 bound source copies | graph `BOUND` units (`FrozenGraphSource`) | 8,131 | 2.20 | -- | copies, floor 0; plumbing pays 7,378 |
| gather: planar to AoSoA tile transposes | `rack` `gather_tiled` (`:2649`) | 6,060 | 1.64 | 757 | transposes, floor 0; replica 540/bank |
| input: sanitise, trim, `+0.0`, boundary scan | slot 0, `identity_chain_block` (`lane` `:1175`) | 9,290 | 2.51 | 1,162 | 12 lane-ops, floor 6,642: 72 % |
| fader and mute, settled | slot 1, `gain_mute_block` x 2 (`lane` `:161`) | 2,860 | 0.77 | 358 | 2 lane-ops, floor 1,107: 39 % |
| **pan 2x2, settled** | slot 2, `matrix2x2_block` (`lane` `:258`) | **14,480** | **3.91** | **1,810** | 4 lane-ops, floor 2,214: **15 %** |
| fold: transpose back, route, master accumulate | graph `fold_resident_tiles` (`runtime.rs:2105`) | 13,230 | 3.58 | 1,654 | route + reduction, 4 lane-ops, floor 2,214 |
| per-bank control: drains, collapse test, slot dispatch, graph unit | `run_with_input`, `Runtime::execute` | 3,910 | 1.06 | 489 | floor 0 |
| enter, source, Output op (nothing left to reduce), exit | graph | ~450 | 0.12 | -- | -- |
| **smoothing and ramp arithmetic** | -- | **~0** | **0** | -- | settled arms only (Q2) |

The rows sum to about 58,400 against the 57,027 p50, a 2 % residual from the probe correction.

The bank units' 49,800 cycles, which is where the row's ~12.3 us over plumbing lives, broken
down:

- **Smoothing and ramps: about 0.** One `u32` max per plane per bank per block.
- **Fader and pan arithmetic at its select-free cost: about 6,600 cycles (1.8 us).** That is the
  fader's 2,860 plus the matrix's 8 x 470.
- **x86 masked-store penalty in the settled matrix: about 10,700 (2.9 us)** by the probe split,
  8 x (1,810 - 470). The prototype of this slice recovered 2.6-2.9 us.
- **Input sanitise and scan: 9,290 (2.5 us).** Required by D7 of every block.
- **The AoSoA round trip:**
  - the gather transposes, 6,060 (1.6 us);
  - the fold's transposes, route and master accumulation, about 6,900 (1.9 us), which replaces
    plumbing's own 4,743-cycle route and reduction;
  - the fold's per-tile stack copy, about 6,300 (1.7 us). That is S-A.
- **Per-bank dispatch and control: 3,910 (1.1 us).**

### The floor, derived as the plumbing floor was

The row's inventory is the identity inventory (`docs/rulings/effect-floor-accounting.md:440-462`;
`BUILTINS_IDENTITY_LANE_OPS = 22.0`, `tools/bench/src/floor.rs:125`, used for this row at
`:241-247`):

| term | lane-ops per lane-sample |
|---|---:|
| sanitise and trim | 7 |
| collapsed identity sections | 1 |
| boundary scan | 4 |
| fader and mute | 2 |
| pan matrix | 4 |
| route | 3 |
| reduction | 1 |
| **total** | **22** |

`22 x 16,384 / (8 lanes x 3.7 ops/cycle) = 12,177 cycles = 3.29 us` at 3.70 GHz. That is
0.743 cycles/lane-sample (`floor.rs:241`).

The gain/pan increment over plumbing's 4 lane-ops (2,214 cycles, 0.60 us) is 18 lane-ops: 9,963
cycles, 2.69 us. The ruling's 3.7 ops/cycle was probed on Zen 5; the plumbing plan's Zen 3 pipe
cross-check puts it within 8 % (PLAN.md Part 1). Every figure below scales with it.

| state (in-process, pinned) | p50 us | cycles | % of the 12,177 floor |
|---|---:|---:|---:|
| base `14f2917b` | 15.23-15.52 | 56,300-57,400 | 21.2-21.6 % |
| + this slice (prototype) | 12.63-12.66 | 46,730-46,840 | 26.0 % |
| + this slice + S-A (prototype) | 10.63-10.97 | 39,330-40,590 | 30.0-31.0 % |
| + S-C pairing on top of this slice (prototype) | 12.45-12.51 | 46,080-46,310 | 26.3 % |

### Q2. Is smoothing evaluated per sample when the parameter is static? No.

Every builtin smoother is linear and windowed:

- `BuiltinSmoothingPolicy` (`crates/builtins/src/lib.rs:378`) offers `None`, `LinearNUpdates` and
  `Linear64CoefficientUpdates`.
- Every live builtin is `BlockTarget` (fader, mute, pan, matrix, trim and polarity; the
  descriptors are at `:496-745`).
- There is no exponential smoother in builtins.

The settled arm is keyed on an **integer** countdown and never on a value:

- `FaderRampStage::process_plane` (`:2731-2745`) takes `gain_mute_block` on the settled `current`
  when `max(remaining[channel]) == 0` over `u32`s.
- `MatrixStage::process` (`:2979-2990`) takes `matrix2x2_block` on the same test.
- The input trim ramp's off gate is `InputStage::ramping` (`:998`).

A ramp's last frame **assigns** `target` (`gain_mute_ramp_block` step 3, `lane` `:211-229`;
`matrix2x2_ramp_block`, `:350`). The bookkeeping then re-assigns `current = target` and
`step = 0` for every lane whose `u32` reached 0 (`:2758-2772`), and recomputes the matrix
identity mask only for settled lanes (`sync_settled`, `:2902`). So the settled value is the
target, bit for bit, and a static parameter pays **no** per-sample smoothing arithmetic. The
effect-side law is the same (`ParameterSmoother`, `crates/effect-contract/src/lib.rs:1301`):
`next_value` (`:1369`) assigns `target` on update N. Even `SmoothingRule::OnePole99`, which no
shipped effect uses, is windowed and snaps.

**The rule, stated precisely.** A class-A fast path may key only on:

- an exact settled state: the integer countdown is 0, and the last update was an assignment of
  the target; or
- a bitwise `current == target` for a linear retarget (the stationary hoist at `:1350`).

It may never key on `|current - target| < eps`. An unwindowed one-pole has no exact settled
state, so it could only take a fast path by snapping, which is a numeric change (class B).

What a static parameter *does* still pay per sample:

- The identity **select** of a settled matrix: always evaluated (`lane` `:256`), and on x86 it is
  the masked store. This slice.
- A 0 dB unmuted fader's multiply-by-1.0 and `andnot`: exact under the pinned MXCSR `0x1F80`
  (`crates/lane/src/fpenv.rs`, FTZ and DAZ clear), so elidable. S-G.
- An exactly-identity matrix's two arms: elidable. S-G.

### Q3. Do 8 tracks' ramps run in one vector? Yes, already, and bit-identical to per-track

- `GainMuteRamp<L>` (`lane` `:172`) and `Matrix2x2Ramp<L>` (`:321`) hold per-lane `current`,
  `target`, `step` and countdown, so lane k is track k's ramp with its own parameters.
- There is one generic body. The per-track scalar path is `FaderRampStage<f32>` and
  `MatrixStage<f32>`, and a bank is the same type at `Simd4` or `Simd8` (doc at
  `crates/builtins/src/lib.rs:2535-2570`).
- Per-lane operations, in order: `remaining - 1`, `le 0`, `select(done, target, current + step)`,
  then `load * current`, then `andnot(done & mute)` (fader) or the 2x2 (matrix). There is no
  `fma` (so the unfused-`fma` question does not arise) and no per-lane `max` or `min` (so D8
  does not arise). The only `max` is the block-level `u32` countdown maximum, which is integer
  control.
- A bank with one ramping lane runs the ramp kernel on all lanes. A settled lane there computes
  `select(done, target, ...) = target = current`, the same bits as the settled kernel.
- Gated today by `banked_fader_and_matrix_are_bit_identical_to_the_per_track_sections`
  (`crates/builtins/tests/stage.rs:1067`) and `a_banked_lane_ramps_exactly_as_the_same_track_alone`
  (`crates/builtins/tests/input_liveness.rs:707`).

**Ramp cost** (replica, every lane of every bank mid-ramp, per bank per block):

| stage | settled | ramping | note |
|---|---:|---:|---|
| fader | 388 | 1,090 | +700 |
| matrix, today | 1,710 | 840 | a **moving** pan is cheaper than a static one on x86: the ramp kernel has no select, so no masked store |
| matrix, after this slice | 470 | 840 | +370 |

A fully ramping 64-track row would add about 8,500 cycles (2.3 us) per block over settled after
this slice. The fused ramping pass is #890 (open), and a row that measures it is S-D.

### Q4. Are fader, mute, pan and matrix one pass? No: on this row they are two slots

Fader and mute are one pass (`gain_mute_block`, once per plane); the matrix is a second pass.
Each bank block passes over its 8 KiB resident AoSoA scratch five times in all:

1. gather (writes it)
2. input (reads and writes)
3. fader (reads and writes each plane)
4. matrix (reads and writes both planes)
5. fold (reads it; writes the master)

There is no intermediate buffer and no block-sized copy inside the strip; every stage is in
place, L1-resident.

The fused settled kernel exists: `fader_matrix_block` (`lane` `:292`), reached through
`BuiltinFaderBank::try_process_settled_with_matrix` (`crates/builtins/src/lib.rs:3796`) inside
`FaderMatrixBankProcessor` (`crates/builtins-compiler/src/lib.rs:1043-1089`). But
`make_fader_matrix` pairs only `BetweenRenderCalls` banks (`:1023-1024`), because #430/#444 found
no queue cutoff for concurrent producers.

| host | delivery | fader and matrix |
|---|---|---|
| web host, main path | `BetweenRenderCalls` (`hosts/host-web/src/lib.rs:7777-7797`) | paired |
| C ABI (`crates/capi/src/runtime/compile.rs:410` into `prepare_host_runtime`, `crates/host-core/src/prepare.rs:508`) | `Concurrent`, console-free | separate |
| console benchmark | `Concurrent`, console-free | separate |

Measured fusion values:

- **Pairing** (S-C), on top of this slice: 180 cycles per bank in the kernel plus one slot
  dispatch, -0.32 us per block in-situ.
- **Fusing fader and matrix into the fold as well:** replica 1,340 against 767 + 665 = 1,432 per
  bank, -90 per bank. A measured null; not briefed.

**Copies.** The 64 bound copies are the harness's feed (#927/#928 own the in-place source path).
The one unnecessary copy on the row is inside the fold: `tile_rows` (`runtime.rs:2174`) compiles
to a zero-filled 256-byte stack array plus a `memcpy` call through the PLT, with `vzeroupper`, per
plane per tile (4 `memcpy` calls in `fold_resident` in the release `bench` binary). That is S-A,
first in line under the copy rule.

### Q5. Class-B opportunities (owner rulings, never in a class-A slice)

- **S-E. Compose fader, pan and route into one settled 2x2 per track.** Use `C = R * M * G` (the
  route already carries its gain, D3), applied by the fold's existing `route_word`. While
  settled, the fader and matrix passes disappear entirely.
  - Replica: gather + input + fold at 2,373 against 3,300 cycles per bank after this slice, so
    about **-930 per bank, about -2.0 us per block**, plus two slot dispatches.
  - The inventory falls from 22 to 16 lane-ops (floor 12,177 to 8,856).
  - Rounding changes (`ll*(g*l)` against `(ll*g)*l`): re-baseline every with-builtins digest.
  - A muted channel becomes a zero column, so a master word can become `-0.0` where it was
    `+0.0`, when every contribution is zero.
  - Composition must decline whenever a post-fader or post-matrix tap is observed (meters, sends)
    and while any lane ramps. The fold eligibility machinery already declines on observation.
  - The composed words cross the builtins-to-graph boundary each settle, which is an interface
    change.
- **S-E', the smaller variant: fold only the fader gain into the pan matrix (`M * G`).** It
  removes the fader pass: about -358 cycles per bank, about -0.75 us per block, 2 lane-ops. The
  same mute and rounding rulings apply.

### The emitted settled matrix loop

`objdump -d --no-show-raw-insn -C target/release/bench`, `<builtins::BuiltinMatrixBank>::process`
at `14f2917b`: the `f32x8` settled loop, one frame per iteration. `ymm5` is `!identity`, so on the
standing fixture it is all ones.

```text
vmovups (%rdx,%rax,1),%ymm6          # l
vmovups (%r8,%rax,1),%ymm7           # r
vmulps  %ymm6,%ymm1,%ymm8            # ll*l
vmulps  %ymm7,%ymm2,%ymm9            # lr*r
vaddps  %ymm9,%ymm8,%ymm8            # yl
vmulps  %ymm6,%ymm3,%ymm6            # rl*l
vmulps  %ymm7,%ymm4,%ymm9            # rr*r
vaddps  %ymm6,%ymm9,%ymm6            # yr
vblendvps %ymm0,%ymm7,%ymm6,%ymm6    # select(identity, r, yr)
vmaskmovps %ymm8,%ymm5,(%rdx,%rax,1) # select(identity, l, yl) folded into a masked store
vmovups %ymm6,(%r8,%rax,1)
```

The same shape appears at `f32x4`, and in the post-ramp tail call. `matrix2x2_ramp_block` has no
select and emits two plain stores, which is why a ramping pan is cheaper than a settled one (Q3).

### Masked stores elsewhere (evidence for S-B)

`objdump -d target/release/bench` at `14f2917b` (release: fat LTO, one CGU) finds `vmaskmovps`
stores in:

| function | stores | note |
|---|---:|---|
| `BuiltinMatrixBank>::process` | 4 | settled and tail loops, `f32x8` and `f32x4` |
| `parametric_eq` `process_bank` | 4 | inside the per-sample section loop (not attributed further) |
| `parametric_eq` `process_bank_mono` | 2 | |
| `transient_shaper` `process_bank` | 6 | |
| `InputStage<f32>::process` | 3 | scalar strips |
| `InputStage<f32x8>::settle` | 2 | per block |
| `InputStage` `settle_filter` | 5 at `f32x8`, 2 at `f32x4` | per block |
| `BuiltinInputBank::process` | 2 | per block |
| graph `ArenaMembers::fold_plane` | 2 | |

The EQ sites are in its hot loop and were not timed here.

### Harness finding (not briefed)

`sixty_four_track_dispatch_only` claims to ask the matrix for "hard identity"
(`tools/console-workload/src/lib.rs:150-179`, `:682-694`). Its `Pan { left: 1.0, right: 1.0 }` is
hard right on both inputs: `ll = lr = 6.1e-17`, `rl = rr = 1.0`. No standing row exercises an
identity matrix. The pair `gain_pan_only` / `dispatch_only` still share one inventory correctly,
since neither has an identity arm, but the row's doc is wrong. Correcting the row to
`MatrixOrPan::Matrix { 1, 0, 0, 1 }` would move its digest, and after S-G its cost. The owner
decides whether to change it.

### Named successors

Each is small, class A unless marked, and independent unless noted.

- **S-A. Read fold tiles without a stack copy** (graph).
  - Change: `tile_rows` (`crates/graph/src/runtime.rs:2174`) builds each row from
    `block.as_chunks::<W>()` (fixed-size `[f32; W]` rows), not a runtime-length `copy_from_slice`
    that LLVM lowers to `memcpy`.
  - Prototype: fold 1,754 to 963 cycles per bank in-situ, -1.7 to -2.0 us on this row, all 16
    digests unchanged. It moves every banked row, since every folded bank takes
    `fold_resident_tiles`.
  - Also check rack's `tile_gather` and `tile_scatter` (`crates/rack/src/lib.rs:263-340`): the
    in-situ gather costs 757 per bank against the replica's 540. Not investigated.
  - First in line under the owner's copy rule.
- **S-B. x86 masked-store audit.** Rewrite, or dispatch around, each select-into-store site in the
  table above, starting with the EQ's per-sample loop. Add a static gate over the release `bench`
  binary that lists `vmaskmovps` stores in render-path functions against an allow list. Wasm and
  NEON have no masked store, so there the change is neutral.
- **S-C. Pair console-free fader and matrix banks under `Concurrent` delivery.**
  - Change: in `make_fader_matrix` (`crates/builtins-compiler/src/lib.rs:1023`), also accept a
    pair whose fader and matrix banks hold **no** consumer on any lane. With no queue there is no
    drain order to preserve, so the #444 admission question is vacuous. The consumer set is fixed
    at preparation: `into_graph_artifact_with_banks` moves each consumer exactly once.
  - Prototype: -0.32 us per block, digests unchanged. It moves the C-ABI console-free host and
    every console row.
  - Needs a coordinator ok, because #430/#444 scoped pairing to serialized delivery.
- **S-D. A ramping gain/pan row.** `sixty_four_track_gain_pan_ramping` prepares builtins with
  console channels, and every track's fader and pan are retargeted with windows longer than the
  measurement. That puts every bank in the ramp arm on every block; pin its digest. This is #890's
  measurement row; today's `console_automation` row ramps a compressor, not the strip.
- **S-E and S-E'.** Class B; above.
- **S-F. Partial banks.** Take the select-free arm when no **member** lane is identity. Padding
  lanes carry `IDENTITY`, so they would compute `1*l + 0*r`: unobservable, but padding scratch
  words can change `-0.0` to `+0.0`. Needs a written padding argument. Pairs with #887.
- **S-G. Prepared-identity elision for fader and matrix.**
  - Skip `gain_mute_block` when every lane is gain `1.0` and unmuted.
  - Skip the matrix pass when every lane is exactly `IDENTITY`.
  - Class A under the pinned MXCSR. It needs the floor-accounting amendment the input sections
    got (`effect-floor-accounting.md` appendix, `:1005`). No standing row moves until the
    harness finding is resolved.

### Considered and not briefed

- **A planar, no-transpose path for effect-free, filter-free strips.** A strip whose input
  sections are all elided is feed-forward, so it could be vectorised along frames per track and
  reduced by #926's pair-fused route and reduce (plumbing's 4,743 cycles for 64 tracks). That
  removes the gather (6,060) and the fold's transposes, an estimated 8,000-10,000 cycles per
  block. It is an architecture change (who banks a strip, and when), and strips with a real HPF
  or LPF recurrence must stay banked. It needs its own issue and an owner ruling.
- **Fusing the fader into the input pass.** Estimated at 0.5 us or less. The mono-collapse seam
  sits between the two stages (`crates/rack/src/lib.rs`, `run_with_input`), so it only works for
  never-collapsing chains.
- **Per-bank control, 3,900 cycles.** Split by the probes, not investigated further.
- **The 64 bound copies.** Owned by #927/#928.

### Rulings (coordinator, 2026-09-26)

1. **S-E / S-E' (class B).** The owner has ruled that a change which moves bits at the rounding
   level is wanted when it measurably improves performance, with digests re-baselined and the new
   arithmetic pinned. S-E is therefore queued, not refused. It is briefed after the class A slices,
   because it crosses the builtins-to-graph interface and needs its own adversarial review of the
   mute sign-of-zero change.
2. **Floor recount: no** in this slice; recount once, with S-G.
3. **S-C: deferred.** #430/#444 scoped pairing to serialized delivery; reopening that is its own
   issue.
4. **Harness finding: correct the row's doc, not its content.** `dispatch_only` keeps its digest;
   its doc is corrected to say both inputs pan hard right. An identity row is added only with S-G.
5. **S-A (the fold's tile copy) is #945,** filed now because it removes a copy on every banked
   row.

### Reproduction

```text
CARGO_INCREMENTAL=0 cargo test --release -p console-workload \
    --test gain_pan_profile --test gain_pan_replicas --no-run
taskset -c 31 target/release/deps/gain_pan_profile-<hash> --ignored --nocapture --test-threads 1
taskset -c 31 target/release/deps/gain_pan_replicas-<hash> --ignored --nocapture --test-threads 1
objdump -d --no-show-raw-insn -C target/release/bench | \
    awk '/^[0-9a-f]+ <.*>:$/ {f=$0} /vmaskmovps %/ {print f}' | sort | uniq -c
```

Replica per bank (cycles, three repeats within 1 %):

| stage | cycles per bank |
|---|---:|
| gather | 540 |
| input | 1,066 |
| fader | 378 |
| matrix, today | 1,709 |
| raw `matrix2x2_block` | 2,084 |
| select-free matrix | 470 |
| matrix with swapped stores | 2,080 |
| fused fader+matrix | 665 |
| fold (verbatim replica) | 767 |
| fold with fader+matrix inside | 1,340 |
| today's chain | 4,523 |
| chain with the select-free matrix | 3,300 |
| paired chain | 3,103 |

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), 2026-09-27, branch `codex/944-matrix-without-identity-select`
on top of #936 and #945 (`21a0dfcf`, the unmodified base of this attempt). Host AMD EPYC 7313P
(Zen 3), rustc 1.97.1, `CARGO_INCREMENTAL=0`, one private scratch target directory. No timed
benchmark was run.

Commits:

- `c41b1c97` test: gates 3 and 4 written and pinned on the unmodified base (step 1).
- `1f3a2ddf` the kernel, `MatrixStage::settled_block`, the witness, gates 1 and 2.
- `20295947` amendment 5 docs (floor comment, `GainPan` doc, ruling 4's `dispatch_only` doc).
- the evidence commit carrying this section and both `MUTATIONS.md` records.

### Design

- `lane::kernels::builtins::matrix2x2_block_without_identity` is contract 1 verbatim: two loads,
  `ll*l + lr*r`, `rl*l + rr*r`, two stores, no `L::select` anywhere, and
  `debug_assert!(!L::mask_any(c.identity))`. Its doc states why it exists, the precondition, and
  the class statement as amended (every non-NaN word unchanged; a NaN word stays a NaN).
  `matrix2x2_block` is not edited.
- `MatrixStage::settled_block` is contract 2 verbatim, inside the existing `REALTIME_POLICY`
  region, with the `#[cfg(test)]` increment of `MATRIX_SELECT_FREE_BLOCKS` (beside
  `FILTER_PREFIX_KERNEL_FRAMES`) in the select-free arm. Both settled call sites use it; the tail
  runs after `sync_settled`. No field was added to `MatrixStage`, and the mask is tested per call.
- **Deviation (small):** `settled_block` carries `#[inline(always)]`, as its neighbour
  `is_settled` does, so each width's body stays specialised inside `process`. The release
  disassembly below confirms all four loops are inline.
- Gate 1 reuses `compare_case`'s inputs by extracting them into `family_source`,
  `hostile_planes` and `hostile_matrix`; `compare_case` renders exactly the words it did.
- Gate 3's input has no non-finite word: every plan sanitises before the matrix, and a NaN
  payload is outside the amended class statement, so a NaN in the pinned scenario would make the
  pin depend on LLVM's operand order.

### Gates

| # | command | result |
| --- | --- | --- |
| 1 | `cargo test -p lane --test fader_matrix`, dev and `--release`; also CI's line `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | green (lane 52 tests dev and release; CI line 97). The negative control holds over the frame-count union at every width. With the NaN clause removed, release is red at `width=4 frames=1 family=3: R[3] new=7fc00000 old=7fc01234` and dev is green, reproducing amendment 1 |
| 2 | `cargo test -p builtins --lib settled_matrix_takes_the_select_free_arm_only_without_an_identity_lane` | green in dev and release. W8, W4 and scalar: 1 per settled block for a full non-identity bank; 0 for an identity member, a `W-1` bank, a 1-member bank; 1 for a mid-block tail toward a non-identity target; 0 for a ramp covering the block; 0 for a tail toward `IDENTITY` and every block after |
| 3 | `cargo test -p builtins --test matrix`, dev and `--release` | step 1 on `21a0dfcf` printed `0e1c5af8e3aa66b66b2bb127149a8eaf10529fed838d69a4912c73bf345640c3` in both profiles; pinned in `c41b1c97`. Re-run on the base checkout and on `20295947`, dev and release: the same digest. The ramp-endpoint asserts (words `0..4 == 4..8`, word 13 `== 0` after block 4) hold for (c) and (d) at every width and member count. The four named existing tests stay green |
| 4 | `cargo test -p console-workload --test chain_shape` | green on the base and on `20295947`: `gain_pan 01e465a7...`, `dispatch 15688888...`, `builtins b63eccd0...`, `console fe5bed9b...`, and the plumbing pin `57535244...` unmoved. All 16 `WORKLOADS` digests unchanged in dev and in release (table below) |
| 5 | mutations M1 to M4 | all red as the brief predicts; recorded in `crates/builtins/tests/MUTATIONS.md` and `crates/lane/tests/MUTATIONS.md` |
| 6 | `cargo test -p builtins-compiler --features test-support --test allocation_tracker`; `check-realtime-policy.sh`, `check-builtins-policy.sh`, `check-lane-policy.sh` | allocation tracker 9 passed; `realtime policy: ok (54 marked regions in 16 files)`; builtins and lane policy ok; `check-workspace-policy.sh` ok |
| 7 | amendment 4 (below) | kernel count 15, roster identical, render closure unchanged |
| 8 | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; `cargo test` of `lane` (dev, release), `builtins` (with and without `test-support`), `builtins-compiler --features test-support`, `console-workload`, `host-core`, `capi`, `bench floor`; `check-builtins-fixtures.sh` | all green: builtins 113 and 113, builtins-compiler 79, console-workload 37, host-core 174, capi 36, bench floor 10; fixtures ok |
| 9 | `objdump -d --no-show-raw-insn -C target/release/bench` | below |

### Codegen (gate 9)

Base (`c41b1c97`), `<builtins::BuiltinMatrixBank>::process`, settled `f32x8` loop: 4 `vmaskmovps`
in the function (settled and tail, `f32x8` and `f32x4`), no `vtestps`.

```text
vmovups (%rdx,%rax,1),%ymm6
vmovups (%r8,%rax,1),%ymm7
vmulps  %ymm6,%ymm1,%ymm8
vmulps  %ymm7,%ymm2,%ymm9
vaddps  %ymm9,%ymm8,%ymm8
vmulps  %ymm6,%ymm3,%ymm6
vmulps  %ymm7,%ymm4,%ymm9
vaddps  %ymm6,%ymm9,%ymm6
vblendvps %ymm0,%ymm7,%ymm6,%ymm6
vmaskmovps %ymm8,%ymm5,(%rdx,%rax,1)
vmovups %ymm6,(%r8,%rax,1)
```

After (`20295947`): each of the four sites is `vtestps` on the identity mask and a `je` to a
select-free loop. The settled `f32x8` loop:

```text
vmovups (%rdx,%rax,1),%ymm4
vmovups (%r8,%rax,1),%ymm5
vmulps  %ymm4,%ymm0,%ymm6
vmulps  %ymm5,%ymm1,%ymm7
vaddps  %ymm7,%ymm6,%ymm6
vmulps  %ymm4,%ymm2,%ymm4
vmulps  %ymm5,%ymm3,%ymm5
vaddps  %ymm5,%ymm4,%ymm4
vmovups %ymm6,(%rdx,%rax,1)
vmovups %ymm4,(%r8,%rax,1)
```

The settled `f32x4` loop and both post-ramp tail loops have the same shape at `xmm` and `ymm`:
two `vmovups` stores, no `vblendvps`, no `vmaskmovps`. The mixed arm keeps its masked store
(the function still has 4 `vmaskmovps`, all in the select form, and 12 `vblendvps` before and
after). The workspace masked-store census (the brief's table) is identical before and after, per
function. Note that the select-free loop computes the right output as `rl*l + rr*r` where the
base computes `rr*r + rl*l`: that is amendment 1's operand order, visible in the code.

### Browser artifact (gate 7, amendment 4)

Built with `scripts/build-web-audioworklet.sh`'s own cargo line (`RUSTFLAGS="-C
target-feature=+simd128 -C strip=debuginfo <remaps>" cargo build --locked --release --target
wasm32-unknown-unknown -p host-web`) from one scratch checkout at base and after, and
`wasm-objdump -d` piped into `check-web-audioworklet-callgraph.py`:

| | base | after |
| --- | --- | --- |
| artifact sha256 | `0b5d6055...` | `025b6b20...` |
| `--callgraph miso_engine_web_v1_render` | closure 8, traps 5, one owner (`render_inner`) | identical |
| `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` | 15 kernels, `f32x4_arith` 11,683 | 15 kernels, 11,725; roster lines identical |
| `meter_poll`, `command_submit` callgraphs | pass | identical |
| `BuiltinMatrixBank::process` ops | 1,648 | 1,963 |
| its `v128.bitselect` / `v128.store` / `f32x4.mul` | 24 / 40 / 36 | 24 / 52 / 60 |

The new arms add 24 multiplies and 12 stores (two widths, two sites) and no `v128.bitselect`,
so the select-free arm carries none. The artifact pin `8934cdd9...` is not repinned here
(it already differs on the base, which carries #945); it is repinned once at the batch boundary.

### All console workloads, 64 blocks (gate 4)

Harness: `digests` from `gain-pan-diagnosis-harnesses.patch`, applied only in a detached scratch
worktree (moved from `c41b1c97` to `20295947`; removed afterwards). Dev and release outputs are
identical, before and after:

```text
nine_track_baseline                 e7c6ef01770ab7da98d4b793a6a817bd50a4dfe682321ecfc023ddc275285a80
nine_track_ragged_strip             17613a3ab693d3f0dfc457b41f77669f880581fa19c433941a1ef2437684198a
sixty_four_track_console            fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de
one_twenty_eight_track_stretch      cba2c94f81544caad0945f0720480b568b1a47808d25fd95911f61bd37f5f9b1
sixty_four_track_eq_only            9b2c56a1da62ebda8aef595973870ea077477d209aacdcd6321637e949d5c828
sixty_four_track_compressor_only    95c9375429fbca3449bf9c5134508a6220f17fc9adda0b3267c061b75bb14175
sixty_four_track_builtins_only      b63eccd09c19eb7a6e0608144024ac5b14c7d5f7d1c56012cbbd49d6aad8f7f0
sixty_four_track_dispatch_only      15688888612d161e507bc400b9eed356fc1776797c8c66ca52d1e7c9114d3a2d
sixty_four_track_idle               de2f256064a0af797747c2b97505dc0b9f3df0de4f489eac731c23ae9ca9cc31
sixty_four_track_console_legacy     f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177
sixty_four_track_eq_comp_simd1      f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177
sixty_four_track_plumbing_only      57535244ba953d82f6c9c19428dc83a8ac412018c66acc167818e1917283f800
sixty_four_track_gain_pan_only      01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4
sixty_four_track_console_mono       fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7
sixty_four_track_console_mono_dual  fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7
sixty_four_track_console_half_mono  4a656cdf63882999b720b7dd765c1b4f7bbcb95e2ab38c10445f71d269665180
```

`sixty_four_track_plumbing_ring` is not in `WORKLOADS`; its own `console-workload` tests are green.

### Mutations (gate 5)

| # | mutation | red |
| --- | --- | --- |
| M1 | always select-free | gate 2, gate 3, `settled_identity_matrix_preserves_signed_zero` (dev by the kernel's `debug_assert!`; release by the counter and by bits, gate 3 digest `b76c1305...`) |
| M2 | never select-free | gate 2 only (`full non-identity bank`, 0 against 3) |
| M3 | `lr` and `rl` swapped in the new kernel | gate 1 (dev and release), gate 3, gate 4 (`gain_pan_only` digest `b8332f35...`) |
| M4 | tail keeps `matrix2x2_block` | gate 2 only (`settled tail`, 0 against 1) |

### For the verifier

1. **A shared-target incident.** The first pass of this attempt used a scratch target directory
   that another agent's verifier was using and deleted at about 00:43 UTC. Every result in this
   section was re-run afterwards in a private target directory. In that window, one gate-4 run
   under M2 failed on its first workload. It never reproduced: 4 more runs under M2 and 53 runs of
   the committed test binary in that same window, then the clean mutation pass in the private
   target. It is attributed to the deletion, and it
   is recorded here rather than dropped.
2. `cargo test -p builtins-compiler` **without** `--features test-support` does not compile (its
   lib tests call graph's test-only functions). `builtins-compiler` is untouched here, and the
   brief's gate uses the feature, which is green.
3. **Stale sentences outside the authorised ranges, not edited.** Each still says
   `dispatch_only` asks the matrix for the identity, or that an identity matrix costs what a real
   one costs:
   - `tools/console-workload/src/lib.rs:232`, the plumbing row's doc;
   - `tools/bench/src/floor.rs:121-123`, the `BUILTINS_IDENTITY_LANE_OPS` doc;
   - `tools/bench/src/floor.rs:580`, a test doc.

   The `floor.rs:122` clause "a settled identity matrix still evaluates both arms" is still true
   of a real identity matrix. They belong with S-G, or with the batch-boundary doc pass.
4. `check-web-audioworklet.sh` itself was not run (amendment 4).
