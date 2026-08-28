# Stationary-smoother hoist: where it wins, and where it measured nothing

**Candidate.** Issue #144 item 6 / #149 workplan item 1 — "stationary-smoother bitwise hoist:
detect block-invariant coefficient smoothers by ticking once and comparing bits; stationary groups
skip ramp arithmetic and re-prepare."

**Status.** Adopted, with two boundaries recorded here because both were predicted to be wins and
one of them measured a regression.

## Boundary 1 — on a session with no parameter traffic, the hoist wins nothing

The optimisation's stated motivation is that "most console blocks have unmoving EQs". That is true,
and it is *already* the fast path: every builtin channel-strip effect in this engine had a
counter-based ramping split before this work landed —

| effect | existing split |
|---|---|
| parametric EQ | `process_section` chooses `svf_block` over `svf_block_ramped` when no lane has `remaining > 0` |
| compressor | `process_block` splits on `max_remaining` and dispatches `frames_loop::<L, false>` |
| gate / expander | `run_block` splits on `ramp_frames_left` |
| 2x2 matrix | `MatrixStage::process` returns early on `if maximum == 0` |
| fader / mute | `process_lane` runs a flat multiply once `remaining` reaches zero |

So on the 64-track qualification fixture as committed — `automation = []`, nothing moving — the
hoist changes no code path and saves nothing. **Measured: no difference, as expected.** Anyone
proposing this optimisation on the strength of "most tracks are untouched" should read that
sentence first: untouched tracks were already free.

What the counter misses, and what a bit compare catches, is a parameter **retargeted to the value
it already holds**. That opens a smoothing window whose every increment is exactly `+0.0`: it
renders the value it started from for the whole window and then snaps to a target that is already
that value. A console re-sends controls it did not move on every automation refresh, and because
the bank kernels take their ramping decision across *all* lanes, one lane's no-op window drags a
whole eight-track bank onto the ramped path. That is the case the hoist is for, and it is the only
case it is for.

## Boundary 2 — the ramp-arithmetic half alone measured a *regression*

The first implementation did exactly what item 6's headline says: skip the ramp arithmetic for a
stationary smoother. Measured on the 64-track fixture under redundant automation traffic, paired
alternation, two measured rounds:

| arm | p50 ns/block | paired delta vs restated |
|---|---|---|
| quiet (no traffic) | 31 220 | — |
| restated (hoist fires) | 39 945 | — |
| moving (real move) | 39 124 | **−821 ns** |

The hoisted arm was **slower**, reproducibly, in both rounds and at both track counts. Two reasons,
and both are worth keeping:

1. The `f64` `design_svf` coefficient design runs on every automation event in *both* arms and
   is nearly the entire cost of the traffic. The ramp arithmetic the hoist was skipping is small
   beside it.
2. The hoist's `settle` writes eighteen lane words (`coef`, `target`, `step`) where the
   `start_ramp` it replaces writes twelve and reads six. Per event, the hoist path cost *more* than
   the ramped block it saved.

Item 6 says "skip ramp arithmetic **and re-prepare**". The re-preparation half is not a detail of
the optimisation; on this engine it is the whole of it.

## What was adopted, and what it measures

Reading the cached designed words back from `Section::target` when a band is restated unchanged —
sound because `BandTarget::words` is a pure function of `(band, sample_rate)` and `Section::target`
holds exactly what it last returned, so the two agree bit for bit by determinism rather than by
tolerance.

Frozen workload: `fixtures/session/v1/console-sixty-four-track.toml`, 48 kHz, 128-frame quantum,
1000 observations, three arms alternated per observation, one warmup pass and two measured rounds.
Runner `scripts/run-console-benchmark.sh`; record
`artifacts/issue149/console-benchmark.accepted.jsonl`.

| workload | quiet | restated | moving | paired delta |
|---|---|---|---|---|
| nine-track ragged strip | 7 845 ns | 8 887 ns | 9 909 ns | **+1 022 ns/block** |
| sixty-four-track console | 31 320 ns | 35 467 ns | 39 545 ns | **+4 078 ns/block** |

The residual between `quiet` and `restated` — 4.1 µs at 64 tracks — is the automation admission
plumbing (span parsing, the `pending` array, the lane writes `start_ramp` still performs). The
hoist does not remove it and this ruling does not claim it does.

**Measurement boundary.** The hoist rows are measured at the **effect-bank** boundary (the EQ banks
a 64-track session forms), not through the whole prepared plan, because delivering per-block
automation through a compiled session's automation array is a different piece of plumbing. The
number is therefore the saving on the EQ bank, not on the whole channel strip: against the
64-track session's 287 µs/block it is about 1.4%, and against the EQ banks alone about 10%.

## What would justify reopening the rejected half

The ramp-arithmetic-only hoist is rejected **as a standalone change**, not as a component. It is
part of the adopted change and pays for itself there. It would become independently worthwhile if
`settle` stopped writing more lane words than `start_ramp` — writing only the words that differ
would remove the per-event cost that made it negative — or if an effect appeared whose ramp
arithmetic were large relative to its coefficient preparation. The limiter is the nearest such
case: it had no ramping split at all, so its hoist is a pure removal with no `settle` to pay for.

## Links

- Optimisation: issue #144 item 6, issue #149 workplan item 1.
- Implementation: `crates/miso-engine-effect-runtime/src/ramp.rs`
  (`LinearRamp::stationary_at`), `crates/miso-engine-parametric-eq/src/lib.rs`
  (`Channel::start_ramp`, `Channel::target_words`, `BandTarget::same_bits`),
  `crates/miso-engine-true-peak-limiter/src/lib.rs` (`ramps_are_stationary`),
  `crates/miso-engine-effect-contract/src/lib.rs` (`ParameterSmoother::set_target`).
- Gates: `crates/miso-engine-effect-runtime/tests/stationary_hoist.rs`,
  `crates/miso-engine-parametric-eq/tests/stationary_hoist.rs`.
- Bench protocol: `scripts/{preflight,run,test}-console-benchmark.sh`.
