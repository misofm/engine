# Read fold tiles without a stack copy

**Ruled** (coordinator, 2026-09-26): successor S-A of #944 (gain/pan research, base `main` at `14f2917b`).
Class A. First in line under the owner's copy rule: a copy that is not strictly needed must not
exist.

## Product outcome

Every folded bank reaches the master through `ArenaMembers::fold_resident_tiles`
(`crates/graph/src/runtime.rs:2105`). For each `W x W` tile of the resident AoSoA block it builds the
tile's frame rows with `tile_rows` (`runtime.rs:2174`), which fills a zeroed `[[f32; W]; W]` stack
array with a runtime-length `copy_from_slice` per row. LLVM lowers that to a `memcpy` call per tile
and plane. The research measured the fold at 1,754 cycles per 8-lane bank in the real plan, and a
prototype that forms the rows from fixed-size chunks instead measured 963, with every console
workload's digest unchanged: about -1.7 to -2.0 us per block on `sixty_four_track_gain_pan_only`. It
moves every banked row, since every folded bank takes this path.

## Amendments (adversarial verification, 2026-09-26; these override any conflicting text below)

The verification reproduced the change: `fold_resident` goes from 4 `memcpy` calls to 0 on x86, its
2 `memory.copy` go on wasm, and the gain/pan row moves -1.2 to -2.0 us in process with every digest
unchanged. Evidence: `docs/handoffs/gain-pan-2026-09-26/GAIN-PAN-VERIFY.md`.

1. **Gates 1 and 6 need witnesses that can see the fold.** `BASE_DIGEST` is the plumbing row, which
   has no bank. The red witnesses are the graph test
   `a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit` and
   `tools/console-workload/tests/chain_shape.rs`'s `the_folded_master_is_the_reductions_own_bits`;
   both go red under the offset mutation, including a `W = 4`-only variant, which gate 6 also
   records. Attach before-and-after output of every console workload's 64-block digest (the
   `digests` harness in `docs/handoffs/gain-pan-2026-09-26/gain-pan-diagnosis-harnesses.patch`,
   applied in a scratch copy, not committed).
2. **Gate 4 (wasm).** Rule 3 never applies (`fold_resident` does not match `4wide6f32x[48]`), and
   `check-web-audioworklet.sh` stops at the artifact pin before a repin. Instead: build the
   AudioWorklet wasm with the build script's own cargo line, pipe `wasm-objdump -d` into the two
   callgraph checks, and record that the kernel count stays 15, the render closure is unchanged, and
   the fold's `memory.copy` calls are gone. The artifact pin is repinned once at the batch boundary.

## Root evidence

- `runtime.rs:2140-2155`: the tile loop, `transpose(tile_rows(block_left))` and the same for the
  right plane, then `fold_words`.
- `runtime.rs:2173-2180` `tile_rows`: `let mut rows = [[0.0; W]; W]` then `row.copy_from_slice(chunk)`
  per row over `block.chunks_exact(W)`.
- `runtime.rs:2074-2076`: instantiated at `W = 4` with `lane::Simd4` and `W = 8` with `lane::Simd8`.
- The transposes are `transpose_tile_4` / `transpose_tile_8` (imported at `runtime.rs:251`); they take
  the rows by value.

## Smallest closable slice

Authorized paths: `crates/graph/src/runtime.rs` (`tile_rows` and, only if needed, its one call site),
its tests, `crates/graph/tests/MUTATIONS.md`, and this spec.

1. Form each tile's rows from fixed-size rows, not runtime-length slices: take
   `block.as_chunks::<W>()` (stable since Rust 1.88; the remainder is empty because the caller hands
   exactly `W * W` words) and build `[[f32; W]; W]` from its first `W` rows by value (for example
   `core::array::from_fn(|i| rows[i])`, or a `try_from` of the `&[[f32; W]]` prefix). The point is
   that every length is a compile-time constant, so no `memcpy` call and no zero-fill remain.
2. Nothing else changes: the transpose, `fold_words`, the ragged tail, the premises and the
   association order stay as they are.
3. No `unsafe`, no `wide`, no intrinsics in `crates/graph`.

## Non-goals

No change to rack's `tile_gather` / `tile_scatter` (the research saw a gap there but did not
investigate it; that is its own issue), to the fold's arithmetic or order, or to which banks fold.

## Objective gates

1. **Digests.** Every console workload's 64-block digest is unchanged, including `BASE_DIGEST` in
   `tools/console-workload/tests/chain_shape.rs` and the gain/pan, dispatch, builtins and console rows.
2. **Standing fold tests.** Every existing `crates/graph` test that exercises `fold_resident` (the
   probes near `runtime.rs:9327`, `:9806`, `:9990` and their callers) passes unchanged, with and
   without `test-support`.
3. **No copy call.** Record in the evidence the release `objdump -d` of the `W = 8` instantiation
   (inside the `bench` binary or a release test binary) showing no `call` to `memcpy` or `memset` in
   `fold_resident_tiles`' tile loop, before and after.
4. **Wasm rule 3.** On the batch's rebuilt AudioWorklet artifact, `scripts/check-web-audioworklet-callgraph.py`
   passes: the `W = 4` instantiation keeps more vector than scalar arithmetic.
5. **Allocation and policy.** `scripts/check-realtime-policy.sh`, `check-graph-determinism.sh`,
   `check-graph-policy.sh`, and the graph allocation tests.
6. **Red mutation** recorded in `crates/graph/tests/MUTATIONS.md`: build the rows from the wrong
   offset (`rows[i + 1]` wrapped) and gate 1 goes red.
7. fmt, clippy with `-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`, `cargo test -p graph -p console-workload`.

## Console benchmark rows

Can move: every banked row. No digest may move.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every "unchanged" gate is a hard stop.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not run the timed runner; do not quote
  a projected saving.
- The AudioWorklet artifact pin and browser qualification are repinned once at the batch boundary.
