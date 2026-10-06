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
  `pub tail_and_rest: fn(sample_rate: u32, quality: EffectQuality) -> EffectTailBound`, with
  `pub struct EffectTailBound { pub tail: TailSamples, pub tail_every_peak: TailSamples, pub rest: RestBound }`
  in `effect-contract`: the three values #1329 defines, `T_decay`, `T_rest = max(T_decay, R(P*))`
  and the exact-rest bound (Amendment 1, ruling 1). It runs on the control thread; render never
  calls it.
- **D2. `RestBound`.** `pub enum RestBound { Bounded(RestSamples), Unstated }` in `effect-contract`.
  `Unstated` is the effect-side counterpart of `TailSamples::Infinite`. It is not a lasting
  variant: it exists only because the per-effect slices land one at a time, and the same stream-G
  sequence deletes it. *Retire the Infinite tail* (#1378) depends on every per-effect slice
  (#1372-#1376) and removes `Unstated` together with `Infinite`, so no release of the sequence's
  end state contains either. Its doc comment says exactly that and names #1378, and says #1378
  retires `Infinite` in both tail fields (`tail` and `tail_every_peak`).
- **D3. One source.** `QualityDescriptor::tail` is deleted. `expected_prepared_metadata` calls
  `(descriptor.tail_and_rest)(rate, quality)` and fills
  `PreparedEffectMetadata::{tail, tail_every_peak, rest}`. `EffectProgramKey` gains
  `tail_every_peak` and `rest`, for the same reason it carries `tail`. The `effect-compiler`
  mismatch check compares `tail_every_peak` and `rest` too.
- **D4. Values unchanged.** Each effect's function returns its current tail,
  `tail_every_peak: TailSamples::Infinite` and `RestBound::Unstated`; the conformance test effect
  does the same. `tail_every_peak` is `Infinite` for all eight effects, even the gate's and the
  transient shaper's `Finite(0)` and the soft clip's `Finite(29)`: `T_rest = max(T_decay, R(P*))`,
  and with `RestBound::Unstated` no `R(P*)` is derived, so no finite `T_rest` can be stated
  soundly (Amendment 1, ruling 1). No rendered bit, latency or canonical plan byte moves.
- **D5. Parameter independence.** The function takes the rate and quality only. Effect parameters are
  live or automatable, so each bound is over the parameter domain at that rate; a per-instance
  refinement is not part of the contract.

## Deliverables

1. `RestBound`, `EffectTailBound`, the descriptor field, the metadata and program-key fields
   (`tail_every_peak`, `rest`), the expected-metadata change, the compiler check.
2. A `tail_and_rest` function in each of the eight effect crates, and in the conformance test
   effect (`crates/conformance/src/effect.rs:90`, `:397`).
3. Doc: `docs/EFFECT_CONTRACT_V1.md` (where tail and rest come from).

## Authorized paths

- `crates/effect-contract/src/lib.rs`, `docs/EFFECT_CONTRACT_V1.md`
- `crates/effect-compiler/src/prepare.rs` (the mismatch check and its one unit test; Amendment 1,
  ruling 2)
- `docs/handoffs/decision-15-2026-10-05/STREAMS.md` (the `prepare.rs` hot-file row only; Amendment 1,
  ruling 2)
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
  `crates/effect-contract/tests/registry.rs` (Amendment 1, ruling 3),
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

1. **Conformance**: for every effect, at every launch rate, `metadata().tail`,
   `metadata().tail_every_peak` and `metadata().rest` of a prepared instance equal the fields of
   `(descriptor.tail_and_rest)(rate, Normal)` (`crates/conformance`, the existing per-effect
   conformance run).
2. **Mismatch refused**: an `effect-compiler` unit test where a processor reports a `rest`, or a
   `tail_every_peak`, different from the expected one gets `effect.metadata.mismatch`.
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
- Gate 2: a compiler that stops checking `rest` or `tail_every_peak` would let a processor's value
  drift from its declared function unseen; no existing test reaches `effect.metadata.mismatch`.

## Amendment 1 (root rulings, 2026-10-06)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`), in the context of decision 15 D15-4(b) and #1329 Amendment 3.
The first worker stopped before code on a spec conflict: #1329 (passed) defines three values per
node, `T_decay`, `T_rest = max(T_decay, R(P*))` (`tail_every_peak`) and `RestSamples`, and this
spec carried only two. No attempt was consumed. These rulings are binding; D1-D4, the
Deliverables, the Authorized paths, gates 1 and 2 and the Test value above are updated to match.

1. **Three values, one struct.** D1's function returns
   `EffectTailBound { tail: TailSamples, tail_every_peak: TailSamples, rest: RestBound }`, using
   the `RestBound`/`RestSamples` names #1329 and this spec established, with no version suffix.
   `PreparedEffectMetadata`, `EffectProgramKey`, `expected_prepared_metadata` and the
   `effect-compiler` mismatch check carry and compare `tail_every_peak` too, and gates 1 and 2
   cover it. D4 sets `tail_every_peak = TailSamples::Infinite` for all eight effects in this slice:
   with `RestBound::Unstated` no `R(P*)` is derived, so even a `Finite(0)` or `Finite(29)` tail
   cannot soundly state a finite `T_rest`. D2's doc names #1378 as retiring `Infinite` in both tail
   fields.
2. **Hot-file order.** #1377 goes ahead of B (#1315, #1345) and G (#1339, #1340) in the
   `crates/effect-compiler/src/prepare.rs` order: those slices are blocked, and #1377's edit there
   is one comparison and one test. The `STREAMS.md` hot-file row records this reason; the later
   slices rebase.
3. **`crates/effect-contract/tests/registry.rs`** is in scope for its struct literal (the field
   only).
4. **GitHub sync of #1377 is pending owner permission.** No GitHub write is made for this issue
   until the owner grants it; the local spec is the record meanwhile.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329), for `RestSamples` and the
  definition.

## Evidence

### Attempt 1

Implementer checkpoint. **Blocked on a required gate:** `cargo clippy --workspace --all-targets
-- -D warnings` (and `--all-features`) is red, `clippy::large_enum_variant` on
`crates/graph/src/runtime.rs` `NodeKind`: `NodeKind::Effect(GraphPreparedEffect)` holds a
`PreparedEffectMetadata` inline, and D3's two new fields (`tail_every_peak`, 16 bytes; `rest`,
24 bytes) take the variant to at least 240 bytes, over the lint's 200-byte threshold (it passed at
`cd984d844`). The fix lies outside the Authorized paths and is a root decision: box the variant's
payload in `crates/graph/src/runtime.rs` (one more indirection per effect node in render's node
table), allow the lint there with a reason, or move the bounds out of the inline metadata. No
value moves either way. Everything else below is green.

What landed:

- `effect-contract`: `RestBound { Bounded(RestSamples), Unstated }` (doc names #1378 and both tail
  fields), `EffectTailBound { tail, tail_every_peak, rest }`, `EffectDescriptor::tail_and_rest`
  (before `observations`, so that field stays last), `QualityDescriptor::tail` deleted,
  `PreparedEffectMetadata` and `EffectProgramKey` gain `tail_every_peak` and `rest`,
  `expected_prepared_metadata` calls `(descriptor.tail_and_rest)(request.sample_rate,
  request.quality)`.
- Each of the eight effects: a `tail_and_rest` returning its old tail, `tail_every_peak: Infinite`,
  `RestBound::Unstated` (D4); the doc names its slice (#1372-#1376). The conformance mock does the
  same with its `Finite(3)` (its output accumulator never rests).
- `effect-compiler`: the mismatch check compares `tail_every_peak` and `rest`.
- Conformance harness: `run_effect_conformance` compares each prepared instance's three values with
  `(descriptor.tail_and_rest)(rate, quality)` (`metadata.tail_bound`), at every declared
  (launch-rate) row of every production effect; mock fault `UndeclaredRestBound` and its row in
  `every_faulty_mock_is_detected` (`crates/conformance/tests/effect_contract.rs`, one row beyond the
  field-only list, since gate 1 lives in `crates/conformance`).
- `docs/EFFECT_CONTRACT_V1.md`: where the three values come from.
- Struct literals, field only: the listed files that needed it, plus
  `crates/effect-contract/tests/registry.rs` (ruling 3). Test doubles state their old tail,
  `Infinite`, `Unstated`.

Tests and test value (each mutation red, green on revert):

- `effect_contract` `tail_bound_tests::prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate`
  (a test descriptor with distinct per-rate values in every field). Defends D3: no shipped effect
  states a `Bounded` rest or finite `tail_every_peak`, so nothing else sees a wrong copy. Red on:
  M1 `expected_prepared_metadata` writes `rest: Unstated`; M2 calls the function at a fixed 48 kHz;
  M3 copies `tail` into `tail_every_peak`; M4 `program_key` crosses `tail_every_peak`; M5
  `program_key` writes `rest: Unstated`.
- Gate 1, conformance `metadata.tail_bound`: M6 removing the check turns the `UndeclaredRestBound`
  row red (detected only as `metadata.changed`/`metadata.exact`/`reset.semantics`). M7
  `expected_prepared_metadata` writing `tail: Infinite`: `gate-expander --test conformance` green
  without the new check (both sides of `metadata.exact` share the bad value), red with it
  (`["metadata.tail_bound"]`). Candidly: an effect that writes its own values is already caught by
  `metadata.exact` now that the program key carries them; the new check's own catch is a wrong
  `expected_prepared_metadata`, seen on real effects.
- Gate 2, `effect-compiler` `prepare::metadata_mismatch_tests::a_processor_whose_rest_or_tail_every_peak_drifts_is_refused`
  (production EQ wrapped so its metadata drifts; nine-track fixture; undrifted control prepares 9).
  Red on dropping either new comparison (two runs). No earlier test reached
  `effect.metadata.mismatch`.
- The four `quality.tail` reads (gate, transient shaper, soft clip, limiter) moved onto
  `tail_and_rest(...).tail` with the same expected value.

Gates run at this checkpoint:

- PASS: `cargo fmt --all -- --check`; test-debug-b (the gate-3 DSP command, exit 0); test-debug-a
  (exact workspace command from `qualification.yml`, exit 0); `conformance_fixtures -- --check`;
  `check-graph-determinism.sh` (100/100); `check-effect-contract.sh` (8 production factories);
  `check-workspace-policy.sh`; `check-realtime-policy.sh`; `check-lane-policy.sh`;
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- FAIL: workspace clippy, default and `--all-features` (above).
- Not run (stopped on the blocker): `check-cross-targets.sh`, `run-wasm-gates.sh`, the worklet
  chain, capi audit/ABI (no C ABI or preparation byte is meant to change; graph canonical text
  copies `metadata.tail` only).

No rendered bit, latency or tail value moved: every value is the earlier one, and the DSP and
workspace suites pass unchanged.
