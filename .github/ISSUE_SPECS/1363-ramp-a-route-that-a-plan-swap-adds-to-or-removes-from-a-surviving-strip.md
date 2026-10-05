# Ramp a route that a plan swap adds to or removes from a surviving strip

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

When a structural edit adds a route (a send, or a strip's path to an output) between two strips
that both keep playing, the route's signal enters at its destination with a fade instead of a step.
When it removes such a route, the signal fades out instead of stopping dead. Every other path stays
bit-identical. D15-9 covers added, edited and removed strips (#1288, #1324, #1325); a route whose
two ends survive was left with a step at its destination.

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

- **D1. Which routes.** A route whose source strip and destination exist in both the displaced
  plan and the successor, and whose ID is in exactly one of the two models (route IDs are stable;
  a changed destination is a removal plus an addition). A route whose end is added or removed is
  covered by that strip's fade (#1288, #1325). A value change on a route that stays is not this
  issue (#1225).
- **D2. Transition route.** The graph gains a route op form that ramps without a lane:
  `LiveRoute::control` becomes `Option`. A transition route binds in this form, never folds, and
  takes an entry in #1288's fade table (D3 there), with `claim = None`.
- **D3. Added route.** Prepared settled at `[+0.0; 4]`, muted. Its fire retargets the ramp to the
  route's prepared gated coefficients (`mute = false`) over `N`. Its delay is its
  `compensation_delay`, so the fade starts at the first block boundary at or after the moment the
  signal leaves the line at rest.
- **D4. Removed route.** The successor is compiled from the committed model plus each removed route
  at its predecessor values (a "fading route"; the committed model and snapshots never contain it).
  Its compensation line carries as any line does (#1283), so the signal continues. It is prepared at
  its predecessor coefficients, and its fire, at the adoption block (delay 0), retargets to
  `[+0.0; 4]`, `mute = true` over `N`.
- **D5. Retiring a fading route.** The inventory lists each fading route with the revision that
  removed it. A later successor omits it when the watermark (#1314) shows that revision in effect
  at `S_r` and `render_sample >= S_r + N`; otherwise it keeps it, with its ramp state carried as a
  live send ramp (#1284). Omitting it removes a silent, muted edge; a latency drop it causes is held
  by the floors (#1285).
- **D6. Length.** `N = LiveRamps::for_session(next model).mute_samples` (a route add or removal is a
  send switched on or off). After the ramp an added route mixes the reference's bits.
- **D7. Reporting.** No scheduled swap; the revision completes at adoption as `EXACT` (or
  `SUPERSEDED`), never a fallback flag (those belong to #1358).
- **D8. Realtime.** No allocation; table and ramps are sized at preparation. Only transition routes
  lose the fold.

## Deliverables

1. D2 in `crates/graph`; D1, D3-D6 in host-core successor preparation and the inventory.
2. Tests below. Header comment and `docs/C_ABI_V1_QUALIFICATION.md` text.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/transition.rs`,
  `crates/host-core/tests/successor_swap.rs`
- `crates/capi/include/miso_engine_v1.h` (comments only), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Value edits of a surviving route (#1225 and its browser twin).
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
4. **Fading route retires.** A second structural transaction prepared after `S + N` drops the fading
   route with no change in output; one prepared before it keeps the ramp going: gate 2's blocks hold.
5. **Realtime.** Zero allocations and frees over the swap and ramp blocks
   (`the_swap_block_allocates_and_frees_nothing`, `successor_swap.rs:476`).
6. Commands:
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
- Gate 3: a transition that touches other routes or folds a ramp into a neighbour turns it red.
- Gate 4: a fading route dropped mid-ramp, or never dropped, turns it red.

## Dependencies

- *Fade in a strip that a swap adds during playback* (#1288).
- *Carry compensation lines across a plan swap* (#1283).
- *Carry strip delay lines and live send ramps across a plan swap* (#1284).
- *Keep every node's latency from dropping during playback* (#1285).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
