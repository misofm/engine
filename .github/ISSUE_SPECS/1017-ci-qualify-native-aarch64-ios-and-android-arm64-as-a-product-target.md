# CI: qualify native AArch64 (iOS and Android arm64) as a product target

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

## Product outcome

The owner ruled on 2026-09-28 that native AArch64 is an official target: fans open and play sessions in iOS and Android apps, which embed the engine through the C ABI (`docs/rulings/engine-footprint-2026-09-28.md`). Today nothing qualifies it. No CI job builds, lints or tests any AArch64 target: `scripts/check-cross-targets.sh:4-5` records that the Android and iOS rows were removed under #378 ("native AArch64 unsupported, no claim"), and `docs/TARGET_MATRIX.md` keeps a deferred-defect register that a revival must reopen. Evidence: `docs/handoffs/dead-code-2026-09-28/VERIFY-DEAD-CODE.md`, finding F4.

## Smallest closable slice

1. Restore the `aarch64-apple-ios` and `aarch64-linux-android` rows in `scripts/check-cross-targets.sh` (`cargo check --all-targets --all-features` for the product crates and `capi`, with clippy `-D warnings`), and add a CI job for them on the required workflow's routing rules.
2. Run the product crates' tests on real AArch64 hardware in CI, on a GitHub-hosted arm64 runner (Linux arm64 and, for Darwin behaviour, macOS arm64), so NEON (`Simd4`) and the AArch64 FPCR path in `lane::fpenv` are exercised.
3. Fix what the new legs expose that is not a known defect: the seven test-only warnings (`host-core/tests/fp_environment.rs:25,26,52,63,81,102`, `lane/tests/fp_env.rs:14`) and tests that assert `Backend::current() == Simd8` (listed in `docs/audits/test-usefulness-2026-09-04/03-compilers-hosts-tools.md:227,240,297`).
4. The two known defects (the Darwin `memset_pattern16` in SVF `flush()`, and LANE-3 `fmaxnm`/`fminnm`) have their own issues. Until they land, the new legs mark exactly those checks as expected failures by name, never by skipping a whole job.
5. Update `docs/TARGET_MATRIX.md` (support level and the register) and `AGENTS.md` if its target wording needs it.

## Objective gates

- The new CI legs run on every pull request that touches engine code, and `qualification`'s verdict table includes them.
- On AArch64 hardware, the class-A digest suites (console-workload `WORKLOADS`, the effect differentials) produce the same digests as x86 and the browser, apart from the named LANE-3 cases.
- The realtime audits (`audit capi` and the allocation/syscall audits) run on the AArch64 legs, apart from the named Darwin memset case.
- A deliberately inserted `Backend::current() == Simd8` assertion, and a deliberately inserted warning, each turn the new leg red.
