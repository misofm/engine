# Prepare C ABI plans with live effect lanes

Slice of the umbrella *Deliver value-only fader, mute and pan transactions to the running C ABI
plan through the live console lanes* (#1053). It is the resource half of the effect part of
follow-up F5 of decision 14 (`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, #1259), and
it does for effect lanes what #1256 does for strip lanes. Anchors verified on `main` at
`54b0a1bf8`; re-verify them after #1257 lands.

## Product outcome

Every C ABI plan carries one live control lane per prepared effect instance, console slots and
inserts alike. A parametric EQ also carries its prepared-target owner. capi keeps the producers
with the plan's provider epoch and charges them to the byte. Nothing pushes yet: #1264, #1265 and
#1266 do. Rendering, latency and tail do not change.

## Context (verified at `54b0a1bf8`)

- **Attaching the lanes.** `attach_effect_live_controls`
  (`crates/effect-compiler/src/prepare.rs:1462-1530`):
  - each queue's depth is `min(depth, automation_capacity)`, so the render-side staging window
    cannot overflow;
  - a target-capable effect (the parametric EQ) gets an `EffectControlOwner`;
  - each lane is seeded from the session's bypass. The delay and the multiband compressor keep a
    prepared bypass (`lowers_session_bypass`, `:266-268`).
- **The producer.** `EffectControlProducer { track_id, address, effect_id, descriptor, .. }`
  (`:635-659`). host-core re-exports it, with `LiveEffectAddress`
  (`crates/host-core/src/lib.rs:180-183`). `HostLiveControlHandles::effect_control_mut` finds one by
  `(track_id, address)` (`crates/host-core/src/prepare.rs:421-430`).
- **The charges in host-core.** `HostPrepareReport::effect_control_resources`
  (`prepare.rs:266-267`) holds the producer table and the owned payload (`EffectControlResources`,
  effect-compiler `:663-688`). host-core checks them against the graph and named-allocation caps
  (`:1163-1182`). But they are not in `graph_session_plus_plan_bytes` (`:1337`), and capi's report
  (`crates/capi/src/runtime/compile.rs:437-467`) has no row for them.
- **The 2026-09-28 prototype** measured 15,361 bytes of effect-control payload on the nine-track EQ
  fixture (`docs/handoffs/live-control-2026-09-28/FINDINGS.md`, section 4).
- **The lane selection.** #1254's `HostLiveLanes::effects`.

## Decisions

- **D1. The lanes.** capi's preparation selects `effects: true` (with whatever #1261 left for
  `strip_input`).
- **D2. The epoch keeps the producers.** `ProviderEpoch` gains
  `effects: Box<[EffectControlProducer]>`, in the handles' order. They are dropped with the epoch,
  after its plan, as #1256's strip producers are.
- **D3. Every byte is charged once, in the row of its holder.**
  - capi charges `effect_control_resources.total_bytes()` in its epoch rows, because capi holds the
    producers and their owners.
  - The queue rings and the target staging are already charged in the graph estimate
    (`effect_control_resource`, `crates/graph-compiler/src/estimate.rs:162-240`), which is
    part of the plan's rows. Do not charge them again in capi.
  - `validate_replacement_peak` and #1257's `validate_live_peak` must see every effect-lane byte of
    both plans.
- **D4. The oracle.** `resource_lifecycle.rs`'s host half replays the same request.
  `HostOwners` gains `effects`, dropped first. The exactness assertion on `capi_retained_bytes`
  holds.
- **D5. A stale comment.** Correct `EffectControlProducer`'s "a producer must be dropped before the
  plan" (`crates/effect-compiler/src/prepare.rs:631-633`): the rings are shared `Arc`s and capi
  drops the plan first, as #1256 explains for the strip producers. Authorized: that comment only.
- **D6. Documentation.** `docs/C_ABI_V1_QUALIFICATION.md` records the new charge and gives the
  nine-track fixture's numbers.

## Authorized paths

- `crates/capi/src/runtime/compile.rs`, `control.rs` and `tests.rs`.
- `crates/capi/tests/resource_lifecycle.rs`.
- `crates/host-core/src/lib.rs`, only for a re-export capi needs.
- `crates/effect-compiler/src/prepare.rs`: the one comment of D5.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- This spec.

## Non-goals

- No push, and no classifier change.
- No change to the effect lanes themselves, or to the browser.

## Objective gates

Run every command from the repository root.

1. **Same rendering, same tail.** Extend #1256's
   `c_abi_plans_with_live_lanes_render_like_lanes_free_plans`, or add a sibling test. Use the
   nine-track EQ fixture and the 10-track parity session (it has a bypassed limiter insert), for 1
   and 10 tracks at the four launch rates:
   - 8 blocks are bit-identical to a lanes-free `host_core::prepare_host_runtime` plan;
   - `latency_samples` and the tail equal those of the same C ABI plan without effect lanes;
   - `providers.effects` holds one producer per effect instance, and the EQ instances have owners.

   *Test value: it turns red if attaching effect lanes changes a rendered bit (a session bypass
   seeded wrongly, or a bank split), or drops the producers.*
2. **Exact charges.** `cargo test --locked -p capi --test resource_lifecycle`. Both oracles pass
   with D3 and D4.
   *Test value: it turns red if an effect producer or an owner is retained by capi but not
   charged, or is charged twice. (The plan's own rows are only a bound, with about 128 KB of slack
   on this fixture, `resource_lifecycle.rs:673-687`, so a missing ring charge smaller than that is
   not caught here.)*
3. **Nothing else changes.**
   - `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi` reports zero allocations, frees, locks and syscalls.
   - `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`
   - The workspace, policy and cross-target gates of #1257's gate 7. 4-lane (NEON) is CI-only
     here.

## Evidence

- The output of every gate command, from the head commit.
- Each new test's name, with its one-sentence test value.
- The resource rows before and after, on the nine-track EQ fixture.

## Dependencies

- *Apply value-only track fader, mute and pan transactions to the running C ABI plan* (#1257)
- *Qualify live C ABI edits against a concurrently rendering plan* (#1258); it edits
  `resource_lifecycle.rs` too

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- "Bit-identical" gates are hard stops.
- A test that greps source or prose is refused.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
