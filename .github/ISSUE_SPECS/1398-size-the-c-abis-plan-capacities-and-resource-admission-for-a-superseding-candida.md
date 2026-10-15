# Size the C ABI's plan capacities and resource admission for a superseding candidate

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9, D15-17).
Split from *Supersede an unadopted candidate plan by compare-and-swap* (#1310), which depends on
it. Code anchors verified on `main` at `6fb211594`; the C ABI files cited below move into
`crates/control-plane/src/` with *Extract the C ABI control plane into a portable crate both hosts
call* (#1309), and this slice edits them there.

## Product outcome

A structural transaction is never refused because render has swapped plans but not yet published
the new epoch. The C ABI's bookkeeping and resource admission hold every plan and compiled model
that supersession (#1310) or a warm successor's transition re-preparation can keep alive at once: the running plan,
a withdrawn or displaced candidate and its successor, and three compiled models. The header states the sizing rule a host's caps must meet.

## Context

- **The lag refusal.** `if self.shared.active_epoch.load(Ordering::Acquire) < self.providers.epoch
  { return Err(CommandError::Backpressure) }` (`crates/control-plane/src/control.rs:1028-1035`). Its
  comment gives the reason: the retired epoch's report row stays for any-thread readers, so the
  report table (capacity 2, `crates/control-plane/src/compile.rs:768`) is full. The control state is
  already correct in that window: `synchronize_plan_epochs` (`control.rs:920-961`) reclaims the
  retired plan and promotes the pending provider; only the atomic and its row lag.
- **Capacities.** Exchange retirement capacity 1 (`compile.rs:198` for the resource report,
  `:762` for the exchange itself), report rows 2 (`:768`), pending providers 1 and retired
  providers 1 (`:803-810`). Refusals they cause: `control.rs:1097-1110` (`PublicationFull`,
  `RetirementFull`, mapped to `Backpressure`) and `:1128-1132` (report table or pending list full).
  `plan_alive == false` (`:1012-1014`) is a separate refusal.
- **Admission.** `validate_replacement_peak` (`compile.rs:371-449`) checks the newest plan's row
  plus the candidate; `validate_live_peak` (`:483-520`) checks the current and pending rows plus
  the compiled models of `CompiledModelAdmission`.
- The C ABI has no engine default caps: every `CompileLimits` field is caller-chosen and nonzero
  (`all_limits_nonzero`, `crates/capi/src/runtime/mod.rs:111-137`).
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

  Each capacity's resource row (`compile.rs:197-199` and the capi retained rows that count the
  report table and provider lists) follows from its capacity, so the plan resource report states
  the new bytes.
- **D2. Delete the lag refusal.** `control.rs:1028-1035` and its comment go. With D1's rows a
  candidate's row always fits beside the lagging one, and admission reads the providers, never
  the atomic. It is not mapped to another error.
- **D3. Impossible refusals.** With D1, `:1097-1110` and `:1128-1132` cannot fire for a candidate; each
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
  - **Re-preparation peak.** A candidate that the control thread may later prepare again without
    a submit, from the same committed model, is admitted with `AdmissionPeak::WithReprepare`. That
    is a warm successor published `Primed`: when it is not ready by its deadline, the control plane
    withdraws it and prepares the transition instead, keeping the withdrawn candidate as the donor
    of its rings (*Fall back to the transition when a warm successor is not ready by its
    deadline*, #1358; *Duck-swap the strips a latency growth restarts, and fall back to the
    transition when a warm successor cannot adopt*, #1397). Admission then also checks a second
    peak: the running plan plus the prospective row twice (the warm candidate and the transition's
    re-preparation, which #1397 keeps no larger than the warm candidate in any D4 term), with the
    same model terms. Every other held candidate is dropped before a re-preparation can run, so it
    is not in that peak. Both peaks must fit, or the submit is refused before commit with the
    cap's diagnostic. `service` then runs no cross-plan admission. The default,
    `AdmissionPeak::Single`, is today's check; *Classify a latency-growth edit and publish its
    warm successor from the control plane* (#1403) D2 passes `WithReprepare` when it admits a
    warm candidate. Releasing the withdrawn plan's row before re-preparing is not an option: the
    withdrawn plan is the donor of the rings it holds.
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
3. **Re-preparation peak (new control-plane unit test).** One running row `R` and a prospective
   row `C` admitted with `WithReprepare`: with each D4 cap set to `R + 2C - 1` the submit is
   refused with that cap's diagnostic, at `R + 2C` it passes; with `Single`, `R + C` passes. With a
   withdrawn candidate also held, both peaks are checked and the larger one decides.
4. **Reference limits.** `./target/release/audit capi` and the two capi integration tests pass
   with their limits (raised only if needed, D5).
5. Commands:
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
- Gate 3: an admission that checks a warm candidate only against the plans held at submit admits a
  warm candidate whose transition re-preparation (three plans) exceeds the caller's caps, so the
  deadline fallback would be refused and the edit could neither adopt nor fall back.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `e2f6cfb24`.

**Batch order (root, 2026-10-15).** Batch 15a is the controls lane: #1344, #1398, #1310, #1324, in
that order. The rest of batch 15 follows.

**Moved anchors.** #1309 has landed; the files are in `crates/control-plane/src/` as the spec
expects. Each fact in Context still holds; only the lines moved.
- The lag refusal and its comment are `control.rs:1579-1586`. `synchronize_plan_epochs` is
  `:1401`.
- `plan_alive == false` is `control.rs:1541-1543`.
- The reservation refusals are `control.rs:1751-1765`. Since #1343 and #1311 the error has four
  arms: `PublicationFull`, `RetirementFull`, `Incompatible` and `EpochExhausted`. D3 covers the
  first two only; the other two stay as they are.
- The full report table or pending list is `control.rs:1781-1788`. The one-candidate refusal
  `if !self.pending_providers.is_empty()` is `:1742-1744`; it is #1310's, not this slice's.
- Capacities in `compile.rs`: the exchange resource row's retirement capacity `:237-239`; the
  exchange's own `:1121` (`one`, `:1092`); report rows `:1125-1128`; pending providers
  `:1168-1171`; retired providers `:1172-1175`.
- `validate_replacement_peak` is `compile.rs:646`; `validate_live_peak` is `:770`;
  `CompiledModelAdmission` is `:82`.
- `all_limits_nonzero` is `crates/capi/src/runtime/mod.rs:144-169`.
- The three #1042 tests are `crates/capi/src/runtime/tests.rs:1715`, `:1885` and `:2005`; the
  outcome helper is `:1687-1706`. Each still asserts `Backpressure` in the window.
- `audit_limits` is `tools/audit/src/capi.rs:599`. `limits` is `crates/capi/tests/plan_swap_race.rs:56`
  and `crates/capi/tests/resource_lifecycle.rs:176`.

**Code notes for the implementer.**
- D1 holds against #1343. The exchange's `retirement_capacity` now counts retirement credits:
  reserved candidates, unadopted candidates and retired plans together
  (`crates/engine/src/realtime/plan_exchange.rs:70-71`). D1's three are exactly those. Only the
  config value changes; `plan_exchange.rs` does not.
- D4 misses one admission term. #1290a D3 added `ControlLimits::maximum_replacement_peak_bytes`
  (`crates/control-plane/src/adapter.rs:81-87`), checked in `validate_replacement_peak` as
  `control.resource.replacement_peak` (`compile.rs:686-697`). It sums graph, models and
  allocated source bytes. D4's "every held row" rule applies to it too, and gates 2 and 3 cover
  it. The C ABI sets it to `u64::MAX` (`crates/capi/src/runtime/mod.rs:211`); the browser sets
  its one budget (`hosts/host-web/src/lib.rs:5081`, and `set_replacement_peak_limit` at
  `:6347`).
- D4's `maximum_capi_retained_bytes` is `ControlLimits::maximum_control_retained_bytes` in the
  control plane (`adapter.rs:59`); the C ABI maps one to the other (`runtime/mod.rs:199`).

**Decisions.** D1-D4 and D6 hold against the code and decision 17. D5 does not hold as written.
- **NEEDS ROOT RULING: the browser now runs this control plane.** D5 says "The browser's caps
  follow the same rule when it adopts the control plane (#1332)", and the non-goals leave the
  browser's caps to #1332. Since #1381a the browser runs `control_plane::SessionState` in its one
  instance (`hosts/host-web/src/lib.rs:6-16`). So D1's capacities and D4's sums reach the browser
  in this slice, through the same code (decision 17 C1). The browser's limits are its one memory
  budget (`control_limits`, `hosts/host-web/src/lib.rs:5053-5083`). D1's larger tables add fixed
  bytes there, and browser byte pins can move. Proposed: this slice covers both hosts. D5's
  browser sentence becomes: "The browser's caps are its memory budget; its replacement-peak term
  must cover the three-plan peak when #1403 admits a warm candidate." Add to Authorized paths,
  for moved pins only, with the measured reason: `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/` and `hosts/host-web/tests/browser-v1/expected.json`. Gate 4 adds
  `cargo test --locked -p host-web --features host-web/test-support`.

**Dependencies on GitHub.**
- #1309 is closed. #1343 and #1311 are closed.
- Downstream: #1310 (batch 15a). #1403, #1358 and #1397 are open and not in batch 15; they are the
  users of `AdmissionPeak::WithReprepare`, which this slice adds unused.

**Gates and commands.** Every named command, package, feature and script exists. Gate 4's
`./target/release/audit capi` is built by gate 5's `cargo build` line.

**Realtime-path edits.** None under `crates/engine/src/realtime`. The capacities are control-side
configuration (`PlanExchangeConfig`), read at boot.
