# Remove the builtins-less render path

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

Safety confirmed empirically: among with-builtins plans only host-core's two forced-`Scalar` tests
reach the fold (fan-in 9 and 3); no `Simd4` or `Simd8` plan reaches the fold, #927's read or #925's
alias; with the fold disabled `host-core --all-features` stays green. Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md`.
1. **ObservedAlias port.** #918's `SourceShape` (`runtime.rs:13863-13886`) has no unbanked alias;
   a `PostSimd1` after `PostInputBuiltins` aliases the bank's buffer and the shape degenerates to
   `Plain`. Add a `SourceShape` field that puts a `PostSimd1` alias directly between the claimed
   Input and its first consumer (as the `routed` track does), and port ObservedAlias onto it.
2. **Record the ported gates' digests on `64b155d0`** (before #936, with #918's fixture), not on
   this issue's base, which already has #936 and would make "moves no bit" circular.
3. **Port or explicitly retire #927's `DeadClaim`**, the only test of clause (b)'s zero-reader arm
   with a recoloured slot, which stays live.
4. **Gate 9:** `graph_fixture --check` already fails on the batch head (#947 and #963); use
   `graph_fixture --manifest` equality before and after instead.
5. **Gate 7:** padding the executor by 32 bytes moved no native pin or fixture byte, so expect no
   native re-pin; run `scripts/check-browser-expected-resources.py` for the wasm side and re-pin
   only what moves, with the reason.

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Product outcome

No host compiles a plan without builtins: the browser boot, the C ABI and the native host all reach
`GraphCompiler::compile_with_builtins` (`crates/host-core/src/prepare.rs:1195`) at the build's
vector width. Yet `crates/graph` carries about 900 lines of render and bind code that only a plan
with **no bank at all** can reach, and on a vector backend every with-builtins track is a bank
member:

- the fused Output route reduction (#926), its kernels (#926/#937) and its in-place source reads
  (#927, `played_planes_group`);
- the alias lowering of unlisted builtin stages (#925).

The shipped AudioWorklet module carries two of those kernels, and they can never run there. Two
benchmark rows measure this path, one of them in the wasm arm. Delete the code, retire the rows,
and point the benchmark at the real no-effects session.

## Root evidence

- Why the fold is bankless-only: `output_route_fold` declines any plan with a bank member
  (`crates/graph/src/runtime.rs:7367`; the membership includes builtin banks, `:4439-4461`).
- Why every with-builtins track banks: on a vector backend every builtin stage is a bank member,
  and short banks are padded (`crates/builtins-compiler/src/lib.rs:1290-1294, 1342`;
  `crates/rack-compiler/src/lib.rs:243-245`).
- Why the in-place Output read needs a builtins-less plan: it needs an Input whose only reader is a
  retired route (`runtime.rs:6280-6284`). With builtins, the reader is `PostInputBuiltins`.
- Why the #925 arm needs a builtins-less plan: it needs a builtin stage outside
  `required_bindings` (`crates/graph/src/program.rs:662`). `compile_with_builtins` lists all
  three stages (`crates/graph-compiler/src/compile.rs:816-832`).
- Shipped builds are `Simd8` natively and `Simd4` in the browser, which ships `simd128` only
  (`crates/lane/src/backend.rs:31-70`; `scripts/build-web-audioworklet.sh:26-30`).
  `Backend::Scalar` is reached only by tests and an unshipped CI compile.

## Smallest closable slice (this issue): delete the Output route fold family

Authorized paths:
- `crates/graph/src/runtime.rs` and `crates/graph/src/lib.rs`;
- `crates/graph/tests/rt10_source_in_place_alloc.rs` and `crates/graph/tests/MUTATIONS.md`;
- this spec.

1. **The fold.** Delete `output_route_fold` and `OutputRouteFold` (`runtime.rs:7287-7439`).
2. **The kernels.** Delete `OutputSources`, `OUTPUT_GROUP`, `route_reduce`, `route_group`,
   `route_pair_vectors`, `route_lone_vectors`, `route_tail`, `route_run`, `mix_chunk` and
   `add_mixed_chunks` (`:622-1087`).
3. **The runtime state.** Delete:
   - the `output_routes` and `output_sources` fields and their two layout mirrors;
   - the constructor parameter and its assert;
   - the `execute` routing;
   - `execute_op`'s `routes` and `sources` parameters and its third host arm, leaving two arms
     (`:3676-3684`);
   - the Output-fold arms of `validate_fold_installation` and `build_sequential`;
   - the seam and the `output_route_folds` accessor.
4. **`source_plane_table`.** Delete clauses (b′) and (e), the `output_producers` parameter and
   `SourcePlanes.output`. Rewrite the doc: a claim is bound in place only by a bank gather, or when nothing reads it
   (clause (b), `runtime.rs:6159`).
5. **`lib.rs`.** Delete `GraphSourcePlanes::played_planes_group` and its realtime region
   (`:1863-1899`), the seam export (`:36`) and the accessor (`:2572`).
6. **Port #936's four graph gates** (`runtime.rs:16433`, `:16452`, `:16484`, `:16515`) onto #918's
   banked source-fed fixture, keeping all four shapes:
   - `Plain`: an inert claim.
   - `ObservedInput`.
   - An observed elided alias, now at a rack boundary (`PostSimd1`) instead of `PostFader`.
   - `TrackDelayed`.
   Record the pre-change digests on the base commit, as `INERT_PRE_CHANGE` does.
7. **Delete the other dedicated tests** (listed in section 3.1 of the study) and the bankless arm
   of `rt10`. Mark the #926, #927 and #937 rows in `MUTATIONS.md` retired.

## Non-goals

- No change to the bank-chain route fold (#218/#915/#945), to #918's in-place bank gathers, to
  #936's skip, to #916's host planes, or to any with-builtins lowering.
- #925's alias arm is #958. The compile entry is #959.

## Objective gates

1. **Digests.** Every console workload's 64-block digest and unit census is unchanged. After the
   companion issue, every row has builtins.
2. **Host-core.** `cargo test -p host-core --all-features` is green, including the two forced-scalar
   tests. They took the fold. Now they run the route ops and the reduction, which #926 proved
   bit-identical.
3. **#936.** Its four ported gates are green on the banked fixture, with digests equal to base.
4. **#918.** `rt10`'s #918 arm is green, with zero allocations over 1,000 blocks.
5. **Nothing left behind.**
   `rg -n 'output_route|route_reduce|route_group|route_tail|OutputSources|played_planes_group|output_sources' crates/`
   finds nothing outside history text.
6. **AudioWorklet artifact.** The build script's cargo line and `scripts/check-web-audioworklet.sh`
   exit 0. The kernel census is the previous one minus one (at least 11), and the render closure is
   unchanged. The pin is repinned at the batch boundary.
7. **Resources.** The `capi` `resource_lifecycle` test and the host-core resource tests pass.
   `size_of::<GraphExecutor>()` shrinks by 32 bytes. Totals charge only layout deltas, which do
   not move, but `runtime_owner_allocation_bytes` does, and so may a `largest_allocation_bytes`
   it dominates (`crates/graph/src/lib.rs:484-536`). Re-pin only what moves, and state why.
8. **Red mutations** in `MUTATIONS.md`. Each must turn a ported #936 gate red:
   - dispatch every `SourceInput` unit;
   - skip an observed Input unit;
   - skip a `TrackDelay` unit.
9. fmt; clippy with `-D warnings`; `cargo test -p graph` with and without `test-support`;
   `-p graph-compiler`, `-p console-workload`, `-p host-core --all-features`, `-p capi`,
   `-p source --all-features`; `scripts/check-graph-determinism.sh` and #947's fixture test (the slice must not move
   `fixtures/graph/v1/*`),
   `check-graph-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh`.

## Console benchmark rows

No row may move a bit. After the companion issue no row reaches the deleted code, so no row may
move a unit either.

## Dependencies

- #956 lands first.
- Land after the plumbing-floor-2 batch (#936 and #937 merged), so the four #936 gates exist to be
  ported.

## Standing rules for the implementer

- Work only from this body. Class A: every "unchanged" gate is a hard stop.
- The render path stays allocation-, lock- and syscall-free, and `crates/graph` stays free of
  `unsafe`.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not run timed benchmarks.

