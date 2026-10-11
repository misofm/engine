# Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-6, D15-7).
Code anchors verified on `main` at `6fb211594`.

Slice 27b of *Submix strips and live aux sends* (#1196): the submix-strip half of the former
*Deliver value-only send and submix-strip edits to the running C ABI plan*, split in the
decision-15 fix round. The send half is *Deliver value-only send edits to the running C ABI plan*
(#1225), which this issue builds on. The design record cited as `DESIGN` is
`docs/handoffs/submix-sends-2026-10-02/DESIGN.md`.

## Product outcome

A C ABI host, such as a fan's phone, can change a bus's (submix strip's) fader dB, lane mute, or
pan/matrix on the running plan. The change never rebuilds the plan, leaves a silent block, resets
a source ring or steps hard. It lands on the same plan, ramped, exactly as the same edit on a track
does today (#1257), and it is never lost, also when the same transaction makes a structural edit
that swaps the plan.

## Context

- **The classifier pairs tracks only.** `host_core::classify_live_delta`
  (`crates/host-core/src/live_delta.rs:211`) refuses a changed track set (`:219-227`), masks each
  track's `fader` and `matrix_or_pan` (`:238-239`) and compares everything else by canonical JSON
  (`:257-263`). Every submix field stays compared (doc `:155-157`, #1053 guard G1), so a submix
  fader edit is `LiveRebuild::Structure`. The per-track record loop is `:268-325`: it checks
  domains through `fader_gains` (`:516`, which calls `checked_fader_gain`) and
  `lower_matrix_or_pan`, emits one `FaderDb` and one `Mute` record per changed lane, and one matrix
  record when a lowered bit changes, into `LiveDelta::strips` (`:110`).
- **Guard G2 reads track sources only.** `follows_mute_from` (`:543-549`) matches
  `RouteSource::Track` only. Today that is enough, because a submix mute is structural. But
  preparation computes `follow_zeroed` for a route from either source kind
  (`crates/graph-compiler/src/compile.rs:327-354`), so a send whose source is a submix can follow
  the submix's mute.
- **Normalized order.** `compile_session` sorts tracks, submixes and routes by ID
  (`crates/session/src/compile.rs:137-155`).
- **The lanes exist already.** host-core attaches one control producer per strip, tracks first,
  then submixes, with the submix ID in `track_id` (`HostLiveControlHandles::strip_controls`,
  `crates/host-core/src/prepare.rs:424-433`). capi keeps all of them in its epoch
  (`StripLanes`, `crates/control-plane/src/control.rs:13-17`; built at
  `crates/control-plane/src/compile.rs:610-623`).
- **capi resolves track producers only.** `commit_live` (`control.rs:1201`) searches
  `strips.controls[..track_count]` (`:1229-1245`) by strip ID. #1309 moves `control.rs` unchanged
  to `crates/control-plane/src/control.rs`; capi's tests stay in capi. The anchors into the
  moved code name its locations in `crates/control-plane/src/`; the others name the locations at
  `6fb211594`.
- **Opcodes.** `SetTrackFader` `0x020f` and `SetTrackMatrixOrPan` `0x0210` address a submix by its
  ID (#1204).
- **After #1225:** route records and `LiveDelta::routes` exist; the classifier still treats every
  submix field as structural (#1225 D1, D2); live values are latest-target cells (#1312); and
  #1225 D8 retargets carried sends. *Carry fader, mute and pan ramps across a plan swap* (#1277 D5)
  retargets carried strips with the classifier's per-strip records.
- **Tests this slice supersedes:**
  - `a_submix_fader_change_is_structural` (`crates/host-core/tests/live_delta.rs:402`);
  - the G1 case "a submix fader" (`crates/capi/src/runtime/live_tests.rs:1058-1062`) of
    `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing` (`:1015`);
  - #1225's gate-2 case "`0x020f` on the bus `b`".

  `a_live_track_edit_beside_a_submix_strip_reaches_its_track` (`live_tests.rs:1115`) stays.

## Decisions frozen for this slice

- **D1. Pairing.** The classifier pairs submixes exactly as it pairs tracks. If the submix IDs
  differ, by count or pairwise in normalized order, the delta is `LiveRebuild::Structure`. The
  step-3 mask then copies each submix's `fader` and `matrix_or_pan` from `current`, as it does for
  tracks. Every other submix field stays compared: builtins (until *Apply value-only submix-strip
  input-section and effect edits to the running C ABI plan*, #1267), console entries, inserts.
- **D2. One per-strip function.** The record loop body becomes one function over a pair of
  `StripRef`s (`crates/session/src/model.rs:339-354`), or over the two fields it reads. It runs
  for each track pair and then for each submix pair. A submix gets exactly the track rules: the
  same domain checks, one `FaderDb` per changed lane, one `Mute` per changed lane, one matrix
  record when a lowered bit changes, and the same ramp lengths (`ramps.fader_samples`,
  `ramps.mute_samples`, the lowered matrix smoothing), including whatever per-edit ramp the track
  records take (D15-1's recorded resolution; the strip's `EditRamps` entries through
  `LiveRamps::resolve`, *Carry an optional per-edit ramp length on live session edits*, #1394
  D6). If #1277 has already extracted
  this function (its D5), reuse it. Never write a second copy.
- **D3. Order.** `LiveDelta::strips` holds the tracks first, then the submixes, each in canonical
  ID order. That is the order of `HostLiveControlHandles::strip_controls`.
- **D4. Guard G2 covers submix sources.** `follows_mute_from` matches a `RouteSource::Submix` as
  well as a `RouteSource::Track`. A lane-mute change on a bus that a `follows_mute` route in `next`
  reads stays `LiveRebuild::FollowedMute` until *Let C ABI sends follow their source strip's mute
  live* (#1226) removes the guard. Without this, a bus mute would be acked live while its
  following send kept the old gate.
- **D5. Resolution.** `commit_live` resolves strip producers over all of `strips.controls`, not
  `[..track_count]`, still by strip ID and with the same wrapping cursor. Strip IDs are unique
  across tracks and submixes, so no ID can match two producers. No new producer, cell or resource
  row: the submix producers are already prepared and charged.
- **D6. Commit order and acks (D15-2).** Unchanged from #1225 D6: classify, build every record,
  run the live admission, resolve every producer, run the protocol's commit predicate, write the
  cells, commit. Every fallible step runs before the first cell write. A submix cell write cannot
  fail and never returns `BACKPRESSURE`. Acked-batch question: an ack can never precede a drop,
  because a later commit before the same block's live snapshot supersedes the earlier one and
  #1312's `live_values_superseded` counts it (#1312 D2, D11). Every value of one transaction,
  submix and track strips alike, takes effect in the same block (#1312 D1; Amendment 1).
- **D7. Carry, then retarget (D15-7).** A carried submix strip whose fader, mute or pan differs
  between the base and the successor's model is retargeted, not restarted. #1277 D3 calls a value
  live when the classifier would emit a record for it, so after D1-D2 these values are live.
  #1277 D5's successor loop runs D2's function for submix pairs as well as track pairs and writes
  the records into the successor's cells.

## Deliverables

1. D1-D4 in the classifier.
2. D5 in the control plane's `commit_live`.
3. D7 in host-core's successor preparation.
4. In `docs/C_ABI_V1_QUALIFICATION.md` (the paragraph at `:78-81` and the "Rebuild" bullet at
   `:280-284`, as #1225 leaves them): a submix strip's fader, mute and pan are value-only. What
   still rebuilds for a submix: its input section and its effects (until #1267), every other
   submix field, and a mute change on a bus that a `follows_mute` send reads (until #1226).
5. The superseded tests in Context, inverted or replaced in the same PR (gate 4).

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the classifier).
- `crates/control-plane/src/control.rs`: `commit_live` only (moved there by #1309).
- `crates/host-core/src/prepare.rs`: #1277 D5's successor retarget loop only.
- These test files:
  - `crates/host-core/tests/live_delta.rs`;
  - `crates/capi/src/runtime/live_tests.rs`;
  - `crates/capi/tests/resource_lifecycle.rs`;
  - `crates/host-core/tests/successor_swap.rs` and `crates/host-core/tests/support/successor.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`, and this spec.

## Non-goals

- No send or route change (#1225). No `follows_mute` change and no follow-source mute (#1226). No
  VCA (#1247).
- No submix input section and no submix effect (#1267).
- No browser change. No new C symbol, opcode or field, no struct layout change, no new resource
  row.

## Hazards

- **Addressing.** Strips are found by ID through the producer table. Never index into the model's
  vectors, and never assume a submix index equals `track_count + i` of some other plan.
- **A muted bus with a delayed send.** A send into the bus keeps running while the bus is muted
  (`DESIGN.md` P4). The bus's comparison point is its own; add the send's compensation delay only
  where the send's output is compared.

## Objective gates

1. **PCM through the C ABI** (new tests beside `live_pcm_shape`, `live_tests.rs:375`).
   - Fixture: `submix_session(false)` (`live_tests.rs:969`): bus `bus`, a send into it, and the
     bus's route into the output. Each lane carries a distinct, non-constant signal.
   - Through `SESSION_TRANSACTION_APPLY`, apply `0x020f` (a dB change, then a left-lane mute, then
     the unmute) and `0x0210` (an asymmetric pan) on `bus`.
   - Each edit keeps the same plan: no new epoch, no pending candidate, no re-seek and no silent
     block.
   - From `latency_samples` plus the record's ramp, the output is bit-identical to a plan compiled
     from the committed model and fed the same sources from sample 0.
   - Run with 1 and 10 tracks at 44.1, 48, 88.2 and 96 kHz.
2. **One transaction, track and bus.** One transaction changes track `eq0`'s fader and `bus`'s
   fader. Both land on the same plan, each on its own strip, and the output equals the reference.
3. **The boundary of liveness.** Each of these still rebuilds (a new epoch), and the plan then
   renders the committed model:
   - a `bus` input `trim_db` edit (until #1267; this replaces the "a submix fader (G1)" case);
   - a left-lane mute on a bus that a second bus reads through a `follows_mute` send (D4).
4. **Classifier unit tests** (`crates/host-core/tests/live_delta.rs`).
   - `a_submix_fader_change_is_structural` becomes `a_submix_fader_change_is_live`: one strip
     record, addressed by the submix ID, with the track fader length.
   - A delta that changes a track and a submix lists the track first.
   - A changed submix set is `Structure`.
   - A bus mute read by a `follows_mute` route from that bus is `FollowedMute`.
5. **No ack before a drop.** A transaction whose second strip value (a bus pan the setter refuses)
   fails the domain check writes no cell; the model, the revision and the replay cache are
   unchanged. Twenty consecutive bus fader edits with no render between them all return `OK`; the
   next render converges to the last value, and `live_values_superseded` rises by exactly 19.
6. **Carry, then retarget** (`crates/host-core/tests/successor_swap.rs`). A has a bus. B is A plus
   a muted track whose ID sorts first, and B also changes the bus fader. Run 1 is the structural
   swap with D7's retarget. Run 2 writes the same record to A just before the swap and prepares B
   from a base that holds it. Every block of the two runs is bit-identical.
7. **Realtime.** Extend
   `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
   (`crates/capi/tests/resource_lifecycle.rs:2900`) with bus edits, over 20 runs:
   `allocations == 0` and `frees == 0` around every render call after warm-up, zero `INTERNAL`
   results, and a final block bit-identical to a fresh plan of the final committed model.
8. **Commands.**
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`
   - The workspace test command of `.github/workflows/qualification.yml` (the "Workspace debug
     tests" step), with `control-plane/test-support` added by #1309.
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`: zero allocations, locks and syscalls.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`.
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job.

## Test value

- Gate 1 turns red if a bus edit is still structural, takes a rule other than a track's, or lands
  late or unramped.
- Gate 2 turns red if `commit_live` still searches only `[..track_count]` (the bus record would
  return `INTERNAL`), or if a cursor bug sends the bus record to a track.
- Gate 3 turns red if the mask copies more than the fader and pan of a submix, or if G2 still
  matches track sources only: the bus mute would be acked live while its follower kept the old
  gate.
- Gate 4 turns red on a wrong record order (tracks and submixes interleaved), a missing submix-set
  check, or a G2 that ignores submix sources. It is the fast unit form of gates 2 and 3.
- Gate 5 turns red if a submix cell is written before a later check fails, or if a superseded
  write is lost without being counted.
- Gate 6 turns red if a carried bus keeps the predecessor's level after a transaction that also
  changed it, or if the retarget applies a block early or late.
- Gate 7 turns red if the widened resolution or the submix drain allocates on the render thread,
  or if a bus edit that races a swap reaches the retiring plan.

## Amendment 1 (root, 2026-10-05): revision-bounded cells

Root's binding requirement (2026-10-05) makes every live value of one revision take effect in one
block. Cells are read under each block's snapshot (#1432 Amendment 1, #1312 Amendment 2). D6's
supersession wording follows.

## Dependencies

- *Deliver value-only send edits to the running C ABI plan* (#1225)
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Carry fader, mute and pan ramps across a plan swap* (#1277), for D2's function and D7
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), for the lengths in D2
- *Carry an optional per-edit ramp length on live session edits* (#1394), for D2's per-edit ramp
