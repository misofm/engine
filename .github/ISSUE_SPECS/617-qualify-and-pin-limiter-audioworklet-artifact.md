# Qualify and pin the combined limiter AudioWorklet artifact

GitHub: https://github.com/misofm/engine/issues/617

Parent: #539. Coordination: #559 and #560.

Issue #539's accepted limiter specialization is integrated with delivered CP8 main at merge
`110dc525ea33742c00bd97a54fa7c60b8738f2af`. Its compiled-source checkpoint is
`d63bc437e948d6284b1b6cbe459f0b46c4ed6566`; the current clean delivery head
`42494ef71f31d4e5e9e497aeb52dba51e1bb47de` adds only evidence packaging and records. The accepted
three limiter files and attempt accounting remain unchanged: attempt 1 FAIL, attempt 2 PASS.

A non-credit ordinary observation reported combined-source Wasm SHA-256
`f80b6392b1ea7141aaac639d08094f88982febb883c4d08c3f1114418093e664` against delivered CP8 pin
`e338adae98454d0a365c0ef281aa6b3dcb24d5dc0a36427f917566682e0ff27b`. Its raw record is under
`artifacts/issue539-cp8-artifact-decision/`. That invocation must not repeat and establishes no
artifact identity or qualification. This issue treats `f80b6392…` only as the expected candidate
hypothesis for one independent scratch qualification.

Sol HIGH coordinates, owns documentation, checkpoints, GitHub synchronization and delivery. Astra
LOW performs every scope, artifact, exact-head and delivery verification. Luna HIGH or XHIGH may
perform only the conditionally authorized repository edit. Lane B owns qualification and pinning.
No benchmark, timing/capture, performance claim or additional limiter implementation is allowed.
#539 and #617 are the only active issue slots.

## Frozen source and exact ownership

Create an isolated successor worktree from clean pushed head `42494ef7`. Before any qualification,
verify current main remains `773682433ef451b89e5359fa8f722e1016c64fb3`, compiled source
`d63bc437` is an ancestor, the source-to-head delta contains only issue/evidence packaging, and the
input identities match the retained observation. The following repository paths are conditionally
owned only after candidate PASS:

- `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`;
- `hosts/host-web/qualification/results.json`, only `candidateCommit` and `wasmSha256`;
- `hosts/host-web/BROWSER_DEPLOYMENT_MATRIX.md`, only generated lineage text;
- this issue's spec and bounded evidence records.

No Rust, JavaScript/TypeScript, ABI, metadata, browser row, version floor, resource expectation,
dependency, lockfile, toolchain/config, script, policy, workflow, corpus, fixture, DSP, session, SDK
surface or other #539 path may change.

## One independent scratch qualification

After Astra LOW scope PASS, create one isolated scratch checkout at compiled source `d63bc437`.
Change only the scratch pin to expected candidate `f80b6392…` plus LF and prove that overlay. Run the
unchanged ordinary builder exactly once into an empty external directory. It must exit zero, emit
exactly six files and produce Wasm `f80b6392…`. Any other digest, build failure, extra output or
scratch-source change is FAIL and stops before repository edits. Do not retry.

Compare all five non-Wasm outputs byte-for-byte with the delivered CP8 six-file manifest retained in
`artifacts/issue539-cp8-artifact-decision/delivered-six.sha256`. Any mismatch stops for rescope.
Preserve exact argv, cwd, clean/source/main identities, toolchain/config/input hashes, overlay diff,
full streams/status, output census, six-file hashes and a checksum manifest.

Only after candidate and five-file identity pass, the scratch checkout may also set
`results.json`'s `candidateCommit` to `d63bc437e948d6284b1b6cbe459f0b46c4ed6566` and `wasmSha256`
to the candidate, then regenerate only the matrix lineage with the unchanged generator. Prove the
second overlay and freeze every browser result row, version floor, gate and resource value.

Run each existing qualification gate once against that exact candidate:

1. shipped Wasm ABI/export/import/memory/realtime-callgraph/SIMD/static/metadata/vocabulary checks;
2. expected resources/native witness and all red mutation checks;
3. hermetic host/worklet policy and mutation checks with an isolated Cargo target;
4. locked SDK package/generated-surface checks;
5. locked browser dependencies installed with `npm ci --ignore-scripts`;
6. one all-browser qualification across Chromium, Firefox and WebKit, including matrix checking and
   self-test mutations;
7. a final diff proving only provisional pin plus two lineage fields and generated matrix lineage,
   with all browser outcomes, versions, gates and resources unchanged.

Astra LOW must return candidate-qualification PASS over the checksum-verified record before any
repository pin or lineage edit. One scratch qualification is authorized; a substantive failure
stops for reviewed rescope and cannot be retried green.

## Conditional promotion and delivery

After candidate PASS, Luna HIGH/XHIGH may copy only the qualified pin and two lineage fields into the
successor worktree and regenerate only the matching matrix lineage. Root checkpoints that exact edit
before further work. Run one ordinary no-bypass post-pin build from the clean pushed promotion head
and require exact six-file identity with the qualified scratch candidate. Then run the existing
static/resource/hermetic/SDK gates plus proportional formatting, branch-diff, workspace and effect-
runtime policies. Do not repeat browsers without a specific failed gate requiring it.

Astra LOW reviews exact pushed head/current main, source ancestry, evidence checksums, three-file
edit, frozen browser rows/resources, generated lineage, pin and post-pin identity. Open one PR only
after PASS. Require the repository `qualification` check and any routed fuzz check, verify live main
immediately before guarded merge, verify merge parents, and require post-main qualification. Close
#539 and #617 only after upstream evidence and GitHub synchronization. Update #559/#560 and remove
both clean delivered worktrees while retaining branches/history.

This issue has one scratch-qualification pass and one conditional repository-edit attempt. It does
not create another #539 implementation attempt and carries no benchmark, timing or speedup claim.
