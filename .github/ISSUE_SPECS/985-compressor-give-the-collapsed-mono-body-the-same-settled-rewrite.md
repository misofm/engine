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


## Attempt 1 evidence

Implementer attempt 1, 2026-09-27, on top of #983+#984 (`0bfc2fd0`), branch
`codex/981-compressor-settled-body`. The verification amendment was followed: M2 is "the short
last chunk is skipped".

### The change

`crates/compressor/src/kernel.rs` only. `process_block_mono` keeps its signature; its idle call
sends `Detector::Main` to the new `#[inline(always)] settled_main_mono` and `Silent`/`Sidechain`
to `frames_loop_mono::<L, false>`, which is unchanged (as is the ramping prefix).
`settled_main_mono` loads `Coef` once, sets `dual_mono = link == DualMono` and
`wet = !bypass && every_lane(coef.wet_identity)` once per block, slices the left plane to the
settled frames, and runs `settled_frames_mono::<L, DUAL_MONO, WET>`: 32-frame chunks, pass 1 the
targets (`settled_detect::<L, DUAL_MONO>(main, main).0`, so `abs(main)` under DualMono and
`link_frame(main, main).0` under Maximum and Average, kept as the brief freezes), pass 2 the
recurrence and the shared `settled_output`. The recursive word lives in a local and is written back
once. All four of #981-#984 land together, as required. Stack scratch `[L; 32]`, 1 KiB at `Simd8`,
holding targets, not audio (the #983 justification). The collapsed contract is unchanged: no right
plane is read or written. The witness `SETTLED_WET_BLOCKS` counts the collapsed arm too.

### Gates

* **Gate 1**: the grid now runs three parameter sets through the collapsed body as well as the
  dual one -- the fixture tracks, `COMPRESSING_TRACKS` (thresholds -30 to -80, ratios 4 to 20,
  attacks 0.1 to 10 ms, makeup 6 to 24, all wet), and the parallel corpus table -- plus the four
  all-wet tables, at `f32`, `Simd4` and `Simd8`, every link mode, both `bypass`, every detector,
  frame counts `[1, 7, 31, 32, 33, 97, 128]` with starts `{0, 1, 18, 40}`, and the hostile input
  (subnormal-only blocks included, for Average). The randomized differentials run the collapsed
  body on every seed. Output and state by bits, with the witness-keyed "both NaN" rule on the arm
  only, and every payload difference in a rejected block. Dev and release.
* **Gate 2** (`scenario_985_collapsed_render_is_pinned`): `Simd4` and `Simd8`, DualMono and
  Average, the fixture (all-wet) and corpus (mixed) tables, the chunk-straddling schedule (the
  test asserts all 72 mid-chunk starts), hostile input. Recorded on B0 and on this slice's base
  (#983): `b48776f5e0d8609db1df969b051c066c8eac0c8abd0be373de4de529c1d9ff8c`, dev and release;
  unchanged after. The digests of #981-#983 are unchanged.
* **Gate 3**: `mono_collapse` (all five, including
  `the_collapsed_body_renders_the_dual_bodys_left_plane` and
  `a_halved_subnormal_does_not_come_back`), `silent_fixed_point`, `cross_target`, `partition`
  green; `cargo test --locked -p compressor` 93 passed in dev and release.
* **Gate 4**: all 30 console digests identical to B0, including `sixty_four_track_console_mono` and
  `_mono_dual` (`fc96d91f...`) and `sixty_four_track_console_half_mono` (`4a656cdf...`).
* **Gate 6**: browser rule 3, roster (compressor collapsed still one function: vector 92 -> 262,
  scalar 0; dual 516, scalar 0), `meter_poll`, `command_submit`, boot budget PASS; artifact
  3,365,419 bytes (+4,429 over #983, +19,804 over B0). Toolchain and policy: the table below.

| gate | command | result |
|---|---|---|
| toolchain | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| compressor | `cargo test --locked -p compressor`, and `--release` | 93 passed, 0 failed, each |
| other crates | `-p effect-runtime` (86), `-p console-workload` (39), `-p builtins-compiler --features test-support` (79) | all passed |
| wasm | `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` (113 passed); `bash scripts/run-wasm-gates.sh` | PASS, 0 mismatches on the native, wasm and wasm `simd128` legs |
| policy | `check-lane-policy.sh`, `check-realtime-policy.sh`, `check-unfused-seal.sh` | PASS |

**The differential under wasm.** In a throwaway scratch copy (never committed) the test module was
exposed to a `cdylib` guest and run under Node 22.23.2 (`--no-liftoff`): with `simd128`, 200 seeds
x 128 blocks of the randomized differential at `f32` and `Simd4`, dual and collapsed (102,400
blocks, 87,300 settled, 13,957 all-wet settled, 30,866 rejected, 2,020 NaN-payload words, all in
rejected blocks), and the grid on all seven tables (123,480 blocks): no failure. The same without
`simd128` (64 seeds): no failure. A guest built with 983-M4 (swapped recursive words) trapped
immediately, so the harness is live.

### Mutations (gate 5)

`MUTATIONS.md`, section "#985": M2 (short last chunk skipped) 7 red, M3 (write-back dropped) 11
red including four `mono_collapse` tests, M4 (wet arm ignores `bypass`) 7 red including
`mono_collapse::a_statically_bypassed_bank_collapses_to_the_dual_bits`. **M1 (Average takes
`abs`) is GREEN and recorded as equivalent**: on one plane `0.5|m| + 0.5|m|` differs from `|m|`
only for `|m| < 2 * f32::MIN_POSITIVE`, and the detector's only consumer, `curve_target`'s
`max(detected, 1e-8)`, maps both to the floor (the brief's expected red came from the retired
design, whose detector ring stored the linked value). The body keeps `link_frame` for Average as
frozen. Maximum taking `abs` (the verification's optional note) is also green and recorded.

### Codegen (recorded)

x86 `Simd8` release, `process_bank_mono`: B0's settled mono loop is 118 instructions per frame;
#985's DualMono-wet instantiation runs pass 1 at 52 and pass 2 at 36 (linked pass 1 60, general
pass 2 52), 3 scalar each. V8, function 380 `process_block_mono<f32x4>`: B0 190 instructions per
frame with 14 branches; #985's loops are unrolled by two frames, DualMono-wet 120 + 92 per two
frames (106 per frame) with 3 branches. As in the dual body, V8 now spills the retained
`Silent`/`Sidechain` mono loop (744 instructions against 317); no collapsed bank reaches it.

### Combined A/B, #981-#985 against B0 (descriptive)

One hold of the timing lock (`flock -w 7200 .../timing.lock`), 50 s, builds done first and outside
it; `CARGO_INCREMENTAL=0`; `taskset -c 31`, AMD EPYC 7313P (Zen 3), core clock 3.69 GHz. Host load
average 17-20 throughout (other agents were building), so treat these as descriptive. B0 is a
separately built `git archive` of `197db1c9` under `scratchpad/work-981/`; the candidate is this
branch's head. Native arms are the two binaries run alternately, twice each; V8 runs both modules
interleaved in one Node process. "Isolate" is the row's p50 minus the builtins-only control's p50,
median of seven 2,000-block rounds, per block of 64 tracks.

| row | B0 | #981-#985 | change |
|---|---:|---:|---:|
| native `Simd8` dispatch, compressor-only isolate | 40.83, 40.42 us | 23.39, 23.59 us | -42 % |
| native `Simd4` dispatch (unbanked `f32` instances), isolate | 314.84 us (repetition 2 hit a load spike: 317.1 in its clean rounds) | 81.69, 83.05 us | -74 % |
| native `console_mono`, isolate | 46.97, 47.85 us | 39.88, 39.58 us | -16 % (whole row 71.5-72.6 to 64.3 us, -11 %) |
| V8 (Node 22.23.2, `--no-liftoff`, `simd128`) compressor-only isolate | 80.86 us | 60.26 us | -25.5 % |
| V8 `console_mono` isolate | 129.93 us | 100.50 us | -22.6 % (whole row 180.9 to 151.2 us, -16 %) |

The V8 row digests matched between the two modules (`95c93754...`, `fc96d91f...`). These agree
with the verification's reproduction (-41 % native, -26 % V8, mono rows -10 % native and -17 %
V8). `scripts/run-console-benchmark.sh` was not run; the paired console benchmark is the batch
boundary's.
