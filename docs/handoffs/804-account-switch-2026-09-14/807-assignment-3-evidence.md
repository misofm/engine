# #807 assignment 3 evidence

Implementation is confined to the EQ implementation and its direct DSP/state fixtures:

- `crates/parametric-eq/src/lib.rs`
  - Applies validated prepared target records on scalar and bank lanes.
  - Keeps general-band enable/kind prepared-only; dedicated HPF/LPF enables are live and retained.
  - Uses current-then-advance ramp timing and snaps exactly after the 64th update, so the target is
    first used at `A+64`.
  - Caches initial designed words and uses the cache for `FullToDefaults`; discontinuity reset
    retains parameters, snaps ramps and clears integrators.
  - Extends each channel payload from 114 to 116 words (464 bytes), with HPF/LPF enable words;
    malformed enable words, old lengths and malformed ramp trajectories refuse atomically.
  - Copies retained target semantic state/words to the existing response records.
  - Adds cfg(test)-only designer call instrumentation.
- `crates/parametric-eq/src/control.rs`: bounded render-side prepared-target decoder.
- `crates/parametric-eq/src/corpus.rs`: refreshed only the ramped-noise digest for the intentional
  timing correction.
- `crates/parametric-eq/tests/support/mod.rs`, `tests/contract.rs`, `tests/bank.rs`: 464-byte shape
  glue and focused scalar/bank application, timing, response, reset and state tests.

Focused gates:

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --locked -p parametric-eq --all-targets -- -D warnings`: pass.
- `cargo check --locked -p parametric-eq --target wasm32-unknown-unknown`: pass.
- `cargo test --locked -p parametric-eq --tests`: unit 32, analytic 7, bank 6 (1 ignored
  descriptive bench), conformance 1, contract 18, determinism 3 (1 ignored pin printer), mono
  collapse 3, response 18, silent-fixed-point 3, stationary-hoist 4, and time-domain 5 all pass.
- The resident-size measurement prints `Channel` width 1/4/8 as 656/2608/5216 bytes and
  `PreparedParametricEq` width 1/4/8 as 2120/7760/15296 bytes. The serialized state remains
  exactly 936 bytes total (8 common bytes plus two 464-byte channels), so cached resident storage is not being
  conflated with the wire payload.
- The permitted graph-compiler reference currently reports actual zero-delay canonical SHA-256
  `eb3ca77606e93cf9aa13f475415ecbf6e70ee1cdd074a0cca9e46cbb18e0ea10`; its old #805 expectation
  fails until the root-owned glue pin is refreshed. The independent size derivation is +16 bytes
  per EQ instance (936 - 920), hence +144 for the nine-EQ fixture: declared effect bytes
  8,280 -> 8,424 and both plan-byte totals 150,415 -> 150,559. No graph source was edited here.

Known scope boundary: the factory `target_preparation()` getter remains unadvertised and the
existing semantic automation route remains reachable for the later cutover assignment. Queue,
graph/rack, host and SDK delivery are untouched.

Root graph glue: independently constructed expected canonical text by changing only the preserved #805 estimate row: 8280 -> 8424 and both150415 -> 150559. SHA256 eb3ca77606e93cf9aa13f475415ecbf6e70ee1cdd074a0cca9e46cbb18e0ea10 exactly matches the compiler result in /tmp/807-track-delay-before-repin.log. Derived text /tmp/807-track-delay-derived.canonical; no baseline worktree or production graph edit needed.
