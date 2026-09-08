# Issue 621 current lowering capture

Capture date: 2026-09-08 UTC

This is the single Astra LOW-authorized, untimed current-source compile/disassembly capture for issue #621. It is evidence only. No Rust, issue-spec, manifest, lockfile, dependency, host, lane, math, workflow, benchmark, timing, or artifact-promotion file was changed.

## Identity and configuration

- Worktree: `/home/bl/misofm/engine-limiter-detector-bounds`
- Branch: `codex/limiter-detector-chunk-bounds`
- HEAD at capture: `c46e7bc8f1e0ecb9b62b31d78f6d5c68558913ef`
- Limiter source: `crates/true-peak-limiter/src/lib.rs`, SHA-256 `677a5596a305039ddbed39d634cde80e90bf99d39439896b1d58a4a539d73b55`
- `rust-toolchain.toml`: SHA-256 `85a45cac04c296adac076f8f0609ca8f4c8ca658957f1f24cbd1e054b6cc44e0`
- `.cargo/config.toml`: SHA-256 `03b0fbd88c069abb0a8fbdca5921ba6a9899298291fe087977b509a29ebb7d0e`
- `Cargo.toml`: SHA-256 `471660ddca768e90ec87550807e140b4754454509f6f5f59986a80764b53effb`
- `Cargo.lock`: SHA-256 `92db9698cc062bd5ccc0a90f0ee43f051d77ce7e52f785ce9718fae3f400b753`
- Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`, LLVM `22.1.6`; `cargo 1.97.1 (c980f4866 2026-06-30)`; `wasm-objdump 1.0.34`.
- Native configuration is the repository x86-64-v3 configuration (`-C target-feature=+avx2,+fma`); the Cargo release profile is the repository profile (`lto = "fat"`, `codegen-units = 1`).
- Every Cargo target directory was outside the worktree under `/tmp/issue621-*`. No target or dependency artifact was copied into this directory.

## Exact primary capture commands

Each command exited 0. `cargo.stdout`, `cargo.stderr`, and `cargo.status` are retained in the corresponding directory. `--emit=asm,llvm-ir` produced the complete current release lowering used for symbol selection and review. The full compiler output was subsequently removed from durable delivery under tracker ruling `80f6e715`; its original byte counts and SHA-256 identities remain in `provenance.json`.

```sh
env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-native-target cargo rustc --locked --release -p true-peak-limiter --lib -- --emit=asm,llvm-ir

env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-wasm-scalar-target RUSTFLAGS='-C target-feature=-simd128' cargo rustc --locked --release --target wasm32-unknown-unknown -p true-peak-limiter --lib -- --emit=asm,llvm-ir

env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-wasm-simd128-target RUSTFLAGS='-C target-feature=+simd128' cargo rustc --locked --release --target wasm32-unknown-unknown -p true-peak-limiter --lib -- --emit=asm,llvm-ir
```

The full files used for the review were:

- `native/true_peak_limiter-61bcf0f0b2475b36.ll` and `.s`
- `wasm-scalar/true_peak_limiter-13565f343cd208d0.ll` and `.s`
- `wasm-simd128/true_peak_limiter-c44b6d723c91aa0c.ll` and `.s`

Their selected complete function intervals and original source line ranges/hashes remain recorded in `selected/manifest.json`. The focused, line-addressed assembly excerpts that support the lowering map are retained in `lowering-map-excerpts.txt`; `lowering-map-excerpts.json` records each logical source, original line range, full-source identity, and raw excerpt hash.

## Wasm object/disassembly note

Cargo's release `--emit=obj` output in this configuration is LLVM bitcode rather than a WebAssembly object. The attempted `wasm-objdump` checks on those temporary outputs therefore exited 1 with `bad magic value`; their Cargo command output, status, and stderr remain under `wasm-scalar/` and `wasm-simd128/` for audit. Additional no-LTO/no-embed-bitcode attempt records likewise remain and are not credited as disassembly.

A direct scalar `rustc` object capture, using already-built repository dependency rlibs and the same release optimization intent, exited 0; `wasm-objdump -x -d /tmp/issue621-wasm-scalar-direct.o` also exited 0. Its complete disassembly was inspected and contained no `v128.` or `f32x4.` instructions; the durable record retains its original size and SHA-256 rather than the full disassembly. The direct SIMD rustc attempt could not resolve the lane rlib in the temporary dependency directory and exited 1; its two small raw stderr streams remain losslessly archived. SIMD portability is credited from the successful Cargo SIMD release leg and the inspected compiler output: the SIMD assembly contained 4,752 `v128.`/`f32x4.` opcode occurrences, while the scalar assembly contained none.

## Current source-to-lowering map

The source shape under capture is `lib.rs:1750-1765`: `detector_chunk` computes `base = (chunk + frame) * width`, loads `L::load(&io[base..])`, stores the detector result to `peaks[frame * width..]`, and writes the twelve-word history back once after the frame loop. The following locations refer to the original selected assembly identities recorded in the manifests and reproduced by original line number in `lowering-map-excerpts.txt`. They separate the repeated loop-internal input slice check from loop termination, chunk/tail controls, arithmetic/store, and post-loop history writeback.

| Current path | Repeated input window check inside frame loop | Frame loop backedge | Separate chunk/tail control | Arithmetic/store and history writeback |
|---|---|---|---|---|
| Native scalar | `selected/native-scalar-core.s:841-851` (`.LBB36_265`; `leaq` plus `cmpq` and `ja`) and the second channel at `1384-1394` | First loop `1217-1221`; second loop `1760-1764` | Chunk span setup at `671-695` (`-32`, `cmpq $1`, `cmpq $32`); these are outside the detector frame loop | Peak store `1201-1202`; history writeback begins at `1226` and again after the second loop |
| Native x86-64-v3 W8 dual | `selected/native-w8-core.s:698-704` (`.LBB34_253`) and second channel `1225-1231` | First loop `1059-1070`; second `1586-1597` | Per-iteration vector-tail check `707-708` / `1233-1235` (`cmpq $7`), distinct from the input window check | Peak store `1056-1057` (second store at `1583-1584`); history writeback starts at `1602` |
| Native x86-64-v3 W8 mono | `selected/native-w8-process-bank-inner.s:871-879` (`.LBB1_76`) | `1229-1240` | Vector-tail check `881-883` (`cmpq $7`) | Peak store `1226-1227`; history writeback starts at `1252` |
| Wasm scalar | `selected/wasm-scalar-scalar-core.s:1626-1655` (`.LBB22_91`, `i32.gt_u` then `br_if`) | `2116-2146` (`i32` frame increment and `br_if`); loop ends at `2147-2149` | The equality/control branch at `1657-1661` is separate from the input bound guard | `f32.store` at `2104-2105`; history writeback starts at `2151` |
| Wasm SIMD128 W4 | `selected/wasm-simd128-w4-core.s:1600-1631` (`.LBB27_91`, `i32.gt_u` then `br_if`) | `2123-2129`; loop ends at `2130-2132` | W4 lane/tail check `1633-1647` (`i32.gt_u` against 3 and `slice_index_fail`) is separate | `v128.load`/`f32x4.mul` at `1665-1674`, `v128.store` at `2083-2084`; history writeback starts at `2134` |

The native scalar and native W8 bodies are the supported native premise. The scalar and W4 Wasm bodies show the same source access shape under the required portability legs. The current repeated input-slice bounds mechanism is therefore **proven present** in the supported native scalar and x86-64-v3 W8 detector callers, with corresponding scalar/W4 Wasm lowering. This record makes no claim about a historical number of comparisons, cycles, percentage speedup, or sound-quality change. It is applicability evidence only; no source implementation attempt was made.

## Completeness and hygiene

- Commands, exit statuses, ordinary stdout/stderr, toolchain identity, source identity, and original full-output hashes remain for all three primary legs and every object/disassembly attempt. Full compiler IR and assembly are omitted from durable delivery under tracker ruling `80f6e715`.
- `selected/manifest.json` records every selected function's original source file, line interval, byte count, and SHA-256; `selected/extract.status` is 0. `lowering-map-excerpts.json` makes the surviving derived excerpts independently traceable to those raw identities.
- The artifact tree contains no `target/` directory, Cargo registry/dependency tree, private key, token, password, or unrelated generated source. Temporary targets and the direct scalar object remain under `/tmp` and are not repository changes.
- This capture remains tied to the authorized source HEAD; later evidence-only and integration commits do not change that identity.

## Durable delivery form

Tracker ruling `80f6e715` excludes full compiler IR and redundant assembly from default-branch evidence. `provenance.json` preserves every retired file's logical path, uncompressed byte count, and SHA-256. `lowering-map-excerpts.txt` contains only the assembly lines needed for this review, with original line numbers; its companion JSON preserves each raw excerpt hash. The display form removes line endings and trailing horizontal whitespace, so the recorded raw hashes remain authoritative.

The two small failed auxiliary stderr streams are stored byte-for-byte in `raw-emitted-output.tar.gz`. `raw-emitted-output.members.json` records their logical paths, sizes, and hashes. The archive uses deterministic zeroed ownership and timestamps.
