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
