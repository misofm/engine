# Make a send's follows_mute live in the browser

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-6, D15-13 E3).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

In the browser, a transaction that turns a send's `follows_mute` on or off is a live edit, not a
rebuild. This applies to a route into a submix whose plan carries a route lane. The response
reports `path: "live"`. The plan does not change. The send moves to its new effective matrix
columns through the existing route ramp. After the ramp, the output is bit-identical to a plan
prepared from the committed model with the same solo state.

No browser code composes the follow. *Let C ABI sends follow their source strip's mute live*
(#1226) makes the toggle a live row of the shared classifier and builds follow records in the
shared commit. *Admit browser live edits in the Worker through the committed model* (#1382 D3)
adds the browser's solo term to that same composition. This slice proves the browser gets the
result, including with solo. It adds no production code unless a gate finds a browser-only gap,
and any such fix goes into the shared commit (D2).

## Context

- **Decision 14, F3.** A `follows_mute` change meets rule 2 (live): the slot is the live-route
  mirror entry, and the route ramp makes the change smooth at no new cost. It was made structural
  without a reason (`docs/rulings/live-update-versus-rebuild-2026-10-04.md:157`, `:279-285`).
- **The mirror marks the flag fixed.** `LiveRoute::follows_mute` is documented "Fixed for the
  plan" (`crates/host-core/src/live_route_state.rs:46`).
  - `LiveRouteState::try_new` seeds `source_lane_muted` from the effective mute only when the
    flag is set (`:97-120`). The browser seeds it from the solo state (`hosts/host-web/src/lib.rs:6952`).
  - `LiveRouteState::follow` (`:189`) and `LiveRouteMuteFollow::delta` (`:232`, `:242`)
    recompute a following route's lanes. The shadow, commit and rollback (`:206`) make a batch all
    or nothing.
  - #1226 D7 adds `LiveRouteState::set_follows_mute`; #1382 D3 calls it from the shared commit.
- **The browser's effective mute** is `user_mute || vca_mute || (any_solo && !solo_safe &&
  !solo)` (`crates/host-core/src/solo.rs:11-13`). Today host-web's own follow pass composes it
  (`hosts/host-web/src/lib.rs:5274-5290`). #1382 D5 deletes that pass; #1382 D3 composes the same
  term in the shared commit.
- **No browser command kind changes `follows_mute`.** Send records use kinds 13-15
  (`hosts/host-web/src/lib.rs:907-916`).
- **Route lanes exist only when the plan is prepared with route lanes.** See
  `crates/host-core/src/prepare.rs:1574-1591`: no control channel, or `lanes.routes` false,
  attaches none.
- **The classifier.** `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`) still has
  the #1053 G2 guard (`:133`, `:278`, `:542-546`). #1226 removes it.

## Decisions frozen for this slice

- **D1. One path.** The browser receives this edit only as a transaction through the Worker's
  committed model (D15-11). This slice adds no worklet command kind and no SDK live-control
  method.
- **D2. One composition.** The follow records come from the shared commit: #1226 D3-D5 for the
  model terms (own mute, VCA mute) and the ramp (the session mute length), and #1382 D3 for the
  solo term and the mirror flag. host-web adds no composition and keeps no second copy. If a
  test of this slice finds the shared composition wrong for the browser, the fix goes into the
  shared commit through #1226 or #1382, never into host-web.
- **D3. No route lane, no live.** Every browser entry that attaches live controls attaches a lane
  per route into a submix (`HostLiveLanes::ALL`, `crates/host-core/src/prepare.rs:382-387`). If a
  plan has no lane for the route (live controls off), the shared classifier returns `rebuild`, the
  browser's structural path (*Replace the running browser session in the Rust host*, #1290). This
  is a correct path, not a fallback.
- **D4. Acked-batch question: can an ack ever precede a drop? No.** This slice adds no queue and
  no write path. The shared commit's checks run before its first write (#1226 D4, #1382 D4), and
  route lanes are latest-target cells (#1347).

## Deliverables

1. Native tests in `hosts/host-web/src/tests.rs`, and the integration test binary
   `hosts/host-web/tests/follows_mute_realtime.rs` (gate 4).

## Authorized paths

- `hosts/host-web/src/tests.rs`, `hosts/host-web/tests/follows_mute_realtime.rs` (new)

## Non-goals

- No change in `crates/host-core` or `crates/control-plane`: #1226 owns the classifier rule and
  the mirror setter, and #1382 owns the solo term in the shared commit.
- No C ABI change. No follow on routes into the output: those are refused at validation and
  never live.
- No SDK surface. `engine.apply` is not changed here.

## Hazards

- **A redundant record moves bits.** Re-entering the ramp kernel on a settled lane can turn `+0.0`
  into `-0.0` (`crates/host-core/src/solo.rs:58-70`). The shared commit writes a record only when
  a target changes (#1226 D3); gate 2 holds the browser to it.

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
     (the route lane's cell is unchanged).
   - Then mute the source, and then solo another track. The send follows each, and the output
     matches a fresh plan.
3. **Each preparation branch.** Gate 1's toggle is `live` with no spectrum, with a single
   spectrum capture and with a spectrum collection (the three branches of `compile_ready`,
   `hosts/host-web/src/lib.rs:6617`). With live controls off it is `rebuild`, and the output after
   adoption equals a fresh plan of the committed model.
4. **Realtime.** Integration test binary `hosts/host-web/tests/follows_mute_realtime.rs`.
   Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
   never in `src/tests.rs`. It links `bench_support::alloc`, calls `assert_installed()` first, and
   runs gate 1's script. The render thread's thread-scoped counters read `allocations == 0 &&
   frees == 0` around every render call after warm-up.
5. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support` (runs gate 4's binary)
   - `bash scripts/check-web-audioworklet.sh`
   - `bash scripts/test-web-audioworklet.sh`
   - the browser legs of *Run the browser control plane in a Worker and keep the AudioWorklet
     render-only* (#1332), gate 7 (`npm run qualify ... --check-matrix --self-test-mutations` in
     all three browsers)
   - `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: turns red if a `follows_mute` toggle is still rebuilt in the browser, zeroes the wrong
  column or lane, carries a stale gain, or ignores the solo term of the browser's effective mute.
  #1226's gates run on the C ABI, which has no solo.
- Gate 2: turns red if the toggle re-emits an unchanged target (sign of zero moves), or if the
  mirror's flag is not updated, so a later source mute or solo is not followed.
- Gate 3: turns red if one preparation branch drops the route lanes, so the toggle falls to
  `rebuild` there, or if the no-lane case is refused instead of rebuilt.
- Gate 4: turns red if the follow composition runs on, or allocates on, the render thread.

## Dependencies

- *Let C ABI sends follow their source strip's mute live* (#1226): the classifier rule, the
  follow composition and the mirror setter.
- *Admit browser live edits in the Worker through the committed model* (#1382): browser
  transactions reach the shared commit only through it, and its D3 composes the solo term.
- *Hold route-lane values in latest-target cells* (#1347): route lanes are cells.
- *Report each transaction's edit path in its response* (#1313).
- *Replace the running browser session in the Rust host* (#1290): gate 3's `rebuild` case.
