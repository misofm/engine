PASS

# #1457 attempt 3: adversarial verdict

Commit reviewed: `0b3957808` (branch `codex/d15-stream-g2`), against its parent `b74d8c69c`, and the
cumulative #1457 change in its paths and spec files. I exported `0b3957808` to
`/tmp/claude-1002/v1457c/tree` and built it with `CARGO_TARGET_DIR=/tmp/claude-1002/v1457c/target`
and `CARGO_INCREMENTAL=0`. I did not edit, build or check out anything in the worktree. The box is
the record's stand-in for the CI-class runner (AMD EPYC 7313P, x86-64-v3, release, `taskset -c 7`).
Another session's headless Chromium loaded the box in bursts (load average up to 18). So I ran
every timing that I report in a quiet window: load average below 3 and the whole box below 8 %
busy, checked before each run (`quiet.sh`). Evidence: `/tmp/claude-1002/v1457c/evidence/`.

## Summary

Attempt 2's MAJOR is fixed:
- The frame-equivalent is now one frame of the slowest frame class, 23.5 ns.
- The budget stays 1,510,000 frame-equivalents, which is now 35.5 ms.
- The band families are in the committed gate-2 workload.
- The stated worst case (34.90 ms) reproduces: my one gate-2 invocation gives 34.40 ms.

My own search did not find a family that is slower per frame-equivalent than the band. It covered
about 37,000 designs: HPF and LPF cutoffs, trims from -144 to +24 dB, one or two sections, one or
two cascades, all four rates, and short and long walks. I re-measured every outlier. Each one was
measurement noise, and the families built from them take 23-28 ms, against 33-34 ms for the band
in the same runs. So 34.9 ms is the worst case for every family that I could build, and it is
inside the 35.5 ms budget.

The other rulings are done:
- `INPUT_BOUND_DESIGN_CHARGE` is gone.
- The horizon-sweep test catches X3, X5, X7 and X9. X7 turns only that test red.
- Gate 8 stays exact. Its boundary is sharp: a section charge of 1,000 turns it red and 999 does
  not; a budget of 1,328,255 turns it red and 1,328,256 does not.
- #1474 is filed, synchronized and ordered correctly.
- #1471 owns the first-touch measurement.

Every gate passes. Two MINORs and three NITs remain:
- the soundness prescription in #1474's spec is not precise enough;
- the specs of #1468 and #1470 are stale.

I rule that the record's gate-2 figure is not tainted: my independent invocation reproduces it to
within 0.5 ms.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. #1474's certified-flush requirement prescribes a treatment that is sound only for
non-negative majorants, and it does not state the rounding-model condition that the proof needs.**
Location: `.github/ISSUE_SPECS/1474-remove-the-data-dependent-cost-of-the-near-top-tail-walk.md:47-54`
(Scope item 2) and gate 5 (`:74-75`).

1. **The prescription does not fit the signed state.** The spec says: "Where a component that the
   walk propagates falls below a stated tiny bound `tau`, round it **up** to `tau` (never down,
   never to zero)". It justifies this with "a larger non-negative majorant is still a majorant
   under the non-negative propagation". Gate 5 then requires a red result "when the flush rounds
   down or to zero". That argument holds for the error radius (`Majorants::first_error`), for the
   later sections' majorants, and for the `Deviation` components. All of these are non-negative,
   and a non-negative map propagates them.

   The cited evidence also names the first section's state (attempt-2
   `probe-subnormal48.log`: "first-state" is subnormal on about 450,000 frames at 16 kHz and
   17.28 kHz at 48 kHz). That state is `Majorants::first` (`crates/math/src/tail.rs:1815-1818`, the `self.first` update),
   and it is signed:
   - The signed matrix `A` of the section propagates it.
   - "Rounding it up" to `tau` is not defined, and making it larger does not bound anything.
   - The sound treatment is to flush the signed state to zero and add its magnitude to
     `first_error`. The spec forbids exactly that ("never to zero"), and gate 5 would turn it red.

   The spec must give the treatment per quantity: radii and non-negative majorants are rounded
   up; the signed state is flushed to zero and its magnitude goes into the radius. Root's
   Amendment 4 wording ("a radius rounded up to a stated tiny bound") is correct for radii. The
   spec generalised it to every component.
2. **The proof needs a normal-range condition.** Each step of the walk is an upper bound only
   under a relative rounding model:
   - `STEP_UP` (`tail.rs:94-107`: "every summand is at least `(1 - u)^3 (1 + 4u) >= 1` times its
     exact value");
   - `nu = 8 u ||R||` for the radius of the first state.

   That model does not hold when a result underflows. A subnormal product has an absolute error,
   and it can round down to zero. So the proof that #1474 asks for must do one of these:
   - keep `tau`, and every product that a floored or flushed quantity enters, in the normal
     range;
   - carry an explicit absolute underflow term.

   The spec does not say so. Today's band frames already run on subnormal majorants, so the walk
   already operates outside the model that its doc states (see the observations). The size is
   negligible: each rounding can lose less than 2^-1075, against thresholds near 1e-9. But the
   bound is called "certified".

**m2. The specs of #1468 and #1470 still name the removed constant and the old figure.**
Locations:
- `.github/ISSUE_SPECS/1468-tighten-the-fixed-input-section-s-peak-gain-past-the-cascade-triangle-inequality.md:150`
  ("restates `builtins::INPUT_BOUND_DESIGN_CHARGE` and `INPUT_BOUND_SECTION_CHARGE`");
- `.github/ISSUE_SPECS/1470-cache-design-bounds-across-browser-preparations-in-the-control-worker.md:28`
  ("about 20 ms of near-top design work") and `:43` (`INPUT_BOUND_DESIGN_CHARGE`).

A successor that follows #1468 literally would restate a constant that no longer exists, or add it
again. These paths were not named for this attempt, and the record reports them correctly as an
open item ("Open items for root"). Root must authorize the edit in the batch follow-ups.

## NIT

- **n1. Two specs cite scratch evidence in `/tmp`.**
  - #1474 cites `/tmp/claude-1002/v1457b/evidence/probe-subnormal48.log` (`:28`).
  - #1471 cites `probe-order.log` and `probe-order-notrim.log` (`:99`).

  `/tmp` does not survive a reboot. Cite the attempt-2 verdict instead
  (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-g/` after the batch copy), which states
  the same figures.
- **n2. `crates/builtins/tests/tail_contract.rs` has two leftovers.**
  - `:465` ends with the orphaned fragment "of the budget: a", which is left from removing the
    design charge. Join it with the next line.
  - `:522` `let fixed = cascade;` is now an alias with no purpose.
- **n3. The calibration's "slowest frame class" is the maximum of single-shot samples, so noise
  can move it in either direction.**
  - Location: `crates/builtins/examples/input_bound_budget.rs:365-410` (`frame_classes`: one
    `one_design` time per point and round; `slowest = slowest.max(high)`).
  - My one quiet invocation reported 24.54 ns once (48 kHz, round 1, 0 dB) and 23.53 ns once
    (96 kHz, round 2, +24 dB). Both are above the committed 23.5 ns. The 24.54 ns row also widened
    the band to 0.71-0.72, which is a sign of interference.
  - Robust measurements put the band at 22.5-23.1 ns: best of 3 over 0.001 steps, best of 7
    interleaved with the reference, and the families (22.2-22.8 ns per charged frame-equivalent).
    The constant is therefore right.
  - But #1474 and #1465 restate the frame-equivalent from this tool. They should take a per-point
    minimum over rounds, or confirm the value against gate 2's band families.
  - In the same run, one round's per-design term came out positive (+1.97 us at 44.1 kHz,
    round 1, from a noisy 18 ns/frame slope). The section charge still covered every class's
    fixed cost: at most 6.76 us a section = 287.7 frame-equivalents, against a charge of 290. So
    `tail.rs`'s "the per-design term is negative at every rate" is true of the record's run, not
    of every run. It is not a defect.

## Observations for root (not findings)

- **The subnormal state is a limit cycle that is not certified.** The first section's state is
  computed in `f64` by a signed contraction. In the band it apparently never decays to zero: the
  attempt-2 counts show it subnormal on about 450,000 frames. A rounded contraction can do this in
  subnormal arithmetic. Its error radius (`nu (|s_1| + |s_2|)`) is relative, so it does not cover
  the absolute rounding there. The effect on any bound is below 1e-300 and does not matter. But
  #1474 is the natural place to make the walk's rounding argument hold over the whole range (m1).
- **The nightly native vectorization report** (`run-native-vectorization-report.sh`, then
  `test-native-vectorization-report.sh target/release/audit`) fails locally with
  "recursive-svf / probe_svf_simd8: forbidden call 'call|callq' is present". The parent
  `b74d8c69c` fails in the same way, so this is not from #1457. The job is nightly and
  `continue-on-error`. No kernel code changed here.
- **Whole-preparation outliers.** My gate-2 run has single-round first preparations of 42.81 ms
  (4,096 band designs, 44.1 kHz) and 40.48 ms (64 band designs, 48 kHz). In the same rounds, the
  no-cache preparations took 35.56 ms and 34.42 ms. This matches the record's candid note (two
  such outliers) and the first-touch explanation that #1471 now owns. The design-bound work in
  those rounds is inside the budget.

## Root's rulings: status

| ruling | status |
|---|---|
| D1(a): the frame-equivalent from the slowest class; band family in the gate-2 probe; worst case stated as the measured maximum with spread and cause; gate 8 exact | Done. 23.5 ns, two band families, 34.90 ms with spread and per-frame cause, gate 8 exact at every rate (96 kHz margin 181,744) |
| (b) refused | `INPUT_BOUND_BUDGET_FRAMES` = 1,510,000 |
| (c) #1474, ordered before #1465 | Filed. The GitHub body equals the local spec. STREAMS row 24 ("after #1457"); #1465 row 37 depends on #1474; the hot-file row "G #1474 ... before #1465, in either order with #1464"; #1465's Dependencies name it. Self-contained apart from m1 and n1 |
| `INPUT_BOUND_DESIGN_CHARGE` removed | Done: the code, the re-export, the tests and #1465/#1471. #1468 and #1470 remain (m2) |
| m2: horizon-sweep test | Done. X3, X5, X7 and X9 are red; X7 is red only in this test |
| n1: first-touch moves to #1471 | Done (#1471 gate 4) |
| n2-n4 | Done. The example doc names the real constants; the long line is wrapped; the measurement history is stated (one calibration, one gate 2) |
| Taint ruling | Not tainted (see gate 2 below) |

## Gate 2 and the adversarial search

**Gate 2: committed `input_bound_budget`, one invocation, `taskset -c 7`.** Load average 1.97
before and 1.82 after; per-core use was 0-7 % just before. Log: `gate2.log`. Ranges over rounds 1
and 2, design work in ms (no cache less the warm rebuild):

| family | mine | record |
|---|---|---|
| 64 band designs | 33.78-34.40 | 33.71-34.90 |
| 4,096 band designs | 33.48-34.34 | 33.97-34.40 |
| 64-design near-top family | 18.43-19.68 | 18.58-19.34 |
| 65,537 distinct near-top designs | 18.27-18.96 (first 53.55-54.65) | 17.05-19.85 (first 52.99-63.18) |
| top pair / LPF at the maximum | 6.37-6.72 / 3.28-3.49 | 6.42-6.67 / 3.32-3.42 |
| 256 typical | 20.11-21.41 | 20.32-21.45 |
| two-section cheap families | 20.25-29.07 | 20.63-29.54 |
| one-section / two-cascade cheap | 16.94-17.21 / 16.75-17.01 | 16.95-18.87 / 16.95-29.07 |

The five slowest design-work rows are all band rows, 34.23-34.40 ms. The example's own assertions
held: identical results first, on rebuild and with no cache; the rebuild walks nothing; frames
walked never exceed the budget.

**Adversarial search.** I used my probe `probe1457c.rs` (in the evidence directory, not committed),
the production `input_section_bounds` and the production budget. Each single-design time is a
one-strip computation with no cache. ns/fe is the time divided by the charge.

1. **Near-top long walks** (`probe-near-top2.log`, quiet, best of 3). 3,152 designs per rate:
   - HPF off, 0.30-0.99 of the maximum in 0.01 steps, or one `f32` below the maximum;
   - into an LPF 0, 1, 2, 4, 16, 64, 256, 1,024, 4,096, 16,384 or 65,536 `f32` steps below the
     maximum;
   - and HPF-only designs near the maximum;
   - trims +24, 0, -60 and -144 dB.

   Every walk of 100,000 frames or more is at 22.2 ns/fe or below, except 3 single samples (25.3,
   24.5 and 27.9). These re-measured at 15.2-17.1 ns/fe (`probe-cmp-verify.log`, best of 7,
   interleaved with the band reference).
2. **Short and medium walks** (`probe-short.log`, quiet, best of 5). 5,570 designs per rate with a
   charge of at most 200,000:
   - a 40-point geometric grid from 10 Hz to the maximum;
   - every HPF/LPF pair in both orders, and single sections;
   - split designs of two two-section cascades;
   - six trims.

   The outliers came in rows: 28-32 ns/fe for the 235 Hz HPF at 44.1 kHz, the 621 Hz HPF at
   96 kHz and the 89.8 Hz HPF at 48 kHz, all at +12 dB. They re-measured at 16-19 ns/fe. Their
   4,096-design families, alternated with the 64 band designs (`probe-short-verify.log`), take
   23.4-27.5 ms against 33.0-34.4 ms for the band.
3. **The band at fine resolution** (`probe-fine.log`, quiet, best of 3). The HPF from 0.650 to
   0.920 of the maximum in 0.001 steps, into the LPF at the maximum, at +24 and 0 dB, and split
   band pairs: 552 designs per rate. The maximum is 22.95 ns/fe. One sample of 25.73 re-measured
   at 22.54 ns/fe.
4. **Families of the slow short class** from the first, loaded scan (`probe-short-families.log`;
   HPF at 0.66 of the maximum and its neighbours): 21.3-28.3 ms.
5. **The trim-0 band at the LPF maximum** (`probe-fcmp48.log`): 33.5 ms in a quiet round, the
   same as the band. Its loaded rounds (36-54 ms) track the reference's loaded rounds.

The loaded scans (`probe-long.log`, `probe-near-top.log`) were superseded by the quiet ones, and I
do not use their figures.

**My calibration**: `input_bound_budget calibrate`, one invocation in a quiet window
(`calibrate.log`).
- Band frames: 22.80-23.53 ns, plus one single-shot 24.54 ns (n3).
- The largest fixed cost per section: 13.52 / 2 = 6.76 us = 287.7 frame-equivalents of 23.5 ns,
  so 290 covers it.

## Test value (one sentence per new or changed test)

- **New** `math::tail::tests::a_walk_takes_at_most_its_horizon_and_finishes_exactly_when_the_horizon_covers_it`:
  - It is red when the replay of the crossing block counts frames without stopping at its horizon
    (X7). That defect turns no other test red in the math lib, `tail_contract`, the builtins lib,
    the builtins-compiler lib or the host-core lib.
  - It is also red on X3, X5 and X9, which other tests catch as well.
  - Its design words are the exact Butterworth `f32` words of a 300 Hz LPF at 44.1 kHz. I checked
    them against `SvfSection::design`'s formula.
  - It runs in test-debug-b and in test-release.
- **Changed** (design-charge terms removed) `tail_contract::a_design_walks_at_most_the_budget_it_is_given`
  and `builtins_compiler::tests::a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing`:
  both are still red on C1 and C3. Their unique catches (C4/C8 and M4/M8) are unchanged by the
  edit.
- **Unchanged** gate 8 `every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`,
  under the new charge:
  - It is red, and only it, at a section charge of 1,100 or 1,000, or at a budget of 1,300,000 or
    1,328,255.
  - It is green at 999 and at 1,328,256.
  - So it holds the 96 kHz stereo charge (1,328,256) exactly.

## Mutation runs

How I ran them:
- Each mutation was applied alone to a second export and then restored from a pristine copy.
- Suites, all with `--no-fail-fast`: the math lib (`--features lane`), `tail_contract`, the
  builtins lib, the builtins-compiler lib and the host-core lib.
- Profile: release with `test-support`, with LTO off.
- The baseline is green in every suite. Logs: `evidence/mut/`.

| id | mutation | red |
|---|---|---|
| X3 | majorant pass counts, does not stop | new math test, `a_design_walks_at_most_the_budget_it_is_given`, gate 7 |
| X5 | deviation walk counts, does not stop | new math test, `a_design_walks...`, gate 5 |
| X7 | replay counts, does not stop | **new math test only** |
| X9 | `take_frame` lets one frame past (`>`) | new math test, `a_design_walks...`, gate 5, gate 7 |
| C1 | sections' charge not reserved before the walk | `a_design_walks...`, gate 5, gate 7 |
| C3 | charge omits the section charge | `a_design_walks...`, gate 5, gate 7 |
| C6 / C6e | section charge 1,100 / 1,000 | gate 8 only |
| C6g | section charge 999 | none (boundary) |
| C7 / C7e | budget 1,300,000 / 1,328,255 | gate 8 only |
| C7g | budget 1,328,256 | none (boundary) |
| C10 | section charge 0 | gate 5 only |

Every row that the record states (X3, X5, X7, X9, C1, C3, C6, C7) reproduces.

## Gates run

All gates ran on the `0b3957808` export, x86-64, with `CARGO_INCREMENTAL=0`.

| gate | result |
|---|---|
| Gate 2: committed `input_bound_budget`, one invocation | ok; see above |
| Calibration, one invocation | reproduces (n3) |
| Gates 5, 6, 7 (`tail_contract` with `test-support`, debug 12 passed + 1 release-only ignored; release 13 passed) and gate 7 at the compiler level | ok |
| Gate 8, the spec's command with `--nocapture` | ok; every figure equals the record (96 kHz charged 1,328,256, margin 181,744) |
| Gate 3 (host-core `a_design_bound_cache_changes_no_prepared_value`) | ok, in test-debug-a |
| `cargo fmt --all -- --check`; `cargo test -p builtins-compiler --no-run` | ok |
| test-debug-a workspace step (exact `--exclude` and `--features`) | 1,465 passed |
| test-debug-a doctests | ok |
| test-debug-b DSP step and doctests; `conformance_fixtures -- --check` | ok |
| Release `-p lane -p math -p wasm-gates --features math/lane` (includes the new math test) | ok |
| Release `filter_liveness`; `tail_contract --no-run` without `test-support` | ok |
| `cargo clippy --workspace --all-targets -D warnings`, with and without `--all-features` | clean |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | clean |
| `audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations; `pcm_digest` `cb10fbface44a3a4` (unchanged) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration` (with its callgraph and boot-budget checks), `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm` and `--self-test`, `strip-wasm-names.py`, `check-web-audioworklet-v8-spill.py` and `--self-test`, `test-web-audioworklet.sh` (empty `TMPDIR` afterwards) | ok |
| SDK: `npm ci`, `check-sdk-generated.sh`, `check-sdk-deletions.py` and `--self-test`, `check-sdk-types.sh`, `check-sdk-headless.sh`, `sdk-package.sh check` | ok |
| `run-wasm-gates.sh --without-v8-spill --without-native` | ok |
| `check-cross-targets.sh` | ok |
| `check-capi-abi.sh` and `--self-test`, then `check-scalar-oracle-absent.py --native target/release/libcapi.so` | ok |
| `check-graph-determinism.sh`, `check-builtins-fixtures.sh`, `check-console-fixtures.sh`, `check-effect-contract.sh`, `check-protocol-wasm-parity.sh`, `run-protocol-allocation-audit.sh`, the three `test-realtime-audit-probes.sh` legs | ok |
| Every `scripts/check-*.sh` and its `scripts/test-*.sh` (note 1) | ok |
| Python checks and self-tests (note 2) | ok |
| Node checks and self-tests (note 3) | ok |
| `test-native-vectorization-report.sh` (nightly, `continue-on-error`) | fails, the same as on the parent `b74d8c69c` (see the observations) |
| AArch64 | runs only in CI |

Notes:
1. The shell scripts: workspace-policy, session-policy, bench-policy, host-core-policy,
   protocol-control-policy, realtime-policy, realtime-audit-leak, artifact-evidence-leak,
   lane-policy, rack-policy, builtins-policy, graph-policy, effect-runtime-policy,
   effect-runtime-fixtures, env-vocabulary, conformance-boundaries, console-benchmark-fixture,
   bench-preconditions, unfused-seal and `--self-test`, parametric-eq-render-contract,
   dsp-research, builtins-listening; and the self-tests test-builtins-fixtures,
   test-realtime-trace-validator, test-console-benchmark, test-dsp-research, test-gate-lib and
   test-sdk-artifact-builder-output-contract.
2. The Python scripts: ci-path-routing and its test, test-support-ci and its test,
   script-reachability and its test, release-shape and `--self-test`, test-npm-publish-modes,
   session-map-shape, the command-kind and command-reason vocabularies, `--self-test` of
   abi-layout-v1, parameter-metadata-v1, web-audioworklet-identity and
   check-web-audioworklet-callgraph. The two reachability scripts need `git ls-files`, so they ran
   in a throwaway git copy of the export.
3. The Node scripts: `check-stem-store-v1.mjs` and `--self-test --budgets`,
   `test-parse-npm-trust-list.mjs` and `test-prepared-control.mjs`.

GitHub (read only): #1457, #1465, #1470, #1471 and #1474 are open, and their titles match the
specs. #1474's body equals its local spec.
