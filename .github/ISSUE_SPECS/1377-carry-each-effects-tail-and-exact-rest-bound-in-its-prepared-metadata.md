# Carry each effect's tail and exact-rest bound in its prepared metadata

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Every native effect gets one place where its tail and its exact-rest bound are computed, per rate,
on the control thread, and carried into its prepared metadata. Values do not change in this slice:
it moves today's declared tails into that home, so each per-effect slice only replaces its own
function body.

## Context

- **Today the tail is a static.** Each effect writes `tail` into a `const fn quality(rate)`
  `QualityDescriptor` (`crates/effect-contract/src/lib.rs:513-521`): EQ
  `crates/parametric-eq/src/lib.rs:648-653` (`Infinite`), compressor `crates/compressor/src/lib.rs:291-296`
  (`Infinite`), limiter `crates/true-peak-limiter/src/lib.rs:235-243` (`Infinite`), multiband
  `crates/multiband-compressor/src/lib.rs:343-349` (`Infinite`), delay `crates/delay/src/lib.rs:257-264`
  (`Infinite`), gate `crates/gate-expander/src/lib.rs:237-244` (`Finite(0)`), transient shaper
  `crates/transient-shaper/src/lib.rs:147-152` (`Finite(0)`), soft clip `crates/soft-clip/src/lib.rs:174-179`
  (`Finite(29)`).
- **The metadata.** `expected_prepared_metadata` (`effect-contract/src/lib.rs:2540-2565`) is "the
  sole conforming metadata" and copies `quality.tail` (`:2560`). `PreparedEffectMetadata`
  (`:1112-1125`) and `EffectProgramKey` (`:1171-1186`, `program_key` `:1187-1204`) carry `tail`.
  `effect-compiler` refuses a prepared effect whose metadata differs from the expected one
  (`crates/effect-compiler/src/prepare.rs:486-506`, `effect.metadata.mismatch`). `graph-compiler`
  reads `metadata.tail` (`crates/graph-compiler/src/compile.rs:271`).
- *State a bounded tail and an exact-rest bound for every node* (#1329) defines the tail contract
  and `RestSamples` in `effect-contract`, and delivers them for the builtins.
- A `const` descriptor cannot hold a value computed from a designer in `f64` (`math::tan` and
  friends are not `const`).

## Decisions frozen for this slice

- **D1. The home.** `EffectDescriptor` gains
  `pub tail_and_rest: fn(sample_rate: u32, quality: EffectQuality) -> (TailSamples, RestBound)`.
  It runs on the control thread; render never calls it.
- **D2. `RestBound`.** `pub enum RestBound { Bounded(RestSamples), Unstated }` in `effect-contract`.
  `Unstated` is the effect-side counterpart of `TailSamples::Infinite`. It is not a lasting
  variant: it exists only because the per-effect slices land one at a time, and the same stream-G
  sequence deletes it. *Retire the Infinite tail* (#1378) depends on every per-effect slice
  (#1372-#1376) and removes `Unstated` together with `Infinite`, so no release of the sequence's
  end state contains either. Its doc comment says exactly that and names #1378.
- **D3. One source.** `QualityDescriptor::tail` is deleted. `expected_prepared_metadata` calls
  `(descriptor.tail_and_rest)(rate, quality)` and fills `PreparedEffectMetadata::{tail, rest}`.
  `EffectProgramKey` gains `rest`, for the same reason it carries `tail`. The `effect-compiler`
  mismatch check compares `rest` too.
- **D4. Values unchanged.** Each effect's function returns its current tail and `RestBound::Unstated`.
  No rendered bit, latency or canonical plan byte moves.
- **D5. Parameter independence.** The function takes the rate and quality only. Effect parameters are
  live or automatable, so each bound is over the parameter domain at that rate; a per-instance
  refinement is not part of the contract.

## Deliverables

1. `RestBound`, the descriptor field, the metadata and program-key fields, the expected-metadata
   change, the compiler check.
2. A `tail_and_rest` function in each of the eight effect crates, and in the conformance test
   effect (`crates/conformance/src/effect.rs:90`, `:397`).
3. Doc: `docs/EFFECT_CONTRACT_V1.md` (where tail and rest come from).

## Authorized paths

- `crates/effect-contract/src/lib.rs`, `docs/EFFECT_CONTRACT_V1.md`
- `crates/effect-compiler/src/prepare.rs` (the mismatch check)
- The descriptor and `quality` functions only in: `crates/parametric-eq/src/lib.rs`,
  `crates/compressor/src/lib.rs`, `crates/true-peak-limiter/src/lib.rs`,
  `crates/multiband-compressor/src/lib.rs`, `crates/delay/src/lib.rs`,
  `crates/gate-expander/src/lib.rs`, `crates/transient-shaper/src/lib.rs`, `crates/soft-clip/src/lib.rs`
- `crates/conformance/src/effect.rs`
- Tests that read `quality.tail`, moved onto `tail_and_rest` with the same expected value:
  `crates/true-peak-limiter/src/lib.rs:5124`, `crates/gate-expander/tests/contract.rs:37`,
  `crates/transient-shaper/tests/contract.rs:35`, `crates/soft-clip/tests/contract.rs:32`
- Struct literals that must gain the new field (`tail_and_rest` on an `EffectDescriptor`, `rest`
  on a `PreparedEffectMetadata` or an `EffectProgramKey`, `tail` removed from a
  `QualityDescriptor`), that field only: `crates/effect-contract/tests/response_analysis.rs`,
  `crates/conformance/tests/effect_contract.rs`, `crates/effect-compiler/tests/native_session.rs`,
  `crates/true-peak-limiter/tests/observation.rs`, `crates/compressor/tests/native_points.rs`,
  `crates/transient-shaper/src/corpus.rs`, `crates/builtins-compiler/src/lib.rs`,
  `crates/host-core/src/control_preparation.rs`, `crates/graph/src/lib.rs`,
  `crates/graph-compiler/src/lib.rs`, `crates/graph-compiler/src/tests/bank_padding.rs`,
  `crates/graph-compiler/src/tests/console_banking.rs`, `crates/rack-compiler/src/lib.rs`,
  `crates/rack/src/lib.rs`, `crates/rack/tests/live_control_bank.rs`, `hosts/host-web/src/tests.rs`.
  The implementer re-greps for each struct name before starting; a literal missed here is in
  scope for the field only.
- this spec

## Non-goals

- Any new value (the per-effect slices). Any graph or C ABI change. Stream A owns payload code in the
  same effect crates; touch only the descriptor and `quality` functions.

## Hazards

- *Validate each effect descriptor once per type, not once per prepared instance* (#1330, stream J)
  also edits descriptor validation; whichever lands second rebases.

## Objective gates

1. **Conformance**: for every effect, at every launch rate, `metadata().rest` and `metadata().tail` of
   a prepared instance equal `(descriptor.tail_and_rest)(rate, Normal)`
   (`crates/conformance`, the existing per-effect conformance run).
2. **Mismatch refused**: an `effect-compiler` unit test where a processor reports a `rest`
   different from the expected one gets `effect.metadata.mismatch`.
3. No bit moves: the existing suites, unchanged.
   - `cargo test --locked --all-targets -p lane -p math -p effect-runtime -p delay -p compressor -p multiband-compressor -p gate-expander -p true-peak-limiter -p transient-shaper -p soft-clip -p parametric-eq -p builtins -p dsp-reference -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/check-graph-determinism.sh`, `bash scripts/check-effect-contract.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: an effect whose prepare writes a tail or rest of its own instead of its declared function
  is red; nothing compares metadata to a computed bound today.
- Gate 2: a compiler that stops checking `rest` would let a processor's `rest` drift from its declared
  function unseen; the existing mismatch test covers only the older fields.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329), for `RestSamples` and the
  definition.
