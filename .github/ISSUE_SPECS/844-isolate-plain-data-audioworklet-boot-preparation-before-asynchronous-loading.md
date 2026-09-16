# Isolate plain-data AudioWorklet boot preparation before asynchronous loading

**GitHub issue:** [#844](https://github.com/misofm/engine/issues/844).

## Outcome

For the supported plain-data grammar, `createMisoAudioWorkletHost(input)` owns all boot data before its first suspension. Unsupported record and array shapes refuse locally without invoking their accessors, setters, traversal methods, iterators, constructors, or species hooks.

This is a deliberately narrower successor to exhausted #843, not a fifth correction. #843 required broad getter and shadowed-method compatibility; all four candidates failed, finally because caller-controlled `entries.every` could retain a callback that wrote into private storage. This issue removes that obligation by refusing accessors and decorated arrays before invocation. Preserve #843's complete failed ledger and never describe its broader contract as passed.

Execution admitted at **2026-09-16 07:17:43 UTC** from clean synchronized main `299c828570f57cfbb4e1c82ced817be2ca5050b6` after fresh Astra XHIGH scope, separate fresh Astra XHIGH adversarial GO, and the required fresh Sol HIGH stateless brief. Local spec, GitHub title/body/number and OPEN state are exact at checkpoint `d7bff077`. This starts the 120-active-minute ceiling.

The first Luna MAX test handoff was stopped after six minutes of inspection with no worktree edits, then the second and final Luna round for this bounded task delivered checkpoint **`d8cc66db`**. Only the receiver test and two mirrored declaration comments changed: 203 test insertions and four comment lines per declaration, with no type/signature or production edit. The two focused selectors reach construction and exit1 only at `boot-data.document.before-construction` and `boot-data.nested.before-construction` against both current main and a byte-exact private `299c8285` host; the nested case retains its sparse-hole assertion after the named value assertion. Root independently confirmed test syntax, declaration parity, diff hygiene and the unchanged default fake suite at exit0. Logs are `/tmp/miso-844-root-{default,document-red,nested-red,baseline-document-red,baseline-nested-red}.log`; Luna report `/tmp/miso-844-luna1b-minimal.md`. This is the required intentional-red checkpoint, not a submitted complete candidate.

The first Luna MAX production tranche delivered checkpoint **`6bcb03c4`**. Only the shipping host JavaScript changed: 275 insertions and 27 deletions add captured host-realm descriptor primitives, schema-specific private boot snapshots, sparse collection-array copying, native branded `Uint8Array` range copying, and captured context/loading values. Root independently confirmed host and test syntax, both focused selectors, the complete fake suite, declaration parity, diff hygiene, and byte-equivalence of the complete host-class and real-receiver regions. The byte-verified `299c8285` baseline fails only at `boot-data.document.before-construction`; the one-line retained-entries mutant fails only at `boot-data.nested.before-construction`; the corrected host passes both. A separate edge probe passed representative hidden-unknown, accessor, decorated-array, forged/shared-view refusal and valid hidden-collection acceptance with no witness invocation or loading activity on refusal. Root logs are `/tmp/miso-844-root-{host-syntax,test-syntax2,document-green,nested-green,default-host,decl-cmp2,diff-check2,ownership-mutant,baseline-document2}.log`; Luna report `/tmp/miso-844-luna2-host.md`. The permanent compact acceptance matrix and frozen-candidate review remain outstanding, so this is not yet submitted candidate 1.

The bounded Luna MAX refusal/storage test task used two rounds and delivered checkpoint **`7fabe470`**. Only the existing fake-host test changed: 258 insertions and five deletions add exact local-refusal/no-activity assertions for getter and setter-only descriptors at all six schema positions, hidden descriptors, decorated arrays including a hidden `every`, unknown and prototype/index shapes, shared/detached/forged byte inputs, accepted decorated offset and empty genuine byte views, unsupported SIMD ordering, and state/rate/quantum drift before construction. Root independently reran host/test syntax, the full fake suite, both focused selectors, declaration parity and diff hygiene at exit0. The accepted/domain and returned-capacity table remains outstanding, so candidate 1 is still not submitted.

The separate accepted/domain Luna MAX task used two rounds and delivered checkpoint **`5f6eaefd`**. Only the existing fake-host test changed: 225 insertions and one deletion cover same-realm literal, null-prototype and frozen records at every schema level; frozen sparse arrays with hidden indices; six/eight/seven-field and named hidden-property rules; prepared-module enumerability; representative numeric, dependency, enum, UTF-8 ID and distinct capture-limit domains; and the captured default quantum of 128. Every accepted host is disposed, every refusal asserts exact local result1/request0 with no loading or node activity, and root independently reran the full required local command set at exit0. The third single-spectrum mutation episode and final frozen-candidate review remain outstanding.

The final bounded Luna MAX episode delivered implementation/test checkpoint **`d7977f52`** with 104 test-only insertions. A single-spectrum boot paused at `addModule`, then caller factory, URL, option and spectrum mutation could not change construction. The returned host proved the derived 77-block default source depth at 48 kHz/64 frames and the zero-to-one command fallback: capacity+1 refused locally with result6/request0, no post or transfer, retained source storage, and no request-ID gap after release. Root froze candidate 1 at that revision and reran host/test syntax, the complete fake suite, both focused selectors, declaration parity and diff hygiene at exit0. The byte-verified `299c8285` baseline exited 1 only at `boot-data.document.before-construction`; a uniquely matched one-line retained-entries mutant exited 1 only at `boot-data.nested.before-construction`. The complete host-class and real-Wasm receiver regions remain byte-equivalent to `299c8285`; the tree is clean and the six changed paths exactly match the issue allowance. Logs are `/tmp/miso-844-c1-{host-syntax,test-syntax,default,document,nested,declarations,diff-check,baseline-document,mutant-nested,host-class-diff,real-receiver-diff,changed-paths,diff-stat,status}.log`. This is submitted complete candidate 1 and awaits its fresh independent Astra MEDIUM verdict.

Fresh independent Astra MEDIUM reviewed frozen candidate 1 at **`c26b3297`** and recorded **FAIL** in `/tmp/miso-844-astra-medium-c1.md`. First, `snapshotBootOptions` selected legacy shape from the value `spectrum === undefined`, so a valid eight-field record with enumerable `spectrum: undefined` and `spectrumCollection` was refused despite the frozen eight-field/null-normalization contract; an independent baseline/candidate probe discriminated loading reached versus local result1. Second, the three episodes did not withhold Ready/ABI or prove postconstruction isolation, mutated explicit source capacity, and captured observation defaults. All required commands and both red controls otherwise behaved correctly, and inspection found no second production defect. Candidate 1 remains failed; candidate 2 is limited to the shape correction and missing ownership/capacity evidence.

Candidate 2 correction A used a fresh Luna MAX child for two bounded rounds and delivered checkpoint **`98bf45b8`**. Production now selects six-field legacy versus eight-field extended shape from optional descriptor enumerability instead of the spectrum value. Compact tests prove eight enumerable fields with undefined spectrum plus null or valid collection, hidden null normalization, and preservation of seven-field and hidden-nonnull refusal. Only the host and existing test changed (43 insertions, three deletions); root independently reran the full required local command set at exit0. Correction B remains evidence-only and will address the Ready/ABI/captured-capacity finding before candidate 2 is submitted.

## Frozen product contract

Assume standard, unmodified host-realm intrinsics and non-Proxy inputs. Inspect only the fixed factory, boot, spectrum, collection, entries-array, and entry schemas.

- A preparation record is a non-array object whose immediate prototype is the executing host module realm's `Object.prototype` or `null`. Every own property must be a data descriptor with a known string name. Unknown enumerable or hidden names, symbols, and accessor or setter-only descriptors refuse. Frozen, sealed, read-only, and null-prototype records remain valid.
- A collection entries value must pass `Array.isArray` and have the host realm's exact `Array.prototype`. Its only own keys may be `length` and canonical array indices below that length. Present indices must be data descriptors. Preserve length, holes, and index enumerability in a fresh ordinary array. Refuse subclasses, custom prototypes, noncanonical index names, symbols, and any extra property—including `every`, `map`, `constructor`, iterator, or species—even when hidden or `undefined`. Invoke none of them and expose no callback.
- Proxies, hostile intrinsic replacement, and malicious context/module shims are outside the supported contract. Reflection may execute Proxy traps, so do not claim universal Proxy detection, trap-free inspection, or atomic capture against reentrant traps. Normalize a thrown inspection failure, but add no Proxy detector, membrane, or compatibility campaign.
- Direct foreign-realm records and arrays are unsupported; callers may construct local plain data. This is not general cross-realm qualification. Capability and byte exceptions below retain their stated realm behavior.

Three values are intentionally not plain records:

1. `context` is the existing AudioContext/BaseAudioContext or structural test capability. Preserve its identity and receiver. Native context accessors and `audioWorklet.addModule` are permitted. Capture identity, initial suspended state, positive-u32 sample rate, effective quantum (`renderQuantumSize ?? 128`), and the `audioWorklet` reference; do not clone or freeze the capability. A context is not refused merely because it originates in another realm, but arbitrary cross-realm Web Audio is not qualified.
2. `preparedModule` preserves the existing `instanceof WebAssembly.Module` check and exact identity. A genuine accepted module skips Wasm fetch/compile, but not SIMD attestation, `addModule`, ABI loading, or Ready validation. This exemption neither authenticates prototype forgeries nor promises foreign-realm module acceptance.
3. `document` must be a genuinely branded `Uint8Array`. Use native typed-array tag and buffer accessors plus a native ArrayBuffer brand check; do not read caller `buffer`, `constructor`, `Symbol.toStringTag`, iterator, species, or call buffer `slice`. Copy the visible range before suspension with the native typed-array copy path into fresh ordinary, nonshared, offset-zero storage. Initially shared-backed, detached, forged, or invalid views refuse; attached empty views pass host validation. Later caller mutation or detachment cannot affect the copy. View/backing decorations are ignored without invocation, and caller storage is neither frozen nor transferred.

Read descriptors at the factory boundary before any direct semantic field access. Reject invalid shape, then use only captured descriptor values. Recurse only through the fixed schema and create writable private construction records; do not create a generic deep copier or emulate cached getters. Preserve original enumerability for validation before normalizing optional spectrum values.

Retain the existing field and domain contract for accepted plain data:

- Factory fields are `context`, `document`, `options`, `simd128ModuleUrl`, `workletModuleUrl`, and optional `preparedModule`. Enumerable `preparedModule: undefined` or `null` refuses; hidden `undefined` remains accepted; a hidden defined module does not satisfy the required enumerable module case.
- Boot fields are `sourceRingFrames`, `maximumMemoryBytes`, `consoleCommandQueueRecords`, `consoleMeterBlocks`, `consoleObservationTaps`, `consoleMasterTrackPlusOne`, `spectrum`, and `spectrumCollection`. Six-field legacy and eight-field extended records accept; seven fields refuse. Preserve null/undefined normalization and mutual exclusion. Six fields plus hidden nonnull `spectrum` refuse; a valid hidden `spectrumCollection` remains effective; hidden required collection `entries` refuses.
- Spectrum fields are `target`, `targetId`, `channels`, `maximumCaptureBytes`; collection fields are `entries`, `maximumCaptureBytes`; entry fields are `target`, `targetId`, `channels`. Keep current targets/channels, nonempty IDs through 127 UTF-8 bytes, and current primitive-only validation.
- Keep numeric u32 `sourceRingFrames`; u64 BigInt memory/console words; command records at most `256n`; taps at most `16n`; meter/master u32 bounds and queue/tap/master dependencies. Single capture bytes remain safe integers `1..1_048_576`; collection bytes remain any positive safe integer. Do not import the single-capture ceiling into collections.

All shape, domain, and storage validation completes in the synchronous prefix. Refusal is an already-rejected Promise with exactly `{tag: "miso.error.v1", requestId: 0, result: 1}` before fetch, compile, `addModule`, or node construction; the async API does not become synchronously throwing. Normalize descriptor/storage failures only at this boundary and preserve later initialization error handling. For otherwise valid preparation, typed unsupported-SIMD refusal still precedes `audioWorklet`, network, and node work.

Immediately before `new AudioWorkletNode`, with no intervening await, require the captured context still to be suspended at the captured sample rate and effective quantum. Drift refuses with result 1 and no node. This earns no postconstruction suspension guarantee. Constructor options, Ready checks, returned-host status, and capacities use the same captured values. Preserve explicit source depth `sourceRingFrames / quantumFrames`, default depth `Math.ceil(sampleRateHz / 10 / quantumFrames) + 2`, command capacity `Number(consoleCommandQueueRecords) || 1`, and observation default `Number(consoleMeterBlocks)`.

The complete host class is out of scope: no changes to request IDs, queues, reservations, transfer ownership, ACK parsing, disposal, or render behavior. Existing saturation must still return result 6 before posting or transfer and without spending an ID. The review question remains “can an ACK ever precede a drop?” Passing existing ownership/saturation regressions proves preservation only; this issue earns no new protected ACK guarantee.

## Exact paths

Only these six paths may change:

| Path | Allowed change |
|---|---|
| `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js` | Boot-only descriptor capture, schema-specific private copies, native byte copy, and factory wiring. No host-class or shared nonboot-validator edits. |
| `scripts/test-web-audioworklet.mjs` | Compact fake-host cases and the two focused selectors below. Keep the real receiver, preflight, pins, and existing controls unchanged. |
| `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` | One concise factory-contract comment; no type, signature, or export change. |
| `sdk/src/browser/shipped-host.d.ts` | Exact mirror of that comment only. |
| `.github/ISSUE_SPECS/#844-isolate-plain-data-audioworklet-boot-preparation-before-asynchronous-loading.md` | This stateless brief and concise evidence ledger. |
| `.github/ISSUE_SPECS/843-snapshot-public-audioworklet-boot-inputs-before-asynchronous-preparation.md` | Import/preserve the complete failed ledger; add successor and eventual supersession links only. |

No worklet, native, Rust, ABI, asset, pin, dependency, workflow, SDK runtime, app, or browser-fixture edit; no new API, option, export, production test hook, or generic harness.

## Compact acceptance evidence

Reuse the existing fake node/port, bounded compile/`addModule`/Ready/ABI pauses, counter snapshots, and `try/finally` cleanup. Keep the cases compact and table-driven:

- Three mutation episodes: legacy boot paused at compile; single spectrum paused at `addModule`; collection with a genuine supplied tiny module paused at `addModule`. Mutate/replace factory and nested records, offset-view bytes, URLs, array, and present entries while retaining a hole. Prove original context/module/URLs and values, fresh nested identities, offset-zero private bytes, supplied-module no-fetch/no-compile, and constructor identities separately from value snapshots.
- A rejection table with one accessor witness at factory, boot, spectrum, collection, array-index, and entry positions, including hidden and setter-only descriptors. Add decorated-array rows for `every` false/undefined/retained-callback functions, `map`, constructor, iterator, and species. Counters stay zero; no callback escapes; every row returns exact result1/request0 with zero loading/node activity. Include representative unknown enumerable/hidden/symbol, custom-prototype/subclass, and noncanonical-index refusals.
- A small accepted/domain table covering literal, null-prototype, and frozen records; frozen sparse arrays and a hidden data index; six/eight/seven fields and the named hidden-property rules; null/undefined and mutual exclusion; representative numeric bounds/dependencies, UTF-8 ID/enums, and distinct single/collection capture limits. Do not build a numeric Cartesian matrix.
- Storage/ordering edges: initially shared/detached refusal, attached-empty acceptance, a genuine byte view with throwing decorations copied without invoking them, and a forged view refused without invoking its iterator. Separately prove state/rate/quantum drift, default quantum 128, and unsupported SIMD before `audioWorklet` access.
- Fold Ready/ABI and returned-host capacity proof into the three episodes. Cover explicit/default source depth, zero-to-one command fallback, observation default, and capacity+1 result6 with no post, transfer, retained plane, or request-ID gap; then release replies and dispose. Do not duplicate the existing ACK, malformed-reply, and lifecycle suite.

Keep exactly two external red controls using the existing host-module override and unchanged sibling `prepared-control.js`:

1. Byte-verified baseline `299c8285`, focused selector `--boot-data-document`: construction must be reached and fail exactly `boot-data.document.before-construction`; the corrected candidate passes.
2. One uniquely matched private substitution in the frozen corrected candidate that retains caller collection entries, selector `--boot-data-nested`: construction must be reached and fail exactly `boot-data.nested.before-construction`; the corrected candidate passes.

Run value assertions before identity assertions. Record source revision, invocation, exit, named assertion, and the mutant's single substitution. Syntax/import failures, timeouts, unrelated assertions, ambiguous substitutions, or discarded late reads do not count. Do not revive #843's getter-read/order matrices, shadowed-method truthiness compatibility, Proxy suite, cross-realm campaign, or thousand-line compatibility harness.

For every frozen complete candidate, record actual exits for:

```sh
node --check hosts/host-web/web/miso-engine-v1-audio-worklet-host.js
node --check scripts/test-web-audioworklet.mjs
timeout 60s node scripts/test-web-audioworklet.mjs
cmp hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts sdk/src/browser/shipped-host.d.ts
git diff --check
```

Also record both red controls, exact changed paths, and inspection that the host class and real-receiver regions are unchanged. Required integration CI still applies at delivery. Do not assemble/promote artifacts, run browsers, run the real-Wasm receiver, rebuild Wasm/Rust, install dependencies for a comments-only type check, benchmark, or perform listening work. Historical #842/#843 downstream evidence may support unchanged code but is not new qualification for this candidate.

## Clean-main execution and bounded workflow

Create a separate worktree/branch from synchronized clean main, record the actual base, and inspect relevant intervening changes. The reviewed comparison baseline was `299c8285`; do not merge or cherry-pick failed candidate `47383caf` or #843's 1,484-line test tranche. Individually inspected small pause/fake-port fragments may be adapted. Import #843's full spec as documentation only and preserve its commits and external reports. Preserve newer #839/#842 upstream delivery records rather than overwriting them from stale main.

After root audits local specs against GitHub, create GitHub issue #844 and its matching numbered spec in one checkpoint; verify number, exact title/body, and OPEN state before implementation. This is the sole active launch-critical implementation WIP. Root owns exact-path checkpoints and status/commit/upstream audits, with at most one uncommitted tranche.

Attempt 1 uses two serial fresh Luna MAX tranches:

1. Tests and the two declaration comments only. Deliver the compact cases and baseline named failure, then stop for root's exact-path checkpoint. Record the new tests as intentionally red; this is not a submitted candidate. Target 20 active minutes and stop/checkpoint before 30.
2. Host JS only. Implement the small descriptor reader, schema-specific copies, and byte helper; pass the frozen tests and ownership mutant, then stop for root's exact-path checkpoint and frozen review. Target 25 active minutes and stop/checkpoint before 30.

The issue-wide submitted-candidate ledger is fixed:

| Candidate | Implementer |
|---|---|
| 1 | Luna MAX, from the two serial tranches above |
| 2, only if needed | Fresh Luna MAX |
| 3, only if needed | Fresh Sol HIGH |
| 4, only if needed | Fresh Astra XHIGH |

Every submitted complete candidate receives one fresh independent Astra MEDIUM adversarial verdict on its frozen revision. A correction after FAIL advances the candidate ledger; handoffs, fixture repairs, checkpoints, or agent replacement never reset it. The hard ceiling is four submitted candidates or 120 active minutes from execution admission through final candidate review, whichever arrives first. Time includes setup, fixture repair, handoffs, checks, checkpoints, and review; only passive remote-CI waiting is excluded.

Stop immediately on exhausted task allowance/candidate/time cap, an extra path, broader accessor/Proxy/cross-realm compatibility, a new harness or subsystem, native/wire/queue changes, artifact work, or out-of-scope CI repair. This ceiling is not permission to omit gates, and no fifth #843 attempt is authorized under another label.

Keep #843 OPEN and exhausted while this successor is only briefed or attempted. Only after successor PASS evidence is upstream may root synchronize and close #844 as completed, then close #843 as superseded/not planned with a link and an explicit statement that #843's broad contract never passed. Distinguish accepted upstream source from merged, required-CI-qualified delivery, and verify both remote issue states.

## Deployment boundary

This hardening is independent of mainnet launch on the already frozen SDK0.3.0/testnet artifact line recorded by #804/#809. The ordinary SDK route already copies document bytes synchronously and constructs fresh boot records, reducing—but not eliminating—exposure; direct callers of the old factory must keep inputs unchanged until initialization settles. No reviewed evidence shows a launch-path incident.

Landing this issue does not republish SDK0.3.0, replace deployed assets, qualify mainnet, or adopt the change in an app. It may land after that frozen mainnet launch. Reconsider launch dependency only if a concrete accepted-input mutation defect is demonstrated in the actual launch path or mainnet elects to adopt newer engine artifacts. This is not an overall launch-readiness verdict.

Close only for the reviewed, upstream-synchronized capability:

> Supported plain-data AudioWorklet boot preparation is owned before asynchronous loading; unsupported accessor and decorated-array shapes refuse without invocation.
