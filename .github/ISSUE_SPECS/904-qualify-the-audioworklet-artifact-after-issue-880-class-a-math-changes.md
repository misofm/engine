## Problem and smallest closable slice

PR #903's source batch for #880 earned Astra xhigh PASS at `b3910b37033d0fb30b98405140d03d56fccc3f49`. Qualification run 36084071070, shipped artifact job 107912071246, builds successfully but rejects the previous AudioWorklet pin:

- previous: `e18acf9ca97af137a1917e52481c4bf962943d6d755369387969f84c3e381106`
- observed CI candidate: `772b65111a22fa07135dd3f90628774a5587ef6891e14da25f28a2989d3d4d56`

The class-A source changes code generation while preserving numeric corpus bits. A compiled-artifact hash is distinct from the frozen numeric digests; neither corpus identity nor a hash change alone qualifies the new browser artifact. This successor owns rebuilding and qualifying the exact accepted source through existing gates, then updating only the live artifact identity and generated qualification lineage needed to deliver #880. It does not implement any pending R1–R4 change.

## Authorized scope

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`
- `hosts/host-web/qualification/results.json` and generated `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
- The live real-Wasm expected hash in `scripts/test-web-audioworklet.mjs`, only if its gate requires the new artifact identity.
- This local spec and concise #880 evidence/decision records.

Do not change DSP/session/host/SDK runtime, numerical PCM/corpus pins, tolerances, toolchain/dependencies, build flags, browser version floors, resource expectations, CI routing, or historical published-release evidence. In particular the npm publish workflow/test pin describes the already published SDK release and is not silently repinned here. No new framework, timed benchmark, registry publication, or app deployment.

## Execution and objective gates

Use the existing artifact workflow described in #766, adapted to the current seven-file artifact. Record exact accepted source commit, tool versions, commands and authentic output. Keep source fixed. Use fresh external output/log directories; never commit compiled artifacts/node_modules.

1. Run the unchanged builder in supported `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1` report mode. Require candidate agreement with the CI value above, or stop for attribution. Root checkpoints the provisional source artifact pin before more implementation.
2. Run the unchanged ordinary builder against that pin to a fresh empty directory. Require success and exactly the seven ordinary files; verify all six non-Wasm outputs against their unchanged source/generated authorities.
3. Run existing static AudioWorklet, expected-resource/native-witness, hermetic/real-Wasm receiver, SDK type/headless/package gates on the same artifact. No gate weakening or PCM fixture re-pin.
4. Run Chromium/Firefox/WebKit qualification with `--record-matrix --candidate-commit <accepted-source> --self-test-mutations`, using the pinned installed browser toolchain. Require all existing numeric/control/resource/mutation checks. Generate/check the matrix; explain any lineage-only differences honestly rather than rewriting gate results.
5. Audit allowed paths and unchanged numeric pins. Any newly discovered source or unrelated tooling defect stops this bounded slice for attribution; do not absorb it automatically.

Luna xhigh implements/qualifies, then Astra xhigh independently reviews. Maximum two attempts, each one coherent pass and one adversarial verdict. Preserve failures; do not weaken gates. Existing source review remains valid unless source changes. No expensive successful gate is rerun for evidence-only commits.

## Delivery

Append the qualification to PR #903, retain #880 open for pending owner rulings. After the successor earns PASS, push one coherent checkpoint, require exact-head qualification green, merge and verify required main qualification. Synchronize both issue bodies/states, close this successor only when delivered, and clean completed worktrees while retaining the active #880 batch for pending owner decisions as needed.

## Attempt 1 qualification evidence

Accepted source was `b3910b37033d0fb30b98405140d03d56fccc3f49`. The only difference from that accepted source before this work was this issue spec; no implementation source was edited. The provisional artifact pin was checkpointed at `48326abb307aebc692eb744a3cecac40d38a8914` before ordinary builds and gates. Builder identity was rustc 1.97.1, cargo 1.97.1, Node 22.23.2, npm 10.9.8, and wasm-objdump 1.0.34. The existing Playwright dependency is 1.62.1; its installed pinned browser revisions were Chromium `chromium-1234`, Firefox `firefox-1538`, and WebKit `webkit-2336`. Browser runtime versions observed were Chromium `151.0.7922.34`, Firefox `153.0`, and WebKit `26.5`.

Report mode ran once with `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue904-attempt1-probe`, exit 0. It emitted exactly one lowercase 64-character hash, `772b65111a22fa07135dd3f90628774a5587ef6891e14da25f28a2989d3d4d56`, matching the PR #903 CI candidate; the probe directory stayed empty. After pin checkpoint `48326abb`, the unchanged ordinary builder ran once with the report variable unset into the fresh `/tmp/issue904-attempt1-artifact`, exit 0. It produced exactly these seven files; all six non-Wasm files compared byte-for-byte with their existing authorities, and the built Wasm matched the pin:

| Artifact | SHA-256 |
| --- | --- |
| `miso-engine-v1-abi-layout.json` | `3e6d667fa64e3b2930b2205e38b1fdbcc14c590d3abfa22020c71b554133fc70` |
| `miso-engine-v1-audio-worklet-host.d.ts` | `01b80480c83f46afcbc1a210c5657cb45635933bef0c513087c11fd4275a8c21` |
| `miso-engine-v1-audio-worklet-host.js` | `1c617cda500f034b7977f4ae351214276f32fdd3350cda1626caa8fcc55dccfe` |
| `miso-engine-v1-audio-worklet.js` | `d45b975476716559f475eb24e195616c2b93204f9d25fcad62b5064d26c0803b` |
| `miso-engine-v1-audio-worklet.simd128.wasm` | `772b65111a22fa07135dd3f90628774a5587ef6891e14da25f28a2989d3d4d56` |
| `miso-engine-v1-parameter-metadata.json` | `f7fb112328d64daf38d29508cbc360d4113771e4b575b8e25ab7ada0187f10b4` |
| `prepared-control.js` | `3f20d99da2fa4e0b461f63cef01050264f67ecebb82495ad04b7318bf9f5dca1` |

Using that same immutable artifact, all existing checks exited 0: `scripts/check-web-audioworklet.sh`; `scripts/check-browser-expected-resources.py --artifacts …` (including all 26 red mutations); `scripts/test-web-audioworklet.sh`; SDK types; SDK headless (284/284 tests); SDK package (11/11 tests); generated matrix `--check`; and the explicit real-Wasm receiver lifecycle gate described below. The browser qualification with `--sdk-root /tmp/miso-engine-880-ma1/sdk --browser all --record-matrix --candidate-commit b3910b37033d0fb30b98405140d03d56fccc3f49 --self-test-mutations` exited 0 in all three browsers. Attestation, AudioWorklet boot, native corpus digest, control path, observation, main-thread stall, and SDK response were recorded as `pass` in every row. One earlier browser invocation omitted the optional SDK root and consequently did not record `sdkResponse`; it is retained in the external logs as incomplete evidence and is superseded by the full SDK-root invocation. No source or artifact changed between those browser runs.

The separate live real-Wasm receiver gate ran as `node scripts/test-web-audioworklet.mjs --real-wasm-receiver --artifacts /tmp/issue904-attempt1-artifact`, exit 0. Its preflight accepted the exact seven-file artifact and new live hash; ordinary boot/status/dispose/repeated-dispose, corrupted-Wasm refusal before module/node construction, and the named disposal-mutant assertion with fallback cleanup all passed. The `scripts/test-web-audioworklet.sh` invocation without the receiver flag exercises the hermetic fake-Wasm lane and does not call this real receiver branch. Therefore only `REAL_WASM_SHA256` in `scripts/test-web-audioworklet.mjs` was moved to the qualified live candidate hash. The npm published-SDK release pin remains unchanged and its SDK package check passed.

`results.json` was recorded by the browser runner, not hand-edited. Its only changes are the accepted candidate commit and Wasm SHA; the generated matrix changes only those same lineage values. All three browser rows retain `sdkResponse: pass`. The frozen numeric/resource expectations in `hosts/host-web/tests/browser-v1/expected.json` and all native corpus/source fixtures are unchanged from the accepted source; browser native-corpus digests and expected resources passed. No numeric PCM pin, tolerance, browser floor, build flag, tool/dependency lock, or CI file changed. The final allowed implementation paths are the source artifact hash pin, `qualification/results.json`, generated `BROWSER_DEPLOYMENT_MATRIX.md`, this real-Wasm gate's expected hash, and this spec. `git diff --check` passes. Full stdout/stderr/exit/invocation records, the exact artifact hash list and authority comparisons are preserved outside the worktree under `/tmp/issue904-attempt1-logs/`; generated artifacts remain under `/tmp/issue904-attempt1-artifact`, and installed dependencies are ignored `node_modules` directories. No compiled artifact was added to the repository.
