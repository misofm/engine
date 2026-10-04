# Apply value-only effect parameter edits to the running C ABI plan

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is part of follow-up F5 of decision 14
(`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259): a parameter whose descriptor
`automation_rate` is `Block` is live. Anchors verified on `main` at `54b0a1bf8`; re-verify them
after #1263 lands.

## Product outcome

A C ABI transaction that changes, adds or removes a live parameter of a native effect (a console
slot's entry or an insert, on a track) is a live update:

- the effect's lane ramps it with the descriptor's own smoothing;
- the protocol's parameter readback reports the new value, exactly as it would after a rebuild.

Two kinds of change still rebuild:

- a parameter the descriptor keeps prepared (`automation_rate` is `None`);
- any parameter of the parametric EQ, until #1265.

## Context (verified at `54b0a1bf8`)

- **The model.** `ConsoleSlot { slot, identity, quality, link_mode }` is session-level;
  `ConsoleEntry { slot, bypass, params }` is per strip (`crates/session/src/model.rs:272-292`); an
  insert is an `Effect` with its own identity and `params`; `EffectParam { parameter_id, channel,
  unit, value }`.
- **The one value authority.** Preparation turns a strip's `params` into the effect's initial
  values inline, in `prepare_native_session_effects`
  (`crates/effect-compiler/src/prepare.rs:387-478`). That code does four things:
  - it resolves `Both`, `Left` and `Right` against the descriptor's channel policy;
  - it fills defaults;
  - it applies `effect_contract::normalize_zero`;
  - it checks the unit, the channel, the domain (`effect_contract::parameter_value_valid`,
    `crates/effect-contract/src/lib.rs:599`) and unknown IDs.
- **The registry.** `effect_compiler::launch_native_effect_registry` (`prepare.rs:202`).
- **The record.** `EffectControlRecord::Parameter { parameter_index, channel, value }`
  (`crates/effect-contract/src/live.rs:54-70`). A `Shared` parameter takes `Both`; a `PerLane` one
  takes one record per lane.
- **The producer.** `EffectControlProducer::preflight` refuses a `Parameter` record on an effect
  that requires prepared targets: the EQ (`prepare.rs:802-809`). Its handle exposes
  `available_capacity()` and `capacity()` (`:775-794`). The queue's capacity is
  `min(depth, automation_capacity)`, so a transaction can need more records than the queue can
  ever hold.
- **The readback.** The provider catalog's `parameter_state` holds each (effect, parameter,
  channel)'s prepared value under a handle (`crates/host-core/src/control_provider.rs:420-520`). A
  rebuild replaces it; nothing updates it in place today.
- **The browser's lowering.** `COMMAND_EFFECT_PARAM` (`hosts/host-web/src/lib.rs:4480-4545`,
  `:5026-5160`).
- **The effect lanes on the C ABI.** #1263.

## Decisions

- **D1. One value authority.** Move the inline resolution into
  `pub fn resolve_initial_values(descriptor: &EffectDescriptor, params: &[EffectParam]) ->
  Result<Vec<InitialParameterValue>, &'static str>` in effect-compiler, returning the diagnostic
  code on failure. Preparation calls it, and so does the classifier. Every existing effect-compiler
  test passes unchanged.
- **D2. The classifier.** `classify_live_delta` masks every console entry's and every insert's
  `params`, and then compares each instance:
  - **Domain.** If resolving either the pre-commit or the post-commit `params` fails, the delta is
    a `Domain` rebuild.
  - **The diff.** For each `(parameter_index, channel)` whose value bits differ, after
    `normalize_zero`:
    - a parameter that is not `automatable`, or whose `automation_rate` is `None`, gives a new
      `LiveRebuild::Prepared`;
    - a parameter of a target-capable effect (the EQ) gives `Prepared` until #1265;
    - any other gives one `EffectControlRecord::Parameter`.
  - **The output.** `LiveDelta` gains `effects: Vec<LiveEffectRecords>`, each with the strip ID,
    the `LiveEffectAddress` and the records.
  - Identity, quality, link mode, sidechain, the order of inserts and the slot set stay unmasked,
    so a change to any of them is `Structure`.
- **D3. capi.**
  - `commit_live` resolves each instance by `(strip_id, address)` in the newest epoch's `effects`.
  - An instance whose record count is over its queue's `capacity()` makes the whole transaction
    take the structural path (it could never fit). Room below that is `Backpressure`.
  - Every record passes `preflight` before the first push.
  - The order of #1053 D6 holds across the builtin and effect queues.
- **D4. The readback.**
  - Add `SessionControlProvider::parameter_handle(&self, track_id, rack, effect_id, parameter_id,
    channel) -> Option<u32>`, a lookup over the catalog's `parameter_metadata`, and
    `SessionControlProvider::set_parameter_value(&mut self, handle, value)`, allocation-free and
    infallible for a handle that exists.
  - Before any push, capi looks up each changed handle; a missing one is `Internal`.
  - After the commit, capi sets every value.
- **D5. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` lists live effect parameters, the
  prepared exceptions and the readback.

## Authorized paths

- `crates/effect-compiler/src/prepare.rs` (D1 only).
- `crates/host-core/src/live_delta.rs`, `crates/host-core/src/control_provider.rs` (D4 only) and
  `crates/host-core/tests/live_delta.rs`.
- `crates/capi/src/runtime/control.rs` and `live_tests.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No EQ (#1265), no bypass (#1266), and no effect on a submix strip (#1225's territory).
- No change to an effect's ramp, staging or descriptor.

## Hazards

- **Effects keep state.** A live parameter change is not bit-identical to a fresh plan of the
  edited session, because the effect's history differs. Gate 2 compares against the browser's live
  path instead, and gate 3 compares the readback.
- **Staging.** The render-side staging window drops a record only when it holds more distinct
  targets than its capacity, and the queue cap at `automation_capacity` rules that out (#140). Do
  not raise a queue's depth above it.

## Objective gates

Run every command from the repository root.

1. **The classifier.** New cases in `crates/host-core/tests/live_delta.rs`:
   - a compressor threshold change on one lane;
   - a `Both` value on a `PerLane` parameter becoming per-lane values (only the lanes that change
     get records);
   - a parameter removed (its default value);
   - a gate-expander attack change (`Prepared`);
   - an EQ band gain (`Prepared`, until #1265);
   - a unit mismatch (`Domain`);
   - an insert reorder (`Structure`).

   *Test value: it turns red if the classifier resolves a value differently from preparation, or
   lets a prepared-only parameter go live (an acked edit that the effect never applies).*
2. **Equal to the browser's lane.** A live compressor and soft-clip parameter edit through the C
   ABI, and hand-built `EffectControlRecord::Parameter` records (written in the test from the
   descriptor, never taken from the classifier) pushed into a host-core plan prepared with
   `HostLiveLanes::ALL` at the same block, render bit-identically, for 1 and 10 tracks at the four
   launch rates.
   *Test value: it turns red if the C ABI path addresses a different instance, lane or parameter
   index than the browser's lane.*
3. **Readback equals a rebuild.** After a live edit, a `ParameterStateRequest` through the C ABI
   returns the same records as after the same edit made structural. Make the comparison on a
   second session that also changes an unread source's content string.
   *Test value: it turns red if the readback keeps the old value after a live edit.*
4. **Fallbacks.** A transaction whose records for one instance exceed its queue's capacity takes the
   structural path (a new epoch) and succeeds.
   *Test value: it turns red if such a transaction would return `BACKPRESSURE` forever.*
5. **Nothing else changes.**
   - `cargo test --locked -p effect-compiler --features test-support`
   - #1257's gates 6 and 7. 4-lane (NEON) is CI-only here.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.

## Dependencies

- *Prepare C ABI plans with live effect lanes* (#1263)

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No ack precedes a drop. Never emit a redundant record.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).

## Attempt record

### Attempt 1 (implementer, on `fe3bb37e8`)

**Changes.**
- D1: `effect-compiler/src/prepare.rs` moves the inline resolution into
  `pub fn resolve_initial_values(descriptor, params) -> Result<Vec<InitialParameterValue>, &'static
  str>`, unchanged in order (unit, channel policy, defaults, `normalize_zero`, domain, unknown ID);
  preparation calls it and pushes the returned code at the instance's path. Every existing
  effect-compiler test passes unchanged.
- D2: `classify_live_delta` masks every track's console-entry and insert `params` (by position;
  IDs, identity, quality, link mode, bypass, sidechain, insert order and slot set stay compared).
  For each instance whose `params` differ (bitwise), it resolves both sides through
  `resolve_initial_values` (failure: `Domain`) and compares the resolved bits. A changed value of
  a parameter that is not automatable, whose `automation_rate` is not `Block`, or of a factory
  with a prepared-target capability (the EQ; exactly the condition under which the producer's
  `preflight` refuses a bare `Parameter`) is the new `LiveRebuild::Prepared`; any other is one
  `EffectControlRecord::Parameter`. `LiveDelta` gains `effects: Vec<LiveEffectRecords>` (strip
  ID, `LiveEffectAddress`, records). The launch registry is built lazily, only when an instance's
  `params` differ. Live is `Block` only, per the umbrella; no launch descriptor declares `Sample`.
  `LiveEffectRecords` is reachable as `host_core::live_delta::LiveEffectRecords`; it is not
  re-exported from `host-core/src/lib.rs`, which is outside the authorized paths.
- D3: `commit_live` resolves each instance by `(strip_id, address)` in the newest epoch's
  `effects`. An instance with more records than its queue's `capacity()` hands the token back for
  the rebuild path. Then the room check covers the effect queues (`BACKPRESSURE` with
  `control.live.backpressure`), every record passes `preflight`, and every readback handle is
  looked up, all before the protocol predicate and the first push. The resolution and the
  capacity check run after the live admission, keeping D6's order (a rebuild's admission is
  stricter than the live one, so the order cannot turn a rebuildable edit into a refusal).
  `ProviderEpoch::effects` loses its `allow(dead_code)`.
- D4: `SessionControlProvider::parameter_handle` (linear search over `parameter_metadata`) and
  `set_parameter_value` (binary search, allocation-free, a missing handle changes nothing). capi
  looks every handle up before any push (missing: `Internal`) and sets every value after the
  commit.
- Test support: `TestTransactionSnapshot` gains `effect_rooms` (test-only, in `control.rs`).
- D5: `docs/C_ABI_V1_QUALIFICATION.md` gains "Value-only effect parameter edits on the running
  plan (#1264)", a bullet in #1257's list, and #1263's "nothing pushes yet" line is updated.
- No resource row moves: the delta and the readback list are transient control-thread
  allocations, and the records ride the lanes #1263 attached.

**Tests and their value.**
- Gate 1 (`crates/host-core/tests/live_delta.rs`, on a two-track model with compressor,
  gate/expander and EQ inserts and a `post_insert` soft-clip console slot):
  - `a_live_parameter_change_is_one_record_on_its_lane`: compressor threshold on the left lane,
    and a console soft-clip drive beside a fader move. Red if a live change is not exactly one
    record on its lane, descriptor index and instance, or a console slot is addressed wrongly.
  - `a_both_value_split_into_lanes_records_only_the_changed_lane`: red if a rewritten
    representation emits a redundant record.
  - `a_removed_parameter_returns_to_its_default`: red if a removed parameter does not return to
    the default a rebuild prepares.
  - `prepared_parameter_changes_need_a_rebuild`: gate/expander attack and EQ band-1 gain are
    `Prepared`; the gate's threshold is live. Red if a prepared-only parameter goes live (an acked
    edit the effect never applies).
  - `params_preparation_refuses_need_a_rebuild`: unit mismatch, out-of-domain and unknown ID are
    `Domain`; the domain bound is live. Red if the classifier admits `params` preparation refuses.
  - `an_insert_reorder_is_structural`: a reorder, and a bypass flip beside a parameter change,
    are `Structure`. Red if the params mask hides an instance change.
- Gate 2 (`live_effect_parameter_edits_render_like_the_browsers_lane`, capi): compressor
  threshold (left) and makeup (both) on the first track and a console soft-clip drive on the last
  track, through `miso_engine_v1_submit_command`, against a `HostLiveLanes::ALL` host-core plan
  that receives records built in the test from each producer's descriptor, at the same block;
  bit-identical for 12 blocks, at 1 and 10 tracks and the four launch rates, with no rebuild, and
  audibly different from an unedited plan. Red if the C ABI path addresses a different instance,
  lane or parameter index than the browser's lane. (The `ALL`-lanes plan renders bit-identically
  to the C ABI plan before the edit, so the input lanes do not perturb the comparison.)
- Gate 3 (`the_parameter_readback_after_a_live_edit_equals_a_rebuilds`): a set (left lane), a
  removal back to the default and a console set, live on one rig and, with a source content edit,
  structural on another; every metadata row and every state record (handle, flags, value bits)
  read back through the C ABI is identical, and differs from before the edit. Red if the readback
  keeps the old value after a live edit.
- Gate 4 (`an_effect_edit_larger_than_its_queue_rebuilds`): on a multiband-compressor insert,
  8 per-lane parameters (16 records, the queue's capacity) are live and fill the queue; 9 (18
  records) commit through the structural path (a candidate, nothing pushed to either epoch's
  queue). Red if such a transaction would return `BACKPRESSURE` forever.
- D3 room check (`a_full_effect_lane_refuses_before_anything_changes`): 16 single-record edits
  fill the compressor's lane, the 17th is `BACKPRESSURE` with `control.live.backpressure` and
  changes nothing, and it commits after one render. Red if the room check skips the effect
  queues (an acked record would then fail to push).

**Mutation runs** (each introduced, run, reverted; logs `/tmp/claude-1002/w1264-a1/M*.log`):
- M1, classifier drops the `automatable`/`Block` test: `prepared_parameter_changes_need_a_rebuild` red.
- M2, classifier treats no effect as target-capable: `prepared_parameter_changes_need_a_rebuild` red (EQ case).
- M3, classifier emits a record for every value, changed or not: four gate-1 tests red.
- M4, classifier resolves the pre-commit side as defaults (`&[]`): the lane-split and removal tests red.
- M10, the mask skips console-entry `params`: `a_live_parameter_change_is_one_record_on_its_lane` red.
- M11, a `post_insert` slot addressed as `console(index)` without the `pre_insert` offset: gates 2
  and 3 red (the host-core gate-1 model has no `pre_insert` slot, so only the capi gates see it).
- M5, capi resolves an effect producer by address only: gate 2 red (10 tracks).
- M6, capi skips `set_parameter_value`: gate 3 red.
- M7, capi drops the over-capacity rebuild: gate 4 red.
- M8, capi drops the effect room check: the room-check test red (the push panics).
- M9, capi maps the readback's `Left` and `Right` the wrong way round: gate 3 red.

**Gates** (from the committed tree; logs in `/tmp/claude-1002/w1264-a1/`):
- `cargo fmt --all -- --check`: pass.
- `cargo test --locked -p effect-compiler --features test-support`: pass.
- `cargo test --locked -p capi`: pass (54 lib, 13 `resource_lifecycle`, doc).
- `cargo test --locked -p host-core --features control-provider,test-support`: pass (`live_delta` 22).
- `cargo test --locked -p protocol --features test-support`: pass.
- The workspace test command of #1257 gate 6: pass (1,342 tests, 0 failed).
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `./target/release/audit capi`: allocations 0, deallocations 0, locks 0, syscalls 0,
  total_violations 0.
- `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`: pass.
- `python3 -B scripts/check-scalar-oracle-absent.py --native target/release/libcapi.so`: pass.
- `for x in host-core realtime workspace protocol-control; do ...check/test...; done`: pass.
- `bash scripts/check-cross-targets.sh`: PASS; the iOS `memset_pattern16` expected-failure counts
  did not move.
- The worklet chain (effect-compiler is in the browser module): `build-web-audioworklet.sh
  --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm` and
  `test-web-audioworklet.sh`: pass.
- 4-lane (NEON), `bash scripts/run-aarch64-tests.sh debug`: not run locally (CI-only).

**Open items for review.**
- The C header's "Live edits" comment (`crates/capi/include/miso_engine_v1.h:34-41`) and
  `docs/CONTROL_PROTOCOL_SEMANTICS.md:15` still say only fader, mute and pan values are live.
  Both are outside this slice's authorized paths, so they are unchanged; the qualification doc
  is updated.

### Follow-ups applied (after the attempt 1 PASS; final batch follow-ups worker)

- **MINOR 1.** `crates/capi/include/miso_engine_v1.h` (comment only; `check-capi-abi.sh` passes) and
  `docs/CONTROL_PROTOCOL_SEMANTICS.md` now state the whole live set: track fader, mute and
  pan/matrix; model-only edits; live effect parameters (automation rate `Block`), the parametric
  EQ's through prepared targets; effect bypass except the delay's and the multiband's, which
  rebuild; every other change rebuilds, submix strips (G1), VCAs (G3) and follow-mute sources (G2)
  included.
- **MINOR 2.** `submix_effect_params_are_structural` (host-core `tests/live_delta.rs`, shaped from
  the verifier's scratch): a submix console-entry and a submix insert `params` change are both
  `Structure`, and the same model's track edit is live. Mutation: the step-3 mask also copies the
  submixes' console-entry and insert `params`: **red** (the submix edits classify live); reverted.
- **NIT 5.** Gate 1(b) gains the `-0.0` makeup case (no record). Mutation M12 (`normalize_zero`
  dropped on the `PerLane` path of `resolve_initial_values`): **red** at the new assertion;
  reverted. It was green on every committed test before.
- NIT 1, NIT 2, NIT 3 and NIT 4 are not applied: NIT 2 and NIT 3 are control-thread performance
  follow-up candidates; NIT 1 and NIT 4 are wording-level and harmless.
- Gates: see "Final batch gates" in the #1266 record.

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1264-attempt1.md`).
MINOR 1 (host-facing docs) and MINOR 2 (G1 effect regression test), with NIT 5, applied in the
final batch follow-ups (above).
