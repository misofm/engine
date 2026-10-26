# W=4 bank counters: read #2132's counters on a wasm guest under wasmtime

Draft slice 0a (#2143) of the banking design (`docs/BANKING_DESIGN.md`). GitHub number to be assigned.
Files after #2132, which it completes for the four-lane width.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Class

A: tooling and a test guest; no engine render change.

## Problem

The finish line is asserted at W=4 and W=8. #2132's counters (`banks`,
`mergeable_partial_pairs`, `bank_slot_runs` per pack group, `silent_skips`, `bypass_skips`) are
read through test-support readers on the native host, which is W=8. The browser is the product
path at W=4, but the shipped `host_web.wasm` has no reader export and must not gain one for a
counter (no test-only surface on the product artifact). V8 gives the timing p50; it cannot give
the counters.

## Smallest closable slice

- `tools/wasm-gate-guest` (the existing wasmtime guest, `simd128`, W=4) gains two exports on the
  pattern of `miso_gate_digest_word`: `miso_gate_console_row_count() -> u32` and
  `miso_gate_console_counter(row: u32, counter: u32) -> u64` (returned as two `u32` halves if the
  ABI needs it). The guest embeds #2132's C1-16 fixture JSON (the four states: `Load`, `Typical`,
  `AllActive`, `Sparse`; one row each), compiles it with the real pipeline at `Backend::Simd4`,
  feeds sources from the row's activity (the `Sparse` row feeds `+0.0` to odd tracks), renders a
  fixed `BLOCKS = 256`, and reads the counters through `graph`'s test-support readers (the guest
  is a test tool and may enable `test-support`). Counters: `0 banks`, `1 mergeable_partial_pairs`,
  `2 bank_slot_runs_total`, `3 bank_slot_runs_track_group`, `4 silent_skips`, `5 bypass_skips`,
  `6 pack_groups`.
- `tools/wasm-gates`: `run_wasm` reads the rows after the digest legs and writes
  `artifacts/steps/<run>/w4-counters.json` (`{ row, counters[] }`); the native leg reads the same
  rows in-process at `Simd4` (the native leg already runs every width the build has, #1112).
- `tools/bench console`'s report joins `w4-counters.json` with the V8 p50 rows by row name, so one
  table carries W=4 counters (wasmtime) and W=4 timing (V8), each labelled with its source.
- If `console-workload` does not build for `wasm32-unknown-unknown` (it links the bench runner),
  the guest depends on `graph-compiler`, `builtins-compiler` and the fixture file only; the row's
  activity is a 20-line function in the guest, not a dependency.

## Gates (one test per claim, red first on a named mutant)

1. `tools/wasm-gates` test `w4_counters_match_between_wasm_and_native_simd4`: for every row,
   every counter read under wasmtime equals the native `Simd4` leg's, integer-exact (a counter is
   the claim, so it is exact, not a ceiling). Red on a guest that renders a different block count
   or feeds the `Sparse` state to the wrong tracks. Test value: red when the wasm guest's plan or
   render differs from native `Simd4` in any bank decision.
2. `tools/wasm-gates` test: the `Load` row's `bank_slot_runs_total` is 0 once #2129 lands and
   equals `banks x slots x BLOCKS` before it; the `AllActive` row's `banks` equals `ceil(16/4)` per
   console rack plus the submix banks of the design's W=4 table (29 BSR per block). Red on a
   counter summed over the wrong units. Test value: red when the W=4 counters stop describing the
   plan the design's finish line is stated for.
3. The guest's existing G5 digest gate is unchanged and green (the new exports add no state to
   the digest path).

## Paths

`tools/wasm-gate-guest/src/lib.rs`, `tools/wasm-gate-guest/Cargo.toml` (test-support features,
the fixture `include_str!`), `tools/wasm-gates/src/lib.rs` and `main.rs`, `tools/bench/src/
console.rs` (the join), `scripts/run-wasm-gates.sh` only if the artifact path changes,
`fixtures/session/v1/console-c1-16.json` (read, not changed), this spec.

## Dependencies

#2132 (the counters and the C1-16 fixture). Serves #2133, #2135, #2136 and slice 5a (#2142)'s W=4
evidence. Not on the browser's product path.
