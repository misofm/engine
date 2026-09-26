# Resolve Output inputs in groups of eight and tighten the pair kernel

**Ruled** (coordinator, 2026-09-26): change 2 of
`docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md`, with the amendments of its adversarial
verification (`DIAGNOSIS-2-VERIFY.md`). Change 4 (a batched call on the public driver trait) is
dropped and is not part of this issue. Class A.

## Product outcome

The session Output op's fused reduction (`route_reduce`, `crates/graph/src/runtime.rs:714`) walks
its inputs a pair at a time. For every pair it resolves each input's planes through
`OutputSources::input` (two indirect calls per in-place claim, the planes returned through memory as
an `Option`), builds slice arrays, checks lengths with `.any()`, splits the host planes, and tests a
`?` on every chunk. That bookkeeping costs more than the arithmetic's own overhead. Resolve inputs
eight at a time into a stack table, check lengths once per group, and run each pair as a plain zip of
`chunks_exact` iterators. Measured in the shipped release profile on
`sixty_four_track_plumbing_ring`, on top of "Skip inert source-input units at render dispatch": -1,370
to -1,710 cycles per block. Class A: the per-frame summation chain and the unfused `mix` operands are
unchanged.

## Root evidence

- `runtime.rs:610` `OutputSources`; `:631` `OutputSources::input` (one table read and one branch per
  input, then the source set's `played_planes`).
- `runtime.rs:714` `route_reduce`, `:783` `route_pair<L, G>`, `:886` `route_run<L, G>`, `:841`
  `route_tail` (non-generic, `#[inline(never)]`), `:938` `mix_chunk`.
- `crates/graph/src/lib.rs:1834` `pub(crate) trait GraphSourcePlanes`; `:1926` its impl on
  `GraphPreparedSourceSet` range-checks the claim (`claim_index >= self.claims.len()` returns
  `None`) and holds each plane to the quantum before any reader sees it.
- Summation order today, per frame and per plane: the first pair stores `m0 + m1`; each later pair
  computes `(load + m_2k) + m_(2k+1)`; an odd last input computes `load + m`. `mix` is
  `lr.fma(r, ll.mul(l))` and `rr.fma(r, rl.mul(l))` with `Lane::fma` unfused (multiply, round, add,
  round). Inputs are in the Output op's edge order.
- The existing oracle `assert_route_reduce_is_the_route_ops_and_the_reduction` (`runtime.rs:14143`)
  sweeps frames {1, 3, 7, 8, 13, 16, 33, 64} x fan-in 2..=19 plus 64 with signed zeros against the
  route-ops-plus-reduction oracle; `assert_route_reduce_reads_lent_inputs_as_the_copy` (`:15892`)
  covers in-place inputs. Both passed on the verification's production-shape build of this change.
- A single stack table for all inputs was measured 1,500 cycles *worse* and would need a compiled
  maximum; do not build it.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs` (`OutputSources`, `route_reduce`, `route_pair`,
`route_run`, a new `route_group`, and their tests), `crates/graph/src/lib.rs` (the internal
`GraphSourcePlanes` trait only), `crates/graph/tests/MUTATIONS.md`, and this spec.

1. **Group resolution.** `route_reduce` walks the inputs in groups of `OUTPUT_GROUP = 8` (a batch
   size, not a track cap: any fan-in works). For each group it resolves every input's planes once into
   a stack `[(&[f32], &[f32]); OUTPUT_GROUP]` and checks every plane's length against the lease's
   frames once. In-place claims keep going through the set's range check. If you add a group method
   to the internal `GraphSourcePlanes` trait, give it a provided default body that loops over
   `played_planes` (test mocks implement the trait), and keep the per-claim range check. Do **not**
   add or change any method of the public `GraphPreparedSourceSetDriver`.
2. **The kernel.** A generic `#[inline(never)] fn route_group<L: Lane>` runs the group's pairs over
   the vector frames as a zip of `chunks_exact(L::WIDTH)` iterators, with no per-chunk `?` and a plain
   store loop for the block's first pair. The odd last input stays a `G = 1` step. The `f32` tail
   frames stay in the non-generic `route_tail`, called per pair after the group's vector frames.
3. **Counters.** Keep every `test_only_count_source_plane` call with its current meaning. The
   verification's prototype dropped them and broke four gates.

## Non-goals

No change to the summation order, to `mix_chunk`, to which inputs are read in place (#927), to
dispatch (the inert-unit issue), or to the public driver trait. No frame-tiled register-accumulator
kernel (measured slower in the engine) and no software prefetch (reopened only after a live-producer
row exists).

## Objective gates

1. **Oracle.** `assert_route_reduce_is_the_route_ops_and_the_reduction` and
   `assert_route_reduce_reads_lent_inputs_as_the_copy` pass unchanged at both lane widths. Add fan-in
   257 (odd, above any power of two the group size divides) to the first test's sweep to prove there
   is no track cap.
2. **Counters.** `[0, claims * BLOCKS, 0]` at `tools/console-workload/src/lib.rs:2036`, the graph
   source-plane counter tests near `runtime.rs:15874` and `:16053`, and host-core
   `tests/source_in_place.rs` pass unchanged.
3. **Digests.** Every console workload's 64-block digest is unchanged.
4. **Wasm rule 3.** On the batch's rebuilt AudioWorklet artifact,
   `scripts/check-web-audioworklet-callgraph.py` rule 3 passes: every `4wide6f32x4` function has more
   vector than scalar arithmetic, and `route_group<f32x4>` is visible under that pattern.
5. **Allocation and policy.** `crates/graph/tests/rt10_source_in_place_alloc.rs`,
   `scripts/check-realtime-policy.sh`, `check-graph-determinism.sh`, `check-graph-policy.sh`.
6. **Red mutations** recorded in `crates/graph/tests/MUTATIONS.md`: reassociate a later pair as
   `load + (m_2k + m_(2k+1))` (gate 1 fails); drop the counter call (gate 2 fails); resolve a group
   of 8 but reduce only its first 7 (gate 1 fails at fan-in 8).
7. `cargo test -p graph` with and without `test-support`, `-p console-workload`, `-p host-core`;
   fmt, clippy with `-D warnings`, and `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`.

## Console benchmark rows

Can move: `sixty_four_track_plumbing_ring` (most) and `sixty_four_track_plumbing_only` (Output phase
only, about -270 cycles). No digest may move.

## Dependencies

None in code. It composes with "Skip inert source-input units at render dispatch"; implement after it
so the benchmark attributes each saving separately.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit. Every "unchanged" gate is a hard stop.
- Render paths stay allocation-free, lock-free and syscall-free. `crates/graph` stays free of
  `unsafe`. Public docs must not link to private items.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not quote a projected saving.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.

## What the implementer will hit

- A stack array of slice pairs needs an initial value; `[(&[][..], &[][..]); 8]` costs nothing, but a
  4 KiB table does (the rejected shape). Keep the table at eight entries.
- `route_pair` is `#[inline(always)]` today; the new `route_group` must be `#[inline(never)]` so the
  wasm symbol stays visible to rule 3, and the scalar tail must stay out of it or rule 3 fails exactly
  as #920's attempt 1 did.
