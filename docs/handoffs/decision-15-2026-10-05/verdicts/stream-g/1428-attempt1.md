PASS

# #1428 attempt 1 -- adversarial verdict

Reviewed `git diff 4e41b6296 a124b40be` on `codex/d15-stream-g`. The range holds two commits:
`50103f0dc` (root: drops #1427 from STREAMS.md; not part of this attempt) and `a124b40be` (the
attempt: one `test-release` step plus the spec's attempt record). The attempt touches only its
authorized paths. Verified in a `git archive a124b40be` export at `/tmp/claude-1002/v1428/tree`
with a fresh `CARGO_TARGET_DIR`, toolchain 1.97.1, x86-64 (AMD EPYC 7313P, 32 threads).

## Findings

No BLOCKER. No MAJOR.

### MINOR

1. **The insertion breaks the next step's comment.** `.github/workflows/qualification.yml:672`
   says "#1044: the step above runs M3 (`m3_determinism`) with FMA on", and `:675-676` says "the
   cfg cargo gives that step's `math`". After the insertion, "the step above" is the new
   `filter_liveness` step (`:670-671`). That step runs no M3. The comment now points a reader at the
   wrong step for the M3/FMA claim that `check-ci-path-routing.py`'s #1044 owner rule defends. This
   is no change in behavior, but the slice introduced it. Fold one of these fixes:
   (a) Change "the step above" and "that step" to name the `Lane and math gates ... in release`
   step.
   (b) Move the new step down one place, after the M3 cfg step and before the loom step. This
   still meets D1's real constraint ("before the steps that set their own `CARGO_TARGET_DIR`"),
   but it changes D1's literal "directly after" text.
   Both fixes are inside the authorized paths.

2. **D2 can be checked, but its wording and remedy are weak** (spec `:82-86`, restated at
   `:170-171`).
   - **Checkable.** Per-step and per-job timings come from
     `gh api repos/misofm/engine/actions/jobs/<id>`. On the baseline run 37329888826,
     `test-release` took 254 s and its lane/math step took 203 s. The slowest jobs took 524 s and
     483 s.
   - **Trigger (b) refers to itself.** As written, "`test-release` exceeds the slowest required
     job of that run" can never be true, because `test-release` is one of the required jobs. It
     must say "exceeds every other required job" (that is, becomes the slowest).
   - **Trigger (a) has no stated reason.** The 240 s limit has no reason that stands apart from
     (b): the jobs run in parallel, and the 15-minute timeout is far away.
   - **The remedy goes against D0.** The remedy moves the step to `nightly.yml`, so a stability
     regression no longer blocks a merge. D0 calls such a regression a merge-blocking defect. At
     the time D2 fires there is a measured need, so a dedicated parallel required job would keep
     the sweep merge-blocking with no cost to the critical path. The cost of that job is one router
     `if:` and one verdict row. The owner's no-shortcuts rule points to that job, not to nightly.
   - **D2 does not fire this time.** Locally the step costs about 35 s, mostly in one thread. On a
     runner that is about 1.5x slower per thread, that gives about 55 s and puts `test-release` at
     about 310 s, well below 483/524 s. So the D2 problem does not change this attempt's
     deliverable. Root should fold the wording and decide whether to amend the D2 remedy before
     the batch run is measured.

### NIT (fold)

3. Spec `:127-128` (gate 1) still says "among 14 passed tests". The binary has 15 tests:
   gate 8 `every_lane_steps_exactly_four_words_from_its_own_countdown` was added. The attempt
   record (`:160-161`) states this correctly, but the gate text was not changed.
4. Spec `:121-122` (Hazards, hot file) still names "J #1427". `50103f0dc` removed #1427 from the
   STREAMS.md `.github/workflows/*.yml` row (it was closed as not planned).
5. Spec `:80-81` (D1) says "only `.github/ISSUE_SPECS/` and `docs/` paths route away from
   [full]". `scripts/ci-path-router.py` also routes `README.md` and `dsp-research/*.md` to evidence,
   and `sdk/` plus the SDK script set to `sdk`. None of these is an input of the sweep, so the
   conclusion holds, but the sentence is not exact.
6. Spec `:118` (Hazards) says "rust-cache ... then caches both". rust-cache removes workspace
   crates from the cache, so the cache never holds the `builtins` builds. Also, the new step
   compiles no third-party crate: after the lane/math step, only `math`, `lane`, `effect-contract`
   and `builtins` compiled. So the step has no effect on the cache at all. That is better than the
   hazard says, but the sentence is wrong.
7. Gates 2 and 4 name "the PR run". In CI-conscious batch mode, root can fast-forward `main`
   without a PR. Then the only qualification run is the `main` push. Word these gates as "the
   batch's qualification run (PR or `main` push)", as gate 1 already does ("PR or batch run").
8. Optional: the `test-release` display name ("release-mode lane, math, and wasm-gates digest
   gates") does not mention the builtins sweep. The authorized paths ("`test-release`'s steps
   only") correctly kept the implementer from changing it. Optional also:
   `check-ci-path-routing.py` pins no rule that `test-release` must run this step unconditionally.
   That is the same as for most test steps; only #1044's deduplicated owners are pinned. Not
   required.

## Judgments requested

- **Placement, cache, CARGO_TARGET_DIR.** The step is at `:670-671`, after the lane/math step and
  before the M3 cfg step and the loom step. The loom step sets `CARGO_TARGET_DIR=target/ci/loom`
  inline for its own command only, so the new step and the loom step do not share a target dir.
  The new step's units have their own feature hashes. After the new step, I re-ran the lane/math
  build: it compiled 0 crates. The M3 cfg step still prints `target_feature="fma"` (exit 0).
- **Router, verdict.** The step inherits `test-release`'s `if: needs.route.outputs.route == 'full'`
  and has no `if:` of its own. With the router (`--event pull_request --path ... --flags`), a
  change to any of these routes `full`: `crates/builtins/{src,tests}`, `crates/lane`, `crates/math`,
  `crates/dsp-reference`, `crates/engine`, `crates/effect-contract`, `Cargo.lock`,
  `.cargo/config.toml`, `rust-toolchain.toml` and `qualification.yml`. `tools/bench-support` also
  routes `full`, because any path that is not evidence or sdk is unknown, and unknown routes
  `full`. `cargo tree` shows that the sweep's workspace inputs are exactly builtins, engine,
  effect-contract, lane, math, bench-support and dsp-reference. The test reads no files. No job is
  added, so the `verdict` needs list (19 jobs) and its expectation table are unchanged. The table
  keys on job results, not on steps. The workflow has no `paths:`/`paths-ignore:`.
- **Coverage (release scale), instrumented.** I added counters to a copy of the sweep in the
  export, then restored the file and confirmed it is byte-identical to `a124b40be`.
  - Release: rates {44100, 48000, 88200, 96000}, quanta 1..=63 (63 values, 252 pairs), 5,544
    histories (252 x 22), 2,838,528 blocks (x 512) and 90,832,896 frames. These are exactly
    4 x 63 x 22 x 512 and 4 x 22 x 512 x 2016.
  - Debug (what `test-debug-b` runs): rate {48000}, 4 quanta, 88 histories and 45,056 blocks.
  - The workflow sets no debug-assertions override (no env, no `--config`, and the release
    profile has none), so CI selects the release branch.
  - Gate 8 has no profile switch. It runs the same at both profiles: 2 rates x
    (`BankWidth::ALL` = {Four, Eight} under the pinned avx2 x 2 modes + scalar) x 400 blocks. It
    passes in release in 0.01 s of test time.
- **Time (re-measured).**
  - After the lane/math step: the step's `--no-run` took 14.9 s wall (user 15.1 s, mostly
    single-threaded fat-LTO). The whole target took 19.9 s wall (15 passed, harness 19.73 s).
  - The sweep alone (`--exact`) took 19.8 s and printed 4 per-rate lines.
  - These agree with the implementer's 13.3 s and 19.8 s. No other job runs `filter_liveness` in
    release: nightly's `release-link-proof` is `--no-run`, and its other release legs name other
    tests. This confirms the spec's "no CI job catches today".
- **Open CI halves.** The attempt record states honestly that these are open until the batch run:
  gate 1's PR half, gate 2's PR-run step time, the `test-release` job time and the slowest job, and
  gate 4.

## Test value (no new or rewritten test; the value is the CI step)

A change that pushes a reachable recursion word of the live input filter past its history's
design norm plus the proven allowance at 44.1, 88.2 or 96 kHz, or at any quantum in 3..=62 other
than 7, now turns `test-release` red. Today no required job catches it: `test-debug-b` reaches only
4 of the 252 (rate, quantum) pairs, and no job runs the target in release. I proved this reach with
instrumented iteration counts. I did not try a product-code mutation that only the release scale
can catch, because the slice adds no test.

## Gates run (export at a124b40be)

- Gate 1 (local): `cargo test --locked --release -p builtins --features builtins/test-support
  --test filter_liveness` exits 0, with 15 passed. The sweep line `... ok` is present.
- Gate 2 (local half): see "Time". The PR half is open.
- Gate 3: these all exit 0.
  - `python3 -B scripts/check-ci-path-routing.py`
  - `python3 -B scripts/test-ci-path-routing.py`
  - `python3 -B scripts/check-test-support-ci.py` ("10 packages")
  - `python3 -B scripts/test-test-support-ci.py`
  - `bash scripts/check-workspace-policy.sh` ("ok")
  - `python3 -B scripts/test-script-reachability.py` (20 cases)
  - `bash scripts/check-artifact-evidence-leak.sh`

  The last two needed `git init` in the export.
- YAML lint: actionlint is not installed. A Python check with a loader that refuses duplicate
  keys found these, all clean:
  - no duplicate keys
  - no unknown step keys
  - no step that has both `run` and `uses`
  - every `needs.route.outputs.*` reference resolves
  - no tabs and no trailing whitespace
  - no `paths` filters
- Gate 4: open (CI).
- GitHub: #1428 is OPEN. Its title matches, and its body is byte-equal to the spec at `a124b40be`.
