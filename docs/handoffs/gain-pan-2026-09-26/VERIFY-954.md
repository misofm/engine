# Adversarial verification of #954 (2026-09-27)

Scratch: detached worktree of `codex/batch-plumbing-floor-2` at `72e0c20e`, own target dir, both
removed afterwards. Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, node 22.23.2 (V8), cpu 14 pinned,
`CARGO_INCREMENTAL=0`. Line numbers below are on `72e0c20e`.

Prototype = the brief's design: `lane::kernels::builtins::fader_matrix_block_without_identity`
(`fader_matrix_block` with both `L::select` removed), and a private dispatcher in `builtins` that takes
it when `!L::mask_any(matrix.identity)`, at all four `fader_matrix_block` call sites. Timing builds add a
static `AtomicBool` toggle so one binary interleaves A/B; the codegen build has no toggle (with fat LTO
LLVM constant-folds a never-stored toggle to false and deletes the new arm, so a toggle build is
useless for disassembly).

## 1. Does the fused path pay a masked store? No. It pays two blends.

Base release `bench`, `<FaderMatrixBankProcessor as GraphPreparedBuiltinBankProcessor>::process`
(the fused kernel is inlined there; `try_process_settled_with_matrix` has no own symbol), f32x8 loop:

```text
vmulps (%rdi),%ymm11,%ymm7 ; vandps %ymm0,%ymm7,%ymm7      # l = load*gl andnot mute
vmulps (%rsi),%ymm12,%ymm8 ; vandps %ymm1,%ymm8,%ymm8      # r
vmulps %ymm7,%ymm3,%ymm9 ; vmulps %ymm4,%ymm8,%ymm10 ; vaddps %ymm10,%ymm9,%ymm9
vblendvps %ymm2,%ymm7,%ymm9,%ymm9                          # select(identity, l, yl)
vmulps %ymm7,%ymm5,%ymm7 ; vmulps %ymm6,%ymm8,%ymm10 ; vaddps %ymm7,%ymm10,%ymm7   # rr*r + rl*l
vblendvps %ymm2,%ymm8,%ymm7,%ymm7
vmovups %ymm9,(%rdi) ; vmovups %ymm7,(%rsi)
```

No `vmaskmovps` anywhere in the function (the binary's masked-store census is unchanged from #944's:
`BuiltinMatrixBank::process` 4 in its mixed arm, EQ, transient shaper, input stage, `fold_plane`). The
fold cannot happen: the select's first arm is `load*g andnot mute`, not the reloaded word. #944's own
verification (`GAIN-PAN-VERIFY.md` §4) already recorded this. The f32x4 loop is the same shape.

Wasm (`host-web`, build script's cargo line minus remap): `BuiltinFaderBank::try_process_settled_with_matrix`
is its own function; its Simd4 loop is `load, mul, and, mul, load, mul, and, mul, add, bitselect, store,
mul, mul, add, bitselect, store`: two `v128.bitselect` per frame (four in the Simd8-as-2xf32x4 loop).

Prototype (no toggle) `bench`: four loops; the two select-free ones are
`vmulps vandps vmulps vandps vmulps vmulps vaddps vmulps vmulps vaddps vmovups vmovups` (no
`vblendvps`), the select arm keeps its two `vblendvps`; the guard is one `vtestps` per call. The
select-free yr is `rl*l + rr*r` while the select form's is `rr*r + rl*l` (LLVM commutes), so NaN
payloads can differ exactly as in #944. Prototype wasm: two new loops with no `bitselect`, the select
loops unchanged, guard `v128.any_true`; `--callgraph miso_engine_web_v1_render` closure 8, traps 5 (same
as base); `--kernel-shape` kernels 15 (same), `f32x4_arith` 11,719 -> 11,751.

## 2. The prize

In-process, one toggle binary, `SessionRuntime` (console-workload), 10 interleaved paired rounds x 600
timed blocks, p50 ns/block. "BRC" = prepared through
`prepare_session_builtins_between_render_calls` with no meters (a scratch switch in `build_full`), which
fuses fader+matrix exactly as the metered row does.

| row (native, Simd8) | off | on | median paired diff |
|---|---:|---:|---:|
| gain_pan_only, BRC | 10,410 | 10,240 | **-170** (all 10 rounds -150..-191) |
| gain_pan_only, Concurrent (control) | 10,340 | 10,360 | +11 (noise) |
| builtins_only, BRC | 24,116 | 23,965 | -150 |
| console, BRC, unmetered | 126,922 | 126,762 | -200 (-401..+101) |
| console_metered (#881) | 147,260 | 147,160 | 0 (-280..+431): unresolvable |

Isolated kernel, W8 bank, 128 frames: 792 -> 668 cycles per bank-block (6.18 -> 5.22 cycles/frame);
x8 banks ~ 1,000 cycles ~ 0.27 us. Mechanism: on Zen 3 `vblendvps` competes with the six `vmulps` for
FP0/FP1. That is ~1 cycle/frame, not #944's ~10.

Wasm under V8 (node 22, x64 Zen 3), the product's browser engine, Backend Simd4 (16 banks):

| | select form | select-free |
|---|---:|---:|
| isolated kernel, 4-lane bank-block | 632 cyc (4.94/frame) | 396 cyc (3.09/frame) |
| x16 banks | 2.73 us | 1.71 us |

In situ: `wasm-console-guest` with two scratch exports, driven from node, 8 paired rounds x 400 blocks:

| row (wasm, V8) | off p50 | on p50 | median paired diff |
|---|---:|---:|---:|
| gain_pan_only, BRC | 21,140 | 19,988 | **-1,132** |
| gain_pan_only, Concurrent (control) | 20,570 | 20,569 | 0 |
| builtins_only, BRC | 51,899 | 50,817 | -1,072 |
| console, BRC | 293,629 | 292,677 | -981 |
| console, Concurrent (control) | 293,679 | 293,709 | +130 |

Note the inversion today: on wasm the product's fused BRC gain/pan plan (21.14 us) is *slower* than the
split Concurrent plan (20.57 us), because #944 gave only the split matrix the select-free arm. #954
makes the fused plan the faster one (19.99 us). ARM (NEON `bsl`, V8 arm64) not measured; expect less.

Verdict on size: native ~0.15-0.2 us per 64-track block (~1/15 of #944's 2.8 us, invisible on the
metered row); browser (V8 x64) ~1 us per 64-track block. Worth a small issue, justified by wasm.

## 3. Exactness

- **Digests.** All 16 `WORKLOADS` rows plus the metered row, 64 blocks, toggle off vs on, under both
  Concurrent and BRC preparation: every digest identical (and BRC digests equal Concurrent digests).
  Concurrent rows record **0** fused calls: the standing pins never reach the fused kernel. The metered
  row and every BRC 64-track row take the select-free arm on all 8 banks (512 calls / 64 blocks). The
  nine-track rows under BRC keep one select call per block (a partial bank; identity padding).
  Wasm: gain_pan/builtins/console digests identical off/on under both deliveries.
- **Differential (lane, scratch test).** Select-free fused vs `fader_matrix_block` with an all-false
  mask, and vs `gain_mute_block` x2 + `matrix2x2_block_without_identity`: 5 hostile families (finite,
  signed zero, subnormal, non-finite with payload, overflow-to-inf `f32::MAX`), 3 gain sets (incl.
  1.0, 0.0, 3.98), 3 mute sets (none, mixed, all), frames {1,3,8,9,128}, guard words, f32/Simd4/Simd8.
  Dev: bit-identical everywhere. Release: 0 non-NaN differences, 0 NaN-vs-non-NaN, **54 NaN-payload
  differences at f32**. Second oracle: 0 differences (NaN as both-NaN). Negative control (mixed identity
  mask, signed-zero family) differs at every width.
  So #944's class statement is the right one here too: every non-NaN word unchanged, NaN stays NaN.
  Mute (`andnot`) and gain are applied identically before the matrix in both arms; a muted lane, gain
  1.0, gain 0, and `-0.0` inputs give identical words. No case differs with no identity lane.
- **No ramp tail exists on the fused path.** `try_process_settled_with_matrix` returns false if any
  lane of either stage has `remaining != 0`; the whole block then runs `fader.bank.process` +
  `matrix.bank.process`, whose post-ramp tail already uses #944's `settled_block`. So there is no
  fused tail call site and no M4 analog.
- **Guard placement.** `L::mask_any(matrix.coef.identity)` must be read inside
  `try_process_settled_with_matrix` (each backend arm), after its `remaining_nonzero` checks, i.e. after
  `FaderMatrixBankProcessor::process` has run `drain_fader_controls` and `drain_matrix_controls`
  (`builtins-compiler/src/lib.rs:1070-1071`); a drained matrix record goes through
  `set_target_smoothed` -> `sync_settled` (`builtins/src/lib.rs:2904`), which rewrites `identity`
  (exact for settled lanes: `remaining == 0 && current == IDENTITY`, and the fused path requires every
  lane settled). Same for the scalar sites: after `is_settled()` in `process_fader_matrix` (drains at
  `builtins-compiler/src/lib.rs:4349-4377`, `:4515`) and in `BuiltinChain::process_dual_mono`. Never
  cached: not at `make_fader_matrix`, not in `BuiltinFaderBank`/`FaderMatrixBankProcessor`/
  `FaderMuteRampBuiltins` (sealed sizes; stale after a same-block instant retarget to `IDENTITY`).

## 4. Gates and scope

- Call sites of `fader_matrix_block` (four): `BuiltinChain::process_dual_mono` (`builtins/src/lib.rs:3119`,
  f32; used by `tools/bench`, `tools/audit` fixtures, compiler tests), `try_process_settled_with_matrix`
  Simd4 `:3833` and Simd8 `:3845` (the product's banked path, via `FaderMatrixBankProcessor`,
  `builtins-compiler/src/lib.rs:1079`), `FaderMuteRampBuiltins::process_fader_matrix` `:4179` (f32;
  per-track fused processors at `builtins-compiler/src/lib.rs:4377`, `:4533`). The kernel is at
  `lane/src/kernels/builtins.rs:346` on this base (the brief's `:292` is the pre-#944 line).
- **Row gate.** "Every console digest unchanged" cannot see the change except on the metered row. The
  discriminating existing gates are `console-workload`'s
  `the_metered_console_row_renders_the_console_bits_and_publishes_every_window` (metered digest ==
  `fe5bed9b...`) and `builtins-compiler`'s
  `composite_live_sequence_matches_original_owners_and_discriminates_both_branches` (fused vs split,
  Simd4 full bank). **M3** (swap `lr`/`rl` in the new kernel): both red (verified; metered digest became
  `d3bc88bd...`).
- **M1** (always select-free): survives every existing release test (`builtins`, `builtins-compiler`,
  `console-workload`, `check-builtins-fixtures.sh`, all verified green under M1); in dev it is caught
  only by the kernel's `debug_assert!`. A new scenario gate must catch it by bits: a full bank with one
  exact `Matrix2x2::IDENTITY` lane and a frame with `l = -0.0, r = +0.0` (unmuted) through the fused
  dispatch.
- **M2** (never select-free): only a dispatch witness sees it.
- "x86 disassembly showing plain stores" is non-discriminating (the base already stores with
  `vmovups`); the codegen evidence must be "no `vblendvps` in the select-free fused loops, the select
  arm keeps them" and, on wasm, "no `v128.bitselect` in the select-free loops".
- Wasm: the host-web artifact changes, so `check-web-audioworklet.sh` stops at the pin (as #944
  amendment 4); census via the build script's cargo line: kernels stay 15, closure 8 / traps 5.
- Unmutated prototype: `lane` (dev and release), `builtins --features test-support`,
  `builtins-compiler --features test-support`, `console-workload --release` all green.
- No native row can show the saving: ~0.2 us against +-0.3 us run noise on the 147 us metered row, and
  #955's unmetered BRC console row (~127 us) is no better. The wasm console arm runs `WORKLOADS`
  (Concurrent) under wasmtime, so it never enters the fused path either.

## Reproduction (scratch only; nothing committed)

Harness files (deleted with the worktree): `tools/console-workload/tests/v954.rs` (`digests`, `rows`,
`kernel`), `crates/lane/tests/v954_exact.rs`, `tools/v954-wasm` (cdylib kernel microbench),
two exports added to `tools/wasm-console-guest`, node drivers `v954_wasm_bench.mjs`,
`v954_guest_bench.mjs`.

```text
CARGO_INCREMENTAL=0 cargo test --release -p console-workload --test v954 --no-run
taskset -c 14 target/release/deps/v954-<hash> --ignored --nocapture --test-threads 1 digests|rows|kernel
RUSTFLAGS="-C target-feature=+simd128" cargo build --release --target wasm32-unknown-unknown -p wasm-console-guest
taskset -c 14 node v954_guest_bench.mjs both
objdump -d --no-show-raw-insn -C target/release/bench   # FaderMatrixBankProcessor::process
```
