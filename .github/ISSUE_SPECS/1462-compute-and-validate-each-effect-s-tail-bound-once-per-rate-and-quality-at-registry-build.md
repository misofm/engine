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
  (b) `rest == RestBound::Unstated` if and only if `tail_every_peak == TailSamples::Infinite`;
  (c) `rest == RestBound::Bounded(_)` if and only if `tail_every_peak` is finite.
  A violation refuses the registry with a typed `RegistryError` code (for example
  `effect.tail_bound.inconsistent`) naming the effect.
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
2. **One red mutant per rule.** For each of D2 (a), (b) and (c), a test registry with a descriptor
   that violates only that rule at one launch rate is refused with the D2 code; a consistent
   descriptor is admitted. Each rule's check removed: its test is red.
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
counts. Each gate-2 test turns red if its consistency rule is dropped, which nothing checks today.

## Dependencies

#1377.

## Attempt record

None yet.
