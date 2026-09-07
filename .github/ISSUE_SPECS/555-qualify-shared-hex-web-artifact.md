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

   - Native shared/static ABI: `env CARGO_TARGET_DIR=/tmp/issue555-native-target bash scripts/check-capi-abi.sh .`
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
