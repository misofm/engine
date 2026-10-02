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

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The list of per-track tables and band spellings that became per-strip.
- The ARTIFACT CHANGED report.

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
