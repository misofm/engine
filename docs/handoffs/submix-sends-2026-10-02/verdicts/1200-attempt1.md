# #1200 *Render a submix strip on its summed input*: Sol verdict, attempt 1

- Reviewed: `git diff 27892bcc ea951253` (branch `codex/batch-submix-k1`, worktree
  `/home/bl/misofm/wt-submix-k1`). That is 14 files, +1387/-87.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1200-render-a-submix-strip-on-its-summed-input.md`,
  including its "Attempt 1 record" and deviations.
- How I ran it: the #1201 implementer is working in the worktree, so I touched nothing there, not the
  branch and not GitHub (I only read it).
  - I exported `ea951253` with `git archive` to `/tmp/claude-1002/v1200/src`, put it under a
    throwaway local git so the policy scripts see tracked files, and built with my own
    `CARGO_TARGET_DIR`.
  - The disk had about 5 GB free, so I built debug with `CARGO_PROFILE_DEV_DEBUG=0` and release with
    `CARGO_PROFILE_RELEASE_DEBUG=0`. Debug info does not change codegen.
  - I applied every mutation to the scratch copy only and reverted each one. Before cleanup the only
    difference from the commit was my scratch test module. I removed the scratch tree afterwards.
- My scratch tests are kept in `submix-verdicts/1200-attempt1-verifier-scratch.rs` for reuse. They
  are a host-core unit-test module, because only `#[cfg(test)]` code can reach the backend seam.

## Verdict: PASS

There is no BLOCKER and no MAJOR.
- Every deliverable (D0-D7) is present, and every objective gate I could run passes when I re-run it.
- No render-path code changed.
- The re-pins are legitimate consequences of the lowering.
- Gate 1's oracle is a real equivalence. It also holds at W4 and Scalar, for nested buses and for a
  keyed bus.
- D6 holds on every producer path.

There are three MINOR findings and three NITs. None of them needs another attempt.
- MINOR-1 is a small fix, or a follow-up issue.
- MINOR-2 and MINOR-3 are about test value and coverage, for the root to route.

## Gates (re-run by me on `ea951253`, x86-64-v3, native W8)

| Gate | Command | Result |
|---|---|---|
| 1-3, 5, 8 | `submix_strip.rs` (inside test-debug-a) | 7/7 ok |
| 4 | `compile_shapes::a_three_input_bus_keeps_one_reduction_on_its_input_stage` | ok |
| 6 | host-web `live_controlled_boot_of_a_bus_with_an_effect_renders` | ok; `feed_and_render` asserts `RESULT_OK` on submit and render |
| re-pins | `issue122_reverse_route_ids_emit_sorted_levels_and_bind`, `level_major_compiler_coloring_matches_independent_live_intervals`, `route_helpers_map_every_typed_variant_to_its_graph_node`, `representative_console_compiles_with_builtins_and_reports_its_shape` | all ok |
| 7 | `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | exit 0 |
| 7 | `check-graph-determinism.sh` | PASS (100/100) |
| 7 | `graph_fixture -- --check` | exit 0. No fixture contains a session submix; the SCC literal is graph-level (D7). |
| 7 | `check-console-fixtures.sh`, `check-builtins-fixtures.sh` | ok; ok (50 files) |
| 7 | `trace-graph-audit.sh` | PASS (1,000,000 blocks) |
| 9 | the test-debug-a workspace command (exact excludes and features from DESIGN section 7) | exit 0; 98 binaries, 1125 passed, 0 failed, 9 ignored |
| 9 | `cargo test --locked --release -p audit -p bench -p console-workload` | exit 0; 110 passed, 0 failed |
| 9 | `cargo fmt --all -- --check`; workspace clippy `--all-targets --all-features -D warnings` | both exit 0 |
| 9 | `check-`/`test-` policy scripts for graph, builtins, realtime, host-core, workspace, session, effect-runtime, rack and lane | all exit 0 |
| extra | `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps` for the five touched crates (host-core with `control-provider`) | exit 0 |
| 9 | `run-aarch64-tests.sh debug` | **Not runnable here.** The host is x86_64, with no `aarch64-unknown-linux-gnu` target installed, no cross linker and no qemu-user. It must run in CI's `aarch64-debug` job at the batch push. See the W4 substitute below. |

- GitHub: #1200 is OPEN, and its title matches the spec's H1. That is correct for batch mode.
- Authorized paths: every file is on the list except the #1199 spec note. The root asked for that
  note in `K1-followups.md`, so it is in scope.

## Adversarial checks

**Gate 1's oracle is a real equivalence, not a circular one.**
- Session B has no submix at all.
- Its source is the test's own `f32` sum: the D3 expression, summed left to right in route-ID order,
  with route IDs permuted against track order.
- So the test compares the bus path against the established track path plus independent route
  arithmetic. That is exactly the claim. The test does not validate the strip's DSP itself; that
  is not this slice's claim.
- Rates: the 32 seeds draw 44.1k ×7, 48k ×10, 88.2k ×5 and 96k ×10, so every launch rate is
  covered.
- Gate 1 does **not** cover multi-level nesting (see MINOR-3), so I checked it with scratch tests.
  - Each test below rendered at **W8, W4 (`Backend::Simd4` on x86) and Scalar**, through host-core's
    `#[cfg(test)]` backend seam.
  - Every result was bit-identical, with NaNs folded.

  | Scratch test | Seeds | Shape |
  |---|---|---|
  | `v_gate_one_at_every_width` | 32 | Gate 1's shape. |
  | `v_nested_bus_into_bus_equals_two_tracks_fed_their_sums` | 24 | Three tracks into bus1 (strip S1), then into bus2 (S2), with t3 routed straight into bus2 and route IDs permuted. The oracle is two chained "track fed its sum" renders; no submix appears in it. |
  | `v_sidechains_through_buses` | 16 | A bus compressor keyed from a track, against a track fed the bus's sum keyed from the same track. |
  | `v_degenerate_buses_render` | n/a | A fan-in-0 bus routed to the output, and a bus with no outgoing route. The output is bit-identical to the track alone, so the `+0.0` fill holds. |
  | `v_mixed_control_bank_with_a_bus_lane` | n/a | A deep track and a bus share a fader bank (`["bus","t0"]`). One lane has a live channel and the other has none. With live controls attached the render equals the live-control-free render, so the "mixed banks" hazard holds. |

- About the W4 substitute: on x86 the factories bind only the widths the build executes, so this is
  the builtins banks at four lanes, not arm64's effect banks. The real 4-lane run is CI's
  `aarch64-debug`.

**PDC through bus strips.**
- Gate 2 passes as committed.
- Nested case (`v_nested_pdc`):
  - Setup: a limiter on bus1, bus1 into bus2, t1 routed straight into bus2, and t0 routed straight
    to the output.
  - The output latency is 486.
  - An impulse on t0 or on t1 peaks at sample 486 on both planes, so every path is aligned.

**Acyclicity.**
- These are all refused at prepare with `graph.cycle`:
  - a bus-to-bus route cycle;
  - a track keyed from the bus it feeds;
  - a bus keyed from its own output.
- The diagnostic path has regressed; see MINOR-1.

**Dual-mono and the collapse guard (D4).**
- `session_structural_symmetry` is still over tracks only.
- `SessionPoolClasses::from_session` seeds each submix `symmetric_except(SOURCE)`. That is
  behaviorally identical to the old "unknown is Stereo": `conjoin` and `pool_as_stereo` can only keep
  it Stereo. It is now explicit, as D4 asks.
- The host's arming join is the only guard. I reproduced the required mutation (submix IDs added to
  `mono_source`). Gate 3 turns red with "right sample 0 is -1.20308: the bus was collapsed".

**D9 summation order.**
- The submix `Input` reduces its route-destination edges in `GraphEdgeId` order, which is route-ID
  order.
- Gate 1 and my nested test, both with permuted IDs, are bit-exact.
- The implementer's reversed-oracle mutation (one ulp red) shows that the test can tell the two
  orders apart.

**Realtime.**
- No runtime code changed. A bus `Input` is the existing `NodeKind::Identity` reduction, and bus
  effects use the existing effect arms.
- Gate 8 gives exact zero allocations after block 0, using the thread-scoped counters and the render
  audit.
- The realtime policy check and the syscall trace pass.

**Portability.** No target-specific code was added. Lane widths are untouched.

**D6 on every producer path.**
- Producers are created only by `attach_effect_live_controls`, and observation handles only by
  `attach_effect_observation`. Both are filtered.
- host-core's `prepare_host_runtime_with_live_controls_policy*` is the only production caller, and
  it serves every path: host-web boot, plan replacement, and between-render-calls.
- console-workload calls them on its own track-only sessions.
- capi creates no effect producers. Its `control-provider` catalog lists bus-effect parameters as
  read-only metadata, which is harmless.
- host-web files only what it receives (`handles.tracks` binary search).
- The filter lives only in the two attach functions, a binary search over sorted `tracks`, never
  `strips()`. The doc comments are verbatim.
- My mutation, removing the filter from `attach_effect_observation` only:
  - gate 5 goes red (`[("bus","glue"),("t0","comp")]` observations against `[("t0","comp")]`
    controls);
  - gate 6 goes red (`web.live_controls.observation`).

**Re-pins.**
- Each re-pin follows from the new lowering:
  - The reverse-route fixture gains 12 strip nodes, so 26 schedule items and 17 levels.
  - The legacy and old-Kahn swaps move to positions 22-24, the same "a submix's end, then its route,
    before the other submix's end" shape. The test still asserts that the shape is distinct and
    invalid.
  - The level mutations move to 15 and 16.
  - The route-helper mapping matches D1.
  - `compile_shapes` becomes `3 * (256 + 32)` because the 32 submixes are banked strips.
- PCM did not move.
- No new digest was added. The one digest re-pinned (the reverse-route graph identity, #241) is
  pre-existing. Whether that compiled-artifact digest should exist under AGENTS.md's digest rule is a
  question that predates this slice and is not its finding.

**Evidence.**
- I re-derived `bank_route_folds()` for gate 1's 32 seeds on this branch. I got
  `[0,3,0,3,0,3,0,0,0,0,0,0,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3]`, exactly as recorded.
- I did not rebuild the base-commit half, because the disk was too full.

**`$.tracks` overflow paths (deviation 4).** These paths fire only on `u64` overflow of element or
byte counts, which no real session can reach. They are not misleading in practice. See NIT-1.

**Other deviations.** All three are acceptable:
- gate 5 adds a track compressor, so the test is not vacuous;
- gate 6 uses a 16-quantum source;
- gate 2 builds the artifact through the same public pipeline host-core uses.

## Test value (one sentence per new test)

- **`a_bus_renders_the_bits_of_a_track_fed_its_sum`:** red if a bus's input section, inserts, fader
  or pan is skipped, misbound or reordered, or if the bus sums out of route-ID order. My mutation
  confirms it: the bus pan replaced by a static identity is red at seed 0, and no other test is red.
- **`a_bus_insert_is_charged_once_in_the_session_estimate`:** red if every strip's
  console and insert vectors are charged twice.
  - My mutation (`strips().chain(strips())` in the vector loop) turns only this test red.
  - Every session test, the heavy test and all of host-core stay green.
- **`heavy_bus_inserts_compile_and_are_estimated_as_track_inserts_are`:** red if submix inserts drop
  out of the canonical preflight bound, or are charged differently from track inserts (#1199
  MINOR-1). This is the implementer's mutation.
- **`a_bus_limiters_latency_is_compensated_on_the_direct_edge`:** red if bus-strip latency is left
  out of arrival times. This is the implementer's mutation.
- **`a_mono_track_panned_left_into_a_processed_bus_leaves_its_right_silent`:** red if a bus enters
  the collapse eligibility set. **But it is not unique**; see MINOR-2.
- **`no_bus_effect_gets_a_live_channel_or_an_observation_handle`:** red if either attach function
  gives a bus effect a producer or observation lane. My observation-only mutation shows it, in the
  host-core package that also runs on arm64.
- **`a_processed_bus_renders_without_allocating`:** red if a bus strip's lowering leaves an
  allocating render path. No other allocation gate renders a processed bus.
- **`a_three_input_bus_keeps_one_reduction_on_its_input_stage`:** red if reductions still key on
  `Submix` nodes. This is the implementer's mutation.
- **`live_controlled_boot_of_a_bus_with_an_effect_renders`:** red if a live-controlled browser boot
  of a bus session refuses a producer or an observation handle. My observation-only mutation shows
  it: `web.live_controls.observation`.
- I also confirmed D2 with my own mutation: without the `submix_inputs` exclusion, 5 of the 7
  `submix_strip` tests are refused at prepare.

## Findings

### MINOR-1: a bus cycle's diagnostic path lost its route

- What happens:
  - A bus-to-bus route cycle is now reported as `graph.cycle  $.submixes`. On the base it was
    `graph.cycle  $.routes[id=ab]`.
  - A key cycle through a bus is reported the same way.
- Why:
  - `compile.rs` takes `cycle.1.first()` as the primary path.
  - The witness starts at the component's smallest node. That is now a `TrackStage { bus, Input }`
    rather than a `Route`, so its first edge is a strip chain edge, whose sealed path is the bare
    collection path `"$.submixes"`.
- Effect:
  - `cycle_edge_paths` still lists the routes, but host-core's failure text carries only the code
    and the primary path.
  - So an agent that wires an accidental bus loop is told only "somewhere in `$.submixes`". The
    session is still refused correctly.
- Fix (diagnostics only, no digest moves):
  - Prefer the first `$.routes[...]` (or `….sidechain`) entry of `cycle_edge_paths` as `path`, and
    fall back to the first entry.
  - Add a test that a bus↔bus cycle's primary path names one of its routes.
  - Do not change the chain-edge path: it is in the sealed canonical text.
  - This is small enough for the next slice that touches `compile.rs` (#1201 or #1203), or for a K1
    follow-up.

### MINOR-2: gate 3's named defect is not unique to gate 3

- What happens: the required mutation (submix IDs added to the host's eligible set) also turns
  these existing tests red:
  - `collapse_arming::a_post_input_builtins_send_after_an_asymmetric_trim_renders_the_dual_bits`;
  - `collapse_arming::a_post_simd1_send_after_an_asymmetric_eq_renders_the_dual_bits`;
  - `randomized::randomized_consoles_render_the_same_bits_armed_dual_and_serialized`.
- Why: those tests route mono tracks into a unity submix. Once this slice made a unity submix a banked
  strip, the guard became visible to them.
- So the record's "red if a bus enters the collapse eligibility set ... no test routes a mono track
  into a processed bus" is literally true, but it does not answer AGENTS.md's question. The brief
  itself mandated the test with that mutation, and this slice is what falsified the brief's
  premise.
- It is not the implementer's error and carries no product risk. I could not find a plausible defect
  that only gate 3 catches.
- Fix: the root picks one.
  - (a) Keep gate 3 as the named pin for a *processed* bus, and correct the record's test-value
    sentence to say it duplicates collapse_arming and randomized for this defect.
  - (b) Amend the spec to cite collapse_arming's submix-send tests as gate 3's evidence, and delete
    the test in the same PR.

### MINOR-3: no committed test renders bus→bus nesting

- What is missing:
  - Gate 1, bank_levels and randomized route buses only to the output.
  - A nested bus is a real path (drum bus → music bus → master).
  - So a level, bank or PDC defect specific to a bus whose input is another bus's `PostMatrix` would
    reach no gate.
- My scratch `v_nested_bus_into_bus_equals_two_tracks_fed_their_sums` and `v_nested_pdc` pass today.
- Fix: route these to the root.
  - Fold one nested level (one bus into a second bus, plus a direct track into the second bus) into
    gate 1's generator, or into #1208's master-strip gates.
  - The scratch file has a ready oracle (two chained "track fed its sum" renders).

### NIT-1: overflow diagnostic paths say `$.tracks`

`session/src/estimate.rs` sums insert and parameter counts over `strips()` but reports overflow at
`"$.tracks"`, and the vector paths at `"$.tracks.inserts.effects"`. These are unreachable `u64`
overflows. Rename them to strip-neutral paths when the file is next touched.

### NIT-2: three doc comments are stale

The interim makes these statements untrue:
- `HostLiveControlHandles::effect_controls` and the comment at `host-core/src/prepare.rs:885` both
  say "one … per prepared effect instance". In K1, bus effects are excluded. #1207 restores the
  statement.
- `graph/src/runtime.rs`'s `upstream_of_seam` doc, and `PlanUnitEligibility::lane_tracks`, say that
  a submix names no track and is not upstream.
  - A submix strip's stages are now `TrackStage` nodes named by the bus.
  - So its `Input` reduction reports a non-vacuous upstream stage with a SYMMETRIC witness.
  - This is harmless: the structural half never contains a bus. Only the wording is out of date.

### NIT-3: the estimate hand count restates the estimator's constants

- `a_bus_insert_is_charged_once…`'s byte assertion restates the estimator's internal constants
  (×10 canonical bound, 1 KiB per item). That brings it close to AGENTS.md's exact-byte-count rule.
- I keep it as is:
  - the spec asked for a hand count;
  - the byte delta is what gives it its one unique catch (uniform double charging);
  - the brittleness only ever produces a false red, never a false green.

## What I could not verify

- `run-aarch64-tests.sh debug` (no arm64 host and no cross toolchain). It must run in CI's
  `aarch64-debug` at the batch push. The W4 substitute above covers the builtins banks at four lanes,
  not arm64's effect banks.
- The base-commit half of the `bank_route_folds()` evidence (the disk was nearly full). The branch
  half matches the record exactly.
