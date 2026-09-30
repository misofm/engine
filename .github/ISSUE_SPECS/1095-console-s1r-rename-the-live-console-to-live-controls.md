# Rename the live console to live controls

Slice S1r of *Console strip: session-level console effects with per-track inserts* (owner
decision 12, `docs/rulings/engine-footprint-2026-09-29.md`, "Naming"; Sol's M6 and amendment 13 in
`docs/handoffs/console-strip-2026-09-29/VERIFY.md`, commit `03aceb94`).

## Problem

The session key is `console` (decision 12), but "console" already names the live-control
attachment: the channel through which a host changes a prepared plan's fader, mute, pan, solo,
effect parameters and bypass while it renders, and reads its meters and observations. Once S1a
lands, "console" would mean two things in the same code, and S1c and S1d, which edit that code,
would inherit the ambiguity.

The owner ruled that the engine's internal live-console names are renamed, for example to "live
controls". The question the owner answered listed the fader, mute and pan lanes, the
`miso_engine_web_v1_console_track_*` exports and the benchmark names. So the owner decided to
rename the internal names (checkpoint 1) and the two exports (checkpoint 2). The console benchmark
and fixture names stay, because they describe console sessions.

The four boot-option words and the SDK's public live-console API are renamed with the exports, so
that one vocabulary crosses the boundary. That is root's application of the owner's decision, not a
separate owner decision (decision 12, "Naming").

## Why this is its own slice

R0 was asked to fold the rename into S1c or give it its own slice, depending on its size. It gets
its own slice. The attachment's vocabulary reaches about 80 code, script and SDK files (about 780
lines) and about 8 live docs, and it includes:
- two sealed wasm exports;
- four pinned boot-option words;
- the SDK's public live-console API.

That is more than S1c's half day on its own. Folding it in would put a class-A mechanical rename
and a semantic addressing change under one verdict. Landing it first, before S1a, lets its SDK
gates run against an unchanged schema. It also means S1a, S1c and S1d edit code in which "console"
has one meaning.

## Smallest closable slice

Work in two checkpoints.

### Checkpoint 1: internal Rust names (class A, nothing pinned moves)

Rename every identifier whose "console" means the attachment. Types take `LiveControl`, and
functions and fields take `live_controls` or `live_control`, whichever reads as the attachment.
The minimum set (M6) and its map:

| Today | Renamed |
|---|---|
| `HostConsoleRequest` (`crates/host-core/src/prepare.rs:279`) | `HostLiveControlRequest` |
| `HostConsoleHandles` (`prepare.rs:342`) | `HostLiveControlHandles` |
| `ConsoleEffectBankStage` (`crates/rack/src/lib.rs:911`) | `LiveControlEffectBankStage` |

The same rule covers the rest of the attachment's names, among them:
- `prepare_host_*_with_console*`, `prepare_*session_builtins_with_console*` and
  `attach_effect_console`;
- `ConsoleInputProcessor`, `ConsoleMatrixProcessor` and `ConsoleFaderProcessor`;
- `ConsoleSoloState` and `ConsoleMuteDelta`;
- `LiveConsoleRecord` and the graph runtime's `ConsoleEffect`;
- host-web's `console_tracks`, `console_attached`, `copy_console_track_id`, `console_solo` and
  `console_request`.

**Keep** the names that describe console sessions or the console benchmark:
- `tools/console-workload`, `tools/bench/src/console.rs` and the `sixty_four_track_console*` rows;
- the `console-sixty-four-track*` fixtures and their derive and check scripts;
- test helpers that build or render those sessions, such as `intended_console` and
  `compile_console_model_*`.

Record the per-identifier classification (rename or keep, with a one-line reason) in the PR
description, not in a committed ledger. Where a name is ambiguous, read what it constructs.

### Checkpoint 2: the boundary (sealed names; in-place V1 amendment)

| Today | Renamed |
|---|---|
| `miso_engine_web_v1_console_track_count` | `miso_engine_web_v1_live_control_track_count` |
| `miso_engine_web_v1_console_track_id` | `miso_engine_web_v1_live_control_track_id` |
| boot words `consoleCommandQueueRecords`, `consoleMeterBlocks`, `consoleObservationTaps`, `consoleMasterTrackPlusOne` (offsets 32, 40, 48, 56) | `liveControlCommandQueueRecords`, `liveControlMeterBlocks`, `liveControlObservationTaps`, `liveControlMasterTrackPlusOne` (same offsets and types) |
| SDK `sdk/src/core/console.ts`, `EngineConsole`, `ConsoleRack` | `sdk/src/core/live-controls.ts`, `EngineLiveControls`, `LiveControlRack` |
| SDK `sdk/src/browser/console.ts`, `createBrowserConsole`, `browser.console()` | `sdk/src/browser/live-controls.ts`, `createBrowserLiveControls`, `browser.liveControls()` |

How decision 12 treats the exports (the `_v1` wire identities stay):
- They are class-2 contract identity (`docs/rulings/de-versioning-inventory.md`, "wasm export
  symbol"), pinned by `scripts/check-abi-layout-v1.py` and `sdk/assets/miso-engine-v1-abi-layout.json`.
- The version-suffix rule governs only their `_v1`, which stays.
- The stem rename is an in-place V1 amendment with no `ABI_VERSION` bump. The app and SDK update in
  lockstep.
- A retired spelling is never exported again, for any meaning. There is no alias.

Regenerate the layout JSON and `sdk/src/generated/abi.ts` from `tools/parameter-metadata`, and
update the AudioWorklet host (`hosts/host-web/web/*.js`, `.d.ts`), the qualification harness,
`scripts/check-browser-expected-resources.py` and the SDK tests. The shipped artifact's bytes
change; the artifact-identity job reports it, and the release pin moves at release
(`docs/RELEASE.md`).

Update the live docs that name the old spellings. Historical handoffs, rulings and closed specs
stay as written. Update the open specs that name them (#1053, #1054, #763).

Write an app handoff note: the old-to-new map for every public SDK name and boot word.

Authorized paths: every file that names the attachment's vocabulary, the generated layout and
bindings, the host-web web files, the listed scripts and open specs, and this spec. No behaviour
change anywhere.

## Owner decisions that bind this slice

- Decision 12's "Naming": the owner renamed the internal names and the exports, and the benchmark
  and fixture names stay. The boot words and the public SDK API follow as root's application.
- Decision 12's "Wire identity": the prelaunch identity stays V1, `_v1` stays in every export, and
  the app updates in lockstep.

## Dependencies

None beyond decision 12. It is the first slice of batch C3, and it must merge **before** *Add the
session console and per-track inserts to the session schema* (S1a, #1093). After S1a, the SDK
suites, `check-sdk-generated.sh` and the SDK-driven browser qualification are red until S1d, so
gate 3 below could not pass. It therefore also precedes *Address console slots and inserts in live
control* (S1c, #1096) and *Ship the session console and inserts in the SDK* (S1d, #1097).

## Objective gates

1. Class A. PR evidence: console digests and the browser qualification outputs are unchanged
   against the pre-change base.
2. `python3 -B scripts/check-abi-layout-v1.py` passes with the renamed exports and words. Its exact
   export list lacks the retired spellings, so an alias or a rename-back fails it.
3. `bash scripts/check-sdk-generated.sh`, `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-web-audioworklet.sh` and `node scripts/test-web-audioworklet.mjs` pass, as do
   the SDK tests and the three browsers in CI mode.
4. `cargo test --workspace`, `bash scripts/check-workspace-policy.sh` and
   `python3 -B scripts/check-script-reachability.py` pass.
5. The PR lists every renamed and every kept identifier with its reason, and the app handoff note
   exists.

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

Terra, 2026-09-30. Branch `codex/1095-rename-live-controls` from `51a1514a` (the head of batch C2,
`codex/batch-console-2`). Checkpoint 1 is `acbb362d` (internal Rust names), checkpoint 2 is
`b07c32f0` (exports, boot words, SDK API), then this record. "Base" below is `51a1514a`.

### What landed

- **Checkpoint 1** (68 files). Every internal identifier whose "console" meant the attachment:
  `HostLiveControlRequest`/`HostLiveControlHandles`, `LiveControlEffectBankStage`,
  `LiveControl{Input,Matrix,Fader}Processor`, `LiveControlSoloState`/`LiveControlMuteDelta`,
  `LiveControlRecord`, `graph`'s `LiveControlEffect`, `prepare_host_{session,runtime}_with_live_controls*`,
  `prepare_{selected_,}session_builtins_with_live_controls*`, `attach_effect_live_controls`, host-web's
  `live_control_tracks`/`live_controls_attached`/`copy_live_control_track_id`/`live_control_solo`/
  `live_control_request`, the estimate's `live_control_{effect,stage}_*` terms, the bindings of
  those types, 21 test names, the test helpers that build a host or plan with live controls, and
  three test files (`effect_live_controls.rs`, `input_liveness_live_controls.rs`,
  `live_control_bank.rs`). The prose that named the attachment follows, and so do the MUTATIONS
  ledgers that cite a renamed test.
- **Checkpoint 2** (74 files). The two exports, the four boot words (offsets and types unchanged)
  with `WebBootOptions`' fields and `live_control_defaults()`, and the SDK's public live-control API
  (`core/live-controls.ts`, `browser/live-controls.ts`, `EngineLiveControls`, `LiveControlEdits`,
  `LiveControlRack`, `LiveControlChannel`, `LiveControlSubmit`, `LiveControlBeforeSubmit`,
  `LiveControlWriter`, `createBrowserLiveControls`, `engine.liveControls()`, the `liveControls`
  options key). Regenerated from `tools/parameter-metadata`: `sdk/assets/miso-engine-v1-abi-layout.json`,
  `sdk/src/generated/abi.ts`, `scripts/fixtures/abi-layout-v1-self-test.json`. The worklet host, its
  `.d.ts` and the SDK mirror, the qualification harness, the V8 harness, the open specs #1053, #1054
  and #763, and the app handoff note
  (`docs/handoffs/console-strip-2026-09-29/APP-LIVE-CONTROLS.md`: every public SDK name, boot word,
  export and diagnostic code, old to new).
- **Gate-pinned spellings moved with their names, in the same commit:**
  `scripts/check-scalar-oracle-absent.py` (checkpoint 1: its FORBIDDEN roster names the three
  per-node live strip owners; the self-test symbols are the #1059 captures with the length-prefixed
  type name re-spelled), `scripts/check-abi-layout-v1.py`, `scripts/check-web-audioworklet.sh`,
  `scripts/check-session-map-shape.py`, `scripts/check-browser-expected-resources.py`.
- **Judgment calls for the verdict.**
  - The internal diagnostic codes are renamed (`web.options.live_controls`,
    `web.live_controls.{config,input_filter,effects,observation}`, `web.internal.live_controls`,
    `host.observation.live_controls`). The spec's list does not name them. They are not pinned by
    any gate or document, and `web.live_controls.effects` would otherwise read as the S1a session
    console's effects. They are in the handoff note.
  - The browser qualification row keeps its name and output schema (`result.console`,
    `stall.console{CommandResult,MeterLeaseResult,MeterFrames}`, `stall-console-load`,
    `runConsoleQualification`, `CONSOLE_*`), so gate 1's outputs stay comparable with the base and
    `--check-matrix`'s `results.json`. Its boot words, SDK calls and prose are renamed.

### Gates

| Gate | Evidence | Result |
|---|---|---|
| 1. Class A: console digests | `console-workload`'s ignored `digests` harness in release (`gain_pan_profile`), base tree against head, same target dir: **22 of 22 rows identical**. `cargo test --locked --release -p audit -p bench -p console-workload`: 114 passed (the pinned console digests included). | identical |
| 1. Class A: browser | `check-browser-expected-resources.py --artifacts`: the committed `expected.json` digests and exact rows agree with the new module (and its self-test, 32 red mutations). Browser qualification in CI mode (`run.mjs --check-matrix --self-test-mutations`, pulseaudio null sink) on chromium 151.0.7922.34, firefox 153.0 and webkit 26.5: every gate passes and every row equals the committed `results.json`. | unchanged |
| 1. Class A: V8 harness | `run-web-mixing-automation-benchmark.sh prepare` + `preflight` at `b07c32f0`: all seven preflight digests equal the base record's (`artifacts/steps/console-strip-base/web-mixing-automation.jsonl`, `preflight_output_sha256`: quiet `014e5f5b…`, automated `e7025b5c…`, EQ `2540aff4…`, compressor `c29d12a7…`, limiter `8db18991…`). | identical |
| 1. Artifact | Shipped module `f767076a03548350a35dc6d758716c328f108de59a40cb31442ace59435a09e8` (3,270,838 B) at base -> `9003bc7d1d75f975b3c6ee8b1a27beb4e76f388e0ffd5e1918e23cc1ed6ca01f` (3,271,342 B, +504 B: export names, symbol names, diagnostic strings). The twin build in `run-wasm-gates.sh` and the V8 harness's `--module-only` build reproduce it. Not re-pinned; the release pin moves at release. | changed, reported |
| 2. ABI layout | `check-abi-layout-v1.py sdk/assets/miso-engine-v1-abi-layout.json` passes; `--self-test` 19 mutations. A copy with `miso_engine_web_v1_console_track_count` appended is refused ("116 module functions, not 117"). | green |
| 3. SDK and worklet | `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh` (284 of 284), `sdk-package.sh check` (tarball gate), `check-web-audioworklet.sh`, `test-web-audioworklet.sh` (runs `test-web-audioworklet.mjs`), `check-scalar-oracle-absent.py --wasm`, `run-wasm-gates.sh` (native + simd128 + V8 spill). The three browsers are row 1. | green |
| 4. Workspace | `cargo test --locked --workspace --no-fail-fast` at each checkpoint: 291 binaries, 0 failed (2,076 passed at `b07c32f0`). `check-workspace-policy.sh`, `check-script-reachability.py`. | green |
| 4. Lint job | `cargo fmt --all --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; and all 48 script steps of the lint job, including `test-ci-path-routing.py` and `check-ci-path-routing.py`. | green |
| 4. Targets | `check-cross-targets.sh` (AArch64 iOS and Android product crates checked and linted, iOS memset ratchet at its #1018 expected counts, wasm simd128, armv7 and scalar wasm refused); the `simd128` checks of `target-smoke`, `protocol`, `dsp-reference` and `conformance`; `check-protocol-wasm-parity.sh`. | green |
| 4. Benchmarks | `test-console-benchmark.sh` (validators, no timed invocation). | green |
| 5. Classification | Below, for the PR description; the handoff note has the app's map. | done |

### Renamed and kept (for the PR description)

Renamed: every name above, with its prose, plus these test names:
`disabled_live_control_input_has_infinite_tail_but_plain_input_does_not`,
`the_per_node_live_control_input_processor_drains_and_folds_its_witness`,
`live_control_requests_are_validated_sealed_and_charged_per_track`,
`a_live_control_lane_starts_from_the_session_bypass`,
`the_observation_lane_costs_one_nullable_pointer_in_the_live_control_effect`,
`a_live_control_parameter_command_applies_at_the_next_block_boundary`,
`idle_live_controls_change_no_rendered_bit`,
`the_live_control_effect_drops_nothing_within_its_prepared_capacity`,
`live_controls_attach_bounded_control_and_meter_halves_in_canonical_track_order`,
`no_live_control_request_attaches_nothing_and_charges_nothing`,
`live_control_and_meter_preparation_keeps_spectrum_in_one_transaction`,
`uncommanded_live_controls_render_the_live_control_free_bytes`,
`the_scalar_live_control_effect_arm_maintains_its_own_live_terms`,
`seam_side_live_control_traffic_leaves_every_lane_eligible`,
`a_live_control_free_bypass_binds_at_any_automation_capacity`,
`a_bypass_or_live_controls_are_charged_at_least_what_they_retain`,
`decoded_command_resource_is_exact_for_live_control_modes_without_effects_or_meters`,
`a_live_control_free_host_refuses_every_live_kind_as_unsupported`,
`live_control_track_id_longer_than_every_source_id_boots_and_round_trips`,
`a_refused_solo_submission_leaves_the_live_controls_untouched`,
`live_controls_that_never_solo_render_what_they_always_did`.

Kept, by class (the leftover grep, `git grep -i console` outside history, is all of these):
- **Console sessions, benchmark and fixtures** (the owner's decision): `tools/console-workload`,
  `tools/bench/src/console.rs`, the `sixty_four_track_console*` rows and `SixtyFourTrackConsole*`,
  the `console-sixty-four-track*` and `parametric-eq-bank-console` fixtures and their scripts, the
  console benchmark scripts and records (including the V8 record key
  `console_command_queue_records`, which committed records carry), and the test helpers that build
  or render those sessions (`intended_console`, `compile_console_model_*`, `render_console_*`,
  `scalar_console_registry`, `console_track_*_binding`, `ArmedConsoleRender`, `bypassed_console`,
  `representative_console`, the bypass tests' `console(tracks, masks)`, the `Console` structs that
  hold a prepared console session in `limiter_linked_session.rs`, `effect_live_controls.rs` and
  `symmetry_witness.rs`, `the_hot_console_*`, `console_run`, the randomized console generators).
- **The session console** (decision 12): "console strip", console slots, `console pre-insert`,
  AGENTS.md's chain line.
- **Real mixing consoles**: "physical console", "console-correct", "a live mixing console",
  `dsp-research`.
- **JavaScript's `console`**: `console.log`, `page.on("console")`, and the worklet's
  `console.`-policy mutations.
- **The browser qualification row and its fixture identities** (output schema, above):
  `console-session.json`, `console-source`, `web-browser-console`.
- **History**: `docs/handoffs`, `docs/rulings`, `docs/audits`, `docs/derivations`, `artifacts/`, and
  closed or batch-C2 specs. #1053's title stays, since it is the GitHub issue's title.
- **External**: `misofm/engine-web-adapter/src/console.ts` (a cited source baseline).
