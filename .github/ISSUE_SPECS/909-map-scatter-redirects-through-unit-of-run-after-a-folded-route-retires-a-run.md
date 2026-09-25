# Map scatter redirects through unit_of_run after a folded route retires a run

## Defect

`apply_scatter_redirects` (`crates/graph/src/runtime.rs`) indexed `units[run]`, but the route fold (#218/#419) skips retired route runs when the unit list is built, so any chain that takes a scatter redirect after a folded chain's retired route either panicked at bind (`index out of bounds`) or repointed another unit's member buffer. Found by #886's implementer while widening redirect eligibility to metered plans; fixed in commit `e8cab034` on `codex/886-direct-scatter-under-observation` by mapping through `unit_of_run`, the same mapping `arm_resident_inputs` already uses. The mapping is the identity whenever no run is retired, so no plan that previously bound changes a bit.

## Evidence

- Regression test `a_redirect_after_a_retired_route_lands_on_its_own_chain` (graph runtime tests): four `PostMatrix` bank-to-route tracks folding into a bus, then the bus runs a one-lane `PostInputBuiltins` bank into a bound `PostFader`, a route and the output; `[redirects, folds]` is `[1, 4]` against `[1, 0]` declined, master bit-equal.
- Reachability from a compiled Session V1 is **not demonstrated**: routes lower only to submix or output inputs and submixes have no strip, so a reaching shape needs a track chain whose first op is later in level-major order than a folded route. Neither the implementer nor the Fable reviewer could construct one from the compiler, nor rule it out.

## Scope of this issue

Bookkeeping and one bounded question. The fix lands with the pure-path batch (`codex/batch-885-886-898-900`); this issue closes with that merge. The one open question, whether a Session V1 document can reach the old index, is a stateless follow-up: either construct the fixture and add it to the graph-compiler corpus, or record in `docs/rulings/` why the shape is unreachable. No engine change is authorized here.

Source: #886 attempt 1 evidence and its Sol verdict in `.github/ISSUE_SPECS/886-keep-the-direct-scatter-eligible-when-a-later-tap-is-observed.md`.
