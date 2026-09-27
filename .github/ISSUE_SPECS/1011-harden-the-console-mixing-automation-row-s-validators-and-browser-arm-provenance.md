# Harden the console_mixing_automation row's validators and browser-arm provenance

## Product outcome

#1003 added the `console_mixing_automation` benchmark row. Its Sol verification passed it, and found three test and provenance gaps. None changes what the row measures, but each would let a later edit weaken the row without any gate noticing. The findings are quoted from the #1003 verdict (`.github/ISSUE_SPECS/1003-*.md`).

## Findings to close

2. **LOW-MEDIUM: the mutation suite does not isolate three new validator rules.** Each was deleted
   alone, and `test-console-benchmark.sh` still passed:
   * the aggregate's cross-round agreement on `preflight_output_sha256` and
     `preflight_bank_collapse_counters`. Its named mutation sets `automated_limiter_only` to the
     preflight `restated` digest, so the per-record A3 rule refuses it first;
   * `preflight_bank_collapse_counters.quiet[0] == quiet[1] × preflight_blocks`, whose mutation the
     A6 equality refuses first;
   * `preflight_blocks == preroll_blocks + 64`, whose mutation the quiet-counter rule refuses first.

   **Failure scenario.** A later edit drops one of these rules and the suite stays green, against
   the suite's own header. **Fix.** Add mutations that change only the guarded field. For example,
   give `.[49]`'s `automated_limiter_only` a fresh digest; or change `preflight_blocks` together
   with both quiet-shaped counters.
3. **LOW: `quiet == restated` does not check the limiter's bases.** Neither limiter engages near its
   held value, so restating it 0.5 dB off leaves every gate green. The validator comment ("the digest
   equality … proves the row restated exactly those") overclaims for the limiter. There is no timing
   effect: a restated ceiling is stationary after the settle. The literal bases in
   `the_eight_controls_resolve_by_id_to_their_held_values` pin `resolve`, not `value(Restated)`.
   **Fix.** Narrow the comment, or assert `value(Restated, _) == base` in a unit test.
4. **LOW: the browser arm's protocol and provenance.**
   * It takes one measured round, with no labelled warmup (the preflight warms it), where AGENTS.md
     asks for one warmup and two rounds. So there is no cross-round digest agreement.
   * `prepare` records no commit, so `run` can pair a module and `controls.json` built at commit A
     with `candidate_commit` B.
   * Its input is one continuous 130 Hz sine shared by every track. The native row feeds per-track
     phase-offset frozen blocks. The record does not say so, although the limiter's engagement
     differs between the two arms partly for that reason.

## Objective gates

1. Each of the three validator rules named in finding 2 has a mutation in `scripts/test-console-benchmark.sh` that changes only the field it guards, and each mutation is refused by that rule alone. Deleting any one of the three rules turns the suite red.
2. A unit test asserts `value(Restated, _) == base` for all eight controls, and the validator comment no longer claims more than the digest equality proves.
3. The browser arm runs one labelled warmup and two measured rounds, with cross-round digest agreement. `prepare` records its commit, and `run` refuses a module or `controls.json` from a different commit. The record states the browser arm's input signal and how it differs from the native row's.
4. The standing digests and the `after-1003` row digests are unchanged, and `scripts/test-console-benchmark.sh` and the policy scripts pass.

## Attempt 1 evidence

Terra, attempt 1. Code `ac2c6eec`; this section and the browser record are the commit after it.
Nothing pushed. Worktree `engine-1011`, branch `codex/1011-automation-row-hardening`, from the
batch head `c671b48e`.

### Gate 1: every validator rule of the row is isolated

A scratch sweep (not committed) made each rule vacuous on a copy of `scripts/`, one at a time, and
ran the whole `test-console-benchmark.sh` against the copy. Every weakened copy was first compiled,
so a syntax slip could not pass for a red.

* **Record and aggregate validators, 66 rules.** Before the fix, the suite missed 26 of them,
  including Sol's three:
  * `preflight_blocks == preroll_blocks + 64`;
  * `preflight quiet[0] == quiet[1] × preflight_blocks`;
  * the aggregate's cross-round agreement on `preflight_output_sha256` and on
    `preflight_bank_collapse_counters`.

  After the fix, 60 of 66 are red, all four of those included.
* **The new mutations each change only the field their rule guards, and keep every other rule
  satisfied.** Examples:
  * `preflight_blocks = 64` together with both quiet-shaped preflight counters;
  * the preflight quiet counter together with the EQ-only arm's;
  * a fresh `$a…4` digest for round two's `automated_limiter_only`. The old mutation used
    `$digest_c`, the preflight `restated` digest, so the per-record A3 rule refused it first.

  Others added the same way: owner-edit and parameter-record counts with consistent push totals,
  a shorter pre-roll with consistent counters, object-shaped controls, non-number values in jq's
  order, a zero-cost quiet arm, restated and automated percentile order, three-element counters,
  automated collapsing more than quiet, an extra preflight arm, a non-digest preflight digest, a
  preflight `automated == restated`, and round-two-only automated and preflight counters and a
  cohort change.
* **The six rules still unisolated are implied by other rules, so no mutation can reach them:**
  * `.record == "console_mixing_automation"`, because the dispatcher selects on it;
  * the two `type == "object"` guards, because the `keys_unsorted` pins imply them;
  * the aggregate's count of two and its kind for the row, implied by the fifty-record arithmetic
    and the per-record kind pin;
  * `quiet`'s cross-round counters, implied by the per-record quiet rule plus the cross-round
    agreement of the other two arms' cohorts.

  They are left in place and named here.

### Gate 2: the restated value equals the base

* `mixing::restated_pushes_exactly_the_held_bases` in `tools/console-workload/tests/automation.rs`
  asserts `value(Restated, block)` equal to the held base, bit for bit, for all eight controls.
  The bases are the fixture's A2 values. It checks blocks 0, 1, 2, 3, 63, 64, 1063 and 1064, and
  also checks `value(Quiet) == None` and the automated alternation.
* **Red.** Sol's mutation restates the limiters at `base − 0.5`. It fails this test with
  `ch16: restated before block 0`, while the other five mixing tests stay green. That green is
  Sol's F3, reproduced.
* **Comment.** The validator comment now says the digest equality proves the held value only for
  the EQ and the compressor, and names this test as what pins all eight.
* **Browser arm.** It asserts the same in `submit`: every restated record's value equals the
  control's base.

### Gate 3: the browser arm's protocol, provenance and input

* **Rounds.**
  * `run` launches the harness three times, one process per round as the console runner does:
    `warmup`, whose record is discarded but whose in-run assertions must pass, then `1` and `2`.
  * The two records must pass the new `scripts/web-mixing-automation-validator.jq`. It holds each
    record to the row's pins and requires rounds exactly {1, 2}. Across the two rounds it requires
    equal digests (restated, automated and preflight), module, commit, controls, native feed,
    Node, V8, flags, document, ring, admissibility and CPU.
  * `test-console-benchmark.sh` gains the validator's cases:
    * the accepted pair, and a per-key `del`/`null` sweep of one round;
    * 57 targeted per-record mutations, each applied to both rounds so the rounds still agree;
    * 15 round-two-only mutations and 5 set-level ones (one round, two round ones, a warmup
      beside a measured round, three rounds).

    The same sweep over this validator's 81 rules and tuple fields: 80 red, and one
    `== mixing_automation_controls` edit that only broke the syntax. The pin itself is refused by
    the lowering mutation.
* **Provenance.**
  * `prepare` requires unmodified tracked files, before and after the build, and a HEAD that did
    not move. It writes `provenance.json` with the commit and the module and control-table digests.
  * `run` refuses a module prepared at another commit, a module changed since `prepare`, or a
    `controls.json` changed since. All three refusals happen before any node launch; each exited 1
    with its message.
  * Each record carries `candidate_commit` and `prepared_commit`, and the validator requires them
    equal.
* **Input.**
  * The record states `input_signal: "tone"` and two structured feeds:
    * `input_feed`: a streamed source, continuous across blocks, track phase 0;
    * `native_input_feed`: a frozen block per track, not continuous, track phase 0.31 rad.

    The validator pins exactly those differences, and pins equal rate and amplitude.
  * Both feeds come from constants, not transcription. `console-workload` now names
    `TONE_RADIANS_PER_FRAME`, `TONE_TRACK_PHASE_RADIANS` and `TONE_AMPLITUDE`. `source_block`
    uses them, with the same `f32` values. The example emits the native feed, and the harness
    streams its tone at the native rate and amplitude read from that feed. The unread right
    plane's expression is unchanged.

### Gate 4: digests unchanged

* **Native row.** `bench console --preflight` prints exactly `after-1003`'s seven preflight
  digests. An untimed scratch replay of the three arms over the frozen 1000 observations gave
  `after-1003`'s quiet, restated and automated digests: `ee0c17bf…`, `ee0c17bf…` and `7b20684b…`.
  The collapse counters were [8512, 8], [3192, 8] and [3192, 8].
* **Standing rows.** An untimed replay of all 17 session rows, taken exactly as
  `SessionMeasurement` takes them, gave `after-1003`'s 17 digests. This covers the renamed tone
  constants. The other record kinds share `source_block` and no other code changed.
* **Browser arm.** `prepare` built `9ac37ae7…` at `ac2c6eec`, as Sol's rebuild did. It is still
  not the pin `8934cdd9…`, and the record says so. The preflight's seven digests, and both
  rounds' quiet, restated and automated digests (`9b1ed9ff…`, `9b1ed9ff…`, `c5fb4e46…`), equal
  `after-1003`'s.

### The browser run (descriptive, uncontrolled)

`run … --step after-1011` with `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, pinned to CPU 31. The lock
was held 23:00:29-23:00:39, with load average 15.5 at the start and 18.1 at the end. Node 22.23.2,
`--no-liftoff`.

µs per block:

| round | quiet p50 / p95 / p99 | restated | automated | ramp Δ | collapse Δ |
|---|---|---|---|---:|---:|
| 1 | 156.0 / 241.1 / 282.5 | 189.6 / 303.8 / 344.8 | 215.8 / 342.0 / 394.6 | 26.3 | 34.3 |
| 2 | 231.1 / 253.7 / 326.6 | 284.4 / 311.9 / 357.8 | 322.7 / 352.1 / 391.4 | 33.6 | 48.6 |

* Round 1 matches `after-1003` (154.3 / 189.2 / 216.6) within noise.
* Round 2 ran about 1.5× slower across all three arms, at load 18-19. The alternation shares that
  drift between the arms, but the absolute numbers of this round are not comparable to round 1.
  Nothing was retried.

### Checks run

* `cargo fmt --all -- --check`.
* `cargo clippy --locked --workspace --all-targets -- -D warnings`.
* `cargo test -p console-workload -p bench`: all pass, including the new test.
* `scripts/test-console-benchmark.sh`: PASS.
* `check-env-vocabulary` (no new names), `check-workspace-policy`, `check-bench-policy`,
  `check-realtime-policy`, `check-effect-runtime-policy`, `check-lane-policy` and
  `check-session-policy`: ok.
* `cargo check -p wasm-console-guest --target wasm32-unknown-unknown`: ok.

### For the verifier

* The sweep scripts are scratch and not committed. Their outcome is above.
* The browser records carry no percentile of the warmup launch.
* Round two's slowdown is host load (18-19). Its digests agree with round one's.
