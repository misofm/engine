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

None yet.
