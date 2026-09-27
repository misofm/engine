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

## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), 2026-09-27, branch `codex/1004-paired-parameter-spans` on
`1010d50c` (the local batch `codex/batch-plumbing-floor-2`). Code commits `4b8e633d` and
`1c6bb0f8` (the second adds the rendered M5 scenario). Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, Node 22.23.2, `CARGO_INCREMENTAL=0`, every
target directory under scratch (deleted afterwards). Nothing pushed, no GitHub state touched.

**Base note.** `1010d50c` is not code-identical to `49f696c7`: #971 and #977-#979 landed in
between. Every "base" below is `1010d50c`, built from a `git archive` of it.

### What changed

- `crates/effect-contract/src/live.rs`, `EffectControlLane::stage`: a one-channel `Parameter`
  record is held back (`deferred`, the first such record of the drain) instead of being folded at
  once. After the drain it is folded through the same `ChannelSymmetryWitness::admit` -- clearing
  `LIVE`, as before -- unless `spans_pair(&staging[..staged])`. Every other record, including every
  one-channel `PreparedTarget`, folds in FIFO order exactly as before (amendment A1). Admission,
  staging, the target FIFO, `dropped`, `unbound` and `target_error` are untouched: the rule reads
  the finished window and writes only the witness.
- `spans_pair` (private, O(n), iterator-only: `first`/`last` and a `zip` over adjacent spans, no
  indexing): a `Left` span must be immediately followed by its twin `Right`, and a `Right`
  immediately preceded by its twin `Left`; `Both` needs none. `twin_spans` compares the parameter,
  kind, both samples, and `start_value`/`end_value` by bits.
- `debug_assert!(staged <= staging.len())`, documented as `staged <= automation_capacity` (both
  callers size the window to it: `rack::ConsoleEffectBankStage::new`, `graph::ConsoleEffect`), so
  the effects' `span_index < automation_capacity` cut-off cannot split a twin (A3).
- Docs: the `stage` docs and the `LIVE` row, the `LIVE` constant and a new "pairing rule" section
  in `symmetry.rs`, including what the rule deliberately does not cover (targets, split pairs,
  re-earning `LIVE`).
- Tests: `crates/effect-contract/tests/paired_spans.rs` (new, 14 tests) and
  `tools/console-workload/tests/paired_spans.rs` (new, 12 tests). `tools/console-workload/Cargo.toml`
  gains `rack` as a **dev**-dependency (plus the matching `Cargo.lock` line), which is outside the
  authorized paths: the differential needs the production `ConsoleEffectBankStage` and `BankChain`
  over real banks so it can read each bank's state payload between blocks, which no plan-level
  surface exposes. The bench and the wasm guest that link the crate are unaffected.

### The interleaving coverage

Drain level (`effect-contract/tests/paired_spans.rs`), against an independent model that replays
the records into final values per `(parameter, channel)` (not the rule's adjacency walk):

- every sequence of up to four records over two parameters x {L, R, Both} x two values (22,621
  sequences), in one drain and split across two drains at every point: `LIVE` equals the model's
  answer, a split drain is judged per drain, and the rule never keeps less than the base drain;
- every order of six mixed atom sets (pairs, lone halves, `Both`, last-wins, restatements, the F1
  shape in spans): `LIVE` and the staged window (compared to a model of the sorted last-wins window)
  match;
- near pairs (one ulp, `+0.0`/`-0.0`, two NaN payloads, the same value on adjacent parameters),
  last-wins `[L v1, L v2, R v2]` / `[L v1, R v2, L v2]` kept and `[L v1, R v1, L v2]` retired;
- split pairs and a second half that never arrives; `Bypass`/`Observe` interleaved with a pair;
  a queue at capacity (the producer refuses the extra record or the second half before anything
  is drained; `staged` and `dropped` unchanged); a window smaller than the drain (dropped halves,
  `dropped` counted exactly as before); a raw pair refused by a target owner (`target_error`,
  nothing staged);
- one-channel targets in all six orders of `[L A, Both C, R A]` and as an equal-word L/R pair:
  always retired; a `Both` target preserved.

Rendered (`console-workload/tests/paired_spans.rs`, dev and release):

- **Gate 1, bank differential.** Real EQ, compressor, limiter and the chained strip, each behind
  the production `ConsoleEffectBankStage` in a production `BankChain` with a seam-side fader
  (different L/R gains), rendered collapsing and forced-dual from the same records. After every
  block: every output word by bits (no NaN relaxation was needed: the bank output is after each
  effect's own non-finite check), every `BankProcessReport`, and every lane's full state payload
  (on a collapsed block: ours-left = dual-left and the dual reference's two channels agree; on a
  dual block: all three sections). Collapse per block must equal the expected keep/retire exactly;
  a scenario marked asymmetric must also make the dual reference's channels actually diverge.
  Scenarios per fx: random both-channel rides (3 seeds, one with hostile audio: `+-0`, subnormals,
  NaN, `+-inf`, 3e38), restatement on every lane, left-only, right-only with hostile audio, one ulp
  apart, one channel twice and the other once to the first value, split across two drains, second
  half never arrives, pair plus an unpaired write of another parameter, a lone `Left` on one
  parameter next to a lone `Right` on the next, last-wins pairs (kept), a `Both` span beside a pair
  (kept; refused on both channels), reset (both kinds) and restore in the middle of a pair (kept),
  the F1 FIFO through the real EQ target designer (retired), per-channel EQ owner edits (retired),
  a queue at capacity holding only pairs (kept) and one refusing a pair's second half (retired).
  Scenario counts: EQ 14 at `Simd8`; compressor 17 at `Simd8`; limiter 17 at `Simd8` and 17 at
  `Simd4`; the chain 38 at `Simd8`. This x86-64-v3 build binds no four-lane EQ or compressor bank
  (D4), so `Simd4` for those is covered through `host_web.wasm` below.
- **Every order of a mixed drain on the chained strip**: 4 span sets x 3 EQ sets, every
  permutation of each (624 renders): kept exactly when no one-channel target and every span pairs
  per the model (84 kept, 540 retired), each rendered against forced dual.
- **Shared vs PerLane.** `every_collapse_capable_launch_effect_is_one_of_these`: the banks that
  support mono collapse are exactly the EQ, compressor and limiter, and none declares a `Shared`
  automatable parameter. A `Both` span on a `PerLane` parameter is refused on both channels
  (rendered scenario above); the drain-level suite covers `Both` interleavings.
- **A3, the per-effect obligation**, over the whole launch registry at every supported link mode:
  a real drain stages a twin pair, one block renders, then another, and both channels must stay
  bit-equal; each pair is refused on both channels or neither. Values: the default, both domain
  edges, the midpoint, one step past each edge, NaN, `+-inf`, `-0.0`. Enumerated:

  | effect | path | link modes | live `PerLane` params | twin pairs | refused on both |
  |---|---|---|---:|---:|---:|
  | compressor | bank, collapse-capable | DualMono, Maximum, Average | 7 | 70 each | 38 each |
  | true-peak-limiter | bank, collapse-capable | DualMono, Maximum | 2 | 20 each | 12 each |
  | parametric-eq | bank, collapse-capable | DualMono | 22 | 218 | 218 (a target owner refuses every span) |
  | gate-expander | bank | DualMono, Maximum, Average | 4 | 40 each | 21 each |
  | multiband-compressor | bank | DualMono, Maximum, Average | 10 | 100 each | 56 each |
  | soft-clip | bank | DualMono | 3 | 30 | 15 |
  | transient-shaper | bank | DualMono, Maximum, Average | 3 | 30 each | 15 each |
  | delay | scalar (no bank here) | DualMono | 4 | 40 | 21 |

  There is no de-esser or dynamic-EQ crate in the registry. Only the first three can sit in a
  collapse-eligible cohort (`BankChain::collapse_prefix_of`); the rest are covered for when they
  gain a one-plane body.
- **Gate 2, console engagement** (mono console, `control: true`, collapsing vs forced dual, every
  block's output digest equal): a both-channel mixed ride keeps all 8 cohorts collapsed on every
  block with transitions `[0, 0, 0]`; a restatement of every track keeps them and renders the same
  bits as setting the value once; an EQ-only `Both` ride keeps them (green on the base tree too);
  left-only compressor, left-only EQ owner edit, one ulp apart, one channel twice, split across
  drains, pair plus unpaired other parameter, and per-channel EQ owner edits each retire exactly
  the written track's cohort on the block they land (transitions `[1, 0, 0]`).
- **Gate 3, pinned.** The mono console's 8-of-64 mixed ride (3 EQ gains as `Both` owner edits,
  3 compressor thresholds and 2 limiter ceilings as `Left` then `Right` records, after every
  track's three controls are set once the same way) over 128 blocks: SHA-256
  `9242f149101f3bcd2e48268096169f1670a4ec368fe11071408f6b44a49c45a4` at `Simd8` and at `Simd4`,
  on the base tree and on the change. Counters: `Simd8` `[0, 8]` at base, `[1024, 8]` with the
  change; `Simd4` `[2048, 16]` on both (no EQ or compressor bank at four lanes natively). The run
  also asserts each of the three ride parts moves the output away from the settled run (VERIFY F6).
- **Gate 3b, P1 break through `host_web.wasm`** (40 blocks, verifier's `P1BREAK` harness):

  | fx | split `[ch0 g1, ch2 g2, ch1 g1]` | pair | both | left |
  |---|---|---|---|---|
  | EQ | `39d2dc01` = | `4a47ef63` = | `4a47ef63` = | `59aefc46` = |
  | compressor | `b787c6c2` = | `e623a045` = | `e623a045` = | `60a6d4ed` = |
  | limiter (-20/-17 dB) | `e0b793b6` = | `c08ea999` = | `c08ea999` = | `d58db10c` = |

  `=` means the change's digest equals base's; all twelve do, and the EQ digests are the
  verifier's. The M4 mutant (below) gives `22e07bf0` on EQ split and base's digest on the other
  eleven -- the verifier's F1 reproduction exactly.
- **V8 identity**: `DIGEST=150` over `mono_mix`, `mono_comp`, `mono_lim`, `mono_eq`, `console_mix`
  x `untouched`, `settled`, `one_point`, `one_left_only`, `eight_of_64`, `all_64`: "all identical"
  (30 pairs). As the verifier found (F6), `lim_ceiling`'s `-3 +- 0.25` dB never engages on this
  fixture, so `mono_lim` has one digest on every arm; the limiter's identity evidence is 3b's
  engaging values, gate 1 and gate 3.

### The mutations (gate 5, as amended)

Each applied to `live.rs`, both new suites run in dev, reverted with `git checkout`. All red.

| # | mutation | drain suite (`effect-contract`) | rendered suite (`console-workload`) |
|---|---|---|---|
| M1 | ignore the value (`twin_spans` without the value bits) | RED 4: near pairs, last-wins, mixed orders, exhaustive | RED 5: compressor ulp near pair renders wrong audio (block 5 lane 3 right: -0.11839593 vs -0.11839594); limiter and chain collapse while the dual channels disagree; console ulp case; mixed drains |
| M2 | lone write paired (only a present-but-unequal L/R pair clears) | RED 10 | RED 5: wrong audio on left-only (compressor), limiter/chain disagreement, console renders non-dual bits at block 9, mixed drains |
| M3 | a lone half carried to the next drain and kept if its twin arrives there | RED 6 (split, second half, exhaustive, ...) | RED 5: wrong audio, disagreement, console, mixed drains |
| M4 | defer one-channel targets too, with the prototype's last-per-channel rule | RED 1 (`one_channel_targets_fold_as_before_in_every_order`) | RED 4: **F1 FIFO renders wrong audio** (EQ block 5 lane 2 right: 0.17237435 vs 0.17215028), chain F1 disagreement, per-channel EQ edits on the console, mixed drains; also host-core `two_per_lane_writes_that_agree_still_decline_the_lane`; and gate 3b red on EQ split through `host_web.wasm` (`22e07bf0`) |
| M5 | pair adjacent spans without comparing `parameter_index` | RED 2 (near pairs, exhaustive) | RED 3: `L p v` next to `R p+1 v` renders wrong audio (compressor) and disagreement (limiter, chain) -- the scenario was added after the first M5 run was green in this suite |
| M6 | accept a `Right` span with no preceding `Left` | RED 4 | RED 4: right-only writes collapse while the dual channels disagree; console renders non-dual bits |

The base tree (the deferral removed entirely) fails exactly the "kept" assertions of both suites
and none of the output, report or state comparisons.

### Realtime, wasm and the existing gates

- `check-realtime-policy.sh` ok (57 marked regions; `spans_pair`/`twin_spans` are inside the
  marked region), `check-env-vocabulary.sh` ok, `check-workspace-policy.sh` ok,
  `check-effect-contract.sh` ok, `check-rack-policy.sh` PASS.
- `check-web-audioworklet-callgraph.py` on the delivery-recipe `host_web.wasm`, base and change:
  **identical reports**. Render closure 8, 5 traps, one trap owner (`render_inner`); kernel roster
  all ok, 14 kernels (`f32x4_arith=13149`); `meter_poll` and `command_submit` unchanged.
- `run-wasm-gates.sh`: ok (native + wasm scalar + wasm simd128; 142 cases, 358 comparisons, 0 mismatches; detector history resident; f64 lane probes vectorised).
- Artifacts: base `2309b243…` (3,389,795 bytes), change `955acffa…` (3,390,202, **+407**). The
  checked-in pin (`8934cdd9…`) is already stale on this batch branch (base differs from it); the
  repin belongs to the batch boundary, so the pin file is not touched here.
- Standing console digests: the 64-block digest of all 14 native session rows (`gain_pan_profile`
  `digests`) is identical on base and change (`sixty_four_track_console` `fe5bed9b…`,
  `…_console_mono` `fc96d91f…`, …).
- `cargo test -p effect-contract -p host-core -p effect-compiler -p host-web -p console-workload`, dev and release: 572 passed, 0 failed, 7 ignored in each profile. That includes the standing `effect-contract` symmetry and live suites, `console-workload` `chain_shape.rs`, `placement.rs` and `automation.rs`, and host-core `input_liveness_console.rs` and `symmetry_witness.rs`.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean. `cargo fmt --all -- --check`: clean.

### Timing (descriptive; µs per 64-track block, cpu 31, shared lock)

**V8** (`host_web.wasm`, delivery recipe, `node --no-liftoff`, the diagnosis's `web_auto.mjs` with
the verifier's subjects, one invocation, `ROUNDS=6 BLOCKS=500`, both modules interleaved per
round). Hold 21:27:44-21:28:42, load average 6.68 → 9.15 (others building). Δ is the median of the
six paired per-round deltas; rounds 3 and 6 carry host-load outliers in both directions, which the
paired median absorbs.

| mono console | base | change | Δ |
|---|---:|---:|---:|
| quiet (`untouched`) | 150.71 | 148.42 | -2.2 (-1.5 %) |
| compressor threshold written once (`mono_comp settled`) | 251.68 | 149.10 | -102.1 (-40.6 %) |
| limiter ceiling written once (`mono_lim settled`) | 250.52 | 148.52 | -101.9 (-40.7 %) |
| EQ gain written once (`mono_eq settled`) | 150.88 | 148.65 | -2.0 (-1.3 %) |
| all three written once (`mono_mix settled`) | 250.49 | 148.69 | -101.8 (-40.6 %) |
| 8 of 64 automated (`mono_mix eight_of_64`) | 291.88 | 175.08 | -116.9 (-40.0 %) |
| all 64 automated (`mono_mix all_64`) | 470.31 | 286.80 | -183.4 (-39.0 %) |
| **stereo** console settled | 248.40 | 249.33 | +0.4 (+0.1 %) |
| **stereo** console 8 of 64 | 289.53 | 289.79 | +0.4 (+0.1 %) |
| stereo console all 64 | 469.04 | 470.46 | +1.3 (+0.3 %) |

The stereo rows are within the 2 % bound. The mono written-once rows sit within 0.5 % of
`untouched` (149.0-149.1 against 148.4-149.0), so the rule engages. The ~2 µs the change is
faster on rows where nothing is written (quiet, EQ-only) is consistent across rounds and is not a
code-path difference (no record is drained on `untouched`); the builtins-only row is 49.85 / 49.86.

**Native `Simd8`** (in-process `SessionRuntime` harness, scratch only; base and change are two
binaries run alternately ABBA, one process per build per round, 6 rounds x 800 blocks per arm,
same ride values as gate 3). Hold 21:30:17-21:30:31, load 9.48 → 9.27; **my own `nice -n 19 -j 4`
test build overlapped this hold** (not retried).

| mono console | base | change | Δ | collapsed/cohorts base → change |
|---|---:|---:|---:|---|
| quiet | 69.31 | 69.48 | +0.5 | 9024/8 → 9024/8 |
| compressor written once | 108.56 | 69.47 | -38.8 (-35.8 %) | 0/8 → 9024/8 |
| limiter written once | 108.28 | 69.52 | -39.0 (-36.0 %) | 0/8 → 9024/8 |
| 8 of 64 automated | 148.78 | 92.21 | -56.6 (-38.0 %) | 0/8 → 9024/8 |
| all 64 automated | 228.78 | 138.50 | -90.2 (-39.4 %) | 0/8 → 9024/8 |
| stereo settled | 107.64 | 107.44 | +0.0 | - |
| stereo 8 of 64 | 147.46 | 147.49 | +0.1 | - |

Native `Simd4` is not tabled: this build binds no four-lane EQ or compressor bank, so it is not
the browser's shape (V8 is).

### For the verifier

- The `rack` dev-dependency of `console-workload` (outside the authorized paths; reason above).
- `host-core/tests/symmetry_witness.rs`'s `two_per_lane_writes_that_agree_still_decline_the_lane`
  is still correct (it drives the EQ owner, so one-channel *targets*) but its prose says a
  per-channel pair "is how the ABI addresses a `PerLane` parameter"; for spans that is now kept.
  Not edited (outside the authorized paths).
- `effect-contract/tests/MUTATIONS.md` does not carry these rows; they are recorded here only.
- The bank differential drives effects directly with hostile audio (the product's input stage
  would sanitize NaN/inf first) and still matched by bits, so no NaN relaxation is used anywhere.
