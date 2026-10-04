# Move a source consumer into a successor graph plan

Slice 2 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A graph plan can be bound with a **vacant** source entry, and at the swap block it takes the
predecessor's `PcmSourceConsumer` for that entry: the ring, its queued blocks, its generation, its
read position and its pending generation-change flag. The successor's next block then reads the same
audio the predecessor would have read. This is the render half of source continuity; *Prepare a
successor plan whose unchanged sources keep playing* (slice 3) builds the host-core half on it.

## Context

- `SourceGraphSourceSetDriver` (`crates/source/src/lib.rs:1407`) owns `Box<[GraphSourceEntry]>`
  (`:1403`, one `PcmSourceConsumer` each, `:955`) and a `pending_generation_change` flag, and drives
  them through `begin_block` (`:1494`), `copy_track_input` and `played_planes` (`:1572`). The graph
  sees it as `Box<dyn GraphPreparedSourceSetDriver>` (`crates/graph/src/lib.rs:2109`, `Send` only)
  inside `GraphPreparedSourceSet` (`:2165`), owned by `GraphExecutor` (`:2628`) in `source_set`.
- `prepare_graph_source_set` (`source/src/lib.rs:1609`) seals consumers into a set and computes its
  resource report.
- Hand-built graph plans with a source set already appear in the source crate's tests
  (`one_four_channel_source_fans_out_to_three_inputs_in_the_sequential_executor`, about `:2346`).
- Every host passes `plan_id: 1` (`crates/host-core/src/prepare.rs:1095`), so `plan_id` cannot tell
  two plans apart.
- *Hand the outgoing plan to its successor at the swap block* (#1270) adds
  `PreparedPlanExecutor::adopt_predecessor`, `as_any_mut` and `CarryOutcome`.

## Decisions frozen for this slice

- **D1. Vacant entries.** `GraphSourceEntry` holds `Option<PcmSourceConsumer>`. A new constructor,
  `SourceGraphSource::vacant(channel_count)`, makes an entry with no ring. A vacant entry renders
  `+0.0` on every claim and reports the block as an underrun in its observation validity. The set's
  resource report charges no PCM payload and no ring overhead for it and counts it separately.
- **D2. Graph identity.** Each bound `GraphExecutor` takes a process-unique, nonzero `u64` from a
  global atomic counter at bind (off the render thread). `graph::plan_identity(&mut
  PreparedRenderPlan) -> Option<u64>` reads it through the engine's `as_any_mut` (add a hidden
  `PreparedRenderPlan::executor_any_mut` accessor in `engine`).
- **D3. Carry program.** `graph::GraphCarryProgram { predecessor: u64, sources: Box<[(u32, u32)]> }`
  (successor source index, predecessor source index), installed before publication by
  `graph::install_carry_program(&mut PreparedRenderPlan, GraphCarryProgram) -> Result<(),
  GraphCarryInstallError>`. It is refused if an index is out of range or a successor index is not
  vacant. Stored in the `GraphExecutor`; its bytes are charged to the plan's report; a plan without
  one charges nothing.
- **D4. Render-side move.** `GraphExecutor::adopt_predecessor`:
  1. downcasts the predecessor (`as_any_mut`); another type returns `NotRequested`;
  2. compares the predecessor's identity with the program's; a mismatch returns
     `PredecessorMismatch` and moves nothing;
  3. for each pair, calls a new `GraphPreparedSourceSetDriver::adopt_sources(&mut self, predecessor:
     &mut dyn GraphPreparedSourceSetDriver, moves: &[(u32, u32)]) -> bool` (default `false`; add
     `as_any_mut` to that trait too). `SourceGraphSourceSetDriver` implements it with
     `core::mem::swap` of the two `Option<PcmSourceConsumer>`s, and ORs the predecessor's
     `pending_generation_change` into its own. The predecessor's entry becomes vacant;
  4. returns `Carried`.

  No allocation, no free, no lock, bounded by the number of moves.

## Deliverables

1. D1 in `crates/source/src/lib.rs`; D2-D4 in `crates/graph` and `crates/engine` (accessor only).
2. Tests in the source crate (gates 1-4).

## Authorized paths

- `crates/source/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/engine/src/realtime/plan.rs` (the accessor only)

## Non-goals

- No host-core preparation, no producers, no C ABI (slices 3-4).
- No DSP state other than sources.

## Objective gates

1. **Gap-free acceptance.** In the source crate's tests, build plan A with one source feeding two
   track inputs (the shape of the existing fan-out test) and plan B with the same claims but a vacant
   entry. Feed the ring a signal with no exact-zero sample. Render A for 4 blocks through
   `engine::realtime::plan_exchange`, install B's program (predecessor = A's identity), publish B,
   render 4 more. All 8 blocks are bit-identical to A rendered for 8 blocks. Block 5 reports
   `carry == Carried`.
2. **Vacant without carry.** B published without a program renders `+0.0` and flags the underrun;
   B with a wrong predecessor identity reports `PredecessorMismatch` and also renders `+0.0`.
3. **Generation change carried.** With two directly owned plans and the synchronous form
   (`adopt_predecessor_plan`; `prepare_source_seek` needs `&mut` access to the plan), a seek prepared
   on A's consumer just before the swap is reported as a generation change in B's first block.
4. **Install refusals.** Out-of-range and non-vacant indices refuse at install, off the render
   thread.
5. Commands:
   - `cargo test --locked -p source -p graph --features graph/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a consumer that is copied rather than moved, or a move that happens after the first
  render, turns it red.
- Gate 2: a mismatch that still moves (handing a ring to a plan it was not built for) turns it red.
- Gate 3: a carry that drops the pending generation flag makes observers trust a block across a
  seek; it turns red.

## Dependencies

- *Hand the outgoing plan to its successor at the swap block* (#1270).
