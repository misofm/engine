# Make the C ABI release build independent of the caller's flag environment

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #2067. The cited closed spec is read
at `da15a2824` (it has left this folder).

## Problem
`scripts/build-capi-release.sh` runs its one `cargo rustc` without setting any flags. Its flags come
from the caller's environment: an exported `RUSTFLAGS` replaces `.cargo/config.toml`'s
`[target.'cfg(target_arch = "x86_64")'] rustflags`, and an exported `CARGO_ENCODED_RUSTFLAGS`
replaces both. So the shipped C ABI library depends on the shell that builds it (for example an
x86-64 build without `+avx2,+fma`, which `lane` refuses, or one with extra flags that change code).
The AArch64 targets get no config flags at all, so for them only the environment decides.

## Evidence
- `scripts/build-capi-release.sh:239-241`: `cargo rustc --locked --release ... -p capi --lib
  --crate-type ...`; the script never sets or unsets `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS`.
- `.cargo/config.toml`: the only flags are `target-feature=+avx2,+fma` for `x86_64`.
- Cargo reference, "Environment variables": `CARGO_ENCODED_RUSTFLAGS` takes priority over
  `RUSTFLAGS`, which takes priority over `target.<cfg>.rustflags`.
- #2067 spec, stated limit: "The C ABI release build (`build-capi-release.sh:239`) takes the
  caller's flag environment; root tracks that as a separate successor."

## Smallest slice
Make the script's flags its own: unset `CARGO_ENCODED_RUSTFLAGS` and `RUSTFLAGS` before the cargo
command (so `.cargo/config.toml` decides on x86-64), with a short comment that says why. Keep the
`cargo rustc` line's spelling if another file finds it by that spelling (check with `git grep`).
Decide, and record, whether the AArch64 targets need explicit flags; if none, say so.

## Gates
- With `RUSTFLAGS` and, separately, `CARGO_ENCODED_RUSTFLAGS` exported to a value without
  `+avx2,+fma`, the built x86-64 library is byte-identical to the library built without them (PR
  evidence, one comparison each; not a committed test).
- The script's own preflight refusals still run before cargo.
- `lint-checks.sh --self-tests` (session scratchpad) and `bash scripts/check-workspace-policy.sh`
  pass.

## Paths
`scripts/build-capi-release.sh`, this spec.

## Decision record (attempt 1, 2026-10-25)
- Root R1: after the preflight, before `cargo rustc`, the script unsets `CARGO_ENCODED_RUSTFLAGS`,
  `RUSTFLAGS`, `CARGO_BUILD_RUSTFLAGS` and every `CARGO_TARGET_*_RUSTFLAGS`. The `cargo rustc` line
  keeps its spelling (`docs/C_ABI_V1_QUALIFICATION.md:16` quotes its shape; no script greps it).
- Root R2: it also unsets every `CARGO_PROFILE_RELEASE_*`: the manifest's release profile is the
  shipped profile. No CI job that runs this script sets one (nightly's `CARGO_PROFILE_RELEASE_LTO`
  job runs `run-release-workspace-tests.sh`, not this script).
- Root R3: unset, never refuse.
- AArch64: no explicit flags. NEON is in the `aarch64-apple-ios` and `aarch64-linux-android`
  baselines and `lane` keys on `target_feature = "neon"`.
- Stated limit: this issue covers the flag environment only. A `build.rustflags` or
  `[profile.release]` in a caller's own `~/.cargo/config.toml`, and `CARGO_BUILD_TARGET`,
  `RUSTC_WRAPPER`, `CARGO_BUILD_RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER` and `CARGO_BUILD_RUSTC`,
  still apply.
- Self-test (attempt 2): `build-flag-env` puts a stub `cargo` on PATH that records its environment
  and fails. It exports `RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `CARGO_BUILD_RUSTFLAGS`, `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS`,
  `CARGO_PROFILE_RELEASE_OPT_LEVEL`, `CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER` and
  `CARGO_TARGET_DIR`. It expects the "cargo rustc failed" refusal, then checks that cargo got none of
  the first five and both of the last two. It is red on three mutants: unset block removed, pattern
  narrowed (no `CARGO_TARGET_*_RUSTFLAGS`), pattern widened to `CARGO_TARGET_*`. Attempt 1's
  `preflight-with-rustflags` case is gone, because the preflight reads no flag variable.
- PR evidence (host staticlib, x86-64, separate scratch `CARGO_TARGET_DIR`s): with
  `RUSTFLAGS='-C target-feature=+avx2,+fma -C target-cpu=native'` and, separately,
  `CARGO_ENCODED_RUSTFLAGS=$'-C\x1fopt-level=1'` exported, `libcapi.a` is byte-identical to the clean
  build (sha256 `044ad68e...bb8a9`). At base `609ccf399` the first gives a different archive
  (`7b00b0c0...ad74`) and the second fails at `lane`'s `compile_error!`.
