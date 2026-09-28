# EQ: write a target's lane with a vector select, and snap ended lanes by mask

Source: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, verified in `VERIFY-AUTOMATION.md`. **The Amendments section at the end supersedes the body wherever they conflict.** Draft names map to issues: automation-5 = #1003, automation-1 = #1004, automation-2 = #1005, automation-3 = #1006, automation-4 = #1007.

Automation follow-up E2 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `49f696c7`;
every `file:line` is `crates/parametric-eq/src/lib.rs` on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, sections 3 ("Target application")
and 5 (E2). The prototype is bit 1 of `parametric_eq::diag::MODE` in
`docs/handoffs/effects-2026-09-27/automation-diagnosis-prototypes.patch`. It is evidence only; do
not commit it.

## Product outcome

Every automated EQ lane receives a prepared target every block. The render thread applies it
(`crates/rack/src/lib.rs:1270` → `apply_target_lane`, `:2148` → `start_ramp`, `:1247`), and
64 samples later it snaps it (`process_section`, `:1376` → `snap`, `:1329`).

Both write one lane of a lane vector with `lane_set` (`:932`): a full-width store, a 4-byte write,
and a full-width reload that the store buffer cannot forward. That is 12 per retarget, 12 more
per lane at the ramp's end, and 18 per stationary hoist (`settle`, `:1157`). It costs about
0.2 µs per target natively and under V8. That is what separates automating every track from
automating one per bank: +22 µs natively for 112 more targets.

This slice writes lanes with `select(one_hot(lane), splat(word), vector)` from a static one-hot
table, and snaps every lane that ended in a segment with one masked `select` per word. Measured on
top of E1, in µs per 64-track block, EQ isolate, Δ against settled:

| arm | native `Simd8` E1 / E1+E2 | native `Simd4` E1 / E1+E2 | V8 E1 / E1+E2 |
|---|---:|---:|---:|
| all 64 | +29.8 / **+18.0** | +33.7 / **+20.2** | +38.7 / **+23.9** |
| 8 of 64 | +8.9 / **+7.5** | +8.7 / **+6.3** | +11.2 / **+9.1** |
| one lane | +1.8 / **+1.6** | +1.5 / **+0.5** | +2.9 / **+2.3** |

Without E1, E2 alone takes `Simd8` all 64 from +88.7 to +76.6. These figures are descriptive
only.

## Invariants

* **Class A.** `select` is bitwise, so every word stored is exactly the word `lane_set` stores,
  `+0.0` steps included. Every output word, coefficient, step, target, `remaining`, identity flag
  and payload is unchanged, at `f32`, `Simd4` and `Simd8`, dual and collapsed.
* The snap of the ended lanes of one segment happens at the same point as today: after the
  segment's kernel, before the next segment. `refresh_identity` runs once per section when any
  lane snapped, as today.
* `lane_get` stays. A narrow load from a wide store forwards.
* Allocation-free. The one-hot table is a `static [[f32; 8]; 8]`. No `unsafe`.

## Interface contract

1. `static ONE_HOT`, `fn lane_mask<L>(lane) -> L::Mask` (a load of the static row, then
   `eq(1.0)`), and `fn lane_put<L>(value, mask, word)`.
2. `settle` and `start_ramp` use `lane_put` for every coefficient, target and step word.
   `start_ramp` stays `#[inline(never)]`.
3. `process_section` builds the ended mask as the `mask_or` of `lane_mask(track)` over the lanes
   that were ramping and reached `remaining == 0` in this segment. For each of the six words it
   then sets `coef = select(ended, target, coef)` and `step = select(ended, 0, step)`.
4. `snap` remains for `restore_track` and any other caller.

## Smallest closable slice

Authorized paths:

* `crates/parametric-eq/src/lib.rs`;
* `crates/parametric-eq/tests/` (extend `automation-2`'s differential file);
* `crates/parametric-eq/tests/MUTATIONS.md`;
* this spec.

## Non-goals

* The rest of the per-target cost: queue pop, decode, `refresh_identity`'s six scans, the
  permitted check. That is about 95 ns per target natively after this change. Skipping
  `refresh_identity` in `start_ramp` is exact (the coefficients do not move there), but it breaks
  a maintenance rule the code keeps deliberately. It is not in this slice.
* E1 and E3.

## Objective gates

1. **Identity.** `automation-2`'s bank differential, run with this change. The prototype ran it
   as modes 2 and 3 at 300 scenarios × 96 blocks per width in release and 40 in dev. Compare
   every output word and payload by bits after every block. Add a unit test that drives
   `start_ramp`, `settle` and a segment snap on every lane of `Simd4` and `Simd8` with words that
   include `+0.0`, subnormals and `f32::MAX`, and compares each vector word by bits against the
   `lane_set` path.
2. **Scenario pinned on base.** `automation-2`'s scenario digest, recorded on `49f696c7`,
   unchanged.
3. **Existing gates unchanged:**
   * the crate's suite in dev and release, including the #144 hoist tests (`restated` must stay
     bit-identical to `quiet`);
   * `console_hoist`'s in-run assertions;
   * every console digest.
4. **Mutations**, each recorded red:
   * one-hot row off by one lane;
   * the snap mask built from `was_ramping` alone, without `remaining == 0`;
   * the snap step set to the target instead of `+0.0`;
   * `settle` without the target word.
5. **Realtime and wasm.** `check-web-audioworklet.sh`: roster unchanged, with no scalar `f32`
   arithmetic added to either EQ `process_bank`. The callgraph closure gains no trap owner (the
   one-hot index is bounded by `W`: use `get` or a `min`, never a panicking index).
6. **No regression through the shipped artifact.** A paired V8 run of `web_auto.mjs eq_gain all`,
   base against change, 6 rounds.
   * The `settled` isolate must not be slower than base by more than 2 %.
   * Record `all_64`. This is descriptive.

## Dependencies

After `automation-2`, which edits the same `process_section`.

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-AUTOMATION.md` and
`verify-automation-raw-timings.txt`.

The slice stands.

* **Reproduced on top of E1**, V8 EQ Δ: 8 of 64 goes +11.4 → **+9.85 µs**, and all 64 goes +38.4 →
  **+24.0 µs**. Natively at `Simd8`, all 64 goes +30.6 → +18.8 µs.
* **Differential:** the bank differential passed at 300 × 96 per width for E2 alone and for
  E1 + E2.
* **Wasm:** the roster is unchanged, and the artifact is 2,762 bytes smaller than E1 alone.

### A1. Unit test (gate 1)

* Add a **`Both`** target, which writes both channels. That is the SDK's shape for a symmetric
  both-channel edit.
* Add the last lane, `W - 1`, of `Simd4` and of `Simd8`.
* `lane_mask` must not be able to panic. Use `ONE_HOT.get(lane)` with a `debug_assert!` and a
  declining fallback, never `ONE_HOT[lane]` as the prototype does.

### A2. Differential

* Run `automation-2`'s amended differential for E2 alone and for E1 + E2: `Both` targets, resets
  mid-ramp and boundary targets.
* The bank output is read after the §4.4 check, so compare it strictly by bits.

### A3. Gate 6, made self-contained, with a mono row

* Use the recipe of `automation-2` amendment A5, with subjects `eq_gain,mono_eq` and arms
  `settled,eight_of_64,all_64`.
* **No regression.** The `settled` isolate must not be more than 2 % slower on either subject.
* **Descriptive.** Record `all_64`.
* **Identity.** `DIGEST=150` must print "all identical".

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1007-eq-vector-lane-writes`, code `6ce112c1` on `d5b7dc6a` (the
#1005 branch with the batch head merged). "Base" below is `d5b7dc6a`, the #1005 kernel. Host: AMD
EPYC 7313P, rustc 1.97.1, Node 22.23.2 (V8 12.4). Every timed command held the shared lock, was
pinned with `taskset -c 31`, and printed its load average. Every arm is a clean build.

### What changed

* **Contracts 1-3.** `static ONE_HOT`, `lane_mask` (a load of the static row, `eq(1.0)`),
  `empty_mask` and `lane_put` (`select(mask, splat(word), value)`). `settle` and `start_ramp`
  write every coefficient, target and step word through `lane_put`; `start_ramp` stays
  `#[inline(never)]` and keeps `lane_get`. A segment's ended lanes snap in `snap_ended`: the
  `mask_or` of `lane_mask(track)` over the lanes that were ramping and reached `remaining == 0`,
  then `coef = select(ended, target, coef)` and `step = select(ended, 0, step)` per word, at the
  same point as before, with `refresh_identity` once per section as before.
* **A1.** `lane_mask` reads `ONE_HOT.get(lane)` behind a `debug_assert!` and declines to the empty
  mask, so a write through it changes nothing and nothing can panic.
* **Deviation: `snap_ended` is `#[inline(never)]`.** Inlined into `process_section`, as in the
  prototype, it gave V8's ramped masked kernel loop (`svf_block_ramped_with_dry_mask`, dual and
  mono) a second carried stack slot in the shipped module (1 -> 2). Out of line, every EQ loop
  carries exactly what it did at base, and the dual masked depth-2 pair carries one fewer (12 ->
  11). It runs once per ramp end, not per frame, and it is not an arithmetic kernel (no `f32x4`
  add, sub, mul or div), so the roster does not see it.
* **Deviation: the per-lane `snap` is unit-test only.** Contract 4 keeps it "for `restore_track`
  and any other caller", but `restore_track` writes with `lane_set` directly and nothing else
  snapped one lane, so a production `snap` would be dead code. It is the oracle's snap.
* **The oracle.** A unit-test-only `LANE_SET_WRITES` switch (default: the shipped `select` path,
  so every unit test exercises it) puts `settle`, `start_ramp` and the segment snap back on the
  #1005 code, kept verbatim as `settle_by_lane_set`, `start_ramp_by_lane_set` and `snap`. The
  differential's arms now carry a kernel path (`RAMPING_LIST`, `LANE_SET_WRITES`) that every
  preparation, target, reset, restore and render runs on.

### Gate 1: identity

* **Against the #1005 kernel** (`select_lane_writes_render_the_1005_bits_*`: the list on both
  arms, `lane_set` against `select`) and **alone** on the batch-head ramping path
  (`select_lane_writes_alone_render_the_lane_set_bits_*`), at `f32`, `Simd4` and `Simd8`, dual and
  collapsed, 300 x 96 blocks per width and body in release and 40 x 96 in dev. It is #1005's
  amended differential: `Both` targets (5,552-107,343 per width and body), resets mid-ramp,
  boundary targets, hostile restores, hostile input. Every output word, report, payload and
  internal word is compared strictly by bits after every block. All pass, with the same ramping
  and elided counts as #1005's own gate (for example `Simd8` dual: 11,316 ramping blocks, 5,584
  elided).
* **Word for word** (`select_lane_writes_match_lane_set_word_for_word`, `f32`, `Simd4`, `Simd8`):
  `settle`, `start_ramp`, a second lane's ramp starting 20 frames later so the two end in different
  segments, the stationary hoist, and a bank-wide ramp ending in one segment, on every lane
  including `W - 1`, with words that include `+0.0`, `-0.0`, `1e-40`, the smallest subnormal and
  `+-f32::MAX`. Every word of the channel is compared after every step. Then a `Both` target (checked
  to be `Both`) is applied to lanes 0 and `W - 1` and rendered across its ramp end, compared by
  bits.

### Gates 2 and 3: pinned scenario and existing gates

`tools/console-workload/tests/eq_ramping_scenario.rs` (pinned on `a1fcab3d`) is unchanged and
green, dev and release. `cargo test -p parametric-eq` dev and release, each with and without
`test-support`: 12 binaries ok, `stationary_hoist` included. `cargo test --release -p
console-workload`: all ok, every digest unchanged. `console_hoist`'s in-run `restated == quiet`
assertion runs only inside the console benchmark, which this attempt does not run; the same premise
is `stationary_hoist.rs`, the hoist step of the word-for-word test, and the mixing preflight's
`restated_eq_only == quiet` under V8 (below). `cargo clippy --workspace --all-targets -- -D
warnings`, `--all-features` on the two crates, `cargo fmt --check`, `RUSTDOCFLAGS=-D warnings cargo
doc -p parametric-eq --features test-support`, and the realtime, lane, env-vocabulary,
EQ-render-contract and workspace policy scripts: clean.

### Gate 4: mutations

All four red (`tests/MUTATIONS.md`): the one-hot row off by one lane, the snap mask from
`was_ramping` alone, the snap increment set to the target, `settle` without the target word.

### Gate 5: realtime and wasm

The module built with the delivery recipe (`7a86a2c4…`, 3,498,302 bytes, 2,772 smaller than base's
3,501,074; the same bytes as the timed module):

* `KERNEL_ROSTER`: `parametric-eq f32x4 dual` 672 / 0 and `collapsed` 336 / 0, each one function;
  kernels 15; `f32x4` arithmetic 14,139 (base 14,137). No scalar `f32` arithmetic in either EQ
  `process_bank`. Render callgraph as base, no new trap owner. `check-web-audioworklet.sh` on the
  assembled seven-file directory: passes.
* `run-wasm-gates.sh`: ok, V8 spill gate ok (dual tail 109, mono pair 78, mono tail 53, none
  carried).
* **Every EQ loop, masked included** (the spill gate's own analysis over both `process_bank`
  functions): identical instruction counts and carried slots to base, except the dual masked
  depth-2 pair, 12 -> 11. No new carried slot.
* V8 output identity (`DIGEST=150`, `eq_gain`, `mono_eq` and the all-six-live stereo and mono
  subjects, all seven arms): all identical. Mixing preflight: all seven digests equal base's.

### Gate 6 and the timing table

µs per 64-track block; Δ against settled in the same run.

**V8** (`web_auto.mjs` as in #1005's evidence, 6 rounds x 500 blocks, load 4.5 -> 3.7):

| arm | `eq_gain` base | change | `mono_eq` base | change | all-six stereo base | change | all-six mono base | change |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| settled isolate | 22.07 | 21.79 | 98.77 | 99.11 | 91.89 | 92.50 | 133.21 | 132.98 |
| one lane | +2.99 | +2.85 | +0.82 | +1.15 | +5.19 | +4.53 | +1.76 | +2.20 |
| 8 of 64 | +11.41 | **+9.97** | +3.99 | **+3.07** | +29.83 | **+27.61** | +13.89 | **+12.94** |
| all 64 | +38.31 | **+24.47** | +19.83 | **+10.36** | +75.49 | **+60.26** | +39.52 | **+29.05** |

Settled is within 2 % on both gate subjects (-1.3 %, +0.3 %). One lane is one target per block,
inside the run-to-run spread either way.

**Browser `console_mixing_automation` arm** (one warmup and two measured launches per module,
alternated; load 3.5 -> 4.5), p50 µs, rounds 1/2. The row rides three EQ tracks, so E2's share is
small:

| module | quiet | restated | automated | paired ramp Δ |
|---|---|---|---|---|
| base | 161.09 / 155.11 | 159.90 / 158.22 | 164.39 / 161.95 | 4.05 / 3.57 |
| change | 155.18 / 153.29 | 159.88 / 156.10 | 163.63 / 159.69 | 3.64 / 3.49 |

**Native** (the #1005 scratch harness, base, change, change, base, 6 rounds x 800 blocks each; load
8.0 -> 8.4; `Simd4` binds four-lane EQ and compressor banks through a scratch-only switch in both
builds). Rows and Δ are the mean of the two invocations:

| arm | stereo `Simd8` base / change | stereo `Simd4` | collapsed `Simd8` | collapsed `Simd4` |
|---|---|---|---|---|
| settled row | 35.22 / 35.17 | 60.23 / 60.06 | 70.17 / 68.74 | 92.88 / 92.40 |
| one lane | +0.94 / +1.30 | +1.29 / +0.96 | +0.12 / +0.36 | +0.28 / +0.66 |
| 8 of 64 | +8.03 / **+7.51** | +9.47 / **+8.00** | +1.93 / +2.04 | +2.57 / **+1.86** |
| all 64 | +30.62 / **+18.32** | +35.58 / **+22.16** | +17.44 / **+10.66** | +17.69 / **+7.61** |

No settled row is slower. The saving is per target: all 64 saves 12-14 µs (stereo) and 7-10 µs
(collapsed) at every width and under V8, and one lane is within the spread.

### For the verifier

* The `snap_ended` placement is the one choice here that the brief did not make; the loop scan
  above is why.
* M1 breaks most EQ unit tests, because preparation settles every lane through `lane_put`.

## Sol attempt 1 verdict: PASS

Sol, 2026-09-28. Judged merged onto the batch head `94690a84` (`git merge-tree`; the merged EQ
`lib.rs` equals `6ce112c1`). The shipped module built with the delivery recipe is `7a86a2c4…`,
3,498,302 bytes, the same bytes as the evidence's timed module; base is 3,501,074 bytes. Every
timed command held the shared lock, pinned to one core, at load 1.6 to 4.6.

**Identity: holds.** A scratch public-surface differential compared the batch head with the merge
by bits after every block, on outputs, reports, payloads and snapshots. It covers the scalar
channel, `Simd4` and `Simd8` banks, and dual, collapsed and mixed bodies. Scenarios include
several targets on one lane in one block (a group cut plus a per-track edit, up to 12 targets
each), symmetric both-channel edits, blocks of 1 to 128 frames (so ramp ends land on a block's
last frame), resets mid-ramp and hostile restores.

* Differences: native release 0 of 140,000 runs, dev 0 of 8,400 (debug assertions on), wasm
  `simd128` 0 of 28,000.
* Ramping-elided counts are equal on both sides: 1,853,311 in release, 112,724 in dev.
* The harness catches a snap that keeps its increment: 14,000 of 14,000 runs differ.
* V8 `DIGEST=150`: `eq_gain`, `mono_eq`, `eq6` and `meq6` × `settled`, `one_point`,
  `eight_of_64` and `all_64`, all identical.

**Gate 6 (A3): holds.** Two paired `web_auto.mjs` runs in opposite module order, 6 × 500, µs per
64-track block.

| subject | settled isolate, base → change | 8 of 64 Δ | all 64 Δ |
|---|---|---|---|
| `eq_gain` | 21.94 → 22.18 (+1.1 %); 22.07 → 22.09 (+0.1 %) | +11.22 → +9.57 | +38.55 → +23.76; +38.54 → +24.45 |
| `mono_eq` | 100.72 → 100.65; 105.38 → 104.67 | +3.79 → +3.19 | +19.90 → +9.55; +19.89 → +9.93 |

* All six bands live: stereo all 64 goes +77.25 → +63.96 and mono +40.67 → +29.71. Their settled
  isolates are -0.6 % and +1.6 %.
* One point per block is within the spread either way.
* Stationary `web.mjs` rows, paired and run twice in opposite order: every isolate is within
  -1.6 % to +1.1 %.

**Gates on the merged tree: pass.**

* `cargo fmt --check`.
* `clippy -D warnings` on `parametric-eq` and `console-workload`, all targets and all features.
* `cargo doc -D warnings`.
* `parametric-eq`: 124 tests in 12 binaries, in dev and release, with and without
  `test-support`.
* `console-workload --release`: 62 tests. `lane`: pass.
* The workspace, realtime, lane, env-vocabulary and EQ render-contract policy scripts.
* `run-wasm-gates.sh`: the V8 spill gate passes; the dual tail, mono pair and mono tail carry
  nothing.
* `KERNEL_ROSTER` is identical, with 15 kernels. The render, meter-poll and command-submit
  callgraphs are unchanged.

**`#[inline(never)]` on `snap_ended`: acceptable.** The render rules ban allocation, locks, I/O,
syscalls and data-dependent unbounded calls. A bounded direct call is none of those. There is at
most one per segment per section, and it contains no loop over frames.

* **Precedent.** `start_ramp` already has this attribute and runs on the render thread. So do
  functions in `compressor/src/kernel.rs` and `true-peak-limiter`.
* **Outlining stands.** The loop census confirms it.
* **Doc comment.** "For the same reason as `start_ramp`" is loose. `start_ramp`'s reason is the
  roster, and this one's is V8 register allocation, which the comment's next sentence says.
* **Trap paths.** The function carries two panicking bounds checks on `section`, as `settle` and
  `start_ramp` do. They are unreachable (`section < 6`) and outside the gated render closure. The
  one-hot index uses `get`, as A1 requires.

**Loop census: the evidence overstates it.** I ran the gate's `analyse` on every innermost loop of
both EQ `process_bank` functions, on the same bytes. What is confirmed:

* The dual masked depth-2 pair goes from 12 carried slots to 11.
* The per-lane snap loops are gone: two in dual, one in mono. Each had 188 instructions and 60
  memory streams.
* The mono function is otherwise unchanged.

What is not: the evidence says every other loop has "identical instruction counts and carried
slots … No new carried slot". That is wrong for one loop.

* **Which loop.** The dual function's per-block leg (c) section scan (stride `0x140`, the -0.0 and
  finite checks, then the flush-shape test).
* **What changed.** It went from 194 to 208 instructions and from 0 carried slots to 1 (one store
  and one reload per section).
* **Size.** It runs once per section per block, not per frame. The worst case is about 0.2 µs per
  64-track block, inside the settled spread above.
* **Status.** This is descriptive, not a failed gate. The record is corrected here. Two tiny scan
  loops also moved by ±1 instruction.

**Dead code: none in production.** `lane_set` still serves `restore_track`, and `snap_section`
still serves `discontinuity_reset`. The per-lane `snap` is now `#[cfg(test)]`, which is right,
since nothing in production snapped one lane. The oracle (`LANE_SET_WRITES`,
`settle_by_lane_set`, `start_ramp_by_lane_set`, `snap`) is test-only. It can go with #1005's
`RAMPING_LIST` switch in a later cleanup; that does not block this.
