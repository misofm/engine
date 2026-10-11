PASS

# #1311 attempt 1: verdict (adversarial verifier, opus-xhigh)

Reviewed: `git diff 899db1be7 efad0080a` on `codex/d15-stream-b` (*Adopt a successor plan no
earlier than a scheduled sample*). The commit was exported with `git archive` to
`/tmp/claude-1002/v1311/tree` and built there with `CARGO_TARGET_DIR=/tmp/claude-1002/v1311/target`.
Mutation runs used a second export (`/tmp/claude-1002/v1311/mtree`). The worktree was not touched.

Verdict: no BLOCKER and no MAJOR. The claim rule is correct as built. Render loads the word
(`Acquire`), then the `Full` cell's schedule (three `Relaxed` loads), then asks the **running**
plan's `prime_ready` only for a due `Primed` candidate, then reserves retirement room, then makes
one claim compare-and-swap. A candidate that is not admitted stays `Full` and withdrawable. A
claimed candidate is adopted in the same `enter_block`. Nothing is held or handed back. There are
two MINOR findings. F1 breaks a required CI step and must be fixed before the batch push. F2 is a
test gap on the production render path; the code there is correct. One root item: the
path-authorization deviation, which is purely mechanical.

## Memory-ordering review (the per-cell schedule atomics)

- Writer: `publish_into` stores the revision word, then the schedule (`write_adoption`: kind,
  due, lead, all `Relaxed`), then `commit`'s `Release` compare-and-swap marks the cell `Full`.
  Both product publication paths (`PlanReplacementReservation::commit` and `republish`) go through
  `publish_into`, so every product publication writes the schedule
  (`crates/engine/src/realtime/plan_exchange.rs:435-446`).
- Reader: a word that shows cell `c` `Full` at generation `g` can only have been written by the
  commit compare-and-swap of that publication. The `Acquire` load synchronizes with it, so the
  publication's schedule stores happen before the reader's loads. The loads cannot see an older
  schedule.
- A newer value in `c`'s atomics is always stored after a successful writer transition away from
  `W(g)`. That transition is a withdraw compare-and-swap, which wins only if the reader's claim
  on `W(g)` loses. Both are read-modify-writes on one word with the same expected value, so
  exactly one wins. The other source of a newer value is a later reader claim of the other cell.
  That claim comes after these loads in program order and synchronizes with the writer's next
  `try_reserve`, so a load cannot read it (no load-buffering cycle). Result: any stale or torn
  (mixed A/B) schedule or readiness answer goes with a claim that fails on the generation. This
  is sound.
- Mutation evidence that this ordering is pinned: **L4** (my own; the schedule is stored after
  the `Release` instead of before it). No unit test turns red. The loom tests
  `..._due_check_of_one_publication_never_claims_another` and
  `..._an_unready_candidate_stays_withdrawable` turn red: render sees the cell's earlier `Next`
  schedule and claims an unready or undue candidate. This is the loom tests' unique product catch.

## D-by-D check

- D1: `PlanAdoption::{Next, NoEarlierThan(u64), Primed { not_before, lead_blocks }}`. The cell has
  three atomics (`AtomicU8`, `AtomicU64`, `AtomicU32`) outside the `UnsafeCell` payload. The
  writer stores them before the `Release`. `UnadoptedCandidate::adoption()` keeps the schedule,
  and `republish` writes it back. Met.
- D2: The order is observe, `scheduled`, `admits`, retirement reservation, then one claim
  compare-and-swap. The payload is touched only after a successful claim. There is no spin and no
  retry. Met (`plan_exchange.rs:718-772`).
- D3: The `prime_ready` default is `false` (`plan.rs:329`). The forwarder returns `false` without
  an executor and sits inside the existing `REALTIME_POLICY` region (`plan.rs:935`). It is called
  on `self.active.1` only, and only from `admits`' `Primed` arm after `block_start >= not_before`.
  Met.
- D4: The claim and the adoption (clock adoption, `carry_from`, retirement commit) are in one
  `enter_block` call. Retirement room is reserved before the claim. `Withdrawal::Taken` is reached
  only through a winning claim. Met.
- D5: Both kinds compare with `>=` against the block start. Tested off-grid for `NoEarlierThan(7)`.
- D6: `reserve_replacement(plan, adoption)`. `publish` passes `Next`. The control plane's only
  publication site (`crates/control-plane/src/control.rs:1098-1101`) passes `Next`.
  `set_revision`, `withdraw` and the `Withdrawal` outcomes are unchanged.
- D7 (acked-batch question): **can an ack ever precede a drop? No.** An unadmitted candidate stays
  `Full`. The retirement room is taken before the claim, so a claimed candidate is always
  adopted. Render discards nothing. Under `Next` the control plane's ack path is unchanged.
- Hazard: `render` uses `time.absolute_sample` (`plan_exchange.rs:821`). `render_contiguous` uses
  the running plan's clock (`:791`). For every call that renders, the clock equals the
  `absolute_sample` argument. For a discontinuous call, the clock is the safer choice: a due
  candidate's first rendered block still starts at the clock, which is at or past S. The passed
  sample could adopt early.
- Render cost: three `Relaxed` loads and one closure call only while a cell is `Full`. At most one
  compare-and-swap. No allocation, lock or syscall (audit capi: 0 everywhere).

## Findings

### MINOR F1: `cargo doc -D warnings` fails on a new private intra-doc link (must fix before push)

`crates/engine/src/realtime/spsc.rs:521`: the public `PlanAdoption` doc links
[`Self::admits`], which is `pub(crate)`. The required "Documentation" step
(`qualification.yml:460`) fails with:

```
error: public documentation for `PlanAdoption` links to private item `Self::admits`
```

This is new in #1311. The step already fails on the two known links (`watermark.rs:85`
`MAXIMUM_READ_ATTEMPTS`, #1314; `crates/capi/src/runtime/mod.rs:148` `limits_are_valid`, #1309).
In the mutation export I put all three names in plain backticks, and
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` then exits 0. So #1311 adds
exactly one failure, and nothing more is hidden downstream of it. Fold the one-line fix into the
follow-ups commit with the other two.

### MINOR F2: the production render path's scheduling has no test (`render_contiguous`)

All seven new unit tests render through `render_once`, which calls `RealtimePlanOwner::render`
(explicit time). The production `RealtimePlanOwner` path is `render_contiguous`. The C ABI reaches
it through `control-plane`'s `PlanHandle::render` (`crates/control-plane/src/plan.rs:257-259`). I
changed `render_contiguous`'s block start (`plan_exchange.rs:791`) to `u64::MAX`, and then to `0`.
Under both mutations every test stays green: `engine --lib`, `engine --features realtime-audit`,
`host-core --test successor_swap` and `capi`. So a defect that makes every scheduled candidate
adopt at once over the C ABI, or never adopt, is not caught. Duck-swap (#1324) and two-phase
removal (#1325) will schedule through this path. Add one `render_contiguous` case, for example a
`NoEarlierThan` candidate at an off-grid sample that adopts at the first contiguous block at or
past it. Record that it goes red on the `u64::MAX` and `0` mutations.

### NIT N1: the `render_contiguous` comment says a mismatched call "schedules nothing"

`plan_exchange.rs:789-790`. `enter_block` runs before the `TimeDiscontinuity` check, so a call at
the wrong sample still claims and adopts a candidate that is due by the clock. The behaviour is
correct, because the successor's first rendered block starts at the clock. Only the wording is
wrong. Suggested wording: "a call at another sample renders nothing, so the clock, not the
argument, decides".

### NIT N2: the attempt record overstates L1's unique value

The record says that no existing loom test catches L1 (generation removed). That is true, but the
existing unit test `spsc::tests::stale_claim_after_withdraw_and_republish_fails_on_the_generation`
(`spsc.rs:1648`) catches L1 (re-run: red). The loom tests' unique catches are L2 (model ordering)
and L4 above (the schedule stored after the `Release`). Record L4 in the attempt record.

### NIT N3: `decode` maps any unknown kind to `Primed`

`spsc.rs:577-586`. This cannot happen, because only `encode` writes the kind. An explicit
`ADOPTION_PRIMED` arm with a `debug_assert!` for the wildcard would document that.

### NIT N4: `MailboxPermit::commit` can publish without a schedule

A caller that commits without `write_adoption` inherits the cell's previous schedule (for example
an earlier `Primed`). Both product paths go through `publish_into`, so this is not reachable today.
A future caller of the `pub(crate)` mailbox could reach it. The revision word (#1314) follows the
same pattern. Making the schedule a `commit` argument would rule this out.

### NIT N5: D5 off-grid is tested for `NoEarlierThan` only

All `Primed` tests use a `not_before` that falls on a block start. The comparison is the same
`>=`, so the risk is low.

## Root item (not a finding)

- **Path authorization.** D6's signature change needed six edits outside the authorized paths. I
  read each diff hunk. Each one only adds the argument `engine::realtime::PlanAdoption::Next`, with
  rustfmt reflow in `tools/audit/src/builtins_graph.rs:913-915` and `tools/audit/src/graph.rs:121-124`.
  There is no other edit. The files are `tools/audit/src/{builtins_graph,graph,realtime}.rs`,
  `crates/graph/tests/rt11_swap_carry_alloc.rs`, `crates/host-core/tests/successor_swap.rs` and
  `crates/host-core/tests/support/successor.rs`. The edits are purely mechanical. Root should
  authorize the paths.

## Test value (one sentence per new test)

- `no_earlier_than_adopts_at_the_first_block_at_or_past_its_sample`: a `NoEarlierThan` that ignores
  its sample (M1, re-run red), or a readiness hook asked on the candidate (M7, re-run red) or for a
  `Next`/`NoEarlierThan` candidate (M12).
- `render_schedules_by_the_hosts_time_not_the_plan_clock`: `render` scheduling by the plan clock
  instead of `time.absolute_sample` (M2, re-run red; it is the only red test).
- `a_ready_primed_candidate_adopts_at_not_before`: a hook asked on the candidate (M7, re-run red),
  before `not_before`, or with the wrong lead (M3, M4).
- `an_unready_primed_candidate_waits_published_until_ready`: a claim that ignores readiness (M6,
  re-run red) or a claim-then-hold design that leaves the cell no longer `Full` (M13, re-run red).
- `a_withdrawn_primed_candidate_republishes_with_its_schedule`: a republish that drops the schedule
  (M8, re-run red; it is the only red test).
- `a_no_earlier_than_candidate_withdrawn_before_its_sample_is_never_adopted`: a not-due candidate
  that render already holds, so withdrawal says `Taken` (M13, re-run red), or a due sample that is
  ignored (M1, re-run red).
- `a_plan_without_a_readiness_check_never_adopts_a_primed_candidate`: a default or executor-less
  readiness of `true` (M10, re-run red; it is the only red test), or readiness that is ignored
  (M6, re-run red).
- loom `spsc_loom_plan_mailbox_readiness_of_one_publication_never_claims_another`: a readiness read
  ordered before the observation (L2, model mutation, re-run red; it is the only red loom test).
  It also catches L1, which an existing unit test catches too.
- loom `spsc_loom_plan_mailbox_due_check_of_one_publication_never_claims_another`: a schedule stored
  after the `Release` that marks the cell `Full` (L4, re-run red; no unit test catches it), and L1.
- loom `spsc_loom_plan_mailbox_an_unready_candidate_stays_withdrawable`: the same L4 (re-run red),
  and L3 (`Primed` ignores readiness).

No re-run mutation turned an existing test red, except L1 (see N2).

## Mutation re-runs (applied to the export, reverted after each)

| # | Defect | Result |
|---|---|---|
| M1 | `NoEarlierThan` ignores its sample | 3 new tests red (as recorded); no other test red |
| M6 | `Primed` ignores readiness (the hook is still called) | `an_unready_primed_...`, `a_plan_without_a_readiness_check_...` red |
| M13 | claim at once, hold in render, adopt when admitted | 4 new tests red (withdraw before S gives `Taken`; the unready test sees the cell no longer `Full`); no existing test red |
| M2 | `render` schedules by the plan clock | `render_schedules_by_the_hosts_time_...` red |
| M7 | hook also asked on the claimed candidate | `a_ready_primed_...`, `no_earlier_than_...` red |
| M8 | `republish` publishes `Next` | `a_withdrawn_primed_...` red |
| M10 | default `prime_ready` returns `true` | `a_plan_without_a_readiness_check_...` red |
| L1 | no generation (`advanced()` keeps the word) | loom: readiness and due-check tests red; unit: existing `stale_claim_after_withdraw_and_republish_fails_on_the_generation` red |
| L2 | model reads readiness before its observation | loom readiness test red |
| L4 (new) | schedule stored after the `Release` | loom due-check and unready tests red; all unit tests green |
| R1 (new) | `render_contiguous` block start `u64::MAX` / `0` | **all green** across engine, host-core `successor_swap` and capi (F2) |

## Gates run (export at `efad0080a`)

| Gate | Result |
|---|---|
| 1. `cargo test --locked -p engine --lib realtime` | 50 passed (7 new) |
| 2. loom `CARGO_TARGET_DIR=.../ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib spsc_loom` | 8 passed (3 new), 25 s |
| 3. `cargo build --locked --release -p audit -p capi` and `audit capi` | 100000 calls; allocations, deallocations, locks, syscalls and `total_violations` all 0 |
| 3. `scripts/check-realtime-policy.sh`, `scripts/test-realtime-policy.sh` | ok |
| 4. `cargo test --locked -p engine --features realtime-audit` | 55 + 4 + 1 passed |
| 4. `cargo test --locked -p control-plane --features test-support` | ok (0 tests in the crate) |
| 4. `host-core --features control-provider,test-support --test successor_swap` | 32 passed |
| 4. `graph --features test-support --test rt11_swap_carry_alloc` | 1 passed |
| 4. `cargo test --locked -p capi` | all binaries green (76, 2, 11, ...) |
| 5. `cargo fmt --all -- --check` | ok |
| 5. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | ok |
| 5. `scripts/check-cross-targets.sh` | PASS |
| `scripts/check-workspace-policy.sh` | ok |
| `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | rc 101: one NEW failure (F1), plus the two known ones. With all three patched in the export, rc 0 |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`, V8 spill (self-test and module, node 22.23.2) | all rc 0; shipped module `412df408c9fed15eaf1eb1223ec3676d03a13382f7450f10178c7e8dd3c0d850` (matches the attempt record) |

`scripts/check-capi-abi.sh` was not run: the C ABI surface does not change in this commit.
