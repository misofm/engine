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

**Umbrella comment (Deliverable 3), ready for the root to post on #1269 as written from here to
the end of "Reading against the budget".**

**Numbers (D3), V8 proxy, both rounds.** Budget: one quantum = 128 / 48000 s = 2.667 ms.

| document | tracks | boot p50 ms | boot max ms | p50 / budget | max / budget | peak Wasm memory |
|---|---|---|---|---|---|---|
| nine-track EQ | 9 | 2.69 / 2.68 | 11.4 / 11.9 | 1.0 | 4.3 / 4.5 | 1.75 MiB |
| 64-track console | 64 | 23.9 / 23.9 | 33.5 / 37.2 | 9.0 | 12.6 / 14.0 | 5.44 MiB |
| 64-track app shape | 64 | 23.4 / 23.5 | 32.4 / 33.0 | 8.8 | 12.1 / 12.4 | 5.50 MiB |
| 64-track sends | 64 | 39.1 / 39.2 | 49.1 / 48.5 | 14.7 | 18.4 / 18.2 | 11.06 MiB |

Minimums sit within 1-2 % of p50 (app shape 23.25 ms, sends 38.5 ms). Dispose p50 is 0.2-0.5 ms
for the 64-track documents and 0.03 ms for the nine-track one.

**D2, one Chromium point** (HeadlessChrome 151.0.7922.34, Playwright 1.62.1, unpinned, loadavg
25.2). The app shape booted inside a test `AudioWorkletProcessor.process()` on the rendering
thread, `Date.now()` (1 ms resolution; Chromium's `AudioWorkletGlobalScope` has no `performance`):
first boot **141 ms** (the first run of the module's code, before V8's tier-up), second boot on a
fresh instance **47 ms**; both audible and disposed. `context.baseLatency` 11.6 ms, `outputLatency`
0 (headless). Record: `artifacts/steps/web-rebuild-base/chromium.json`. The warm Chromium boot is
about 2x the V8 proxy's p50; the cold one about 6x.

**Caveats the numbers carry.**
- The V8 figure is a lower bound for the browser. The proxy runs Node with `--no-liftoff`
  (optimizing tier only); Chromium tiers up dynamically, and the D2 point above is 2x (warm) and 6x
  (cold) the proxy's p50.
- A boot is a proxy for a replacement, not the same work. It includes bridge allocation and Wasm
  memory growth on a fresh instance, which a replacement mostly skips; it excludes the carry
  program a replacement also builds and runs. Neither direction is measured here.
- The budget compared is one quantum (2.667 ms). Headless Chromium reported `baseLatency` 11.6 ms;
  every 64-track p50 under the V8 proxy exceeds even that margin 2x-3x.
- The run was uncontrolled (loadavg 26-28 on 32 CPUs). The p50s are not load-dominated (minimums
  within 1-2 % of p50); the maxima are not interpretable.

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

### Attempt 1 verdict (Sol)

PASS with 3 MINOR and 4 NIT, no BLOCKER or MAJOR. The verifier re-ran every inherited gate, rebuilt
the module byte-identical to the measured one (`30d075d3...`), reproduced M1-M3, ran 23 synthetic
validator cases, ruled the relaunch after the refused launch 1 legitimate (no boot was timed, and
the fix did not touch the frozen workload or the validator), and found the uncontrolled run stated
candidly. Verdict copy: `docs/handoffs/seamless-swap-phase1-2026-10-04/1289-attempt1.md`.

### Review follow-ups (no re-timing)

- **MINOR-1 (validator does not cross-check the printed summaries).** `rebuildRefusalReasons` now
  also requires: `boot_p50_ns` (nearest rank), `boot_max_ns` and `boot_min_ns` equal to their
  recomputation from `boot_ns`; ordered positive integer `dispose_p50_ns <= dispose_max_ns` (the
  record keeps no raw dispose times, so only the summary's shape is checkable);
  `observations_per_document == 25`; a non-blank `measurement_control`; each document's
  `fixture_id` and `tracks` equal to its frozen `REBUILD_DOCUMENTS` entry and a 64-hex
  `document_sha256`; and the two rounds agree on every document's `document_sha256`. The committed
  `artifacts/steps/web-rebuild-base/web-rebuild-cost.jsonl` re-validated with it: accepted (no
  re-timing; `validator.json` left as written by the run, same verdict). The verifier's 23 cases
  rerun: the 18 refusal cases and the valid case unchanged in verdict; the 4 probes it accepted
  (p50 inconsistent, no control, wrong fixture under the same kind, document digests disagreeing),
  plus `observations_per_document = 0`, are now refused.
- **MINOR-2 (gate-2 cases not reproducible).** `scripts/test-console-benchmark.sh` now carries the
  rebuild validator's cases beside the sibling browser arm's: a synthetic base pair (accepted) and
  32 refusals run through the real `rebuild-validate` (protocol, failed/silent boot, documents and
  fixtures, observations and summaries, frozen workload, cross-round agreement). Mutation evidence:
  against the pre-follow-up validator (`git show HEAD:` copy) the suite fails exactly the 12 cases
  of the new rules; with the failed/audible check replaced by `false` (M1) it fails "a boot that
  rendered silence" and "a document that does not count its failed boots" (the failed-boot case is
  still refused by the dispose check, as M1 found). Restored: PASS. Note: the suite now runs the
  real harness, which at load reads `sdk/assets/miso-engine-v1-abi-layout.json` and imports
  `hosts/host-web/web/prepared-control.js`; neither is in the `console-benchmark` router entry
  (`scripts/ci-path-router.py`, not an authorized path), so a change there reaches this suite
  only in `nightly.yml`.
- **MINOR-3 (D2 record lacks control and provenance).** The committed `chromium.json` is not edited:
  its load average ("25.2", one figure) and Playwright version (1.62.1) are recorded in prose only,
  above, and the commit it ran at was not recorded anywhere (the harness was unchanged between
  `52ac9f88d` and `d169ef5f8`, and the module digest equals the measured one, but the exact HEAD is
  not a recoverable fact). `hosts/host-web/qualification/rebuild-cost.mjs` now records
  `candidate_commit`, `playwright_version`, `loadavg_start` and `loadavg_end` for any rerun; the
  commit and version are read before the overwrite check, so a missing fact fails before Chromium
  launches (exercised untimed with an existing OUT path: provenance read, then "refusing to
  overwrite", status 1; no browser launched).
- **NIT 1 (raw stdout lost on a parse failure).** `rebuildRun` now keeps a measured launch's stdout
  that does not parse in `web-rebuild-cost.unparsed.txt` (exclusive create, and in the pre-launch
  overwrite refusal list; not `*.log`, which is gitignored) and refuses the run naming it.
- **NIT 2.** The dispose sentence now gives 0.2-0.5 ms for the 64-track documents and 0.03 ms for
  the nine-track one.
- **NIT 3.** The umbrella comment above now carries the three caveats: the `--no-liftoff` V8 figure
  is a lower bound for the browser; a boot includes bridge allocation and memory growth and
  excludes the carry program a replacement also builds and runs; the budget is one quantum, and
  even Chromium's 11.6 ms `baseLatency` is exceeded 2x-3x.
- **NIT 4.** Accepted as is: the refused launch's stderr is gitignored; the essential line is
  quoted in the timed-invocation entry above.

**Follow-up gates.** `node --check` on both changed `.mjs`; `bash -n` on the changed `.sh`;
`rebuild-validate` on the committed record (accepted); `rebuild-preflight` on the measured module
(untimed, all four documents audible, memory figures equal to the record's `preflight`);
`rebuild-run` pre-launch refusal of an existing `web-rebuild-cost.unparsed.txt`;
`scripts/test-console-benchmark.sh` PASS; `check-` and `test-workspace-policy.sh`;
`check-` and `test-script-reachability.py`. No workflow or nightly command changed.

**Router follow-up (closes MINOR-2's note).** `scripts/ci-path-router.py`'s `console-benchmark`
self-test key now names `hosts/host-web/web/prepared-control.js` and
`sdk/assets/miso-engine-v1-abi-layout.json`, the two files the real rebuild harness loads, so a
change to either alone selects `scripts/test-console-benchmark.sh` in `gate-self-tests` instead of
reaching it only nightly. `scripts/test-ci-path-routing.py` names both independently of the table.
Router answers (`--event pull_request --path <p> --flags`): `prepared-control.js` gives
`route=full`, `self_tests=["console-benchmark"]`; the ABI layout gives `route=sdk`,
`self_tests=["console-benchmark","sdk-deletions"]` (it was `["sdk-deletions"]` through the `sdk/`
prefix). Mutation: deleting either entry from the router's table turns
`test-ci-path-routing.py` red on that path's assertion (`('hosts/host-web/web/prepared-control.js',
[])`, `('sdk/assets/miso-engine-v1-abi-layout.json', ['sdk-deletions'])`); restored, green. Gates:
`check-ci-path-routing.py` and `test-ci-path-routing.py` PASS; `check-workspace-policy.sh` PASS.
