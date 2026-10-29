# Project graph, builtins and control rows against the aggregate budget before allocation (dense-shape residual of #2194)

GitHub #2199. Successor of #2194 (root ruling R3 on its mapping, 2026-10-29).

## Problem

#2194 projects effect state against the aggregate memory budget before allocation. The other
retained rows (graph plan, builtins, control rows) of a boot are still bounded only by the parse
projection, `PARSE_TRANSIENT_MULTIPLIER` (20) x document bytes. The #2191 measurement of a dense
6,000-track console shape (a 16.5 MB document) peaked at about 638 MB during boot, above the
512 MiB default budget, before the post-allocation aggregate check refused it.

## Scope

Project the graph, builtins and control retained rows (from the compiled model, without
allocation) against the remaining aggregate budget before they are allocated, on boot, replace and
rebuild, with the same single-source rule #2194 uses for effect state.

## Gate

The dense 6,000-track shape is refused before its boot peak exceeds the budget (native
thread-scoped peak; wasm linear-memory growth at most budget plus one page). Ordinary console
sessions still boot. One gate is red on the code before the change.
