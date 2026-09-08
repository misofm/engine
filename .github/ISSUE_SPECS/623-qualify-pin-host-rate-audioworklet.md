# Qualify and pin the host launch-rate AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/623

Parent: #622 (lane B, IO21). Coordination: #559/#560. Concurrent disjoint issue: #621.

The source-accepted #622 branch at frozen checkpoint `ca5a8b49` changes `host-core`, which is compiled into `host-web`. Its one ordinary no-bypass build completed compilation and correctly stopped before publishing because candidate simd128 Wasm SHA-256 `ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3` differs from delivered pin `f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`. The complete checksum-verified failed probe is preserved under `artifacts/issue622-artifact-probe/` at parent head `5276932b`.

This is the bounded artifact-promotion successor required by #622. #622 is a passive source dependency while #623 occupies lane B's active slot. Lane-A #621 owns only true-peak limiter source/tests/evidence; it must request a later lane-B artifact decision if its accepted source changes the six-file output. #621/#623 fill the two active issue slots with no overlapping source, evidence, worktree, or artifact ownership.

Sol HIGH coordinates and owns artifact qualification/pinning decisions, checkpoints, GitHub synchronization, and delivery. Astra LOW performs every scope, scratch-candidate, promotion, post-pin, exact-head, and delivery verification. Luna HIGH or XHIGH performs only the conditionally authorized repository promotion. No benchmark or timing workload is allowed.

## Frozen source and scratch qualification

The artifact source is exactly `ca5a8b49`; accepted production source is `fece7a2c`. Later issue/evidence commits do not change compiled inputs. Before qualification, verify the frozen source is an ancestor of the clean pushed successor head, current main is still the recorded merge-base or document and review any integration, every probe checksum passes, and these inputs match the failed probe:

- `scripts/build-web-audioworklet.sh`
- `Cargo.lock`, root `Cargo.toml`, `rust-toolchain.toml`, and `.cargo/config.toml`
- the delivered web source/pin files
- the four accepted #622 source/test identities

Create one isolated scratch checkout from the frozen source. It may first differ only at `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, provisionally set to the observed candidate plus LF. Prove that overlay and run the unchanged ordinary builder exactly once to an empty external directory. It must emit exactly six files and the Wasm must hash to the frozen candidate. A different digest, extra output, build failure, or any other scratch source change is FAIL and stops this issue before repository edits.

Compare the five non-Wasm files byte-for-byte with the delivered #619/PR #620 files and record all six hashes. Preserve exact argv, cwd, source/toolchain/config/input hashes, overlay diff, full streams/status, output census, comparison, and checksum manifest. Any non-Wasm difference stops for rescope.

After exact candidate and five-file identity are established, the scratch checkout may also change only `hosts/host-web/qualification/results.json` fields `candidateCommit` and `wasmSha256`, then regenerate only `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Prove this overlay separately. Freeze all browser result rows, browser version floors, gate vocabulary, and resource values. These lineage changes are temporary qualification inputs, not repository edits.

## Qualification gates

On that exact six-file candidate and scratch source, run the unchanged repository gates once:

1. Shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource checks with `scripts/check-web-audioworklet.sh`.
2. Hermetic host/worklet policy and mutation checks with an isolated `CARGO_TARGET_DIR` through `scripts/test-web-audioworklet.sh`.
3. Applicable SDK package/generated-surface checks, using locked installs only.
4. Install only locked browser qualification dependencies with `npm ci --ignore-scripts`.
5. Run exactly one Chromium, Firefox, and WebKit qualification against the candidate with matrix checking and self-test mutations. Preserve actual versions, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, lineage, resources, and mutation outcomes.
6. Verify the qualification output changes only candidate source/digest lineage. Browser outcome rows, version floors, gate vocabulary, and resource limits remain byte-equivalent except for generated lineage text.

Astra LOW must return scratch-candidate PASS over complete checksum-verified evidence before any repository pin or lineage edit.

## Conditional repository promotion and delivery

After candidate PASS, Luna HIGH/XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the approved candidate plus LF;
- `hosts/host-web/qualification/results.json` only for `candidateCommit` and `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator, yielding matching lineage text;
- this numbered spec/evidence as root-owned records.

No Rust, JS/TS, ABI, metadata, browser result row, version floor, resource expectation, dependency, lockfile, toolchain/config, script, policy, workflow, corpus, fixture, DSP, session, SDK surface, or #621 file may change.

Root checkpoints the exact promotion before further work. Run one ordinary no-bypass post-pin build from the clean pushed repository head and require exact six-file identity with the qualified scratch candidate. Run the proportional static/resource/hermetic/SDK, matrix, formatting/diff, workspace and effect-runtime gates without repeating successful browser qualification. Astra LOW then reviews exact pushed head/current main, frozen-source ancestry, evidence checksums, exact three-file promotion, unchanged rows/resources, pin/lineage, and post-pin identity.

Open one PR only after exact-head/current-main Astra LOW PASS. Require the repository `qualification` check, verify live main immediately before guarded exact-head merge, verify merge parents and post-main qualification, synchronize and close #622/#623, update #559/#560, and remove the clean delivered #622/#623 worktrees while retaining branches/history/evidence.

One scratch qualification and one Luna promotion attempt are initially authorized after scope and candidate PASS respectively. A candidate mismatch or substantive gate failure stops for reviewed rescope; do not retry builds, browsers, or timed work to obtain a green result. The repository three-attempt limit remains binding.
