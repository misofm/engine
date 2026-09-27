# Keep the mono collapse through paired both-channel parameter spans

Source: `docs/handoffs/effects-2026-09-27/AUTOMATION-DIAGNOSIS.md`, verified in `VERIFY-AUTOMATION.md`. **The Amendments section at the end supersedes the body wherever they conflict.** Draft names map to issues: automation-5 = #1003, automation-1 = #1004, automation-2 = #1005, automation-3 = #1006, automation-4 = #1007.

**Decision recorded (root, 2026-09-27):** the form is *spans only*: pair both-channel `Parameter` spans at the drain, and fold one-channel `PreparedTarget`s into the witness as today (amendments). The prototype's target half renders wrong audio (VERIFY-AUTOMATION F1). The host-`Both` alternative can acknowledge a command and then drop it (F5), which fails the acked-batch rule in `AGENTS.md`. The owner may overrule the form; the product need is not in question.

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

## Amendments (Sol verification, 2026-09-27)

Evidence: `docs/handoffs/effects-2026-09-27/VERIFY-AUTOMATION.md` (F1, F2, F5, F9, section 3)
and `verify-automation-raw-timings.txt`. **These amendments supersede the body wherever they
conflict.**

### A1. Scope: pair spans only; the target half is removed

The prototype's target rule is unsound. It compares the last `(slot, Left)` target with the last
`(slot, Right)` target and ignores any `Both` target between them.

* **Failure.** A drain FIFO `[L A, Both C, R A]` leaves left = `C` and right = `A`. P1 keeps
  `LIVE`, and the collapse renders the left plane on both channels.
* **Reachability.** It is reachable through the SDK: three submissions between two render quanta,
  left-only, then both-channel, then right-only.
* **Reproduced** through the shipped `host_web.wasm`: base output digest `39d2dc01…`, P1
  `22e07bf0…`.
* **Consequence.** On disengage, the chain copies the left state over the right, so the
  acknowledged right-only edit is lost.

The target half is also unnecessary. The SDK sends a symmetric both-channel EQ edit as **one
`Both` target**, because `prepare_targets` → `fill_targets` coalesces bit-equal sections, and the
witness already preserves that target.

Contracts 1-3 become:

1. In `EffectControlLane::stage`, only a `Parameter` record whose channel writes one channel sets
   `deferred` instead of calling `admit`. **Every `PreparedTarget` calls `admit` exactly as today**,
   so a one-channel target still clears `LIVE`. `Bypass` and `Observe` are unchanged.
2. `fn spans_pair(spans: &[PreparedAutomationSpan]) -> bool` is private, O(n) and iterator-only.
   * Staging leaves the window strictly increasing in `(parameter_index, channel)`, with
     `Left (1) < Right (2) < Both (3)` (`effect-contract/src/lib.rs:157`). So a one-channel pair
     is two **adjacent** spans.
   * A `Left` span must be immediately followed by a `Right` span with the same `parameter_index`
     and the same `start_value` bits.
   * A `Right` span must be immediately preceded by that `Left` span.
   * `Both` spans are skipped.
   * Use `windows(2)` or a zip, and no panicking index.
   * The prototype's any-over-all search is O(n²) and must not be copied.
3. At the end of `stage`: `if deferred && !spans_pair(&staging[..staged])`, clear `LIVE`.

Delete these parts of the body:

* the "Targets" bullets under "The pairing rule";
* the targets paragraph under "Why a pair leaves the channels equal";
* gate 1's "EQ targets for a lane whose L and R prepared `(enabled, kind)` differ" (it has nothing
  left to test).

`writes_pair` is not written.

### A2. Corrected product outcome (replaces the mechanism bullets and the table)

* Only records that reach the drain as one-channel `Parameter` records retire the collapse. On the
  web path those are **the compressor's and the limiter's** (and any other launch effect's
  `PerLane` parameters), through `into_effect_records`'s `(PerLane, 2)` arm.
* The EQ does not retire it. V8, mono console, each control written once on all 64 tracks and then
  left settled (shipped `host_web.wasm`, cpu 31, lock):

  | written | base µs | P1 spans only µs |
  |---|---:|---:|
  | nothing | 148.8 | 148.1 |
  | EQ gain only | **148.7** | 148.3 |
  | compressor threshold only | 248.0 | 148.2 |
  | limiter ceiling only | 247.8 | 148.6 |
  | all three | 247.7 (245.4 in a second run) | 148.2 |
  | 8 of 64 automated (mixed) | 290.5 (287.4) | 173.4 |

  Stereo console, 8 of 64 automated: 284.7 µs at base, 283.1 µs with P1 spans only.
* The native `SessionRuntime::push_parameter` harness sends an EQ edit as two owner transactions,
  so it produces one-channel targets. That is a harness shape, not the product's.
* The C ABI, the native host and the mobile host attach no effect console, so the web host is the
  only affected product path.

### A3. The obligation a span pair rests on, stated per effect

A twin pair leaves the channels equal only if every effect that can sit in a collapse-eligible
cohort applies `pending[0][p]` and `pending[1][p]` with channel-symmetric validity. That covers the
compressor, the true-peak limiter, and every other launch effect with live `PerLane` parameters:
the gate-expander, the de-esser, and so on.

* Enumerate them in the evidence.
* For each, add a unit test. It feeds one bank lane with bit-equal channels a twin pair, a pair
  whose values straddle a domain edge, and a pair with one value refused. It then asserts
  bit-equal left and right state afterwards.
* The capacity cut-off (`span_index < automation_capacity`) cannot split a twin, because the
  staging window is the capacity. Assert `staged <= automation_capacity` in debug.

### A4. Gates added or changed

* **Gate 1, additional scenarios (dev and release, W4 and W8):**
  * the F1 FIFO on one EQ lane: it must **retire** the cohort and match the forced-dual reference;
  * spans `[L v1, L v2, R v2]` on one parameter: last-wins leaves a pair, so the cohort is **kept**
    and matches;
  * spans `[L v1, R v2, L v2]`: kept, and matches;
  * a pair on one parameter plus a lone `Left` on another: retired;
  * EQ edits made both ways, as one `ParameterChannel::Both` owner edit
    (`SessionRuntime::push_parameter(ch, p, Both, v)` → one `Both` target, the SDK's shape) and as
    per-channel owner edits.

  Output words compare by bits, and the bank output is after the §4.4 check. A "both NaN" match is
  allowed only on a plane read before that check, and only where the forced-dual reference rejects
  the block.
* **Gate 2.** Add: an EQ-only both-channel ride keeps every cohort collapsed **on the base as well**
  (no P1 needed).
* **Gate 3.** Pin the scenario on the current tip, which is code-identical to `49f696c7`. Use the
  product shapes: EQ as `Both` owner edits, compressor and limiter as Left then Right records.
* **New gate 3b, through the shipped artifact.** Build base and change with the delivery recipe
  (below).
  * Run the P1-break check: on one mono track, three submissions through `prepared-control.js`
    before one render, in each of these orders:
    * `split`: `ch0 g1`, then `ch2 g2`, then `ch1 g1`;
    * `pair`: `ch0 g1`, then `ch1 g1`;
    * `both`: `ch2 g1`;
    * `left`: `ch0 g1`.
  * Do this for the EQ (gain 4.5 / 1.5 dB), the compressor (threshold −20 / −12 dB) and the
    limiter (ceiling −20 / −17 dB; the values must engage, and −12 dB does not on this fixture).
  * Every 40-block output digest must equal base's.
  * The harness is the `P1BREAK` mode (`P1FX`, `P1ORDER`) that
    `verify-automation-harness.patch` adds to `web_auto.mjs`. Apply it on top of the diagnosis
    patch.
* **Gate 5 mutations.**
  * Keep: "ignore the value", "lone write paired", and "split across drains kept".
  * Drop the two target mutations.
  * Add, each recorded red:
    * "defer one-channel `PreparedTarget`s too, with the prototype's last-per-channel rule" (red
      on the F1 FIFO, in gate 1 and gate 3b);
    * "pair adjacent spans without comparing `parameter_index`";
    * "accept a `Right` span with no preceding `Left`".
* **Gate 7, the V8 recipe.**
  1. Extract `docs/handoffs/effects-2026-09-27/automation-diag-tools/` from the diagnosis patch
     into scratch.
  2. In `web_auto.mjs`, edit the three hard-coded paths: the `prepared-control.js` import, the ABI
     layout JSON and `ROOT`. They name a deleted worktree and scratch root.
  3. Add the subjects:
     * `mono_eq = {...eq_gain, doc: "mono", effectIndex: 0}`;
     * `mono_comp = {...comp_threshold, doc: "mono", effectIndex: 1}`;
     * `mono_lim = {...lim_ceiling, doc: "mono", effectIndex: 0}`.
  4. Run `gen_docs.py`.
  5. Build each artifact with `scripts/build-web-audioworklet.sh`'s flags into
     `…/t-web-NAME/wasm32-unknown-unknown/release/host_web.wasm`. The harness names modules by
     that path.
  6. Under the lock, run `taskset -c 31 node --no-liftoff web_auto.mjs` with `ROUNDS=6
     BLOCKS=500`. Subjects `mono_mix,mono_comp,mono_lim,mono_eq` on arms
     `untouched,settled,eight_of_64`, and `console_mix` on `settled,eight_of_64`.

  The stereo rows must not be more than 2 % slower. The mono `settled` rows should sit within 2 %
  of `untouched`; that is descriptive, and a miss means the rule is not engaging. Then run
  `DIGEST=150` over the same subjects: it must print "all identical".

### A5. If the owner rules for the host-`Both` form instead

The rule is exact, but it carries two preconditions this draft does not meet (VERIFY-AUTOMATION
F5).

* **Capacity.** The web host derives automation capacity as `max(segments, queue records, 1)`,
  and the queue depth equals the records. Expanding one `Both` record into two spans can therefore
  overflow the staging window after an OK ack. Example: records = 2, and one submission with
  threshold and ratio `channel = 2` gives four spans in a two-span window. Capacity must be derived
  as at least 2 × the queue depth, and preparation must assert it.
* **Policy.** The drain must know each parameter's channel policy, because a `Shared` `Both` stays
  one span.

Re-brief rather than extend this slice.

### A6. Split pairs

The web host submits and renders on the AudioWorklet thread, so both halves of one submission are
always in one drain. A threaded host that splits them loses the pair, which clears `LIVE` as today.
That is correct and conservative.
