Verdict: PASS

# #1260 attempt 1 -- Sol adversarial verdict

Commit under review: `31b53b62c` (parent `0539791a1`), worktree `/home/bl/misofm/wt-1053`, reviewed
from the export `/tmp/claude-1002/v1260-a1` (built with `CARGO_TARGET_DIR=/tmp/claude-1002/vtarget-1053`).
Held against the slice spec (D1-D4, gates 1-5), umbrella #1053 D1, D6, D8, D9, D10 and D12,
AGENTS.md realtime rules and the acked-batch question.

No BLOCKER and no MAJOR. Two MINOR and three NIT findings. The mask is correct, no consumer of a
prepared plan reads a masked field, and the live admission charges a growing model exactly.

## (a) Can preparation, a resource report or render read a masked field?

No. I searched every production consumer.

- **`session_id`.** Production code reads it only in the session grammar and parser, the protocol
  edit (`protocol/src/model.rs:534`), the session wire codec and the classifier mask. Two consumers
  read it indirectly, and neither breaks D9:
  - **The builtins seal.** `session_sha256` hashes the whole canonical JSON
    (`builtins-compiler/src/lib.rs:3109-3115`, set at `:3605`). Its only production check is
    `validate_for_session` during graph compilation (`graph-compiler/src/compile.rs:88`). That check
    compares the plan with the same `CompiledSession` it is being prepared from. Nothing re-checks a
    running plan against the committed model. The seal already diverges after every #1257 fader
    commit, which masks the same way.
  - **The session resource estimate.** `compiled_model_bytes` includes the retained string bytes
    and the actual canonical bytes (`session/src/estimate.rs:303-330`), and `single_allocation_bytes`
    includes the canonical upper bound. Preparation reads both for its cap checks
    (`host-core/src/prepare.rs:1252-1290`). The live arm charges both models through
    `compiled_model_admission` (`capi/src/runtime/compile.rs:51-66`); see (e).
- **`render_profile.id`, `output_profile.id`.** Nothing reads them. `output_profile.channels` is
  read (`host-core/src/shape.rs:62`) and stays compared. `RenderMode` and `SampleFormat` each have
  one value.
- **`automation`.** No production code in builtins-compiler, graph-compiler, effect-compiler,
  host-core or capi reads it; every hit is a test's `.automation.clear()`. Validation of its
  targets (`session/src/validate.rs:900-970`) runs in the protocol's prospective compile, before
  classification, so the live and rebuild arms both get it.
- **capi reports.** `PlanResourceReport` has no session-model row; the canonical JSON is charged
  with the model, not in capi rows (`compile.rs:149-151`). `prepared_capi_resources` reads source
  and strip IDs only. The provider catalog lists effect parameters only.
- **Render.** graph, engine, builtins and effects read none of the four fields.
- **Snapshots.** `SessionSnapshotGet` returns the committed model, which is what D9 intends.

## (b) Is the automation mask safe today?

Yes. No capi path renders or schedules stored automation. The protocol's `AUTOMATION_ENQUEUE`
queue is a separate thing, and only cancellation drains it (`docs/CONTROL_PROTOCOL_SEMANTICS.md:15`).
The coupling is documented where it matters: in the function doc (`live_delta.rs:108-112`), on the
mask line (`:158-160`) and in #1058's spec. The #1058 bullet is not on GitHub yet; the root must
sync it (the worker rules forbid GitHub writes).

## (c) The migrated rebuild triggers

`SetSourceContent` is structural by construction: `sources` is never masked. I re-ran gate 3 by
turning `rebuild_edit` back into a `SetSessionId`. Six lib tests went red:
- `structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic`
- `control_calls_inside_a_plan_swapping_render_call_keep_replacement_live`
- `a_rejected_render_call_that_swaps_plans_leaves_the_c_session_live`
- `every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits`
- `capi_controller_dispatches_every_advertised_command_family`
- `exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact`

So those triggers carry weight, and each still asserts the epochs, publications and retirements
it asserted before. The tags never collide: in the swap-window test, 0x10 is followed by
0x11-0x14, and the race alternates tags because `admitted` is incremented before the next request
is built.

The two tests that already passed with the old trigger:
- **`plan_first_destroy_guards_structural_publication_without_visible_mutation`.** Migrating it
  changes nothing either way. `plan_alive` (`control.rs:792`) refuses before `commit_live`
  classifies. Migration keeps the test's name true. See NIT 1.
- **`ffi::tests::compile_publishes_both_children_and_source_control_is_region_checked`.** It asserts
  only result codes, so it never observed the swap its comment describes. Migration changes nothing
  either way. See NIT 2.

The kept `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes` is a correct D3 keep.
`EVENTS[3]` is the `capi.render.activity` diagnostic, not a swap artefact. It passes with every
pinned vector unchanged.

The two `race_plan_swaps` tests have no swap witness. I confirmed this with a mutation; see
MINOR 2.

## (d) The acked-batch question for model-only commits

No ack can precede a drop. `commit_live` (`control.rs:949-1043`) runs an empty delta through the
steps below. Nothing changes before the last fallible check:
1. classify;
2. the D8 admission;
3. no resolution and no room check (there are no records);
4. `check_prepared_structural`;
5. no push;
6. the commit, which cannot fail under the same borrow;
7. the response.

The protocol's preparation reserves room for the reliable event before `commit_live` runs. A
pending candidate was prepared from the pre-edit model, and it reads none of the masked fields, so
D9 still holds. Queued `AUTOMATION_ENQUEUE` batches are cancelled with an explicit
`AUTOMATION_CANCELED`, exactly as on a rebuild. That is a reported cancellation, not a silent drop.

## (e) D8 when the canonical JSON grows

It holds. `compiled_model_admission` adds both models' `compiled_model_bytes`, which carry the
actual canonical bytes. It also takes the larger of their `single_allocation_bytes`, which carry
the canonical upper bound. On the C ABI, with only fader and matrix lanes, the live graph term
`graph(current) [+ graph(pending)] + model(current) + model(prospective)` is at least as strict as
preparation's own check, `graph + model + effect/route controls`. The C ABI has no effect or route
controls.

I checked this with a temporary probe test in `resource_lifecycle.rs`, since reverted and
`cmp`-verified. The probe renames the session to a 77-byte ID, which grows the model:
- At the exact live peak, `maximum_graph_session_plus_plan_bytes` = 292,908, the edit is `OK` and
  raises the revision by one. The snapshot holds the new ID, and the PCM is bit-identical to an
  unedited run. A rebuild needs the candidate's graph term as well, so it would have been refused
  at this cap. The admission therefore came from the live arm.
- One byte below, the edit is `COMPILE_REJECTED` with `graph.resource.limit\t$`, and the snapshot
  and revision are unchanged.

## Findings

### MINOR 1 -- Two host-facing documents still say every other change replaces the plan

- `crates/capi/include/miso_engine_v1.h:34-41` says "Any other change takes the replacement path
  exactly as before: ... and every other field."
- `docs/CONTROL_PROTOCOL_SEMANTICS.md:15` says "every other transaction still replaces the plan".

Both are now false for session-ID, profile-ID and stored-automation transactions. Umbrella D10
requires both documents to state the classification. The slice spec authorized only
`C_ABI_V1_QUALIFICATION.md`, so the implementer was right not to touch these two files. This is a
gap in the spec, not a defect in the code.

The practical risk is low: a host that seeks anyway is unharmed, and replacement detection is
unchanged.

**Fix (root follow-up, before the umbrella closes).** After the header's list of live edits, add
one sentence: "A transaction that changes only the session ID, a profile's id or the stored
automation table commits with no replacement and no live value." Put the same clause in
`CONTROL_PROTOCOL_SEMANTICS.md:15`, and name #1260 there. The header edit is comment-only, so
`check-capi-abi.sh` still passes, as it did for #1257's header comment.

### MINOR 2 -- The #1042 race tests stay green when nothing swaps

`race_plan_swaps` (`crates/capi/tests/resource_lifecycle.rs:1874-2100`) asserts
`admitted == swaps`, an overlap, the block count and zero render allocations. None of these needs
a swap. I made `command()` emit a model-only `SetSessionId` instead:
- `double_live_oracle_drives_exact_and_one_below_c_caps` and
  `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly` went red;
- `control_calls_racing_plan_swapping_renders_never_wedge_replacement` and
  `resource_queries_racing_plan_swaps_always_find_the_published_row` stayed green, while swapping
  nothing.

This is exactly what would have happened, unnoticed, had #1260 kept the old trigger. The new
trigger does swap; the double-live oracle proves that. But gate 3's test value ("turns red if a
migrated trigger no longer rebuilds") does not hold for these two tests.

**Fix.** Add one swap witness to `race_plan_swaps`. For example, feed one chunk at generation 1
before the race, and after it assert that a generation-1 submission is refused until a seek. Or
count the admissions whose next immediate resubmission returns `BACKPRESSURE` while a candidate is
pending, and assert that the count is greater than zero. Then record the mutation above going red.

### NIT 1 -- The live arm's `plan_alive` refusal is untested

`control.rs:792` refuses before `commit_live`, so a live or model-only edit on a destroyed plan
returns `BACKPRESSURE`. No test covers that arm. `plan_first_destroy_...` now uses a rebuild
trigger only. This gap predates #1260 (it dates from #1257). Optionally, also loop that test over a
model-only edit.

### NIT 2 -- The ffi test does not check the swap its comment claims

The comment at `crates/capi/src/ffi.rs:1653-1654` says "the replacement plan this test applies
below". The test checks only `RESULT_OK` from the render. Either soften the comment, or add a
source submission after the render and assert that it is refused until a seek.

### NIT 3 -- The qualification bullet names only one event

`docs/C_ABI_V1_QUALIFICATION.md` (the Model-only bullet) says "emits the same response and
`SESSION_COMMITTED`". Under D10 it also emits one `AUTOMATION_CANCELED` per queued batch. Say "the
same response and reliable events as a replacement".

## Test value (one sentence each)

- **host-core `model_only_edits_are_live_with_no_records`** (`tests/live_delta.rs:544`). It turns
  red if any of the four mask lines is missing: the session ID, either profile ID, or automation
  still compared. It also turns red if a masked field swallows a fader record that rides with it.
  No other test treats these fields as live, and the deleted 1(k) case asserted the opposite.
  Re-run: dropping the `session_id` line turns it red at `:549`.
- **host-core `an_output_profile_channel_change_is_structural`** (`:588`). It turns red if the mask
  is widened to the whole `output_profile`, which would mask `channels`, the only non-ID profile
  field that preparation reads (`shape.rs:62`).
- **capi `model_only_edits_commit_without_a_plan_rebuild`** (`runtime/live_tests.rs:1329`). It
  turns red if a model-only edit through `miso_engine_v1_submit_command` still rebuilds: a new
  epoch or a pending candidate, a ring reset that fails the generation-1 feed, or a block that is
  not bit-identical to the reference. It also turns red if the edit is missing from the snapshot or
  the events. The host-core tests cannot see the capi arm. Re-run: dropping the `session_id` mask
  gives `(43, 0, 1)` against `(43, 0, 0)`.
- **The migrated tests.** Each keeps its previous test value, and the gate 3 re-run above shows the
  new triggers are load-bearing. The race tests are the exception (MINOR 2).
- **`structural_edits_need_a_rebuild`.** The `session id` case was removed because it is
  superseded and inverted (spec gate 1), not weakened.

## Gates re-run (export of `31b53b62c`)

| Gate | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo test --locked -p host-core --features control-provider,test-support --test live_delta` | pass, 16 tests |
| `cargo test --locked -p capi` | pass, 50 lib + 13 `resource_lifecycle` |
| `cargo build --locked --release -p audit -p capi`, then `./target/release/audit capi` | 100,000 calls; 0 allocations, deallocations, locks and syscalls; `total_violations` 0 |
| `scripts/check-capi-abi.sh` (on the release build) and `--self-test` | ok and ok |
| `check-` and `test-` policy scripts for host-core, realtime and workspace | all pass |
| `cargo clippy --locked -p host-core -p capi --all-targets --all-features -- -D warnings` | pass |

Mutations re-run, each reverted and `cmp`-verified:
- the `session_id` mask dropped (M1/M6): red in host-core and in capi;
- `rebuild_edit` turned back into `SetSessionId` (gate 3): 6 lib tests red;
- `command()` made model-only: 2 red, and the 2 race tests green (MINOR 2);
- the (e) growth probe: passes as described above.

Not re-run, because the caller asked for a lean build:
- the workspace-wide test command;
- workspace-wide clippy;
- `check-cross-targets.sh`.

The worklet chain does not apply: `live_delta` is gated on `control-provider`, which only capi
enables. AArch64 runs only in CI.

## Scope

Every changed path is authorized: the mask and its doc in `live_delta.rs`, the host-core test, the
capi test files, `ffi.rs` inside `mod tests` (from `:1128`), the #1058 bullet, the qualification
doc and this spec. No production code changed outside the classifier.
