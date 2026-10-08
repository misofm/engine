PASS

# #1476 attempt 1 -- adversarial verdict

Commit `0e01f7b4c` (parent `859fb98a8`), worktree `/home/bl/misofm/wt-d15-misc`. I reviewed an exported tree
(`git archive`) and never used the worktree. #1480's uncommitted edits in the worktree are not part of this review. The diff
touches only the authorized paths: `sdk-response-entry.ts`, `run.mjs`, `hosts/host-web/MUTATIONS.md`, and the spec,
where it only adds the attempt record (`diff -rq` of the parent export against the head export lists these four files and no others).
There is no engine, SDK or worklet change. The rebuilt module is `37274bf13d0e...`, the same as the module that the implementer reports.

## Decisions

- **D1 holds.** All seven `createEngine` sites (`sdk-response-entry.ts:309, 424, 484, 1160, 1435, 1610, 1851`)
  go through `createSdkEngine` (`:72`). `browser.close()` occurs only inside `closeSdkEngine` (`:111`). No other
  path makes a worklet instance: `createDefaultHost` is used only as the `createHost` option of those calls.
  `createResponsePreview` makes a dedicated Worker, and the SDK's scratch boot is a Worker with no render-locked
  count. `candidate.close()` in `engine.ts:537` closes a rejected `AudioContext`, not a host. The headless
  `WasmBoundary` (`:1199`, `headless.dispose()` `:1426`) has no row, as the Hazards section requires. The
  call order is fully sequential (no `Promise.all`), so the 17-label read order is deterministic. This order
  agrees with the frozen list (resident, track-response, sdk-observation, 3 queries, the closed query, hop-256, the collection,
  hop-1024 and 7 live-bypass renders in the order of `:1930-1936`).
- **D2 and D4 hold for every offline instance (15 of 17).** The read comes after `startRendering()` resolves
  and before close. A nonzero `result` is a refusal. Rows are `{workload, count}`, and the entry returns
  `renderAllocationInstances`. See MINOR 1 for the two live `AudioContext` instances.
- **D3 holds.** I tested it: in a copy of the entry, I made the first live-bypass body throw and its read reject.
  The qualification error was the body's error (`verifier: planted body error`) with
  `renderAllocationReadFailures: ["live-bypass:authored-none: render allocation count failed: verifier:
  planted read failure"]` attached (`logs/d3-chromium.log`). For a body error with a good read, the row is pushed and the body error
  propagates. For a refused read with no body error, the refusal is thrown. A mid-body refused read (the spectrum loop
  `:935`, sdk-observation `:1707`) propagates because the instance has already left the `open` map.
- **D4: no missing or duplicated read can satisfy the predicate.** I ran `validateSdkRenderAllocations` (taken from
  `run.mjs`) in node against crafted row sets. All of these are red: a duplicate row in place of a
  missing row, an extra duplicate with the instance count raised to match, a missing row with the
  instance count lowered to match (the frozen list catches this), swapped order, a missing or string `"0"`
  count, a missing instance count, missing rows, a null response, and empty rows. Only the clean set is green. The 17 labels
  are distinct, so a duplicate can never stand in for a missing row.
- **D5 holds.** Both mutations start with `sdk-` and are filtered out when `sdkResponse === null` (`run.mjs:762-764`).
  A run without `--sdk-root` ran the mutation proofs without them.

## MINOR

**m1. For the two live `AudioContext` instances, the read comes before their last render, so D2's literal
text does not hold. Strict compliance would not catch any more defects.**
`spectrum-collection` (`sdk-response-entry.ts:1150-1152`) and `spectrum-continuous-hop-1024` (`:859-862`)
continue to render after `closeSdkEngine` reads the count. The renders stop only when `browser.close()` makes the worklet process
`host.dispose()`. D2 says "The read happens after the instance's last render". The attempt record discloses this
("their read is immediately before close and covers every quantum up to it"). Measured evidence: in
a copy of the entry, a 250 ms wait after the read let both live contexts render 91 more quanta
(`currentTime` 0.072 -> 0.315 and 0.176 -> 0.419, state `running`). With an allocation planted in the
render-side spectrum `capture()` one-shot branch after a continuous stream has run (`started_epoch != 0`), hop-1024's
row read 0 at the committed read point, and its count reached 182 after the wait (`logs/e4c-*.log`). The counter did not see these renders.
But when I suspended the context immediately after the read, it rendered 0 more quanta, and the count did not change in 3 of 3 Chromium
runs (`logs/e4b-*.log`). Any strict form (suspend, then read) stops those renders. It does not count them. So no plausible
defect is red under the strict form and green under the committed form: the gap is in the claim, not in
detection. Fix either (a) the code: in `closeSdkEngine`, if the context is a running `AudioContext`,
`await suspend()` before the read. This works in all three browsers, because both live probes already
boot and subscribe while the context is suspended, and my second read after `suspend()` was answered. Or fix (b) the text:
amend D2 to say "after the instance's last render, or for a running AudioContext immediately before close,
covering every quantum up to the read". Related observation (out of scope for this issue): no design that reads before `host.dispose()`
can cover the renders inside the SDK's own `close()` sequence (meter feeds closed, observations invalidated,
response workers closed) while a live context still runs. Only an SDK-side read could reach them, and the Non-goals
exclude an SDK change.

## NIT

- **n1. The committed self-test does not pin the gate name.** `mutationProofs` accepts any error that starts with
  `<browser>:` (`run.mjs:768`). This applies to every existing mutation, so it is not a regression. I confirmed
  gate 2's "with the gate named" claim by instrumentation (see below), and it holds.
- **n2. D3 does not cover a few throws before the try or in the finally.** `runSpectrumCollectionQualification` throws "spectrum
  collection fixture is incomplete" (`:996`) after its engine exists and before its `try`. The builders'
  `browser.host.node.connect` follows `createSdkEngine` with no try. In the finally blocks, a throwing
  `shared?.close()` or `subscription?.close()` (`:731-732`, `:861`) skips `closeSdkEngine`. Each of these
  leaves an instance with no read and no close. The run fails anyway, and the code had the same shape before this change.
- **n3. A non-object body error loses a read failure.** If the body throws a non-object, a read failure is dropped and not attached (`:105-111`).
  There is nothing to attach it to. Accepted.
- **n4. `createEngine` still appears at seven sites.** `createSdkEngine` takes a thunk, so `createEngine(...)` remains at seven call sites. A future raw
  call looks the same as a wrapped call, and it would escape the instance count, the rows and the frozen list. No runtime
  check can see an unregistered instance. If the helper took the options and called `createEngine` itself,
  the file would reference `createEngine` once.

## Test value

- `sdk-render-allocations` (gate) goes red in two cases that no existing gate catches. The first is a render-thread allocation on a render-locked
  path that only an SDK instance reaches. I replicated gate 3: I planted `drop(black_box(Vec::<u8>::with_capacity(1)))` in
  `EffectControlLane::stage`'s `Bypass` arm and rebuilt the module (`f1baf065...`). With the SDK leg, the gate was red, with
  `live-bypass:authored-none:live-desk-hi-bypass=2 ...:live-ins-mid-bypass=2 ...desk-hi-lift=2
  ...ins-mid-lift=2`. Without `--sdk-root`, all ten raw `render-allocations` rows were `=0`. The raw workloads send only
  `COMMAND_MATRIX` commands (`qualification.js:347, 562`). The second case is an instance that closes without its read. I replicated gate 4:
  `closedBrowser.close()` replaced both `closeSdkEngine(closedBrowser)` calls, and the run was red with "16 rows for 17 instances".
- Self-test mutations `sdk-render-allocations` and `sdk-render-allocations-missing`. With the new gate, both are red in
  Chromium, Firefox and WebKit, and the error names the gate ("...allocated on its render thread:
  resident-observation=1" and "...16 rows for 17 instances"). With `validateSdkRenderAllocations` disabled,
  both mutations escaped in all three browsers (`logs/e1-*.log`). So no existing gate catches them.

## Gates run (exported tree, CI invocations)

- Artifacts: `scripts/build-web-audioworklet.sh --named-twin <named> <artifacts>` -> module `37274bf1...`.
  Each leg: `npm ci` in `sdk/` and in `qualification/`, a private pulseaudio null sink as `qualification.yml` sets up, and
  `npm run qualify -- --artifacts <a> --sdk-root <tree>/sdk --browser <b> --check-matrix --self-test-mutations`.
  The log says "sdk bundle: the source at .../sdk/src (CI's mode)". `sdk/dist` did not exist during the browser runs.
- Gate 1. Chromium 151.0.7922.34: pass. WebKit 26.5: pass. Each prints all 17 SDK rows `=0` and the ten raw rows `=0`.
  Firefox 153.0: the first two runs failed on `sdk-spectrum-continuous: ... false: gap`. This is the #1480 flake, a non-goal here:
  the predicate is computed in the body before the new read, and the diff does not touch it. Another agent's Firefox qualification was running at the same
  time. Then I ran base `859fb98a8` and head alternately, 4 runs each, and all 8 passed. The head runs printed all 17 SDK rows `=0`.
- Gate 2. Confirmed above (instrumented copy of `run.mjs`, all three browsers).
- Gate 3. Replicated above. Gate 4. Replicated above.
- Gate 5. `check-workspace-policy.sh` ok. `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`,
  `check-sdk-headless.sh` (362/362) and `sdk-package.sh check` all exit 0. I deleted the `sdk/dist` that `sdk-package.sh check` left.
- Extra. An ad-hoc `tsc --noEmit` of the entry (CI does not typecheck it) gives the same 20 pre-existing
  implicit-any errors on base and head. The change adds no type errors.
- Acked-batch question: no queue is involved. The rows are pushed only after a `result === 0` reply, and a refused or
  failed read is thrown or attached. It is never dropped while a row is recorded.

Evidence logs: `/tmp/claude-1002/v1476/logs/`.
