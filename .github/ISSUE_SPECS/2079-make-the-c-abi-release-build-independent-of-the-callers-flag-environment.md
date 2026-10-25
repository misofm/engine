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
