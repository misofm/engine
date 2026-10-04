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
