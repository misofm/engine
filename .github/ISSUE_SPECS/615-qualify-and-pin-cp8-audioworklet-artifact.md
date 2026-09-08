# Qualify and pin the CP8 AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/615

Parent: #614. Coordination: #560. Passive dependent: #539.

The source-accepted CP8 branch at frozen checkpoint `0c715de9fbdf0b3873c707e10c52095fad750287` changes code compiled into `host-web`. Its one ordinary no-bypass build completed compilation and correctly stopped before publishing because observed simd128 Wasm SHA-256 `e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b` differs from the delivered pin `39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55`. The complete failed-probe record is preserved under `artifacts/issue614-artifact-probe/` at parent head `d47a8ededad4a6c4549b35a49ac3d134c97744a0`.

This issue is the bounded artifact-promotion successor required by #614's anticipated drift rule. #614 and this issue occupy the two active slots. #539 remains open as a passive dependent: its separate limiter candidate `7e242eb8f283bcba2ef5778cd430950fee7df92ef66b6ddaaf2a0a68e5bc7409` is not qualified here, and #539 must integrate the delivered CP8/main state and make its own later artifact decision.

Sol HIGH coordinates, owns artifact qualification/pinning decisions, checkpoints, GitHub synchronization and delivery. Astra LOW performs every scope, scratch-candidate, source/pin, exact-head and delivery verification. Luna HIGH or XHIGH performs any authorized repository edits. No benchmark or timing workload is allowed.

## Frozen source and scratch qualification

The artifact source is exactly `0c715de9fbdf0b3873c707e10c52095fad750287`; later issue/evidence commits do not change compiled inputs. Before qualification, verify the source commit is an ancestor of the clean pushed successor head, current main is still the recorded merge-base or document and review any integration, and these inputs match the failed probe:

- `scripts/build-web-audioworklet.sh`
- `Cargo.lock`, root `Cargo.toml`, `rust-toolchain.toml`, and `.cargo/config.toml`
- the four shipped web source/pin files
- the CP8 `effect-contract` and `effect-package` source identities

Create one isolated scratch checkout from the frozen source. The scratch checkout may differ only at `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, provisionally set to the observed candidate digest plus LF. Prove that one-file overlay and run the unchanged ordinary builder once to an empty external directory. It must emit exactly six files and the Wasm must hash to the frozen candidate. A different digest, extra output, build failure, or any other scratch source change is FAIL and stops this issue before repository edits.

Preserve exact argv, cwd, source/toolchain/config/input hashes, overlay diff, full streams/status, output census, six-file hashes and checksum manifest. Compare the five non-Wasm outputs byte-for-byte to the delivered #587/#608 manifest; record any mismatch and stop for rescope.

## Qualification gates

On that exact six-file candidate and scratch source, run the repository's existing gates without changing them:

1. Shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource checks with `scripts/check-web-audioworklet.sh`.
2. Hermetic host/worklet policy and mutation checks with an isolated `CARGO_TARGET_DIR` through `scripts/test-web-audioworklet.sh`.
3. Existing SDK package/generated-surface checks that apply to the six-file artifact.
4. Install only the locked browser qualification dependencies with `npm ci --ignore-scripts`.
5. Run Chromium, Firefox and WebKit qualification against the candidate with matrix checking and self-test mutations. Preserve actual browser versions, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, lineage, resource and mutation results.
6. Verify the qualification output changes only the candidate source/digest lineage expected for this candidate. Browser outcome rows, version floors, gate vocabulary and resource limits must remain unchanged unless a real failure stops the issue.

Astra LOW must return candidate-qualification PASS over the complete retained evidence before any repository pin or consumer-lineage edit.

## Conditional repository edits and delivery

After candidate PASS, Luna HIGH/XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the approved candidate digest plus LF;
- `hosts/host-web/qualification/results.json` only for `candidateCommit` and `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator, yielding matching lineage text;
- this numbered spec/evidence as root-owned records.

No Rust, JS/TS, ABI, metadata, browser result row, version floor, resource expectation, dependency, lockfile, toolchain/config, script, policy, workflow, corpus, fixture, DSP, session, SDK surface or #539 file may change.

Root checkpoints the exact edit before further work. Run one ordinary no-bypass post-pin build from the clean repository head and require exact six-file identity with the qualified scratch candidate. Re-run the static/resource/hermetic/SDK gates and proportional formatting/diff/policy checks. Astra LOW then reviews exact pushed head/current main, source ancestry, evidence checksums, three-file edit, generated lineage, pin and post-pin identity.

Open one PR only after that PASS. Require the repository's `qualification` check, verify live main immediately before exact-head merge, verify merge parents and post-main qualification, then synchronize and close this issue and #614. Update #560 and #559 so #539 can resume, and remove clean delivered worktrees while retaining branches/history.

One scratch qualification and one Luna repository-edit attempt are authorized. A candidate mismatch or substantive gate failure stops for a reviewed rescope; do not retry browser or build workloads to obtain a green result. The repository three-attempt limit remains binding.
