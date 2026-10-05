# Write a following send's route cell from the shared commit

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A6 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in the same push as drafts 13a
*Render stored mute automation on the strip* and 13b *Follow an automated mute on following sends
in render*.**

## Product outcome

On a send that follows a strip whose mute the session automates, every live edit that changes the
send's gate reaches it, on both hosts, through the shared commit: a route gain, matrix or mute edit,
a live `follows_mute` toggle, and a change of the source strip's static mute, VCA mute or solo. Each
writes the route's seven-word cell (draft 13b D1) and gives no folded follow record, so render
composes the gate with the curve at the next event, and nothing a live edit writes can contradict
the curve. Solo and VCA mutes compose with the curve for the send as for the strip.

## Context

- **The C ABI's route records.** *Deliver value-only send edits to the running C ABI plan* (#1225)
  D3 emits route records from `route_target(model, route)`; *Let C ABI sends follow their source
  strip's mute live* (#1226) D2-D3 makes `follows_mute` live and builds follow records from each
  strip's effective mute; #1226 D7 adds `LiveRouteState::set_follows_mute`. #1226 D6 leaves the
  follow of an automated mute to #1058.
- **The browser.** *Make a send's follows_mute live in the browser* (#1342) D1-D2: the browser
  receives the edit only as a transaction through the Worker's committed model, and its follow
  records come from the shared commit. *Admit browser live edits in the Worker through the committed
  model* (#1382) D3 composes the solo overlay and the route mirror in the shared commit.
- **The mirror.** `LiveRouteMuteFollow::delta` (`crates/host-core/src/live_route_state.rs:232-257`)
  yields only changed routes; the mirror holds `follows_mute` and `source_lane_muted` (`:46-53`).
- **The cell.** Draft 13b D1: for a route into a submix whose source strip has an automated mute
  lane, the cell holds the open transform, a flags word (route `mute`, `follows_mute`, `terms[2]`,
  `automated[2]`) and the ramp.

## Decisions frozen for this slice

- **D1. Route edits write the cell.** For a route of draft 13b D1, the shared classifier emits one
  route-cell write instead of the route records that #1225 and #1226 emit for that route (#1226 adds no record type
  of its own, #1226 D3). It carries the
  open transform from `route_values` (`crates/graph-compiler/src/ids.rs:309-322`), the route
  `mute`, `follows_mute` and `terms`, and the ramp from #1226 D5's rule. It writes only when a word's
  bits change. The path is `live`.
- **D2. Terms.** `terms[lane]` is, for an automated source lane, `vca_mute || solo_mute`; for a lane
  that is not automated, its whole effective mute (#1247 D2's effective mute, with #1382 D3's solo
  term). A change of any of them writes the cell. On the C ABI the overlay is absent, so the solo
  term is false; the code is the same on both hosts.
- **D3. `follows_mute`.** A toggle writes the cell and, through #1226 D7's setter, the mirror, in the
  same shadow as #1382 D3 requires, so a later solo change composes with the new flag.
- **D4. No second path.** #1225, #1226, #1342 and #1347 are not amended: they land before this slice,
  and this slice adds the branch for routes whose source mute is automated to the code they built.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** Every check of a
  transaction and of the overlay precedes the first cell write; writes cannot fail; render
  generates curve events from the plan.

## Deliverables

1. D1-D3 in the shared classifier and the commit path, with tests.

## Authorized paths

- `crates/host-core/src/{live_delta.rs,live_route_state.rs}`,
  `crates/host-core/tests/{live_delta.rs,live_routes.rs}`
- `crates/control-plane/src/` (the route-cell write and the overlay composition)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`

## Non-goals

- The route lane, its layout and the route op (draft 13b). Send automation (OQ2).

## Hazards

- **One push with 13a and 13b.** Root merges the three together.
- **A redundant write moves bits** (README "Change only"). D1 writes only changed words.

## Objective gates

1. **`follows_mute` toggle** (both hosts, new). On a playing engine with `t` automated-muted, a
   transaction (C ABI) and a browser edit turn `follows_mute` off, then on: each is `live`; after
   the route ramp the output equals a fresh plan of the committed model.
2. **Composition** (browser, new). Solo on another track mutes `t` by solo while its curve is 0; the
   send follows; unsolo restores the curve's state. A VCA mute of `t` composes the same way on both
   hosts. A route into the output never follows.
3. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A `follows_mute` toggle on a draft 13b
   route gives one route-cell write and no follow record; a static mute change on the source's
   non-automated lane gives that lane's `Mute` record and one route-cell write; a route gain edit
   gives one route-cell write with the new transform; an edit that leaves every word unchanged gives
   nothing.
4. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a`) in `.github/workflows/qualification.yml`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
5. **No rendered bit moves** for a session with no stored automation: `cargo build --locked
   --release -p audit -p capi && ./target/release/audit capi` gives the same `pcm_digest` at base
   and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if `follows_mute` is still read from the prepared route only, so a live toggle is lost
  on an automated route.
- Gate 2: red if solo or a VCA mute overwrites the curve instead of composing with it for the send.
- Gate 3: red if an edit still emits a folded record that the curve's next event would contradict,
  or writes an unchanged cell.

## Dependencies

Batch R2. Direct dependencies:

- Draft 13b *Follow an automated mute on following sends in render* (same push).
- *Make a send's follows_mute live in the browser* (#1342): the browser's `follows_mute` edit. It
  brings #1225, #1226 and #1382.
