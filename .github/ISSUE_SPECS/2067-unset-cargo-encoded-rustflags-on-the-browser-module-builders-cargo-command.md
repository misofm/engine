# Unset CARGO_ENCODED_RUSTFLAGS on the browser module builder's cargo command

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #2046.

## Problem
`scripts/build-web-audioworklet.sh` sets the browser module's flags through `RUSTFLAGS` on its one
`cargo build` line. Cargo reads `CARGO_ENCODED_RUSTFLAGS` before `RUSTFLAGS`: when a caller has it
set (for example a cargo-invoked wrapper or a CI step that exports it), cargo ignores the builder's
`RUSTFLAGS` completely. The module then builds without `+simd128`, without the strip flag and without
the three `--remap-path-prefix` remaps, and the script gives no error.

## Evidence
- `scripts/build-web-audioworklet.sh:172-175`: `RUSTFLAGS="-C target-feature=+simd128 $strip_flag $remap"`
  is the only flag source on the `cargo build ... --target wasm32-unknown-unknown -p host-web` line;
  the script never unsets `CARGO_ENCODED_RUSTFLAGS` (no match for it in the file).
- Cargo reference, "Environment variables": `CARGO_ENCODED_RUSTFLAGS` takes priority over `RUSTFLAGS`.

## Smallest slice
Run the builder's cargo command with `CARGO_ENCODED_RUSTFLAGS` removed from its environment
(`env -u CARGO_ENCODED_RUSTFLAGS` on that line, or `unset` in its subshell), with a one-line comment
that says why. No other change.

## Gates
- With `CARGO_ENCODED_RUSTFLAGS` set to a value without `+simd128` in the caller, the built module
  is byte-identical to the module built without it (PR evidence, one comparison; not a committed test).
- `scripts/check-web-audioworklet.sh`, `scripts/check-artifact-evidence-leak.sh` and
  `scripts/test-artifact-evidence-leak.sh` pass. (`lint-checks.sh --self-tests` is a session scratchpad
  script that runs CI's lint-job policy scripts, not a file in this repository.)

## Paths
`scripts/build-web-audioworklet.sh`; `scripts/run-wasm-gates.sh`; `.github/workflows/qualification.yml`
(the three loom steps only); this spec.

## Decision and evidence record (2026-10-25)
- Shape: `unset CARGO_ENCODED_RUSTFLAGS` as the first statement of the builder's existing subshell, not
  `env -u` on the cargo line: `check-artifact-evidence-leak.sh` and three other files find or quote that
  line by its `cargo build` spelling, which stays byte-identical.
- Root widened the slice to the same exposure in `run-wasm-gates.sh` and the loom steps. In
  `run-wasm-gates.sh` the unset is script-wide: the stable host runner (`cargo run -p wasm-gates`) takes
  its x86-64-v3 flags from `.cargo/config.toml`, which an encoded value also replaces, so a guest-only
  unset still fails. The three loom steps each start with `unset CARGO_ENCODED_RUSTFLAGS;`.
- Correction to the scope note's gate: `--check-pin` cannot be the comparison. The pin is the release
  pin (#1061, docs/RELEASE.md), and the current tree's module differs from it with or without the
  variable (observed `d7440810...`, pin `47e9c699...`). The gate run is the spec's own byte-identity
  comparison instead.
- PR evidence, with `CARGO_ENCODED_RUSTFLAGS=$'-C\x1fopt-level=1'` exported:
  - Builder before the change: build fails, `lane` refuses wasm32 without simd128 (exit 101).
  - Builder after the change: the module is `d7440810145e3ea9...`, byte-identical to the build without
    the variable.
  - `run-wasm-gates.sh --without-v8-spill --without-native` before: exit 101 (same `lane` refusal);
    after: exit 0, `wasm gates: ok`.
