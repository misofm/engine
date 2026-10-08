# Carry every node's tail bound in one node-neutral struct

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by root order: slice A1 of *Define how node tails compose through gain in the graph
extent* (#1379), Amendment 1 (H1, H2, H8). Code anchors verified on `main` at `7e8379523` and on
`codex/d15-stream-g2` at `f3956e63c` (#1461); re-verify every anchor at start, after #1461, #1462
and #1457 have landed.

## Product outcome

Every node, a native effect or a builtin input section, states its tail in one struct,
`NodeTailBound`. Beside today's `tail`, `tail_every_peak` and `rest`, it carries the four values
that #1379 needs to compose node tails through gain: `decay`, `peak_gain`, `tail_gain` and `stall`.
In this slice every node states them as `CompositionBound::Unstated`, so no reported tail, no
certified value and no rendered bit moves. Later slices state the values (fixed input section,
live input section, each effect) and #1379 composes them.

## Context

- **Effects.** `effect_contract::EffectTailBound { tail, tail_every_peak, rest }` (#1377) is
  carried by `PreparedEffectMetadata` and `EffectProgramKey`, copied by
  `expected_prepared_metadata` and compared by `effect-compiler`'s mismatch check
  (`crates/effect-compiler/src/prepare.rs`). Since #1461 the prepare result carries the metadata
  (`PreparedEffect { processor, metadata }`); the check is
  `a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused`.
  `NativeEffectRegistry::new` computes and validates each effect's bound once per rate and quality
  (#1462).
- **Builtins.** `builtins::InputSectionBound { tail, tail_every_peak, rest: Option<RestSamples> }`
  (`crates/builtins/src/tail.rs:30`, `max` at `:68-90`) is returned by `input_section_bound`,
  `input_section_bounds` (`crates/builtins/src/lib.rs:3483`, `:3502`), `input_section_live_bound`
  (`tail.rs:211`) and `PreparedBuiltinsSession::input_bounds`
  (`crates/builtins-compiler/src/lib.rs:2167`), and charged as `(Box<str>, InputSectionBound)` per
  strip (`builtins-compiler/src/lib.rs:3811`, `:3814`; `tools/audit/src/fixture_builtins.rs:3526-3530`).
- **Users of `EffectTailBound`** (re-grep before starting): `crates/effect-contract/src/lib.rs`,
  `tests/registry.rs`, `tests/response_analysis.rs`; the eight effect crates' `tail_and_rest`
  (`parametric-eq`, `compressor`, `true-peak-limiter`, `multiband-compressor`, `delay`,
  `gate-expander`, `transient-shaper`, `soft-clip`); `crates/conformance/src/effect.rs`;
  `crates/effect-compiler/tests/native_session.rs`; `crates/graph/src/lib.rs`;
  `crates/builtins-compiler/src/lib.rs`; `hosts/host-web/src/tests.rs`.
- **`PreparedEffectMetadata {` literals** outside `effect-contract` (gain the field): for example
  `crates/transient-shaper/src/corpus.rs:190` (stream A's payload code), `crates/gate-expander/src/lib.rs`,
  `crates/multiband-compressor/src/lib.rs`, `crates/graph/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
  and test helpers. Re-grep at start.
- `InputSectionBound` is named in `docs/EFFECT_CONTRACT_V1.md:84` (after #1461),
  `docs/BUILTINS_AND_METERING_V1.md:55` and `docs/derivations/1329-input-section-tail-and-rest.md:13`.

## The contract the new values carry (#1379 Amendment 1, H1)

Notation: `eps = 10^(-144/20)`; a node with latency `L`, input `x`, output `y`; `u = 2^-24`;
`N` the first sample of silence. Each value is a certified upper bound at the node's rate, over its
parameter domain or its prepared design, computed on the control thread; a stereo node states the
maximum over its two channels. A node with a sidechain states each value for every sidechain input.

| field | symbol | unit | meaning |
|---|---|---|---|
| `decay` | `D` | samples per further 20 dB | (N2) |
| `peak_gain` | `G_p` | millibels, rounded up, or `Zero` | (N1) |
| `tail_gain` | `G_t` | millibels, rounded up, or `Zero`; `G_t <= G_p` | gain to an input that arrives at or after `N`, (N2) |
| `stall` | `sigma` | millibels re 1.0, rounded up, or `Zero` | absolute flush stall at the output |

`g_p = 10^(G_p/2000)`, `g_t = 10^(G_t/2000)`; `Zero` is a node whose output is exactly `+-0.0`
for every input.

- **(N1) Peak.** If `|x[n]| <= X` for all `n`: `|y[n]| <= g_p X + sigma` for every `n`.
- **(N2) Tail at every decade.** With no control event at or after `N`: if `|x[n]| <= X` for all
  `n` and `|x[n]| <= epsilon` for every `n >= M` (some `M >= N`), then for every integer `k >= 0`
  and every `n >= M + L + T + k D`: `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma`.
- **(N3) Rest.** #1329 D2's `RestSamples`, read with "zero from `M`" (`M >= N`).

## Decisions frozen for this slice

- **K-D1. One rename.** `effect_contract::EffectTailBound` becomes `NodeTailBound`, once: no alias,
  no second struct (root ruling (c) of #1379: no parallel struct).
- **K-D2. The new field and types.**

  ```rust
  pub struct NodeTailBound {
      pub tail: TailSamples,
      pub tail_every_peak: TailSamples,
      pub rest: RestBound,
      /// The four composition values; `Unstated` until the node's slice derives them.
      pub composition: CompositionBound,
  }
  pub enum CompositionBound {
      Stated { decay: TailDecay, peak_gain: PeakGain, tail_gain: PeakGain, stall: FlushStall },
      Unstated,
  }
  pub struct TailDecay(pub u64);                // D: samples per further 20 dB
  pub enum PeakGain { Zero, Millibels(i32) }    // rounded up; negative is an attenuation
  pub enum FlushStall { Zero, Level(i32) }      // sigma: millibels re 1.0, rounded up
  ```

  The four values are stated together or not at all. The implementer may refine the names (no
  version suffix) and states the final names and the contract above in `docs/EFFECT_CONTRACT_V1.md`.
- **K-D3. The metadata path.** `PreparedEffectMetadata` and `EffectProgramKey` carry `composition`
  as they carry `rest`; `expected_prepared_metadata` copies it; `effect-compiler`'s mismatch check
  compares it.
- **K-D4. Two registry rules.** `NativeEffectRegistry::new` refuses, with a typed error,
  (a) `Stated` with `tail` `Infinite`, and (b) `tail_gain > peak_gain` (`Zero` is below every
  `Millibels`).
- **K-D5. `InputSectionBound` is deleted.** `input_section_bound`, `input_section_bounds`,
  `input_section_live_bound` and `PreparedBuiltinsSession::input_bounds` return `NodeTailBound`.
  `rest: None` becomes `rest: RestBound::Unstated`; a function that returns `Option` today (`None`
  at a rate off the launch set) keeps that return. `InputSectionBound::max` becomes a function on
  `NodeTailBound` with the same componentwise rule; for `composition`, each value by maximum
  (`Zero` below every `Millibels`), and `Unstated` if either side is.
- **K-D6. Every node states `Unstated`.** All eight effects, the conformance mock and every builtin
  bound state `CompositionBound::Unstated`.

## Deliverables

1. The rename, the field, the types and their docs; the metadata, the program key and the mismatch
   check; the two registry rules.
2. `InputSectionBound` replaced in `builtins`, `builtins-compiler` and every user; the docs that name
   it.
3. Gates K1-K5; each moved resource count re-pinned with its reason.

## Authorized paths (named exceptions are marked)

- `crates/effect-contract/src/lib.rs`, `crates/effect-contract/tests/registry.rs`,
  `crates/effect-contract/tests/response_analysis.rs`
- `crates/effect-compiler/src/prepare.rs` (the mismatch comparison and its unit test; hot-file
  slot), `crates/effect-compiler/tests/native_session.rs` (the rename only)
- The `tail_and_rest` functions of the eight effect crates (the rename and the field only; named
  exception) and every `PreparedEffectMetadata {` literal in the effect crates' payload code and
  test helpers, for example `crates/transient-shaper/src/corpus.rs:190` (the field only; named
  exception, stream A's payload code); `crates/conformance/src/effect.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/src/lib.rs` (the bound entry points and
  re-exports), `crates/builtins/tests/tail_contract.rs` (named exceptions, stream A's)
- `crates/builtins-compiler/src/lib.rs` (the bound type, the seal, `input_bounds`, the tail-entry
  charge, their unit tests; named exception, hot-file slot),
  `crates/builtins-compiler/tests/metered_preparation.rs` (only if a size moves)
- `crates/graph/src/lib.rs` (the rename only; named exception, hot-file slot)
- `hosts/host-web/src/tests.rs`, `crates/host-core/tests/live_lanes.rs` (the rename only)
- `tools/audit/src/fixture_builtins.rs`, `fixtures/builtins/v1/resources.jsonl`,
  `fixtures/builtins/v1/MANIFEST.tsv`, `tools/audit/src/builtins_graph.rs` (the manifest identity),
  `fixtures/graph/MANIFEST.tsv`, and the browser's expected-resource fixtures, each only where a
  recorded byte count moves
- Any other struct literal that must gain the field or the new name, those lines only; the
  implementer re-greps `EffectTailBound`, `InputSectionBound`, `PreparedEffectMetadata {` and
  `EffectProgramKey {` before starting and lists every file in the attempt record
- `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md` (`:55`),
  `docs/derivations/1329-input-section-tail-and-rest.md` (the name column of `:13`)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Non-goals

- Any composition value other than `Unstated` (later slices: the fixed input section, the live
  input section, #1372-#1376 for the effects).
- Any graph or report change (#1379). Any change to `tail`, `tail_every_peak` or `rest`.

## Hazards

- **Hot files.** `crates/effect-contract/src/lib.rs` and `crates/effect-compiler/src/prepare.rs`
  follow #1462; `crates/builtins-compiler/src/lib.rs`, `crates/parametric-eq/src/lib.rs`,
  `crates/graph/src/lib.rs` and the effect crates' payload code are stream A's: the slots in
  `STREAMS.md` govern, and the later slice rebases.
- **Render memory.** `composition` grows `PreparedEffectMetadata`; since #1461 no processor holds
  that record, so no render-owned memory grows. If a render-owned structure still holds it, stop
  and report.
- **Resource counts.** The builtins' tail entry grows by `size_of` of the field; `resources.jsonl`
  and the counts that name the tail entry move by exactly that delta and nothing else.

## Objective gates

- **K1. Nothing certified moves.** Every existing assertion in `crates/builtins/tests/tail_contract.rs`,
  `crates/effect-contract/tests/*`, `builtins-compiler`'s tail unit tests and
  `crates/host-core/tests/live_lanes.rs` passes with only the rename applied to it.
- **K2. The metadata path carries `composition`.** `tail_bound_tests` (effect-contract) shows that
  `expected_prepared_metadata` and `program_key` copy a `Stated` value per rate. #1461's forgery
  table, `a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused`, gains a row: a
  prepare result whose returned metadata's `composition` differs gets `effect.metadata.mismatch`.
- **K3. The registry refuses inconsistent statements.** A test descriptor stating `Stated` with
  `tail` `Infinite`, and one with `tail_gain` above `peak_gain`, are each refused at
  `NativeEffectRegistry::new` with the typed error; each of the eight effects and the conformance
  mock states `Unstated` and is accepted.
- **K4. `max` keeps the componentwise rule.** `NodeTailBound::max` of a `Stated` and an `Unstated`
  bound is `Unstated`; of two `Stated` bounds, each value is the larger one (`Zero` below every
  `Millibels`).
- **K5. Re-pins and bits.** `resources.jsonl` and the counts that name the tail entry move only by
  the tail entry's size delta, each audited with its reason; no rendered bit moves (`audit capi`'s
  `pcm_digest`, the wasm G5 digests, the builtins PCM fixtures).
- **Commands:** the `test-debug-a` workspace command from `.github/workflows/qualification.yml`;
  `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`;
  `bash scripts/check-effect-contract.sh`; `cargo build --locked --release -p audit &&
  ./target/release/audit capi`; `bash scripts/check-builtins-fixtures.sh . target/release/audit`;
  `bash scripts/check-workspace-policy.sh`; `cargo run --locked -p conformance --example
  conformance_fixtures -- --check`; `bash scripts/run-wasm-gates.sh`;
  `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

The attempt record carries a mutation table (each defect applied, the named test run, the file
restored) with at least the mutants in "Test value"; each is red.

## Test value

- K1: a rename that changes a value (a field crossed in a literal, `rest` mapped wrongly from
  `None`) turns an existing value assertion red.
- K2: a metadata path that drops or crosses `composition`, or a mismatch check that does not
  compare it, is red in #1461's forgery table; no other test compares the new field.
- K3: a registry that accepts a `Stated` composition with an `Infinite` tail, or a tail gain above
  the peak gain, would feed #1379 a contradiction; nothing else validates the statement.
- K4: a `max` that keeps one side's `Stated` values when the other is `Unstated` states a
  composition no one derived.
- K5: reuses the existing pins; a size delta other than the tail entry's is a layout defect.

## Dependencies

- *Keep each effect processor's render memory free of its prepared metadata* (#1461): no
  control-only bytes in render-owned memory (#1329 R5).
- *Compute and validate each effect's tail bound once per rate and quality at registry build*
  (#1462): the registry that K-D4 extends.
- *Cache design bounds across preparations within a stated preparation budget* (#1457): its cache
  value type becomes `NodeTailBound`; its key does not change.
- Hot-file slots and named exceptions: `STREAMS.md` (#1379 Amendment 1, H8).

## Attempt record
