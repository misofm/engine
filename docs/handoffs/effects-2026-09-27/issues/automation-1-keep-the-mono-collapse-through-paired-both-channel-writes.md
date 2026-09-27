# Keep the mono collapse through paired both-channel effect writes

Automation follow-up P1 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `49f696c7`;
every `file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, sections 2, 3 ("Mono collapse") and
5 (P1). The prototype is `effect_contract::live_diag::PAIRED` in
`docs/handoffs/effects-2026-09-27/automation-diagnosis-prototypes.patch`. It is evidence only; do
not commit it.

## Product outcome

Mono stems are the product's common case. On a collapse-eligible track, the first live write to
any EQ, compressor or limiter parameter retires the mono collapse of that track's whole bank for
the rest of the plan. That includes a restatement of the held value and a knob moved on both
channels.

The mechanism:

* The web host lowers `COMMAND_EFFECT_PARAM` with `channel = 2` on a `PerLane` parameter to a
  Left record and a Right record (`hosts/host-web/src/lib.rs`, `into_effect_records`, the
  `(PerLane, 2)` arm). All three effects' parameters are `PerLane`.
* The EQ owner emits one `PreparedTarget` per channel.
* `EffectControlLane::stage` (`crates/effect-contract/src/live.rs:260`) folds each record into the
  lane's witness as it drains it (`:284`). A one-channel record is `Desymmetrize`
  (`symmetry.rs:319`), which clears `LIVE`.
* No site ever sets `LIVE` again, and one declining lane declines the cohort.

The builtins input stage met the same problem for trim. #210 phase 3 made `channel = 2` one `Both`
record "or it would retire the track's mono collapse". Effect parameters never got the fix.

This slice keeps `LIVE` when a drain's one-channel writes all pair up, which leaves the two
channels in bit-equal state. Measured on the prototype (in process natively, the shipped
`host_web.wasm` under V8, timing lock, cpu 31), in µs per 64-track block:

| mono console row | native `Simd8` | native `Simd4` | V8 `simd128` |
|---|---:|---:|---:|
| untouched (collapsed) | 69.2 | 119.2 | 151.2 |
| every track written once, then settled: base / P1 | 107.6 / **69.3** | 188.2 / **119.5** | 243.2 / **151.5** |
| 8 of 64 tracks automated: base / P1 | 147.8 / **91.9** | 224.7 / **139.7** | 284.8 / **177.3** |

On the stereo console, P1 changes nothing measurable: V8 8 of 64 went 281.9 → 282.6 µs, all 64
went 464.7 → 464.0 µs. These figures are descriptive only.

## Invariants

* **Class A.** Every output word, report, observation reading and state payload is unchanged, at
  `f32`, `Simd4` and `Simd8`, on every target. The collapse renders the dual bits whenever the
  two channels' upstream state is bit-equal. That is its standing premise, gated by mono-collapse
  M1-M3. This slice changes only *when* the witness claims that premise.
* **The pairing rule.** Evaluate it at the end of one `stage` call, over what that call drained:
  * **Spans.** For every staged span with `channel` Left (or Right), a staged span exists with the
    same `parameter_index`, the other channel, and a `start_value` equal by bits. The staging is
    already last-wins per `(parameter, channel)`, so this compares final values.
  * **Targets.** For every one-channel target in the retained FIFO prefix, the *last* target with
    the same `(slot, channel)` and the last target with the same slot and the other channel both
    exist and have equal `words`. The EQ's state after a FIFO of retargets of one section is that
    of its last target: `start_ramp` re-derives every step from `coef`.
  * `Both` records, `Bypass` and `Observe` fold as today.
  * If any one-channel write fails to pair, `LIVE` is cleared exactly as today.
* **Why a pair leaves the channels equal.**
  * Spans: `apply_automation` (compressor `lib.rs:347`, limiter `lib.rs:2769`) validates and
    applies `pending[0][p]` and `pending[1][p]` by the same code, from the same value, onto
    channels that the witness already holds equal.
  * The validity terms are symmetric for a pair: kind, sample, value, parameter, and order and
    capacity, which staging guarantees.
  * Targets: `apply_target_lane` (EQ `lib.rs:2148`) permits a target by the channel's prepared
    `(enabled, kind)`. Paired words carry those fields. The owner's publication validates every
    target against its candidate configuration (`crates/effect-compiler/src/control.rs`,
    `validate_targets`), whose enabled and kind entries are the prepared values, which are not
    automatable. So equal words mean equal prepared `(enabled, kind)`, and a pair is permitted on
    both channels or on neither. Gate 1 carries a case that would break this.
* **Conservative everywhere else.** A write split across two drains is not a pair. The rule
  never sets `LIVE`; it only declines to clear it.
* Allocation-free and bounded: at most `capacity²` comparisons per drain over fixed slices. No
  `unsafe`. No change to state layout or to any record type.

## Interface contract

1. In `EffectControlLane::stage`, a one-channel `Parameter` or `PreparedTarget` record sets a
   local `deferred` flag instead of calling `admit`. Every other record calls `admit` as today.
2. `fn writes_pair(spans: &[PreparedAutomationSpan], targets: &[PreparedEffectTarget]) -> bool`,
   private, iterator-only (no indexing that can panic), with the rule above.
3. At the end of `stage`, if the flag is set and `!writes_pair(..)`, clear `LIVE`.
4. The doc tables in `symmetry.rs` (the `LIVE` row) and `live.rs` state the rule.

## Smallest closable slice

Authorized paths:

* `crates/effect-contract/src/live.rs`;
* `crates/effect-contract/src/symmetry.rs` (docs only);
* a new test file under `crates/effect-contract/tests/`;
* `tools/console-workload/tests/` (the scenario and differential);
* this spec.

Steps:

1. **On the base:** write gate 3's scenario and pin its digests and collapse counters.
2. Contracts 1-4.
3. The gates, and the evidence.

## Non-goals

* The host-side alternative: lowering `channel = 2` to one `Both` record, with the drain expanding
  it. That changes admission and the owner path. It is the owner's ruling (diagnosis section 8,
  item 1). If the owner prefers it, this slice is re-briefed, not extended.
* Re-establishing `LIVE` after a genuinely asymmetric write. That needs a state comparison and is
  a liveness question.
* Any change to collapse eligibility's other terms, to the dispatch, or to a kernel.

## Objective gates

1. **Witness soundness, differential.** For every scenario, render the same bank twice: the
   production collapse against a forced-dual reference (`force_mono_collapse_off`). After every
   block, compare output words (a NaN compares as "both NaN", because the mono and dual bodies
   are separate instantiations and the compiler may commute a NaN-producing operation) and every
   track's full state payload by bits. Run in dev and in release. Scenarios:
   * the EQ, the compressor, the limiter, and a chain of all three;
   * `Simd4` and `Simd8`;
   * random both-channel writes (same value);
   * near-pairs: values one ulp apart, one channel written twice with the other written once to
     the first value, Left and Right in two different drains, a pair plus an unpaired write of
     another parameter;
   * EQ targets for a lane whose L and R prepared `(enabled, kind)` differ in a disabled band;
   * left-only writes;
   * hostile audio (`±0`, subnormals, NaN, ±inf).

   Every block must match, and a near-pair must retire the cohort.
2. **Engagement witness.** Using `bank_collapse_counters`:
   * a both-channel ride keeps every cohort collapsed on every block;
   * a restatement keeps it;
   * a left-only write retires exactly its cohort;
   * each near-pair case retires its cohort.
3. **Scenario pinned on base.** The mono console (`sixty_four_track_console_mono`, `control:
   true`), 128 blocks at W8 and W4, with the 8-of-64 mixed ride of the diagnosis. Pin one SHA-256
   of all output words on `49f696c7`. The digest must not move; only the collapse counters may.
4. **Existing gates unchanged:**
   * `effect-contract` tests (symmetry, live);
   * `tools/console-workload/tests/chain_shape.rs` and `placement.rs`;
   * `crates/host-core/tests/input_liveness_console.rs`;
   * every console workload's 64-block digest.
5. **Mutations**, each recorded red:
   * pair spans by parameter only (ignore the value);
   * pair targets by slot only (ignore the words);
   * compare the first target rather than the last per `(slot, channel)`;
   * treat a lone one-channel write as paired;
   * keep `LIVE` when any record was one-channel and the other half arrived in the next drain.
6. **Realtime and wasm.** `check-realtime-policy.sh` and the allocation gates must pass.
   `check-web-audioworklet.sh`: the render closure gains no trap owner or allocator, and the
   roster and kernel count are unchanged.
7. **No regression through the shipped artifact.**
   * Build base and change with the delivery recipe, and run the diagnosis's V8 harness
     (`automation-diag-tools/web_auto.mjs`) once, paired, 6 rounds.
   * The stereo `console_mix` settled and `eight_of_64` rows must not be slower than base by more
     than 2 % (median of paired per-round deltas).
   * Record the mono rows. This is descriptive; if the bound is exceeded, record it and escalate;
     do not tune.

## What the implementer will hit

* The rack decides the block's mode from the witness right after the drain, and applies targets
  later (`crates/rack/src/lib.rs:1258-1270`). The rule must be decided at the drain, from the
  records alone. That is why it relies on admission's per-channel target validation, not on
  application results.
* `symmetry.admit` is the structural hook. Keep `LiveConsoleRecord` as the only way a record
  reaches the witness, and add the deferral beside it rather than a second path per record kind.
* A pair that the effect rejects on both channels is harmless: nothing moved.

## Dependencies

None. It is independent of the kernel changes, and it multiplies their value on mono stems.
