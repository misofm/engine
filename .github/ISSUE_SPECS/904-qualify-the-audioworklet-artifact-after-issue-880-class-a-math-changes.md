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
