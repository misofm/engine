# Qualify prepared-effect release integration with contemporaneous provenance

GitHub: https://github.com/misofm/engine/issues/642

Parent: #636. Superseded implementation parent: #633. Audit parent: #560 CP1. Coordination: #559. Frozen source branch: `codex/cp1-prepared-effect-qualification`; formatted source commit: `74108ec994a7647dedd6149df5de8550134228a5`.

#636 preserved a technically sound prepared-effect ownership implementation and a causal bitwise correct-versus-crossed PCM oracle, but reached its three-attempt hard stop because its release-integration execution lacked contemporaneous command, environment, status, preflight and execution-head records. Retrospective executor testimony cannot repair that provenance gap.

This successor owns one release-integration qualification and, if it passes, delivery of the already frozen source. It permits no source, test, manifest, dependency, lock, profile, policy, workflow or artifact change. Sol HIGH coordinates the issue, checkpoints, GitHub, artifact exclusion and delivery. Hypatia, the Luna HIGH agent `issue583_luna_impl`, is the sole executor for the single frozen command. Astra LOW reviews scope, evidence, exact head, CI and delivery.

## Smallest closable slice

Before execution, record the exact cwd, full HEAD, upstream equality, clean tracked and untracked status, Rust/Cargo versions, literal argv and environment, and confirm the predeclared target path is absent including as a dangling symlink. Then run exactly once:

```text
CARGO_TARGET_DIR=/tmp/issue642-release-track-delay CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --test track_delay
```

Capture separate complete stdout/stderr and numeric shell status in `/tmp/issue642-release-evidence/`. Stop on any preflight mismatch, nonzero status, missing test execution, tracked drift or unexpected diagnostic. Do not retry, choose another target, delete a populated target, or run another compiler/test command.

The result qualifies release-test integration only. It grants no shipped-abort-profile, compiler-artifact, performance, allocation or Cargo-root-cause claim. Git may retain only the exact command, identities, numeric status, hashes, concise diagnostics and test totals. Full or compressed streams, `.ll`, `.s`, objects, archives, binaries, `rlib`, `rmeta` and target directories remain temporary and untracked.

## Objective gates

Astra LOW must pass the clean synchronized brief and fresh-target precondition before Luna execution. Astra then verifies contemporaneous provenance, 8/8 actual `track_delay` execution, no source/tree drift, artifact exclusion and the inherited #636 technical source PASS. A qualification PASS permits exact-head/current-main PR review and ordinary required qualification, guarded merge, post-main success, GitHub synchronization and clean worktree removal. A failure stops this issue without retry or gate weakening.

## Attempt 1 Astra LOW evidence review — PASS

Hypatia ran the single command once at clean source/upstream
`428f94a24c3c8ac7952c35f34b85398dfd269d7a`. The fresh preflight recorded the
exact cwd, source, toolchain, literal command/environment, clean tree and absent
target. The numeric status is 0 and all 8/8 `track_delay` tests executed and
passed; postflight remained clean.

The temporary record identities are:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `preflight.txt` | 683 | `a2fd322649639c4ec2dbe13d12cd39dd91793abeac64d4f57dc2de3141778070` |
| `stdout.txt` | 518 | `44fb9d323f7dab9d66d5a17de74503522ee4ac95db8f35f346375f1505ed1465` |
| `stderr.txt` | 4,315 | `00c22abbd9a4eb0e20eb396c9b8335d78c75a90d621bba75ef431e19337b8587` |
| `status.txt` | 2 | `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa` |
| `postflight.txt` | 652 | `f6ac4dd3a3ab498c6f28d5090451ed2b55e9617f1ac1db8d11dd2612fecf244d` |

The raw preflight's `executor=bl` identifies the OS account; Hypatia's
contemporaneous agent completion supplies the agent attribution. Astra LOW
accepted the combined provenance without modifying the raw record and found no
retry evidence. Full streams and the populated target remain only in `/tmp`; no
artifact or stream is committed.

This PASS qualifies release-test integration only. It preserves #636's final
FAIL and makes no shipped-profile, compiler-artifact, Cargo-root-cause,
performance or allocation claim. Exact-head/current-main PR-readiness review is
required before PR creation.
