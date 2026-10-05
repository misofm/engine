# Size the C ABI's plan capacities and resource admission for a superseding candidate

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Split from *Supersede an unadopted candidate plan by compare-and-swap* (#1310), which depends on
it. Code anchors verified on `main` at `6fb211594`; the C ABI files cited below move into
`crates/control-plane/src/` with *Extract the C ABI control plane into a portable crate both hosts
call* (#1309), and this slice edits them there.

## Product outcome

A structural transaction is never refused because render has swapped plans but not yet published
the new epoch. The C ABI's bookkeeping and resource admission hold every plan and compiled model
that supersession (#1310) can keep alive at once: the running plan, a withdrawn candidate and its
successor, and three compiled models. The header states the sizing rule a host's caps must meet.

## Context

- **The lag refusal.** `if self.shared.active_epoch.load(Ordering::Acquire) < self.providers.epoch
  { return Err(CommandError::Backpressure) }` (`crates/capi/src/runtime/control.rs:892-899`). Its
  comment gives the reason: the retired epoch's report row stays for any-thread readers, so the
  report table (capacity 2, `crates/capi/src/runtime/compile.rs:765`) is full. The control state is
  already correct in that window: `synchronize_plan_epochs` (`control.rs:783-826`) reclaims the
  retired plan and promotes the pending provider; only the atomic and its row lag.
- **Capacities.** Exchange retirement capacity 1 (`compile.rs:170-171` for the resource report,
  `:758-759` for the exchange itself), report rows 2 (`:765`), pending providers 1 and retired
  providers 1 (`:800-807`). Refusals they cause: `control.rs:961-974` (`PublicationFull`,
  `RetirementFull`, mapped to `Backpressure`) and `:992-996` (report table or pending list full).
  `plan_alive == false` (`:876-878`) is a separate refusal.
- **Admission.** `validate_replacement_peak` (`compile.rs:339-417`) checks the newest plan's row
  plus the candidate; `validate_live_peak` (`:443-480`) checks the current and pending rows plus
  the compiled models of `CompiledModelAdmission`.
- The C ABI has no engine default caps: every `CompileLimits` field is caller-chosen and nonzero
  (`all_limits_nonzero`, `compile.rs:482-508`).
- **Tests that pin the refusal.** Three issue-#1042 tests run the two halves of a plan-swapping
  render call apart and make control calls between them (`crates/capi/src/runtime/tests.rs:1472`):
  `control_calls_inside_a_plan_swapping_render_call_keep_replacement_live` (`:1492`),
  `a_valid_edit_inside_the_swap_window_is_backpressured_not_compile_rejected` (`:1663`) and
  `a_rejected_render_call_that_swaps_plans_leaves_the_c_session_live` (`:1784`). Each asserts a
  structural `Backpressure` inside the window.
- *Let the control thread withdraw an unadopted candidate plan* (#1343) replaces the publication
  queue with a two-cell mailbox and gives every candidate a retirement credit (its D6). This slice
  does not touch publication; it sizes retirement.

## Decisions frozen for this slice

- **D1. Capacities.**
  - Report rows 4: a lagging epoch's row, the running plan, a withdrawn candidate, its successor.
  - Pending providers 2: the withdrawn candidate's epoch and its successor's coexist until #1310 D5
    drops the former.
  - Retirement capacity 3 (exchange and resource report alike): a displaced plan not yet
    reclaimed, the withdrawn candidate's credit (held until it is dropped or republished), the
    successor's reservation.
  - Retired providers 2.

  Each capacity's resource row (`compile.rs:169-172` and the capi retained rows that count the
  report table and provider lists) follows from its capacity, so the plan resource report states
  the new bytes.
- **D2. Delete the lag refusal.** `control.rs:892-899` and its comment go. With D1's rows a
  candidate's row always fits beside the lagging one, and admission reads the providers, never
  the atomic. It is not mapped to another error.
- **D3. Impossible refusals.** With D1, `:961-974` and `:992-996` cannot fire for a candidate; each
  becomes `CommandError::Internal`, with a debug assertion. `plan_alive == false` stays
  `BACKPRESSURE`.
- **D4. Admission sums what is held.** `validate_replacement_peak` and `validate_live_peak` take
  every plan row the control plane holds unreclaimed (the running plan and every pending or
  withdrawn candidate) plus the prospective one, and every compiled model the control plane holds
  (including a model #1310 D3 keeps) plus the prospective one. This applies to each row checked
  today: `maximum_graph_session_plus_plan_bytes` (plans plus models, `graph.resource.limit`);
  `maximum_source_total_bytes` and `maximum_source_overhead_bytes` (a carried ring counted once,
  `source.resource.limit`); `maximum_effect_state_bytes` and `maximum_effect_scratch_bytes`
  (`effect.resource.limit`); `maximum_builtin_retained_bytes` and `maximum_capi_retained_bytes`
  (`capi.resource.limit`). The largest-allocation check folds every held row. Before #1310 the
  held set is at most two plans, so no current admission changes.
- **D5. Sizing rule and reference limits.** The header states it: each cap above must be at least
  three times one plan's row, and the graph cap also covers three compiled models. The repository's
  reference limits must admit three plans of their sessions: `audit_limits`
  (`tools/audit/src/capi.rs:579`), `limits` in `crates/capi/tests/plan_swap_race.rs:33` and in
  `crates/capi/tests/resource_lifecycle.rs:160`. Any that does not is raised here. The browser's
  caps follow the same rule when it adopts the control plane (#1332).
- **D6. Acked-batch question.** D2 removes a refusal; the only newly acked edits are ones that fit.
  No queue is added. An ack can never precede a drop.

## Deliverables

1. D1-D4 in `crates/control-plane/src/` (the moved `control.rs` and `compile.rs`).
2. D5's header paragraph and `docs/C_ABI_V1_QUALIFICATION.md` sentence; any raised reference limit.
3. Gates below.

## Authorized paths

- `crates/control-plane/src/` (package `control-plane`, created by #1309).
- `crates/capi/include/miso_engine_v1.h` (prose only), `crates/capi/src/runtime/tests.rs`,
  `crates/capi/tests/plan_swap_race.rs` and `crates/capi/tests/resource_lifecycle.rs` (limits and
  the assertions named below), `tools/audit/src/capi.rs` (limits only),
  `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- Supersession itself (#1310). The mailbox and credits (#1343). The browser's caps (#1332).

## Objective gates

1. **No lag refusal (rewrite the three #1042 tests).** In each of the three tests named in Context,
   the structural edit made inside the window now returns OK, its plan is adopted at the next
   block, and the report table holds three rows meanwhile. Each test keeps its other assertions
   (no `Internal`, no provider parked for good, both source submits land in the carried ring).
   `a_valid_edit_inside_the_swap_window_is_backpressured_not_compile_rejected` is renamed
   `a_valid_edit_inside_the_swap_window_is_admitted_against_the_running_plan`: with its limit
   (nine EQs beside eight fit, nine beside nine do not), the edit inside the window is admitted.
2. **Admission sums every held row (new control-plane unit test).** Three held rows and three
   models, with each cap in D4 one byte below the sum in turn, refuse with that cap's diagnostic;
   at the sum they pass. With one held row the result equals today's for the same inputs.
3. **Reference limits.** `./target/release/audit capi` and the two capi integration tests pass
   with their limits (raised only if needed, D5).
4. Commands:
   - `cargo test --locked -p capi`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-capi-abi.sh`, `bash scripts/check-capi-abi.sh --self-test`
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1: the lag refusal kept, or a report table one row short, refuses the edit in the window;
  an admission that still pairs with the atomic's row compile-rejects it. These three tests are
  rewritten, not added: their old `Backpressure` assertions are the superseded ones.
- Gate 2: an admission that forgets a held plan or model (a missing third term) passes one byte
  over the cap.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
