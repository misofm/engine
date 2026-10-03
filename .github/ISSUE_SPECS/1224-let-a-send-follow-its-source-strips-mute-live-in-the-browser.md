# Let a send follow its source strip's mute live in the browser

Slice 26 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3, which closes here: the root pushes K3 once after this slice's verdict.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

When a producer mutes a track or a bus, or solos another track (which mutes the rest), every live
send marked `follows_mute` from that strip goes quiet with it, through the same declicked ramp, in
the same all-or-nothing submission. Unmuting or un-soloing brings it back.

A soloed vocal then no longer carries the muted drums' pre-fader reverb send. The original probe
measured that leak: a solo-muted track's pre-fader send stayed audible in its bus, at 0.3155/0.3549
where 0 was expected.

There is no render code here. This is control-plane composition, like solo. Only a route into a
submix can follow (DESIGN P11), and every such route is live in a live-controlled plan, so nothing
is left that only a plan replacement could move.

## Context (verified on `fe8ac679`)

**Every `hosts/host-web/src/lib.rs` line number below is pre-K2.** Batch K2 (*Address submix strips
in browser live commands*, #1213) widened the bands to strips and routed kind 4 through the per-strip solo
state; slices 24 and 25 added the route band and kinds. Re-read the current lines before editing.

- **Mute and solo composition in host-web** (`hosts/host-web/src/lib.rs`):
  - Kind 4 (mute) is lowered through the solo state (`:4349-4380` on `fe8ac679`): `set_user_mute`,
    then, since K2, the state's own `effective_mute(strip, lane)` (the inline copy at `:4374` was
    deleted by K2, VERIFY-2 M4), then `record_emitted`. It always stages one `TrackFaderRecord::Mute`.
  - Kind 9 (solo) stages nothing itself (`:4381-4399`); it records `solo_smoothing`.
  - After the batch loop, the coalescing pass (`:4594-4634`) emits, per strip, only the lanes whose
    effective mute differs from what the render plane was last told (`track_delta`), with
    `solo_smoothing`.
  - Staged entries are room-checked before any push, and `admit_commands` (`:4235`) commits or rolls
    back the mirrors (`:4256-4262`).
- **The "never a redundant record" rule** (`crates/host-core/src/solo.rs:28-45`). A redundant
  retarget of a settled lane re-enters the ramp and turns an exact `+0.0` into `-0.0` for a negative
  input. It is digest-visible, so it is a correctness rule. `track_delta` (`:236-254`) implements it;
  the composition is `effective_mute` (`:178-182`, per strip with solo-safe submixes since K2).
- **Staging capacity** is `command_staging_count` (`lib.rs:6253-6257`):
  `2 * MAXIMUM_COMMAND_RECORDS + 2 * (strip count)` after K2. *Admit live send commands in the
  browser* (#1222) deliberately did not grow it; this slice does (VERIFY-2 MINOR 9).
- **After *Let a route into a submix follow its source strip's mute in the session* (#1218):** a route's
  prepared `follow_zeroed` is its source strip's session mutes when it has `follows_mute`; a route
  into the output cannot have it; a route silenced on both lanes is inactive when undelayed and mixed
  with `[+0.0; 4]` when delayed.
- **After slices 22-25:**
  - every route into a submix is live in a live-controlled plan;
  - `host_core::LiveRouteState` mirrors each live route's `gain_db`, `matrix`, `mute`,
    `follows_mute`, `source_strip` and `source_lane_muted`, with shadow, commit and rollback;
  - `RouteControlProducer::{record, push, free}` exist, and route records are staged as
    `AdmittedCommand::Route` on the route band;
  - kind 14 `routeMute` mutes one send explicitly.
- **Ramp length.** There is no session smoothing table until #1054.
- **The harness.** `hosts/host-web/src/tests.rs` (`render_pair_and_compare`, `:6681`). Its
  `feed_and_render` (`:2667-2684`) feeds one constant to identical planes of one shared source;
  *Admit live send commands in the browser* added a feeder with distinct per-channel, per-lane
  signals, which every gate here uses (VERIFY-2 M13).
- **`docs/BUILTINS_AND_METERING_V1.md`** has "Solo in place" (`:115-151`) and "Metering and
  observation while soloed" (`:168-180`).

## Decisions frozen for this slice

- **D1. Effective route values.** For a live route with `follows_mute`, the values sent are the
  mirror's `gain_db`, `matrix` and `mute`, with
  `source_lane_muted = [effective_mute(source_strip, 0), effective_mute(source_strip, 1)]`.
  - The mute is P7's effective mute: user mute, or solo-derived for tracks. Submix strips are
    solo-safe, so for a bus source only its own mute counts. The VCA umbrella adds `vca_mute`.
  - Routes without `follows_mute` keep `source_lane_muted = [false; 2]` and are never touched here.
  - Routes into the output never follow (P11), so there is nothing to compose for them.
- **D2. One composition function, in host-core.**

  ```rust
  pub struct LiveRouteMuteFollow;
  impl LiveRouteMuteFollow {
      /// Live routes whose follow-mute input changed: `(route index, new source_lane_muted)`.
      pub fn delta<'a>(routes: &'a LiveRouteState,
                       effective_mute: &'a dyn Fn(usize /* strip */, usize /* lane */) -> bool)
          -> impl Iterator<Item = (usize, [bool; 2])> + 'a;
  }
  ```

  - It yields a route only when `follows_mute` is set and the new `source_lane_muted` differs from
    the mirror's, so it never yields a redundant record.
  - It takes the effective mute as a function, so the C ABI can supply the committed model's mutes
    (*Let C ABI sends follow their source strip's mute live*, #1226) without host-web's solo state.
- **D3. Placement in admission.**
  - host-web calls `delta` once per batch, **after** every kind 4 and kind 9 record and the solo
    coalescing pass, so it reads the batch's final effective mutes.
  - For each yielded route it updates the mirror's `source_lane_muted` (shadowed), builds the record
    with `RouteControlProducer::record`, and stages it on the route's slot.
  - Its ramp length is the smoothing of the **last strip-mute record staged for that source strip in
    this batch, in staging order** (kind 4 records in wire order, then the coalescing pass's records,
    which carry `solo_smoothing`) (VERIFY-2 MINOR 9).
- **D4. All or nothing.**
  - Strip mute records and follow records are room-checked together before any push.
  - A full route queue refuses the whole batch as typed backpressure, and both mirrors (solo and
    route) roll back.
  - `command_staging_count` grows by the live-route count (at most one follow record per live route
    per batch).

## Deliverables

1. `LiveRouteMuteFollow::delta` in host-core (D2), with unit tests.
2. host-web wiring (D1, D3, D4).
3. `hosts/host-web/MUTATIONS.md` rows:
   - `delta` emitting every follows route (red on gate 2);
   - the follow pass placed before the solo coalescing pass (red on gate 1);
   - follow records pushed before the room check (red on gate 3).
4. A "Sends follow mute" subsection under "Solo in place" in `docs/BUILTINS_AND_METERING_V1.md`, and
   one sentence in "Metering and observation while soloed": a pre-fader send without `follows_mute`
   stays audible under solo, and only a route into a submix can follow.
5. `AGENTS.md`: remove the decision-13 qualifier from the route-mute and follow-mute sentences
   (*Record the submix, send and VCA ruling* D5), changing nothing else.

## Authorized paths

- `crates/host-core/src/` (the route-state module from *Admit live send commands in the browser*,
  `solo.rs` for docs only, `lib.rs`) and `crates/host-core/tests/`
- `hosts/host-web/src/{lib.rs,tests.rs}`, `hosts/host-web/MUTATIONS.md`
- `docs/BUILTINS_AND_METERING_V1.md`
- `AGENTS.md` (those qualifiers only)
- this spec

## Non-goals

- No C ABI wiring (*Let C ABI sends follow their source strip's mute live* calls `delta`).
- No change to taps: pre-fader taps stay un-gated, and `follows_mute` is the opt-in.
- No VCA mute (the VCA umbrella feeds it through the same effective mute).
- No new command kind, reason or export. No render-plane change.

## Hazards

- **A redundant record moves bits.** `delta` emits only changes. A route whose source mute did not
  change, or that has no `follows_mute`, gets no record.
- **Ordering.** Follow records must use the batch's final effective mute, after user mute and solo
  composition. Using the previous batch's state leaves a send open after a solo engages.
- **A delayed send.** A follow-muted delayed send stays mixed with zero coefficients (DESIGN P4). The
  render plane already does this; gate 6 confirms the browser path does not undo it.

## Objective gates

All gates run in the native host-web harness (`hosts/host-web/src/tests.rs`) unless stated. Every
session feeds distinct non-constant signals per track and per lane, every send matrix is asymmetric,
and every destination bus has no console slots, no stateful inserts and an identity input section,
so a comparison at the output is exact.

1. **Solo silences followed pre-fader sends.**
   - Track `drums` sends `pre_fader` with `follows_mute` to bus `verb`, track `bass` sends
     `pre_fader` with `follows_mute` to bus `room`, and `vocal` routes to the output.
   - Soloing `vocal` makes `verb`'s and `room`'s inputs exactly `+0.0` once the mute ramps complete,
     bit-identical to a host booted with `drums` and `bass` muted.
   - Un-soloing restores both, bit-identically to an unedited host.
   *Test value: it turns red if solo composition ignores routes, reads the wrong strip or lane, or
   leaves a residue after the ramp. Today's leak is this test failing.*
2. **No redundant records.**
   - Soloing a track that is no `follows_mute` source pushes no route record.
   - Re-muting an already muted track pushes none.
   - A redundant solo toggle leaves the output digest unchanged.
   *Test value: it turns red if `delta` re-emits unchanged targets and moves a settled lane's bits.*
3. **All or nothing.**
   - A submission whose follow records would overfill a route queue is typed backpressure.
   - No strip mute record and no route record is pushed, and both the solo and the route mirrors are
     unchanged.
   *Test value: it turns red if strip mutes commit while their follows do not.*
4. **Per lane.** A left-only kind 4 mute (channel 0) on the source zeroes only the left source
   column. The output equals a host booted with that lane muted, whose prepared route has the same
   column zeroed.
   *Test value: it turns red if the follow path maps the source lanes to the wrong columns, or zeroes
   both columns for a one-lane mute.*
5. **A bus as the source.** Kind 4 muting bus `drums`, which has a `follows_mute` send to `verb`,
   silences that send; un-muting restores it. Both match a freshly booted host with the bus muted and
   unmuted.
   *Test value: it turns red if follow composition reads only tracks' mutes, so a bus mute leaves its
   sends open.*
6. **A delayed follow-muted send.** The send from `drums` to `verb` carries a 486-sample compensation
   delay (a true-peak limiter insert on a sibling contributor, 48 kHz; asserted from the plan). Host
   A mutes and unmutes `drums` with kind 4 (smoothing 480), so the send follows. Host B has the same
   session with that send's `follows_mute` false, and sends the same kind 4 records plus kind 14
   `routeMute` on the send with the same smoothing. Both render the same bits on every block,
   including the 486 samples after each edit.
   *Test value: it turns red if the follow path deactivates a delayed send, re-sends a stale target,
   or uses a different ramp length from the strip mute it follows.*
7. **`delta` unit tests** (host-core): several follow routes from one strip; a strip that is the
   source of no route; a mute that flips one lane only; an unchanged effective mute yielding nothing;
   a route without `follows_mute` never yielded.
   *Test value: it turns red if `delta` yields a redundant or a non-following route, or misses one
   lane's change.*
8. **Browser artifact** (host-web has no aarch64 leg; VERIFY-2 M9):
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/check-sdk-headless.sh <A>`
   - `bash scripts/test-web-audioworklet.sh`
9. **Render allocates nothing** while solo toggles and follow records are admitted, measured as in
   the host-web live-control allocation tests: `crate::ffi::live_response_ffi_tests::measured` (the
   pattern at `hosts/host-web/src/tests.rs:3638-3647`; host-web has no `bench-support`), after warm-up.
   *Test value: it turns red if the follow pass or the grown staging allocates on the render-call
   path.*
10. **Policy, workspace and 4-lane:**
    - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
    - `bash scripts/check-realtime-policy.sh`
    - the test-debug-a command (DESIGN section 7)
    - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
      K3 push (the host-core unit tests; host-web is not in its list)
    - `cargo fmt --all -- --check`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
11. **K3 boundary gates, once at the batch head before the root pushes K3:**
    - the test-debug-a and test-debug-b commands, and
      `cargo test --locked --release -p audit -p bench -p console-workload` (DESIGN section 7)
    - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
      `./target/release/audit capi`, `bash scripts/trace-graph-audit.sh target/release/audit`,
      `bash scripts/check-graph-determinism.sh`,
      `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`,
      `bash scripts/check-builtins-fixtures.sh . target/release/audit` and
      `bash scripts/check-console-fixtures.sh target/release/session_validator`
    - `bash scripts/check-protocol-wasm-parity.sh`
    - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`
    - `bash scripts/check-sdk-generated.sh <A>`, `bash scripts/check-sdk-types.sh` and
      `bash scripts/sdk-package.sh check <A>`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name, with its one-sentence test-value answer.
- The `MUTATIONS.md` rows, with their observed red results.
- The K3 boundary gate log and the ARTIFACT CHANGED report for the shipped module.

### Attempt 1 record (Terra)

Base `932f348a8` (#1223 attempt 1). Anchors re-found by symbol.

**Implementation.**

- `crates/host-core/src/live_route_state.rs` (exported from `lib.rs`):
  - `LiveRouteMuteFollow::delta(routes, effective_mute)` is D2's frozen signature. It yields
    `(route, [effective_mute(source, 0), effective_mute(source, 1)])` only for a route with
    `follows_mute` whose lanes differ from the mirror's.
  - `LiveRouteState::follow(route, effective_mute)` records those lanes under the transaction
    shadow. It refuses an unknown index or a non-following route, whose lanes stay `[false; 2]`.
    `try_new`, `delta` and `follow` share one private `followed_lanes`, so D1's lane read is
    written once.
  - `solo.rs`: one doc sentence pointing at `delta`.
- `hosts/host-web/src/lib.rs`, `admit_commands_staged` (D3, D4):
  - The follow pass runs once per batch, after the kind 4 records and the solo coalescing pass,
    when the batch staged a strip mute or moved a solo bit and the plan has live sends.
  - For each yielded send it builds the record from the mirror through
    `RouteControlProducer::record`, with the new lanes. The ramp and the wire index come from the
    last `TrackFaderRecord::Mute` staged on the source strip's fader slot, scanning the staged
    entries backwards, which is staging order (kind 4 in wire order, then coalesced).
  - The record is staged on slot `3S + E + route` and counted in `command_wanted`. Nothing is
    pushed: the existing room check covers it with every other entry. The mirror lanes move with
    `follow` after every record is built. `admit_commands` already commits or rolls back both
    mirrors.
  - `command_staging_count(strip_count, route_count)` adds the live-send count, and the bridge
    accounting charges it.

**Decisions inside the spec.**

1. **A follow with no strip mute record is refused `malformed`.** Every effective-mute change
   stages a strip mute record, so this guard cannot fire on a correct tree. Mutations that break
   the invariant trip it, and the MUTATIONS rows also record their reds with it bypassed.
2. **A window past `ROUTE_RAMP_LENGTH_MAXIMUM` (`2^22`) refuses the strip mute `domain`** at its
   wire index, when a send follows that strip's change. It is not clamped, because D3 ties the
   ramp to the strip's. Before this slice such a kind 4 was admitted. It now refuses only when it
   moves a followed send. Gate 3 covers it.
3. **Cost.** The ramp lookup scans the staged entries once per yielded send, `O(sends x
   staged)`, on the worklet's control path. That is at most 572 entries in the staging test,
   and nothing allocates. Any per-strip memo is left to the weekly pass.
4. **The `effective_mute` closure** reads `ready.solo.effective_mute`, the one composition, so a
   bus source follows only its own mute (solo-safe).

**Tests** (each answer is *which plausible defect turns it red that no existing test catches*):

- `live_route_state::tests::delta_yields_every_following_send_of_a_changed_strip_and_nothing_else`
  (gate 7). Red if `delta` stops at a strip's first following send, yields an unchanged send, or
  reads another strip.
- `live_route_state::tests::delta_follows_one_lane_at_a_time` (gate 7). Red if a lane maps to the
  other column, one lane is read for both, or a bus source loses the track offset.
- `live_route_state::tests::a_send_without_follow_never_follows_and_a_followed_change_is_not_yielded_twice`
  (gate 7). Red if a non-following send is yielded or followed, `follow` records lanes other than
  `delta`'s, or a follow escapes the shadow.
- `tests::soloing_a_track_silences_the_followed_pre_fader_sends_of_the_rest` (gate 1). Red if
  solo leaves the pre-fader follow sends open: the measured leak, red at block 6 with the follow
  pass removed. Also red if it reads the wrong strip, or leaves a residue after the un-solo.
- `tests::a_follow_never_pushes_a_redundant_send_record` (gate 2). Red if `delta` re-emits an
  unchanged target, or the mirror is not updated, so the next batch re-emits.
- `tests::a_full_send_queue_refuses_the_strip_mutes_it_follows` (gate 3). Red if strip mutes
  commit while their follows do not: an early push, an early commit, or a clamped ramp.
- `tests::a_one_lane_mute_follows_into_its_own_source_column` (gate 4). It covers all 12
  transitions between the four lane states, plus solo over a left-only mute and back. Red on a
  swapped or duplicated lane.
- `tests::muting_a_bus_silences_its_followed_send` (gate 5). Red if follow reads tracks only, so
  a bus mute leaves its send open, or if solo mutes a bus's send.
- `tests::a_delayed_send_follows_like_an_explicit_send_mute` (gate 6). The plan is asserted by
  behaviour: fed `drums` alone, the output is exactly zero for frames 0-485 and nonzero at 486,
  and `graph_delay_bytes > 0`. Red if the follow ramp is the first strip record's, has length 0,
  or uses stale lanes.
- `tests::follow_records_admit_and_render_without_allocating` (gate 9). Red if the follow pass
  allocates.
- `tests::the_decode_staging_holds_a_full_batch_and_its_follow_records` (D4): 6 tracks x 6 buses,
  so 545 entries against the old 536. Red if staging is not grown by the send count.

Every row in `hosts/host-web/MUTATIONS.md` "Issue #1224" and `crates/host-core/tests/MUTATIONS.md`
1224-H1 to H5 was applied, run red and reverted. Deliverable 3's three rows:

- `delta` emitting every follows route: gate 2 red;
- the follow pass before coalescing: gate 1 red;
- the push before the room check: gate 3 red.

**Docs.** `docs/BUILTINS_AND_METERING_V1.md` has "Sends follow mute (issue #1224)" under "Solo in
place", and the required sentence in "Metering and observation while soloed". In `AGENTS.md`, only
the route-mute and follow-mute qualifier sentence ("Planned under decision 13 ... a route has
neither.") is removed. The VCA qualifier stays, because V4 owns it.

**Gates** (x86-64-v3 AVX2; A = `target/ci/k3-1224-artifacts`, B = `target/ci/k3-1224-named`).
Every gate below returned rc 0.

- Gate 8:
  - `build-web-audioworklet.sh --named-twin B A`;
  - `check-web-audioworklet.sh A B/…named.wasm`;
  - `check-browser-expected-resources.py --artifacts A`: digests and exact rows agree, 32 red
    self-test mutations, no re-pin;
  - `check-sdk-headless.sh A`;
  - `test-web-audioworklet.sh`.
  - CI's browser legs: `npm run qualify -- --artifacts A --sdk-root sdk --browser
    {chromium,firefox,webkit} --check-matrix --self-test-mutations` under a private PulseAudio
    null sink, as in `qualification.yml`, with the SDK source bundle (CI's mode). All three:
    "all qualification gates passed".
- Gate 10:
  - `check-host-core-policy.sh`, `test-host-core-policy.sh` and `check-realtime-policy.sh`;
  - test-debug-a: 108 binaries, 1,233 passed, 0 failed, 9 ignored;
  - `cargo fmt --all -- --check`;
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, rc 0 after
    one `needless_range_loop` fix in a new test. The final tree re-ran the host-web and host-core
    tests: 359 passed.
  - `run-aarch64-tests.sh debug`: not run, because the host is x86-64. It is left to CI's
    `aarch64-debug` at the K3 push.
- Gate 11 (K3 boundary):
  - test-debug-b: 791 passed, 0 failed, 24 ignored;
  - `cargo test --release -p audit -p bench -p console-workload`: 110 passed;
  - the release build, `audit capi`, `trace-graph-audit.sh`, `check-graph-determinism.sh`,
    `graph_fixture --check`, `check-builtins-fixtures.sh` and `check-console-fixtures.sh`;
  - `check-protocol-wasm-parity.sh`, and `check-capi-abi.sh` with and without `--self-test`;
  - `check-sdk-generated.sh A`, `check-sdk-types.sh` and `sdk-package.sh check A`.
- **ARTIFACT CHANGED** (expected: the follow pass and the grown staging): `97a758d8…` (2,763,281 B,
  #1223's record) -> `e323e4b17ff9a76bb38ae94ec3a18657ff43b68aab4eb8deae0f9f2006552e45`
  (2,765,144 B, +1,863). The named twin is `90d7d9cb…` (3,155,519 B). The module digest is not a
  per-change pin (#1061).

## Dependencies

- *Enumerate sends and drive them from the SDK* (#1223)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Admission is all or nothing. No ack precedes a drop. Never emit a redundant record.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused.
- Commit on the K3 batch branch. The root pushes K3 once after this slice's verdict.
- Attempt budget: five attempts, one adversarial verdict each.
