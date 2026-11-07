# Graph compiler: effect node IDs and processor placement are partial vectors whose readers disagree on None

Item GR-10 of the implicit-state-machine tracker #2258 (audit of 2026-11-06, agent-reported,
re-verified here on `b1fc70c0f`).

## Problem
The graph compiler's front and back halves (`crates/graph-compiler/src/compile.rs`) carried two
per-entry vectors of `Option`s. Each was filled by one pass and read by later ones, an order held
only by convention, and each reader decided for itself what `None` meant.

- `effect_ids: Vec<Option<EffectNodeId>>` was created all-`None` in `graph_front` and filled in
  its node loop (`effect_ids[index] = Some(id)`). Its seven readers read `None` five ways:
  - `.expect`: the sidechain-edge loop (`prepared_effect_node(..).expect("prepared effect node
    assigned")`) and `ids::into_effects`;
  - dropped silently by `filter_map`: the per-node set passed to `placement_levels`;
  - `graph.internal.invariant`: `banks::rack_chains` and `banks::uncollected_console_slot`;
  - "renders per node" through `is_none_or`: `GraphProjection::per_node_entries` (which host-core
    charges processor bytes by) and `graph_back`'s preparation loop.
- `processors: Vec<Option<Box<dyn PreparedNativeEffect>>>` meant "not supplied: prepare it if per
  node" before `graph_back`'s preparation loop and "bank member, never prepared" after it, and
  `into_effects` read the second meaning. `compile_projected` built it all-`None`,
  `compile_with_builtins` all-`Some`, and the latter unwrapped the handed-back vector with
  `.expect("every processor was supplied and kept")`.

**Not reachable today (the proof).** `graph_front` returns before its node loop when any of
`graph.effect.duplicate_prepared`, `missing_prepared`, `unexpected_prepared` or
`metadata_mismatch` is pushed. Past that point the entries' keys are unique, every key is a
declared session key, and every declared key has an entry, so the node loop assigns every slot:
no `None` survives into any reader. In `graph_back`, every per-node entry either receives a
processor or the function returns, so after the loop `None` can only mean "banked". No session can
reach a mismatch, so no regression test can be red on base; this issue records the proof and makes
both states unrepresentable instead.

## Contract
- `ids::EffectNodeIds` is one total vector, one `EffectNodeId` per entry, derived from the entry's
  own key and built once in `graph_front` straight after validation. Every reader takes it; no
  reader has a `None` arm, and no reader can run before it exists. `prepared_effect_node` is
  deleted, and the test helper `indexed_effect_ids` uses the same constructor.
- `graph_back` places every entry once, after binding the planned banks, as
  `ids::EffectPlacement`:
  - `PerNode(processor)`;
  - `Banked { kept }`, where `kept` is the supplied processor that `compile_with_builtins` keeps
    beside the bank, as before.

  `into_effects` lowers only placements.
- The caller's processors are a typed `ProcessorSupply`: every entry's (`Vec<Box<..>>`,
  `compile_with_builtins`) or none (`Unprepared`, `compile_projected`). It is handed back unchanged
  on a failure, so `compile_with_builtins` loses its `.expect`.
- Nothing observable moves. Diagnostics, their order and the preparation calls are the base's, and
  so are the plan shape, banking and rendered bits.

## Non-goals
- GR-9 (automation obligations in `builtins-compiler` and `compile.rs`).
- `crates/graph/src/lib.rs` (GR-6's branch).
- Whether `compile_with_builtins` should keep a banked member's supplied processor beside its bank.
  It always has, and that is preserved.
- No render-path code changes.

## Gates
- No red-on-base test (unreachable, see the proof). The existing gates defend the placement; see
  the mutation evidence.
- A one-time no-bit-moved comparison over the session fixture corpus, base against head.
- `cargo test --locked -p graph-compiler`.
- `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`.
- `cargo fmt --check`.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p graph-compiler`.
- `bash scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)
Pending.
