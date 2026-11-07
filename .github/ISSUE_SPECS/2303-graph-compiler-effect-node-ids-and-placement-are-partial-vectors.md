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
  own key. Every reader takes it; no reader has a `None` arm, and no reader can run before it
  exists. `prepared_effect_node` is deleted.
- One constructor owns the slot space: `PreparedEffectIndex::from_entries` returns the key index,
  the `EffectNodeIds` and the duplicate count together (`IndexedEffects`), so the index and the
  nodes cannot number different entries. The nodes are `None` as a whole only when some entry's ID
  is not a stable ID, which the front's validation always refuses first; past validation the front
  reads them with one `graph.internal.invariant` arm. The test helper `indexed_effect_ids` uses the
  same constructor.
- `graph_back` refuses with `graph.internal.invariant`, handing every input back, when the
  entries' nodes are not the front's, entry for entry. `GraphProjection::effects_mut` exposes the
  public `entries` `Vec`, so a caller can add, remove, reorder or re-key entries between `project`
  and `compile_projected`; the zips would otherwise drop or mispair them silently.
- `graph_back` places every entry once, after binding the planned banks, as
  `ids::EffectPlacement`:
  - `PerNode(processor)`;
  - `Banked { kept }`, where `kept` is the supplied processor that `compile_with_builtins` keeps
    beside the bank, as before.

  `into_effects` lowers only placements.
- The caller's processors are a typed `ProcessorSupply`: every entry's (`Vec<Box<..>>`,
  `compile_with_builtins`) or none (`Unprepared`, `compile_projected`). It is handed back unchanged
  on a failure, so `compile_with_builtins` loses its `.expect`.
- For every input that compiled or was refused on base, nothing observable moves: diagnostics,
  their order and the preparation calls are the base's, and so are the plan shape, banking and
  rendered bits. The only new refusal is the misaligned-entries one above.

## Non-goals
- GR-9 (automation obligations in `builtins-compiler` and `compile.rs`).
- `crates/graph/src/lib.rs` (GR-6's branch).
- Whether `compile_with_builtins` should keep a banked member's supplied processor beside its bank.
  It always has, and that is preserved.
- No render-path code changes.

## Gates
- No red-on-base test for the two `None`s (unreachable, see the proof). The existing gates defend
  the placement; see the mutation evidence. The misaligned-entries refusal folded in after review
  has its own test, red without the refusal.
- `graph.effect.control_layout` had no test; one now covers it on both compile paths.
- A one-time no-bit-moved comparison over the session fixture corpus, base against head.
- `cargo test --locked -p graph-compiler`.
- `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`.
- `cargo fmt --check`.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p graph-compiler`.
- `bash scripts/check-workspace-policy.sh`.

## Evidence (attempt 1)

**Verified on `b1fc70c0f`.** Both vectors and their readers are as described in Problem. Neither
mismatch is reachable (see the proof), so no red-on-base test exists. GR-9's obligations are not
touched.

**Fix.**
- `crates/graph-compiler/src/ids.rs`:
  - `EffectNodeIds` (`get(slot)`, `iter`) replaces `Vec<Option<EffectNodeId>>` and
    `prepared_effect_node`. As attempt 1 first shipped it, it was built by its own
    `of_validated`; the review fold moved it into `PreparedEffectIndex::from_entries` (see the
    verdict).
  - `EffectPlacement { PerNode, Banked { kept } }` is the only input `into_effects` lowers.
- `compile.rs`:
  - `graph_front` builds `EffectNodeIds` once, after validation, and the node loop reads it
    instead of filling it.
  - `graph_back<S: ProcessorSupply>` places every entry once, after `bind_planned_banks`.
    `Vec<Box<..>>` keeps every supplied processor (banked ones as `Banked { kept: Some }`).
    `Unprepared` prepares only the per-node entries, in entry order, and still prepares them when
    a `graph.effect.control_layout` diagnostic is already pending, as the base did.
  - `compile_with_builtins` drops its `.expect`, and `compile_projected` drops its all-`None`
    vector.
- `banks.rs`: `rack_chains`' and `uncollected_console_slot`'s unreachable "entry with no node"
  `graph.internal.invariant` arms are deleted.
- `GraphProjection::per_node_entries` reads `!banked.contains(node)`. Its public signature is
  unchanged.

**Test value.** No test is added. The change makes the two states unrepresentable, so no behaviour
test can tell base from head (see the proof), and the existing gates defend the placement itself
(mutations below). No test became useless.

**Mutations (each reverted).**
- `Unprepared::place` prepares banked members too (`if banked && false`): red.
  `projected_compile::a_projected_compile_prepares_no_banked_members_processor` (`:362`) and
  `a_banks_bytes_are_within_its_row` (`:435`) fail.
- `Unprepared::place` with the banked test inverted: red, `a_projected_compile_prepares_no_banked_members_processor` (`:362`).
- `per_node_entries` inverted: red, the same two tests (`:367`, `:435`).
- `Vec` supply drops a banked member's supplied processor (`kept: None`): **survives** the whole
  `cargo test -p graph-compiler`. Nothing in this crate observes that `compile_with_builtins`
  keeps a banked member's processor beside its bank; see the follow-up.

**No bit moved (one-time, not committed).** A scratch integration test was run in release on base
`b1fc70c0f` and on head, then removed. It covered every `fixtures/session/v1/*.json`, plain and
with a `miso.delay` appended to the first track's inserts, at `Simd4` and `Simd8`, through both
`compile_with_builtins` and `project` + `compile_projected`: 140 configurations, of which 84
compile and 56 are refused (the same refusals on both). For each it recorded:
- `GraphCompiler::sha256`;
- the `graph_resource_estimate` and the full `GraphCompileReport` (`Debug`), which includes the
  rack cohorts and bank plan;
- every `effect_bank_members` list (940 bank lines);
- `per_node_entries` (26 projected plans mix banked and per-node entries);
- an FNV-1a hash of 32 rendered quanta of PCM from deterministic sources.

The two outputs are byte-identical: sha256 of both files
`1f453d5d2a8f2a5743fbb544d2d87611ec345370bf540d466e9feb61ff8bac87`. Banking did not change.

**Gates (head).**
- `cargo test --locked -p graph-compiler`: 23 test binaries, 204 passed, 0 failed, 2 ignored.
- `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p graph-compiler`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- Realtime policy tool: not run; no render-path code changed (compile-time only, and the PCM above
  is unchanged).

**Follow-ups (not in this issue).**
- `compile_with_builtins` keeps a banked member's supplied processor as a per-node
  `GraphPreparedEffect` beside its bank, and no graph-compiler test observes it (the surviving
  mutation above). The reviewer confirmed it is dead weight, unobserved and freed at bind; the
  coordinator tracks it in #2258.

## Verdict (attempt 1)
**PASS** (Sol's adversarial review), with these folded in on the same branch after merging
`origin/main` (`5f75ca78d`):
1. **Misaligned entry lists.** `GraphProjection::effects_mut` exposes the public `entries` `Vec`,
   so a caller could change it between `project` and `compile_projected`, and attempt 1's zips
   then dropped or mispaired entries silently. `graph_back` now rebuilds the nodes from its own
   entries and refuses with `graph.internal.invariant`, handing every input back, unless they
   equal the front's. That is stronger than a length check: it also catches reordered or re-keyed
   entries. Narrowing `effects_mut` to per-entry lane attachment was not done. Its one caller,
   host-core, hands it to `effect_compiler::attach_projected_effect_live_controls(&mut
   EffectProjectedSession)`, so narrowing it changes an effect-compiler API and host-core. Even a
   slice would still let a caller re-key entries through their public fields, which the
   comparison catches anyway.
2. **One slot space.** `PreparedEffectIndex::from_entries` returns `IndexedEffects { index, ids,
   duplicates }`, and `EffectNodeIds::of_validated` is deleted. The nodes come back `None` as a
   whole only when some entry's ID is not a stable ID. Such an entry matches no declared key, so
   validation refuses it first, and the front's one remaining arm is `graph.internal.invariant`.
   `rack_chains`' unreachable `effects.entry(slot.index())` arm is replaced by
   `EffectInputs::at(slot)`.
3. **`graph.effect.control_layout` test.**
   `projected_compile::a_lane_other_than_the_planned_one_is_refused_on_both_compile_paths` plans
   lanes (`plan_projected_effect_live_controls`), attaches none, and expects
   `graph.effect.control_layout` from both `compile_projected` and `compile_with_builtins`.
4. **Follow-up.** The kept banked processor goes to #2258.

**Fold tests and mutations (each reverted).**
- `a_projected_compile_refuses_entries_changed_since_its_projection` removes the per-node delay,
  or swaps the first and last entries, through `effects_mut`.
  - With the refusal disabled (`if false && ...`), both cases compile: red, "a
    projection whose entries were removed compiled" when run alone, and "...swapped compiled".
  - Removing a banked last entry instead is already refused, by the bank bind. So the test removes
    the per-node one, which only the new check catches.
  - Test value: red when the back half lowers entries other than the front's, pairing each with
    another entry's node, bank and placement.
- `a_lane_other_than_the_planned_one_is_refused_on_both_compile_paths`:
  - Reviewer's M3: the `Vec` supply's pending-diagnostic refusal removed. Red on the prepared path
    ("a prepared compile without its planned lanes compiled").
  - The `control_layout` push removed: red on the projected path ("a projected compile without
    its planned lanes compiled").
  - Test value: red when a back half lowers a plan whose estimate charged lanes it does not hold.

**No bit moved (re-run after the fold, one-time, not committed).** The same probe ran on the
merge parent `5f75ca78d` (`origin/main` as merged) and on head `50e4147fa`. Both outputs are
byte-identical, sha256 `1f453d5d2a8f2a5743fbb544d2d87611ec345370bf540d466e9feb61ff8bac87`, which
is also attempt 1's value: 140 configurations, 84 compiled and rendered, 940 bank lines.

**Gates (head after the fold).**
- `cargo test --locked -p graph-compiler`: 23 test binaries, 206 passed, 0 failed, 2 ignored.
- `cargo clippy --locked -p graph-compiler --all-targets -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p graph-compiler`: clean.
- `bash scripts/check-workspace-policy.sh`: ok.
