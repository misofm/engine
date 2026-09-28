# Accept only the launch sample rates in effect descriptors

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved: accept only the launch rates.

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R5. The ruling to record: "The engine supports exactly 44.1, 48, 88.2 and 96 kHz. The
extended research rates (176.4, 192, 352.8 and 384 kHz) are removed from every accepted set."

## Context

- **No rendering path accepts an extended rate today.** `crates/session/src/validate.rs:36-43`
  refuses any non-launch session rate (`SampleRateUnsupportedAtLaunch`). host-web boot, host-core
  and capi all compile through `session`. No native effect declares an extended quality row: see,
  for example, `crates/delay/src/lib.rs:265-270`.
- **They survive only as descriptor metadata:**
  - `crates/engine/src/lib.rs:55-84`: `EXTENDED_COMPATIBILITY_SAMPLE_RATES` and the predicate
    `is_extended_compatibility_sample_rate`. The unit test
    `sample_rate_tiers_are_exact_sorted_disjoint_and_classified` is at `:111`.
  - `crates/effect-contract/src/lib.rs:44` and `:761`: `validate_descriptor` accepts a quality row
    at a launch **or** extended rate, while still requiring all four launch rates (`:781-788`);
  - `crates/effect-package/src/wire.rs:13-14`: the descriptor-wire decoder uses the same predicate.
    This part goes with `R6-…` if that lands first.
  - `fixtures/conformance/v1`: 4 of its 11 files use extended rates. CI checks it with
    `cargo run -p conformance --example conformance_fixtures -- --check`.
- **Other uses:** no coefficient or math table depends on the extended rates (searched, not
  exhaustively). The true-peak limiter's exact box-sum bound `R <= 961` is derived for rates up to
  96 kHz (`true-peak-limiter/src/lib.rs:94-99`).
- **Shipped module:** `validate_descriptor` is in its closure, so the module may change slightly.

## Smallest closable slice

1. Delete `EXTENDED_COMPATIBILITY_SAMPLE_RATES` and its predicate, and reduce the engine test to
   the launch tier.
2. Make `validate_descriptor`, and the descriptor-wire decoder if it still exists, accept launch
   rates only. Flip every descriptor test and reference vector that accepted an extended row to
   expect refusal, and list them; `scripts/effect-descriptor-v1-reference.py` is one candidate.
3. Regenerate or trim `fixtures/conformance/v1` to launch rates, and update its manifest and
   `conformance_fixtures --check`.
4. Update AGENTS.md ("176.4 … 384 kHz are extended compatibility/research evidence only") and
   `docs/EFFECT_CONTRACT_V1.md`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web -p dsp-reference -p conformance`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `bash scripts/run-wasm-gates.sh` passes. Rates are not part of any console row's
   render.
3. **Shipped artifact.** Build base and change on one machine, as audit section 11 describes. The expected change is the removed
   extended-rate branch of `validate_descriptor`. Explain every changed function from
   `wasm-objdump -d`, and re-pin.
4. **CI routing.** `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   passes with the updated corpus. `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   `check-effect-descriptor-v1.sh` passes, if it still exists.
5. **No live claim lost.** Launch-rate descriptor acceptance and session rate refusal keep their
   tests. The only tests removed or flipped are the extended-rate ones. List them from the
   `-- --list` diff and the test diff (audit section 11).

## Dependencies

The owner ruling. It can land before or after `R6-…`.

## Standing rules for the implementer

- Commit on `codex/<issue>-launch-rates-only`. Do not run timed benchmarks.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F9. The recommendation stands; the scope is larger than
"descriptor metadata, about 50 lines".

1. **A shipped-module predicate also accepts extended rates.**
   `builtins::validate_builtin_filter_cutoff` (`crates/builtins/src/lib.rs:343-370`) admits
   176.4-384 kHz through `is_extended_compatibility_sample_rate`, and `BuiltinChain::new` relies
   on it (pinned by `builtins/tests/contract.rs:297-309`). It is reached from
   `builtins-compiler/src/lib.rs:4937` and `builtins/src/lib.rs:3217`, both in the shipped module.
   Add it to step 2. The module changes in `builtins` as well as `effect-contract`.
2. **Files the slice omits** (`rg 'EXTENDED_COMPATIBILITY_SAMPLE_RATES|is_extended_compatibility_sample_rate'`
   lists 12 files outside `engine/src/lib.rs`):
   - the conformance mock effect declares eight quality rows, four of them extended
     (`crates/conformance/src/effect.rs:99-107`); a tightened `validate_descriptor` rejects it, so
     `conformance` tests go red unless the mock is trimmed;
   - `conformance/src/block.rs`, `src/fixture.rs`, `tests/fixtures.rs` and
     `examples/conformance_fixtures.rs`;
   - `builtins/tests/response.rs`, `tests/stage.rs` (RBJ-oracle and stage tests at extended rates);
   - `engine/src/realtime/mod.rs:688`; `effect-contract/tests/response_analysis.rs:71-74`;
   - `fixtures/effect-descriptor/v1/comprehensive-b.json` and
     `fixtures/effects/runtime-v1/valid/descriptor.toml:5` (hash-checked; goes stale).

   Failure scenario: an implementer follows the listed files only and the `conformance` and
   `builtins` test binaries go red on files the draft never named.
3. **Gates.** Add `-p builtins -p conformance` to gate 1's test run, and extend gate 3's expected
   module diff to the `builtins` cutoff branch.
4. **Mobile scope: no change.** A device whose hardware rate is outside the launch set is the
   separate "no implicit SRC" rule, which this ruling does not touch.
