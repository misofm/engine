# Complete headless SDK ABI capability parity

## Objective

Expose every Engine V1 control-plane capability already present in the zero-import Wasm ABI through
the headless TypeScript SDK. In particular, a Node or Bun agent must be able to seek streamed
sources and lease/poll the same decimated meter frames the browser host exposes, without reaching
into generated layouts or numeric exports.

This is the smallest closable prerequisite for issue #207's shared semantic console. Typed command
construction and Effect integration are successor slices: they consume this parity surface but do
not belong in its low-level binding proof.

## Product contract

- `OfflineEngine.seekSource()` stages a bounded UTF-8 source ID and calls the frozen source-seek
  export with a positive generation and absolute source frame.
- `OfflineEngine.meters()` takes or releases the engine meter lease and returns a typed call result.
- `OfflineEngine.pollMeters()` returns either no completed window or a copied, immutable meter
  frame with track peaks, master peaks, per-track gain reduction, optional master gain reduction,
  absolute sample bounds, window count, and engine sequence.
- Headless session-map and status types use the same semantic field names as the shipped browser
  host wherever the underlying capability is shared. Browser-only AudioContext timing telemetry is
  explicitly not fabricated headlessly.
- Callers never name a Wasm export, buffer kind, byte offset, or numeric result constant.

## Scope

- The shared Wasm boundary and headless public entry.
- Shared capability/result types that do not depend on DOM or Node APIs.
- Live-Wasm behavioral evals, strict declaration checks, and public barrel assertions.

No command-factory redesign, effect-name addressing, browser host mutation, telemetry emulation,
Effect dependency, or registry publication is in scope.

## Objective gates

1. A live headless session seeks a source to a new generation/frame, submits from that generation,
   renders, and remains usable; malformed generations/frames refuse before crossing the ABI.
2. With meters prepared, lease + render + poll yields the expected `2T + 2` peak words, `T` track
   gain-reduction words, optional master value, and exact absolute sample window.
3. A released lease yields no frame, and a session prepared without meters returns the engine's
   typed `unsupported` answer rather than zeros.
4. The implementation reads all header fields and frame sizing through generated ABI data; a red
   shape mutation is discriminated by the eval.
5. Existing SDK behavior, strict type, generated-surface, package, and deletion gates remain green.

## Decision record

- Meter polling copies the completed frame out of Wasm memory. Holding a public view would let a
  later render mutate an allegedly historical observation and would retain detachable memory.
- `pollMeters()` is synchronous because it directly drains the in-process engine. The browser host
  remains callback/Promise based because MessagePort transport is genuinely asynchronous.
- Render timing telemetry remains browser-only: it measures AudioWorklet deadline behavior, which a
  faster-than-realtime headless loop does not possess.
- The acked-batch review question remains answered by the existing command path: command admission
  is whole-batch and the report is read only after `command_submit` returns. This issue introduces
  no queued acknowledgement path.

## Evidence

Implementation attempt 1:

- The direct Wasm boundary now exposes typed status/session-map snapshots, source seek, meter lease,
  and copied meter polling; `OfflineEngine` and the headless barrel carry the complete surface.
- Meter frames validate the generated structure size, ABI version, track count, completed-window
  count, master-presence flag, and exact generated `3T + 3` buffer shape before exposing values.
- Source IDs share one bounded staging path between submit and seek. Invalid generations and frames
  refuse locally; engine refusals return the generated result-name union.

Local gates on 2026-09-02:

- `check-sdk-headless.sh`: PASS, 105 tests / 26 suites against live Wasm.
- `check-sdk-types.sh`: PASS, including shipped-host mirror and new declarations.
- `check-sdk-deletions.py`: PASS over 36 SDK source files.
- `sdk-package.sh check`: PASS; clean tarball import/type/embedded-boot smoke remains green.
- Focused capability eval: PASS for status/map, seek generation/frame, a two-block observed meter
  window at exact samples `[0, 256)`, historical-frame detachment, release, and unsupported lease.

Adversarial review:

- PASS locally on ABI coverage, generated-layout use, copied-memory lifetime, refusal semantics, and
  the acked-batch question. Final issue closure requires the implementation commit's upstream CI,
  browser qualification, and release-build results.
