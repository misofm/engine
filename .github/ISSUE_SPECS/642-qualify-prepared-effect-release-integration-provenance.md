# Qualify prepared-effect release integration with contemporaneous provenance

GitHub: https://github.com/misofm/engine/issues/642

Parent: #636. Superseded implementation parent: #633. Audit parent: #560 CP1. Coordination: #559. Frozen source branch: `codex/cp1-prepared-effect-qualification`; formatted source commit: `74108ec994a7647dedd6149df5de8550134228a5`.

#636 preserved a technically sound prepared-effect ownership implementation and a causal bitwise correct-versus-crossed PCM oracle, but reached its three-attempt hard stop because its release-integration execution lacked contemporaneous command, environment, status, preflight and execution-head records. Retrospective executor testimony cannot repair that provenance gap.

This successor owns one release-integration qualification and, if it passes, delivery of the already frozen source. It permits no source, test, manifest, dependency, lock, profile, policy, workflow or artifact change. Sol HIGH coordinates the issue, checkpoints, GitHub, artifact exclusion and delivery. One Luna HIGH or XHIGH executor runs the single frozen command. Astra LOW reviews scope, evidence, exact head, CI and delivery.

## Smallest closable slice

Before execution, record the exact cwd, full HEAD, upstream equality, clean tracked and untracked status, Rust/Cargo versions, literal argv and environment, and confirm the predeclared target path is absent including as a dangling symlink. Then run exactly once:

```text
CARGO_TARGET_DIR=/tmp/issue642-release-track-delay CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --test track_delay
```

Capture separate complete stdout/stderr and numeric shell status in `/tmp/issue642-release-evidence/`. Stop on any preflight mismatch, nonzero status, missing test execution, tracked drift or unexpected diagnostic. Do not retry, choose another target, delete a populated target, or run another compiler/test command.

The result qualifies release-test integration only. It grants no shipped-abort-profile, compiler-artifact, performance, allocation or Cargo-root-cause claim. Git may retain only the exact command, identities, numeric status, hashes, concise diagnostics and test totals. Full or compressed streams, `.ll`, `.s`, objects, archives, binaries, `rlib`, `rmeta` and target directories remain temporary and untracked.

## Objective gates

Astra LOW must pass the clean synchronized brief and fresh-target precondition before Luna execution. Astra then verifies contemporaneous provenance, 8/8 actual `track_delay` execution, no source/tree drift, artifact exclusion and the inherited #636 technical source PASS. A qualification PASS permits exact-head/current-main PR review and ordinary required qualification, guarded merge, post-main success, GitHub synchronization and clean worktree removal. A failure stops this issue without retry or gate weakening.
