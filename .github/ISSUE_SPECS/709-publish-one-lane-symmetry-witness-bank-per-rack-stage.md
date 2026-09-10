# Publish one lane-symmetry witness bank per rack stage

## Authority and smallest closable outcome

Parent: #559 RT6. Companion tracker: #560. Base and current main are
`898bdc94b0143288049397629f3afeded384f8c2`.

Lane A owns this low-level render-path slice. Lane B independently owns #705 and
all AudioWorklet qualification/pin work. #703 is a passive SOURCE PASS parent,
not a second implementation slot. This issue and active lane-B #705 occupy the
two shared implementation slots.

Sol HIGH coordinates. Astra XHIGH implements because this changes render-time
low-level dispatch. A separate Astra LOW reviewer performs scope review and the
adversarial verdict for every implementation attempt. Agents are bounded
assignees and do not own the issue.

The smallest closable outcome replaces the collapse witness's `lanes × slots`
virtual calls at the **`BankChain` → `BankStage` boundary** with one fixed-width
lane-witness publication per visited rack stage, then combines those witnesses
locally. Nested processor forwarding outside `rack` is an unchanged residual.
The issue changes no DSP arithmetic, render ordering, queue semantics, PCM,
public ABI, session schema, effect contract, artifact, pin, benchmark, or
performance budget.

## Exact-path ownership

Product and inline tests:

```text
crates/rack/src/lib.rs
```

Decision record:

```text
.github/ISSUE_SPECS/709-publish-one-lane-symmetry-witness-bank-per-rack-stage.md
```

Coordinator-only concise disposition rows in #559/#560 are synchronized outside
the implementation checkout. No other path is owned. If a correct solution
requires another crate, generated artifact, benchmark framework, policy change,
or public contract, stop and split it into a new stateless issue.

## Current fact pattern

`BankStage::lane_symmetry(lane)` is a virtual query. `BankChain::lane_symmetry`
walks every active slot for one lane, while `all_lanes_symmetric`,
`all_lanes_preserve_agreement`, `symmetry_counters`, and
`active_lane_eligibility` walk lanes through that method. On the armed collapse
path, the dispatch therefore performs a virtual call for each visited
lane/slot pair even though lane width is fixed at four or eight and each stage
already holds event-maintained witness state.

Production `BuiltinStage` implementations outside this issue's ownership can
forward `lane_symmetry` into their own boxed processor traits. #709 does not
remove or measure that nested dispatch. A bulk `BankStage` default must preserve
each existing implementation's `lane_symmetry` override through the concrete
implementation's default-method instantiation; it may not substitute an
unconditional declined bank for already classified external stages.

`BankChain::run` must continue to call every active slot's `begin_block` before
reading a witness. That ordering makes an admitted live-channel record affect
the first sample of the drained block and prevents an addressed one-channel
retarget from being rendered collapsed. Designed terms may remain prepared or
cached; live terms must remain current after the drain.

The original audit location and cost classification are hypotheses, not current
measurements. Acceptance is structural and behavioral. No timing or speedup
claim is authorized.

## Product contract

1. A rack stage publishes the witnesses for the fixed maximum of eight lanes
   through one stage-level `BankStage` virtual query. A private
   `[ChannelSymmetryWitness; 8]` or byte-equivalent representation is acceptable;
   it must not allocate. The supplied valid width and slot-active mask bound the
   publication: W4 never queries lanes 4–7, and identity lanes never cause a
   query that the existing walk skipped.
2. An active collapse decision invokes each visited active `BankStage` at most
   once to obtain its lane witnesses across both eligibility and subsequent
   agreement-preservation evaluation. It must not perform a second aggregate
   query when eligibility declines. The chain combines the single aggregate
   locally for W4, W8, partial cohorts, identity slots, inactive lanes,
   eligibility, and agreement preservation.
3. `begin_block` remains complete and ordered before witness publication. A live
   update drained for block N changes block N's eligibility.
4. Identity slots contribute nothing. Inactive lanes remain declined and never
   become accidental collapse candidates. The stage-level default delegates only
   the supplied valid, active lanes to the implementation's existing
   `lane_symmetry` override, preserving external classified stages. An
   implementation that never classified `lane_symmetry` still inherits its
   conservative declined result.
5. Mono-collapse arming, forced-off behavior, recovery, `channels_agree`,
   desymmetrization, transition counters, gather/scatter selection, fault
   propagation, and observation/report/latency state retain their existing
   behavior and order.
6. Render remains allocation/free, lock, I/O, logging, syscall, and structural
   mutation free. No new data-dependent unbounded work is permitted.
7. Existing public evidence helpers retain their results. A private helper may
   share the aggregate, but no public API expansion is required.

## Attempt and checkpoint discipline

The current five-attempt maximum applies. Each attempt is one coherent
implementation pass followed by one separate Astra LOW adversarial verdict. A
failed prerequisite, compile, test, policy, portability, allocation, behavior,
or evidence gate stops that attempt. No gate may be weakened. After attempt five
fails, preserve evidence and hard-stop this issue; no disguised sixth attempt.

Astra XHIGH edits only the owned product path and this issue record. It stops at
a coherent compiling checkpoint for Sol to audit, commit, and push before any
revision. Raw targets, compiler streams, binaries, and temporary evidence remain
outside Git.

## Objective gates

Before implementation, Astra LOW must confirm exact main/branch/spec/GitHub
identity, exact path ownership, live slot independence from #705, and that the
current source still has the stated virtual-call shape.

Each implementation attempt must provide:

1. Focused W4 and W8 tests proving exact lane-witness results for full and partial
   cohorts, inactive lanes, identity slots, mixed eligible/ineligible stages,
   live-channel drain changes, and conservative defaults.
2. Instrumented private test stages proving the entire armed collapse decision,
   including a declining eligibility result followed by agreement-preservation
   evaluation, uses at most one `BankChain` → `BankStage` bulk virtual call per
   visited active stage. The gate also proves W4 never queries lanes 4–7 and
   identity lanes introduce no query. It must fail under restoration of the old
   outer-boundary `lanes × slots` dispatch shape. Nested builtin-processor
   forwarding is explicitly outside this structural claim.
3. Existing mono-collapse PCM/state/transition/fault tests, including disengage,
   recovery, forced-off, observations, bypass, and exact operation order.
4. A render allocation gate covering the changed armed and declining paths.
5. `cargo test -p rack` in debug and release-unwind, strict Clippy for `rack`,
   formatting and exact diff review.
6. Workspace policy, realtime dependency policy, realtime mutation gates, and
   supported native x86-64-v3 plus Wasm scalar/simd128 compilation appropriate
   to the changed crate.

Astra LOW independently reviews the exact pushed checkpoint, reruns the
claim-discriminating focused gates, and records PASS or FAIL. Source PASS grants
no timing, allocation reduction beyond the tested render gate, artifact, pin,
PR, merge, or delivery claim.

## Delivery boundary

After SOURCE PASS, Sol performs exact-head/current-main review. If the rack
change is in the browser artifact dependency closure, lane B alone owns any
separately numbered AudioWorklet applicability/qualification and pin decision;
this lane-A issue remains passive while that bounded work runs and does not
consume an implementation slot.

Required PR qualification, guarded live head/base merge, post-main
qualification, exact GitHub synchronization, and clean delivered-worktree
removal follow the repository workflow. Failed worktrees, branches, commits,
and evidence are preserved.

## Scope review record

Initial Astra LOW review at pushed brief `757bb598` returned **SCOPE FAIL**
without implementation or attempt consumption. The source fact pattern and
rack-only boundary were valid, but the virtual-call claim accidentally included
nested builtin-processor forwarding outside the owned path. This amendment
limits the claim to `BankChain` → `BankStage`, preserves external overrides,
bounds publication by valid width and the slot-active mask, and requires one
aggregate to serve both eligibility and agreement preservation. Fresh Astra LOW
exact-head review is required before implementation.

## Attempt 1 — FAIL

Astra XHIGH supplied the implementation at clean, pushed source checkpoint
`95af1e99d564a0e8795500c1bb46b916b18de13d`. Raw logs and Cargo targets are
preserved at `/tmp/issue709-attempt1-28LQihgQ`; no timing, performance,
allocation reduction, artifact, or delivery claim is made.

Commands ran from `/home/bl/misofm/engine-rt6-stage-symmetry-witnesses` with
`CARGO_TARGET_DIR=/tmp/issue709-attempt1-28LQihgQ/target`:

| Exact command after the target-directory assignment | Result | Log |
| --- | --- | --- |
| `cargo test --locked -p rack stage_witness_bank -- --nocapture` | Exit 0; all 3 claim-discriminating inline tests passed | `01-focused.log` |
| `cargo test --locked -p rack` | Exit 0; 33 inline, 10 console-bank and 4 re-engagement tests passed | `02-debug.log` |
| `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked -p rack --release` | Exit 0; the same 47 tests passed | `03-release.log` |
| `cargo clippy --locked -p rack --all-targets -- -D warnings` | Exit 101; blocking `clippy::collapsible_if` in the new inline test's `WitnessStage::begin_block`, `crates/rack/src/lib.rs:5092` | `04-clippy.log` |

The attempt stopped at that first failed gate without a fix or retry. The
render-allocation harness, workspace/realtime policies and mutation gates,
native/Wasm portability builds, final formatting gate and exact diff review
were not completed. No allocation-harness files were created.

Astra LOW (`/root/issue709_astra_low_scope`) recorded **Attempt 1 FAIL** on that
clean, pushed checkpoint: the Clippy log confirms the blocker; allocation,
policy and portability qualification remain incomplete; **no SOURCE PASS**.
No additional blocking source finding emerged. The reviewer authorized
Attempt 2 solely to combine the nested conditional without suppression,
update this decision record, and rerun the complete required gate sequence
within the same owned paths, preserving Attempt 1 evidence and stopping at
the first failure.

## Attempt 2 — correction in progress

Sol authorized Astra XHIGH to apply only that conditional correction from
`95af1e99d564a0e8795500c1bb46b916b18de13d`. Attempt 2 evidence is separate at
`/tmp/issue709-attempt2-nTfNNWlH`. The first tranche is limited to
`cargo fmt --check`, the three focused tests and strict rack Clippy, followed
by a pause for Sol's checkpoint before the remaining gates. No verdict or
qualification claim is implied by this authorization.
