# Compare stored automation bits on AArch64 and across seek times

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A5, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch Q.

## Product outcome

The native host on AArch64 (NEON, FPCR) renders the stored-automation fixtures of #1669 to the
same bits as the browser and x86-64, in the debug and the shipping release profile. And a session
seek that lands at two different render samples, or at two quanta, gives the same output once the
ramps it started have completed: stored automation after a seek depends only on the session and
the seek target. With #1669 this completes the evidence for the owner's ruling that stored
automation renders identically on every platform. It adds no new digest owner. Batch Q.

## Context

- **The fixtures and pins.** #1669 adds `automation-session.json` and
  `automation-matrix-session.json` to `hosts/host-web/tests/browser-v1/`, their transcript (96
  blocks of exact integer-formula PCM, one session seek at block 40), two native pin tests in
  `hosts/host-web/src/tests.rs` and their pins in `expected.json`. The three older native pin tests
  are `hosts/host-web/src/tests.rs:1632`, `:1751` and `:6690`.
- **AArch64 legs.** `scripts/run-aarch64-tests.sh` runs the product crates' tests in debug (capi's
  workspace closure, `:132-149`) and `lane`, `math`, `console-workload` and the audits in release
  (`:153-195`). `host-web` is in neither list, so no session digest is compared on AArch64 today
  (README A5). The `aarch64-debug` and `aarch64-release` jobs run it on an arm64 runner
  (`.github/workflows/qualification.yml:924-981`).
- **Known defects.** Expected failures are rows of `scripts/lib/aarch64-known-defects.py`, by name
  and reason; a row's test must fail as that test for that reason. LANE-3 (#1019) is open: on
  AArch64 release builds a `max`/`min` fold changes `exp2_lane` and `log2_lane` on NaN and
  signed-zero inputs (`docs/TARGET_MATRIX.md:163-167`).
- **Seeks.** A1.4: where a seek reaches a node, builtin lanes are set exactly and each effect cell
  stages a `Point`; A5: effect parameters settle after their smoothing (64, or 128 for the delay
  time), and effect state after the pre-seek audio's tail reaches exact rest (D15-4). The delay
  time's jump waits for a running crossfade (#1637's "no restart" rule).

## Decisions frozen for this slice

- **D1. The AArch64 leg.** `scripts/run-aarch64-tests.sh` runs #1669's two automation pin
  tests and the three existing native pin tests in both modes:
  `cargo test --locked -p host-web --features test-support --lib -- --exact <names>` in debug,
  and the same with `--release` in release. They compare with the same pins in `expected.json`. A
  failure that is LANE-3's (#1019) is recorded as a row of `aarch64-known-defects.py` naming #1019
  and its reason, by the script's own rule; any other failure fails the leg.
- **D2. Seek-time invariance.** A new test renders `automation-session.json` on two C ABI engines
  with silent sources before the seek and #1669's transcript after it. Engine A seeks to `T` at
  render sample `r1`, engine B at `r2`, with `r2 - r1` not a multiple of 64; once at quantum 128
  and once at quantum 100. Aligned at the seek, the two outputs are equal bit for bit from 256
  samples after the seek reaches the output: the longest ramp a seek starts is the delay time's
  held jump plus its crossfade (2 x 128). Builtin lanes are set exactly at the seek, and silent
  inputs leave no pre-seek effect state to wait for.
- **D3. One-ulp evidence (PR evidence, not committed).** With #1637's `value_at` changed by one
  ulp, both AArch64 legs turn red; revert.

## Deliverables

1. D1's script steps (and a #1019 row only if D1 needs one).
2. D2's test.
3. D3's record in the PR.

## Authorized paths

- `scripts/run-aarch64-tests.sh`, `scripts/lib/aarch64-known-defects.py` (a #1019 row only; never an
  `IOS_MEMSET_CEILINGS` row, README F19).
- `crates/capi/src/runtime/tests.rs` (D2).

## Non-goals

- Any engine change. A leg that disagrees is a defect for its own issue, recorded here with its
  first differing sample.
- New fixtures or pins (#1669 owns them).
- Seek invariance with audible pre-seek audio: its tail is bounded by #1329's rest bound, not by
  the automation, and is not this slice's claim.

## Hazards

- **LANE-3 (#1019).** The fixture's compressor and limiter reach `exp2_lane`. If the AArch64
  release leg differs and #1019 is still open, the difference is that defect, not automation; D1
  records it by the script's rule, and #1019's fix removes the row.
- **No silent skip.** The script refuses a test that returns early on a backend width (`:81-92`).
  The pin tests must run at the width the leg has.
- **Runner time.** The release leg builds `host-web`'s tests in release once more; keep the run to
  the five named tests.

## Objective gates

1. **AArch64.** `bash scripts/run-aarch64-tests.sh debug` and `bash scripts/run-aarch64-tests.sh release`
   pass on the arm64 runner (the `aarch64-debug` and `aarch64-release` jobs), with the five pin
   tests run in each.
2. **Seek-time invariance** (D2): `cargo test --locked -p capi` passes with the new test at both
   quanta.
3. **One ulp turns both AArch64 legs red** (D3, PR evidence).
4. **Workspace.** `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   `python3 -B scripts/check-script-reachability.py`.

## Test value

- The AArch64 runs turn red if a NEON build or FPCR changes a bit of a session render. No session
  digest is compared on AArch64 today.
- The seek test turns red if automation after a seek depends on when the seek landed or on the
  quantum, which no existing test checks.

## Dependencies

Batch Q. Direct dependencies:

- #1669 *Compare stored automation bits across the browser and native hosts*: fixtures, pins
  and the two pin tests.
- *Make the compressor's AArch64 release build bit-identical to the browser (LANE-3)* (#1019), for
  a clean release leg (D1).

#1642 *Seek the timeline and every source in one C ABI call* and #1637 *Compile stored
automation into per-cell events in node time* arrive through #1669.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- No design change. It inherits #1669's added rows. #1019 is still open. Anchors re-read at filing.

## Amendment (controls coordinator, batch 20, 2026-10-17)

Checked against `e7722bd19`. The owner narrowed #1669 to the rows that render today (see #1669's
batch 20 amendment).

- #1669 adds one fixture, `automation-session.json`, one pin test,
  `native_automation_timeline_digest_pins_the_wasm_parity`, and one pin,
  `directOracle.nativeAutomationPcmF32leSha256`. There is no `automation-matrix-session.json`.
  Read "the two automation pin tests", "both fixtures" and "the two pins" in this issue as these
  single items.
- D1 and D2 use #1669's narrowed transcript: 96 blocks of 128 frames, a plain session seek to
  timeline 30,000 at block 40, generation 2.
- The matrix fixture and the rows that #1657, #1658, #1660 and #1667 add are phase-2 extensions,
  as in #1669.
- The narrowed fixture has no insert, so D2's "equal from 256 samples after" bound (a delay's
  held jump plus its crossfade) does not apply. The longest ramp a seek leaves is a mute hold
  (`mute_ms`, 480 samples) followed by a fader ramp to the curve (`fader_ms`, 960 samples), so
  the bound is 1,440 samples after the seek at 48 kHz (`docs/SESSION_SCHEMA_V1.md`, the
  fader-row paragraph). A pre-seek unmute ramp still running at the seek carries across it, as
  documented. Read "five pin tests" in gate 1 and the hazards as four. The LANE-3 compressor and
  limiter hazard does not apply: the narrowed fixture has no insert.


## Amendment (easy fixes batch 2, 2026-10-27)

Baseline `3ef18112fe639ddc0109b77f1c922b07e38dd4d0`. Sol implements by the owner's explicit
selection for this batch; independent verification follows before delivery.

- Read-only GitHub checks confirm #1669 closed on 2026-10-17 and #1019 closed on 2026-10-19.
  The stale dependency hold and LANE-3 exception proposal above no longer apply. No new known
  defect row is authorized or needed by this slice.
- D1 has exactly four existing owner tests: identity, SET_CONTROL timeline, observation timeline,
  and `native_automation_timeline_digest_pins_the_wasm_parity`. Both profiles reuse their existing
  pins. The no-silent-backend-skip scan includes `hosts/host-web` now that this leg runs it.
- D2 retains #1669's fixture, seek target 30,000, generation 2, integer-formula PCM and 7,168
  post-seek frames (the transcript's blocks 40..96). Pre-seek PCM is silent. Its configurations
  are `(quantum, arrival)` = `(128, 5120)`, `(128, 5248)`, `(100, 5100)`, `(100, 5200)`.
  Plain seeks apply at block entries, so two quantum-128 arrivals necessarily differ by a
  multiple of 64; the cross-quantum arrival difference of 20 and the quantum-100 difference of
  100 change that phase. All four outputs compare from relative sample 1,440, the existing
  narrowed bound. Source seek reports prove the generation, arrival and target alignment.
- The compared window must contain nonzero audio in each plane and differ from the same fixture
  with its automation removed. No fixture or digest owner is added.

Test value: the one new C ABI regression fails if settled stored automation depends on absolute
render time or block partitioning; the existing C ABI seek regression uses only quantum 128 and
a fader ride flat around the seek target. The reused pin tests need no new tests or pins.

Verification evidence and the independent verdict will be appended before delivery. This x86-64
host has no `qemu-aarch64`; native debug/release results do not substitute for the required NEON
legs or their one-ulp mutation evidence.


### Attempt 1 implementation checkpoint

- The focused C ABI owner regression passes on x86-64-v3: `cargo test --locked -p capi
  stored_automation_after_a_seek_is_invariant_to_arrival_and_quantum_on_the_c_abi -- --nocapture`
  (one owner test, all four configurations; 0.50 s). Raw log is preserved outside the checkout
  under `engine-work-evidence/easy2-20261027/1670/capi-focused.log`.
- The first execution exposed a test setup constraint: shared `limits()` uses a 1,024-frame
  source ring, which is not divisible by quantum 100. D2 now uses one quantum per source ring,
  matching #1669's existing native/browser transcript. Its original rejection log is retained;
  this was no engine discrepancy and required no production edit or comparison-bound change.
- The shared serial runner used a separate worktree target, two Cargo jobs and one test thread.
  The offline compiler was paused while another engine stream compiled, and resumed only after
  its engine workloads were quiet. No test was paused and no unrelated process was signaled.
- Remaining evidence: full C ABI suite, formatting and script checks, fresh independent review,
  and the real AArch64 debug/release controls, actual-script one-ulp failures and final required
  qualification. No adversarial PASS or remotely synchronized completion is claimed here.


### Local qualification evidence (attempt 1)

Implementation checkpoint `64eac0ec9a764c0e713ab84ed574675cf80705b8`:

- `cargo test --locked -p capi`: PASS, 188 unit tests and 42 integration tests, including the new
  owner; the unit suite took 155.12 s with one test thread. Existing realtime/resource and
  concurrent plan-swap gates remain green. Log: `engine-work-evidence/easy2-20261027/1670/capi-full.log`.
- `cargo fmt --all -- --check`: PASS; `bash -n scripts/run-aarch64-tests.sh`: PASS;
  `python3 -B scripts/check-script-reachability.py`: PASS.
- A PR-only fake-Cargo probe invokes the actual runner in both modes, records and checks its
  exact argv, and returns a deliberate failure at its first Cargo invocation. Both modes reach
  exactly the four existing host-web owner names, with `--release` only in release, and abort
  before any workspace matrix. The runner's real no-silent-backend-skip scan also passes with
  `hosts/host-web` included. This is command-wiring evidence only: no AArch64 build or execution.
  Probe and log: `engine-work-evidence/easy2-20261027/1670/verify-aarch64-wiring.py` and
  `aarch64-wiring.log` in that directory. All checks used the shared serial wrapper.

Root will run the actual AArch64 one-ulp proof on a temporary, unmerged evidence branch. External
PR-only artifacts in the same evidence directory are `value-at-one-ulp.patch`,
`judge-arm-mutation.py` and `arm-mutation-proof.md`. The patch moves only a `value_at` linear
result to its adjacent f32; mute steps are untouched. Green controls run the four pins in debug
and release first. The mutated actual `run-aarch64-tests.sh debug` and `release` must each exit
101 with the automation owner failing at the frozen native/browser digest assertion and the
other three pins passing, with no later suite. The exact patch is then reversed. No mutant
source, extra pins or new corpus owner enters the delivered branch.

Still pending: fresh independent adversarial verdict, those real AArch64 controls and mutation
failures, the full unmutated AArch64 legs in required qualification, and the integrated batch's
workspace Clippy gate. Local x86 evidence does not substitute for any pending gate. GitHub
synchronization and closure remain root's delivery responsibility after upstream evidence.


## Independent review — attempt 1 PASS (2026-10-27)

Fresh Astra verified that both actual Arm script modes run the same four existing digest owners
before their profile branches, without new pins or expected-failure exceptions. The C ABI owner
compares settled generation-2 output at timeline 30,000 across all four frozen quantum/arrival
configurations, proves source alignment, and guards against silence and absent automation.
The author's full C ABI suite passed 230 tests; the independent integrated policy gates and
workspace Clippy passed, including Clippy after the newer main was integrated.

Native Arm proof [run 40452370315](https://github.com/misofm/engine/actions/runs/40452370315)
passed on `aarch64-unknown-linux-gnu`, Rust 1.99.1. Both debug and release controls passed all
four exact owners. With the temporary one-ULP linear `value_at` mutation, both actual
`run-aarch64-tests.sh` modes exited 101: exactly three other owners passed and the automation
owner failed at its frozen native/browser digest assertion. Each failed invocation reported one
test suite, so neither continued into the full matrix. The strict judge rejected other failure
shapes; the workflow reversed the patch and verified a clean source diff before recording PASS.

Fresh review downloaded the artifact and checked its runner, compiler, logs, judge bytes and
provenance. The unmerged evidence commit `b4b7aaa6adf009e233c67c2853221e1333add38b` differs
from the reviewed production source only in its dispatch-only qualification workflow. Neither
that workflow nor the fault is merged into product source. Raw artifacts and independent
validation remain in `engine-work-evidence/easy2-20261027/fresh-astra/arm-proof/` and
`arm-proof-validation.txt`.

Test value: the one new C ABI regression turns red when settled stored automation depends on
absolute render arrival or block partitioning, which the existing quantum-128 flat-target seek
row cannot detect. Reusing the existing four digest owners adds real NEON/profile execution
without a second corpus; the one-ULP experiment proves that their Arm wiring detects an actual
arithmetic change.

All local verification used the shared serial runner (two Cargo jobs, one Rust test thread).
The final combined production source is `24000af983207d5f18aecb2aece5d1a3466741c1`.
Full required PR `qualification`, including both unmutated AArch64 legs and both standard
browser matrices with their mutations, remains mandatory before delivery and issue closure.
Those full browser rows were deliberately deferred to required CI, not reported as local PASS.
