PASS

# #1345 attempt 3: adversarial verdict

Under review: `git diff 38594285a ca5c5e60d` (commits `c7791ea9e`, `bb8827b4c`, `ca5c5e60d`) on
`codex/d15-stream-b3`, and the whole #1345 change on the rebased branch (attempt 1
`71b4ecdb8..754bd170e`, attempt 2 `822bce738..c0f825939`, reconciliation `cffc3006f`), against the spec
at `ca5c5e60d` (Amendments 1-4) and root's "D2 Amendment 2 (root, 2026-10-12; attempt-2 M1)". I
exported `ca5c5e60d` with `git archive` to `/tmp/claude-1002/v1345c/tree`, git-initialized it, and built
it with private target dirs. I read the worktree only through `git diff`/`git log`/`git show`.
Probes, mutation scripts and trimmed logs: `submix-verdicts/evidence/1345-attempt3/`.

**Summary.** Root's ruling is implemented as written. The headless engine has no render wait: the
exact attempt-2 hang (`await handle.close()` before any render) now rejects in 0 ms with
`MisoEngineError` code `backpressure`, admits nothing, leaves the owner free, and a render plus the same
call succeeds; update and subscribe behave the same, and a refused update leaves the configuration and
the engine's taps unchanged. The browser wait is bounded by one deadline that spans every resend, ends
in the same typed `backpressure`, clears its timers, stops polling, and is cut short by an engine close
or a hung `host.status()`. The 5,000 ms fallback is the SDK's existing `requestDeadlineMs` default, not
a new number. The three claimed mutations are red on exactly the named evals. Every gate job is green.
No BLOCKER or MAJOR. Three MINOR findings (two test gaps and one doc gap) and two NITs; one MINOR is a
successor issue root must file (the real-worklet browser case).

## BLOCKER

None.

## MAJOR

None.

## MINOR

- **m1. No test holds "one deadline spans all resends".** `sdk/src/core/observation-subscriptions.ts:907`
  (`deadline ??= Date.now() + wait.boundMs`). Mutation Md (`??=` -> `=`, a fresh deadline per resend):
  370/370 SDK evals stay green. Under that defect the bound becomes up to 9 x `requestDeadlineMs` (the
  8-resend cap times the deadline): 45 s at the default instead of 5 s, against the attempt-3 record's own
  claim ("one deadline of `boundMs` is fixed at the first refusal and spans all resends"). The current code
  is right (probe F: renders every 40 ms that never free the slot, deadline 200 ms -> typed
  `backpressure` at 200 ms after 5 sends). Add to the scripted-host suite a case whose renders do not free
  the slot and assert the rejection lands within the deadline, not later. Fold into the next attempt or
  a follow-up; it does not block.
- **m2. The real-worklet browser case is missing; root must file it.** The browser evals run the real
  `createEngine` over a scripted host (`sdk/test/observation-subscription-evals.mjs:481-614`). The host is
  faithful at the seam it replaces: result 6 / reason 8 is the ABI's `backpressure`/`backpressure`
  (`sdk/assets/miso-engine-v1-abi-layout.json:620,626`), a refusal admits 0, a render frees the slot, and
  everything above `host.command`/`host.status` is shipped code. It is not faithful in two ways no local
  test can check: it has one slot for all taps (the engine has one per tap; harmless for the single-tap
  cases tested), and it models the order "`renderedQuanta` moves after the drain", which only the real
  worklet's status publication proves. The CI browser legs
  (`hosts/host-web/qualification/sdk-response-entry.ts`, outside this issue's paths) should gain: subscribe,
  then `await close()` on a context that is not rendering with a small `requestDeadlineMs` -> typed
  `backpressure`; after the context renders the same close resolves. I checked the existing
  resident-observation flow (`sdk-response-entry.ts:1600-1690`): after `startRendering()` it makes one
  update (keys exclude channels, so only the `gate` disarm is sent, on a drained slot) and one close
  (the `comp` disarm, slot drained), so the new bound does not change it, as the implementer said. This is
  qualification work under AGENTS.md ("target matrices ... can be stateless successor issues"): file it
  as a successor issue; it does not block this slice.
- **m3. The user-facing README does not state the new headless contract.** `sdk/README.md:276-279` says
  "headless consumers call `pump()` between explicit renders" and "refused updates preserve the prior
  configuration", but not that a second managed subscribe/update/close on the same tap before a render is
  refused with a retryable `backpressure`. On `main` (64-record shared queue) that sequence resolved at
  once, so this is a behaviour change visible to SDK users; only the TS doc comments say it
  (`sdk/src/headless/engine.ts:220-226`, `observation-subscriptions.ts:122-126`). `sdk/README.md` is not in
  the paths or the named exceptions; root should authorize one sentence there (here or in #1524's docs
  pass).

## NIT

1. `sdk/src/core/observation-subscriptions.ts:122-126`: the shared `ObservationSubscription.update` doc
   says the call "is refused ... when ... no render has drained it yet". That is the headless contract;
   the browser first waits for a render up to `requestDeadlineMs` (stated only on
   `BrowserEngine.subscribeObservations`, `browser/engine.ts:204-212`). Say both in the shared doc.
2. No eval holds "an engine close ends the browser wait at once" (`browser/engine.ts:1026`, `if (closed)
   return finish(false)`). Mutation Mh (line removed): 370/370 green. The defect is bounded by the
   deadline (the wait ends at `requestDeadlineMs` and the epoch check then throws the stale-owner
   error), so it is a NIT, not a gap that can hang. Probe A shows the current code ends it in about 55 ms
   with the stale-owner `MisoUsageError`.

Not findings, recorded for root:
- `requestDeadlineMs` is validated only on the default scratch-boot path (`browser/scratch.ts:62-64`); an
  injected `scratchBoot` lets 0, -1, NaN or Infinity through. For the wait every such value ends at once
  in typed `backpressure` (probe H; Infinity is clamped by the timer to 1 ms), never in a hang. This is
  pre-existing and the same for the spectrum and response deadlines.
- While a browser wait runs (a few ms on a rendering context, up to the deadline on a suspended one) the
  owner's `#enqueue` guard refuses other owner operations with "already in flight" (probe G). Pre-existing
  serialization, now bounded in time.
- If `host.status()` rejects, the wait rejects with that host error rather than `backpressure` (probe C).
  Not a hang; a dead host is not flow control.

## Judged points

**1. Headless hang probe (re-run).** `probe/probe-headless.mjs` against the unmutated artifact
(`c98f68380b79...`, the digest the attempt-3 record states):
- `await handle.close()` before render: `MisoEngineError backpressure` in 0 ms; the line after the await
  runs (attempt 2: Node exit 13, "unsettled top-level await").
- Owner free: `pump()` resolves. Tap state consistent: after one render `comp` is `pending` (armed), not
  `unarmed`; a second `close()` resolves and after a render `comp` is `unarmed`.
- `update` on a held slot: refused typed; `configuration.windowBlocks` stays 4. A refused re-selection
  (`comp` -> `gate`, i.e. disarm comp + arm gate in one submission) leaves the configuration on `comp`,
  and after a render `gate` is still `unarmed`: nothing of the refused submission was admitted.
- `subscribe` while a close's disarm holds the slot: refused typed; a `subscribe` of another tap (`gate`)
  at the same moment resolves (the slot is per tap). After a render the same subscribe arms `comp`.
- A caller's retry loop (`catch backpressure -> render -> retry`) completes update and close.

**2. Browser `renderWait`.** Read and probed (`probe/probe-browser.mjs`, scripted host as in the evals,
with knobs for a hung or rejecting `status()`, slot refills and timer accounting):
- One deadline fixed at the first refusal spanning all resends: yes (`??=`; probe F: 200 ms with 5 sends).
  Each wait gets only what is left; `remaining <= 0` throws before a wait.
- `boundMs = options.requestDeadlineMs ?? 5_000`: consistent. 5,000 ms is the SDK's existing default for
  `requestDeadlineMs` everywhere it is read (`browser/engine.ts:699,762,819-820,884-885,1166-1167`,
  `browser/scratch.ts:62`, `browser/response.ts:63`, `core/observation-subscriptions.ts:1550,2345,2354,2415`).
  It is a repeated literal, as before this change, not a new magic number.
- Timers cleared on settle: `finish` clears `limit` and `poll`. Probe E: live timers before = after the
  wait; 0 `status()` calls after settle. If a `status()` was in flight at settle, the loop sets one more
  4 ms tick and then returns on `settled` without another call, so at most one short-lived timer, no leak.
  Mutation Me2 (no `clearTimeout(poll)` and no `settled` check) turns the suspended-context eval red.
- Hung `host.status()`: cut off by the deadline (probe B: 101 ms at 100 ms).
- No polling after settle: confirmed (probe E, eval).
- Engine close ends the wait: `closed` is set synchronously in `close()` (`browser/engine.ts:1307`) and the
  owner is invalidated; the wait ends at the next tick and the epoch check throws the stale-owner error
  (probe A, about 55 ms). Untested (NIT 2).
- Hang: no path found. Leak: no. Double resend: no; a refusal admits nothing and each resend follows a
  `true` from `next`, capped at 8 (probe D: a render that never frees the slot -> 9 close sends, then typed
  `backpressure` in 85 ms). Reorder of Observe records: no; `#enqueue` serializes every owner operation, so
  one managed submission is outstanding and a resend precedes any later operation.
- Ack before drop: no. `#submit` returns only an `ok` report; every refusal admitted 0 and is either resent
  whole or thrown.
- Fake-host fidelity: see m2.

**3. m1 (retry only on `backpressure`).** A `malformed` refusal rejects at once as `invalidArgument`, with 2
commands logged (subscribe + one close) and no wait (eval at `:596`). Mutations re-run against all 370
SDK evals (13 baseline failures in my copy are path-resolution tests of `check-sdk-headless` caused by the
symlinked copy; red sets below exclude them):

| mutation | new red |
|---|---|
| Ma: headless transport gets `renderWait` whose `next` never resolves | the two headless evals (`:271`, `:296`) only |
| Mb: browser `next` loses its deadline timer | the suspended-context eval (`:571`) only |
| Mc: `#submit` retries on any reason | the malformed eval (`:596`) only |
| Md: deadline reset per resend | none (m1) |
| Me: `if (settled) return` removed | none (equivalent: `clearTimeout(poll)` already stops the loop) |
| Me2: `clearTimeout(poll)` and `settled` check removed | the suspended-context eval only |
| Mf: no resend after a render | the rendering-context eval (`:555`) only |
| Mg: `update` writes the configuration before the submit | the headless update eval (`:296`) only |
| Mh: wait ignores `closed` | none (NIT 2) |
| Mi: owner treats a `backpressure` report as success | the headless update eval and both browser wait evals |

The implementer's three claimed mutations reproduce exactly.

**4. Real-worklet browser case.** A gap, MINOR, to be filed by root as a successor (m2). It cannot run on
this machine and the entry file is outside the paths.

**5. Attempts 1-2 on the rebased tree.** Attempt 3 changes no engine code: the Rust diff is two doc
comments (`crates/host-core/src/prepare.rs:350,1677`, `crates/effect-compiler/src/prepare.rs:62`) and one
test doc (`hosts/host-web/src/tests.rs:6974`). The reconciliation `cffc3006f` touches only
`crates/host-core/tests/successor_swap.rs` (the CellPlan test helper now stamps through LivePlan's gate).
Amendment 3's engine side (a second Observe for one tap is malformed; one record per tap; the room pass),
the D8 direct drain, the M2 estimate and the doc set are exercised by the test-debug-a/b, release,
audit-native, worklet and ABI gates below, all green, with the realtime region floor still 118 in 27
files and the shipped module digest unchanged from the attempt-3 record. The attempt-2 NITs are folded:
host-core and effect-compiler prose now name the observation queue and the control lane that drains it;
the `a_submission_names_each_observation_tap_once` doc now says the reason code is the catch;
`round_trip.rs` stays (accepted); `worklet-host.js` stays with #1524. No producer/fan/listener role prose
was added. No version suffix or naming issue.

## Test value (new or rewritten tests)

- `an awaited close before any render is refused with typed backpressure; a render and a retry succeed`
  (`observation-subscription-evals.mjs:271`): turns red if the headless owner waits for a render that only
  the caller can make (the attempt-2 hang, mutation Ma), or if a refused close disarms the tap or leaves
  the owner busy; no other eval awaits a managed close on an undrained slot.
- `a subscribe or update before the slot is drained is refused with typed backpressure and changes nothing`
  (`:296`): turns red if a refused update changes the handle's configuration (mutation Mg) or if a refusal
  for the slot waits (Ma); it is the only eval that refuses an update and a subscribe on a held slot.
- `a close whose slot is taken waits for the context to render and goes through` (`:555`): turns red if the
  browser owner stops resending after a render (mutation Mf), which would turn every same-tap operation
  on a live context into an error.
- `a suspended context ends the wait in typed backpressure at the deadline, and the polling stops` (`:571`):
  turns red if the browser wait has no time bound (Mb) or keeps polling the host after it settles (Me2).
- `a refusal that is not backpressure rejects at once and is never sent again` (`:596`): turns red if the
  owner retries on a reason other than `backpressure` (Mc), which left all 367 attempt-2 evals green.
- Deleted with their behaviour, as AGENTS.md requires: the attempt-2 "close waits for the render" and
  "dispose while waiting" evals (the headless wait no longer exists).

## Gates run (export of `ca5c5e60d`, this machine, no browsers, no AArch64)

All 77 workflow steps I ran exited 0 (`evidence/1345-attempt3/gates-summary.txt`). Runner:
`evidence/1345-attempt3/runjobs.py` (every `run:` step of the named jobs, verbatim, toolchain installs
skipped) and `chainC.sh`.

| job / gate | result |
|---|---|
| `lint`, all 29 steps (fmt, clippy `-D warnings`, doc, workspace/session/env/bench/test-support-ci/scalar-oracle self-test/script-reachability/console-fixture/builtins-manifest/bench-preconditions/host-core/protocol-control/realtime (+ audit-leak, artifact-evidence-leak)/trace-validator/lane/unfused-seal/rack/builtins/graph/effect-runtime/conformance/PEQ seal/release-shape/npm-publish/stem-store/stem-identity policies and mutation tests, the three sub-v3 lane-guard probes) | pass; realtime policy `ok (118 marked regions in 27 files)` |
| `test-debug-a` (builtins-compiler `--no-run`; workspace `--all-targets` with the job's excludes and features; doctests) | 1,615 passed, 0 failed (20 ignored); doctests 24 passed |
| `test-debug-b` (DSP crates; doctests; `conformance_fixtures --check`) | 916 passed, 0 failed; doctests 7; fixtures ok |
| `test-release`, all steps incl. the loom SPSC leg, `filter_liveness`, `tail_contract`, `ramp_endpoint`, `designer_total`, FMA cfg, M1 exhaustive, F1 exhaustive | pass (lane/math/wasm-gates 133 passed) |
| `audit-native`, all steps (release build; audit/bench/console-workload tests 113 passed; issue-544 capi audit + record validation; delay/compressor/PEQ/gate audits; builtins 1M-block direct and graph traces; graph; protocol; realtime probes and 1M trace; probe mutation tests; effect 1M-call trace; `check-capi-abi.sh` + self-test; scalar-oracle-absent `--native`; 100-process graph determinism; builtins fixture corpus; console fixtures; effect contract bench) | pass |
| worklet chain: `build-web-audioworklet.sh --named-twin` (shipped `c98f68380b79b9a68cf7c216d15f3694943969b33f05551f1f1555a0fa4e44dc`, as recorded), `strip-wasm-names.py` self-test and check, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (private TMPDIR, left empty), V8 spill self-test and gate | pass |
| `check-abi-layout-v1.py` on the shipped layout + `--self-test` | pass (22 mutations caught) |
| `sdk` job checks after `npm ci`: `check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types`, `check-sdk-headless` (370 passed, 0 failed), `sdk-package.sh check`; stray `sdk/dist` deleted | pass |
| `check-cross-targets.sh` | `cross-target matrix: PASS` |
| SDK mutations (table above) and the two probes | as stated |

Not run: the `browser` job (no browsers here), the AArch64 legs of `cross-target`, `wasm-guests`,
`release-shape`, `gate-self-tests`, `docs-gates`.
