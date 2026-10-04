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

## Attempt record

### Attempt 1 (implementer, on `31b53b62c`)

**Changes.**
- D1: `compile.rs` adds `C_ABI_LIVE_LANES` (`effects: true` over `FADER_AND_MATRIX`; input and route
  lanes stay off) and `prepare_runtime` prepares with it.
- D2: `ProviderEpoch` gains `effects: Box<[host_core::EffectControlProducer]>`, in
  `HostLiveControlHandles::effect_controls` order, built from the host-core vector at its exact
  capacity, at compile and at every structural replacement. It is dropped with the epoch, after its
  plan. Nothing reads it outside tests until #1264, so it carries
  `#[cfg_attr(not(test), allow(dead_code))]`. No `host-core/src/lib.rs` change was needed: it
  already re-exports `EffectControlProducer` and `EffectControlResources`.
- D3: `capi_resources` takes `HostPrepareReport::effect_control_resources` and adds its
  `total_bytes()` as an epoch row, so it is in `epoch_retained`, `active_retained`
  (`capi_retained_bytes`), `validate_replacement_peak` and `validate_live_peak`. Its
  `largest_allocation_bytes()` (not the row sum) feeds `largest`. The rings and target staging are
  not charged again. To stay within clippy's argument limit, `capi_resources` now takes the strip
  table's bytes precomputed instead of the strip count and ID bytes.
- D4: `resource_lifecycle.rs`'s host half spells the selection independently
  (`HostLiveLanes { strip_input: false, effects: true, routes: false }`), keeps the effect
  producers as a boxed slice, and `HostOwners` gains `effects`, dropped first. The exact
  `capi_retained_bytes` equation adds `owners.effects`, `assert_host_owners_are_charged` checks
  `owners.effects == effect_control_resources.total_bytes()`, and `capi_epoch_terms` adds the
  effect charge.
- D5: the `EffectControlProducer` comment now says the ring is a shared `Arc` and either half may
  drop first.
- D6: `docs/C_ABI_V1_QUALIFICATION.md` gains a "Live effect lanes on every plan (#1263)" section.

**Tail and latency.** Attaching the effect lanes moves neither, on any gate-1 session: the nine-track
EQ fixture, the one- and ten-track parity sessions (the latter with the bypassed limiter insert)
and the bare sessions, at the four launch rates.

**Resource rows, nine-track EQ fixture, x86-64 eight lanes, `limits()`** (before is `31b53b62c`):

| row | before | after |
|---|---|---|
| `graph_session_plus_plan_bytes`, `graph_incremental_plan_bytes` | 253,934 | 382,918 |
| `graph_metadata_bytes` | 56,137 | 185,121 |
| `capi_retained_bytes` | 258,231 | 273,640 |
| `largest_named_allocation_bytes` | 90,720 | 90,720 |
| every other row | unchanged | unchanged |

The capi move is +15,409 = the effect producer table (9 x 104 = 936) + owned payload (14,425: the
IDs and the nine EQ owners) + 16 bytes in each of the three provider-epoch slots. The effect-control
total, 15,361, matches the 2026-09-28 prototype exactly. The graph rows were not anticipated by this
spec: they move +128,984 because the lanes make bind build the live-control owner (the banked
`LiveControlEffectBankStage` with its packed span window and shunt) as well as the rings and the
EQ target staging, all already charged by `effect_control_resource`. A scratch measurement by track
count (not committed) gives 2,104 bytes per member and 55,024 per eight-lane bank. The prepared
plan's observed bytes move by exactly +128,984 (152,857 -> 281,841), so the oracle's slack is
unchanged. `REFERENCE_BUDGETS` raises the three graph ceilings with that reason: eight lanes
measured + 10 %; four lanes derived as an upper bound (the four-lane baseline plus the eight-lane
move, plus 10 %), because AArch64 runs only in CI. The other oracle sessions: soft-clip nine-track
capi 143,931 -> 145,023; browser identity (no effect) 132,021 -> 132,069 (+48, the epoch slots
only); routed submix 149,142 -> 151,020.

**Tests and their value.**
- `c_abi_plans_with_live_lanes_render_like_lanes_free_plans` (extended): it now also runs the
  nine-track EQ fixture at the four rates, compares latency and tail with the C ABI selection
  less its effect lanes, and checks one producer per effect instance (strip ID, effect ID) with
  an owner exactly on the parametric EQs. It turns red if attaching the effect lanes changes a
  rendered bit or the tail, or if capi drops or loses producers.
- `capi_retained_bytes_charge_every_byte_the_compile_retains` (updated oracle): it turns red if an
  effect producer or owner is retained but not charged, or charged twice.

**Mutation runs** (each introduced, run, reverted):
- M1, `C_ABI_LIVE_LANES` with `effects: false`: the render test is red (producer list) and the
  oracle is red (the host half selects effects independently).
- M2, the effect row charged as 0: the oracle is red (`capi_retained_bytes` equation).
- M3, the effect row charged twice: the oracle is red.
- M4, capi drops the producers and keeps an empty slice: the render test is red (producer list) and
  the oracle is red.
- M5, `attach_effect_live_controls` seeds a non-target lane with `false` instead of the session
  bypass (`effect-compiler`, temporary): the render test is red at the bit comparison (the
  ten-track session's bypassed limiter).

**Gates** (from the working tree that is committed; logs in `/tmp/claude-1002/w1263-a1/`):
- `cargo fmt --all -- --check`: pass.
- `cargo test --locked -p capi`: pass (50 lib, 13 `resource_lifecycle`, doc).
- `cargo test --locked -p effect-compiler`: pass.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: pass.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `./target/release/audit capi`: allocations 0, deallocations 0, locks 0, syscalls 0,
  total_violations 0.
- `bash scripts/check-capi-abi.sh && bash scripts/check-capi-abi.sh --self-test`: pass.
- `for x in host-core realtime workspace protocol-control; do bash scripts/check-$x-policy.sh &&
  bash scripts/test-$x-policy.sh || exit 1; done`: pass.
- `bash scripts/check-cross-targets.sh`: PASS; the iOS `memset_pattern16` ceilings did not move.
- 4-lane (NEON), `bash scripts/run-aarch64-tests.sh debug`: not run locally (CI-only). The derived
  four-lane graph ceilings are first measured there.
- The worklet chain: not run. No line compiled into the browser module changed; the only
  non-capi edit is a doc comment in `effect-compiler`.

**Open items for review.**
- `host-core/src/prepare.rs` docs still say the C ABI prepares with `FADER_AND_MATRIX`
  (`HostLiveLanes::FADER_AND_MATRIX`, `HostLiveControlHandles::route_controls`). That file is
  outside this slice's authorized paths, so they are unchanged.
- The graph-row move (+128,984 on the reference session; about 2.1 KB per effect plus 55 KB per
  eight-lane bank) is a real memory cost on mobile that the spec did not foresee. It is charged
  and documented; whether to defer building the banked live-control owner until a push needs it
  is an owner question, not this slice's.

### Follow-ups applied (after the attempt 1 PASS; final batch follow-ups worker)

- **MINOR 1.** The effect-control payload is admitted twice, recorded as a conservative double
  admission: a clause at the capi charge site (`crates/capi/src/runtime/compile.rs`), a sentence in
  the "Charges" bullet of `docs/C_ABI_V1_QUALIFICATION.md` (the initial compile needs the graph row
  plus the compiled model plus the payload `capi_retained_bytes` charges, 15,361 bytes on the
  reference session), and this line. The optional oracle boundary pin was not added.
- **MINOR 2.** The qualification doc now gives the formula (EQ member 2,104 B, other member
  1,208 B; at a 128-frame quantum an eight-lane bank 8,944 + 360 * S plus 64 * L for latency L,
  a four-lane bank 4,560 + 200 * S) and labels the table as the reference session's at
  `limits()`.
- **MINOR 3.** `host-core/src/prepare.rs` (`FADER_AND_MATRIX` and `route_controls` docs) and
  `effect-contract/src/live.rs` (the ring is shared; the last owner frees it; capi drops the plan
  first) are corrected. Doc comments only.
- **NIT 1.** Resolved at head: #1264 reads the field and no `allow(dead_code)` remains.
- **NIT 3.** Gate 1 (`c_abi_plans_with_live_lanes_render_like_lanes_free_plans`) gains
  `mixed_effect_parity_session`: the ten-track parity session plus an enabled compressor insert on
  two tracks and a bypassed delay and multiband insert on another, at the four rates. Mutation M6
  (`attach_effect_live_controls` seeds every non-target lane `true`): **red** at "mixed effects 10
  tracks at 44100 Hz: block 3"; reverted. At this head M6 also turns three #1264/#1266 PCM gates
  red, but gate 1 is the only test that compares mixed non-EQ cohorts against a lanes-free plan.
- **NIT 4.** `largest` chains the named rows (`control_table_row`, `source_id_row`,
  `strip_table_bytes`) instead of slicing `epoch_rows[..len - 1]`. Behaviour-neutral: every capi
  test and resource oracle passes unchanged.
- **NIT 2** (tighten the four-lane ceilings) waits for the first AArch64 CI run; the owner FYI
  (bound the windows by lane depth) is Q5 in `docs/handoffs/live-updates-1053/README.md`.
- Gates: see "Final batch gates" in the #1266 record (one run for the #1263-#1266 follow-up set).

### Verdict

**Verdict.** Sol attempt 1: PASS (verdict file `docs/handoffs/live-updates-1053/1263-attempt1.md`).
MINOR 1 (double admission), MINOR 2 (formula) and MINOR 3 (stale docs), with NIT 3 and NIT 4,
applied in the final batch follow-ups (above); NIT 2 and the window FYI stay follow-up candidates.
