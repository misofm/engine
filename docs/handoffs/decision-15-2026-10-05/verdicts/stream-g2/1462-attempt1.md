PASS

# #1462 attempt 1: adversarial verdict

Commits reviewed: `7a8a30f1c` (parent `a249b3cc1`) and `8bb56d9b5` (parent `ffc7b8805`), as
`git diff a249b3cc1 8bb56d9b5` minus `ffc7b8805`'s nine #1379 spec/design files (73 files). Built
and tested from an export of `8bb56d9b5` (`/tmp/claude-1002/v1462/tree`,
`CARGO_TARGET_DIR=/tmp/claude-1002/v1462/target`). The worktree was not touched.

Verdict: the slice does what D0-D4 ask. `NativeEffectRegistry::new` is the only caller of
`(d.tail_and_rest)` in the workspace (`crates/effect-contract/src/lib.rs:2545`). Every preparation
reads an entry that only the registry can build (`RegisteredTailBound`, private fields,
`lib.rs:2443`). `expected_prepared_metadata` refuses an entry of another effect, rate or quality
(`lib.rs:2870-2879`). Every gate is green, `audit capi` gives `pcm_digest` `cb10fbface44a3a4` with 0
allocations and 0 syscalls, and every mutation the record claims goes red as stated. There is no
BLOCKER and no MAJOR.

## BLOCKER

None.

## MAJOR

None.

## MINOR

- **m1. The lookup's quality key is untested.**
  `crates/effect-contract/tests/tail_bound.rs:159` (`a_request_with_another_rows_entry_is_refused`)
  never looks up a non-`Normal` row. Mutant X3 changes `NativeEffectRegistry::tail_bound`
  (`lib.rs:2591`) to match on `sample_rate` only (`.find(|row| row.sample_rate == sample_rate)`),
  so the High request gets the Normal entry. No test goes red in effect-contract, effect-compiler
  or conformance. No launch effect declares a second quality, so no other crate can catch it. The
  defect is loud, not silent: `effect.tail_bound.mismatch` would refuse every non-Normal
  preparation. The test's descriptor already declares High rows. One assertion closes the gap:
  `registry.tail_bound(DESCRIPTOR.id, 96_000, High)` returns an entry whose `quality()` is `High`
  and whose metadata prepares. Fold this into the follow-ups.

## NIT

- **n1. A false citation in the attempt record.** Spec record line 156 says the registry's own test
  that compares the table with the statement is `tests/registry.rs`. It is `tests/tail_bound.rs`
  (`:140`). `tests/registry.rs` calls no statement directly.
- **n2. The preview's error-code change is wider than the record says.**
  `crates/host-core/src/response.rs:547` looks up the entry before `prepare_response` runs. A
  doubly invalid preview request now reports `effect.quality.unsupported`. That includes an
  undeclared rate combined with an unsupported link mode or a zero limit (which the record names),
  and also an undeclared rate combined with a too-small `maximum_prepared_bytes`: the EQ's
  `prepare_response` checks `ResourceLimit` first (`crates/parametric-eq/src/response.rs:621`), and
  the record does not name that case. This is **not a contract regression**:
  - the only host consumer, `hosts/host-web/src/ffi.rs:797-808`, maps every
    `ResponsePreviewError::Owner(_)` to `RESULT_INVALID_ARGUMENT`;
  - the C ABI has no preview;
  - no test or doc pins the precedence;
  - the effect compiler keeps its precedence exactly, because the lookup replaced the declared-row
    check in the same place (`crates/effect-compiler/src/prepare.rs:459-472`).

  Add the `ResourceLimit` case to the record.
- **n3. A redundant registry build in a bench.** `tools/bench/src/console.rs:1033`: `eq_request`
  builds a whole launch registry for each lane, but `HoistArm::new` (`:876`) already holds
  `registry`. This is setup only and not timed. Pass the entry in.
- **n4. Doc formatting and the frozen-code list.**
  - `docs/EFFECT_CONTRACT_V1.md:118` is a 130-column line inside a paragraph that is otherwise
    wrapped at 100.
  - The "Session preparation additionally freezes" list (`:283-301`) leaves out
    `effect.tail_bound.inconsistent`, but the prose at `:308` says the code is "raised the same way"
    as `effect.descriptor.invalid`, which the list includes. Say whether the code joins the frozen
    set, or why it does not. On the host path it surfaces as `host.effect.registry`,
    `crates/host-core/src/prepare.rs:1348`.
- **n5. STREAMS ordering for root to confirm.** `docs/handoffs/decision-15-2026-10-05/STREAMS.md:90`
  puts G #1462 ahead of A's payload slices (#1279, #1280) on `crates/parametric-eq/src/lib.rs`.
  Those slices are not on `main`. The edit is three test-request fields, and it is placed the same
  way #1461 placed itself on `crates/graph/src/{lib,runtime}.rs`. Root owns the hot-file order, so
  root should confirm it.
- **n6. Gate 1 mirrors the graph compiler's bank binding instead of driving it.**
  `crates/effect-compiler/src/prepare.rs:2241` replays `bank_preparation.request()` into
  `bind_homogeneous_bank`. That is exactly what `crates/graph-compiler/src/banks.rs:621-641` does:
  graph-compiler only forwards the replayed requests and pads by cloning. So the claim "red if any
  preparation or binding path calls `tail_and_rest` again" holds for every path that exists today.
  A new direct call inside graph-compiler would not be counted. The dependency direction rules out
  graph-compiler in effect-compiler's unit tests, so this is acceptable as it stands.

## Spec problems for root (not defects of this attempt)

- **R1. D2 (b) and (c) are one rule (flag 1).** `RestBound` and `TailSamples` each have exactly two
  variants, so "Unstated iff Infinite" and "Bounded iff finite" are the same proposition. Under the
  literal text, a fixture cannot break one without the other, and removing one check can never
  turn its test red, because the other check still refuses. Gate 2 therefore cannot be met as
  written.

  The implementer splits the rule into its two directions:
  - (b') Unstated implies Infinite;
  - (c') Bounded implies finite.

  (b') and (c') together are exactly the spec's (b), which is the same as its (c). The split admits
  and refuses exactly the set the spec's D2 admits and refuses, and each direction has a fixture
  and a unique red mutant (M3, M4). That is the only reading under which gate 2 can be satisfied,
  and I accept it.

  Root should restate D2 in an amendment. If root meant a different, independent third rule,
  nothing checks it now. One candidate is `RestSamples::peak_plus_24_dbfs <= any_sanitized_input`:
  a bound over a superset of inputs cannot be shorter. This slice must not invent that rule.
- **R2. The registry is rebuilt per call, and #1457 does not cover it (flag 3).**
  `launch_native_effect_registry()` (`crates/effect-compiler/src/prepare.rs:208`) builds a fresh
  registry on each call:
  - every session preparation (`crates/host-core/src/prepare.rs:1348`);
  - every live-delta rebuild call (`crates/host-core/src/live_delta.rs:484`);
  - every response preview (`crates/host-core/src/response.rs:534`).

  Each build now evaluates all 32 launch rows (8 effects x 4 launch rates x `Normal`; every launch
  `QUALITIES` has 4 rows), whichever effects the session uses. Today every body is a constant, so
  this costs nothing measurable. Once #1372-#1376 put certified derivations behind `tail_and_rest`,
  every preparation, rebuild and interactive EQ preview pays all 32 derivations. Before this slice
  a preview paid one.

  #1457 does not cover this. Its cache is `builtins::input_section_bounds`, keyed by the builtin
  input section's design key, and it never mentions `NativeEffectRegistry`. #1462's spec meets its
  stated outcome ("once, not once per instance and per bank member"). Its non-goal excludes only
  caching across processes.

  The values depend only on (type, rate, quality), and the descriptors are `&'static`. A
  process-lifetime launch registry or table (for example a `OnceLock` or an engine-held registry)
  would be sound. Root should decide this before #1372 lands its first derivation.

## Flagged points

1. D2 (b)/(c): see R1. The reading is correct, and the spec problem is root's.
2. Preview precedence: see n2. This is not a contract regression; only the internal Rust-level
   code moves.
3. Per-call registry rebuild: see R2. This is a real gap, and neither this spec nor #1457 covers it.
4. Forced caller edits: all within the named exceptions. Every changed file is in the attempt
   record's forced-caller list or in the authorized paths (`effect-contract` tests included).
   - The edits outside the effect-compiler, the preview and the harness are field or call only:
     `tail_bound:` fields, `Box::new(..)` arguments, and per-rate entry reassignments.
   - The harness API change (`Box<dyn NativeEffectFactory>`, and `EffectDifferential { registry,
     effect }`) goes beyond "the harness's call only", but it is forced. A `RegisteredTailBound`
     can come only from a registry, `NativeEffectRegistry::new` needs an owned `'static` box, and
     the only other route is a fake descriptor-only factory.
   - The harness now also refuses an invalid response-analysis descriptor through the registry,
     which is strictly more checking.
   - `conformance::tail_bound_for_request`'s stand-in entry for an undeclared row
     (`crates/conformance/src/randomized.rs:329`) is test-only and is never read. The EQ's 176.4 kHz
     refusal tests (`crates/parametric-eq/tests/response.rs:1141,1246`) prove validation refuses
     first.
   - The delay corpus (`crates/delay/src/corpus.rs:158-164`) now builds a registry on the control
     side, and the G5 wasm and native pins still match.
5. No bit moved:
   - `audit capi`: `pcm_digest` `cb10fbface44a3a4` (the value recorded for #1460, #1461 and the
     stream G batch), 100,000 calls, 0 allocations, 0 deallocations, 0 locks, 0 syscalls,
     0 violations.
   - `run-wasm-gates.sh --without-v8-spill --without-native`: 143 cases, 252 comparisons,
     0 mismatches.
   - Release `g5_native_digests_match_pins` and `m3_corpus_digests_match_pins`: ok.
   - `check-browser-expected-resources.py --artifacts` (native and simd128 parity): ok.
   - `graph_fixture --check` and 100-process determinism: PASS.
   - The conformance research fixtures `--check`: ok.
   - D4 holds by construction too: every launch `tail_and_rest` is a constant, and M13 shows the
     table routes each rate's own statement.

## Realtime and memory

- No render code changed.
- `RegisteredTailBound` lives in `PrepareEffectRequest` and `EffectBankPreparation`, both
  control-side. `into_effects` (`crates/graph-compiler/src/ids.rs:383-418`) moves only `metadata`
  and `processor` into the plan.
- No processor or bank keeps a request.
- The registry's table is control-side memory.
- No queue is involved, so the acked-batch question does not apply.
- Version-suffix and naming rules are respected (`RegisteredTailBound`, `tail_bound_consistent` and
  `RegistryEntry` are unversioned).

## Test value (one sentence per new or rewritten test)

- `effect-compiler` `tail_bound_count_tests::preparation_and_bank_binding_never_evaluate_the_tail_bound`:
  red when `expected_prepared_metadata` (or anything on the compiler's prepare and the factory's
  bank-bind path) evaluates the descriptor's `tail_and_rest` again instead of reading the entry, or
  when the registry evaluates a row twice. M1 and X4 turn it red and nothing else in
  effect-contract, effect-compiler or conformance. With constant bodies, no value-comparing test can
  see either defect.
- `effect-contract/tests/registry.rs` `a_tail_over_every_peak_below_the_tail_is_refused`: red when
  the registry stops enforcing `tail_every_peak >= tail` (with `Infinite` largest), or checks only
  some rate rows (M2 red only it; M9).
- `an_unstated_rest_with_a_finite_tail_over_every_peak_is_refused`: red when the registry admits
  `Unstated` beside a finite `tail_every_peak`, or checks only some rows (M3 red only it; M9).
- `a_bounded_rest_with_an_infinite_tail_over_every_peak_is_refused`: red when the registry admits
  `Bounded` beside an infinite `tail_every_peak`, or checks only some rows (M4 red only it; M9).
- `effect-contract/tests/tail_bound.rs` `a_request_with_another_rows_entry_is_refused`: red when
  `expected_prepared_metadata` stops comparing the entry's effect, rate or quality with the
  request's (M6, M7, M8), or when `tail_bound` falls back to a row for an undeclared rate (M10).
  Each mutant turns only this test red. It does not catch X3 (m1).
- `prepared_metadata_and_program_key_carry_the_stated_bounds_per_rate` (#1377's test, moved out of
  `src/lib.rs` and rewritten to prepare through registry entries, with the old copy deleted): red
  when the table stores another rate's statement under a row (M13: the 44.1 kHz row evaluated at
  48 kHz; only this test red) or when `program_key` drops `rest` (M14; only this test red). It is
  also red on M11 and M12.

## Mutation runs (mine)

Each mutant was applied to the export and checked with
`cargo test -p effect-contract -p effect-compiler -p conformance --features effect-compiler/test-support --no-fail-fast`.
The source was restored and confirmed pristine after every run.

| Mutant | Change | Red tests |
|---|---|---|
| M1 | `expected_prepared_metadata` calls `tail_and_rest` again | gate-1 test only |
| M2 / M3 / M4 | rule (a) / (b') / (c') dropped | its own gate-2 test only |
| M9 | rules checked at 44.1 kHz only | all three gate-2 tests |
| M6 / M7 / M8 | effect / rate / quality comparison removed | mismatch test only |
| M10 | lookup falls back to the first row | mismatch test only |
| M11 | every row evaluated at 48 kHz | three gate-2 tests and the moved #1377 test |
| M12 | `tail_every_peak` copied from `tail` | gate-1 test and the moved #1377 test |
| M13 (new) | the 44.1 kHz row evaluated at 48 kHz | moved #1377 test only |
| M14 (new) | `program_key` writes `rest: Unstated` | moved #1377 test only |
| X4 (new) | the registry evaluates each row twice | gate-1 test only |
| X3 (new) | lookup ignores quality | **none** (m1) |

## Gates run (x86_64, from the export, all green)

- `cargo fmt --all --check`.
- test-debug-b (`--all-targets`, its features) and its doctests; the conformance
  research-fixture `--check`.
- test-debug-a (`--all-targets`, its features) and its doctests. Across both legs: 2,371 passed,
  0 failed, 35 ignored. All six new or moved tests ran and passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and the same without
  `--all-features`.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- `check-workspace-policy.sh`: ok. `check-realtime-policy.sh`: ok (89 regions in 25 files).
- Release build of `audit`, `bench`, `capi` and `session-validator`; release tests of `audit`,
  `bench` and `console-workload`.
- `audit capi`: see flagged point 5. `audit delay`, `audit compressor` and `audit parametric-eq`
  (100,000 blocks each) and `audit gate-expander`: 0 violations.
- `trace-effect-contract-audit.sh` (1,000,000 blocks): ok.
- `check-capi-abi.sh` and `--self-test`: ok.
- `check-graph-determinism.sh` (100/100) and `graph_fixture --check`: PASS.
- `check-effect-contract.sh target/release/bench`: ok (8 production factories).
- Release `-p lane -p math -p wasm-gates --features math/lane`: ok. Release `ramp_endpoint` for the
  seven effect crates and effect-runtime: ok.
- `cargo build --release -p wasm-gates`; `run-wasm-gates.sh --without-v8-spill --without-native`:
  ok.
- `check-cross-targets.sh`: PASS (x86-64-v3; aarch64 iOS and Android checked; the #1018 expected
  failures only).
- Worklet chain:
  - `build-web-audioworklet.sh --named-twin` (shipped module sha256 `b387e216…44a`);
  - `strip-wasm-names.py --self-test` and its twin check;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts`;
  - `check-scalar-oracle-absent.py --wasm`;
  - `test-web-audioworklet.sh` with a private TMPDIR (left empty).
- Not run: AArch64 (CI only); the V8 spill gate (artifact-gates job, Node 22.23.2); browser legs.
