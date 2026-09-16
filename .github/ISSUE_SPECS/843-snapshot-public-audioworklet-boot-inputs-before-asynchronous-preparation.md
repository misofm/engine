# Snapshot public AudioWorklet boot inputs before asynchronous preparation

**Outcome and current baseline.**  
Make `createMisoAudioWorkletHost(input)` isolate boot preparation from subsequent caller mutation. Capture and validate private preparation values synchronously before the first await, use that snapshot throughout initialization and host-limit derivation, and recheck the captured context immediately before constructing the node.

Product baseline is qualified main `299c828570f57cfbb4e1c82ced817be2ca5050b6`. #839’s native protected-credit refusal, #841’s test-only lint repair and #842’s artifact qualification are merged through PR #840. Required merged-main qualification run `35054558108` passed. None of those changes repaired this public factory.

At baseline, the factory validates caller objects before loading, copies document/options after `addModule`, reads `options.context.sampleRate` during Ready validation, and rereads context/options after ABI loading to construct the returned host. The Wasm URL and supplied module are already consumed before the first suspension; the worklet URL, context reference, preparation records and later configuration reads remain exposed to mutation.

The smallest closable capability is:

> Public boot preparation is isolated from caller mutation.

**Execution admission.** Fresh Astra XHIGH current-main scope `/tmp/miso-snapshot-current-scope.md` returned GO after replacing the stale artifact assumption with qualified `b47d05f...`. Separate fresh Astra XHIGH adversary `/tmp/miso-snapshot-current-adversary.md` returned NO-GO until a synchronous single-read oracle, postconstruction captured-context/default-capacity proof and exact enumerable-key compatibility were added. A second fresh Astra XHIGH adversary `/tmp/miso-snapshot-current-adversary2.md` independently checked the corrected body against source and returned GO with no further wording changes. Execution admitted at **2026-09-16 04:38:21 UTC** on clean qualified main `299c828570f57cfbb4e1c82ced817be2ca5050b6`; this starts the four-active-hour clock. GitHub assigned actual issue **#843**. No implementation has begun.

Fresh Luna MAX attempt1 handoff1 changed only `scripts/test-web-audioworklet.mjs`; root pushed intentional-red checkpoint **`4a8735c2`** after the agent paused at the 30-minute tranche boundary. The 954-line insertion is candidly larger than the desired compact matrix but remains one existing test file and covers the frozen cases without a new generic harness. Root independently verified syntax, diff and environment-vocabulary gates; unchanged existing mode passed; current document, nested and single-read modes each exited1 at only `snapshot.document.before-construction`, `snapshot.nested.before-construction` and `snapshot.single-read` respectively. Private baseline `299c8285` also reached construction and failed only the required document assertion; report `/tmp/miso-843-luna-max1a.md`, logs `/tmp/miso-843-{baseline,current,root}-*.log`. Existing real-Wasm ordinary lifecycle passed against preserved qualified `b47d05f...`. No production, declaration, SDK, worklet, artifact, workflow or GitHub implementation surface changed. This is a recoverable red test checkpoint within attempt1, not a submitted candidate or PASS; host correction remains handoff2.

Fresh Luna MAX attempt1 handoff2 changed only the host JS and stopped under the frozen-test rule when the full suite reached a defective enumerable-undefined oracle; root pushed useful buildable production checkpoint **`fbc43a73`** with that failure candidly open. Snapshot implementation is77 insertions/25 deletions: private visible document copy with initial shared/detached refusal, enumerable-shape-preserving nested copies with sparse holes, captured factory/context identities and context shape, immediate preconstruction drift check, and captured Ready/returned-host limits. Root independently verified syntax/diff plus document, nested, single-read and existing modes all exit0. Default mode exits1 only at `snapshot.shape.enumerable-prepared-undefined`: `snapshotFactory({ preparedModule: undefined })` triggers the helper parameter's default valid module before constructing the enumerable field, so it cannot exercise the frozen compatibility clause. Agent report `/tmp/miso-843-luna-max1b.md`, root logs `/tmp/miso-843-root-host-*.log`. No second production file or scope widening. A fresh tiny Luna MAX test-only checkpoint correction may repair this exact helper defect within attempt1 before the remaining matrix, mutant, candidate assembly and real-Wasm proof; this is not a submitted-candidate verdict or a new issue-wide attempt.

Fresh Luna MAX tiny attempt1 checkpoint correction replaced only the defective `snapshotFactory` omission/explicit-undefined distinction using `Object.hasOwn`; root pushed exact test-file checkpoint **`717c1fbb`**. Syntax/diff and document/nested/single-read/existing modes pass. Default suite now reaches the next exact test-oracle defect: `snapshot.limits.explicit-source-overflow` correctly returns existing typed `RESULT_BACKPRESSURE` **6**, while the new helper hardcodes invalid-argument result1. Logs `/tmp/miso-843-luna-max1c-final-default.log` and `/tmp/miso-843-root-helper-*.log`, report `/tmp/miso-843-luna-max1c.md`. This correction remains inside attempt1 and changed no production or delivery surface. A fresh tiny Luna MAX test-only correction must preserve backpressure and verify no post/transfer/request-ID burn; it may not change host behavior or broaden the harness.

This issue does not expose protected native boot or establish protected browser EQ delivery.

**Frozen implementation boundary.**  
Only these implementation/test files may change:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`
- `scripts/test-web-audioworklet.mjs`

The matching numbered issue spec holds the decision and evidence record. Temporary mutant modules, candidate assemblies and raw logs remain outside Git.

No declaration, SDK, worklet, Rust, ABI, workflow, dependency, artifact-pin or browser-fixture edits. No new public option, exported API, loader framework or production test hook. Keep the receiver’s existing real-Wasm mode, identity constant, preflight, assertions and controls unchanged; add the new snapshot coverage to its fake-host testing surface.

Prior-boundary synchronization of already-upstream #839/#842 delivery evidence is separate bookkeeping, completed before this implementation tranche.

**Snapshot contract.**

1. **Capture before suspension.** Capture factory values, boot values and relevant nested records before the first await. Validate the captured values and original enumerable-key shapes. Do not validate caller values and then independently reread them to populate the snapshot. After capture, initialization must not reread caller preparation records.

2. **Preserve field validation.** Retain existing `Object.keys` semantics over own enumerable string keys and exact accepted field sets. Copying must not discard an enumerable unknown field and thereby make invalid input valid. Do not add descriptor or prototype hardening. Preserve:
   - Factory fields `context`, `document`, `options`, `simd128ModuleUrl`, `workletModuleUrl`, with the existing optional `preparedModule`.
   - An own enumerable `preparedModule: undefined` or `preparedModule: null` rejects. A non-enumerable `preparedModule: undefined` remains outside `Object.keys` and preserves its current acceptance boundary.
   - Six-field legacy boot records and eight-field extended records accept when their values are valid; seven-field records reject.
   - Spectrum `null`/`undefined` normalization and refusal when both spectrum forms are nonnull.
   - Unknown factory, boot, spectrum, collection and entry fields reject, including invented protected options.

3. **Own the document.** Copy precisely the visible `Uint8Array` range into private ordinary storage. Preserve offset/length semantics and caller ownership; neither transfer nor detach the caller’s buffer. Mutation or detachment after synchronous capture must not alter the owned copy. Do not freeze a nonempty typed array.

   Intentionally reject initially shared-backed or detached storage locally with `miso.error.v1`, request ID `0`, result `1`, before fetch, `addModule` or construction. Baseline does not already provide this early refusal. An attached zero-length view remains admissible through host validation; that does not promise successful native boot.

4. **Own nested preparation records.** Copy the boot record, single-spectrum record, collection record, collection entries array and each present entry. Preserve sparse holes. Current collection validation uses `.every()`, which skips holes; current worklet staging rejects missing entries. Do not densify holes or change that refusal boundary.

5. **Preserve domains.** Keep the current validators’ numeric types, bounds and dependency checks:
   - Numeric u32 `sourceRingFrames`.
   - u64 BigInt memory and console words.
   - Command queue maximum `256n`; observation taps maximum `16n`.
   - Meter/master words bounded by u32, with existing queue/tap/master dependencies.
   - Nonempty target IDs of at most 127 UTF-8 bytes and existing target/channel enums.
   - Single-spectrum capture allowance `1..1,048,576`; collection allowance remains a positive safe integer without importing the single-spectrum ceiling.

6. **Capture identities and context shape.** Preserve the context and supplied-module references and the URL strings. Capture sample rate and effective quantum, `renderQuantumSize ?? 128`. Immediately before `new AudioWorkletNode`, with no intervening await, require the same captured context to remain suspended with unchanged sample rate/effective quantum. Otherwise reject locally with result `1` and construct no node.

   This is a preconstruction check. It does not prevent later caller resume or establish suspended-browser scheduling guarantees.

7. **Use one preparation consistently.** Constructor options, Ready resource validation, returned-host sample rate/quantum, source in-flight bound, command capacity and default observation window all derive from captured values. Preserve the existing formulas:
   - Explicit source depth: `sourceRingFrames / quantumFrames`.
   - Default source depth: `Math.ceil(sampleRate / 10 / quantumFrames) + 2`.
   - Command capacity: `Number(consoleCommandQueueRecords) || 1`.
   - Default observation window: `Number(consoleMeterBlocks)`.

8. **Preserve initialization compatibility.** A supplied module bypasses Wasm fetch/compile while retaining SIMD attestation and `addModule`. Preserve typed unsupported-SIMD refusal before network/node work. Preserve existing error handling and node cleanup outside the explicitly changed early refusals. Stronger native-owner reclamation after post-Ready ABI failure belongs to a successor.

**Focused acceptance evidence.**  
The following snapshot tests are new proposed gates; scoping has not executed them. Use a compact case matrix and existing override/fake-port facilities, with bounded pauses and cleanup. Do not build a Cartesian mutation matrix or a new testing framework.

| Gate | Required discrimination |
|---|---|
| Preconstruction document mutation | Pause Wasm loading; mutate the original visible bytes and replace the document field. Constructor evidence must retain the captured bytes. Include an offset view and caller-buffer preservation. |
| Nested mutation | Pause `addModule`; mutate nested values, replace spectrum/collection records and replace/mutate collection entries. Constructor evidence must retain the captured preparation. Cover ordinary, single-spectrum and collection shapes across the compact matrix. |
| Captured references | Replace caller context/module fields and URL strings during appropriate pauses. Construction must use captured identities/strings. Cover a supplied real `WebAssembly.Module`, proving no Wasm fetch/compile on that route. |
| Ready pause | Deliver Ready only after postconstruction mutation of caller references and the fake context’s shape. Validation must use captured preparation numbers. This directly detects late Ready-validator reads; it asserts no new postconstruction suspension guarantee. |
| ABI pause and host limits | Mutate scalar boot values while ABI loading is paused. Verify returned-host behavior for explicit source depth, command capacity including zero→one, and `observe({windowBlocks: 0})` substitution. In a default-depth case, keep captured `sourceRingFrames` zero, mutate the captured fake context's sample rate/effective quantum after construction, require `host.status()` to accept a reply with the original captured rate/quantum, and prove default source capacity from those original values through overflow refusal, retained caller storage and request-ID continuity. Constructor equality alone is insufficient. These assertions establish captured host configuration, not a postconstruction suspension guarantee. |
| Storage edges | Initially shared/detached input rejects locally before loading/construction. Detachment after capture preserves the private copy. Attached empty input passes host validation. |
| Shape/domain compatibility | Exercise six/eight-field acceptance, seven-field refusal, enumerable unknown nested fields, null/undefined normalization, own enumerable undefined/null `preparedModule`, a non-enumerable undefined compatibility witness, mutually exclusive spectrum forms, sparse-hole preservation and representative numeric/domain boundaries. |
| Context drift | Separately change captured-context state, sample rate and effective quantum before construction; each refuses locally with result `1`, request ID `0`, and zero node constructions. Preserve default-quantum behavior. |
| Existing behavior | Keep the existing receiver suite, typed SIMD refusal, malformed Ready/ACK handling, saturation, transfer ownership and disposal regressions passing. |

The existing `FakeNode` retains constructor options by reference. Capture constructor data synchronously for the new oracle so subsequent mutation cannot rewrite evidence. Keep context/module identity observations separately where cloning would obscure identity.

For behavioral saturation checks, hold replies, demonstrate the captured capacity, demonstrate local refusal without an extra post/transfer or burned request ID, then release pending replies and clean up. Use valid zero-command-capacity boot fixtures with dependent console words disabled.

**Two new snapshot red controls.**

- **Original factory:** Run the new tests against a private copy of the factory from baseline `299c828570f57cfbb4e1c82ced817be2ca5050b6`. Mutation during a preconstruction pause must reach node construction and fail the literal assertion `snapshot.document.before-construction`.
- **Shallow-copy mutant:** From the corrected candidate, make one uniquely matched private substitution that retains caller ownership at a tested nested-copy site. Preserve document/scalar capture and the remaining factory behavior. A nested mutation before construction must reach construction and fail `snapshot.nested.before-construction`.

Run these discriminating cases before other new assertions can mask their failures. Preserve the sibling `prepared-control.js` import. Record baseline/candidate identities, substitution match count, actual invocation, exit and named assertion. Import failures, timeouts, syntax failures, unrelated assertions or ambiguous substitutions do not count.

After both named controls, add one synchronous single-read discriminator: expose ordinary enumerable `sourceRingFrames` through a getter that returns `256` on its first read and `512` thereafter. Require exactly one caller-field read, constructor value `256`, and no later reread through initialization. This directly rejects validate-then-reread snapshot construction without adding another mutant or harness.

These are two **additional snapshot controls**. The existing real-Wasm corrupted-byte and disposal controls remain unchanged.

**Qualified Wasm and candidate assembly.**  
Use the currently qualified Wasm SHA-256:

```text
b47d05f053dca81687f0065306fb97159f892277e9543326cea7e21544186b61
```

The verified preserved source directory is:

```text
/tmp/miso-842-audioworklet-ordinary.ODpb0F
```

Its seven-file manifest, recomputed during scoping, is:

```text
8cd125f03d7bd5e98d93c4756af3bfe3282d709bceed1b55772140d5b57cb25d  miso-engine-v1-abi-layout.json
63ff9a8df39045c7f70ce3272607cf4d13bd6ea795744ff212014aa8c64b96d1  miso-engine-v1-audio-worklet-host.d.ts
3c8f1ca3dc0776f1363e45d93c07fdb0b5e6a666a7ed99114bace82973028f8e  miso-engine-v1-audio-worklet-host.js
76d08e74b6082e3d1c8accc4e3f3b291a8cfc794f884f2e0d3529929286c3191  miso-engine-v1-audio-worklet.js
b47d05f053dca81687f0065306fb97159f892277e9543326cea7e21544186b61  miso-engine-v1-audio-worklet.simd128.wasm
f7fb112328d64daf38d29508cbc360d4113771e4b575b8e25ab7ada0187f10b4  miso-engine-v1-parameter-metadata.json
3f20d99da2fa4e0b461f63cef01050264f67ecebb82495ad04b7318bf9f5dca1  prepared-control.js
```

Reverify this directory before use. Assemble a **fresh** external nonsymlink candidate directory containing exactly those seven regular nonsymlink filenames:

- Copy only the Wasm from the preserved qualified output.
- Copy the two JSON siblings from same-named files under `sdk/assets/`.
- Copy host JS, host declaration, worklet JS and `prepared-control.js` from same-named files under `hosts/host-web/web/` at the frozen candidate revision.

The corrected host must come from candidate source. Its hash is expected to change; the other six artifact hashes must remain as above. Compare all six non-Wasm companions byte-for-byte with candidate source authorities. Record all seven candidate hashes before and after proof, with logs outside the candidate directory.

Do not modify the preserved directory, repin Wasm, change `REAL_WASM_SHA256`, invoke report-mode rebuilding, silently rebuild a missing artifact, or substitute stale output. Missing/mismatched preserved bytes require a separate artifact decision.

Current `preflightRealArtifact` already checks seven-file membership, regular-file entries, qualified Wasm identity, repository pin and all six source companions. It supports this candidate assembly without loader changes.

Run the unchanged command:

```bash
timeout 60s node scripts/test-web-audioworklet.mjs \
  --real-wasm-receiver --artifacts "$SNAPSHOT_CANDIDATE_DIR"
```

`SNAPSHOT_CANDIDATE_DIR` must name the newly recorded absolute directory. Run without fake-module overrides, `--qualification-module`, or unreviewed Node loader overrides.

Require ordinary Ready/OK, `simd128`/numeric backend `1`, 48 kHz/128 frames, zero rendered-quanta/absolute-sample counters, correlated status/disposal, exactly one first-disposal send, repeated-disposal suppression, and saved native-handle status-pointer zero after disposal. Preserve both existing real-Wasm red controls and their cleanup.

This is ordinary real-Wasm receiver regression evidence under Node’s Web Audio facade. It is not new browser suspension qualification, protected native boot, protected asset attestation, artifact publication or SDK release evidence.

**Proportional local checks.**  
On the frozen corrected candidate, record actual exits for:

```bash
node --check hosts/host-web/web/miso-engine-v1-audio-worklet-host.js
node --check scripts/test-web-audioworklet.mjs
node scripts/test-web-audioworklet.mjs
cmp hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts \
    sdk/src/browser/shipped-host.d.ts
bash scripts/check-sdk-types.sh
git diff --check
```

Also require the two new snapshot controls, unchanged real-Wasm invocation and exact allowed-path diff. Declaration byte parity and installed TypeScript were verified during scoping; recheck execution prerequisites rather than assuming temporary state persists.

No local Rust rebuild, benchmark, listening corpus or new browser matrix is needed for this JS preparation change. Existing required integration CI remains mandatory; failures do not authorize out-of-scope repairs.

**ACK and realtime boundary.**  
Snapshot copying and initial validation precede the first await. The context recheck occurs after asynchronous loading, immediately before node construction. All remain on the main control thread. No changes to `process()`, render, native admission, request allocation, reservation, transfer machinery, queue capacities, receipt retention or ACK validation.

The review question remains: **can an ACK ever precede a drop?** For this issue, evidence is the unchanged queue/ACK implementation plus passing existing saturation/ownership tests and captured-limit behavioral tests. A local saturation refusal must still precede posting/transfer and must not burn a request ID.

This issue earns no new protected ACK-before-drop guarantee. #839’s accepted native refusal evidence remains applicable, but does not establish browser-side refusal before callbacks, preparation work, transfer or staging.

**Explicit successor boundaries.**  
The following remain separate stateless slices:

- Protected asset preflight: exact fetched-byte identity, layout verification, worklet sibling identity and the future protected policy for opaque supplied modules.
- Protected public/raw guards, bounded receipt/result retention, native protected boot/status identity and suspended-browser lifecycle.
- Native-owner failure reclamation and stronger concurrent-disposal/lost-transport semantics.
- Aggregate ordinary/protected monitoring accounting and preservation of peaks, GR, clock behavior and PCM under #835’s conditional coexistence ruling.
- Target-selection identity and delayed-receipt/capture handling.
- SDK/app integration, package publication, adoption and release.
- Broader artifact provenance, production trace/PCM tooling and exhaustive platform qualification.

#839 resolved the named native unsupported-operation credit-spending defect. Do not carry that historical defect forward as still open, or treat its resolution as completion of these broader successors.

**Issue-first execution and delivery.**  
Before implementation, root audits `.github/ISSUE_SPECS/` against GitHub and synchronizes relevant boundary evidence. Preserve upstream commit `99cf4711777f416c07ff6e0e161355786497d265`, which contains #839/#842’s final delivery records; do not overwrite their GitHub bodies with qualified main’s older text.

After independent review of this draft, create the matching GitHub issue and local numbered spec in the same brief checkpoint. Use the number actually assigned; verify number, exact title/body and OPEN state before implementation. Root owns integration and checkpoints. This issue is the sole active launch-critical implementation WIP.

Use two serial fresh **Luna MAX** tiny handoffs for issue-wide attempt 1:

1. **Tests/red controls:** modify only the receiver test file; deliver the compact snapshot cases, baseline named failure and the precise shallow-control recipe. Pause for root’s exact-path checkpoint. Document the intentional red state; do not describe it as green.
2. **Host correction:** modify only host JS to satisfy the frozen tests; complete the declared shallow-mutant evidence and focused checks. Pause as soon as the coherent tranche is ready for root’s exact-path checkpoint.

The shallow candidate control is executed once the corrected factory exists. That completion is part of the same coherent attempt, not another handoff-based attempt allowance.

The issue-wide submitted-candidate ceiling is:

| Attempt | Implementer |
|---|---|
| 1 | Luna MAX; the two serial handoffs above |
| 2, only if needed | Fresh Luna MAX |
| 3, only if needed | Fresh Sol HIGH, once |
| 4, only if needed | Fresh Astra XHIGH, once |

Every submitted candidate receives one fresh independent **Astra MEDIUM** adversarial verdict on a frozen revision. Handoffs, checkpoints, fixture corrections and agent replacement never reset the ledger. A failed verdict followed by correction advances the attempt count. Stop after attempt 4 fails.

Keep at most one uncommitted implementation tranche. Root audits status, exact changed paths, commit and upstream state at each checkpoint; no next tranche begins while its predecessor awaits the required checkpoint. Push promptly in checkpoint-push mode. If CI-conscious batching is explicitly selected, retain frequent local checkpoints on one batch branch and defer pushes/PR updates until the declared boundary.

The hard limit is **four active hours from execution admission through final candidate review**, including setup, tests, corrections, handoffs and checkpoint work; passive remote CI waiting is excluded. This cap is plausible for the compact two-file fix and is a stop rule, not permission to omit gates.

Record candidate revision, changed paths, commands/exits, red-control assertion evidence, artifact manifests, elapsed active time, attempt owner and verdict in the issue. Freeze the candidate during review. Publish PASS evidence, synchronize the issue body/state and verify closure before counting issue completion. Distinguish accepted upstream source from merged delivery until required PR/main qualification succeeds.

After merged delivery and synchronized evidence, remove completed clean worktrees only when all checkpoints are upstream and unique evidence is preserved. Retain the primary checkout, active worktrees and history.

**Stop conditions.**  
Stop and rebrief if the work requires a third implementation file; declaration/API/wire change; worklet/native/SDK modification; queue or ACK redesign; broader cleanup; loader/preflight repair; artifact rebuild/repin; new browser qualification machinery; weakened or nondiscriminating tests; unexplained sibling/hash drift; exhausted attempts; or the four-active-hour limit.

Do not turn a snapshot failure into a protected-browser implementation project. Close only for the reviewed and synchronized capability:

> Public boot preparation is isolated from caller mutation.
