# Commit model-only C ABI transactions without a plan rebuild

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It closes follow-up F8 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259). Anchors verified on `main` at
`54b0a1bf8`; re-verify them after #1257 lands.

## Product outcome

A C ABI transaction that changes only fields no prepared plan reads commits without a plan
rebuild. It needs no live record either: the running plan, its source rings and its effect state
continue untouched. The fields are:

- the session ID;
- the render profile's ID;
- the output profile's ID;
- the stored automation table, while nothing renders it.

Today each such transaction is a full rebuild, with a source-ring reset and a silent block.

## Context (verified at `54b0a1bf8`)

- **The edits.** `SetSessionId` (`0x0001`), `SetRenderProfile` (`0x0004`) and `SetOutputProfile`
  (`0x0005`) replace `session_id`, `render_profile` and `output_profile`
  (`crates/protocol/src/model.rs:534-544`). The automation edits are `0x0600`-`0x0603`.
- **The profiles.** `RenderProfile { id, mode }`, where `mode` has one value; `OutputProfile { id,
  channels, sample_format }` (`crates/session/src/model.rs:124-158`).
- **What preparation reads.** It reads `output_profile.channels` (`crates/host-core/src/shape.rs:62`).
  It reads neither profile ID and not the session ID. Nothing in builtins-compiler, graph-compiler
  or effect-compiler reads `automation`; their only uses clear it in tests. Stored automation
  renders nothing on any host (#1058, `docs/CONTROL_PROTOCOL_SEMANTICS.md:15`).
- **The classifier (#1255).** It masks each track's `fader` and `matrix_or_pan` and compares the
  canonical JSON bytes. A delta whose masked model equals the current one is live, and an empty
  delta commits with zero records through #1257's `commit_live`.
- **`SetSessionId` is the capi tests' rebuild trigger today.** It drives a structural replacement
  in these tests:
  - `crates/capi/src/runtime/tests.rs:627`, `:862`, `:1125`, `:1260`, `:1397`, `:1475`, `:1545`,
    `:1792`, `:1876` and `:1925`;
  - `crates/capi/src/ffi.rs:1637`;
  - the `command()` helper of `crates/capi/tests/resource_lifecycle.rs` (`:192-209`). That helper
    feeds the double-live oracle (`:1263`), the race tests (`:1541`, `:1770`) and `lifecycle()`
    (`:241-244`), which `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly`
    (`:387`) uses. `lifecycle()` relies on a pending candidate refusing the second `SetSessionId`,
    so it needs a rebuild trigger too.

  The protocol's own tests use it too, but they test the controller and do not move.

## Decisions

- **D1. The mask grows.** `classify_live_delta` also copies `current`'s `session_id`,
  `render_profile.id`, `output_profile.id` and `automation` into the masked model before it
  compares. Every other field of the two profiles is still compared.
- **D2. The automation coupling.** Masking `automation` is correct only while no host renders
  stored automation. The function's documentation says so and names #1058. #1058's spec gains a
  coordination note (D4).
- **D3. The tests keep their rebuilds.** Each capi test that needs a rebuild changes its trigger
  from `SetSessionId` to a structural edit that renders identically. Use a `SetSourceContent` that
  changes only a source's `content` string, keeping its length where a test compares byte counts.
  A test that only needs a committed transaction, with no swap (for example the pinned response
  and event vectors), keeps `SetSessionId`. Its bytes are unchanged, because a live commit emits
  the same response and the same `SESSION_COMMITTED`.
- **D4. #1058's spec.** Append one coordination bullet to
  `.github/ISSUE_SPECS/1058-research-render-stored-session-automation-in-the-engine-identically-on-every-pla.md`,
  and sync it to GitHub: the C ABI's live classifier treats automation edits as model-only, so the
  first issue that renders stored automation must remove `automation` from that mask.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the mask only) and `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/tests.rs`, `crates/capi/src/runtime/live_tests.rs`,
  `crates/capi/src/ffi.rs` (its test module only) and `crates/capi/tests/resource_lifecycle.rs`.
- `.github/ISSUE_SPECS/1058-*.md` (one bullet).
- `docs/C_ABI_V1_QUALIFICATION.md` (the list of value-only transactions).
- This spec.

## Non-goals

- No change to the sample rate, the quantum, the output channel count or the sample format. They
  stay structural, or are refused by validation.
- No rendering of stored automation.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`. #1255's gate 1(k)
   case for a `session_id` change (`Structure`) is superseded: invert it in the same PR.
   - a session ID change, a render-profile ID change, an output-profile ID change and an
     automation upsert each give `Ok` with no entries;
   - the same edits together with a fader change give exactly the fader records;
   - an output-profile `channels` change gives `Structure`.

   *Test value: it turns red if a model-only field is still compared, or if a profile field that
   preparation reads is masked.*
2. **The C ABI.** New cases in `crates/capi/src/runtime/live_tests.rs`. Each of the four edits,
   through `miso_engine_v1_submit_command`, does all of these:
   - returns `OK` and raises the revision by one;
   - emits one `SESSION_COMMITTED`;
   - leaves `providers.epoch` and `pending_providers` unchanged;
   - shows in the canonical snapshot;
   - lets the host keep submitting with no seek;
   - keeps the output bit-identical to an uninterrupted render of the same session.

   *Test value: it turns red if a model-only edit still rebuilds the plan.*
3. **Rebuilds are still tested.** Every migrated test passes, and still observes the swap it
   observed before: the same epoch, publication and retirement counts.
   *Test value: it turns red if a migrated trigger no longer rebuilds, which would leave the old
   assertions unexercised.*
4. **Nothing else changes.**
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-core --features control-provider,test-support --test live_delta`
   - The workspace test command:
     `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi` reports zero allocations, frees, locks and syscalls.
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
5. **Workspace and policy.**
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - 4-lane (NEON): `bash scripts/run-aarch64-tests.sh debug` is CI-only here; record it as not
     run locally.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The list of migrated tests, each with its new trigger.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257)
- *Qualify live C ABI edits against a concurrently rendering plan* (#1258). It edits
  `resource_lifecycle.rs` too; land after it.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- A superseded test is deleted or migrated in the same PR, never weakened.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `0539791a1`)

**What changed.**
- `host_core::classify_live_delta` (D1, D2): the mask also copies `current`'s `session_id`,
  `render_profile.id`, `output_profile.id` and `automation` before the canonical-byte comparison;
  every other profile field is still compared. The documentation names the model-only fields and
  says the `automation` term holds only while no host renders stored automation (#1058).
- `crates/host-core/tests/live_delta.rs`: gate 1(k)'s `session id` case is removed from
  `structural_edits_need_a_rebuild` (superseded and inverted by the new test below).
- capi rebuild triggers migrated (D3). `runtime/tests.rs` gains `rebuild_edit(document, tag)`: a
  `SetSourceContent` that changes only the first source's `content` to `blake3:` + `tag` as hex
  x32 (same length as every valid content string), with distinct tags for successive rebuilds.
  `resource_lifecycle.rs`'s `command()` takes a content tag instead of a session ID, and the
  double-live oracle builds its prospective document from the model with the same content.
- `.github/ISSUE_SPECS/1058-*.md`: the coordination bullet (D4). **Not synced to GitHub**: the
  batch rules forbid GitHub writes from workers; the root must sync it.
- `docs/C_ABI_V1_QUALIFICATION.md`: a "Model-only (#1260)" bullet in the #1257 section.
- No production code outside the classifier changed. `commit_live` already runs an empty delta
  through the live admission, the room check (vacuous), and the protocol commit predicate; the
  reliable-event reservation is taken at protocol preparation, before `commit_live` is reached, so
  a model-only edit is refused before anything changes exactly as a fader edit is.

**New tests and their value.**
- `host-core` `model_only_edits_are_live_with_no_records`: red if a model-only field is still
  compared (each of the four alone gives no records; each, and all four, with a fader move give
  exactly the fader record; removing stored automation is model-only too).
- `host-core` `an_output_profile_channel_change_is_structural`: red if the mask copies a profile
  field that preparation reads (`output_profile.channels`) along with its ID.
- capi `live_tests::model_only_edits_commit_without_a_plan_rebuild`: each of the four edits
  through `miso_engine_v1_submit_command` returns OK, raises the revision by one, emits exactly one
  `SESSION_COMMITTED`, leaves `providers.epoch` and `pending_providers` unchanged, shows in the
  `SessionSnapshotGet` canonical JSON (compared with `protocol::apply_session_edit` on the prior
  snapshot), and the host keeps feeding generation 1 with no seek while every following block is
  bit-identical to an uninterrupted lanes-free render of the original session (each compared block
  carries signal). Red if a model-only edit still rebuilds the plan.

A second candidate test (a model-only edit with a full reliable lane is `Backpressure` and changes
nothing) was written and deleted: its path is the one #1258's
`a_live_edit_without_reliable_event_room_is_protocol_backpressure_and_pushes_nothing` already
covers, and the mutation below stayed green, so it caught nothing new.

**Migrated tests (trigger `SetSessionId` -> `rebuild_edit`/`SetSourceContent`).**

| Test | New trigger | Red with `SetSessionId` under the new mask |
|---|---|---|
| `runtime::tests::structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic` | `rebuild_edit(SESSION, 0x01)` | yes |
| `runtime::tests::control_calls_inside_a_plan_swapping_render_call_keep_replacement_live` | tags `0x10 + round` | yes |
| `runtime::tests::a_rejected_render_call_that_swaps_plans_leaves_the_c_session_live` | tag = request ID | yes |
| `runtime::tests::plan_first_destroy_guards_structural_publication_without_visible_mutation` | `0x01` | no (a live commit with the plan destroyed is refused too); migrated so it keeps naming the structural publication |
| `runtime::tests::every_structural_phase_and_ordered_dual_fault_preserves_owners_and_credits` | `0x01`, retry `0x02` | yes |
| `runtime::tests::capi_controller_dispatches_every_advertised_command_family` | `0x01` | yes |
| `runtime::tests::exported_c_replay_revision_event_and_publication_pressure_statuses_are_exact` | `0x01`, `0x02`; every pinned response/event vector unchanged | yes |
| `runtime::live_tests::a_live_edit_inside_a_plan_swapping_render_call_commits_without_a_candidate` | `content_edit` | yes |
| `ffi::tests::compile_publishes_both_children_and_source_control_is_region_checked` | inline `SetSourceContent` (`cd` x32) | no (asserts only result codes); migrated because it renders "the matched replacement plan" |
| `resource_lifecycle` `command()`: `lifecycle()` / `exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly`, `double_live_oracle_drives_exact_and_one_below_c_caps`, the two `race_plan_swaps` tests | content tags `0x01`/`0x02` (alternating in the race) | yes for the first two; the race tests were not observed red (they pass either way within their admission rules) |

Kept on `SetSessionId` (no swap needed): `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
(pinned responses and events; its bytes are unchanged under a live commit, as D3 predicts).
Each migrated test passes and asserts the same epoch, publication and retirement counts as
before (gate 3).

**Mutation runs** (each reverted; `live_delta.rs`/`control.rs` restored byte-identical with `cmp`).

| Mutation | Result |
|---|---|
| M1 drop the `session_id` mask line | host-core `model_only_edits_are_live_with_no_records` red |
| M2 drop the `render_profile.id` mask line | same test red |
| M3 drop the `output_profile.id` mask line | same test red |
| M4 drop the `automation` mask line | same test red |
| M5 mask the whole `output_profile` instead of its ID | `an_output_profile_channel_change_is_structural` red |
| M6 drop the `session_id` mask (capi) | `model_only_edits_commit_without_a_plan_rebuild` red: `(43, 0, 1)` vs `(43, 0, 0)` (a pending candidate) |
| M7 drop the `automation` mask (capi) | same capi test red at the automation upsert (`(46, 0, 1)`) |
| M8 drop the `output_profile.id` mask (capi) | same capi test red at the output-profile edit (`(45, 0, 1)`) |
| M9 skip `check_prepared_structural` for an empty delta in `commit_live` | the deleted full-lane test stayed green (event room is reserved at preparation); the test was removed |
| Gate 3: the new mask with the old `SetSessionId` triggers | 9 tests red (7 lib, 2 `resource_lifecycle`), listed above |

**Gates (all from the working tree that is this commit).**
- `cargo fmt --all -- --check`: pass.
- `cargo test --locked -p capi`: pass (50 lib, 13 `resource_lifecycle`).
- `cargo test --locked -p host-core --features control-provider,test-support --test live_delta`:
  pass (16).
- The workspace test command (spec gate 4): pass (117 test binaries, no failure).
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `./target/release/audit capi`: 100000 calls, `allocations` 0, `deallocations` 0, `locks` 0,
  `syscalls` 0, `total_violations` 0.
- `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`: pass.
- `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`: pass.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- `bash scripts/check-cross-targets.sh`: PASS (the `ios-asm-memset-pattern16` rows are the
  standing #1018 expected failures).
- Worklet chain: not run. `host_core::live_delta` is `#[cfg(feature = "control-provider")]`, and
  only `capi` enables that feature, so no line compiled into the browser module changed.
- 4-lane (NEON) `bash scripts/run-aarch64-tests.sh debug`: CI-only; not run locally.

### Follow-ups applied (after the attempt 1 PASS; batch follow-ups worker)

- **MINOR 1.** `crates/capi/include/miso_engine_v1.h` (comment only) and
  `docs/CONTROL_PROTOCOL_SEMANTICS.md` now say that a transaction changing only the session ID, a
  profile's id or the stored automation table commits with no replacement and no live value.
  `scripts/check-capi-abi.sh` passes, so the ABI is unchanged.
- **MINOR 2.** `race_plan_swaps` (the two #1042 race tests) now carries a swap witness: before the
  race the first plan's source ring is seeked to generation 2; after the race two more blocks render
  on the test thread and a generation-1 chunk must be accepted, which only a fresh plan's ring does.
  Mutation: `command()` made model-only (a `SetSessionId`): both
  `control_calls_racing_plan_swapping_renders_never_wedge_replacement` and
  `resource_queries_racing_plan_swaps_always_find_the_published_row` went **red** on the witness
  ("a generation-1 chunk was refused"); they were green under this mutation before. Reverted.
- Gates: as recorded in #1258's follow-ups note (capi tests, race x5, fmt, clippy, ABI, realtime and
  workspace policies).

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1260-attempt1.md`). MINOR 1 (header and protocol semantics name model-only commits) and MINOR 2 (swap witness on the #1042 race tests) applied in `fe3bb37e8`.
