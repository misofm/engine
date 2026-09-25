# Skip the permanent observer walk for units without observers

## Product outcome

On the permanent observation path the executor calls `observe_unit` for every unit every block; for a bank it evaluates an O(W) eligibility predicate and iterates every member before discovering that no member has an observer. Record at bind whether a unit has any observer and skip the walk otherwise. Class A.

## Root evidence

- `crates/graph/src/lib.rs:2251` calls `runtime.observe_unit` per unit when no activation is bound; `crates/graph/src/runtime.rs:1915` `observe_unit` walks members and `member.observers`.
- The controlled path (`observe_active_unit`, near `:1848`, delivered by #816) already visits only active entries.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs`, their tests, and this spec.

## Non-goals

No change to what an observer sees or when.

## Objective gates

1. New test: observation output for a plan with observers on some units is identical before/after; a plan with no observers performs zero `observe` calls (counter).
2. `cargo test -p graph` green; `scripts/check-graph-policy.sh`, `scripts/check-realtime-policy.sh`.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Attempt 1 evidence

Implementer: Terra (attempt 1). Branch `codex/900-skip-observer-walk-without-observers`, implementation commit `2cc64e77`.

### Design

`UnitIdentity` gains `observed: bool`, one per unit, in the struct's existing padding (`rt9_identity_metadata_has_no_retained_or_peak_layout_delta` still holds: same size and alignment as the layout without `resident_input` or `observed`, so no retained or reported byte moves). It is derived once, in `Runtime::new_with_observation_activation` -- the one constructor that `build_sequential` and the test-only `Runtime::new` both pass through -- as "any op of this unit holds an observer binding" (`RuntimeUnit::has_observers`, which reads exactly the `op.observers` / `member.observers` slices the walk visits). Any value a caller writes is overwritten there. `Runtime::observe_unit` returns `Ok(())` at its top when the flag is false, before the member walk and the bank resident-eligibility predicate. For such a unit the removed walk had no production effect: it called no observer method, could not return an error, and its only other work was pure reads (`final_output_lane` and the chain getters) plus `test-support` counters.

Nothing recomputes the flag after construction because nothing can change an observer set after construction. `RuntimeOp::observers` is a `Box<[_]>` whose only writer is the struct literal in `build_op` (fed by `take_observers`); no production code assigns, `mem::take`s or `mem::replace`s it; `Runtime::units` is set only by the constructor; and every later mutable walk (`begin_observation_block`, `execute`, `observe_unit`, `observe_active_unit`, `complete_pending`, `invalidate_observers*`, `arm_mono_collapse`, `force_mono_collapse_off`) iterates elements and cannot change a boxed slice's length. The pre-construction passes (`apply_scatter_redirects`, `arm_resident_inputs`) run before the derivation and do not touch observers. A rebind builds a new runtime through the same constructor. `observe_unit` is reached only when no activation endpoint is bound, which is fixed for the executor's lifetime. The controlled path (`observe_active_unit`) is unchanged.

### Tests added

- `runtime::tests::observed_flag_is_derived_at_construction_and_skips_only_unobserved_units`: builds a runtime whose identity placeholders are all the wrong answer (an op with none, an op with two, a bank observed only on its last member, a bank with none) and asserts the derived flags are `[false, true, true, false]`. Per unit it then asserts `[observe calls, observer accesses, bank member accesses, observer visits]` with the skip on and, as the control, with the unconditional walk forced back on.
- `tests::permanent_observer_skip_moves_no_observed_or_rendered_bit` (gate 1, first half): five observer placements on the four-track builtin-bank plan (one input op; the bank's last member only; its first member with resident acceptance; op + bank + output; every node). Also 50 seeded random DAGs (sends, submixes, sidechains, PDC, elided alias stages) with a seeded third of the observable nodes observed. Each case renders twice, once with the unconditional walk and once with the skip. PCM bits, the globally ordered observation log (handle, first sample, resident flag, left/right words), observer accesses and `[planar, resident offered, resident accepted]` meter inputs must be identical. The per-op `observe` count must fall to exactly the ops of the observed units: hand-counted for the bank plan, and derived from `node_op` / `Tap::after_op` for the DAGs. All 50 seeds mix observed and unobserved units, and 101 observers sit on elided alias stages.
- `tests::a_plan_without_observers_makes_zero_observe_calls` (gate 1, second half): the bank plan and eight random DAGs without observers make zero `observe` calls, zero bank-member accesses and zero meter inputs, and their PCM is identical to the unconditional walk's. That walk is the control: it counts every op (9 x blocks on the bank plan).

Instrumentation added: `TEST_ONLY_OBSERVE_CALLS` (incremented at the top of `observe`) and `TEST_ONLY_OBSERVER_SKIP_DISABLED` (ORed into the flag in `observe_unit`). Both are `#[cfg(test)]` only, so they are absent from production and from `test-support` builds.

Mutation controls (each applied alone, then reverted). Every one fails at least one new test:

| Mutation | Failing tests |
| --- | --- |
| Early out removed | all three new tests |
| Bank flag reads the first member only | derivation test, skip test |
| Constructor does not derive (placeholders kept) | derivation test, skip test, existing `aliased_identity_stages_do_not_change_audio` |
| Flag always true | all three new tests |
| Banks never flagged | derivation test, skip test |

### Gates

- `cargo test -p graph`: PASS, 84 lib tests (81 existing + 3 new), plus `rt1_direct_bank_alloc` 1 and doctests 0. `cargo test -p graph --features test-support`: PASS, 84 lib, `rt1` 1, `rt9_resident_bank_input_alloc` 8.
- `bash scripts/check-graph-policy.sh`: `graph policy: PASS`.
- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (50 marked regions in 14 files)`.
- `cargo fmt --all --check`: PASS.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: PASS (exit 0, 157 crates checked, no warnings), run once before the evidence commit.
- `cargo clippy -p graph --all-targets --features test-support -- -D warnings`: PASS.

### Deviations and notes

- No spec anchor drift on this base: `lib.rs:2251`, `runtime.rs:1915` and `:1848` were exact before the change.
- `lib.rs` production code is unchanged. The call site and its source-shape gates (`rt9_resident_entry_has_one_guarded_production_caller_and_control`, `resident_meter_entry_has_one_final_output_dispatch_and_admission_control`) pass untouched. The early out sits above the bank eligibility predicate that #885 edits, and is disjoint from it.
- The two policy scripts are mode 100644 in git, so they were run as `bash scripts/...`.
- `cargo clippy -p graph --all-targets -- -D warnings` without `test-support` fails on `main` too (dead code in `tests/rt9_resident_bank_input_alloc.rs`, which this change does not touch). The workspace gate uses `--all-features` and is unaffected.
- A fully observed plan now pays one load of the identity row `execute` already read this block, plus one predictable branch per unit per block. No arithmetic, order or observer input changed. The failure-only `invalidate_observers*` walks are unchanged.
- No benchmark row is listed, and no saving is claimed.
