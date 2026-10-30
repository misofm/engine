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

## Decision record

Root rulings (2026-10-30, attempt 1):

- Q1: the boot may run the graph front twice (once to project, once in the preparation it
  admits). `HostProjection` is not threaded through `compile_children`.
- Q2: a control row that exists only for a successor (later carry sections, restarts, fade
  arms, watch, pairings, transition send lanes) is charged at its count bound. The ceiling may be
  loose there. Superseded for every admission before allocation by D3 (attempt 2): Q2's ceiling
  refused the C ABI's exact boundaries, so it stays only in the browser boot's projection (D4),
  and `successor_swap` keeps its exact equality (successor = fresh + carry program).
- Q3: there is no fallback. Every launch effect answers
  `NativeEffectFactory::designed_channel_symmetry` from its request alone, without instance state.
- Q4: the wasm gate may boot at the default 512 MiB budget.

Implementation decisions:

- D1. Effect entries are projected first (`EffectProjectedEntry`: a prepared entry minus its
  processor). The graph compile splits into `GraphCompiler::project` (front, bank plan, estimate)
  and `GraphCompiler::compile_projected` (bind the planned banks, prepare a processor only for an
  instance the plan leaves per node, lower). Bank planning binds each slot to read the factory's
  answer and drops the bank at once; no per-instance processor exists beside a bound bank. This
  closes #2194 residual (c).
- D2. `prepare_or_project` in host-core returns a `HostProjection` and admits it
  (`host.graph.resource.limit`) before any processor, bank, plan, ring or lane exists. Exact rows:
  graph, builtins, effect producers, sources, carried sources, source control set, provider
  catalog, strip heap and observation counts. Ceiling rows: send lanes, carry beyond the carry
  program (Q2) and the state inventory. Preparation checks its report against the projection and
  refuses a disagreement as an internal fault (`host.graph.projection`, `host.control.projection`).
- D3. Every admission before allocation charges the exact rows only: the carry program, not the
  successor's later carry sections; no send-lane or inventory ceiling. So it never refuses a
  session or candidate that the exact admission after bind admits (the exact-boundary tests of the
  C ABI stay green unchanged). The exact admission after bind is unchanged and still authoritative.
  Stated limit: a candidate can pass the pre-check and be prepared before the exact admission
  refuses it only inside the rows D3 leaves out (send lanes, inventory, later carry sections).
  The verify sweep of the dense replace measured that window at under 512 KiB, about 0.1 % of the
  budget (refused before prepare at 418,643,968 B or less, prepared then refused at 418,906,112 B,
  admitted at 419,168,256 B or more).
- D3a. The dense replace gate runs at a 256 MiB replacement peak, not the plan's 512 MiB: the
  dense candidate fits from about 400 MiB, so 512 MiB would admit it and test nothing.
- D4. The browser boot charges the whole projection (`RetainedProjection`: graph, builtin and
  control rows beside the bridge and sources), a ceiling of the exact aggregate, before it
  prepares. It projects under unbounded caps so the budget, not a per-row cap, classifies the
  refusal (`host.budget.retained_projection`). A budget equal to the exact total of a session can
  now refuse it when the ceiling rows are loose; the structural-apply budget test boots at the
  larger of the two.
- D5. The control plane's replacement pre-check (`admit_prospective_projection`) projects the
  candidate through `RuntimePreparer::project` (default implementation; web and C ABI override it)
  and admits `projected_plan` beside every held plan and model (`validate_replacement_peak`,
  single peak, with the backpressure split), before the candidate is prepared.
- D6. #2194 residual (a): `bridge_fixed_bytes` is the bridge minus the model, the boot's control
  rows and the session document, and a replace parse adds the held control rows
  (`held_control_bytes`), so grown control rows are charged and the boot staging is not.
- D7. Observation lanes are still attached before the projection (their counts are exact). The
  graph front retains about the plan's graph metadata during the projection (about 18.8 bytes per
  document byte for the dense shape); this is a stated limit for very dense documents.
- D8. The realtime source policy refuses a `#[path]` with a parent component, so each test builds
  the dense shape itself and `hosts/host-web/examples/dense_document.rs` emits it for the wasm leg.

## Gates and evidence

- Dense shape, native (`hosts/host-web/tests/boot_transient_budget.rs`,
  `the_dense_shape_is_refused_before_its_rows_are_allocated`): base boots to a 637,957,932-byte
  thread peak and is refused `retained_exact`; head is refused `retained_projection` before
  preparation, peak 309,697,184 under the 329,235,980-byte parse transient.
- Dense shape, wasm (`scripts/check-web-boot-budget.mjs`): base traps
  (`RuntimeError: unreachable`); head refuses it with linear-memory growth 267,976,704 of the
  536,870,912 budget (document 16,461,799 bytes).
- Dense replace (`crates/control-plane/tests/session_replace.rs`,
  `a_replace_whose_graph_cannot_fit_is_refused_before_it_is_prepared`): base peaks at 632,513,152
  bytes before the post-preparation refusal; head refuses `control.resource.replacement_peak`
  before preparation, peak 304,317,916.
- Fix (a) (`a_replace_parse_projects_the_replaced_sessions_control_rows`): base live term
  4,242,900, under the replaced console's 4,737,924; head 4,927,060 (bound: at most 44 per mille
  over).
- Projection against preparation (`crates/host-core/tests/retained_projection.rs`): exact rows
  equal, ceiling rows at least the report's, for the nine-track fixture and the 64-track console,
  all lanes and fader/matrix only.
- Graph split (`crates/graph-compiler/tests/projected_compile.rs`): one processor per per-node
  instance and none for a banked member (red with prepare-all in the back half: 129 against 1);
  the projection's estimate is the compiled plan's (red with an uncapped estimate).
- Channel symmetry (`every_launch_factory_designs_its_processors_channel_symmetry`): red on an EQ
  value-based symmetry and on a soft-clip skipped word.
- Drift test inverted to a ceiling
  (`representative_retained_projection_tracks_the_post_prepare_exact_aggregate`): any undershoot
  is red (red without the graph row: 1,011,065 < 1,118,335). Measured over-charge 1.80, 12.16 and
  71.56 per mille; pinned at 2, 14 and 78 per mille (measured plus 8 %).
- No bit moved: 300 blocks of every preparable `fixtures/session/v1` session (17 sessions, the
  console fixtures included, plain and live: 34 digests, plus the refused and not-compilable
  lines) give identical digests on base 09f978bb4 and on head (PR evidence, not a committed test).
- Browser qualification `--check-matrix` on Chromium and WebKit, single and worker legs,
  console-limits included: all gates passed.

## Residuals of #2194 closed

- (a) closed by D6.
- (c) closed by D1.
