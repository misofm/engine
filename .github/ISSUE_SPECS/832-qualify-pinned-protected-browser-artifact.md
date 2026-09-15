# Qualify pinned protected-browser AudioWorklet resource fixture and browser matrix

## Outcome and authority

The frozen protected-browser EQ branch at `f2355988c0e51f4d283dd028bb98ce17f5038113` builds one seven-file shipped artifact whose simd128 Wasm SHA-256 is `18b9dbfa61ae1188fcb00f18317702e37feb37c4843ac2b885194a4c77322cab`. PR #831's required qualification run 35027155140 failed before browser execution because its checked matrix names the older Wasm `87555e1e...`; the artifact gate also rejects stale ordinary-boot fixture rows. This issue owns one smallest closable qualification outcome: derive and promote truthful resource and three-browser evidence for the frozen artifact, then restore green required qualification. It does not alter engine/runtime behavior, artifact bytes, SDK/API behavior, package versions, CI workflows or gates.

Fresh Astra XHIGH scoping returned GO for a frozen-artifact qualification successor. Independent Astra XHIGH adversarial planning returned NO-GO on an unamended pin-first recipe and requires the exact layout accounting, five-field freeze, SDK browser coverage and post-record lineage checks below. No implementation begins until a fresh plan recheck accepts this amended brief.

## Frozen changes and scope

The existing direct Wasm oracle, executed with `MISO_ENGINE_WEB_ORACLE_PRINT=1` on the preserved artifact, differs from `hosts/host-web/tests/browser-v1/expected.json` in exactly five fields. PCM digests and every other oracle field agree:

| `directOracle` field | old | measured new |
|---|---:|---:|
| `simd128.resources.bridgeMetadataBytes` | 1146207 | 1149255 |
| `simd128.resources.bridgeRetainedBytes` | 1166716 | 1169764 |
| `simd128.initialStatus.memoryBytes` | 1310720 | 1376256 |
| `simd128.beforeDisposeStatus.memoryBytes` | 1310720 | 1376256 |
| `commandTimeline.beforeDisposeStatus.memoryBytes` | 1376256 | 1441792 |

The two bridge rows move together by 3048 bytes; three linear-memory capacity snapshots move by one 65536-byte Wasm page each. The bridge delta is ordinary-boot layout/accounting, not #830's protected-only 2097152-byte payload charge. Before promoting the fixture, record compiler/layout evidence for the named changed terms: actual wasm32 `size_of::<AudioWorkletEngineHost>()` growth, the newly included full containing `size_of::<RefCell<ObservationStaging>>()` and unchanged heap/staging terms. Explain how those terms sum to exactly 3048. Explain memory pages as capacity snapshots and preserve the oracle's no-growth-during-tested-timelines assertions; do not infer the cause of a page increase from the bridge delta alone. Compile any transient witness in a separate scratch checkout and target/output directory; its module must never replace or masquerade as the preserved qualification artifact.

Allowed tracked files: this spec/evidence; `hosts/host-web/tests/browser-v1/expected.json` for only the five frozen oracle fields; generated `hosts/host-web/qualification/results.json` and `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`. A transient layout witness may be compiled in an isolated scratch checkout, then removed before checkpoint; do not introduce a production field, a permanent mirror, another benchmark framework or a new eighth artifact. Unexpected behavioral failures, extra oracle drift, or browser floor/outcome changes require attribution and rebrief rather than blindly re-pinning.

## Serial bounded execution

1. Root verifies source/artifact identity and obtains the actual wasm32 layout accounting. A fresh Luna MAX agent edits only `expected.json`'s five fields after the reviewable accounting is recorded; focused direct-oracle equality and existing resource/native comparator gate must pass. Root commits/pushes that exact-path checkpoint before more tracked edits.
2. Root first verifies that `sdk/dist/core` is absent, or proves any present distribution derives from the frozen candidate inputs; `buildSdkBundle` prefers that directory over SDK source. A fresh Luna MAX agent then runs the existing qualification recorder on the same artifact with full 40-hex candidate SHA, `--sdk-root`, `--browser all`, `--self-test-mutations` and `--record-matrix`; it edits only its two generated outputs `results.json` and `BROWSER_DEPLOYMENT_MATRIX.md`. Recorder writes outputs only after Chromium, Firefox and WebKit pass all existing gates. Root commits/pushes the exact generated pair before evidence layers.
3. Root runs `--check-matrix --self-test-mutations` after recording, checks generated document and resource negative controls, records independent Astra MEDIUM issue-wide verdict, then runs PR #831's required `qualification`. Close this issue only after verdict/evidence commit is upstream, PR qualification green, matching GitHub body/state synchronized; verify remote closure. The PR and app delivery remain separate until merged and released.

Each implementation handoff is at most two coupled files and one focused objective gate. The **single issue-wide ceiling** is two Luna MAX implementation rounds total, then one Sol HIGH round, then one Astra XHIGH round if still unsatisfactory; no slice, checkpoint or test correction resets that ceiling. A task reaches Sol after at most two unsatisfactory Luna rounds and reaches Astra after Sol's one unsatisfactory round. Each coherent attempt receives one adversarial verdict. Preserve evidence and rebrief instead of disguising a further retry. Only one launch-critical implementation tranche may be uncommitted at a time.

## Objective gates

- Exact seven-file artifact set, current source pin and six companion authorities; root separately verifies source/commit existence and artifact provenance because matrix candidate-lineage checks SHA syntax only. No rebuild/repin/version change in this issue.
- Five-field measured oracle red/green discrimination, unchanged PCM digests and other rows, normal direct-oracle equality, `scripts/check-browser-expected-resources.py --artifacts` including native witness and 26/26 red mutations, normal artifact/static/resource gates.
- Real Chromium, Firefox and WebKit AudioWorklet/attestation/native digest/control/observation/stall/SDK response gates using the pinned Playwright 1.62.1 and existing private PulseAudio sink. Recorded browser floors/outcomes must be witnessed, not copied.
- Matrix check mode with SDK root verifies candidate/artifact lineage, every checked row, document generation and lineage/artifact-set/result mutations. Preflight `sdk/dist/core` against frozen source before recording and check mode. Malformed candidate and wrong Wasm must still fail.
- Focused fmt/diff and required PR qualification once the coherent issue evidence is pushed. Browser qualification is not a descriptive benchmark; do not time/tune/retry it as one.

The recording command uses `--candidate-commit f2355988c0e51f4d283dd028bb98ce17f5038113`, not its abbreviated SHA; evidence-only checkpoints afterward do not change the frozen source or artifact. Do not combine `--record-matrix` with stale `--check-matrix`, which would refuse before browser execution. Run check mode after recording.

## Evidence/decision record

Initial CI: shipped artifact, SDK, static/object, boot-budget and other qualification leaves succeeded; three browsers stopped at matrix lineage before browser behavior; resource oracle stopped on five stale fields. Independent read-only scoper and adversary found no established browser runtime failure. Qualification and app deployability remain unclaimed until gates above pass.

Fresh independent Astra MEDIUM final plan recheck: **PLAN GO, no concrete blockers** after the issue-wide attempt ceiling, SDK distribution provenance, scratch isolation and syntax-only candidate-lineage clarifications. The five-field fixture and two generated-matrix slices are authorized by the amended stateless brief; implementation and remote CI evidence remain pending.
