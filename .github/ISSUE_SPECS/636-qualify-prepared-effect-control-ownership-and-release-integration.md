# Qualify prepared-effect control ownership and release integration

GitHub: https://github.com/misofm/engine/issues/636

Parent: #560 CP1. Supersedes: #633. Coordination: #559. Delivered-main baseline: `62045f40048ec230298fe0fd3935da3333f90b83`. Preserved source branch: `codex/cp1-prepared-effect-index`; hard-stop record: `14a9e0eb51f4aedacefab4af9913906fec039e06`.

#633 replaced repeated owned `(track, rack, effect)` compiler handoffs with compiler-private index-aligned prepared-effect identity in `crates/graph-compiler/src/{compile,ids,banks}.rs`. Three Astra LOW reviews found the production mapping coherent and identified no production defect, public-boundary drift, dependency change, or lock change. #633 nevertheless reached its three-attempt hard stop because the final crossed-control fixture proved only nonzero output rather than comparing against correctly attached controls or an independent expected result. Its release `track_delay` leg also stopped before tests at Cargo duplicate `effect-package` outputs and E0463; the cause has not been qualified as baseline/tooling. #633 is closed as superseded with all checkpoints preserved.

This successor owns a fresh, narrowly bounded qualification workflow for those two residual gates. It inherits the frozen #633 production implementation and accepted processor-crossing, routed-sidechain, bank-association, deterministic, diagnostic, canonical-identity, debug/release library, Clippy, formatting and policy evidence. It does not reopen or relabel any #633 attempt.

Sol HIGH coordinates scope, checkpoints, GitHub and delivery. Luna HIGH or XHIGH executes the authorized correction and commands. Astra LOW performs every scope, source, evidence, exact-head, CI and delivery review. Lane-A #635 is the only other active issue and owns disjoint transient-shaper documentation/evidence paths.

## Smallest closable slice

Only `crates/graph-compiler/src/lib.rs` may change, and only in the existing #633 association fixture or a directly adjacent test helper. Production `compile.rs`, `ids.rs`, `banks.rs` is frozen unless the new oracle exposes a real product defect; such a defect stops this issue for a new scope ruling rather than authorizing an immediate production edit.

The control-ownership oracle must use the same deterministic graph/input and the same explicitly targeted command to compare correctly attached consumers with a deliberately crossed consumer association, or compare the crossed result with an independently derived exact expected result. The assertion must fail when the target command reaches the wrong effect and must identify the intended track/rack/slot/effect separately from the structure under test. Nonzero-output, producer-label-only, or same-implementation self-comparison assertions are insufficient.

The existing accepted processor-payload crossing, exact sidechain source/destination/port, homogeneous/heterogeneous bank membership/program order, repeated compilation, canonical graph/semantic identity and diagnostic fixtures must remain intact.

Separately, qualify the release `track_delay` integration leg with this one
predeclared command after verifying that its target path does not already exist:

```text
CARGO_TARGET_DIR=/tmp/issue636-release-track-delay-34982484 cargo test --locked --release -p graph-compiler --test track_delay
```

Before execution, Astra must review this exact package/target/profile invocation
and determine whether it can discriminate the previously colliding workspace
artifact shape without changing manifests or dependencies. No features or target
triple are added. Run it once and stop on failure; no retry, alternate command, or
manifest repair is allowed. If it still stops before tests, preserve the command,
status and diagnostic and split the runner/tooling defect without weakening the
control-ownership gate.

No full compiler output or `.ll`/`.s` capture, artifact generation/pinning, benchmark, timing, allocation measurement, dependency/manifest/lock change, public API change, scheduling/PDC work, or broader refactor belongs here. Allocation qualification remains a later separately numbered successor after this issue passes and a slot is free.

## Objective gates

Before Luna work, Astra LOW must pass current applicability, inherited-source integrity, exact test-only scope, the causal oracle design, the single release-integration command, and disjoint ownership from #635. Luna then makes one coherent test tranche and runs each authorized focused/full debug/release library, release integration, strict affected Clippy, rustfmt, workspace policy and diff-hygiene gate at most once, stopping at the first failure. Root checkpoints the exact tranche before Astra review.

Astra must adversarially verify that the oracle independently distinguishes correct from crossed live-control ownership; that all inherited association and contract coverage remains valid; that the release integration result is attributable; and that production, lock, dependency and artifact paths are unchanged. The ordinary three-attempt rule applies afresh to this successor. PASS permits a separately numbered transient-allocation qualification issue when a slot is free; it does not itself make an allocation or performance claim.
