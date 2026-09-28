# EQ: refuse elision on a restored subnormal integrator in the stationary cascade (class-A gap)

## Problem

The stationary EQ cascade elides dead (identity) sections on admitted blocks. Its leg (c) admits a live section whose integrators are finite and not `-0.0` (`section_state_is_finite_without_negative_zero`, `crates/parametric-eq/src/lib.rs`, used by `cascade_sections` and `cascade_sections_mono`). That admits a **restored subnormal** integrator, and the `-0.0` induction behind elision assumes kernel-written states (the kernel never writes a subnormal; it flushes below `FLUSH_EPS`).

Found by the #1005 implementer and reproduced on batch code `a1fcab3d`: one live high shelf at `m0 = m2 = 0.5` (gain `-6.0206` dB as an `f32`), restored `ic1 = ic2 = -2^-149`, input `-2^-149`, every other section dead. The elided cascade renders `0x80000000` (`-0.0`); the full cascade renders `0x00000000` (`+0.0`). The live section emits `-0.0` on its first frame, and an elided identity after it passes the `-0.0` where the executed identity would write `+0.0`.

It needs a restore payload carrying a subnormal integrator, so it is not reachable from rendering alone; it is inaudible (a signed zero), but it breaks the engine's class-A guarantee that elision renders the full cascade's bits.

## Smallest closable slice

Close it in one of the two ways the finding names, whichever is simpler and cheaper at render:

1. tighten the stationary leg (c) to #1005's `section_state_is_flush_shaped` (every integrator of a kept section is `+0.0` or finite with `|x| >= FLUSH_EPS`), so a restored subnormal refuses the block, the refused block flushes the words, and the next block engages; or
2. make `restore_track` refuse (or flush to `+0.0`) a non-zero integrator below `FLUSH_EPS`, if restore semantics allow it (check the effect contract's restore rules first; a restore must not silently change state it accepted before without a documented reason).

Coordinate with #998 (which caches leg (b) and is in review) and #1005 (which added `section_state_is_flush_shaped` for the ramping lists): land after whichever of them merges first, and reuse #1005's helper.

## Objective gates

1. A test pinned on the batch head reproduces the gap (elided `-0.0` against full `+0.0`) and passes after the fix, at `f32`, `Simd4` and `Simd8`, dev and release.
2. The #977/#979 differentials and the #1005 differential stay bit-identical; every `WORKLOADS` digest is unchanged.
3. Mutation: reverting to the old leg (c) turns gate 1 red.
4. Timing (under the timing lock, built first outside it): the shipped `host_web.wasm` one-band, two-band and builtins isolates, and native `Simd8`/`Simd4` `eq_only`, no slower than the batch head; `scripts/run-wasm-gates.sh` (spill gate) green, and the V8 listing scan over all EQ loops, masked included, shows no new carried slot.

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1015-eq-stationary-subnormal`, code `c0514d42` on the batch head
`94690a84` (which carries #1005). Option 1, as Sol ruled. Host: AMD EPYC 7313P, rustc 1.97.1,
Node 22.23.2 (V8 12.4). Every timed command held the shared lock, was pinned with `taskset -c 31`,
and printed its load average; every arm is a clean build.

### What changed

* `cascade_sections` and `cascade_sections_mono` take leg (c) from #1005's
  `section_state_is_flush_shaped`: every integrator of a kept section is `+0.0` or finite with
  `|x| >= FLUSH_EPS`. That is the old leg (finite, not `-0.0`) plus the refusal of a tiny restored
  word. The old predicate had no other caller and is removed. The stationary and ramping lists now
  share one gate.
* The proof on `cascade_sections` states where the high-shelf case needs it (`ic1` is `+0.0` or at
  least `FLUSH_EPS`), and the leg (c) comment and the `section_state_is_flush_shaped` doc give the
  counterexample.
* Engagement is unchanged for anything the kernel wrote: `flush` leaves only `+0.0` or
  `|x| >= FLUSH_EPS`. A restored tiny live word refuses one block, which runs every section and
  flushes it, and the next block engages.
* One existing test moved its seeds: `elision::an_elided_cascade_is_the_full_cascade_bit_for_bit`
  seeded live sections with `1e-40` / `-1e-41` to show engagement with a non-zero state. Those
  seeds now refuse, by design, so they are `1.5e-20` / `-1.25e-20`, kernel-writable words just
  above `FLUSH_EPS`. The subnormal case is `stationary_subnormal`'s.

### Gate 1: the reproduction

`stationary_subnormal` (in-crate): one live high shelf at `m0 = m2 = 0.5` (checked), every other
section dead, `ic1 = ic2 = -2^-149` on the last lane, input `-2^-149` there.

* `a_restored_subnormal_live_state_renders_the_full_cascade_bits`: the stationary cascade against
  the full per-section cascade, dual and collapsed, output and integrators by bits.
* `…_through_the_contract`: the payload restores through `restore_track_state_payload` (so the
  shape is reachable), then `process_bank` / `process_bank_mono` (the scalar instance: `render` /
  `render_mono`) against the full cascade on an identically restored copy.

On the batch head both fail at `f32`, `Simd4` and `Simd8`, dual and collapsed, in dev and in
release: every row reads "rendered 80000000 where the full cascade rendered 00000000". After the
fix both pass, dev and release.

### Gates 2 and 3: differentials, digests, mutation

* `cargo test -p parametric-eq` dev and release, each with and without `test-support`: 12 binaries
  ok. That includes the #976/#977/#979/#999 pinned scenarios in `tests/bank.rs`, the in-crate
  `interleave_identity`, `elision` and `boundary_fold` differentials, and #1005's
  `ramping_elision` differential and structural gates. `cargo test -p console-workload` dev and
  release: all ok, every digest unchanged (`chain_shape`, `eq_ramping_scenario`, `paired_spans`).
* Mutation: the old leg (c) in both lists, the dual only, or the collapsed only turns both
  reproduction tests red at every width, on the matching rows, in dev and release
  (`tests/MUTATIONS.md`).
* `cargo clippy --workspace --all-targets -- -D warnings`, `--all-features` on the two crates,
  `cargo fmt --check`, `RUSTDOCFLAGS=-D warnings cargo doc -p parametric-eq --features
  test-support`, and the realtime, lane, env-vocabulary, EQ-render-contract and workspace policy
  scripts: clean.

### Gate 4: timing and loops

The module (`9b77248e…`, 3,501,181 bytes, +107 over the batch head's 3,501,074; the same bytes as
the timed module): `KERNEL_ROSTER` unchanged (EQ dual 672 / 0, collapsed 336 / 0, kernels 15, `f32x4`
arithmetic 14,137 both), render callgraph unchanged, `check-web-audioworklet.sh` passes,
`run-wasm-gates.sh` ok. **All-loop V8 scan** (every innermost EQ loop of both `process_bank`
functions, masked included): every loop has base's instruction count and carried slots; the
listings are 12 (dual) and 3 (mono) lines longer outside the loops. `DIGEST=150` on the one-band,
two-band and mono subjects, all seven arms: all identical.

**V8** (`web_auto.mjs` from #1005's evidence plus a two-band document, settled arms, 6 rounds x
500 blocks, three launches alternating module order; load 4.8 -> 4.4), µs, base -> change:

| row | launch 1 | launch 2 | launch 3 | mean |
|---|---|---|---|---|
| one-band isolate | 22.11 -> 21.57 | 21.93 -> 22.29 | 22.70 -> 22.15 | 22.25 -> 22.00 |
| two-band isolate | 33.40 -> 33.37 | 34.57 -> 34.73 | 35.41 -> 34.69 | 34.46 -> 34.26 |
| builtins row | 49.62 -> 50.13 | 50.18 -> 49.98 | 50.05 -> 50.05 | 49.95 -> 50.05 |
| mono console isolate | 98.59 -> 98.19 | 97.88 -> 98.27 | 98.37 -> 97.62 | 98.28 -> 98.03 |

**Native** (settled rows, isolate = row - builtins-only in the same round; `Simd4` binds four-lane
EQ and compressor banks through a scratch-only switch in both builds), µs, base -> change:

| row | run 1 (A B B A, 6 x 800; load 5.3 -> 5.0) | run 2 (A B B A A B, 12 x 1600; load 4.5 -> 4.9) |
|---|---|---|
| `eq_only` isolate, `Simd8` | 10.25 -> 10.47 | 10.51 -> 10.13 |
| `eq_only` isolate, `Simd4` | 18.43 -> 18.54 | 18.83 -> 18.81 |
| mono console isolate, `Simd8` | 43.83 -> 43.68 | 43.70 -> 43.88 |
| mono console isolate, `Simd4` | 50.27 -> 50.67 | 50.99 -> 51.25 |

No row is slower beyond the run-to-run spread (±0.3 µs natively, ±0.8 µs under V8), and the
differences change sign between runs. The builtins-only rows, whose code this does not touch,
move by as much as the EQ rows.

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-28. I reviewed code `c0514d42` and evidence `8834a668`, judged merged onto
the new batch head `901a1c88` (which adds #1007). `lib.rs` merges cleanly, and its diff is
line-for-line the branch's. `tests/MUTATIONS.md` has an append conflict; I resolved it by keeping
both sections, #1007's and then #1015's. Every arm was built from `git archive` in its own
scratch target with `CARGO_INCREMENTAL=0`.

**Soundness (class A).** `flush` keeps `x` exactly when `|x| >= FLUSH_EPS` (NaN included) and
otherwise writes `+0.0`. Every other writer (reset, §4.4, prepare, the mono disengage copy) writes
`+0.0` or a kernel word. So the finite words the kernel can write are exactly
`lane_is_flush_shaped`'s set. The non-finite ones were refused before and still are. No
kernel-written state is newly refused. With both integrators flush-shaped, the high-shelf case
closes:
- `m2 * v2 = -0.0` needs `v2 = -2^-149`.
- If `|ic2| >= FLUSH_EPS`, `ic2 + d2` is a multiple of at least `2^-91`, so it cannot be
  `-2^-149`. Hence `ic2 = +0.0`, `v3 = v0 = -2^-149`, and `a2 * v3` underflows.
- Then `v1` is `+0.0`, or at least `FLUSH_EPS * 2^-24` in magnitude, so `m1 * v1 != -0.0`.

By the same analysis the hazard needs both integrators tiny.

**My fuzz** ran in-crate on the merged tree. It compares `render` / `render_mono` (so §4.4 and the
silent fixed point are in both arms) against a no-elision oracle: forced refusal on stationary
blocks, `process_block` on ramping ones.
- **What varies:** every layout, including a -6.0206 dB shelf and live/dead mixes per lane;
  four rates; restored `ic1`/`ic2` drawn independently from 17 classes (`+-0.0`, `+-2^-149`,
  subnormals, tiny normals, `+-FLUSH_EPS` and its bit neighbours, moderate, large, the ceiling
  and one bit above it, `+-f32::MAX`, non-finite); mid-run restores; ramps (#1007's select
  writes); hostile, silent and `-2^-149` inputs; and 1 to 128 frames.

| arm | scenarios | blocks | elided | differing |
|---|---:|---:|---:|---:|
| this leg (c), release | 1,800,000 | 14.4M | 5.16M | 0 |
| this leg (c), dev (debug asserts) | 60,000 | 480k | | 0 |
| old leg (c), release (non-vacuity) | 600,000 | | 2.14M | 876, all `80000000` vs `00000000` |

**Refusal cost.**
- **Re-engagement:** checked in scratch at every width, dual and mono, for a high shelf, a bell
  and a low pass. A restored `+-2^-149`, `+-1e-30` or `FLUSH_EPS`-minus-one-bit word refuses one
  block (six kept), and the next block engages. That holds with noise, silence or `-2^-149`
  input.
- **Admitted at once:** `+-FLUSH_EPS`, `+-f32::MAX` and `+0.0`. `-0.0` still refuses.
- **Timing:** the refused block is one full-cascade block. On one live band, under the lock and
  pinned (load about 8, descriptive), it costs +3.4 us per `Simd8` dual bank (1.14 -> 4.6),
  +2.1 us mono, +3.0 us `Simd4` and +3.7 us scalar. It is paid once per restore that carries a
  tiny live word.

**Gates on the merged tree.** All green:
- `-p parametric-eq`: 126 passed and 3 ignored, dev and release, with and without
  `test-support`.
- Also passing: lane 70, effect-runtime 90, console-workload 62 (dev and release),
  builtins-compiler 79, wasm-gates 9, bench floor 9 and rack 55.
- clippy `-D warnings` (workspace, plus `--all-features` on the two crates), fmt, and doc
  `-D warnings`.
- The lane, realtime, EQ-render-contract, env-vocabulary, unfused-seal, workspace and
  effect-runtime policy scripts, and `test-console-benchmark.sh`.
- `run-wasm-gates.sh` passes. The spill gate reads dual tail 109, mono pair 78 and mono tail 53,
  identical to the head.
- **Web check:** `check-web-audioworklet.sh` fails its artifact pin on the batch head as well as
  on the merged tree (mid-batch; the batch repins at its boundary). With a local pin, it passes on
  both.
- **Roster and rule 3:** identical to the head (15 kernels, `f32x4` arithmetic 14,139, EQ
  672/336, scalar 0). The module grows 107 bytes.
- **Wasm diff:** only the three EQ bodies change, and no hunk lies inside an arithmetic loop of
  either `f32x4` bank body.
- **Digests:** all 17 `WORKLOADS` digests are identical to the head's.
- **Mutations:** 1015-M1d and M1m replicated red.

**#1007 interaction:** none. #1007 writes only coefficient, step and target words, and leg (c)
reads only the integrators.

Findings:

1. **LOW (test coverage).** Nothing structurally pins the stationary leg (c)'s tiny band.
   `stationary_subnormal` pins the one hazardous shape through the bits. Nothing asserts that a
   live `1e-30` or subnormal refuses, that `+-FLUSH_EPS` is admitted, or that the next block
   engages. The ramping lists and #979 gate 3 (for leg (b)) each have such a test. As a result, a
   boundary mutation (`>` for `>=`, or refusing subnormals only) survives the suite. Moving the
   seeds keeps what that test proved (engagement and bit identity with a non-zero live state).
   The old seeds never exercised a flush of the seed, because frame 0 is noise or a 1.0 impulse.
   But they should have become that refusal and re-engagement assertion. The gap is speed-only,
   so it does not block.
2. **LOW (stale docs).** The `elision` module doc still says "seeded subnormal-adjacent state".
   It also lists leg (c)'s refusal as only "a `-0.0` integrator in a live one".
3. **INFO (proof text).** The high-shelf paragraph uses `|v3| = 2^-149` without deriving
   `ic2 = +0.0`, and its #1015 sentence names only `ic1`'s bound. `ic2`'s bound is load-bearing
   too (see the soundness argument above).
