## Problem and smallest closable slice

Issue #880's approved class-B source changes (L3 lane logarithm, transient-shaper fast dB tier and compressor coefficient-domain ramps) can change the compiled AudioWorklet identity. The current live pin qualifies the earlier class-A source, not the new batch. This bounded successor rebuilds and qualifies the final Astra-accepted #880 source through the existing gates, then updates only the live artifact identity and generated qualification lineage needed for delivery.

## Authorized scope

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`
- `hosts/host-web/qualification/results.json` and generated `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`
- The live `REAL_WASM_SHA256` in `scripts/test-web-audioworklet.mjs`, if the artifact identity changes.
- This spec, a concise review/evidence document, and #880's delivery record.

No DSP, runtime, SDK, numerical/resource expectations, tolerances, toolchain, dependencies, build flags, browser floors, CI routing or historical published-release evidence changes. No new framework, timed benchmark, registry publication or deployment. If current artifact identity is unchanged, preserve the live pin and qualify/report that fact.

## Execution and gates

Use the existing workflow from #904 with fresh external artifact/log directories. Final source must first receive Astra xhigh PASS; record that exact source commit and keep implementation source fixed during qualification.

1. Run the unchanged builder in `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1` report mode; record the candidate hash. Root checkpoints any provisional live-pin change before further work.
2. Run the ordinary builder against that pin into a fresh directory. Require exactly the seven ordinary files, and compare all six non-Wasm outputs with their unchanged authorities.
3. Run existing static AudioWorklet, expected-resource/native-witness, hermetic, explicit real-Wasm receiver, SDK type/headless/package gates on the same artifact. Preserve every numeric/resource gate.
4. Run Chromium, Firefox and WebKit qualification with `--sdk-root <accepted-source-sdk> --record-matrix --candidate-commit <accepted-source> --self-test-mutations`. All numeric, control, resource, SDK-response and mutation checks must pass. Generate/check the matrix. Do not manually edit qualification results.
5. Audit allowed paths, unchanged source and numeric pins, and retained historical npm-release identity. Preserve authentic logs externally and concise provenance/results in this spec. No compiled artifacts or node_modules in Git.

Luna xhigh implements/qualifies, then Astra xhigh independently reviews. Maximum two coherent attempts with one verdict each. Newly discovered unrelated source/tooling defects stop this bounded slice for attribution. Do not rerun expensive successful gates merely for evidence-only commits.

## Delivery

Include this successor in #880's coherent class-B batch. After PASS, push once, require exact-head qualification green, merge, and verify required main qualification. Synchronize #880, #902 and this successor; close only after the evidence is upstream and their required work is complete. Clean completed worktrees only when all checkpoints are pushed and evidence is preserved.
