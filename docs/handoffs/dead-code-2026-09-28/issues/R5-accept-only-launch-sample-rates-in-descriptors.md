# Accept only the launch sample rates in effect descriptors

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
