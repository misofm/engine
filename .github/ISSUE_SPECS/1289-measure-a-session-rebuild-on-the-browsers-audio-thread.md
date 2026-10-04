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

## Attempt record

### Attempt 1 (implementer)

**Shape.** A standalone `scripts/web-rebuild-cost.mjs` fails `check-ci-path-routing.py` (the
`console-benchmark` suite would read a file its router entry does not list, and the router is not
an authorized path), so the harness is the spec's alternative: a `rebuild-*` mode of
`scripts/web-mixing-automation-benchmark.mjs`, dispatched before anything reads the control table,
with two runner subcommands in `scripts/run-web-mixing-automation-benchmark.sh`
(`rebuild-preflight WORKDIR`, `rebuild-run WORKDIR --step NAME`; the module comes from the existing
`prepare`). D2 is `hosts/host-web/qualification/rebuild-cost.mjs`, harness code only.

**Frozen workload (D1).** `miso_engine_web_v1_boot` alone in the clock (`process.hrtime.bigint`),
on a fresh `WebAssembly.Instance` of the shipped module per boot (memory growth is inside the
clock), Node `--no-liftoff`, boot options = SDK default command queue, no meters, observation taps,
master or spectrum, documents as checked in. 25 boots per document per round, the four documents
alternated per observation. After every timed boot, outside the clock, the session is fed a sine,
renders 8 blocks and must be audible, then is disposed (dispose timed on its own). Runner: one
warmup launch (discarded) and two measured launches, `taskset` to the highest CPU, one process per
round.

**Validator (gate 2)** (`rebuildRefusalReasons`, `rebuild-validate`): refuses unless exactly two
records, rounds 1 and 2, all four documents in frozen order, 25 positive integer boot times each,
`failed_boots == 0`, `booted == audible_after_boot == disposed == 25`, a peak memory, and the same
module and commit in both rounds. Synthetic record cases: valid accepted; one round, a failed boot,
a silent boot, a missing document, a short round, a kept warmup, disagreeing modules each refused.

**Mutation runs.**
- M1 (gate 2, the spec's test value): validator's failed-boot/audible check replaced by `false` →
  the silent-boot record is **accepted** (red); with the dispose check also removed, the failed-boot
  record (one boot refused, its time 1.2 µs) is accepted. Reverted → both refused.
- M2 (gate 1): boot options require 44.1 kHz → `rebuild-preflight` exits 1, "boot refused with
  result 9". Reverted → passes.
- M3 (gate 1): the fed PCM zeroed → `rebuild-preflight` exits 1, "rendered no audible block".
  Reverted → passes.
- Pre-launch refusals checked without launching: an existing output file ("refusing to
  overwrite"), modified tracked files, an invalid `--step` name.

**Timed invocations (gate 3).**
1. Launch at `52ac9f88d` refused at the warmup's document load, before any boot was timed: one
   loader call had not been renamed when the mode was merged into the harness, and the preflight
   did not reach that path. No number was taken. Its verdict is kept at
   `artifacts/steps/web-rebuild-base/refused-launch-1/validator.json` (the stderr log is `*.log`,
   gitignored: `ENOENT ... open '<root>/[object Object]'` in `loadDocument`). Fixed in `0b477c585`,
   where `rebuild-preflight` now runs the whole round path with zero timed observations.
2. The one measured invocation, at `0b477c585`, module
   `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4` (not the release pin
   `6c952a2c...`; the tree has moved since the last release). **Uncontrolled**: loadavg 26-28 on 32
   CPUs from other agents' builds, `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1`, pinned to CPU 31.
   Validator accepted. Record, verdict and report: `artifacts/steps/web-rebuild-base/`
   (`web-rebuild-cost.jsonl`, `validator.json`, `report.md`).

**Numbers (D3), V8 proxy, both rounds.** Budget: one quantum = 128 / 48000 s = 2.667 ms.

| document | tracks | boot p50 ms | boot max ms | p50 / budget | max / budget | peak Wasm memory |
|---|---|---|---|---|---|---|
| nine-track EQ | 9 | 2.69 / 2.68 | 11.4 / 11.9 | 1.0 | 4.3 / 4.5 | 1.75 MiB |
| 64-track console | 64 | 23.9 / 23.9 | 33.5 / 37.2 | 9.0 | 12.6 / 14.0 | 5.44 MiB |
| 64-track app shape | 64 | 23.4 / 23.5 | 32.4 / 33.0 | 8.8 | 12.1 / 12.4 | 5.50 MiB |
| 64-track sends | 64 | 39.1 / 39.2 | 49.1 / 48.5 | 14.7 | 18.4 / 18.2 | 11.06 MiB |

Minimums sit within 1-2 % of p50 (app shape 23.25 ms, sends 38.5 ms); dispose is 0.2-0.6 ms.

**D2, one Chromium point** (HeadlessChrome 151.0.7922.34, Playwright 1.62.1, unpinned, loadavg
25.2). The app shape booted inside a test `AudioWorkletProcessor.process()` on the rendering
thread, `Date.now()` (1 ms resolution; Chromium's `AudioWorkletGlobalScope` has no `performance`):
first boot **141 ms** (the first run of the module's code, before V8's tier-up), second boot on a
fresh instance **47 ms**; both audible and disposed. `context.baseLatency` 11.6 ms, `outputLatency`
0 (headless). Record: `artifacts/steps/web-rebuild-base/chromium.json`. The warm Chromium boot is
about 2x the V8 proxy's p50; the cold one about 6x.

**Reading against the budget (no Q4 recommendation).** Every 64-track boot exceeds one quantum's
budget by 9x-15x at p50 under the V8 proxy and more in Chromium; the nine-track boot is at the
budget at p50. Under the spec's Deliverable 3, B2 waits for the owner's ruling on Q4.

**Gates (all PASS, after the timed run, no Rust or Wasm-module source changed):** `cargo fmt --all
-- --check`; `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; `check-` and
`test-workspace-policy.sh`; `check-` and `test-realtime-policy.sh`; `check-capi-abi.sh`; `cargo
build --locked --release -p audit -p capi && ./target/release/audit capi`;
`check-cross-targets.sh`; `check-` and `test-script-reachability.py`; `check-ci-path-routing.py`,
`test-ci-path-routing.py`; `test-console-benchmark.sh` (the browser runner's stub cases still
pass with the new subcommands). The worklet chain was not run: no code compiled into the module
changed, and the prepared module's digest matches a module built before the harness edits.

**Not done here:** Deliverable 3 (the umbrella comment) needs GitHub, which this worker may not
touch; the numbers above are its content.
