# Admit browser live edits in the Worker through the committed model

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This slice follows *Swap and retire browser plans through the Worker's service loop* (#1381).

## Product outcome

In the browser, every live edit commits to the Worker's committed model, through the same
control plane and classifier as the C ABI. A live fader, mute, pan, matrix, trim, polarity,
input-filter, effect, send or VCA edit advances the committed revision. Its reply carries
`{revision, path}`, and render applies it from the plan's live lanes. A structural edit
(#1290) therefore prepares from a model that already holds every live value. No merge is
needed, and nothing is lost.

The worklet no longer admits commands. The SDK's live-control API does not change.

Solo, observe subscriptions and the meter lease have no transaction form. They stay a host-local
overlay in the Worker's control half.

## Context

- **Admission runs on the worklet thread today.**
  - `miso_engine_web_v1_command_submit` (`hosts/host-web/src/ffi.rs:3839`) calls
    `AudioWorkletEngineHost::submit_commands` (`hosts/host-web/src/lib.rs:3322`), then
    `admit_commands` (`:4596`) and `admit_commands_staged` (`:4641`).
  - This is host-web's own admission: kinds 1-17 (`:827-939`), composed with solo, VCA and
    follow state (`:5274-5290`).
  - It writes plan lanes and never updates any session model; the browser has none.
- **The command kinds:**
  - model values: pan 1, matrix 2, fader 3, mute 4, effect parameter 5, effect bypass 6, trim 10,
    polarity 11, input filters 12, send gain 13, send mute 14, send matrix 15, VCA fader 16,
    VCA mute 17;
  - host-local: observe subscribe 7, observe unsubscribe 8, solo 9 (`:851-869`), and the meter
    lease export (`ffi.rs:3919`).
- **The static gate holds this export on the render thread.** It applies the allocation-only
  call-graph check to `command_submit` (`scripts/check-web-audioworklet.sh:494-500`).
  *Gate AudioWorklet render against allocation statically and at runtime* (#1333 D2) puts it in
  the render-locked set.
- **The C ABI's live path.**
  - `SessionState::command` (today `crates/capi/src/runtime/control.rs:843`) classifies a
    transaction with `classify_live_delta` (`crates/host-core/src/live_delta.rs:211`).
  - `commit_live` (`control.rs:1065`) checks everything, pushes, then commits.
  - #1309 moves both into `crates/control-plane`.
- **Ramps.** Session matrix and pan edits carry `smoothing_samples`
  (`crates/protocol/src/session_wire.rs:881-912`). #1054 (D15-1) gives every live row a model
  ramp, where 0 means the session default.
- **Solo state.** Solo lives in `LiveControlSoloState` (`crates/host-core/src/solo.rs:129`). The
  session model and the protocol have no solo field. #1057 item 8 is the design note's ruling on
  host-local monitoring state.

## Decisions frozen for this slice

- **D1. Where.** In `worker` mode, the page-to-control-plane port of #1332 D7 carries the
  existing internal command message to the Worker. The Worker calls the admission export there.
  The worklet's export set drops `command_submit`. In `single` mode, the worklet's own control
  handler calls it outside the render-locked window.
- **D2. Lowering.**
  - The Worker lowers each batch's model-value records (kinds 1-6 and 10-17) into one session
    transaction at the committed revision. It applies that transaction through
    `control_plane::SessionState`'s transaction apply.
  - A record's `smoothingSamples` becomes the edit's model ramp. A record with no ramp resolves
    to the session default (*Resolve an absent live ramp to the session default on the browser and
    in the SDK*, #1364).
  - Composition comes from the shared classifier and commit (strip, VCA, follow), never from a
    second browser copy.
  - The reply adds `revision` and `path` to today's command report. The message stays internal
    (D15-11).
- **D3. Host-local overlay.**
  - Solo (kind 9), observe subscribe and unsubscribe (kinds 7 and 8) and the meter lease stay
    outside the committed model. They live as an overlay in the Worker's control half.
  - The overlay never advances the revision and never appears in a path.
  - Effective mute is `model mute || VCA mute || solo term`. The commit composes it from the
    model and the overlay, through `LiveControlSoloState`.
  - A batch that mixes overlay and model records is all or nothing across both.
  - Whether this overlay is right in the long term is ruled by #1057's design note (item 8). This
    slice implements the position that note evaluates. A different ruling reopens this decision
    through a new issue.
- **D4. Acked-batch rule.** Every fallible check over the whole batch (domain, the overlay's own
  checks, room in the FIFO lanes, the protocol token) runs before the first write. A refusal
  leaves model, revision, overlay and lanes unchanged. Cell lanes (#1312) are never refused for
  room. An ack therefore never precedes a drop.
- **D5. Superseded code.** host-web's own admission composition is deleted, not kept beside the
  shared path. Specifically: the strip, VCA and follow passes inside `admit_commands_staged` that
  the shared commit now performs. Its tests that assert host-web-only composition move to the
  shared path's equivalents, or are deleted in the same PR. The static gate's worklet leg for
  `command_submit` (`check-web-audioworklet.sh:494-500`) is removed with the export's worklet
  role.

## Deliverables

1. D1-D5 in `hosts/host-web/src/{lib.rs,ffi.rs,control_targets.rs}`, the control Worker and
   worklet scripts, and `scripts/check-web-audioworklet.sh`.
2. Native tests in `hosts/host-web/src/tests.rs`.
3. `sdk/src/browser/shipped-host.d.ts` and the host `.d.ts`: the reply's two new fields.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/qualification/`
- `sdk/src/browser/shipped-host.d.ts`, `sdk/src/browser/live-controls.ts` (the reply fields
  only)
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`

## Non-goals

- No change to the shared classifier or commit (streams B and F own them).
- No new SDK edit API. `engine.apply` is #1296, the builder is #1383.
- No ruling on solo (#1057).

## Objective gates

1. **Every live kind commits to the model.** Native host-web test, with the control half and
   render half on two threads. Submit one batch per model-value kind (1-6, 10-17), on tracks,
   submixes, sends and VCAs. For each:
   - the reply is `path: "live"` with the revision advanced by 1, and the plan epoch is
     unchanged;
   - the committed document contains the edited value;
   - after the ramp, every block is bit-identical to a plan prepared from the committed model,
     with the same solo overlay, fed the same PCM from frame 0.
2. **The overlay.**
   - Solo, observe subscribe and the meter lease leave the revision unchanged.
   - Soloing track 2 mutes the others through the same render result as today: a fresh plan of
     the committed model plus the same solo command.
   - A structural transaction (#1381's republish hook) keeps the overlay in effect.
3. **All or nothing.** A batch of a valid fader record, a solo record and an out-of-domain pan
   record is refused. Model, revision, overlay and lanes are unchanged. The next 8 blocks are
   bit-identical to a run without the call.
4. **Realtime.** The render thread counts `allocations == 0 && frees == 0` around every render
   call (`bench_support::alloc`) while the Worker thread admits 1,000 batches. On the isolated
   browser leg, `miso_engine_web_v1_render_allocation_count` stays 0 (#1333).
5. **The SDK is unchanged for its users.** Every live-control row of the browser qualification
   passes unchanged in all three browsers, on both legs.
6. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`
   - `npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix --self-test-mutations`
     in `hosts/host-web/qualification`, for chromium, firefox and webkit
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `bash scripts/check-workspace-policy.sh`

## Test value

- Gate 1: turns red if a live edit reaches the lanes without reaching the committed model (the
  defect #1291/#1292's merge existed to patch), or if a kind is lowered to the wrong field, lane
  or ramp.
- Gate 2: turns red if solo or a lease advances the revision, or if the overlay is lost when the
  plan is swapped.
- Gate 3: turns red if any record or overlay change is written before a later check refuses.
- Gate 4: turns red if admission work runs on, or allocates on, the render thread.
- Superseded tests: host-web's composition tests named in D5 are moved or deleted in the same PR.

## Dependencies

- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Report each transaction's edit path in its response* (#1313).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), and #1364.
- The shared classifier rows for every browser kind, so that none becomes a rebuild:
  - *Deliver value-only send and submix-strip edits to the running C ABI plan* (#1225);
  - *Deliver value-only VCA edits to the running C ABI plan* (#1247);
  - *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261);
  - *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
    (#1262);
  - *Apply value-only submix-strip input-section and effect edits to the running C ABI plan*
    (#1267).
- *Design: one edit API on every host over the core's committed session model* (#1057), item 8,
  for D3's ruling.
