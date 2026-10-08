PASS

# #1480 attempt 1 -- adversarial verdict

Commit `e0804b9b2` (parent `9dd5ae182`), worktree `/home/bl/misofm/wt-d15-misc`. I reviewed an exported tree
(`git archive e0804b9b2`) and did not use the worktree. `git diff 9dd5ae182 e0804b9b2` touches four files, all
authorized: the spec (attempt record only), `hosts/host-web/qualification/sdk-response-entry.ts` (inside
`runContinuousSpectrumQualification` only), `hosts/host-web/qualification/run.mjs` (one new predicate in the
`sdk-spectrum-continuous` list) and `hosts/host-web/MUTATIONS.md` (two new rows). There is no engine, SDK or worklet change.

I rebuilt the qualification artifacts in the export with the CI command
(`bash scripts/build-web-audioworklet.sh --named-twin ... target/ci/qualification-artifacts`). All 7 files are
byte-identical to the artifact that the implementer used again (module `6175e70c63e5...`), so the implementer's
runs apply to this commit.

## Decisions

- **D1 holds; the root cause is coherent.** I read the implementer's raw traces (root scratchpad `i1480/d1/trace-*.json`).
  Run 21 (fail): a read sent at the render start returns `status 6, windows 1, dropped 0, end 2048`. It pops
  window 0 before window 1 exists. The first notification is `ready 0/0`, and the wait loop exits. The next read
  returns `status 3, dropped 15`, after `gap` was computed. Run 40 is the same. Run 16 (gap true): the first
  read during the render returns `status 3, dropped 6`. The difference is only this: does a native read pop the queue
  while the render is between window 0 (frame 2,048) and window 1 (frame 2,304)? The native code agrees:
  `try_read_continuous_record` (`crates/host-core/src/spectrum.rs:749`) reports `Gap` before it pops, and only
  when `drops != seen_drops`. `continuous_publish` (`:1973`) increments `drops` on each failed push into the
  one-slot queue (`SPECTRUM_RESULT_SLOTS = 1`, `:43`).
- **D3 holds.** In run 21, all 15 native drops reach the next SDK notification (`missed 15, skipped 1`).
  The SDK loss accounting (`observation-subscriptions.ts:1650-1696`) is correct, and the SDK evals
  (`sdk/test/spectrum-evals.mjs`, 20 `nativeMissedWindows` assertions) cover the interleaved cases, so
  holding reads in this probe removes no unique product coverage. The defect is the probe's race, not a counter defect.
- **D2 holds, and the arithmetic is right.** The window start S is the first observed block (sample 0). Windows end at
  S+2,048+k*256, and the last ends at 6,144 (`OfflineAudioContext(2, 6144, 48000)`). So there are
  (6,144-2,048)/256+1 = 17 windows. One goes into the one-slot queue, and 16 are dropped. The host-core unit test
  `continuous_held_record_remains_immutable_after_wrap_and_drop` (`spectrum.rs:4031`) gets `Gap { dropped_captures: 16 }`
  for the same 2,048+4,096 frames at hop 256. The mechanism is certain by construction:
  - `readsHeld` is set (`sdk-response-entry.ts:643`) before the probe waits for the in-flight reads (`:644`). Any
    wrapper call after that awaits the hold (`:582`). A read that started earlier called the native method
    synchronously and is in `inFlightReads`. So no native read message is in flight or sent while
    `startRendering()` runs.
  - The SDK sends every spectrum read as `host.readSpectrumStream(...)` (`sdk/src/browser/engine.ts:884`). The lookup
    occurs at call time, so the wrapper is always in the path. The SDK poll is single-flight (`#polling`,
    `observation-subscriptions.ts:900`), so a held read also stops the 1 ms timer from sending more reads.
  - The wrapper's `push` runs before the `allSettled` continuation (the reactions are registered in that order),
    so `readsBeforeRender` (`:645`) counts every pre-render read, and `nativeReads[readsBeforeRender]` is the first read after the render.
- **D4 holds.** There is no sleep (the hold is a promise), no retry, no rerun, no change to the 10 s deadline,
  no change to `gap` (`:698-701`, `run.mjs:566` still requires `true`), no browser branch and no skip.
- **No deadlock.** `releaseReads()` is in a `finally` around `startRendering()` (`:647-651`). Nothing between `:643`
  and that `try` can throw. `allSettled` waits only for reads that were already sent, and never for held reads.
  Pre-render spectrum requests always get a reply: the stream start and every warming read in all traces reply before the render.
  The outer `finally` closes the subscriptions after the hold is released.
- **The same run still proves the other properties.** Pre-render warming reads publish nothing (only `gap`/`failed`
  replace a publication, `observation-subscriptions.ts:1641`), so `callbackSeen` can resolve only from a
  post-render automatic poll. `automaticDelivery` is decided before the pump loop starts. `sharedJob`, `ownedArrays`,
  `sharedAfterFirstClose` and `staleReadRefused` are unchanged, and they passed in every run. `sdk-spectrum-recovery` now always sees `16/1`.
- **#1476 is intact.** `createSdkEngine`/`closeSdkEngine` are not touched (`git diff 0e01f7b4c e0804b9b2` adds only
  #1479 and #1480 hunks). Each of my 100 green runs prints
  `sdk-render-allocations` with 17 rows, all 0, and the #1476 self-test mutations pass inside `--self-test-mutations`.
- **Acked-batch question.** This change adds no queue and no ack. The native queue reports `Gap` before it pops
  and never discards a drop count.

## Findings

No BLOCKER, MAJOR or MINOR findings.

- **NIT 1 -- Gate 4's CI half is still open.** The spec requires "CI's three browser legs pass". There was no push
  (batch mode), and the record says this. Root must confirm the three `qualification.yml` browser legs at the batch
  push before it closes #1480.
- **NIT 2 -- the D1 evidence example and its storage.** The attempt record uses run 16 as the "typical pass".
  That run failed the full qualify command on `sdk-spectrum-hop` (the record names this under Open items). The
  continuous-gate facts that it quotes are correct (trace-16: `gap: true`, `status 3, dropped 6`), but a run that
  passed every gate would be a clearer example. The raw traces are only in the root session scratchpad
  (`i1480/d1`, 50 traces). Keep them in a durable location, or accept the record's summary as the evidence.
- **NIT 3 -- the pre-render settle has no bound of its own** (`sdk-response-entry.ts:644`). If a worklet reply
  to a pre-render read were lost, the run would fail on the next locator's 30 s timeout, not on a named predicate.
  I do not request a change. Every run shows these replies arrive before the render, and the probe already depends on
  pre-render replies (stream start, source acknowledgements). A wall-clock bound would add a timing assumption.

## Test value

- **`renderLoss`** (`run.mjs:570-571`; new): it turns red when a native spectrum read reaches the worklet during the
  offline render (the hold is removed, bypassed or opened early by a probe or SDK read-path change). That condition
  silently brings back #1480's race. It also turns red when the browser path's loss count is not the
  17-window, one-slot result. No existing predicate catches this reliably: with the hold removed, `gap` stayed green
  on 22 of my 25 Firefox runs, and every other continuous predicate stayed green on all 25. The native arithmetic itself
  is already covered by the host-core unit test above, so the unique value of `renderLoss` is the probe's premise in the real browser path.
- **`gap`** (unchanged, now deterministic): it turns red exactly when the SDK hides a real native loss
  (mutation 1: 15 of 15 red, `false: gap` alone). The spec's test-value claim is true: green in 110 of 110 unmutated runs.

## Gates run (all sequential, private PulseAudio null sink per run as in `qualification.yml`)

Command per run: `npm run qualify -- --artifacts <export>/target/ci/qualification-artifacts --sdk-root <export>/sdk
--browser B --check-matrix --self-test-mutations`. The SDK is in source-bundle mode ("sdk bundle: the source ... (CI's mode)"), and there is no `sdk/dist`.
The machine is a 32-thread AMD EPYC.

| Gate | Result |
|---|---|
| 2: Firefox 153.0, fix | 50 of 50 pass every gate; leg mean 16.5 s (max 17.7) |
| 2: Chromium 151.0.7922.34, fix | 25 of 25 pass; mean 5.7 s |
| 2: WebKit 26.5, fix | 25 of 25 pass; mean 8.1 s |
| Extra: Firefox, fix, 24 `yes` CPU hogs running | 10 of 10 pass; mean 17.4 s (max 22.7) |
| 3a: SDK `nativeMissedWindows: 0n`, `skippedPublications: 0n` (`observation-subscriptions.ts:1693-1694`) | red 5/5 Chromium, 5/5 Firefox, 5/5 WebKit, each `false: gap` alone (`renderLoss` true) |
| 3b: hold removed (`:582` and `:644` deleted), Firefox | red 24 of 25: 21 `renderLoss` alone (status 2 with 0 dropped ×8; status 3 with 2-14 dropped ×13), 3 `gap, renderLoss (status=6, dropped=0)` (the D1 race, at the stream G verifier's 3-in-25 rate); 1 green |
| 4: `--self-test-mutations` | passed in all 110 fix runs (3 browsers); CI legs not run (NIT 1) |
| 5: `bash scripts/check-workspace-policy.sh` | `workspace policy: ok` |

Observation for root (not a finding): the implementer saw `sdk-spectrum-hop` and `sdk-spectrum-collection` fail
once each in 50 pre-change Firefox runs. Neither occurred in my 110 runs. Their causes are not known, and they are outside
the scope of this issue. Root must decide whether each needs a successor.

Evidence kept: `/tmp/claude-1002/v1480/runs/*/summary.txt` and per-run logs; build log `/tmp/claude-1002/v1480/logs/build.log`.
