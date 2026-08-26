# The compressor's idle-lane ramp guard — under-resolved at console level

**Not a null.** This ruling records a measurement that could not resolve an effect, so that the
next round starts from the right question instead of re-running the wrong row.

**Candidate.** Compressor effect-optimisation round 1, strategy S3. During a ramping prefix
`Channel::advance_ramps` walked every lane of the bank and called `LinearRamp::next_value` seven
times per lane per frame, whether or not that lane had anything in flight. A ramping *block* is not
a ramping *lane*: `process_block` cuts its prefix from the longest ramp anywhere in *either*
channel, so one automated track drags every other lane of its bank — and both channels of the
automated track itself — through the ramping body for the length of the window.

**Claim under test.** That a per-lane early-out is a measurable saving on a real console plan.

## What was built

A per-lane `is_ramping` scan with an early-out; a lane with nothing in flight now reads `remaining`
seven times and does no more. Bit identity is structural: `next_value` on a finished ramp
(`remaining == 0`) returns `current` and mutates nothing, so calling it is the identity and not
calling it is too. A ramping lane's resting parameters take `current`, which is the same `f32` bit
pattern `next_value` would have returned.

## The gate that had to be built first

No row in the console benchmark could see this at all. `console_model` clears the fixture's
automation table unconditionally, both fixture gates assert the standing sessions declare none, and
the one arm that does deliver spans — `console_hoist` — drives banks of *parametric EQs*. **No
compressor in the benchmark had ever seen an automation span.** The `console_automation` row was
added for this round: one Point span per block, on one track, through the real live-console control
queue and a real prepared plan, with `quiet`/`restated`/`automated` arms.

## Measurement, and why it does not resolve

Ramping surcharge = `automated − restated`, paired per observation (`paired_ramp_delta_median_ns`):

| arm | round 1 | round 2 |
|---|---|---|
| baseline | 2184 ns | 2155 ns |
| patched | **1873 ns** | **2094 ns** |

−14.2% on round 1, −2.8% on round 2. The round-to-round spread of the patched arm (11%) is larger
than the baseline's (1.3%), so **this row cannot separate the effect from its own noise.** The
direction is right in both rounds and no round regressed, but the row does not resolve the size.

Two structural reasons, both known before the run and neither fixable by re-running it:

1. **One bank of eight ramps.** Sixty-four tracks is eight eight-lane banks; automating one track
   puts a window in flight in exactly one of them, and the other seven never enter the ramping body
   at all. At most one eighth of the compressor's per-frame ramp work is even reachable.
2. **The surcharge is not all `advance_ramps`.** It also contains the control-queue drain, span
   validation, `apply_automation`, and the ramping lane's own `design_lane` — the last of which
   re-enters the static-curve design and is entirely untouched by the guard. The researcher's
   probe-level figure (−23% of the pure `advance_ramps` cost, at bank-block level) is not
   contradicted by this; it is simply not what this row measures.

## Decision and boundary

S3 is **kept**: it is bit-identical by construction, probe-proven at kernel level, and measured
non-regressing here. What is recorded is that the console automation row **under-resolves** it.
Nothing is claimed for S3 from the console table.

**To resolve it, a later round needs one of:**

* a **more-lanes-ramping row variant** — automate one track per bank, or several parameters per
  track, so the ramping body is reached by every bank rather than one in eight; or
* a **kernel-level paired arm** that times `advance_ramps` directly, where the surcharge is not
  diluted by the queue drain and the surviving `design_lane`.

Deliberately not chased in round 1: both are new measurement subjects, and the round's product
change was already complete and gated without them.

## Evidence

* `artifacts/compressor-round1/README.md` — the round's record.
* `crates/miso-engine-compressor/tests/ramps.rs` — the bit-identity properties the guard rests on,
  including an idle lane dragged through the ramping body by a neighbour's window.
* Red-mutation evidence: a guard that treats a single-parameter window as idle is red on
  `an_idle_lane_is_untouched_by_a_neighbours_ramp` and on four pre-existing ramp tests.
