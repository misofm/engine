# Retire the browser master boot word

Successor of #1683 (the master strip is a core `master` monitor row), decision 17 C1
(`docs/rulings/one-control-abstraction-2026-10-13.md`: a behaviour that exists on one host only is
a defect). Code anchors verified on branch `codex/controls-batch-14aw` at `5ef2c81d8` (the #1683
tree). Issue number: assigned at filing (written here as #1689). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A browser app designates the master strip one way only: it writes the strip's `master` row
(`strip.master(enabled)`, `SET_CONTROL`), as a C ABI app does. The boot options carry no master
word. The master reading (`masterGrDb`) and the meter header's `masterTrackPlusOne` are the
values they are today; only the second route to the designation is gone.

## Context

- **Two routes today.** #1683 made the master a core monitor row held by strip ID
  (`controls::MonitorOverlay::master`). The browser boot word still designates the first plan's
  master by strip index. It seeds the same overlay, so the two routes are one state, but the word
  is a browser-only feature, it needs a strip that exists at boot, and it ties the master reading
  to observation capacity at boot (`live_control_master_track_plus_one != 0` needs
  `live_control_observation_taps != 0`).
- **The core path of the word.**
  - `HostLiveControlRequest::master_track` (`crates/host-core/src/prepare.rs:401`, default
    `None` at `:411`); the preparation resolves it for a first plan and refuses a successor
    request that names one, both with `host.observation.master_track` (`:1942-1955`); the boot
    overlay is built from it (`:1957-1968`).
  - `HostLiveControlHandles::master_track` echoes it (`prepare.rs:536`, filled at `:3289`).
  - `PreparedRuntime::master_track` (`crates/control-plane/src/compile.rs:30`, set at `:937` and
    `:1039`, ignored at `:1130`), which seeds the session overlay at boot (`compile.rs:1083-1095`).
    `crates/control-plane/src/control.rs:1699` ignores it in a pattern.
- **The browser path of the word.**
  - `WebBootOptions::live_control_master_track_plus_one` at offset 56, the last word of the 64-byte
    block (`hosts/host-web/src/lib.rs:1103`, default `:1121`; `hosts/host-web/src/ffi.rs:519`).
  - Its validation, `web.options.live_controls` (`lib.rs:4926-4930`); its translation into the
    request (`live_control_request`, `lib.rs:6459-6462`); the successor reset to `None`
    (`lib.rs:5578`); the plan shape (`PlanShape::master_track`, `lib.rs:1398`; built at
    `:5841` and `:5891`; resolved after every control command by `ReadyOwnership::sync_master`,
    `:1655-1675`).
  - The precedent for a retired word: offset 32 was the live-command queue word. #1565 reserved it
    as `reserved1`, kept it in the layout, and refuses a writer that sets it with
    `RESULT_REFUSED_OPTIONS` and `web.options.reserved1` (`lib.rs:4917-4924`), by the worklet's `writeBootOptions`
    (`miso-engine-v1-audio-worklet.js:939-957`) and by `validBootOptions`
    (`miso-engine-v1-audio-worklet-host.js:630-645`).
- **Every place the word is laid out or written** (all verified by search on the #1683 tree):
  - Rust host: `hosts/host-web/src/lib.rs`, `hosts/host-web/src/ffi.rs`, the layout tests in
    `hosts/host-web/src/tests.rs` (`:315` and the boot helpers at `:3085`, `:4415`, `:4459`,
    `:5234`, `:8097`, `:8242`, `:8271`).
  - Worklet JS: `hosts/host-web/web/miso-engine-v1-audio-worklet.js` (the field list `:81`,
    `writeBootOptions` and its observation check `:939-957`), `miso-engine-v1-audio-worklet-host.js`
    (the field list `:73`, the descriptor copy `:520`, `validBootOptions` `:640-642`), and the declaration `miso-engine-v1-audio-worklet-host.d.ts:718` with its
    byte-identical twin `sdk/src/browser/shipped-host.d.ts:718`.
  - SDK boot policy: `sdk/src/browser/host-mirror.ts:45-58,81` (the `masterTrackPlusOne` word and
    its "requires liveControlObservationTaps" check), `sdk/src/core/abi.ts:118-124` (the
    `liveControls.masterTrackPlusOne` option), `:145` (`POLICY_WORDS`), `:190` (the write),
    `sdk/src/core/boundary.ts:204-209` (the option copy), and the tests that set it
    (`sdk/test/host-mirror.ts:41`, `sdk/test/browser-evals.mjs:38,141`,
    `sdk/test/observation-subscription-evals.mjs:564,592`, `sdk/test/capability-evals.mjs:227,290,342`).
  - ABI layout generator and checkers: `tools/parameter-metadata/src/abi_layout.rs:275-279`,
    `tools/parameter-metadata/tests/abi_layout.rs:300`, `scripts/check-abi-layout-v1.py`
    (`BOOT_OPTION_FIELDS` `:252`, `POLICY_WORDS` `:259`), `scripts/fixtures/abi-layout-v1-self-test.json:124`;
    generated mirrors `sdk/assets/miso-engine-v1-abi-layout.json:124` and
    `sdk/src/generated/abi.ts:192`.
  - Qualification harness and scripts: `hosts/host-web/qualification/qualification.js`
    (`:21` `OBSERVATION_MASTER_TRACK_PLUS_ONE`, `:58`, `:71`, `:663`, `:886`),
    `hosts/host-web/qualification/suspended-host.mjs` (`:46`, `:427`, `:579`, `:739`),
    `hosts/host-web/qualification/rebuild-cost.mjs` (`:72`, `:109`),
    `hosts/host-web/tests/browser-v1/browser-correctness.js:21`,
    `scripts/test-web-audioworklet.mjs` (`:65`, `:1043`, `:3268-3287`),
    `scripts/web-mixing-automation-benchmark.mjs` (`:290`, `:776`),
    `scripts/check-browser-expected-resources.py` (`:308`, `:508`, `:519`, `:597`: it edits the
    word to build its mutations).
  - Docs: `docs/EFFECT_OBSERVATION_V1.md`.
- **Not this word.** `WebMeterHeader::master_track_plus_one` (offset 40, `lib.rs:975`) is the
  frame's report of the resolved master. It stays.

## Decisions frozen for this slice

- **D1. The field stays, reserved, never reused.** The word at offset 56 becomes `reserved2`
  (`u64`). The block stays 64 bytes and every other offset is unmoved. No later word takes the
  offset. This is the #1565 pattern for `reserved1`.
- **D2. A boot that sets it is refused.** `validate_options` refuses a nonzero `reserved2`
  with `RESULT_REFUSED_OPTIONS` and the typed reason `web.options.reserved2`, as `reserved1`
  does. The worklet's `writeBootOptions` and the host's `validBootOptions` refuse it the same way
  before the module sees it. The `web.options.live_controls` rule loses its master clauses.
- **D3. The core request loses the field.** `HostLiveControlRequest::master_track`,
  `HostLiveControlHandles::master_track` and `PreparedRuntime::master_track` are deleted, with
  the first-plan overlay seeding and the `host.observation.master_track` refusals that existed
  only for them. The master of a plan is the overlay's, by strip ID, for a first plan and a
  successor alike. A boot catalog lists no master; the app writes the row after boot.
- **D4. The SDK drops the option.** `liveControls.masterTrackPlusOne` leaves the SDK's boot
  options and `POLICY_WORDS`; the SDK always writes `reserved2` as `0n`. `strip.master(enabled)`
  (the generated builder from #1683) is the SDK's one way. The ABI-layout JSON, the generated
  TypeScript and the self-test fixture are regenerated, not edited.
- **D5. The master needs no boot capacity.** A strip's `master` row is a monitor write that spends
  no revision. Its `masterGrDb` reading still needs an armed observation tap on the master strip;
  it no longer needs `liveControlObservationTaps` at boot to be nonzero for the word. (An app
  that wants taps still asks for capacity, as before.)
- **D6. The qualification harness designates through the row.** Every harness boot that set the
  word (`OBSERVATION_MASTER_TRACK_PLUS_ONE`, the mock options of `test-web-audioworklet.mjs`, the
  benchmark scripts) boots with `0n` and sends `SET_CONTROL master` for the strip after the
  boot, or for a harness with no command path, boots a session the row can already address.

## Deliverables

1. D1-D3 in `hosts/host-web/src/{lib.rs,ffi.rs}`, `crates/host-core/src/prepare.rs`,
   `crates/control-plane/src/{compile.rs,control.rs}` and the Rust tests that name the word.
2. D1-D2 in the worklet JS and its two declaration twins.
3. D4 in `sdk/src/**`, `sdk/test/**` and the regenerated mirrors.
4. D6 in the qualification harness and the scripts listed above.

## Authorized paths

- `hosts/host-web/src/{lib.rs,ffi.rs,tests.rs}`, `hosts/host-web/tests/control_parity.rs` (only
  where it boots with the word), `hosts/host-web/tests/browser-v1/**`,
  `hosts/host-web/qualification/**`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet{,-host}.js`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `sdk/src/browser/shipped-host.d.ts`
  (byte-identical twins).
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/**` (callers of the request field),
  `crates/control-plane/src/{compile.rs,control.rs}`.
- `sdk/src/{core,browser,generated}/**`, `sdk/test/**`, `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/assets/miso-engine-v1-parameter-metadata.json` only if the generator moves it.
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`,
  `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`,
  `scripts/test-web-audioworklet.mjs`, `scripts/web-mixing-automation-benchmark.mjs`,
  `scripts/check-browser-expected-resources.py`, `docs/EFFECT_OBSERVATION_V1.md`.

## Non-goals

- No new master behaviour: the `master` row, its monitor overlay, `sync_master` and the meter
  header's `masterTrackPlusOne` keep their #1683 meaning.
- No change to the 64-byte block size or to any other boot word.
- No master reading on the C ABI (#1690).
- No change to the memory pins in `expected.json` unless the measurement moves; if it does, the
  PR records the measured numbers.

## Objective gates

Commands: `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo test --locked -p host-web --all-features`,
`cargo test --locked -p parameter-metadata`, `python3 -B scripts/check-abi-layout-v1.py` and its
self-test, `bash scripts/check-sdk-generated.sh <artifacts>`, `bash scripts/check-sdk-types.sh`,
`cargo run --locked --release -q -p realtime-policy`, and the worklet gates of #1534 gate 3
(`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
`test-web-audioworklet.sh`).

1. **Boot with the word at zero; the row sets the master; the reading is today's.** In
   `hosts/host-web/src/tests.rs`, a session with a compressor track and observation capacity boots
   with `reserved2 == 0`, the app writes `SET_CONTROL master` for the track, arms the tap, and the
   header's `masterTrackPlusOne` and the master gain word equal the values the
   boot-word designation gives on the base (`structural_apply::a_master_designated_after_an_empty_boot_reads_as_a_boot_designation`
   already asserts the bit-for-bit equality with the track's own word; this gate keeps it and
   removes the boot-word half). In the browser qualification, `masterMatchesTrack` and
   `masterPresent` (`qualification.js:733-736`) hold, and `armedMasterEqualsTrack` in
   `expected.json` stays `true`.
2. **The reserved field refuses.** A native boot with `reserved2 != 0` returns
   `RESULT_REFUSED_OPTIONS` with the reason `web.options.reserved2`; the worklet's
   `writeBootOptions` and the host's `validBootOptions` refuse it in `test-web-audioworklet.mjs`; both
   refuse for the values `1n` and `0xffff_ffff_ffff_ffffn`. Boot with `0n` is accepted.
3. **Mirrors current.** `check-abi-layout-v1.py` and its self-test pass with the field named
   `reserved2` at offset 56; `check-sdk-generated.sh` and `check-sdk-types.sh` pass; the two
   declaration twins are byte-identical; the SDK exposes no `masterTrackPlusOne` boot option and
   writes `reserved2` as `0n`.
4. **Nothing else moved.** The memory pins, the meter frame bytes and `control-list.json` are
   unchanged (the PR records a base run).

## Test value

- Gate 1 turns red if the master designation or its reading depends on the boot word, or if the
  row path gives another value than the word did.
- Gate 2 turns red if a writer that still sets the retired offset is accepted and ignored, so a
  later word at offset 56 would silently inherit an old writer's value.
- Gate 3 turns red if the generated SDK, the layout JSON or a declaration twin still names the
  word, so the SDK writes a field the module refuses.
- Gate 4 turns red if the retirement moves a published browser byte.

## Dependencies

- #1683 (the `master` row and `sync_master`). It must land first; this slice rebases after it.
- *Boot an empty session on both hosts* (#1682) landed before #1683.
- `hosts/host-web/src/lib.rs` and the worklet files are shared with the open browser streams
  (#1332, #1381, #1588); this slice rebases after any of them that edits the same lines.

## Size

Half a day.

## Amendment (controls coordinator, batch 15a fold, 2026-10-16)

- **Pin moves: the non-goal governs.** The non-goal allows a memory pin in `expected.json` to move
  when the measurement moves, with the measured numbers in the PR. Gate 4 now reads: the meter
  frame bytes and `control-list.json` are unchanged; each memory pin in `expected.json` either
  equals the base run or moves to the measured value, with the base and measured numbers and the
  reason in the PR. No other published browser byte moves. Gate 4's test value line reads the same.
- **Dependencies.** #1381 is closed, so it is not an open stream to rebase after. The open browser streams are #1332 and #1588. #1683 is closed: the `master` row
  and `sync_master` are on `main`.
