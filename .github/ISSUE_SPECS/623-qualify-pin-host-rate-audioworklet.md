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

After exact candidate and five-file identity are established, the scratch checkout may also change only `hosts/host-web/qualification/results.json`: set `candidateCommit` exactly to `ca5a8b492a41ba85b3e90d8dedb2b49b787f2f00` and `wasmSha256` exactly to `ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3`, then regenerate only `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Prove this overlay separately. Freeze all browser result rows, browser version floors, gate vocabulary, and resource values. These lineage changes are temporary qualification inputs, not repository edits.

## Qualification gates

On that exact six-file candidate and scratch source, run the unchanged repository gates once:

1. Shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource checks with `scripts/check-web-audioworklet.sh`.
2. Run `python3 -B scripts/check-browser-expected-resources.py --artifacts <candidate-directory>` as a separate resource gate. Require native-witness agreement and all 26 red mutations; the shipped-artifact gate does not subsume this check.
3. Hermetic host/worklet policy and mutation checks with an isolated `CARGO_TARGET_DIR` through `scripts/test-web-audioworklet.sh`.
4. Applicable SDK package/generated-surface checks, using locked installs only.
5. Install only locked browser qualification dependencies with `npm ci --ignore-scripts`.
6. Run exactly one Chromium, Firefox, and WebKit qualification against the candidate with matrix checking and self-test mutations. Preserve actual versions, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, lineage, resources, and mutation outcomes.
7. Verify the qualification output changes only candidate source/digest lineage. Browser outcome rows, version floors, gate vocabulary, and resource limits remain byte-equivalent except for generated lineage text.

Astra LOW must return scratch-candidate PASS over complete checksum-verified evidence before any repository pin or lineage edit.

## Conditional repository promotion and delivery

After candidate PASS, Luna HIGH/XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the approved candidate plus LF;
- `hosts/host-web/qualification/results.json` only to set `candidateCommit` to `ca5a8b492a41ba85b3e90d8dedb2b49b787f2f00` and `wasmSha256` to `ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator, yielding matching lineage text;
- this numbered spec/evidence as root-owned records.

No Rust, JS/TS, ABI, metadata, browser result row, version floor, resource expectation, dependency, lockfile, toolchain/config, script, policy, workflow, corpus, fixture, DSP, session, SDK surface, or #621 file may change.

Root checkpoints the exact promotion before further work. Run one ordinary no-bypass post-pin build from the clean pushed repository head and require exact six-file identity with the qualified scratch candidate. Run the proportional static gate; the separate expected-resource/native-witness gate with all 26 red mutations; hermetic/SDK, matrix, formatting/diff, workspace and effect-runtime gates; and do not repeat successful browser qualification. Astra LOW then reviews exact pushed head/current main, frozen-source ancestry, evidence checksums, exact three-file promotion, unchanged rows/resources, pin/lineage, and post-pin identity.

Open one PR only after exact-head/current-main Astra LOW PASS. Require the repository `qualification` check, verify live main immediately before guarded exact-head merge, verify merge parents and post-main qualification, synchronize and close #622/#623, update #559/#560, and remove the clean delivered #622/#623 worktrees while retaining branches/history/evidence.

One scratch qualification and one Luna promotion attempt are initially authorized after scope and candidate PASS respectively. A candidate mismatch or substantive gate failure stops for reviewed rescope; do not retry builds, browsers, or timed work to obtain a green result. The repository three-attempt limit remains binding.

## Astra LOW initial scope review — FAIL

Astra LOW returned **FAIL** at exact clean pushed head
`3b6b1f9b28101e7d4bd851f387ca3fc3e6130478`, live main
`cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, and synchronized tracker
`c0c7ead3a3a7c3fe62d8be9d360d7799a3af764b`. All 19 parent probe checksums, status 1, candidate
digest, zero-output census, ancestry, compiled-input identity, one-build design, promotion boundary,
delivery controls, and #621 disjointness passed. The brief omitted the independent browser expected-
resources/native-witness gate with its 26 red mutations and did not fix the two scratch/promoted
lineage fields to exact values. Those requirements are now explicit above, including post-pin
resource verification. No scratch qualification or repository promotion was authorized by this
verdict.

## Astra LOW corrected scope review — packaging FAIL

Astra LOW confirmed both substantive corrections at exact clean pushed head
`2103c0e990517f9332792e3febe2bf3b22552c2c`, live main
`cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, and synchronized tracker
`51a0ad8b6540ce979ab997121334f92a8a569467`. Local/GitHub coordination, exact lineage values, the
separate resource/native-witness gate with 26 mutations, post-pin repetition, and #621
disjointness passed. Authorization remained blocked because branch-wide `git diff --check` treated
intentional whitespace in the raw inherited `source-main.diff` capture as patch errors.

Root losslessly compressed only that capture with deterministic gzip. It decompresses byte-for-byte
to SHA-256 `af126a437c16e903e78219a255cfe1a26698155f3e10d424ec7255db010eaa8b`; the README, separate raw-
identity record, and evidence manifest are refreshed. No qualification input, raw evidence content,
source, artifact, pin, or lineage changed. Corrected-head Astra LOW confirmation remains required
before any scratch execution.

## Astra LOW corrected scope review — PASS

Astra LOW returned **PASS** at exact clean pushed head
`f9c4e92dca0a5d9232ebd15e169170f74a995f65`, live main
`cf9e079cd5ef80d1c7284e9edd0ffcc90b0db335`, and synchronized tracker
`a078948a6b27ed108c48d3075078b9860e77ded6`. Deterministic gzip reproduces the original raw capture
at SHA-256 `af126a437c16e903e78219a255cfe1a26698155f3e10d424ec7255db010eaa8b`; all 20 parent probe
manifest entries verify; branch diff hygiene passes; local/GitHub #559/#560/#622/#623 bodies match;
and all substantive scope corrections remain satisfied. Exactly one frozen-source scratch
qualification sequence is authorized. It must stop without retry on any failure. Repository
promotion remains unauthorized until Astra LOW returns candidate PASS.
