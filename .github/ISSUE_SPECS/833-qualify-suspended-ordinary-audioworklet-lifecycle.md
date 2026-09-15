# Qualify ordinary AudioWorklet boot/status/disposal in a suspended real AudioContext

## Outcome and authority

Before protected browser boot changes, determine whether the existing ordinary shipped host completes `boot -> status -> dispose -> dispose -> boot -> status -> dispose` in one **real, suspended, never-resumed** 48 kHz `AudioContext` in Chromium, Firefox and WebKit. This is a two-hour browser-scheduling feasibility issue. A PASS establishes only the ordinary idle lifecycle; it does not qualify protected boot, capture delivery, concurrent disposal, lost transport, or app release. A failed browser/stage stops protected production implementation for a new brief; this issue must preserve the failure rather than repair runtime behavior.

Fresh Astra XHIGH adversarial review split a 13-handoff protected browser brief into small successors and put this feasibility gate first. Fresh independent Astra MEDIUM plan recheck returned GO for the corrected one-file qualification below. Both reviews found current offline/fake-export and autoplay-enabled fixtures unable to prove the never-resumed case.

## Frozen inputs and scope

Exactly one implementation file may change: `hosts/host-web/qualification/suspended-host.mjs`. This numbered brief/evidence record is bookkeeping. No host/worklet/native runtime, SDK, server, dependencies, generated browser matrix, seven-file artifact, or CI workflow changes belong here. Use `startQualificationServer` and `exactArtifacts` from the existing server, installed Playwright 1.62.1, and the preserved seven-file artifact supplied as `--artifacts DIR`. `--browser all|chromium|firefox|webkit` defaults to `all`; closure requires `all`.

The frozen document is `hosts/host-web/tests/browser-v1/session.json` (48 kHz, 128-frame quantum, one track). Factory inputs are exactly `{ context, document, options, simd128ModuleUrl, workletModuleUrl }` with `sourceRingFrames: 0n`, `maximumMemoryBytes: 67108864n`, all four console options `0n`, no spectrum preparation and no `preparedModule`. Serve host, sibling dependencies, worklet, Wasm and generated ABI layout exclusively from the supplied exact artifact. The preserved Wasm SHA-256 is `18b9dbfa61ae1188fcb00f18317702e37feb37c4843ac2b885194a4c77322cab`, layout SHA-256 `8cd125f03d7bd5e98d93c4756af3bfe3282d709bceed1b55772140d5b57cb25d`; originating frozen source is `f2355988c0e51f4d283dd028bb98ce17f5038113`. Rehash every artifact after each browser run, including failures, and compare source-authoritative companions/pin. Identity hashes do not by themselves prove publication provenance.

## Objective gates

1. Preflight exact seven-file membership and hash all seven files, document, source commit, Node/Playwright versions and artifact provenance before browser launch. Report browser versions and launch options. Use no autoplay-enabling flags, preference overrides, activation clicks or fake exports.
2. Navigate to `/qualification/index.html` and install a counting, rejecting `AudioContext.prototype.resume` trap **before** importing/constructing the host. Create a real `AudioContext({sampleRate: 48000})`, record its initial state/time, and if initially running await suspension before constructing the host. Capture the post-suspension clock baseline.
3. In the same context, run the exact sequential lifecycle above. Each real status reply requires `result === 0`, `state === 2`, `lastResult === 0`, numeric `backend === 1` while host backend is `"simd128"`, rate 48000, quantum 128, `renderedQuanta === 0n`, and `nextAbsoluteSample === 0n`. Require context state `suspended`, exactly unchanged `currentTime`, and zero resume attempts at every lifecycle boundary. End this invariant before test-owned context teardown.
4. Run isolated red controls: suppress one actual matching status reply at the test transport boundary and require that same request's 15-second deadline to fail specifically as timeout, then restore delivery before cleanup; separately mutate each rendered counter in a captured status copy and require the assertion to refuse; invoke the resume trap and require refusal with unchanged state/time and without native resume.
5. Bound every asynchronous stage to 15 seconds and each browser to 120 seconds with a Node-side deadline covering stalled `page.evaluate`; bound teardown separately. Always attempt host/context/browser/server cleanup, distinguish a proven native disposal acknowledgement from unknown cleanup after loss, and report each stage/cleanup outcome. Record BigInts as decimal strings. Any missing browser, skipped case, unexpected failure or artifact drift exits nonzero with no PASS.

Do not use an `OfflineAudioContext`, synthetic `process()`, `startRendering`, forced resume, autoplay override, or concurrent-disposal requirement. Existing `dispose()` resolves `undefined`; a successful host promise through the verified unchanged receiver is the observable acknowledgement for this qualification.

## Execution and delivery

Root creates the matching GitHub issue with exact number/title and checkpoints this brief before implementation. Fresh Luna MAX implements only the one file; root audits status and commits/pushes its exact-path compiling/focused-test checkpoint before evidence revisions. Run:

```bash
node hosts/host-web/qualification/suspended-host.mjs --artifacts /tmp/miso-830-artifacts.eFrYdg --browser all
```

Preserve structured raw logs outside the shipped artifact. The **one issue-wide implementation attempt sequence** is Luna MAX attempts 1 and 2 if needed, Sol HIGH attempt 3, Astra XHIGH attempt 4; each candidate gets fresh Astra MEDIUM adversarial verification. Agent replacement, file slice, checkpoint or fixture correction never resets the issue ledger. If the bounded experiment cannot pass inside two hours or a browser fails for a product reason, stop and rebrief; never weaken the gate or fix engine behavior in this qualification issue.

After PASS and evidence checkpoint upstream, synchronize the GitHub issue body, close it, verify remote state and report the capability accurately. This issue does not claim protected boot or app deployment.

## Evidence and decision record

Initial: PR #831 is merged at `16534d9acfb224d6177a9b07bbf4110cf1cbfe40`; #832's pinned artifact and three-browser qualification are upstream, GH #832 is closed and its body matches local. Fresh Astra MEDIUM recheck verified Playwright 1.62.1 and browser executables, real host `status()`/`dispose()` shape, ordinary numeric backend `1`, current Wasm/layout pins and sibling source bytes. It did not execute suspended lifecycle. Main-push qualification run 35031475998 is still running; no Q0 PASS is claimed.
