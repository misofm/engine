# Measure a session rebuild on the browser's audio thread

Slice B1 of *Swap a rebuilt plan without an audio gap* (#1269). Measurement and evidence only: no
engine change. Code anchors verified on `54b0a1bf8` (unchanged at `24029badb`). It may run at any
time, beside the umbrella's current feature slice.

## Product outcome

The owner learns how long the browser engine blocks its audio thread to rebuild a session, for real
session sizes, before the browser's replacement path (B2-B5) ships. The browser has only one thread
that can touch the engine's memory: the Wasm instance lives inside the `AudioWorkletProcessor`
(`hosts/host-web/web/miso-engine-v1-audio-worklet.js:286`), the Web Audio specification runs
`AudioWorkletGlobalScope` on the rendering thread, and `scripts/check-web-audioworklet.sh` refuses
shared memory and atomics in the shipped module (`:372-382`). So a replacement is prepared between
two `process()` calls. If that takes longer than the output buffering absorbs, the replacement
itself is a dropout. This number answers owner question Q4.

## Context

- A boot today runs the whole pipeline a replacement will run: document parse, session compile,
  host-core preparation (effects, builtins, graph compile, PDC, bind) and the bridge buffers
  (`AudioWorkletEngineHost::boot_with_spectrum_config`, `hosts/host-web/src/lib.rs:2039`; export
  `miso_engine_web_v1_boot`, `hosts/host-web/src/ffi.rs:3681`). A replacement additionally builds
  the carry program and runs the carry, and skips most bridge allocation.
- The V8 benchmark tooling boots the shipped module under Node: `scripts/web-mixing-automation-benchmark.mjs`
  (`boot()`, `:311`). The 64-track documents are fixtures:
  `fixtures/session/v1/console-sixty-four-track.json`, `console-sixty-four-track-app.json` and
  `console-sixty-four-track-sends.json`.
- The budget one render quantum leaves is `128 / 48000 s = 2.667 ms` at the default quantum and
  48 kHz; the browser's own output buffering adds a platform-dependent margin.
- AGENTS.md benchmark rules: freeze the workload and validator first, one invocation, one warmup,
  two measured rounds, no tuning.

## Decisions frozen for this slice

- **D1. What is timed.** The wall time of `miso_engine_web_v1_boot` on the shipped module under
  Node's V8, with the browser's live-control boot options (the SDK default command queue), for:
  the 9-track `parametric-eq-nine-track.json`, the 64-track console, the 64-track app shape and the
  64-track sends session. Each boot is followed by `miso_engine_web_v1_dispose`.
- **D2. One Chromium point.** The 64-track app-shape boot timed once inside a real AudioWorklet
  through the browser qualification harness (`hosts/host-web/qualification`), to check that the V8
  proxy is in the same range. If the harness cannot time inside the worklet without a product
  change, record that and skip D2.
- **D3. Record.** `artifacts/steps/web-rebuild-base/` with the raw record, the validator result and
  a short report: p50 and max boot time per document, the ratio to one quantum's budget, and the
  peak Wasm memory.

## Deliverables

1. A script `scripts/web-rebuild-cost.mjs` (or a mode of the existing V8 benchmark) with a validator
   and an untimed preflight.
2. One timed invocation and its record (D3).
3. A comment on the umbrella with the numbers and the planner's reading for Q4. B2 waits for this
   comment, and for the owner's ruling on Q4 if a 64-track boot exceeds one quantum's budget.

## Authorized paths

- `scripts/web-rebuild-cost.mjs` and its validator, or `scripts/web-mixing-automation-benchmark.mjs`
  and `scripts/run-web-mixing-automation-benchmark.sh` for a new mode
- `hosts/host-web/qualification/` (D2 only, test harness code)
- `artifacts/steps/web-rebuild-base/`

## Non-goals

- No product code. No optimisation of boot.
- No decision on Q4: the owner rules.

## Objective gates

1. The preflight boots every document once without timing and checks it renders an audible block.
2. The validator refuses a record without all four documents, without two measured rounds, or with
   a boot that failed.
3. Exactly one timed invocation.
4. Commands: `python3 -B scripts/check-script-reachability.py` and
   `python3 -B scripts/test-script-reachability.py` pass with the new script, plus the umbrella's
   inherited gates.

## Test value

- Gate 2: a record that silently timed a failed boot (fast, because it refused early) would
  understate the cost; the validator turns red on it.

## Dependencies

None. It may run at any time.
