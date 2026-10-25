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
- `scripts/check-web-audioworklet.sh` passes; `lint-checks.sh --self-tests` passes.

## Paths
`scripts/build-web-audioworklet.sh`; this spec.
