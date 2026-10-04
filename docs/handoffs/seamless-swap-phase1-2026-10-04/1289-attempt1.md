# #1289 attempt 1 verdict: PASS

PASS with minors. No BLOCKER and no MAJOR.

- Commits reviewed: `52ac9f88d`, `0b477c585` and `d169ef5f8`, on parent `41517fc35`. I reviewed the diff `git diff 41517fc35 d169ef5f8`.
- Export: `/tmp/claude-1002/v1289/attempt1/`. I ran `git init` there so the git-dependent gates could run.
- Verifier logs: `/tmp/claude-1002/v1289/attempt1-logs/`.
- Work directory for `prepare`: `/tmp/claude-1002/v1289/attempt1-work/`.

Scope: measurement only. The diff changes no Rust, Cargo or shipped-module source. All 9 changed paths are authorized:
- `scripts/web-mixing-automation-benchmark.mjs`
- `scripts/run-web-mixing-automation-benchmark.sh`
- `hosts/host-web/qualification/rebuild-cost.mjs`
- `artifacts/steps/web-rebuild-base/**`
- the slice spec

No `node_modules` symlink and no `*.log` file is committed. Both exist in the implementer worktree, but they are ignored and untracked. All three commits carry the `Co-Authored-By` trailer.

## Gates re-run in the export (all PASS)

- `cargo fmt --all -- --check`
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
- `check-workspace-policy.sh` and `test-workspace-policy.sh`
- `check-realtime-policy.sh` and `test-realtime-policy.sh`
- `check-capi-abi.sh`
- `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`: 0 allocations, 0 syscalls, 0 locks, 0 violations.
- `check-cross-targets.sh`: PASS. Only the known #1018 `memset_pattern16` expected failures appear.
- `check-script-reachability.py` and `test-script-reachability.py` (gate 4)
- `check-ci-path-routing.py` and `test-ci-path-routing.py`
- `test-console-benchmark.sh`: the browser runner's stub cases still pass with the two new subcommands.
- `run-web-mixing-automation-benchmark.sh prepare` on the exported tree. It built `host_web.wasm` with sha256 `30d075d3ce6382f21235675996184c675753acf6d451e11d7d676a3d50aaeff4`.
  - This is byte-identical to the module the record measured.
  - So the measured module is the module this tree builds. The parent builds the same module, because no Rust changed.
  - It is not the release pin `6c952a2c...`. That mismatch already exists on main, and the record says so.
- Worklet chain: not run. No code compiled into the module changed, and the digest above confirms the module is unchanged.
- `rebuild-preflight`: PASS. All four documents booted and were audible. Peak memory per document: 1835008, 5701632, 5767168 and 11599872 bytes. These match the committed record's `preflight` field.
- Existing `preflight` mode: PASS. The new dispatch does not disturb the render benchmark.

I did not rerun the timed benchmark (`rebuild-run` past its refusals) or the D2 Chromium timing. The one-invocation rule forbids it.

## Spec gates and decisions

### Gate 1 (untimed preflight, audible block)
Met. I checked it with my own mutations in the export, each reverted afterwards:
- **Fed PCM zeroed:** `rebuild-preflight` exits 1 with `preflight nine_track_eq: rendered no audible block`.
- **`requireQuantumFrames` set to 256:** it exits 1 with `boot refused with result 9`.
- **Reverted:** exits 0.

### Gate 2 (validator)
Met. I built 23 synthetic record sets from the committed jsonl and ran them through `rebuild-validate`.
- **Accepted:** the committed record.
- **Refused:**
  - one round
  - a kept warmup, both as three records and as rounds [0,1]
  - rounds [1,1]
  - a failed boot with a 1.2 µs time
  - a silent boot
  - a missing document
  - a reordered document list
  - an extra document
  - a short round (24 observations)
  - a zero boot time
  - a non-integer boot time
  - no peak memory
  - a wrong quantum
  - a missing `failed_boots`
  - disagreeing module
  - disagreeing commit

I also reproduced the record's mutation M1:
- I replaced the failed/audible check with `false`. The silent-boot record was then **accepted**. The failed-boot record was still refused, by the dispose check.
- I then also removed the dispose check. The failed-boot record was then **accepted**.
- After reverting, both are refused.

### Gate 3 (exactly one timed invocation)
Met. Below is my ruling on the relaunch.

### D1, D2 and D3
All met.
- **D1:** only `miso_engine_web_v1_boot` is inside `hrtime`. Boot options are the SDK default command queue (`defaultCommandQueueRecords` = 64) with nothing else attached. That matches `sdk/src/core/abi.ts:189-192`, where `liveControls` is absent by default. Every boot is followed by a dispose, timed separately.
- **D2:** one Chromium in-worklet point.
- **D3:** the record has p50, max, budget ratio and peak memory.

I recomputed every p50 (nearest rank), max and min, and the dispose and memory figures, from the raw `boot_ns` arrays. They agree with the record, `report.md` and the spec table.

### Non-goals
Held. There is no product code and no Q4 decision.

## Ruling: the relaunch is legitimate, not a disguised retry

The first `rebuild-run` launch, at `52ac9f88d`, timed no boot.
- **Where it stopped:** the warmup child died in `rebuildRound` at `REBUILD_DOCUMENTS.map(loadDocument)`. That is the old harness's loader, called with an object, so it opened `<root>/[object Object]`. I read the ignored stderr log in the implementer worktree; its stack shows `rebuildRound` at line 331 calling `loadDocument` at line 551.
- **Nothing else ran:** the failure came before the premise boots and before any timed observation. The runner broke on the warmup failure, so the two measured rounds never launched.
- **So nothing could be cherry-picked:** no measurement existed. AGENTS.md's "do not retry" rule protects against re-timing to get a better number, and that did not happen here.
- **The fix changed only the loader and the preflight.** The fix commit `0b477c585` changes:
  - the loader call
  - `rebuildRound`'s observation count, now a parameter so the preflight can reuse the round with 0 observations
  - a `preflight` field in the record

  It does not change the frozen workload (documents, 25 observations, boot options, clock placement, pinning, launches) or the validator. The validator is byte-identical between `52ac9f88d` and `d169ef5f8`.
- **The fix closes the preflight gap:** `rebuild-preflight` now runs the whole round path. That gap was an AGENTS.md preflight defect ("preflight ... without launching the timed workload"), and closing it is correct.
- **The record is candid:**
  - The refused verdict is committed at `refused-launch-1/validator.json`.
  - The attempt record names the launch, the cause, the commit and the fix, and says no number was taken.

## Ruling: the uncontrolled measurement is stated candidly and does not change the reading

The "uncontrolled" statement appears in three places:
- each jsonl record's `measurement_control` field: "uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived loadavg_above_ceiling; loadavg 28.07 ...". Each record also has `loadavg_start`/`loadavg_end` (26-27).
- the header of `report.md`
- the attempt record

All three say "descriptive only". The conclusion does not depend on the load. Each document's minimum sits within 0.7-1.5 % of its p50 (app shape 23.25 vs 23.42 ms, sends 38.55 vs 39.12 ms). So the p50 is not load-dominated, and a 9x-15x excess over one quantum is far outside anything the load could explain. Only the tails (max 32-49 ms) are uninterpretable under this load.

## D2 clock claim: verified

I ran a timing-free probe in Chromium (HeadlessChrome 151.0.7922.34, Playwright 1.62.1 from the main checkout's qualification `node_modules`). Inside the `AudioWorkletGlobalScope`:
- `typeof performance === "undefined"`
- `Date.now` steps in 1 ms
- the only time-like globals are `Date` and `currentTime`

So `Date.now()` at 1 ms is the best clock available without a product change. That is about ±2 % on 47 ms and less than 1 % on 141 ms, which is enough for the D2 range check.

The spec says D2 is "timed once". The probe times two boots in one run (cold 141 ms, warm 47 ms). That is disclosed and more informative than one boot. I do not count it as over-scoping.

## Deliverable 3

It is not posted, which is correct: workers may not write to GitHub. The content is ready in the attempt record at spec lines 138-161:
- the numbers table
- the D2 point
- the budget reading
- an explicit "no Q4 recommendation"

The root can post it. NIT 3 below lists the caveats the comment should carry.

## Test value

There are no new committed tests. Each check that guards a gate, with the defect it catches:
- **Validator failed/audible/dispose checks** (`web-mixing-automation-benchmark.mjs:200-206`): a record that timed a refused or silent boot, which is fast because it returns early, would understate the rebuild cost. M1, reproduced above, shows such a record accepted once those checks are removed.
- **Validator round, document and observation checks** (`:170-199`): a record missing a round or a document, or with a short round, would publish a number from an incomplete workload.
- **Preflight audible and boot assertions** (`:317-325`): a document that cannot boot, or renders silence, under the frozen options would otherwise give fast, meaningless boot times. The zeroed-PCM and quantum-256 mutations turn them red.

## Findings

### MINOR

1. **The validator does not cross-check what `report.md` prints.**
   - **Where:** `scripts/web-mixing-automation-benchmark.mjs:168-217`.
   - **Problem:** `rebuildReport` (`:399-424`) prints `boot_p50_ns`, `boot_max_ns`, `dispose_p50_ns` and `peak_memory_bytes` straight from the record. The validator never recomputes them from `boot_ns`.
   - **Probes the validator accepts:** a record with p50 = 1 µs on the sends document, a record without `measurement_control`, a 64-track kind pointing at the nine-track fixture, and rounds that disagree on `document_sha256`.
   - **Impact here:** none. My recomputation shows the committed record is consistent, and the run's tracked-clean checks keep fixtures stable between rounds.
   - **Fix:** in `rebuildRefusalReasons`:
     - recompute `nearestRank(boot_ns, 50)`, max and min, and compare them with the record
     - require each `fixture_id` to equal the frozen `REBUILD_DOCUMENTS` entry
     - require `document_sha256` to agree across the two rounds
     - require a non-empty `measurement_control`

2. **The gate-2 refusal cases are not reproducible from the tree.**
   - **Problem:** the attempt record lists synthetic cases, but none is committed. The sibling browser arm's jq validator has committed cases in `scripts/test-console-benchmark.sh:1454+`. This mode has no `--self-test` and no fixture records.
   - **Why acceptable now:** `test-console-benchmark.sh` is not an authorized path, and the mode is one-shot.
   - **Fix:** if the mode is rerun, for example to re-measure after B2-B5, add a self-test in a slice that authorizes `test-console-benchmark.sh`. My 23 cases are in `/tmp/claude-1002/v1289/attempt1/.verifier/cases.py` and can be reused.

3. **The D2 raw record lacks its control and provenance facts.**
   - **Where:** `hosts/host-web/qualification/rebuild-cost.mjs:208-231`.
   - **Problem:** `chromium.json` has no loadavg, Playwright version or commit. The attempt record states "loadavg 25.2" and "Playwright 1.62.1", but only in prose. `report.md` does not mention D2 at all.
   - **Fix:** add `loadavg_start`/`loadavg_end`, the Playwright version and the candidate commit to the record.

### NIT

1. **A measured child's raw stdout can be lost.**
   - **Where:** `scripts/web-mixing-automation-benchmark.mjs:461`.
   - **Problem:** `JSON.parse(child.stdout)` throws before that stdout is written anywhere. AGENTS.md says to preserve raw output when post-workload tooling fails.
   - **Fix:** append each measured child's stdout to a raw file before parsing it, or catch the parse error and keep the stdout under the refused name.
   - **Impact here:** none. The child prints `JSON.stringify` output, and this run parsed it.

2. **Spec line 148 overstates dispose cost for one document.** It says "dispose is 0.2-0.6 ms", which holds for the 64-track documents. The nine-track dispose p50 is 0.03 ms.

3. **The umbrella comment should carry three caveats the record gives only implicitly.**
   - **The V8 proxy runs `--no-liftoff` (TurboFan-only).** Chromium uses dynamic tiering, so the proxy is a lower bound for the browser. D2's 2x warm and 6x cold figures show this.
   - **A boot is a proxy for a replacement, not the same thing.** It includes bridge allocation and memory growth on a fresh instance. It excludes building and running the carry program.
   - **The budget compared is one quantum (2.667 ms).** Headless Chromium reported `baseLatency` 11.6 ms. Even that margin is exceeded 2x-3x by every 64-track V8 p50.

4. **The refused launch's stderr is not in the tree.** `refused-launch-1/validator.json` says "see web-rebuild-cost.stderr.log", but `*.log` is gitignored, so the log is gone once the worktree is removed. The attempt record quotes the essential line, which is enough.

## Not verified

- **D2 invocation count:** I cannot tell from the artifacts whether the D2 probe was run only once. `chromium.json` refuses to overwrite itself, but an earlier run to another path would leave no trace.
- **The implementer's claim that a standalone `scripts/web-rebuild-cost.mjs` would fail `check-ci-path-routing.py`:** not tested. It is plausible: the `console-benchmark` router entry, `ci-path-router.py:64-76`, lists the runner's inputs exactly. It also does not matter, because the spec explicitly allows the alternative the implementer chose.
