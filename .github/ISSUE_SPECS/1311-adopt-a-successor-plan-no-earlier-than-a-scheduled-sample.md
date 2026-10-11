# Adopt a successor plan no earlier than a scheduled sample

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-9; D15-8 (round-5 amendment)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The plan exchange can hold a published candidate until a chosen render sample. A candidate
published "no earlier than S" is adopted at the first block that starts at or after S. A candidate
published `Primed { not_before, lead_blocks }` is adopted at the first block that starts at or
after `not_before` and at which the running plan reports it ready for a prime of `lead_blocks`
blocks. Render checks the due sample and the readiness before it claims, and it adopts in the same
step as the claim, so a candidate is always either unclaimed and withdrawable, or adopted. Nothing
is ever handed back from render. Duck-swap and two-phase removal (stream D) schedule swaps with
this, and the warm successor (stream C) publishes `Primed`.

## Context

- Today the render thread adopts a reserved candidate at the first block after publication
  (`enter_block`, `crates/engine/src/realtime/plan_exchange.rs:375-437`; applied at `:436`). There
  is no notion of a due sample.
- The incoming plan adopts the outgoing plan's clock at the swap (`:414-418`), so "the block that
  starts at S" means `absolute_sample == S` on the host's render timeline, in both
  `render_contiguous` (`:448`) and `render` (`:473`).
- `RealtimePlanOwner` keeps a saturating `carried_count` (`:366`).
- The executor seam is `PreparedPlanExecutor` (`crates/engine/src/realtime/plan.rs:269`). Its
  defaulted hooks (for example `adopt_predecessor`, reached through `carry_from`, `:917`) are how
  the engine asks a graph plan something without depending on graph types.
- *Let the control thread withdraw an unadopted candidate plan* (#1343) replaces publication with a
  two-cell mailbox in `crates/engine/src/realtime/spsc.rs`: one `AtomicU64` state word holds a
  generation and a 2-bit state per cell (`Empty`, `Full`, `Active`). Render claims a `Full` cell by
  one compare-and-swap that makes it `Active` and the old `Active` cell `Empty`. Control withdraws a
  `Full` cell by compare-and-swap: `Withdrawal::{Withdrawn, Taken, Nothing}`. It also removes
  `deferred_count` and `DeferredRetirementFull`.
- D15-8 (round-5 amendment) replaces the off-thread catch-up with prime adoption. It deletes the
  exact-sample adoption ("adopt exactly at S, else return") and the return path this issue once
  carried; the warm successor is published `Primed` instead.

## Decisions frozen for this slice

- **D1. Adoption kind on the reservation.** `PlanAdoption::{Next, NoEarlierThan(u64), Primed {
  not_before: u64, lead_blocks: u32 }}` is fixed when the control thread reserves a replacement and
  stored in the cell beside the plan. `Next` is today's behaviour; every existing caller passes it.
  - The kind, the due sample (`NoEarlierThan`'s S or `Primed`'s `not_before`) and `lead_blocks` are
    three atomics of the cell (`AtomicU8`, `AtomicU64`, `AtomicU32`), outside the `UnsafeCell` that
    holds the plan payload. Control stores them (`Relaxed`) before the state word's `Release` that
    marks the cell `Full`. Render loads them before it claims (D2), while control may withdraw and
    republish into the same cell, so they are never fields of the payload: a plain read of the
    payload before the claim would race with that republish.
  - `UnadoptedCandidate` keeps its `PlanAdoption`. A withdrawn candidate that is republished (#1310
    D4's refusal path) is published with the same kind and due sample.
- **D2. Render decides before it claims.** Render loads the state word (`Acquire`), then the `Full`
  cell's kind, due sample and `lead_blocks` from their atomics (D1; `Relaxed` suffices, because the
  claim below re-checks the generation). It touches the payload only after the claim succeeds.
  - It claims (#1343's one compare-and-swap, generation included) only when the block's start is at
    or past the due sample and, for `Primed`, the readiness hook (D3) returns `true`.
  - If control withdrew and republished between the loads and the claim, the generation changed and
    the claim fails. Render then does nothing this block and looks again next block. Render never
    spins and never claims a candidate that is not due or not ready, so such a candidate stays
    withdrawable.
- **D3. The readiness hook.** `PreparedPlanExecutor` gains `fn prime_ready(&self, block_start:
  u64, lead_blocks: u32) -> bool`, defaulting to `false`; `PreparedRenderPlan` forwards it.
  - Render calls it on the **active** plan, never on the candidate, and only for a `Primed`
    candidate that is due. It runs inside the render scope and must obey render's rules: no
    allocation, free, lock, syscall or unbounded loop.
  - The graph plan's implementation is *Adopt a warm successor with a raw-frame prime at the first
    ready block* (#1355), over *Let a source consumer check and replay its next blocks for a prime*
    (#1320). It reads per-ring `prime_required` atomics that control stores before the `Release`
    that publishes the candidate. A stale read is harmless: a read that belongs to another
    publication goes with a state word of another generation, so the claim fails (D2).
  - The default `false` means a plan that cannot check readiness never adopts a `Primed`
    candidate. Only #1355's slices publish `Primed`, and they publish it only over graph plans.
- **D4. Claim and adoption are one step.** A claimed candidate is adopted in the same `enter_block`
  call, before the block renders: the clock adoption and `carry_from` (`:414-421`), and for
  `Primed` the prime that #1355 runs in that block. There is no in-flight, returned or partial
  state. `Withdrawal::Taken` keeps meaning adopted, and only that.
- **D5. Off-grid schedules.** `NoEarlierThan(S)` and `Primed { not_before: S, .. }` accept any `S`
  and adopt at the first qualifying block whose start is at or past it.
- **D6. Control-side API.** `PlanPublisher::reserve_replacement(plan, adoption)`.
  `PlanPublisher::withdraw()` keeps #1343's outcomes `Withdrawn`, `Taken` and `Nothing`.
  - The control plane threads `PlanAdoption::Next` through its publication.
  - A `Primed` or `NoEarlierThan` candidate is an ordinary `Full` cell for every other rule:
    `PlanPublisher::set_revision` routes to it (#1314 D2), and supersession withdraws it (#1310).
  - The deadline after which a `Primed` candidate that never became ready falls back is control's
    business (*Fall back to the transition when a warm successor is not ready by its deadline*,
    #1358): it withdraws the candidate, and `Taken` there means render adopted it exactly.
- **D7. Acked-batch question: can an ack ever precede a drop? No.** Scheduling changes when a
  candidate is adopted, never whether its content survives. An unclaimed candidate stays in its
  cell, withdrawable (D2). A claimed one is adopted in the same step (D4). Nothing render or the
  service step does discards a candidate.

## Deliverables

1. D1-D5 in `spsc.rs` (the cell atomics), `plan_exchange.rs` and `plan.rs` (the D3 hook and its
   forwarder), with doc comments that state the claim rule.
2. D6 in the control plane.
3. A loom model of the due and readiness check, the claim and withdrawal.

## Authorized paths

- `crates/engine/src/realtime/spsc.rs` (the mailbox state machine and its loom tests),
  `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/plan.rs` (the D3
  hook only), `crates/engine/src/realtime/mod.rs` (exports and its exchange tests).
- `crates/control-plane/src/` (D6 call sites only).
- Amendment 1, the added `PlanAdoption::Next` argument only (D6's signature change):
  `tools/audit/src/{builtins_graph,graph,realtime}.rs`, `crates/graph/tests/rt11_swap_carry_alloc.rs`,
  `crates/host-core/tests/successor_swap.rs`, `crates/host-core/tests/support/successor.rs`.

## Non-goals

- Choosing S for any edit (duck-swap #1324, two-phase removal #1325, warm successor #1287).
- The graph plan's readiness check and the prime (#1320, #1355).
- Supersession (#1310).

## Hazards

- The due-sample load, the readiness check and the claim are separate steps. D2's generation in
  the word is what makes a stale due sample or a stale readiness answer harmless. A design that
  checks after claiming would let render hold a candidate it cannot adopt and make it
  unwithdrawable.
- `render` (`:473`) takes an explicit time: compare against `time.absolute_sample`, not the plan's
  own clock.

## Objective gates

1. **Engine unit tests (new, `crates/engine/src/realtime/mod.rs`).** A test executor whose
   `prime_ready` returns a test-controlled flag and records its arguments:
   - `NoEarlierThan(S)`: blocks before S render the predecessor (`SwapOutcome::None`, its plan ID);
     the first block starting at or after S applies the swap.
   - `Primed { not_before: S, lead_blocks: k }` with the flag `true`: not adopted before S; adopted
     at the first block starting at or after S, and `prime_ready` saw that block's start and `k`.
   - The same with the flag `false` until block `S + 3q`: blocks S to `S + 2q` render the
     predecessor and the candidate stays `Full`; it is adopted at `S + 3q`, the first block after
     the flag turns `true`.
   - A `Primed` candidate withdrawn while unready yields `Withdrawn` with its epoch, plan ID and
     `PlanAdoption` intact; republished, it is adopted with the same `not_before`.
   - `prime_ready` is never called for a `Next` or `NoEarlierThan` candidate, nor for a `Primed`
     one before its `not_before`.
   - A `NoEarlierThan` candidate withdrawn before S is never adopted.
   - A plan whose executor keeps the default `prime_ready` never adopts a `Primed` candidate.
   - Retirement credits balance after each case (reserve, withdraw, drop or republish, adopt).
2. **Loom (new `spsc_loom_plan_mailbox_*` tests in `spsc.rs`).** Render checks the due sample and a
   readiness flag that control stores before its `Release`, and claims, while control withdraws and
   republishes with another kind and another readiness flag: exactly one side wins each race, a
   candidate is adopted at most once, an undue or unready candidate is never claimed, and a
   readiness flag read from one publication never lets render claim another. Command:
   `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
   --locked --release -p engine --lib spsc_loom` (the CI step, `qualification.yml:678`).
3. **Render stays allocation-free and syscall-free.** `cargo build --locked --release -p audit
   -p capi && target/release/audit capi`: allocations, deallocations, locks, syscalls and
   `total_violations` 0. `bash scripts/check-realtime-policy.sh` and
   `bash scripts/test-realtime-policy.sh`.
4. **No behaviour change for `Next`.** `cargo test --locked -p capi`,
   `cargo test --locked -p control-plane --features test-support`,
   `cargo test --locked -p host-core --features control-provider,test-support --test
   successor_swap`, `cargo test --locked -p graph --features test-support --test
   rt11_swap_carry_alloc`, `cargo test --locked -p engine --features realtime-audit`.
5. **Workspace.** `cargo fmt --all -- --check`; `cargo clippy --locked --workspace
   --all-targets --all-features -- -D warnings`; `bash scripts/check-cross-targets.sh`.

## Test value

- Gate 1, `NoEarlierThan`: a swap applied at the first block regardless of S.
- Gate 1, `Primed` unready: a claim that ignores readiness adopts a warm successor whose prime
  frames are missing, so its carried nodes diverge from the predecessor at S.
- Gate 1, readiness arguments: a hook called on the candidate instead of the active plan, or with
  the wrong block start, checks the wrong consumers or the wrong window.
- Gate 1, withdrawn and republished: a republish that drops the kind adopts a warm successor
  without its prime or before its `not_before`.
- Gate 1, default hook: a default of `true` lets a plan that cannot check readiness adopt
  unprimed.
- Gate 2: an ordering bug between the readiness read, the claim and withdraw that only some
  interleavings expose.

## Amendment 1 (root, 2026-10-05)

D6's `reserve_replacement(plan, adoption)` breaks six callers outside the original authorized
paths. Root authorized them for the added `PlanAdoption::Next` argument only (the attempt-1
verifier confirmed each edit is mechanical).

## Dependencies

- *Let the control thread withdraw an unadopted candidate plan* (#1343). This slice follows it in
  `spsc.rs` and `plan_exchange.rs`, and both precede stream C's edits there.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309), for D6.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the cell's
  revision word and the routing rule D6 relies on.

## Attempt record

### Attempt 1 (implementer, Opus 5.5, branch `codex/d15-stream-b` on `899db1be7`)

**What changed.**

- `spsc.rs`: `PlanAdoption::{Next, NoEarlierThan(u64), Primed { not_before, lead_blocks }}` and
  its claim rule `PlanAdoption::admits(block_start, prime_ready)`. Each mailbox cell gains three
  atomics outside the payload (`AtomicU8` kind, `AtomicU64` due sample, `AtomicU32` lead), new
  invariant I8. The writer stores them `Relaxed` into the `Empty` cell through
  `MailboxPermit::write_adoption`, before the `Release` CAS that marks the cell `Full`. Render
  loads them `Relaxed` through `MailboxReader::scheduled(observation)` after its `Acquire`
  observation and before its claim. `MailboxWithdrawal::Withdrawn` now carries the schedule as
  well as the revision word (`Withdrawn(T, u64, PlanAdoption)`); #1314's revision semantics are
  unchanged.
- `plan.rs`: `PreparedPlanExecutor::prime_ready(&self, block_start, lead_blocks) -> bool`, which
  defaults to `false`, and the `PreparedRenderPlan::prime_ready` forwarder, which returns `false`
  without an executor.
- `plan_exchange.rs`: `reserve_replacement(plan, adoption)`. `publish(plan)` publishes `Next`.
  The reservation and `UnadoptedCandidate` keep `adoption()`, and `republish` writes the
  candidate's own schedule back. `enter_block(block_start)` uses this order: observe, then
  `scheduled`, then `admits` (asks the **running** plan's `prime_ready` only for a due `Primed`
  candidate), then the retirement reservation, then the one claim CAS. `render` passes
  `time.absolute_sample`. `render_contiguous` passes the running plan's clock, which is the only
  start a contiguous block can have. A call at another sample renders nothing, so it also
  schedules nothing.
- `mod.rs`: exports `PlanAdoption`; the gate-1 tests (`tests::scheduled_adoption`).
- `crates/control-plane/src/{lib.rs,control.rs}`: D6. The structural publication passes
  `PlanAdoption::Next`.

**Deviation (authorized paths).** D6's signature change `reserve_replacement(plan, adoption)` breaks
six callers outside the authorized paths, and gate 4 and gate 5 must compile them. Each caller
got only the mechanical argument `engine::realtime::PlanAdoption::Next`, with no other edit:
`tools/audit/src/{builtins_graph.rs,graph.rs,realtime.rs}`,
`crates/graph/tests/rt11_swap_carry_alloc.rs`, `crates/host-core/tests/successor_swap.rs` and
`crates/host-core/tests/support/successor.rs`.

**D7: can an ack ever precede a drop? No.** Render claims only after `admits` returns `true` (an
unadmitted candidate stays `Full`; the unit test `an_unready_primed_candidate_waits_published_until_ready`
checks the cell state, and mutation M13 below shows that a claim-then-hold design turns it red).
A claim adopts in the same `enter_block` (no held, returned or partial state exists in the code).
A stale schedule or readiness read loses its claim on the generation (loom L1). Nothing in render
drops a candidate: `Withdrawal::Taken` is reached only through the claim CAS that adopts.

**Gates.**

1. Engine unit tests (`cargo test -p engine --lib realtime`): 7 new tests in
   `realtime::tests::scheduled_adoption`, all pass.
2. Loom (`CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
   --locked --release -p engine --lib spsc_loom`): 8 passed (3 new), about 23 s.
3. Realtime: `target/release/audit capi`: 100000 calls, allocations 0, deallocations 0, locks 0,
   syscalls 0, `total_violations` 0. `check-realtime-policy.sh` and `test-realtime-policy.sh` ok.
4. `Next` regression: `cargo test --locked -p capi` ok (all binaries green). `-p control-plane
   --features test-support` ok (the crate has 0 tests of its own). `host-core ... --test
   successor_swap` 32 passed. `graph ... --test rt11_swap_carry_alloc` 1 passed. `-p engine --features
   realtime-audit` ok (lib 55 passed).
5. Workspace: `cargo fmt --all -- --check` ok. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` ok.
   `scripts/check-cross-targets.sh` PASS. Worklet chain (engine code is browser-compiled):
   `build-web-audioworklet.sh --named-twin` ok (shipped module `412df408...`),
   `check-web-audioworklet.sh` ok, `check-browser-expected-resources.py --artifacts` ok,
   `test-web-audioworklet.sh` ok, V8 spill gate ok. `scripts/check-workspace-policy.sh` ok.
   `scripts/check-realtime-policy.sh` ok (93 regions in 26 files; the new regions are `PlanAdoption`'s
   claim rule and the cell-schedule load and store). `scripts/test-realtime-policy.sh` ok.

**Mutation runs.** For each run, the named defect was applied to the attempt-1 tree and the
listed tests went red. With the defect reverted, every test is green.

| # | Defect | Red tests |
|---|---|---|
| M1 | `NoEarlierThan` ignores its due sample | `no_earlier_than_adopts_at_the_first_block_at_or_past_its_sample`, `render_schedules_by_the_hosts_time_not_the_plan_clock`, `a_no_earlier_than_candidate_withdrawn_before_its_sample_is_never_adopted` |
| M2 | `render` schedules by the plan clock, not `time.absolute_sample` | `render_schedules_by_the_hosts_time_not_the_plan_clock` |
| M3 | `Primed` ignores `not_before` | `a_ready_primed_candidate_adopts_at_not_before`, `an_unready_primed_candidate_waits_published_until_ready`, `a_withdrawn_primed_candidate_republishes_with_its_schedule` |
| M4 | hook gets lead 0 | the same three |
| M5 | hook gets `not_before` as the block start | `an_unready_primed_candidate_waits_published_until_ready` |
| M6 | `Primed` ignores readiness | `a_ready_primed_...`, `an_unready_primed_...`, `a_withdrawn_primed_...`, `a_plan_without_a_readiness_check_never_adopts_a_primed_candidate` |
| M7 | hook also asked on the candidate (claimed plan) | `a_ready_primed_candidate_adopts_at_not_before`, `no_earlier_than_adopts_...` |
| M8 | `republish` drops the kind (publishes `Next`) | `a_withdrawn_primed_candidate_republishes_with_its_schedule` |
| M9 | withdrawal returns `Next` instead of the cell's schedule | `a_withdrawn_primed_...`, `a_no_earlier_than_candidate_withdrawn_...`, `a_plan_without_a_readiness_check_...` |
| M10 | default `prime_ready` returns `true` | `a_plan_without_a_readiness_check_never_adopts_a_primed_candidate` |
| M11 | forwarder returns `true` without an executor | `a_plan_without_a_readiness_check_never_adopts_a_primed_candidate` |
| M12 | `Next` asks the readiness hook | `no_earlier_than_adopts_at_the_first_block_at_or_past_its_sample` |
| M13 | the hazard's design: claim an unadmitted candidate and hold it in render | all 7 new unit tests, including the withdraw-before-S test (`Taken` instead of `Withdrawn`) and the unready test (cell no longer `Full`) |
| L1 | no generation in the word (`advanced()` keeps the word) | loom `..._readiness_of_one_publication_never_claims_another`, `..._due_check_of_one_publication_never_claims_another` (none of the existing loom tests turns red) |
| L2 | render reads readiness before its observation (ordering bug, applied to the model's render step) | loom `..._readiness_of_one_publication_never_claims_another` |
| L3 | `Primed` ignores readiness | loom `..._readiness_of_one_publication_...`, `..._an_unready_candidate_stays_withdrawable` |

**Test value (one sentence each).**

- `no_earlier_than_adopts_...`: a swap applied at the first block regardless of S, or a `Next` or
  `NoEarlierThan` candidate that asks the readiness hook (M1, M12).
- `render_schedules_by_the_hosts_time_...`: a due check against the plan clock instead of the
  host's explicit time (M2).
- `a_ready_primed_...`: a hook called before `not_before`, called on the candidate, or called with
  the wrong lead (M3, M4, M7).
- `an_unready_primed_...`: a claim that ignores readiness, or that is asked with the wrong block
  start, or a candidate claimed early and held (M5, M6, M13).
- `a_withdrawn_primed_...`: a withdrawal or republish that drops the kind or the `not_before`
  (M8, M9).
- `a_no_earlier_than_candidate_withdrawn_...`: a not-yet-due candidate that render already holds,
  so withdrawal reports `Taken` (M13, M1).
- `a_plan_without_a_readiness_check_...`: a default or executor-less readiness of `true` (M10,
  M11).
- Loom models: an ordering or generation bug between the readiness or due read, the claim and
  the withdraw-and-republish, which only some interleavings expose (L1, L2, L3).

**Open items.** None in scope. #1355 implements the graph plan's `prime_ready` and the prime.

### Batch follow-up (2026-10-05)

Folds the attempt-1 verdict (`PASS`; MINOR F1-F2, NIT N1-N3 and N5). N4 (make the schedule a
`commit` argument) and the path-authorization item are root's and are not touched here.

- **F1.** The public `PlanAdoption` doc no longer links the `pub(crate)` `admits`: it says "the
  schedule admits the block" and names `PlanAdoption::admits` in plain backticks.
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` now exits 0 (with #1314's
  two link fixes from the batch follow-up before this one).
- **F2.** `realtime::tests::scheduled_adoption::render_contiguous_adopts_no_earlier_than_at_the_first_block_at_or_past_it`:
  through `render_contiguous`, the C ABI's path, a `NoEarlierThan(7)` candidate renders the
  predecessor at 0, 2, 4 and 6 and adopts at 8. Mutations of `render_contiguous`'s block start in
  `plan_exchange.rs`: `u64::MAX` (every scheduled candidate adopts at once) and `0` (none ever
  adopts): each turns only this test red (engine with `realtime-audit`: 56 passed, 1 failed).
  Reverted: green.
- **N1.** The `render_contiguous` comment now says that a call at another sample renders nothing,
  so the clock, not the argument, decides the adoption, and that a candidate due by the clock is
  still adopted in that call.
- **N2.** Correction to the attempt-1 record above. L1 (no generation in the word) is not
  loom-only: the existing unit test
  `spsc::tests::stale_claim_after_withdraw_and_republish_fails_on_the_generation` (#1343) turns red
  under it too. The D7 paragraph's "loses its claim on the generation (loom L1)" therefore rests on
  that unit test as well. The loom models' own catches are L2 (readiness read ordered before the
  observation, a model mutation) and **L4** (found by the verifier): the cell's schedule stored
  after the `Release` that marks the cell `Full` instead of before it. Under L4 the loom models
  `..._due_check_of_one_publication_never_claims_another` and `..._an_unready_candidate_stays_withdrawable`
  turn red (render sees the cell's earlier `Next` schedule and claims an undue or unready
  candidate) and every unit test stays green. L4 is the defect only loom catches.
- **N3.** `PlanAdoption::decode` has an explicit `ADOPTION_PRIMED` arm. The wildcard, which only a
  broken invariant reaches (only `encode` writes the kind), fires a `debug_assert!` and decodes to
  `NoEarlierThan(u64::MAX)`, a schedule that never admits a block, so such a candidate stays
  published and withdrawable instead of being adopted under an invented schedule. No test: the
  arm is unreachable through the mailbox's API.
- **N5.** `realtime::tests::scheduled_adoption::an_off_grid_primed_candidate_adopts_at_the_first_block_past_not_before`:
  a `Primed` candidate with `not_before` 5 is not asked about or adopted at block 4 and adopts at
  6, where the hook saw `(6, 1)`. Mutation (the `Primed` arm admits at `block_start + 1 >=
  not_before`, an off-by-one invisible to every on-grid `not_before`): only this test red (56
  passed, 1 failed). Reverted: green.

Test value:
- `render_contiguous_adopts_no_earlier_than_...`: `render_contiguous` scheduling by anything but
  the running plan's clock (block start `u64::MAX` or `0`); every engine, host-core and capi test
  was green under both (verifier run R1).
- `an_off_grid_primed_candidate_...`: a `Primed` due check that admits the block before an
  off-grid `not_before`; every on-grid `Primed` test stays green under it.

Gates after the three batch follow-up commits (#1343, #1314, #1311; x86_64 Linux):
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` rc 0;
`cargo test --locked -p engine --features realtime-audit` 57 + 4 + 1; loom (`spsc_loom`) 8 passed;
`cargo test --locked -p capi` 81 + 3 + 11; `host-core --features control-provider,test-support
--test successor_swap` 32; `check-realtime-policy.sh` ok (93 regions in 26 files),
`test-realtime-policy.sh` ok; `audit capi` 0 allocations, deallocations, locks and syscalls,
`total_violations` 0; `check-capi-abi.sh` ok; `cargo fmt --all -- --check` ok; `cargo clippy
--locked --workspace --all-targets --all-features -- -D warnings` ok; `check-cross-targets.sh`
PASS; `check-workspace-policy.sh` ok; worklet chain (named twin build, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`, V8 spill) all rc 0, shipped module
`412df408...d850`, unchanged.

### Batch follow-up 2: N4 (2026-10-05)

- **N4.** `MailboxPermit::commit(value, revision, adoption)` now takes the revision word (I7) and
  the schedule (I8) as arguments and stores both before its `Release` CAS. `write_revision` and
  `write_adoption` are gone, so no publication can skip a store and inherit what an earlier
  publication left in the cell. This supersedes the attempt-1 description above of the schedule
  being stored "through `MailboxPermit::write_adoption`"; the ordering argument is unchanged.
  `publish_into` passes both through; the loom and unit tests pass them explicitly.
- Test: `realtime::spsc::tests::republication_into_a_reused_cell_never_inherits_its_schedule_or_revision`.
  A is published `Primed { not_before: 96, lead_blocks: 2 }` with revision 5 and withdrawn. B reuses
  the same cell with `Next` and revision 0 (the revision of a host that numbers no revisions).
  Render must see `Next`, claim B, and then read revision 0.
  Mutation 1 (commit stores the schedule only when it is not `Next`, so a default schedule inherits
  the cell's old one): only this test is red (54 passed, 1 failed). Reverted: green.
  Mutation 2 (commit stores the revision only when it is not 0): only this test is red (54 passed,
  1 failed). Reverted: green. A blunter mutation (commit never stores the schedule) also turns ten
  `scheduled_adoption` tests red.
- Test value: a publication whose default schedule (`Next`) or default revision (0) falls back to
  the value an earlier publication left in the reused cell. No other test reuses a cell after a
  non-default value and then publishes the default.
- Gates: `cargo test --locked -p engine` green; loom (`spsc_loom`) 8 passed. The batch-end gate
  list is in the commit message of the batch follow-up commits.
