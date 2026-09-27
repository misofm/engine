# Compressor: run the settled dual body as a chunked loop with its recursive words in locals

Compressor slice 1 of 5 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `6ca203f8`;
every `file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/COMPRESSOR-DIAGNOSIS.md`. The prototype this slice formalises is
mode 1 of `docs/handoffs/effects-2026-09-27/compressor-diagnosis-prototypes.patch`. That patch
applies on top of `compressor-diagnosis-harness.patch`. Neither patch may be committed.

## Product outcome

In the browser (wasm `simd128`, Node/V8 12.4, TurboFan), the compressor's settled body carries,
for every frame:

* four slice bounds checks;
* a `match` on the detector kind;
* the store and reload of each channel's recursive word through `Channel`.

The frame loop is `frames_loop::<L, false>` (`crates/compressor/src/kernel.rs:452-497`). V8
compiles it to 321 machine instructions per frame. 72 of them are scalar bookkeeping and branches:

* spilled loop indices;
* the bounds-check pairs;
* the detector test;
* the stack-guard check;
* the recursive words' loads and stores in linear memory.

This slice rewrites the settled body of `Detector::Main` blocks as a chunked iteration, with both
recursive words held in locals and the detector matched once per block. The frame law, and the
order in which it runs over frames and channels, do not change.

Measured on the prototype, in process, under the timing lock, pinned to cpu 31 (AMD EPYC 7313P,
Zen 3):

* compressor-only minus builtins-only, 64 tracks, wasm under V8: **-12 %** (82.9 to about 72.6 us
  per block);
* kernel under V8: -11 % (67.9 to 60.3 cycles per channel-frame);
* native `Simd8`: flat (39.3 to 40.2 us, within run-to-run noise);
* the unbanked scalar path (every sidechain-connected compressor, and the native `Simd4` dispatch
  row): -22 % (315.6 to 245.4 us for 64 tracks).

These figures are descriptive only. The paired console benchmark is run by someone else, at the
batch boundary.

## Invariants

* **Class A.** Every rendered word, every recursive word, every report and every state payload is
  unchanged at `f32`, `Simd4` and `Simd8`, on every target. Each frame runs `link_frame` and then
  `one_frame` for the left channel, then for the right, in today's order, on today's values.
* Only the idle (settled) body of `process_block` changes, and only for `Detector::Main`.
  `Detector::Silent` and `Detector::Sidechain` blocks keep calling `frames_loop::<L, false>`
  unchanged. The ramping prefix (`frames_loop::<L, true>`) is untouched, and so is the collapsed
  body (`process_block_mono`, `:502-572`; slice 5 owns it).
* The recursive words are written back to `channel_left.gain_reduction_db` and
  `channel_right.gain_reduction_db` after the loop, on every exit path. `MUTATIONS.md` row 7 is the
  existing gate that goes red without the write-back.
* Render stays allocation-free, lock-free and syscall-free. There is no `unsafe`, and no crate
  other than `lane` names `wide`.
* `process_block::<Simd4>` stays the only arithmetic-carrying function that matches the
  callgraph roster's `compressor6kernel13process_block.*4wide6f32x4`
  (`scripts/check-web-audioworklet-callgraph.py:189`). The new body is `#[inline(always)]`. The
  prototype's `#[inline(never)]` was for disassembly only.

## Interface contract

`crates/compressor/src/kernel.rs` only. `process_block` (`:403`) keeps its signature. Its idle
call (`:434-447`) becomes:

```rust
if ramping < frames {
    match detector {
        Detector::Main => settled_main::<L>(
            left, right, ramping, frames, link, bypass, channel_left, channel_right,
        ),
        _ => frames_loop::<L, false>(/* today's arguments */),
    }
}
```

New private `#[inline(always)] fn settled_main<L: Lane>(left, right, start, end, link, bypass,
channel_left, channel_right)`:

1. `Invariants::new(link, bypass)`, then `Coef::load` for each channel, once.
2. `let left = &mut left[start * W..end * W]`, and the same for `right`. Then
   `let (mut gl, mut gr) = (channel_left.gain_reduction_db, channel_right.gain_reduction_db)`.
3. `for (l, r) in left.chunks_exact_mut(W).zip(right.chunks_exact_mut(W))`: load `ml` and `mr`,
   run `link_frame(Detector::Main, 0, ml, mr, &inv)`, then `one_frame(ml, dl, &coef_left, &mut gl,
   &inv).store(l)`, then the same for the right channel into `r`.
4. `channel_left.gain_reduction_db = gl; channel_right.gain_reduction_db = gr`.

No public API changes. `link_frame`, `one_frame`, `curve_target`, `ballistic` and `gain_mix` are
not edited.

## Smallest closable slice

Authorized paths:

* `crates/compressor/src/kernel.rs`, which includes a new `#[cfg(test)] mod settled_body_tests`
  for gates 1 and 2;
* `crates/compressor/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On the unmodified base**, write gate 2's scenario test with its expected digest empty. Run it
   in dev and in release, record the printed digest in this spec, and then pin it.
2. Add `settled_main` and route the `Detector::Main` idle call through it.
3. Gates, then the evidence record.

## Non-goals

* The all-wet arm, the two-pass body and the DualMono arm (slices 2-4). The collapsed body
  (slice 5). The ramping prefix (slice 6).
* Any change to `frames_loop`, which stays the oracle and the non-`Main` body.
* A floor recount.

## Objective gates

1. **Old body is the oracle (kernel-level).** In `settled_body_tests`, run `settled_main` and
   `frames_loop::<L, false>` on two identically prepared `Channel` pairs, at `L` in `f32`,
   `Simd4` and `Simd8`:
   * for every `LinkMode`;
   * with `bypass` both false and true;
   * with the corpus track table (`crates/compressor/src/corpus.rs:71`) as per-lane coefficients;
   * over 48 blocks with frame counts `[1, 7, 31, 32, 33, 128]`.

   The input is hostile: `-0.0` and `+0.0` on every lane, subnormals, `1e29` and `-3e30`, both
   infinities, NaN payloads `0x7fc01234` and `0xffa00001`, and noise at 0.2 and 1.5. Assert every
   output word and both recursive words are equal. Compare NaN words by bits here: this slice
   commutes nothing, so a payload difference is a failure. Apply `finish_channel` to both sides
   after each block. Run in dev **and** release (`cargo test -p compressor` and
   `cargo test --release -p compressor`).
2. **Scenario pinned on base.** A fixed 24-block, heterogeneous render at `Simd4` and `Simd8`
   through `process_block`, DualMono and Maximum, with the hostile input above. Fold every output
   word and recursive word into one SHA-256, pinned to the digest recorded in step 1.
3. **Existing gates stay green:**
   * `cross_target` (the pinned `C1_DIGESTS`, every width);
   * `lane_identity`, `partition` (including `linked_sidechain_partitions_are_invariant`),
     `identity`, `nonfinite`, `causality`, `silent_fixed_point`, `mono_collapse`, `oracle`,
     `ramps`.
4. **Console digests.** All 15 standing workloads' 64-block digests are unchanged at both dispatch
   widths. Use the `digests` harness in `compressor-diagnosis-harness.patch`, in a scratch copy;
   do not commit it. The base digests, which are identical at `Simd8` and `Simd4` dispatch:
   * `sixty_four_track_compressor_only` `95c9375429fbca3449bf9c5134508a6220f17fc9adda0b3267c061b75bb14175`
   * `sixty_four_track_console` `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de`
   * `sixty_four_track_eq_comp_simd1` `f68febb7a10e242be704a7646e17b66213a52e6b833633fb13a71d7a3f89a177`
   * `sixty_four_track_console_mono` `fc96d91f6a397e916bb02651300163782170e5caee3d23189ff640b5c545b3d7`
   * `one_twenty_eight_track_stretch` `cba2c94f81544caad0945f0720480b568b1a47808d25fd95911f61bd37f5f9b1`
   * `nine_track_ragged_strip` `17613a3ab693d3f0dfc457b41f77669f880581fa19c433941a1ef2437684198a`
   * every other row, as printed by the harness on base.
5. **Mutations.** Apply each alone, record it red in `MUTATIONS.md`, and revert it:
   * M1: the recursive words are never written back. `cross_target` and gate 1 go red.
   * M2: `start` is ignored, so the slice begins at frame 0. `partition`'s automated partitions go
     red.
   * M3: every detector takes `settled_main`, so `Silent` and `Sidechain` read the main input.
     `causality` or `partition::linked_sidechain_partitions_are_invariant` goes red. If neither
     does, add a gate-1 case with `Detector::Sidechain` and a quiet main, and record which test
     went red.
   * M4: right is processed before left, which swaps the state words. Gate 1 goes red.
6. **Browser artifact.**
   * Run `bash scripts/check-web-audioworklet.sh`. Its `--kernel-shape --kernel-pattern
     '4wide6f32x[48]' --kernel-min 11` run must pass, with the compressor rows still matching
     exactly one function.
   * Run `--callgraph miso_engine_web_v1_render`: no allocator or trap may be added.
   * Record the artifact's size delta. The artifact pin and browser qualification are repinned
     once, at the batch boundary.
7. **Toolchain and policy.**
   * `cargo fmt --all --check`;
   * `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   * `bash scripts/check-realtime-policy.sh`, `check-lane-policy.sh`, `check-unfused-seal.sh`. The
     fast-dB seal is enforced by clippy (`clippy.toml:81-82`).
8. **Codegen evidence** (recorded, not asserted). The release `bench` binary's settled loop in
   `compressor::kernel::process_block::<f32x8>` has no per-frame `cmp`/`ja` bounds checks and no
   detector test, and does not store `gain_reduction_db` inside the loop. Quote the loop and its
   instruction count. Also quote V8's loop (`node --print-wasm-code`) and its scalar and branch
   counts, which were 56 and 16 per frame on base.

## Console benchmark rows

* **Can move:** every row with a compressor. Primary: `sixty_four_track_compressor_only` (wasm
  arm). Also `sixty_four_track_console`, `eq_comp_simd1`, `console_legacy`, the 128-track stretch,
  `nine_track_ragged_strip`, and the mono and half-mono rows (their dual banks only).
* **Must not move:** `eq_only`, `builtins_only`, `dispatch_only`, `gain_pan_only`,
  `nine_track_baseline`.

## Dependencies

None. Slices 2-4 build on this body.

## Standing rules for the implementer

* Work only from this body. Read the cited functions first; do not survey the workspace.
* Class A means no rendered bit moves. Every bit-identical gate is a hard stop, never a tolerance.
* Keep render paths allocation-free, lock-free and syscall-free.
* Run fmt, clippy and the focused tests before every checkpoint. Commit on a `codex/<issue>-<slug>`
  branch, and touch no path outside the list.
* Do not quote a projected saving. `scripts/run-console-benchmark.sh` is not run by the
  implementer.

## What the implementer will hit

* **Native shows nothing, and that is expected.** On x86 the three removed costs are cheap. The
  saving belongs to V8, whose loop drops from 72 scalar instructions and branches per frame to 23,
  and to the scalar path. Do not chase a native number here.
* **Do not apply this to the collapsed body.** The same hygiene alone made
  `process_block_mono` slower natively (+7 % at console level, +19 % at kernel level at `Simd8`).
  Slice 5 applies all four changes to the mono body together.
* **`frames_loop::<L, false>` is still needed.** It runs `Silent` and `Sidechain` blocks, and it
  is gate 1's oracle. Do not delete it or "unify" it with the new body.
* **Keep `#[inline(always)]`.** An out-of-line `settled_main::<f32x4>` would carry the hot
  arithmetic, and the callgraph roster (rule 3) would then be checking only the ramping prefix.

## Research findings

See `COMPRESSOR-DIAGNOSIS.md`, sections 1-3. On the per-frame costs this slice removes:

* **x86 `Simd8`.** The base loop is 241 instructions per frame. Of those, 21 are scalar:
  * 4 bounds checks;
  * 2 detector-match branches;
  * loop counters;
  * a spilled pointer reload.

  In addition, the two recursive words round-trip through memory every frame
  (`vmovaps 0x660(%rbx)` load and store).
* **V8.** Today's settled loop is 321 instructions per frame: 56 scalar (spilled indices,
  address arithmetic) and 16 branches. The chunked body's loop has 21 scalar instructions and 2
  branches.
  Neither loop materialises splat constants in place; the ramping loop does (22 per frame), and
  slice 6 owns it.
