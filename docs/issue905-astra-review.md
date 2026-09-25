# Issue #905 — Astra adversarial review

Reviewed 2026-09-25 by Astra xhigh. **Attempt 1: PASS. No blocking findings.**
The reviewed checkpoint is `04f70e5845d12211aa036da812d3892c0a1df64c` on
`codex/batch-880-class-b`. The accepted implementation source remains
`bb9efd095bd8d70be8b8503cba6c3617fd51b74e`, as confirmed in
[`issue880-class-b-astra-review.md`](issue880-class-b-astra-review.md).
This verdict covers the bounded artifact qualification; it does not reopen the
accepted DSP or claim completed delivery.

## Artifact and source identity

I independently hashed the retained ordinary-build artifact. Its Wasm SHA-256 is
`25e75763f1e6ea815a938de60367549c973cf3c5dc551a3a3af5d5ac5b79e20a`, matching the
report-mode stdout, live pin, receiver expectation and generated browser record.
The artifact directory contains exactly seven regular files, without symlinks or
extra entries. All six non-Wasm files match both their current authorities and
those authorities' accepted-source Git blobs. The report-mode output directory
remains empty. The provisional-pin checkpoint `4bd4d666` changes only the live
hash and precedes the ordinary-build evidence.

The accepted-source-to-reviewed-checkpoint diff contains only the four evidence
documents/specs and four live-identity/generated-lineage paths listed in the
implementation record. The #902 document change records its already accepted
review; it introduces no additional implementation. The receiver script differs
only in `REAL_WASM_SHA256`. DSP, host and SDK implementation, fixture expectations,
numeric/resource limits, dependency locks, toolchain, build script/flags, browser
floors and CI remain unchanged. The historical npm publication workflow and its
test retain their prior identity.

All 86 archived SDK/license Git entries match accepted source, including the
literal target of the checked-in `prepared-control.js` symlink. The built SDK's
seven artifact assets match the qualified artifact. I also compared its 96
distribution files with the SDK distribution used by the package gate: only the
two bundled worker files differ, exclusively in esbuild's source-path comments
(`// sdk/dist/` versus `// dist/`). Their executable content is identical.

## Qualification evidence

I inspected the retained commands, compiler/gate stdout and stderr, exit records,
tool identities and unchanged gate implementations. Both builders and all
prescribed gates have zero exit records. The evidence supports:

- Static/object AudioWorklet checks and expected-resource/native-witness checks,
  including all 26 resource red mutations.
- Hermetic policy, host/control and mutation checks. The failure diagnostics in
  the hermetic log belong to deliberately rejected mutations; its final suite
  passes.
- The explicit real-Wasm receiver: boot/status/dispose/repeated-dispose,
  corrupted-Wasm refusal, and the disposal mutant with independent cleanup.
- SDK types, headless 284/284, package CLI 11/11, artifact-builder contract and
  publishable-tarball smoke checks.
- One SDK-inclusive all-browser recording run with the accepted source commit and
  `--self-test-mutations`. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5
  each pass all seven recorded gates, including SDK response. The unchanged runner
  performs validation and mutation checks before emitting each passing row.

An independent structural comparison confirms that `results.json` changes only
`candidateCommit` and `wasmSha256`; every other field, browser floor and passing
gate remains identical. The matrix differs only in those same lineage values.
I independently ran its cheap generated-document check and the bounded
accepted-source-to-reviewed-checkpoint `git diff --check`; both passed. No costly
successful gate or timed workload was rerun for this review.

The execution evidence remains under `/tmp/issue905-attempt1-logs/`, with the
artifact under `/tmp/issue905-attempt1-artifact/` and the accepted SDK under
`/tmp/issue905-attempt1-source/sdk/`. Review hashes of the retained evidence and
artifact, plus the SDK distribution comparison, are under
`/tmp/issue905-astra-review-dt08fvaf/`. This review independently checks retained
bytes and evidence consistency; it does not claim a second independent build or
browser run.

Root must checkpoint this verdict, deliver the coherent batch, obtain exact-head
qualification and required main qualification, synchronize/close #880, #902 and
#905 after their evidence is upstream, and preserve evidence before removing
completed worktrees. No source edit, commit, push or issue closure was performed
by this reviewer.
