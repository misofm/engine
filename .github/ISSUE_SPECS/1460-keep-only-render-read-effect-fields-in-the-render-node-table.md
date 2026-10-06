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
- `crates/host-core/src/response.rs`, its test module only (the gate-4 test; attempt 1 found that
  no existing test reaches the prepared-bypass path)
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

- `response::tests::a_per_node_effect_snapshot_reports_its_prepared_bypass` (host-core, added in
  attempt 1): a snapshot row whose prepared bypass comes from the wrong place (the moved field
  filled with `false`) is red; with that mutant, every test of `graph`, `graph-compiler`,
  `host-core`, `capi`, `host-web`, `builtins-compiler` and `effect-compiler` stayed green.
- A wrong `quantum` in `EffectNode::new` that is smaller than the plan quantum is caught by the
  existing graph tests (every per-node effect render refuses the block). A larger one is not
  caught and cannot be: `EffectProcessBlock::new` uses the quantum only as an upper bound on the
  block's frames, and the plan's blocks never exceed the prepared quantum.

## Dependencies

- None open. #1377 depends on this slice.

## Attempt record

### Attempt 1 (2026-10-06)

Commits on `codex/d15-stream-g`: `23a32823a` (#1377 attempt 1 reverted, history kept; workspace
clippy clean at that point), `fcb7151b7` (this spec, #1377 Amendment 2, STREAMS.md), `0427e8647`
(implementation), `d9441fb2b` (the gate-4 test). GitHub issue #1460 was created with the spec as
filed; this record is local until root syncs it.

**What changed.** `runtime::EffectNode { processor, quantum }` is `NodeKind::Effect`'s payload and
`LiveControlEffect::effect` (D1). `GraphPreparedEffect` keeps its shape and is documented as the
plan's control-side record; bind reads `automation_capacity`, `latency` and the window check from
it, as before, then moves only the processor and `quantum` (D2). `ResponseOwnerFacts` replaces the
`(bool, &'static str)` response map value and carries `prepared_bypass` into
`ResponseOwnerBinding` (D3). `graph::EFFECT_RENDER_NODE_BYTES` replaces
`size_of::<GraphPreparedEffect>()` in `graph-compiler`'s live-control owner charge and its test
mirror (D4).

**Gate 2, sizes** (x86-64, `size_of`, debug and release equal; measured with a temporary test that
was removed before the commit):

| Type | Before | After |
|---|---|---|
| `NodeKind` | 200 | 32 |
| `RuntimeOp` | 280 | 112 |
| `RuntimeUnit` | 280 | 248 (the `Bank` variant now sets it) |
| `LiveControlEffect` (boxed) | 304 | 128 |
| `ResponseOwnerBinding` | 72 | 72 (`prepared_bypass` sits in padding) |
| `GraphPreparedEffect` (control side, unchanged) | 200 | 200 |
| `EffectNode` (new) | -- | 24 |

**Gate 1, bit identity** (one-time, not committed). Base = `23a32823a`'s `crates/graph` and
`crates/graph-compiler` checked out over head, rebuilt, and run beside head:
`audit capi` gives `pcm_digest` `cb10fbface44a3a4` on both and the whole JSON record is identical
(no resource count moved); `graph_fixture` output is byte-identical. At head:
`graph_fixture --check`, `conformance_fixtures --check`, `check-graph-determinism.sh` (100/100),
`check-builtins-fixtures.sh` and `check-browser-expected-resources.py --artifacts` ("expected.json
digests and exact rows agree with the built simd128 module") pass against the committed values.
No fixture, digest or resource count was re-pinned.

**Gate 3.** `audit capi`: 0 allocations, 0 deallocations, 0 syscalls, 0 violations.
`check-realtime-policy.sh` passes. `execute_op` reads `effect.quantum` and `effect.processor`
inline in the op, as it read `effect.metadata.quantum` before; no pointer step was added.

**Gate 4.** No render-owned struct holds `PreparedEffectMetadata`: `NodeKind` and
`LiveControlEffect` hold `EffectNode`; the snapshot's prepared bypass reads the binding row.

**Gate 5.** `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets
[--all-features] -- -D warnings` (both, no allow added); `RUSTDOCFLAGS='-D warnings' cargo doc
--locked --workspace --no-deps`; test-debug-a and test-debug-b (the `qualification.yml`
commands); `check-workspace-policy.sh`, `check-lane-policy.sh`; `check-capi-abi.sh` and its
`--self-test`; `check-cross-targets.sh` (PASS, the known #1018 iOS memset rows only);
`run-wasm-gates.sh --without-v8-spill --without-native`; the worklet chain (build into fresh
output directories, `strip-wasm-names.py check`, `check-web-audioworklet.sh
--without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
`check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`). All pass. AArch64 runs in CI
only.

**Mutation evidence.**

| Mutant | Result |
|---|---|
| `prepared_bypass: false` for every per-node effect (`RuntimeParts::new`) | Green on every test of `graph`, `graph-compiler`, `host-core`, `capi`, `host-web`, `builtins-compiler`, `effect-compiler`; red on the new `a_per_node_effect_snapshot_reports_its_prepared_bypass` (`left: false, right: true`); green after revert, and green on the base code |
| `quantum: metadata.quantum / 2` in `EffectNode::new` | Red: 8+ graph tests (for example `enabled_and_bypass_pdc_align_at_launch_rates_and_quanta`, `fifty_random_dag_sessions_render_deterministic_nonsilent_pcm`) |
| `quantum: metadata.quantum + 1` | Green, and benign (Test value) |

**Open.** The GitHub body of #1460 does not carry this record or the Authorized-paths and
Test-value updates above (one `gh issue create` was authorized). #1377 re-applies `d0af9d53d` on
top of this slice.
