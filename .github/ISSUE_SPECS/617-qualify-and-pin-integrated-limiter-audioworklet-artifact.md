# Qualify and pin the integrated limiter AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/617

Parent: #539. Coordination: #559 and #560.

Issue #539's accepted attempt-2 limiter source is integrated with delivered CP8 main at merge
`110dc525ea33742c00bd97a54fa7c60b8738f2af`. The clean synchronized parent record is
`1705cd0e5b6e7314e6465f8c33276e4cfc33e540`; no compiled input changed after evidence head
`d63bc437e948d6284b1b6cbe459f0b46c4ed6566`. An unauthorized background ordinary builder at that
evidence head reported simd128 Wasm digest
`f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664` against delivered CP8 pin
`e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b`, then exited before
publishing files. Astra LOW permanently denied that invocation qualification credit and prohibited
an ordinary-builder rerun, but allowed its digest solely as the expected candidate hypothesis for
one independently approved scratch qualification.

This issue owns that bounded scratch qualification and conditional three-file promotion. #539 and
this successor are the two active issue slots. Sol HIGH coordinates, owns artifact decisions,
checkpoints, GitHub synchronization and delivery. Astra LOW performs scope, scratch-candidate,
post-pin and exact-head/current-base review. Luna HIGH or XHIGH performs only an authorized
repository promotion. Lane A supplies the frozen #539 source and may not build, qualify or repin
the artifact independently.

## Frozen source and non-credit boundary

Freeze exact clean source `1705cd0e5b6e7314e6465f8c33276e4cfc33e540`. Before scratch work,
prove that its compiled AudioWorklet inputs are byte-identical to observation source `d63bc437` and
that it descends from merge `110dc525` with parents `2fd79659` then delivered main `77368243`.
Verify the accepted limiter files remain:

- `crates/true-peak-limiter/src/lib.rs` SHA-256 `677a5596a305039ddbed39d634cde80e90bf99d39439896b1d58a4a539d73b55`;
- `crates/true-peak-limiter/tests/allocation.rs` SHA-256 `7825d30723192724a0cab7165953eeb00860e5dad66dc6b7d74333d1ca0ea920`;
- `crates/true-peak-limiter/tests/mono_collapse.rs` SHA-256 `1f42a58d6f71232a3af6a0a72717ae57bcf1acc0391a9384eb07e21bfbb2dd6d`.

The background observation remains permanently non-credit. Do not rerun its ordinary pre-pin
builder, relabel its executor, treat its empty output as candidate identity, or use it as browser,
SDK, resource, static, hermetic or six-file evidence. Preserve its raw status and checksum record.

## Single scratch qualification

After Astra LOW scope PASS, create one isolated detached scratch worktree at the frozen source. It
may first change only `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the
expected candidate digest plus LF. Prove that one-file overlay and run the unchanged ordinary
builder exactly once to a fresh empty external directory. It must exit zero, emit exactly six files,
and independently reproduce Wasm SHA-256 `f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664`.
A different digest, extra file, build failure or any other source change is FAIL and stops before
repository edits.

Compare the five non-Wasm files byte-for-byte to the delivered #615 manifest. After exact candidate
and five-file identity are established, the scratch may also change only
`hosts/host-web/qualification/results.json` fields `candidateCommit` and `wasmSha256`, then
regenerate `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` with the unchanged generator. Prove this
second overlay separately and freeze every browser row, version floor, gate and resource value.

Run the existing qualification gates once on that exact candidate, without changing them:

1. `scripts/check-web-audioworklet.sh` static ABI/export/import/memory/realtime/SIMD/metadata and
   resource/native-witness checks, including its mutation suite.
2. `scripts/test-web-audioworklet.sh` hermetic host/worklet policy and mutation checks with an
   isolated target directory.
3. The locked SDK package/generated-surface qualification against the same six files.
4. One locked Playwright dependency install and exactly one all-browser qualification invocation
   covering Chromium, Firefox and WebKit, matrix checking and self-test mutations.
5. Final matrix/result comparison proving only source/digest lineage changed; browser outcomes,
   version floors, gates and resources remain unchanged.

Preserve exact argv, cwd, source/toolchain/config/input identities, overlays, full streams/status,
browser versions, output census, six hashes and a repository-verifiable checksum manifest. Do not
commit generated six-file outputs or caches. Astra LOW must return scratch-candidate PASS before any
repository pin or lineage edit.

## Conditional promotion and delivery

After scratch PASS, Luna HIGH or XHIGH may change only:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256` to the qualified digest plus LF;
- `hosts/host-web/qualification/results.json` only for `candidateCommit` and `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md` only through the unchanged generator;
- this numbered spec and bounded evidence as root-owned records.

No Rust, JavaScript/TypeScript, ABI, metadata, browser result row, version floor, resource value,
dependency, lockfile, toolchain/config, script, policy, workflow, corpus, fixture, DSP, benchmark or
timing change is allowed. Root checkpoints the exact promotion before further work. Then run one
ordinary no-bypass post-pin build from the clean pushed promotion head and require exact six-file
identity with the qualified scratch candidate. This post-pin proof is distinct from, and does not
rehabilitate or repeat, the unauthorized pre-pin observation.

Run proportional static/resource/hermetic/SDK, format/diff and workspace/effect-runtime policy
checks after promotion. Astra LOW reviews the exact pushed head/current main, source ancestry,
evidence checksums, three-file edit, generated lineage, pin and post-pin identity. Open one PR to
deliver #539 and this successor only after PASS. Require the repository `qualification` check and
applicable fuzz workflow, reconfirm exact live head/base before guarded merge, verify merge parents
and post-main qualification/fuzz, synchronize and close both issues, update #559/#560, then remove
all clean delivered and detached scratch worktrees while retaining branches and history.

One scratch qualification and one Luna repository-promotion attempt are authorized after their
respective Astra PASS gates. A candidate mismatch or substantive qualification failure stops for a
reviewed rescope. Do not retry a build or browser workload to obtain green, weaken a gate, claim a
performance result, or perform a disguised fourth #539 optimization attempt.
