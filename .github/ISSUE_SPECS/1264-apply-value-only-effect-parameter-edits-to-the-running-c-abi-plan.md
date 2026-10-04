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
