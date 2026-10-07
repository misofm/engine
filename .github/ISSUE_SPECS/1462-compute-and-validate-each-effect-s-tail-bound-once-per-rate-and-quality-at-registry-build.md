# Compute and validate each effect's tail bound once per rate and quality at registry build

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (root rulings 2 and 3 on #1377's verdict). Code anchors verified on
`codex/d15-stream-g` at `03db4574f`. Ordered after #1377 and before #1372.

## Product outcome

Each native effect's three tail values (`tail`, `tail_every_peak`, `rest`, #1377's
`EffectTailBound`) are computed once per (effect type, launch rate, quality) when the
`NativeEffectRegistry` is built, checked for consistency there, and read from that table by every
preparation. A session with many instances and bank members of one effect pays the computation
once, not once per instance and per bank member; and an effect whose descriptor states an
inconsistent set of values is refused at registry build instead of shipping.

## Context

- **#1377 (passed).** `EffectDescriptor::tail_and_rest: fn(sample_rate, quality) -> EffectTailBound`
  (`crates/effect-contract/src/lib.rs:619`). `expected_prepared_metadata` (`:2669`) calls it for
  every prepared instance (`:2677`); bank binding recomputes each candidate member's metadata
  (each `bind_homogeneous_bank`), and `EqResponseConfiguration::prepare` calls it again. Today
  every body returns constants.
- **Why now.** #1372-#1376 put certified derivations behind `tail_and_rest` (the parametric EQ's
  bound, the multiband, the delay, the gate/transient shaper/soft clip). #1329's builtin derivation
  cost 0.2-8.8 ms per design and needed #1457's cache. Root ruled the cache lands first, so the
  derivations land on the cached path and no per-effect slice builds its own cache.
- **#1330 (stream J, landed).** `NativeEffectRegistry::new` (`:2389-2417`) is the one
  release-build validation point: each descriptor is `&'static` and validated once there
  (`validate_descriptor`, `:795`); `validate_prepare_request` relies on that (`:2560-2580`).
- **D5 of #1377.** The values are a function of (type, rate, quality) only.
- **Launch rates.** 44,100, 48,000, 88,200 and 96,000 Hz (owner ruling R5); qualities are the
  descriptor's quality rows.

## Decisions frozen for this slice

- **D0. Root rulings (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled: compute `tail_and_rest` once
  per (type, rate, quality) at registry build, beside #1330's once-per-type validation, in an issue
  ordered before #1372; the same issue validates the three values per launch rate at registry
  build, refusing an inconsistent descriptor, with one red mutant per rule.
- **D1. The table.** `NativeEffectRegistry::new` evaluates each admitted descriptor's
  `tail_and_rest` for every launch rate and every quality the descriptor declares, once, and keeps
  the results in the registry beside the factory (control-side memory only). Preparation
  (`expected_prepared_metadata` and every other caller: bank binding, the EQ response
  configuration, the conformance harness) reads the table entry for the request's (rate, quality)
  instead of calling `tail_and_rest`. A rate outside the launch set is already refused before this
  point; if a caller can reach the lookup with one, it is a typed error, never a fallback call.
- **D2. The consistency rules.** At registry build, for every launch rate and quality:
  (a) `tail_every_peak >= tail` (with `Infinite` the largest);
  (b) `rest == RestBound::Unstated` only with `tail_every_peak == TailSamples::Infinite`;
  (c) `rest == RestBound::Bounded(_)` only with a finite `tail_every_peak`;
  (d) in a `RestBound::Bounded(rest)`, `rest.peak_plus_24_dbfs <= rest.any_sanitized_input` (the
  bound for every sanitized input is never smaller than the bound for a peak limited to +24 dBFS).
  (b) and (c) are the two independent directions of "`Unstated` if and only if `Infinite`" (root
  ruling R1 after attempt 1's verdict: as first written, (b) and (c) were each that whole
  equivalence, one proposition, so no fixture could break one without the other; (d) is the third
  independent rule R1 adds). A violation refuses the registry with the typed `RegistryError` code
  `effect.tail_bound.inconsistent` naming the effect.
- **D3. Callers keep one source.** No caller calls `tail_and_rest` after registry build; the
  function remains the descriptor's statement and is evaluated only by the registry (and by the
  registry's own tests).
- **D4. Class A.** No prepared value and no rendered bit changes.

## Deliverables

1. The registry table and lookup (D1), with the API change carried to every caller.
2. The D2 rules in `NativeEffectRegistry::new` and their error code.
3. Tests: gate 1's counted test and gate 2's per-rule tests.
4. The doc updates in `crates/effect-contract` and `docs/EFFECT_CONTRACT_V1.md`.

## Authorized paths

- `crates/effect-contract/src/lib.rs` (registry, `expected_prepared_metadata`, docs, tests)
- `crates/effect-compiler/src/` (callers of `expected_prepared_metadata` and bank binding)
- `crates/parametric-eq/src/lib.rs` (`EqResponseConfiguration::prepare`'s call only)
- `crates/conformance/src/effect.rs` (the harness's call only)
- other callers that the API change forces, field/call only (re-grep `tail_and_rest` and
  `expected_prepared_metadata` at start; list each in the attempt record)
- `docs/EFFECT_CONTRACT_V1.md`
- this spec; `docs/handoffs/decision-15-2026-10-05/STREAMS.md` (this slice's row)

## Non-goals

- Any effect's tail values (#1372-#1376, #1378).
- Builtins' bounds (#1329, #1457).
- Caching across processes or persisting the table.

## Hazards

- `crates/effect-contract/src/lib.rs` and `crates/effect-compiler/src/prepare.rs` are hot files
  (STREAMS rows): #1377 landed first; #1461 also edits them; order this slice relative to #1461 by
  the STREAMS row (both before #1372).
- A test-only descriptor built outside the registry must still get its values through the same
  table path (build a registry in the test), not by calling `tail_and_rest` directly.

## Objective gates

1. **Once per (type, rate, quality).** A counted test (a test-support counter in the descriptor's
   `tail_and_rest` of a test effect, or a registry-level counter): build a registry, prepare a
   session with many instances and bank members of one effect at one rate and quality, and bind the
   banks: the count equals the number of (rate, quality) pairs evaluated at registry build and does
   not grow with preparation. Red if preparation calls `tail_and_rest` again (mutant: restore the
   call in `expected_prepared_metadata`).
2. **One red mutant per rule.** For each of D2 (a), (b), (c) and (d), a test registry with a
   descriptor that violates only that rule at one launch rate is refused with the D2 code; a
   consistent descriptor is admitted. Each rule's check removed: its test is red.
3. **No value moves.** Every effect's prepared metadata is unchanged (the existing #1377
   `tail_bound_tests`, conformance fixtures, `graph_fixture --check`); `audit capi`'s
   `pcm_digest` unchanged.
4. **Workspace gates.** fmt; workspace clippy with and without `--all-features`;
   `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`; the `test-debug-a` and
   `test-debug-b` commands from `.github/workflows/qualification.yml`;
   `bash scripts/check-effect-contract.sh`; `bash scripts/check-workspace-policy.sh`;
   `bash scripts/check-realtime-policy.sh`; `bash scripts/run-wasm-gates.sh`;
   `bash scripts/check-cross-targets.sh`; `bash scripts/check-capi-abi.sh` and `audit capi`
   (0 allocations, 0 syscalls); the worklet chain.

## Test value

Gate 1's test turns red if any preparation path calls `tail_and_rest` again, which no existing test
counts. Each gate-2 test (one per D2 rule, (a) through (d)) turns red if its consistency rule is
dropped, which nothing checks today.

## Dependencies

#1377.

## Attempt record

### Attempt 1 (implementer, 2026-10-06; on `codex/d15-stream-g2` after #1461 `f3956e63c` and #1454 `a249b3cc1`)

**Design.**

- **D1 table.** `NativeEffectRegistry` now keeps, per admitted effect, its factory and a table of
  `RegisteredTailBound` entries, one per declared quality row (every launch rate of every declared
  quality, as `validate_descriptor` requires). `RegisteredTailBound` names its effect, rate and
  quality, has private fields and only the registry builds one, so no request can carry a bound
  the registry did not compute and check. `NativeEffectRegistry::tail_bound(id, rate, quality)`
  returns the entry, or `effect.native.unavailable` / `effect.quality.unsupported` (a rate outside
  the launch set has no row): never a fallback call.
- **How the entry reaches every preparation.** `PrepareEffectRequest` gains `tail_bound:
  RegisteredTailBound`. `expected_prepared_metadata` runs `validate_prepare_request` first (every
  existing error and its precedence unchanged), then refuses an entry of another effect, rate or
  quality with `effect.tail_bound.mismatch`, then copies the entry's three values. It never calls
  `tail_and_rest`. The effect factories' own `expected_prepared_metadata` calls in `prepare` and
  `bind_homogeneous_bank` needed no edit: the request carries the entry. The effect compiler reads
  the entry once per instance (`registry.tail_bound`, replacing its declared-row check with the same
  `effect.quality.unsupported` code) into `EffectBankPreparation::tail_bound`, which
  `request()` replays into every bank member's request, so graph-compiler bank binding needs no
  edit either.
- **D2 rules.** `tail_bound_consistent` checks each row with three independent booleans:
  (a) `tail_every_peak >= tail`, `Infinite` largest; (b) `Unstated` only with `tail_every_peak:
  Infinite`; (c) `Bounded` only with a finite `tail_every_peak`. **Spec note for root:** as
  written, D2 (b) "`Unstated` iff `Infinite`" and D2 (c) "`Bounded` iff finite" are one statement
  (`RestBound` and `TailSamples` each have two variants), so no descriptor can violate one without
  the other and gate 2's "violates only that rule ... each rule's check removed: its test is red"
  cannot hold literally. The slice checks the equivalence as its two independent directions, which
  together admit and refuse exactly the set the spec's (b) and (c) admit and refuse, and gives each
  direction its own fixture and red mutant. A row that breaks a rule refuses the registry with
  `effect.tail_bound.inconsistent` and the effect's id.
- **D3.** After this slice `tail_and_rest` is called only in `NativeEffectRegistry::new` and in the
  registry's own test (`tests/tail_bound.rs`, comparing the table with the statement). The four
  descriptor-statement assertions that called it directly
  (`crates/{gate-expander,transient-shaper,soft-clip}/tests/contract.rs`, the true-peak limiter's
  descriptor test in `src/lib.rs`) read the registry entry instead.
- **Harness API change (forced).** A harness that receives one factory now admits it through a
  registry of its own: `run_effect_conformance`, `d7_report_violations`, `assert_d7_reports`,
  `EffectDifferential::{edge_ramp_restore_violations, assert_edge_ramps_restore}` take
  `Box<dyn NativeEffectFactory>`; `EffectDifferential` carries `registry` and `effect` in place of
  `factory`; `conformance::admit` builds the registry; `conformance::tail_bound_of` reads one entry
  (panics on an undeclared row); `conformance::tail_bound_for_request` gives a test request built
  for an undeclared row (to be refused) its first row's entry, which `expected_prepared_metadata`
  never reads, because validation refuses the request first. `run_effect_conformance` reports a
  registry refusal as `descriptor.validation` (`effect.descriptor.invalid`) or the registry's code.
- **Behaviour note (host-core response preview).** `prepare_response_preview` reads the entry
  before it builds the request, so a preview request at an undeclared rate or quality is refused
  with `effect.quality.unsupported` (as before) even when it is also wrong in a field
  `validate_prepare_request` checks earlier (link mode, a zero limit). Only a doubly invalid preview
  request's reported code can move; no prepared value moves.
- **D4.** Class A: no prepared value and no rendered bit moved (gate 3 below).

**Forced callers edited (field/call only)** besides the authorized files: the effect crates'
test request builders and rate reassignments (`crates/{compressor,delay,gate-expander,
multiband-compressor,parametric-eq,soft-clip,transient-shaper,true-peak-limiter}` `src/` test
modules, `tests/support` or `tests/common`, `tests/ramp_endpoint.rs`, `tests/randomized.rs`,
`tests/contract.rs`, `tests/padding.rs`, `tests/product.rs`, the limiter's `tests/{allocation,
gain_law,mono_collapse,observation,padding,seedless}.rs`, `compressor/tests/mono_collapse.rs`,
`compressor/examples/lane_sample_timing.rs`), `crates/delay/src/corpus.rs` (the frozen corpus
request reads the entry from a registry), `crates/conformance/src/{effect,randomized,lib}.rs` and
`tests/effect_contract.rs`, `crates/effect-compiler/tests/{response_analysis,
symmetry_designed_words}.rs`, `crates/graph/src/lib.rs` (one test request),
`crates/graph-compiler/tests/bypass_shunt_identity.rs`, `crates/host-core/src/response.rs` and
`tests/effect_observation.rs`, `tools/audit/src/{compressor,delay,gate_expander,parametric_eq}.rs`,
`tools/bench/src/{console,effect_contract}.rs`, `tools/console-workload/tests/paired_spans.rs`.
`crates/parametric-eq/src/response.rs` (`EqResponseConfiguration::prepare`) needed no edit: its
request carries the entry. STREAMS: the `crates/graph/src/{lib,runtime}.rs` and
`crates/parametric-eq/src/lib.rs` hot rows now name G #1462 (test request fields only).

**Tests and test value** (each mutation run: applied, red; reverted, green):

- `crates/effect-compiler/src/prepare.rs` `tail_bound_count_tests::
  preparation_and_bank_binding_never_evaluate_the_tail_bound` (gate 1): a counting effect admitted
  into a registry (count 4: four declared rows), sixteen instances prepared through
  `prepare_native_session_effects` and bound as four four-lane banks from their
  `bank_preparation` requests, as the graph compiler binds; the count stays 4 and every entry
  carries the stated values. *Red if any preparation or binding path calls `tail_and_rest` again,
  which no other test counts.* Mutant M1 (`expected_prepared_metadata` calls
  `(descriptor.tail_and_rest)(...)` again): this test red, and the only red test in
  effect-contract, effect-compiler and conformance.
- `crates/effect-contract/tests/registry.rs` (gate 2), each fixture breaking one rule at 96 kHz
  only, its consistent twin admitted: `a_tail_over_every_peak_below_the_tail_is_refused` (finite
  and infinite-tail cases), `an_unstated_rest_with_a_finite_tail_over_every_peak_is_refused`,
  `a_bounded_rest_with_an_infinite_tail_over_every_peak_is_refused`. *Each is red if its rule is
  dropped, or if the registry checks only some rows, which nothing checked before.* M2 (rule (a)
  dropped): (a) test red only. M3 (rule (b) dropped): (b) test red only. M4 (rule (c) dropped):
  (c) test red only. M9 (rules checked at 44.1 kHz only): all three red.
- `crates/effect-contract/tests/tail_bound.rs` `a_request_with_another_rows_entry_is_refused`:
  *red if `expected_prepared_metadata` prepares with another effect's, rate's or quality's entry,
  or `tail_bound` falls back to a row for an undeclared rate.* M6 (effect comparison removed), M7
  (rate), M8 (quality), M10 (`tail_bound` falls back to the first row): this test red only.
- `prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` (#1377's test, moved from
  `src/lib.rs` `tail_bound_tests` to `tests/tail_bound.rs` so the crate's library carries no
  `NativeEffectFactory` impl, which `check-effect-contract.sh` would read as a product; it now
  prepares through the registry's entries): M11 (the registry evaluates every row at 48 kHz) and
  M12 (`tail_every_peak` copied from `tail`): red.
- Deleted: the earlier draft test `a_consistent_tail_bound_is_admitted` (M5, rule (a) reversed,
  was also caught by the moved #1377 test, so it had no unique catch); its claim is the fixture
  check inside the gate-2 helper.

**Gates (x86_64, local).**

- fmt: pass. Clippy `--workspace --all-targets` with and without `--all-features`, `-D warnings`:
  pass. `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: pass.
- test-debug-a (workspace minus DSP crates, with its features) and its doctests: pass.
  test-debug-b (DSP crates, with its features) and its doctests: pass (first run red in
  `compressor/tests/{contract,padding}.rs`, `multiband-compressor/tests/product.rs` and
  `parametric-eq/tests/response.rs`, which reassign a request's rate; fixed by giving each
  reassigned request its entry, and the EQ support builder `tail_bound_for_request`).
- `cargo test --release -p audit -p bench -p console-workload`; release `ramp_endpoint` gates;
  `cargo test --release -p lane -p math -p wasm-gates --features math/lane` (G5 native digests):
  pass. Conformance research-fixture `--check`: pass.
- Gate 3: `audit capi` 0 allocations, 0 syscalls, 0 violations, `pcm_digest` `cb10fbface44a3a4`
  (the value #1460 and #1461 recorded on main). `scripts/check-graph-determinism.sh`
  (`graph_fixture`, 100 fresh processes): PASS. #1377's `tail_bound` test and the conformance
  fixtures: pass. `audit delay|compressor|parametric-eq --blocks 100000` and `audit gate-expander`:
  0 violations; `trace-effect-contract-audit.sh` (1,000,000 blocks): ok.
- `check-effect-contract.sh target/release/bench`: ok (8 production factories);
  `check-workspace-policy.sh`: ok; `check-realtime-policy.sh`: ok; `check-cross-targets.sh`: PASS;
  `check-capi-abi.sh` and `--self-test`: ok; `run-wasm-gates.sh --without-v8-spill
  --without-native`: ok.
- Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `test-web-audioworklet.sh` (private TMPDIR left empty): pass.
- AArch64: CI only, not run here.

**Open items for root.** (1) The D2 (b)/(c) restatement above. (2) `launch_native_effect_registry`
is still built per call by its callers (the host-core preview, test helpers), so once #1372-#1376
put derivations behind `tail_and_rest`, each such registry build pays them; the spec's once-per-
registry-build contract holds, and caching a registry is out of this slice's scope.

### Batch follow-ups (stream G2 part 1, after the attempt-1 PASS)

From `/home/bl/misofm/submix-verdicts/1462-attempt1.md` and root's ruling R1.

- **m1, the lookup's quality key.** `tests/tail_bound.rs`
  `a_request_with_another_rows_entry_is_refused` gains one assertion: the High row at 96 kHz is
  looked up, its entry's `quality()` is `High`, and a High request carrying it prepares. The
  test's descriptor already declares High rows. *Red if `NativeEffectRegistry::tail_bound`
  ignores the quality key (the High lookup returns the Normal row's entry), which no test caught
  (verdict mutant X3).* Mutation: `.find(|row| row.sample_rate == sample_rate)` -> this test red
  (`tail_bound.rs:194`, the `quality()` assertion); revert -> green.
- **R1, D2 restated.** D2 and gate 2 now state (b) and (c) as the two independent directions, as
  implemented: (b) `Unstated` only with an infinite `tail_every_peak`; (c) `Bounded` only with a
  finite one.
- **R1, rule (d).** `tail_bound_consistent` adds a fourth independent check: in a `Bounded` rest,
  `peak_plus_24_dbfs <= any_sanitized_input`, refused with `effect.tail_bound.inconsistent`. D2,
  gate 2, the Test value and `docs/EFFECT_CONTRACT_V1.md` (*Tail and exact rest*) state it. Every
  launch effect's statement satisfies it (the launch registry builds in every test below). New
  `tests/registry.rs` `a_rest_for_a_limited_peak_above_the_rest_for_any_input_is_refused`, its
  fixture breaking only (d) at 96 kHz (41 against 40) beside the consistent twin. *Red if the
  registry drops rule (d), or checks only some rows, which nothing checked before.* Mutation:
  remove `rest_ordered` from the conjunction -> this test red alone in `effect-contract`'s
  registry tests; revert -> green.
- **n1.** The D3 record line now names `tests/tail_bound.rs` (the test that compares the table
  with the statement), not `tests/registry.rs`.
- **n2.** The response-preview behaviour note also covers an undeclared rate or quality combined
  with a too-small `maximum_prepared_bytes`: the EQ's `prepare_response` checks `ResourceLimit`
  first, so before this slice that request reported the limit; now it reports
  `effect.quality.unsupported`. Still only a doubly invalid preview's code moves; the one host
  consumer maps every owner error to `RESULT_INVALID_ARGUMENT`.
- **n4.** `docs/EFFECT_CONTRACT_V1.md` *Tail and exact rest* is rewrapped to 100 columns. By the
  doc's existing rule for registry-build codes (`effect.descriptor.invalid`, raised only by
  `NativeEffectRegistry::new`, is on the frozen list), `effect.tail_bound.inconsistent` joins the
  "Session preparation additionally freezes" list, and the doc says so. On the host path it
  surfaces as `host.effect.registry`, as `effect.descriptor.invalid` does.
  `scripts/check-effect-runtime-policy.sh`'s required-code list is outside this follow-up's paths
  and is not changed (it does not list `effect.automation.rate` either).
- **n6.** Gate 1's test mirrors the graph compiler's bank binding instead of driving it:
  `effect-compiler`'s test replays `bank_preparation.request()` into `bind_homogeneous_bank`,
  which is what `graph-compiler/src/banks.rs` does (it only forwards the replayed requests and pads
  by cloning). The dependency direction keeps graph-compiler out of effect-compiler's unit tests, so
  a new direct `tail_and_rest` call inside graph-compiler would not be counted by it.
- **Gates.** All pass on the follow-up tree (x86_64): fmt; workspace clippy `-D warnings` with and
  without `--all-features`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
  --exclude gate-expander` (gate-expander's own doc failure is part 2's);
  `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-effect-runtime-policy.sh`;
  test-debug-a 1,462 passed, 0 failed; test-debug-b 890 passed, 0 failed; `conformance_fixtures
  --check`; release `lane`/`math`/`wasm-gates` (G5 `g5_native_digests_match_pins`) 123 passed;
  `run-wasm-gates.sh` (native 144 cases, wasm simd128 144 cases, 0 mismatches; V8 spill ok);
  `check-cross-targets.sh` PASS (known-defect rows unchanged: builtins 5, host-core 4, soft-clip 1);
  the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `test-web-audioworklet.sh` with a private TMPDIR left empty, the V8 spill gate on the named twin):
  all pass. AArch64: CI only.

### Batch follow-ups, part 2 (stream G2, 2026-10-07; root-authorized)

- **Required diagnostics.** `scripts/check-effect-runtime-policy.sh`'s required-code list now
  includes `effect.tail_bound.inconsistent` (raised by `NativeEffectRegistry::new`, this slice) and
  `effect.automation.rate` (session preparation, #1335); both are in `crates/effect-contract`,
  `crates/effect-compiler` and `docs/EFFECT_CONTRACT_V1.md`, so the gate passes.
- **Self-test case** (`scripts/test-effect-runtime-policy.sh`, run by `qualification.yml`): for each
  of the two codes, a temporary copy renames it everywhere the gate searches and must fail with
  "missing diagnostic <code>"; the copy is restored and must pass again.
  - Test value: dropping either code from the required list (or a rename of the code out of the
    contract and its doc) escapes the gate; this case turns red on the first and the gate on the
    second.
  - Mutation: the two codes removed from the list: red ("effect runtime missing diagnostic
    escaped: effect.tail_bound.inconsistent: effect runtime policy: ok"); restored: green
    ("effect runtime policy mutations: ok"), and `check-effect-runtime-policy.sh` ok.
