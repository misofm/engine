# Ramp a route that a plan swap adds to or removes from a surviving strip

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

When a structural edit adds a route (a send, or a strip's path to an output) between two strips
that both keep playing, the route's signal enters at its destination with a fade instead of a step.
When it removes such a route, the signal fades out instead of stopping dead. A route re-pointed to
another destination, source or tap does both, and its source strip keeps playing untouched. When a
strip fades in after a swap (added, duck-swapped or restored), its sends from taps before the fader
fade in with it. Every other path stays bit-identical. D15-9 covers added, edited and removed strips
(#1288, #1324, #1325) at their faders; the routes that bypass a fader were left with a step.

## Context

- A route: `Route { id, source, destination, channel_matrix, gain_db, mute, follows_mute }`
  (`crates/session/src/model.rs:782-800`); a destination is a submix input or an output input
  (`:823-831`). A muted route keeps its edge and compensation and contributes silence (`:793-795`).
- Only routes into a submix can be live: `attach_route_controls`
  (`crates/graph/src/lib.rs:1690-1697`) turns each such route op into a `LiveRoute`
  (`crates/graph/src/runtime.rs:861`), which ramps by an `IndexedRamp`
  (`crates/lane/src/kernels.rs:1097`, mixed by `route_mix_ramp_block`, `:1206`) and never folds.
  Routes into an output keep their prepared constants and their fold into the bank unit
  (`crates/graph/src/lib.rs:50`). `LiveRoute::control` is a required lane (`runtime.rs:861-877`).
  The C ABI attaches no route lane (`crates/capi/src/runtime/compile.rs:14-21`); the browser
  attaches them only with live commands enabled (`HostLiveLanes::routes`,
  `crates/host-core/src/prepare.rs:378`).
- A route op mixes its input after the edge's compensation delay (`LiveRoute::delayed`,
  `runtime.rs:868-870`). A new edge's compensation line starts at rest: it emits zeros for its
  `compensation_delay` (`RouteTiming`, `crates/graph/src/lib.rs:337`).
- So the predecessor cannot ramp most routes, and making every route live would give up the fold on
  every plan. Both ramps therefore run in the successor, from the adoption block.

## Decisions frozen for this slice

- **D1. Which routes.** Three cases, all decided on the control thread from the base model and the
  successor's model:
  - (a) **Added or removed.** A route whose source strip and destination exist in both plans and
    whose ID is in exactly one of the two models: D3 or D4.
  - (b) **Re-pointed (D15-9), or a prepared value change.** A route whose ID is in both models and
    either (1) whose source strip, tap or destination differs, or (2) whose source strip, tap and
    destination are equal but one of `gain_db`, `mute`, `channel_matrix` or `follows_mute` differs
    (or its `follows_mute` source's mute does) and the change is prepared on this host: the route
    has no live lane in the plan render runs or in the successor, or `classify_live_delta`
    (`crates/host-core/src/live_delta.rs:211`) refuses that change were it the only change to the
    base model. Today (2) is every send value on the C ABI, which attaches no route lane; it shrinks
    as *Deliver value-only send edits to the running C ABI plan* (#1225), *Let C ABI sends follow
    their source strip's mute live* (#1226) and *Make a send's follows_mute live in the browser*
    (#1342) make those changes live. A value change on a route with a live lane that the classifier
    accepts is carried and retargeted instead (#1284, #1225 D8) and takes no entry here. Either form
    is the old route removed (D4, a fading copy at its predecessor values that takes over the
    predecessor's line) plus the new route added (D3, at its successor values, its line at rest),
    both over `N`, so a prepared value change glides instead of stepping at `S`. Its source strip is never ducked:
    *Carry compensation lines across a plan swap* (#1283) and *Carry strip delay lines and live send
    ramps across a plan swap* (#1284) no longer put it in the restart set (stream A changes them).
  - (c) **Sends that bypass an armed fader.** Every route out of a strip whose fader the successor
    arms (#1288 D1, #1324 D3, #1325 D6) whose tap precedes the fader (`input`, `post_input`,
    `insert_send`, `insert_return`, `pre_fader`; `SendTap`, `crates/session/src/model.rs:856-871`).
    It is prepared and fired as an added route (D3), with the strip's claim. Routes from
    `post_fader` and `post_pan` pass the armed fader and are not armed.

  A route of case (a) or (b) whose source strip's fader is armed takes no entry of its own when its
  tap follows the fader (the fader's fade already shapes it; two ramps would multiply), and is case
  (c) when its tap precedes it. A route whose destination strip is added or removed passes that
  strip's fader and needs nothing here. The ramp-out of a removed or duck-swapped strip's pre-fader
  sends runs on the predecessor in phase 1 (#1325 D2, #1324 D4). A live value change on a route
  that stays is not this issue (#1225); a prepared one is case (b).
- **D2. Transition route.** The graph gains a route op form that ramps without a lane:
  `LiveRoute::control` becomes `Option`. A transition route binds in this form, never folds, and
  takes an entry in #1288's fade table (D3 there), with the claim D3 gives it. A route that has a
  live lane (*Hold route-lane values in latest-target cells*, #1347) keeps it and arms the same
  way; the arm is route-op state, not a lane record. Once *Give every route whose tap precedes its
  strip's fader a live lane on every plan* (#1391) lands, every route of case (c) is such a route.
- **D3. Added route.** Prepared settled at `[+0.0; 4]`, muted. Its fire retargets the ramp to the
  route's prepared gated coefficients (`mute = false`) over `N`. Its compensation line starts at
  rest, also in case (b) with unchanged endpoints, where #1283 D1 alone would carry the warm line
  under the shared route ID: D4's alias gives that line to the fading copy, never to both. Its
  delay is the latency from its
  source strip's input to its tap (0 for a route of case (a) or (b) from a surviving strip, whose
  tap is already warm) plus its `compensation_delay`, so the fade starts at the first block boundary
  at or after the moment the signal leaves the line at rest. Its claim is its source strip's claim
  in case (c), else `None`. Like every entry of #1288's table it fires no earlier than adoption.
- **D4. Removed route.** The successor is compiled from the committed model plus each removed route
  at its predecessor values (a "fading route"; the committed model and snapshots never contain it).
  Its graph ID is one no model contains, derived on the control thread from the route ID and the
  removing revision, and recorded in its inventory row (D5) so later successors reuse it.
  - **Line alias.** At the swap that creates it, the fading route's edges have no predecessor edge
    of their own ID, so #1283 D1 would start their lines at rest: `compensation_delay` samples of
    silence at `S`, then a cut. This issue therefore adds an alias to #1283's line lookup: each
    edge of the fading route carries from the predecessor's edge of the same kind
    (`GraphEdgeId::RouteSource` / `RouteDestination`, `crates/graph/src/lib.rs:306-311`) for the
    original route ID, under #1283 D1's same-endpoints rule (the fading route has the predecessor's
    endpoints, so it always holds). A predecessor line named by an alias carries only into the
    alias: the successor's edge under the original route ID starts at rest (D3), even when its
    endpoints are unchanged. At later swaps the fading route's edges exist under its own ID in
    both plans and carry by #1283 D1 without an alias.
  - So the signal continues: the predecessor's pending line content plays out under the fading
    route, which is prepared at its predecessor coefficients. Its fire retargets to `[+0.0; 4]`,
    `mute = true` over `N`.
  - **Fire block.** A fading route of case (a) has no replacement: its fire is at the adoption
    block (delay 0). A fading route of case (b) has one, the added route under the original ID
    (D3), whose line starts at rest and whose fire waits its `compensation_delay` `C`. The fading
    copy fires at its replacement's fire block (the same delay, so the same block), never at
    adoption. Until then it passes the source at its predecessor values, so the destination hears
    no dip: the two ramps run over the same `N` samples. Firing the copy at adoption would fade it
    out before the replacement fades in: with a true-peak limiter upstream, `C = rate / 100 + 6`
    (`crates/true-peak-limiter/src/lib.rs:236-242`, 486 samples at 48 kHz), which exceeds the
    default mute ramp of 480 samples (10 ms), so every prepared gain edit on such a route would
    drop out for about 20 ms.
- **D5. Retiring a fading route.** The inventory lists each fading route with the revision that
  removed it and its fire delay `d` (D4: 0 in case (a), its replacement's delay in case (b)). A
  later successor omits it when the watermark (#1314) shows that revision in effect at `S_r` and
  `render_sample >= S_r + f + N`, where `f = ceil(d / quantum) * quantum` is the offset of its fire
  block (adoption is at a block boundary, so the fire block is `S_r + f`); otherwise it keeps it,
  with its ramp state carried as a live send ramp (#1284), and a copy that has not fired yet keeps its pending fire (#1288's table carries it). Omitting it removes a silent, muted edge; a latency drop it causes is held
  by the floors (#1285).
- **D6. Length.** `N = LiveRamps::for_session(next model).mute_samples` (a route add or removal is a
  send switched on or off). After the ramp an added route mixes the reference's bits.
- **D7. Reporting.** No scheduled swap; the revision completes at adoption as `EXACT` (or
  `SUPERSEDED`), never a fallback flag (those belong to #1358).
- **D8. Realtime.** No allocation; table and ramps are sized at preparation. Only transition routes
  lose the fold.

## Deliverables

1. D2 in `crates/graph`; D1, D3-D6 in host-core successor preparation and the inventory; case (c)
   reads the armed strip set #1288's arm entry point receives.
2. Tests below. Header comment and `docs/C_ABI_V1_QUALIFICATION.md` text.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/transition.rs`,
  `crates/host-core/tests/successor_swap.rs`
- `crates/capi/include/miso_engine_v1.h` (comments only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Live value edits of a surviving route (#1225 and its browser twin); a prepared value change is
  case (b).
- Making every route live on every plan.
- No crossfade between plans; ghost strips stay deferred by D15-9 until a listening test measures
  an audible dip.

## Objective gates

In `successor_swap.rs`, quantum 128, `N = 2000`, at `Backend::Simd8`, `Backend::Simd4` and prepared
between render calls.

1. **Added route fades in.** Track A and submix B both play; a transaction adds route A -> B with a
   compensation delay of 300 samples. Every block equals a fresh plan of the successor session fed
   the same frames, in which the route is live, muted at start, and unmuted with `N` at the block
   that starts at or after `S + 300`: bit-identical.
2. **Removed route fades out.** The reverse transaction. Every block from `S` equals the
   predecessor's output with the route live-muted with `N` at `S`: bit-identical.
3. **Only the route moves.** With A's source fed exact zeros, gates 1 and 2 leave every block equal
   to a run without the transaction.
4. **Re-pointed send (D15-9).** Track A sends `post_fader` to submix B; a transaction re-points that
   route (same ID) to submix C. B's input equals gate 2's fade-out, C's input equals gate 1's
   fade-in, and A's own output to the master equals a run with no transaction, bit for bit (A is
   not ducked). The same with the tap changed from `post_fader` to `pre_fader` instead. Both cases
   are run a second time, at 48 kHz, with a track through a true-peak limiter into B and into C, so
   A's old edge into B and its new edge into C each carry a compensation delay (`C_B`, `C_C` > 0).
   Let `F` be the first block boundary at or after `S + C_C`. Before `F`, B's input from A equals a
   run without the transaction (the predecessor's `C_B` pending samples play out, not zeros) and
   C's input from A is exact zeros. From `F`, B's input is gate 2's fade-out and C's is gate 1's
   fade-in, both fired at `F`.
5. **Prepared send value change (D1 (b) (2)).** Prepared without route lanes (as the C ABI
   prepares before #1225): track A sends `post_fader` into submix B at `gain_db = -12`; a
   structural transaction adds a muted track and also sets that send to `gain_db = 0`. The
   reference is a plan compiled through D4's fading-route path with both routes, the old one
   (-12 dB) live-muted with `N` and the new one (0 dB) muted at start and live-unmuted with `N`,
   both fired at the same block `F`, the first block boundary at or after `S + C`. B's input from
   A equals it bit for bit in every block. The same with the send's `follows_mute` changed from
   `false` to `true` while A is muted.
   - Without compensation (`C = 0`), `F = S`.
   - At 48 kHz with a true-peak limiter on A's path into B, `C = 486`. Run it with `N = 2000` and
     with `N = 480` (the default mute ramp, shorter than `C`). From `S` to `F`, B's input from A
     equals the predecessor's output (the old copy at -12 dB, the pending samples included, no
     fade yet); from `F` both ramps run together, so with A fed a constant nonzero signal B's
     input from A is never zero.
   - With route lanes and #1225's classifier the gain edit instead carries and retargets: no
     fading copy is prepared.
6. **Pre-fader send of an added strip.** A transaction adds track T with a `pre_fader` send into
   submix R. R's input from T equals a fresh plan of the successor session with that route muted at
   start and unmuted with `N` at T's fader fire block (#1288 D3); before it, exact zeros.
7. **Fading route retires.** A second structural transaction prepared after `S + N` drops gate
   2's fading route with no change in output; one prepared before it keeps the ramp going: gate 2's
   blocks hold. With gate 5's compensated copy (`f = F - S`), a second transaction prepared after
   `S + N` but before `F + N` keeps it and gate 5's blocks hold; one after `F + N` drops it with
   no change in output.
8. **Realtime.** Zero allocations and frees over the swap and ramp blocks
   (`the_swap_block_allocates_and_frees_nothing`, `successor_swap.rs:476`).
9. Commands:
   - `cargo test --locked -p graph -p host-core -p capi --features graph/test-support,host-core/test-support,builtins-compiler/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-graph-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-cross-targets.sh`,
     `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a fade that starts at `S` and ignores the at-rest compensation line (the signal arrives
  mid-ramp as a step) turns it red.
- Gate 2: a removal that drops the route at `S`, or a fading route whose line does not carry, turns
  it red.
- Gates 4 and 5 with compensation: a fading copy whose line starts at rest (no alias: a
  `C`-sample hole at `S`), a same-endpoint new route that keeps the warm line (the pending
  audio played twice, once at each value), or a fading copy fired at adoption instead of at its
  replacement's fire block (a dip, and with `N = 480 < C` a full dropout, between the two fades)
  turns them red.
- Gate 3: a transition that touches other routes or folds a ramp into a neighbour turns it red.
- Gate 4: a re-point treated as a value change (a step at both destinations) or one that ducks
  the source strip turns it red.
- Gate 5: a prepared send value change rebuilt at its new constant (a step at `S`), or a live
  one that still takes a fading copy (a needless double route), turns it red.
- Gate 6: a pre-fader send left unarmed beside an armed fader (it enters at full level at
  adoption, a step at R) turns it red.
- Gate 7: a fading route dropped mid-ramp (retired at `S_r + N` though it fired at `S_r + f`),
  or never dropped, turns it red.

## Dependencies

- *Fade in a strip that a swap adds during playback* (#1288).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Carry compensation lines across a plan swap* (#1283).
- *Carry strip delay lines and live send ramps across a plan swap* (#1284).
- *Keep every node's latency from dropping during playback* (#1285).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
