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

## Attempt 1 qualification evidence

Astra xhigh accepted #880 source `bb9efd095bd8d70be8b8503cba6c3617fd51b74e` with PASS. Qualification used that value as `candidateCommit`; later changes before and during qualification were evidence, the provisional artifact pin, generated records, and this issue spec. No engine, host, SDK, fixture, or gate source changed. The report-mode run was performed at `c80cd2dd`; the ordinary build and gates used the tree after provisional-pin checkpoint `4bd4d666`, whose only qualification input change was the live artifact hash.

Report mode ran once as `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/issue905-attempt1-probe`, exit 0. Its stdout was exactly `25e75763f1e6ea815a938de60367549c973cf3c5dc551a3a3af5d5ac5b79e20a`; the probe directory remained empty. Root checkpointed the provisional live pin before the ordinary build. The unchanged ordinary builder then ran once as `bash scripts/build-web-audioworklet.sh /tmp/issue905-attempt1-artifact`, exit 0. It produced exactly seven regular files, with no symlinks or extra entries:

| Artifact | SHA-256 |
| --- | --- |
| `miso-engine-v1-abi-layout.json` | `3e6d667fa64e3b2930b2205e38b1fdbcc14c590d3abfa22020c71b554133fc70` |
| `miso-engine-v1-audio-worklet-host.d.ts` | `01b80480c83f46afcbc1a210c5657cb45635933bef0c513087c11fd4275a8c21` |
| `miso-engine-v1-audio-worklet-host.js` | `1c617cda500f034b7977f4ae351214276f32fdd3350cda1626caa8fcc55dccfe` |
| `miso-engine-v1-audio-worklet.js` | `d45b975476716559f475eb24e195616c2b93204f9d25fcad62b5064d26c0803b` |
| `miso-engine-v1-audio-worklet.simd128.wasm` | `25e75763f1e6ea815a938de60367549c973cf3c5dc551a3a3af5d5ac5b79e20a` |
| `miso-engine-v1-parameter-metadata.json` | `f7fb112328d64daf38d29508cbc360d4113771e4b575b8e25ab7ada0187f10b4` |
| `prepared-control.js` | `3f20d99da2fa4e0b461f63cef01050264f67ecebb82495ad04b7318bf9f5dca1` |

All six non-Wasm files compare byte-for-byte with both their current authorities and the same paths at accepted source: the four checked-in worklet/host files and the two SDK-generated JSON assets. The `REAL_WASM_SHA256` receiver expectation was moved to the candidate hash; the historical npm release identity remains unchanged. Numeric/resource fixtures, browser floors, toolchain, dependencies, flags and CI routing are unchanged.

Every existing artifact gate passed once against that same artifact: `scripts/check-web-audioworklet.sh`; `scripts/check-browser-expected-resources.py` including its 26 red mutations and native witness; `scripts/test-web-audioworklet.sh`; the explicit `--real-wasm-receiver` lifecycle gate; SDK types; SDK headless (284/284); and SDK package (11/11 CLI tests, package smoke, and artifact-builder contract). The receiver confirmed ordinary boot/status/dispose/repeated-dispose, corrupted-Wasm refusal before module/node construction, and the disposal mutant with fallback cleanup. The browser SDK root was staged from the accepted commit and built against the same artifact outside the repository.

Browser qualification ran once with `npm run qualify -- --artifacts /tmp/issue905-attempt1-artifact --sdk-root /tmp/issue905-attempt1-source/sdk --browser all --record-matrix --candidate-commit bb9efd095bd8d70be8b8503cba6c3617fd51b74e --self-test-mutations`, exit 0. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 each passed all seven recorded gates, including SDK response. The runner recorded `results.json`; `npm run matrix -- --check` passed. Against the provisional-pin checkpoint, the result object and generated matrix differ only in `candidateCommit` and `wasmSha256`; each browser row retains every gate as `pass`.

Tool identities were rustc/cargo 1.97.1, Node 22.23.2, npm 10.9.8, wasm-objdump 1.0.34 and Playwright 1.62.1, using the installed Chromium 1234, Firefox 1538 and WebKit 2336 revisions. Exact commands, raw stdout/stderr and exit records, the toolchain identities, authority comparisons and browser record audit are preserved outside the worktree under `/tmp/issue905-attempt1-logs/`; the compiled artifact is under `/tmp/issue905-attempt1-artifact/`. No compiled artifact or `node_modules` was added to Git.

## Independent verdict

Astra xhigh recorded **attempt-1 PASS** with no blocking findings in
`docs/issue905-astra-review.md`. It independently checked the seven-file identity,
accepted-source authorities and SDK provenance, unchanged numerical/resource gates
and historical release identity, authentic successful logs, and lineage-only browser
record changes. No costly successful gate or timed workload was rerun for review.
Exact-head required CI, merged delivery and GitHub synchronization remain before closure.
