# Charge the browser boot's projection without processors for banked members

Filed by root from the #2199 verify (attempt 1 MINORs). Follow-up; not launch-blocking.

## Problem
After #2199 the browser boot admits on its projection (`RetainedProjection`, #2199 D4), a ceiling of the exact aggregate. Two of its rows are loose:
- the effect row still charges a processor for every banked member, although #2199 slice 2 prepares no processor for a banked member;
- the send-lane, inventory and later-carry rows are charged at their count bound.

On the dense shape (`dense_document(6_000)`) the projection is 47.8 MB (8.2 %) above the exact total. So the browser boot refuses a session that fits the budget by up to about 8 %, while a replace of the same session at the same budget is admitted (the replace pre-check charges exact rows, #2199 D3).

## Scope
1. Charge the effect row of the browser boot projection with no processor for a member the bank plan banks (the same rule `compile_projected` follows).
2. Tighten the remaining ceiling rows where the count is known before preparation; state the residual slack for each row that stays a ceiling.
3. Size the dense gate shape with margin: today its exact total is only 0.22 % over the 512 MiB budget, so tightening the projection can turn the gate's "exceeds the budget" expectation red. Pick a shape whose exact total is clearly over the budget, and re-measure the native and wasm legs.
4. One builder for the dense shape: today it is copied in `hosts/host-web/tests/boot_transient_budget.rs`, `hosts/host-web/examples/dense_document.rs` and `crates/control-plane/tests/session_replace.rs` (the realtime policy refuses a `#[path]` with `..`). Share it through a crate the tests already depend on, or record why three copies must stay.

## Gates
- The browser boot admits every fixture session at a budget equal to its exact total plus the stated residual slack, and the slack is under 1 % on the dense shape.
- The dense gate still refuses before its rows are allocated, native and wasm.
- No rendered bit moves (one-time comparison against the pre-change base, PR evidence only).

## Decisions

- D1. `GraphProjection::per_node_entries` names, per projected entry, whether the bank plan leaves
  it per node: the planned banks' members, which `compile_projected` binds unchanged, so these
  are exactly the entries it prepares a processor for. host-core charges
  `HostProjection::per_node_processor_bytes` (`EffectProjectedEntry::processor_bytes`: inline
  bytes plus undeclared heap) over them alone, and preparation holds the plan's processor count
  (instances less bank members) to `per_node_effects` (`host.graph.projection`). The browser boot
  takes its effect row from `boot.host` and no longer projects effects itself.
  `EffectStateProjection` keeps the declared state and scratch only; its all-instance processor
  rows and `retained_bytes` are deleted, so nothing can charge every instance a processor.
- D2. Root ruling Q2: the diagnostic keeps `effect_state_bytes`. Its definition
  (`RetainedProjection`) and the two places that parse it (`check-web-boot-budget.mjs` gate D,
  `boot_transient_budget.rs` gate C) say that it excludes the processor of a banked member.
- D3. Root ruling Q1: the send-lane predicate is not lifted. Measured on the dense shape, the send
  lanes are exact (0 projected, 0 prepared) and so is the boot's carry (no successor). The one
  residual is the state inventory (238,000 B at 7,000 tracks, 0.038 % of exact): its route rows
  charge every model route a `GraphLiveRoute` and its ID text, and the dense shape's routes get
  no live op (24 B + ID per route), and its meter ID text is charged at the longest strip ID
  (1,000 B). The live-route count is the send-lane predicate's, so it stays a ceiling. The
  residual is under 1 %, so no successor is proposed. The plan's claim that the inventory slack
  is zero on the dense shape was wrong; this is the measured value.
- D4. Root ruling Q3: the dense gate shape is `session::test_support::DENSE_GATE_TRACKS`, 7,000
  tracks (about 19.2 MB of JSON), exact 627,576,439 B, 16.9 % over the 512 MiB budget.
- D5. One builder: `session::test_support::dense_document`, behind the new `session/test-support`
  feature, which host-web and control-plane enable from `[dev-dependencies]` only. The wasm leg's
  example uses it (examples see dev-dependencies). The three copies are deleted; the 6,000-track
  output was byte-identical to the old example's before the shape moved.

## Evidence (attempt 1)

- Browser boot ceiling over exact on the dense shape (native boot with the module maximum lifted
  in scratch only): base 6,000 tracks 47,820,000 B (8.89 %), 7,000 tracks 55,790,000 B (8.89 %);
  head 6,000 tracks 204,000 B (0.038 %), 7,000 tracks 238,000 B (0.038 %).
- Fixture drift ceiling (`representative_retained_projection_tracks_the_post_prepare_exact_aggregate`),
  re-pinned in millionths at measured + 8 %: identity 32 / 1,118,367 = 28.6 (pin 31), nine-track
  288 / 1,474,483 = 195.3 (pin 211), console 7,496 / 4,953,706 = 1,513.2 (pin 1,635); was 2, 14
  and 78 per mille. Red with every instance charged a processor: identity over-projection 2,016
  exceeds its bound 34.
- Per-node entries (`a_projected_compile_prepares_no_banked_members_processor`, extended): red
  when `per_node_entries` counts a banked member per node (129 against 1).
- Dense boot, native: refused `host.budget.retained_projection` at peak 343,361,120 B under the
  384,115,980 B parse transient (margin 40.8 MB) and under the 536,870,912 B budget; red when the
  projection leaves out the graph row (refused `host.budget.retained_exact` after preparation).
- Dense boot, wasm (`check-web-boot-budget.mjs`): growth 299,368,448 B of the 536,870,912 B
  budget (margin 237.5 MB), document 19,205,799 B.
- Dense replace (`a_replace_whose_graph_cannot_fit_is_refused_before_it_is_prepared`): refused
  `control.resource.replacement_peak` at peak 337,084,980 B under the 384,115,980 B transient.
- No bit moved: 300 blocks of every preparable `fixtures/session/v1` session, plain and live (34
  digests, plus the refused and not-compilable lines) are identical on base 6c1c943da and on head
  (PR evidence, harness in scratch).
