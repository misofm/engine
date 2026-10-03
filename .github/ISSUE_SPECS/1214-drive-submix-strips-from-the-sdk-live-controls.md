# Drive submix strips from the SDK live controls

Slice 17 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K2. K2 closes here, and the root pushes K2 once after this slice's verdict.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

An app or agent using the TypeScript SDK addresses a bus by its ID:

```ts
const controls = engine.liveControls();
await controls.submit(
  controls.edit.submix("drums").faderDb(-6),
  controls.edit.submix("verb").mute(true),
  controls.edit.submix("drums").insert("glue", "miso.compressor").parameter("threshold", -18),
);
```

It moves a bus's fader, mute, pan or matrix, trim, polarity, input filters, console slots and inserts
with the same ergonomics, smoothing and refusals as `edit.track(id)`, in the headless engine and in
the browser. The engine side accepts strip indices since *Address submix strips in browser live
commands* (#1213); this slice gives the SDK a name for them and tells the app team how to move.

## Context (verified on `fe8ac679`)

After *Name submix strips in the browser session map and the SDK measurement* (#1210), `SessionMap` (headless
`sdk/src/core/boundary.ts:250-256` and the browser path's `sdk/src/browser/live-controls.ts:48-52`)
carries `submixes: readonly string[]` in canonical order. After *Address submix strips in browser live
commands*, the record's index word is a strip index (`T + j` addresses submix `j`), and kind 9 (solo)
at a submix index refuses with reason 12, `notSoloable`.

- **`LiveControlEdits`** (`sdk/src/core/live-controls.ts:398-416`): holds `#tracks: Map<id, index>`
  built from `SessionMap.tracks`, and an optional `#layout` from `layoutOf(session, map.tracks)`
  (`:203-237`), which resolves effects by ID for an SDK-built session. `track(id)` throws
  `MisoUsageError` for an unknown ID and returns `new TrackEdits(index, trackId, layout)`.
- **`layoutOf`** walks `model.tracks` only (`:212-225`) and refuses a session whose declared track IDs
  differ from the compiled ones (`:226-233`). Its `LiveControlLayout` (`:178-184`) maps track ID to the
  track's console instances and inserts.
- **`TrackEdits`** (`:423-610`): `pan` `:434`, `matrix` `:444`, `faderDb` `:456`, `mute` `:464`, `solo`
  `:472`, `trimDb` `:479`, `polarityInvert` `:487`, `hpfHz` `:496`, `lpfHz` `:505`, `inputFilters`
  `:514`, `effect(rack, effectIndex, effectId)` `:531-554` (an **index**, not a name), `console(slot,
  effectId)` `:560-571`, `insert(insert, effectId)` `:574-595` (by stable ID or index). Every method
  builds a `LaneEdit` whose `trackIndex` is the record's index word (`trackEdit`, `:367-389`). There is
  no `effect(name)`; the name-based forms are `console(slot, effectId)` and `insert(id, effectId)`
  (VERIFY-2 M9).
- **`EngineLiveControls`** (`:760-`) holds the map and exposes `edit` and `submit`.
- **Evals.**
  - `sdk/test/live-controls-evals.mjs` runs the shipped wasm through `createOfflineEngine`
    (`sdk/src/headless/engine.ts`), e.g. `liveControls.edit.track("t")` at `:81`.
  - `sdk/test/browser-defaults-evals.mjs:420-460` drives the **browser** `liveControls()` path with a
    stub host whose `sessionMap()` reply builds the browser `SessionMap`.
  - Type tests live in `sdk/test/live-controls-types.ts` (checked by `bash scripts/check-sdk-types.sh`).
  - `bash scripts/check-sdk-headless.sh <A>` runs every `sdk/test/*-evals.mjs` against the shipped
    module.
- **The handoff folder** `docs/handoffs/submix-strips-and-sends/` was created by *Build submix strips
  and bus taps in the SDK and teach agents to author them* (#1205, it holds `APP-SDK.md`). The precedent for a
  live-controls handoff is `docs/handoffs/console-strip-2026-09-29/APP-LIVE-CONTROLS.md`.
- **The master option** keeps its spelling, `liveControls.masterTrackPlusOne` (`sdk/src/core/abi.ts:122`),
  and now takes a strip index plus one (DESIGN P17). misofm/app passes live-control options under an
  older policy key (`console:`) and pins an older SDK.

## Decisions frozen for this slice

- **D1.** `LiveControlEdits.submix(id): SubmixEdits` resolves `id` in `SessionMap.submixes` and
  encodes index `T + j`, where `T = SessionMap.tracks.length`. An unknown ID throws `MisoUsageError`
  naming the known submixes. A track ID passed to `submix()`, or a submix ID passed to `track()`, is
  unknown.
- **D2.** `SubmixEdits` offers exactly `TrackEdits`' surface **minus `solo`**. Implement it by sharing
  the strip-level implementation (one class parameterised by strip index, or a common base), never by
  copying it. `SubmixEdits` is exported through `sdk/src/index.ts`'s existing
  `export * from "./core/live-controls.ts"`.
- **D3.** `layoutOf` resolves effect IDs over every strip: it maps each track **and** each submix
  (`model.submixes`, whose `console` and `inserts` have the track's shapes since K1) to its
  instances, and its consistency check compares declared tracks and submixes with the compiled
  `SessionMap.tracks` and `SessionMap.submixes`. So `edit.submix("drums").insert("glue",
  "miso.compressor")` and `edit.submix("drums").console("comp", "miso.compressor")` resolve for an
  SDK-built session exactly as on a track.
- **D4. App handoff.** Create `docs/handoffs/submix-strips-and-sends/APP-LIVE.md` on the
  `APP-LIVE-CONTROLS.md` model, with:
  - "Live submix strips": `edit.submix(id)`, the absent `solo` (and reason `notSoloable` if a raw
    record solos a bus), `SessionMap.submixes`, and the per-submix meters (`MeterUpdate.submixes`, the
    `miso.meter.v1` fields `submixCount`, `submixPeaks`, `submixGrDb`);
  - "Master designation": `liveControls.masterTrackPlusOne` keeps its name and now takes a strip
    index plus one (tracks first, then submixes), so a mix bus can be the master;
  - "Policy key": the app still passes live-control options under the older `console:` key and must
    move to `liveControls` with this SDK.

## Deliverables

- D1-D3 in `sdk/src/core/live-controls.ts`.
- Evals in `sdk/test/live-controls-evals.mjs` and `sdk/test/browser-defaults-evals.mjs`, and type
  tests in `sdk/test/live-controls-types.ts`.
- D4.

## Authorized paths

- `sdk/src/core/live-controls.ts`, `sdk/src/core/boundary.ts` (only if a `SessionMap` type needs it),
  `sdk/src/index.ts` (only if the barrel needs an explicit export)
- `sdk/test/live-controls-evals.mjs`, `sdk/test/browser-defaults-evals.mjs`,
  `sdk/test/live-controls-types.ts`, `sdk/test/barrel-surface.ts` (only if it pins the exported names)
- `docs/handoffs/submix-strips-and-sends/APP-LIVE.md` (new)
- this spec

## Non-goals

- No engine change.
- No send (route) edits (*Enumerate sends and drive them from the SDK*, #1223).
- No submix solo.
- No rename of `masterTrackPlusOne` (P17).

## Hazards

- **Index base.** The submix index is `T + j`, never `j`. A wrong base silently edits a track. Gates 1
  and 2 pin it through the shipped module.
- **Duplication.** Copying `TrackEdits` would let the two surfaces drift. D2 forbids it.
- **The browser path.** Headless evals pass even if the browser `SessionMap` lacks `submixes`; gate 4
  covers the browser path.

## Objective gates

1. **Encoding.** In `live-controls-evals.mjs`, for a session with 3 tracks and submixes `drums` and
   `verb`: `controls.edit.submix("verb").faderDb(-6).trackIndex === 4`, and
   `controls.edit.submix("drums").pan(-1, 1).trackIndex === 3`; `edit.submix("kick")` (a track ID) and
   `edit.submix("nope")` throw `MisoUsageError`; `edit.track("drums")` throws.

   *Test value: it turns red if the SDK indexes submixes from 0, resolves them against tracks, or lets
   a submix ID through `track()`.*
2. **It moves the bus in the shipped module.** In `live-controls-evals.mjs` (shipped wasm, through
   `check-sdk-headless.sh <A>`): a session whose bus `drums` has no console slots, no inserts and an
   identity input section, fed distinct signals per track; `edit.submix("drums").faderDb(-6)` with
   smoothing 0 is submitted at a block boundary; every block from that boundary is bit-identical to an
   engine booted with `drums.fader` at -6 dB and fed the same sources.

   *Test value: it turns red if the SDK's record reaches the wrong strip or the wrong band in the
   shipped module.*
3. **Effects by ID on a bus.** For an SDK-built session with console slot `comp` and a bus insert
   `glue`, `edit.submix("drums").insert("glue", "miso.compressor").parameter("threshold", -18)`
   encodes the bus's index, the inserts rack and the insert's address, and
   `edit.submix("drums").console("comp", "miso.compressor")` the console rack and slot index; both are
   admitted (`ok`) by the shipped module.

   *Test value: it turns red if `layoutOf` still walks tracks only, or refuses a session with submixes
   as mismatched.*
4. **The browser path.** In `browser-defaults-evals.mjs`, with a stub host whose `sessionMap()` reply
   is `{ tracks: ["t"], submixes: ["bus"], sources: [], metersAttached: false }`,
   `controls.edit.submix("bus").faderDb(-6)` is submitted and the stub receives one record whose index
   word is 1.

   *Test value: it turns red if the browser path's `SessionMap` drops `submixes`, so `edit.submix()`
   throws in the app while passing headless (VERIFY-2 M8).*
5. **Types.** `sdk/test/live-controls-types.ts` asserts that `SubmixEdits` has no `solo` member and
   that `submix()` returns `SubmixEdits` (`bash scripts/check-sdk-types.sh`).

   *Test value: it turns red if solo is exposed on a bus or the return type widens to `TrackEdits`.*
6. **SDK checks.**
   - `bash scripts/check-sdk-types.sh`
   - `bash scripts/check-sdk-headless.sh <A>`
   - `bash scripts/sdk-package.sh check <A>`
7. **K2 boundary gates, run once at the batch head before the root pushes K2:**
   - the workspace test command and the DSP-crates-and-conformance command (DESIGN.md section 7);
   - `cargo test --locked --release -p audit -p bench -p console-workload`;
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`, `bash scripts/trace-graph-audit.sh target/release/audit`,
     `bash scripts/check-graph-determinism.sh`,
     `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`,
     `bash scripts/check-builtins-fixtures.sh . target/release/audit` and
     `bash scripts/check-console-fixtures.sh target/release/session_validator`;
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`;
   - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`;
   - `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`;
   - `python3 -B scripts/check-browser-expected-resources.py --artifacts <A>`;
   - `bash scripts/test-web-audioworklet.sh`;
   - `bash scripts/check-sdk-generated.sh <A>`;
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`;
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
     push (recorded "at batch push").

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new eval's name with its one-sentence test-value answer.
- The K2 boundary gate log, and the K2 push link with its CI run.

### Attempt 1 record (Terra)

- **D1/D2** (`sdk/src/core/live-controls.ts`): `LiveControlEdits.submix(id)` resolves in
  `SessionMap.submixes` only and encodes `T + j`; unknown IDs throw `MisoUsageError` naming the
  known submixes (or `none`). The strip surface moved to one exported base, `abstract class
  StripEdits` (protected `stripIndex`; messages name `track 'x'` or `submix 'x'`); `TrackEdits
  extends StripEdits` adds only `solo`, `SubmixEdits extends StripEdits` adds nothing. `TrackEdits`'
  public constructor is unchanged. Exported through the existing `export *`; `index.ts` and
  `barrel-surface.ts` untouched. **D3**: `layoutOf(session, map)` walks `model.tracks` and
  `model.submixes` through one `stripInstances()`; the layout keeps separate `tracks`/`submixes`
  maps, and the consistency check compares each list with the compiled one. **D4**:
  `docs/handoffs/submix-strips-and-sends/APP-LIVE.md`.
- **Tests and test value** (each mutation applied, run red, reverted):
  - `live-controls-evals.mjs` "a submix encodes its strip index T + j and IDs resolve only in their
    own list" (gate 1; shipped module, tracks kick/snare/vox, buses drums/verb; also submits all
    nine builtin edits on `verb`, admitted 9): red if submixes index from 0, resolve against tracks,
    or a submix ID passes `track()`. Mutations: base 0 -> red; base +1 -> red; `submix()` reads
    `#tracks` -> red; `track()` falls back to submixes -> red.
  - "a live bus fader equals the bus booted at that fader from the edit's block on" (gate 2;
    distinct LCG source per track; blocks 0-3 must differ, blocks 4-7 bit-identical L and R,
    `appliedAtSample` 512): red if the record reaches another strip or band. Mutations: base 0 /
    +1 -> red; `faderDb` forced to the left lane -> red (only this test).
  - "console slots and inserts resolve by ID on a bus of an SDK-built session" (gate 3; slots
    `eq`,`comp`; drums inserts `tone`,`glue`; verb `glue` alone; both edits admitted `ok`): red if
    `layoutOf` walks tracks only, refuses submixes as mismatched, resolves a bus against another
    strip's chain, or drops the submix consistency check. Mutations: submixes walk `[]` -> red;
    tracks+submixes compared against `map.tracks` -> red; bus instances read from the tracks map
    -> red (only this test); submix comparison dropped -> red.
  - `browser-defaults-evals.mjs` "a browser engine's live controls address a submix by its strip
    index" (gate 4): red if the browser `SessionMap` drops `submixes`. Mutation: browser map
    `submixes: []` -> red.
  - `live-controls-types.ts` gate 5 (`Exact<ReturnType<submix>, SubmixEdits>`, no `solo` key,
    `TrackEdits` minus `solo` equals `SubmixEdits`' keys, `@ts-expect-error` on `bus.solo` and on
    assigning a `SubmixEdits` to `TrackEdits`): red if solo is exposed on a bus or `submix()`
    widens. Mutations: `submix()` returns `TrackEdits` -> 3 TS errors; `SubmixEdits.solo` added
    -> 4 TS errors.
- No existing test edited or superseded; no digest, oracle or canonical text pinned.
- **Gates** (x86_64 AVX2; A = `target/ci/k2-1214-artifacts`, B = `target/ci/k2-1214-named`; logs
  in `target/ci/k2-1214-logs/`), all rc 0: 6: `check-sdk-types.sh`; `check-sdk-headless.sh` (346
  pass, 0 fail); `sdk-package.sh check`. 7 (at `c13ac5e1d` plus this slice): workspace test
  command (103 binaries, 1172 passed, 0 failed); DSP crates and conformance (145 binaries, 787
  passed); `cargo test --release -p audit -p bench -p console-workload` (110 passed); release
  build; `audit capi`; `trace-graph-audit.sh`; `check-graph-determinism.sh`; `graph_fixture
  --check`; `check-builtins-fixtures.sh`; `check-console-fixtures.sh`; `check-capi-abi.sh` and
  `--self-test`; `build-web-audioworklet.sh --named-twin`; `check-web-audioworklet.sh`;
  `check-browser-expected-resources.py --artifacts` (32 red mutations); `test-web-audioworklet.sh`;
  `check-sdk-generated.sh`; `check-/test-host-core-policy.sh`; `check-/test-realtime-policy.sh`;
  `cargo fmt --check`; workspace clippy `-D warnings`; `cargo doc -D warnings`.
  `run-aarch64-tests.sh debug`: no arm64 host; at batch push.
- **Artifact unchanged:** shipped module `7d6c0a8b...0f90b` (2 695 834 B), byte-identical to
  #1213's (no engine change).
- K2 push link and CI run: the root's, at the batch push.

## Decision record (K2 follow-ups)

- **Amendment A1 (verdict MINOR-1).** Authorized path added:
  `sdk/src/core/observation-subscriptions.ts`. `LiveControlEdits.strip(id)` resolves a track, else a submix (strip IDs are unique across both),
  and returns `StripEdits`, so it never offers `solo`. The managed observation owner arms and
  disarms through it, so `subscribeObservations()` accepts a bus tap. New headless eval
  (`capability-evals.mjs`, "a managed subscription to a bus tap arms it at the bus's strip index
  and reads it ready"). Test value: red if the managed owner builds its edit with
  `edit.track()`; the mutation (restore `edit.track`) turns it red with "no track 'bus'".
  `live-controls-types.ts` adds that `strip()` has no `solo`. `APP-LIVE.md` documents it.
- **The browser path pins the submix order** (verdict MINOR-2). Gate 4's stub host replies with
  two submixes, `["aaa", "bus"]`, and requires index word 2 for `bus` and 1 for `aaa`. Test value:
  red if the browser map reorders the host's list. Mutation M3 (`[...remoteMap.submixes].reverse()`)
  turns it red. The worklet half is #1210's MINOR-1, recorded there.
- **NITs:** NIT-3 applied (`LiveControlRack` and constructor docs, `sdk/README.md`); NIT-4 applied
  (`APP-LIVE.md`: a master designation needs observation taps and reads `null` until its strip
  publishes; the `console:` policy key fails as a TypeScript excess-property error first). NIT-1
  (brand `SubmixEdits`) and NIT-2 (`stripIndex` visible at runtime) are not taken: type-only and
  harmless, with the runtime ID separation intact.

## Verdict

- **Attempt 1** (`4df439f2c`): Sol PASS, no BLOCKER or MAJOR; two MINOR, four NIT, applied in the K2
  follow-up commit as above. `docs/handoffs/submix-sends-2026-10-02/verdicts/1214-attempt1.md`.

## Dependencies

- *Name submix strips in the browser session map and the SDK measurement* (#1210)
- *Address submix strips in browser live commands* (#1213)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Prefer the SDK's existing patterns (`MisoUsageError`, the `TrackEdits` encoders); add no new
  framework.
- A test that greps source or prose is refused.
- Commit on the K2 batch branch. The root pushes K2 once after this slice's verdict.
- Attempt budget: five attempts, one adversarial verdict each.
