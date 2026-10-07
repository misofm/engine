# Measure the iOS memset_pattern16 ratchet on the shipped post-LTO capi staticlib

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Filed 2026-10-07 by root order (NIT 4 of the verdict on *Remove the libc memset calls from the
true-peak limiter's reset on Apple targets*, #1456, attempt 2). Ordered after #1456. Verify every
code anchor on the branch before starting.

## Product outcome

The iOS `memset_pattern16` ratchet (known defect #1018, `ios-asm-memset-pattern16`) counts the calls
in the library an iPhone app actually links: the release `capi` staticlib after the release
profile's fat LTO. A call that LTO adds or keeps in the shipped code is caught, and one that LTO
removes is not counted against a crate. Each count is still attributed to a crate where the
assembly allows it.

## Context

- **What the ratchet reads today.** `scripts/check-cross-targets.sh` (the `cross-target` job)
  emits each product crate's iOS release assembly as an **rlib**
  (`cargo rustc --release --target aarch64-apple-ios -p <crate> --lib --crate-type rlib -- --emit
  asm=...`), counts `^\tbl\t_memset_pattern16$` per crate, and
  `scripts/lib/aarch64-known-defects.py judge-memset` judges the counts against one row per crate
  (`IOS_MEMSET_CEILINGS`). With `lto = "fat"` and `codegen-units = 1`, the per-crate `--emit asm` is
  LLVM's **pre-link** output, not the code that ships.
- **What ships.** An iPhone app links `capi` as a staticlib. Its release assembly after fat LTO is
  one module: every shipped function after cross-crate inlining. The same script already emits that
  form for Android (`--target aarch64-linux-android -p capi --lib --crate-type staticlib -- --emit
  asm=...`, the "no eight-lane code in the Android library" row); no Xcode or NDK is needed.
- **The verifier's measurement (#1456 attempt 2, NIT 4).** Post-LTO iOS `capi`: base 11 calls, head
  5. The limiter's 6 pre-link calls (3 in `clear_runtime`, 3 in `ChannelState::new`) are gone after
  #1456 in both forms. The 5 left are all in `builtins_compiler` preparation code
  (`into_graph_artifact_with_banks` 3, `BuiltinChain::new`, `FaderMuteRampBuiltins::new`). The
  pre-link scan reports `builtins` 5, `host-core` 4 and `soft-clip` 1, so the two forms disagree on
  both the totals and the crate names. Post-LTO, `clear_runtime::<f32>` is also vectorized
  (`stp q0, q0`), which the pre-link form does not show.
- **Attribution.** After LTO a call sits in the function it was inlined into, so a crate's call can
  appear under a caller in another crate. The release profile's line tables keep inlined callee
  names (the eight-lane scan relies on this), and each function label is a mangled path that names
  its crate.

## Decisions frozen for this slice

- **D1.** The ratchet's counted artifact is the post-LTO `capi` staticlib for `aarch64-apple-ios`,
  emitted in the same form as the Android row. A crate's count is the number of
  `bl _memset_pattern16` lines inside function bodies whose label demangles to a path in that crate.
- **D2.** Where the assembly's inline records name the inlined function that holds a call, the call
  is attributed to that function's crate, not the caller's; the implementation records which rule
  it used and why, with an example from the real assembly. If no reliable inline attribution is
  available, the label's crate is used and the record says so.
- **D3.** `IOS_MEMSET_CEILINGS` is re-based to the post-LTO counts, one row per crate that has
  calls, each with its owning issue as today. A row at zero, a count above its ceiling, a crate with
  calls and no row, and a call attributed to no product crate each fail, as `judge-memset` does
  today. The per-crate pre-link scan is removed if it adds nothing the post-LTO count does not
  catch; otherwise the record states what it alone catches and keeps it.
- **D4.** The eight-lane scan of the iOS assembly (#1112) moves to the same post-LTO file if the
  pre-link scan is removed, so its coverage does not drop.

## Deliverables

- `scripts/check-cross-targets.sh`: the post-LTO iOS emission and the per-crate count (D1, D2,
  D4).
- `scripts/lib/aarch64-known-defects.py`: the re-based rows (D3), the parser for the post-LTO
  counts, and its `--self-test` cases for each refusal.
- `docs/TARGET_MATRIX.md`: the #1018 paragraph names the counted artifact.

## Authorized paths

- `scripts/check-cross-targets.sh` (the `ios-asm-memset-pattern16` section and the iOS eight-lane
  scan only).
- `scripts/lib/aarch64-known-defects.py` (the memset rows, the judge and its self-test).
- `docs/TARGET_MATRIX.md` (the #1018 paragraph).

## Non-goals

- Removing any remaining `memset_pattern16` call (each stays owned by its row's issue).
- `bzero` and `memcpy` (#1456 Amendment 1, Q3: not #1018's defect).
- Any engine source change. No rendered bit moves.

## Objective gates

1. `scripts/check-cross-targets.sh` passes on the branch, and its log shows the post-LTO per-crate
   counts and their rows.
2. Mutation: restoring the limiter's `self.required_ring.fill(1.0); self.box_ring.fill(1.0)` in
   `ChannelState::clear_runtime` turns the row red with the limiter named; reverted, green.
3. Mutation: a `memset_pattern16`-producing `fill(1.0)` of a runtime-length `Vec<f32>` added to a
   crate with no row turns the row red ("calls and no row"); reverted, green.
4. `python3 -B scripts/lib/aarch64-known-defects.py --self-test` passes, and each new refusal case
   is red when its check is removed.
5. If the pre-link scan is removed, the eight-lane scan still goes red on an eight-lane function
   compiled into the iOS library (mutation recorded).

## Test value

The self-test cases defend the judge's refusals on the new count format; gates 2 and 3 defend that
the shipped library, not a pre-link rlib, is what is counted.

## Dependencies

- #1456 (stream G).

## Attempt record
