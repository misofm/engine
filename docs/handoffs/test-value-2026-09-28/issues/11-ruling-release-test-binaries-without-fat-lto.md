# Ruling: build the `audit` and `bench` tooling unit tests without fat LTO

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §3.3 and §5 item 11). Base `a9414c0c`. Paths
starting `../` are relative to the audit's handoff folder.

**Needs owner ruling R4.** D12 says "one release profile for every artifact the engine ships, so a
benchmark, a CI probe and a shipped host all measure and execute the same code". Are the unit tests
of the evidence *tooling* such artifacts?

## Problem

`audit-native`'s step "Audit and console-workload unit tests in release" (`qualification.yml:574-575`)
runs `cargo test --release -p audit -p bench -p console-workload` in the shipping profile
(`lto = "fat"`, `codegen-units = 1`). In PR #1016's log:
- it compiles for **3 min 30 s**, after the 99 s binary build;
- it runs its tests for **30 s**.

`audit-native` is the run's longest job (463 s).

The tests fall into two kinds:
- **`console-workload` (64 tests):** class-A render claims on the product path. These are
  codegen-sensitive and must stay in the shipping profile. LANE-3 is the proof that an optimizer
  fold can move bits: on AArch64 release builds `max`/`min` fold to `fmaxnm`/`fminnm`.
- **`audit` (49) and `bench` (66):** mostly class-T tooling, such as fixture-validator mutations,
  benchmark record plumbing, percentiles and argument parsing. Their claims do not depend on the
  optimizer. They were moved to release because they were slow in debug.

Out of scope: `test-release`'s lane, math and wasm-gates G-gates. They exist to check the shipped
codegen and stay in the shipping profile.

## Outcome (if R4 is yes, for class-T tooling only)

- The step splits in two:
  - `cargo test --release -p console-workload`, unchanged;
  - `cargo test -p audit -p bench` with `CARGO_PROFILE_RELEASE_LTO=off` and
    `CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16` in its own `CARGO_TARGET_DIR`, or in debug if the
    measured time is acceptable.
- Every binary that CI executes *as a product or an audit* keeps the shipping profile.
- `Cargo.toml`'s D12 comment gains one sentence: tooling unit-test binaries are not artifacts.

## Scope

Authorized paths: `.github/workflows/qualification.yml` (one step split in two), the D12 comment in
`Cargo.toml`, and this issue's spec.

## Gates

1. **Measure first.** Before deciding, run the split once with `cargo build --timings` for both
   profiles, and record the compile time of each half. Adopt only if the `audit`/`bench` half saves
   at least 60 s. The audit could not separate LTO from codegen in the logs.
2. **Same verdicts.** Every `audit` and `bench` unit test passes under the new profile. Four of their
   seeded negative cases still fail as expected:
   - `fixture_builtins::tests::check_rejects_all_twenty_four_format_mutations`;
   - `check_rejects_manifest_grammar`;
   - `check_rejects_owned_jsonl_tuple_mutations`;
   - `issue067_graph_pdc_and_dependent_identity_mutations_are_rejected`.
3. **The shipped-profile gates are unchanged.** The audit-native binary build, the release audits,
   `console-workload` and the Wasm artifact gates still use the shipping profile. Show the profile
   lines in the log.
4. **Cost.** `audit-native` is shorter by at least the saving measured in gate 1. Record the job
   duration.

## Saving and risk

- **Saving:** unmeasured. The whole step compiles for 210 s, and the `audit`/`bench` share is
  probably 1-2 minutes. Gate 1 decides.
- **Risk:** none for class-T tooling claims. The codegen-sensitive tests keep the shipping profile.
