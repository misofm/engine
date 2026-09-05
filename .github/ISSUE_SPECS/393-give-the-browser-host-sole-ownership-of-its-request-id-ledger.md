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
