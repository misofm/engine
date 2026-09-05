# Give the browser host sole ownership of its request-id ledger

**Issue:** #393, `Give the browser host sole ownership of its request-id ledger`  
**Baseline:** `origin/main` at `0e248bb0`  
**Status:** scope, decision record, and objective gates approved for Luna attempt 1; Astra performs the adversarial PR review. These explicit user model assignments replace the default Terra/Sol implementation/review roles for this issue only.

## Smallest closable product slice

Make `MisoAudioWorkletHost` the sole allocator of request IDs for every request it sends. Public request objects no longer accept `requestId`; wire messages still carry it, and acknowledgements/errors still return it for correlation. Migrate the two in-repository consumers of the public seam: the SDK browser console and the browser qualification harness.

This closes the demonstrated collision: `status()`, `sessionMap()`, and `dispose()` already allocate from the host's private ledger, while `command`, `observe`, `meters`, `telemetry`, `submitSource`, and `seekSource` currently accept independently allocated IDs. The SDK console compounds that by deriving its own counter from one `sessionMap()` acknowledgement.

No worklet, wire, Rust, render, ABI, session, source-shape, console receipt, or lifecycle behavior changes. In particular, the raw worklet's stale-ID branch is outside this symptom and outside this issue.

## Contract and implementation decision

1. The host has one private allocation point in the common request path. Callers pass payload only. The host stamps a positive, strictly increasing safe-integer ID on the outbound port message immediately before transport admission.
2. Existing validation and per-class saturation checks remain before allocation. A malformed request, a request refused because its response class is at capacity, and a request after disposal must not consume an ID or post a message. Their local error remains typed and uses `requestId: 0`, because no correlation ID was allocated.
3. A synchronous `postMessage` failure may leave a consumed ID: reuse would be unsafe because delivery cannot be inferred from an exception. Existing pending-release and sticky/failure behavior remains authoritative. The issue must not promise a gapless ledger across transport failure.
4. Before incrementing, the allocator checks `Number.MAX_SAFE_INTEGER`. Exhaustion rejects locally with the existing invalid-request result and `requestId: 0`, posts nothing, does not wrap or reuse an ID, and remains deterministic on repeated calls. Do not add a public counter accessor or test hook; exercise exhaustion through a private implementation seam only if the current harness can do so without production API expansion. If it cannot, use a source mutation/static discriminator rather than adding public surface.
5. Remove `requestId` from the exact-field guards and TypeScript request types for command, observation, source submission, source seek, meter lease, and telemetry lease. Preserve `readonly requestId` on response types and frames where already present. Old request objects containing `requestId` are rejected as extra-field invalid input at runtime and fail TypeScript excess-property checking; there is no compatibility shim.
6. `observe()` delegates to `command()` without allocating twice. `status()`, `sessionMap()`, and `dispose()` also enter the same allocator without computing a candidate ID themselves.
7. Preserve command-batch semantics byte-for-byte: whole-batch validation/admission, reason, rejected index, admitted count, applied-at sample, and returned record ownership are unchanged.

## Allowed paths

- `.github/ISSUE_SPECS/393-give-the-browser-host-sole-ownership-of-its-request-id-ledger.md`
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
- `sdk/src/browser/shipped-host.d.ts`
- `sdk/src/browser/console.ts`
- `scripts/test-web-audioworklet.mjs`
- `sdk/test/console-evals.mjs`
- `sdk/test/console-types.ts`
- `hosts/host-web/qualification/qualification.js`
- `sdk/test/package-tarball-smoke.mjs` only if needed to prove the packed consumer-facing declaration and runtime shape

Generated package staging output and untracked build output are evidence, not committed paths. If implementation requires any other tracked path, stop and amend the issue before editing it.

## Explicit exclusions

- `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, all Rust/Cargo paths, Wasm or artifact hashes, port-message field sets/tags, ABI and boot options.
- New error vocabulary, request broker, counter accessor, compatibility layer, cancellation, retry, render fence, console coalescing, session/readback/revision/automation, source metadata, storage, decoder, adapter, app, or lifecycle redesign.
- Issues #369 and #370 and every other DX-plan slice.

## Objective gates

1. **Public shape and hard break.** Type tests prove all six caller-controlled request categories omit `requestId`, all affected responses retain their existing readonly ID, and an old-shaped request is a compile error. Runtime tests prove an extra caller ID is rejected before `postMessage` with invalid-request and ID zero.
2. **Mixed-call ownership regression.** On one host, run at least 200 interleaved calls covering `command`, `observe`, `meters`, `telemetry`, `submitSource`, `seekSource`, `status`, and `sessionMap`, including SDK-console calls mixed with direct host calls. All successful responses have unique strictly increasing IDs in actual send order, with no caller counter and no retry. Include both orderings of meter/command and the reproducer `createBrowserConsole(host) -> direct meters -> console.submit`.
3. **Capacity/disposal do not burn IDs.** Hold each relevant bounded class at capacity, verify the next local refusal is ID zero and sends nothing, then settle capacity and verify the next success follows the prior allocated ID. After disposal, requests reject locally with ID zero and no send. Existing pending counters and source ownership remain balanced.
4. **Transfer ownership.** A valid `submitSource` transfers each unique `ArrayBuffer` once and returns the existing planes on acknowledgement. Invalid shape and local saturation leave buffers usable in the caller. Mixed source, seek, and control traffic cannot collide IDs.
5. **Safe-integer exhaustion.** A discriminating test proves the last safe ID can be allocated once; every later request refuses locally, never posts, never wraps/reuses, and returns the same typed terminal result. Restore unchecked `+ 1` as a red mutation and require failure.
6. **Single allocator and delegation.** Static/runtime evidence shows one production increment/allocation site, no `#lastRequestId + 1` call-site allocation, no SDK counter, and exactly one ID consumed by `observe()` despite its command delegation. No public next-ID accessor exists.
7. **Receipts unchanged.** Existing command tests continue to prove atomic whole-batch success/refusal, `appliedAtSample`, `admitted`, rejection reason/index, and transferred record return. This issue adds no new receipt meaning.
8. **Actual consumers.** Browser qualification uses payload-only requests and passes. The SDK console uses payload-only commands. A freshly packed SDK consumer typechecks the new browser declaration; where the existing tarball harness can instantiate the host fixture, it also exercises a payload-only call from package output.
9. **Proportional gates.** Run and attach exact results for `scripts/test-web-audioworklet.sh`, `scripts/check-sdk-types.sh`, `scripts/check-sdk-headless.sh`, `scripts/check-sdk-generated.sh`, `scripts/sdk-package.sh check`, and the focused browser qualification command already used by this repository. Run the packed tarball smoke when gate 8 changes it. Do not rebuild or test every Rust binary for this JavaScript/TypeScript-only slice unless a repository gate directly invokes a focused generator/oracle.
10. **Scope proof.** Diff contains only allowed paths; the worklet JS, Wasm, artifact hash, Rust, wire fields, and render callback are byte-identical to baseline.

## Review questions for Astra

- Can any public request still choose, predict, or indirectly allocate an ID twice?
- Can validation, saturation, disposal, or safe-integer exhaustion mutate the ledger or transfer buffers?
- Does `observe()` consume exactly one ID?
- Can any acknowledged command batch later be dropped, or did this refactor alter the existing atomic receipt fields?
- Do source buffers retain the same transfer/return behavior on success, refusal, and capacity pressure?
- Does packed TypeScript expose the same request shape tested in source?

## Delivery record

Root must first synchronize this approved body to the correctly named local issue spec and GitHub issue #393, then commit that issue-only checkpoint. Luna may implement only after that checkpoint. One coherent implementation tranche is followed by a root status/commit audit and Astra adversarial review; the standard three-attempt stop remains in force.

## Luna attempt 1 implementation evidence (commit-ready, not yet reviewed)

Changed only the approved host, SDK declaration/consumer, and qualification paths. The browser
host now validates payloads and saturation before allocating one private strictly increasing safe
integer, stamps that ID immediately before `postMessage`, and returns request ID zero for local
refusals. `observe()` delegates to `command()` with no second allocation. The SDK console and
qualification consumers no longer carry a caller counter or request ID in public requests.

Evidence so far:

- `./sdk/node_modules/.bin/tsc -p sdk/tsconfig.json --noEmit` passed.
- `bash scripts/check-sdk-types.sh` passed, including the shipped-host mirror pin.
- `node scripts/test-web-audioworklet.mjs` passed; output is saved at
  `/private/tmp/dx-393-evidence/test-web-audioworklet.log`.
- `node --check` passed for changed JavaScript/ESM files and `git diff --check` passed.
- The full `scripts/test-web-audioworklet.sh` reached its browser-harness checks but stopped in the
  environment WebDriver self-test because binding `127.0.0.1` is denied (`PermissionError`); no
  source workaround was made. The direct hermetic runtime gate passed independently.

Root must checkpoint these exact paths before further edits; Astra review remains pending.
# Issue #393 — attempt 2 addendum (Sol approved)

**Attempt 1 verdict:** FAIL at `f21c426e`; Astra review: `/private/tmp/dx-393-astra-review.md`.  
**Implementer/reviewer:** Luna performs this bounded revision under the user's explicit override; Astra re-reviews attempt 2.  
**Scheduling:** do not begin while adapter issue #19 has an uncheckpointed tranche.  

The approved product contract is unchanged: the shipped browser host alone allocates request IDs; public request payloads omit them; wire messages and acknowledgements retain them. No raw-worklet, Rust, ABI, render, receipt, or artifact-pin change is authorized.

## Required corrections

1. Update the authoritative `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` so command, observation, source, seek, meter, and telemetry request payloads exactly match the payload-only runtime. Keep every response `readonly requestId`. Keep `sdk/src/browser/shipped-host.d.ts` byte-identical to that authority.
2. Narrowly update `scripts/check-command-reason-vocabulary.py` observation-shape assertions from `[requestId, subscriptions]` to `[subscriptions]`. Preserve every command-reason, subscription, binding, acknowledgement, and mutation discriminator. Add/adjust self-test mutations so restoring `requestId` in either the declaration or `observe()` exact-field guard turns the validator red; do not loosen comparison or delete a check.
3. Complete the frozen regression evidence rather than substituting source inspection:
   - compile-time and runtime old-shape refusal for all six caller-controlled request categories, each local refusal asserting `requestId === 0` and no post;
   - at least 200 successful mixed calls on one real host fixture spanning command, observe, meters, telemetry, source, seek, status, and sessionMap, with strictly increasing unique acknowledgements in send order;
   - both meter/command orderings and `createBrowserConsole(realHost) -> direct meters -> console.submit`, with no retry;
   - every bounded response class: saturation refuses locally with ID zero/no send/no burn, then the next accepted request advances by exactly one; repeat after disposal;
   - malformed source retains caller ownership, while accepted and engine-refused source paths preserve the existing transfer/return behavior;
   - safe-integer boundary: a test-only transformed import starts the private counter at `MAX_SAFE_INTEGER - 1`, proves the last safe ID is emitted once, and proves repeated exhaustion rejects locally with invalid-request/ID zero, no post, wrap, or reuse. Do not add a production accessor or hook. The named mutation removing the exhaustion guard must fail this test.
4. Extend the packed SDK consumer gate to compile payload-only calls for all six request categories against the staged declaration. Runtime packed-host coverage may reuse the existing host harness; no new browser matrix is required.

## Meter callback note

Attempt 1 removed caller IDs but did not intentionally redesign lease callbacks. Astra must compare the `meters()`/`telemetry()` callback assignment timing with `0e248bb0`. Tests for saturation/no-burn must not silently normalize unrelated pre-existing callback behavior. If attempt 1 introduced a callback-state regression, restore baseline behavior in the host JS; if baseline already mutates callbacks before admission, record it outside #393 and do not broaden this revision.

## Amended exact allowed paths

- `.github/ISSUE_SPECS/393-give-the-browser-host-sole-ownership-of-its-request-id-ledger.md` (attempt/evidence record only)
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`
- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`
- `sdk/src/browser/shipped-host.d.ts`
- `sdk/src/browser/console.ts`
- `hosts/host-web/qualification/qualification.js`
- `scripts/test-web-audioworklet.mjs`
- `scripts/test-web-audioworklet.sh` only for the safe-integer red mutation runner if the existing module override cannot express it in the `.mjs` test
- `scripts/check-command-reason-vocabulary.py` (newly approved, narrow observation request-shape update plus discriminating mutations)
- `sdk/test/console-evals.mjs`
- `sdk/test/console-types.ts`
- `sdk/test/package-tarball-smoke.mjs`

No other path is authorized. In particular, do not touch `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, any Rust/Wasm/generated ABI asset, or `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`. The artifact-pin mismatch is under separate baseline investigation and must not be repinned or attributed to #393.

## Validation required before Astra re-review

- Focused: `node scripts/test-web-audioworklet.mjs`; `python3 -B scripts/check-command-reason-vocabulary.py --self-test`; `python3 -B scripts/check-command-reason-vocabulary.py`; `scripts/check-sdk-types.sh`; `scripts/check-sdk-generated.sh`.
- Full proportional gates from the frozen spec: `scripts/test-web-audioworklet.sh`, `scripts/check-sdk-headless.sh`, `scripts/check-sdk-deletions.py`, `scripts/sdk-package.sh check`, focused browser correctness, and browser qualification.
- Attach the safe-integer red-mutation failure, exact mixed-call count, acknowledgement range/order, per-class saturation/no-burn table, all-six type/runtime shape results, packed-consumer result, and diff proof that raw worklet/Rust/Wasm/artifact pin remain unchanged.

Attempt 2 passes only when both P1 findings are corrected and every previously missing frozen gate has executable evidence. Do not lower a gate because attempt 1 omitted it.

Root baseline qualification ruling: unchanged base `0e248bb0` and PR `f21c426e` both rebuild on Darwin arm64/Rust 1.97.1 to digest `2f7941af57dbbee29f9407ee8a65cd58eac376f45f336ac40bd92690d415563b`, while the frozen pin is `22e4c25cba7f97b66db720ad8ac8cf653de0afcabe84101693f4fa166b90d4e6`. This repeats the existing cross-host reproducibility limitation recorded in #333/#345, not a Rust change in this issue. Use the exact successful baseline CI run `33930536895`, artifact `9958403991` (`audioworklet-0e248bb07cfbf7dd136ec48649ec61ee9171d15b`), retaining its unchanged pinned Wasm/worklet/metadata and overlaying only this PR's host JS/declaration for package/browser checks. Do not repin. Root retains baseline logs and downloaded originals separately.

## Luna attempt 2 checkpoint evidence

Corrected the authoritative host declaration and the observation-shape validator, with its old-shape mutations retained as red discriminators. Added all-six type/runtime probes, readonly response IDs, 250 mixed real-host calls and real SDK-console/direct-host interleaving, bounded-class no-send/no-burn and disposal assertions, malformed-source ownership, and safe-integer exhaustion with the unchecked-increment red mutation. Packed consumer probes cover all six payload categories.

Focused host tests, full `scripts/test-web-audioworklet.sh`, observation validator/self-test, SDK types, SDK generated surface and `git diff --check` pass. Headless evals: 133 pass, one platform-capability skip. `scripts/sdk-package.sh check /private/tmp/dx-393-current-artifacts` passes the freshly staged and packed consumer gate using the verified CI Wasm closure described above, overlaid only with current host JS/declaration. Logs are under `/private/tmp/dx-393-evidence/attempt2-*`. Raw worklet, Rust, Wasm, ABI assets and artifact pin are unchanged. Browser qualification and Astra attempt 2 verdict remain pending; this checkpoint is not a PASS claim.
