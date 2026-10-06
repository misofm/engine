# Remove the libc memset calls from the true-peak limiter's reset on Apple targets

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-06 by root order (Amendment 2 of *Let the builtins splat their chain constants
without iOS memset calls*, #1451: "ensure an issue owns the render-reachable remainder of #1018").
Code anchors verified on `codex/d15-stream-g` at `44585c80f`. Ordered after #1452.

## Product outcome

No reset or failed-block recovery of the true-peak limiter makes a libc call on an iPhone. After
#1451, three `bl _memset_pattern16` calls remain in the limiter's `aarch64-apple-ios` release
assembly on a path the render thread reaches: `ChannelState::clear_runtime`, which runs at a reset
and on a failed block. The realtime rules forbid any libc call in render. After this slice that
path makes none, with no rendered bit moved.

## Context

- **#1451 D1/D2 (root cause, fixed for splats).** `wide`'s `splat` reached LLVM as an array-repeat
  store loop, and LLVM's `LoopIdiomRecognizePass` turns a loop that stores one constant `f32`
  pattern into `llvm.experimental.memset.pattern`, which only Darwin lowers to
  `bl _memset_pattern16`. #1451 builds splats from an array literal; the iOS count fell from 2,122
  to 16 calls. The 16 left are scalar fills of a real length, which loop-idiom turns into the same
  call: `builtins` 5 (preparation constructors), `host-core` 4 (spectrum arrays), `soft-clip` 1 (a
  test-corpus fill), `true-peak-limiter` 6.
- **The limiter's six.** Three `fill(1.0)` in `ChannelState::new` (preparation, off the render
  thread) and three in `ChannelState::clear_runtime`
  (`crates/true-peak-limiter/src/lib.rs:634-646`): `self.required_ring.fill(1.0)`,
  `self.box_ring.fill(1.0)` and `self.prefix.fill(1.0)`. `clear_runtime` is called from
  `reset_to_defaults` (`:579-584`, `FullToDefaults`) and `reset_keeping_parameters` (`:616-622`,
  `DiscontinuityKeepParameters`), reached from the effect's `reset` (`:3513`) and from the D7
  recovery of a failed block when every active lane failed (`reset_failed_lanes`, `:3679`, called from the block path at
  `:3636-3637`), inside `process`. Its doc (`:630-633`) records that it is out of line (#1091) to keep the count
  where it was. The per-lane form `clear_lane_runtime` (`:659`) writes the same words at a
  lane's stride and makes no call.
- **The ratchet.** `scripts/check-cross-targets.sh` counts `^\tbl\t_memset_pattern16$` per crate in
  the `aarch64-apple-ios` release assembly (`cargo rustc --release --target aarch64-apple-ios
  --crate-type rlib --emit asm`; runs locally, no Xcode); `scripts/lib/aarch64-known-defects.py`
  `IOS_MEMSET_CEILINGS` holds `true-peak-limiter` at 6, owned by #1018. A count below the row asks
  for it to be lowered.
- **#1452** (Stream G, after #1451) undo 4 judges `clear_runtime`'s `#[inline(never)]`; it expects
  the fills may still make calls and then keeps the attribute. It does not remove the calls. This
  slice lands after it and starts from its shape.
- **#1018** stays open for the other rows (preparation, observation and test code).

## Decisions frozen for this slice

- **D0. Root decision (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled in #1451 Amendment 2 that
  #1018 stays open with its 16 remaining calls and that an issue must own the render-reachable
  remainder; none did, so this issue is filed for it.
- **D1. Evidence first.** Confirm with `-print-changed` (or the IR before and after
  optimisation) that each of the three calls is loop-idiom's rewrite of the `fill(1.0)` loop, and
  record the assembly around each call.
- **D2. One shape for all targets.** Write the three fills so that LLVM does not form a
  memset-pattern idiom: for example, fill by `lane` vector stores of a splat (`Lane::splat(1.0)`
  stored per chunk, with a scalar tail), or derive the ring's rest words from one already-written
  word. No `cfg(target_os)`, no Apple-only path, no disabling of an LLVM pass, no `#[inline(never)]`
  wrapper added for this purpose. If D1 shows that every reasonable shape is still recognised,
  stop and return the evidence to root.
- **D3. Class A.** The reset writes the same words; no rendered bit moves.
- **D4. The ceiling only falls.** Lower the `true-peak-limiter` row as the check asks (expected 6
  -> 3, the constructor's three remain #1018's unless the same shape removes them, in which case
  the row falls further).

## Deliverables

1. D1's evidence in this spec's attempt record.
2. The D2 change with a doc stating why the fills are written that way (the Darwin libc call).
3. `scripts/lib/aarch64-known-defects.py`'s `true-peak-limiter` row lowered (or deleted at zero).
4. The PR evidence: the differential (gate 1) and the counts before and after (gate 2).

## Authorized paths

- `crates/true-peak-limiter/src/lib.rs` (`ChannelState::clear_runtime` and its doc; `ChannelState::new`
  only if the same shape applies)
- `scripts/lib/aarch64-known-defects.py` (the `true-peak-limiter` row and the comment above it)
- this spec

## Non-goals

- The other #1018 rows (`builtins`, `host-core`, `soft-clip`) and closing #1018.
- The `#[inline(never)]` decision (#1452 undo 4).
- Any change to the limiter's reset semantics, state, latency or tail.

## Hazards

- #1452 edits the same function's attribute and doc; this slice lands after it.
- `crates/true-peak-limiter/src/lib.rs` is edited by stream A's carry slices and by #1409/#1411
  (STREAMS.md hot-file rows); keep to `clear_runtime`.
- A vector fill must handle ring lengths that are not a multiple of the lane width.
- Another implementer may be working in the same worktree; commit exact paths only.

## Objective gates

1. **No rendered bit moves.** A base-versus-head differential through the limiter at scalar,
   `Simd4` and `Simd8`: both reset kinds, a failed block of every active lane and of some lanes,
   at 44.1 and 96 kHz, quanta 1 and 128: every output and every state word identical. PR evidence.
   The limiter's corpus digests, `conformance_fixtures --check` and the crate's tests pass
   (`a_lane_reset_is_the_whole_reset_at_one_lanes_stride` holds the per-lane list equal).
2. **iOS count.** `bash scripts/check-cross-targets.sh` passes with `true-peak-limiter`'s row
   lowered, and the iOS assembly of `clear_runtime` (and every function it is inlined into) has no
   `bl _memset_pattern16`.
3. **Workspace gates.**
   - `cargo test --locked --all-targets -p true-peak-limiter -p conformance`
   - the `test-debug-a` workspace command from `.github/workflows/qualification.yml`
   - `cargo run --locked -p conformance --example conformance_fixtures -- --check`
   - `bash scripts/run-wasm-gates.sh`
   - `bash scripts/check-workspace-policy.sh`, `python3 -B scripts/lib/aarch64-known-defects.py --self-test`
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

This slice adds no new test. The iOS ratchet turns red if the call comes back (a count above the
lowered row); the per-lane equality test and the differential turn red if a reset word moves.

## Dependencies

#1451, #1452.

## Attempt record

### Attempt 1 (2026-10-06, implementer)

**D1 evidence (Rust 1.97.1, `aarch64-apple-ios` release, base `b5cfd2b60`).** The crate's iOS
assembly has 6 `bl _memset_pattern16`: three in `ChannelState::clear_runtime` and three in
`ChannelState::new`. `-C llvm-args=-print-changed=quiet -filter-print-funcs=<both>` shows each one
appear first in an `IR Dump After LoopIdiomRecognizePass` of that function, as
`call void @llvm.experimental.memset.pattern.p0.f32.i64(ptr, float 1.000000e+00, i64 n, i1 false)`:
in `clear_runtime` the loops of `required_ring.fill(1.0)`, `box_ring.fill(1.0)` and
`prefix.fill(1.0)` (the zero fills become `llvm.memset`, lowered to `_bzero`); in `new` the
`extend_with` store loops of `vec![1.0; n]` (`.loc` `core/src/ptr/mod.rs:1933`). The assembly at each
`clear_runtime` site is `ldr x0, [x19, #off]; lsl x2, x8, #2; adrp/add x1, l_.memset_pattern;
bl _memset_pattern16` (`.loc` `slice/specialize.rs:25`).

**Shapes tried.** (a) `Simd4::splat(1.0)` stored per 4-word chunk plus a scalar tail: *worse*,
6 calls in `clear_runtime` (the 16-byte vector constant is itself a memset-pattern idiom, and the
tail loop is another). (b) The words written lane by lane at the run-time lane stride, in
`clear_lane_runtime`'s order (one loop over lanes, two `step_by(width)` ring loops and the lane's
`prefix` word): 0 calls in `clear_runtime`; the pass cannot prove a run-time stride contiguous.
Chosen (b): one shape on every target, no `cfg`, no pass switch, no new attribute.
`ChannelState::new` now allocates the three `1.0` planes zeroed (`calloc`, no store loop): the
values `vec![1.0; n]` wrote were dead, because `new` calls `reset_to_defaults`, which writes them
through `clear_runtime`. So the same shape removes the constructor's three too, and the row was
deleted at zero rather than lowered to 3.

**Cost (open item, not measured).** The `1.0` fills are now scalar strided stores (iOS: one
`str w` per word; LLVM vectorises neither target's loop), where the base used libc's vectorised
`memset_pattern16` on Apple and broadcast stores elsewhere. Worst case is 96 kHz at bank width 8:
about 31k word stores per reset across both channels, at a reset or a failed block only, never in
the frame loop. The four `_bzero` calls for the zero fills remain (outside this spec's D2 and
not counted by the ratchet).

**Gate 1 (PR evidence, not committed).** A temporary in-crate harness drove `LimiterCore` at `f32`,
`Simd4` and `Simd8`, 44.1 and 96 kHz, quanta 1 and 128, `DualMono` and `Maximum`; each case runs
five phases of `3 + 1100 / quantum` noise blocks with automation: none, `FullToDefaults`,
`DiscontinuityKeepParameters`, a NaN input on every lane (whole D7 reset) and a NaN input on lane 0
(per-lane D7 at widths 4 and 8; whole at width 1). The harness wrote every output word and every
state word of both channels (each plane by bits, ramps, lane shapes, cursors, report, #990 record)
after construction, every block and every reset. 120 phase reports, the same on both builds (every
failure phase reported its failure). Base (`b5cfd2b60` source) and head output: 5,081,610,702 bytes
each, `cmp` identical (sha256 `190075ad...46cc9`). The harness discriminates: a mutant that writes
`0.5` into the last lane of `box_ring` differs at byte 3607 (the construction dump).

**Gate 2.** iOS count for `true-peak-limiter` is 6 -> 0; `bash scripts/check-cross-targets.sh` PASS
with the row deleted (builtins 5, host-core 4, soft-clip 1 remain #1018's). `clear_runtime` is
`#[inline(never)]` and has no `bl _memset_pattern16`; no function in the crate has one.

**Ratchet mutation (test value).** Putting back `self.box_ring.fill(1.0)` in place of the strided
`box_ring` loop: limiter count 1, `judge-memset` red ("1 memset_pattern16 calls and no row: a new
libc call in the iOS build (#1018)", rc 1). Reverted: count 0, judge green.

**Gate 3.** All PASS: `cargo test --locked --all-targets -p true-peak-limiter -p conformance`
(89 passed, including `a_lane_reset_is_the_whole_reset_at_one_lanes_stride`), the `test-debug-a`
command (1458 passed), `conformance_fixtures -- --check`, `bash scripts/run-wasm-gates.sh`,
`check-workspace-policy.sh`, `check-realtime-policy.sh`, `aarch64-known-defects.py --self-test`,
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`, `cargo clippy --locked
--workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.

**Open items.** (1) `scripts/check-cross-targets.sh`'s comment above the scan still lists the
limiter's calls among "the 16 left"; that file is outside this spec's authorized paths. (2) The
reset cost of the scalar strided fill above has no benchmark.
