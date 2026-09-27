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
