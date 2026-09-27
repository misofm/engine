# Adversarial verification of compressor briefs #981-#986

Date 2026-09-27. Tree: detached worktree of `codex/batch-plumbing-floor-2` at `28964870`. The
compressor, lane, math and effect-runtime sources are byte-identical to the diagnosis base
`6ca203f8`. Both diagnosis patches applied cleanly. Read-only: no push, no issue edits, and
`scripts/run-console-benchmark.sh` was not run. Every timed run held `timing.lock` and was pinned
with `taskset -c 31`; the longest hold was 1m45s. Host load average ranged from 4 to 27 (other
agents were running); each figure below names its run. Harness sources and raw logs are in
`scratchpad/verify-comp-evidence/`.

## 0. Verdicts

| issue | verdict | the one thing that must change |
|---|---|---|
| #981 hygiene | **brief with amendments** | M4 is not a red mutation (proved green); the "sidechain-connected compressor" claim is wrong |
| #982 all-wet arm | **brief with amendments** | M3 names `oracle`, which stays green (proved); the finish_channel dependency needs an owner acknowledgement |
| #983 two-pass | **brief with amendments** (minor) | gate 1 must drive a non-zero, non-chunk-aligned `start`; state the no-in-place-form justification |
| #984 DualMono arm | **brief as-is**, preferably **merged into #983** | none (the proof holds) |
| #985 mono rewrite | **brief with amendments** (minor) | M2 cannot be expressed (the mono body has no right plane) |
| #986 ramping prefix | **do not brief as written** | "choose whichever the measurement ranks first" cannot rank options that are not implemented |

Order: #981, #982, #983 with #984, then #985, all in one batch. #986 is separate and needs a
prototype first. Measure the batch once at its boundary against a **separately built unmodified
base**, never the in-binary mode 0 (section 3.4).

## 1. Exactness (severity: none found; every class-A claim holds)

### 1.1 The randomized differential

`verify_diff_module.rs` was added to the kernel in scratch only. It drives two identical `Channel`
pairs, block by block:

* oracle: today's ramp prefix, then `frames_loop::<L, false>` / `frames_loop_mono::<L, false>`;
* candidate: the same ramp prefix, then the prototype `settled_main` / `settled_main_mono`.

Coverage:

* **Modes:** 1, 3, 5, 7, 9, 11, 13, 15 and 79 (two-pass at 128-frame chunks).
* **Widths:** `f32`, `Simd4` and `Simd8`, dual and collapsed.
* **Input profiles:**
  * clean noise at 0.2-4.0 amplitude;
  * noise with sprinkled hostile words: ±0, subnormals, MIN_POSITIVE, ±inf, qNaN 0x7fc01234,
    sNaN 0xffa00001 and 0x7f800001, ±MAX, 1.8e29 (below BLOCK_LIMIT), -3e30 (above it);
  * all +0 and all -0;
  * tiny levels (1e-6 to 1e-20);
  * exact ±1, ±0.5 and ±0.25 (levels that land exactly on thresholds);
  * subnormal-only input;
  * NaN-free hostile input;
  * sNaN over a quiet level (the dry-identity case).
* **Per-lane parameters:** the domain edges and interior. Threshold -80, 0, -1.4e-45 and random.
  Ratio 1, 20, 1.0000001 and random. Knee 0, 24, 2.8e-45, 1e-40, 1e-38 and random. Attack 0.1
  and 200. Release 5 and 5000. Makeup ±24, 0, 3 and -12. Mix 0, 1, 0.999, 0.99999994 and random.
  In 60 % of seeds, mix = 1 on every lane (so the wet arm is taken); 40 % of those also have
  makeup 0 (dry identity).
* **Link and bypass:** DualMono, Maximum and Average. Bypass is on in 20 % of seeds and toggled in
  3 % of blocks.
* **Automation:** `set_parameter_target` on random lanes and channels in 25 % of blocks. Ramps
  therefore cross block boundaries and end mid-block, so the settled body starts at arbitrary
  offsets, including non-chunk-aligned ones (e.g. "frames 81 ramping 18"). There is a
  discontinuity reset in 2 % of blocks and a full reset in 1 %.
* **Sample rates and frame counts:** 44.1, 48 and 96 kHz. Frame counts `{1,7,31,32,33,63,64,65,97,127,128}`
  or random in 1..128.

After each block the harness compares:

* every output word by bits, with NaN payloads relaxed **only** when mode bit 2 (wet) is set;
* every state word by bits: the recursive word, all 8 coefficient words, and all
  current/target/step/remaining fields of the 7 parameter ramps and 2 rate ramps;
* the `finish_channel` masks, then every output and state word again after `finish_channel`, now
  strictly;
* that every NaN-payload difference falls in a block that `finish_channel` rejected.

Results. There were 0 failures anywhere.

* **Native dev:** 8 seeds x 120 blocks, 54 configurations.
* **Native release:** 150 seeds x 120 blocks. That is 18,000 blocks per configuration, about
  15,000 of them with a settled body, 2,300-4,900 taking the wet arm and 4,700-5,200 taking the
  DualMono arm. `Simd8` dual alone compared 35.6 M words per mode.
* **Wasm `simd128` under Node 22.23.2 (V8 12.4, `--no-liftoff`),** `Simd4` and `f32`: 60 seeds
  x 120 blocks.
* **NaN payloads:** modes without the wet bit (1, 3, 9, 11) showed **zero** payload differences on
  every target, so #981, #983 and #984 are bit-exact including NaN. With the wet bit, the payload
  differences (sNaN quieted by `x * 1.0`) occurred **only** in blocks that `finish_channel`
  rejected: 0 in accepted blocks on every target.
* **The inlined, shipped shape** (S15, section 3) was re-verified the same way, natively (release)
  and in wasm.

**Existing gates with the prototype forced on.** The `MODE` default was set to 15, 13, 5 and then
1, and the whole `cargo test -p compressor` suite ran in dev and release: 86 passed and 0 failed
in each of the 8 runs.

**Console digests** (`compressor_diag digests`). All 30 rows (15 workloads x `Simd8`/`Simd4`
dispatch) are identical in modes 0, 1, 5, 13 and 15, and in S15, and they equal the values quoted
in #981 gate 4 and #985 gate 4. Caveat for the briefs: native "`Simd4` dispatch" runs the
compressor as unbanked `f32` instances (`lib.rs:800`), so these rows never exercise the `Simd4`
bank. The fixture is DualMono, mix = 1, no automation and no NaN, so the digests exercise exactly
one arm. They are weak evidence; the kernel differential is the real gate.

### 1.2 #983: is the recurrence's input independent of its own output?

Yes.

* **The target is feed-forward.** Frame k's target is `curve_target(link(|x_k|), coef)`
  (`kernel.rs:316-356`). It is a function of frame k's input and of coefficients that are
  constant over the settled body: `ramping = min(max_remaining, frames)` (`kernel.rs:416-419`)
  guarantees every parameter ramp, and therefore every slaved rate ramp, has finished. No
  `gain_reduction_db` feeds the target.
* **There is no feedback detector.** The compressor is feed-forward only (`lib.rs:1`, "The launch
  feed-forward peak compressor"; `LinkMode` has no feedback mode). `Sidechain` and `Silent` never
  reach the new body.
* **In-place I/O is safe.** Pass 1 reads the chunk's inputs before pass 2 overwrites any of them,
  and pass 2 reloads x_k before storing y_k. Rust's `&mut` rules out left/right aliasing.
* **Evidence:** mode 9 (two-pass without the wet arm) is bit-exact including NaN payloads on
  every target.

### 1.3 #982: is the admission exact, and re-checked?

**The algebra.** With `bypass = false` and `mix == 1` on every lane, `wet_identity` is all-true
and `dry_mix_zero` all-false, so `gain_mix` (`kernel.rs:372-385`) returns
`select(dry, x, x*g)` with `dry = (smoothed == 0) & (makeup == 0)`.

* Whenever `dry` holds, `smoothed` is `+0`, because `flush` clears -0 (`lane/src/lib.rs:133`),
  and `+0 + ±0 = +0`.
* `fast_gain_from_db(+0) = 1.0` exactly. This is structural: `f = +0`, and the Horner term
  vanishes.
* `x * 1.0 == x` for every non-NaN x. That includes subnormals, because FTZ/DAZ are pinned clear
  at every native render entry (`lane/src/fpenv.rs`) and wasm has no flush mode.
* The **only** difference is an sNaN x, which comes out quieted.

**Admission and re-checking.** The arm is decided per block, after the ramp prefix, from a
freshly loaded `Coef`. Mix automation, resets and restores are therefore seen at the next block.
`bypass` is read per block. The arm is taken exactly when mix is bit-for-bit 1.0 (a finished ramp
snaps to its target exactly) and never on 0.99999994.

**Why the sNaN difference is invisible.**

* Both production callers apply `finish_channel` before the output leaves the effect: `lib.rs:522`
  then `:544`, and `lib.rs:581` then `:597`.
* `check_block`'s `|x| < 1e30` rejects every NaN, zeroes the whole channel and resets its state
  (`bank.rs:80-89`, `:264-272`).
* The mask depends on NaN-ness, not on the payload.

So the effect-boundary output, state, reports and masks are identical. Differential: 0 NaN-payload
differences in accepted blocks.

**For the owner.** The frozen 013 identity ("when `G==0` and makeup is positive zero, return `z`
exactly for any mix", BRIEFS/013 line 181) now holds at the effect boundary, not at the kernel
word. It is the one relaxation in the six briefs. The caller's definition of class A includes NaN,
so this needs an explicit owner acknowledgement. It cannot be avoided: `x * 1.0` quiets an sNaN
on x86 and in V8, and re-adding the select costs 3 of the 9 saved operations per channel-frame.

### 1.4 #984: `abs` alone equals `link_frame` under DualMono

* **The mask.** `Invariants::new` sets `linked` to the all-zero mask for DualMono
  (`kernel.rs:300-304`).
* **The select.** `link_frame` returns `L::select(linked, combined, magnitude)`
  (`kernel.rs:336-337`), and `select` is bitwise per lane on every backend:
  * scalar: `(a & m) | (b & !m)`;
  * x86: `blendv` on the sign bit;
  * wasm: `v128.bitselect`.

  With `m = 0` the result is exactly `magnitude`, which is `source.abs()` with `source = main`
  for `Detector::Main` (`:323-329`).
* **The arm.** It computes `main.abs()`: the same sign-bit operation on the same operand. The bits
  are therefore equal for every input, NaN included, and no select ever inspects data.
* **Stability.** `link` is `metadata.link_mode`, which is prepared and part of the program key, so
  it cannot change within a plan.
* **Evidence:** mode 11 (the DualMono arm without the wet arm) is bit-exact including NaN on every
  target.

**For #985.** In the collapsed body, Maximum is *also* bit-equal to `abs`. D8 `max(|m|, |m|)`
returns the second operand, `|m|`, on every backend. Average is not: `0.5|m| + 0.5|m|` loses
the smallest subnormal. So the brief's restriction to DualMono is correct but conservative. Do not
list "Maximum takes the arm" as a red mutation: it would stay green.

## 2. The three "can never act" clamps (for the owner)

`clamp_probes()` in `verify_diff_module.rs`; output in `vc-release-diff.log`.

1. **The reduction clamp `.max(-100).min(0)` (`kernel.rs:353-355`) CAN act. Both halves are
   reachable with legal parameters.**
   * **`max(-100)` swallows a NaN target.** The trigger is a knee in `(0, ~1.5e-39]`, such as
     2.8e-45 (`0x00000002`):
     * `parameter_value_valid` accepts it (`params.rs:128`: finite and in `[0, 24]`), and so do
       initial values and automation.
     * `GainComputerCoef::new` computes `1.0 / (2.0 * knee)` (`dynamics.rs:69`), which overflows
       to `+inf`.
     * Whenever `d = level - T` falls in `(-W/2, W/2]`, the knee arm (`dynamics.rs:120`)
       computes `(v*v = 0) * inf = NaN`. A concrete case is threshold 0 and an input sample of
       exactly ±1.0 (`fast_level_db(1.0) = +0`).
     * The clamp maps the NaN to a -100 dB target. Probe: after 8 frames at 1.0 input the output
       is 0.8267 and the recursive word -1.65 dB, heading toward -100.
     * Without the clamp the NaN would reach the recurrence and the output, and the block would be
       rejected. That is different output bits, so removing the clamp is not class A. It is also
       **a latent DSP defect in its own right:** an "almost hard" knee makes every sample that sits
       exactly on the threshold drive the gain toward -100 dB. `multiband-compressor` shares
       `GainComputerCoef`. It deserves its own issue: treat a knee `<= f32::MIN_POSITIVE` as a hard
       knee at design time, which is a class-B / owner change.
   * **`min(0)` clamps a positive delta during a ratio ramp toward 1.0.**
     * Linear-64 accumulation lands on R = 0.99999994 before the snap. It did so in 2,450 of
       2 million random ramps, for example from `0x3f800363` at step 61.
     * `design_lane` then gives `1/R - 1 = +1.19e-7`, so above the threshold `delta > 0`, and
       `min(0)` clamps it.
     * Removing the clamp moves bits in the ramping prefix, because `curve_target` is shared.
   * **`-0.0`.** The diagnosis's "`-0.0` needs a knee arm" misses the above arm. A subnormal
     threshold (-1.4e-45) at level 0 gives `d * (1/R - 1) -> -0.0`. This case alone is harmless:
     the ballistic produces identical state and output for a -0 or +0 target.
2. **`fast_exp2`'s `[-126, 127]` clamp (`fast_db.rs:171`) is unreachable from the compressor, but
   only because the reduction clamp stays.** The payload restore pins the recursive word to
   `[-100, 0]`, normal or zero (`state.rs:68`). The one-pole is a convex update, and makeup is in
   `[-24, 24]`. So the argument lies in `[-20.7, 4.0]`.
3. **`fast_log2`'s `max(MIN_POSITIVE)` (`fast_db.rs:196`) is unreachable from the compressor.**
   The kernel's D8 `max(1e-8)` (`kernel.rs:349`) returns 1e-8 for a NaN operand and floors
   everything else to `[1e-8, +inf]`.

**Both 2 and 3 live in the shared sealed tier.** gate-expander, transient-shaper and
multiband-compressor also call it, and its documented contract is "a NaN in gives `2^-126`, never
NaN or inf" (`fast_db.rs:102-108`). They can only be dropped through new compressor-only sealed
entry points: a seal change, so an owner ruling. The replica measured about -2 % native and -5 %
V8 for the reduction clamp, and that one is not removable.

## 3. Measured savings (reproduced)

Four binaries per target:

* **B0:** the harness patch only, i.e. the true unmodified kernel.
* **P:** the recorded prototypes, `#[inline(never)]`, mode switch.
* **S1:** the #981 shape, `#[inline(always)]`.
* **S15:** the final #981-#985 shape, `#[inline(always)]`, four `DM x WET` instantiations. This
  is what the briefs mandate.

In every binary, mode 0 is `frames_loop`.

### 3.1 Native `Simd8`, console isolate (compressor-only minus builtins-only, 64 tracks, us/block)

| arm | run 1 (load 9-10) | run 2 (load 9-11) |
|---|---:|---:|
| B0 base | 40.2 | 39.9 |
| P mode 0 / 1 / 5 / 13 / 15 | 39.8 / 40.2 / 34.8 / 25.4 / 23.2 | 39.4 / 39.9 / 34.4 / 26.3 / 22.9 |
| S1 (#981 shipped shape) | 41.4 (+3 %) | noisy (load spike) |
| **S15 (shipped final shape)** | **23.4** | **23.7** |

The claim of 39.3 to 23.0 (-41 %) is **reproduced**: 39.9-40.2 to 23.4-23.7, which is -41 %.

### 3.2 Native `Simd4` dispatch (unbanked `f32` instances)

| arm | us/block |
|---|---:|
| B0 | 314.9 |
| P mode 1 | 244.5-245.2 |
| S1 | 251.4 |
| P mode 15 | 86.3-87.3 |
| **S15** | **81.5-82.1** |

S15 is -74 %.

### 3.3 Kernel, one bank-block (load 8-9, hold 6)

**Native `Simd8`:**

* `real_kernel`: B0 4.90-4.99 us, S15 2.69-2.70 us (-45 %).
* `real_bank` (the public contract): 4.96-5.01 us to 2.87 us (-43 %).

**V8:**

* dual: B0 4.65-4.67 us, S15 3.34-3.44 us (-27 %).
* mono: 3.47 us to 1.68-1.69 us (-51 %).

**Mono, native (hold 3, at high load):** 3.07 us to 1.69-1.78 us.

**Hygiene alone on mono:** +19 % natively (P and S1 both 3.66 us against 3.07). This confirms
#985's "land all four together".

### 3.4 V8 console isolate (Node 22.23.2, TurboFan only)

| arm | isolate us/block | compressor-only p50 |
|---|---:|---:|
| B0 (two clean runs) | 81.6, 81.6 | 132.6-132.7 (stable, fast mode) |
| P mode 0 / 1 / 5 / 13 / 15 (clean run) | 82.2 / 72.7 / 64.0 / 62.0 / 59.4 | |
| S1 (#981 shipped shape) | 74.4-75.1 (**-8 to -9 %** against B0, not the -12 % claimed) | |
| **S15** | **60.5-60.8 (-26 %)** | 111.1-112.1 |

The claim of "about -27 %" is **reproduced** against the true base.

**The in-binary mode 0 is not a clean base under V8.** In S1 and S15, mode 0 ran consistently in
V8's slow mode: 152.5-155 us p50, isolate 101.5-103.9. Against that, S15 would look like -41 %.
The kernel shows the same effect: `real_kernel` in mode 0 inside S15 is 5.9-6.0 us, against 4.65
in B0 (+27 %).

**Why it does not matter for the product.** The retained `frames_loop::<Simd4, false>` is only
reachable for `Silent`/`Sidechain` detectors, and banks always use `Main` (`lib.rs:1132-1135`;
connected sidechains never bank). The `f32` sidechain path in wasm was not measured.

**The bimodality.** The unmodified B0 was stable at 132.7 us in both clean runs. 171-183 us
appeared only in a run at load 27, where the control row was also inflated.

### 3.5 The ramping prefix is not regressed by the inlined body

**Natively,** the ramp rows (which run in mode 0) are +0.5 % in S15 against B0.

**Under V8,** with the new settled body active in the ramp rows (hold 7):

* all lanes: 12.68 to 12.09 us;
* one lane: 9.51 to 8.90 us.

That equals the settled-half saving, so the prefix itself did not move.

### 3.6 Console mono row (#985)

| | B0 | S15 | change |
|---|---:|---:|---:|
| native | 71.9-72.2 us | 64.0-64.8 us | -10 to -11 % |
| V8 | 177.1-179.4 us | 148.1-149.1 us | -16 to -17 % |

The digests are unchanged, and the claim is **reproduced**.

### 3.7 Code size

* The console guest's wasm code section grows by 20.8 KB (+0.8 %) from B0 to S15, and by 5.2 KB
  for S1.
* `process_block<f32x4>` grows from 1,926 to 4,232 disassembly lines.
* Exactly **one** `process_block<f32x4>` and one `process_block_mono<f32x4>` symbol remain in S15,
  so rule 3 holds. The P build outlines 20 kernel bodies and would break the roster's intent.

## 4. Per-issue findings and exact amendments

### #981: brief with amendments

1. **Gate 5 M4 is not red. Proven:** with right before left in the one-pass body and `MODE=1`
   forced, all 86 tests and the differential pass. The two channels' updates are independent,
   exactly the argument #983 relies on.
   *Replace with:* "M4: the right channel's `one_frame` receives `&mut gl` (the left recursive
   word). Gate 1 goes red." Also drop the invariant wording "left, then right" as a
   requirement: channel order is unobservable.
2. **Research claim, line 39.** "the unbanked scalar path (every sidechain-connected
   compressor, ...)" is wrong. Connected sidechains take `Detector::Sidechain`/`Silent`
   (`lib.rs:405-418`) and keep `frames_loop`, so they gain nothing.
   *Replace with:* "unbanked `Main`-detector instances (the native `Simd4` dispatch row and scalar
   tails)." The same fix applies to COMPRESSOR-DIAGNOSIS.md:372.
3. **Gate 1** must include a non-zero `start` (a 1..64-frame ramp prefix, as `process_block`
   produces), so that M2 is caught by gate 1 itself and not only by `partition`.
4. **V8 figure.** "-12 %" is the `#[inline(never)]` prototype. The mandated `#[inline(always)]`
   shape measured -8 to -9 % (B0 81.6 to 74.4-75.1) and +3 % natively. Say so.
5. **Note for the implementer:** under V8, the inlined body makes the retained
   `frames_loop::<Simd4, false>` about 27 % slower. It is unreachable for banks. Record it; do not
   chase it.

### #982: brief with amendments

1. **Gate 6 M3.** "Gate 2 and `oracle` go red" is wrong about `oracle`: it stays green. Its mix-1
   case has makeup 0, and its makeup-3 case has mix 0.7.
   *Replace with:* "Gates 1 and 2 and `cross_target` go red". `cross_target` does go red, because
   its `f32` lane 7 has mix 1 and makeup 6.
2. **Invariants.** Add: "The relaxation depends on every production caller (`Instance::render`,
   `render_mono`) applying `finish_channel` before the output leaves the effect. A new caller that
   exposes kernel output must use the general law." Also record the owner's acknowledgement of
   the frozen-013 identity point (§1.3).
3. **Naming.** Rename `all_lanes(mask) -> bool` (for example to `every_lane`).
   `lane::kernels::builtins::all_lanes<L>() -> Mask` already exists with a different meaning.

### #983: brief with amendments (minor)

1. **Gate 1** must drive `start = ramping` in `{1, 18, 40}`, so the first chunk is not aligned.
2. **Copy-rule justification.** "No in-place form exists: pass 2 needs both `x_k` and
   `target_k`, and writing targets into the plane destroys `x_k`. The scratch is 2 KiB
   (`Simd8`), is zero-filled once per settled call, and is never audio."
3. **Merge #984 into this issue.** #984 is five lines inside this pass 1, and its V8 safety
   exists only there.

The gains are reproduced: native 34.4-34.8 to 25.4-26.3 us, V8 64.0 to 62.0 us.

### #984: brief as-is, or merged into #983

The proof holds (§1.4), the gates are correct, and the gains are reproduced: native 25.4-26.3 to
22.9-23.2 us, V8 62.0 to 59.4 us.

### #985: brief with amendments (minor)

1. **M2** ("the body reads the right plane") cannot be expressed, because `process_block_mono`
   has no right plane.
   *Replace with:* "the outer loop uses `chunks_exact`, so the short last chunk is skipped. Gate 1
   (33 and 97 frames) goes red."
2. **Optional:** Maximum may also take `abs` in the mono body (bit-equal, §1.4). Never list it as
   a red mutation.

### #986: do not brief as written

* **The choice cannot be made from the step-1 data.** Step 1 measures today's cost only, so "one
  of (a)/(b)/(c), whichever the measurement ranks first" leaves an unranked design choice to a
  small-model implementer. It also invites tuning after timing.
* **Split it:**
  * a measurement slice (frozen `bench_ramp` threshold arms, MQ-2 protocol);
  * a diagnosis that prototypes (a), (b) and (c) under the lock, as #981-#985 were;
  * then one decided implementation brief.
* **Oracle.** Require that `frames_loop::<L, false>`, the oracle of #981-#985, stays unchanged:
  put the ramping body in its own function.

The ramp costs are reproduced (native, one bank-block):

| row | us |
|---|---:|
| settled | 4.96 |
| one lane, threshold | 8.74 |
| all lanes, threshold | 13.22 |
| attack | 10.64-10.69 |
| makeup | 12.31-12.37 |

## 5. Gates, policy and order

* **Allocation:** stack-only scratch. `conformance`'s `process.allocation` passed in every forced
  mode.
* **Lane policy:** no `wide` and no intrinsics; the code is generic over `L: Lane`.
* **Wasm rule 3:** holds with `#[inline(always)]` (§3.7). The briefs are right to forbid
  `inline(never)`.
* **Fast-dB seal:** the #982 contract (one `applied_gain` carrying X2) keeps eight crossings. The
  prototype's `output` fn had a second X2 `#[expect]`, which must not survive.
* **Merge grouping:** implement #981, #982 and #983+#984 as sequential class-A slices, then #985,
  on one batch branch.
  * #981 alone is +3 % natively.
  * #983 without #982 regresses V8 (diagnosis mode 9).
  * The DualMono arm outside the two-pass body regresses V8.

  So land and benchmark them only as a set.
