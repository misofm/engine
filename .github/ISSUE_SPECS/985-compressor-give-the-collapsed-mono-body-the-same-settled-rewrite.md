# Compressor: give the collapsed (mono) body the same settled rewrite


Compressor slice 5 of 5 (research 2026-09-27, base `6ca203f8`; builds on slices 1-4). Evidence:
`docs/handoffs/effects-2026-09-27/COMPRESSOR-DIAGNOSIS.md`. The prototype is
`settled_main_mono`, `mono_one` and `mono_two` in `compressor-diagnosis-prototypes.patch`
(mode 15). Neither patch may be committed.

## Product outcome

A track whose two channels read one source channel, with symmetric settings, renders through the
collapsed body `process_block_mono` (`crates/compressor/src/kernel.rs:502-572`) whenever the
chain's mono collapse engages. Mono stems are the product's common case: vocals, bass, kick,
guitars.

The collapsed body runs the same frame law on one plane. It has half the independent work of the
dual body, so the latency problem of slice 3 is worse:

* native: 71.0 cycles per mono frame (`Simd8`), 4 % more per channel than dual;
* V8: 100.5 cycles per mono frame, 50 % more per channel than dual.

This slice applies slices 1-4 to the collapsed body, all together:

* chunked iteration;
* the recursive word in a local;
* `Detector::Main` matched once per block;
* the all-wet arm;
* the two-pass body (targets first, 32-frame chunks);
* the DualMono `abs` arm in pass 1.

Measured on the prototype (EPYC 7313P, pinned, in process):

* mono kernel, one bank-block:
  * native `Simd8`: **-38 %** (2.455 to 1.513 us);
  * native `Simd4`: -41 % (2.434 to 1.432 us);
  * V8: **-51 %** (3.478 to 1.690 us);
* whole `sixty_four_track_console_mono` row (EQ, compressor and limiter, collapsed):
  * native: 71.8 to 64.0-65.8 us per block (-9 to -11 %);
  * V8: 177.7 to 148.2 us (-17 %).

## Invariants

* **Class A**, stated exactly as slices 1-4 state it: the wet arm's NaN-payload rule is the only
  relaxation, and the boundary check makes it unobservable.
* The collapsed contract is unchanged. Today, a collapsed block reads no right plane, computes
  the detector from the left plane twice (`link_frame(main, main)`), and writes only the left plane
  (`crates/compressor/src/lib.rs:556-599`). Under the DualMono arm the detector is `main.abs()`,
  which equals `link_frame(main, main).0` under DualMono. Under Maximum and Average the body keeps
  `link_frame(main, main).0`: the Average arm's `0.5|m| + 0.5|m|` is not `|m|` for subnormal `m`,
  so it must not be simplified.
* Silent and Sidechain blocks, and the ramping prefix, keep `frames_loop_mono`.
* **All four changes land together.** Loop hygiene alone made the collapsed body slower natively
  (console_mono +7 %, kernel +19 % at `Simd8`).
* The stack scratch is `[L; 32]`: 1 KiB at `Simd8`. `#[inline(always)]`, because the roster row
  `compressor6kernel18process_block_mono.*4wide6f32x4` must still match exactly one
  arithmetic-carrying function.

## Interface contract

`crates/compressor/src/kernel.rs` only. `process_block_mono` keeps its signature. Its idle call
becomes `Detector::Main => settled_main_mono(...)`, and every other detector goes to
`frames_loop_mono::<L, false>`. `settled_main_mono`:

* loads `Coef` once;
* sets `DM = link == DualMono` and `WET = !bypass && all_lanes(coef.wet_identity)`;
* runs the two-pass body over the left plane;
* uses the shared `settled_output`;
* writes `gain_reduction_db` back.

## Smallest closable slice

Authorized paths:

* `crates/compressor/src/kernel.rs`, including `settled_body_tests`;
* `crates/compressor/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On this slice's base**, add a mono scenario to gate 2, with its digest empty. It covers
   `Simd4` and `Simd8`, DualMono and Average, all-wet and mixed, frame counts that straddle chunks,
   and hostile input. Record the digest in dev and release, then pin it.
2. Add the body.
3. Gates, then the evidence record.

## Non-goals

The dual body (done in slices 1-4), the ramping prefix (slice 6), and the collapse witness or
seam (`rack`).

## Objective gates

1. **Old body is the oracle.** `settled_main_mono` against `frames_loop_mono::<L, false>`:
   * at `f32`, `Simd4` and `Simd8`;
   * every link mode, both `bypass` values;
   * frame counts `[1, 7, 31, 32, 33, 97, 128]`;
   * hostile input, including subnormals, for the Average case;
   * the three parameter sets fixture, compressing and parallel.

   Compare output and recursive words by bits, except the wet arm's "both NaN" rule. Run in dev
   and release.
2. **Pinned scenario.** Step 1's digest.
3. **Existing gates stay green:**
   * `mono_collapse` (all five), including `the_collapsed_body_renders_the_dual_bodys_left_plane`
     and `a_halved_subnormal_does_not_come_back`;
   * `silent_fixed_point`, `cross_target`, `partition`.
4. **Console digests.** All 15 workloads at both dispatch widths, in particular
   `sixty_four_track_console_mono` and `_mono_dual`, both
   `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7`, and
   `sixty_four_track_console_half_mono`,
   `4a656cdf63882999b720b7dd765c1b4f7bbcb95e2ab38c10445f71d269665180`.
5. **Mutations.** Apply each alone, record it red, and revert it:
   * M1: the Average case uses `abs`. `a_halved_subnormal_does_not_come_back` or gate 1 goes red.
   * M2: the body reads the right plane as the detector. `mono_collapse` goes red.
   * M3: the write-back is dropped. Gate 1 and `mono_collapse` go red.
   * M4: the wet arm ignores `bypass`.
     `mono_collapse::a_statically_bypassed_bank_collapses_to_the_dual_bits` goes red.
6. **Browser artifact, toolchain and policy.** As slice 1's gates 6 and 7.

## Console benchmark rows

* **Can move:** `sixty_four_track_console_mono` (primary) and `sixty_four_track_console_half_mono`.
* `_mono_dual` forces the collapse off, so it runs the dual body and moves only with slices 1-4.
* **Must not move:** rows without a compressor.

## Dependencies

Slices 1-4 (`settled_output`, `all_lanes`, `settled_detect`).

## Standing rules for the implementer

As slice 1.

## What the implementer will hit

* **Do not land the hygiene half first.** That half, measured alone, is a native regression here.
* **`link_frame(main, main)` is deliberate.** The collapsed body's Average arithmetic must match
  the dual body's left plane bit for bit. Only the DualMono arm may shorten it.

