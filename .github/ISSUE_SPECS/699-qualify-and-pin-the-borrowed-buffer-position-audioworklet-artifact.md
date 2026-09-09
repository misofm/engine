# Qualify and pin the borrowed buffer-position AudioWorklet artifact

## Authority and outcome

Parent: #560/#559/#349 CP1. Source peer: #698. This is lane B's second and final active slot; CP1 remains partial. The source review descendant is `43a154093e754a1c96be280aa9656c8dbc6a4824`, with product source checkpoint `13ce1c1f841d3369b59a3f4c0f2b0a127b3a4166`, main `a703f7574d88ea74841c9dbc6001e5e31eb50300`, and tracker `d65d65895254e80332f9a33204c76d564624c1e3`.

Sol HIGH coordinates. Astra XHIGH scopes and verifies. Luna HIGH performs the non-delicate mechanical artifact and pin work. Astra HIGH performs delicate audio/DSP implementation; none is authorized here. Lane B alone owns AudioWorklet qualification and pinning.

This issue determines whether #698's borrowed buffer-position scratch lookup changes shipped AudioWorklet bytes, qualifies the exact generated artifact, and promotes its identity only after the gates below. It changes no product behavior, fixture, test, script, workflow, dependency, lockfile, browser expectation, or predecessor path.

## Source and path ownership

Branch: `codex/qualify-buffer-position-artifact`

Worktree: `/home/bl/misofm/engine-cp1-buffer-position-artifact`

The only allowed tracked paths are this numbered spec, the pin file `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, only the `candidateCommit` and `wasmSha256` fields in `hosts/host-web/qualification/results.json`, and regenerated lineage in `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`.

No other source, fixture, test, script, workflow, dependency, lockfile, browser row, expectation, generated payload, or artifact path may change. The reviewed source file `crates/graph-compiler/src/schedule.rs` has SHA-256 `559abff738fe8fe9155065561d4c47fdc13f426c0b8f4a02414d8e15b0d0dfd1`.

Preserve every predecessor worktree, branch, target, artifact, and evidence path. Never commit `.ll`, `.s`, compiler streams, binaries, generated Wasm/SDK payloads, targets, or raw evidence.

Fresh attempt-1 roots must be absent, including dangling symlinks, before execution:

```text
/tmp/cp1-buffer-position-artifact-a1-evidence
/tmp/cp1-buffer-position-artifact-a1-probe-output
/tmp/cp1-buffer-position-artifact-a1-artifact
/tmp/cp1-buffer-position-artifact-a1-target
/tmp/cp1-buffer-position-artifact-a1-hermetic-target
/tmp/cp1-buffer-position-artifact-a1-tmp
```

## Phase 1: repin identity probe

After Astra XHIGH returns exact-head SCOPE PASS against the clean pushed issue branch, GitHub parity, current tracker and main, source identities, ownership, preserved predecessors, six absent roots, and absence of a relevant process, one explicitly named Luna HIGH executor may run exactly one repin probe. The probe is identity evidence only and pauses for Astra review.

Before any creation, Luna must observe and durably record all six roots absent with both ordinary existence and dangling-symlink checks. It then exclusively creates the evidence directory, durably records the preceding observations, separately creates the tmp directory, then separately creates the probe-output directory as an empty ordinary directory. The artifact directory and both Cargo targets remain absent. Record the complete inherited environment before dispatch. Run exactly:

```text
(cd /home/bl/misofm/engine-cp1-buffer-position-artifact && TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1 bash scripts/build-web-audioworklet.sh /tmp/cp1-buffer-position-artifact-a1-probe-output)
```

Require actual exit status 0, exactly one lowercase 64-hex digest followed by LF on stdout, an empty probe-output directory, and unchanged tracked bytes. The current delivered pin is `6745de399c56e322303e2da55d69a5cbd538e075f0fd480c6620b1896d566645` plus LF. The receipt must count only actual `write_stdin` polls as polls and preserve the initial launch separately. Original tool transcripts are execution authority. A matching digest does not establish qualification. Stop for Astra XHIGH PROBE EVIDENCE review before Phase 2.

## Phase 2: provisional lineage and ordinary qualification

After Astra accepts the probe, set `candidateCommit` in `hosts/host-web/qualification/results.json` to the full product source checkpoint `13ce1c1f841d3369b59a3f4c0f2b0a127b3a4166` and `wasmSha256` to the observed probe digest. Change the pin only if the digest differs. Regenerate only the matrix lineage. Checkpoint and push that qualification-pending overlay, synchronize the issue, and obtain fresh Astra XHIGH exact-head SCOPE PASS before execution.

Use a separate Phase 2 evidence subdirectory and a finite self-excluding manifest. Before the ordinary builder, record that the artifact directory and both Cargo targets are absent, including dangling symlinks; then exclusively create the artifact directory as an empty ordinary directory. Leave both Cargo targets absent for their commands to create. Record complete actual argv, inherited environment plus explicit assignments, cwd, head, dirty state, timestamps, stdout, stderr, actual result, persistent-session launch and each `write_stdin` receipt for every command.

Regenerate the matrix once before the qualification sequence:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target node hosts/host-web/qualification/generate-matrix.mjs
```

Then run exactly once and in order, stopping at the first failure:

```text
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target bash scripts/build-web-audioworklet.sh /tmp/cp1-buffer-position-artifact-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target bash scripts/check-web-audioworklet.sh /tmp/cp1-buffer-position-artifact-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target python3 -B scripts/check-browser-expected-resources.py --artifacts /tmp/cp1-buffer-position-artifact-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-hermetic-target bash scripts/test-web-audioworklet.sh
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target npm_config_cache=/tmp/cp1-buffer-position-artifact-a1-tmp/npm-cache npm --prefix sdk ci --ignore-scripts
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target bash scripts/sdk-package.sh check /tmp/cp1-buffer-position-artifact-a1-artifact
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target npm_config_cache=/tmp/cp1-buffer-position-artifact-a1-tmp/npm-cache npm --prefix hosts/host-web/qualification ci --ignore-scripts
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target npm_config_cache=/tmp/cp1-buffer-position-artifact-a1-tmp/npm-cache npm --prefix hosts/host-web/qualification run qualify -- --artifacts /tmp/cp1-buffer-position-artifact-a1-artifact --browser all --check-matrix --self-test-mutations
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target node hosts/host-web/qualification/generate-matrix.mjs --check
env -u MISO_ENGINE_WEB_AUDIOWORKLET_REPIN TMPDIR=/tmp/cp1-buffer-position-artifact-a1-tmp CARGO_TARGET_DIR=/tmp/cp1-buffer-position-artifact-a1-target git diff --check
```

Immediately after the builder, require exactly six ordinary artifact files. The Wasm digest must equal the probe and resulting pin. The five non-Wasm authority hashes are:

```text
40f6fe2e23e1b47500011c14871750a75922ab194136add8b387a4b40eb56919  miso-engine-v1-abi-layout.json
445254e7c6ddf3330bdf20cafa8cacec4d0e2489805f72a833859db52bc038cf  miso-engine-v1-audio-worklet-host.d.ts
21c8947d8aad2d1d9a23e553c2c7b983dbd5a622aabfbab9a41c622d1a50229a  miso-engine-v1-audio-worklet-host.js
225bc06043ed6e2c62a38d63f1c2015b40480d673e3a53109c938eba481556cb  miso-engine-v1-audio-worklet.js
6eac2cb3e30931b6c01b10c63af4eedd2d59337274565a129c7a3f328a09938d  miso-engine-v1-parameter-metadata.json
```

Any unexpected non-Wasm, resource, PCM, SDK, matrix, browser, or mutation drift fails. If Wasm is unchanged, require full six-file equality with the delivered authority. If Wasm changes, qualify the new six-file set without a size, timing, allocation, performance, or sound-quality claim. Only newly created ignored `sdk/node_modules`, `sdk/dist`, and qualification `node_modules` may appear in the worktree.

## Attempts and delivery

Phases 1 and 2 form one attempt; a review pause consumes no attempt. Any execution or evidence failure stops immediately. A later attempt requires fresh numbered roots and fresh Astra XHIGH scope. Three failed attempts hard-stop without weaker gates or a disguised fourth attempt. A status file containing `0` is not authenticated execution evidence.

After final Astra XHIGH evidence PASS, deliver one PR with #698, required aggregate PR qualification, guarded live-head/base merge, post-main aggregate qualification, GitHub body synchronization and closure, #559/#560/#349 accounting, and eligible clean delivered-worktree cleanup. No artifact, pin, source, performance, allocation, timing, or sound-quality claim is delivered before those remote steps.

## Phase 1 probe evidence and provisional overlay

Astra XHIGH returned **PROBE EVIDENCE PASS** at exact clean pushed head `82a40f7193832acdc8fe017f5b7c7dcf1c6d6c7b`. Luna HIGH launched the exact probe once in persistent session `31099`; three actual `write_stdin` polls ended with authenticated exit status 0. Stdout was exactly `3a9de0b07c8242922ff773114ce44306b8bae7c3444b3785cbbe8bd18028cc16` plus LF. The probe-output directory remained empty, the named artifact and Cargo targets remained absent, and tracked bytes stayed unchanged.

All 27 Phase 1 evidence files under `/tmp/cp1-buffer-position-artifact-a1-evidence` match the finite self-excluding manifest with SHA-256 `221da545ff700bcfce0269b7a829b5d2b5c7e5177ba01654a022704060ca0309`. Original receipt authority is `/home/bl/.codex1/sessions/2026/09/09/rollout-2026-09-09T15-00-29-01a086af-476c-72e3-b3e4-91cc52346125.jsonl`.

The review preserves three limits. The saved preflight receipt is a summary, and the original observation mislabeled spec-only `HEAD~1` as the product checkpoint; Git independently establishes product `13ce1c1f`. Preflight/postflight printed command output rather than full numeric tool results, and the postflight wrapper did not propagate every assertion; Astra independently verified the required observable facts while the probe's zero exit is separately authenticated. No continuous write monitoring is claimed. Phase 1 proves digest identity only.

The qualification-pending overlay is checkpointed at `173b701e92f9491af7e9937f738be2ff0a72a418`. It changes only the provisional pin, the two allowed results fields, and regenerated matrix lineage: product candidate `13ce1c1f841d3369b59a3f4c0f2b0a127b3a4166`, Wasm digest `3a9de0b07c8242922ff773114ce44306b8bae7c3444b3785cbbe8bd18028cc16`. No Phase 2 command has run. Fresh Astra XHIGH exact-head scope PASS remains required before Phase 2 execution.
