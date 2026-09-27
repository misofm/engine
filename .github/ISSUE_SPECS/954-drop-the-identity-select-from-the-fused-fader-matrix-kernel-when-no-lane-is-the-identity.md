# Drop the identity select from the fused fader-matrix kernel when no lane is the identity

## Product outcome

#944 gave the settled, unpaired pan matrix a select-free arm when no lane of a bank is the exact identity, and the gain/pan row moved from 14.1 to 11.1 us per block (every digest unchanged). The #881 verification found that the default web boot does not take that path: between-render-calls delivery fuses each cohort's fader and matrix into one stage, `fader_matrix_block` (`crates/lane/src/kernels/builtins.rs`), which #944 left untouched as a non-goal. The product's fused stage therefore still evaluates the per-lane identity select every frame (on x86 folded into a `vmaskmovps` masked store, which #944 measured at about 10 extra cycles per frame on Zen 3; on wasm a `bitselect`). Give the fused kernel the same select-free arm.

## Smallest closable slice

Mirror #944 on the fused pair: a `fader_matrix_block_without_identity` kernel (the second arm verbatim, no `L::select`), chosen once per call when `!L::mask_any(identity)` over all lanes, padding included, at every settled call site of the fused pair; the existing kernel stays the oracle. Same class statement as #944 (every non-NaN word unchanged, a NaN stays NaN; LLVM may commute the final add), same gates adapted: dev and release differential against the oracle at `f32`, `Simd4` and `Simd8`; a scenario test pinned on base; every console digest unchanged; red mutations; x86 disassembly showing plain stores; wasm census via the build script's cargo line.

## Rows it can move

`sixty_four_track_console_metered` (#881) and any plan prepared with between-render-calls delivery. No digest may move.

## Dependencies

#944 (merged in the optimisation batch), #881.

Found by the #881 attempt 1 verification.
