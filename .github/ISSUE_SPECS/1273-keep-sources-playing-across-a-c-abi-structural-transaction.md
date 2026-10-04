# Keep sources playing across a C ABI structural transaction

Slice 4 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`). **Re-verify the `Structural` arm, `ProviderEpoch`,
`prepare_runtime` and the `resource_lifecycle` helpers first if any of #1256, #1257 or #1260 has
landed** (see Coordination).

## Product outcome

On a phone or any C ABI host, adding a track, removing a track or rerouting while audio plays no
longer silences the swap block and no longer resets the sources. Every unchanged source keeps its
ring, generation and position; the host neither seeks nor refills. For a session whose unchanged
paths hold no DSP state, the output is bit-identical to a plan that had the edit from the start.
(Slices 7-14 extend that to every state family.)

## Context

- `SessionState::command` (`crates/capi/src/runtime/control.rs:694`), `Structural` arm (`:718-857`):
  1. `plan_alive`, the epoch-lag backpressure and the response-size check;
  2. `prepare_runtime` (`:748`; `crates/capi/src/runtime/compile.rs:412`), which calls
     `prepare_host_runtime` and builds the `PlanResourceReport`;
  3. `validate_replacement_peak` (`compile.rs:258`) with `compiled_model_admission` (`compile.rs:46`);
  4. backpressure if a candidate is already pending (`control.rs:792`);
  5. `reserve_replacement` (`:795`) and the report-table room check;
  6. the protocol commit `prepared.commit(&mut self.controller)` (`:840-842`), which **can fail**
     (`commit_prepared_structural`, `crates/protocol/src/controller.rs:1944-2004`), then the catalog
     swap, `pending_providers.push`, `reports.push` and `reservation.commit()` (`:843-848`).
- `ProviderEpoch` (`control.rs:9`) pairs an epoch with its `SourceControlSet`.
  `synchronize_plan_epochs` (`:634-676`) promotes a pending provider when its plan becomes active.
  `submit` (`:960`) and `seek` (`:973`) always use `self.providers`, never a pending one.
- `StructuralSourceStatePolicy::ResetAtReplacementBoundary` (`:46-55`) names today's reset;
  `structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic`
  (`crates/capi/src/runtime/tests.rs:600`) drives a `SetSessionId` transaction (`:627`) and pins the
  swap block as all zeros (`:726-741`).
- Exported-C test helpers in `crates/capi/src/runtime/tests.rs`: `submit_c` (`:144`),
  `boxed_c_children` (`:180`), `command_c` (`:198`), `render_parity_shape` (`:394`);
  `crate::ffi::test_source_seek` (`crates/capi/src/ffi.rs:1070`) and `crate::ffi::test_render`
  (`:1089`).
- Race tests: `control_calls_racing_plan_swapping_renders_never_wedge_replacement`
  (`crates/capi/tests/resource_lifecycle.rs:1770`, driver `race_plan_swaps` `:1541-1660`, which
  submits no PCM) and `control_calls_inside_a_plan_swapping_render_call_keep_replacement_live`
  (`tests.rs:851`). Resource oracles: `capi_retained_bytes_charge_every_byte_the_compile_retains`
  (`resource_lifecycle.rs:730`) and `double_live_oracle_drives_exact_and_one_below_c_caps` (`:1263`).
- `audit capi` (`tools/audit/src/capi.rs`) runs its 100,000 render calls inside one
  `in_render_scope` (`:164-188`); CI checks every counter and only the format of `pcm_digest`
  (`.github/workflows/qualification.yml`, the Issue-544 validator).
- *Prepare a successor plan whose unchanged sources keep playing* (#1272) provides
  `PreparedHost::inventory`, `SuccessorBase`, `prepare_host_runtime_successor` and
  `SourceControlSet::adopt_persisting`.

## Decisions frozen for this slice

- **D1. Successor preparation.** `prepare_runtime` gains a `SuccessorBase` built from the newest
  epoch's inventory (`ProviderEpoch` keeps the `PlanStateInventory` of its plan) and the committed
  model **before** the transaction (`self.controller.session().compiled().normalized_model()`). The
  newest epoch is the pending one if any, else the current one; with one pending candidate it is the
  current one. `compile_session` passes no base.
- **D2. Commit order.** Every fallible step runs first, as today. The producer hand-over runs
  **immediately after `prepared.commit(..)` returns `Ok`**, before `replace_session_catalog`:
  `candidate.sources.adopt_persisting(&mut newest.sources)`. It is infallible. No refusal path,
  including a failed protocol commit, moves a producer.
- **D3. Routing.** From the commit on, `submit` and `seek` use the newest epoch's sources (pending if
  any, else current). A persisting source is reached through its moved producer; an added source in
  its new ring, before the swap; a removed source returns `source.id.unknown`.
- **D4. Delete the reset policy.** Remove `StructuralSourceStatePolicy`, its constant and its use at
  `:748`. Replace the pinning assertions at `tests.rs:726-741` with gate 1 (a superseded test is
  replaced in the same PR).
- **D5. Resources.** The candidate's `PlanResourceReport` counts what the plan will own once active:
  the rings it allocated plus the rings it carries. The double-live admission counts each carried
  ring once. Charge each epoch's inventory and the carry program, and move both `resource_lifecycle`
  oracles by exactly those rows; state the numbers in the PR.
- **D6. Documentation.** In the header comment (no symbol or size change),
  `docs/C_ABI_V1_QUALIFICATION.md` and `docs/CONTROL_PROTOCOL_SEMANTICS.md`: unchanged sources keep
  their rings across a structural transaction and must not be reseeked; submits and seeks address
  the newest committed session from the commit on; PCM accepted for a removed source is discarded
  with its plan; an added source starts at generation 1, frame 0, until the host seeks it.

## Deliverables

1. D1-D5 in `crates/capi/src/runtime/control.rs`, `compile.rs`, `mod.rs`; D6.
2. Tests (below). `race_plan_swaps` also submits PCM for every source between renders, so the race
   exercises a carried ring, and alternates `UpsertTrack` and `RemoveTrack` so it never reaches the
   track cap.
3. `audit capi`: apply one structural transaction (`UpsertTrack`, a muted track) between two render
   calls, outside the render scope (split the run into two scopes around it), so one carrying swap
   block is among the audited calls. Read the carry outcome through a `#[doc(hidden)] pub fn` in
   `crates/capi/src/ffi.rs` (not `#[cfg(test)]`: `mod runtime` is private and the crate re-exports only
   `abi` and `ffi`, `crates/capi/src/lib.rs:6-11`); the tool exits nonzero if no swap carried. The record's keys do
   not change; `pcm_digest`'s value moves (CI checks its format only); say so in the PR.

## Authorized paths

- `crates/capi/src/runtime/control.rs`, `compile.rs`, `mod.rs`, `plan.rs`, `tests.rs`
- `crates/capi/src/ffi.rs` (the hidden carry-outcome accessor only)
- `crates/capi/tests/resource_lifecycle.rs`
- `crates/capi/include/miso_engine_v1.h` (comments)
- `tools/audit/src/capi.rs`
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_SEMANTICS.md`

## Non-goals

- No new C symbol, struct field, size or `ABI_VERSION` change.
- No DSP state carry (slices 7-14), no anchored seek (slices 5-6).
- No change to which transactions are structural (#1053's slices own that).

## Objective gates

1. **Gap-free acceptance, through the exported entry points.** Session: tracks `eq0`-`eq2` of
   `parametric-eq-nine-track.json` on `fixture-source`, console sections empty, no inserts, `hpf_hz`
   and `lpf_hz` set to 0.
   - Swapped run: render 6 blocks with `test_render`, feeding a signal with no exact-zero sample
     through `submit_c` one block ahead; submit a `SESSION_TRANSACTION_APPLY` with `UpsertTrack` (a
     muted track whose ID sorts first, on `fixture-source`) and its route; keep feeding; render 6
     more. No seek.
   - Reference: compile the committed post-edit session (from `SessionSnapshotGet`) as a new C
     session, feed the same PCM from frame 0, render 12 blocks.
   - All 12 blocks bit-identical, at 48 kHz and at 96 kHz.
2. **Removed track and source.** `RemoveTrack` plus `RemoveSource` on a two-source session: the
   remaining source's output equals the fresh reference; a submit for the removed source returns
   `MISO_ENGINE_V1_INVALID_ARGUMENT` with `source.id.unknown`.
3. **Commit atomicity.** For every fault phase of
   `every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits` (`tests.rs:1423`), and
   for a new `#[cfg(test)]` fault phase that makes the protocol commit return `Err`, a refused transaction leaves every producer where it was: a
   submit to each source still succeeds and renders through the current plan.
4. **Race.** The two race tests pass with PCM submitted during the swap window landing in the carried
   ring (no `source.frame.noncontiguous`, no `source.ring.vacated` after the swap), using an
   `UpsertTrack` trigger.
5. **Resources.** Both `resource_lifecycle` oracles pass with the D5 rows.
6. **Realtime.** `audit capi` (deliverable 3) reports zero allocations, frees and syscalls and one
   carrying swap.
7. Commands:
   - `cargo test --locked -p capi` and `cargo test --locked -p capi --test resource_lifecycle`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a successor with fresh rings, a producer left in the old set, or a submit routed to the old
  set after the commit turns it red. Today's pinning test asserts the opposite.
- Gate 3: a producer moved before a fallible step, including the protocol commit, orphans the running
  plan's rings; it turns red.
- Gate 4: a submit routed to the current epoch during the swap window lands in a set whose entry is
  vacated; it turns red.

## Coordination

- *Prepare C ABI plans with live track fader and matrix lanes* (#1256) and *Prepare C ABI plans with
  live effect lanes* (#1263) change `prepare_runtime` and `ProviderEpoch`: the successor entry with a
  live-control request (`prepare_host_runtime_with_live_controls_successor`) serves them.
- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257) adds a
  live branch before this slice's successor preparation in the same arm.
- *Commit model-only C ABI transactions without a plan rebuild* (#1260) rewrites the `SetSessionId`
  trigger at `tests.rs:627` and the `command()` helper at `resource_lifecycle.rs:192-209`. Whichever
  lands second rebases; this slice's gates use `UpsertTrack`, which stays structural.

## Dependencies

- *Prepare a successor plan whose unchanged sources keep playing* (#1272).

## Attempt record

### Attempt 1 (implementer, on `2a6f2c10c`)

**Change.** D1-D6 as frozen. `ProviderEpoch` keeps its plan's `PlanStateInventory`;
`prepare_runtime` takes an `Option<SuccessorBase>` (the `Structural` arm passes the newest epoch's
inventory and the pre-transaction committed model; `compile_children` passes `None`) and calls
`prepare_host_runtime_successor` when given one. `adopt_persisting` runs immediately after
`prepared.commit(..)` returns `Ok`, before `replace_session_catalog`. `submit` and `seek` use the
newest epoch (`SessionState::newest_providers_mut`). `StructuralSourceStatePolicy`, its constant and
the swap-block-all-zeros assertions in
`structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic` are deleted. A new
`#[cfg(test)]` fault phase `ProtocolCommit` makes `ObservedPreparedToken::commit` return `Err`.
`crate::ffi::plan_carry_counts` (`#[doc(hidden)] pub unsafe fn`, no exported symbol) reads the
owner's carry counters; a `#[cfg(test)]` wrapper `test_plan_carry_counts` serves the unit tests.

**Resources (D5).** The candidate's report counts carried rings in `source_total_bytes`,
`source_overhead_bytes` and `source_pcm_payload_bytes`, and the carry program in
`graph_session_plus_plan_bytes`; `validate_replacement_peak` subtracts the candidate's carried
total/overhead, so each carried ring counts once; capi's epoch row charges each epoch's inventory
(`inventory_retained_bytes`). Oracle moves: `capi_retained_bytes_charge_every_byte_the_compile_retains`
adds the observed inventory owner to capi's side (and no longer subtracts it from the host half);
`double_live_oracle_drives_exact_and_one_below_c_caps` (soft-clip nine-track, SetSessionId trigger,
which carries the source) subtracts the carried ring from both source rows and adds the successor's
inventory to the capi epoch row. Numbers at x86-64-v3 (eight lanes): carried ring 10,608 bytes
(2,416 overhead), carry program 8 bytes, inventory 62 bytes; double-live requirements: graph
549,774, source-total 13,660, source-overhead 5,468, capi 167,770 (all exact-admitted and refused one
below).

**Gates.**

| Gate | Evidence | Result |
|---|---|---|
| 1 | `runtime::tests::a_c_abi_structural_transaction_keeps_the_source_playing` (exported entries: `submit_c`, `command_c`, `test_render`, `SESSION_SNAPSHOT_GET`), 48 kHz and 96 kHz, 12 blocks bit-identical, carry counts `(1, 0)` | pass |
| 2 | `removing_a_track_and_its_source_keeps_the_other_source_playing` (removed source sorts first, so the remaining one changes graph index); post-swap blocks bit-identical; removed-source submit `(MISO_ENGINE_V1_INVALID_ARGUMENT, "source.id.unknown")` | pass |
| 3 | `a_refused_structural_transaction_moves_no_producer`: all six phases of the fault matrix plus `ProtocolCommit`; after each refusal the source accepts PCM and renders bit-identical to an unedited session | pass |
| 4 | `control_calls_inside_a_plan_swapping_render_call_keep_replacement_live` (UpsertTrack/RemoveTrack trigger, a submit while the candidate is pending and one inside the window, swap block non-silent) and `race_plan_swaps` (UpsertTrack/RemoveTrack, PCM submitted between every control call; only OK or ring-full backpressure accepted; every applied swap carried) | pass |
| 5 | both `resource_lifecycle` oracles with the rows above | pass |
| 6 | `audit capi`: transaction after call 0, outside both render scopes; allocations, deallocations, locks, syscalls and every other counter 0; carry counts `(1, 0)`; `pcm_digest` moves `ff6cdcb96cdcdad5` -> `7281b6c931e05dcc` (format-only check in CI; keys unchanged) | pass |
| 7 | `cargo test --locked -p capi` (38 + 11), `cargo test --locked -p audit capi::`, `check-capi-abi.sh` and `--self-test`, fmt, clippy (workspace, all targets, all features), doc, workspace and realtime policies and their mutation tests, `check-cross-targets.sh` | pass |

No browser Wasm code changed (capi and tools/audit are native only): no worklet rebuild, artifact
unchanged.

**Mutations** (each applied alone, test red, reverted, green):

| Mutation | Red |
|---|---|
| M1 `Structural` arm passes no successor base (fresh rings) | gates 1, 2, 4 (window), structural-epochs test (`source.frame.noncontiguous` on the post-commit submit) |
| M2 `adopt_persisting` removed (producer left in the old set) | gates 1, 2, 4 (window) (`source.ring.vacated`) |
| M3 `submit` routed to `self.providers` | gates 1, 2, 4 (window and both `race_plan_swaps` tests) |
| M4 producer moved before `prepared.commit` | gate 3, `ProtocolCommit` phase only (the other six phases return before the move) |
| M5 host-core carry joined by successor index, not the inventory row's (temporary, reverted) | gate 2 (bit-identity) |
| M6 an unknown source falls back to the current set | gate 2 (refusal) |
| M7 inventory not charged in capi's epoch row | both capi oracles and the tiny-frame oracle |
| M8 carried ring counted twice in the double-live peak | double-live oracle (exact cap refused) |
| M9 carry program not charged in the graph row | double-live oracle |
| A1 audit applies no transaction | `audit capi` exits nonzero (carry counts `(0, 0)`) |
| A2 (with M1) | `audit capi` exits nonzero (post-commit submit refused) |

**Amendment (coordinator, after attempt 1's report).** Authorized paths gained
`crates/capi/Cargo.toml` (a `bench-support` dev-dependency only), `Cargo.lock`, the new
`crates/capi/tests/plan_swap_race.rs`, and one allowlist alternative in
`scripts/check-realtime-policy.sh`. `race_plan_swaps` and its two tests move from
`resource_lifecycle.rs` to `plan_swap_race.rs`. They cannot stay where they were:
`bench_support::alloc` installs a `#[global_allocator]`, and naming any `bench_support` item from
`resource_lifecycle.rs` fails to link next to that file's own `LifecycleAllocator` ("the
`#[global_allocator]` in this crate conflicts with global allocator in: bench_support"). In the
new file:

- `render_while_producing` drives the control side. Its producer state is `&mut Control`, one
  `Control::step` per iteration.
- Each block waits until its own PCM is queued (`next_frame >= (rendered + 1) * 128`), or until
  the race is over.
- Allocation is checked with `bench_support::alloc`'s thread-scoped counters around every render
  call. `assert_installed()` runs first and the JSON statics are warmed before that.
- The reader threads keep an outer `std::thread::scope` with a `StopOnDrop` guard.
- The block bound is `swaps * (WEDGE_BLOCKS + 2)`. Blocks after the last admission render
  nothing.

The dead `track_command`, `last_error_c`, `dequeue_c` and `StopOnDrop` were removed from
`resource_lifecycle.rs`. `cargo tree -e normal,build -p capi` contains no `bench-support`.

The added mutations all go red, none hangs, and each run sat under a 300 s timeout:

| Mutation | Result |
|---|---|
| P1: render-side panic at block 5 | red in 0.06 s, both tests, including the two-reader variant |
| P2: producer-side panic after 3 admissions | red in 0.07 s, through the poisoned lock |
| P3: producer stops feeding | red after `QUEUED_DEADLINE` (10 s) |
| P4: render-side allocation | red |
| M1, M2, M3 | still red in `plan_swap_race`, on "a control call lost the race" |

The race file passed eight repeated runs. Gates re-run green:

- `cargo test --locked -p capi` (38 + 2 + 9)
- fmt; clippy (workspace, all targets, all features)
- the workspace and realtime policies and their mutation tests
- `check-cross-targets.sh`; `check-capi-abi.sh`

### Follow-ups after attempt 1 PASS

The attempt-1 verdict's three MINORs, closed in the #1275 slice:

1. **D1's pre-transaction committed model is defended at the C ABI.** The verdict's suggested
   `carried_count() == 1` no longer holds since #1276: the second, source-changing swap still
   carries the strip input sections, so the count is 2 without any defect. The check is
   behavioural instead: after the second swap,
   `structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic` submits generation 1
   from frame 0 to the changed source and expects it accepted (a new ring). Under V10 (pass the
   prospective model as `committed`) it is red: `NonContiguous { expected: SourceFrame(256),
   actual: SourceFrame(0) }`, the old ring carried.
2. **D3's seek routing is defended.** Gate 2 (`removing_a_track_and_its_source_keeps_the_other_
   source_playing`) asserts that a seek on the removed source after the commit reports
   `(RESULT_INVALID_ARGUMENT, "source.id.unknown")`. Under V4 (`seek` back to `self.providers`) it
   is red; `structural_command_keeps_…` stays green under V4, so this is the only catch.
3. **The host contract covers a changed source.** The header,
   `docs/C_ABI_V1_QUALIFICATION.md` and `docs/CONTROL_PROTOCOL_SEMANTICS.md` each gained one
   sentence: a source whose declaration the transaction changed restarts in a new ring at
   generation 1, frame 0, its accepted PCM discarded with its plan, and the host restarts its feed.
