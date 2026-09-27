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

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-26/27. Code commit `b8df5b52` on `codex/945-fold-tiles-without-copy`
(on top of #936's branch tip `a7d8fc0d`, which is the "base" everywhere below). Host AMD EPYC 7313P
(Zen 3), rustc 1.97.1, `.cargo/config.toml` pin `+avx2,+fma`, `CARGO_INCREMENTAL=0`, one scratch
target directory. The digest harness, the base/after release binaries and the wasm builds were made
in a detached scratch worktree of the base with `gain-pan-diagnosis-harnesses.patch` applied, and
`runtime.rs` copied in for the "after" builds; the worktree and the patch were removed afterwards
and nothing from them is committed. No timed runner and no timing were run; no saving is quoted.

### Design

Only `tile_rows` changed (authorized path; its one call site is untouched):

```rust
let (rows, _) = block[..W * W].as_chunks::<W>();
core::array::from_fn(|row| rows[row])
```

Row `i` is words `i * W .. (i + 1) * W`, exactly the rows the zeroed tile plus `copy_from_slice`
formed from a `W * W`-word block, taken by value, so no bit can move. The transpose, `fold_words`,
the ragged tail, the premises and the association order are unchanged. No `unsafe`, no `wide`, no
intrinsics.

**Deviation (form, not scope).** The brief's prototype is `block.as_chunks::<W>()` without the
`[..W * W]` slice. That form was built and checked first: 0 `memcpy`, all 16 digests identical. But
the length of `rows` is then a runtime value (the `ChunksExact::zip` constructor stays out of line,
so LLVM cannot see that every chunk is `W * W` words), and the release `W = 8` tile loop kept a
chain of five row-bounds compares per tile (`test %rsi; cmp $0x1; cmp $0x2; cmp $0x3; cmpq $0x20`).
Slicing to exactly `W * W` words makes `rows.len() == W` a constant: the loop keeps one
`cmp $0x40,%rax; jb` per tile, a panic path that states the caller's premise. A shorter block would
now panic where the old code zero-padded; the caller hands exactly `W * W` words
(`chunks_exact(W * W)`), so neither is reachable.

### Gate 1 -- digests: PASS (unchanged)

The `digests` harness (release), every standing workload's 64-block output digest:

| workload | base `a7d8fc0d` | after `b8df5b52` |
|---|---|---|
| nine_track_baseline | `e7c6ef01770ab7da...` | same |
| nine_track_ragged_strip | `17613a3ab693d3f0...` | same |
| sixty_four_track_console | `fe5bed9becdbc101...` | same |
| one_twenty_eight_track_stretch | `cba2c94f81544caa...` | same |
| sixty_four_track_eq_only | `9b2c56a1da62ebda...` | same |
| sixty_four_track_compressor_only | `95c9375429fbca34...` | same |
| sixty_four_track_builtins_only | `b63eccd09c19eb7a...` | same |
| sixty_four_track_dispatch_only | `15688888612d161e...` | same |
| sixty_four_track_idle | `de2f256064a0af79...` | same |
| sixty_four_track_console_legacy | `f68febb7a10e242b...` | same |
| sixty_four_track_eq_comp_simd1 | `f68febb7a10e242b...` | same |
| sixty_four_track_plumbing_only | `57535244ba953d82...` | same |
| sixty_four_track_gain_pan_only | `01e465a797036fb4...` | same |
| sixty_four_track_console_mono | `fc96d91f6a397e91...` | same |
| sixty_four_track_console_mono_dual | `fc96d91f6a397e91...` | same |
| sixty_four_track_console_half_mono | `4a656cdf63882999...` | same |

All 16 full 64-hex digests are byte-identical (`diff` of the two outputs is empty). The gain/pan,
dispatch, builtins, console and plumbing values equal the pins in `GAIN-PAN-VERIFY.md` section 3.
In the tree: `cargo test --locked -p console-workload` passes (lib 5, automation 4, chain_shape 24,
placement 3), including `the_plumbing_row_is_input_route_output_and_renders_the_base_bits`
(`BASE_DIGEST`) and `the_folded_master_is_the_reductions_own_bits`.

### Gate 2 -- standing fold tests: PASS

`cargo test --locked -p graph`: lib 113 of 113, rt10 2, rt1 1, rt9 1. `cargo test --locked -p graph
--features test-support`: lib 113 of 113, rt10 2, rt1 1, rt9 8. Among them the resident witness
`a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit` (W4 and W8, 1 and 2 cohorts,
4/8, 13, 16 and 128 frames), `a_resident_fold_declines_before_writing_on_a_broken_premise`, and the
probes near the old `:9327`, `:9806` and `:9990`. No test was edited.

### Gate 3 -- no copy call: PASS

`cargo build --locked --release -p bench`, then `objdump -d -C` over the `nm -S` extent of
`<graph::runtime::ArenaMembers as rack::BankMembers>::fold_resident` (both widths are inlined into
it):

| `fold_resident` in `bench` | base | after |
|---|---:|---:|
| size (bytes) | 7,285 | 6,111 |
| `call` to `memcpy`/`memset`/`memmove` | 4 | 0 |
| `vzeroupper` | 18 | 9 |
| instructions addressing `%rsp` | 454 | 311 |

Base, the head of the `W = 8` tile loop: zero both stack tiles (16 `vmovups %ymm0` stores), then one
runtime-length `memcpy` per plane, each behind a `vzeroupper` and a spill/reload of the live rows:

```text
327c37: vmovups %ymm0,0x210(%rsp)   ... 8 stores, 0x210..0x130   ; zero tile (left)
327cab: lea    0x130(%rsp),%rdi ; mov 0x30(%rsp),%rsi ; mov 0x4c8(%rsp),%rdx
327cc0: vzeroupper
327cc3: call   *0x20394f(%rip)        # memcpy
327cd6: vmovups 0x130(%rsp),%ymm2   ... 8 reloads
327d22: vmovups %ymm0,0x210(%rsp)   ... 8 stores                 ; zero tile (right)
327d8b: vmovups %ymm4,0x3a0(%rsp)   ... 8 spills
327dd3: vzeroupper
327dd6: call   *0x20383c(%rip)        # memcpy
327ddc: vmovups 0x5e0(%rsp),%ymm2   ... 8 + 8 reloads
327ee0: vunpcklps %ymm1,%ymm2,%ymm0                               ; transpose
```

The `W = 4` loop had the same shape (`0x3276da`, `0x32777d`: two more `memcpy`, 64-byte tiles).

After, the `W = 8` tile loop reads its rows straight from the resident block into the transpose:

```text
327a0a: cmp    $0x40,%rax ; jb <panic>                             ; the one `[..W * W]` check
327a1f: cmp 0x18(%rsp),%r11 ; jae ... ; cmp 0x8(%rsp),%rsi ; ja ... ; master_left[base..base + W]
327a38: vmovups -0xe0(%r10),%ymm0
327a41: vmovups -0xc0(%r10),%ymm1
   ...  (six more rows, -0xa0 .. 0x0)
327a6b: vmovups (%r10),%ymm7
327a70: vunpcklps %ymm1,%ymm0,%ymm8                               ; transpose
```

The `W = 4` loop is the same (`vmovups -0x30(%r9)..(%r9),%xmm0..3`, then `vunpcklps`). The only
remaining calls in the function are `master_planes`, the `ChunksExact::zip` constructor (both
pre-existing, outside the loop) and cold panic entries.

### Gate 4 -- wasm (amendment 2): PASS

Built with the build script's own line, from the scratch worktree:
`CARGO_TARGET_DIR=<scratch> RUSTFLAGS="-C target-feature=+simd128 -C strip=debuginfo
--remap-path-prefix=$CARGO_HOME=/cargo --remap-path-prefix=<repo>=/repo" cargo build --locked
--release --target wasm32-unknown-unknown -p host-web`, then `wasm-objdump -d host_web.wasm` piped
into both callgraph checks. Artifacts: base `319719052a107f78...`, after `0b5d6055240e264a...`.
The base already differs from the committed pin `8934cdd9...` (issue #936's change is in it); the
pin was not touched, per the batch-boundary repin.

- `--callgraph miso_engine_web_v1_render`: base and after both `closure=8 traps=5`, the same trap
  owner (`PreparedRenderPlan::render_inner`) and entry (`slice_index_fail`). PASS, unchanged.
- `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11`: base and after both
  `f32x4_arith=11683 kernels=15`; the full outputs, every roster row included, are identical.
  PASS, unchanged.
- Rule 3 is vacuous here, as the amendment says: `fold_resident`'s v0 name carries no
  `4wide6f32x[48]`, because the generic tile body is inlined into the non-generic trait method.
- `fold_resident` (func 1952), base -> after: `memory.copy` 2 -> 0, `memory.fill` 0 -> 0,
  `v128.store` 14 -> 6, `v128.load` 18 -> 146 (the rows now load directly), ops 4,547 -> 3,289,
  `f32x4.{mul,add,sub,div}` 160 -> 160, scalar `f32` arithmetic 192 -> 192 (the ragged tail).
  Direct calls 20 -> 24: four more cold `slice_index_fail` sites, the `[..W * W]` check per plane
  and width; `fold_resident` is not in the render export's direct-call closure, which is unchanged.

### Gate 5 -- allocation and policy: PASS

- `bash scripts/check-realtime-policy.sh`: `realtime policy: ok (54 marked regions in 16 files)`.
  `tile_rows` is inside a marked region; indexing is permitted there, and it has no `.expect(` or
  `.unwrap(`.
- `bash scripts/check-graph-policy.sh`: `graph policy: PASS`.
- `bash scripts/check-graph-determinism.sh`: `graph fresh-process determinism: PASS (100/100)`. The
  script hard-codes `target/debug`, so it was run through a temporary `target` symlink to the
  scratch target directory, removed afterwards.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- The counting-allocator tests rt1, rt9 and rt10 pass with `test-support` (gate 2).

### Gate 6 -- red mutation: PASS

Recorded in `crates/graph/tests/MUTATIONS.md`, "Issue #945":

- 945-1, `rows[(row + 1) % W]`: graph 4 of 113 red (the resident witness at
  `Four, 4 frames, 1 cohort(s), store true: left master`), chain_shape
  `the_folded_master_is_the_reductions_own_bits` red, and 13 of 16 release digests move
  (gain/pan `01e465a7` -> `26fcd980`). Only idle, plumbing (`BASE_DIGEST`, no bank) and
  console_half_mono keep their bits: they take no resident fold.
- 945-1b, `W = 4` only: the same 4 graph tests red; chain_shape and all 16 digests green, because a
  native x86-64-v3 bank is eight lanes wide. The resident witness, which runs `BankWidth::Four`
  itself, is `W = 4`'s gate (the width the browser ships).
- 945-1c, `W = 8` only: graph 3 red (the witness at `Eight, 8 frames, ...`), chain_shape red, the
  same 13 digests move.

### Gate 7 -- fmt, clippy, doc, tests: PASS

- `cargo fmt --all --check`: clean.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: clean (178 crates
  checked).
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: clean.
- `cargo test --locked -p graph` (with and without `test-support`), `-p console-workload`: as gates
  1 and 2. `cargo test --locked -p capi`: 32 + 4 passed.

### Deviations and notes for the verifier

1. The `[..W * W]` slice beyond the prototype (see Design). It is bit-neutral (checked: the
   prototype form and this form both reproduce all 16 digests) and removes the per-row bounds
   chain; its only cost is one cold panic site per plane and width.
2. Gate 1's `W = 4` coverage on this host comes only from graph tests: no console workload banks at
   four lanes natively (945-1b).
3. The base wasm artifact does not reproduce the committed pin because #936 is under it; the
   callgraph checks were run on the piped disassembly instead, per amendment 2.

## Sol attempt 1 verdict: PASS

Reviewer: Sol, 2026-09-27, `git diff a7d8fc0d..60eaae13`. Every gate was re-run independently: the
worktree for fmt, clippy, doc, tests and policy scripts; a `git archive` copy of `60eaae13` with
`gain-pan-diagnosis-harnesses.patch` applied for digests, mutations, release `bench`, wasm and
determinism, with `runtime.rs` swapped between `a7d8fc0d` and `60eaae13`. One scratch target
directory, `CARGO_INCREMENTAL=0`, no timed runner, no timing. Nothing from the scratch copy is
committed, and the worktree was left clean with no `target` entry.

### Gates, reproduced

| gate | result |
|---|---|
| 1 digests | All 16 `digests` rows byte-identical base against after, and equal to the evidence table (gain/pan `01e465a7...`, plumbing `57535244...`). `chain_shape` 24 of 24, including `BASE_DIGEST` and `the_folded_master_is_the_reductions_own_bits`. |
| 2 fold tests | `cargo test -p graph`: lib 113, rt10 2, rt1 1, rt9 1. With `test-support`: lib 113, rt10 2, rt1 1, rt9 8. No test edited. |
| 3 x86 | `fold_resident` in release `bench`: 7,285 -> 6,111 bytes, `memcpy` calls 4 -> 0, `vzeroupper` 18 -> 9, `%rsp` instructions 454 -> 311. The `W = 8` loop loads rows with `vmovups -0xe0(%r10)..(%r10)` straight into `vunpcklps`. The `W = 4` loop does the same with `xmm`. |
| 4 wasm (amendment 2) | Artifacts reproduce the implementer's exactly: base `319719052a107f78...`, after `0b5d6055240e264a...`. The render callgraph, kernel shape, meter_poll and command_submit outputs are identical base against after (`closure=8 traps=5`, same owner and entry; `f32x4_arith=11683 kernels=15`). In func 1952 `fold_resident`, `memory.copy` goes 2 -> 0, `v128.store` 14 -> 6, and direct calls 20 -> 24. |
| 5 policy | `check-realtime-policy.sh` ok (54 regions in 16 files), `check-graph-policy.sh` PASS, `check-workspace-policy.sh` ok, `check-graph-determinism.sh` PASS 100/100. The determinism script ran through a symlink in the scratch copy, which was removed afterwards. |
| 6 mutations | 945-1: graph 4 of 113 red, chain_shape red, 13 of 16 digests move (gain/pan -> `26fcd980`). 945-1b: graph 4 red, chain_shape 24 green, 0 digests move. 945-1c: graph 3 red, chain_shape red, the same 13 digests. All exactly as recorded. |
| 7 hygiene | fmt clean. clippy `--workspace --all-targets --all-features -D warnings` clean. `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps` clean. `-p console-workload` 5/4/24/3, `-p capi` 32 + 4. |
| scope | Only `runtime.rs` (`tile_rows` alone, `:2173-2183`), `MUTATIONS.md` and this spec changed. There is no `unsafe`, `wide` or intrinsic in the diff. The brief and amendments are untouched; the evidence is append-only. |

### Exactness and the `[..W * W]` deviation

- The caller hands exactly `W * W` words. The premise `resident_left.len() == frames * W` is at
  `runtime.rs:2125`, and `tiled = frames - frames % W` at `:2138` makes `tiled * W` a multiple of
  `W * W` no larger than the length, so `chunks_exact(W * W)` at `:2140-2141` yields only full
  blocks. `block[..W * W]` is therefore the whole block and never out of bounds. `as_chunks::<W>`
  yields exactly `W` rows, and `from_fn(|row| rows[row])` is row `i` = words `iW .. (i + 1)W` by
  value, the same words the zeroed tile plus `copy_from_slice` formed. A typed copy of `[f32; W]`
  moves bits unchanged (x86 `vmovups`, wasm `v128.load`/`load32_lane`). The panic cannot fire on
  the render thread.
- Adversarial extra (not recorded in MUTATIONS.md): zeroing only the last row at `W = 4` makes 5
  graph tests red, including
  `a_resident_fold_stores_its_first_contributor_so_a_negative_zero_master_keeps_its_sign`. So the
  `W = 4` witnesses see a single-row defect, not only a rotation.
- I built the brief's prototype form (no slice) to judge the deviation. On x86 it keeps the
  five-compare chain per `W = 8` tile, as the evidence claims (`test %rsi; cmp $0x1; cmp $0x2;
  cmp $0x3; cmpq $0x20`), with 6,251 bytes and 11 `vzeroupper`. In wasm it adds the same four panic
  sites as `panic_bounds_check` (calls 20 -> 24). The deviation is therefore neutral to better on
  every axis.

### Findings (severity-ranked; none blocking)

1. **Low: `W = 4` has no executing end-to-end witness.** Amendment 1 says chain_shape goes red
   under a `W = 4`-only variant. It does not on an x86-64-v3 host: native banks are eight wide
   (`BankWidth::for_backend(Simd8)`). The implementer disclosed this correctly (945-1b), and I
   reproduced it. `W = 4`, the width the browser ships, is guarded only by native graph unit tests
   that run `BankWidth::Four`: `runtime.rs:9875`, `:11496`, `:11915` and `:14111`, all red under
   945-1b. No wasm gate executes the fold:
   - the callgraph and kernel checks are static;
   - `fold_resident` does not match rule 3's pattern;
   - the browser-v1 parity fixture has one track, so it never forms a full folded bank;
   - qualification has no aarch64 leg.
   Because the change is target-independent Rust, the native `W = 4` tests are a sound witness
   for #945. A wasm or aarch64 executed console digest would be a separate, stateless follow-up.
2. **Info: four cold wasm panic sites are acceptable.** func 1952 goes from 10 to 14
   `slice_index_fail` calls and from 14 to 18 `unreachable`; native x86 panic calls go from 14 to
   15. They are unreachable by construction (above) and the same class as the function's existing
   14 checked-index sites. The realtime policy permits indexing in marked regions (`:1790-2228`).
   `fold_resident`'s only direct caller is `rack::BankChain::scatter`, which is reached by
   `call_indirect`, so the function is outside the render export's direct-call closure. The pinned
   census is unchanged and never covered this function. No safe form of the brief's design avoids
   the sites, short of a silent fallback tile.
3. **Info: evidence wording.** "`v128.load` 18 -> 146" counts the whole `v128.load*` family. The
   128 added loads are `load32_zero` and `load32_lane` in the `W = 8` instantiation, which the
   browser never runs. They replace 128 `f32.load`s from the old stack tile (238 -> 110). The
   shipped `W = 4` path's plain `v128.load` count stays 18 and now reads the resident block. The
   "ops" figures are line counts; instruction counts are 4,514 -> 3,257. The conclusions are
   unaffected.
4. **Info, out of scope.** In wasm, func 2020, the `W = 8` `constants` `from_fn` at
   `runtime.rs:2135`, keeps one `memory.copy` before and after. It runs once per cohort, outside
   the tile loop, only at `W = 8` (unreached in the browser), and has no x86 counterpart. It is
   listed only for the copy-rule inventory.

The AudioWorklet artifact pin (`8934cdd9...`) is intentionally not repinned. Per the brief it is
repinned once at the batch boundary, together with #936's change beneath this branch.
