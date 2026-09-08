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

## Astra LOW scope review — PASS

Astra LOW passed exact clean pushed brief
`b1b9eb63ab4e93711bf47436814a9868265c7d73` against live main
`62045f40048ec230298fe0fd3935da3333f90b83`. #633 is closed with its source and
three failed attempts preserved. #635 now identifies #636 as its disjoint peer
and #633 as closed, so #635/#636 are the two synchronized active slots.

Luna attempt 1 may edit only the existing association fixture or an adjacent test
helper in `crates/graph-compiler/src/lib.rs`. Production and every inherited
processor, sidechain, bank, deterministic, diagnostic and canonical-identity
control remain frozen. The new oracle must compare correct and crossed live-
control ownership or use an independent exact result that visibly rejects the
wrong target.

After rechecking that `/tmp/issue636-release-track-delay-34982484` is absent, Luna
may run the literal release-integration command in this brief exactly once. Run
each other authorized focused/full library, Clippy, formatting, workspace-policy
and diff-hygiene gate at most once and stop at the first failure. Preserve the
actual status and complete diagnostic. No retry, alternate invocation, manifest
repair, production change, allocation measurement, artifact or compiler capture
is authorized. A recurring pre-test collision moves to a separate tooling
disposition and receives no test credit.

## Attempt 1 source/evidence review — FAIL

Astra LOW reviewed exact clean pushed checkpoint
`10cdc31ce384e7c156597d95cd6b53db4fbf1172`. Production, Cargo manifests,
`Cargo.lock` and all inherited processor/sidechain/bank controls remain unchanged.
The focused debug test and diff hygiene passed, but the live-control oracle still
does not satisfy the causal gate: the change assigns different target/decoy
parameter values, then asserts only that the crossed render contains a nonzero
sample. It never compares crossed PCM with correctly attached PCM or an
independently derived exact result.

The predeclared release integration command ran exactly once after the target-
absence preflight and stopped before tests at duplicate `effect-package` outputs
and E0463 missing `effect_compiler`. No retry or later gate ran. The result receives
no test credit and must move to a separately numbered tooling issue; it may not be
called a baseline defect without evidence.

Attempt 2 is limited to the literal bitwise correct-versus-crossed PCM divergence
assertion, using identical deterministic input and commands with separately
identified owners. Preserve production and all accepted controls. The tooling
split and remaining-gate command set must be recorded before Luna continues. No
release integration retry is authorized in #636.

## Coordination pause

#636 is queued after attempt 1 and does not consume an active issue slot. #635's
post-main qualification reproduced an offline dependency-cache miss after its
single authorized failed-job rerun, so a bounded qualification-cache preparation
issue must occupy the available slot beside #635 until main is green. #636 retains
its exact attempt-2 test authorization but Luna may not edit or run remaining
gates while queued. After #635 delivery releases its slot, create the separately
numbered graph-compiler release-collision tooling issue required above, synchronize
the two-slot ownership, and only then resume #636 attempt 2.
