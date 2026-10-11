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

The self-test cases defend the judge's refusals on the new count format. Gates 2 and 3 defend that
a new call in a crate is caught and named, but alone they do not prove that the shipped library is
what is counted: the removed pre-link scan would also go red on both. What proves the counted
artifact is the re-based row values: post-LTO `builtins` 4 and `lane` 1, with the pre-link
`host-core` 4 and `soft-clip` 1 rows gone (pre-link: `builtins` 5, `host-core` 4, `soft-clip` 1).
(Corrected by root's ruling of 2026-10-08.)

## Dependencies

- #1456 (stream G).

## Attempt record

### Attempt 1 (2026-10-08, implementer)

Anchors verified on `04fc3cc8f`: the per-crate rlib loop, `judge-memset`, the Android staticlib
row, `IOS_MEMSET_CEILINGS` (`builtins` 5, `host-core` 4, `soft-clip` 1), the #1018 paragraph, and
the release profile (`lto = "fat"`, `codegen-units = 1`, `debug = 1`).
`scripts/check-ci-path-routing.py` pins two lines of the scan (the products file and the
`judge-memset "$asm_out/counts" "$asm_out/products"` call); both are kept byte for byte.

**D1, D2 (counted artifact and attribution).** The scan emits `capi` for `aarch64-apple-ios` as
the release staticlib, in the Android row's form, and the new `count-memset` subcommand reads it.
Rule used: **DWARF inline records, innermost product-crate frame.** With `debug = 1` the
assembly keeps `__debug_info` as raw `.byte`/`.long`/`.quad` cells (no verbose comments). The
counter decodes the abbreviation tables, the units and their DIEs, resolves `low_pc`/`high_pc`
(`Lset = end-low`) and `DW_AT_ranges` (`Ldebug_rangesN`, entries relative to the unit's
`Lfunc_beginN`) to code-label lines, and collects every subprogram and inlined subroutine whose
code holds each `bl _memset_pattern16` line. A frame's crate is the outermost
`DW_TAG_namespace` above its abstract origin. The call is charged to the innermost frame in a
product crate, so a `core`/`alloc` frame such as `<[T]>::fill` is skipped. With no product frame,
the function label's crate is charged, and the judge refuses it. Any DWARF it cannot read is
refused: an unknown form, a unit that does not end at its length, a `high_pc` that is not an end
label minus `low_pc`, or a call in zero or two described function bodies.
Why not the alternatives: `.loc` gives only the innermost file, which here is a `core` file for
two of the five calls (`slice/iter/macros.rs`, `iter/adapters/mod.rs`, at line 0). The
unlinked Mach-O object has unrelocated addresses, so `llvm-symbolizer` and `llvm-dwarfdump
--lookup` cannot read it (both tried). The `cross-target` runner has no LLVM tools, and the
workflow is outside this slice.
Example from the real assembly: line 629997, `bl _memset_pattern16` in the function label
`builtins_compiler::PreparedBuiltinsSession::into_graph_artifact_with_banks`. The inline records
put it in `lane::kernels::builtins::lanes_below::<f32x4>` (its `.loc` is
`crates/lane/src/kernels/builtins.rs:95`, `*flag = 1.0`), so it is charged to `lane`.
Limit: a frame's function name follows LLVM's location for the synthesized call. Line 54806
(`BuiltinChain::new`) reports `builtins::zero` as its innermost frame. The crate is right, but
the function is approximate.

**Post-LTO counts (`04fc3cc8f` engine code):** 5 calls, the verifier's number.

| line | charged to | via | function label (crate) |
|---|---|---|---|
| 54806 | builtins | `zero` (in `BuiltinChain::new`) | `BuiltinChain::new` (builtins) |
| 85369 | builtins | `FaderMuteRampBuiltins::new` | same (builtins) |
| 627638 | builtins | `BuiltinInputBank::new` | `into_graph_artifact_with_banks` (builtins-compiler) |
| 628665 | builtins | `BuiltinFaderBank::new` | `into_graph_artifact_with_banks` (builtins-compiler) |
| 629997 | lane | `lanes_below::<f32x4>` | `into_graph_artifact_with_banks` (builtins-compiler) |

**D3.** Rows re-based to `builtins` 4 and `lane` 1 (both #1018). The `host-core` and `soft-clip`
rows are deleted: their code is not in the shipped library (`SpectrumAnalyzer` and the soft-clip
corpus do not appear in the post-LTO assembly). The judge also gives calls charged to a crate
outside the product closure their own message.
**The pre-link scan is removed.** Every product crate ships only through `capi`, so a pre-link
call that LTO removes cannot reach a phone. One that LTO keeps is in the post-LTO count. The
pre-link scan caught nothing the post-LTO count misses. It did count code no app links, and it
missed calls inlined across crates.
**D4.** The iOS eight-lane scan runs on the same post-LTO file, as the Android row does.

**Gates.**
1. `scripts/check-cross-targets.sh`: PASS (1m50s). The log lists the five calls above, then
   `expected failure (#1018): builtins 4 calls` and `lane 1 calls`.
2. Limiter mutation (`self.required_ring.fill(1.0); self.box_ring.fill(1.0)` in
   `clear_runtime`): RED, `true-peak-limiter: 4 memset_pattern16 calls and no row`. The calls
   are in `clear_runtime::<f32>` and `::<f32x4>`, 2 each, and the limiter is named. Reverted:
   GREEN.
3. `let mut probe = vec![0.0_f32; black_box(levels.len())]; probe.fill(1.0); black_box(&probe);`
   in `builtins-compiler`'s `into_graph_artifact_with_banks` (no row): RED,
   `builtins-compiler: 1 memset_pattern16 calls and no row`. The other 5 are charged as before.
   Reverted: GREEN.
4. `--self-test`: PASS. Mutation runs, each RED:
   - The label's crate charged instead of the inline frame (D2).
   - The innermost frame charged, `core` included.
   - With no product frame, nothing charged instead of the label crate.
   - The one-function-body check dropped. This goes RED as an uncaught `IndexError`, not as a
     refusal.
   - The unknown-form refusal dropped. The case changes a zero-byte `flag_present` to an
     undefined form, so the DIEs stay aligned and only this check can refuse it.
   - The `high_pc` check dropped.
   - The judge reading product crates only (the new non-product case).

   The judge's new non-product branch only words the message: with it dropped, the no-row
   refusal still refuses (GREEN). With both dropped: RED. The unit-length check stays as an
   invariant. A short unit is also refused by the section bounds check, so no case isolates it.
5. Eight-lane mutation (`core::hint::black_box(wide::f32x8::splat(black_box(2.0)))` in
   `lane::kernels::builtins::lanes_below`, compiled into the iOS library): RED,
   `eight-lane code is back in the iOS library (#1112): 2 lines` (the inlined
   `black_box<wide::f32x8_::f32x8>` names in `__debug_str`). Reverted: GREEN.

Also run: `scripts/check-workspace-policy.sh` ok; `scripts/check-ci-path-routing.py` and
`scripts/test-ci-path-routing.py` passed. No engine source changed. No rendered bit moves.

Open: D1 counts `bl` only, as frozen. A tail call `b _memset_pattern16` would not be counted (none
today). `docs/TARGET_MATRIX.md:10` (the platform table, outside the authorized paragraph) still
says the scan runs "over every product crate".

### Attempt 1 follow-up (2026-10-08, implementer; root's rulings of 2026-10-08)

Applies the verdict on attempt 1 (`/home/bl/misofm/submix-verdicts/1472-attempt1.md`, PASS).

- **Root's amendment of D1 (ruling, 2026-10-08).** The count reads every `_memset_pattern16`
  reference in the post-LTO assembly and refuses any that is not a `bl` line in `__TEXT,__text`
  (`MEMSET_REFERENCE` in `Assembly.__init__`). A tail call `b _memset_pattern16` or an address
  load now turns the gate red instead of leaving the count. The verifier found no such reference in
  the shipped file today (only the five `bl` lines). Self-test case "a tail call" (the last `bl`
  of the synthetic function as `b`).
- **Test value (root's ruling, 2026-10-08).** The Test value section is corrected: gates 2 and 3
  alone would also go red under the removed pre-link scan; the re-based row values (`builtins` 4,
  `lane` 1; `host-core` and `soft-clip` gone) are what prove that the shipped library is counted.
- **`docs/TARGET_MATRIX.md:10` (root authorized, 2026-10-08).** The platform table now names the
  scan's artifact: "the release-assembly scan `ios-asm-memset-pattern16` of the post-LTO `capi`
  staticlib, charged per product crate".
- **MINOR-2.** Attempt 1's record said no case isolates the unit-end check. That was false. New
  case "a unit whose DIE tree does not close": the compile unit's closing `.byte 0` deleted. The
  unit length still matches its labels, so only the `or stack` part of the check refuses it.
- **NIT-2.** Two synthetic variants, so the self-test catches what only the real-assembly rows
  caught: `synthetic_assembly(nested_module=True)` declares `inner` in a module `crate_a` inside
  `crate_b` (as `lane::kernels::builtins` is named like the crate `builtins`), and
  `synthetic_assembly(leading_unit=True)` puts a 25-byte unit first, so the main unit's references
  are unit-relative at a nonzero offset. Both must count `{crate-a: 1, crate-b: 2}`.
- **NIT-3.** `count_memset` tests a frame's crate against the product set, not against `counts`,
  so a call charged to a non-product label crate cannot capture a later call's charge. No case:
  the run is refused either way, and only the message changed.
- **NIT-4.** A `.byte`/`.short` operand that is not a decimal number is refused; only `.long` and
  `.quad` may be symbols. Case "a .byte operand that is not a decimal number": `DW_AT_external`'s
  attribute code as `0x3f`. Zero-filled it reads as attribute 0, which the count ignores, so the
  count would stay right and only this check refuses it.
- **NIT-5.** The two comment lines over 100 columns (58, 64) are wrapped.
- **Note (NIT-6).** With innermost-product-frame attribution, every fill in a generic `lane` helper,
  inlined into any crate, is charged to `lane`'s one row. Replacing one such fill with another (fix
  `lanes_below`, add a render-path fill in another `lane` helper) leaves the count unchanged and
  would not show in the ratchet. The log line of each call names its function, so a reviewer can
  see the swap; the count cannot.

**Mutation runs** (`--self-test` on a mutated copy; each reverted is GREEN):

| mutation | result |
|---|---|
| tail-call refusal removed (`if False:`) | RED, `AssertionError: a tail call` |
| unit-end check without `or stack` | RED, `AssertionError: a unit whose DIE tree does not close` |
| unit-end check removed whole | RED, same case |
| innermost namespace (`crate = attributes.get(AT_NAME) if crate is None else crate`) | RED, `AssertionError: a module named like another crate` |
| unit offset dropped (`value += 0`) | RED, refused: `line 6: a function holding the memset call names no crate` (the leading-unit case) |
| non-decimal `.byte` accepted as a symbol | RED, `AssertionError: a .byte operand that is not a decimal number` |

A first innermost-namespace mutant (`crate = crate or ...`) stayed GREEN only because the
synthetic string offset of `crate_a` is 0, which is falsy; the `is None` form is the real defect.

**Gates.** `python3 -B scripts/lib/aarch64-known-defects.py --self-test`: PASS.
`scripts/check-cross-targets.sh`: PASS (2m04s; run when the disk had 41.9 GiB free, after it had
been 23.0 GiB at the start). The real post-LTO file passes the new refusals (no non-`bl` reference,
no non-decimal `.byte`/`.short`), and the log charges the same five calls (now at lines 56709,
85417, 627300, 628327, 629659 on `d060b8a70`): `builtins` 4, `lane` 1.
`scripts/check-workspace-policy.sh`: ok. `scripts/check-ci-path-routing.py` and
`scripts/test-ci-path-routing.py`: passed. No engine source changed; no rendered bit moves.
