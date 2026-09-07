Status: OPEN; source freeze assigned by root.

QUALIFICATION_SOURCE_CHECKPOINT: e4f46fa808e413507d204e81b6a4ebc27254869c. Dedicated branch begins at its source-identical evidence checkpoint f2e87ab0b010b5f706038c5eaa8461a1e26fa7a4. All product/config bytes must match the frozen source. Earlier brief statements that no candidate exists are historical and superseded by this assignment. No repository pin edit is authorized before Astra MEDIUM PASS.

# Qualify the current AudioWorklet artifact before repinning

Status: numbered-ready stateless brief; after the corrected source checkpoint exists, root assigns the number, creates/synchronizes the spec and GitHub issue, creates the dedicated worktree from the frozen checkpoint, owns checkpoints/delivery, and may adjust `#555` only if already allocated.

## Premise and smallest closable outcome

Required qualification run `34108626421`, job `101699476520` (`shipped AudioWorklet artifact`), checked out its PR merge candidate and completed the pinned Rust `1.97.1` release Wasm build. The existing hermetic builder then correctly refused delivery: source pin `1bc18ab8cfb3e2a3e5a0ebeda185a64551e8398870abb2f3581077da0dfd3a3f`, observed module `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`. Raw evidence is `/tmp/issue349-partials/543-artifact-job.log`. Because the builder exits before copying and deletes its temporary target, that observation alone is not a retained or qualified artifact. The same required run also failed native runner policy. Root amended #543 at `758cd3e772b4f45d4952e3e5ca0f5ae1434c1036`, and a final Luna HIGH correction must replace the native runner's test-only `engine` dev-dependency with `bench-support`, its one test-helper delegate, and the corresponding lockfile edge. Therefore neither the failed run's merge candidate nor `97915c6d2d0d82addae049b6106f72beda23e519` is the source candidate for this qualification.

After that correction is checkpointed, root must explicitly assign and record its exact immutable source commit as `QUALIFICATION_SOURCE_CHECKPOINT` before any successor command runs. No final candidate exists at brief time. Qualify only that root-frozen post-correction #543 source as a six-file shipped artifact, using the current hermetic builder and current gates. Compare it with the retained current-engine #537 baseline `/tmp/issue537-delivery/artifact`, have Astra MEDIUM adjudicate every artifact delta and the gate evidence, and only after Astra PASS permit Luna HIGH to replace the repository pin. Then require the ordinary no-bypass builder to reproduce the prequalified six files byte-for-byte and required CI to report actual `qualification` SUCCESS.

This is artifact qualification supporting #543/PR553. It does not reopen or alter the accepted Rust hex implementation, does not claim #543 changed DSP/render behavior, and does not close audit #349 CP-20; CP-20 still requires delivered #552.

## Frozen scope

Before execution, root freezes `QUALIFICATION_SOURCE_CHECKPOINT` to the exact checkpoint after the stated Luna HIGH native-runner correction and records the commit identity in the issue evidence. Qualification must use that immutable checkout; a moving branch, PR head, merge ref, `758cd3e772b4f45d4952e3e5ca0f5ae1434c1036`, and `97915c6d2d0d82addae049b6106f72beda23e519` are not substitutes. Before Astra PASS, repository source and pins are read-only. Luna HIGH may create evidence and an isolated scratch copy outside the repository. The scratch copy may differ from the frozen source only at `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, set provisionally to the independently reproduced candidate hash so the unchanged builder can emit its output. That file is a comparison gate and is not an input to the Wasm bytes. Record and prove this one-file/one-line overlay; do not present it as a repository repin.

After Astra PASS and a root checkpoint/authorization, the only product/config edit is the single lowercase 64-hex line, with final newline, in:

`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`

The issue spec/evidence record may also be updated by root. Do not edit DSP, render, ABI, JS/TS, metadata, fixture/corpus/resource/PCM expectations, browser results/matrix, Cargo manifests/lockfile, toolchain/config, scripts, workflows, or test machinery. In particular, keep `Cargo.lock`, `rust-toolchain.toml`, `.cargo/config.toml`, `hosts/host-web/tests/browser-v1/expected.json`, `hosts/host-web/qualification/results.json`, and `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` unchanged.

No generic tooling/framework work, benchmark, listening run, matrix expansion, new corpus, algorithm/DSP change, expectation relaxation, or second automatic pin is in scope. Any gate or second-hash failure stops for a concrete new decision.

## Finite execution sequence and existing commands

Step 0 is a hard precondition: root assigns `QUALIFICATION_SOURCE_CHECKPOINT` only after the Luna HIGH correction is checkpointed, verifies that the frozen commit contains the #543 amendment plus the corrected native-runner dependency/helper/lockfile edge, records the exact commit, then numbers the successor and creates its dedicated worktree from that checkpoint. The probe in step 1 runs from that exact frozen worktree checkout. Steps 2-4 run from the isolated scratch copy whose sole recorded overlay is the provisional pin; this is necessary because the current builder deliberately deletes the built module when the source pin disagrees. All commands use separately empty `/tmp/issue555-*` output/target paths. Capture exact argv, cwd, frozen checkpoint identity, source/config identities, environment affecting Rust/Cargo, stdout, stderr and numeric status. Preserve both failures from required run `34108626421`.

1. Reproduce the frozen source hash without emitting or changing a pin:

   `env MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue555-repin-probe-empty`

   `/tmp/issue555-repin-probe-empty` must exist, be empty and non-symlink. Status must be 0 and stdout must be exactly the previously observed expected hash `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`. The directory remains empty by current builder design. A different reproduced digest stops execution for root/Sol assessment; it is neither automatically accepted nor authority to select a new candidate or pin.

2. Make an isolated scratch source copy of exact `QUALIFICATION_SOURCE_CHECKPOINT`, excluding `.git`, `target`, dependencies and prior outputs. Prove its file inventory/content equals the frozen checkpoint before overlay. Apply only the provisional pin overlay described above, record its exact diff, and build with the unchanged hermetic builder:

   `bash scripts/build-web-audioworklet.sh /tmp/issue555-qualified-artifact`

   Run this command from the scratch copy so the builder's existing path remapping remains authoritative. It must return 0, emit exactly six files, and the Wasm must independently hash to `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`.

3. Record a sorted name/byte-count/SHA-256 manifest for `/tmp/issue555-qualified-artifact` and compare every file against `/tmp/issue537-delivery/artifact`. The six expected names are exactly:

   - `miso-engine-v1-abi-layout.json`
   - `miso-engine-v1-audio-worklet-host.d.ts`
   - `miso-engine-v1-audio-worklet-host.js`
   - `miso-engine-v1-audio-worklet.js`
   - `miso-engine-v1-audio-worklet.simd128.wasm`
   - `miso-engine-v1-parameter-metadata.json`

   Do not require an unexplained file to be equal or different. Astra must classify every observed delta. A changed ABI/static/metadata file is not silently accepted merely because the Wasm pin is the reported failure.

4. Qualify the retained candidate artifact with existing gates only:

   - Native shared/static ABI: `env CARGO_TARGET_DIR=/tmp/issue555-native-target MISO_ENGINE_CAPI_LIBRARY=/tmp/issue555-native-target/release/libcapi.so MISO_ENGINE_CAPI_STATIC_LIBRARY=/tmp/issue555-native-target/release/libcapi.a bash scripts/check-capi-abi.sh .`
   - Shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource-budget gates: `bash scripts/check-web-audioworklet.sh /tmp/issue555-qualified-artifact`
   - Resource rows plus all current red controls and the three pinned PCM/native parity digests: `env CARGO_TARGET_DIR=/tmp/issue555-native-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/issue555-qualified-artifact`
   - Hermetic host/worklet policy and mutation tests: `env CARGO_TARGET_DIR=/tmp/issue555-hermetic-target bash scripts/test-web-audioworklet.sh`
   - Install the already-pinned browser dependencies: `npm --prefix hosts/host-web/qualification ci --ignore-scripts`
   - Actual browser attestation, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, lineage and mutation gates without recording or expanding the matrix: `npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/issue555-qualified-artifact --browser all --check-matrix --self-test-mutations`

   Every command must return 0. Zero selected/skipped browser execution is not a pass. Do not use `--record-matrix`; committed results and deployment matrix must remain byte-identical.

5. Astra MEDIUM performs a read-only adversarial review of the exact retained artifact, #537 baseline and frozen #543 source identity manifests, scratch-overlay proof, raw commands/streams/statuses and relevant #537/#543 records. Astra must issue explicit PASS before any repository pin edit. Hash novelty, successful compilation, or source-level claims alone are insufficient.

6. Only after Astra PASS and root authorization, Luna HIGH changes the one repository pin from `1bc18ab8cfb3e2a3e5a0ebeda185a64551e8398870abb2f3581077da0dfd3a3f` to `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`. Root checkpoints that exact-path tranche before further work.

7. On the unchanged accepted source plus approved pin, run the ordinary builder with no repin environment/bypass into a new empty directory:

   `bash scripts/build-web-audioworklet.sh /tmp/issue555-postpin-artifact`

   It must return 0. Its complete six-file manifest must byte-match `/tmp/issue555-qualified-artifact`; otherwise stop. Run `git diff --check` and the existing proportional policy/format checks selected by root, but do not repeat or widen DSP qualification.

8. Root pushes/synchronizes the successor evidence and pin, obtains required workflow `qualification` actual SUCCESS for the exact PR head/current base merge candidate, and only then resumes #543 delivery. A green local run, an in-progress workflow, or success on an earlier head is not delivery evidence.

## Acceptance gates

- Root explicitly assigns the exact post-correction `QUALIFICATION_SOURCE_CHECKPOINT` before execution and all qualification evidence traces to that immutable commit. No earlier candidate or moving ref qualifies.
- The repin probe and scratch-overlay ordinary builder independently reproduce the observed expected Wasm SHA-256 `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`; any different reproduced hash stops for assessment.
- The retained artifact has the exact six-file set and a complete manifest; every delta from the #537 current-engine baseline is explicitly classified.
- Existing native ABI, shipped static/object, resource, PCM/native parity, hermetic, and actual Chromium/Firefox/WebKit gates all return 0 with their built-in mutation controls.
- Astra MEDIUM finds the artifact delta consistent with accepted current-engine changes and no ABI, resource, PCM, browser, realtime/static, provenance or evidence regression.
- Only the approved pin line and issue/evidence bookkeeping change in the repository. No expected PCM/resource/browser result is repinned.
- The post-pin no-bypass build produces six files byte-identical to the prequalified artifact.
- Required `qualification` reports actual SUCCESS on the delivered exact head/base candidate.
- #543 remains the accepted Rust slice; #552 remains required to close CP-20.

## Root stop and bounded prequalification recovery

Root intentionally interrupted the first Luna HIGH worker (PID975263) and its verified inventory script after three inventory passes before any specified artifact build or qualification gate. This was a root throughput intervention, not an unexplained external host timeout. The worker exited1 without a terminal verdict. Raw launch records/streams, partial inventory outputs under /tmp/issue555-luna-a1, and exact root interruption record remain preserved; no partial inventory is PASS evidence. Repository product and pin bytes remain unchanged.

Sol MEDIUM selected a bounded replacement proof using Git blob identity and Git tracked executable/symlink semantics. Root authorizes one fresh actual Luna HIGH pre-pin invocation on the same frozen source and original finite gates, with a new capture prefix. It must preserve the stopped invocation, prove exact file identities without another general inventory framework, and stop on a failed source proof or specified qualification gate. No repository pin edit is authorized. The existing artifact/ABI/resource/PCM/browser gates and Astra MEDIUM-before-pin requirement remain unchanged. New issue/evidence bookkeeping must not be mistaken for product-source changes.

## Safe handoff: real probe succeeded; qualification unfinished

SolMEDIUM accepted independentrootGitidentityproof for all8439entries in /tmp/issue555-luna-a3/probe-source, productsourcee4f46fa8. ActualLunaHIGH then ran only the existing repinprobeonce. Status0 and stdout exactly6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c plusnewline were verifiedbyroot; /tmp/issue555-repin-probe-empty remains emptynonsymlinkdirectory. Rawprobe, actualargv, SolMEDIUMruling, identitytables and priorfailure/interruption records are losslesslypreserved in artifacts/issue555-handoff.

User requests a cleanstop. No candidateartifact was retained by probe design; no laterABI/resource/PCM/browser gate or Astraverification ran, and repositorypin is unchanged. Nextowner starts step2 from the verifiedfrozenexport, provisionalpin in isolatedscratchonly, then fulloriginalqualification and AstraMEDIUM-before-pin. Do not rerun sourceinventory/wrapper campaigns. Productfreeze is e4f46fa8; orchestrationHEAD advanced only for this evidence, so do not reuse staleexpectedHEAD2c865f18 as a productidentity check. On anotherhost, recreateexportfromexactproductcommit and obtain/reproduce approved537baselineartifact from its recordedproducer; /tmp paths are conveniences, not remotelyavailable artifacts. Alllatergates, postpinbyteidentity and exactCI remainmandatory.

## Root decision after retained artifact and native-ABI invocation stop

Luna HIGH retained the exact six-file artifact from frozen product source `e4f46fa808e413507d204e81b6a4ebc27254869c`; the ordinary builder returned zero and the Wasm independently reproduced `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`. The five non-Wasm files byte-match the approved #537 baseline, and the Wasm is the sole classified delta at ten additional bytes. Repository source and pin remain unchanged.

The first native ABI invocation returned one after successfully building both libraries under `/tmp/issue555-native-target/release`: `check-capi-abi.sh` honored `CARGO_TARGET_DIR` for Cargo but its default lookup remained `target/release/libcapi.so` and `target/release/libcapi.a`. This is an invocation mismatch, not an observed ABI, link, symbol, or header failure. The script already exposes `MISO_ENGINE_CAPI_LIBRARY` and `MISO_ENGINE_CAPI_STATIC_LIBRARY` as first-class overrides, so no checker or product change is justified.

Root preserves that failed invocation and amends only the native ABI command in step 4 to pass both actual isolated-target paths explicitly. The next Luna HIGH tranche runs that amended command once against the already-built frozen-source libraries. If it returns zero, continue with the remaining step-4 gates in their original order; do not rebuild the candidate artifact, rerun the probe, change the checker, or omit the preserved failure. Any further failure stops again for root assessment. Astra MEDIUM review and the no-pin-before-PASS rule remain unchanged.

## Root decision after wrong-working-directory browser stop

The amended native ABI gate, shipped Wasm static/object gate, resource/PCM/parity gate with 26 red controls, hermetic host/worklet mutations and pinned dependency install all returned zero. The final browser command returned one before browser launch because Luna HIGH ran step 4 from the repository qualification worktree, whose unchanged pin is the approved #537 digest. The runner correctly rejected the new candidate against that old lineage before Chromium, Firefox or WebKit started. This was a procedural working-directory error: the finite execution sequence above already requires steps 2–4 to run from the isolated frozen-source scratch copy carrying the sole provisional-pin overlay.

Root preserves every command and the zero-browser failure. Because the five passing commands inspected product/config bytes identical to the frozen source except for the irrelevant old repository pin, they remain valid supporting evidence and are not repeated. The next Luna HIGH tranche runs only the unchanged browser command once from `/tmp/issue555-source-e4f46fa8`, after proving that scratch tree still has the single provisional-pin overlay and that its remaining tracked product/config bytes match frozen source `e4f46fa808e413507d204e81b6a4ebc27254869c`. No inventory campaign or artifact rebuild is authorized. Any source proof or browser failure stops again. Astra MEDIUM review and the repository no-pin-before-PASS rule remain unchanged.

## Root decision after scratch-proof wrapper stop

The bounded scratch proof compared all 8,438 non-overlay tracked paths with frozen commit `e4f46fa808e413507d204e81b6a4ebc27254869c` and found zero missing and zero mismatched paths. The visible overlay pin bytes were the candidate digest plus a final newline. Luna HIGH's wrapper nevertheless returned one because its final shell assertion encoded a literal backslash-plus-`n` rather than a newline. The browser command did not run, and neither repository nor scratch bytes changed.

This was a progress-only verification wrapper error, not an implementation attempt or product/gate failure. Root preserves it, ends that agent's repeated qualification assignment, and reassigns the already-proved one-command browser run to fresh Luna XHIGH. The replacement first performs a simple direct byte comparison of the overlay pin to the 65 expected bytes, without regenerating the full path proof, then runs the unchanged browser command once from the scratch source. All earlier passing gates and failure evidence remain part of the record. Any further proof or browser failure stops for a new root decision; no pin edit or Astra review is authorized yet.

## Root decision after scratch dependency preflight stop

Fresh Luna XHIGH verified the scratch pin as the exact 64-character candidate digest plus LF and confirmed the committed 0/8,438 non-overlay mismatch proof. The browser command then ran once from the correct scratch source and returned one before browser launch because that scratch qualification directory had no installed `playwright` package. Chromium, Firefox, WebKit and the mutation controls all remained unexecuted. Repository and scratch tracked bytes did not change.

The earlier pinned `npm ci` success ran from the repository worktree and therefore did not satisfy the scratch execution precondition. Root preserves both records. The next bounded Luna XHIGH tranche runs the existing pinned dependency-install command once from `/tmp/issue555-source-e4f46fa8`, verifies that committed package manifests remain byte-identical, and, only if installation succeeds, runs the unchanged browser command once from that same scratch source. Creation of ignored `node_modules` content is the expected effect of the pinned installer and is not a product/config overlay. Any install, manifest-identity, or browser failure stops; no pin edit or Astra review is authorized yet.

## Root decision after checked-results lineage stop

Pinned dependency installation in the scratch tree returned zero and left `package.json` and `package-lock.json` byte-identical. The browser command then returned one before launch because `--check-matrix` compares the candidate artifact digest with `qualification/results.json`, which still carries the approved #537 digest. This check is independent of the already overlaid repository artifact pin. No browser or mutation ran, and no tracked byte changed.

The original single-overlay premise is therefore insufficient for the existing `--check-matrix` gate: keeping the old checked-results digest makes candidate qualification impossible, while recording new results before qualification would reverse the evidence order and is forbidden. Root authorizes a bounded scratch-only lineage overlay on two additional tracked files before the next browser invocation:

- in `hosts/host-web/qualification/results.json`, change only `candidateCommit` to frozen source `e4f46fa808e413507d204e81b6a4ebc27254869c` and `wasmSha256` to candidate `6452f0db237da1d57b3594e7d95dd53a089a604d5b0791ea8b3533c5930c5a1c`; every browser row, version floor, outcome, gate, resource and other field remains byte-identical;
- regenerate only the scratch `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` from that overlaid result using the repository's unchanged `generate-matrix.mjs`, and prove its diff is limited to the corresponding source/artifact lineage text.

These scratch files are comparison inputs, not repository repins or newly recorded qualification results. Preserve exact before/after diffs and semantic equality of all non-lineage JSON fields. Then run the unchanged `--check-matrix --self-test-mutations` browser command once from the scratch source. It must compare actual Chromium, Firefox and WebKit rows with the unchanged #537 browser expectations and exercise every mutation control. `--record-matrix` remains forbidden. Any overlay proof or browser failure stops; repository pin/results/matrix remain read-only until Astra MEDIUM reviews the completed candidate evidence.
