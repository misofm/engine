# Issue #904 — Astra adversarial review, attempt 1

**Verdict: PASS — bounded AudioWorklet artifact qualification.**

Reviewed on 2026-09-25 by Astra xhigh. Accepted #880 source:
`b3910b37033d0fb30b98405140d03d56fccc3f49`; reviewed delivery checkpoint:
`c37a9dce02142802703919ecf5a42ad108f74308`. This is #904's first verdict,
not another #880 MA-3 attempt. No blocking findings or required corrections.

The tracked delta is restricted to the #904 spec, live artifact pin, generated
qualification record/matrix, and the live real-Wasm receiver's expected hash.
The receiver test changes only its hash constant. DSP/runtime sources, SDK sources,
numeric/resource expectations, corpus pins, tolerances, toolchain/dependencies,
build flags, browser floors, builder and CI routing remain unchanged. The historical
npm release pin remains unchanged.

Independent checks and reviewed evidence:

- Read the original GitHub job log for run `36084071070`, job `107912071246`.
  It reports the old-pin mismatch and exit 1 with observed candidate
  `772b65111a22fa07135dd3f90628774a5587ef6891e14da25f28a2989d3d4d56`.
- Reviewed source/tool identities, report-mode and ordinary-builder invocations,
  build output, and recorded exits in `/tmp/issue904-attempt1-logs/`.
  Report mode returned the same candidate; its output directory remains empty.
  Provisional pin checkpoint `48326abb` precedes the ordinary build and gates.
- Independently hashed `/tmp/issue904-attempt1-artifact/`: exactly seven regular,
  non-symlink files. Every current hash matches the ordinary-build hash list.
  The Wasm hash matches CI, report mode, the live pin, and qualification lineage.
  All six non-Wasm outputs compare byte-for-byte with both their accepted-source
  Git authorities and current authorities, including both generated JSON files.
- Reviewed substantive static/object, expected-resource/native-witness, hermetic,
  SDK type, SDK headless, SDK package, full-browser, and real-Wasm receiver output.
  Their preserved exit records are zero. Headless reports 284/284 tests and package
  reports 11/11; expected-resource self-tests report 26 red mutations. Hermetic
  stderr's intentional negative-control failures are accompanied by passing
  mutation checks, not hidden ordinary gate failures.
- The initial browser run omitted `--sdk-root` and was incomplete for this scope.
  Its logs remain preserved. The subsequent full invocation uses the same artifact,
  accepted source, all three browsers, SDK root, and `--self-test-mutations`.
  Chromium 151.0.7922.34, Firefox 153.0, and WebKit 26.5 all pass. The unchanged
  runner validates results and mutations before writing the record; the final
  record retains `sdkResponse: pass` in every row. The selected SDK has no tracked
  source difference from the accepted commit.
- The separate real-Wasm receiver log proves exact artifact/hash acceptance,
  boot/status/dispose/repeated-dispose, corrupted-Wasm refusal, and the disposal
  mutant with independent cleanup. The hermetic wrapper is correctly distinguished
  from this real receiver gate.
- Independently compared old/new result objects: only `candidateCommit` and
  `wasmSha256` change. The generated matrix carries only that lineage change.
  Re-ran the cheap matrix `--check` and `git diff --check`: both pass.

No builder, browser qualification, other expensive successful gate, or timed
workload was rerun during review. The original logs and artifact remain external
evidence; no compiled artifact was added to Git. This evidence qualifies the
existing browser/control/PCM/resource contracts for the new binary; it does not
claim universal binary equivalence from the unchanged numeric fixtures.

Root still owns the coherent PR #903 update, exact-head and post-merge qualification,
GitHub evidence/state synchronization, and completed-worktree cleanup. #904 should
close only after that delivery. #880 remains open pending R1/R2/R3/R4 owner rulings;
this artifact PASS authorizes no Class-B implementation or registry publication.
