# Final combined metering review — issues 516, 519 and 520

**Verdict: PASS for the combined engine capabilities and the exact qualified Linux artifact.**

Evidence candidate: `a1aeefb830b2ac192274153539847765385ed7f1`.
Executable source was frozen at `11cb3c2e`; subsequent committed changes are artifact identity and issue/failure evidence only.
Shipped Wasm SHA-256: `671d615de9a74232c2af623592fc85eca68b8cd571fb5b7542ba648e621af4e3`.

Reviewer independently hashed `/private/tmp/engine-519-linux-final-output/miso-engine-v1-audio-worklet.simd128.wasm`. The result matches the committed pin and the actual generated `qualification/results.json`, whose candidate is `a1aeefb8`. This is the canonical Linux/amd64 Rust 1.97.1 build; earlier Mac/candidate hashes and their qualification outcomes are not substituted for this artifact.

## Accepted capabilities

- #516: exact per-window track/final-master peak publication, stable empty/invalid poll behavior, bounded stale/lost interval recovery and lease epochs, and explicit independent GR age semantics. Strict main-realm transport validation accepts and validates the emitted metadata.
- #519: engine-owned source/tap/metric selection usable through shared host-core preparation, explicit snapshot presence, preserved default full numerical path, sanitization in every selected numeric pass, and actual removal of unrequested energy/count/held/sqrt work. Fixed-layout queue, seal and host pending storage remains resource-accounted.
- #520: O(1) return before track/effect scans when no complete master interval exists, continuous observation of required samples, and retained bounded recovery at ready boundaries. No publication cadence or UI/subscription API is added.

These findings incorporate the independently executed focused reviews recorded in `/private/tmp/engine-516-attempt2-review.md`, `/private/tmp/engine-519-review.md`, and `/private/tmp/engine-520-review.md`. No new broad reruns were performed for this final evidence review.

## Final inspected qualification

- `/private/tmp/engine-519-520-static.log`: unchanged static/object/callback, kernel roster, ABI/metadata, vocabulary and boot-budget high-water checks PASS; zero budget mismatches.
- `/private/tmp/engine-519-520-browser.log` and actual generated matrix/results: exact six-file shipped set and Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 qualification PASS, including mutation proofs.
- `/private/tmp/engine-519-520-hermetic.log`: actual worklet/host execution and malformed-message, allocator, opcode/callgraph and protocol mutation proofs PASS. Intentional negative mutation diagnostics are not candidate failures.
- Prior source-checkpoint focused/full test results remain applicable; the final evidence/pin commits do not modify executable source.

No gate or callgraph allowlist was weakened. The preexisting platform tan-ULP test failure was independently reproduced on untouched pre-metering and issue-parent sources and is not silently claimed fixed.

## Performance and delivery limits

The sole descriptive timing invocation failed during warmup because the timing fixture omitted its final end-of-region marker. No measured rounds or speedup figures are available. Failed JSONL/log evidence is committed under `docs/evidence/metering-519-520/`; the bounded repair and new successor-owned invocation are explicitly assigned to issue #522. This is a transparent tooling transfer under the repository ceremony boundary, not a retry or a relaxation of engine correctness/work-avoidance evidence. No microsecond, percentage, cycle or end-to-end performance gain is claimed by this PASS.

There are no remaining technical blockers for #516/#519/#520 in the reviewed scope. At final review, the newly generated browser matrix/results were still root-owned uncommitted changes. Root must commit/push final evidence, synchronize issue/PR state and verify CI before claiming remote delivery. This verdict does not claim those delivery steps are already complete and does not authorize or perform a merge.

## CI resource-fixture follow-up

The combined verdict is **qualification-conditional again** after broader CI found two stale fixture authorities: the native builtin resource ABI pin and browser direct-oracle retained metadata rows. These gates were not exercised by the earlier scoped static/browser logs. They must pass before restoring final qualification PASS; no limits or comparisons may be weakened.

Independent native layout probe, linking existing baseline/current builtins rlibs, found MeterSnapshot=160 (alignment 8) and Option<MeterSnapshot>=160 unchanged, while MeterAccumulator increased 224 -> 232 (alignment 8). The metric presence byte occupies snapshot padding; it does not enlarge queue slots. The resource formula therefore increases meter/retained totals by exactly 8 per observer: +8 for the one-meter rows, +56 for seven meters, zero for no meters. Allocation counts, meter items, queue sizes and maximum-single-allocation rows remain unchanged. Expected pairs (meter,total): tracks1/meters1=(1166,2291);1/7=(11522,12647);4/1=(1178,5750);4/7=(11606,16178);65537/1=(1178,77169311);65537/7=(11606,77179739).

The actual Linux module's no-meter browser direct oracle reports bridgeMetadataBytes4243 ->4339 and bridgeRetainedBytes24752 ->24848, both +96. Because this fixture prepares no observers, the delta is retained host-shell metadata from the #516 delivery state, not a meter queue/snapshot payload increase. A bounded authority/manifest refresh with exact byte rationale and unchanged resource checks is appropriate; timing remains unavailable and no rerun is implicated.

## Resource-authority correction accepted

**Combined technical PASS restored** after the bounded resource-fixture correction and exact previously failing gates.

Reviewed the final four authority/fixture paths: native observer-size literal224 ->232, six resource rows with exactly8bytes per observer, resource-only MANIFEST.tsv entry, matching joined manifest identity, and the two browser expected bridge rows increasing96bytes. Independently hashed the final manifest: `31798260263396c242c0b90042e01abb18624f383fd88029341dffecde662796`, exactly matching the updated joined-corpus assertion. The resource payload remains2361bytes with SHA-256 `a2bb672738d5b2bca6e60ec030467d30a990a61f1266e725d1cdda4df2fded45`. No PCM/response fixture or resource-limit predicate changed.

Independent Wasm compiler-layout diagnostics resolved the browser sum exactly: ReadyOwnership952 ->1040 (+88), comprising92added field bytes with end padding7 ->3 (4bytes reused); AudioWorkletEngineHost1424 ->1520 (+96), adding the embedded-ready88bytes and8-byte activation sample with unchanged outer padding. The no-meter direct fixture therefore adds96only to bridgeMetadataBytes and bridgeRetainedBytes.

Inspected `/private/tmp/engine-521-resource-oracle-gates.log`: fixture check PASS; issue064 read-only corpus PASS; all24corruption rejections PASS; issue067 graph/PDC PASS; actual browser expected resources against the qualifiedLinux671d artifact PASS; all26resource comparator mutations PASS. The fixture author produced a preexisting macOS tangent-response difference, which was explicitly excluded rather than copied or used to weaken DSP expectations. Only the derived resource/manifest files were adopted.

The correction updates test authorities to the independently measured layouts and restores exact gate agreement; it does not weaken resource gates or change the production artifact. The prior combined functionality, static/browser and no-performance-claim findings stand. Remote push, GitHub synchronization and the next CI result remain root delivery obligations; no merge is performed or implied by this restored technical PASS.

## Live benchmark manifest consumers synchronized

Reviewed checkpoint `5934fe0f125f1b025ba352cf922387ce13cdd4a1`: **PASS**. Exactly five literal references across the current benchmark preflight, record validator, lifecycle fixture and bench source now identify the independently verified resource manifest `31798260263396c242c0b90042e01abb18624f383fd88029341dffecde662796`. No workload/input hashes, validator predicates, timing behavior or historical result artifacts changed. An independent search found no remaining old identity in active scripts/tools/fixtures; old references in historical issue431/473 prose were appropriately preserved.

Root/Sol report Linux/amd64 current benchmark validators/lifecycle PASS with real workload launches0, the bench manifest unit test PASS1, and formatting/bash-syntax/diff checks PASS. No descriptive benchmark was retried. This resolves the remaining active-consumer identity propagation gap without relaxing a gate or changing the shipped engine module.

Combined technical PASS stands. No known local qualification blocker remains; final remote CI verification and GitHub delivery synchronization remain outstanding root obligations. No merge or performance-gain claim is included.
