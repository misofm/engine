# Port the tools and audits to the with-builtins compile

Split from #959 (option (b), owner ruling 2026-09-27: "We should remove everything related to a builtins-less compile because a builtins-less compile is never needed in production"). Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` and `VERIFY.md`.

## Smallest closable slice

Port every tool that builds a builtins-less plan to `compile_with_builtins` at `Backend::current()`:
- `graph_fixture` (`graph_fixture.rs:89` compiles at `Backend::Scalar` today); regenerate `fixtures/graph/v1/*` and its manifest in this change. This absorbs #947: add #947's regenerate-and-compare test here and close #947 with this issue.
- the #650 audit (`prepared_effect_allocations.rs:226-231`, including the `Zero64` and `CrossedSmall` corpora at `Backend::Scalar`);
- the #006 compile benchmark, including its `graph_validate_65537_tracks` row (needs the linear-compile fix first);
- `tools/audit/src/graph.rs:256` (`audit graph`) and `tools/audit/src/source.rs:374` (`audit source`), which hand-build builtins-less plans with `PreparedGraphPlan::new` and run in CI (`qualification.yml:693-694`). `audit builtins-graph` (`tools/audit/src/builtins_graph.rs:566`) already audits the with-builtins graph and is the model.

## Objective gates

- `rg -nP 'GraphCompiler::compile(?!_with_builtins)' tools` and `rg 'PreparedGraphPlan::new' tools` find nothing.
- Every ported tool's CI step passes; the regenerated fixtures pass #947's compare test; `scripts/check-graph-determinism.sh` passes.
- Every console workload digest unchanged.

## Dependencies

The linear-compile fix; #957.

