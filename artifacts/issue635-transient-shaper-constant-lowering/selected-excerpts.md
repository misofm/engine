# Selected lowering excerpts

These are compact excerpts from the temporary payloads named in
`payload-manifest.md`. Every physical line reference below was checked against
the retained payload in `/tmp`; complete `.s` and `.ll` payloads remain
outside the repository.

## Caller and link-mode map

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
load link mode and switch over `1`, `2`, and `3` before the loop body. This
ties the three link specializations to the same `process_block` loop families.

## Native AVX2 W8

The W8 function starts at `.s:9521-9524`. Its ramping-prefix setup is at
`.s:9958-9975`, and the ramping frame body begins at `.LBB5_90` (`.s:9980`).
Mapped FX4 constants inside that body are:

```text
.s:10030 vbroadcastss .LCPI5_3,  %ymm5  ; FLOOR
.s:10093 vbroadcastss .LCPI5_2,  %ymm9  ; 0.5 constant occurrence (link branch not retained)
.s:10211 vbroadcastss .LCPI5_22, %ymm8  ; DB_PER_OCTAVE
.s:10215 vbroadcastss .LCPI5_23, %ymm8  ; +24
.s:10219 vbroadcastss .LCPI5_24, %ymm8  ; -24
.s:10242 vbroadcastss .LCPI5_26, %ymm8  ; +18
.s:10246 vbroadcastss .LCPI5_27, %ymm8  ; -18
.s:10250 vbroadcastss .LCPI5_28, %ymm4  ; OCTAVES_PER_DB
.s:10740 cmpq %r9,%r10; je .LBB5_91
.s:10742 .LBB5_56                 ; next ramp frame
```

The stationary suffix begins at `.LBB5_91` (`.s:11081`); current-state and
coefficient packing occupy `.s:11096-11177`, with loop-entry constants at
`.s:11180-11191`. The stationary frame loop is `.LBB5_93` at `.s:11198`:

```text
.s:11256 vbroadcastss .LCPI5_3,  %ymm5  ; FLOOR in stationary body
.s:11493 vbroadcastss .LCPI5_22, %ymm7  ; DB_PER_OCTAVE
.s:11497 vbroadcastss .LCPI5_23, %ymm9  ; +24
.s:11508 vbroadcastss .LCPI5_24, %ymm2  ; -24
.s:11601 vbroadcastss .LCPI5_26, %ymm15 ; +18
.s:11615 vbroadcastss .LCPI5_27, %ymm3  ; -18
.s:11626 vbroadcastss .LCPI5_28, %ymm1  ; OCTAVES_PER_DB
.s:11913 addq $32,%r9; decq %rax
.s:11915 jne .LBB5_93             ; stationary backedge
```

The pool mapping is verified from `.s:9441-9514` and source bit patterns:
`.LCPI5_3` is `FLOOR` (`0x322bcc77`), `.LCPI5_22` is `DB_PER_OCTAVE`
(`0x40c0a8c1`), `.LCPI5_23/.24` are `+/-24`, `.LCPI5_26/.27` are `+/-18`,
and `.LCPI5_28` is `OCTAVES_PER_DB`. `.LCPI5_2` is `0.5`, but its link-mode
branch attribution is not retained. No average-link or spill/reload conclusion
is retained: the shown stack traffic is not individually mapped to a constant
candidate.

## Native scalar

The scalar `process_block` starts at `.s:17044-17047`. A mapped scalar frame
body is `.LBB6_38` at `.s:17527`; its folded scalar operands include:

```text
.s:17611 vmovss .LCPI6_3,  %xmm4          ; FLOOR
.s:17754 vmulss .LCPI6_19, %xmm4, %xmm4    ; DB_PER_OCTAVE
.s:17794 vminss .LCPI6_23, %xmm4, %xmm4    ; +24 clamp operand
.s:17797 vmaxss .LCPI6_24, %xmm4, %xmm14   ; -24 clamp operand
.s:17800 vmulss .LCPI6_25, %xmm14, %xmm4   ; OCTAVES_PER_DB
.s:17803 vmaxss .LCPI6_26, %xmm4, %xmm4    ; +18 clamp operand
.s:17806 vminss .LCPI6_27, %xmm4, %xmm4    ; -18 clamp operand
```

The scalar ramp/control path reaches `.LBB6_46` at `.s:18214` and its frame
count boundary is `.s:18457-18464`. The stationary suffix entry is
`.LBB6_145` at `.s:21310`; scalar current-state loads are `.s:21324-21347`
and the selected frame path begins at `.LBB6_148` (`.s:21478`). Its mapped
clamp/multiply operands are `.s:22115-22121`. The retained native-scalar
conclusion is limited to folded operands in these individually located frame
paths; no scalar spill/reload or average-link conclusion is retained.

## Wasm scalar

The scalar `process_block` symbol begins at `.s:146920`. The ramping parameter
loop is `.LBB6_5` at `.s:147170-147173`. The stationary frame loop is
`.LBB6_32` at `.s:148650-148653`, with its backedge at `.s:149738-149756`.
Mapped FX4 constants in that stationary body are:

```text
.s:149497 f32.const 0x1.815182p2   ; DB_PER_OCTAVE
.s:149507 f32.const 0x1.8p4         ; +CONTRAST_LIMIT_DB (+24)
.s:149518 f32.const -0x1.8p4        ; -CONTRAST_LIMIT_DB (-24)
.s:149561 f32.const 0x1.2p4         ; +SHAPE_LIMIT_DB (+18)
.s:149569 f32.const -0x1.2p4        ; -SHAPE_LIMIT_DB (-18)
.s:149577 f32.const 0x1.542a5ap-3   ; OCTAVES_PER_DB
.s:149754-149756 .LBB6_33 end_loop
```

The nearby `0x1.815182p2` is `DB_PER_OCTAVE`, not a math-kernel constant.
Other constants are omitted unless mapped to an FX4 candidate. No average-
link, zero-hoisting, or spill conclusion is retained from this excerpt.

## Wasm simd128 W4

The W4 ramping loop is `.LBB4_5` at `.s:1312-1316`, with its backedge at
`.s:3775-3791`. The mapped vector constants in that body are
`DB_PER_OCTAVE` at `.s:3076-3078` and `OCTAVES_PER_DB` at `.s:3123-3126`.
The stationary loop is `.LBB4_128` at `.s:3958-3963`; selected mapped values
inside its body are:

```text
.s:4286 v128.const 0x1.815182p2, ...       ; DB_PER_OCTAVE
.s:4333-4336 v128.const 0x1.542a5ap-3, ... ; OCTAVES_PER_DB
.s:4950-4963 pointer/count updates; br_if 0
.s:4964-4965 .LBB4_129 end_loop
```

The `+/-24` and `+/-18` vector constants occur in the same families
(`.s:2737-2750` and `.s:3978-3991`), but the temporary assembly contains
multiple inlined math paths; they are not claimed as individually attributable
here. The retained W4 conclusion is limited to the individually mapped
`DB_PER_OCTAVE` and `OCTAVES_PER_DB` vector constants in both loop families.
