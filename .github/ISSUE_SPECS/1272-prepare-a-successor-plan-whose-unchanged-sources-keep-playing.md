# Prepare a successor plan whose unchanged sources keep playing

Slice 3 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`). This slice builds the host-core carry scaffold (inventory,
successor entries, the join) that slices 7-15 extend, with sources as its first family.

## Product outcome

Host-core can prepare a **successor** of a running plan. Every source that the transaction did not
change keeps its ring: the successor allocates no ring for it, the carry of *Move a source consumer
into a successor graph plan* (#1271) hands it over at the swap block, and a producer hand-over moves
the host's producer. The successor renders the next block as if no swap had happened. Hosts call
this: the C ABI in slice 4, the browser in B2.

## Context

- `prepare_host_runtime_with_live_controls_policy_and_spectrum` (`crates/host-core/src/prepare.rs:797`)
  allocates one ring per session source (`PcmSourceRing::prepare_host_region`, `:850`; generation 1,
  frame 0, region `0..source.frames`), pushes the producer into a `ControlSourceBuilder` and the
  consumer into a `SourceGraphSource`, seals them (`prepare_graph_source_set`, `:1183`), checks the
  source caps (`maximum_source_total_bytes`, `maximum_source_overhead_bytes`, `:1185-1190`), binds
  (`into_bound_with_source_set`, `:1260`) and returns `PreparedHost { plan, sources, report, .. }`
  (`:453`).
- `SourceControlSet` (`crates/host-core/src/source.rs:118`) holds one `ControlSource` (`:16`) per
  source with its `HostChunkProvider`; `submit` (`:143`) and `seek` (`:188`) look up by ID.
- The width seam is `#[cfg(test)] pub(crate) prepare_host_runtime_with_live_controls_backend`
  (`prepare.rs:657-672`); `crates/host-core/tests/` cannot reach it (`lib.rs:113-116`).
- Allocation gates in host-core tests use `bench_support::alloc` thread counters and the engine
  render audit after a warm-up (`assert_renders_without_allocating`,
  `crates/host-core/tests/route_mute.rs:1111-1160`).
- `fixtures/session/v1/parametric-eq-nine-track.json` enables a 20 Hz high-pass and a 20 kHz
  low-pass filter on every track (`hpf_hz`/`lpf_hz` of `0` disables them).

## Decisions frozen for this slice

- **D1. Inventory.** A control-side `host_core::PlanStateInventory` (Send, plain data) is built at
  the end of preparation and returned as a new public field `PreparedHost::inventory`. It holds the
  plan's graph identity (`graph::plan_identity`) and, for this slice, one row per source: source ID,
  its `PcmSourceRingConfig` and its index in the graph source set. Later slices add one row list per
  state family. It is never read on the render thread.
- **D2. Successor entries.** A `SuccessorBase<'a> { inventory: &'a PlanStateInventory, committed:
  &'a session::SessionModel }`, where `committed` is the predecessor's committed model at the moment
  the successor is prepared (umbrella P1.4). The private policy function takes
  `Option<SuccessorBase>`. Public entries: `prepare_host_runtime_successor(compiled, caps, base)` and
  `prepare_host_runtime_with_live_controls_successor(compiled, caps, live_controls, base)`. Under
  `test-support`, one entry of each kind (fresh and successor) that also takes a `Backend`, so width
  tests can live in `crates/host-core/tests/`. With no successor base every existing entry behaves as
  today.
- **D3. Rule for sources.** A source carries when its ID is in both, its declaration
  (`session::Source`: content, channels, bit depth, frames) is equal in `committed` and in the new
  model, and its `PcmSourceRingConfig` equals the inventory row. Track-to-source mappings do not
  matter.
- **D4. Vacant preparation.** A carried source is prepared as `SourceGraphSource::vacant` (slice 2),
  and its `ControlSource` holds no provider. The join installs the program
  (`graph::install_carry_program`) before the plan leaves preparation. The single-plan source caps
  count the rings this plan allocates **plus** the rings it will carry (both appear in the report as
  separate rows), so the active plan never exceeds `maximum_source_total_bytes` after the swap.
- **D5. Producer hand-over.** `SourceControlSet::adopt_persisting(&mut self, predecessor: &mut
  SourceControlSet) -> usize` moves the provider of every vacant entry from the predecessor's entry
  with the same ID. Infallible and allocation-free; a missing ID leaves the entry vacant. A submit or
  seek on a vacant entry returns a new `SourceControlError::Vacated` (`"source.ring.vacated"`, not
  backpressure, not internal). A host calls it after its last fallible step (slice 4).
- **D6. Mismatch.** If the carry later reports `PredecessorMismatch` (unreachable on both hosts by
  construction, umbrella P11), the vacant sources render `+0.0`, since their producers have moved.
  Document it on `adopt_persisting`.

## Deliverables

1. D1-D6 in `crates/host-core/src/prepare.rs`, `source.rs`, `lib.rs`.
2. The integration test file `crates/host-core/tests/successor_swap.rs` with a support helper that
   renders a swapped run and a reference run and compares them block by block; slices 7-15 add their
   cases to it.

## Authorized paths

- `crates/host-core/src/prepare.rs`, `source.rs`, `lib.rs`
- `crates/host-core/src/render_session.rs` (its exhaustive `PreparedHost` destructure, `:180-195`)
- `crates/host-core/tests/successor_swap.rs`, `crates/host-core/tests/support/`
- `crates/capi/tests/resource_lifecycle.rs` (its `PreparedHost` destructure, `:604`, and the
  inventory's charge in the oracle)
- `crates/builtins-compiler/src/lib.rs` (only if the bind wrapper must forward the identity)

## Non-goals

- No DSP state carry (slices 7-14): the test sessions hold no stateful DSP on unchanged paths.
- No C ABI or browser change; no anchored seek.

## Objective gates

1. **Gap-free acceptance, both widths.** Session A: tracks `eq0` and `eq1` of
   `parametric-eq-nine-track.json` on `fixture-source`, both console sections empty, no inserts,
   `hpf_hz` and `lpf_hz` set to 0, default trim, fader and pan. Session B: A plus a muted track whose
   ID sorts first, on the same source.
   - Swapped run: prepare A; render 6 blocks through `engine::realtime::plan_exchange`, feeding a
     signal with no exact-zero sample through `PreparedHost::sources` one block ahead; prepare B with
     `SuccessorBase { &a.inventory, a_model }`; `b.sources.adopt_persisting(&mut a.sources)`; reserve
     and commit B; render 6 more, feeding through `b.sources`.
   - Reference: B prepared fresh, fed the same PCM from frame 0.
   - All 12 blocks bit-identical at `Backend::Simd8` and `Backend::Simd4`; block 7 reports `Carried`.
   - The same run with B prepared fresh (no successor base) differs at block 7: the oracle can fail.
2. **Removed source.** A has two sources and one track on each; B removes the second track, its
   route and its source. Every block equals a fresh B fed the first source; a submit for the removed
   source through `b.sources` returns `UnknownSource`.
3. **Changed source.** B changes the source's `frames`: no move pair; B allocates and charges a new
   ring.
4. **Caps.** A successor whose allocated plus carried rings exceed `maximum_source_total_bytes`
   refuses with `host.source.resource.limit`.
5. **Realtime.** The swap block (carry included) makes zero allocations and frees
   (`bench_support::alloc::current_thread_delta_since` and the engine render audit, after warm-up).
6. **No regression.** With no successor base, every existing `host-core`, `host-web` and `capi` test
   passes unchanged, and `cargo test --locked -p console-workload` keeps its digests.
7. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support,graph/test-support`
   - `cargo test --locked -p host-web --features host-web/test-support` and `cargo test --locked -p capi`
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - the umbrella's inherited gates.

## Test value

- Gate 1: a successor that still allocates and plays a fresh ring, or a hand-over keyed by index,
  turns it red.
- Gate 3: a rule that carries a source whose declaration changed turns it red.
- Gate 4: a cap that counts only allocated rings lets the active plan exceed its cap after the swap;
  it turns red.

## Dependencies

- *Move a source consumer into a successor graph plan* (#1271).
