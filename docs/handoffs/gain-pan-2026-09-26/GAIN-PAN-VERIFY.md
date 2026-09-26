# Adversarial verification of #944 and #945 (2026-09-26)

Scratch: detached worktree of `origin/main` (65671c21 = 14f2917b + docs only), harness patch applied,
one target dir, all removed afterwards. Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, cpu 12 pinned
(idle at start), load 2-6 from other agents. `CARGO_INCREMENTAL=0`. The prototype was the brief's
code verbatim: `matrix2x2_block_without_identity` + `MatrixStage::settled_block` at both settled
sites; `tile_rows` = `let (rows, _) = block.as_chunks::<W>(); core::array::from_fn(|i| rows[i])`.
A second build added two `AtomicBool` toggles so the same binary could interleave A/B on one runtime.

## 1. #944 root cause: confirmed

Base `bench` release binary, `<builtins::BuiltinMatrixBank>::process`, settled f32x8 loop (the brief's
quote is exact): `vmulps x4, vaddps x2, vblendvps (yr), vmaskmovps %ymm8,%ymm5,(%rdx,%rax,1)` with
`ymm5 = ~identity` (`vpcmpeqd`+`vpxor`), then `vmovups` for right. Four masked stores in that function
(settled and tail, f32x8 and f32x4); the rest of the masked-store table reproduces exactly.

Prototype binary: each of the 4 loops is now preceded by `vtestps` on the mask and a `je` to a
select-free loop:

```text
vmovups (%rdx,%rax,1),%ymm4 ; vmovups (%r8,%rax,1),%ymm5
vmulps %ymm4,%ymm0,%ymm6 ; vmulps %ymm5,%ymm1,%ymm7 ; vaddps %ymm7,%ymm6,%ymm6
vmulps %ymm4,%ymm2,%ymm4 ; vmulps %ymm5,%ymm3,%ymm5 ; vaddps %ymm5,%ymm4,%ymm4
vmovups %ymm6,(%rdx,%rax,1) ; vmovups %ymm4,(%r8,%rax,1)
```

The mixed arm keeps its `vmaskmovps` (4 left). f32x4 and the tail sites are the same shape.

Standalone microbenchmark, same loop shape, 128 frames, intrinsics:

| form | cycles/frame |
|---|---:|
| blendv + `vmaskmovps` store (today's codegen) | 13.19 |
| two blendv + two plain stores (select kept, fold blocked) | 3.46 |
| plain stores (select-free) | 3.39 |

So the masked store alone costs about 9.7 cycles per frame on Zen 3; the blend costs about 0.1.
uops.info, `VMASKMOVPS (M256, YMM, YMM)`: Zen+/2/3/4 have 42-44 uops and a 12-cycle throughput;
Zen 5 has 2 uops and 0.5; Haswell to Alder Lake have 3-4 uops and 1. So the penalty is real on
Zen 3 **and Zen 4**, small on Zen 5 and Intel. The select-free kernel only ever removes work: a
blend plus a masked store become a plain store. NEON and wasm have no masked store; there it
removes one `bsl`/`v128.bitselect` per plane per frame (the wasm build confirms the new arm has
none). The only costs are one mask test per call and code size (wasm `BuiltinMatrixBank::process`
grows from 1,638 to 1,951 ops). It cannot be slower on any target.

Replicas (base, 3 repeats, per bank): matrix bank settled 1,703-1,712; raw `matrix2x2_block` 1,967;
select-free 473-475; swapped stores 1,984-1,986; fold 772-774; gather 543; input 1,065; fader
381. Ramps: settled matrix 1,712 against ramping 849; fader 383 against 1,093. All reproduce the
brief's numbers.

## 2. In-process A/B on the real plan (one binary, toggles, 6 interleaved rounds, p50 ns)

| row | base | #944 | #945 | both |
|---|---:|---:|---:|---:|
| gain_pan_only | 14,618 | 11,813 | 13,445 | 10,610 |
| builtins_only | 28,815 | 26,090 | 27,543 | 24,797 |
| console | 131,020 | 128,234 | 129,727 | 126,902 |
| plumbing_only | 3,237 | 3,237 | 3,236 | 3,246 |

Separate exact-code binaries, 3 interleaved rounds, the harness's `phase_profile_rows`, median p50:

| row | base | #944 | #945 | both |
|---|---:|---:|---:|---:|
| gain_pan | 15,499 | 12,725 (-2.77 us) | 13,466 (-2.03) | 10,760 (-4.74) |
| dispatch | 15,710 | 12,534 (-3.18) | 13,536 (-2.17) | 10,671 (-5.04) |
| builtins | 29,998 | 26,852 (-3.15) | 28,395 (-1.60) | 25,619 (-4.38) |
| plumbing | 3,246 | 3,266 | 3,166 | 3,166 |

#944's saving is stable at about 2.8 us. #945's saving depends on the build: 1.2 us in the toggle
binary and 2.0 us in the exact binaries, because the base's own time moves by 0.9 us with code layout.
Descriptive only. The base phase profile reproduces: shape [8, 24], 73 units, p50 57,316 cycles
at 3.700 GHz, bank phase 49,774 cycles.

## 3. Exactness

- **Digests.** All 16 `WORKLOADS` 64-block digests are identical across base, #944, #945 and both.
  That holds for release (toggles) and for the dev profile (exact code). #944 gate 4's four pins
  plus the plumbing pin equal the base harness output exactly:
  - gain_pan `01e465a7...`
  - dispatch `15688888...`
  - builtins `b63eccd0...`
  - console `fe5bed9b...`
  - plumbing `57535244...`

  `chain_shape.rs`'s `render` helper hashes the same way.
- **`select(all-false, a, b) == b`.** This is bitwise on every backend: `blendvps` on the sign bit,
  wasm `v128.bitselect`, NEON `vbsl`, and the scalar bit formula. Masks are canonical
  (`mask_from_flags` is a `gt`), and `mask_any` is `movemask != 0`, `any()`, or `m != 0` over
  exactly `L::WIDTH` lanes, padding included. So the guard is right, and it is exact for every
  word.
- **HIGH: gate 1 fails in release.** It is not the select: `b` itself is computed differently.
  LLVM commutes the commutative `fadd`. Base release already emits yr as `vaddps %ymm6,%ymm9,%ymm6`,
  that is `rr*r + rl*l`. The select-free loop emits `rl*l + rr*r`. When l and r are both NaN, x86
  returns SRC1's payload. A prototype of gate 1 passes under `cargo test -p lane` (dev), but under
  `--release` (which CI runs, `qualification.yml:526`) it gives
  `R[3] new=7fc00000 old=7fc01234` for f32, Simd4 and Simd8 on the non-finite family. Rendered plans
  cannot see this, because D7 sanitises non-finite input before the matrix, and the digests are
  unchanged.
- **Negative control.** At `L = f32, frames = 1` the mixed mask degenerates to "all identity" with
  inputs (-0, +0), which the arithmetic reproduces. The two kernels do not differ there. Every
  other (width, frames) case differs.
- **Mutations.**
  - M1 (always select-free) makes `settled_identity_matrix_preserves_signed_zero` red in release,
    by bits.
  - A #945 row offset `rows[(i+1)%W]` makes 4 graph tests red, including
    `a_resident_fold_is_the_staged_scatter_and_cohort_fold_bit_for_bit`. It is still red when
    applied at W=4 only.
  - The same offset makes `chain_shape::the_folded_master_is_the_reductions_own_bits` red.
- **Unmutated suites green.** `graph` (with and without test-support), `builtins --features
  test-support`, `console-workload --release` and `lane` all pass. `check-realtime-policy`,
  `check-graph-policy`, `check-lane-policy` and `check-builtins-policy` pass.
  `check-graph-determinism.sh` hard-codes `target/debug`, so it was not run.

## 4. Coverage of callers

`matrix2x2_block` has exactly two callers, `lib.rs:2988` and `:3017`, and the prototype covers
both. `fader_matrix_block` (paired banks, and `BuiltinChain`'s scalar fused path at `:3099` and
`:4159`) keeps its select. That select is not a masked store, because its first arm is `l*g`, not
the reloaded word. No FaderBank function has a `vmaskmovps`.

The web host's main path pairs the fader and matrix (`BetweenRenderCalls`), so #944 does not
touch the browser's main path. No rack collapse seam calls the matrix.

The pan law cannot give the identity: `lr = cos(pi/2) = 6.1e-17`, and every fixture track uses
`pan`. An explicit `matrix {1,0,0,1}` (session or `matrix_*` retarget) does set the mask, so the
whole bank keeps today's masked store, and one such track slows its bank of 8. Note that
`balance_matrix(0.0)` also returns `IDENTITY`; it is unwired today.

## 5. Wasm

The base build reproduces the pin `8934cdd9...`; the prototype is `6cd3baa9...`. So
`check-web-audioworklet.sh` fails at the pin before any callgraph check: gate 7 as written is
unachievable until the repin.

Piping `wasm-objdump -d` into the callgraph script gives:

- The render closure is unchanged: closure 8, traps 5, same trap owner.
- The kernel shape passes with the same 15 kernels and the same roster; `f32x4_arith` goes from
  11,683 to 11,725.

Rule 3 is vacuous for both issues: `BuiltinMatrixBank::process` and `fold_resident` carry no
`4wide6f32x[48]` in their v0 names, because the generics are inlined into non-generic methods.
`fold_resident` is 160 vector against 192 scalar both before and after, and is unmatched. #945
does remove the two `memory.copy` from wasm `fold_resident` (`v128.store` falls from 14 to 6).

## 6. #945 codegen

Bench `fold_resident`:

| | base | prototype |
|---|---:|---:|
| `memcpy` calls | 4 | 0 |
| `vzeroupper` | 18 | 11 |
| stack spills | 100 | 66 |

The W=8 tile now loads its 8 rows directly with `vmovups -0xe0(%r10)..(%r10)` into the transpose.
A short `cmp`/`je` chain of per-row bounds checks from `rows[i]` remains, which is harmless.

## 7. Stale docs and the class-B note

- `tools/bench/src/floor.rs:236-240` and the `SixtyFourTrackGainPanOnly` doc
  (`console-workload/src/lib.rs:271-283`) state that the matrix has no data-dependent path. After
  #944 it has one.
- Ruling 4 (correct `dispatch_only`'s doc) is not in #944's authorised paths.
- S-E misses a hazard. A mute folded in as a zero column is not a hard mute: `0*inf` and `0*NaN`
  are NaN, while `andnot` makes any input exactly `+0.0`. That is a safety semantic, not a
  rounding change.
