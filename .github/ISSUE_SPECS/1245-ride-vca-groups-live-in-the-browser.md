# Ride VCA groups live in the browser

Slice V6 of *VCA groups* (#1239). It is in the VCA batch, after *Compose live VCA moves in host-core*
(#1244). It is the browser's admission of two new command kinds, as *Admit live send commands in the
browser* (#1222) was for sends: read its attempt record (`.github/ISSUE_SPECS/1222-*.md` or git
history) for the vocabulary re-anchoring this slice repeats. *Enumerate VCA groups and drive them
from the SDK* (#1246) publishes the VCA order and the SDK surface.

The design record cited below (`DESIGN`, `VERIFY-2`) is committed in
`docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

In the browser engine, one live command rides or mutes a VCA. Every member strip's fader follows
through its existing declicked fader ramp, together, all or nothing:

- a member's own fader move still works on top of the VCA, and the member keeps its own balance;
- a VCA mute mutes every member, survives solo, and silences its members' `follows_mute` sends,
  exactly as a fresh plan prepared with the VCA muted renders them (#1242 made preparation agree);
- render code is unchanged: composition happens at admission, through #1244's `LiveVcaState`.

## Context (verified on `8c6268967`; `hosts/host-web/src/lib.rs` unless stated)

- **After #1244:** `host_core::LiveVcaState` (`try_new`, `empty`, `vca_count`, `reaches`,
  `reached_by`, `reached_strip_count`, `set_vca_db`, `set_vca_mute`, `set_member_db`, `effective_db`, `vca_mute`,
  `fader_delta`, `record_emitted_db`, `commit`, `rollback`) and
  `LiveControlSoloState::set_vca_mute(strip, [bool; 2])`. After #1242, the solo state composes
  `user_mute || vca_mute || solo-term` and host-web seeds `vca_mute` before `LiveRouteState`
  (`:6282-6420`).
- **Kinds** are 1-15 (`:799-888`; `COMMAND_ROUTE_MATRIX = 15` at `:888`); **reasons** 0-13
  (`:969-1019`; `COMMAND_REASON_UNKNOWN_ROUTE = 13` at `:1019`). The decode whitelist is
  `:4040-4058`. `into_track_record` (`:4119-4228`) holds kinds 3 and 4's shape rules: `rack` must
  be 255, `channel` a lane selector (`lane_selector`), `values[1..]` zero (else `malformed`);
  kind 3's `values[0]` in `[-144, 24]` (else `domain`, `:4160-4163`), kind 4's exactly 0 or 1.
- **Admission** (`admit_commands`, `:4442-4483`, commits or rolls back `solo` and `routes` at
  `:4463-4471`; `admit_commands_staged`, `:4485-`):
  - the strip-index check skips the route kinds (`:4537-4547`) because their index word is a
    live-route index;
  - kind 3 is lowered in the shared track-record arm (`:4549-4566`) to one `FaderDb` on slot
    `strip_count + strip`;
  - kind 4 goes through `set_user_mute`, `effective_mute` and `record_emitted` (`:4580-4610`);
  - kind 9 (solo) stages nothing and records `solo_seen`, `solo_first_wire_index` and
    `solo_smoothing` (`:4614-4638`);
  - the coalescing pass (`:4879-4919`) runs `if solo_seen` and stages each strip's `strip_delta`
    with `solo_smoothing` at `solo_first_wire_index`;
  - the follow pass (`:4921-5010`, #1224) runs `if (solo_seen || mute_seen)`, takes its ramp from
    the last strip-mute record staged for the source strip that covers a changed lane, and refuses
    a ramp past `ROUTE_RAMP_LENGTH_MAXIMUM` at that record's wire index;
  - staged entries are room-checked on every queue before any push.
- **Staging** is `command_staging_count(strip_count, route_count)` (`:6738-6742`):
  `2 * MAXIMUM_COMMAND_RECORDS + 2 * strip_count + route_count`; its decoded array is charged at
  `:6574-6592`. The route mirror's bytes are charged into `bridge_metadata_bytes` and
  `bridge_retained_bytes` at `:6418-6440` (#1222 deviation 3). The browser fixture's retained rows
  are budgets, not exact pins (#1060).
- **`ReadyOwnership`** (`:1452-`) owns `solo` and `routes`; it is built at `:6594-6625`.
- **The seven spellings of a kind:** `COMMAND_*` here; the decode whitelist; host JS `COMMAND_KINDS`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:259`); the `.d.ts` `MisoCommandKind`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:191-297`, with the applied-kind prose
  at `:15-24` and the index, smoothing and values docs at `:355-375`) and its byte-identical mirror
  `sdk/src/browser/shipped-host.d.ts`; the metadata generator rows
  (`tools/parameter-metadata/src/lib.rs:156-186`); `wire_command_kinds`
  (`tools/parameter-metadata/src/abi_layout.rs:1760-1775`); the schema-gate lists
  (`scripts/check-parameter-metadata-v1.py:34-38`, `scripts/check-abi-layout-v1.py:77-81`); and the
  self-test fixtures (`scripts/fixtures/parameter-metadata-v1-self-test.json:21`,
  `scripts/fixtures/abi-layout-v1-self-test.json:621`).
- **The reason spellings:** the Rust constant; host JS `COMMAND_REASONS`
  (`miso-engine-v1-audio-worklet-host.js:223-239`); the `.d.ts` `MisoCommandReason` (`:300-340`) and
  its mirror; `tools/parameter-metadata/src/lib.rs:198-216` and `abi_layout.rs:1777-1791`; the
  schema-gate lists (`check-parameter-metadata-v1.py:54-58`, `check-abi-layout-v1.py:82-86`); and both
  self-test fixtures (`parameter-metadata-v1-self-test.json:37`, `abi-layout-v1-self-test.json:622`).
- **Self-test anchors that move:**
  - `scripts/check-command-kind-vocabulary.py`: the "added and not threaded" mutation
    `COMMAND_SOLO_MODE: u32 = 16` after `COMMAND_ROUTE_MATRIX` (`:384-397`); the kind-set literals
    `[1 … 15]` (`:433-451`), the undecoded `16` (`:450`), the `<= 15` literal (`:458`), and the two
    "added last" mutations that drop `routeMatrix` (`:548-558`);
  - `scripts/test-web-audioworklet.sh`: the kind-set literal (`:277`) and the `16` mutation
    (`:290-296`); the `FUTURE_TAP = 14` mutation (`:245`);
  - `scripts/check-command-reason-vocabulary.py`: `FUTURE_TAP = 14` after `UNKNOWN_ROUTE = 13`
    (`:326-334`), the renumber targets `UNKNOWN_TAP = 14` (`:340`) and worklet
    `UNSUPPORTED_KIND = 14` (`:413`), the `reason <= 13` bound (`:365`) and the list literals
    (`:348`, `:428`);
  - `scripts/check-parameter-metadata-v1.py:639`: `commandReasons[13].update(value=14)`;
  - `hosts/host-web/MUTATIONS.md:132-133` (the reason drift rows).
- **The SDK vocabulary eval** `sdk/test/live-controls-evals.mjs:111-122` asserts that the semantic
  methods cover the generated `wireCommandKinds` exactly (`kindNames`); the regenerated assets make
  it red until the SDK names the new kinds (#1222 deviation 2 used an "awaiting" exclusion).
- **The native harness** is `hosts/host-web/src/tests.rs`: `render_pair_and_compare` (`:7280`),
  `submit_strip_source` (`:9232`) and `strip_planes` (`:9222`) for distinct per-strip, per-lane
  signals, #1224's follow fixture (`SOLO_FOLLOW_*`, `:11915-11930`), `stage_solo` (`:7234`),
  `stage_lane_mute` (`:11870`), and the allocation measure
  `crate::ffi::live_response_ffi_tests::measured` (`hosts/host-web/src/ffi.rs:4572`, used by
  `follow_records_admit_and_render_without_allocating`, `tests.rs:12624`).
- **The callgraph rule** (`scripts/check-web-audioworklet-callgraph.py`, run by
  `check-web-audioworklet.sh` on `miso_engine_web_v1_command_submit --allocation-only`) refuses a
  reachable function named with `free`.

## Decisions frozen for this slice

- **D1. Kinds** (in-place V1 amendment; append, never renumber): 16 `vcaFaderDb`
  (`COMMAND_VCA_FADER_DB`) and 17 `vcaMute` (`COMMAND_VCA_MUTE`), both on the render plane and
  applied.
  - Record shape: kind 3's and kind 4's rules -- `rack` 255, `channel` 0/1/2 (left, right, both),
    `values[0]` the offset in `[-144, 24]` dB (else `domain`) or exactly 0 or 1 (else `domain`),
    `values[1..]` 0.0, `smoothing_samples` the ramp -- plus `effect_index == 0` and
    `parameter_id == 0` (else `malformed`), as the route kinds rule (`into_route_edit`,
    `:4229-4236`); kinds 3 and 4 do not check those two words.
  - The index word is a **VCA index**, in canonical VCA-ID order (the normalized model's `vcas`).
    The strip-index check skips the VCA kinds, as it skips the route kinds.
  - A host without live controls refuses them with `unsupportedKind`, as the route kinds.
- **D2. Reason** 14 `unknownVca` (`COMMAND_REASON_UNKNOWN_VCA`, result `RESULT_INVALID_ARGUMENT`): a
  VCA index at or past `vca_count` (a session without VCAs included) refuses at its wire index and
  stages nothing. Reusing `unknownTrack` or `unknownRoute` would misname the address.
- **D3. Admission.** `ReadyOwnership` gains `vcas: LiveVcaState`, built at boot from the normalized
  model after the solo state; `admit_commands` commits or rolls it back where it does `solo` and
  `routes`.
  - **Kind 16** calls `set_vca_db`, records `vca_fader_seen`, the first such record's wire index and
    the last one's smoothing, and stages nothing itself.
  - **Kind 17** calls `set_vca_mute`, then, at once, for every strip in `vcas.reached_by(index)`,
    `solo.set_vca_mute(strip, vcas.vca_mute(strip))`, so a later kind 4 in the same batch composes
    with it; it records `vca_mute_seen`, the first such record's wire index and the last one's
    smoothing, and stages nothing itself.
  - **Kind 3 on a reached strip** calls `set_member_db`, then stages `FaderDb` with
    `effective_db` for the covered lanes: one record with the command's lanes when the covered lanes'
    effective values are equal, else one `Left` and one `Right`; then `record_emitted_db`. Kind 3 on
    a strip no VCA reaches lowers exactly as today, byte for byte.
  - **The VCA fader pass**, after the batch loop and before the mute coalescing pass, runs
    `if vca_fader_seen`: for every reached strip it stages each `fader_delta` entry on the strip's
    fader slot, with the last kind 16 record's smoothing at the first kind 16 record's wire index,
    and `record_emitted_db`s it.
  - **The mute coalescing pass** runs `if solo_seen || vca_mute_seen`; its smoothing is that of the
    last kind 9 or kind 17 record in wire order, and its wire index that of the first. Otherwise it
    is unchanged.
  - **The follow pass** runs `if solo_seen || mute_seen || vca_mute_seen`, unchanged otherwise: its
    ramp is the coalesced mute record's, so a kind 17 whose smoothing exceeds
    `ROUTE_RAMP_LENGTH_MAXIMUM` and moves a following send refuses the whole submission (`domain`)
    at the coalesced record's wire index (the batch's first kind 9 or 17 record), as a solo's does.
  - Order inside one admission: (1) VCA fader records, (2) strip mute records, (3) follow records,
    all staged before the one room check; nothing is pushed before every destination's room is
    known.
- **D4. Staging and resources.** `command_staging_count` grows by `2 * vcas.reached_strip_count()`
  (at most one `FaderDb` per reached lane per batch; the mute pass is already bounded by
  `2 * strip_count`). `LiveVcaState::retained_bytes()` is charged into `bridge_metadata_bytes` and
  `bridge_retained_bytes` beside the route mirror, and `largest_allocation_bytes()` is folded into
  `largest_bridge_allocation_bytes` and `largest_named_allocation_bytes`. A session without VCAs
  adds nothing to any of them.
- **D5. Smoothing** is the record's `smoothingSamples` until #1054 gives the session a table.
- **D6. Vocabulary.** Every kind and reason spelling in the Context; the regenerated
  `sdk/assets/**` and `sdk/src/generated/**` (`node codegen/assets.mjs && node codegen/generate.mjs`
  in `sdk/`); the self-tests re-anchored past the new last kind and reason:
  `COMMAND_SOLO_MODE: u32 = 18` after `COMMAND_VCA_MUTE`, the undecoded kind 18, the kind-set
  literals `[1 … 17]` and `<= 17`, the "added last" mutations dropping `vcaMute`;
  `COMMAND_REASON_FUTURE_TAP: u32 = 15` after `UNKNOWN_VCA = 14` (script, shell test,
  `MUTATIONS.md`), the reason renumber targets and the `<= 14` bound at the next free value, and
  `commandReasons[14].update(value=15)` in `check-parameter-metadata-v1.py`. In
  `sdk/test/live-controls-evals.mjs`, `kindNames` names `vcaFaderDb` and `vcaMute` as awaiting
  #1246 (an explicit exclusion list, so any other kind without a method still fails); #1246 moves them
  into `kindNames`.

## Deliverables

1. host-web D1-D5 (constants, decode whitelist, admission, the two passes, staging, resources,
   commit and rollback).
2. Every kind and reason spelling and the regenerated SDK assets (D6).
3. `hosts/host-web/MUTATIONS.md` rows for each new kind, the new reason, the moved drift mutations
   and each gate's red mutation.
4. A "Live VCA groups" paragraph under "Solo in place" in `docs/BUILTINS_AND_METERING_V1.md`.

## Authorized paths

- `hosts/host-web/src/{lib.rs,tests.rs}`, `hosts/host-web/web/**`, `hosts/host-web/MUTATIONS.md`
- `tools/parameter-metadata/**` (the kind and reason rows only)
- `scripts/check-command-kind-vocabulary.py`, `scripts/check-command-reason-vocabulary.py`,
  `scripts/check-parameter-metadata-v1.py`, `scripts/check-abi-layout-v1.py`,
  `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/fixtures/parameter-metadata-v1-self-test.json`, `scripts/test-web-audioworklet.sh`,
  `scripts/test-web-audioworklet.mjs` (only where it spells kinds or reasons)
- `sdk/assets/**`, `sdk/src/generated/**` (regenerated only), `sdk/src/browser/shipped-host.d.ts`,
  `sdk/test/live-controls-evals.mjs` (the `kindNames` exclusion only)
- `docs/BUILTINS_AND_METERING_V1.md`
- this spec

## Non-goals

- No export, `SessionMap` field or SDK `vca(id)` API (#1246).
- No change to host-core's composition (#1244) or to preparation (#1242).
- No C ABI path (#1247). No VCA solo, no send trim and no automation. No render-code change.

## Hazards

- **A redundant record moves bits.** VCA-derived fader and mute records go out only for lanes whose
  effective value changed.
- **Clamping.** A member pushed past +24 dB by a VCA clamps; when the VCA comes back, the member
  returns to its own balance.
- **Fan-out and backpressure.** A 64-member VCA move is up to 128 fader records plus mute and follow
  records in one admission; a full member or send queue refuses the whole submission.
- **Ordering.** Follow records must use the batch's final effective mutes, after user mute, VCA mute
  and solo.
- **Vocabulary churn.** A missed spelling fails the kind or reason gate, and a missed self-test
  anchor makes a mutation "match nothing", which fails the self-test. Fix both; never silence them.

## Objective gates

All gates run in the native host-web harness (`hosts/host-web/src/tests.rs`) unless stated. Every
session feeds distinct, non-constant signals per strip and per lane, and compares at a destination
with no stateful downstream (empty console, no stateful inserts, identity input section).

1. **A VCA ride lands on a fresh plan's bits.** After the ramp, a random VCA move (nested,
   overlapping, clamping; 8 seeds) renders bit-identically to a host booted from the session with
   the committed VCA values.
   *Test value: it turns red if live admission emits to the wrong member, lane or slot, or composes
   differently from preparation.*
2. **Member and VCA compose.** The sequence member fader -3 dB, then VCA -6 dB, then member +2 dB,
   each with smoothing 0 at a block boundary, ends bit-identical to a host booted with the member
   at +2 dB in the VCA at -6 dB; in a second case the member is at +20 dB, the VCA moves to +24 dB
   and back to 0 dB, and the member is bit-identical to an unedited host. Un-muting the VCA leaves
   a member's own mute on.
   *Test value: it turns red if a member move overwrites the VCA term or the reverse, if the clamp
   is stored, or if a VCA un-mute clears a member's own mute.*
3. **No redundant records.** A VCA move that changes no member's effective value (every member
   clamped at -144 dB) stages no record and leaves the output digest unchanged; a VCA mute of an
   already user-muted member stages no mute record.
   *Test value: it turns red if composition re-emits unchanged targets and moves settled zeros.*
4. **All or nothing.** A VCA move that would overfill one member's fader queue, and a VCA mute
   whose follow record would overfill a send's queue, are each typed backpressure: nothing is
   pushed, and the VCA, solo and route mirrors are unchanged (a later identical submission behaves
   as if the first never happened).
   *Test value: it turns red if any record is pushed, or a mirror commits, before every
   destination's room is checked.*
5. **VCA mute flows to sends and survives solo.** In #1224's follow fixture with `drums` and `bass`
   in VCA `band`: muting `band` silences both members' `follows_mute` pre-fader sends,
   bit-identically at the destinations' inputs to a host booted with `band` muted; soloing `vocal`
   and un-soloing it keeps them silent; un-muting `band` restores them bit-identically to an
   unedited host.
   *Test value: it turns red if the VCA mute term does not reach `LiveRouteMuteFollow::delta`, if
   un-soloing clears it, or if the live path and preparation disagree about a VCA-muted member's
   send.*
6. **Addressing and shape.** `vcaFaderDb` and `vcaMute` at an index at or past the VCA count, and in
   a session without VCAs, refuse with `unknownVca` and stage nothing; a wrong `rack`, a nonzero
   `values[1]` or a bad `channel` is `malformed`; an offset of 24.5 dB or a mute value of 0.5 is
   `domain`; a nonzero `effect_index` or `parameter_id` is `malformed`; every existing host-web
   live-control test passes unchanged.
   *Test value: it turns red if a VCA index is checked against another table, a bad index is refused
   with a reason that misnames it, or the shape rules differ from kinds 3 and 4's rules plus the
   route kinds' zero `effect_index` and `parameter_id`.*
7. **Staging.** A batch of 256 records with every member of a VCA reaching all strips moved fits the
   grown staging; `command_staging_count` for a session without VCAs is unchanged.
   *Test value: it turns red if staging is not grown by the reached strips, so a full batch plus its
   VCA records overruns.*
8. **Render and admission allocate nothing** during VCA rides and mutes (with follow records), as
   `follow_records_admit_and_render_without_allocating` measures: `allocations == 0` and
   `deallocations == 0`.
   *Test value: it turns red if VCA admission or its passes allocate on the audio thread.*
9. **Vocabulary, browser and SDK** (`npm ci` in `sdk/` first; `<A>`, `<B>` fresh empty directories):
   - `python3 -B scripts/check-command-kind-vocabulary.py --self-test` and
     `python3 -B scripts/check-command-kind-vocabulary.py`
   - `python3 -B scripts/check-command-reason-vocabulary.py --self-test` and
     `python3 -B scripts/check-command-reason-vocabulary.py`
   - `rm -rf <A> <B> && mkdir -p <A> <B> && bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`
   - `python3 -B scripts/check-parameter-metadata-v1.py --self-test` and
     `python3 -B scripts/check-parameter-metadata-v1.py <A>/miso-engine-v1-parameter-metadata.json`
   - `python3 -B scripts/check-abi-layout-v1.py --self-test` and
     `python3 -B scripts/check-abi-layout-v1.py <A>/miso-engine-v1-abi-layout.json`
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`
   - `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-sdk-generated.sh <A>`, `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh <A>`, `bash scripts/sdk-package.sh check <A>`
   - The browser legs: in `hosts/host-web/qualification`, `npm ci`,
     `npx playwright install <browser>`, then
     `npm run qualify -- --artifacts <abs A> --sdk-root <abs sdk> --browser <browser> --check-matrix --self-test-mutations`
     for `chromium`, `firefox` and `webkit`, under a private PulseAudio null sink as the `browser`
     job of `.github/workflows/qualification.yml` (`:362-430`) sets it up.
10. **Workspace and policy.**
    - `cargo test --locked --workspace --all-targets --exclude lane --exclude math --exclude effect-runtime --exclude delay --exclude compressor --exclude multiband-compressor --exclude gate-expander --exclude true-peak-limiter --exclude transient-shaper --exclude soft-clip --exclude parametric-eq --exclude builtins --exclude dsp-reference --exclude conformance --exclude audit --exclude bench --exclude console-workload --exclude wasm-gates --exclude wasm-gate-guest --exclude wasm-gate-corpus --features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`
    - `cargo fmt --all -- --check`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
    - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
    - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
      (`check-host-core-policy.sh` scans every `hosts/*/src`)

host-web has no aarch64 leg; its 4-lane coverage is the shipped simd128 module through
`check-sdk-headless.sh` and `check-browser-expected-resources.py --artifacts` (DESIGN section 7).

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The re-anchored mutation list, with each mutation's observed red result.
- The shipped module's ARTIFACT CHANGED report.

## Dependencies

- *Compose live VCA moves in host-core* (#1244)
- *Let a send follow its source strip's mute live in the browser* (#1224)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Admission is all or nothing. No ack precedes a drop. Never emit a redundant record.
- In-place V1 amendment: append kinds and reasons; never renumber.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- A test that greps source or prose is refused.
- Commit on the VCA batch branch; do not push until the batch closes.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
