# Qualify and pin the limiter detector AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/627

Parent: #621 (lane A, FX2). Coordination: #559/#560. Predecessor: delivered #625/PR #626.

This is the separately numbered lane-B artifact successor required by #621. The accepted limiter access change is frozen at `crates/true-peak-limiter/src/lib.rs` SHA-256 `32ab4abf975b32d47c85a748e617e74c9547b22e1b585f0d36713be439a62908`. #621 integrated delivered main `30680709c58f0be99e09d006d8d661c1ce96324d` and passed Astra LOW review at merge checkpoint `77e9c7d7536916f045a8aa9c66aea86b5c2fe0c2`; its synchronized record head is `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`. The delivered AudioWorklet Wasm pin is `ac71c64033b0cfc637cf14edcacaa6ed1b3bbf7093a5caa641ef84adea7e88e3`, attributed to #623 source `ca5a8b492a41ba85b3e90d8dedb2b49b787f2f00`.

#621 remains the one launch-critical implementation issue. This successor occupies the second issue slot and exclusively owns the artifact applicability decision, qualification, pin, lineage, and evidence. Sol HIGH coordinates and owns checkpoints, artifact qualification/pinning decisions, GitHub synchronization, and delivery. Astra LOW performs every scope, candidate, promotion, post-pin, exact-head, and delivery review. Luna HIGH or XHIGH may perform only a conditionally authorized repository promotion. No compiler capture, benchmark, timing workload, DSP edit, or new `.ll` artifact is allowed.

## Smallest closable outcome

At a clean isolated checkout of exact frozen source `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251`, run one ordinary no-bypass AudioWorklet build attempt into an empty external directory and preserve its complete status, streams, source/toolchain/config/input identities, output census, and hashes. The build must use the unchanged `scripts/build-web-audioworklet.sh`; do not set the repin bypass.

If the existing pin accepts the build, require exactly six emitted files and byte-for-byte identity with the six delivered #623 files. That proves artifact non-applicability: record Astra LOW PASS, make no pin or lineage change, and release this successor for closure.

If the builder rejects only because the observed Wasm digest differs from the delivered pin, preserve the failed probe with zero published output and the observed digest. Astra LOW must independently verify the probe, compiled inputs, frozen source, and mismatch before authorizing one scratch qualification. Any build failure with another cause, missing digest, extra output, dirty source, or changed build input is FAIL and stops for reviewed rescope. Do not retry the probe.

## Conditional scratch qualification

After Astra LOW mismatch PASS, create one detached scratch checkout from exact frozen source. It may first differ only by setting `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the verified candidate digest plus LF. Prove that overlay, run the ordinary builder exactly once into a new empty external directory, require exactly six files, require the candidate Wasm digest, and compare all five non-Wasm files byte-for-byte with the delivered #623 artifact. Any difference outside Wasm stops for rescope.

The scratch checkout may then change only `hosts/host-web/qualification/results.json` to set `candidateCommit` exactly to `dc14ca856e10cb5ad7f6fdacb0ea9322342a9251` and `wasmSha256` exactly to the verified candidate digest, then regenerate only `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Browser rows, browser version floors, gate vocabulary, resource values, ABI, SDK surface, and all other result fields remain frozen.

On that exact six-file candidate and scratch source, run each unchanged gate once:

1. shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary/resource checks;
2. the separate expected-resource/native-witness gate and its full existing red-mutation set;
3. hermetic host/worklet policy and mutation checks with an isolated Cargo target directory;
4. applicable SDK package and generated-surface checks using locked installs only;
5. locked qualification dependency installation;
6. exactly one Chromium, Firefox, and WebKit qualification, including matrix self-tests, AudioWorklet boot/control/observation/stall, native-corpus PCM identity, resources, and lineage;
7. proof that qualification changes only the authorized candidate source/digest lineage while the frozen browser outcomes, versions, gates, and resources remain identical.

Preserve compact checksum-verified evidence. Full compiler IR/assembly captures and archives whose purpose is retaining them are forbidden. Astra LOW must return scratch-candidate PASS before any repository pin or lineage edit.

## Conditional promotion and delivery

After scratch-candidate PASS, Luna HIGH/XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the approved digest plus LF;
- `hosts/host-web/qualification/results.json` only at `candidateCommit` and `wasmSha256` with the exact approved values;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator, yielding matching lineage text;
- this numbered spec and bounded evidence under this issue.

No Rust, JS/TS, ABI, metadata, browser result row, version floor, resource expectation, dependency, lockfile, toolchain/config, build/check script, policy, workflow, corpus, fixture, DSP, session, SDK surface, #621 source/test/evidence, or historical artifact evidence may change.

Root checkpoints the exact promotion before further work. Run one ordinary no-bypass post-pin build from the clean pushed repository head and require exact six-file identity with the qualified scratch candidate. Run the proportional static gate, expected-resource/native-witness mutation gate, hermetic/SDK checks, matrix generation/check, formatting/diff, workspace and effect-runtime policies once; do not repeat successful browser qualification. Astra LOW then reviews the exact pushed head against current main, frozen-source ancestry, evidence checksums, exact promotion scope, unchanged rows/resources, pin/lineage, and post-pin identity.

Open one PR only after Astra LOW exact-head/current-main PASS. Require repository `qualification`, verify live main immediately before a guarded exact-head merge, verify exact merge parents and post-main qualification, synchronize and close this successor and #621, update #559/#560, and remove clean delivered and detached worktrees while retaining branches/history/evidence.

One applicability probe and, conditionally, one scratch qualification plus one Luna promotion attempt are initially authorized only after their preceding Astra LOW PASS. A candidate mismatch beyond the expected Wasm digest, substantive gate failure, or tooling defect stops for reviewed rescope; do not rerun builds or browsers to obtain a green result. The repository three-attempt rule remains binding.
