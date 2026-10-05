VERDICT: PASS

Issue #1248, attempt 1, commit `3854c03bb` (base `0e3e21b68`), decision-15 stream J.
Verifier: adversarial verifier, 2026-10-05. Every build and run below was done in an export of
`3854c03bb` at `/tmp/claude-1002/v1248/src`, with planted runs in a second copy at
`/tmp/claude-1002/v1248/plant`. The worktree `/home/bl/misofm/wt-d15-j` was only read. Its one
modification, `tools/bench-support/src/producer.rs`, belongs to the other implementer.

## Findings

No BLOCKER. No MAJOR. No MINOR.

### NIT 1: the recorded gate-2 plant delays the race's signal, not `onUpdate`
Location: spec attempt record, "Gate 2", `.github/ISSUE_SPECS/1248-...md:113-121`.

Plant A only removes `callbackResolve` from `onUpdate` and resolves `callbackSeen` 500 ms after
`startRendering()` returns. Delivery itself still happens on time. The spec asks for "a 500 ms
delay before the first `onUpdate`". I ran a closer plant (B): every `onUpdate`, automatic or
pumped, is held until 500 ms after rendering returns and then replayed in order. The fix passes it
in chromium, firefox and webkit. The old code fails it with
`false: automaticDelivery, windows >= 1 (windows=0), gap`, which also shows the old 48-pump loop
giving up before any window is delivered.

Fix: optionally add plant B to the record as the stronger gate-2 evidence. The code does not
change.

### NIT 2: the 10 s deadline literal is duplicated
Location: `hosts/host-web/qualification/sdk-response-entry.ts:567`; the hop probe has its own
`10_000` at `:736`.

The comment couples the two values ("the hop probe's 10 s"). The hop probe is #1106's path, so it
is outside #1248's authorized paths.

Fix: when #1106 lands, possibly in the same PR, it can hoist one shared spectrum-deadline constant.
Not required here.

## Scope judgement: deleting `CONTINUOUS_BLOCKS`
This is acceptable. `git grep` at the base shows exactly two uses: the definition (`:18`) and the
replaced loop (`:573`). The authorized change made the constant dead. The path limit exists so the
change does not touch other probes, and removing a constant that only the replaced loop used does
not touch them. Keeping it would leave dead code, which the no-shortcuts principle argues against.
The attempt record discloses the deletion. The entry is bundled by esbuild with no type check, so
nothing forced the deletion, but it is the clean outcome.

## Review against the spec

- **Slice 1 (name the false predicate): met.** In `run.mjs:484-500` the pass condition is
  `false-list empty`. That is logically identical to the old conjunction: the reads are pure JSON
  property reads with no side effects, and a missing `continuous` still yields all-false. The
  message keeps the old text and adds `; false: <names>`. `windows` carries the observed count.
  Only the one `sdk-spectrum-continuous` lifecycle gate changed.
- **Slice 2 (deadline): met.**
  - One deadline (10 s) starts after rendering returns.
  - The race timer uses the same span in the same tick, and is cleared once the race settles.
  - The ready loop exits on the same two success conditions, or at the deadline.
  - The loop yields 5 ms between pumps.
  - No pump happens before the race, so `automaticDelivery` still proves timer-driven delivery
    only.
  - The typed `did not publish a window` failure and every predicate are unchanged.
  - Worst case is bounded at about 10 s, not 20 s: plant E below.
  - The page's default timeout is 120 s (`run.mjs:806`), so the typed failure surfaces before any
    locator timeout.
- **Non-goals: respected.** There is no SDK, worklet, engine or hop-probe change and no change to a
  pass condition.
- **Normal path is unchanged in cost.** I instrumented the unplanted new code. The race resolved in
  4 ms (chromium), 9 ms (firefox) and 17 ms (webkit). The loop took 0 ms and exited on its first
  check (1 iteration) in all three browsers.
- **AGENTS.md:** this is harness JavaScript only. No render-path or realtime rule applies, and
  `setTimeout` and `performance.now` are portable across the three engines.
- **Evidence honesty:** the record's claims match my reproduction:
  - shipped module digest `a9a51862...`;
  - all three browsers pass;
  - gate 2 is red on the old code and green on the new;
  - gate 3 names the predicate;
  - gate 4 is bounded (I measured 10004 ms; the record says 9994 ms of loop wait).

## Gates run (export)

1. **Gate 1.** I built the artifact as the `artifact` job does: shipped module
   `a9a518625f4c0621c00a515be5a1c50b257601e9a7e492dbac45dbbcb5637b55`, matching the record. Then I
   ran `npm ci --no-audit --no-fund` in `sdk/` and `npm ci` in the qualification directory. The
   `browser`-job invocation (`--sdk-root ... --check-matrix --self-test-mutations`) ran on a
   private PulseAudio null sink, as in CI:
   - chromium 151.0.7922.34: `all qualification gates passed` (5.6 s).
   - firefox 153.0: `all qualification gates passed` (12.8 s).
   - webkit 26.5: `all qualification gates passed` (7.5 s).
2. **Gate 2 (slow automatic delivery).** New `run.mjs` was used for both the new and the old entry:
   - Plant A, the implementer's (`callbackSeen` resolved 500 ms after render):
     - new: chromium passes;
     - old: chromium fails with `...; false: automaticDelivery`.
   - Plant B, mine (every `onUpdate` held until 500 ms after render, then replayed):
     - new: chromium, firefox and webkit all pass; instrumented race wait is 499-500 ms and the
       loop takes 0 ms;
     - old: chromium fails with `...; false: automaticDelivery, windows >= 1 (windows=0), gap`.
3. **Gate 3 (named predicate).** Plant C forces `staleReadRefused = false`.
   - New `run.mjs`: chromium fails with
     `sdk-spectrum-continuous: continuous spectrum did not prove warmup, shared ownership, capture
     loss, or close lifecycle; false: staleReadRefused`.
   - Same plant with the base `run.mjs`: the old unnamed message, with no `; false:` suffix.
4. **Gate 4 (never publishes).**
   - Plant D (`pump` returns `undefined`, `readLatest` returns `undefined`):
     - new: the typed `continuous spectrum did not publish a window` 10004 ms after render;
     - old: the same error after only 6 ms, because the 48 pumps returned immediately. That is
       the poll-count defect itself.
   - Plant E (D plus no automatic delivery at all):
     - new: the typed error 10005 ms after render. The two waits share one deadline, so the bound
       is about 10 s, not 20 s.

Every plant was applied only to the plant copy. The copy was restored and checked against the
export with `cmp` afterwards.

## Test value

No test is committed. The spec makes gates 2-4 PR evidence, and the qualification gate is itself
the check.

- **Rewritten lifecycle gate (`run.mjs:484-500`):** a run in which exactly one lifecycle
  predicate is false (e.g. `staleReadRefused`) now names that predicate. The old gate reported only
  the fixed sentence, which is the #1238 Firefox diagnosis gap.
  - Reproduced: plant C is named with the new `run.mjs` and unnamed with the base `run.mjs`.
  - The pass condition is unchanged, so the gate catches the same set of failures.
- **Deadline-bounded waits (`sdk-response-entry.ts:565-586`):** a correct engine whose first
  automatic delivery lands more than 100 ms after rendering, or whose publication needs more than
  48 immediately-returning pumps, no longer fails. A subscription that never publishes still fails
  with the typed message within about 10 s.
  - Reproduced: plants A and B are red on the old code and green on the new code.
  - Reproduced: plant D gives up after 6 ms on the old code and is bounded at 10 s on the new code.
