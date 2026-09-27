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


## Attempt 1 evidence

Implementer attempt 1, 2026-09-27, on top of #982 (`3acd0e88`), branch
`codex/981-compressor-settled-body`. Per the verification, #984 is merged into this issue and
lands in the same commit; its own evidence is in its spec.

### The change

`crates/compressor/src/kernel.rs` only. `settled_frames::<L, DUAL_MONO, WET>` renders the settled
slice in chunks of `SETTLED_CHUNK = 32` frames (`chunks_mut`, so the last chunk may be short):
pass 1 computes each frame's `(curve_target(left), curve_target(right))` into a stack array
`[(L, L); 32]`; pass 2 reloads each frame's input, steps both recurrences with `ballistic` and
stores `settled_output::<L, WET>` for both channels. `curve_target`, `ballistic`, `link_frame` and
the output law are called unchanged; the recurrence visits the frames in today's order. The
chunks are of the settled slice, which starts at `start = ramping`: a chunk need not be aligned
to the block, and nothing depends on alignment (loads are unaligned; the misaligned head is just
the first chunk). `#[inline(always)]` throughout; `settled_main` now instantiates four bodies per
width (`DUAL_MONO` x `WET`).

**Why it is exact.** A frame's target depends only on that frame's input and on coefficients
constant over the settled slice (`ramping = min(max_remaining, frames)` guarantees every ramp has
finished), and the compressor is feed-forward: no recursive word feeds the curve. Pass 1 reads a
chunk's inputs before pass 2 overwrites any of them, and pass 2 reloads each input before storing
its output. The two channels' recurrences share nothing, so stepping both before either output is
exact. NaN payloads included: no NaN relaxation beyond #982's arm.

**Copy-rule justification** (the owner's rule on block copies). `targets` holds `curve_target`
values, which the one-pass body kept in registers or spill slots; it is not audio and not a block
copy. No in-place form exists: pass 2 needs both a frame's input and its target, and writing the
targets into the plane would destroy the input. It is stack scratch, 2 KiB at `Simd8` (1 KiB at
`Simd4`, 256 B at `f32`), zero-filled once per settled call; no heap. The code comment on
`settled_frames` states the same.

### Gates

* **Gate 1**: #981's grid with frame counts `[1, 7, 31, 32, 33, 97, 128]` and settled starts
  `{0, 1, 18, 40}` (so `start` is non-zero and not chunk-aligned), every link mode, both `bypass`,
  every detector, on the corpus table, the all-wet tables and the fixture tracks; plus the three
  randomized differentials (starts anywhere in `1..=127`). By bits, except #982's "both NaN" rule
  on its arm. Dev and release.
* **Gate 2**: `scenario_981` `57cfd7ce...` and `scenario_982` `cd2d5b11...` unchanged;
  `scenario_983_chunk_straddling_render_is_pinned` (blocks of 31, 32, 33, 64, 97 and 128 frames,
  six 24-frame fully ramping blocks whose ramps end at frame 40 of the next block, both tables,
  every link mode, `Simd4` and `Simd8`; the test asserts all 108 mid-chunk starts happened)
  recorded on B0 and on this slice's base (#982):
  `47ffff05a0f1b605a42acd320948d09b7137d92e2f3b7925381b6c6dc0aed424`, dev and release; unchanged
  after the change.
* **Gate 3**: `partition`, `cross_target`, `lane_identity`, `identity`, `nonfinite`, `causality`,
  `silent_fixed_point`, `oracle`, `ramps` green; `cargo test --locked -p compressor` 91 passed in
  dev and release.
* **Gate 4**: all 30 console digests identical to B0.
* **Gate 6**: `conformance`'s `process.allocation` gate passes (part of the crate run).
* **Gate 7**: browser artifact rule 3, roster (compressor dual one function: vector 516, scalar 0),
  `meter_poll`, `command_submit`, and `check-web-boot-budget.mjs` PASS; artifact 3,360,990 bytes
  (+9,884 over #982, +15,375 over B0).

| gate | command | result |
|---|---|---|
| toolchain | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| compressor | `cargo test --locked -p compressor`, and `--release` | 91 passed, 0 failed, each |
| other crates | `-p effect-runtime` (86), `-p console-workload` (39), `-p builtins-compiler --features test-support` (79) | all passed |
| wasm | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (113 passed); `bash scripts/run-wasm-gates.sh` | PASS, 0 mismatches on the native, wasm and wasm `simd128` legs |
| policy | `check-lane-policy.sh`, `check-realtime-policy.sh`, `check-unfused-seal.sh` | PASS |

### Mutations (gate 5)

`MUTATIONS.md`, section "#983 and #984": M1 (`targets[k + 1]`) 20 red, M2 (short last chunk
skipped) 13 red including gate 1's 31/33/97-frame blocks and both `partition` tests, M3 (pass 1
only for the first chunk) 16 red including gates 1 and 2, M4 (`gl`/`gr` swapped in pass 2) 15 red.

### Codegen (gate 8, recorded)

x86 `Simd8` release (harness binary, `Instance<f32x8>::render`): four settled instantiations, each
two inner loops, per frame:

| instantiation | pass 1 | pass 2 |
|---|---:|---:|
| DualMono, wet (the standing fixture) | 100 | 69 |
| linked, wet | 108 | 68 |
| DualMono, general law | 98 | 102 |
| linked, general law | 109 | 103 |

Each loop carries 3 scalar instructions (pointer add, counter, branch). The prototype had 99 and
70. B0's one-pass settled loop was 239.

V8 (Node 22.23.2, `--no-liftoff`, function 383 `process_block<f32x4>`): DualMono-wet 122 and 94,
linked-wet 134 and 94, DualMono-general 121 and 132, linked-general 133 and 133 instructions per
frame, each with 2 branches and 9-10 scalar. The prototype had 123 and 94. Recorded, not chased
(verification note 5): V8 now compiles the retained `Silent`/`Sidechain` `frames_loop::<Simd4,
false>` with heavy spilling (407 and 910 instructions in its two loops, against 325 on B0); banks
always detect `Main`, so no bank reaches it.

### A/B

Measured once as the set #981-#985 against a separately built B0; see #985's evidence.
