# Make a send's follows_mute live in the browser

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-6, D15-13 E3).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

In the browser, a transaction that turns a send's `follows_mute` on or off is a live edit, not a
rebuild. This applies to a route into a submix whose plan carries a route lane. The response
reports `path: "live"`. The plan does not change. The send moves to its new effective matrix
columns through the existing route ramp. After the ramp, the output is bit-identical to a plan
prepared from the committed model with the same solo state. *Let C ABI sends follow their source
strip's mute live* (#1226) is the C ABI half, and it also delivers the shared classifier rule.

## Context

- **Decision 14, F3.** A `follows_mute` change meets rule 2 (live): the slot is the live-route
  mirror entry, and the route ramp makes the change smooth at no new cost. It was made structural
  without a reason (`docs/rulings/live-update-versus-rebuild-2026-10-04.md:157`, `:279-285`).
- **The mirror marks the flag fixed.** `LiveRoute::follows_mute` is documented "Fixed for the
  plan" (`crates/host-core/src/live_route_state.rs:46`).
  - `LiveRouteState::try_new` seeds `source_lane_muted` from the effective mute only when the
    flag is set (`:97-120`).
  - `LiveRouteState::follow` (`:189`) and `LiveRouteMuteFollow::delta` (`:232`, `:242`)
    recompute a following route's lanes.
  - The shadow, commit and rollback (`:206`) make a batch all or nothing.
- **The browser composes follows today, but only for mute changes.**
  - The admission's follow pass runs after strip, solo and VCA mute records
    (`hosts/host-web/src/lib.rs:5274-5290`). It reads the browser's effective mute
    `user_mute || vca_mute || (any_solo && !solo_safe && !my_solo)` (`:860-868`).
  - The mirror is seeded from the solo state (`:6952`).
  - Send records use kinds 13-15 (`:907-916`). No command kind changes `follows_mute`.
- **Route lanes exist only when the plan is prepared with route lanes.** See
  `crates/host-core/src/prepare.rs:1574-1591`: no control channel, or `lanes.routes` false,
  attaches none.
- **The classifier.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) still has
  the #1053 G2 guard (`:133`, `:278`, `:542-546`). #1226 removes that guard and makes a
  `follows_mute` change on a live route a live row of the shared classifier.

## Decisions frozen for this slice

- **D1. One path.** The browser receives this edit only as a transaction through the Worker's
  committed model (D15-11). This slice adds no worklet command kind and no SDK live-control
  method.
- **D2. Composition, in the Worker's control half.** For each live route whose `follows_mute`
  changes in the delta:
  1. set the mirror's flag with `LiveRouteState::set_follows_mute` (#1226 D7). It goes through
     the shadow, and turning the flag off also clears `source_lane_muted`;
  2. recompute `source_lane_muted` with `LiveRouteState::follow`, through the browser's effective
     mute (user, VCA and solo terms, from the control half's solo state);
  3. if the lanes changed, build one record with `RouteControlProducer::record` from the
     mirror's post-commit gain, matrix and mute plus the new lanes, over the session's route ramp
     (an absent ramp resolves to the session default, per #1364; an explicit 0 stays a step).

  A toggle that leaves `source_lane_muted` unchanged writes no record.
- **D3. Ordering inside one transaction.** The follow pass runs after every strip, solo and VCA
  mute change and every send value change in the same transaction. It reads their post-commit
  values. A transaction that mutes the source and turns on `follows_mute` produces one record per
  route.
- **D4. All or nothing.** Every fallible check (domain, room or cell write rules, protocol token)
  runs before the first write, as in #1053's commit order. A refusal leaves the model, revision,
  mirror and lanes unchanged.
- **D5. No route lane, no live.** If the plan has no lane for the route, the classifier returns
  `rebuild`. That is the browser's structural path (*Replace the running browser session in the
  Rust host*, #1290). This is a correct path, not a fallback.

## Deliverables

1. D2-D4 in host-web's Worker-side control half.
2. Native tests in `hosts/host-web/src/tests.rs`.

## Authorized paths

- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/control_targets.rs`,
  `hosts/host-web/src/tests.rs`

## Non-goals

- No change in `crates/host-core` (#1226 owns the classifier rule and the mirror setter).
- No C ABI change. No follow on routes into the output: those are refused at validation and
  never live.
- No SDK surface. `engine.apply` is not changed here.

## Hazards

- **A redundant record moves bits.** Re-entering the ramp kernel on a settled lane can turn `+0.0`
  into `-0.0` (`crates/host-core/src/solo.rs:58-70`). D2 writes a record only when the lanes
  change.
- **A stale flag.** If the mirror's flag is not updated, a later mute of the source is followed,
  or ignored, wrongly. Gate 2 checks this.

## Objective gates

1. **Toggle is live and exact.** Native host-web test, driven through the Worker-side control half
   and the render half on two threads:
   - Setup: track `t` sends `pre_fader` into bus `b` with `follows_mute: false` and an asymmetric
     matrix. Track `u` and `t` feed distinct, non-constant signals per lane. `t`'s left lane is
     muted.
   - Apply `follows_mute: true`, then `false`, then `true` with `u` soloed, so `t` is muted by
     solo.
   - Each response is `path: "live"` with one new revision. The plan epoch does not change.
   - After the route ramp plus the send's compensation delay, every block is bit-identical to a
     plan prepared from the committed model (with the same solo command) fed the same PCM from
     frame 0.
   - Run at 44.1, 48, 88.2 and 96 kHz, with 1 and 10 tracks.
2. **No redundant record, and the flag carries.**
   - Turn on `follows_mute` for a send whose source is unmuted. Assert no route record is written
     (the route lane's free count or dirty mask is unchanged).
   - Then mute the source. The send follows, and the output matches a fresh plan.
3. **All or nothing.** A transaction that turns `follows_mute` on and also sets an out-of-domain
   send gain is refused. Model, revision, mirror and every lane are unchanged. The next 8 blocks
   are bit-identical to a run without the call.
4. **Realtime.** In gate 1, the render thread counts `allocations == 0 && frees == 0` around every
   render call after warm-up (`bench_support::alloc` thread-scoped counters).
5. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `bash scripts/check-web-audioworklet.sh`
   - `bash scripts/test-web-audioworklet.sh`
   - the browser legs of *Run the browser control plane in a Worker and keep the AudioWorklet
     render-only* (#1332), gate 7 (`npm run qualify ... --check-matrix --self-test-mutations` in
     all three browsers)
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: turns red if a `follows_mute` toggle is still rebuilt, zeroes the wrong column or lane,
  carries a stale gain, or ignores the solo term of the browser's effective mute.
- Gate 2: turns red if the toggle re-emits an unchanged target (sign of zero moves), or if the
  mirror's flag is not updated, so a later source mute is not followed.
- Gate 3: turns red if the mirror flag or a record is written before a later check refuses.
- Gate 4: turns red if the follow composition runs on, or allocates on, the render thread.

## Dependencies

- *Let C ABI sends follow their source strip's mute live* (#1226): the classifier rule and the
  mirror setter.
- *Admit browser live edits in the Worker through the committed model* (#1382): browser
  transactions reach the shared classifier only through it.
- *Hold route-lane values in latest-target cells* (#1347): route lanes are cells.
- *Resolve an absent live ramp to the session default on the browser and in the SDK* (#1364).
- *Report each transaction's edit path in its response* (#1313).
