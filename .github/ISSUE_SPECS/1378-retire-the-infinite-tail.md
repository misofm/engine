# Retire the Infinite tail

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b), D15-4(c)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Every plan reports a finite tail and every node states its exact-rest bound, so neither the engine
nor a host can report "no bound". After the per-effect slices, no node returns `Infinite` or
`Unstated`; this slice deletes both from the contract, the graph, the canonical plan text and the
C ABI, so nothing can bring them back.

## Context

- **The variants.** `TailSamples::{Finite(u64), Infinite}` (`crates/effect-contract/src/lib.rs:131-134`);
  `RestBound::{Bounded, Unstated}` added by
  *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377).
- **Graph.** PDC's `Infinite` arms and helpers (`crates/graph-compiler/src/pdc.rs:121-137`,
  `shifted_tail`/`max_tail` `:162-176`); the cap `maximum_finite_tail_samples`
  (`crates/graph/src/lib.rs:702`, `pdc.rs:132-136`); canonical text `finite:{n}`/`infinite`
  (`crates/graph-compiler/src/canonical.rs:141-145`).
- **C ABI.** `MISO_ENGINE_V1_TAIL_FINITE`/`_TAIL_INFINITE` (`crates/capi/include/miso_engine_v1.h:127-128`)
  and the `tail_kind` field of `miso_engine_v1_plan_resource_report` (`:252`); Rust side
  `crates/capi/src/abi.rs:36-39`, `:260-263`, mapping `crates/capi/src/runtime/compile.rs:649-650`;
  `crates/capi/tests/c/abi_smoke.c:20`.
- **Fault injection.** The conformance test effect's `ChangingTail` fault sets `Infinite`
  (`crates/conformance/src/effect.rs:396-398`).

## Decisions frozen for this slice

- **D1. Types.** `TailSamples` becomes `pub struct TailSamples(pub u64)`; `RestBound` is deleted and
  `PreparedEffectMetadata::rest` is `RestSamples`.
- **D2. Graph.** PDC adds tails without a special case; `maximum_finite_tail_samples` is renamed
  `maximum_tail_samples` everywhere it is set. Canonical text writes the tail as a plain integer.
  Every canonical digest that moves is re-pinned one at a time, with the reason in the commit.
- **D3. C ABI.** Remove `tail_kind`, `MISO_ENGINE_V1_TAIL_FINITE` and `MISO_ENGINE_V1_TAIL_INFINITE`;
  `tail_samples` is always the bound. This is a prelaunch V1 layout change (AGENTS.md: prelaunch
  identity stays `V1`); the header, `abi.rs`, the layout checker's expectations and
  `abi_smoke.c` change together. If *Document the seek contract and the C ABI growth rule in the
  header* (#1317) has made fields append-only before this lands, the field stays with a single
  documented value instead, and the spec records that ruling.
- **D4. Fault injection.** `ChangingTail` changes the tail by one sample instead of to `Infinite`.

## Deliverables

1. The type changes, every producer and consumer updated, the C ABI change, the canonical text change
   and its re-pins.
2. Docs: `docs/EFFECT_CONTRACT_V1.md`, `docs/BUILTINS_AND_METERING_V1.md`, `docs/C_ABI_V1_QUALIFICATION.md`.

## Authorized paths

- `crates/effect-contract/src/lib.rs`, `crates/effect-compiler/src/prepare.rs`
- The `tail_and_rest` functions of the eight effect crates (type change only)
- `crates/builtins/src/tail.rs`, `crates/builtins-compiler/src/lib.rs` (type change only)
- `crates/graph/src/lib.rs`, `crates/graph-compiler/src/`, `crates/graph-compiler/tests/`,
  `fixtures/graph/v1/`, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/`
- `crates/capi/include/miso_engine_v1.h`, `crates/capi/src/`, `crates/capi/tests/`,
  `scripts/check-abi-layout-v1.py` (expectations only)
- `crates/conformance/src/effect.rs`
- the three docs above, this spec

## Non-goals

- Any new bound or value. Any browser wire change (the browser status carries no tail today).

## Objective gates

1. **No unbounded node**: at preparation, every effect's and builtin's `rest` is `RestSamples` and
   its tail a number; a conformance run over every effect at every rate checks both against
   `tail_and_rest` (the #1377 gate, now on the new types).
2. **C ABI**: `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`;
   `cargo build --locked --release -p audit && ./target/release/audit capi`; the C smoke test.
3. **Re-pins** limited to canonical text: `bash scripts/check-graph-determinism.sh`; every moved digest
   differs from its old text only in tail tokens (stated in the commit).
4. Commands: the `test-debug-a` and `test-debug-b` commands from `.github/workflows/qualification.yml`;
   `bash scripts/check-effect-contract.sh`; `bash scripts/check-workspace-policy.sh`;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

## Test value

- The type change is the guard: an `Infinite` or `Unstated` value can no longer be written, so no
  new test is needed for it. The updated `ChangingTail` fault keeps the metadata-mismatch path
  covered. Superseded: tests that assert `Infinite` (`crates/host-core/tests/live_lanes.rs`,
  `crates/graph-compiler/src/lib.rs` fixtures, `crates/capi/src/runtime/tests.rs:2725-2727`,
  `:2764-2767`) are rewritten to numbers in this PR.

## Dependencies

- *State the parametric EQ's bounded tail and exact-rest bound* (#1372)
- *State the multiband compressor's bounded tail and exact-rest bound* (#1373)
- *State the delay's bounded tail and exact-rest bound* (#1374)
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375)
- *State exact-rest bounds for the gate, transient shaper and soft clip* (#1376)
- *Define how node tails compose through gain in the graph extent* (#1379)
