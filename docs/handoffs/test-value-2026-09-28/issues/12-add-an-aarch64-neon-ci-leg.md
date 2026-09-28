# Add an AArch64/NEON CI leg, and run the 4-lane production path natively

Draft, not a GitHub issue. A **gap**, not a cut. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §6 items 1-2). Base `a9414c0c`. Paths starting `../`
are relative to the audit's handoff folder. **Needs owner
ruling R1**, which reverses #378's "native AArch64 unsupported, no claim". The owner has since said
that fans open sessions in iOS and Android apps that embed the engine natively.

## Problem

**No CI job builds or runs anything for AArch64.**
- `scripts/check-cross-targets.sh:3-6` removed the Android and iOS rows under #378 (2026-09-04).
- `docs/TARGET_MATRIX.md:86-104` records open defects for the target:
  - **LANE-3 (#366):** on AArch64 release builds the D8 `max`/`min` fold into `fmaxnm`/`fminnm`,
    moving bits away from the pinned oracle. **Gate G1 is red there.** This is a class-A defect on
    the mobile target.
  - The SVF `flush()` compiles to two `memset_pattern16` calls per frame on Darwin, a realtime
    violation. The register cites `crates/lane/src/kernels.rs:280-281`; the splat is now in `flush`
    at `crates/lane/src/lib.rs:134`.
  - Tests that assert or return on `Backend::current() == Simd8` (for example
    `crates/effect-compiler/tests/bank_state.rs:913`, `crates/compressor/tests/bench_ramp.rs:377`,
    `crates/gate-expander/src/lib.rs:1201`) fail, or silently skip, on a 4-lane native target.

**The 4-lane path is what browsers (`simd128`) and phones (NEON) both run, but the native test run
exercises it only in part.**
- The x86-64-v3 CI host is `Simd8`, and its effect factories decline most 4-lane banks.
- So the `Simd4` bank-count pins in `crates/graph-compiler/tests/bank_levels.rs` never run
  ([#966 handoff `README.md:13-19`](https://github.com/misofm/engine/blob/5379e46ca3b349b9d277d642c008bb7a9643fb76/docs/handoffs/bug-966-2026-09-27/README.md#L13-L19),
  removed by #1031).
- `crates/gate-expander/tests/identity.rs:223` returns unless `Simd4`.
- The only tests written *for* non-x86 are stubs that assert almost nothing:
  `crates/lane/tests/g6_ftz_inert.rs:123` and `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:214`.

The repository is public, so GitHub's `ubuntu-24.04-arm` runners cost nothing.

## Outcome

**Start outside the required workflow.** A new job, `aarch64` (`runs-on: ubuntu-24.04-arm`), goes in
`nightly.yml` or in a new non-required workflow that runs on pull requests. It cannot start in
`qualification.yml`: that workflow's verdict requires every full-route job to pass (`:860-925`), and
the job's first result is an expected red, LANE-3. It moves into `qualification.yml` when it is
green, after #366 fixes LANE-3.

The job runs:
1. the lane and math gates in the **shipping release profile**
   (`cargo test --release -p lane -p math -p wasm-gates --features math/lane`). LANE-3 is an
   optimizer fold, so a non-LTO profile could hide it;
2. the G5 class-A corpus (`g5_native_digests_match_pins`) against the same pins as x86-64. This is
   the single owner proposed in issue 08;
3. the DSP debug shard (`test-debug-b`'s package list), where the 4-lane native backend binds the
   4-lane banks.

**Width-agnostic tests.** Tests that assert, or silently return, on `Simd8` and run in this job
become width-agnostic: they compare at `Backend::current()` and name the width in the failure
message.
- In scope: `crates/effect-compiler/tests/bank_state.rs:913` and
  `crates/gate-expander/src/lib.rs:1201`.
- Out of scope:
  - `crates/compressor/tests/bench_ramp.rs:377` and the `tools/bench` `*_active.rs` preflights, which
    issue 06 deletes under R6;
  - `tools/wasm-console/src/main.rs:155`, which the job does not run.
- The two `cfg(not(x86))` stubs (`crates/lane/tests/g6_ftz_inert.rs:123`,
  `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:214`) gain real assertions for AArch64.

**The bug-966 `Simd4` pins get a home.** They run on this job, where `Backend::current()` is
`Simd4`. Alternatively, commit
[`wasm-pins-harness.patch`](https://github.com/misofm/engine/blob/5379e46ca3b349b9d277d642c008bb7a9643fb76/docs/handoffs/bug-966-2026-09-27/wasm-pins-harness.patch)
(removed by #1031) as a `wasm-guests` step.

**`cargo check` rows** for `aarch64-linux-android` and `aarch64-apple-ios` return to
`check-cross-targets.sh` (`docs/TARGET_MATRIX.md:59-66`). Darwin realtime, the per-frame `memset`,
needs a macOS arm64 runner. Record it as a follow-up issue, not part of this slice.

## Scope

Authorized paths:
- `.github/workflows/nightly.yml`, or one new non-required workflow file;
- `scripts/check-cross-targets.sh`;
- the two tests and two stubs named above (width-agnostic assertions only);
- `docs/TARGET_MATRIX.md`;
- this issue's spec.

No product code: LANE-3's fix belongs to #366. Moving the job into `qualification.yml` and its
verdict table is a follow-up, once the job is green.

## Gates

1. **The leg sees the known defect.** On the base tree, the job's G1 tests
   (`crates/lane/tests/g1_op_identity.rs`) fail on the D8 `max`/`min` cases. The PR records the
   failing test names and the input words (ties and signed zeros), so the red is identified as
   LANE-3 and not as something else.
2. **Class A across targets.** With LANE-3 fixed on a branch, the G5 corpus on AArch64 matches the
   x86-64 and `simd128` pins exactly. Record every case's digest.
3. **The 4-lane banks are exercised natively.**
   - On the AArch64 job, `Backend::current()` is `Simd4`.
   - `bank_levels.rs`'s `Simd4` pins execute and pass.
   - Re-injecting #966 with `../tools/revert.py` turns `bank_levels.rs` red there, at `Simd4`.
4. **No silent skip is left in the job's scope.** In the packages the job runs, `git grep` finds no
   test that `return`s on a backend width without asserting. The two rewritten tests fail on a
   seeded wrong-width bank.
5. **Cost.** Record the job's duration. It is outside the required path, so it does not lengthen a
   PR's wait. Estimate: 8-10 minutes on the arm64 runner.

## Saving and risk

- **Saving:** none, this adds coverage. Cost: one parallel job, estimated 8-10 minutes of runner
  time.
- **Risk of not doing it:** the mobile apps ship an engine whose class-A claim is already known false
  on their CPU (LANE-3), with no gate that would say so.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Do not file: merge into #1017.**
   - #1017 (filed) owns the AArch64 CI leg, the `aarch64-apple-ios` and `aarch64-linux-android`
     cross-target rows, a macOS arm64 leg for Darwin, and the realtime audits on AArch64.
   - It marks LANE-3 (#1019) and the Darwin `memset` (#1018) as expected failures **by name**. That
     supersedes this draft's "start outside the required workflow".
2. **Carry these into #1017 as an amendment.** #1017 lacks them:
   - the lane, math and G5 legs run in the **shipping release profile**, because LANE-3 is an
     optimizer fold;
   - `bank_levels.rs`'s `Simd4` pins run natively, and re-injecting #966 turns them red at `Simd4`;
   - a no-silent-skip gate over the packages the job runs;
   - the two `cfg(not(x86))` stubs gain real assertions.
3. **References.** LANE-3's fix is #1019 (not #366), and the Darwin `memset` is #1018.
