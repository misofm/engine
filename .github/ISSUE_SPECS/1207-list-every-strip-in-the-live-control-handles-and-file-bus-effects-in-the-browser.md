# List every strip in the live-control handles and file bus effects in the browser

Slice 10 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

host-core's live-control addressing authority lists every strip, tracks first and then submixes, so
the later K2 slices (meters, the master designation, the browser frame and live commands) have one
enumeration to index into.

The browser files a bus's effects the way it files a track's: every console slot and insert a
submix carries gets its live-control producer and observation handle in host-web's dense effect
tables. That lets the K1 interim rule (DESIGN P16) go: live controls and observation attach to bus
effects again, and a live-controlled browser boot of a session whose bus carries console slots and
inserts keeps working. No command can address a bus yet; *Address submix strips in browser live
commands* (#1213) opens that.

## Context (verified on `fe8ac679`)

After batch K1 (slices 02-08): a submix is a strip with builtins, console entries, inserts, fader
and pan or matrix; the effect compiler prepares its effects (entry key `(strip_id, rack,
effect_id)`); `declared_live_addresses` (`crates/effect-compiler/src/prepare.rs:1650`) walks strips;
and the builtins compiler's `known_tracks` set (`crates/builtins-compiler/src/lib.rs:3327-3332`) is a
strip set, so control and meter requests may name a submix. *Count and cap submix strips in host
preparation and the C ABI* (#1206) added `maximum_submixes`; it moved no anchor below.

- **The K1 interim rule (DESIGN P16).** *Render a submix strip on its summed input* (#1200) made
  `attach_effect_live_controls` (`crates/effect-compiler/src/prepare.rs:1457`) and
  `attach_effect_observation` (`:1577`) skip every prepared entry whose owning strip is not a track,
  because host-web refuses any producer it cannot file. Its gate boots a live-controlled bus session
  in host-web. This slice removes the rule.
- **Handles.** `HostLiveControlHandles` (`crates/host-core/src/prepare.rs:342-366`) has
  `tracks: Vec<Box<str>>`, the canonical normalized track IDs and the addressing authority;
  `track_controls` and `meters` are parallel to it. It is built only at `prepare.rs:1299-1306`, from
  the local `live_control_tracks` (`:913-918`). That local also drives the control requests
  (`:919-928`), the default meters (`:939-942`), the `canonical_index` that orders controls and meters
  (`:1186-1220`), and the master validation (`:1222-1228`).
- **Readers of `handles.tracks`** (exact):
  - `crates/host-core/src/limiter_linked_session.rs:388-389`, `:397-398`, `:418-419`, `:463-465`,
    `:484-486`;
  - `crates/host-core/tests/effect_live_controls.rs:77`, `:179`, `:211`;
  - `crates/host-core/tests/effect_observation.rs:110`, `:180`;
  - `crates/host-core/tests/input_liveness_live_controls.rs:116`;
  - `crates/host-core/tests/live_addressing.rs:167`;
  - `crates/host-core/tests/prepare.rs:705-706`, `:760`;
  - `crates/host-core/tests/symmetry_witness.rs:133`;
  - `hosts/host-web/src/lib.rs:5872` (and the comment at `:5879`), `:6135`.

  capi does not read the handles on `fe8ac679`. #1053 (open) keeps `track_controls` in capi's
  `ProviderEpoch` and may read the handles' ID list.
- **host-web's effect tables** (`hosts/host-web/src/lib.rs`):
  - `rack_effects` and `prepared_mutes` are built per **track** from `model.tracks`
    (`:5872-5898`); `effect_base[t]` is the number of effects of every earlier track
    (`:5899-5913`), and `total_effects` is their sum.
  - Every effect producer is filed by binary search over `handles.tracks`, then
    `dense_effect_slot(effect_base[track], rack_effects[track], address)` (`:5920-5937`); a miss
    refuses boot with `web.live_controls.effects`.
  - Every observation handle is filed the same way (`:5990-6005`), refusing with
    `web.live_controls.observation`. `observation_tracks[slot]` holds the owning track index, and
    `observation_present` is sized `track_count` (`:5986-5989`).
  - `resolve_observation` (`:5072-5082`) binary-searches `ready.tracks` by ID;
    `observation_selection_for_address` (`:5031-5040`) indexes `ready.tracks` by the command's index.
  - `ReadyOwnership.tracks` (`:1426`) is the track list (built from `handles.tracks` at `:6135`);
    `live_control_tracks()` (`:2019-2023`) returns it and feeds the `_track_count` and `_track_id`
    exports, the worklet's track list and `SessionMap.tracks`.
  - `queue_count = 3 * track_count + total_effects` (`:5970-5973`). The three per-track bands, the
    effect band base `3 * tracks` (`queue_available`, `push`, `preflight_effect`, `:1605-1690`) and the
    command guard (`:4323-4326`) are per track.
  - The meter-frame gain-reduction fold (`:3368-3430`) writes `gain_base + observation_tracks[slot]`
    with `gain_base = 2T + 2`; the frame has `3T + 3` words.
- **The binary searches are unsound for strips.** Tracks are sorted, then submixes are sorted; the
  concatenation is not sorted, so a submix ID that sorts before a track ID is never found (VERIFY-2
  M5).

## Decisions frozen for this slice

- **D1. Strips in the handles.**
  - `HostLiveControlHandles.tracks` becomes `strips: Vec<Box<str>>`: every strip, tracks in canonical
    ID order, then submixes in canonical ID order. Add `track_count: usize`; for `i < track_count`,
    `strips[i]` is exactly today's `tracks[i]`.
  - `track_controls` and `meters` stay parallel to `strips[..track_count]`: the control requests and
    the default meters iterate the track prefix only. `canonical_index` covers every strip (it is the
    addressing authority). The master designation is still validated against `track_count`
    (*Meter any boundary of a submix strip and designate a master strip in host-core* (#1208) widens it).
- **D2. A segment-aware strip lookup in host-web.** One function,
  `strip_index(tracks, submixes, id) -> Option<usize>`, binary-searches the track prefix, then the
  submix suffix, and returns `T + j` for submix `j`. It replaces the binary searches at `:5924`,
  `:5993` and in `resolve_observation` (`:5078`). `ReadyOwnership.tracks` keeps the track prefix and
  gains `submixes: Vec<Box<str>>`; `live_control_tracks()` stays the track prefix.
- **D3. Per-strip effect tables in host-web.** `rack_effects` and `effect_base` are built over
  tracks, then submixes (`model.submixes`, whose `console` and `inserts` are the track's shapes since
  K1). Because tracks lead, every track effect keeps its dense index and a session without submixes
  builds byte-identical tables. `observation_tracks[slot]` holds a strip index, and
  `observation_present` is sized `T + S`. `queue_count` grows by the bus effects (`3T +
  total_effects`); the three bands, the effect band base and the command guard stay per track until
  *Address submix strips in browser live commands*.
- **D4. The GR fold skips buses until the frame has room.** Until *Carry submix strips in the
  browser meter frame* (#1209) widens the frame, the gain-reduction fold skips any observed strip index
  `>= T`. (A bus effect cannot be armed before *Address submix strips in browser live commands*
  anyway; the skip keeps a bus index from ever writing the master's or past the frame.)
- **D5. Remove the P16 rule.** `attach_effect_live_controls` and `attach_effect_observation` attach to
  every prepared entry again, as on `fe8ac679`.
- **D6. #1053 (VERIFY-2 M11).** If #1053 has landed and its capi code reads `handles.tracks`, update
  those uses to `strips[..track_count]` in this slice. If it has not landed, #1053's spec carries the
  rename note from *Record the submix, send and VCA ruling*.

## Deliverables

1. host-core D1, with doc comments: `strips` is the addressing authority and tracks lead it.
2. Every `handles.tracks` reader listed in the Context is updated.
3. host-web D2, D3 and D4.
4. effect-compiler D5, with the doc comments that named the interim rule removed. The interim gate
   *Render a submix strip on its summed input* added to `crates/host-core/tests/submix_strip.rs`
   ("no bus effect gets a live channel in K1") is superseded: delete it in this PR (gate 2 below
   asserts the opposite claim). Gate 1 below is also a superset of the host-web K1 boot test
   (*Render a submix strip on its summed input* gate 6, extended by *Carry every console slot on
   every submix strip* (#1202) gate 7) in `hosts/host-web/src/tests.rs`: delete that test, or fold its
   assertions into gate 1's, in this PR.
5. D6 if #1053 has landed.
6. `hosts/host-web/MUTATIONS.md` rows for the segment-aware lookup and the per-strip effect tables.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` (re-exports only, if needed),
  `crates/host-core/src/limiter_linked_session.rs`
- `crates/host-core/tests/`: `effect_live_controls.rs`, `effect_observation.rs`,
  `input_liveness_live_controls.rs`, `live_addressing.rs`, `prepare.rs`, `symmetry_witness.rs`, and
  any other test the rename breaks
- `crates/effect-compiler/src/prepare.rs` (D5 only) and `crates/effect-compiler/tests/` (only a test
  that pinned the interim rule)
- `crates/host-core/tests/submix_strip.rs` (only to delete the superseded K1-interim test)
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs`, `hosts/host-web/MUTATIONS.md`
- `crates/capi/src/runtime/**`, **only** for D6 and only if #1053 has landed
- this spec

## Non-goals

- No command addresses a bus (strip-indexed admission, bands and solo are *Address submix strips in
  browser live commands*).
- No meter, master or frame change (the next three slices).
- No change to `track_controls`, `meters` or the solo state's size.

## Hazards

- **Index drift.** Every per-track table must keep `[0..T)` exactly as today. A session without
  submixes must build byte-identical tables, render byte-identically and refuse identically.
- **The unsorted concatenation.** Any binary search over `tracks ++ submixes` silently misses a bus.
  Gate 1 draws submix IDs that sort before track IDs.
- **An observation lane exists only beside a control lane** (`crates/graph/src/runtime.rs:4051-4053`).
  D5 restores both attach functions together.
- **Strip indices become visible before they are addressable.** The observation map
  (`hosts/host-web/src/lib.rs:2465-2480` reports `observation_tracks[slot]` as a binding's
  `track_index`) and `copy_eq_target_config` (`:2679-2690`, through `effect_slot`) now see bus
  effects at strip index `T + j`. A command at such an index is still refused by the per-track guard
  until *Address submix strips in browser live commands*; this holds inside batch K2 only, which is
  pushed once after that slice. Do not widen the guard here.

## Objective gates

1. **A bus session boots live-controlled and files every bus effect.** New native test in
   `hosts/host-web/src/tests.rs`:
   - the session declares console slots `pre_insert: [eq]`, `post_insert: [limiter]`; two tracks;
     submixes `aaa-bus` and `zzz-bus` (one sorts before every track ID, one after), each carrying
     every console slot and a `miso.compressor` insert; tracks route into the buses, and the buses to
     the output;
   - it boots with `live_control_command_queue_records = 64` and `live_control_observation_taps = 1`
     and renders 8 blocks with `RESULT_OK`;
   - every prepared effect, bus effects included, has a filed producer
     (`effect_controls[slot].is_some()`) and every effect that declares a tap has a filed
     observation handle, at the dense slot `dense_effect_slot(effect_base[strip], rack_effects[strip],
     address)` with `strip = T + j` for a bus.

   *Test value: it turns red if a bus producer or observation handle is refused or misfiled, if a
   lookup binary-searches the unsorted strip list (missing `aaa-bus`), or if the P16 rule is left in
   place (bus slots stay `None`).*
2. **Handles list strips.** New host-core test (in `crates/host-core/tests/`, for example
   `strip_handles.rs`): for a 3-track, 2-submix session prepared with a queue depth, `strips` is the
   tracks then the submixes, each in canonical ID order; `strips[..track_count]` equals the tracks;
   `track_controls.len() == track_count`; and the bus effects' producers in `effect_controls` carry
   the submix IDs.

   *Test value: it turns red if a submix lands inside the track prefix, if the control requests
   start to cover buses before the solo and band work exists, or if bus effects stay unattached.*
3. **Track tables are unchanged.** For the existing host-web live-control and observation tests,
   only the field rename changes; for a session without submixes, `effect_base`, `rack_effects` and
   `queue_count` equal the base commit's (recorded as PR evidence).
4. **Render allocates nothing** with bus effects filed. With gate 1's host, a live-control submission
   to a track effect plus `render_next` measured with `crate::ffi::live_response_ffi_tests::measured`
   (the pattern at `hosts/host-web/src/tests.rs:3638-3647`) reports `allocations == 0` and
   `deallocations == 0`.

   *Test value: it turns red if filing bus effects adds a render-time allocation, for example a
   per-block lookup that builds a table.*
5. **Browser artifact.** host-web has no arm64 leg, so its 4-lane coverage is the shipped module:
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/check-sdk-headless.sh <A>`
6. **Workspace and policy.**
   - The workspace test command (DESIGN.md section 7).
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `bash scripts/check-effect-runtime-policy.sh` and `bash scripts/test-effect-runtime-policy.sh`
     (it polices effect-compiler)
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the K2
     push (host-core and effect-compiler; host-web is not in it).

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The list of updated `handles.tracks` readers.
- The gate-3 table comparison against the base commit.
- Whether #1053 had landed, and so whether D6 applied.

### Attempt 1 record (Terra)

- **D1** (`host-core/src/prepare.rs`): `HostLiveControlHandles.tracks` -> `strips` (tracks, then
  submixes, each in canonical ID order) plus `track_count`; the control requests and default
  meters iterate `strips[..track_count]`; `canonical_index` covers every strip; the master is
  still validated against the track count. Doc comments name `strips` the addressing authority.
- **Readers updated** (anchors as cited unless noted): `limiter_linked_session.rs` (the
  `prepare`/`controls`/`observers` walks and both `placement` walks, now
  `strips[..track_count]`); tests `effect_live_controls.rs` (`:78`, `:180`, `:212`),
  `effect_observation.rs` (`:111`, `:181`), `input_liveness_live_controls.rs:117`,
  `live_addressing.rs:168`, `prepare.rs` (`:701-707` to `strips[..track_count]`, `:761`),
  `symmetry_witness.rs:134` (each was one line below the cited anchor); host-web build and
  `ReadyOwnership.tracks` (now `drain`ed from `strips`, the tail becoming `submixes`).
- **D2/D3/D4** (`host-web/src/lib.rs`): `strip_index(tracks, submixes, id)` searches the two
  segments apart and replaces the producer, observation and `resolve_observation` searches;
  `rack_effects`/`effect_base` cover tracks then `model.submixes`; `observation_present` is
  `T + S`; `queue_count = 3T + total_effects`; bands, effect-band base and command guard
  unchanged; the GR fold skips a strip index `>= T`. Boot refuses
  (`web.live_controls.effects`) if `track_count > strips.len()` or the model's track/submix
  counts disagree with the handles.
- **D5** (`effect-compiler/src/prepare.rs`): both attach functions attach to every entry again;
  `track_owned` and the interim doc paragraphs deleted; producer/observer `track_id` docs now say
  "strip". Superseded tests deleted: `host-core/tests/submix_strip.rs::no_bus_effect_gets_a_live_channel_or_an_observation_handle`
  (and its module-doc lines) and `host-web/src/tests.rs::live_controlled_boot_of_a_bus_with_an_effect_renders`
  (its live-controlled bus boot with meters on is kept inside gate 1's host).
- **D6**: #1053 has not landed (its spec is still open; capi reads no handles), so D6 did not
  apply.
- **Tests and test value** (mutation each, reverted after; rows in `hosts/host-web/MUTATIONS.md`):
  - `host-web tests::a_bus_session_boots_live_controlled_and_files_every_bus_effect` (gate 1;
    `aaa-bus`/`zzz-bus` around `t0`/`t1`, console `[eq]`/`[limiter]`, compressor insert per bus,
    queue 64, taps 1, meters 2 blocks, 8 blocks `RESULT_OK`; all 12 effects checked at
    `dense_effect_slot(effect_base[s], rack_effects[s], ..)` with `s` computed by the test): red
    if a bus producer or observer is refused or misfiled, if a lookup binary-searches the unsorted
    strip list, or if P16 is left in place. Mutations: one search over `tracks ++ submixes` ->
    boot refuses; P16 restored -> `aaa-bus` console slot 0 has no producer.
  - `host-web tests::a_bus_session_admits_and_renders_without_allocating` (gate 4; bypass of `t0`'s
    compressor plus `render_next` under `ffi::live_response_ffi_tests::measured`, 0/0): red if
    filing bus effects adds an admission- or render-time allocation. Mutation: allocate in
    `ReadyOwnership::effect_slot` -> `admission/render allocated`.
  - `host-core tests/strip_handles.rs::handles_list_tracks_then_submixes_and_file_bus_effects`
    (gate 2; 3 tracks, buses declared `zzz-bus` then `aaa-bus`): red if a submix enters the track
    prefix, if control requests or default meters cover buses, or if a bus effect lacks a channel
    or observer. Mutations: controls over all strips -> red; strips sorted as one list -> red;
    P16 restored -> red.
- **Gate 3** (PR evidence, not committed): a temporary probe printed `effect_base`,
  `rack_effects`, `queue_count` (`in_flight.len()`), `command_wanted.len()`,
  `effect_controls.len()`, `observation_tracks`, `observation_present.len()` and `tracks` for 12
  submix-free hosts (compressor, multiband, gate, input-filter, effect+input-filter,
  live-control, paired nine-track, effect, eight-track bank, observation, same-track
  observation, five-track effect-solo); the output at `8f8c013e6` and at this commit is
  byte-identical (`diff` empty). No existing host-web live-control/observation test changed.
- **Gates** (x86_64, this commit's tree): `cargo fmt --all -- --check` ok; clippy
  `--workspace --all-targets --all-features -D warnings` ok; `cargo doc` with `-D warnings` ok;
  the DESIGN section 7 workspace test command exit 0 (101 test binaries, 1151 passed, 0 failed);
  `check-/test-` pairs for host-core, realtime, effect-runtime and workspace policy all exit 0;
  browser: `build-web-audioworklet.sh --named-twin` ok (shipped module `0a6e44c9...`),
  `check-web-audioworklet.sh` ok, `check-browser-expected-resources.py --artifacts` ok,
  `check-sdk-headless.sh` ok. `run-aarch64-tests.sh debug`: not an arm64 host, so at batch push
  (CI `aarch64-debug`).
- **Open (hazard, as the spec says):** `observation_binding` and `copy_eq_target_config` now see
  bus effects at strip index `T + j`; commands at such an index are still refused by the
  per-track guard until #1213.

## Decision record (K2 follow-ups)

- **Gate 1 sees the submix order** (verdict MINOR-1). `bus_effect_host`'s buses now differ in
  shape: `aaa-bus` carries the compressor then `t1`'s EQ, `zzz-bus` the compressor alone, and
  gate 1 checks every insert of every strip (13 dense slots). Test value: red if host-web fills
  its per-strip tables (`rack_effects`, `effect_base`) in another submix order than
  `handles.strips`. Mutation M9 (`for submix in model.submixes.iter().rev()` in `compile_ready`)
  turns it red: the misfiled producers refuse the boot. The boot consistency check still compares
  counts only; part 1 of the fix is enough.
- **Gate 4 omits `poll_meters`** (verdict MINOR-2): folded into #1209 gate 5,
  `bus_meters_render_and_poll_without_allocating`.
- **INFO-4:** K2 is pushed whole, never split between #1207 and #1210 (root).
- **NITs:** `effect_controls`' doc says "strip effect instance"; the effect-compiler `address`
  docs say "within its strip (a track or a submix)"; host-web's `observation_track_index` export
  doc says strip index. `prepare.rs`'s "cannot disagree" comment and `solo.rs` were corrected by
  #1211.

## Verdict

- **Attempt 1** (`7bdb45187`): Sol PASS, no BLOCKER or MAJOR; two MINOR, three INFO and NITs,
  applied in the K2 follow-up commit as above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1207-attempt1.md`; the verifier's probe is
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1207-attempt1-verifier-scratch.rs`.

## Dependencies

- *Count and cap submix strips in host preparation and the C ABI* (#1206)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded to one value (decision 10).
- Render stays allocation-, lock- and syscall-free.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
