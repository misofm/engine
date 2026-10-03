# #1214 *Drive submix strips from the SDK live controls*: Sol verdict, attempt 1

- Reviewed: `git diff c13ac5e1d 4df439f2c` (6 files, +535/-65). Branch `codex/batch-submix-k2`,
  worktree `/home/bl/misofm/wt-submix-k2`.
- Binding: `AGENTS.md`, and `.github/ISSUE_SPECS/1214-drive-submix-strips-from-the-sdk-live-controls.md`
  with its Attempt 1 record.
- How I ran it:
  - I did not modify the worktree, the branch or GitHub. A read-only `gh issue view 1214` shows it
    OPEN, and the title matches the spec's H1.
  - I exported `4df439f2c` to `/tmp/claude-1002/v1214/src`, copied `sdk/node_modules`, and used my
    own `TMPDIR` and `CARGO_TARGET_DIR`. I ran mutations in place and restored them, then compared
    every touched file byte for byte against `git show 4df439f2c:<path>`: all identical.
  - I deleted the scratch tree afterwards.

## Verdict: PASS

D1-D4 are implemented as frozen, and every gate in the spec's gates 1-6 passes. Every test-value
claim I re-ran reproduces.

- D2 is a real shared implementation, not a copy. `StripEdits` holds every strip method.
  `TrackEdits` adds only `solo`, and `SubmixEdits` adds nothing.
- The submix order is correct by construction. `SessionMap.submixes` is identical on both SDK
  paths and in host-web's command addressing:
  - The headless path (`boundary.ts:478-483`) reads `miso_engine_web_v1_live_control_submix_id(i)`
    in index order.
  - The worklet (`miso-engine-v1-audio-worklet.js:618-631`) reads the same export in the same
    order.
  - That export returns `ready.submixes[i]`, which is `handles.strips[track_count..]`
    (`host-web/src/lib.rs:5992-5996`). It is the same list that `strip_index` and the command
    lowering use to resolve `T + j`.

There are no BLOCKER or MAJOR findings. There are two MINOR findings and four NITs. MINOR-1 is a
gap #1214 inherits, not something it introduced. The #1210 verdict deferred it to this slice, and
K2 closes here, so the root should decide on it before the K2 push.

## Gates (all run from `4df439f2c`, x86_64 AVX2)

| Gate | Result |
|---|---|
| `build-web-audioworklet.sh --named-twin B A` | rc 0. Shipped `7d6c0a8b57dd51361d5ed20f350cddc01871a941a9d025d6d2e4a61af750f90b` (2 695 834 B), equal to the record. Named twin `66eb0bde...5699`. |
| 6: `check-sdk-types.sh` | rc 0 |
| 6: `check-sdk-headless.sh A` | rc 0, 346 pass, 0 fail (equal to the record) |
| 6: `sdk-package.sh check A` | rc 0, "SDK publishable-tarball gate passed" (see note below) |
| `check-sdk-generated.sh A` | rc 0 |
| `node scripts/test-web-audioworklet.mjs` (baseline for mutation M5) | rc 0 |

- Note on `sdk-package.sh`: my first run raced the still-running artifact build and was missing the
  metadata JSONs. That was my sequencing error, not a defect. The rerun after the build exited
  passed.
- Not run: gate 7's Rust workspace, clippy, doc, C ABI and aarch64 legs. The diff is SDK and docs
  only, the artifact is byte-identical, and the root runs gate 7 once at the K2 head.

## Mutations (each applied alone, run red or green, restored)

| # | Mutation | Result |
|---|---|---|
| M1 | `LiveControlEdits` assigns submix indices over `[...map.submixes].reverse()` | red: gates 1, 2, 3 |
| M2 | encode `j`, not `T + j` | red: gates 1, 2, 3 and browser gate 4 |
| M3 | browser `createBrowserLiveControls` map: `[...remoteMap.submixes].reverse()` | **green across the whole SDK suite (346/346)**. See MINOR-2. |
| M5 | worklet builds `submixIds` with `unshift` (reversed) | **green** in `test-web-audioworklet.mjs`. See MINOR-2 (this is #1210 MINOR-1, still open). |
| M6 | a `SubmixEdits` reads its instances from `layout.tracks` | red: gate 3 only |
| M7 | `layoutOf` drops the submix consistency comparison | red: gate 3 only |
| M8 | `layoutOf` walks `[]` for submixes | red: gate 3 only |
| M9 | `track(id)` falls back to `#submixes` | red: gate 1 only |
| M10 | `submix(id)` falls back to `#tracks` | red: gate 1 only |
| M11 | `SubmixEdits` constructs with kind `"track"` | red: gate 3 only |
| M12 | headless `boundary.ts` returns `submixes` reversed | red: gates 1, 2, 3 |
| T1 | add `solo()` to `SubmixEdits` | `check-sdk-types.sh` red, 4 TS errors |
| T2 | `submix()` returns and constructs `TrackEdits` | red, 3 TS errors (`live-controls-types.ts:132,145,147`) |

## Test value (one sentence per new test)

- **"a submix encodes its strip index T + j and IDs resolve only in their own list"** (gate 1). It
  turns red if submix indices start at 0 or the order is reversed (M1, M2, M12). It is the only
  test that turns red when `track()` or `submix()` falls back to the other list (M9, M10). It also
  proves the shipped module admits all nine builtin edits at a bus index.
- **"a live bus fader equals the bus booted at that fader from the edit's block on"** (gate 2). It
  turns red if the record reaches a track or the other bus in the shipped module (M1, M2, M12).
  The record's left-lane-only mutation is caught by this test alone.
- **"console slots and inserts resolve by ID on a bus of an SDK-built session"** (gate 3). It is the
  only test that turns red if `layoutOf` ignores submixes (M8), skips their consistency check (M7),
  resolves a bus chain from the tracks map (M6), or treats a submix as a track (M11).
- **"a browser engine's live controls address a submix by its strip index"** (gate 4). It turns red
  if the browser map drops `submixes` or uses base 0 (M2). It does not pin order (M3); see MINOR-2.
- **`live-controls-types.ts` gate 5.** It turns red if a bus gains `solo` (T1) or `submix()` widens
  to `TrackEdits` (T2).

No existing test was edited or superseded, and nothing greps source or pins a digest.

## Findings

### MINOR-1: `subscribeObservations` on a bus tap still throws, and the #1210 verdict deferred it to this slice

`ObservationSubscriptionOwner.#edit` (`sdk/src/core/observation-subscriptions.ts:841`) builds every
managed arm and disarm edit with `liveControls.edit.track(entry.selection.trackId)`.

- D1 correctly keeps submix IDs out of `track()`. So a managed subscription to a bus binding
  fails, even though `observationMap()` lists that binding and `readObservations()` reads it.
- I probed it headless on the shipped module, with #1210's `busObservationDocument()` and
  `observationTaps: 2`:
  - `engine.subscribeObservations({ selections: [{ trackId: "bus", rack, effectSlotId: "bus-comp", tapId, channels: "both" }], windowBlocks: 1 })`
    throws `MisoUsageError: the compiled session has no track 'bus'; expected one of t`.
  - The same arm through `controls.edit.submix("bus").insert(0, "miso.compressor").observe("Gain Reduction", true, 1)`
    is admitted (`ok`).
- This is not a regression: before #1214, `track("bus")` threw too. But the #1210 verdict says
  "Subscribing to a bus tap is #1214's `edit.submix()`", and K2 closes with this slice. So the
  browser and headless `subscribeObservations` ship in K2 refusing every bus tap, with a misleading
  message.
- `APP-LIVE.md`'s "An effect observation on a bus ... names the submix ID" is true for the raw
  `observe()` edit and for `readObservations()`. An app reading it would still expect the managed
  API to work.
- Fix, which needs a spec amendment because `observation-subscriptions.ts` is not an authorized
  path, or else a successor issue:
  - In `#edit`, resolve the strip, not the track. For example, add a module-internal
    `LiveControlEdits.strip(id): StripEdits` that tries `#tracks`, then `#submixes`, and call
    `.effect(...)` on it.
  - Add a headless eval: subscribe to `bus`/`bus-comp` Gain Reduction, render two blocks, `pump()`,
    and require `readLatest()[0]` to be `ready` with `trackId === "bus"`.
- Until either lands, add one line under "Per-submix meters" in `APP-LIVE.md`: managed
  `subscribeObservations` does not yet accept a bus tap; use `observe()` plus `readObservations()`.

### MINOR-2: nothing pins the submix order on the browser path

- Gate 4's stub host replies with one submix (`["bus"]`). A browser adapter that reorders
  `remoteMap.submixes` therefore passes the entire SDK suite (M3). A worklet that enumerates the
  export in another order passes `test-web-audioworklet.mjs` (M5).
- A reordering cannot happen in the code as written. But the browser is where the app runs, and
  #1214 makes the order load-bearing there: a wrong order silently edits the other bus. That is the
  spec's own "Index base" hazard, moved to the order.
- Fix (both are cheap):
  1. In gate 4 (an authorized path), reply `submixes: ["aaa", "bus"]`, submit
     `edit.submix("bus").faderDb(-6)`, and require index word 2. Also require
     `edit.submix("aaa")` to give index word 1.
  2. Land #1210 MINOR-1 (the K2 ledger already lists it): give `createFakeExports` two submix IDs
     and assert their posted order at `test-web-audioworklet.mjs:3181`. #1213 attempt 2 already
     edits that harness, so it is the natural home.

### NIT-1: a `TrackEdits` is assignable to `SubmixEdits`

`SubmixEdits` and `StripEdits` are structurally identical, and `TrackEdits` is a structural
subtype. So `const bus: SubmixEdits = controls.edit.track("t")` compiles (probed with `tsc`). A
helper typed `(bus: SubmixEdits)` therefore silently accepts a track builder. The runtime ID
separation (D1) is intact, so this affects typing only.

Fix: brand `SubmixEdits`, for example with `readonly #submix = true;`, and add an
`@ts-expect-error` that assigns `edit.track(..)` to `SubmixEdits` in `live-controls-types.ts`.

### NIT-2: `stripIndex` is a visible own property at runtime

`TrackEdits` used to keep `#trackIndex` private. It now exposes `stripIndex` as an own property
(`protected` is compile-time only), so `Object.keys(edit.track("t"))` now returns `["stripIndex"]`.
It is harmless. If you want true privacy back, use a `#` field read through a module-private
accessor.

### NIT-3: stale track-only prose

- `LiveControlRack`'s doc (`live-controls.ts:17-23`) still says "every track's `console` array",
  "the track's insert" and `TrackEdits.console()`.
- The `EngineLiveControls` constructor doc (`:840`) still names only `edit.track(id)`.
- `sdk/README.md:196` documents only `edit.track`. The README is not an authorized path, so the
  root can take this as a follow-up.

### NIT-4: `APP-LIVE.md` precision

- **Master designation.** `masterTrackPlusOne != 0` is refused at boot unless
  `observationTaps > 0` (`host-web/src/lib.rs:5663-5665`, `host-mirror.ts:64`). `masterGrDb` is
  `null` until the designated strip publishes an observation (`lib.rs:3512-3519`). Both facts are
  already true for tracks, but an app that designates a bus will hit them first.
- **Policy key.** A TypeScript app that passes `policy: { console: {...} }` as an object literal
  gets an excess-property type error before it ever boots without live controls. The runtime
  sentence is accurate, but the compile-time symptom is the one the app will see.

## Other claims I checked and found accurate

- The handoff's remaining claims check out:
  - the reason `notSoloable` (12) with result `invalidArgument`, end to end (`capability-evals.mjs:452`);
  - the names `MeterUpdate.submixes` and `TrackMeter { peakLeft, peakRight, gainReductionDb }`;
  - the frame fields `submixCount`, `submixPeaks` and `submixGrDb`;
  - the `miso.sessionmap.v1` reply;
  - `masterTrackPlusOne` as a strip index plus one, with an out-of-range value refused at boot
    (`host-core/src/prepare.rs:1269-1272`);
  - `ABI_VERSION` unchanged across K2, where the only struct growth is the meter header's appended
    `submixCount`;
  - the app still passing `console:` (`misofm/app` `src/lib/mixer/engine/boot.ts:227`,
    `open-session.ts:303`), and that key being ignored at runtime by `sdk/src/browser/policy.ts`.
- The new public `abstract class StripEdits` is acceptable API surface:
  - It has to be exported, because an exported class cannot extend an unexported one under
    declaration emit.
  - Its constructor is `protected`.
  - The emitted `dist/core/live-controls.d.ts` is clean.
  - `barrel-surface.ts` pins no exhaustive name list, and `check-sdk-deletions.py` scans only
    numeric ABI offsets. The package, types and generated gates all pass.
- `TrackEdits`' public constructor signature is unchanged.
- Error messages now name `submix 'x'`. A `TrackEdits` built directly with no ID still reads
  `track 'undefined'`, exactly as before.
