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

## Sol attempt 1 verdict: PASS

Verifier: Sol, 2026-09-27, on `e767bbf4` merged onto the current batch head `9693f577` (clean). That
head now carries #1004, #1012, #995, #1002 and #1009. Host EPYC 7313P, rustc 1.97.1, Node v22.23.2,
`CARGO_INCREMENTAL=0`, a scratch target.

**What reproduces on the merged tree.**

* `cargo fmt --check` and `cargo clippy --locked --workspace --all-targets -- -D warnings` are clean.
* `cargo test -p console-workload -p bench` passes.
* `test-console-benchmark.sh` passes.
* The eight policy scripts pass, including the artifact-leak check, and so does `cargo check` of the
  wasm guest.
* The aggregate validator accepts `after-1003` and `after-1004`. The web validator accepts
  `after-1011`.
* `bench console --preflight` prints the seven `after-1003` preflight digests, now with every
  cohort collapsed on every arm (#1004).
* `prepare` built `a383a188…` (the batch's module, with #1004) and recorded the merge commit.

**Gate 1: every rule I deleted turns the suite red.** Each rule was deleted alone, on a copy of
`scripts/`, and compile-checked first:

* my three #1003 rules, with the aggregate's two preflight fields taken separately;
* thirteen other mixing-record rules;
* ten web-validator rules, among them both cross-round rules, `prepared == candidate`, the pin
  honesty rule, rounds `{1,2}` and both feed phases.

All 29 are red, each by the mutation named for it. The six rules the evidence calls implied stay
green when deleted, and each is implied:

* **`.record` pin.** `mixing_automation_record_valid` is reached only through the dispatcher's
  `.record` branch.
* **The two object guards.** Without them, `keys_unsorted` returns indices for an array and errors
  on a scalar, so the record is still refused.
* **The aggregate's count of two.** Fifty records minus the 48 whose counts are pinned for the
  other seven record types leaves two.
* **The aggregate's kind for the row.** The per-record rule pins it.
* **`quiet`'s cross-round counters.** Per record, `quiet[0] = quiet[1] × 1064`, and `quiet[1]`
  equals `restated[1]`, which agrees across rounds.

The multi-field mutations are single-claim: they keep the dependent counts consistent, so that only
the target rule refuses them. The sweep confirms it.

**Gate 2.** My limiter-at-base − 0.5 mutation now fails only
`restated_pushes_exactly_the_held_bases`. The comment is narrowed as asked.

**Gate 3.**

* **Rounds agree on digests and provenance only, never on timings.** Round 2's 1.5× slowdown under
  load is therefore accepted, correctly.
* **`run`'s refusals** each exited 1 before anything was launched or created:
  * a provenance commit other than HEAD;
  * a changed module;
  * a changed `controls.json`;
  * a missing `provenance.json`;
  * a provenance record without the module digest;
  * a modified tracked file.
* **An existing record** is still refused at the start.
* **Disclosure.** I launched one full `run` outside `timing.lock` by mistake: about 10 s on CPU 31,
  at load 7.4. Its timings are discarded. Its two rounds reproduce `after-1011`'s digests on the
  merged module, and the validator accepted them. The protocol therefore works end to end with
  #1004 in.

**Gate 4.** No rendered bit moved (above). `source_block`'s constants are literal-identical.

### Findings

1. **LOW-MEDIUM: a post-run validator refusal destroys the evidence.**
   * **What happens.** The rounds are written only to a `mktemp -d` that the EXIT trap removes.
     When `web-mixing-automation-validator.jq` refuses them, the runner prints one generic line and
     exits 1. It deletes both measured records and writes no disposition.
   * **Why it matters.** AGENTS.md asks that the raw output be preserved and the failure recorded.
     The console runner keeps `raw.jsonl` and a disposition.
   * **Failure scenario.** The two V8 rounds disagree on a digest, which is the one failure the new
     agreement rule exists to catch. The one authorised run is consumed, and nothing says which
     digest differed.
   * **History.** #1003's single `jq` check behaved the same way, but #1011 made the post-run check
     the protocol's main gate.
   * **Fix.** Keep the rounds as `web-mixing-automation.raw.jsonl` (or keep `$raw` and print its
     path) on refusal.
2. **LOW: `set -o noclobber` is now dead code.**
   * **What changed.** The record is written with `cp`, which `noclobber` does not govern. #1003
     wrote it with a redirect, which refused an existing file at write time.
   * **Why the impact is small.** The stderr log is created before the rounds and checked at the
     start, so it still refuses a concurrent run in all but a millisecond window.
   * **Fix.** Write `cat -- "$raw/rounds.jsonl" >"$record"` under `noclobber`.
3. **LOW: provenance guards against accidents, not against tampering.**
   * **Re-forged provenance.** A `provenance.json` re-forged to match a garbage module passes, and
     `node` is launched on it.
   * **No post-run check.** `run` does not re-check HEAD and the clean tree after its three
     launches, although each launch re-reads the tracked harness, fixture and ABI layout.
     `prepare` does re-check.
   * **Fix.** Repeat `require_clean_tree` and the HEAD comparison after the rounds.

None of these weakens what the row or its gates measure. All three are runner follow-ups.

## Follow-up: runner record keeping

Sol's three runner findings, fixed on `codex/1011-runner-keeps-records` from `b4e44e98`. Terra.

1. **A refused run keeps its rounds and says why.**
   * The records are now kept as `web-mixing-automation.refused.jsonl` whenever the run is refused
     after a measured round: a validator refusal, a failed launch, or a post-run check.
   * The runner prints the reason, logs it, and exits 1.
   * The claims moved to `scripts/web-mixing-automation-lib.jq`. Each is named, and
     `web_mixing_refusal_reasons` lists every failed claim, every missing or unexpected key, a
     wrong round set, and each field the rounds disagree on. `web-mixing-automation-validator.jq`
     is now the one-line verdict.
2. **Nothing is overwritten.**
   * The record, the refused file and the log are all checked before any launch.
   * Every write now goes through a redirect under `noclobber`, and the `cp` is gone. A record
     that appears during the rounds is refused at the write, left as it was, and the rounds are
     kept.
3. **Provenance is re-checked after the rounds.** After the last launch the runner checks HEAD, the
   tracked files, the module, `controls.json` and `provenance.json` again. It compares them with
   what it read before the rounds, and a change refuses the run and keeps the rounds.
   * Limit: `run` checks the files against `provenance.json`, never the provenance against a
     build, so forging all three consistently before `run` still passes. The record states the
     module digest, and `prepare` is reproducible, so a rebuild is the check for that.

**Tests.** `test-console-benchmark.sh` runs the real runner, untimed, in throwaway git repositories
whose harness is a stub. The stub prints the base browser round, after the one disturbance the case
names. The cases are:
* an undisturbed control;
* rounds that disagree;
* each of the three artifacts already present;
* a record that appears during round 2;
* HEAD, a tracked file, the module or the provenance changing during round 2.

With `b4e44e98`'s runner swapped in, 20 cases fail, covering all three findings. The control still
passes, so the fixture is not what fails. With this runner, all pass.

**Checks.** `test-console-benchmark.sh` passes. The env-vocabulary, workspace, bench, realtime,
effect-runtime, lane and session policy scripts pass, and so do the bench-preconditions self-test
and the artifact-leak check. No Rust changed. `after-1011`'s rounds still pass the verdict.
