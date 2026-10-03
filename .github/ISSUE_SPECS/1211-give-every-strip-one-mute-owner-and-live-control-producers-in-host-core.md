# Give every strip one mute owner and live-control producers in host-core

Slice 14 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

host-core gives every strip, bus included, the live-control channels a track has: a matrix/pan, a
fader/mute and an input-section queue. And it gives every strip **one** mute owner: the solo state,
sized per strip, with submix entries solo-safe (never soloable, never solo-muted), so a soloed
track stays audible through its buses and returns (DESIGN P7).

Without one owner, a bus mute would have no control-plane home, and the follow-mute composition of
K3 could not read it (VERIFY-1 MAJOR-2). This slice is host-core only; the browser starts addressing
buses in *Address submix strips in browser live commands* (#1213).

## Context (verified on `fe8ac679`)

After *List every strip in the live-control handles and file bus effects in the browser* (#1207):
`HostLiveControlHandles.strips` lists tracks then submixes, `track_count` marks the prefix, the
control requests iterate the track prefix, and `canonical_index` covers every strip. After batch K1,
the builtins compiler's `known_tracks` set (`crates/builtins-compiler/src/lib.rs:3327-3332`) is a
strip set, so a `TrackControlRequest` may name a submix and gets that strip's three builtin queues.

- **Control requests** are built at `crates/host-core/src/prepare.rs:919-928`, one
  `TrackControlRequest { track_id, queue_capacity }` per entry of the local strip list's track
  prefix. They come back as `bound.track_controls`, checked against `control_requests.len()`
  (`:1177-1182`), refused with `host.control.order` if an ID is outside `canonical_index`, and sorted
  into canonical order (`:1191-1205`). The handle field is `HostLiveControlHandles.track_controls`
  (`:349`), built at `:1299-1306`.
- **Readers of `track_controls`:**
  - `crates/host-core/src/prepare.rs:524`, `:722` (`debug_assert!`s);
  - `crates/host-core/tests/collapse_arming.rs:288`;
  - `crates/host-core/tests/input_liveness_live_controls.rs:117`, `:212`;
  - `crates/host-core/tests/prepare.rs:713`, `:716`, `:731`, `:758`;
  - `crates/host-core/tests/randomized.rs:543`, `:653-655`;
  - `crates/host-core/tests/spectrum.rs:429`;
  - `crates/host-core/tests/symmetry_witness.rs:893`;
  - `hosts/host-web/src/lib.rs:5794`, `:6117`.

  #1053 (open) keeps `track_controls` in capi's `ProviderEpoch` (its Scope 2), so it will read this
  field.
- **The solo state** `host_core::LiveControlSoloState` (`crates/host-core/src/solo.rs`):
  - module doc composition `effective_mute(track, lane) = user_mute || (any_solo && !solo(track))`
    (`:11`); fields `:89-99`; `try_new(mutes: &[[bool; 2]])` `:113`, one entry per track;
  - `any_solo` `:141`, `solo` `:154`, `user_mute` `:160`, `emitted_mute` `:170`, the public
    `effective_mute(track, lane)` `:180-182`, `set_solo` `:191`, `set_user_mute` `:207`,
    `record_emitted` `:224`, `track_delta` `:238`, `commit` `:257`, `rollback` `:262`;
  - the "never a redundant record" rule at `:32-45`;
  - unit tests from `:300` (`try_new` at `:305`).

  host-web builds it at `hosts/host-web/src/lib.rs:6129` from one `[left_mute, right_mute]` per track
  (`prepared_mutes`, `:5880-5898`). host-web also composes the effective mute **inline** at kind 4
  (`lib.rs:4374`: `muted || (any_solo && !solo(track))`); *Address submix strips in browser live
  commands* deletes that copy and calls `effective_mute` (VERIFY-2 M4). This slice makes
  `effective_mute` the strip-aware composition it will call.
- **The randomized differential** (`crates/host-core/tests/randomized.rs`) draws live records over
  `0..handles.track_controls.len()` (`:543`) and pushes them (`:653-655`). Its generator emits 0-2
  submixes.

## Decisions frozen for this slice

- **D1. Control producers for every strip.**
  - When `control_queue_depth` is set, one `TrackControlRequest` is built per **strip** (tracks, then
    submixes).
  - `HostLiveControlHandles.track_controls` becomes `strip_controls: Vec<TrackControlProducer>`,
    parallel to `strips`. Its doc says so.
- **D2. The solo state is per strip (DESIGN P7).**
  - `pub struct StripMuteSeed { pub mutes: [bool; 2], pub solo_safe: bool }`, re-exported from
    `crates/host-core/src/lib.rs`. `LiveControlSoloState::try_new(&[StripMuteSeed])` takes one per
    strip: tracks are not solo-safe, submixes are.
  - `set_solo` on a solo-safe entry returns `false` and changes nothing.
  - `any_solo` counts only soloable entries (a solo-safe entry never has its bit set).
  - `effective_mute(strip, lane) = user_mute || (any_solo && !solo_safe && !soloed)`. It stays the
    single public composition; the module doc and the method doc say it is the one place.
  - The "never a redundant record" rule, the shadow, `commit` and `rollback` are unchanged and cover
    every strip, including solo-safe flags (which never change after construction).
- **D3. host-web: compile fix only.** host-web builds the state with one `StripMuteSeed` per **track**
  (`solo_safe: false`) and stores `strip_controls` in `ReadyOwnership.controls`; nothing addresses an
  index `>= T` until *Address submix strips in browser live commands* (the command guard is still per
  track). host-web's behaviour does not change.
- **D4. #1053 (VERIFY-2 M11).** If #1053 has landed and its capi code reads `track_controls`, rename
  those uses to `strip_controls` (the C ABI's track lanes are `strip_controls[..track_count]`) in this
  slice. If it has not, #1053's spec carries the rename note from *Record the submix, send and VCA
  ruling*.

## Deliverables

- host-core D1 and D2, with unit tests in `crates/host-core/src/solo.rs`.
- The `LiveControlSoloState` doc comment at `crates/host-core/src/solo.rs:86`, which still names
  `HostLiveControlHandles::tracks`: it names `HostLiveControlHandles::strips` (renamed by *List every
  strip in the live-control handles and file bus effects in the browser*) and says strip indices.
- The source-semantics comment *Iterate session strips, not tracks, wherever strip semantics apply*
  (#1198) could not add because `solo.rs` was outside its authorized paths: at the solo-safe guard
  in `crates/host-core/src/solo.rs`, the line `// Source semantics: tracks only.`, then one line
  saying that only tracks are soloable and submix entries are solo-safe.
- Every `track_controls` reader in the Context renamed. `randomized.rs` draws over `strip_controls`
  (tracks and submixes), so its differential now also drives bus builtins.
- host-web D3.
- D4 if #1053 has landed.
- `crates/host-core/tests/MUTATIONS.md` rows for the solo-safe guard and the per-strip requests.

## Authorized paths

- `crates/host-core/src/{prepare.rs,solo.rs,lib.rs}`, `crates/host-core/src/limiter_linked_session.rs`
  (if it reads `track_controls`)
- `crates/host-core/tests/` (the readers above, one new test file, `MUTATIONS.md`)
- `hosts/host-web/src/lib.rs` (the solo-state construction at `:5880-5898` and `:6129`, and the
  `track_controls` uses at `:5794`, `:6117` only), `hosts/host-web/src/tests.rs` (only if a test reads
  the renamed field)
- `crates/capi/src/runtime/**`, **only** for D4 and only if #1053 has landed
- this spec

## Non-goals

- No browser addressing of a bus, no queue band change, no reason vocabulary change (the next two
  slices).
- No submix solo, no implied upstream solo, and no `solo_safe` flag on tracks.
- No change to the redundant-record rule or to the pre-existing redundant `Mute` that host-web stages
  under solo at `lib.rs:4379`.

## Hazards

- **Do not let a bus take a solo-derived mute.** A bus that did would silence the soloed track's own
  bus path. `solo_safe` is the guard; gate 1 pins it.
- **Order.** `strip_controls` must stay parallel to `strips`: canonical tracks, then canonical
  submixes. A bus producer at a track's index edits the wrong strip.
- **Index drift.** For a session without submixes, `strip_controls` equals today's `track_controls`,
  and host-web stages byte-identically.

## Objective gates

1. **Solo-safe state.** host-core unit tests in `crates/host-core/src/solo.rs`:
   - a solo-safe entry's effective mute never changes when any soloable entry's solo engages or
     releases;
   - `set_solo` on a solo-safe entry returns `false`, and `any_solo` stays false;
   - `rollback` restores every strip's user mute and solo, solo-safe entries included, after a refused
     batch;
   - with no solo-safe entry, every existing solo test passes unchanged.

   *Test value: it turns red if `any_solo` counts a solo-safe entry, if a bus can be soloed or
   solo-muted, or if the shadow covers only the track prefix.*
2. **A bus's producers drive the bus.** New host-core test (for example `strip_controls.rs`): a
   3-track, 1-submix session prepared with `prepare_host_runtime_with_live_controls` and a queue depth;
   `strip_controls.len() == 4`, `strip_controls[3].track_id` is the submix; a
   `TrackFaderRecord::FaderDb` of -6 dB with smoothing 0 pushed on `strip_controls[3].fader` renders,
   from the next block, bit-identically to a host prepared from the session with that bus's fader at
   -6 dB and fed the same sources. The bus has no console slots, no stateful inserts and an identity
   input section, so the comparison is exact.

   *Test value: it turns red if a submix gets no control request, if its producer is parallel to the
   wrong strip, or if its fader record lands on a track's lane.*
3. **The randomized differential reaches bus builtins.** `randomized.rs` passes, and its `Reach`
   (or an added counter) records live records pushed to a submix's producer in at least one seed.
   Record the counts in the PR.
   *Test value: a randomized differential is judged by what its generator reaches; the new reach
   counter turns the gate red if the generator stops driving a bus strip's producer, so a bus
   builtins defect could no longer surface in the differential.*
4. **Unchanged where no submix exists.** Every existing host-core and host-web live-control, solo and
   meter test passes with only the rename and the `StripMuteSeed` construction.
5. **Workspace and policy.**
   - The workspace test command (DESIGN.md section 7).
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the K2
     push (`host-core` is in it).
   - If D4 applied: `bash scripts/check-capi-abi.sh`, `bash scripts/check-capi-abi.sh --self-test`,
     and `./target/release/audit capi` after
     `cargo build --locked --release -p audit -p bench -p capi -p session-validator`.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The randomized reach counts.
- Whether #1053 had landed, and so whether D4 applied.

### Attempt 1 record (Terra)

- **D1** (`host-core/src/prepare.rs`): the control requests iterate `live_control_strips` (tracks,
  then submixes); `HostLiveControlHandles.track_controls` -> `strip_controls`, documented parallel
  to `strips` (track lanes `strip_controls[..track_count]`); the `control_queue_depth` doc, the
  handles doc and the request/order comments say "strip". `canonical_index` and the sort were
  already over every strip (#1207), so the order needed no code change.
- **D2** (`host-core/src/solo.rs`, `lib.rs`): `StripMuteSeed { mutes, solo_safe }` re-exported;
  `try_new(&[StripMuteSeed])`; a construction-only `solo_safe` array (no shadow: no transaction
  changes it); `set_solo` returns `false` before taking the shadow on a solo-safe entry (so a
  refused bus solo opens no transaction), under the comment `// Source semantics: tracks only.`
  and the one line the #1198 verdict asked for; `effective_mute = user_mute || (any_solo &&
  !solo_safe && !soloed)`, the module doc and method doc name it the one composition; new
  `solo_safe(strip)` reader; the struct doc names `HostLiveControlHandles::strips` and strip
  indices; parameters renamed `track` -> `strip`. The redundant-record rule, shadow, `commit` and
  `rollback` are unchanged (they copy whole arrays, so they cover every strip).
- **D3** (`host-web/src/lib.rs`): one `StripMuteSeed { solo_safe: false }` per track,
  `handles.strip_controls` into `ReadyOwnership.controls`, the `input_filter_shadows` emptiness
  check renamed. **Deviation (doc only):** the `ReadyOwnership.controls` field doc (outside the
  cited lines) now says per-strip and that commands address only the track prefix, so it does not
  claim per-track after this change. The command guard (`track >= ready.tracks.len()`) is
  untouched, so no index `>= T` reaches `controls` or the solo state.
- **D4**: #1053 has not landed (capi reads no live-control handles); its spec carries the rename
  note (`1053-*.md:165`). D4 did not apply.
- **Readers renamed**: `collapse_arming.rs`, `input_liveness_live_controls.rs` (two),
  `prepare.rs` (four), `randomized.rs`, `spectrum.rs`, `symmetry_witness.rs` (anchors moved by
  K2's earlier slices, found by symbol), plus `strip_handles.rs` (#1207, not in the Context list):
  its "none per bus" assertion was superseded and is rewritten to `controls == strips`.
  `prepare.rs:675`'s `bound.track_controls` is the builtins compiler's field and stays.
  **Not renamed:** `LiveControlSoloState::track_count()` and `track_delta()` keep their names
  (doc now says strips); renaming them touches `host-web/src/tests.rs`, which this slice may edit
  only for the renamed field.
- **Tests and test value** (each mutation applied, run red, reverted; rows 1211-M1..M7 in
  `crates/host-core/tests/MUTATIONS.md`):
  - `solo::tests::a_solo_safe_strip_keeps_its_user_mute_through_every_solo_transition` (gate 1):
    red if a bus can be solo-muted. M2 (drop `!solo_safe`) -> red.
  - `solo::tests::a_solo_safe_strip_cannot_be_soloed` (gate 1): red if a bus can be soloed or
    `any_solo` counts a solo-safe entry. M1 (drop guard) -> red; M4 (guard bumps `solo_count`)
    -> red.
  - `solo::tests::rollback_restores_every_strip_including_the_solo_safe_ones` (gate 1): red if the
    shadow restores only the soloable entries. M3 (rollback skips solo-safe user mutes) -> red;
    M1 -> red.
  - The seven pre-existing solo unit tests run unchanged on all-soloable seeds (helper `state`).
  - `tests/strip_controls.rs::a_bus_fader_record_renders_as_the_session_with_that_fader`
    (gate 2; 3 tracks into `bus`, `bus` to output, queue depth 8; `strip_controls == [t0, t1, t2,
    bus]`; blocks 0-1 differ from the -6 dB twin, blocks 2-5 bit-identical after a `FaderDb`
    -6 dB / smoothing 0 push on `strip_controls[3].fader`): red if a submix gets no control
    request, its producer is not parallel to its strip, or the record lands on a track's lane.
    M5 (tracks-only requests) -> red; M6 (sort by ID) -> red; pushing on `[2]` -> red.
  - `tests/strip_handles.rs::handles_list_tracks_then_submixes_and_file_bus_effects` (rewritten
    assertion): red if the builtin controls stop being parallel to the strips. M5 -> red
    (`left: ["t0", "t1", "t2"]`).
  - `tests/randomized.rs` (gate 3): `live_records` draws over `strip_controls.len()`; new
    `Reach.bus_live_records`, asserted `> 0`. Red if the generator stops driving a bus strip's
    producer. M7 (draw over `track_count`) -> `bus_live_records: 0`, red.
- **Randomized reach** (12 seeds): `consoles: 12, refused: 0, armed_collapse_blocks: 53,
  live_records: 180, bus_console_entries: 11, bus_live_records: 6`; all three arms bit-identical.
- **Gates** (x86_64 AVX2 host, this commit's tree):
  - 1-3: the tests above pass.
  - 4: every existing host-core and host-web live-control, solo and meter test passes with only
    the rename and the `StripMuteSeed` construction (no host-web test edited).
  - 5: DESIGN section 7 workspace test command rc 0 (103 test binaries, 1162 passed, 0 failed);
    `check-/test-host-core-policy.sh`, `check-/test-realtime-policy.sh`,
    `check-/test-workspace-policy.sh` ok; `cargo fmt --all -- --check` ok; workspace clippy
    `--all-targets --all-features -D warnings` clean; `cargo doc` with `-D warnings` clean.
  - `run-aarch64-tests.sh debug`: no arm64 host; at the K2 push (CI `aarch64-debug`).
  - D4 gates: not applicable.
- No digest, oracle or canonical text re-pinned; superseded: the `strip_handles.rs` "none per bus"
  control assertion (rewritten in place).

## Dependencies

- *List every strip in the live-control handles and file bus effects in the browser* (#1207)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- One composition: never write the effective-mute formula a second time.
- Never emit a redundant solo-derived record.
- Render stays allocation-, lock- and syscall-free.
- A test that greps source or prose is refused.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
