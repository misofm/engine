One-line summary: Let eligible built-in meter observers consume the final resident
AoSoA output of a successfully executed bank at the existing observation point,
while retaining the current scalar meter arithmetic and planar fallback.

## Status, authority, and dependency

This is a queued, placement-only child of #349 RT10, coordinated by lane A under
#559/#560. The opening baseline is main
`b1f9128f3e06532afdfc16aad661c4b2deb5dea1`; reconciled tracker authority is
`1a6bb3d12f900445e9aa9ebc65589d804ea59bce`. Astra XHIGH supplied the read-only
scope and delivery audit. Sol must approve the brief and Astra LOW must review
its scope before implementation authority exists.

Implementation is blocked until #713's qualified product source
`7576d1b6c794f01df4abeb7256a8309d45879b21` is integrated into main with its
required delivery evidence. Its SOURCE PASS record is `bd10831e`; that is
source qualification, not merged delivery. After integration, record the exact
new main and dependency merge, revalidate the source anchors and finite preflight
below, and obtain explicit implementation authorization. Do not cherry-pick,
modify, requalify, or otherwise consume #713's passive worktree during this brief.
This is an intra-lane-A source dependency, not a cross-lane dependency or a new
lane-B obligation. The queued issue consumes no active implementation slot.

Lane B alone owns AudioWorklet qualification, artifact disposition and pins.
This brief grants no authority over those paths or #705's execution. It makes no
timing, speedup, improvement, load-elimination, full-RT10, or delivery claim.

## Delivery audit and inherited evidence

RT10 is still open. #203 recorded costly planar meter observation; its closure
does not establish resident metering. #516/#519/#520 delivered coherent meter
publication and selective metric/observer preparation through
[PR #521](https://github.com/misofm/engine/pull/521), reviewed head
`6567ef4d639dce05e359096e106588aa95c0602b`, merge
`be781895decc72328f727dcd816b8b40a2ab6051`. That merge is an ancestor of this
brief's main baseline. Required [PR qualification 34034182930](https://github.com/misofm/engine/actions/runs/34034182930)
and [post-main qualification 34034536132](https://github.com/misofm/engine/actions/runs/34034536132)
completed successfully at those respective exact heads. They qualify that prior
delivery; they are not evidence for this child. The historical metering timing
invocation failed warmup; no speedup is inherited.

Current source still routes `MeterObserver::observe` in
`crates/builtins-compiler/src/lib.rs:4311` to planar
`MeterAccumulator::observe` in `crates/builtins/src/lib.rs:3783`.
`crates/graph/src/runtime.rs:1630` acquires planar arena input for each observer;
`observe_segment`/`observe_selected_segment` at builtins lines 4000/4048 retain
scalar per-channel loops. Relevant issue/PR history and current source show no
resident AoSoA meter implementation. The original RT10 comment explicitly
separates placement from optional vectorization.

#713's accepted resident-input behavior, observer/send semantics, and resource
accounting are inherited constraints. Its review evidence is preserved at
`/tmp/issue713-astra-low-review-wcrdc9ai/verdict.json` and implementation evidence
at `/tmp/issue713-attempt4-_dmwn7zs/`. Reuse only evidence whose exact source and
applicability are proved after integration; this child must earn its own gates.

## Smallest closable product slice

At the existing post-execution observation point, offer each eligible observer
an immutable view of its actual final bank-output lane. The built-in meter
adapter accepts it and runs the same meter state machine over strided resident
words. Other observers and all ineligible cases retain planar acquisition.
The child keeps scatter, observer dispatch order, per-meter scalar scans and
all arithmetic. A separate read of resident scratch is still a read: placement
alone does not prove fewer total loads or a faster render.

Do not insert metering as an executing DSP stage, move publication before a
processor completes, batch observers across failure boundaries, precompute or
publish another observer's state, materialize planar meter scratch, or change
bank membership/merge eligibility. Existing `BankChain::run`, #713's
`run_with_resident_input`, and graph `chains_into` contracts remain intact.

## Exact ownership

At the opening checkpoint, only this numbered specification is owned. Concise
replacement updates to the existing #559/#560 disposition and current-lane
paragraphs are coordination work, not implementation authority.

After the dependency, scope approval, preflight, and implementation authorization,
the complete product/test allowlist is:

- `crates/rack/src/lib.rs`: borrowed final-output view and focused rack tests.
- `crates/graph/src/lib.rs`: optional resident-observer hook and focused tests.
- `crates/graph/src/runtime.rs`: admission, safe field borrowing, ordered dispatch,
  physical discriminator, and focused tests.
- `crates/builtins/src/lib.rs`: shared planar/resident meter body and validation.
- `crates/builtins/tests/meter.rs`: differential coverage using the existing corpus.
- `crates/builtins-compiler/src/lib.rs`: built-in meter adapter, real prepared-plan
  integration coverage, and existing resource-boundary checks.
- This numbered specification: scope, attempt ledger, evidence and decisions.

The named compiler adapter path is part of this lane-A child, not a transfer of
compiler-wide ownership. Root records its exact-path availability at activation.
No graph-compiler, `program.rs`, lane kernel, manifest, policy, host, artifact,
pin, benchmark, or generic harness changes are allowed. Keep temporary harness
source/manifests, mutation checkouts, targets and raw evidence outside Git.

## Finite pre-implementation gates

After #713 is integrated, freeze the exact method/type signatures in this record
and complete one bounded preflight before product edits:

1. Demonstrate ordinary safe disjoint borrowing of immutable bank scratch and
   mutable member observer bindings at `Runtime::observe_unit`. The rack view
   contains only borrowed final left/right words and checked frame/width/lane
   shape. Its lifetime cannot outlive the chain borrow. No unsafe, retained
   reference, additional PCM buffer, or callback from inside `BankChain::run`.
2. Freeze a backward-compatible optional method on `GraphRuntimeObserver` that
   returns `None` to decline without mutation and `Some(Result<(), RenderError>)`
   after accepting. The default declines. An accepted error propagates directly;
   it must never cause planar retry. A separate immutable resident block carries
   the checked lane view and first-sample time. Existing `observe` remains intact.
   The builtins adapter bridges to a checked strided meter input without adding
   a builtins-to-rack or builtins-to-graph dependency.
3. Prove final logical right-plane validity after full-prefix and partial-prefix
   collapse, decline and recovery. At the opening baseline, `run` copies left
   into right at the collapse seam and then runs the dual suffix before scatter;
   use the actual final planes, not a previous-collapse flag or a blanket
   left-for-right substitution. The integrated source must retain that proof.
4. Record native and Wasm scalar/SIMD128 size/alignment, retained total, prepare
   peak and largest-allocation deltas for affected owners. The intended delta
   is zero: borrowed views and a default trait method require no retained table,
   flag or scratch. Reconcile existing authoritative estimates and one-byte
   resource-boundary rejections. If any nonzero retained/layout delta or needed
   accounting change cannot be proved within the allowlist, stop and split the
   prerequisite; do not approximate, omit, or expand ownership.

A failed or unresolved preflight is a scope blocker, not permission to improvise
an API, add state or start fixtures. Report the exact seam and required owner.
No implementation attempt is consumed by a read-only scope/preflight review.

## Eligibility and order contract

Graph owns the proof that this unit successfully executed in the current block,
that the observed node/member corresponds to the chain's final output, and that
its active lane and frame range match the view. Rack validates local dimensions
and bounds; it cannot certify graph freshness or observer identity from caller
assertions alone. Keep these responsibilities explicit in code and tests.

The first positive form is a normal emitted bank with an exact final-output
member mapping. Direct and observed-alias bindings at that same final output
remain positive, including multiple meters and mixed meter/custom observers.
An extra planar reader or send does not disqualify an otherwise identical final
output: scatter still occurs. Send transform, crossfeed, delay/PDC and fanout
continue to read their original planar data in their original order.

Scalar units, intermediate members of merged chains, retired/non-emitted units,
shape mismatch, stale prior-block data, and output redirects/route-fold forms
without an established identical output mapping take the ordinary planar path.
No broader eligibility is required by this child. Partial populations are valid
when the exact active lane is proved; inactive lanes and out-of-frame words are
never meter input. No new maximum track count or runtime SIMD dispatch.

Preserve `execute(A) -> observe(A) -> execute(B)`, member ordering, stable-handle
ordering after direct/alias merge, and first-error short circuit. Process failure
prevents observations; observer failure prevents later observers and successor
execution. Accepted resident failure does not retry or mutate twice. Ordinary
fallback acquires the same planar slices and executes the same observer once.
PCM bits, render reports, processor/automation state, bypass, collapse transitions,
command drains, PDC cursors, sends and reductions remain unchanged.

## Meter and realtime invariants

Preserve independent left/right state and increasing sample order per lane:
scalar `f64` square/add energy, RMS publication, D8 comparison/select peak law,
normal-or-zero sanitization, exact clip/sanitize counts, hold/decay and subnormal
flush placement. No energy reassociation, `f32` substitution, new `f64` lanes,
horizontal reduction or cross-channel shortcut.

Preserve all metric selections and disabled-field zeros, silence behavior,
partial/multiple windows, zero-frame input, first-sample/time-overflow validation,
discontinuity and reset state, interval/cumulative counters, generation, sequence,
queue-drop accounting, and publication order. Validate new input shape before
any meter mutation; existing planar validation/error behavior remains unchanged.
The prepared bounded SPSC queues and consumer ownership remain the same.

Render remains allocation/free/lock/syscall/I/O/logging-free, bounded and safe.
No preparation or structural plan mutation moves into observation. The bank view
is read-only and does not expose mutable audio or retained scratch ownership.

## Discriminating evidence and gate order

Reuse existing fixtures; do not grow a second fixture framework. Record exact
source HEAD, owned-file hashes, argv, environment, exits, logs, and limitations
under a fresh external per-attempt directory. Stop at the first unexpected
failure, preserve it, and request the next adversarial verdict; do not retry or
repair within that attempt.

1. Formatting, smallest focused compile, existing `cargo test --locked -p
   builtins --test meter`, and focused rack/graph/builtins-compiler tests first.
   Pause for root's exact-path checkpoint as soon as the coherent tranche
   compiles and focused tests pass; root commits/pushes before broader work.
2. Compare current planar reference and resident candidate for Scalar/W4/W8,
   differing lanes/channels, partial populations, poisoned inactive/right and
   out-of-frame storage, signal/silence, NaN/Inf/subnormals/signed zero, window
   boundaries, all metric selections, reset/discontinuity and queue overflow.
   Pin every snapshot field (float bits), reports and subsequent state, not just
   aggregate PCM. Exercise mono, partial-prefix collapse, decline and recovery.
3. Use a real prepared-plan fixture to compare direct/alias/multiple/mixed
   observers, nonunity crossfeed and delayed-PDC sends, extra readers, scalar and
   retired/intermediate/incompatible controls. An ordered failing-observer trace
   must prove that later meters publish nothing and the successor does not run.
4. A private test-only discriminator must prove accepted production meters obtain
   resident input without the old planar acquisition, while declines still take
   it exactly once. A structural validator must cover GraphExecutor execution/
   observation order and the single intended production dispatch site. Restore
   old planar dispatch in one external physical mutation: semantic/reference
   comparisons must remain meaningful, while the resident-path assertion fails
   specifically. An admission/order-bypass control must fail the structural gate.
   Preserve input/diff/command/output, restore exact source and rerun the decisive
   gate. An unexpected pass or unrelated failure stops the attempt.
5. Run a real prepared-plan render allocation/free gate with a live positive
   detector, both accepted and declined paths, and meter emission/queue overflow.
   Use the existing allocation support from an external harness; setup and drops
   occur outside the counted render interval. No new unsafe allowlist entry.
6. Run affected rack/graph/builtins/builtins-compiler debug and release-unwind
   suites (`cargo test --locked -p <crate>`, then with `--release` and
   `CARGO_PROFILE_RELEASE_PANIC=unwind`), strict all-target/all-feature Clippy
   (`cargo clippy --locked -p <crate> --all-targets --all-features -- -D warnings`),
   and `cargo fmt --all -- --check`. Run `bash scripts/check-<name>-policy.sh`
   and `bash scripts/test-<name>-policy.sh` for each of workspace, rack, graph,
   builtins, lane and realtime; rack/graph/builtins also enforce dependency
   boundaries. Run `bash scripts/check-realtime-audit-leak.sh` and
   `bash scripts/test-realtime-audit-leak.sh`, then
   `bash scripts/check-unfused-seal.sh` and its `--self-test`. Inspect the
   affected dependency closure against `docs/REALTIME_DEPENDENCY_POLICY.md`.
   No policy changes are authorized.
7. For packages rack, graph, builtins and builtins-compiler, run
   `cargo build --locked --release --lib -p rack -p graph -p builtins -p builtins-compiler`
   natively with the unchanged `.cargo/config.toml` x86-64-v3 AVX2/FMA flags.
   Run the same command with `--target wasm32-unknown-unknown` twice, first with
   `RUSTFLAGS='-C target-feature=-simd128'`, then with
   `RUSTFLAGS='-C target-feature=+simd128'`, using distinct external
   `CARGO_TARGET_DIR` values. Run `bash scripts/check-wasm-realtime-atomics.sh`
   with an explicit external target-directory argument and
   `bash scripts/test-wasm-realtime-atomics.sh`. Record exact commands/flags.
   Compilation does not claim target execution.
8. Review exact current base/head, owned-path diff, source identities, resource
   accounting, retained/peak/largest deltas and unchanged dependency contracts.
   Astra LOW reviews the source and evidence; SOURCE PASS is not delivery.
   Later delivery requires exact reviewed-head CI, guarded merge/current-base
   review, successful post-main qualification and synchronized issue closure.
   Artifact disposition, where required, remains exclusively lane B's workflow.

## Successor boundary and acceptance

This child closes only when eligible production built-in meters use resident
input at the unchanged observation point, all declines/errors preserve their
original behavior, and the finite semantic/physical/realtime/resource/target
gates pass. Full RT10 remains open. Across-track vector meter state, optional
scatter/load fusion, RT10/IO8 master-peak consolidation, and performance evidence
belong to separately scoped successors. New `f64` lane arithmetic requires an
owner ruling. No host, AudioWorklet, artifact, pin, timing or benchmark work is
part of this child, and no load-elimination or improvement claim follows from it.

## Decision record and attempt ledger

Opening scope: Astra XHIGH, 2026-09-10. Fetched origin; main remains
`b1f9128f3e06532afdfc16aad661c4b2deb5dea1`. All 349 numbered main specs had a
matching issue number in the 472-entry all-state issue roster; the active #713
and tracker bodies were reconciled read-only. No existing resident-meter child
was found. GitHub atomically assigned #714 on creation, titled
"Observe resident bank output in built-in meters". The matching local file is
`.github/ISSUE_SPECS/714-observe-resident-bank-output-in-built-in-meters.md`;
its body and title/number must be verified against GitHub before reporting the
opening checkpoint. Opening audit evidence is outside Git at
`/tmp/issue-rt10-scope-ue4bs0nj/`.

| Stage | Status | Evidence/next authorization |
|---|---|---|
| Opening scope | Astra LOW PASS at reviewed head `5cff0d4740ccec8f815820743c93fc627a7d3893` | Placement-only scope accepted; #713 integration blocks implementation |
| Dependency/API/resource preflight | Astra XHIGH PREFLIGHT PASS; Astra LOW independent PREFLIGHT PASS | Evidence and frozen-signature findings recorded below; no attempt consumed |
| Implementation attempts 1-5 | Attempts 1-2 FAIL; consumed; Astra LOW verdicts recorded below | Attempt 3 is limited to scoping the call-count validator to the intended observer dispatcher while retaining admission, mutation, and one-dispatch obligations; stop at the first unexpected failure |
| Qualification/delivery | Not started | No product, test, SOURCE PASS, artifact or delivery credit |

Scope review: Astra LOW PASS at `5cff0d4740ccec8f815820743c93fc627a7d3893`,
against main `b1f9128f3e06532afdfc16aad661c4b2deb5dea1` and tracker
`6063cd58d3a966db94d9fb728d422ed860f3a734`. This records scope approval only:
there is no implementation authorization, attempt consumption, or credit. The
#713 dependency, integration gate, lane-B ownership boundary, and all finite
preflight requirements remain unchanged.

No implementation, build, test, mutation, timing, benchmark, artifact or pin work
was performed for the opening scope. Future verdicts and failed gates must be
recorded candidly without overwriting earlier attempt evidence.

## Current integration baseline — #713 dependency delivered

The #713 dependency is delivered through PR #718 at merge
`0df770b0f1f002b563246db70b71a3991fe8a8c7`, with parents `0dc06633` and
`be50921a`. Required run `34453900286` and verdict job `102797510876` passed;
post-main run `34454589139` and verdict job `102799403320` passed at the exact
merge. The current integration baseline is main
`0df770b0f1f002b563246db70b71a3991fe8a8c7`; #713's accepted product/test
identity remains `7576d1b6c794f01df4abeb7256a8309d45879b21`, and RT9 remains
partial. The dependency blocker is cleared, and the read-only API/resource
preflight against this baseline has passed independent Astra LOW review; Attempt
1 is authorized within the existing allowlist and focused gate order. The exact
existing allowlist and all
no-claim boundaries remain unchanged; no product, test, timing, performance,
artifact, pin, SOURCE PASS, or full-RT10 claim is made.

## Astra XHIGH PREFLIGHT PASS — Attempt 1 authorized

Astra XHIGH completed the read-only API/resource preflight at evidence root
`/tmp/issue714-preflight-qxddd3h5`; its manifest SHA-256 is
`f41d01a56c79898f7670bba34d2ffd431351c341b133111c6fe21f5599aa1b71`.
Frozen signatures were taken from `FROZEN_API.md`. The findings retain the safe
borrow ownership, E0506, final-right, default-decline, accepted-error, and lazy
fallback requirements. Native, Wasm scalar, and Wasm SIMD128 resource accounting
recorded zero retained, peak, largest, and allocation deltas. Wasm remains
compile-only for this preflight. Astra LOW independently returned PREFLIGHT
PASS. Zero implementation attempts were consumed; Attempt 1 is explicitly
authorized to Astra XHIGH within the existing allowlist and focused gate order.
This records no artifact, pin, timing, performance, SOURCE PASS, or full-RT10
claim.

## Astra XHIGH ATTEMPT 1 FAIL — Astra LOW verdict

Astra XHIGH's first unexpected focused-gate failure is preserved at evidence
root `/tmp/issue714-attempt1-8ba98eid`; the manifest SHA-256 is
`026876d4642eefd8d46b627e8187f2d7e695cf2a37bd304348b6108a82d59cae`.
Astra LOW independently returned an ATTEMPT 1 FAIL verdict. The exact ordered
commands and exits were:

    rustfmt --edition 2024 --config-path rustfmt.toml --config skip_children=true crates/rack/src/lib.rs crates/graph/src/lib.rs crates/graph/src/runtime.rs crates/builtins/src/lib.rs crates/builtins/tests/meter.rs crates/builtins-compiler/src/lib.rs  # exit 0
    cargo check --locked -p builtins-compiler --lib --features test-support  # exit 0
    cargo test --locked -p builtins --test meter  # exit 0; 9/9 passed
    cargo test --locked -p rack --lib resident_output_lane  # exit 101; E0382

The rack failure is the active `active` borrow after the suffix loop consumed
it with `active.into_boxed_slice()` in the prior suffix iteration. The failure
consumed Attempt 1; no later graph or compiler selectors, broader suites,
policy, mutation, allocation, target, timing, benchmark, artifact, or pin
gates ran. The candid failed checkpoint contains exactly these six authorized
dirty files: `crates/builtins-compiler/src/lib.rs`, `crates/builtins/src/lib.rs`,
`crates/builtins/tests/meter.rs`, `crates/graph/src/lib.rs`,
`crates/graph/src/runtime.rs`, and `crates/rack/src/lib.rs`. It earns no
qualification, delivery, timing, performance, artifact, pin, or full-RT10
credit; the tree is authorized for this candid failed checkpoint.

Attempt 2 is authorized only to clone or reconstruct the fixture's active mask
per suffix, then resume the existing focused gate order. It must stop at the
first unexpected failure. No product or API expansion, artifact, pin, timing,
benchmark, or broader repair is authorized by this record.

## Astra XHIGH ATTEMPT 2 FAIL — Astra LOW verdict

Astra XHIGH's second focused pass is preserved at evidence root
`/tmp/issue714-attempt2-pb4c9_pt`; the manifest SHA-256 is
`7dcfe07119a16378b56fad29f66c45697938a7ffc7f49eae1ef22f38063afc85`.
The one-line rack fixture clone completed; rustfmt exited 0 and the rack
fixture selector exited 0 with 1/1 passed. The graph behavioral test passed,
but the structural-valid-source check failed because its call-count validator
counted an unrelated existing processor `.observe_resident(` in addition to the
intended meter dispatch. No later gate ran and this attempt earns no
qualification, delivery, timing, performance, artifact, pin, or full-RT10
credit. Astra LOW's verdict is that the validator has scoping fragility; the
production checkpoint is unchanged and suitable as a candid failed checkpoint.

Attempt 3 is authorized only to scope the call-count validator to the intended
observer dispatcher while retaining the admission, mutation, and one-dispatch
obligations, then resume the existing focused gates and stop at the first
unexpected failure. No production or API change, artifact, pin, timing,
benchmark, or broader repair is authorized.


## Recovery checkpoint and Attempt 4 authorization

The owner transferred coordination to this session. Astra LOW implements;
Astra XHIGH scopes and verifies, superseding earlier prospective model routing.
The five-attempt policy delivered in #707 applies; Attempts 1–3 remain consumed.

Independent Astra XHIGH recovery review confirmed Attempt 3 FAIL from
`/tmp/issue714-attempt3-4eobg44i/VERDICT.md`. The preserved validator correction
passed graph resident-meter tests (2/2) and the execution-order selector (1/1),
then the builtins-compiler fixture failed E0616 on private `PreparedGraphPlan.routes`.
No later gate ran. Current runtime.rs matches the preserved implementation diff;
all six owned source hashes and manifest entries verify. Manifest SHA-256:
`2a3d5642a89ca6feb9776c88c9969e208ff4baab9367a784d0a305ce3dc275fa`.
This is a candid failed checkpoint, with no source or delivery PASS.

Attempt 4 may change only the private builtins-compiler fixture: supply its
nonunity crossfeed transform during PreparedGraphPlanParts construction through
a private parameterized helper, preserving the old wrapper's default behavior,
and remove the illegal post-construction private-field mutation. Public API and
production behavior stay unchanged. Run formatting and the existing compiler
resident-meter selector first; checkpoint the coherent correction before
continuing the original finite qualification gates. Stop at first unexpected
failure and preserve its result. Astra XHIGH independently reviews the attempt.

## Attempt 4 focused checkpoint

Astra LOW repaired only private compiler fixture construction: the route
transform is supplied through PreparedGraphPlanParts; existing fixture callers
retain their default transform. Formatting passed and
`cargo test --locked -p builtins-compiler --features test-support --lib resident_meter`
passed both tests. Exact commands, exits, source identity, diff and logs are
preserved in `/tmp/issue714-attempt4-1kttzoal/`. This is a focused checkpoint,
not SOURCE PASS. Astra XHIGH now owns adversarial verification under the user's
latest routing; the original remaining finite gates and stop-on-failure rule apply.

## Attempt 4 verification incomplete; fifth attempt bounded

Astra XHIGH stopped on an external mutation-driver setup assertion (exit 1):
`let resident = if eligible` occurs three times in runtime.rs because test
strings repeat the production token. The driver incorrectly required one global
occurrence. No product failure was demonstrated. Evidence and exact failed
command are `/tmp/issue714-attempt4-review-tll88otr/VERDICT.md` and adjacent
`unexpected-setup-command.py`, status, logs and final-source record.

Independent tests passed: builtins meter 9/9, rack resident output 1/1, graph
resident meter 2/2, compiler resident meter 2/2 and actual executor structural
selector 1/1. An extra guessed selector selected zero tests and earns no credit.
The physical planar mutation retained 9/9 meter semantics and failed the intended
acquisition assertion with `[1,0,0]` rather than `[0,1,1]`. Shared source stayed
clean; the external source was restored exactly before the setup failure.
Restored decisive reruns, allocation, remaining policies/suites/targets and
resource reconciliation are incomplete. No SOURCE PASS or delivery credit.

Attempt 5 is the final attempt under the user-approved ceiling. Root authorizes
only external verification-driver correction and completion of the original
finite gates on unchanged product source. Scope admission mutation to the actual
production observe_unit body, excluding embedded validator strings; require an
exact single-site diff and restoration. Preflight actual selectors, script
arguments and the existing-support allocation harness before executing their
gates. Reuse existing fixtures and resource probes; no new evidence framework,
production correction, policy weakening, timing or artifact work. Stop at the
first unexpected failure, preserve it and return an adversarial verdict. A fifth
failure requires a candid hard stop and bounded rescope, never a sixth retry.

## Attempt 5 FAIL — frozen at the hard stop

Astra XHIGH stopped at the required realtime policy gate on unchanged product
source `b0f80bc4fbf39c7a2121daa6caa3b9d4eb7283f9` (product hashes equal
ad9685fc). `bash scripts/check-realtime-policy.sh` exited 1:
`crates/graph/src/runtime.rs has unmatched realtime policy markers`. The
structural fixture embeds two complete END-marker strings at lines 4144/4152;
the raw scanner counts eight BEGIN and ten END lines, despite eight actual
paired regions. No render arithmetic or allocation failure is demonstrated.

Evidence: `/tmp/issue714-attempt5-review-aj9srv1e/VERDICT.md`, exact per-command
receipts/logs and source copies; manifest SHA-256
471ff0b7722872d670114f0499996bd379d3f2681cd889153f26dd237e616848.
Earned gates: admission/order physical negative controls and exact restored
reruns; live detector and accepted/declined prepared renders with zero
allocations/frees, eight queued plus one later publication and two drops;
268 tests in each debug/release-unwind profile; all four strict Clippy suites;
formatting; workspace/rack/graph/builtins/lane policies and self-tests.

Realtime self-tests, audit-leak, unfused seal, native/Wasm build/atomics and
current candidate layout measurements did not run after the failure. Fifteen
retained owner definitions and preparation allocation contracts remain unchanged,
but this does not grant the unexecuted gates. No SOURCE PASS or delivery.

All five attempts are consumed. Implementation is frozen; no sixth repair or
retry is authorized. An independently scoped, bounded successor may correct
only the structural fixture delimiter expressions while preserving production
and policy behavior, then earn its stated remaining qualification. Preserve
all earlier failures and gate evidence; do not silently restart this feature.

## Superseded by bounded qualification successor #722

#714 remains exhausted at five failed attempts. #722 owns only the two
structural-test delimiter corrections and completion of inherited qualification
and delivery. This issue is closed as superseded, not delivered; no capability
or parent-finding credit is awarded until #722 actually delivers.
