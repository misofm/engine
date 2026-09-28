# Ruling: build release-mode *test binaries* without fat LTO

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §3.3 and §5 item 11). Base `a9414c0c`. **Needs
owner ruling R4:** D12 says "one release profile for every artifact the engine ships, so a benchmark,
a CI probe and a shipped host all measure and execute the same code". Are unit-test binaries such
artifacts?

## Problem

Two per-PR steps compile tests in the shipping release profile (`lto = "fat"`,
`codegen-units = 1`). Each test binary gets its own fat-LTO link, so compiling dwarfs testing. From
PR #1016's logs:

| step | job | compile | run |
|---|---|---:|---:|
| `cargo test --release -p lane -p math -p wasm-gates` | test-release | 182 s; the last crate started compiling at 05:29:50 and the build finished at 05:32:35, so **≈ 165 s is linking about 30 test binaries** | ≈ 20 s |
| `cargo test --release -p audit -p bench -p console-workload` | audit-native | **3 min 30 s** for 9 test binaries, after the 99 s binary build | 30 s |

These two jobs, with test-debug-a, are the three longest in the run (446 s and 463 s). They set how
long every PR waits.

The claims these tests defend are numeric identity (lane G-gates, math M1-M3, the G5 corpus) and
tooling behaviour (audit fixture validators, console workloads). Rust performs no floating-point
contraction or reassociation at any optimisation level, and the class-A claim is the same with or
without LTO. The shipped profile is still exercised elsewhere:
- the release audits run the real `audit`, `bench` and `capi` binaries built with the shipping
  profile;
- the Wasm artifact gates inspect the shipped module;
- `run-wasm-gates.sh` runs the G5 corpus through the shipped-profile guest.

## Outcome (if R4 is yes)

- Both unit-test steps run with `CARGO_PROFILE_RELEASE_LTO=off` and
  `CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16`, each in its own `CARGO_TARGET_DIR`, so the shipping-profile
  binaries built in the same job are not disturbed.
- Every binary that CI *executes as a product or an audit* keeps the shipping profile.
- AGENTS.md or `Cargo.toml`'s D12 comment gains one sentence: unit-test binaries are not artifacts.

## Scope

Authorized paths: `.github/workflows/qualification.yml` (two steps), the D12 comment in `Cargo.toml`,
and this issue's spec.

## Gates

1. **Same numeric results.** Both builds of `-p lane -p math -p wasm-gates` pass with identical G5,
   M3 and G1-G6 digests. Record the printed digests from each build in the PR, and diff them.
2. **Same catches.** One re-injected historical bug per class runs under the new profile and still
   goes red: `../tools/revert.py 994` against the compressor tests. The console-workload paired
   spans still go red on a seeded one-bit change in a banked EQ render.
3. **The shipped-profile gates are unchanged.** The audit-native binary build, the release audits and
   the Wasm artifact gates still use the shipping profile. Show the `Finished release` line and the
   profile in the log.
4. **Cost.** test-release and audit-native each finish at least 90 s sooner on a full-route PR.
   Record both job durations.

## Saving and risk

- **Saving:** about 300 s of runner time per PR. Two of the three longest jobs drop by about 2-3
  minutes, so a PR waits about 3 minutes less.
- **Risk:** an LTO-only miscompile that affects a *test binary* and no shipped binary would go
  unseen. The shipped binaries and the Wasm artifact keep their own gates.
