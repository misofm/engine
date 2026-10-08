# Read every SDK qualification instance's render allocation count before it closes

Stream H follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-10). Filed
2026-10-08 by root from the #1333 attempt-1 verdict, MINOR 3 (open item 1). #1333 gave every
worklet instance a render-locked allocator count and made the raw qualification workloads read
it; the SDK-path instances in the browser qualification still do not read it, and the real
render-thread allocation #1333 found was in one of them.

No engine source changes. No rendered bit moves.

## Problem (verified on `main` at `1e78d7820`)

- **The count exists and the raw workloads read it.** `MisoAudioWorkletHost.renderAllocationCount()`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1318`) asks the worklet for its
  per-instance count of allocator calls inside the render-locked exports. The raw workloads in
  `hosts/host-web/qualification/qualification.js` read it before each `dispose()` through the
  helper `renderAllocationCount(host)` (`:181-185`; call sites `:168`, `:363`, `:470`, `:579`,
  `:708`), and `run.mjs`'s `render-allocations` gate holds exactly ten rows to zero
  (`hosts/host-web/qualification/run.mjs:239-246`).
- **The SDK instances do not read it.** `hosts/host-web/qualification/sdk-response-entry.ts`
  creates its engines with the SDK's `createEngine` (`:243`, `:357`, `:417`, `:1083`, `:1356`,
  `:1529`, `:1757`; several helpers run more than once: one spectrum browser per query, one
  live-bypass render per variant). They cover the spectrum query, the continuous spectrum stream,
  the configured hop-1024 boot, the spectrum collection, the track-response subscription, the
  resident and SDK observations and live bypass. Each closes with `browser.close()` (for example
  `:664`, `:791`, `:1075`, `:1348`, `:1520`, `:1626`, `:1686`, `:1822`). The file never calls
  `renderAllocationCount`.
- **The SDK exposes the host.** `BrowserEngine.host` is the `MisoAudioWorkletHost`
  (`sdk/src/browser/engine.ts:192`, `:195`), so the entry can read the count without an SDK
  change.
- **Why it matters.** #1333 found the real render-thread allocation (the collection capture's
  cloning accessor) in an SDK collection instance; the raw `staging-reads` workload was added to
  reach that one path. Any other render-locked export that only an SDK instance reaches (the hop
  boot, the live-bypass command path, the continuous stream with its subscription cadence) has no
  runtime allocation proof in the browser.

## Decisions

- **D1. One helper owns every SDK instance.** Every `createEngine` call in
  `sdk-response-entry.ts` goes through one helper that takes a stable workload label and records
  the instance. One closing helper reads `browser.host.renderAllocationCount()` and then calls
  `browser.close()`. No SDK instance is closed any other way.
- **D2. The read, its refusal and its row.** The read happens after the instance's last render
  and before its close. A reply whose `result` is not `0` throws, as the raw helper does. Each
  read adds one row `{ workload, count }` to `sdkResponse.renderAllocations`. The entry also
  reports `sdkResponse.renderAllocationInstances`, the number of instances the creating helper
  recorded.
- **D3. Errors.** If an instance throws before it closes, the closing helper still reads the count
  when the host booted. A failed read never masks the error that came first: the first error is
  rethrown, and the read failure is attached to it.
- **D4. The gate.** `run.mjs` adds the gate `sdk-render-allocations` in `validateSdkResponse`:
  `renderAllocations` is an array, its length equals `renderAllocationInstances`, its workload
  labels equal one frozen list in `run.mjs` (in order, with repeats), and every `count` is `0`.
  The failure message names each nonzero row.
- **D5. Mutations.** `MUTATIONS` gains `sdk-render-allocations` (one row's `count` set to `1`) and
  `sdk-render-allocations-missing` (one row removed). Both start with `sdk-`, so they are skipped
  when the SDK leg is off, as the other SDK mutations are.

## Authorized paths

- `hosts/host-web/qualification/sdk-response-entry.ts` (the creating and closing helpers, their
  call sites and the returned rows only)
- `hosts/host-web/qualification/run.mjs` (the new gate, its two mutations and its stdout line)
- `hosts/host-web/MUTATIONS.md` (the new rows)
- this spec

## Non-goals

- Any change to `renderAllocationCount()`, to the worklet's count or to the raw workloads' rows.
- Any SDK API change; the entry reads `browser.host`.
- The continuous-spectrum `gap` flake (#1480, stream J, same file). This slice does not
  touch the continuous predicates.
- The hermetic test of the reply check (#1477).

## Hazards

- `sdk-response-entry.ts` is also edited by J #1480 (the continuous-spectrum gate). Either order;
  the later slice rebases (STREAMS hot-file row).
- `browser.close()` disposes the host, so a read after it fails. Read first.
- The headless engine in the track-response subscription (`headless.dispose()`, `:1347`) has no
  worklet and no count. Do not add a row for it.

## Objective gates

1. The browser qualification (`npm run qualify -- ... --sdk-root ... --self-test-mutations`, as
   `qualification.yml:445` runs it) passes in Chromium, Firefox and WebKit with the new gate. The
   log shows the SDK rows and their counts.
2. `--self-test-mutations` turns both new mutations red in all three browsers, each with the
   `sdk-render-allocations` gate named.
3. **The gate catches a real allocation that no raw workload reaches (PR evidence, not
   committed).** Plant one heap allocation (for example `drop(Vec::<u8>::with_capacity(1))`) in a
   render-locked export path that an SDK instance reaches and no raw workload reaches. The
   implementer names the path and shows why no raw workload reaches it. Rebuild the artifact.
   `sdk-render-allocations` is red with that workload named; `render-allocations` stays green.
   Reverted, both are green.
4. **Every instance is counted (PR evidence).** Bypass the closing helper for one instance (call
   `browser.close()` directly): `sdk-render-allocations` is red on the length check.
5. `bash scripts/check-workspace-policy.sh` and the SDK package checks (`qualification.yml`'s
   `sdk` job) pass.

*Test value.* The new gate is red when an SDK-only render-locked path allocates on the render
thread (gate 3), which the raw `render-allocations` gate cannot see; the length check is red when
an instance closes without its read (gate 4).

## Evidence

- Gate 1 logs (three browsers), gate 2 output, gate 3's planted path, its reason and both runs,
  gate 4's run.

## Dependencies

- After (same stream): none open (#1333 is on `main`).
- After (other streams): none. Either order with J #1480 (hot file).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: under half a day.

## Attempt record

### Attempt 1 (implementer, 2026-10-08, base `859fb98a8`)

Change:
- `sdk-response-entry.ts`: `createSdkEngine(workload, () => createEngine({...}))` is the one way
  the entry makes an engine (all seven `createEngine` sites); it counts the instance and records
  its label. `closeSdkEngine(browser)` is the one way it closes one: it reads
  `browser.host.renderAllocationCount()`, pushes `{ workload, count }` (a nonzero `result` is a
  refusal), then calls `browser.close()`; a second call for the same instance does nothing (the
  spectrum-query and SDK-observation bodies close mid-flow and again in `finally`). D3: each body
  gained `catch (error) { throw sdkEngineFailed(browser, error); }`, which records the body's
  first error; after one, a failed read is attached to that error as
  `renderAllocationReadFailures` and not thrown, so the body's error propagates. The returned
  object carries `renderAllocations` and `renderAllocationInstances`; the registry resets at the
  start of `runSdkResponseQualification`. Labels: `spectrum-query:<target>`,
  `spectrum-query-closed`, `spectrum-continuous-hop-<hop>`, `spectrum-collection`,
  `track-response-subscription`, `resident-observation`, `sdk-observation`, and
  `live-bypass:authored-<bypass>[:live-<target>-<bypass|lift>]`. The headless engine has no row.
  In the continuous-spectrum region (#1480's) only the `createEngine` line, its closing `}));`,
  the added `catch` and the close line changed.
- `run.mjs`: frozen `SDK_RENDER_ALLOCATION_WORKLOADS` (17 labels, read order);
  `validateSdkRenderAllocations` (gate `sdk-render-allocations`: array, length equals
  `renderAllocationInstances`, labels equal the frozen list, every `count` 0, the message names
  each nonzero row), called first in `validateSdkResponse`; mutations `sdk-render-allocations`
  (row 0 `count = 1`) and `sdk-render-allocations-missing` (row 1 removed); a stdout line
  `<browser>: sdk-render-allocations: <workload>=<count> ...`.
- `MUTATIONS.md`: four rows (gate 3, gate 4, the two self-test mutations).

Read timing: for the offline instances the read follows `startRendering()`; the two live
`AudioContext` instances (`spectrum-collection`, `spectrum-continuous-hop-1024`) keep rendering
until close, so their read is immediately before close and covers every quantum up to it.

Evidence (local, SDK source-bundle mode, no `sdk/dist`; artifact built by
`scripts/build-web-audioworklet.sh --named-twin ...`, module `37274bf1...`):
- Gate 1: `npm run qualify -- --artifacts <artifacts> --sdk-root <sdk> --browser <b> --check-matrix
  --self-test-mutations` exits 0 in chromium 151.0.7922.34, firefox 153.0 and webkit 26.5. Each log
  prints `<b>: sdk-render-allocations:` with all 17 rows `=0` (and the ten raw rows `=0`).
- Gate 2 (temporary uncommitted print in `mutationProofs`, all three browsers): `sdk-render-allocations`
  red with `<b>: sdk-render-allocations: an SDK worklet instance allocated on its render thread:
  resident-observation=1`; `sdk-render-allocations-missing` red with `<b>: sdk-render-allocations:
  an SDK instance closed without its render allocation read: 16 rows for 17 instances`.
- Gate 3: planted `drop(std::hint::black_box(Vec::<u8>::with_capacity(1)))` in
  `EffectControlLane::stage`'s `EffectControlRecord::Bypass` arm (`crates/effect-contract/src/live.rs`),
  rebuilt the module. Path: the render thread's control drain applying a live bypass record. Why no
  raw workload reaches it: the raw workloads' only `command()` records are `COMMAND_MATRIX`
  (live-control, stall) and their other control is `observe()` (Observe records); none submits a
  bypass. Chromium with the SDK leg: red, `chromium: sdk-render-allocations: an SDK worklet instance
  allocated on its render thread: live-bypass:authored-none:live-desk-hi-bypass=2
  live-bypass:authored-none:live-ins-mid-bypass=2 live-bypass:authored-desk-hi:live-desk-hi-lift=2
  live-bypass:authored-ins-mid:live-ins-mid-lift=2` (2 = one record per dual-mono lane; the three
  authored-only renders stay 0). `render-allocations` precedes it in `validate` and passed; the same
  planted module without `--sdk-root` passes with all ten raw rows `=0`. Reverted (the clean module
  above): both green (gate 1).
- Gate 4: both `closeSdkEngine(closedBrowser)` calls replaced by `closedBrowser.close()`
  (one instance): chromium red, `chromium: sdk-render-allocations: an SDK instance closed without
  its render allocation read: 16 rows for 17 instances`. Restored.
- Gate 5: `bash scripts/check-workspace-policy.sh` ok; the `sdk` job's checks
  (`check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`,
  `check-sdk-headless.sh`, `sdk-package.sh check`) exit 0; the `sdk/dist` they left was deleted.
- No engine source or worklet artifact changed, so the worklet chain was not rerun.

Test value: `sdk-render-allocations` is red when a render-locked path only an SDK instance reaches
allocates (gate 3, which `render-allocations` passes), and red on the length check when an
instance closes without its read (gate 4).

### Follow-ups (2026-10-08, after the attempt 1 PASS; verdict m1, n2, n3, n4)

Change (`sdk-response-entry.ts` only):
- **n4.** `createSdkEngine(workload, options)` takes the `createEngine` options and calls
  `createEngine` itself, so the file calls `createEngine` at exactly one site (`:78`); the other
  references are the import and two types. A raw call would now be a second visible call site.
- **n2 (D3 escapes).** (1) `createSdkEngine` also makes the `node.connect` that each builder made
  after it; a throwing connect records the error, reads and closes the instance, then rethrows.
  (2) `finishSdkEngine(browser, ...cleanups)` is each body's `finally` where other cleanups run
  (`shared?.close()`, `subscription?.close()`, `headless.dispose()`): it runs every cleanup, then
  reads and closes the instance whatever they did; a cleanup error is thrown after the close when
  the body did not throw, and is attached (`sdkCleanupFailures`) when it did. (3) The collection's
  "fixture is incomplete" check now runs before its engine exists. (4) Backstop for any other
  throw outside a body's `try` (for example the headless boot in the track-response probe):
  `runSdkResponseQualification` reads and closes every instance still open when the run ends, on
  every path. On the error path every read or close failure is attached to the run's error; on the
  success path a swept instance has no row, so the length check is red. A failing
  `browser.close()` is now also attached when a body error came first, not thrown over it.
- **n3.** `sdkEngineFailed` wraps a non-object throw in an `Error` (`non-object error: <value>`,
  `cause` = the value) and returns that, so a read failure always has an object to attach to.
- **m1 (D2).** `closeSdkEngine` awaits `suspend()` before the read when the context is a running
  `AudioContext` (`spectrum-continuous-hop-1024`, `spectrum-collection`), so no render follows the
  read. Offline contexts are unchanged (their render has finished before the read).
- #1480's continuous region is unchanged except that its `finally` calls `finishSdkEngine`; the
  read hold and `renderLoss` are intact.

Evidence (worktree tree, CI browser-leg command `npm run qualify -- --artifacts <a> --sdk-root
sdk --browser <b> --check-matrix --self-test-mutations`, SDK source-bundle mode, no `sdk/dist`, a
private PulseAudio null sink per run; module rebuilt from this branch with
`build-web-audioworklet.sh --named-twin`, `7c6ee735...`):
- **Gate 1.** Exit 0 in chromium 151.0.7922.34, firefox 153.0 and webkit 26.5; each prints all 17
  SDK rows `=0`.
- **Gate 2 (#1476 mutations, temporary print in `mutationProofs`, reverted).** In all three
  browsers `sdk-render-allocations` is red with `<b>: sdk-render-allocations: an SDK worklet
  instance allocated on its render thread: resident-observation=1`, and
  `sdk-render-allocations-missing` is red with `... 16 rows for 17 instances`.
- **#1480 mutations, still red.** `nativeMissedWindows: 0n`, `skippedPublications: 0n` in
  `#notifySpectrum`: red in chromium, firefox and webkit (1 run each), `false: gap`. Read hold
  deleted (`await readsHeld` and the `Promise.allSettled`): firefox red on 5 of 5 runs, 4 naming
  `renderLoss` (status 3, 4-12 drops) and 1 naming `gap, renderLoss` (status 6, 0 drops).
- **m1, planted render after the stream stops.** Planted
  `if self.continuous.started_epoch != 0 { drop(black_box(Vec::<u8>::with_capacity(1))) }` in
  the one-shot branch of `SpectrumCaptureObserver::capture` (`crates/host-core/src/spectrum.rs`),
  rebuilt (`aaf33dde...`), reverted after. The plant reaches renders after the hop-1024 stream
  stops: with a temporary 250 ms wait before the suspend, chromium reads
  `spectrum-continuous-hop-1024=200`. A temporary probe that reads again 250 ms after the read and
  fails on a change: with the old read point (no suspend), a render followed the read in every
  browser (chromium `0 -> 182`, firefox `14 -> 212`, webkit `6 -> 198`); with the suspend, the
  probe never fired in any browser. So D2's "after the instance's last render" now holds.
  **Detection did not change**, as the verdict predicted: chromium reads hop-1024 `=0` at both
  read points (old: 2 runs, new: 3 runs, all exit 0), and firefox and webkit read it nonzero at
  both (new: firefox 14, webkit 2). The renders after the old read point are stopped, not counted,
  so this planted defect is not red under the strict read and green under the old one. The brief's
  "0 at the old read point and nonzero now" result was not obtained; the change is kept because
  it makes D2's claim true, not because it catches more.
- **D3 escapes (temporary plants, chromium, each with a planted read failure for the same
  instance).** Throwing `subscription.close()` in hop-1024's `finally`; throwing connect for
  `resident-observation`; a throw before the track-response body's `try`; `throw "planted
  string"` in a live-bypass body. Each run fails with the planted error (the string as
  `non-object error: planted string`), and each error carries
  `renderAllocationReadFailures: ["<workload>: render allocation count failed: planted read
  failure"]`, so every one of these instances was read and then closed.
- **Gate 5 and other checks.** `bash scripts/check-workspace-policy.sh`: ok. No SDK, engine or
  worklet source changed, so the SDK checks and the worklet chain were not rerun. An ad-hoc
  `tsc --noEmit` of the entry (CI does not typecheck it) has 26 errors, 31 before. The
  pre-existing mismatch between the injected contexts and `AudioContextLike` is now reported as 6
  TS2322 at `createContext` instead of 6 of the 7 TS2769 at the `createEngine` calls, and the six
  `destination` TS2339 at the builders' connects are now one at the single connect; no other
  error changed.

Test value: no new test; the gate and its two mutations are unchanged and still red.

