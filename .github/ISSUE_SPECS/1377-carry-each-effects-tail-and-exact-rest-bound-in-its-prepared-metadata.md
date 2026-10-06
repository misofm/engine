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

1. **Conformance** (as amended by the root ruling of 2026-10-06, see "Follow-up"): for every
   effect, at every launch rate, `metadata().tail`, `metadata().tail_every_peak` and
   `metadata().rest` of a prepared instance equal the fields of
   `(descriptor.tail_and_rest)(rate, Normal)`. Two existing checks hold this, and no separate
   conformance check does: the per-effect conformance run's `metadata.exact` compares the prepared
   program key, which carries all three fields (D3), with the one from
   `expected_prepared_metadata`; and `effect-contract`'s `tail_bound_tests` (with three
   `graph-compiler` launch fixture tests) checks that `expected_prepared_metadata` and the program
   key copy the function's values for each rate.
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

- Gate 1: an effect whose prepare writes a tail, tail over every peak or rest of its own instead
  of its declared function is red in `metadata.exact` (D3 put the fields into the program key), and
  a wrong `expected_prepared_metadata` is red in `tail_bound_tests` (and, for a wrong `tail`, in
  three `graph-compiler` launch fixture tests). Before this slice nothing compared metadata to a
  computed bound.
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

## Amendment 2 (root ruling, 2026-10-06)

Made by the decision-15 root coordinator under the owner's no-shortcuts delegation
(`no-shortcuts-correctness-first`). Attempt 1 (`d0af9d53d`) added `tail_every_peak` and `rest` to
`PreparedEffectMetadata`, which `graph`'s render node table holds inline
(`runtime::NodeKind::Effect(GraphPreparedEffect)`), so the variant passed clippy's
`large_enum_variant` limit and workspace clippy went red outside this spec's paths. Root refused
boxing the variant or allowing the lint: control-only data must not live in render-owned memory
(#1329 ruling R5). These rulings are binding:

1. **A prerequisite slice first.** *Keep only render-read effect fields in the render node table*
   (#1460) lands first: the render node keeps exactly the fields render reads, and
   `PreparedEffectMetadata` lives in the prepared plan's control-side effect table keyed by node.
   Attempt 1's commit was reverted (`23a32823a`, history kept) and is re-applied on top of #1460.
2. **Where D3's fields live.** `tail_every_peak` and `rest` live in `PreparedEffectMetadata` in
   that control-side table, never in the render node. (Note, 2026-10-06: each effect's prepared
   processor still holds its own `PreparedEffectMetadata` copy, which render-owned memory carries;
   *#1461* removes that copy.) This slice makes no graph change of its
   own: the "Any graph or C ABI change" non-goal stands, and `crates/graph/src/lib.rs` stays in the
   Authorized paths for struct literals only.
3. **Attempts.** Attempt 1 was stopped by a blocker outside its paths and is not counted: the next
   verified run of this slice remains attempt 1.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329), for `RestSamples` and the
  definition.
- *Keep only render-read effect fields in the render node table* (#1460), for the control-side
  table (Amendment 2).

## Evidence

### Attempt 1

**History.** First run `d0af9d53d` (2026-10-06) stopped on workspace clippy
(`clippy::large_enum_variant` on `graph::runtime::NodeKind`: `NodeKind::Effect` held a whole
`PreparedEffectMetadata` inline, and D3's two fields took the variant past the lint's limit).
Root reverted it (`23a32823a`, history kept) and landed #1460 first (Amendment 2). This record is
for the re-apply: `d0af9d53d` cherry-picked onto `320baffce` (#1460 attempt 1) as `a5b4d0727`.
Amendment 2 keeps this the same attempt 1.

**What changed from `d0af9d53d`.** No code. The cherry-pick applied cleanly to every source file;
the only conflict was this spec. After #1460, `PreparedEffectMetadata` reaches the graph only in
`PreparedGraphPlan::effects` (`GraphPreparedEffect`, the control-side table); the render node is
`runtime::EffectNode { processor, quantum }`, so D3's `tail_every_peak` and `rest` are never in the
render node table (Amendment 2, ruling 2). `crates/graph/src/lib.rs` changes in test struct
literals only.

What landed (as in `d0af9d53d`):

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
  field-only list, since gate 1 lives in `crates/conformance`). *Removed by the follow-up (root
  ruling, 2026-10-06): superseded, see "Follow-up".*
- `docs/EFFECT_CONTRACT_V1.md`: where the three values come from.
- Struct literals, field only: the listed files that needed it, plus
  `crates/effect-contract/tests/registry.rs` (ruling 3). Test doubles state their old tail,
  `Infinite`, `Unstated`.

**Sizes** (x86-64 `size_of`, measured with a temporary test removed before commit; base
`320baffce` against head):

| Type | Base | Head |
|---|---|---|
| `NodeKind`, `RuntimeOp`, `RuntimeUnit` | 32, 112, 248 | 32, 112, 248 |
| `LiveControlEffect`, `EffectNode` | 128, 24 | 128, 24 |
| `GraphPreparedEffect` (control side) | 200 | 240 |
| `PreparedEffectMetadata` | 104 | 144 |
| `EffectProgramKey` | 112 | 152 |
| `PreparedBankMetadata` | 120 | 160 |
| `EffectDescriptor` | 112 | 120 |
| `QualityDescriptor` | 64 | 48 |

The last four rows are the verifier's x86-64 measurements (verdict n3), added by the follow-up.
None of them is render-owned storage; `PreparedBankMetadata` is what each bank processor's
`metadata()` returns by value.

The render node table does not move. `graph-compiler`'s live-control owner charge is
`EFFECT_RENDER_NODE_BYTES` (#1460 D4), so no resource estimate moves.

**Tests and test value** (each mutation re-run on `a5b4d0727`; red with the defect, green on
revert):

- `effect_contract` `tail_bound_tests::prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate`
  (a test descriptor with distinct per-rate values in every field). Defends D3: no shipped effect
  states a `Bounded` rest or finite `tail_every_peak`, so nothing else sees a wrong copy. Red on
  each of: M1 `expected_prepared_metadata` writes `rest: Unstated`; M2 calls the function at a
  fixed 48 kHz; M3 copies `tail` into `tail_every_peak`; M4 `program_key` crosses
  `tail_every_peak`; M5 `program_key` writes `rest: Unstated`.
- Gate 1, conformance `metadata.tail_bound`: M6 (the check disabled) turns
  `every_faulty_mock_is_detected` red ("fault UndeclaredRestBound was detected as
  ["metadata.changed", "metadata.exact", "reset.semantics"], not "metadata.tail_bound""). M7
  (`expected_prepared_metadata` writes `tail: Infinite`): `gate-expander --test conformance` is
  red with the check (`launch gate failures: ["metadata.tail_bound"]`) and green without it (both
  sides of `metadata.exact` share the bad value). Corrected by the verdict (m1): the check has no
  unique catch. `metadata.exact` catches an effect that writes its own value, and
  `tail_bound_tests` (with three `graph-compiler` fixture tests for M7) catches a wrong
  `expected_prepared_metadata`. The follow-up removed the check (root ruling, 2026-10-06).
- Gate 2, `effect-compiler` `prepare::metadata_mismatch_tests::a_processor_whose_rest_or_tail_every_peak_drifts_is_refused`
  (production EQ wrapped so its metadata drifts; nine-track fixture; undrifted control prepares 9).
  Red on dropping the `tail_every_peak` comparison, and red on dropping the `rest` comparison (two
  runs). No earlier test reached `effect.metadata.mismatch`.
- The four `quality.tail` reads (gate, transient shaper, soft clip, limiter) moved onto
  `tail_and_rest(...).tail` with the same expected value.

**Gates at `a5b4d0727`, all PASS:** `cargo fmt --all -- --check`; `cargo clippy --locked
--workspace --all-targets [--all-features] -- -D warnings` (both, no allow);
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; test-debug-a and
test-debug-b (the `qualification.yml` commands); `conformance_fixtures -- --check`;
`graph_fixture --check`; `check-graph-determinism.sh` (100/100); `check-workspace-policy.sh`;
`check-realtime-policy.sh`; `check-lane-policy.sh`; `audit capi` (0 allocations, 0
deallocations, 0 locks, 0 syscalls, 0 violations, `pcm_digest` `cb10fbface44a3a4`, the value #1460
recorded at its base and head); `check-capi-abi.sh` and its `--self-test`;
`check-builtins-fixtures.sh` (50 files); `check-effect-contract.sh` (8 production factories, 0
failed gates); `check-cross-targets.sh` (PASS, the known #1018 iOS memset rows only);
`run-wasm-gates.sh --without-v8-spill --without-native`; the worklet chain (fresh output
directories, `build-web-audioworklet.sh --named-twin`, `strip-wasm-names.py check`,
`check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
`test-web-audioworklet.sh`). AArch64 runs in CI only.

No rendered bit, latency, tail value, fixture, digest or resource byte count moved; nothing was
re-pinned.

**Open.**

- Each effect's prepared processor keeps its own `PreparedEffectMetadata` copy (for example
  `PreparedGate::metadata`, `crates/gate-expander/src/lib.rs`), which its `metadata()` returns for
  the prepare-time check. That copy sits in the processor's heap object, which render owns, and
  D3's two fields add 40 bytes to it. The copy predates this slice and #1460 left processors out
  of scope; whether it is control-only data in render-owned memory (#1329 ruling R5) is for root.
  Root ruled that it is: *#1461* removes the processor's copy.
- GitHub sync of #1377 waits for owner permission (Amendment 1, ruling 4).

### Follow-up (verdict MINOR/NITs; root ruling, 2026-10-06)

Attempt 1 passed (`/home/bl/misofm/submix-verdicts/1377-attempt1.md`: one MINOR, three NITs).

**Root ruling (2026-10-06, binding), on verdict m1.** Remove gate 1's conformance check
`metadata.tail_bound`, the conformance mock's `UndeclaredRestBound` fault and its row in
`every_faulty_mock_is_detected`. They are superseded: D3 put the three values into
`EffectProgramKey`, so `metadata.exact` refuses an effect that writes its own value, and
`tail_bound_tests` (with three `graph-compiler` launch fixture tests) refuses a wrong
`expected_prepared_metadata`. AGENTS.md: a change that supersedes a test deletes it. Gate 1 and
its test value above are restated to name what covers each defect, and the attempt-1 record's
sentence is corrected.

What changed:

- `crates/conformance/src/effect.rs`: the `metadata.tail_bound` comparison in
  `run_effect_conformance`, the `UndeclaredRestBound` variant and its `metadata()` branch, and the
  then-unused `RestSamples` import are removed. The mock's `tail_and_rest` stays (D1, D4).
- `crates/conformance/tests/effect_contract.rs`: the `UndeclaredRestBound` row is removed.
- `docs/EFFECT_CONTRACT_V1.md`: the harness sentence names `metadata.exact` (program key) in
  place of `metadata.tail_bound`; n1, the opening field list names `tail_every_peak` and `rest`;
  n2, the `tail_every_peak` bullet names `EffectTailBound`, `PreparedEffectMetadata` and
  `EffectProgramKey`.
- This record: n3, four rows added to the size table; Amendment 2 ruling 2 and "Open" point to
  #1461 for each processor's own metadata copy.

**The defects stay red elsewhere** (each re-run once on the follow-up tree, then reverted):

- M6-style, an effect whose prepared metadata writes its own `rest`
  (`DualAccumulatorDelay::metadata` sets `rest: Bounded(RestSamples::ZERO)` for every fault):
  `conformance --test effect_contract` `correct_mock_passes_every_enabled_conformance_gate` red,
  `["metadata.changed", "metadata.exact", "reset.semantics"]`.
- M7, `expected_prepared_metadata` writes `tail: TailSamples::Infinite`: red in
  `effect-contract` `tail_bound_tests::prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate`
  and in `graph-compiler`'s `launch_gate_expander_fixture_retains_width_correct_banks_and_scalar_fallbacks`,
  `launch_soft_clip_fixture_closes_banks_tails_pdc_support_and_transactional_caps` and
  `launch_transient_shaper_fixture_closes_banks_tails_pdc_and_transactional_caps` (90 passed,
  3 failed); `gate-expander`, `soft-clip` and `transient-shaper` `--test conformance` green, as
  expected with the check gone.

Gates on the follow-up tree (both follow-ups together), all PASS: `cargo fmt --all -- --check`;
`cargo clippy --locked --workspace --all-targets [--all-features] -- -D warnings` (both);
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; test-debug-a and test-debug-b
with their doctest steps (the `qualification.yml` commands); `conformance_fixtures -- --check`;
`graph_fixture --check`; `check-graph-determinism.sh`; `check-effect-contract.sh`;
`check-workspace-policy.sh`; `check-realtime-policy.sh`; `check-lane-policy.sh`;
`run-wasm-gates.sh --without-v8-spill --without-native`; `audit capi` (0 allocations,
0 deallocations, 0 locks, 0 syscalls, 0 violations, `pcm_digest` `cb10fbface44a3a4`, unchanged);
the worklet chain into fresh directories (`build-web-audioworklet.sh --named-twin`,
`strip-wasm-names.py check`, `check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`). No
rendered bit, fixture, digest or resource count moved; nothing was re-pinned.
