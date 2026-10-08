FAIL

# #1457 attempt 2: adversarial verdict

Commit reviewed: `b74d8c69c` (branch `codex/d15-stream-g2`), cumulative against `5dc6f96e9` in #1457's
paths and spec files. The commits between them that belong to other slices (#1469, #1462, #1461,
#1454, #1459, #1458, #1456, #1470) are not judged here. I exported `b74d8c69c` to
`/tmp/claude-1002/v1457b/tree` and built it with `CARGO_TARGET_DIR=/tmp/claude-1002/v1457b/target`
and `CARGO_INCREMENTAL=0`. I did not edit, build or check out anything in the worktree. The box
is the record's stand-in for the CI-class runner (AMD EPYC 7313P, x86-64-v3, release, `taskset -c
7`). Evidence: `/tmp/claude-1002/v1457b/evidence/`.

## Summary

MJ2 is fixed and well tested. The charge mechanism is correct, and every value is still a pure
function of the session. m1-m5 and n1-n5 are done. The attempt fails on one MAJOR: the stated
worst case (22.7 ms) is again false, by about 1.5x. One band of near-top designs walks its frames
at about 1.7 times the cost that a frame-equivalent assumes. A session of these designs spends
33.6-34.0 ms of design-bound work under the production budget at 44.1, 48 and 96 kHz. The
committed gate-2 workload has no family in that band, so it cannot see this. The fix needs a root
ruling (D1).

## BLOCKER

None.

## MAJOR

**MJ1. The stated worst case is false: one band of near-top designs takes 33.6-34.0 ms under the
budget, against the stated 22.7 ms. The frame-equivalent is not a constant cost.**

Locations:
- `crates/builtins/src/tail.rs:289-292` says "A frame-equivalent is the time of one near-top frame
  on the CI-class runner, `1 / 75,500` ms".
- `crates/builtins/src/tail.rs:301-307` says the budget is "20 ms of near-top design work" and
  "bounds the real work of every design family measured".
- Spec lines 345-347 (Amendment 3's definition of the frame-equivalent).
- Spec lines 576-579 ("What the charge does not model" names only short two-section walks).
- Spec lines 606-611 (stated worst case 22.7 ms; whole preparation of up to 4,096 strips at most
  25.6 ms).
- `crates/builtins/examples/input_bound_budget.rs:100-174` (`FAMILIES` has no family in the band).
- `input_bound_budget.rs:267` (the calibration fits only walks of at most 16,384 frames, so it
  leaves out every long near-top walk).

**What I measured.** I used my probe (`evidence/probe1457b.rs`), the production
`input_section_bounds` and the production budget, on the same box and core. Each family is 4,096
distinct designs, made by stepping the HPF by `f32` ulps. Each design is an HPF at about 0.78 of
the maximum cutoff into the LPF at the maximum, +24 dB. Rounds 1 and 2, in ms:

| rate | HPF | design work (no cache less rebuild) | first preparation | frames walked | exact |
|---|---|---|---|---|---|
| 44.1 kHz | 17,154.5 Hz | **33.65-33.98** | 36.15-36.81 | 1,505,840 | 3 / 4,096 |
| 48 kHz | 18,671.6 Hz | **33.66-34.04** | 36.09-36.40 | 1,505,840 | 3 / 4,096 |
| 88.2 kHz | 34,838.2 Hz | 26.78-27.52 | 29.38-30.01 | 1,505,840 | 3 / 4,096 |
| 96 kHz | 37,343.1 Hz | **33.92-33.97** | 35.63-35.66 | 1,505,840 | 3 / 4,096 |

- A 64-design session in the same band takes 33.70-34.03 ms (44.1 and 96 kHz).
- The same band at 0 dB or -24 dB trim takes 32.8-33.6 ms (48 kHz).
- The record's own 64-design near-top family walks exactly the same 1,505,840 frames in
  18.5-19.1 ms.
- My one invocation of the committed gate 2 reproduces the record: its largest design work is
  22.35 ms, against the record's 22.72 ms. The record's numbers are honest. Its families miss the
  band.
- Logs: `evidence/probe-families-band.log`, `probe-families.log` and `gate2.log`.

**The cause is the walk, not the fixed charge.**
- Single designs at 48 kHz, +24 dB, LPF at the maximum, best of 7: every design walks
  449,000-456,000 frames. Almost all of them are majorant-pass frames, with the same phase split.
- The time per frame is 12.0-14.7 ns for most HPF cutoffs, and 21-23 ns for HPF cutoffs in a
  band of about 0.74-0.85 of the maximum. Every launch rate has this band
  (`evidence/probe-hpf-sweep-all.log`, `probe-one48.log`, `probe-phases48.log`).
- So a near-top frame costs anywhere from 12 to 23 ns. "One near-top frame, 1 / 75,500 ms" is not
  a fixed unit: in the band the budget buys 1.7 times the reference time.

**Likely cause, not proven.** On an instrumented copy (`evidence/probe-subnormal48.log`), the
first section's state or its error radius is subnormal on about 449,000 of about 450,000
majorant frames in the band. It apparently never decays to zero. Outside the band (12 kHz, the top
cutoff), fewer than 50 frames are subnormal. Presence alone does not predict every point: 17.28 kHz
is subnormal throughout but ran at 12.9 ns in the clean build. `perf` counters are not available to
this user (`perf_event_paranoid` = 4).

If the cause is subnormal operands, the cost depends on the CPU. A core with a larger subnormal
penalty than this AMD Zen 3 part could be slower still. The browser runs the same arithmetic, since
Wasm has IEEE subnormals. I did not measure any other CPU.

**Why this is MAJOR.** The issue's product outcome is "the worst-case preparation cost of the
bounds is a stated, gated figure". Amendment 3 says the charge exists "so the budget measures the
real work and the stated ms is a true bound". The stated figure is wrong by 1.5x (34 ms against
22.7 ms). This is the same defect class as attempt 1's MJ1, at 1.5x instead of 2x.

#1465-#1468 now rerun "#1457's gates 2 and 8" with the committed workload. That workload has no
family in the band, so a successor that makes the band worse stays green.

**Root must decide (D1).** The budget constant does not change without a ruling. The options:
- **(a) Restate.** Restate the worst case at the band's measured figure (about 34 ms of design work
  and about 37 ms for a whole preparation of 4,096 strips, on this box). Define the
  frame-equivalent from the slowest measured frame, not the near-top reference.
- **(b) Lower the budget** to about 20 ms at the slowest frame (about 870,000-890,000
  frame-equivalents). This breaks gate 8: the 64-track documents charge 1,290,880 at 88.2 kHz and
  1,387,136 at 96 kHz. So D1's purpose would have to be restated as well.
- **(c) Remove the data-dependent cost** from the majorant pass. If the cause is subnormal
  operands, this means a sound treatment of components that can no longer reach the threshold. It
  may change bound bits; it is a successor's work, perhaps #1465's.

Under every option:
- Add a band family to `input_bound_budget.rs`. For example, HPF at 0.78 of the maximum into the
  LPF at it, +24 dB, at each rate.
- Widen the calibration or the stated reference so that long near-top walks are in it.

## MINOR

**m1. `INPUT_BOUND_DESIGN_CHARGE = 0` is dead arithmetic that no test can see** (`tail.rs:241`,
`:267`, `:283`). This is the implementer's point (b).
- The reservation (`checked_sub`) and the addition to `charge` do nothing at 0.
- Any defect in them is invisible. If a successor's recalibration makes the constant non-zero, the
  path goes live untested.
- My recalibration confirms that the per-design term is negative at every rate: -2.05 to -2.94 µs
  (`evidence/calibrate.log`). So the measurement says a design has no fixed cost beyond its
  sections'.
- Under the owner principle (no placeholders), root should rule one of these:
  - remove the constant and its code, and state that the fixed cost is per section; or
  - keep it, with a rule that a successor that sets it above zero adds the test that sees it.
- Root's MJ1(b) ruling named a per-design constant. That is why I do not count it as a defect of
  the attempt.

**m2. The replay loop's horizon cap is tested by nothing (X7).** Location:
`crates/math/src/tail.rs:2230-2233`.
- X7 makes the replay of the crossing block count frames without stopping. It turns no test red in
  `tail_contract`, the builtins lib, the builtins-compiler lib or the host-core lib.
- The overrun is bounded: at most `BLOCK` = 256 frames. The code is correct; I read the cap.
- But the record's test-value sentence says the new test is "red when a walk loop counts frames
  without stopping at its horizon". Amendment 3's gate 7 says "counted frame by frame by the walk
  itself". The test's budgets never stop a design inside the replay.
- A math-level test fixes this. Sweep `horizon` over `0..=frames` for one cheap design (about 513
  frames). Assert `walk.frames <= horizon`, and `result.is_none()` exactly when
  `horizon < frames`. That one test turns X3, X5, X7 and X9 red, and costs almost nothing.

## NIT

- **n1. Point (d) is not a cache cost.** Spec lines 613-616 say the 65,537-strip first preparation
  takes 11-12 ms more "than the same preparation without a cache". I ran the preparations in other
  orders (`evidence/probe-order.log`):
  - no cache first: 52.5-54.7 ms; cold cache next: 52.5-54.2 ms; warm cache: 34-35 ms;
  - with glibc heap trimming off (`probe-order-notrim.log`): cold = no cache = 39 ms, warm 20 ms;
    only the first preparation in the process pays about 54 ms.

  So the extra is allocator first-touch of the preparation's own memory, as the record guessed. It
  is not the cache. A host's first preparation pays it too. The 22.6 ms keying figure holds only on
  warm heap memory; with fresh memory the keying is about 34 ms. Restate the sentence.
- **n2.** `input_bound_budget.rs:24-26` names `INPUT_BOUND_CASCADE_CHARGE`, which does not exist,
  and says "the cascade and section constants". The constants are the design and section charges.
- **n3.** `crates/builtins/src/tail.rs:172` is 122 columns (a doc line that rustfmt does not wrap).
- **n4. Point (c): measurement process.**
  - The calibration is a different measurement from the descriptive gate 2. Root's MJ1(b) ruling
    requires it to set constants. Its workload is committed, and the AGENTS.md
    "descriptive, do not tune" rule does not apply to it.
  - Three of gate 2's runs measured changed code (section charges 640, 500, 520). The 500-to-520
    change came from the calibration's 13.71 µs, not from a gate-2 timing, so nothing was tuned
    against gate 2.
  - The one true retry was the loaded-box run. It is disclosed and was not used. My single
    independent invocation reproduces the reported gate-2 figures, so the reruns did not bias the
    record.
  - I find no breach of substance. The record should say how many times the calibration itself
    ran (the "clean calibration" implies a discarded loaded one). Successors should run each once.

## Observations for root (not findings)

- **(e) #1471's seeding.** #1471's spec correctly states the conflict. D1 seeds each session with a
  copy of the engine's cache after the compile. That fails gate 1 when the compile's insertions
  clear the engine's cache. The spec defers this to root before implementation. One way to resolve
  it: seed from the compile's own designs. `input_section_bounds_within` already builds a
  per-preparation `designs` map, but it keeps no charges.
- **Gate 8 headroom.** At 96 kHz the four stereo 64-track documents use 1,387,136 of 1,510,000
  (122,864 left, 92 %). The section charge took 133,120 of the margin. Option (b) of MJ1 conflicts
  with D1's purpose.
- **Purity, re-checked with the charges.** Every stop records a true `ChargeAbove(R)`: a design
  whose sections' charge passes `R` stops before it walks, and a walk stopped at horizon
  `R - fixed` has `charge > R`. The right channel gets `R - left.charge`. A `Bound` hit is admitted
  on `charge <= remaining`, which is exactly when a walk finishes. Gate 5's caches, warmed under
  other budgets, hold this. The `designs` map, the memoryless shortcut and the stop that spends the
  remaining budget are unchanged from attempt 1. My gate-2 run asserts that the first, rebuild and
  no-cache results are identical for every family. My band families assert it too.
- **Cache memory** reproduces exactly: 8,192 one-section designs at 48 kHz retain 2,486,520 bytes,
  303.5 bytes an entry (`evidence/probe-memory.log`).
- **Calibration** reproduces (`evidence/calibrate.log`, one invocation):
  - per section 6.93-8.31 µs, per design -2.05 to -2.94 µs;
  - the largest per-section cost is 13.68 / 2 = 6.84 µs = 516 frame-equivalents, so 520 covers it.

## Attempt-1 findings: status

| finding | status |
|---|---|
| MJ1 | Ruled (b) and implemented, but the stated worst case is false again (MJ1 above) |
| MJ2 | Fixed. `CascadeWalk { result, frames }`; the counter adds real frames; X3 is red in two tests |
| m1 | Fixed. The host-core test's catch is X1, unique |
| m2 | Fixed. The `tails` field doc and the module doc |
| m3 | Fixed. The cache-cap doc and the memory figure; #1471 records it |
| m4 | Fixed. Amendment 1 and #1465-#1468 say "gates 2 and 8" with commands; the gate-2 workload is committed |
| m5 | Fixed. `input_section_bounds_within` is private; the gates use `test_support::input_section_bounds_within` |
| n1-n5 | Fixed |

## Test value (one sentence per new or changed test)

- **New** `tail_contract::a_design_walks_at_most_the_budget_it_is_given`: red when the majorant
  pass or the deviation walk counts frames without stopping (X3, X5), or when a walk takes one
  frame past its horizon (X9). Red, and only it, when the right channel of a two-cascade design
  gets the whole budget (C4) or its charge is dropped (C8). It is blind to the replay loop (X7, m2).
- **Changed** `tail_contract::the_live_bound_is_taken_exactly_when_the_budget_is_exhausted`
  (gate 5): red, and only it, when a cache hit is admitted on its frames, not its charge (C2), or
  when the section charge is zero (C10, through its `charge > frames` premise). Also red on X5, X9,
  C1, C3, C5 and C9.
- **Changed** `builtins_compiler::tests::a_preparation_walks_at_most_the_budget_and_a_warm_cache_walks_nothing`
  (gate 7): red, and only it, when a stopped design is not cached (M8) or a cached stop is honoured
  with `<` (M4). Also red on X3, X9, C1, C3, C5 and C9.
- **Changed** `builtins_compiler::tests::every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`
  (gate 8): red, and only it, when the section charge is 1,100 (C6) or the budget is 1,350,000 (C7).
- **Changed** `host_core::prepare::tests::a_design_bound_cache_changes_no_prepared_value` (native
  gate 3): red, and only it, when the policy function passes `None` instead of its cache (X1).
- The cache-cap test and the live-bound table test are unchanged since attempt 1, and they pass.

## Mutation runs

How I ran them:
- Each mutation was applied alone to a second export (`mtree`) and then restored from a pristine
  copy.
- Suites, all with `--no-fail-fast`: `tail_contract` (13 tests), the builtins lib, the
  builtins-compiler lib and the host-core lib.
- Profile: release with `test-support`, all four rates, with LTO off (`CARGO_PROFILE_RELEASE_LTO`)
  so that the builds stay fast.
- The baseline is green in every suite. Logs: `evidence/mut/`.

| id | mutation | red |
|---|---|---|
| X3 | majorant pass counts, does not stop | new test, gate 7 |
| X5 | deviation walk counts, does not stop | new test, gate 5 |
| X7 | replay loop counts, does not stop | **none** (m2) |
| X9 | `take_frame` allows one frame past the horizon (`>`) | new test, gate 5, gate 7 |
| C1 | the sections' charge not reserved before the walk | new test, gate 5, gate 7 |
| C2 | hit admitted on `frames`, not `charge` | gate 5 only |
| C3 | charge omits the section charge | new test, gate 5, gate 7 |
| C4 | right channel gets the whole budget | new test only |
| C5 | identical right channel charged again (mine) | new test, gate 5, gate 7, host-core test |
| C6 | `INPUT_BOUND_SECTION_CHARGE` 1,100 | gate 8 only |
| C7 | `INPUT_BOUND_BUDGET_FRAMES` 1,350,000 | gate 8 only |
| C8 | two-cascade charge omits the right channel | new test only |
| C9 | a stopped design leaves the remaining budget | gate 5, gate 7 |
| C10 | `INPUT_BOUND_SECTION_CHARGE` 0 (mine) | gate 5 only |
| M4 | cached stop honoured with `<` | gate 7 only |
| M8 | stopped design not cached | gate 7 only |
| X1 | host-core passes `None` to builtins-compiler | host-core test only |

Every row the record states (X3, X5, C1-C4, C6-C9, M4, M8, X1) reproduces.

## Gates run

All gates ran on the `b74d8c69c` export, x86-64, with `CARGO_INCREMENTAL=0`.

| gate | result |
|---|---|
| Gate 2: committed `input_bound_budget`, one invocation, `taskset -c 7`, quiet box (load 0.48) | ok; reproduces the record (largest design work 22.35 ms); `evidence/gate2.log` |
| Gate 2, my families: the band, the record's two-section families, two cascades | see MJ1 |
| Calibration: `input_bound_budget calibrate`, one invocation | reproduces the constants |
| Gate 8, the spec's command with `--nocapture` | ok; every figure equals the record (96 kHz margin 122,864) |
| `cargo fmt --all -- --check` | clean |
| `cargo test -p builtins-compiler --no-run` | ok |
| test-debug-a workspace step (exact `--exclude` and `--features`) | 1,465 passed |
| test-debug-a doctests | ok |
| test-debug-b DSP step | ok |
| Release `-p lane -p math -p wasm-gates --features math/lane` | ok |
| Release `filter_liveness` | ok |
| Release `tail_contract` | 13 passed |
| `tail_contract --no-run` without `test-support` | compiles |
| `cargo clippy --workspace --all-targets -D warnings`, with and without `--all-features` | clean |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` | clean |
| `audit capi` | 0 allocations, 0 deallocations, 0 locks, 0 syscalls, 0 violations; `pcm_digest` `cb10fbface44a3a4` (unchanged) |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm` and `--self-test`, `strip-wasm-names.py` | ok |
| `test-web-audioworklet.sh` | ok; empty `TMPDIR` afterwards |
| `check-web-audioworklet-v8-spill.py --self-test` and on the module | ok |
| SDK: `npm ci`, `check-sdk-generated.sh`, `check-sdk-deletions.py` and `--self-test`, `check-sdk-types.sh`, `check-sdk-headless.sh`, `sdk-package.sh check` | ok |
| `run-wasm-gates.sh --without-v8-spill --without-native` | ok |
| `check-cross-targets.sh` | ok |
| `check-capi-abi.sh` and `--self-test` | ok |
| `check-scalar-oracle-absent.py --native target/release/libcapi.so` | ok in CI order (see note 1) |
| `check-graph-determinism.sh`, `check-builtins-fixtures.sh`, `check-console-fixtures.sh`, `check-effect-contract.sh`, `check-protocol-wasm-parity.sh` | ok |
| The three `test-realtime-audit-probes.sh` legs | ok |
| Every lint-job `check-*.sh` and its `test-*.sh` (see note 2) | ok |
| `check-ci-path-routing.py` and `test-ci-path-routing.py`; `check-test-support-ci.py` and `test-test-support-ci.py` | ok |
| `check-script-reachability.py` and `test-script-reachability.py` | ok (see note 3) |
| `check-release-shape.py` and `--self-test`, `test-npm-publish-modes.py`, `check-session-map-shape.py`, the command-kind and command-reason vocabularies, `check-abi-layout-v1.py --self-test`, `check-parameter-metadata-v1.py --self-test` | ok |
| AArch64 | runs only in CI |

Notes:
1. My first run read `libcapi.so` straight after the four-package release build. That build
   unifies `audit`'s `graph/test-support` into capi. The check fails there, as the workflow's own
   comment says it would. In CI order, after `check-capi-abi.sh` rebuilds `-p capi` alone, it
   passes: none of the 17 scalar-path identifiers are present.
2. The lint-job scripts: workspace-policy, session-policy, bench-policy, host-core-policy,
   protocol-control-policy, realtime-policy, realtime-audit-leak, artifact-evidence-leak,
   lane-policy, rack-policy, builtins-policy, graph-policy, effect-runtime-policy,
   effect-runtime-fixtures, env-vocabulary, conformance-boundaries, console-benchmark-fixture,
   bench-preconditions, unfused-seal and `--self-test`, parametric-eq-render-contract, dsp-research,
   builtins-listening, test-builtins-fixtures, test-realtime-trace-validator, test-console-benchmark,
   test-dsp-research, test-gate-lib and test-sdk-artifact-builder-output-contract.
3. The two reachability scripts need `git ls-files`. They ran in a throwaway git repository made
   from the export.
