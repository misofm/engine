# Address console slots and inserts in live control

Slice S1c of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`; Sol's M5 and amendment 5 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

Live controls address an effect by `(track_id, rack, effect_index)`, with the index taken in session
declaration order within the rack:
- `crates/host-core/src/prepare.rs:337-352`;
- `declared_effect_indices` in `crates/effect-compiler/src/prepare.rs:1450-1473`;
- `miso.command.v1`.

The racks carry six encodings: `RackName` 1-4, `ParameterRack` 1-4, `RackId` 1-3, `RackLocation`
1-3 in another order, `EffectRack` and two browser encodings.
- The browser's frozen 48-byte command record encodes rack `0` simd1, `1` dynamic, `2` simd2 and
  `255` not applicable (`hosts/host-web/src/lib.rs:3760-3770`).
- The live-response owner record encodes `0` input filters and `1..3` as `RackId`.
- The browser spectrum targets are `trackPostInputBuiltins` and `trackPostMatrix`
  (`scripts/check-abi-layout-v1.py:89`).

After S1a, the session addresses `console` slots and `inserts`, and none of these encodings can say
"console".

## Smallest closable slice

1. **Live address.**
   - A console slot is `(track_id, console, slot_index)`. `slot_index` is the slot's position in the
     session's slot order: `pre_insert`, then `post_insert`, which equals the index into the
     track's `console` array.
   - An insert is `(track_id, inserts, index)`.
   - Internally the address maps through the lowering: `pre_insert[k]` is `(Simd1, k)`,
     `post_insert[j]` is `(Simd2, j)` and `inserts[i]` is `(Dynamic, i)`.
   - Whether `EffectRack` becomes the session-facing `{Console, Inserts}` or stays internal behind a
     boundary translation is this slice's choice; record why. `RackId`, `TrackStage`, `MeterTap` and
     `RackLocation` stay (decision 12).
2. **The browser's 48-byte command record.** The rack byte keeps `1` for `inserts`. `0` and `2` are
   retired and refused with `unknownRack`. `3` is appended for `console`, and `255` stays "not
   applicable". The layout, size and `ABI_VERSION` are unchanged.
3. **The observation and live-response encodings** follow the same rule: `inserts` keeps its code,
   the simd1/simd2 codes are retired and refused, and `console` is appended. In the owner record,
   `2` stays `inserts`, `1` and `3` are retired, and `4` is `console`.
4. **The spectrum targets** follow the tap rename with codes unchanged: `trackPostInput` (1),
   `trackPostPan` (2) and `output` (3).
5. Update the host's `.d.ts` and refresh its SDK mirror (`sdk/src/browser/shipped-host.d.ts`,
   compared by `scripts/check-sdk-generated.sh`). Regenerate the ABI layout JSON and bindings.
6. **The V8 benchmark harness.** It writes the record's rack byte from the controls table
   (`scripts/web-mixing-automation-benchmark.mjs:279`), and it resolves rack names in its lookup at
   `:136-140`; this slice owns both. The table's codes come from
   `tools/console-workload/src/mixing_automation.rs`, through
   `examples/mixing_automation_controls.rs`, and today they are `0`/`1`/`2` with an index within
   the section (S1a's interim). This slice moves them to `3` plus the slot index for console slots
   and `1` plus the index for inserts. Without it, S4's V8 run refuses at preflight.

Authorized paths:
- `crates/effect-compiler/src/prepare.rs` (addressing);
- `crates/host-core/src/prepare.rs` and the live-control attach path;
- `hosts/host-web/src/**` and `hosts/host-web/web/**`;
- `tools/parameter-metadata/src/abi_layout.rs`;
- the generated `sdk/assets` layout, `sdk/src/generated/abi.ts` and the mirror;
- `scripts/check-abi-layout-v1.py` and its self-test fixture;
- `docs` pages that document the record;
- `scripts/web-mixing-automation-benchmark.mjs` (the rack byte only),
  `tools/console-workload/src/mixing_automation.rs` and
  `tools/console-workload/examples/mixing_automation_controls.rs`;
- this spec.

The SDK's builder and types are S1d's.

Live bypass after P1 (#1087; P1 verdict, L4): a live control can lift the session bypass of
every effect except two. The delay is seeded bypassed and keeps its prepared bypass. The
multiband keeps its prepared bypass under P1b (#1100). Document both exceptions where live
bypass is documented, and test that lifting either is refused or has no effect, whichever the
code does today.

## Owner decisions that bind this slice

Decision 12's "Wire identity": nothing is renumbered, every retired code is refused, there is no
`ABI_VERSION` bump, and the app updates in lockstep.

## Dependencies

- *Add the session console and per-track inserts to the session schema* (S1a, #1093).
- *Carry the session console and inserts in the control protocol* (S1b, #1094).
- *Rename the live console to live controls* (S1r, #1095), which has merged before S1a in the same
  batch (C3).

## Objective gates

1. A live parameter change and a live bypass reach the right lane:
   - on a console slot in `pre_insert` and in `post_insert`, addressed by slot index;
   - on an insert;
   - in both the native host-core path and the browser record path.

   A planted off-by-one in the section split (post_insert slot addressed as pre_insert) turns it red.
2. Records carrying rack `0` or `2`, and owner or observation records carrying a retired code, are
   refused with their typed reasons.
3. `python3 -B scripts/check-abi-layout-v1.py` (layout and self-test) and
   `bash scripts/check-capi-abi.sh` pass. The C ABI carries no rack addressing today, so it should
   not move; if it does, explain why.
4. The host-web qualification that drives the raw exports passes in the three browsers in CI mode.
   Run it without `--sdk-root` (`hosts/host-web/qualification/run.mjs:837-858`), which leaves out
   the SDK bundle and runs only the raw-export subset. SDK-driven qualification is S1d's gate.
5. The benchmarks still run. Both checks are untimed, on a committed clean tree:
   - `bash scripts/operator/preflight-console-benchmark.sh --step <an unused scratch name>`;
   - `bash scripts/run-web-mixing-automation-benchmark.sh prepare WORKDIR`, then `preflight
     WORKDIR`, in an empty scratch directory.

   The V8 preflight exercises the new rack codes through the shipped module.
6. PR evidence: console digests are unchanged.

## Standing rules for the implementer

- Work only from this body, the umbrella issue and decision 12. Read the cited code first.
- Class A: every gate that says "bit-identical" is a hard stop, not a tolerance. NaNs fold to one
  value (decision 10).
- Render stays allocation-, lock- and syscall-free. Only `crates/lane` names `wide` or intrinsics.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features --
  -D warnings` and the focused tests before every checkpoint.
- Every new test names the plausible defect that turns it red (AGENTS.md "Test value"). A
  one-time "no bit moved" comparison against the pre-change base is PR evidence, not a committed
  test.

## Attempt 1 evidence

Terra, attempt 1, on `codex/1096-live-console-addressing` from `105064a7` (S1r, S1a's C3 rebase
and S1b stacked). Commits `fa9c53a4` (addressing), `51bb08f9` (gates), `e77d42c5` (oracle,
processor, metadata-test comment) and `318df3fc` (line reflow). Every gate below ran on
`318df3fc`, x86-64-v3, except where a row says otherwise. The class-A base is `105064a7`.

### What landed

- **The choice: `EffectRack` stays internal, behind one boundary translation.** `EffectRack`
  (`Simd1`/`Dynamic`/`Simd2`) is still the graph's identity for a prepared instance:
  `EffectPreparedEntry` keys the graph by `(track, rack, effect id)`, and `pre_insert` and
  `post_insert` must lower to different racks for chain order, PDC, stage names and the sealed
  `MISO-GRAPH-V1` text (class A by lowering). A two-valued session enum cannot carry that. The
  session-facing address is new: `effect_compiler::LiveEffectAddress { rack: LiveEffectRack
  { Console, Inserts }, index }`. `EffectControlProducer` and `EffectObservationHandle` now carry
  only that `address` (their lowered `rack`/`effect_index` are gone), so nothing can address a
  channel by its lowered position and each struct has one index. `declared_live_addresses`
  derives it once from the normalized model (`pre_insert[k]` -> `console(k)`, `post_insert[j]` ->
  `console(pre_insert.len() + j)`, `inserts[i]` -> `insert(i)`), and `LiveEffectAddress::lower`
  is the one translation back (`console(k)` -> `(Simd1, k)` or `(Simd2, k - pre_insert)`,
  `insert(i)` -> `(Dynamic, i)`). `HostLiveControlHandles::effect_control_mut(track, address)` is
  the native lookup.
- **Browser record** (`hosts/host-web`). `RACK_INSERTS = 1`, `RACK_CONSOLE = 3` (appended),
  `RACK_NOT_APPLICABLE = 255`; `0` and `2` decode to nothing. Every effect-addressed kind refuses
  them with `unknownRack`; `effect_index` is the slot index or the insert index. One helper,
  `dense_effect_slot` (through `lower`), both places each producer and observation handle in the
  dense table at boot and finds it at admission, the configuration copy and the companion
  record. Layout, size and `ABI_VERSION` are unchanged.
- **Observation and live-response encodings.** Observation selections, results, the
  `observation_rack` export and the companion record carry `1`/`3` and refuse `0`/`2`
  (`RESULT_INVALID_ARGUMENT`; the companion is `malformed`; the configuration copy
  `RESULT_INVALID_ARGUMENT`). The live-response owner record keeps `0` (input filters) and `2`
  (inserts), retires `1`/`3` and appends `4` (console, either section): the capture translates the
  engine's native owner rack, and the analysis refuses a snapshot whose owner carries any other
  code. The worker-side copy keeps the record's code; the query does not read it, and the
  section is not recoverable from it by design.
- **Spectrum targets** follow the tap rename with codes unchanged: `trackPostInput` (1),
  `trackPostPan` (2), `output` (3) in the layout, the worklet host and processor, the `.d.ts` and
  the Rust constants (`SPECTRUM_TARGET_TRACK_POST_INPUT`/`_PAN`). The internal
  `SpectrumTarget` variants stay.
- **Layout JSON and bindings.** Regenerated through `sdk/codegen/assets.mjs` and
  `generate.mjs`; the mirror is `cp`'d. The layout also gains two code tables, `racks`
  (`1 inserts`, `3 console`, `255 notApplicable`) and `liveResponseRacks` (`0 inputFilters`,
  `2 inserts`, `4 console`), so the codes have one machine-readable source (the SDK and S1d can
  read them) and `check-abi-layout-v1.py`, the second implementation, pins them exactly and
  refuses a table that reallocates a retired code. Its self-test gains three mutations (a
  retired rack reallocated to console, a retired owner code returned, a pre-rename spectrum name)
  and catches 22. `prepared-control.js`, the direct oracle and the V8 harness read the codes from
  the layout.
- **JS host.** `validCommand`, subscriptions, observation addresses, map bindings and rows accept
  only `1`/`3` (and `255` for builtin commands), so a retired code is refused locally before the
  port, as an out-of-set code always was. `prepared-control.js` classifies an EQ ride at `1` or
  `3`; a retired code goes to the engine as an ordinary record and is refused there.
- **V8 harness and controls table.** `mixing_automation.rs` resolves each control's address
  against the session itself (`console[k]` entry or `inserts.effects[i]`) and emits `3` + slot
  index or `1` + insert index; the harness's `liveSlot` reads the same two shapes. The record's
  fields are unchanged (it never recorded the rack).
- **Live bypass exceptions** (P1 verdict L4, #1100), documented on `COMMAND_EFFECT_BYPASS` and
  the shipped `.d.ts` `EffectBypass`: a live un-bypass of a session-bypassed delay or multiband is
  admitted and renders nothing different (both keep a prepared bypass; neither can be a console
  slot).
- **Docs.** `docs/EFFECT_OBSERVATION_V1.md` (addressing), the record table in `lib.rs`, the
  `.d.ts` (rack codes, reasons, spectrum names, the solo tap prose's new tap spellings).

### Files outside the spec's authorized list, with the gate that needed each

- `crates/graph-compiler/src/lib.rs` (test module only), `crates/host-core/src/lib.rs` (re-export),
  `crates/host-core/src/limiter_linked_session.rs`, and host-core's `effect_live_controls`,
  `effect_observation` and `symmetry_witness` tests: they read the producer's removed
  `rack`/`effect_index` (clippy and `cargo test --workspace`). The new
  `crates/host-core/tests/live_addressing.rs` is gate 1's native half.
- `scripts/test-prepared-control.mjs` and `scripts/test-web-audioworklet.mjs`: the artifact-gates
  hermetic suite (`test-web-audioworklet.sh`) drives the changed JS with the old codes and names.
- `tools/parameter-metadata/tests/round_trip.rs`: a red-mutation comment named the deleted
  `counts[rack]` leg; it now names the mutation that still turns it red (verified).
- The protocol's catalog (`host-core` `control_provider`, S1a) names the same instances by string
  identity, `(track_id, rack, effect_id)`, with the session/BTLV rack codes (inserts `2`, console
  `5`). That is a different encoding from the browser record's byte on purpose: decision 12's rule
  applies per encoding (each keeps its inserts code and appends console), which closes S1a's
  verdict info item on the `5`-versus-`0/1/2` mismatch.

### Gates

| Gate | Evidence |
|---|---|
| 1. A live parameter change and a live bypass reach the right lane, pre, post and insert, native and browser; a planted section-split off-by-one turns it red | Native: `host-core/tests/live_addressing.rs`. On track `eq5` of an eight-track strip (console `pre_insert` [eq, pre-eq2], `post_insert` [post-eq], insert [ins-eq], every instance an EQ with its own frequency and a per-track gain; the console slots bank), a live bypass at `console(0)`, `console(1)`, `console(2)` and `insert(0)` renders bit for bit what the document renders with that instance's session `bypass` set, is audible, and differs per address; a live EQ ride at each address renders exactly what the same ride renders pushed to the channel found by `effect_id`, and differs per address; every channel's address reads back as `console[k]` / `inserts.effects[i]` on every track. Browser: `host-web` `a_browser_bypass_reaches_the_addressed_lane` (record rack `3`/`1`: exactly the addressed instance's queue is loaded, and the render equals the session-bypass document) and `a_browser_parameter_reaches_the_addressed_lane` (the prepared EQ path: configuration copy and companion at `3`/`1`, exactly one queue loaded, audible and distinct per address). |
| 1, mutations (each run once, then restored) | M1 `declared_live_addresses` gives `post_insert` base `0` (the post slot addressed as `pre_insert` slot 0): native 3/3 red, browser 2/2 red (boot refuses the colliding table). M2 inserts based after the console: native 3/3 red. M3 `lower` takes `console(k >= pre)` into `Simd1`: browser 3/3 red. M4 rack `3` decoded as the inserts: browser bypass and ride red. M5 companion rack resolved through the lowered codes: browser ride red. The `lower` unit table (`a_live_address_lowers_through_the_section_split`) names its own four. |
| 2. Rack `0`/`2`, and owner or observation records with a retired code, refused with typed reasons | `retired_rack_codes_are_refused_on_every_browser_record`: `effectParam`, `effectBypass`, `observeSubscribe` and `observeUnsubscribe` at `0` and `2` -> `RESULT_INVALID_ARGUMENT`/`unknownRack`, nothing admitted; the configuration copy -> `RESULT_INVALID_ARGUMENT`; a companion record at `0`/`2` -> `malformed`; the valid twins at `3` are admitted and the render equals the base. At the base, rack `0` effect `0` was this session's `eq`. `observation_records_address_console_slots_and_inserts` (ffi): the map and read rows carry `3` (console fixture) or `1` (insert fixture), and a selection at `0`, `2`, `4` or the unused rack is `RESULT_INVALID_ARGUMENT`. `live_response_owners_name_console_slots_inserts_and_input_filters` (ffi): owners in signal order are `input-filters` `0`, `eq` `4`, `ins-eq` `2`, `post-eq` `4`, and an owner patched to `1`, `3`, `5` or `255` makes the analysis `RESULT_INVALID_ARGUMENT`. JS: the worklet host refuses `0`/`2`/`4` locally before the port (`test-web-audioworklet.mjs`), and `prepared-control.js` never treats a retired code as an EQ owner (`test-prepared-control.mjs`). Mutations: M5 decode `0`/`2` -> the retired-codes test red; encode console as `0`, or decode `0`/`2`, in the observation helpers -> the observation test red; pass the native owner rack through, or accept any owner code -> the owner test red; classify `rack <= 2` in `prepared-control.js` -> its test red. |
| 3. `check-abi-layout-v1.py` (layout and self-test); `check-capi-abi.sh` | Pass; the self-test catches 22 mutations (3 new). `check-capi-abi.sh` passes: the C ABI carries no rack addressing and did not move. |
| 4. Raw-export qualification in the three browsers | `npm run qualify -- --artifacts target/ci/qualification-artifacts --browser <b> --self-test-mutations`, without `--sdk-root`, each with a private PulseAudio null sink: chromium 151.0.7922.34, firefox 153.0 and webkit 26.5, "all qualification gates passed", the 7-file artifact set pinned. `--check-matrix` cannot pass without `--sdk-root`: the checked rows carry `sdkResponse`, which only an SDK run adds (`run.mjs` `normalizedRow`); on chromium every gate passed and only that row comparison refused. |
| 5. The benchmarks still run (untimed, committed clean tree `318df3fc`) | `operator/preflight-console-benchmark.sh --step s1c-1096-scratch-preflight`: PASS, 0 workload launches, nothing written. `run-web-mixing-automation-benchmark.sh prepare` then `preflight` in an empty scratch directory: PASS. |
| 6. Console digests unchanged (PR evidence) | `console-workload` `gain_pan_profile digests` (64 blocks), release, base `105064a7` (a `git archive` build) against `318df3fc`: all 21 session rows identical, `sixty_four_track_app_shape` `c740fa2dd904` included. |
| V8 harness, class A | `prepare` + `preflight` at base `105064a7` (a detached scratch worktree, since removed) and at `318df3fc`: all 7 arm digests identical (`quiet` `014e5f5b190e`, `automated` `e7025b5cbcea`, `automated_limiter_only` `8db18991bed9`, ...) and both documents identical (console `d913ad961d2d`, app `3dd8b2fff4b9`). `controls.json` differs only in the address: the EQs and compressors `0` -> `3` at the same index, the limiters `2`/`0` -> `3`/`2` (the `post_insert` slot at console slot 2). |
| Live bypass exceptions | `a_live_bypass_cannot_lift_the_delay_or_multiband_session_bypass`: one track, inserts [delay, multiband, compressor], all session-bypassed; a live un-bypass is admitted (`none`) on each; the delay's and the multiband's renders equal the base, the compressor's does not. Dropping the delay, or the multiband, from `lowers_session_bypass`'s exclusions turns it red (both run). |
| `cargo fmt --all --check`; clippy `--workspace --all-targets --all-features -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | Pass |
| `cargo test --locked --workspace --all-features --no-fail-fast` (dev) | 294 result lines, 2,113 passed, 0 failed, 40 ignored |
| Wasm `simd128` | `cargo check --target wasm32-unknown-unknown` with `+simd128` for `target-smoke`, `protocol`, `dsp-reference` and `conformance`; the cfg probe; `check-protocol-wasm-parity.sh`: `ok (simd128)` |
| AArch64 iOS and Android | `check-cross-targets.sh`: PASS (the known #1018 memset rows are expected failures) |
| `scripts/run-wasm-gates.sh` (full) | `ok (native + wasm simd128 + V8 EQ loops)` |
| Artifact job | `build-web-audioworklet.sh`: `miso-engine-v1-audio-worklet.simd128.wasm` `e7f2ad317b37187eaa77c25618003e3521ee4bb68376bef9c4b75ac85cdc80be`, 3,283,569 bytes; closure `17d297ed6178dd5d50e853175614ff2b45c930913e6cb3644a904cacc294fbc7`. The V8 `prepare` built the same bytes. Not the release pin, as expected; the batch boundary re-pins it. |
| Artifact gates | `check-web-audioworklet.sh --without-metadata-regeneration` and `check-browser-expected-resources.py` (browser-correctness digests unchanged against the built module), `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`, the V8 spill gate and its self-test: pass |
| `test-console-benchmark.sh`; `check-ci-path-routing.py` and `test-ci-path-routing.py` | Pass |
| Lint job, every step (31), `check-dsp-research.sh`, the gate self-tests (5) | Pass, run step by step from `qualification.yml`: workspace, session, env-vocabulary, bench, test-support-ci, scalar-oracle self-test, script reachability, console benchmark fixture, builtins fixtures, bench preconditions, host-core, protocol-control, realtime and its trace validator, lane, unfused seal, rack, effect-runtime, conformance boundaries, parametric-EQ render contract, release-shape self-test, npm publish modes, stem store, stem identity, the three sub-v3 lane-guard probes; `test-env-vocabulary.sh`, `test-conformance-boundaries.sh`, `test-console-benchmark.sh`, `check-sdk-deletions.py --self-test`, `test-dsp-research.sh` |
| SDK: `check-sdk-generated.sh` (with and without the artifact directory), `check-sdk-deletions.py` | Pass |
| SDK: `check-sdk-types.sh`, `sdk-package.sh check`, `check-sdk-headless.sh` | Fail, S1d's. `check-sdk-types` and `sdk-package`'s `tsc` now stop on exactly three errors, all the SDK passing the pre-rename spectrum names (`sdk/src/browser/engine.ts:687`, `:738`, `host-mirror.ts:126`) into the renamed `MisoSpectrumTarget` (S1a's rebase evidence records `check-sdk-types` passing). `check-sdk-headless` fails 86 of 284, the same count as after S1a (the old-schema documents). The SDK still hard-codes `{ simd1: 0, dynamic: 1, simd2: 2 }` and the old owner/observation codes (`core/live-controls.ts`, `observation.ts`, `live-response.ts`) and the old spectrum names (`core/spectrum.ts`); S1d moves them to the `racks`/`liveResponseRacks`/`spectrumTargets` tables. |

### For S1d (#1097) and the app

- The browser codes changed in lockstep: an effect is `1` + insert index or `3` + console slot
  index; `0`/`2` are refused; live-response owners are `0`/`2`/`4`; spectrum targets are
  `trackPostInput`/`trackPostPan`/`output`. The layout JSON's `racks` and `liveResponseRacks`
  tables are the vocabulary. S1d's app handoff should carry the map for an app that drives the
  raw worklet host.
- The SDK-driven qualification entry points (`run.mjs`'s `sdk-*` gates and
  `sdk-response-entry.ts`) still expect `trackPostInputBuiltins`/`trackPostMatrix`.
