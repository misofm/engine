# EQ: fold the 4.4 boundary scan into the cascade's final store on admitted stationary blocks

## Product outcome

After the stationary EQ cascade renders an admitted block, `render` re-reads both output planes in `check_block` to apply the 4.4 block-limit rule (`|y| < BLOCK_LIMIT`, otherwise zero the block and reset). The cascade's last pass already holds every output word in a register when it stores it. EQ-6 in `docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md` measured folding that check into the final store at about -294 cycles per bank under V8 (about -1.3 us per 64-track block) and -125 natively at `Simd8` (about -0.3 us). The depth-1 kernel is latency-bound, so the extra compare and `and` per vector ride free there.

## Smallest closable slice

For an admitted stationary block only, the last pass (depth-2 pair or depth-1 tail) accumulates the `abs(y) < BLOCK_LIMIT` verdict per stream as it stores, and `render` uses that verdict instead of calling `check_block`. A zero-section plan, a refused block and every ramped block keep `check_block` unchanged. This needs a verdict-returning variant of the lane kernel in `crates/lane`. Class A: the verdict, the zeroing and the reset must be the same on every block.

**Wasm hazard.** #977 showed that V8's register allocation of the one-band depth-1 tail depends on the code around the loop: attempt 1 spilled an integrator and ran +21 % slower in the shipped artifact while every in-crate timing looked fine. This change touches that loop. Measure through the shipped `host_web.wasm` render export and inspect the V8 listing for a carried stack slot (the #977 verifier's harness: `v8loops.py`), not only a private wasm guest.

## Objective gates

1. **Verdict equality.** A test asserts the folded verdict equals `check_block` on the same output planes for every admitted stationary block, including outputs exactly at, just below and just above `BLOCK_LIMIT`, `+-inf` and NaN produced from admitted input where reachable, at `f32`, `Simd4` and `Simd8`, dev and release. Compare NaN words as "both NaN" where the compiler may commute.
2. **Bit-identity.** Rendered words, integrators and the reset behaviour equal the batch head's on the #977/#979 differentials and a block-limit scenario pinned on the batch head. Every `WORKLOADS` digest unchanged.
3. **Mutations** (each alone, red): the folded verdict ignores the second stream; uses `<=` instead of `<`; is used on a refused block.
4. **Timing, no regression** (under the timing lock): shipped `host_web.wasm` one-band, two-band and builtins isolates, and native `Simd8`/`Simd4` `eq_only` isolates, none slower than the batch head; the V8 listing of the one-band tail carries no stack slot.
5. Clippy, fmt, `scripts/check-web-audioworklet-callgraph.py` (rule 3 in particular) and the wasm gates pass. Render stays allocation-free.

## Attempt 1 evidence

Implementer: Terra, attempt 1, 2026-09-27, branch `codex/999-eq-fold-boundary-scan` from the batch
head `fc43c97d` (`codex/batch-plumbing-floor-2`). Host AMD EPYC 7313P (Zen 3), `rustc 1.97.1`,
`x86-64-v3`, Node 22.23.2 (V8 12.4); every build `CARGO_INCREMENTAL=0`, scratch builds in their own
target directories. The base arm of every comparison is a separate `git archive fc43c97d` build.

### The change

- `crates/lane/src/kernels.rs`: `svf_cascade_interleaved_bounded` and
  `svf_cascade_interleaved_with_dry_masks_bounded`, the interleaved body with a monomorphized store
  observer (`StoreObserver`: `Unobserved` for the existing entry points, `StoreBound` for the new
  ones). `StoreBound` folds `ok = ok AND (|y| < limit)` per stream as each output vector is stored
  and returns `NOT mask_any(NOT ok)` per stream: `check_block`'s fold over the same words. The
  skewed kernels are untouched (their short-block fallback passes `Unobserved`). No `unsafe`, no
  allocation, `#[inline(always)]`.
- `crates/parametric-eq/src/lib.rs`: the depth-one tail of `interleave` and `interleave_mono` calls
  the bounded twins (the tail rule of #976/#977 unchanged: masked when a lane of either channel is
  dry, select-free otherwise) and returns the verdict; `process_channels*` pass it up
  (`Option<[bool; 2]>` / `Option<bool>`), and `render`/`render_mono` use it instead of
  `check_block`. `None` (the scan) for: a ramped block, a refused or all-live list, a plan with no
  live section, and an even list (see deviation 1). Docs on `render`, `process_channels*`,
  `interleave` ("The tail judges the §4.4 bound as it stores"), `interleave_mono`.
- Tests: gate 1 in-crate (`boundary_fold`), gate 2 public scenario (`tests/bank.rs`), the lane
  kernel gate (`g2_bounded_cascade_is_the_cascade_and_judges_what_it_stores`), both
  `MUTATIONS.md`.

### Shape: the tail folds, the pairs do not (the V8 hazard decided it)

Two shapes were built and measured through the shipped artifact before any test was written:

| arm | what folds | V8 listing of `PreparedParametricEq<f32x4, 4>::process_bank` (TurboFan, `carried.py`) |
|---|---|---|
| base `fc43c97d` | nothing | dual select-free tail 83 instructions, no carried slot; masked tail 88 with `[rbp-0xc0]` carried (as at #977) |
| v1 | the tail, and the last pair of an even list (a bounded skewed kernel) | tail 96, no carried slot; **bounded dual pair 194 instructions with four carried slots** (`[rbp-0x170]`, `[rbp-0x1f0]`, `[rbp-0x2f0]`, `[rbp-0x360]`, two of them flushed integrators: a `vandnps` result stored and reloaded next iteration) |
| **v2 = this change** | the tail only | **every EQ loop clean**: dual select-free tail 96 (was 83), masked tail 102 and now clean (was 88 with a carried slot), pairs 181/216 as base; mono tails 47/49 clean (base's masked mono tail carried `[rbp-0xa0]`) |

v1 also made the native per-node (`f32`, the x86 `Simd4` plan) two-band isolate 4.6 us (4.8 %)
slower, and bought 0.7-0.9 us on the web two-band row (holds 1-3, table below). The brief's
"depth-2 pair or depth-1 tail" is therefore narrowed to the tail (deviation 1).

The dual select-free tail loop (the standing console fixture's whole cascade) is 96 instructions
against base's 83. The four integrators stay in registers: `carried.py` finds no stack slot read
before it is written in the body. The additions per iteration are the fold (per stream a `vandps`
against the abs-mask constant, a `vcmpps (lt)` against the limit and a `vpand` into the
accumulator), the limit splat rematerialised from a constant and parked in `[rbp-0x110]`, and the
two accumulators, which V8 carries across the back edge in registers but spills at the loop top
(`[rbp-0x120]`, `[rbp-0x100]`) and reloads just before their `vpand`. That round trip lengthens the
accumulator's own one-`vpand` chain by a store-to-load forward. It is not on the integrator
recurrence, and `carried.py`'s rule does not flag it, because the first access in the body is the
store. The verifier may want to judge it by the timing below rather than by the rule.

### Gate 1: verdict equality (`boundary_fold::the_folded_verdict_is_the_boundary_scan`, in-crate)

Every block is rendered through `process_channels` and `process_channels_mono` exactly as `render`
calls them, and the returned verdict is compared with `check_block` of the rendered planes. It
also asserts that a verdict is returned exactly when the block is stationary and the kept list is
odd (1, 3 or 5 sections), and never for 0, 2, 4 or 6 sections or a ramped block. The stored words
are steered to the limit with *scale* sections (live, `c1 = a2 = a3 = m1 = m2 = +0.0`, chosen
`m0`: the output is `m0 * x` rounded once) and a *clash* section (`m2 * v2 = -inf` against a
restored `ic2 = 1e30`, giving `-inf`, or `NaN` on `x = 1e30`). Every input word is admitted
(finite, at most `1e30`, never `-0.0`) except the refused variants (one `-0.0`, one `NaN`). 11 live
masks (0 to 6 sections, a cut or a band alone, the LPF last with dry lanes) x 5 tail pairings
(tame/hot per channel, loud/quiet input) x 3 variants x 1 and 16 frames, stationary and ramped,
plus a corpus family: designed bands over input scaled to `1e30`, some lanes with restored
`ic1 = f32::MAX`, `ic2 = -f32::MAX` (admitted by leg (c)), and quiet seeds. Coverage, identical in
dev and release:

| width | folded blocks | scanned blocks | dual verdicts pass-pass / fail-pass / pass-fail / fail-fail | mono pass / fail | stored words at / just below / just above `1e30` | `+inf` / `-inf` / `NaN` | corpus folds failing / passing |
|---|---:|---:|---|---|---|---|---|
| `f32` | 720 | 1,956 | 132 / 32 / 23 / 17 | 452 / 64 | 89 / 457 / 58 | 154 / 258 / 139 | 8 / 4 |
| `Simd4` | 864 | 1,812 | 87 / 31 / 31 / 73 | 346 / 296 | 1,034 / 1,812 / 286 | 776 / 1,800 / 1,035 | 8 / 4 |
| `Simd8` | 864 | 1,812 | 70 / 30 / 24 / 98 | 274 / 368 | 2,112 / 3,730 / 482 | 1,348 / 3,962 / 2,225 | 8 / 4 |

The test asserts every cell of the coverage is non-zero. Verdicts are booleans, so the "both NaN"
comparison has nothing to relax; a NaN word fails both sides.

**Kernel gate (lane).** `g2_bounded_cascade_is_the_cascade_and_judges_what_it_stores`: both
bounded kernels against `svf_cascade_interleaved[_with_dry_masks]` at `f32`, `Simd4`, `Simd8`, one
and two streams, depths 1 and 2, the G2 coefficients and the identity words, limits `1e30`, `1.0`
and infinity, frames 1, 2, 3 and 128, 24 carried blocks of hostile input with edge words (the
limit and its neighbours, both infinities, `NaN`, `-0.0`): output and integrator words "both NaN,
or equal bits", and each stream's verdict equal to the scan of what it stored (the scan itself
checked word by word against `|x| < limit`). Every pair of two-stream verdicts occurs, and some
stored word sits exactly at the limit, at every width. Green in dev and release.

### Gate 2: bit-identity

- **Scenario pinned on the batch head** (`admitted_blocks_over_the_block_limit_render_the_base_bits`,
  `tests/bank.rs`, public API only). 8 tracks (4 voices, so the per-track digest does not depend on
  the bank width), 24 blocks (128 frames, a 37-frame block every fifth, one 1-frame block). Six
  shapes: a hot bell (the select-free tail), three live sections (pair then tail), the LPF with dry
  lanes as the tail (prepared targets, so its ramp blocks scan), two hot bells (pair only), nothing
  live, and a bell restored with `ic1 = MAX`, `ic2 = -MAX` on the left then the right channel.
  Input cold, warm (`1e28`), hot (`9e29`) or at the edge (`1e30`, its neighbours) per plane per
  block, so planes fault left only, right only, both or neither. A `-0.0` and a `NaN` block refuse.
  Output words, reports and state payloads after every block, one SHA-256 per leg. Pinned with the
  test file copied into a `git archive fc43c97d` tree, identical in dev and release: scalar
  `69929ee05f9192faebe174ef7a6d48a5de4abd584e18e1a4834ec0913b4a7c95`, bank
  `033bb41c2daae4fcc72b4cc61e62ce74e298cbb0234479be41fbf4f40518ff0c`, bank-mono
  `0da773b7d5ee458d4175f31b1523a035673dc39c6dc8c16eb5f425335864e289`. The change reproduces all
  three in dev and release, with and without `test-support`. Fault counts (left only, right only,
  both), per shape: scalar and bank `[44,24,72] [48,24,72] [50,30,42] [46,24,72] [26,24,24]
  [44,26,74]`; the test asserts each hot shape faults each plane alone, and every shape faults.
- **The #976-#979 scenarios** (`odd_live_counts…`, `admitted_blocks…without_selects`,
  `two_and_four_live_sections…`, `a_cut_switched_off…`) keep their pins, dev and release,
  with and without `test-support`.
- **Differential (the #977/#979 verifier's harness,** `verify-977/harness`, rebuilt against
  `fc43c97d` and this change with the scratch four-lane binding hook added to both scratch trees):
  seeds 0-19,999 x 7 legs (scalar; `Simd8` and `Simd4` banks, dual, mono and mixed), every output
  word, report field, target and restore result and every lane payload after every block:

  | build | runs | differing |
  |---|---:|---:|
  | native release (fat LTO) | 140,000 | 0 |
  | native dev | 10,500 | 0 |
  | wasm `simd128` release (Node 22, `Simd4` banks) | 28,000 | 0 |
  | wasm `simd128` dev | 3,360 | 0 |

  A scratch counter in a third copy (identical output lines to the change) counts the fold: 3.82M
  folded blocks over the native release run, 312,405 of them with a failing channel (13.6M blocks,
  7.40M admitted).
- **Rows.** 64-block digests of all 15 `WORKLOADS` rows at `Scalar`, `Simd4`, `Simd8`, one band and a
  scratch-only second band (90 lines): identical to the batch head, and equal to #976's table; the
  wasm console guest (`+simd128`, release) renders the same 30 digests at both commits, each equal to
  the native one. `chain_shape` (release) 23 passed.

### Gate 3: mutations

Recorded in `crates/parametric-eq/tests/MUTATIONS.md` and `crates/lane/tests/MUTATIONS.md`
("Issue #999"). Each alone, in a scratch copy of the change, release:

| # | mutation | red on |
|---|---|---|
| M1 | the fold ignores the second stream | lane gate (`S=2 … stream 1: the folded verdict is not the scan`); gate 1 (`[true, true]` against `[true, false]`); gate 2 (non-vacuity: no right-only fault; scalar `914a4db8…`, bank `e301d357…`) |
| M2 | the fold uses `<=` | lane gate (a dry lane stored `1e30`); gate 1 (`[true, true]` against `[false, true]`). Gate 2 stays green: no scenario stores a word exactly at `1e30` on a folded block |
| M3 | a refused or all-live block (the masked pairs) takes the folded path with a verdict of `[true, true]` | gate 1 (`kept 6, stationary true: only an admitted list ending in the depth-one pass folds`); gate 2 (scalar `8cde7a90…`) and all four #976-#979 scenarios |
| M3m | the same in `interleave_mono` | gate 1 (mono); the bank-mono leg of all five scenarios |
| M4 (added) | a plan with no live section takes `[true, true]` | gate 1 (`kept 0`); gate 2 (scalar `99dc6ad1…`: the nothing-live shape's `1e30` words are no longer zeroed) |
| M5 (added) | the dry-lane tail judges against `f32::INFINITY` | gate 1; gate 2 (non-vacuity, scalar `2f36e7b8…`) |

### Gate 4: timing (descriptive, under the lock)

Builds first, outside the lock: a `git archive fc43c97d` tree (base) and the same tree with this
change's `lane` and `parametric-eq` (final), each with its own target directory; v1 (tail and last
pair folded) as a reference arm in the first three holds. Every hold `flock -w 7200
…/scratchpad/timing.lock`, `taskset -c 31`, under a minute each.

- **Shipped artifact**: the #977 verifier's `web.mjs` (64 tracks, the console fixture with only the
  EQ at one band `eq1` and two bands `eq2`, and no racks `builtins`; stereo sine through
  `source_submit`; median of ten rounds of the per-round p50 of 1,000 blocks), arms in both orders.
- **Native**: a scratch `console-workload` example (not committed): `sixty_four_track_eq_only` at
  one and two live bands (a scratch-only knob), minus `sixty_four_track_builtins_only`, at `Simd8`
  and `Simd4` (on x86 the `Simd4` plan renders the EQ per node, `PreparedParametricEq<f32, 1>`);
  1,000 warm-up blocks, six rounds of 1,500, median of the round p50s; arms alternated three rounds.

Holds 1-3 (base, final, v1, rotated orders) ran at load average 6.9-9.5. Other agents' parallel
test suites then pushed the host to 10-20, so holds 4 and 5 (base against final, web x3) and hold 6
(native x2, load 11-13.5) are filtered by a rule fixed before comparing arms: a web invocation is
dropped when any row's median exceeds that row's best round by more than 2 %, and a native round
pair when either arm's `builtins` p50 exceeds its hold median by more than 3 %. The web pool
therefore holds 15 of 18 invocations, and the native pool holds holds 1-3 and 6 (15 paired
rounds). The native legs of holds 4 and 5 were visibly disturbed (isolates up to 19.6 us) and are
not used. Values in us per 64-track block; deltas are final minus base within one invocation or
round:

| row | base | final | paired delta, mean (median) | final lower | v1 (holds 1-3) |
|---|---:|---:|---|---:|---:|
| web one-band isolate | 20.33 | **19.71** | **-0.62** (-0.65) | 15/15 | 20.05 |
| web one-band row | 70.21 | 69.51 | -0.70 (-0.59) | 15/15 | 69.30 |
| web two-band isolate | 30.54 | 30.64 | +0.10 (+0.01) | 7/15 | 29.62 |
| web two-band row | 80.43 | 80.44 | +0.01 (+0.07) | 6/15 | 78.87 |
| web builtins | 49.89 | 49.80 | -0.08 (+0.03) | 7/15 | 49.25 |
| native `Simd8` `eq_only` isolate | 9.41 | **9.24** | **-0.17** (-0.17) | 11/15 | 9.08 |
| native `Simd4` (per-node `f32`) `eq_only` isolate | 73.46 | **71.13** | **-2.33** (-2.21) | 15/15 | 70.93 |
| native `Simd8` two-band isolate | 13.97 | 14.10 | +0.14 (+0.04) | 7/15 | 14.27 |
| native `Simd4` two-band isolate | 95.08 | 94.89 | -0.20 (-0.09) | 9/15 | **99.64** |

Holds 1-3 alone read the web two-band isolate at +0.35 us (row +0.15, builtins -0.21; final lower in
1/6), and the six undisturbed invocations of holds 4-5 read it the other way; pooled, neither the
row nor the isolate is distinguishable from the batch head. The gate's rows: the one-band web
isolate and both native `eq_only` isolates are faster; the web builtins row is flat (-0.2 %); the
web two-band row is flat (+0.01 us, 0.01 %). EQ-6's replica projected -1.3 us (web) and -0.3 us
(native `Simd8`); the shipped artifact gives -0.62 us and native -0.17 us. The per-node `f32` path
(-2.3 us) was not in the projection. No saving is claimed beyond these descriptive numbers.

**V8 listing** (`node --no-liftoff --no-wasm-lazy-compilation --print-wasm-code-function-index`,
`v8loops.py`, `carried.py`): no EQ loop of `process_bank` or `process_bank_mono` carries a stack
slot in the final artifact (base: the masked dual tail `[rbp-0xc0]` and the masked mono tail
`[rbp-0xa0]`). The one-band dual tail is 96 instructions (83 at base); see "Shape" above for its
accumulator spill.

**Native codegen** (release `console-workload` example, fat LTO): `process_bank::<f32x8>`'s dual
select-free tail loop is 68 instructions (59 at base): per stream one `vandps` (abs), one
`vcmplt_oqps` and one `vandps` (accumulate); its stack operands are all coefficient loads (11, was 7),
none stored. The unchanged select-free pair loop (128 against 127) differs from base only in a
general-purpose register: the left plane's base pointer is reloaded from the stack once per
iteration (`mov rsi,[rsp+N]`); every vector instruction and vector stack operand is the same.

### Gate 5: toolchain, policies, artifact

All on `a9adc750` (the evidence commit changes only this file), `CARGO_INCREMENTAL=0`:

| command | result |
|---|---|
| `cargo fmt --all --check` | pass |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` (and `--all-features`) | pass |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps -p lane -p parametric-eq` | pass |
| `cargo test --locked -p parametric-eq`, dev and release, with and without `--features test-support` | 111 passed, 3 ignored (each of the four) |
| `cargo test --locked -p lane`, dev and release | 70 passed, 2 ignored |
| `cargo test --locked -p console-workload`, dev and release | 39 passed, 2 ignored (`chain_shape` 23) |
| `bash scripts/run-wasm-gates.sh` | ok (native + wasm scalar + wasm simd128), 358 comparisons, 0 mismatches |
| `bash scripts/check-env-vocabulary.sh` | ok (134 names) |
| `check-realtime-policy.sh`, `check-lane-policy.sh`, `check-parametric-eq-render-contract.sh`, `check-unfused-seal.sh`, `check-workspace-policy.sh` | ok (57 marked regions), ok, PASS, ok, ok |

**AudioWorklet artifact** (the build script's cargo line, not repinned; base `eee596e8…`, this change
`b5f2ef75…`), `wasm-objdump -d` into `scripts/check-web-audioworklet-callgraph.py`, identical
before and after: `--callgraph miso_engine_web_v1_render` closure=8 traps=5, one trap owner
(`render_inner`); `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` ok,
kernels=14 (rule 3), f32x4_arith=13149, `parametric-eq f32x4 dual` one kernel vector=672 scalar=0,
`collapsed` 336/0, every other roster row unchanged; `meter_poll` and `command_submit
--allocation-only` ok. The rule counts `f32x4.{mul,add,sub,div}`; the fold adds `f32x4.abs`,
`f32x4.lt` and `v128.and`, so the counts do not move. Render stays allocation-free (the
callgraph's forbidden-symbol scan, the realtime policy, and no new heap type: the fold is two mask
registers and a splat).

### Deviations and notes for the verifier

1. **The pair is not folded.** The brief says "the last pass (depth-2 pair or depth-1 tail)". The
   pair fold was built (v1) and measured: in the shipped artifact V8 carried four values of the
   bounded dual pair through stack slots, two of them flushed integrators (the #977 hazard, on the
   recurrence), and the native per-node two-band isolate got 4.6 us (4.8 %) slower. It bought
   0.7-0.9 us on the web two-band row. Only the tail folds; an even list (2 or 4 live sections) scans.
   `svf_cascade_skewed` is untouched, and there is no bounded skewed kernel.
2. **The mono path folds too** (`interleave_mono`, `render_mono`): the brief names `render`; the
   collapsed body is the dual body with a channel deleted, and gate 1, gate 2's bank-mono leg and
   M3m cover it.
3. **The V8 tail's accumulators** round-trip through `[rbp-0x100]`/`[rbp-0x120]` inside each
   iteration (stored at the loop top, reloaded before their `vpand`). `carried.py` does not flag
   them (first access is a store), and they are not on the integrator recurrence; the timing is the
   evidence that they cost nothing measurable. A committed V8 listing gate is #1000's.
4. **Two-band rows**: the pair path's source is unchanged, but V8's and LLVM's allocation around it
   moved (web pair loops same shape, other registers; native one GPR reload per pair iteration).
   Pooled, both two-band rows are flat; holds 1-3 alone read the web two-band isolate +0.35 us.
5. **M2 moves no public digest**: no gate-2 shape stores a word exactly at `1e30` on a folded block,
   so M2 is caught by gate 1 and the lane gate only.
6. **Floor accounting unchanged**: `EQ_LANE_OPS = 27` counts 24 for the section and 3 for the boundary
   scan; the fold executes the same three operations per word inside the tail's loop.
7. **Artifact not repinned** (`eee596e8…` at the batch head already differs from the committed pin
   `8934cdd9…`; repinned at the batch boundary). `scripts/run-console-benchmark.sh` was not run.
8. Scratch material (never committed) is in `scratchpad/impl-999/`: the harness example `zz999.rs`,
   `build_arm.sh`, `v8web.sh`, `webhold.sh`, `nathold.sh`, `webpool.py`, `natpool.py`, `mutate.sh`,
   the hold logs, the V8 listings, and the rebuilt differential harness (`diff/`); target
   directories and source copies were deleted.

## Sol attempt 1 verdict: FAIL

Verifier: Sol, 2026-09-27, on `a9adc750`, judged merged onto the current batch head `b03edde4`,
which carries #1000's V8 spill gate. The merge is clean (`git merge-tree`), and between `fc43c97d`
and `b03edde4` the batch changed nothing in `lane`, `parametric-eq`, `effect-runtime` or
`host-web`. Every arm was built from `git archive` into its own target, `CARGO_INCREMENTAL=0`,
outside the lock.

**The change is exact (class A).**

- **The verdict is `check_block`'s.** `StoreBound` folds the same predicate, `abs(y).lt(limit)`,
  over exactly the words the depth-one tail stores, and those are the block's final words.
  Elided sections after the tail do not run, and nothing writes the planes between the cascade and
  the check. For a masked tail it is the selected word.
- **Same fold, same range.** The fold starts from the same all-true mask and reduces with
  `mask_any(mask_not)`. `render`'s planes are exactly `frames * W` words (`EffectProcessBlock` and
  `EffectBankProcessBlock` enforce it), so the fold and the scan cover the same range.
- **Zeroing and reset.** A failing verdict still runs `nonfinite_lane_mask` and zeroes and resets
  from the plane, so both are unchanged.
- **Which blocks fold.** Only an odd, admitted list returns a verdict. Ramped, refused, all-live,
  nothing-live and even-length lists still scan.

**Differential.** My harness adds a block-limit shape: a 0 dB bell that stores its input
unchanged, dry LPF lanes, and input at `1e30`, one ulp below, `9.99e29` and `+0.0`. Against
`b03edde4`:

| build | differing runs |
|---|---|
| native release | 0 of 140,000 |
| native dev | 0 of 4,200 |
| wasm `simd128` release | 0 of 28,000 |
| wasm `simd128` dev | 0 of 1,400 |

Those runs contain 3.94M folded blocks: 692k faulted, and 585k carried input at or above the
limit's predecessor. The 90 native and 30 wasm console digests are identical.

**Mutations and gates.** M1 (the second stream ignored), M2 (`<=`) and M3 (a pass verdict on the
six-entry list) are red when re-run in a scratch copy; M3 also turns the pinned bank scenario red.
On the merged tree these are green:

- `-p parametric-eq`, 111 passed and 3 ignored, dev and release, with and without `test-support`;
- `-p lane` 70; effect-runtime, console-workload (dev and release), `chain_shape`,
  builtins-compiler, `wasm-gates` tests, bench floor;
- fmt, clippy and doc `-D warnings`;
- the lane, realtime, EQ-contract, env-vocabulary, unfused-seal and workspace policies, and
  `test-console-benchmark.sh`.

**AudioWorklet checks.** The render, `meter_poll` and `command_submit` callgraphs are identical to
base. Rule 3 holds: 14 kernels, `--kernel-min 11`. The roster is unchanged, EQ dual 672 and
collapsed 336 with scalar 0; the fold's `f32x4.abs`/`lt`/`v128.and` are not counted.

Findings:

1. **HIGH (blocking): the merge fails the batch's wasm gates.** `bash scripts/run-wasm-gates.sh` on
   the merged tree exits 1. So does
   `scripts/check-web-audioworklet-v8-spill.py <merged host_web.wasm>`, while the batch head's
   artifact passes:

   ```text
   FAIL dual depth-1 tail, select-free: V8 carries [rbp-0x100], [rbp-0x120] from one iteration to the next (96 instructions ...)
   ```

   - **What the slots are.** They are the tail's two new verdict accumulators. V8 stores each at the
     loop top and reloads it just before its `vpand`, so `acc = acc AND cmp` runs through memory
     every iteration. This is a real recurrence through a slot, and the path crosses the loop
     header, so #1009's planned fix for intra-iteration false positives will not clear it.
   - **Benign for speed?** Probably. The chain is one `vpand` plus a store-to-load forward, far off
     the roughly 20-cycle integrator recurrence, and the one-band timing is faster. But the gate is
     `qualification.yml`'s `wasm-guests` job, which the merge must pass. The implementer's evidence
     predates #1000, so this could not have been seen then.
   - **A remedy exists.** I built one scratch variant: keep a per-stream `bool` and fold
     `failed |= mask_any(mask_not(abs(y) < limit))` at each store. It is the same predicate, so it
     is exact by construction. It passes the spill gate: tail 109 instructions, no carried slot,
     mono tail 53.
   - **For attempt 2.** Adopt that or an equivalent. Show the spill gate green on the merge. Re-time
     one band against the batch head, since the variant adds a `movmsk`/`or` per stored vector. Do
     not loosen the gate.
2. **INFO (timing).**
   - **My own holds are inconclusive.** I ran two, under `timing.lock` with `taskset -c 31`, but other
     agents held the host at load average 18-31, and only 2 of 14 web invocations meet the
     implementer's undisturbed rule. Those two read one band -0.47 us and two bands -0.48 us.
   - **The implementer's filter is sound.** It drops whole invocations, both arms at once, on
     within-invocation dispersion only. The three it dropped had two-band deltas of -0.71, +2.48 and
     -13.97, mixed in sign. Dropping them moves the pooled two-band delta from -0.60 to +0.10, which
     is against the change.
   - **The +0.35 us is not supported.** Holds 1-3's two-band +0.35 us (5 of 6 positive) is
     contradicted by holds 4-5's nine undisturbed pairs (-0.07). Pooled, 7 of 15 are lower. The pair
     path's source is unchanged, and V8's pair loops keep their shape. I read it as noise. It is not
     a regression shown.
3. **LOW.** The pair was left unfolded on evidence (v1 carried four slots, two of them integrators).
   Deviation 1 narrows the brief's "pair or tail" correctly and should be recorded on the issue.

**#998.** It should not start on top of this branch yet. Attempt 2 will rewrite the same depth-one
tail and its accumulators. #998 then edits `cascade_sections` and the state-write sites in the same
`process_bank` body, whose V8 allocation both issues must keep green. Start #998 after #999's
attempt 2 passes and lands, and re-run the spill gate and the one-band artifact timing on the
combined code.
