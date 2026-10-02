# Ride VCA groups live in the browser

Slice V3 of *VCA groups*. It is in the VCA batch, with V1, V2 and V4. **Drafted; anchors re-verified
at filing:** they were read on `fe8ac679`, before batches K1-K3 of *Submix strips and live aux sends*
moved them; the Context says which slice moved what.

## Product outcome

In the browser engine, a live VCA command rides or mutes a VCA. Every member strip's fader follows
through its existing declicked fader ramp, together, all or nothing.

- A member's own fader or mute move still works on top of the VCA.
- A VCA mute silences its members' `follows_mute` sends, exactly as a fresh plan with the VCA muted
  renders them (*Apply VCA offsets and mutes at preparation* made the prepared path agree).
- Render code is unchanged: composition happens on the control plane.

This slice is host-core's composition and the browser's admission of the two VCA command kinds.
*Enumerate VCA groups and drive them from the SDK* publishes the VCA order, the metadata and the SDK
surface.

## Context (verified on `fe8ac679`; re-verify at filing)

- **The per-strip mute state** (`crates/host-core/src/solo.rs`). After *Give every strip one mute
  owner and live-control producers in host-core* and *Apply VCA offsets and mutes at preparation* it
  holds, per strip lane, `user_mute`, `vca_mute`, solo and `emitted`; composes
  `(user_mute || vca_mute) || (any_solo && !solo_safe && !soloed)` through
  `effective_mute(strip, lane)`; and has shadow, `commit` and `rollback`. It emits mute deltas only,
  for lanes whose effective mute changed (`track_delta`, `:236-254` at `fe8ac679`).
- **Never emit a redundant record** (`solo.rs:32-45`). Re-entering the ramp kernel on a settled lane
  turns an exact `+0.0` into `-0.0` for a negative input: digest-visible. The same holds for a
  redundant `FaderDb` retarget of a settled lane.
- **Fader records.** `TrackFaderRecord::{FaderDb, Mute}` (`crates/builtins-compiler/src/lib.rs:129`)
  feed each strip's fader queue. host-web pushes them after a free-room pass (`queue_available`, then
  `push`). Kind 3 (fader dB) goes directly to the fader queue (`hosts/host-web/src/lib.rs:4330-4348`
  at `fe8ac679`); kind 4 (mute) goes through the mute state (`:4349-4380`; *Address submix strips in
  browser live commands* deleted its inline composition).
- **Follow mute.** After *Let a send follow its source strip's mute live in the browser*,
  `LiveRouteMuteFollow::delta` turns effective strip-mute changes into route records, never redundant,
  inside the same admission, after the solo coalescing pass.
- **The VCA reach and effective values** come from the session helper of *Apply VCA offsets and mutes
  at preparation* (`SessionModel::effective_strip_faders`, `strips()` order).
- **Kinds and their spellings.** At `fe8ac679` kinds are 1-12 (`lib.rs:797-862`). *Admit live send
  commands in the browser* added 13 `routeGainDb`, 14 `routeMute`, 15 `routeMatrix`, moved the
  generic bounds check after kind dispatch, and moved the self-test anchors past 15:
  - the "added and not threaded" mutation (`scripts/check-command-kind-vocabulary.py:384-396`,
    `scripts/test-web-audioworklet.sh:295`) inserts `COMMAND_SOLO_MODE: u32 = 16` after the
    route-matrix line; the "set gains a kind the wire does not decode" mutation uses 16;
  - the kind-set literal and `<= 15` anchors (`check-command-kind-vocabulary.py:432`, `:440`,
    `:448`, `:457`; `test-web-audioworklet.sh:277`).

  The seven spellings of a kind: `COMMAND_*` in `lib.rs`; the decode whitelist (`:3869-3884`); host JS
  `COMMAND_KINDS` (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:257`); the `.d.ts`
  `MisoCommandKind` (`miso-engine-v1-audio-worklet-host.d.ts:188`) and its byte-identical mirror
  `sdk/src/browser/shipped-host.d.ts`; `tools/parameter-metadata/src/lib.rs:154-182`;
  `scripts/check-parameter-metadata-v1.py:34-37`; the shipped metadata JSON. Also
  `tools/parameter-metadata/src/abi_layout.rs` `wire_command_kinds` (`:1743-1756`).
- **Reasons.** At `fe8ac679` reasons are 0-11 (`lib.rs:926-966`). *Add the notSoloable command reason
  to every vocabulary spelling* added 12 and *Admit live send commands in the browser* added 13
  `unknownRoute`; the `COMMAND_REASON_FUTURE_TAP` drift mutation sits at 14
  (`scripts/check-command-reason-vocabulary.py:326-334`, `scripts/test-web-audioworklet.sh:245`,
  `hosts/host-web/MUTATIONS.md:132`). The reason spellings: the Rust constant, host JS
  `COMMAND_REASONS`, the `.d.ts` `MisoCommandReason` and its SDK mirror, the metadata generator rows,
  the schema-gate list, `tools/parameter-metadata/src/abi_layout.rs` (around `:1758-1770`),
  `scripts/check-abi-layout-v1.py:81-84`, `scripts/fixtures/abi-layout-v1-self-test.json`, and
  `scripts/fixtures/parameter-metadata-v1-self-test.json:28-32`; `scripts/check-parameter-metadata-v1.py:637`
  re-numbers the last reason in its self-test.
- **Staging capacity** is `command_staging_count` (`lib.rs:6253-6257` at `fe8ac679`; per strip and
  grown by the live-route count after K2-K3).
- **The native harness** is `hosts/host-web/src/tests.rs`, with `render_pair_and_compare` (`:6681`);
  `feed_and_render` (`:2667-2684`) feeds one constant to both planes of one shared source, so gates
  that claim a lane or member error feed distinct sources (`DESIGN.md` 7, VERIFY-2 M13).

## Decisions frozen for this slice

- **D1. `LiveVcaState`** (host-core, new `vca` module) holds per VCA lane `{db, mute}`, per member lane
  the member's own `db` (its own mute stays in the mute state), and the reach sets from preparation.
  It has shadow, `commit` and `rollback`, and host-web commits or rolls it back where it commits or
  rolls back the mute and route mirrors.
- **D2. Composition** (the rule of *Apply VCA offsets and mutes at preparation*, recomputed live). A
  VCA change recomputes the effective gain and the VCA mute term for every reached member lane. In one
  admission, in this order:
  1. emit `FaderDb` records only for lanes whose effective gain changed;
  2. feed the new `vca_mute` values into the mute state, whose delta emits `Mute` records only for
     lanes whose effective mute changed;
  3. feed the effective-mute changes into `LiveRouteMuteFollow::delta` (its existing pass, after the
     solo coalescing pass).

  A member's own kind 3 updates the member mirror and recomputes that member's effective gain; its
  own kind 4 goes through the mute state as before.
- **D3. All or nothing** across every member fader queue, every route queue reached through follow
  mute, and every mirror. A full queue is typed backpressure, with nothing pushed and every mirror
  unchanged. `command_staging_count` grows by twice the member-lane count (one `FaderDb` and one
  `Mute` per member lane at most).
- **D4. Kinds and reason** (in-place V1 amendment; append, never renumber):
  - 16 `vcaFaderDb` (`values[0..1]` per lane, as kind 3) and 17 `vcaMute` (as kind 4), if 13-15 are
    the route kinds; otherwise the next free values, stated in the PR. The index word is a VCA index
    in canonical ID order (the order `effective_strip_faders`' reach sets use).
  - A VCA index at or past the VCA count refuses with a new reason, 14 `unknownVca` (the next free
    value), and nothing is staged. The reused route or track reason would misname the address.
  - Every kind and reason spelling listed in Context; the drift mutations re-anchor past the new last
    kind (`COMMAND_SOLO_MODE` and the undecoded-set mutation move to 18) and past the new last reason
    (`COMMAND_REASON_FUTURE_TAP` moves to 15).
- **D5. Smoothing** is the record's `smoothingSamples`, applied to every member record it emits, until
  #1054 gives a session table.

## Deliverables

1. D1-D2 in host-core, with unit tests (diamonds, clamp and restore, no redundant records).
2. D3-D5 in host-web (constants, decode whitelist, admission, staging, room check, push).
3. Every kind and reason spelling, with the regenerated `sdk/assets/**` and `sdk/src/generated/**`.
4. `hosts/host-web/MUTATIONS.md` rows for each new kind, the new reason and the moved drift
   mutations; a "VCA groups" paragraph under "Solo in place" in `docs/BUILTINS_AND_METERING_V1.md`.

## Authorized paths

- `crates/host-core/src/` (a new `vca` module, plus `solo.rs` and `lib.rs`) and
  `crates/host-core/tests/`
- `hosts/host-web/src/{lib.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `tools/parameter-metadata/**` (the kind and reason rows and layout constants only)
- `scripts/check-command-kind-vocabulary.py`, `scripts/check-command-reason-vocabulary.py`,
  `scripts/check-parameter-metadata-v1.py`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/fixtures/parameter-metadata-v1-self-test.json`, `scripts/test-web-audioworklet.sh`,
  `scripts/test-web-audioworklet.mjs` (only where it spells kinds or reasons)
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only), `sdk/src/browser/shipped-host.d.ts`
- `docs/BUILTINS_AND_METERING_V1.md`
- this spec

## Non-goals

- No export, `SessionMap` entry, metadata family or SDK `vca(id)` API (*Enumerate VCA groups and
  drive them from the SDK*).
- No C ABI path (*Deliver value-only VCA edits to the running C ABI plan*).
- No VCA solo, no send trim and no automation.
- No render-code change.

## Hazards

- **A redundant record moves bits.** D2 emits only changed lanes, for gains and for mutes alike.
- **Clamping.** A member pushed past +24 dB by a VCA clamps. When the VCA comes back, the member
  returns to its own balance, because the mirror holds the member's own value, never the effective
  one.
- **Fan-out and backpressure.** A 64-member VCA move is up to 128 fader records plus follow records in
  one admission. A full member queue refuses the whole submission.
- **Ordering.** Follow records must use the batch's final effective mutes, after user mute, VCA mute
  and solo composition.
- **Vocabulary churn.** A missed spelling fails the kind or reason gate, and a missed self-test anchor
  makes a mutation "match nothing", which fails the self-test. Fix both; never silence them.

## Objective gates

All gates run in the native host-web harness (`hosts/host-web/src/tests.rs`) or host-core's tests
unless stated. Tracks read distinct source channels with non-constant signals on each lane.

1. **A VCA ride lands on a fresh plan's bits.** After the ramp, a random VCA move (nested,
   overlapping, clamping) renders bit-identically to a plan prepared with the committed VCA values.
   The members feed a destination with no stateful downstream (empty console, no inserts, identity
   input section).
   *Test value: it turns red if live composition differs from preparation's rule in order, reach or
   clamp, or if it emits to the wrong member or lane.*
2. **Member and VCA compose.** The sequence is member fader -3 dB, then VCA -6 dB, then member
   +2 dB. The effective gain is `clamp(2 + (-6))`, bit-identical to a fresh plan. Un-muting the VCA
   leaves a member's own mute on.
   *Test value: it turns red if a member move overwrites the VCA term, or the reverse.*
3. **No redundant records.** A VCA move that changes no member's effective value (every member clamped
   at -144) pushes nothing, and the digest is unchanged.
   *Test value: it turns red if composition re-emits unchanged targets and moves settled zeros.*
4. **All or nothing.** A VCA move that would overfill one member's queue, or a follow route's queue,
   is typed backpressure: nothing is pushed, and the VCA, mute and route mirrors are unchanged.
   *Test value: it turns red if a member's records are pushed, or a mirror commits, before every
   destination's room is checked.*
5. **VCA mute flows to sends.** Muting a VCA silences its members' `follows_mute` pre-fader sends,
   bit-identically (at the destination's input) to a fresh plan prepared with the VCA muted; un-muting
   restores them bit-identically to an unedited host.
   *Test value: it turns red if the VCA mute term does not reach `LiveRouteMuteFollow::delta`, or if
   the live path and preparation disagree about a VCA-muted member's send.*
6. **Addressing.** `vcaFaderDb` and `vcaMute` at an index at or past the VCA count refuse with
   `unknownVca` and stage nothing; every existing host-web live-control test passes unchanged.
   *Test value: it turns red if a VCA index is checked against another table, or a bad index is
   refused with a reason that misnames it.*
7. **Render allocates nothing** during VCA rides (the host-web live-control allocation test pattern):
   `allocations == 0` on the render thread after warm-up.
   *Test value: it turns red if a VCA move puts any allocation on the render path (composition is
   control-plane only).*
8. **Vocabulary, browser and policy.**
   - `python3 -B scripts/check-command-kind-vocabulary.py --self-test` and
     `python3 -B scripts/check-command-kind-vocabulary.py`
   - `python3 -B scripts/check-command-reason-vocabulary.py --self-test` and
     `python3 -B scripts/check-command-reason-vocabulary.py`
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-parameter-metadata-v1.py --self-test` and
     `python3 -B scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json`
   - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
     `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh <A>`, `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/test-host-core-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh`
   - the workspace test command (`DESIGN.md` section 7), `cargo fmt --all -- --check` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/run-aarch64-tests.sh debug` (host-core's composition tests) on an arm64 host, or
     CI's `aarch64-debug` at the batch push

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The re-anchored mutation list, with each mutation's observed red result.
- The shipped module's ARTIFACT CHANGED report.

## Dependencies

- *Apply VCA offsets and mutes at preparation*
- *Let a send follow its source strip's mute live in the browser*

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Admission is all or nothing. No ack precedes a drop. Never emit a redundant record.
- In-place V1 amendment: append kinds and reasons; never renumber.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
