# Admit browser live edits in the Worker through the committed model

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-10, D15-11, D15-17).
Code anchors verified on `main` at `6fb211594`.

This slice follows *Swap and retire browser plans through the Worker's service loop* (#1381).

## Product outcome

In the browser, every live edit commits to the Worker's committed model, through the same
control plane, classifier and commit as the C ABI. A live fader, mute, pan, matrix, trim,
polarity, input-filter, effect, send or VCA edit advances the committed revision. Its reply
carries `{revision, path}`, and render applies it from the plan's live lanes. A structural edit
(#1290) therefore prepares from a model that already holds every live value. No merge is needed,
and nothing is lost.

The worklet no longer admits commands. The SDK's live-control API does not change.

Solo, observe subscriptions and the meter lease have no transaction form. They stay a host-local
overlay in the Worker's control half. This slice owns how the shared commit composes the solo
term into effective mute, for strips and for sends that follow them.

## Context

- **Admission runs on the worklet thread today.**
  - `miso_engine_web_v1_command_submit` (`hosts/host-web/src/ffi.rs:3839`) calls
    `AudioWorkletEngineHost::submit_commands` (`hosts/host-web/src/lib.rs:3322`), then
    `admit_commands` (`:4596`) and `admit_commands_staged` (`:4641`).
  - This is host-web's own admission: kinds 1-17 (`:827-939`), composed with solo, VCA and
    follow state (the follow pass at `:5274-5290`).
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
  - It first compiles the prospective session, in `prepare_command_frame` (`control.rs:855-862`).
    Then `commit_live` (`:1065`) reads it (`:1066-1070`), classifies, checks everything, pushes,
    and commits. Compiling allocates. #1309 moves both into `crates/control-plane`.
  - The C ABI has no solo. *Let C ABI sends follow their source strip's mute live* (#1226) makes
    the shared commit build follow records from each strip's effective mute (own or VCA mute),
    and adds `LiveRouteState::set_follows_mute` for the browser.
- **Ramps.** Session matrix and pan edits carry `smoothing_samples`
  (`crates/protocol/src/session_wire.rs:881-912`). Decision 15 gives every live edit an optional
  per-edit ramp length, carried end to end: absent means the session default, and an explicit 0
  is a legal step. *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and
  pan changes* (#1054) owns the session key and its defaults; *Carry an optional per-edit ramp
  length on live session edits* (#1394) owns the transaction field `ramp_samples: Option<u32>` on
  the live-value edits (its D1) and `LiveRamps::resolve` (its D5); *Resolve an absent live ramp to the
  session default on the browser and in the SDK* (#1364) owns the browser's absent sentinel
  (`SESSION_DEFAULT_RAMP = u32::MAX`, #1364 D1) and its resolution.
- **Solo state.** Solo lives in `LiveControlSoloState` (`crates/host-core/src/solo.rs:129`). The
  session model and the protocol have no solo field. #1057 item 8 is the design note's ruling on
  host-local monitoring state.

## Decisions frozen for this slice

- **D1. Where.**
  - `worker` mode: the page-to-control-plane port of #1332 D7 carries the existing internal
    command message to the Worker. The Worker calls the admission export there. The worklet's
    export set drops `command_submit`.
  - `single` mode: the worklet's control handler calls the same export in its message handler,
    never inside `process()` and outside the render-locked window. The shared apply compiles the
    prospective session there, so it allocates on the audio thread's realm. Decision 15 accepts
    exactly this for a non-isolated page (D15-10, single mode): every such allocation and free is
    counted in #1332's `singleModeControlAllocations` and reported, and the render-locked count
    stays exactly 0.
- **D2. Lowering.**
  - The Worker lowers each batch's model-value records (kinds 1-6 and 10-17) into one session
    transaction at the committed revision. It applies that transaction through
    `control_plane::SessionState`'s transaction apply.
  - Ramps: each ramped record's length word lowers into the op's `ramp_samples` field (#1394 D1).
    The sentinel `SESSION_DEFAULT_RAMP` lowers to `None`, which the classifier resolves to the
    session default (#1394 D5, the same rule #1364 applies to commands). Any other value, including
    0, lowers to `Some` of that many samples. Kinds with no ramp (effect parameter 5, bypass 6,
    input filters 12) carry none; the bypass crossfade's length is the session mute ramp (#1341).
  - Composition (strip, VCA, follow, solo) comes from the shared commit (D3), never from a
    second browser copy.
  - The reply adds `revision` and `path` to today's command report. The message stays internal
    (D15-11).
- **D3. The host-local overlay, composed in the shared commit.** This slice owns this change to
  `crates/control-plane`'s live commit.
  - Solo (kind 9), observe subscribe and unsubscribe (kinds 7 and 8) and the meter lease stay
    outside the committed model, as an overlay in the Worker's control half. The overlay never
    advances the revision and never appears in a path.
  - The live commit gains one optional input, the host overlay: a `LiveControlSoloState` and the
    `LiveRouteState` route mirror. The C ABI passes none; its records and bits do not change.
  - With an overlay, a strip's effective mute is `model mute || VCA mute || solo term`. The strip
    mute records and #1226's follow records are built from that effective mute. A solo change
    with no model change goes through the same code and yields the same records.
  - **Solo's ramp.** Solo has no session edit, so #1394's transaction field does not reach it.
    The solo record's own length word (`smoothing_samples`, `hosts/host-web/src/lib.rs:854-868`)
    becomes the overlay change's ramp: `SESSION_DEFAULT_RAMP` lowers to `None`, anything else
    (0 included) to `Some`. The commit resolves it with #1394 D5's
    `LiveRamps::resolve(<solo row>, ramp)`, and the solo row's session default is the
    `control_smoothing` mute key, `mute_ms` (#1054 D3's table). Every strip mute and follow
    record that the solo change produces takes that one length. A model edit in the same batch
    keeps its own resolved ramp for the records it alone produces; a record that both change is
    built once and takes the model edit's ramp.
  - When a transaction changes a route's `follows_mute`, the commit sets the mirror's flag with
    #1226 D7's `set_follows_mute`, in the same shadow, so a later solo change composes with the
    new flag.
  - A batch that mixes overlay and model records is all or nothing across both: the overlay's
    shadow commits or rolls back with the model commit.
  - Whether this overlay is right in the long term is ruled by #1057's design note (item 8). This
    slice implements the position that note evaluates. A different ruling reopens this decision
    through a new issue.
- **D4. Acked-batch rule.** Every fallible check over the whole batch (domain, the overlay's own
  checks, the protocol token) runs before the first write. A refusal leaves model, revision,
  overlay and lanes unchanged. Live values go through latest-target cells (#1312, #1345, #1346,
  #1347), which are never refused for room, so no live value meets BACKPRESSURE. An ack therefore
  never precedes a drop.
- **D5. Superseded code.** host-web's own admission composition is deleted, not kept beside the
  shared path: the strip, solo, VCA and follow passes inside `admit_commands_staged`. Its tests
  that assert host-web-only composition move to the shared path's equivalents, or are deleted in
  the same PR. The static gate's worklet leg for `command_submit`
  (`check-web-audioworklet.sh:494-500`) is removed with the export's worklet role.

## Deliverables

1. D3 in `crates/control-plane`'s live commit.
2. D1, D2, D5 in `hosts/host-web/src/{lib.rs,ffi.rs,control_targets.rs}`, the control Worker and
   worklet scripts, and `scripts/check-web-audioworklet.sh`.
3. Native tests in `hosts/host-web/src/tests.rs`, and the integration test binary
   `hosts/host-web/tests/worker_admission_realtime.rs` (gate 4).
4. `sdk/src/browser/shipped-host.d.ts` and the host `.d.ts`: the reply's two new fields.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/tests/worker_admission_realtime.rs` (new),
  `hosts/host-web/web/`, `hosts/host-web/qualification/`
- `sdk/src/browser/shipped-host.d.ts`, `sdk/src/browser/live-controls.ts` (the reply fields
  only)
- `scripts/check-web-audioworklet.sh`, `scripts/check-web-audioworklet-callgraph.py`
- `scripts/test-web-audioworklet.mjs` (the fake exports and messages for admission moving to the
  Worker)
- Outside stream H's ownership; root sequences it after #1309 and #1226:
  `crates/control-plane/src/`, D3's overlay input and composition only.

## Non-goals

- No change to the shared classifier's rows (streams B and F own them). The only shared-commit
  change is D3.
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
2. **Ramps carry end to end.** For fader, mute, pan and solo: a record with an explicit ramp of 0
   steps at the next block; a record with an explicit ramp of 480 reaches its target after 480
   samples; a record with the sentinel ramps over the session's `control_smoothing` length (for
   solo, `mute_ms`).
3. **The overlay.**
   - Solo, observe subscribe and the meter lease leave the revision unchanged.
   - Soloing track 2 mutes the others, and a send that follows a soloed-out strip follows it,
     through the same render result as today: a fresh plan of the committed model plus the same
     solo command.
   - A structural transaction (#1381's republish hook) keeps the overlay in effect.
4. **All or nothing.** A batch of a valid fader record, a solo record and an out-of-domain pan
   record is refused. Model, revision, overlay and lanes are unchanged. The next 8 blocks are
   bit-identical to a run without the call.
5. **Realtime.** Integration test binary `hosts/host-web/tests/worker_admission_realtime.rs`.
   Decision 15 rules that host-web's native allocation-count gates live in an integration binary,
   never in `src/tests.rs`. It links `bench_support::alloc` and calls `assert_installed()` first.
   The render thread's thread-scoped counters read `allocations == 0 && frees == 0` around every
   render call while the Worker thread admits 1,000 batches. In the browser,
   `miso_engine_web_v1_render_allocation_count` stays 0 on both legs (#1333), and on the
   non-isolated leg `singleModeControlAllocations` grows across the live-control rows.
6. **The C ABI is unchanged.** `cargo test --locked -p capi` passes, and
   `target/release/audit capi` reports the same `pcm_digest` as the base (PR evidence).
7. **The SDK is unchanged for its users.** Every live-control row of the browser qualification
   passes unchanged in all three browsers, on both legs.
8. **Commands:**
   - `cargo test --locked -p host-web --features host-web/test-support` (runs gate 5's binary)
   - `cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`
   - `cargo build --locked --release -p audit -p capi && target/release/audit capi`
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
  defect #1291/#1292's merge existed to patch), or if a kind is lowered to the wrong field or lane.
- Gate 2: turns red if the lowering drops a per-edit ramp, reads an explicit 0 as "default",
  reads the sentinel as a length, or resolves solo's default from a key other than `mute_ms`.
- Gate 3: turns red if solo or a lease advances the revision, if the shared commit ignores the
  solo term for strips or following sends, or if the overlay is lost when the plan is swapped.
- Gate 4: turns red if any record or overlay change is written before a later check refuses.
- Gate 5: turns red if admission work runs on, or allocates on, the render thread, or if single
  mode's control allocations go uncounted.
- Gate 6: turns red if the overlay input changes C ABI records when it is absent.
- Superseded tests: host-web's composition tests named in D5 are moved or deleted in the same PR.

## Dependencies

- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Report each transaction's edit path in its response* (#1313).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Hold effect parameter, bypass and EQ-target values in latest-target cells* (#1345): lane cells.
- *Hold strip input-lane values in latest-target cells* (#1346): lane cells.
- *Hold route-lane values in latest-target cells* (#1347): lane cells.
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054): the session key and its defaults.
- *Carry an optional per-edit ramp length on live session edits* (#1394): the per-edit ramp
  field.
- *Resolve an absent live ramp to the session default on the browser and in the SDK* (#1364).
- *Let C ABI sends follow their source strip's mute live* (#1226): the follow composition D3
  extends, and the mirror setter.
- *Deliver value-only send edits to the running C ABI plan* (#1225): a shared classifier row.
- *Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan* (#1390): a
  shared classifier row.
- *Deliver value-only VCA edits to the running C ABI plan* (#1247): a shared classifier row.
- *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261): a shared
  classifier row.
- *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262): a shared classifier row.
- *Apply value-only submix-strip input-section and effect edits to the running C ABI plan*
  (#1267): a shared classifier row.
- *Design: one edit API on every host over the core's committed session model* (#1057), item 8,
  for D3's ruling.
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332): the
  `singleModeControlAllocations` counter.

The six shared classifier rows (#1225, #1390, #1247, #1261, #1262, #1267) cover every browser
kind, so that none becomes a rebuild.
