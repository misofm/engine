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
