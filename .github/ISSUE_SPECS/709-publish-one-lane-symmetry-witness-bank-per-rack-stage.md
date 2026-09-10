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

## Attempt 2 — implementation gates complete; review pending

Astra XHIGH applied only the combined conditional, without suppression, after
Attempt 1. The correction tranche passed formatting, the three focused tests
and strict Clippy (`01-fmt.log`, `02-focused.log`, `03-clippy.log`) before Sol
checkpointed and pushed `dbe7ea7be2bfd26da3cd20084179aa3b4a017bca`.

The complete sequence below was then restarted against that clean, frozen
checkpoint. Source SHA-256 is
`266133f6870f2bf1bfc8d923125605ac840dba9fe3e1ddcb998420a3c822b676`.
All 20 gates completed without retry. Only this decision record was edited
afterward; the product source stayed frozen.

Commands ran from `/home/bl/misofm/engine-rt6-stage-symmetry-witnesses`.
In the table, `E` expands to `/tmp/issue709-attempt2-nTfNNWlH`, `B` to
`898bdc94b0143288049397629f3afeded384f8c2`, and `H` to
`dbe7ea7be2bfd26da3cd20084179aa3b4a017bca`. Unless overridden in a row,
`CARGO_TARGET_DIR="$E/target"`. Logs are under `E`; `full-results.jsonl`
preserves each literal argv, environment override, source commit and exit.

| Command | Result | Log |
| --- | --- | --- |
| `cargo test --locked -p rack stage_witness_bank -- --nocapture` | Exit 0; 3 focused tests | `full-01-focused.log` |
| `cargo test --locked -p rack` | Exit 0; 33 inline + 10 console-bank + 4 re-engagement tests | `full-02-debug.log` |
| `CARGO_PROFILE_RELEASE_PANIC=unwind cargo test --locked -p rack --release` | Exit 0; same 47 tests | `full-03-release.log` |
| `cargo clippy --locked -p rack --all-targets -- -D warnings` | Exit 0; unchanged configuration warnings noted below | `full-04-clippy.log` |
| `cargo fmt --check` | Exit 0 | `full-05-fmt.log` |
| `CARGO_TARGET_DIR="$E/allocation-target" cargo run --offline --release --manifest-path "$E/allocation-harness/Cargo.toml"` | Exit 0; detector liveness and all 24 armed render calls passed | `full-06-allocation.log` |
| `bash scripts/check-workspace-policy.sh` | Exit 0 | `full-07-workspace-policy.log` |
| `bash scripts/check-rack-policy.sh` | Exit 0; dependency/safety boundary | `full-08-rack-policy.log` |
| `bash scripts/check-realtime-policy.sh` | Exit 0; 43 marked regions in 12 files | `full-09-realtime-policy.log` |
| `bash scripts/test-rack-policy.sh` | Exit 0 | `full-10-rack-mutations.log` |
| `bash scripts/test-realtime-policy.sh` | Exit 0 | `full-11-realtime-mutations.log` |
| `bash scripts/check-realtime-audit-leak.sh` | Exit 0 | `full-12-audit-leak-policy.log` |
| `bash scripts/test-realtime-audit-leak.sh` | Exit 0 | `full-13-audit-leak-mutations.log` |
| `cargo build --locked --release -p rack --target x86_64-unknown-linux-gnu` | Exit 0; repository x86-64-v3 AVX2/FMA pin | `full-14-native.log` |
| `CARGO_TARGET_DIR="$E/wasm-scalar" cargo build --locked --release -p rack --target wasm32-unknown-unknown` | Exit 0 | `full-15-wasm-scalar.log` |
| `CARGO_TARGET_DIR="$E/wasm-simd128" CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUSTFLAGS="-C target-feature=+simd128" cargo build --locked --release -p rack --target wasm32-unknown-unknown` | Exit 0 | `full-16-wasm-simd128.log` |
| `CARGO_TARGET_DIR="$E/mutation-target" cargo test --offline --manifest-path "$E/old-dispatch-mutation/Cargo.toml" --lib tests::stage_witness_bank_entire_armed_decision_publishes_once_even_when_declining -- --exact --nocapture` | Expected test exit 101 at the required bulk-call assertion; mutation validator exit 0 | `full-17-old-dispatch-mutation.log` |
| `git rev-parse HEAD`; `git status --porcelain=v1`; `git diff --check "$B" "$H"`; `git diff --name-only "$B" "$H"`; `git diff --stat "$B" "$H"`; `git diff "$B" "$H" -- crates/rack/src/lib.rs .github/ISSUE_SPECS/709-publish-one-lane-symmetry-witness-bank-per-rack-stage.md` | All exits 0; clean exact head, only the two owned paths; complete diff reviewed | `full-18-exact-diff.log` |
| `cargo test --locked -p builtins --test mono_collapse` | Exit 0; 2 existing report/state tests | `full-19-builtin-mono-report.log` |
| `cargo test --locked -p host-core --test effect_observation every_bank_lane_publishes_its_own_reduction -- --exact` | Exit 0; existing per-lane observation test | `full-20-observation.log` |

The implementation publishes one eight-entry witness array per active stage,
bounded by `BankWidth` and the prepared slot mask. Existing stage overrides
remain the default's source; unclassified stages still decline. The armed
path publishes after all drains and reuses that aggregate for eligibility and
agreement maintenance. The forced-off path retains its lazy publication after
disengagement. The former activity-count test now expects ten constant slot
checks rather than eight: the two additional checks guard stage publications;
its zero lane-mask-inspection assertion remains intact.

The three inline tests cover exact terms, public evidence-helper results,
full/partial W4/W8 cohorts, identity and inactive lanes, default/external-style
overrides, all-drains-before-publication, and the complete armed eligible,
`UNBYPASSED`-declined and `LIVE`-declined decisions. Each visited stage is
asserted to publish once across the whole decision, with no direct outer
per-lane query. The external mutation restores lane/slot virtual calls only
in the aggregate; its single run compiled and failed specifically because
bulk counts were `[0, 0, 0]` instead of `[1, 1, 0]`. The authoritative checkout
was never mutated. Original/mutated sources, manifest and exact diff are under
`$E/old-dispatch-mutation/`; `mutation-input-sha256.json` records their hashes.
The mutation diff SHA-256 is
`42232d23c7c4d73a60d928f4c7b4dc672fc40ed9d34ebc32b33300664d17636a`.

The external allocation harness uses the public rack API and the inherited
stage bulk default. It first proves alloc/zeroed-alloc/realloc/free detector
liveness (three allocation and three free observations), then runs 12 cases:
W4/W8 × full/partial cohorts × eligible/bypass-declined/live-declined outcomes.
Each case audits its initial collapsed block and its next block after a drain
changes the witness. All 24 armed calls allocate/free zero times, with exact
collapse, agreement and transition assertions. Harness sources, manifest,
lockfile and binaries remain outside Git. Input SHA-256 values are:

- `$E/allocation-harness/Cargo.toml`:
  `b346af2a8a154577fc1e5a77c60e5c62352051a9fe4b76a70d96864c41f19292`.
- `$E/allocation-harness/src/main.rs`:
  `ce3e96abff03e4b047c24a192d3c4a3876c0a7d0423586335dd68ad96efe2e1f`.

Clippy exits successfully but emits the pre-existing unreachable-path
configuration warnings at `clippy.toml:81–82`; no suppression or configuration
change was made. Allocation evidence is limited to the tested render paths;
it is not a syscall trace. The Wasm evidence is crate compilation, not browser
execution. Nested builtin-processor forwarding remains unchanged. There is
no timing, speedup, artifact, pin, merge, delivery or **SOURCE PASS** claim.
Astra LOW's independent adversarial review and Sol's delivery workflow remain
pending; both attempt evidence directories are preserved.

## Attempt 2 — Astra LOW SOURCE PASS

Astra LOW (`/root/issue709_astra_low_scope`) returned **SOURCE PASS** at exact
clean pushed head `ac6cb00ebf2a53ac65ca01a7c6b03855421fe3fb` for product source
`dbe7ea7be2bfd26da3cd20084179aa3b4a017bca`. The reviewer verified upstream
identity, exact GitHub body parity, both owned paths, the complete Attempt 2
evidence, the allocation harness and the old-dispatch mutation. Fresh review
evidence is preserved at `/tmp/issue709-low-review-99sx3syb`.

With a fresh external target, the reviewer reran:

| Command | Result |
| --- | --- |
| `cargo test --locked -p rack stage_witness_bank -- --nocapture` | Exit 0; all 3 discriminating tests passed |
| `cargo clippy --locked -p rack --all-targets -- -D warnings` | Exit 0 |
| `cargo run --offline --release --manifest-path /tmp/issue709-attempt2-nTfNNWlH/allocation-harness/Cargo.toml` | Exit 0; detector liveness and all 24 allocation/free-free render calls passed |

The adversarial review found no blocker. The defaulted Rust trait method keeps
existing implementations and overrides source-compatible and changes neither
the exported ABI nor the effect contract. The reviewer confirmed width/mask
bounds, conservative defaults, drain ordering, aggregate reuse, forced-off and
recovery ordering, and the mutation gate's exact discrimination.

Sol fetched origin after the verdict. `origin/main` remains
`898bdc94b0143288049397629f3afeded384f8c2`, the issue's original base and the
branch merge base, with no current-main drift. The branch diff remains limited
to the two owned paths. Because `rack` is in the browser artifact dependency
closure through graph/compiler/host composition, retained AudioWorklet identity
cannot be assumed. Lane B alone owns any separately numbered applicability,
qualification and pin action. This lane-A source PASS grants no timing, speedup,
artifact, pin, PR, merge or delivery claim and does not authorize lane A to
touch AudioWorklet state.

## Current-main combined delivery coordination

User routing: Astra LOW implementation, Astra XHIGH scope/verification.
Independent XHIGH scope PASS permits combined delivery with #722 from delivered
main878db254. Admit only the separately accepted #709 and #722 deltas. #709's
rack changes remain owned by #709 and are not repairs or expanded authority
under exhausted #714 or qualification successor #722. All #722 production
additions remain unchanged. Verify the shared rack file as the composition of
accepted deltas; no additional production correction is authorized. Stop on
conflicts or unexpected source drift. Preserve #713 resident-input machinery.

Root checkpoints each history merge separately. Then run the existing rack
suite, meter tests, graph resident/executor selectors and compiler resident
selectors in debug and release-unwind, formatting and realtime policy. Retarget
and rerun only the existing #709 and #722 allocation harnesses once to establish
joint-checkout behavior, including live detectors. No new harness/full campaign.
XHIGH integration PASS precedes one root-owned shared artifact qualification
and PR/main pair. Each issue retains separate acceptance and closure evidence.
No timing, full-RT10, nested-forwarding removal or publication claim.

## Preserved-history integration checkpoint

Accepted #709 history4c165c1f merged cleanly onto delivered878db254. Only the
owned rack/spec paths enter this checkpoint. The admitted source patch matches
the accepted branch patch; #713 resident-input machinery is preserved and
stage witness publication remains after drains in common run_with_input.
No new production correction. Joint integration tests await the separately
checkpointed #722 merge; this is not integration or delivery PASS.

## Joint current-main qualification checkpoint

Joint source cbc3c4f7917d5704396fa904b68e0c5c021da610 passed all twelve focused
commands. Each debug/release-unwind profile passed rack50, meter9, graph
resident2, executor/order1 and compiler resident2 tests. Formatting and realtime
policy passed. Existing #709 allocation harness proved detector3/3 and24
allocation/free-free render calls. Existing #722 harness proved detector1/1,
zero allocations/frees on accepted and declined paths, correct dispatch counts
and queue overflow/publication counters. Copied harness sources/locks are
unchanged; locked offline metadata resolves their local dependencies to the
joint checkout. Actual commands, environments, exits and source identities
are external at `/tmp/joint709722-gates-itstq384/`. No retry or source edit.

Independent XHIGH SOURCE COMPOSITION PASS reconstructs9573fe16 as the disjoint
union of accepted #709 and current-main/#713 edits, then removes the exact
three #722 rack insertion blocks from joint source to recover9573fe16 byte
for byte. Five other files exactly match accepted #722; merge parents and
clean source verified. Final joint receipt review precedes shared artifact
qualification and PR/main delivery.
