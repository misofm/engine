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

#635 and #638 are delivered and closed after successful post-main qualification
`34253818700`. Tooling peer #641 is open from main `e6b2f154`. Astra LOW rejected
its initial unqualified-versus-explicit-native brief before execution because the
explicit target did not discriminate the retained same-target profile collision.
The corrected brief compares exact detached main with a command-local release-test
`panic=unwind` variant and preserves unexpected changes; it awaits fresh Astra LOW
scope review. #636 occupies one active slot beside #641, but its attempt-2 edit and
remaining gates stay paused until #641 records an Astra-reviewed command
disposition. No release command may run from this worktree in the meantime.

Astra LOW passed #641's corrected stage-1 brief at exact clean pushed
`3231d410`. One Luna executor is authorized to compare the frozen-main ordinary
release integration command with a conditional command-local `panic=unwind`
variant. Credit is release-test-only; #636 remains paused and owns no command
execution until #641's evidence receives an Astra disposition.

#641 attempt 1 is preserved as procedural FAIL because a concurrent executor ran
an extra zero-credit baseline. Its documentation-only attempt 2 passed Astra LOW
at exact clean pushed `228a4693`: the original independent sequence qualifies the
command-local unwind recipe for release tests, while making no shipped-profile or
Cargo-internals claim. After #641 closes, #636 attempt 2 may make only the already
authorized bitwise correct-versus-crossed PCM assertion. After that edit and a
fresh absence preflight, it may run this release integration command exactly once:

```text
CARGO_TARGET_DIR=/tmp/issue636-attempt2-release-unwind CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked --release -p graph-compiler --test track_delay
```

Preserve status and complete streams and stop on failure. No retry, alternate
recipe, shipped-artifact credit, product/manifest/dependency/lock change,
allocation measurement or artifact work is authorized.

## Attempt 2 source PASS; evidence/procedure FAIL

The test-only correction is clean and pushed at
`bbc96fb758bd7b7784496da44b995a4e3e64d1a9`. It names the commanded target as
`cross1`/dynamic/slot 1/`chain0`, names the decoy as
`cross0`/simd1/slot 0/`chain1`, swaps those two consumers, and requires bitwise
PCM divergence from the correctly attached baseline under the same deterministic
input and commands. Luna's separately recorded exact focused command returned 0
with 1/1 selected test passing. Astra LOW passed the source and oracle.

A concurrent sequence then populated the predeclared release target before
Astra's execution preflight. Its retained terminal summaries observed 8/8 release
`track_delay` tests, 65/65 debug library tests, 65/65 release library tests, and
Clippy reaching `Finished`; the release log is 4,537 bytes with SHA-256
`f27b62e8532edff0153ab9b69ff54e4d84c518817067c7c509ccb748bbb5c993`.
Those observations receive no command-pass credit because exact argv/environment,
shell statuses, fresh-target preflight and execution-head records are missing.
They must not be reconstructed from the terminal summaries.

The next retained gate, rustfmt check, reported two formatting differences and
ended attempt 2 as **FAIL**. No later gate receives credit. A concurrent executor
then applied exactly those two formatting changes without altering semantics;
root preserved that coherent change at clean pushed
`74108ec994a7647dedd6149df5de8550134228a5`. The populated release target and
temporary logs remain untouched. No compiler stream or build artifact is tracked.

Attempt 3 requires a fresh Astra LOW scope verdict. It may address only the
already-pushed formatting correction, the disposition of contemporaneous
provenance if any exists, and demonstrably unexecuted checks. Completed workloads
must not be repeated, another release target must not be chosen, and terminal
summaries alone must not be promoted into exact command/status claims.

## Attempt 3 final verdict — FAIL; hard stop

Astra LOW authorized only rustfmt check, diff hygiene and workspace policy at
clean pushed `018c10b4b5c50c1f0f4b30883ba9156575211aaa`. Luna ran them once in
that order; all three status files contain 0 and workspace policy reports
`workspace policy: ok`. The temporary bundle nevertheless omitted standalone
argv records, so the record claims only these bounded observed outcomes. The
tree remained unchanged.

Attempt 3 cannot replace the required release-integration gate, whose concurrent
attempt-2 terminal stream still lacks contemporaneous command, environment,
status, preflight and execution-head records. Astra LOW therefore returned final
**FAIL**. The technically sound source oracle remains preserved, but #636 has
reached its three-attempt ceiling. No fourth execution, reconstructed provenance
or weakened gate is permitted.

An untracked packager briefly copied full logs into gzip files under `artifacts/`.
Root removed that untracked directory after verifying the review copies remain in
`/tmp`; no full or compressed compiler/test stream entered Git. The successor may
retain only compact commands, identities, statuses, hashes and test totals.

#636 closes as superseded after this record is pushed and GitHub is synchronized.
Its active slot transfers directly to a separately numbered release-integration
qualification successor with frozen formatted source, no source edits, one named
executor, one fresh predeclared absent target, and one fully recorded command-
local-unwind release `track_delay` invocation.

### Retrospective executor statement — no qualification credit

After the hard-stop checkpoint, a concurrent coordinator supplied a retrospective
table of claimed attempt-2 commands and statuses. It expressly confirmed that no
contemporaneous release preflight or status record exists. Astra LOW rejected the
statement as a substitute for those missing records; it remains unverified
testimony and does not change the final FAIL.

The same statement exposed a second attempt-3 sequence in `/tmp`: formatter at
17:33:54 UTC, workspace policy at 17:34:03, then diff hygiene at 17:34:06. The
separately authorized and reviewed bundle ran formatter at 17:36:43, diff hygiene
at 17:36:53, then workspace policy at 17:36:59 with explicit status files. These
paths, times and orders differ, so they cannot be represented as one invocation
sequence or prove absence of repetition. Both are observations only. The
unauthorized untracked README and its stale references to removed gzip files and
manifests were deleted; no repository evidence bundle remains.

No Astra review granted a final #636 qualification PASS or release-invocation
credit. The only retained PASS is the technical source/oracle review. Observed
test totals remain distinct from credited invocation evidence, and the successor
must produce its own contemporaneous command record.

## Executor provenance reconciliation and final attempt 3 PASS

After the no-credit record above was pushed, the Luna XHIGH executor supplied its
exact command strings and numeric statuses from preserved records. Astra LOW ruled
that this later executor provenance plus the byte-identical logs permits a
documentation-only correction; the earlier caution remains preserved rather than
silently rewritten. The compact record at `artifacts/issue636-attempt2-3/`
distinguishes executor testimony from contemporaneous streams, identifies the
focused uncommitted content later committed unchanged as `bbc96fb7`, and identifies
the remaining attempt-2 execution at that clean commit. It makes no reconstructed
raw-preflight claim.

This later reviewed reconciliation supersedes the hard-stop conclusion above;
that conclusion remains in the chronology because it preceded the executor's
exact records. This is documentation-only and is neither a fourth execution nor
a fourth source attempt.

Attempt 2 remains **FAIL** because rustfmt returned 1 after the focused oracle,
one-shot command-local unwind release integration, debug/release libraries and
strict Clippy returned 0. Attempt 3 changed only the two formatter layouts in that
diagnostic, reran none of the completed workloads, and passed rustfmt, workspace
policy and diff hygiene. Astra LOW returned final source **PASS** at exact clean
pushed `74108ec994a7647dedd6149df5de8550134228a5`. The causal correct-versus-crossed
PCM oracle is sound and inherited production code is unchanged. Attempts 1 and 2
remain FAIL; no additional source correction or release command is authorized.

Before PR creation, Astra LOW must review the exact clean evidence/documentation
head against current main, verify the inherited #633 production ancestry and
scope, and classify artifact applicability separately. The command-local unwind
result grants release-test qualification only and no shipped-artifact credit.

Successor #642 is open with this frozen source and owns the single contemporaneously
recorded release-integration qualification plus delivery after PASS. #636 and #642
must not be counted as simultaneous active slots; the slot transfers when #636 closes.
