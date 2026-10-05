# Add miso_engine_v1_service for bounded control work between edits

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The engine owns no thread, so the host drives control work. A C host calls
`miso_engine_v1_service(session)` from its control thread whenever it likes (for example once per
UI frame), and every other session control call performs the same work first. Each call does a
bounded amount: it reclaims retired plans, brings provider epochs up to render, refreshes the
session counters, and stages render telemetry. It never takes or drops a pending candidate. The
warm successor's deadline check joins the same step later (D7), so a host that services keeps
every pending edit progressing without ever waiting on render. The browser Worker
calls the same engine-side step.

## Context

- Today the step exists in pieces. `synchronize_plan_epochs`
  (`crates/control-plane/src/control.rs:920-961`) promotes a pending provider when render has
  adopted its plan and reclaims retired plans (`retirer.try_reclaim`, loop at `:926-942`;
  `PlanRetirer::try_reclaim`, `crates/engine/src/realtime/plan_exchange.rs:512`).
  `collect_render_activity` (`control.rs:625`) stages the render peak and counter telemetry.
  `command` then refreshes the provider's telemetry counters from the controller's queues
  (`set_telemetry_counters`, `control.rs:986-989`); no other call does.
- Two specs rely on that refresh running on every control call: *Hold live values in
  latest-target cells on both hosts* (#1312, D8: `live_values_superseded`) and *Report each
  configured counter's own value in the C ABI counter snapshot* (#1351, D3: the snapshot reads
  the refreshed values).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311) lets a candidate wait in its
  mailbox cell, `Full` and withdrawable, until a due sample (`NoEarlierThan`) or until render finds
  it ready (`Primed`). Render adopts in the same step as it claims, so nothing is ever handed back
  to control.
- Which control calls run them: `command` runs both (`:984-985`); `dequeue_event` runs both
  (`:1536-1541`); `submit` (`:1646`), `seek` (`:1661`) and `seek_at` (`:1678`) run only the
  epoch synchronization. A host that renders but makes no control call never reclaims a retired
  plan, so the plan's memory stays allocated until its next call, and with retirement capacity 1
  (`crates/control-plane/src/compile.rs:759-764`) the next swap waits for that call.
- The engine handle owns no session (`crates/capi/src/abi.rs:364-369`); the control state is the
  session's `SessionState`. Decision 15 records that the entry point takes the session handle (the
  agreed plan's `miso_engine_v1_service(engine)` spelling is superseded).
- Session thread rule: `crates/capi/include/miso_engine_v1.h:18-22`.
- The control path moves to `crates/control-plane` under
  *Extract the C ABI control plane into a portable crate both hosts call* (#1309, D1-D2).

## Decisions frozen for this slice

- **D1. One step.** `SessionState::service(&mut self) -> Result<(), CommandError>` in the
  control-plane crate runs, in order: `synchronize_plan_epochs` (it never takes a pending
  candidate, D7); the counter refresh (D8); `collect_render_activity`; the counter refresh again,
  so a record that staging coalesced or dropped is counted in the same call.
  It is `pub`: it is the engine-side hook the browser Worker calls
  (*Swap and retire browser plans through the Worker's service loop*, #1381).
- **D2. Every control call services.** `command`, `dequeue_event`, `submit`, `seek`, `seek_at` (and
  any session query added later, such as #1316's seek report) call `service()` first, replacing
  their individual calls. No call services twice.
- **D3. Bounded.** One call reclaims at most the retirement queue's capacity of plans and stages
  at most one render observation; it never loops on render progress and never waits. Its cost does
  not depend on how long the host went without calling. The deadline check (D7) keeps this bound:
  it compares two sample counts, and only a passed deadline adds one re-preparation of the
  pending candidate's session.
- **D4. C ABI.** `uint32_t miso_engine_v1_service(miso_engine_v1_session *session)`. Thread:
  session, serialized with the other session calls (added to the list at `miso_engine_v1.h:18-22`).
  Results: `OK`; a dead or wrong handle as every session call; an internal epoch failure as
  `MISO_ENGINE_V1_INTERNAL` with the session diagnostic the other calls already set for it
  (`SourceFailure::Internal`'s text). Feature bit `MISO_ENGINE_V1_FEATURE_SERVICE`: the next free
  bit when this merges, never a reused one; tests and the smoke program use the symbol, never a
  number.
- **D5. Duty text** in the header and `C_ABI_V1_QUALIFICATION.md`: a pending edit progresses only
  while the host makes control calls, the same duty as draining events. A host that renders but
  never services keeps retired plans allocated, and a warm successor that misses its deadline
  falls back only at its next control call; the watermark (#1314) shows it. Call `service` at least
  once per render-buffer period while edits are pending.
- **D6. Acked-batch question: can an ack ever precede a drop? No.** Service commits nothing and
  acknowledges nothing, and adds no queue. It discards nothing a host was acked for: retired plans
  are already displaced, and a pending candidate is kept (D7).
- **D7. Pending candidates and the deadline check.** Neither `synchronize_plan_epochs` nor any other
  part of this step takes a pending candidate: it stays in its cell with its plan, epoch, revision
  word and retirement credit until render adopts it or the control plane withdraws it (#1311 D2,
  D6). The only service work on a pending candidate is the warm successor's deadline check: when a
  `Primed` candidate is still pending `prime_deadline_samples` of render after its `not_before`
  (#1358 D1), the step withdraws it and publishes the transition instead (`Taken` means it was
  adopted exactly). The check and the fallback belong to *Fall back to the transition when a warm
  successor is not ready by its deadline* (#1358) and *Check the warm-successor deadline in
  miso_engine_v1_service and report its outcome* (#1360); they run inside this step, after
  `synchronize_plan_epochs`. The revision stays committed, and the watermark (#1314) reports it when
  render adopts the candidate or its replacement.
- **D8. Counter refresh.** The refresh moves `command`'s `set_telemetry_counters` call
  (`control.rs:986-989`) into service, so every control call runs it. It sets every counter the
  provider serves from a source outside the provider: the controller's telemetry counters today,
  and `live_values_superseded` once #1312 lands (#1312 D8 and #1351 D3 call this the control
  plane's refresh). A counter added later joins this refresh, not a single call.

## Deliverables

1. D1-D3, D7's no-take rule and D8 in the control path; D4 export, constant and bit; `abi_smoke.c` and `header_smoke.cpp`;
   the frozen symbol list; D5 text.

## Authorized paths

- `crates/control-plane/src/control.rs` (or `crates/capi/src/runtime/control.rs` if #1309 has not
  landed)
- `crates/capi/src/abi.rs`, `ffi.rs`, `lib.rs`, `runtime/tests.rs`, `include/miso_engine_v1.h`,
  `tests/c/abi_smoke.c`, `tests/c/header_smoke.cpp`
- `scripts/check-capi-abi.sh` (frozen list only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- The warm successor's deadline check and fallback (#1358, #1360; the browser's is *Check the
  warm-successor deadline in the browser Worker's service loop and report its outcome*, #1361).
  The browser Worker loop itself (#1381). A thread owned by the engine.

## Objective gates

1. **Service reclaims.** C ABI test: commit a rebuild, render the swap block, then call only
   `miso_engine_v1_service`: the retired plan is disposed (the `current_plan_disposed` test
   counter grows by 1) and the provider epoch is promoted; a second rebuild then reserves without
   `BACKPRESSURE`.
2. **Every call services.** Same setup, but call `miso_engine_v1_source_submit_planar_f32` instead
   of `service`: with a meter handle configured, the retired plan is disposed and a render-peak meter record is staged, as with
   `service`.
3. **Counters refresh on every call.** Configure a lossy telemetry lane to drop records, render
   blocks, then call only `miso_engine_v1_source_submit_planar_f32`. Right after each submit, the
   provider's counter snapshot (read through a `test-support` observer, with no other control call
   in between) reports the drops counted by that submit's service step, equal to the controller's
   own count; a final `COUNTERS_GET` for `TELEMETRY_DROPPED` reports the same value end to end.
   *Red if the refresh stays in `command` only, or if the step's second refresh is missing.*
   (Amendment 1: `COUNTERS_GET` is itself a command and services first, so it alone cannot see a
   refresh missing from `submit`.)
4. **A pending candidate survives service (control-plane test, `test-support`).** Publish a
   rebuild, through a `test-support` publication hook, as `NoEarlierThan(S)` with `S` 64 blocks
   ahead, render one block, then call `service`
   1,000 times with no render: the candidate's cell stays `Full`, its provider stays pending, no
   plan is disposed, and retirement credits balance. Render up to `S`: the candidate is adopted at
   the block that starts at `S`, and the next `service` promotes its provider.
5. **Bounded and idle-safe.** `service` on a session with nothing pending returns `OK` and changes
   no counter; 1,000 consecutive calls with no render change nothing.
6. **ABI.** `abi_smoke.c` tests `MISO_ENGINE_V1_FEATURE_SERVICE` with `&` and calls
   `service(NULL)` → `INVALID_ARGUMENT`; frozen list grows by one.
7. Commands:
   - `cargo test --locked -p capi` and `cargo test --locked -p control-plane --features test-support`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: red if `service` does not reclaim or does not promote (the second rebuild would then hit
  the pending-candidate refusal); no call exists today that does only this work.
- Gate 2: red if a control call skips the telemetry half of the step, as `submit` does today.
- Gate 3: red if a control call other than `command` leaves the provider's counters stale, which
  #1312 and #1351 would then report wrongly.
- Gate 4: red if `synchronize_plan_epochs` treats an unadopted scheduled candidate as stale and
  disposes of it or promotes its provider early (an acked revision's plan lost, or the control
  plane addressing a plan render does not run).
- Gate 5: red if the step does work that grows with idle time or mutates state with nothing pending.

## Amendment 1 (root, 2026-10-05)

Gate 3's red claim could not hold as first written: `COUNTERS_GET` is a command, every command
services first (D2), so it always reports refreshed counters even when `submit` skips the
refresh. Root ruled that the verifier decides, and the attempt-1 verifier confirmed it. Gate 3 now
reads the provider's counter snapshot through a `test-support` observer right after each submit,
and keeps the end-to-end `COUNTERS_GET` check; the refresh-in-`command`-only mutant turns it red.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the duty
  text's reference to the watermark.
- *Adopt a successor plan no earlier than a scheduled sample* (#1311): the scheduled candidate
  that D7 and gate 4 rely on.

## Attempt record

### Attempt 1 (implementer, branch `codex/d15-stream-b`, base `efad0080a`)

**What changed.**

- `crates/control-plane/src/control.rs`: `pub fn SessionState::service(&mut self) -> Result<(),
  CommandError>` runs `synchronize_plan_epochs`, the counter refresh, `collect_render_activity`
  and the counter refresh again (D1). `refresh_provider_counters` is `command`'s former
  `set_telemetry_counters` block, moved (D8). `command`, `dequeue_event`, `submit`, `seek` and
  `seek_at` call `service()` first in place of their individual calls; each services exactly once
  (D2). `command`'s and `commit_live`'s fallible-check order is unchanged: `service()?` replaces
  the same leading lines. `dequeue_event` keeps its error mapping and the source calls keep
  `SourceFailure::Internal`. `synchronize_plan_epochs` itself is unchanged: it promotes a provider
  only on `active_epoch` or a reclaimed plan, so it never takes a pending candidate (D7).
  `test-support` gains `test_set_next_adoption` (the gate-4 publication hook: the next structural
  publication reserves with that `PlanAdoption` instead of `Next`, taken once, cleared by
  `test_reset_lifecycle_observer`) and `test_provider_counters` (the provider's counter snapshot
  read without a control call). The superseded `test_synchronize_plan_epochs` hook is deleted; its
  two call sites now call `service()`.
- `crates/capi`: `miso_engine_v1_service(session)` (`ffi.rs`), `FEATURE_SERVICE = 1 << 7` (the
  next free bit; mask `0xff`) in `abi.rs`, the header's prototype, define, session thread list and
  "Control service" duty paragraph (D4, D5), `abi_smoke.c` (bit tested with `&`, `service(NULL)`
  -> `INVALID_ARGUMENT`, exit 6), `header_smoke.cpp` (bit in mask, signature), five runtime tests.
- `scripts/check-capi-abi.sh`: frozen list grows by one (17 symbols).
- `docs/C_ABI_V1_QUALIFICATION.md`: the amendment, bound, D6 answer and duty text.

**D3 bound, by construction.** The reclaim loop drains the retirement queue. Each retired plan
holds one retirement credit (#1343), the credits number the queue's capacity, and only a
publication (never `service`) reserves one, so one call reclaims at most that capacity of plans
with no explicit cap. `collect_render_activity` stages at most one observation (it compares one
render sequence word). Nothing loops on render progress or waits.

**D6, acked-batch question: can an ack ever precede a drop? No.** `service` commits nothing,
writes no response, acknowledges nothing and adds no queue. The only things it frees are plans
render already displaced (retirement queue) and nothing else: no `withdraw`, no
`reserve_replacement`, no mailbox call. Gate 4 is the evidence: a committed (acked) scheduled
candidate survives 1,000 service calls and render still adopts it at its sample.

**Gate 4 location.** Gate 4 is a control-plane test run under `test-support`, but the
control-plane crate has no adapter of its own and the authorized paths name no test file in it, so
it lives in `crates/capi/src/runtime/tests.rs` with the other control-plane tests, through the new
`test-support` hook. It renders through the C ABI entry point, because only that path publishes the
render observation (`render_sequence`) a plausible early-promotion defect would key on.

**Gate 3 deviation (spec's red claim).** Read literally, gate 3 cannot turn red when the refresh
stays in `command` only: `COUNTERS_GET` is itself a `command`, and its own service step refreshes
before it answers. The test therefore checks the provider's counter snapshot through the
`test-support` observer right after each submit, with no control call between (red under that
defect), and then checks `COUNTERS_GET` end to end.

**Tests and mutation runs** (`cargo test --locked -p capi --lib`, 81 tests; each defect applied
alone, then reverted; reverted runs all green):

| Test | Named defect | Red under it | Existing tests red |
|---|---|---|---|
| `service_alone_reclaims_the_retired_plan_and_promotes_its_successor` (gate 1) | M1: the export returns `OK` without running the step | gate 1, gate 4, gate 5 tests | none |
| `a_source_submit_services_the_session_first` (gate 2) | M2: `submit` runs only `synchronize_plan_epochs` (the pre-#1348 shape) | gate 2, gate 3 tests | none |
| `every_control_call_refreshes_the_provider_counters` (gate 3) | M3a: the refresh stays in `command` only | gate 3 test only | none |
| same | M3b: the step refreshes only before it stages (second refresh removed) | gate 3 test only | none |
| `a_scheduled_candidate_survives_service_until_render_adopts_it` (gate 4) | M4b: `synchronize_plan_epochs` promotes the pending provider once any block rendered since the last observation (the pre-#1311 "next block adopts" assumption) | gate 4 test only | none |
| same | M4: promote any pending provider unconditionally | gate 4 test | 18 existing tests |
| `service_with_nothing_pending_changes_nothing` (gate 5) | M5: `collect_render_activity` restages the last observation on every call | gate 5, gate 2, gate 3 tests | `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes` |
| same, and `abi_smoke.c` (gate 6) | M6: the export returns `OK` for a null session | gate 5 test; `check-capi-abi.sh` fails (abi-smoke exit 6) | none |

Test-value sentences: gate 1 is red if `service` does not reclaim or promote, which no other test
reaches through the step alone; gate 2 is red if a control call skips the telemetry half (M2);
gate 3 is red if a call other than `command` leaves the provider counters stale or the step does
not refresh after staging (M3a, M3b); gate 4 is red if an unadopted scheduled candidate is treated
as stale (M4b, which every existing reclaim test, all publishing `Next`, misses); gate 5 is red if
the step mutates state with nothing new (M5; also caught by the existing exact-event oracle test,
on the command/dequeue path) or the new export accepts a null session (M6).

**Gates.**

- `cargo test --locked -p capi`: ok (81 lib + 2 + 11 integration).
- `cargo test --locked -p control-plane --features test-support`: ok (the crate has no own tests).
- `bash scripts/check-capi-abi.sh`: ok (shared and static); `--self-test`: ok.
- `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`: 0
  allocations, 0 deallocations, 0 syscalls, `total_violations` 0.
- `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-realtime-policy.sh`: ok.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: ok. `cargo fmt --all --
  --check`: ok.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p control-plane -p capi --no-deps`: no new
  failure (only the known capi `limits_are_valid` link). The workspace run stops first in
  `engine` on two pre-existing failures: the known `watermark.rs` `MAXIMUM_READ_ATTEMPTS` link and
  a third one not on the known list, `spsc.rs:521` linking private `Self::admits` (from #1311,
  `efad0080a`); this change touches neither.
