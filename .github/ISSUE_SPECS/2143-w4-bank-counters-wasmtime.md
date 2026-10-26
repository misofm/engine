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

## Evidence and decision record (attempt 1, 2026-10-26)

### What was built

- **One counter read for both legs.** `console_workload::bank_counters(workload, blocks)`
  (`tools/console-workload/src/lib.rs`) builds a row at `Backend::current()`, renders `blocks`
  blocks through the row's own feed (`SessionRuntime::new`, `before_block`, `render`), and returns the
  seven counters in `BANK_COUNTER_NAMES` order. `W4_COUNTER_WORKLOADS` (C1 16 in `Load`, `Typical`,
  `AllActive`, `Sparse`) and `W4_COUNTER_BLOCKS = 256` are beside it. The guest and the native leg
  run this function; neither has a copy of the activity.
- **Guest.** `tools/wasm-gate-guest` depends on `console-workload` and exports
  `miso_gate_console_row_count() -> u32` and `miso_gate_console_counter(row, counter) -> u64`
  (wasm has `i64`, so no halves), memoised per row like `miso_gate_digest_word`. It traps on an
  out-of-range row or counter. No `test-support` feature is needed: every counter comes from
  readers every build keeps (`graph::test_only_bank_slot_runs`, `graph::bank_effect_lanes`, the
  compile's `BankUtilisation`).
- **Host.** `wasm_gates <guest.wasm> --expect-backend simd4 [--w4-counters <file>]` reads the rows
  after the digest legs and the out-of-range trap check, fails the leg on any counter that differs
  from `wasm_gates::W4_CONSOLE_COUNTERS`, and writes `w4-counters.json`
  (`{ source, width, blocks, counters (names), rows: [{ row, counters }] }`). The evidence line
  gains `console_rows` and `console_mismatches`. `scripts/run-wasm-gates.sh` passes
  `--w4-counters "$output_dir/w4-counters.json"`.
- **Bench.** `bench console --bank-report-w4 <w4-counters.json> [<v8-records.jsonl>]` prints one
  table: the counters (labelled with the file's `source`) beside the V8 `p50_ns` of the document
  with the same `workload_kind`, labelled `V8`, or "not measured: no V8 document of this name".

### Decisions taken without a ruling (for review)

1. **The native `Simd4` leg runs on arm64 only, and both legs meet at pinned counts.** The spec
   says the native leg reads the rows in-process at `Simd4`. On this x86-64-v3 host it cannot: an
   eight-lane build's factories do not bind four-lane EQ, compressor, gate or drive banks (D4), so
   C1's console at `Simd4` is refused with `console.slot.foreign_width` (#1694), and
   `SessionRuntime::build_with_dispatch(Simd4)` folds the console into inserts first, which is a
   different plan. A four-lane native build (NEON) builds the same plan as the `simd128` guest at
   `Backend::current()`. So the comparison is G5's own: both legs equal one pinned table, exact
   (a count is the claim, not a digest). The wasm leg is held to it in `wasm-guests` (x86); the
   native leg in `w4_counters_match_between_wasm_and_native_simd4`, which `aarch64-release` runs
   (`cargo test --release -p wasm-gates`) and which is `ignore`d, with its reason, on an eight-lane
   build. **The pins come from the wasm run** (there is no third oracle for `Typical`'s seeded
   draw); gate 2 holds them to the design's arithmetic, and the arm64 job is the first native
   reading. It has not run yet: this branch is not on a PR.
2. **`bank_counters` lives in `console-workload`**, which is not in the spec's paths: the guest is
   a `cdylib` and cannot be linked by the host, and a second copy of the row activity in the guest
   would be a second subject (the crate's own Cargo comment). The spec's fallback ("if
   `console-workload` does not build for wasm32") did not apply: it builds unchanged.
3. **Counter definitions.** `bank_slot_runs_track_group` is the runs of every row of
   `test_only_bank_slot_runs` (a pack group, or a unit in none) that holds a track's lane, so total
   minus it is the submixes' runs. `silent_skips` is the same rows' whole-chain skips (unit-blocks).
   `pack_groups` is the rows spanning two units or more (a pack group always does).
   `bypass_skips` is the constant 0: no bank-slot skip exists before #2129, so no bypassed slot
   skips. #2129 replaces it with its reader; its `Load` runs fall to 0 and gate 2 goes red until it
   re-pins, so the constant cannot outlive the skip silently.

### Red evidence (each mutant run, then reverted)

- Gate 1 (wasm leg, `wasm_gates` against the pins): the guest renders `W4_COUNTER_BLOCKS + 1`
  blocks -> exit 1, e.g. `c1_console_16_load bank_slot_runs_total: expected 7424, got 7453`; the
  guest reads row `(row + 2) % 4` (the wrong state for a row) -> exit 1, `c1_console_16_load
  silent_skips: expected 0, got 259`; the track counter drops its track filter -> exit 1 (track runs
  7424 against 4096).
- **Not red: the `Sparse` state fed to the wrong tracks.** Silencing tracks 8..15 instead of the odd
  ones leaves every counter unchanged (exit 0), and `Sparse` equals `AllActive` on every counter.
  No track bank skips on input silence today: a fed `+0.0` is not a silence fact until slice 4
  (#2133) propagates it; the 259 skips in both rows are the submixes' (aux A's sends are muted).
  The spec's named mutant therefore cannot turn gate 1 red before #2133; #2133's W=4 evidence
  should see `Sparse` part from `AllActive` here.
- Gate 2 (`w4_counters_describe_the_designs_plan`): re-pinning the table from the unfiltered track
  counter (track runs 7424 / 7165) -> red, `c1_console_16_load: track bank-slot runs, left 7424,
  right 4096`.
- Gate 3: the G5 digest legs are unchanged and green: 155 cases, 270 wasm comparisons, 385 native,
  no mismatch, residency pin 70 limiter functions, f64 censuses as before.

### Gates run

- `bash scripts/run-wasm-gates.sh artifacts/steps/bank-design-0a` (native leg, simd128 guest,
  residency and f64 censuses, V8 EQ spill gate): ok.
- `cargo test --locked -p console-workload -p wasm-gates` and `cargo test --locked --release -p bench
  -p console-workload -p wasm-gates`: all pass; `w4_counters_match_between_wasm_and_native_simd4`
  is ignored on this eight-lane host by design (decision 1).
- The lint job's gates (`.github/workflows/qualification.yml`, "fmt, clippy, doc, and hermetic
  policy gates"): `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
  --no-deps`, and every hermetic policy script and self-test of that job, `check-lane-policy.sh`
  included: 48 of 48 ok.
- Not run: the arm64 native leg (no arm64 host here; CI's `aarch64-release` runs it).

### Measurement (W=4 wasmtime, `artifacts/steps/bank-design-0a/`)

`scripts/run-wasm-gates.sh artifacts/steps/bank-design-0a` at this commit; `w4-counters.json` and
`w4-table.md` (`bench console --bank-report-w4`, no V8 file: the V8 runner has no C1 document, so
the p50 column is "not measured"). 256 blocks per row. Beside #2132's W=8 native numbers for the
same rows (round 2, `artifacts/steps/bank-design-0/`):

| row | W | banks | pairs | BSR static | BSR/block | track BSR/block | silent skips (256 blocks) | pack groups | p50 us |
|---|---|---|---|---|---|---|---|---|---|
| c1 16 load | 8 | 10 | 0 | 20 | 20.00 | - | - | - | 125.2 (native) |
| c1 16 load | 4 | 15 | 0 | 29 | 29.000 | 16.000 | 0 | 0 | not measured |
| c1 16 typical | 8 | 10 | 0 | 20 | 20.00 | - | - | - | 125.1 (native) |
| c1 16 typical | 4 | 15 | 0 | 29 | 29.000 | 16.000 | 0 | 0 | not measured |
| c1 16 all active | 8 | 10 | 0 | 20 | 16.66 | - | - | - | 96.9 (native) |
| c1 16 all active | 4 | 15 | 0 | 29 | 27.988 | 16.000 | 259 | 0 | not measured |
| c1 16 sparse | 8 | 10 | 0 | 20 | 16.66 | - | - | - | 96.4 (native) |
| c1 16 sparse | 4 | 15 | 0 | 29 | 27.988 | 16.000 | 259 | 0 | not measured |

Against the design's W=4 "today" column (section 7): BSR 29 predicted, 29 measured. The banks are
8 track banks (`ceil(16/4)` per console rack) and 7 submix banks, the design's W=4 plan exactly:
pre over `{4 groups, aux B, aux A}` (2 banks, 6 BSR), drive over `{4 groups, aux B}` (2, 2), aux A's
drive (1, 1), the master's pre and drive (2, 4). Unlike W=8 (12 submix BSR against the design's 13),
W=4 has no gap. No W=4 row has a pack group, so no track bank packs at four lanes either (#2142's
finding holds at W=4). In the load and typical states every BSR runs; all active skips 259
unit-blocks (about 1.01 BSR per block, aux A's silent chain), against 3.34 per block at W=8.
