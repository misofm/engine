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


## Attempt 1 evidence

Implementer attempt 1, 2026-09-27, branch `codex/981-compressor-settled-body` from the local
optimisation batch at `197db1c9` (the unmodified base, "B0" below). The verification amendments
posted on the issue (`VERIFY-COMPRESSOR.md`) override this body where they conflict and were
followed: M4 is "the right channel updates `gl`"; gate 1 drives a non-zero `start`.

### The change

`crates/compressor/src/kernel.rs` only. `process_block`'s idle call now matches the detector once:
`Detector::Main` goes to the new `#[inline(always)] settled_main`, and `Silent`/`Sidechain` keep
`frames_loop::<L, false>` unchanged. `settled_main` loads `Invariants` and both `Coef`s once,
slices the planes to `start..end`, iterates `chunks_exact_mut(W)` of both planes, keeps both
recursive words in locals, runs `link_frame(Detector::Main, ..)` then `one_frame` left then right
per frame, and writes both words back after the loop. `frames_loop`, `link_frame`, `one_frame`,
`curve_target`, `ballistic` and `gain_mix` are not edited. All three clamps are untouched.

### The gate module (`kernel::settled_body_tests`)

* `reference`: `process_block`, `process_block_mono` and both `frames_loop` bodies exactly as they
  stood at B0, kept in the test code as the fixed oracle. Every comparison is production against
  it, by bits: both output planes, the recursive words, every coefficient word, every ramp field;
  then the `finish_channel` masks, the finished planes and the state again.
* **Gate 1, deterministic grid** (`the_settled_body_is_the_base_body_on_the_corpus_table`): the
  corpus track table as per-lane coefficients (right channel rotated by three lanes), `f32` (eight
  one-lane groups), `Simd4` (two groups) and `Simd8`, every `LinkMode`, `bypass` false and true,
  and every detector (`Main`, `Silent`, `Sidechain` under a main scaled by 1e-3), dual and
  collapsed. The schedule is 42 blocks: each count of `[1, 7, 31, 32, 33, 128]` is preceded in turn
  by nothing and by a fully ramping block of `64 - p` frames, so the settled body starts at frame
  `p` in `{0, 1, 18, 40}`. Input cycles through ten profiles covering every hostile word of the
  brief (`±0` on every lane, subnormals, `1e29`, `-3e30`, `±inf`, `±MAX`, NaN payloads
  `0x7fc01234`, `0xffa00001`, `0x7f800001`, noise at 0.2, 0.7 and 1.5, exact threshold levels,
  levels at the detector floor). NaN words compare by bits. (Deviation: 42 blocks per configuration
  rather than 48; the schedule is the exhaustive cross of the counts and the four starts.)
* **Randomized differential** (`randomized_differential_{f32,simd4,simd8}`), the verification's
  differential committed: per seed, random per-lane parameters at the domain edges (subnormal
  knees included), 60 % all-wet seeds, a random link mode and sample rate, automation on random
  lanes in 25 % of blocks, discontinuity and full resets, bypass toggles, 10 % `Silent` and 10 %
  `Sidechain` blocks, and frame counts from `{1, 7, 31, 32, 33, 63, 64, 65, 97, 127, 128}` or
  `1..=128`; dual and collapsed. 10 seeds x 128 blocks in dev, 320 x 128 in release, per width.
  Coverage is asserted non-vacuous (settled, mid-block starts, all-wet settled, rejected and
  sidechain blocks).
* **Gate 2** (`scenario_981_heterogeneous_hostile_render_is_pinned`): 24 blocks, `Simd4` and
  `Simd8`, DualMono and Maximum, the corpus table, hostile input, one automation point that leaves
  a 23-frame ramp prefix. SHA-256 over every kernel output word, the recursive words, the masks,
  the finished words and the recursive words again. Recorded on B0 before the change:
  `57cfd7ce05050c68ab73e6585ccc72bd5403543565cd38471fce47bb263e62a0` in dev and in release
  (identical), then pinned; unchanged after it.

### Gates

| gate | command | result |
|---|---|---|
| 1, 2, 3 | `cargo test --locked -p compressor` / `--release` | 87 passed, 0 failed, each |
| 4 console digests | throwaway harness (the brief's `digests`, without the mode switch), B0 and #981 built separately | all 30 rows (15 workloads x `Simd8`/`Simd4` dispatch) identical to B0 and to the brief's six quoted values |
| 6 browser | the `build-web-audioworklet.sh` cargo line, then `check-web-audioworklet-callgraph.py --callgraph miso_engine_web_v1_render`, `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11`, the `meter_poll` and `command_submit` closures, `check-web-boot-budget.mjs` | all PASS; compressor dual row matches exactly one function (vector 178 -> 267, scalar 0); artifact 3,345,615 -> 3,348,729 bytes (+3,114). Not repinned (the in-tree pin `8934cdd9...` already differs from B0's own build, `574f6ce9...`; the batch repins once) |
| 7 | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| 7 | `check-realtime-policy.sh`, `check-lane-policy.sh`, `check-unfused-seal.sh` | PASS |
| other crates | `-p effect-runtime` (86), `-p console-workload` (39), `-p builtins-compiler --features test-support` (79) | all passed |
| wasm | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (113 passed); `bash scripts/run-wasm-gates.sh` | PASS: native, wasm scalar and wasm `simd128` legs, 142 cases, 358 comparisons, 0 mismatches (the compressor corpus is among them) |

### Mutations (gate 5)

Recorded in `crates/compressor/tests/MUTATIONS.md`, section "#981". All four are red: M1 (no
write-back) 27 red tests, M2 (`start` ignored) 12, M3 (every detector takes `settled_main`) 6, M4
(the right channel updates `gl`) 17. M3 is caught by gate 1's `Sidechain`/`Silent` cases and by
`contract::links_are_exact_and_connected_sidechain_is_distinct_from_main_detection` and
`nonfinite::a_nan_in_the_sidechain_alone_is_clamped_to_the_level_floor`; `causality` and
`partition::linked_sidechain_partitions_are_invariant` stayed green, which is why the grid
carries the sidechain cases.

### Codegen evidence (gate 8, recorded)

x86 (`Simd8`, release, the harness binary's
`Instance<f32x8>::render` into which `process_block::<f32x8>` is inlined; `x86_loops.py`):

* B0 settled loop: 239 instructions per frame, 19 scalar: the four bounds-check pairs
  (`cmp %r15,%rdi; ja`, `cmp $0x7,%r9; jbe`, `cmp %r13,%rdi; ja`, `cmp $0x7,%r8; jbe`), the
  detector test (`test %rcx,%rcx; je`), two spilled-pointer reloads (`mov 0x40(%rsp),%r11`) and
  the counters; the recursive words are stored every frame (`vmovaps %ymm6,0x660(%r14)`,
  `vmovaps %ymm2,0xce0(%r14)`).
* #981 settled loop: 226 instructions per frame, 3 scalar (`add $0x20,%r8; dec %r15; jne`), no
  bounds check, no detector test, and no store other than the two output planes.

V8 (Node 22.23.2, `--no-liftoff --print-wasm-code`, the wasm console guest, function 383
`process_block<f32x4>`, `v8_loops.py`): B0's settled loop is 325 instructions per frame with 60
scalar (30 register, 30 memory) and 16 branches; #981's is 254 with 8 scalar (3 register, 5
memory) and 2 branches. The retained `frames_loop::<Simd4, false>` bodies for `Silent` and
`Sidechain` follow it unchanged in shape (309 and 432 instructions).

### A/B

Measured once, as the set #981-#985, against a separately built B0 (verification amendment 4);
see #985's evidence. The brief's descriptive figures are not re-quoted here.

## Sol attempt 1 verdict: PASS

Adversarial verification, 2026-09-27, of `aa3c0d27` on branch `codex/981-compressor-settled-body`
(head `3514a4e4`). The oracle is a separately built base: `git archive 197db1c9` in scratch, whose
kernel was exposed through a scratch-only hook and linked beside each commit's own kernel. The
harness and logs are scratch only and were not committed.

**Exactness (independent of `kernel::settled_body_tests`).** A seeded differential ran each
block on identically prepared channels in the base kernel and in `aa3c0d27`'s. Per seed it drew:

* a link mode;
* a sample rate from 44.1 to 192 kHz;
* per-lane parameters at the domain edges: subnormal threshold, knees `2.8e-45`, `1e-40` and
  `MIN_POSITIVE`, ratio `1.0000001`, mix `0.99999994` and `1e-45`, raw `-0.0` makeup;
* optionally all-wet or makeup-zero tables;
* optionally a partial bank, with dead lanes held at zero.

Per block it drew:

* automation on random lanes and channels, and resets;
* a bypass toggle, and `Main`, `Silent` or `Sidechain` with hostile sidechain planes;
* 1 to 399 frames;
* dual, collapsed, or collapse transitions (`copy_state_from` at disengage);
* thirteen input profiles: ±0, subnormals, qNaN and sNaN payloads, ±inf, ±MAX, `1e29` to `-3e30`
  around `BLOCK_LIMIT`, exact threshold levels, levels at the detector floor, sNaN over silence.

It compared by bits every output word and every state word: all 64 coefficient words, the
four fields of all 72 ramps (seven parameter and two rate ramps per lane), and the recursive
words. It then applied `finish_channel` on both sides and
compared the masks, every word and the state again. For this commit no NaN relaxation is admitted.
Results, 0 failures everywhere:

* native release: 3,000 seeds x 160 blocks per width at `f32`, `Simd4` and `Simd8` (480,000 blocks
  each; 324,000 settled `Main`, 66,000 mid-block starts, 26,000 not 32-aligned, 167,000
  collapsed, 10,500 transitions);
* native dev: 96 seeds x 128 blocks;
* wasm under Node 22.23.2: `simd128` under TurboFan (`--no-liftoff`, 200 seeds) and under default
  tiering (48 seeds), and scalar wasm (64 seeds), each x 128 blocks;
* an effect-boundary differential through the public factory, scalar with and without a
  connected sidechain, and banks dual, collapsed and transitioning: 1,200 seeds.

The harness is live. With 981-M1, M2, M3 and M4 applied to a scratch copy, it fails at every width,
and M3 also fails through the public factory.

**In-tree oracle.** `settled_body_tests::reference` diffed against `197db1c9`'s `process_block`,
`frames_loop`, `process_block_mono` and `frames_loop_mono`. It is verbatim except for the dropped
`debug_assert`s and `#[inline(always)]`. `scenario_981` (`57cfd7ce...`) reproduces on the true base
kernel (base production code carrying head's test module), in dev and release.

**Gates.** At this commit: fmt, compressor clippy `-D warnings`, and `cargo test -p compressor`
(87 passed in dev and in release). At the head, all green:

* fmt, workspace clippy `-D warnings`, rustdoc `-D warnings`;
* `-p compressor`: 93 passed in dev and 93 in release;
* `-p effect-runtime` 86, `-p console-workload` 39, `-p builtins-compiler --features test-support` 79;
* lane, math and wasm-gates in release: 113;
* `run-wasm-gates.sh`: 0 mismatches;
* the lane, realtime and unfused policy scripts;
* the AudioWorklet stages: rule 3, the roster (compressor dual row is one function), `meter_poll`,
  `command_submit` and the boot budget;
* all 30 native console digests identical to base, and to the brief's quoted values.

**981-M3 is enough.** Two public-API tests catch it (`contract::links_are_exact_...`,
`nonfinite::a_nan_in_the_sidechain_alone_...`), independently of the in-tree oracle, and so do the
grid's `Silent`/`Sidechain` cases. That is the fallback the brief prescribes. My boundary
differential also goes red on it. `causality` and `partition` cannot see it: the mutation stays
causal and partition-invariant.

**The three clamps are kept.** `curve_target` still ends `.max(reduction_min).min(zero)`
(`crates/compressor/src/kernel.rs:363-365`), and `crates/math` is untouched. Removing
`max(-100)` in scratch turns the differential red, because the knee-`2.8e-45` NaN is reachable.
The knee defect stays with #994.

**Findings (severity-ranked).** None blocking.

1. *Info.* The in-tree `reference` (`kernel.rs:1149`) shares `link_frame`, `one_frame`, `Coef`,
   `Invariants` and `advance_ramps` with production. It is therefore a fixed oracle only for loop
   structure, which the module doc states (`kernel.rs:1112`). A future slice that edits a frame-law
   helper (#986, or the clamp ruling) must diff against a separately built base, as this
   verification did.
