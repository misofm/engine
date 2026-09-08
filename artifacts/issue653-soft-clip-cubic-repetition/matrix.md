# Repetition matrix

The source calls are `crates/soft-clip/src/kernel.rs:199` (even) and `:201` (odd).
A count is greater than one only for identical actual values with separate
materialization instructions within the same frame-loop iteration. The `count`
column is the per-iteration count; loop-entry values are labeled separately and
have per-iteration count zero. Pool entries without loads, one-load/local reuse
and different folded values are excluded.

| shape | actual value | even site | odd site | count | classification and source ownership |
|---|---:|---|---|---:|---|
| native AVX2 W8 | `-1`, bits `0xbf800000` | `.s:3968`, `vbroadcastss .LCPI14_3` | `.s:4013`, `vbroadcastss .LCPI14_3` | 2 | Separate per-frame broadcasts in the two calls. Same threshold from the inlined helper; source-owned repetition candidate without arithmetic movement. |
| native AVX2 W8 | folded `-3`, bits `0xc0400000` | `.s:3972`, `vbroadcastss .LCPI14_5` | `.s:4023`, `vbroadcastss .LCPI14_5` | 2 | Separate per-frame broadcasts. This is the compiler's signed divisor for source `u - p1/3`; it is an identical folded operand in both calls. |
| native AVX2 W8 | `+1`, bits `0x3f800000` | excluded: `%ymm5` loaded once before loop at `.s:3269` | excluded: reuses `%ymm5` | 0 (loop-entry=1) | Loop-entry value, no within-iteration repetition. |
| native AVX2 W8 | source `-2/3`, bits `0xbf2aaaab` | `.s:3976`, `.LCPI14_6` | excluded: odd emits folded `-1/3` | 1 | Different actual odd value; `-1/3` is not a repeat of `-2/3`. |
| native AVX2 W8 | source `+2/3`, bits `0x3f2aaaab` | `.s:3978`, `.LCPI14_7` | excluded: odd emits folded `+1/3` | 1 | Different actual odd value; `+1/3` is not a repeat of `+2/3`. |
| native AVX2 W8 | folded odd `-1/3`, bits `0xbeaaaaab` | excluded | `.s:4056`, `.LCPI14_9` | 1 | Different actual value from source `-2/3`; one odd-site materialization. |
| native AVX2 W8 | folded odd `+1/3`, bits `0x3eaaaaab` | excluded | `.s:4058`, `.LCPI14_10` | 1 | Different actual value from source `+2/3`; one odd-site materialization. |
| native scalar | `+1`, bits `0x3f800000` | `.s:5870`, `.LCPI18_20` load | excluded: register reuse | 1 | One load before the even body is reused by odd. |
| native scalar | `-1`, bits `0xbf800000` | `.s:5871`, `.LCPI18_18` load | excluded: register reuse | 1 | One load before the even body is reused by odd. |
| native scalar | folded `-3`, bits `0xc0400000` | `.s:5872`, `.LCPI18_17` load | excluded: `%xmm0` reuse at `.s:5993` | 1 | One folded divisor load reused by odd. |
| native scalar | source `-2/3`, bits `0xbf2aaaab` | `.s:5933`, `.LCPI18_19` | excluded: odd emits folded `-1/3` | 1 | Different actual odd value. |
| native scalar | source `+2/3`, bits `0x3f2aaaab` | `.s:5938`, `.LCPI18_21` | excluded: odd emits folded `+1/3` | 1 | Different actual odd value. |
| native scalar | folded odd `-1/3`, bits `0xbeaaaaab` | excluded | `.s:6013`, `.LCPI18_23` | 1 | Different actual value from source `-2/3`; one odd-site materialization. |
| native scalar | folded odd `+1/3`, bits `0x3eaaaaab` | excluded | `.s:6017`, `.LCPI18_24` | 1 | Different actual value from source `+2/3`; one odd-site materialization. |
| Wasm scalar | `-1` | `.s:4789`, `f32.const -0x1p0` | `.s:4911`, `f32.const -0x1p0` | 2 | Separate per-frame immediates in even and odd calls; same helper threshold. |
| Wasm scalar | `+1` | `.s:4793`, `f32.const 0x1p0` | `.s:4917`, `f32.const 0x1p0` | 2 | Separate per-frame immediates in even and odd calls; same helper threshold. |
| Wasm scalar | folded `-3` | `.s:4785`, `f32.const -0x1.8p1` | `.s:4901`, `f32.const -0x1.8p1` | 2 | Separate per-frame immediates for the signed divisor folded from source divide/subtract order. |
| Wasm scalar | source `-2/3`, bits `0xbf2aaaab` | `.s:4615`, `f32.const -0x1.555556p-1`; LLVM `.ll:2416` select operand | excluded: odd emits folded `-1/3` | 1 | One per-frame stack materialization before interpolation work in `.LBB11_22`, consumed once by the even select; odd actual value differs. |
| Wasm scalar | source `+2/3`, bits `0x3f2aaaab` | `.s:4614`, `f32.const 0x1.555556p-1`; LLVM `.ll:2417` select operand | excluded: odd emits folded `+1/3` | 1 | One per-frame stack materialization before interpolation work in `.LBB11_22`, consumed once by the even select; odd actual value differs. |
| Wasm scalar | folded odd `-1/3`, bits `0xbeaaaaab` | excluded | `.s:4885`, `f32.const -0x1.555556p-2` | 1 | Different actual value from source `-2/3`; one odd-site materialization. |
| Wasm scalar | folded odd `+1/3`, bits `0x3eaaaaab` | excluded | `.s:4884`, `f32.const 0x1.555556p-2` | 1 | Different actual value from source `+2/3`; one odd-site materialization. |
| Wasm W4 | `+2/3` | `.s:5873`, `v128.const`, local 62 | excluded: local 62 reuse at `.s:6000` | 1 | One per-frame materialization, then local reuse. |
| Wasm W4 | `-2/3` | `.s:5875`, `v128.const`, local 63 | excluded: local 63 reuse at `.s:6001` | 1 | One per-frame materialization, then local reuse. |
| Wasm W4 | folded `-3` | `.s:5883`, `v128.const`, local 64 | excluded: local 64 reuse at `.s:6018` | 1 | One per-frame materialization, then local reuse. |
| Wasm W4 | `-1` | `.s:5888`, `v128.const`, local 65 | excluded: local 65 reuse at `.s:6026` | 1 | One per-frame materialization, then local reuse. |
| Wasm W4 | `+1` | `.s:5893`, `v128.const`, local 66 | excluded: local 66 reuse at `.s:6034` | 1 | One per-frame materialization, then local reuse. |

Native pool entries not loaded at a site do not count. Native odd `±1/3`
(`.s:4056/.4058` and `.s:6013/.6017`) have bits `0xbeaaaaab/0x3eaaaaab` and
are different actual values from source `±2/3`. Wasm scalar's even `±2/3`
materializations at `.s:4614/.4615` are stack operands used once; its odd
`±1/3` (`.s:4884/.4885`) have bits `0x3eaaaaab/0xbeaaaaab`. W4 has no folded
odd `±1/3`; its `±2/3` locals are reused. All are excluded from repeat counts.
