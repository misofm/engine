# Address submix strips in browser live commands

Slice 16 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A producer in the browser can ride a drum-bus fader, mute a return, pan a bus, trim a bus input,
sweep its filters, and move or observe a bus console slot's or insert's parameters. All of it is
live, with the existing declicked ramps, and behaves exactly as it does on a track. Soloing a track
keeps its buses and returns audible, and a muted bus can be unmuted while a track is soloed.

A mix bus with a limiter can be the designated master: the frame's master gain-reduction word then
reports that bus limiter's reading. The command record's index word is a **strip index**: tracks
first, then submixes in canonical order (DESIGN P9), so every existing track index is unchanged.

## Context (verified on `fe8ac679`)

Earlier K2 slices moved these anchors: *List every strip in the live-control handles and file bus
effects in the browser* (#1207, per-strip `effect_base`, `rack_effects`, `observation_tracks`,
`observation_present`, the segment-aware `strip_index`, `ReadyOwnership.submixes`); *Carry submix
strips in the browser meter frame* (#1209, frame `3(T + S) + 3`, strip-indexed gain-reduction fold); *Name
submix strips in the browser session map and the SDK measurement* (#1210, `SessionMap.submixes`); *Give
every strip one mute owner and live-control producers in host-core* (#1211, `strip_controls` for every
strip, `LiveControlSoloState::try_new(&[StripMuteSeed])` with `solo_safe`, the public
`effective_mute(strip, lane)`); *Add the notSoloable command reason to every vocabulary spelling*
(#1212, reason 12). The line numbers below are `fe8ac679`'s; re-read each site.

- **The record** is the frozen 48-byte `miso.command.v1` record; its index word `track_index` (`u32`)
  is at offset 4 (field `:659`, `:692`, `:716`). Kinds are 1-12 (`hosts/host-web/src/lib.rs:797-862`);
  9 is solo; 7 and 8 are observation subscribe and unsubscribe (`:821`, `:823`).
- **Admission** is `admit_commands_staged` (`:4276`). Its generic guard (`:4323-4326`) refuses
  `track_index >= track_count` with `COMMAND_REASON_UNKNOWN_TRACK` before kind dispatch, with
  `track_count = ready.tracks.len()` (`:4286`).
- **Kinds 1-3 and 10-12** lower directly into the strip's queues (`:4330-4348`).
- **Kind 4 (mute)** (`:4349-4380`): `set_user_mute`, then the effective mute composed **inline** at
  `:4374` as `muted || (any_solo && !solo(track))`, then `record_emitted`, then one `Mute` record
  carrying the effective value. The inline copy ignores `solo_safe`: with a submix entry it would
  stage `Mute { muted: true }` for a bus **unmute** while any track is soloed (VERIFY-2 M4).
  Pre-existing and out of scope: under solo, an unmute of a solo-muted track stages a redundant
  `Mute { muted: true }` (`:4379`).
- **Kind 9 (solo)** (`:4381-4399`) stages nothing; the coalescing pass (`:4594-4634`) emits each
  changed strip's net mute records from `track_delta`, on slot `track_count + track`.
- **Kinds 5-8 (effects and observation)** resolve `effect_slot(track, address)` (`:1596`) and stage on
  `3 * track_count + effect` (`:4447-4570`).
- **Queue bands** in `ReadyOwnership` (`:1391-1470`): `0..T` matrix/pan, `T..2T` fader/mute, `2T..3T`
  input section, `3T..` effects. The `tracks` base is spelled at: `queue_available` and `push`
  (`:1605-1650`); `preflight_effect` (`:1686-1690`); the prepared-owner slot (`:3592-3610`, with
  `builtin_input_slot` `:3580`); the admission slots (`:4341-4342`, `:4379`, `:4399`, `:4414`,
  `:4506`, `:4527`, `:4565`, `:4607-4608`, `:4657`, `:4711`, `:4739`, `:4869`, `:4987`, `:5005`);
  `queue_count` (`:5970-5973`); and `command_staging_count` (`:6253-6257`,
  `2 * MAXIMUM_COMMAND_RECORDS + 2 * track_count`).
- **Per-track tables still built from tracks:** `controls` is now `strip_controls` (all strips);
  `prepared_mutes` (`:5880-5898`) and the solo state (`:6129`, built per track with
  `solo_safe: false` by slice 14); `input_filter_shadows` (`:5793-5810`, from `model.tracks`);
  `observation_selection_for_address` (`:5031-5040`, indexes `ready.tracks`).
- **The master designation in the browser.** The boot word `live_control_master_track_plus_one`
  (`WebBootOptions`, doc `:1116-1121`) is translated in `live_control_request` (`:6172-6200`) into
  `HostLiveControlRequest.master_track`, which host-core validates against the strip list since
  *Meter any boundary of a submix strip and designate a master strip in host-core* (#1208). The meter header
  echoes it as `master_track_plus_one` (doc `:1033`), and `ReadyOwnership.master_track` (doc `:1462`)
  selects the gain-reduction word the frame reports as the master's (`:3431-3446`). A bus effect can be
  observed only once kind 7 addresses a bus.
- **Docs that still say "track" or "submixes carry no effect racks"** (spellings stay, DESIGN P17;
  VERIFY-2 M14 and MINOR 12): `hosts/host-web/src/lib.rs:1033`, `:1118-1121`, `:1462`; the host
  `.d.ts` `liveControlMasterTrackPlusOne` doc (`miso-engine-v1-audio-worklet-host.d.ts:729-733`, with
  the stale "submixes and outputs carry no effect racks" at `:731`) and its byte-identical SDK mirror;
  `sdk/src/core/abi.ts:122`; `sdk/src/browser/host-mirror.ts:62-68`;
  `docs/EFFECT_OBSERVATION_V1.md:128-133`; and the solo formula in
  `docs/BUILTINS_AND_METERING_V1.md:115-125`.
- **Test helpers** (`hosts/host-web/src/tests.rs`): `live_control_host` (`:2576`), `stage_command`
  (`:2288`), `observe` (`:5731`), `solo_host` (`:6624`), `stage_solo` (`:6635`),
  `render_pair_and_compare` (`:6681`) and `solo_is_bit_identically_mute_on_the_complement` (`:6715`).
  `feed_and_render` (`:2667-2684`) feeds one constant to identical planes of one shared source, which
  hides lane and index errors (VERIFY-2 M13); the gates below feed distinct signals. Allocation
  counts use `crate::ffi::live_response_ffi_tests::measured` (`:3638-3647`).

## Decisions frozen for this slice

- **D1. The index word is a strip index.** `i < T` is track `i`; `T <= i < T + S` is submix strip
  `i - T`. The generic bound moves to the strip count with the same reason (`UNKNOWN_TRACK`, P17).
  Every band, every per-track table and every slot spelling in the Context uses the strip count
  `N = T + S` in place of the track count: the bands are `0..N`, `N..2N`, `2N..3N` and `3N..`;
  `queue_count = 3N + total_effects`; `command_staging_count` adds `2 * N`; `input_filter_shadows`,
  the prepared-owner slots and `observation_selection_for_address` are per strip.
- **D2. Kinds 1-8 and 10-12 apply to any strip**, with the same lowering as a track.
- **D3. One mute composition (DESIGN P7, VERIFY-2 M4).** host-web builds the solo state per strip
  (tracks with `solo_safe: false`, then submixes with `solo_safe: true`, seeded from each strip's
  session fader mutes). Kind 4 calls `set_user_mute`, then takes the effective value from
  `ready.solo.effective_mute(strip, lane)` for the record's lanes; the inline formula at `:4374` is
  deleted. The coalescing pass iterates strips. The "never a redundant record" rule is unchanged.
- **D4. Solo addresses tracks only.** Kind 9 at a submix index refuses with reason 12
  (`notSoloable`), at that record's wire index, and stages nothing. Submix strips are never
  solo-muted.
- **D5. Observation and the master on a bus.** Kinds 7 and 8 at a submix index arm and disarm a bus
  effect's tap; its gain reduction folds into the frame's word for that strip. A boot word naming a
  submix (`T + j + 1`) designates that bus as the master.
- **D5a. Refusal reasons at a bus index.** The eq-config refusal classifier compares
  `trackIndex >= trackCount` (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1526-1532`,
  `sdk/src/core/boundary.ts:1205-1216`), so a bad rack or effect at a bus index would report
  `unknownTrack`. Both compare against the strip count `T + S` instead (the worklet's track and
  submix counts; in the SDK, the `live_control_track_count` and `live_control_submix_count`
  exports).
- **D6. Docs** (P17): every site in the Context says "strip index (tracks first, then submixes) plus
  one" for the boot word and the header field, and the stale "submixes carry no effect racks" text
  goes; `docs/BUILTINS_AND_METERING_V1.md`'s solo formula becomes
  `effective_mute(strip, lane) = user_mute || (any_solo && !solo_safe(strip) && !soloed(strip))`, with
  one sentence that submixes are solo-safe.

## Deliverables

- host-web D1-D5 and D5a: bands, admission, staging count, every per-track table, and the solo state's
  construction from strips.
- D6 across the listed docs, the `.d.ts`, its SDK mirror and the SDK sources (doc comments only).
- `hosts/host-web/MUTATIONS.md` rows for the moved bound, the per-strip solo state, the deleted inline
  composition and the new reason's use.

## Authorized paths

- `hosts/host-web/src/{lib.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `sdk/src/core/boundary.ts` (the D5a refusal classifier only)
- `sdk/src/browser/shipped-host.d.ts`, `sdk/src/core/abi.ts` (doc comment only),
  `sdk/src/browser/host-mirror.ts` (doc comment and message text only)
- `docs/EFFECT_OBSERVATION_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`
- `crates/host-core/src/solo.rs` (doc comments only, if the module doc still says "track")
- this spec

## Non-goals

- No route (send) live controls (*Admit live send commands in the browser*, #1222).
- No C ABI live path (#1053).
- No submix solo, no implied upstream solo, no `solo_safe` flag on tracks.
- No SDK `submix(id)` edit surface (*Drive submix strips from the SDK live controls*, #1214).
- No fix of the pre-existing redundant `Mute` under solo noted in the Context.
- No rename of the boot word, the header field, the SDK option or `track_index` (P17).

## Hazards

- **The acked-batch question.** A batch mixing track and submix records checks room in every
  destination queue before any push, and stages nothing when any record or queue fails.
- **Do not let a bus take a solo-derived mute.** It would silence the soloed track's own bus path.
  `solo_safe` is the guard, and gate 2 pins it.
- **Index drift.** Every per-track table must keep `[0..T)` exactly as today. A session without
  submixes must render, stage and refuse byte-identically.
- **A band spelling missed** puts a bus's fader record in an input queue or an effect slot. Gate 1
  sends every kind to a bus.

## Objective gates

All gates are native tests in `hosts/host-web/src/tests.rs` unless stated, at 48 kHz, quantum 128.

1. **Live strip edits on a bus equal the same edits on a track.** Paired hosts
   (`render_pair_and_compare` pattern):
   - host A: three tracks, each reading its own source channel with a distinct, non-constant
     per-sample signal per lane, routed with asymmetric matrices into submix `bus`, whose strip S has
     trim, HPF/LPF, a compressor insert, a fader and a pan, and `bus` routes to the output; no console
     slots;
   - host B: one track `ref` with strip S, fed the bus's `f32` sum as its source, computed per block
     in the test by the D3 expression (two roundings) summed in route-ID order;
   - for each kind 1-8 and 10-12 (insert parameter, bypass and observation included), the same
     command is sent to `bus` in A (index `T + 0`) and to `ref` in B, with smoothing 0 and with
     smoothing 480. Both render the same bits.

   *Test value: it turns red if an index at or past `T` is refused, lands in a track's band, or
   reaches another strip's queue, or if any band spelling still uses the track count. No test has
   ever addressed a bus.*
2. **Mute and solo on strips.**
   - Kind 4 at a bus index mutes the bus, bit-identically to a host booted with that bus's fader
     muted, once the ramp has settled.
   - Soloing track `a` (which feeds bus `drums` and return `verb`) renders bit-identically to
     explicit mutes on every other track, paired as
     `solo_is_bit_identically_mute_on_the_complement` does; `drums` and `verb` stay audible.
   - Kind 9 at a bus index refuses with reason 12 and result `RESULT_INVALID_ARGUMENT` at its wire
     index, and nothing is staged.

   *Test value: it turns red if solo composition reaches strips past `T`, if a bus can be soloed, or
   if a bus mute has no owner and is refused or dropped.*
3. **A bus can be unmuted under solo** (VERIFY-2 M4). With bus `drums` muted at boot and track `a`
   soloed, kind 4 unmuting `drums` makes it audible: after the ramp, the output is bit-identical to a
   host booted with `drums` unmuted and the same solo.

   *Test value: it turns red if kind 4 composes the effective mute inline without `solo_safe` (it
   then stages `Mute { muted: true }` for the unmute).*
4. **All or nothing.**
   - A batch of a valid bus fader record and an out-of-domain track record stages nothing and returns
     one refusal at the bad record's index.
   - A batch that overfills one bus queue is typed backpressure, with no push.

   *Test value: it turns red if admission pushes to a bus queue before checking every destination.*
5. **A bus gain-reduction word** (moved here by VERIFY-2 M6). With meters and observation taps on,
   kind 7 at the bus's index arms its compressor insert's gain-reduction tap; driven over its
   threshold, the frame's word for that strip (`gain_base + T + j`; the native test reads this frame word,
   which the worklet copies to the message's `submixGrDb[j]`) equals, bit for bit, the word a paired host reports for `ref` with the same strip fed the
   bus's sum. Kind 8 disarms it, and the word returns to `0`.

   *Test value: it turns red if a bus effect cannot be armed, folds into a track's slot, or is read
   at the wrong frame offset.*
6. **A bus master** (moved here by VERIFY-2 M6). Booting with `live_control_master_track_plus_one =
   T + j + 1` (bus `j`, whose strip carries a true-peak limiter insert driven 6 dB over its ceiling)
   and the limiter's tap armed makes the header's `master_gr_present` 1 and the frame's master word
   equal to that bus's gain-reduction word, nonzero. A value of `T + S + 1` refuses at boot with the
   existing result and diagnostic `host.observation.master_track`.

   *Test value: it turns red if the boot word still indexes tracks only, or the master reading is
   taken from the wrong strip's slot.*
7. **Render allocates nothing** while bus edits and a bus observation are submitted:
   `crate::ffi::live_response_ffi_tests::measured` around admission and `render_next` reports
   `allocations == 0` and `deallocations == 0`.

   *Test value: it turns red if strip-sized admission or the bus fold allocates on the render path.*
8. **`S = 0` is unchanged.** Every existing host-web live-control, solo, observation and meter test
   passes with no edit beyond helper signatures.
9. **Browser artifact and SDK.**
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh <A>` and `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-sdk-headless.sh <A>`
10. **Workspace and policy.**
    - The workspace test command (DESIGN.md section 7).
    - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh` (the solo
      doc touch)
    - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
    - `cargo fmt --all -- --check`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
    - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`

## Amendment A1 (earlier K2 verdicts, recorded at attempt 1)

- **A1a (#1210 open item, K2 verdict).** host-web `observation_selection_for_address` resolves its
  index per strip (tracks, then submixes, as `strip_index`), so a selected read of a bus tap at
  `T + j` succeeds. Gate: a headless `readObservations` on a bus tap returns a row with
  `trackId: "bus"`; the mutation "tracks-only lookup" must turn it red.
- **A1b (#1208 condition).** A browser boot naming a bus as master reports `masterGrDb`; gate 6
  covers it.
- **A1c.** Gate 5 must turn red under "restore #1207's GR skip of strip indices `>= T`" (run and
  recorded).
- **A1d (#1211 PASS verdict MINOR-1).** `LiveControlSoloState::track_count()` becomes
  `strip_count()` and `track_delta()` becomes `strip_delta()` (it counts strips while
  `HostLiveControlHandles::track_count` counts tracks). host-web's own copy of the effective-mute
  formula is deleted in favour of host-core's `effective_mute` (D3).
- **A1e (#1212 PASS verdict NIT-2).** A gate asserts reason 12 with result 1 together on the real
  shipped path.
- **Authorized paths added:** `crates/host-core/src/solo.rs` (the A1d rename, its callers and
  docs; widens "doc comments only"), `sdk/test/capability-evals.mjs` (the A1a, A1e and D5a headless
  gates).

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The list of per-track tables and band spellings that became per-strip.
- The ARTIFACT CHANGED report.

### Attempt 1 record (Terra)

- **D1 (per strip, `N = T + S`, tracks first).** `ReadyOwnership::strip_count()`; bands in
  `queue_available`, `push`, `preflight_effect`; `builtin_input_slot` and
  `prepared_queue_address`; every admission slot (generic guard, the three per-strip bands, the
  prepared-owner markers, coalescing, companion validation, push and commit passes);
  `queue_count = 3N + effects`; `command_staging_count(N)` (`2N` solo term);
  `input_filter_shadows` from track then submix builtins; the solo seeds;
  `observation_selection_for_address` (A1a). The meter frame and GR fold were already per strip
  (#1209).
- **D2.** No kind-specific code: kinds 1-8, 10-12 reach a bus through the moved bound and bands.
- **D3.** Seeds per strip (tracks `solo_safe: false`, submixes `true`, from each fader's mutes);
  kind 4 takes `ready.solo.effective_mute(strip, first covered lane)`; the inline formula is gone
  (no copy of it left in host-web); coalescing walks strips.
- **D4.** Kind 9 checks `ready.solo.solo_safe(strip)` after decoding and refuses
  `COMMAND_REASON_NOT_SOLOABLE` (result `RESULT_INVALID_ARGUMENT`) at the record's index.
- **D5.** Needed no code beyond D1: the fold and master read were per strip since #1209/#1208.
- **D5a.** Worklet compares against `trackCount + submixIds.length`; SDK against
  `live_control_track_count + live_control_submix_count`.
- **D6.** Docs at the cited sites (`WebMeterHeader.master_track_plus_one`,
  `WebBootOptions.live_control_master_track_plus_one`, `ReadyOwnership.master_track`, the `.d.ts`
  and its byte mirror, `abi.ts`, the `host-mirror.ts` message, both docs). `COMMAND_SOLO` doc
  names `notSoloable`. `solo.rs` module doc already said "strip"; only A1d touched it.
- **A1d.** Renamed; callers `host-web/src/lib.rs` (coalescing) and `tests.rs` (two
  `strip_count()`); two existing `MUTATIONS.md` rows name `strip_delta`.
- **Tests and test value** (each mutation applied, run red, reverted; rows in
  `hosts/host-web/MUTATIONS.md`). Sessions feed distinct splitmix noise per lane per source.
  - `live_strip_edits_on_a_bus_equal_the_same_edits_on_a_track` (gate 1; 13 single-kind commands
    incl. per-lane fader/mute/trim/polarity, insert parameter, bypass on/off, subscribe and
    unsubscribe, then kind 12 through the prepared companion and one 4-kind batch; smoothing 0 and
    480; every output sample bit-compared): red if a strip index is refused, misbanded or reaches
    another strip. Mutations: guard on `tracks.len()` -> red (pan refused); `push` band on
    `tracks.len()` -> red (255); `queue_count` per track -> red (7); shadows tracks-only -> red.
  - `a_bus_mute_command_equals_the_bus_booted_muted` (gate 2a): red if a bus mute has no owner.
    Mutation: seeds per track only -> red (`unknownTrack`).
  - `soloing_a_track_keeps_its_bus_and_return_audible` (gate 2b; also proves `a` reaches the
    output only via the submixes): red if solo composition mutes a strip past `T`. Mutation:
    submixes seeded `solo_safe: false` -> red.
  - `a_solo_at_a_bus_index_refuses_not_soloable_and_stages_nothing` (gate 2c; both buses, behind a
    valid record): red if a bus can be soloed or the refusal's reason/index moves. Mutations: drop
    the D4 check -> red (reason 2); `solo_safe: false` -> red (admitted).
  - `a_bus_can_be_unmuted_while_a_track_is_soloed` (gate 3): red if kind 4 composes inline
    without `solo_safe`. Mutation: the old inline formula -> red (block 4 differs).
  - `a_bus_record_behind_a_bad_track_record_is_never_pushed` (gate 4a): red if a bus record is
    pushed before validation ends. Mutation: push bus faders in pass one -> red.
  - `overfilling_a_bus_queue_is_typed_backpressure_with_no_push` (gate 4b): red if the room check
    skips a bus queue. Mutation: skip bus fader slots in the room pass -> red (255).
  - `a_bus_compressor_reports_its_gain_reduction_in_the_bus_word` (gate 5 + A1a native): bus word
    bit-equal to the track twin's and > 0, track words 0, selected read at `T` names `bus`/`comp`
    with the twin's values, `T + 1` invalid, disarm returns the word to 0. Red if a bus effect
    cannot be armed, folds elsewhere or is skipped, or the read is tracks-only. Mutations (A1c):
    #1207's `>= T` GR skip restored -> red (word 0); tracks-only lookup -> red.
  - `a_bus_limiter_can_be_the_designated_master` (gate 6, A1b): limiter ceiling -6 dB on the bus,
    master word `T + 1`; `master_gr_present == 1`, master word == bus word > 0; `T + S + 1`
    refuses `RESULT_REFUSED_DOCUMENT` with `host.observation.master_track`. Red if the master
    reading is not the bus's. Mutation: `>= T` GR skip -> red (`master_gr_present` 0).
  - `bus_edits_and_a_bus_observation_admit_and_render_without_allocating` (gate 7; 7-record batch:
    bus fader, pan, trim, insert parameter, subscribe, mute, plus a track solo): 0/0. Mutation: a
    `submixes.len()` `Vec` in admission -> red (1 allocation).
  - `capability-evals.mjs` "a selected read of a bus tap names the bus, unarmed and then armed at
    its strip index" (A1a headless): red if host-web resolves the read against tracks only.
    Mutation: tracks-only lookup, artifact rebuilt into a scratch dir -> red (`invalidArgument`).
  - `capability-evals.mjs` "a bad effect at a bus index is an unknown effect, and only past every
    strip an unknown track" (D5a): red if the SDK classifier bounds by the track count. Mutation:
    track count only -> red. The worklet's twin classifier has no harness path (no existing test
    drives `receiveEqTargetConfig`), so its one-line change is untested beyond the build gates.
  - `capability-evals.mjs` "a solo at a bus index is refused notSoloable with invalidArgument
    through the shipped module" (D4, #1212 PASS NIT-2): `reason === 12` and `result === 1`
    together from the real admission. Red if a bus can be soloed, or reason 12 reports another
    result. Mutation: drop the D4 check, artifact rebuilt into a scratch dir -> red (reason 2).
- **Gate 8:** no existing host-web test edited except the two A1d renames; all pass.
- **Gates** (x86_64 AVX2; A = `target/ci/k2-1213-artifacts`, B = `target/ci/k2-1213-named`), all
  rc 0: `build-web-audioworklet.sh --named-twin`; `check-web-audioworklet.sh`;
  `check-browser-expected-resources.py --artifacts` (32 red mutations); `test-web-audioworklet.sh`;
  `check-sdk-generated.sh`; `check-sdk-types.sh`; `check-sdk-headless.sh` (342 pass, 0 fail); DESIGN
  section 7 workspace test command (103 binaries, 1172 passed, 0 failed); `check-/test-` host-core,
  realtime and workspace policy; `cargo fmt --check`; workspace clippy `-D warnings`; `cargo doc`
  `-D warnings`. `run-aarch64-tests.sh debug`: no arm64 host; at the K2 push.
- **ARTIFACT CHANGED:** shipped module `7d6c0a8b...0f90b` (2 695 834 B) against #1212's
  `19d19812...`; the bytes move because admission, the build tables and the worklet classifier
  changed. No re-pin; CI's `artifact-identity` line is the authority.
- No digest, oracle or canonical text re-pinned; no test superseded.

### Attempt 2 record (Sol, after the attempt 1 FAIL verdict)

Docs and tests only; no engine logic changed. **Authorized path added (A2):**
`scripts/test-web-audioworklet.mjs` (the verdict's two harness assertions).

- **MAJOR-1 (`frameSlot`).** `MisoObservationBinding.frameSlot` is defined as the **strip index**
  (equal to `trackIndex`): below `MisoMeterFrame.trackCount` it indexes `trackGrDb`, from there
  `submixGrDb[frameSlot - trackCount]`. The shipped host already reports `frameSlot: trackIndex`;
  only its comment changed. Same pass, `.d.ts` and its byte mirror `sdk/src/browser/shipped-host.d.ts`:
  the addressing header, `MisoCommandReason.UnknownTrack`, `MisoCommand.trackIndex`,
  `MisoSessionMap.tracks`, `MisoObservationSubscription.trackIndex` and `sessionMap()` now say
  strip index (tracks, then submixes); `docs/EFFECT_OBSERVATION_V1.md` states the `frameSlot`
  mapping. No SDK source reads `frameSlot`. No wire change.
- **MINOR-1.** `a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track` (gate 1's prepared-EQ
  arm; the verifier's probe).
- **MINOR-2.** `a_single_lane_bus_mute_equals_the_bus_booted_with_that_lane_muted` (the verifier's
  right-lane probe, both lanes); `solo_bus_host` now delegates to `solo_bus_host_lanes`. The
  verifier's track-under-solo probe is not committed: it does not discriminate the lane mutation.
- **MINOR-3.** The worklet's D5a classifier is driven through `makeProcessor()` and
  `receiveEqTargetConfig` in `testProcessor()`. This corrects attempt 1's "no harness path".
- **MINOR-4.** `WebObservationSelection.track_index` and `WebObservationResult.track_index` say
  strip index; `admit_commands`' doc says two more per strip and `2 * strip_count`.
- **NITs.** 1: gate 7's doc drops the bus-fold clause (the fold runs in `poll_meters`, outside
  `measured`; #1209's test covers it). 2: gate 4a renamed
  `a_bus_record_ahead_of_a_bad_track_record_is_never_pushed` (test and `MUTATIONS.md` row). 3: the
  optional typed command builder is not done. 4: attempt 1's gate 1 sentence overclaimed the
  prepared-EQ spellings; the MINOR-1 arm now covers them.
- **Tests and test value** (each mutation applied, run red, restored; rows in
  `hosts/host-web/MUTATIONS.md`):
  - `a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track`: red if the prepared-owner EQ
    path spells its queue base or admission owner marker with the track count. Mutations:
    `prepared_queue_address` Eq base `ready.tracks.len() * 3` -> red; admission marker
    `queue_slot: ready.tracks.len() * 3 + effect` -> red. The committed attempt 1 suite survived both.
  - `a_single_lane_bus_mute_equals_the_bus_booted_with_that_lane_muted`: red if kind 4 takes the
    effective mute of the wrong lane (an acknowledged right-lane mute that never mutes). Mutation:
    `let lane = 0_usize` -> red.
  - `test-web-audioworklet.mjs` worklet classifier block: red if a bad rack or effect at a bus index
    reports `unknownTrack`. Mutation: `message.trackIndex >= this.trackCount` -> red (`2` vs `4`).
  - `test-web-audioworklet.mjs` bus `observe()` binding: red if a bus binding's `frameSlot` does
    not resolve, through the documented mapping, to the frame's `submixGrDb[0]`. Mutations: host
    `frameSlot: 0` -> red; host `Math.min(trackIndex, 1)` -> red; the mapping read as a plain
    `trackGrDb` index (the old contract) -> red (`undefined` vs `1.25`).
- **Gates** (x86_64 AVX2; A = `target/ci/k2-1213c-artifacts`, B = `target/ci/k2-1213c-named`), all
  rc 0: `cargo fmt --check`; `cargo test -p host-web --lib` (141 passed, 0 failed, 1 ignored);
  `build-web-audioworklet.sh --named-twin`; `check-web-audioworklet.sh`;
  `check-browser-expected-resources.py --artifacts`; `test-web-audioworklet.sh`;
  `check-sdk-generated.sh`; `check-sdk-types.sh`; `check-sdk-headless.sh` (346 pass, 0 fail); `sdk-package.sh check`;
  `check-/test-` realtime, host-core and workspace policy; workspace clippy `-D warnings`;
  `cargo doc` `-D warnings`. Not rerun: the DESIGN section 7 workspace test command (disk; this
  attempt touches only host-web tests, docs and the JS harness).
- **ARTIFACT UNCHANGED:** shipped module `7d6c0a8b...0f90b` (2 695 834 B), the same as attempt 1.
  The Rust doc edits keep `lib.rs`'s line count, so no panic-location line moves.

## Dependencies

- *Carry submix strips in the browser meter frame* (#1209)
- *Give every strip one mute owner and live-control producers in host-core* (#1211)
- *Add the notSoloable command reason to every vocabulary spelling* (#1212)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Admission is all or nothing. A refusal is decided before anything is pushed. No ack precedes a drop.
  Never emit a redundant solo-derived record, and never write the effective-mute formula a second
  time.
- In-place V1 amendment: append, never renumber; keep the P17 spellings.
- "Bit-identical" gates are hard stops. NaNs are folded to one value (decision 10).
- A test that greps source or prose is refused.
- Commit on the K2 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
