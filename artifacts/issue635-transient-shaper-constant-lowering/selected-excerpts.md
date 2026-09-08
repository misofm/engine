# Selected lowering excerpts

These excerpts are copied as compact claim-supporting records from the
payloads named in `payload-manifest.md`. Line numbers are physical lines in
the temporary payload. No complete compiler payload is retained.

## Caller map

The LLVM caller edges establish the stationary/ramping loop entry points for
the production callers:

```text
native .ll:9455  define ... Shaper<f32,1>::process_block
native .ll:5611  define ... Shaper<wide::f32x8,8>::process_block
native .ll:21295 call ... Shaper<f32,1>::process_block       ; PreparedNativeEffect::process
native .ll:21315 define ... PreparedTransientShaperBank<wide::f32x8,8>::process_bank
native .ll:21384 call ... Shaper<wide::f32x8,8>::process_block
native .ll:22846 define ... PreparedTransientShaperBank<wide::f32x8,8>::process_bank_mono

wasm-scalar .ll:44468 define ... Shaper<f32,1>::process_block
wasm-scalar .ll:56006 call ... Shaper<f32,1>::process_block ; PreparedNativeEffect::process

wasm-simd128 .ll:448   define ... Shaper<wide::f32x4,4>::process_block
wasm-simd128 .ll:24145 define ... PreparedTransientShaperBank<wide::f32x4,4>::process_bank
wasm-simd128 .ll:24214 call ... Shaper<wide::f32x4,4>::process_block
wasm-simd128 .ll:25418 define ... PreparedTransientShaperBank<wide::f32x4,4>::process_bank_mono
```

The corpus also instantiates scalar/W4/W8 `process_block` bodies, but the
public scalar process edge and the bank edges above are the caller evidence
used for the target conclusions.

The monomorphized `process_block` entry has the link-mode dispatch before the
loop lowering. For example, the native W8 LLVM body starts at line 5611 and
contains:

```text
%_9 = load i32, ptr %self+1428
switch i32 %_9, label %default.unreachable [ i32 1, label %bb4
                                                i32 2, label %bb3
                                                i32 3, label %bb2 ]
bb4: %spec.select = select ... <8 x float> splat (float 1.0) ...
```

The Wasm W4 body has the corresponding switch at `.ll:448` over the link
mode and begins its selected branch with the bypass mask. Thus the link modes
are actual callers/branches into the same ramping and stationary frame-loop
families, rather than an assumption based only on the Rust `match`.

## Native AVX2 W8

```text
.s:9535  .loc 1 495 ... vmovaps ...              ; locals/envelopes copied
.s:9553  vbroadcastss .LCPI5_1, %ymm4
.s:9554  vmovups %ymm4, 352(%rsp)                ; entry invariant (abs mask)
.s:9560  vbroadcastsd .LCPI5_6, %ymm0
.s:9563  vbroadcastsd .LCPI5_7, %ymm0
.s:9566  vbroadcastss .LCPI5_19, %ymm0
.s:9568  vbroadcastss .LCPI5_25, %ymm0        ; loop-entry zero
.s:9570  jmp .LBB5_56
.s:9573  .LBB5_90:                              ; ramping frame body
.s:9600  vbroadcastss .LCPI5_3, %ymm5
.s:9640  vbroadcastss .LCPI5_4, %ymm0
.s:9647  vbroadcastss .LCPI5_5, %ymm8
.s:9656  vbroadcastss .LCPI5_8, %ymm13
.s:9658  vbroadcastss .LCPI5_2, %ymm9
.s:9661  vbroadcastss .LCPI5_9, %ymm14
.s:9663  vbroadcastss .LCPI5_10, %ymm14
.s:9665  vbroadcastss .LCPI5_11, %ymm15
.s:9671  vbroadcastss .LCPI5_12, %ymm15
.s:9678  vbroadcastss .LCPI5_13, %ymm15
.s:9695  vbroadcastss .LCPI5_20, %ymm15
.s:9697  vbroadcastss .LCPI5_0, %ymm3
.s:9711  vbroadcastss .LCPI5_22, %ymm8
.s:9713  vbroadcastss .LCPI5_23, %ymm8
.s:9715  vbroadcastss .LCPI5_24, %ymm8
.s:9722  vbroadcastss .LCPI5_26, %ymm8
.s:9724  vbroadcastss .LCPI5_27, %ymm8
.s:9726  vbroadcastss .LCPI5_28, %ymm4
.s:9728  vbroadcastss .LCPI5_29, %ymm8
.s:9730  vbroadcastss .LCPI5_30, %ymm8
.s:10735 cmpq %r9, %r10; je .LBB5_91
.s:10736 .LBB5_56: incq %r10; ... ; next ramp frame

.s:11180 vbroadcastss .LCPI5_1, %ymm0; stationary setup
.s:11184 vbroadcastsd .LCPI5_6, %ymm0
.s:11186 vbroadcastsd .LCPI5_7, %ymm0
.s:11188 vbroadcastss .LCPI5_19, %ymm0
.s:11190 vbroadcastss .LCPI5_25, %ymm0
.s:11198 .LBB5_93:                              ; stationary frame body
.s:11214 vbroadcastss .LCPI5_3, %ymm5
.s:11222 vbroadcastss .LCPI5_4, %ymm5
.s:11225 vbroadcastss .LCPI5_5, %ymm5
.s:11234 vbroadcastss .LCPI5_8, %ymm5
.s:11236 vbroadcastss .LCPI5_2, %ymm0
.s:11239 vbroadcastss .LCPI5_9, %ymm5
.s:11241 vbroadcastss .LCPI5_10, %ymm5
.s:11243 vbroadcastss .LCPI5_11, %ymm5
.s:11915 jne .LBB5_93                           ; stationary backedge
```

The pool mapping is fixed by the source bit patterns: `.LCPI5_4` is `FLOOR`
(`0x322bcc77`), `.LCPI5_22` is `DB_PER_OCTAVE` (`0x40c0a8c1`),
`.LCPI5_23/.24` are `+/-24`, `.LCPI5_26/.27` are `+/-18`, `.LCPI5_28` is
`OCTAVES_PER_DB`, and `.LCPI5_25` is zero. The W8 ramping and stationary
frame bodies therefore re-broadcast these candidates in the loop. The
parameter/coefficient vectors copied before `.LBB5_93` are loop-entry
materialization; they are loaded from stack slots in the body rather than
recomputed from source splats.

## Native scalar

```text
.s:17527 .LBB6_38:                            ; scalar frame body
.s:17534 vbroadcastss .LCPI6_0, %xmm4          ; abs mask in frame
.s:17573 vmovss .LCPI6_2, %xmm13
.s:17611 vmovss .LCPI6_3, %xmm4                ; FLOOR
.s:17649 vmulss .LCPI6_1, %xmm7, %xmm8
.s:17754 vmulss .LCPI6_19, %xmm4, %xmm4
.s:17757 vmovss .LCPI6_20, %xmm7
.s:17771 vmaxss .LCPI6_21, %xmm4, %xmm4
.s:17794 vminss .LCPI6_23, %xmm4, %xmm4
.s:17797 vmaxss .LCPI6_24, %xmm4, %xmm14
.s:17800 vmulss .LCPI6_25, %xmm14, %xmm4
.s:17803 vmaxss .LCPI6_26, %xmm4, %xmm4
.s:17806 vminss .LCPI6_27, %xmm4, %xmm4
.s:18214 .LBB6_46:                            ; next ramp prefix/control path
.s:18459 incq %r13; cmpq %r13,%r9; je .LBB6_145

.s:21310 .LBB6_145:                            ; stationary suffix entry
.s:21327-.21347 loads current scalar state/ramps
.s:21349 vbroadcastss .LCPI6_0, %xmm10
.s:21350 vmovss .LCPI6_2, %xmm14
.s:21351 vmovss .LCPI6_1, %xmm15
.s:21357 jmp .LBB6_148
.s:21478 .LBB6_148:                            ; stationary frame body
.s:21480 vmovss (...,%r13,4), %xmm5
.s:21482 vandps %xmm5,%xmm10,%xmm4
.s:22121 vminss .LCPI6_27, %xmm5, %xmm5
.s:22134 jbe .LBB6_147
.s:21468 incq %r13; cmpq %r13,88(%rsp); je .LBB6_156
```

Native scalar uses folded scalar memory operands for several constants
(`vmovss`/`vmulss`/`vminss` directly from `.LCPI6_*`) instead of vector
broadcasts. The loop-entry loads at `.LBB6_145` are current state and
coefficient materialization. The repeated frame operations remain present in
the stationary suffix and ramping body; whether a scalar constant is loaded
from the pool or held in a register is a backend choice, not a source-level
residual by itself.

## Wasm scalar

```text
.s:146920 _...ShaperfKj1_E13process_block...    ; scalar public body
.s:148650 .LBB6_32: loop                      ; stationary frame loop
.s:148654 local.get 6; f32.load 0              ; frame input
.s:148665 f32.const 0x0p0                      ; zero candidate in body
.s:149346 f32.const -0x1.0000fep23             ; math kernel constant
.s:149497 f32.const 0x1.815182p2               ; math kernel constant
.s:149507 f32.const 0x1.8p4                    ; +24 clamp
.s:149518 f32.const -0x1.8p4                   ; -24 clamp
.s:149739 local.get 7; i32.const 4; i32.add
.s:149752 local.tee 8; br_if 0
.s:149754 .LBB6_33: end_loop
```

The scalar Wasm body uses scalar `f32.const` operands in the frame loop. The
LLVM/assembly also shows the same loop-entry load pattern for the current
parameter state before `.LBB6_32`; no audio path was executed.

## Wasm simd128 W4

```text
.s:1312 .LBB4_5: loop                         ; ramping prefix
.s:2718 .loc ... input v128.load
.s:2729 v128.const 0x1.fcp6, ...              ; math kernel
.s:2737 v128.const -0x1.2p4, ...
.s:2741 v128.const 0x1.2p4, ...
.s:2746 v128.const -0x1.8p4, ...              ; -24 clamp
.s:2750 v128.const 0x1.8p4, ...               ; +24 clamp
.s:2758 v128.const 0x1.5798eep-27, ...
.s:3780 local.set 6; local.get 7; i32.const -1; i32.add; br_if 0
.s:3790 .LBB4_126: end_loop

.s:3958 .LBB4_128: loop                      ; stationary suffix
.s:3963 v128.load 0
.s:3970 v128.const 0x1.fcp6, ...
.s:3978 v128.const -0x1.2p4, ...
.s:3982 v128.const 0x1.2p4, ...
.s:3987 v128.const -0x1.8p4, ...
.s:3991 v128.const 0x1.8p4, ...
.s:3995 v128.const 0x1p-126, ...
.s:3999 v128.const 0x1.5798eep-27, ...
.s:4950 local.get 6; i32.const 16; i32.add
.s:4963 local.tee 7; br_if 0
.s:4964 .LBB4_129: end_loop
```

Wasm SIMD emits vector constants in both loop bodies. The W4 parameter and
coefficient locals loaded before `.LBB4_128` are loop-entry materialization;
the frame constants shown inside that loop are repeated broadcasts/constants.
