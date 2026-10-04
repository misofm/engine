# Apply value-only effect bypass edits to the running C ABI plan

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259), and it keeps the C ABI clear of
follow-up F4. Anchors verified on `main` at `54b0a1bf8`; re-verify them after #1264 lands.

## Product outcome

A C ABI transaction that switches the bypass of a console slot's entry or an insert, on a track, is
a live update through the latency-preserving bypass shunt.

One exception still rebuilds the plan: an effect whose session bypass is prepared, the delay and
the multiband compressor. On those, a live lift would be acked and never heard (decision 14, F4).

## Context (verified at `54b0a1bf8`)

- **The model.** `ConsoleEntry::bypass` (`crates/session/src/model.rs:285-292`) and an insert
  effect's `bypass` (`:519-533`).
- **The record.** `EffectControlRecord::Bypass(bool)` (`crates/effect-contract/src/live.rs:95`). The
  rack's `BypassShunt` (`:867-890`) selects the latency-matched dry signal for the lane, and the wet
  path keeps running, so the effect's state continues either way.
- **The prepared bypass.**
  - `lowers_session_bypass(effect_id)` is false for `NEVER_BANKED_EFFECTS` (`miso.delay`) and
    `PREPARED_BYPASS_EFFECTS` (`miso.multiband-compressor`)
    (`crates/effect-compiler/src/prepare.rs:244-268`).
  - Such an effect is prepared bypassed (`:546`), and its lane is seeded from the session's bypass
    (`:1522-1528`). So a live lift is admitted and renders nothing different.
  - Every other effect is prepared enabled, and its bypass rides the lane.
- **The browser's lowering.** `COMMAND_EFFECT_BYPASS` (`hosts/host-web/src/lib.rs:836-844`,
  `:4485-4497`).
- **The classifier and the effect output.** #1264 adds `LiveDelta::effects`.

## Decisions

- **D1. The classifier.** `classify_live_delta` also masks every console entry's and every insert's
  `bypass`. For each instance whose bypass changes:
  - if `lowers_session_bypass(effect_id)` is false, the delta is a rebuild, in either direction,
    because the running plan's prepared state may differ from the model's; use a new
    `LiveRebuild::PreparedBypass`;
  - otherwise it emits one `EffectControlRecord::Bypass(post)` on the instance's
    `LiveEffectRecords`, ahead of its parameter records.
- **D2. capi.** These are effect records like #1264's: the same resolution, room, preflight and
  order.
- **D3. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists bypass as live, with the two
  exceptions and their reason.

## Authorized paths

- `crates/host-core/src/live_delta.rs` and `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/live_tests.rs`, and `control.rs` only if the push needs it.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No crossfade on the shunt (decision 14, F7: a measurement decides it first).
- No change to the browser's acceptance of a lift on a prepared bypass (F4 is the browser's own
  issue).

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - a compressor insert bypassed gives one `Bypass(true)`;
   - a console EQ entry un-bypassed gives `Bypass(false)`;
   - a delay or a multiband compressor bypass change, in either direction, gives `PreparedBypass`.

   *Test value: it turns red if a prepared bypass is classified live (an acked edit that is never
   heard).*
2. **Equal to a rebuild.** Bypass a compressor insert live, then lift it. The wet path keeps running
   under the shunt, so the effect's state is the same either way. After each toggle, from block
   E + ceil(`latency_samples` / quantum) + 1 on, the output is bit-identical to a plan compiled from
   the committed snapshot and fed the same source from sample 0. Run it for 1 and 10 tracks at the
   four launch rates.
   *Test value: it turns red if a live bypass record differs from what preparation bakes, or lands
   on the wrong instance.*
3. **The exception.** A multiband compressor bypass change through the C ABI produces a new epoch,
   and after the rebuild the output follows the committed model.
   *Test value: it turns red if the C ABI acks a lift that the plan never renders.*
4. **Nothing else changes.** #1257's gates 6 and 7. 4-lane (NEON) is CI-only here.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.

## Dependencies

- *Apply value-only effect parameter edits to the running C ABI plan* (#1264)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `b055a48d4`)

**Changes.**
- D1 (`crates/host-core/src/live_delta.rs`): the step-3 mask also copies every track console
  entry's and insert's `bypass` from `current`; submix fields stay compared (G1). In
  `effect_records`, an instance whose `bypass` differs is `LiveRebuild::PreparedBypass` (new
  variant) when `effect_compiler::lowers_session_bypass(effect_id)` is false (delay, multiband
  compressor), in either direction; a non-native identity is `Structure`. Otherwise one
  `EffectControlRecord::Bypass(post)` is inserted at the front of the instance's `records`. The
  EQ's targets are still designed from its `Parameter` records alone. A bypass-only EQ change has
  `targets: None`, so its record is pushed like any other.
- D2 (`crates/capi/src/runtime/control.rs`, `commit_live`; within the authorized "only if the push
  needs it"): an EQ owner's `edit_owner` loop skips `Bypass`; the over-capacity check and the room
  check count an EQ's `Bypass` record plus its targets (the owner's own preflight counts only the
  targets, and the bypass is pushed first); every `Bypass` record passes `preflight` before the
  first push and has no readback row; in the push step an EQ's `Bypass` is pushed ahead of
  `publish_candidate_targets`. A small `bypass_records` helper counts them.
- D3 (`docs/C_ABI_V1_QUALIFICATION.md`): the live list names bypass with the two exceptions; the
  #1264 rebuild list says "a prepared bypass"; a new "Value-only effect bypass edits" section.
- `an_insert_reorder_is_structural` loses its bypass-beside-parameter assertion, which this slice
  makes live; that case moves into the new gate-1 test with its live result.
- The browser module is unchanged: `build-web-audioworklet.sh --module-only` gives
  `5045e3bb767346fc945c57aee7ac230ecbbf62e6148c83a22d014661a3cf324a` at `b055a48d4` (temporary
  detached worktree, removed) and on this change, so the worklet chain was not run.

**Tests and their value.**
- Gate 1 (`crates/host-core/tests/live_delta.rs`, on `with_bypassable`: #1264's model plus a
  bypassed console EQ `pre_insert` slot and a delay and multiband insert):
  - `a_live_bypass_flip_is_one_record_ahead_of_its_parameters`: compressor insert bypassed gives
    `[Bypass(true)]` at `insert(0)`; console EQ un-bypassed gives `[Bypass(false)]` at
    `console(0)` with no targets; a bypass beside a threshold edit gives `[Bypass(true),
    Parameter]`. Red if a flip is not exactly one record with the post-commit value, on its
    address, ahead of the parameters, or if the mask hides it.
  - `an_eq_bypass_beside_a_gain_change_rides_ahead_of_its_targets`: red if the EQ's bypass
    reaches its target designer or displaces its edits.
  - `a_prepared_bypass_change_needs_a_rebuild`: delay and multiband, bypassed and lifted, each
    `PreparedBypass`. Red if a prepared bypass is classified live (acked, never heard).
  - `a_submix_effect_bypass_is_structural` (G1): red if the bypass mask reaches a submix's effects.
- Gate 2 (`live_effect_bypass_edits_render_like_a_rebuild`, capi): bypass the first track's
  compressor insert and the last track's console soft-clip through `SetEffectBypass`, then lift
  both; each commits one revision with no plan prepared; from E + K (`window(latency, 0,
  quantum)`) on, 3 blocks are bit-identical to `Reference::run` of the committed snapshot from
  sample 0, carry signal, and the bypassed blocks differ from the unedited session; 1 and 10
  tracks at the four launch rates. Red if a live bypass record differs from what preparation
  bakes or lands on another instance.
- D2 on the EQ (`a_live_eq_bypass_beside_a_gain_change_renders_like_the_browsers_lane`): eq0's
  console EQ bypassed beside a band-1 gain edit, against a `HostLiveLanes::ALL` host-core plan
  given `Bypass(true)` then the owner's designed targets at the same block; bit-identical for 12
  blocks at the four rates. Red if the EQ's bypass is dropped beside its owner transaction.
- D2 room (`an_eq_bypass_and_targets_without_room_for_both_refuse_before_anything_changes`): 15
  one-target edits leave room 1; a bypass plus a one-target gain is `BACKPRESSURE`, revision,
  canonical model, replay, reliable lane, every owner and the room unchanged; the retry after a
  render is live. Red if the room check counts the targets without the bypass (acked, then the
  publication fails).
- Gate 3 (`a_prepared_bypass_change_rebuilds_and_renders_the_committed_model`): a multiband
  insert bypassed, then lifted, through the C ABI: each prepares a candidate, the snapshot holds
  the new bypass, after the swap a new epoch renders with no candidate pending, and 6 blocks after
  the seek are bit-identical to `Reference::run` of the snapshot and differ from the previous
  model's. Red if the C ABI acks a prepared bypass change on the running plan.

**Mutation runs** (each introduced in the source, run, then reverted; the tree was restored from a
saved copy):
- M1 `if false && !lowers_session_bypass(..)` (prepared bypass live): host-core 1 failed
  (`a_prepared_bypass_change_needs_a_rebuild`); capi gate 3 red ("the change prepares a
  candidate", pending 0).
- M2 insert `bypass` mask removed: host-core 3 failed (gate-1 flip tests to `Structure`).
- M3 record carries `!bypass` (pre-commit value): host-core 2 failed; capi gate 2 red (block
  mismatch against the rebuild).
- M4 `records.push(Bypass)` (after the parameters): host-core 2 failed.
- M5 room check counts an EQ's targets only: the room test red (the publication panics with
  `Capacity` after the bypass push).
- M6 EQ bypass pushed after the targets instead of ahead: the EQ render test stays green (both
  drain at one block boundary, so the order is not observable in PCM); its doc no longer claims
  it, and the order is held by the classifier's record order and the code comment.
- M7 EQ bypass never pushed: the EQ render test red.
- M8 submix inserts' bypass also masked: `a_submix_effect_bypass_is_structural` red.

**Gates** (from the change, all from the repository root):
- `cargo test -p host-core --all-features`: pass; `--test live_delta` 29 passed.
- `cargo test --locked -p capi`: 75 passed, 0 failed.
- `cargo test --locked -p protocol --features test-support`: pass.
- #1257 gate 6 workspace test command: pass (1357 passed, 0 failed).
- `audit capi` (release): calls 100000, allocations 0, deallocations 0, locks 0, syscalls 0,
  total_violations 0.
- `scripts/check-capi-abi.sh`: pass.
- `scripts/check-cross-targets.sh`: PASS (iOS memset_pattern16 expected-failure rows unchanged
  in kind; no ceiling moved).
- `check`/`test` policy scripts for host-core, realtime, workspace and protocol-control: pass.
- `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- 4-lane (NEON): CI-only.

**Outside the authorized paths:** nothing.
