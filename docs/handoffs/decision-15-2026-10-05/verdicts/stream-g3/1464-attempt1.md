PASS

# #1464 attempt 1: adversarial verdict

Commit reviewed: `da99d396b` (parent `ae3fcbf16`), `git diff ae3fcbf16 da99d396b` (34 files).
Built and tested from an export of `da99d396b` (`/tmp/claude-1002/v1464/tree`,
`CARGO_TARGET_DIR=/tmp/claude-1002/v1464/target`). The worktree was not touched.

Verdict: the slice does what K-D1 to K-D6 ask. `EffectTailBound` is renamed `NodeTailBound` once,
with no alias. A grep outside `.github/ISSUE_SPECS` and the handoff history finds no
`EffectTailBound`, no `InputSectionBound` and no `type`/`use ... as` alias. The one remaining code
hit is a historical re-pin comment, `tools/audit/src/fixture_builtins.rs:5326`. The new types are
exactly as K-D2 gives them.
`PreparedEffectMetadata` and `EffectProgramKey` carry `composition`.
`expected_prepared_metadata` and `program_key` copy it, and the mismatch check compares it
(`crates/effect-compiler/src/prepare.rs:635`). The registry refuses (e) and (f) with
`effect.tail_bound.inconsistent`. `InputSectionBound` is deleted, and every builtin entry point
returns `NodeTailBound`, with the `Option` returns kept. All eight effects, the conformance mock and
every builtin bound state `Unstated`. Every gate is green. `audit capi` gives `pcm_digest`
`cb10fbface44a3a4`, the value recorded in the stream-G and G2 batch verdicts, with 0 allocations
and 0 syscalls. Every claimed mutant goes red on exactly the named tests. No render-owned memory
grows. There is no BLOCKER and no MAJOR.

## BLOCKER

None.

## MAJOR

None.

## MINOR

- **m1. The K4 test claims that it checks `max`'s #1329 rule, but it does not check that rule's
  "larger" half. The moved `max` has no test that discriminates it.**
  `crates/effect-contract/tests/tail_bound.rs:286-288` says "The other three values keep #1329's
  componentwise rule" and then asserts `left.max(unstated).tail == Finite(0)` (both tails are 0)
  and `left.max(UNBOUNDED) == UNBOUNDED`. Those two asserts check only the absorbing
  `Infinite`/`Unstated` case. I ran two mutants of `NodeTailBound::max`
  (`crates/effect-contract/src/lib.rs:252`):
  - X3: a finite `tail` is taken by minimum.
  - X4: the rest's `any_sanitized_input` is taken from `self` only.

  Both stay green on `tail_contract` in release (13/13). Both also stay green on every `builtins`,
  `builtins-compiler` and `effect-contract` lib and integration test in debug with the test-support
  features (295/295). The gap is older than this slice: `InputSectionBound::max` was moved
  verbatim, and no test was removed. But the new test's comment states a coverage that it does not
  give. A stereo section whose `max` took the shorter channel's `T_decay` would under-report its
  tail. That would make PDC and silence skipping wrong, and nothing would turn red.
  Fix (follow-up): give `left` and `stated(...)` different finite `tail`, `tail_every_peak` and
  `rest` values on each side. Then assert the componentwise result in both operand orders.
- **m2. `NodeTailBound` has two different "max" operations.** The type derives a lexicographic
  `PartialOrd`/`Ord` (`crates/effect-contract/src/lib.rs:211`, inherited from `EffectTailBound`).
  It also gets the inherent componentwise `max` that this slice moved from `InputSectionBound`,
  which derived no `Ord`. `a.max(b)` resolves to the inherent method. But `std::cmp::max(a, b)`,
  `Ord::max` and `Iterator::max()` compile and silently pick the lexicographic order, and that
  order is a bound of neither channel. #1379's later slices fold node bounds, so this is a
  realistic trap. I removed `Hash, Ord, PartialOrd` from the derive and ran
  `cargo check --workspace --all-targets --all-features`. It succeeds, so nothing uses that order.
  Fix (follow-up): derive only `Clone, Copy, Debug, Eq, PartialEq` on `NodeTailBound`.
  `CompositionBound` and its parts keep `Ord`, which `EffectProgramKey` and `max` need.

## NIT

- **n1. No admitted twin pins the boundary of rules (e) and (f).** Two mutants survive every
  `effect-contract` and `effect-compiler` test:
  - X1: rule (e) reads `tail_every_peak` in place of `tail`.
  - X2: rule (f) refuses a `Zero` tail gain under a `Millibels` peak gain.

  Both are over-refusals. They would show at registry build in a later slice that states such a
  bound. That makes them loud, so this is a NIT. The fix is one more admitted descriptor in
  `crates/effect-contract/tests/registry.rs`. It states today's launch shape (finite `tail`,
  `tail_every_peak: Infinite`, `rest: Unstated`) with a stated composition whose `tail_gain` is
  `Zero` under a `Millibels` peak gain.
- **n2. The K4 test uses an inconsistent operand.** `tail_bound.rs:250` (`right`) states
  `peak_gain: Zero` with `tail_gain: Millibels(-900)`, which the registry refuses under rule (f).
  Consistent operands give the same discrimination: left `(-300, -900)` and right `(Zero, Zero)`
  still order `Zero` below `Millibels` in both gain fields.
- **n3. A doc comment was not updated.** `crates/effect-contract/src/lib.rs:2611`
  (`RegisteredTailBound::bound`) still says "The three values". The bound now has four fields.
- **n4. A size comment gives the wrong "before" value.** `tools/audit/src/fixture_builtins.rs:333-334`
  says "(`NodeTailBound`: ... 88 bytes; 72 before)". The bound grew from 56 to 88 bytes. The entry
  `(Box<str>, _)` grew from 72 to 104. I measured these with a standalone layout probe of the same
  types: bound 56 -> 88, entry 72 -> 104, `CompositionBound` 32.
- **n5. The re-pin history is out of order.** The #1464 re-pin note (`fixture_builtins.rs:5320`) is
  placed before the #1329 note, not after it as the last entry.
- **n6. One evidence wording is not exact.** The record says "the browser expected resources did
  not move". That is true of `expected.json`. The measured `builtinRetainedBytes` row moved by
  exactly the tail-entry delta. The browser fixture has one track, and two wasm32 entries grew from
  64 to 96 bytes, so the row is now 2,017 of its 2,048 ceiling, with 31 bytes free. No re-pin is
  needed, because the row is a ceiling row and the value grew. But the next slice that grows a
  builtin retained byte will exceed this ceiling.

## Focus items

- **K-D decisions and K gates.** K-D1 to K-D6 are implemented as specified, and no name is
  refined. K1: the existing assertions pass with only the rename applied. In `tail_contract.rs` the
  only non-rename edit is mechanical: `Some(RestSamples ..)` becomes `RestBound::Bounded(..)`, and
  six `.rest.expect(msg)` become `stated_rest(.rest, msg)`. K2: the tail_bound fixture states a
  distinct, consistent `Stated` per rate. Both `metadata.composition` and `key.composition` are
  compared, and the forgery table has 17 rows. K3: both refusals use the typed code and break only
  at 96 kHz. K4: the composition rule is covered. The #1329 half is m1. K5: see below.
- **Files outside the list.** These edits are all rename-only or field-only, and the spec's
  re-grep clause and Deliverable 2 ("every user") cover them:
  - `effect-contract/tests/support/mod.rs`: rename plus the field, in the helper.
  - `builtins/examples/input_bound_budget.rs`: import plus one `Vec<_>` type.
  - `rack-compiler/src/lib.rs:459`: one `EffectProgramKey {` test literal, field only.
  - `rack/tests/live_control_bank.rs:40`: one `EffectProgramKey {` test literal, field only.
  - `effect-contract/tests/tail_bound.rs`: this is the only possible referent of K2's
    `tail_bound_tests`, because no module of that name exists.
- **K5 re-pins.** Each one is checked:
  - `BOXED_TAIL_ENTRY_BYTES` 72 -> 104 equals `size_of::<(Box<str>, NodeTailBound)>()`, and the
    audit layout check passes.
  - `resources.jsonl` moves only in `engine_owned_processor_payload_bytes` and
    `engine_owned_retained_payload_bytes`, by +64 per track (two vectors x 32): 2,069 -> 2,133,
    8,348 -> 8,604, and 139,035,061 -> 143,229,429 = +64 x 65,537. All nine rows check, including
    the meter-sum rows. `maximum_single_allocation_bytes` (1,080 x tracks, the input-entry vector)
    and the allocation counts do not change. The file length stays 2,370.
  - `MANIFEST.tsv` changes only in its `resources.jsonl` row, and that row's digest is the file's
    sha256.
  - Both manifest-identity pins equal `sha256(MANIFEST.tsv)` = `cd2b9fe6...`.
  - The reason is recorded once, at the `fixture_builtins.rs` identity pin, as for every earlier
    re-pin. The `builtins_graph.rs` twin keeps its usual bare value.
  - `check-builtins-fixtures.sh` regenerates all 50 files byte-identical, so no PCM, meter,
    response or benchmark fixture moved. `fixtures/graph` and browser `expected.json` do not move.
- **Render-owned memory.** It does not grow:
  - The builtins `tails` and `seal` are dropped at attach. `into_graph_artifact_with_banks` keeps
    `graph`, processors, observers, `track_controls` and `meter_consumers` only.
  - Since #1461 no processor holds `PreparedEffectMetadata`.
  - `EffectBankStage` and `LiveControlEffectBankStage` keep the processor and scalars only
    (`crates/rack/src/lib.rs:753`, `:947`).
  - `PreparedBankMetadata`, `GraphPreparedEffect` and `GraphPreparedEffectBank` are
    control-side.
  - No builtins render struct holds a bound. The `OnceCell` of #1329's early attempts is gone.

  Control-side estimates that use `size_of` of these records grow by 32 bytes for each effect or
  bank. Nothing pins them.
- **`stated_rest`** (`crates/builtins/tests/tail_contract.rs:31`) is a local, behavior-equivalent
  replacement for `Option::expect`: it panics with the same message on `Unstated`. No accessor of
  that kind exists on `RestBound`. M1 proves that the reads still discriminate: 7 tests go red.
- **The open item (K3's last clause).** Acceptable. No finding.
  - Acceptance of the eight effects is exercised wherever the launch registry is built
    (`build_launch_native_effect_registry`, `prepare.rs:252`): `native_session.rs`, the capi
    audit, and `check-effect-contract.sh` (8 production factories, 0 failed gates).
  - Acceptance of the mock is exercised by `run_effect_conformance` and by graph-compiler tests that
    register `DualAccumulatorDelayFactory`.
  - "States `Unstated`" is visible in the code: each of the nine `tail_and_rest` bodies is a
    branch-free constant with `CompositionBound::Unstated`.
  - A committed pin of `Unstated` would defend against no plausible defect. Its only red would be
    the later slices' deliberate restatement (#1372-#1376, #1465, #1467). AGENTS.md's test-value
    rule refuses such a test.

## Test value (new or extended tests)

- `registry.rs::a_stated_composition_with_an_infinite_tail_is_refused`: goes red when the registry
  admits a `Stated` composition beside an `Infinite` tail (M6). No other test validates that.
- `registry.rs::a_tail_gain_above_the_peak_gain_is_refused`: goes red when the registry admits
  `tail_gain > peak_gain`, including a `Millibels` tail gain over a `Zero` peak gain (M7, M8, X5).
- `tail_bound.rs::max_states_a_composition_only_when_both_channels_state_one`: goes red when `max`
  keeps a `Stated` side beside an `Unstated` one, or takes any composition value by minimum or from
  one fixed side (M9, M10, X6). It does not defend the #1329 half (m1).
- `prepare.rs` forgery table, row `composition`: goes red when the mismatch check stops comparing
  `composition` (M5). This is the only test that compares it.
- `tail_bound.rs::prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` (extended):
  goes red when `expected_prepared_metadata` or `program_key` drops `composition` (M3, M4).

## Mutations re-run

M1 and M2 ran against `tail_contract` in release. M3 to M10 ran against every `effect-contract` and
`effect-compiler` lib and integration test in debug. Each mutant was applied, run and restored, and
the baseline was green before and after.

| # | defect | result |
|---|---|---|
| M1 | `bound_from_cascade` gives `rest: Unstated` | red: 7 tests (`live_bound_carries_every_term...`, `live_bound_covers_every_scanned...`, `live_bound_holds_on_the_real_kernel...`, `live_bound_table_is_the_computed...`, `live_bounds_leave_headroom...`, `live_rest_bounds_are_within...`, `tail_every_peak_is_the_rest...`) |
| M2 | live table crosses `peak_plus_24_dbfs` and `any_sanitized_input` | red: `live_bound_table_is_the_computed_live_bound_at_every_launch_rate` only |
| M3 | `expected_prepared_metadata` writes `Unstated` | red: `prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` only |
| M4 | `program_key` writes `Unstated` | red: the same test only |
| M5 | mismatch check drops `composition` | red: `a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused` only |
| M6 | rule (e) dropped | red: `a_stated_composition_with_an_infinite_tail_is_refused` only |
| M7 | rule (f) dropped | red: `a_tail_gain_above_the_peak_gain_is_refused` only |
| M8 | `(Millibels, Zero)` admitted | red: the same test only |
| M9 | `max` keeps a `Stated` side beside `Unstated` | red: `max_states_a_composition_only_when_both_channels_state_one` only |
| M10 | `max` takes `peak_gain` by minimum | red: the same test only |
| X1 | rule (e) reads `tail_every_peak` | survives (n1) |
| X2 | rule (f) refuses `Zero` under `Millibels` | survives (n1) |
| X3 | `max` takes a finite `tail` by minimum | survives in `tail_contract` release and in `builtins`, `builtins-compiler` and `effect-contract` debug (m1) |
| X4 | `max` takes `any_sanitized_input` from `self` | survives the same set (m1) |
| X5 | rule (f) compared the wrong way round | red: 4 effect-contract tests |
| X6 | `max` takes `decay` from `self` | red: the K4 test |

A note on my harness: the first M5 run also showed `tail_bound` red. The cause was my revert, which
restored the old mtime and so left the M4 build fresh. After a `touch`, a clean re-run showed only
the forgery table red, and the baseline was green.

## Gates run (from the export)

- `test-debug-a`, CI's exact commands: `builtins-compiler --no-run`, the `--all-targets` workspace
  run with CI's features, and the doctests. EXIT 0; 1,490 passed, 0 failed.
- `test-debug-b`, CI's exact commands: the DSP crates `--all-targets`, the doctests and
  `conformance_fixtures -- --check`. EXIT 0; 903 passed, 0 failed.
- `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`:
  13 passed.
- `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
  `cargo test --release -p audit -p bench -p console-workload`: 113 passed.
- `audit capi`: `pcm_digest` `cb10fbface44a3a4`. Allocations, deallocations, locks, syscalls and
  violations are all 0.
- `check-builtins-fixtures.sh`: ok (50 files).
- `check-effect-contract.sh`: ok (8 prepared configurations, 0 failed gates).
- `run-wasm-gates.sh` (native, wasm simd128 and the V8 spill gate): ok.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` (CI's form): clean.
- `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`: clean.
- `cargo fmt --check`: clean.
- `check-workspace-policy.sh`: ok. `check-realtime-policy.sh`: ok (89 regions, 25 files).
- `check-cross-targets.sh`: PASS.
- The worklet chain: `build-web-audioworklet.sh --named-twin`,
  `check-web-audioworklet.sh --without-metadata-regeneration`,
  `check-browser-expected-resources.py --artifacts` (all rows within budget, 32 red self-test
  mutations), `check-scalar-oracle-absent.py --wasm`, and `test-web-audioworklet.sh` with a private
  `TMPDIR` (no leftovers). All exit 0.
- Not run: `check-graph-determinism.sh`, which the spec does not list (`fixtures/graph` is
  unchanged), and the AArch64 tests, which run in CI only.

Acked-batch question: this slice adds and changes no queue. Not applicable.
