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

### Attempt 1 (2026-10-08, implementer; branch `codex/d15-stream-g3`, on `ae3fcbf16`)

**Anchors re-verified** after #1457, #1461, #1462 and #1469 landed: `EffectTailBound` at
`crates/effect-contract/src/lib.rs:209`, `InputSectionBound` at `crates/builtins/src/tail.rs:34`
(`max` at `:68`), the entry points at `crates/builtins/src/lib.rs:3486`, `:3531`, `tail.rs:435`,
`:452`, `PreparedBuiltinsSession::input_bounds` at `crates/builtins-compiler/src/lib.rs:2172`, the
tail-entry charge at `:3833`, `:3836`, the mismatch check at `crates/effect-compiler/src/prepare.rs:621-637`
and its forgery table (16 rows). Since #1461 no processor holds a `PreparedEffectMetadata`; no
render-owned structure holds an `EffectProgramKey` either (`crates/rack/src` and
`crates/graph/src/runtime.rs` name none; `RackProgram<EffectProgramKey>` is the cohort planner's,
control-side). No render-owned memory grows.

**What changed.**

- `effect_contract::EffectTailBound` is renamed `NodeTailBound` (no alias) and gains
  `composition: CompositionBound`. New types exactly as K-D2: `CompositionBound { Stated { decay:
  TailDecay, peak_gain: PeakGain, tail_gain: PeakGain, stall: FlushStall }, Unstated }`,
  `TailDecay(pub u64)`, `PeakGain { Zero, Millibels(i32) }`, `FlushStall { Zero, Level(i32) }`; the
  derived `Ord` puts `Zero` below every level. No name was refined. H1's contract (N1)-(N3) is on
  `CompositionBound`'s doc and in `docs/EFFECT_CONTRACT_V1.md` (*Tail and exact rest*).
- `NodeTailBound::ZERO`, `NodeTailBound::UNBOUNDED` (moved from `InputSectionBound`, same values,
  composition `Unstated`) and `NodeTailBound::max` (K-D5's rule; composition by maximum, `Unstated`
  if either side is). `builtins::tail`'s `from_cascade` is now the private `bound_from_cascade`.
- `PreparedEffectMetadata` and `EffectProgramKey` carry `composition`; `program_key` and
  `expected_prepared_metadata` copy it; the mismatch check compares it; the forgery table has 17
  rows (`composition`).
- `NativeEffectRegistry::new`'s consistency check gains rules (e) `Stated` needs a finite `tail`
  and (f) `tail_gain <= peak_gain`. Both refuse with the existing typed error
  `effect.tail_bound.inconsistent` (naming the effect), so the frozen error list does not change.
- `builtins::InputSectionBound` is deleted; `input_section_bound`, `input_section_bounds`,
  `input_section_live_bound`, `input_section_live_bound_table`, `ChargedInputBound::bound` and
  `PreparedBuiltinsSession::input_bounds` use `NodeTailBound` (`rest: None` -> `RestBound::Unstated`,
  `Some(r)` -> `RestBound::Bounded(r)`; the `Option` returns are kept).
- All eight effects, the conformance mock and every builtin bound state `CompositionBound::Unstated`.

**Files** (re-grep of `EffectTailBound`, `InputSectionBound`, `PreparedEffectMetadata {`,
`EffectProgramKey {`): `crates/effect-contract/src/lib.rs`, `tests/{registry,response_analysis,tail_bound}.rs`,
`tests/support/mod.rs` (a test helper the spec's list missed; rename and field only);
`crates/effect-compiler/src/prepare.rs`, `tests/native_session.rs`; the eight effect crates'
`tail_and_rest` (`compressor`, `delay`, `gate-expander`, `multiband-compressor`, `parametric-eq`,
`soft-clip`, `transient-shaper`, `true-peak-limiter`); `crates/transient-shaper/src/corpus.rs`
(field); `crates/conformance/src/effect.rs`; `crates/builtins/src/{tail,lib}.rs`,
`tests/tail_contract.rs`, `examples/input_bound_budget.rs` (rename only; not in the spec's list,
it names the deleted type); `crates/builtins-compiler/src/lib.rs`; `crates/graph/src/lib.rs`
(rename and two test descriptors' field, three test `PreparedEffectMetadata {` literals' field);
`crates/rack-compiler/src/lib.rs` and `crates/rack/tests/live_control_bank.rs` (one
`EffectProgramKey {` test literal each, the field only); `hosts/host-web/src/tests.rs`;
`tools/audit/src/{fixture_builtins,builtins_graph}.rs`; `fixtures/builtins/v1/{resources.jsonl,MANIFEST.tsv}`;
the three docs. `crates/host-core/tests/live_lanes.rs` names neither type and is unchanged.
`tail_contract.rs`'s only non-rename edit is mechanical: `Some(RestSamples ..)` became
`RestBound::Bounded(..)`, and six `.rest.expect(msg)` reads became `stated_rest(.rest, msg)`, a
local helper that panics with the same message on `Unstated` (K1: no assertion's value changed).
The STREAMS row needs no change.

**K5 re-pins (one size, its reason).** `(Box<str>, NodeTailBound)` is 104 bytes against 72 for
`(Box<str>, InputSectionBound)`: +32, the `CompositionBound` (32 bytes, the niche of `PeakGain`'s
tag holds `Unstated`). `audit`'s `BOXED_TAIL_ENTRY_BYTES` 72 -> 104 (its layout check passes).
`resources.jsonl` moves in `engine_owned_processor_payload_bytes` and
`engine_owned_retained_payload_bytes` only, by exactly 2 x 32 per track (the payload's and the
seal's tail vectors): 2,069 -> 2,133 (1 track), 8,348 -> 8,604 (4), 139,035,061 -> 143,229,429
(65,537 = 64 x 65,537); `maximum_single_allocation_bytes`, `retained_allocation_count` and the meter
columns do not move. `MANIFEST.tsv`'s `resources.jsonl` row moves with it, so the accepted manifest
identity `1a8fd9a1...` -> `cd2b9fe6...` in `builtins_graph.rs` and `fixture_builtins.rs`. No PCM,
meter, response or benchmark fixture moved (every other manifest row is byte-identical after
`audit fixture-builtins --write`). `audit capi` `pcm_digest` `cb10fbface44a3a4`, unchanged (0
allocations, 0 syscalls, 0 violations); the wasm G5 digests pass. The browser `expected.json`
did not move, but the measured `builtinRetainedBytes` row grew by +64 (two wasm32 tail entries, 64
-> 96 bytes each, one track) to 2,017 of its 2,048 ceiling: 31 bytes of headroom are left, so the
next slice that grows a builtin retained byte by more than 31 will exceed it. (Corrected by the
follow-up below; this sentence first said the browser resources did not move.)

**Gates.** All pass: `test-debug-a` (workspace debug tests, CI's command and features) and
`test-debug-b` (DSP crates); `cargo test --release -p builtins --features builtins/test-support
--test tail_contract` (13 passed); `check-effect-contract.sh`; `cargo build --release -p audit &&
audit capi`; `check-builtins-fixtures.sh` (50 files); `cargo test -p audit` (33 passed);
`check-workspace-policy.sh`; `conformance_fixtures -- --check`; `run-wasm-gates.sh`; clippy
`-D warnings`; `cargo fmt --check`; `check-realtime-policy.sh`; `check-cross-targets.sh`; the worklet
chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`check-scalar-oracle-absent.py`, `test-web-audioworklet.sh` with a private `TMPDIR`, nothing left).

**Tests.** K1: existing assertions, rename only (above). K2: `tail_bound.rs`'s fixture now states a
consistent `Stated` composition with distinct values per rate and field, and
`prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` compares
`metadata.composition` and `key.composition`; the forgery table's `composition` row. K3:
`registry.rs` `a_stated_composition_with_an_infinite_tail_is_refused` (with an admitted
finite-tail twin and an admitted all-unstated descriptor) and `a_tail_gain_above_the_peak_gain_is_refused`
(one millibel above, and any `Millibels` above `Zero`), each broken at one rate only. The eight
effects and the mock are accepted by the existing `launch_registry_prepares_the_accepted_nine_track_parametric_eq_fixture`
and the conformance suite; no test pins their `Unstated` (it is the value the later slices change,
and no plausible defect of this slice sets it). K4: `tail_bound.rs`
`max_states_a_composition_only_when_both_channels_state_one`.

**Mutation table** (each applied, the named set run, the file restored; M3-M10 run against every
`effect-contract` and `effect-compiler` test, M1-M2 against `tail_contract` in release; every
listed test is the only one red; all green on revert):

| # | defect | red |
|---|---|---|
| M1 | `bound_from_cascade` maps `rest` to `Unstated` (K1) | 7 `tail_contract` tests, e.g. `live_bound_table_is_the_computed_live_bound_at_every_launch_rate` |
| M2 | the live table literal crosses `peak_plus_24_dbfs` and `any_sanitized_input` (K1) | `live_bound_table_is_the_computed_live_bound_at_every_launch_rate` |
| M3 | `expected_prepared_metadata` writes `Unstated` for `composition` (K2) | `prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` |
| M4 | `program_key` writes `Unstated` for `composition` (K2) | the same |
| M5 | the mismatch check drops the `composition` comparison (K2) | `a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused` |
| M6 | the registry drops rule (e) (K3) | `a_stated_composition_with_an_infinite_tail_is_refused` |
| M7 | the registry drops rule (f) (K3) | `a_tail_gain_above_the_peak_gain_is_refused` |
| M8 | the registry orders `Zero` above a `Millibels` (K3) | the same |
| M9 | `max` keeps a `Stated` side beside an `Unstated` one (K4) | `max_states_a_composition_only_when_both_channels_state_one` |
| M10 | `max` takes `peak_gain` by minimum (K4) | the same |

### Attempt 1 follow-up (2026-10-08, worker; on `32f623059`): verdict PASS, m1-m2 and n1-n6

- **m1.** `tail_bound.rs::max_states_a_composition_only_when_both_channels_state_one` now gives
  its operands different finite `tail`, `tail_every_peak` and `rest` values, crossed so that each
  side supplies at least one larger value (left: 12, 40, (30, 50); right: 20, 35, (25, 60);
  componentwise: 20, 40, (30, 60)), and asserts the whole componentwise `max` in both operand
  orders. The `UNBOUNDED` absorption is asserted in both orders too.
- **m2.** `NodeTailBound` derives only `Clone, Copy, Debug, Eq, PartialEq`; the lexicographic
  `Hash, Ord, PartialOrd` are gone, so `std::cmp::max` and `Iterator::max` no longer compile on it.
  No user needed them (`cargo check --workspace --all-targets --all-features` passes).
  `CompositionBound` and its parts keep `Ord`.
- **n1.** `registry.rs` adds the admitted descriptor `LAUNCH_SHAPE_STATED` (finite `tail`,
  `tail_every_peak: Infinite`, `rest: Unstated`, a stated composition with a `Zero` tail gain under
  a `Millibels(-100)` peak gain), asserted admitted in both the (e) and the (f) test.
- **n2.** The K4 operands are all statements the registry admits: `left` `(-300, -900)`, `right`
  `(Zero, Zero)`, `crossed` `(600, -500)`.
- **n3.** `RegisteredTailBound::bound`'s doc says "The four values".
- **n4.** `audit`'s `BOXED_TAIL_ENTRY_BYTES` comment: the bound grew 56 -> 88 bytes; the entry was
  72.
- **n5.** The #1464 re-pin note in `fixture_builtins.rs` now follows #1329's.
- **n6.** Corrected in Attempt 1's K5 paragraph above.

**Mutation runs** (each applied to `crates/effect-contract/src/lib.rs`, every `effect-contract`
and `effect-compiler` test run in debug with `--all-features --no-fail-fast`, the file restored;
baseline 116 passed, 0 failed, before and after):

| # | defect | red |
|---|---|---|
| X3 | `NodeTailBound::max` takes a finite tail by minimum | `max_states_a_composition_only_when_both_channels_state_one` only (115/1) |
| X4 | `max` takes `rest.any_sanitized_input` from `self` only | the same test only (115/1) |
| X1 | rule (e) reads `tail_every_peak` in place of `tail` | `a_stated_composition_with_an_infinite_tail_is_refused`, `a_tail_gain_above_the_peak_gain_is_refused` (114/2) |
| X2 | rule (f) refuses a `Zero` tail gain under a `Millibels` peak gain | the same two tests (114/2) |

All four were green on the previous commit (the verdict's X1-X4 survivors).
