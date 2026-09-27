# Recover the sidechained compressor's 3.5% V8 regression from the settled-body rewrite

## Product outcome

The compressor's settled-body rewrite (#981-#985) made every banked compressor shape faster (native Simd8 isolate 40.5 -> 24.5 us, V8 80.4 -> 60.1 us per 64-track block), except one: a compressor with a **connected sidechain** is 3.5% slower under V8 (373 -> 387 us over three runs), introduced by #983 (targets before recurrence); it is flat natively. The #981 verification recorded it; V8 now spills the retained Silent/Sidechain loop, which the new settled body does not cover.

## Smallest closable slice

Diagnose the V8 spill in the retained sidechain loop (register pressure from the shared frame-law helpers or the loop shape #983 left behind), and restore the sidechain shape to at least its pre-#983 V8 cost without moving a bit; optionally extend the settled body to sidechained compressors if that is the cleaner fix.

## Objective gates

- Sidechained compressor under V8: no slower than base `197db1c9` (A/B against a separately built base, under the timing lock), and native not slower.
- Bit-identical output and state against the base kernel (the #981 randomized differential).
- Every console digest unchanged.

Weekly-optimisation issue per AGENTS.md; not a release blocker.

## Attempt 1 evidence

Implementer attempt 1 (Terra), 2026-09-27, branch `codex/995-compressor-sidechain-v8` from the
local batch head `fc43c97d`; code commit `b5407795`. Oracles: the batch head's kernel on every
input (it carries #994's knee design); `197db1c9` for the timing A/B, built separately from its
own sources. Host: AMD EPYC 7313P (Zen 3), rustc 1.97.1, Node 22.23.2 (V8 12.4, `--no-liftoff`).
Every timed run held `timing.lock`, was pinned with `taskset -c 31`, and names its load; the
longest hold was 2m32s. `scripts/run-console-benchmark.sh` was not run.

### Diagnosis: where the 3.5 % went

Measured first in a shape a real host reaches, through the **shipped** `host_web.wasm` render
export (built with the production web recipe): the standing 64-track fixture reduced to its
compressors (`simd1`), each given a routed sidechain from its neighbour track's `input` tap, so
every compressor is prepared `Connected`, never banks, and renders as an unbanked `f32` instance
with `Detector::Sidechain` blocks. The web host boots that session unchanged. Isolate = the row
minus the same session with builtins only: `197db1c9` 345.5 us, batch head 357.4 us (+3.4 %),
reproducing the verification's +3.5 % in the product artifact.

V8's listing of the shipped artifact's `process_block::<f32>` (`--print-wasm-code`, natural loops
found by dominators, one iteration counted over the loop's laid-out blocks, deferred trap paths
excluded):

| build | retained `frames_loop::<f32, false>` iteration | frame (`[rbp-X]`) accesses | constants rebuilt from `[rip]` |
|---|---:|---:|---:|
| `197db1c9` | 381 instructions | 54 | 5 |
| batch head | 387 | 60 | 11 |

The per-frame detector `match` and the eight bounds checks are the same in both. What the head's
enlarged function (four two-pass instantiations beside it) did to the loop: two of its four frame
indices now live in stack slots (`movl rdx,[rbp-0x40]; addl rdx,4` at the latch, stored back
mid-body: a round trip every frame), the block bound is re-stored to `[rbp-0x198]` every frame
and all eight bounds checks compare against memory, and two 128-bit constants are rebuilt every
frame (`movq r10,[rip+..]; vmovq; movq; vpinsrq`) where base loaded them from the frame. That is
the carried and round-tripped stack traffic, and it is loop-shape noise, not arithmetic: no
operation of the frame law changed.

### The change (class A)

`crates/compressor/src/kernel.rs` only (plus `tests/MUTATIONS.md`). A block whose detector is a
connected sidechain, present or absent, now renders its settled frames through
`settled_sidechain` instead of the retained one-pass loop:

* the same `settled_frames` two-pass body as `Detector::Main` (#983), with a `source` argument:
  pass 1 reads the chunk's own planes for `Main` (every `settled_main` call site passes the
  literal, so it folds away: the native `Simd8` main-detector loops are the head's, instruction
  for instruction), the matching frames of the sidechain planes (sliced to the settled frames) for
  `Sidechain`, and nothing for `Silent`, whose one target (`link_frame(Silent)` then
  `curve_target`) is computed once per block;
* the DualMono `abs` arm (#984) carries over: under DualMono `link_frame` returns `abs` of
  whichever words it read;
* **the all-wet arm (#982) does not**: its sNaN relaxation stays confined to the main detector,
  where it was reviewed and still awaits the owner's acknowledgement. A sidechained block is
  bit-exact against the one-pass body, NaN payloads included;
* outlined and `#[cold]`. Inlined (measured first), it moved the main-detector bodies' register
  allocation: natively an extra `ymm` spill in the unbanked instance's vectorised first pass
  (about +0.4 % on 64 unbanked main-detector instances, slower in 14 of 15 alternations), under
  V8 one extra reload in a linked bank's first pass. `#[inline(never)]` alone kept the native
  spill (+0.8 %). With `#[cold]` the native `Simd8` main-detector loops are the head's, the
  native `f32` ones have the head's stack traffic or less (none more; several one to seven
  accesses fewer), and every V8 main-detector loop of `process_block::<f32>` and `<f32x4>` has
  the head's instruction and frame-access counts (two are one instruction shorter).

`frames_loop` keeps its source and its `RAMPING` parameter; production now instantiates only the
ramp prefix (`RAMPING = true`), and the one-pass settled form lives on as the oracle in
`settled_body_tests::reference`. The collapsed body is untouched (only banks collapse, and banks
never carry a sidechain).

### Timing (us per 128-frame block, 64 tracks unless noted; two reps each, rep1 / rep2)

**V8, shipped `host_web.wasm` render export** (9 rounds x 1,000 blocks, arms interleaved, median
of per-round p50; hold at load 15.5 falling to 9.5):

| session | `197db1c9` | batch head | #995 | #995 against base |
|---|---:|---:|---:|---:|
| sidechained, DualMono: row | 394.80 / 394.84 | 408.55 / 407.35 | 189.51 / 187.99 | |
| sidechained, DualMono: isolate (row - builtins-only) | 345.4 / 345.0 | 358.5 / 357.7 | 139.8 / 138.6 | **-60 %** |
| sidechained, Maximum link | 391.95 / 391.80 | 405.12 / 404.11 | 181.03 / 182.81 | -54 % (row) |
| 3 sidechained tracks | 20.95 / 20.99 | 21.65 / 21.73 | 11.43 / 11.35 | -46 % (row) |
| settled dual bank, DualMono wet | 130.08 / 129.84 | 110.67 / 109.57 | 109.40 / 109.46 | head to #995: flat |
| settled dual bank, Maximum, wet | 130.20 / 130.03 | 112.05 / 111.61 | 112.05 / 111.83 | flat |
| settled dual bank, Maximum, mix 0.7 | 130.10 / 130.06 | 126.42 / 125.92 | 126.78 / 125.22 | flat |
| collapsed mono bank | 92.45 / 93.18 | 63.57 / 64.00 | 63.90 / 63.60 | flat |
| 3 unbanked main-detector tracks | 21.06 / 21.04 | 11.08 / 11.07 | 11.13 / 11.08 | flat |
| builtins-only control | 49.40 / 49.83 | 50.08 / 49.63 | 49.73 / 49.38 | |

**Native, the same sessions through the production console pipeline** (`SessionRuntime`; 5 x
1,500 blocks; same hold):

| session | `197db1c9` | batch head | #995 |
|---|---:|---:|---:|
| sidechained, DualMono (`Simd8` dispatch) | 341.51 / 346.40 | 341.37 / 342.54 | 138.28 / 137.84 (-60 %) |
| sidechained, Maximum | 341.72 / 347.94 | 341.32 / 341.64 | 132.87 / 133.25 |
| 3 sidechained tracks | 18.50 / 18.55 | 18.51 / 18.51 | 8.93 / 8.99 |
| settled dual bank | 64.88 / 66.14 | 48.61 / 48.80 | 48.26 / 48.31 |
| Maximum wet bank / mix 0.7 bank | 65.34 / 65.85, 66.11 / 65.89 | 50.06 / 49.62, 56.95 / 56.33 | 50.10 / 49.65, 57.05 / 56.91 |
| collapsed mono bank | 38.39 / 38.24 | 30.13 / 30.69 | 30.80 / 30.08 |
| 3 unbanked main-detector tracks | 18.48 / 18.50 | 7.59 / 7.58 | 7.58 / 7.63 |
| `Simd4` dispatch (every compressor unbanked `f32`): sidechained | 358.74 / (disturbed) | 358.85 / 358.96 | 153.93 / 155.27 |
| `Simd4` dispatch: main detector | 357.93 / (disturbed) | 124.26 / 124.34 | 124.12 / 124.54 |

(The base's second `Simd4` rep ran through a load spike: its control read 58.6 against 42.)

**The verification's reachable-shape harness** (`vbench`, 64 scalar instances or 8 banks, head
against #995, 5 x 600 blocks natively and 5 x 500 under V8; loads 7.4 to 9.1):

| shape | native head / #995 | V8 head / #995 |
|---|---:|---:|
| scalar, connected sidechain | 309.4 / 104.5 (-66 %) | 386.2 / 125.0 (-68 %) |
| scalar, connected but absent sidechain (`Silent`) | 305.4 / 90.3 (-70 %) | 315.7 / 92.2 (-71 %) |
| scalar, sidechain with a ramp every block | 435.7 / 333.5 (-23 %) | 599.1 / 446.7 (-25 %) |
| scalar, main detector | 76.3 / 76.3 | 121.8 / 121.3 |
| bank settled DualMono wet / Maximum mix 0.7 | 24.9 / 24.7, 33.0 / 32.3 | 59.8 / 59.6, 76.9 / 77.3 |
| bank mono DualMono wet | 14.69 / 14.66 | 31.8 / 31.7 |
| bank, one lane ramping / all lanes ramping | 64.1 / 64.6, 102.1 / 102.0 | 144.0 / 144.4, 198.3 / 197.8 |
| bank mono, one lane ramping | 40.8 / 40.8 | 96.3 / 96.2 |
| scalar, main detector with a ramp every block (added shape; 4 native and 2 V8 reps, load 4.2 to 8.1) | 315.1 / 315.9 | 470.0 / 466.0 |

Every shape's 64-block digest is identical between the builds, natively and under V8.

### V8 listing after the change (the carried-slot check)

* `process_block::<f32>`: the retained loop is gone. The ramp prefix and all twelve main-detector
  loops have the head's instruction and frame-access counts (the prefix one write fewer).
* `process_block::<f32x4>` (the roster's dual kernel): all eight main-detector loops have the
  head's counts, two of them one instruction and one frame access fewer; the ramp prefix is one
  instruction longer with the same frame accesses; the retained 407-instruction loop (84 frame
  accesses, 34 rebuilt constants per frame) is gone. `process_block_mono::<f32x4>` is identical.
* `settled_sidechain::<f32>` (V8 compiles it on its own): LLVM vectorises pass 1 across frames
  (110 instructions, 26 frame accesses and 1 rebuilt constant per iteration) with a scalar
  remainder loop; pass 2 is 128 instructions and 18 frame accesses per frame. Pass 2 carries
  one 128-bit value through a stack slot, as the main-detector pass 2 does, and additionally
  round-trips one pointer. That residual is recorded, not chased (weekly rule).

### Exactness

* **In-tree** (`kernel::settled_body_tests`, dev and release): the grid and the three randomized
  differentials already ran `Silent` and `Sidechain` blocks against the one-pass `reference` and
  now assert that sidechained settled bodies started mid-block (release: about 6,800 settled
  sidechained blocks per width in the differentials, 2,376 per grid table, 2,112 of them
  mid-block). No relaxation applies to them. `scenario_995_sidechain_render_is_pinned` (both
  sources, `f32`/`Simd4`/`Simd8`, every link mode, both tables, blocks straddling 32-frame
  chunks, ramps ending at frame 40, 792 mid-chunk starts) was recorded on the **unmodified batch
  head** before the change, `25b39c7a6331a1571d2b5bc023e8a7e2d54ffefb9393e6b860b1b95a27e4c482`,
  identical in dev and release, and is unchanged after it. `scenario_981`, `982`, `983` and `985`
  are unchanged (`scenario_dual` gained a source parameter whose sidechain draws come from their
  own generator, so the `Main` scenarios render exactly what they rendered).
* **Independent** (scratch harness, not committed): the #981 verification's differential with the
  batch head's kernel as `Base`, built from its own sources beside #995's; every output word,
  every state word (coefficients, all ramp fields, recursive words), then the masks and all
  words again after `finish_channel`, **strictly by bits** (no NaN relaxation: the head already
  has the wet arm). Native release: 3,000 seeds x 160 blocks per width at `f32`, `Simd4` and
  `Simd8`, 0 failures, about 40,500 settled sidechained and 40,700 settled silent blocks per
  width, 8,300 of them mid-block, 27,000 multi-chunk; through the public factory (scalar with a
  connected sidechain present and absent, banks dual, collapsed and transitioning): 1,200 seeds x
  96 blocks, 0 failures. Under V8 (TurboFan): 1,500 seeds x 128 blocks at `f32` and `Simd4`,
  0 failures, about 16,000 settled sidechained and 16,000 settled silent blocks per width, and
  600 factory seeds, 0 failures; default tiering 48 seeds, 0 failures. The harness is live:
  995-M1 and M2 fail at every width and through the factory, and M5 fails at the kernel word only.
* **Digests**: all 30 standing console rows (15 workloads x `Simd8`/`Simd4` dispatch) and 16
  session rows (the sidechained, banked, mono and unbanked sessions above) are identical across
  `197db1c9`, the batch head and #995; the 15 wasm console-guest rows are identical between the
  head and #995; the shipped artifact's 64-block output digests of all eleven sessions are
  identical across the three builds.

### Gates

| gate | result |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` (and with `--all-features`) | PASS |
| `cargo test --locked -p compressor`, dev and `--release` | 99 passed, 0 failed, each |
| `cargo test --locked -p console-workload` | 39 passed |
| `bash scripts/run-wasm-gates.sh` | PASS: native, wasm scalar and wasm `simd128`, 142 cases, 358 comparisons, 0 mismatches |
| `bash scripts/check-env-vocabulary.sh` | PASS (134 names) |
| `check-web-audioworklet-callgraph.py --callgraph miso_engine_web_v1_render` | PASS, closure 8, traps 5 (the allowed `render_inner` owner only), unchanged |
| `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` | PASS; compressor dual row one function, vector 516 to 427 (the retained loop left it), scalar 0; kernels 14 to 15 (`settled_sidechain::<f32x4>`, dead for banks, matches no roster row) |
| `meter_poll`, `command_submit` closures; `check-web-boot-budget.mjs` | PASS |
| artifact | 3,381,477 to 3,391,362 bytes (+9,885); not repinned (the batch repins once) |

### Mutations

`crates/compressor/tests/MUTATIONS.md`, section "Issue #995": M1 (pass 1 reads the main planes) 9
red, M2 (the sidechain offset never advances) 8, M3 (sidechain not sliced to the prefix's end) 7,
M4b (`Silent` detects the main planes) 7, M5 (the all-wet arm for sidechained blocks) 4, M6 (the
`abs` arm for every link mode) 7. Recorded green, with reasons: M4 (the silent target left
zero-filled: it is `+0.0` for every legal parameter set) and M7 (the `abs` arm never taken for a
sidechain: performance-only).

### For the verifier

* **One native residual outside the gated shapes.** The `Simd8` bank's ramp prefix
  (`frames_loop::<f32x8, true>`, source untouched) compiles 7 instructions and 5 stack accesses
  shorter per frame (the per-frame detector test is now a register, and three spills are gone),
  yet `vbench`'s "one lane ramping every block" measured +0.5 % in 6 of 6 alternations (64.1-64.3
  to 64.5-64.8 us) while "all lanes ramping" measured -0.2 %. A build with every loop aligned to
  64 bytes measured +0.5 % and +1.9 % for the same pair, so the all-lanes shape changes sign with
  layout alone. I read the pair as layout rather than work, but have not proved it; under V8 both
  are flat. Not chased.
* **And one on the unbanked prefix.** The native `f32` ramp prefix went from 23 to 26 stack
  accesses per frame (same instruction count), and the added "scalar, main detector with a ramp
  every block" shape measured +0.25 % (315.1 to 315.9 us, slower in 3 of 4 alternations); under
  V8 it is flat or faster. The settled unbanked shapes, which the gate names, are flat.
* The inlined form's +0.36 % on native unbanked main-detector instances was the reason for
  `#[cold]`; with it, that row is flat (124.26 / 124.34 against 124.12 / 124.54). `#[cold]` is
  a register-allocation weight, and a later change around `process_block` can move it again,
  which is what #1000's spill gate is for.
* The sidechain keeps the general output law on purpose. Extending #982's arm to it would save
  more, but widens a relaxation the owner has not yet acknowledged; M5 shows the gates see it.
* Scratch harness sources, session documents and raw logs:
  `scratchpad/impl-995/{harness,sessions,logs}`.
