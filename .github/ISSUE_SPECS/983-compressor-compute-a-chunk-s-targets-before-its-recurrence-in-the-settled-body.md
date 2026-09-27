# Compressor: compute a chunk's targets before its recurrence in the settled body


Compressor slice 3 of 5 (research 2026-09-27, base `6ca203f8`; builds on slices 1 and 2).
Evidence: `docs/handoffs/effects-2026-09-27/COMPRESSOR-DIAGNOSIS.md`, section 3. The prototype is
mode 13 of `compressor-diagnosis-prototypes.patch` (`two_pass_body`), which applies on top of
`compressor-diagnosis-harness.patch`. Neither patch may be committed.

## Product outcome

The compressor's frame law is one dependent chain per channel. It runs:

1. detector;
2. `frexp` and a 10-step Horner (`fast_level_db`);
3. the curve;
4. the one-pole recurrence;
5. an 8-step Horner (`fast_exp2`);
6. the output.

That is about 140 cycles from load to store. The settled loop issues each channel's whole chain
in one body, so Zen 3 keeps only about two chains in flight. `Simd4` and `Simd8` then cost the
same per frame (65.7 against 68.4 cycles per channel-frame), and the loop retires 1.1 vector
operations per cycle, where a mixed stream on this core retires 4.

Only steps 4-6 depend on the previous frame. Steps 1-3, the target, are independent across
frames. This slice renders the settled body in chunks of 32 frames:

* **pass 1** computes the chunk's targets for both channels into a stack array;
* **pass 2** runs the recurrence and the output frame by frame.

The same operations run on the same values; only their order across frames changes.

Measured on the prototype (EPYC 7313P, pinned, in process), on top of slices 1 and 2:

* compressor-only minus builtins-only, 64 tracks:
  * native `Simd8`: **-26 %** (34.1 to 25.3 us per block);
  * wasm under V8: -4 % (64.2 to 61.6 us);
* kernel: 59.6 to 42.6 (`Simd8`), 58.0 to 41.9 (`Simd4`) and 52.4 to 50.3 (V8) cycles per
  channel-frame;
* the unbanked scalar path, 64 per-node compressors: 201.5 to 89.6 us.

These figures are descriptive only.

## Invariants

* **Class A.** Every output word, recursive word, report and payload is unchanged, NaN payloads
  included (slice 2's NaN relaxation still applies to its arm and to nothing else). Frame `k`'s
  target is computed by today's operations from today's inputs. The recurrence visits frames in
  today's order, left before right. The output law is slice 2's, unchanged.
* The chunk is 32 frames (`const CHUNK: usize = 32`); the last chunk may be shorter. Every
  `frames` count from 1 to the quantum is legal, including 1, 31, 33 and 127.
* Scratch lives on the stack: `[(L, L); CHUNK]`, which is 2 KiB at `Simd8`, 1 KiB at `Simd4` and
  256 B at `f32`. There is no heap. It holds intermediate targets, not audio, so it is not a block
  copy under the owner's copy rule. This spec records that justification.
* Only the settled body of `Detector::Main` blocks changes. The ramping prefix, `Silent`,
  `Sidechain` and the collapsed body are untouched.
* `#[inline(always)]`: `process_block::<Simd4>` stays the one roster function.

## Interface contract

`crates/compressor/src/kernel.rs` only. `settled_main` (slices 1-2) runs this body for both `WET`
arms:

```rust
let mut targets = [(L::zero(), L::zero()); CHUNK];
for (lc, rc) in left.chunks_mut(CHUNK * W).zip(right.chunks_mut(CHUNK * W)) {
    // Pass 1: no frame depends on another.
    for ((l, r), t) in lc.chunks_exact(W).zip(rc.chunks_exact(W)).zip(targets.iter_mut()) {
        let (dl, dr) = link_frame(Detector::Main, 0, L::load(l), L::load(r), inv);
        *t = (curve_target(dl, coef_left, inv), curve_target(dr, coef_right, inv));
    }
    // Pass 2: the recurrence, then the output, frame by frame.
    for ((l, r), t) in lc.chunks_exact_mut(W).zip(rc.chunks_exact_mut(W)).zip(targets.iter()) {
        let (ml, mr) = (L::load(l), L::load(r));
        let sl = ballistic(t.0, &mut gl, coef_left);
        let sr = ballistic(t.1, &mut gr, coef_right);
        settled_output::<L, WET>(ml, sl, coef_left, inv).store(l);
        settled_output::<L, WET>(mr, sr, coef_right, inv).store(r);
    }
}
```

`curve_target`, `ballistic`, `link_frame` and the output law are called unchanged. The prototype
calls both ballistics before both outputs, and that interleaving is class A, since the channels
are independent. Gate 1 proves it.

## Smallest closable slice

Authorized paths:

* `crates/compressor/src/kernel.rs`, including `settled_body_tests`;
* `crates/compressor/tests/MUTATIONS.md`;
* this spec.

Steps:

1. **On this slice's base**, extend gate 2 with frame counts that straddle chunks: blocks of 31,
   32, 33, 64, 97 and 128 frames, including a ramp that ends at frame 40, so that the settled body
   starts mid-chunk. Record the digest in dev and release, then pin it.
2. Replace the settled loop with the two-pass body.
3. Gates, then the evidence record.

## Non-goals

* The DualMono arm (slice 4), the collapsed body (slice 5) and the ramping prefix (slice 6).
* Other chunk sizes. 16, 64 and 128 measured the same as 32 to within 3 %; keep 32 for the
  smallest stack.
* Op-level interleaving of frames (two or four frames per iteration). Natively it is within 5 % of
  this body, and under V8 it is unstable: ±30 % between builds.
* Moving the targets into a heap scratch, or into the effect's declared scratch bytes.

## Objective gates

1. **Old body is the oracle.** Slice 1's gate 1, unchanged: `settled_main` against
   `frames_loop::<L, false>`, at `f32`, `Simd4` and `Simd8`, every link mode, both `bypass`
   values, frame counts `[1, 7, 31, 32, 33, 97, 128]`, and the hostile input. Compare outputs and
   recursive words by bits, except the wet arm's "both NaN" rule. Run in dev and release.
2. **Scenario pinned on base.** Slice 1's and slice 2's pinned digests are unchanged, plus the
   chunk-straddling digest recorded in step 1.
3. **Existing gates stay green:** `partition` (blocks of 1, 7, 64, 128 and 512, with automation,
   which is what cuts the ramp mid-block), `cross_target`, `lane_identity`, `identity`,
   `nonfinite`, `causality`, `silent_fixed_point`, `oracle`, `ramps`.
4. **Console digests.** All 15 standing workloads, both dispatch widths: slice 1's gate 4.
5. **Mutations.** Apply each alone, record it red, and revert it:
   * M1: pass 2 reads `targets[k + 1]`, off by one. Gate 1 and `cross_target` go red.
   * M2: the last, short chunk is skipped (`chunks_exact` in place of `chunks` on the outer loop).
     Gate 1 (33 frames) and `partition` go red.
   * M3: pass 1 runs only for the first chunk, and later chunks reuse its targets. Gates 1 and 2
     go red.
   * M4: `gl` and `gr` are swapped in pass 2. Gate 1 goes red.
6. **Allocation.** `cargo test -p compressor` still passes its conformance `process.allocation`
   gate (`tests/conformance.rs`).
7. **Browser artifact, toolchain and policy.** As slice 1's gates 6 and 7. Record the
   AudioWorklet artifact's size delta, and run `node scripts/check-web-boot-budget.mjs` or its
   `check-web-audioworklet.sh` stage.
8. **Codegen evidence** (recorded). The release `bench` binary shows two inner loops in the
   settled body. The prototype had 99 and 70 instructions per frame (x86 `Simd8`). V8's had 123
   and 94. Quote both.

## Console benchmark rows

As slice 1. The native rows move the most.

## Dependencies

Slices 1 and 2. On V8, this body without the wet arm was 5 % slower than slice 1 alone (mode 9,
76.3 us, against mode 1, 72.6 us). Land it after slice 2.

## Standing rules for the implementer

As slice 1.

## What the implementer will hit

* **It looks like a copy.** It is not one: the array holds `curve_target` values, which the
  one-pass body keeps in registers or spill slots. Cite this spec's invariant in the PR.
* **Do not interleave the frames by hand.** Pairing two frames' chains op by op (a `Pair<L>` lane
  type) spills sixteen registers. It measured 33.5 and then 49.6 cycles per channel-frame under V8
  in two builds of the same source.
* **The ramp tail starts mid-chunk.** `settled_main` receives `start = ramping`, so its first
  chunk may begin at any frame. The chunks are of the settled slice, not of the block.

## Research findings

COMPRESSOR-DIAGNOSIS.md, sections 1-3:

* **Stage truncations.** Appending the gain conversion to the frame costs 23 cycles for 19 lane-ops
  per channel-frame, because its chain waits on the recurrence.
* **The prototype's passes alone (mode 15, which includes slice 4).** Pass 1 takes 24.2 cycles per channel-frame and pass 2 takes
  17.1; run together they take 38.3.
* **The Zen 3 pipe floor for this op set.** It is 16.5 cycles per channel-frame, set by the
  add/max/compare pipes, so the body is still latency-bound after this slice.

