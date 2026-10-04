# #1071 attempt 2 verdict: PASS

PASS with two MINORs and two NITs. Commit reviewed: `983ac85bd` (attempt 1 was `1199b53f9`, parent
`41517fc35`). I reviewed both `git diff 1199b53f9 983ac85bd` and the cumulative
`git diff 41517fc35 983ac85bd`. The commit was exported from `/home/bl/misofm/wt-swap-fx` to
`/tmp/claude-1002/v1071/attempt2/`; the export was byte-identical to the commit after my runs. I
ran the probes and mutations in a second export,
`/tmp/claude-1002/v1071/attempt2/verifier-scratch/mut/`. Logs are in `verifier-scratch/logs/`.
The probe files are `mut/crates/soft-clip/tests/zz_probe_{chain,overshoot,nonfinite}.rs`, and the
mutation driver is `verifier-scratch/mutate.py`. The worktree's later `c2808bb04` (#1278 attempt
1) touches neither soft-clip nor this spec.

**MAJOR-1 is fixed.** The effect now restores every state it produces in my tests, including
chained retargets that start from an overshoot. Attempt 1's MINOR-1 (a subnormal gain step was
accepted) and NIT-1 (the `X`/`e` doc) are fixed too. What remains is test coverage: half of the
new acceptance function's decision points can break without any committed test turning red
(MINOR-1, MINOR-2). The code itself is correct.

## The bound checks out

I re-derived the bound in `ramp_current_valid`'s doc, `crates/soft-clip/src/lib.rs:285-296`.
Every set_target uses 64 samples (`lib.rs:636/639`), and the kernel and `LinearRamp` both use
plain f32 `current += step`. So for a ramp from `s` to `t`:

`current - (t - remaining*step) = a + 64b + c`

where:

- `|a| <= ulp(2h)/2`, because `|t-s| < 2h`, even from an overshoot start;
- `|64b| <= 2^-144`, because `b` is only nonzero when the quotient is subnormal;
- `|c| <= 63 * ulp(2h)/2`, because every running value is below `2h`.

Measured against the ramp's own line, the bound does not depend on `s`, so overshoots cannot
build up across retargets. Outward motion is monotone in the step's sign. An outward step needs a
target beyond the edge, and targets are always in range (`convert_parameter` refuses anything
else). So no chain can carry `current` further than one ramp's rounding past an edge. The f64
evaluation is exact except for one rounding near 2^-53. The order of `||` evaluation means the
target and step are validated before the line is computed.

**Empirical check** (`zz_probe_chain.rs`, release build, 40,000 seeds). The probe ran random
chained retargets: targets at the edges, near the edges or anywhere in range, blocks of 1 to 128
frames, and both channels. After every block it took a snapshot and restored it into a fresh
scalar instance. The bank half ran the same automation through an `Eight` bank (AVX2 host) and
took per-track snapshots.

- 1,339,910 of the effect's own snapshots: all restored, and every scalar snapshot was
  identical after the restore.
- 6,369 of them had a current outside the converted range.
- Largest `|current - line|`: 15.75 ulp(2h) for both gains and 7.875 ulp(2) for the mix. The
  tolerance is 64. This matches the implementer's scratch measurement.

## Gates I re-ran in the export

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | 0 |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | 0 |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | 0 |
| `check-workspace-policy.sh`, `test-workspace-policy.sh` | 0, 0 |
| `check-realtime-policy.sh`, `test-realtime-policy.sh` | 0, 0 |
| `check-capi-abi.sh` | 0 |
| `cargo build --release -p audit -p capi && audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations |
| `check-cross-targets.sh` | PASS. Only the #1018 iOS rows, which are expected; soft-clip `memset_pattern16` is 22, at its ceiling |
| `cargo test --locked -p soft-clip -p conformance` | green: soft-clip `allocation` 3/3, `state_roundtrip` 9/9, `randomized` 2 passed plus the pre-existing #1073 ignore |
| `MISO_ENGINE_RANDOMIZED_SCALE=100 cargo test --release -p soft-clip --test randomized` | green in 1.1 s (25,600 near-edge seeds) |
| `cargo test --locked --release -p console-workload` | green: console digests unchanged |
| `conformance_fixtures --check` | 0 |
| `cargo test --locked -p effect-compiler` | green |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh` | all 0 |

I independently confirmed **ARTIFACT CHANGED**. My build of the shipped module has the digest
`94c443c08f93ac6ee5b5f1fe14b7d488d3c92d6a35af2bd2efc4d17e154240ad`, the same one the implementer
reports. Under `docs/RELEASE.md` there is no per-change re-pin, and the commit does not touch the
pin file. That is correct.

Gate 3 holds. The only code changes are to the control-plane decode and its helpers (no
allocation, stack-only). The kernel, ramp and snapshot are untouched. The console and conformance
digests are unchanged.

## Attempt-1 probes re-run against attempt 2

- `probe_overshoot.rs` (four rows): all four snapshots now restore (`logs/probe-attempt1-rerun.log`):
  - mix at a negative subnormal, `0x80000008`;
  - mix at `1.000001`;
  - drive above `gain(+36 dB)`;
  - output below `gain(-24 dB)`.
- `probe_nonfinite.rs`: unchanged, as expected. `inf` stays in `X` with finite output, and the
  effect's own snapshot is refused with `effect.state.history`. The attempt record hands this
  (attempt-1 MINOR-2) to #1278, which is outside #1071's subnormal scope. See NIT-2.

## Mutations (each run against the whole soft-clip suite and my probes, then reverted; lib.rs md5 restored)

| # | Mutation | Committed tests red | Probes red |
|---|---|---|---|
| MA | in-flight current back to the strict range (attempt 1) | all 4 new round-trip tests, near-edge randomized | all |
| ME1 | tolerance `1 * ulp(2h)` | mix-to-one, drive-to-top, near-edge randomized | chain, overshoot |
| ME8 / ME12 | tolerance 8 / 12 ulp(2h) | drive-to-top, near-edge randomized | chain |
| ME16 | tolerance 16 ulp(2h) | none | none. Equivalent in practice: the true maximum is 15.75 |
| MH | `ulp_at(high)` instead of `2*high` (tolerance 32 ulp(2h)) | none | none. Equivalent in practice |
| MB | mix step must be zero or normal | mix-to-zero, attempt-1 subnormal round trip, near-edge randomized | chain |
| MD | gain step: any finite (attempt-1 MINOR-1 reverted) | rejection test `bad(6, 1)` | none |
| MC | in-flight current: any finite (no line check) | rejection test (`bad(0, 1e6)`) | none |
| **MS** | **sign error: `line = t + remaining*step`** | **none** | `probe_overshoot_then_retarget_inward` |
| **ML** | **line drops its slope: `line = t`** | **none** | `probe_overshoot_then_retarget_inward` |
| **MR** | **rest guard dropped (`remaining == 0 \|\|`)** | **none** | `probe_hostile_rows_for_the_new_guards` (at-rest row) |
| **MZ** | **`-0.0` guard dropped from the in-flight allowance** | **none** | `probe_hostile_rows_for_the_new_guards` (`-0.0` row) |

## Test value, one sentence each

- `a_mix_ramp_to_zero_that_crosses_into_negative_subnormals_restores`: turns red if a restore
  refuses a negative-subnormal in-flight mix (MA) or a subnormal mix step (MB). It is the only
  deterministic test that reaches the bottom edge of the mix.
- `a_mix_ramp_to_one_that_crosses_above_one_restores`: turns red if a restore applies the strict
  range (MA) or a near-zero tolerance (ME1) at the top edge of the mix.
- `a_drive_ramp_to_its_top_that_crosses_above_it_restores`: turns red under MA or any tolerance
  of 12 ulp(2h) or less (ME12, ME8, ME1). Among the deterministic tests it comes closest to the
  observed maximum.
- `an_output_ramp_to_its_bottom_that_crosses_below_it_restores`: turns red if a restore applies the
  strict range below a gain's low edge (MA). It is the only deterministic gain-bottom case.
- `bad(6, 1)`: turns red if a subnormal gain step is accepted again (MD). Moving the row to word 6,
  a ramp at rest, is right: on the in-flight drive ramp the line check would hide MD.
- `a_restored_near_edge_ramp_continues_bit_for_bit`: a randomized differential, so I judged it by
  what its generator reaches.
  - It reaches all six edges, including drive-bottom and output-top, which no deterministic test
    reaches.
  - It restores into a scalar instance and into a bank lane at the native width, then checks a
    96-sample continuation by bits.
  - It turns red under MA, ME1, ME8, ME12 and MB.
  - Its per-edge reach assertion keeps a drifting generator from quietly weakening it. The seeds
    are fixed, and `crossed` is computed before the width-dependent draw, so the counts are the
    same on every target.
  - It does not reach a ramp that starts from an overshoot (MINOR-1).

## Findings

### MINOR-1: the line's `remaining * step` term is untested, and the attempt record says it cannot be tested, which is false

The code is at `crates/soft-clip/src/lib.rs:311`. The attempt record says: "Not covered by a test:
a sign error in the line … no self-produced state tells the two apart." That is wrong for the case
the doc itself covers, "any value the effect held, itself possibly an overshoot".

**Reproduction** (`probe_overshoot_then_retarget_inward`):

1. Drive from 35.999985 dB to +36 dB over 63 frames. The current is 15 ulp(2h) above the top.
2. A 1-frame block retargets the drive 5 to 200 decibel-ulps inward. The current is still above
   the top, with `remaining = 63`.
3. Its distance from the true line is 0.03 to 0.18 ulp(2h). Its distance from the sign-flipped
   line is 67 to 1461 ulp(2h).

Six of these eight self-produced states would be refused under MS. ML (`line = t`, effectively
"within 64 ulp of the target") refuses them too. The 40k random chain also produced 7 such states,
all from the bank. Every committed test stays green under both mutations.

Both mutations bring back MAJOR-1's failure class: the effect refuses its own mid-ramp snapshot,
and #1278's carry drops that lane to rest. #1278 is about to rework restore in every banked
effect, so a regression here is plausible.

**Fix:**

- Add a deterministic round-trip test of this shape:
  - Block 1: from a start found by search (or pinned), ramp the drive to +36 dB over 63 frames.
  - Block 2: 1 frame, with a point some 50 decibel-ulps below 36 dB.
  - Assert `current > gain(36)` and `remaining == 63`.
  - Restore into a fresh instance, assert snapshot identity, then continue bit for bit.
  - It must be red under MS and ML.
- Or give `near_edge_case` a second block that retargets mid-overshoot, with a reach count.
- Correct the sentence in the attempt record.

### MINOR-2: the two new hostile exclusions in `ramp_current_valid` have no rejection rows

The code is at `crates/soft-clip/src/lib.rs:306`. Dropping `remaining == 0 ||` (MR) or
`is_negative_zero(current) ||` (MZ) leaves every committed test green, and each mutation admits a
word that gate 2 says must stay refused.

**Reproduction** (`probe_hostile_rows_for_the_new_guards`):

- *At-rest row:* an output prepared at +24 dB, at rest, with its current set to
  `gain(24).next_up()`. Refused today; accepted under MR.
- *`-0.0` row:* the mix ramp from 40 subnormal units to 0, 48 frames in, with its current set to
  `-0.0`. Refused today; accepted under MZ.

The existing `bad()` rows cannot catch either mutation. The in-flight left ramp in the fixture is
the drive, whose line is far from both a range edge and zero.

**Fix:** add these two rows to the rejection test. Each needs its own small fixture, the way the
probe builds them.

### NIT-1: an accepted crafted state can evolve into an own snapshot that is refused

`probe_accepted_hostile_state_drifts_out` crafts an in-flight drive state:

- `target = top`, `step = 0.6 ulp(top)`, `remaining = 63`;
- current about 64 ulp(2h) off the line and above the top.

The restore accepts it. After 40 frames the effect's own snapshot is 71.9 ulp(2h) off the line,
and restoring it is refused. Only a crafted restore can reach this. The parent's hostile-finite-step
acceptance had the same property, and #1278's carry only moves states the effect produced, so it
is no regression. Still, the doc's "accept every word the effect itself can hold" should say that
the accepted set is not closed under render once a hostile word is in. Then nobody builds on that
assumption, for example a harness that asserts own-snapshot restores after a crafted restore.

### NIT-2 (for Sol; outside this slice's paths)

`.github/ISSUE_SPECS/1278-*.md:32` still cites the deleted `SubnormalStateRefusedOnRestore`
(attempt-1 NIT-2). #1278's brief also does not yet carry the handed-off attempt-1 MINOR-2: `inf`
in `X` with finite output, so D7 never fires and the effect refuses its own snapshot for up to 31
samples. Only #1071's attempt record mentions it. Whoever owns #1278 should add both.

## Other checks

- Hostile validation (gate 2) is unchanged except for the in-flight rounding margin:
  - `bad(0, 1)`, a subnormal in-flight drive, stays refused by the line check.
  - Hostile out-of-range currents that are now accepted lie on their own ramp's line. Each is
    reachable in one sample from a parent-accepted state with a hostile finite step, so the harm
    does not change.
- A self-produced gain step is always zero or normal. Gains stay at or above 0.0625 even after an
  overshoot, so `|Δ| >= 2^-27` and `step >= 2^-33`. The 1.34M-snapshot probe confirms it.
- Every new code path is target-independent control-plane f64/f32 arithmetic, with one shape and
  no `unsafe`. No source-grepping tests, no digest pins.
- Commit hygiene is fine:
  - four exact paths (the slice spec, `lib.rs`, two test files);
  - the attempt record and mutation table are in the slice spec;
  - the message ends with the `Co-Authored-By` trailer.
