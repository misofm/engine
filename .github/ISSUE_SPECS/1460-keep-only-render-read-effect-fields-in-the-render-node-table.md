# Keep only render-read effect fields in the render node table

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by the decision-15 root coordinator's ruling on *Carry each effect's tail and
exact-rest bound in its prepared metadata* (#1377, attempt 1). A prerequisite slice: #1377 rebases
onto it. Code anchors verified on `codex/d15-stream-g` at `23a32823a` (#1377 attempt 1 reverted).

## Product outcome

The render node table holds, for a per-node prepared effect, exactly the fields render reads: the
processor and its block quantum. The rest of the prepared effect -- its `PreparedEffectMetadata`,
node identity and response facts -- stays in the prepared plan's control-side effect table, keyed
by node. Adding a control-only field to the metadata (as #1377 does with `tail_every_peak` and
`rest`) then costs render-owned memory nothing.

## Context

- **The render node.** `runtime::NodeKind::Effect(GraphPreparedEffect)`
  (`crates/graph/src/runtime.rs:1136`) holds the whole record inline: `id: EffectNodeId`,
  `metadata: PreparedEffectMetadata` (104 bytes), `processor: Box<dyn PreparedNativeEffect>`,
  `response_snapshot_declared`, `native_id` (`crates/graph/src/lib.rs:889-898`). That variant sets
  the size of every op in the table: `size_of::<NodeKind>()` 200, `RuntimeOp` 280, `RuntimeUnit`
  280 (x86-64). `LiveControlEffect` holds another copy (`runtime.rs:1185`, 304 bytes).
- **What render reads.** `execute_op` reads `effect.metadata.quantum` and `effect.processor`
  (`runtime.rs:3478-3507`, `:3545-3590`); `channel_symmetry` reads the processor (`:1822-1824`).
- **What else reads it, all at bind or off the render path.** `LiveControlEffect::new` sizes the
  staging window and shunt from `metadata.automation_capacity` and `metadata.latency` and checks
  the window (`:1200-1245`, bind). `RuntimeParts::new` copies `response_snapshot_declared` and
  `native_id` into the response rows (`:4517-4545`, bind). The response snapshot reads
  `metadata.bypass` from the node (`:1507-1508`). Nothing else in the workspace reads a
  `GraphPreparedEffect` after bind; `graph-compiler` builds it (`ids.rs:408`) and charges its size
  for the live-control owner (`estimate.rs:197`, mirrored by its test at `lib.rs:3181`).
- **Why now.** #1377 adds `tail_every_peak` and `rest` to `PreparedEffectMetadata`; the variant
  then passes clippy's `large_enum_variant` limit. Root refused boxing it or allowing the lint:
  render-owned memory carries no control-only data (#1329 ruling R5).

## Decisions frozen for this slice

- **D0. Root ruling (2026-10-06, binding).** The complete form, as a prerequisite slice: render's
  node keeps exactly the fields render reads; the prepared metadata lives in the prepared plan's
  control-side table keyed by node. #1377 rebases onto this slice and keeps its D3 fields there.
- **D1. The render node.** A new `runtime::EffectNode { processor, quantum }` is the payload of
  `NodeKind::Effect` and the `effect` field of `LiveControlEffect`. Bind builds it from the
  plan's record; the processor moves, `quantum` is `metadata.quantum`. Render reads the same two
  values it reads today, inline, with no added pointer step.
- **D2. The control-side table.** `PreparedGraphPlan::effects` (one `GraphPreparedEffect` per
  effect node, keyed by `id`) is that table; the record keeps its fields and its public shape, so
  `graph-compiler` and every construction site are unchanged. Bind reads the metadata there, on
  the control thread, and the record never enters render-owned memory.
- **D3. The snapshot's prepared bypass** moves to the response owner's binding row
  (`ResponseOwnerBinding::prepared_bypass`, filled at bind from `metadata.bypass`; `false` for
  every other owner). It fits the row's padding, so the row's size does not change.
- **D4. The estimate** charges the render node's size for the live-control owner:
  `graph::EFFECT_RENDER_NODE_BYTES` (`size_of::<runtime::EffectNode>()`) replaces
  `size_of::<GraphPreparedEffect>()` in `estimate.rs` and in its test mirror.

## Deliverables

1. `EffectNode`, the two `NodeKind`/`LiveControlEffect` changes, the response row field (D1, D3).
2. `EFFECT_RENDER_NODE_BYTES` and the estimate change (D4).
3. Every resource byte count the smaller op moves, re-pinned one at a time with its reason.
4. The attempt record: sizes before and after, the bit-identity differential, the gates.

## Authorized paths

- `crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs` (Stream A's; named exception)
- `crates/graph-compiler/src/estimate.rs` (the live-control owner charge) and
  `crates/graph-compiler/src/lib.rs` (its test mirror only) (Stream A's; named exception)
- Resource fixtures whose recorded bytes include the runtime op size, each re-pinned with its
  reason in the attempt record (the attempt record lists them)
- `docs/handoffs/decision-15-2026-10-05/STREAMS.md` (the hot-file note and the Stream G row)
- `.github/ISSUE_SPECS/1377-*.md` (Amendment 2), this spec

## Non-goals

- Any change to `PreparedEffectMetadata`, to what an effect computes, or to render arithmetic.
- Banks, builtins, routes, bound processors: their render data is not in this variant.
- A retained post-bind copy of the metadata: nothing reads it after bind today. A later reader
  states its own need and home.

## Hazards

- Stream A owns `crates/graph` and `crates/graph-compiler`. This slice lands before A's graph
  work (STREAMS.md hot-file note); A rebases onto it.
- The smaller op moves the graph runtime-metadata estimate (`RuntimeOp`/`RuntimeUnit` sizes per
  emitted op). Every moved byte is a resource count, not audio; each is named with its reason.

## Objective gates

1. **Bit-identical render.** A one-time differential, base (`23a32823a`) against head, recorded in
   the attempt record and not committed as a digest test: `graph_fixture --check`,
   `conformance_fixtures --check`, `check-graph-determinism.sh`, `check-builtins-fixtures.sh`,
   `audit capi` (`pcm_digest`) and the browser expected digests (`run-wasm-gates.sh`, the worklet
   chain) give the same audio digests on both.
2. **The render node table shrinks**, measured: `size_of` of `NodeKind`, `RuntimeOp`,
   `RuntimeUnit`, `LiveControlEffect` before and after.
3. **No allocation and no new indirection on the render path.** `audit capi` 0 allocations and 0
   syscalls; `check-realtime-policy.sh`; `execute_op` reads `quantum` and `processor` from the node
   inline (review).
4. **Every control-side reader moved:** no render-owned struct holds `PreparedEffectMetadata`
   (review of `runtime.rs`; the snapshot's prepared bypass comes from the binding row).
5. **Clippy clean without an allow**, with and without `--all-features`; fmt, doc,
   `test-debug-a/b`, the policy scripts, cross targets and wasm gates.

## Test value

- No new test is planned. The two plausible defects are each covered by an existing test, which
  the attempt record proves by mutation: a wrong `quantum` in `EffectNode::new` (every per-node
  effect render refuses the block) and a wrong `prepared_bypass` source (the snapshot of a
  prepared-bypass effect reports the wrong state or is refused). If a mutation is green, the
  attempt adds the discriminating test and records it red on the mutation.

## Dependencies

- None open. #1377 depends on this slice.

## Attempt record
