# Selected lowering excerpts

These compact excerpts retain only the independently mapped
`DB_PER_OCTAVE` and `OCTAVES_PER_DB` cases. They come from the temporary
payloads named in `payload-manifest.md`; complete `.s` and `.ll` payloads
remain outside the repository.

## Caller and loop map

```text
native .ll:9455   define ... Shaper<f32,1>::process_block
native .ll:5611   define ... Shaper<wide::f32x8,8>::process_block
native .ll:21295  call ... Shaper<f32,1>::process_block       ; public scalar process
native .ll:21315  define ... PreparedTransientShaperBank<wide::f32x8,8>::process_bank
native .ll:21384  call ... Shaper<wide::f32x8,8>::process_block
native .ll:22846  define ... PreparedTransientShaperBank<wide::f32x8,8>::process_bank_mono

wasm-scalar .ll:44468 define ... Shaper<f32,1>::process_block
wasm-scalar .ll:56006 call ... Shaper<f32,1>::process_block ; public scalar process

wasm-simd128 .ll:448   define ... Shaper<wide::f32x4,4>::process_block
wasm-simd128 .ll:24145 define ... PreparedTransientShaperBank<wide::f32x4,4>::process_bank
wasm-simd128 .ll:24214 call ... Shaper<wide::f32x4,4>::process_block
wasm-simd128 .ll:25418 define ... PreparedTransientShaperBank<wide::f32x4,4>::process_bank_mono
```

The native W8 LLVM body at `.ll:5611` and Wasm W4 body at `.ll:448` each
load link mode and switch over `1`, `2`, and `3` before their loop families.
The retained constant claims below are restricted to the listed physical
instruction locations inside those caller-reachable bodies.

## Native AVX2 W8

The W8 function starts at `.s:9521-9524`. Its ramping frame body is
`.LBB5_90` at `.s:9980`; its frame backedge is `.s:10740-10742`. The mapped
constants are repeated broadcasts in that ramping body:

```text
.s:10211 vbroadcastss .LCPI5_22, %ymm8  ; DB_PER_OCTAVE (0x40c0a8c1)
.s:10250 vbroadcastss .LCPI5_28, %ymm4  ; OCTAVES_PER_DB (0x3e2a152d)
```

The stationary suffix begins at `.LBB5_91` (`.s:11081`) and its loop is
`.LBB5_93` at `.s:11198`, with backedge `.s:11913-11915`:

```text
.s:11493 vbroadcastss .LCPI5_22, %ymm7  ; DB_PER_OCTAVE
.s:11626 vbroadcastss .LCPI5_28, %ymm1  ; OCTAVES_PER_DB
```

The pool values are verified at `.s:9477-9494` and match the source bit
patterns. No other W8 candidate is retained.

## Native scalar

The scalar `process_block` starts at `.s:17044-17047`. A ramping frame body
is `.LBB6_38` at `.s:17527`, with the ramp/control boundary at
`.s:18457-18464`. The mapped constants are folded scalar memory operands:

```text
.s:17754 vmulss .LCPI6_19, %xmm4, %xmm4  ; DB_PER_OCTAVE (0x40c0a8c1)
.s:17800 vmulss .LCPI6_25, %xmm14, %xmm4 ; OCTAVES_PER_DB (0x3e2a152d)
```

The stationary suffix enters at `.LBB6_145` (`.s:21310`) and the selected
frame path begins at `.LBB6_148` (`.s:21478`). The same folded operands recur
in that path:

```text
.s:21690 vmulss .LCPI6_19, %xmm4, %xmm4  ; DB_PER_OCTAVE
.s:21736 vmulss .LCPI6_25, %xmm4, %xmm6  ; OCTAVES_PER_DB
.s:22066 vmulss .LCPI6_19, %xmm6, %xmm6  ; DB_PER_OCTAVE
.s:22115 vmulss .LCPI6_25, %xmm6, %xmm5  ; OCTAVES_PER_DB
```

The pool values are verified at `.s:17010-17023`. No other scalar candidate
is retained.

## Wasm scalar

The scalar `process_block` symbol begins at `.s:146920`. The ramping parameter
loop is `.LBB6_5` at `.s:147170-147173`; the mapped constants in its first
frame path are:

```text
.s:147773 f32.const 0x1.815182p2  ; DB_PER_OCTAVE
.s:147853 f32.const 0x1.542a5ap-3 ; OCTAVES_PER_DB
```

The stationary frame loop is `.LBB6_32` at `.s:148650-148653`, with backedge
`.s:149738-149756`. Its corresponding mapped constants are:

```text
.s:149497 f32.const 0x1.815182p2  ; DB_PER_OCTAVE
.s:149577 f32.const 0x1.542a5ap-3 ; OCTAVES_PER_DB
```

`0x1.815182p2` is `DB_PER_OCTAVE`, not a math-kernel constant. No other Wasm
scalar candidate is retained.

## Wasm simd128 W4

The W4 ramping loop is `.LBB4_5` at `.s:1312-1316`, with backedge
`.s:3775-3791`. Its mapped vector constants are:

```text
.s:3076-3078 v128.const 0x1.815182p2, ...  ; DB_PER_OCTAVE
.s:3123-3126 v128.const 0x1.542a5ap-3, ... ; OCTAVES_PER_DB
```

The stationary loop is `.LBB4_128` at `.s:3958-3963`, with end/backedge
`.s:4950-4965`. The same constants recur:

```text
.s:4286-4288 v128.const 0x1.815182p2, ...  ; DB_PER_OCTAVE
.s:4333-4336 v128.const 0x1.542a5ap-3, ... ; OCTAVES_PER_DB
```

No other Wasm SIMD candidate is retained.
