# Build parameter-metadata once per PR, and remove the other duplicate CI work

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 items 3 and 13). Base `a9414c0c`. No ruling
needed.

## Problem

Several required jobs repeat work another job already did at the same commit. Figures are from PR
#1016's logs and `../data/script-gates-verify.md` §2 and §8.

| duplicate | where | cost |
|---|---|---:|
| native fat-LTO `cargo run --release -p parameter-metadata` | `artifact` (writes the metadata into the artifact), then `artifact-gates` (`scripts/check-web-audioworklet.sh:492`, `--check`) and `sdk` (`sdk/codegen/assets.mjs:44`, twice via `scripts/check-sdk-generated.sh` and `scripts/sdk-package.sh:43`) | 61.6 + **75.0 + 72.3 s** |
| `node scripts/test-web-audioworklet.mjs` | `lint` (`qualification.yml:305-306`), and three runs again in `artifact-gates` via `test-web-audioworklet.sh` | 0.6 s plus the lint job's Node setup |
| capi release rebuild | `scripts/check-capi-abi.sh:28` rebuilds 23 crates because `-p capi` alone resolves different features than the audit build | 36.4 s |
| effect-runtime policy and its mutation suite | `lint`, and again inside `scripts/check-effect-contract.sh:30-31` in `audit-native` | 4.9 s |
| "Math M3 digests on an FMA-enabled build" | `qualification.yml:540-541` sets `RUSTFLAGS=+fma`, but `.cargo/config.toml` already builds every x86-64 target with `+avx2,+fma`, so the main release leg is already FMA-enabled. The override only drops AVX2 | 9 s plus a separate target dir |

The `--check` in `artifact-gates` compares the generator's output with the same generator's output
at the same commit. It can only go red if the generator is non-deterministic.

## Outcome

- **`artifact-gates`:** `check-web-audioworklet.sh` skips `parameter-metadata --check` when it is
  given the downloaded, pin-verified CI artifact directory. Local no-argument use keeps the check.
- **`sdk`:** `check-sdk-generated.sh` compares `sdk/assets/miso-engine-v1-{parameter-metadata,abi-layout}.json`
  byte for byte against the downloaded artifact's copies instead of re-running cargo. The job drops
  the Rust toolchain and cache if nothing else in it needs them. `sdk-package.sh` stops calling
  `check-sdk-generated.sh` a second time.
- **`lint`:** drops its copy of `test-web-audioworklet.mjs`.
- **`audit-native`:** builds `check-capi-abi.sh`'s `libcapi` in its own cached
  `CARGO_TARGET_DIR=target/ci/capi-abi` (capi-only features). It does **not** set `SKIP_BUILD=1`,
  which would point the ABI check at an audit-feature build.
- **`check-effect-contract.sh`:** drops its re-run of the effect-runtime policy.
- **`test-release`:** drops the M3 "FMA" step.

## Scope

Authorized paths:
- `scripts/check-web-audioworklet.sh`;
- `scripts/check-sdk-generated.sh`, `scripts/sdk-package.sh`, `sdk/codegen/assets.mjs`;
- `scripts/check-capi-abi.sh`, `scripts/check-effect-contract.sh`;
- `.github/workflows/qualification.yml`;
- this issue's spec.

## Gates

1. **The SDK drift claim still discriminates:**
   - a one-byte hand edit to `sdk/assets/miso-engine-v1-parameter-metadata.json` fails the `sdk` job;
   - a Rust change that alters the generated metadata without regenerating `sdk/assets` fails the
     `sdk` job, because the artifact copy differs.
2. **The ABI check still discriminates.** In a scratch branch, an extra `#[no_mangle]` export and a
   header drift each fail `check-capi-abi.sh`, as its `--self-test` cases do. The check runs against
   a capi-only build: record the `cargo metadata` feature set used.
3. **M3 still runs with FMA.** The test-release log shows `target_feature="fma"` in
   `rustc --print cfg` for the leg that runs `m3_determinism`, and the M3 pins still pass.
4. **Nothing else lost.** Every step removed here names the step that still runs the same check.
   The effect-runtime policy still fails lint on a seeded dependency addition.
5. **Cost.** Measured on a full-route PR:
   - `artifact-gates` about −70 s;
   - `sdk` about −70 s plus toolchain setup;
   - `audit-native` capi step about −30 s on a warm cache;
   - `test-release` −9 s.

## Saving and risk

- **Saving:** about 190 s of runner time per full PR.
- **Risk:** none identified. Each removed step duplicated a check that stays.
