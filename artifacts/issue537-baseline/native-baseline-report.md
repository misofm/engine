# Issue #537 unchanged native production baseline (DYN3)

## Boundary and status

This is the required baseline-first capture before any source edit. The worktree was clean at capture and remains clean. No source, Cargo, configuration, test, CI, Git, or GitHub files were changed. This report does not make an implementation or acceptance decision and contains no timing or speedup claim.

- Worktree: `/home/bl/misofm/engine-multiband-detector-access`
- Branch: `codex/multiband-detector-access`
- Captured HEAD: `69acb3cb20bc70395e0a93ea940f2811300d7ad2`
- Captured `git status --porcelain=v1`: empty
- Capture helper: `/tmp/issue537-baseline-capture.py` (SHA-256 `ca4ff975cff95f3feffc19d7c825ccb4ed895ff6c18385270dfa185e71c53c5f`)
- Selection helper: `/tmp/issue537-extract-selected.py` (SHA-256 `5340bcd1c6c44c4d3520caab0b59abb9c0d97b58651783382cd13041551a2b50`)
- Capture directory: `/tmp/issue537-baseline`
- Isolated target: `/tmp/issue537-baseline-target`

## Exact capture

The capture helper ran this exact command from the worktree root:

```text
python3 /tmp/issue537-baseline-capture.py native-release cargo rustc --locked --release -p multiband-compressor --lib -- --emit=asm,llvm-ir
```

The helper's recorded child argv is:

```text
cargo rustc --locked --release -p multiband-compressor --lib -- --emit=asm,llvm-ir
```

The command metadata, stdout, stderr, and numeric status are preserved separately:

- metadata: `/tmp/issue537-baseline/native-release.command.json` (SHA-256 `a5bae8a877382baaa1f022e2f06de1938727872e23d6f678d7975dd27f30db3b`, 2300 bytes)
- stdout: `/tmp/issue537-baseline/native-release.stdout` (SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`, 0 bytes)
- stderr: `/tmp/issue537-baseline/native-release.stderr` (SHA-256 `1cad23254d1d93469e4b2d985b5843a722f83732b42471771f6a704a1dcba5a2`, 765 bytes)
- status: `/tmp/issue537-baseline/native-release.status` (contents `0`, SHA-256 `9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa`)

The captured environment records `PATH` with `/home/bl/.cargo/bin` prepended, `RUSTFLAGS` unset (`null`), and `CARGO_TARGET_DIR=/tmp/issue537-baseline-target`. The pinned toolchain output records rustc/cargo `1.97.1`, rustc commit `8bab26f4f68e0e26f0bb7960be334d5b520ea452`, LLVM `22.1.6`, and rustup's override from this worktree's `rust-toolchain.toml`.

The release profile and target policy were read from the captured source/config identities: `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`, `debug = 1`; `.cargo/config.toml` supplies exactly `-C target-feature=+avx2,+fma` for x86_64. No extra RUSTFLAGS were present.

Captured source/config identities are recorded in the metadata JSON. Their SHA-256 / Git blob pairs are:

| path | SHA-256 | Git blob |
| --- | --- | --- |
| `crates/multiband-compressor/src/lib.rs` | `1058d6043d38c80f5ee3f61e794b5e35a8a4b35bf447661ed077c46739879779` | `c0419d76efff1b1ca6bf7f67b5fabcbe5adc7223` |
| `Cargo.toml` | `471660ddca768e90ec87550807e140b4754454509f6f5f59986a80764b53effb` | `23944595dc9167dbb5a3a2ced7234dfdbadf3d0d` |
| `Cargo.lock` | `ef85bfaac8b4df80b651fa89e5e9a67bfef141b58076bdaac513044161b0b853` | `1b74fb016a04a090d2d1d37ea15fd2917fb85491` |
| `.cargo/config.toml` | `03b0fbd88c069abb0a8fbdca5921ba6a9899298291fe087977b509a29ebb7d0e` | `de2180972c37406f1f16913901bd1d4fc2e660e3` |
| `rust-toolchain.toml` | `85a45cac04c296adac076f8f0609ca8f4c8ca658957f1f24cbd1e054b6cc44e0` | `6b6739839ba23906921aeb4c268d275dcc459fb6` |

## Original compiler outputs

The compiler emitted these original files into the isolated target. The original files are retained and were not rewritten after selection.

| output | bytes | SHA-256 |
| --- | ---: | --- |
| `/tmp/issue537-baseline-target/release/deps/multiband_compressor-524a987dfea296bf.ll` | 7,688,799 | `fbc3e01eac068449ab3589bd33e74a155dc60c14ab46a22070cb971284be12dd` |
| `/tmp/issue537-baseline-target/release/deps/multiband_compressor-524a987dfea296bf.s` | 1,986,754 | `70745fbb77d26f4f7d2ca6031f3542c3e39e6d01d584e0964274aeee58ffa6e0` |

## Complete selected production callers

Selection was performed from the exact original files by `/tmp/issue537-extract-selected.py`; it selects one exact emitted function for each caller and writes complete bodies through the closing `}` / `.size`. The manifest is `/tmp/issue537-baseline/selected/manifest.json` (SHA-256 `33915ded27438ae4b44f0fb878504e6fde7951886f5650aaec46559d32eed30f`). The static extraction's generated stdout was the manifest path and six label/range/hash lines; no compiler or source command was rerun for selection. Complete output identities are:

| actual caller | kind | original line range | selected file | bytes | SHA-256 |
| --- | --- | ---: | --- | ---: | --- |
| `PreparedMultibandCompressor::process` (W1 scalar) | LLVM | 8663–15496 | `selected/process-scalar.ll` | 587,244 | `8e6b7fd64e1763224c029fa6ae714242c6a4470671de8913d8db9eabff384f60` |
| same | ASM | 11837–21863 | `selected/process-scalar.s` | 209,072 | `8181969f634aafbed2a3b1169bcea931c04af1be32526e703a57d4e715ea2caa` |
| `PreparedMultibandCompressorBank<wide::f32x4, 4>::process_bank` (W4) | LLVM | 15508–26383 | `selected/process-bank-w4.ll` | 1,041,654 | `1931a3e6fde94d99e7ac3d36db808d9c216e7a09cde84bb269ef865abe1966df` |
| same | ASM | 21981–35088 | `selected/process-bank-w4.s` | 274,477 | `4f509883f16d9391c9ea2cb5264f33a6cb48ab50e81525631cafb4b610b1cfe2` |
| `PreparedMultibandCompressorBank<wide::f32x8, 8>::process_bank` (W8) | LLVM | 27169–41879 | `selected/process-bank-w8.ll` | 1,443,613 | `85a76cc379781eb06a9cd60aa6cbd4dea9fbd4c8a1a6f9353119336b4a724184` |
| same | ASM | 36331–55203 | `selected/process-bank-w8.s` | 383,220 | `469cffe1c27f52ff3af46fbb0849eec660b9b0ef2ec4356ffa2ae937a87d5b25` |

W4 is an emitted generic instantiation in this native release artifact; it is not evidence of a native W4 dispatch target. W8 is the supported native AVX2 bank instantiation. W1 is the scalar prepared production caller. There is no separate `detector_tap` symbol in these selected LLVM callers: the `#[inline(always)]` helper is inlined, so the selected complete callers are the relevant production evidence.

## Access and packing observations

The source seam is `detector_tap` at `crates/multiband-compressor/src/lib.rs:864–880`, with the actual four calls at lines `1005–1012`. The source computes each lane's wrapped row and reads `ring[row * W + track]`; the ring is row-major by bank width. These observations follow the emitted address arithmetic and load instructions, with debug locations used only to orient the source callsite.

### W1 scalar

In the first complete scalar render path, selected ASM lines `1188–1237` form the wrapped detector indices and perform the low-ring detector loads at lines `1234` and `1237` (`vmovss` from two dynamic ring bases). The high-ring pair appears at lines `1415–1424`. The same scalar structure is repeated in the other emitted branch variants in the complete selected function. Width one has no cross-lane packing opportunity: every detector read is one scalar `vmovss`; a uniform offset is naturally the existing scalar path.

### W4 emitted bank

In the first complete W4 detector block, selected ASM lines `4192–4359` show four independent `cursor + offset` additions, compare/subtract wrap operations, byte scaling, and per-index bounds checks for the near and far channels. The low/high detector words are assembled at lines `4461–4478` with `vmovd` plus three `vpinsrd` loads per four-word vector. For example, lines `4462–4465` load four different addresses from one ring base using `%r8`, `%rbp`, `%rbx`, and `%rcx`; lines `4468–4478` do the same for the second ring. The later branch variants retain the same shape.

This is real residual access/packing work in the emitted W4 body. When the four offsets are equal, the four resulting addresses in a row would be the contiguous words `row*4 + 0..3`; the baseline has no equality classification and still performs the per-lane address calculations, checks, and scalar-load/pin assembly. This W4 finding is emitted-only and does not establish a native W4 runtime path.

### W8 native AVX2 bank

In the first complete W8 detector block, selected ASM lines `6246–6566` show eight independent wrapped offset calculations, byte scaling by eight for row-major indexing, and eight per-index bounds checks for each channel. The detector words are assembled from scalar loads and pins at lines `6673–6752`: the first two ring vectors are built at `6675–6697` and `6710–6721`, while the second channel's two vectors are built at `6723–6752`; `vinserti128` combines 128-bit halves at lines `6716`, `6729`, `6745`, and `6752`.

This is real native W8 residual access/packing work. For equal offsets, each eight-lane row is contiguous (`row*8 + 0..7`), so a bounded uniform path could potentially replace the scalar-load/pin assembly with a safe contiguous `L::load` over the existing row, while the ragged path preserves the current indexing and checks. The baseline itself does not establish that any particular replacement is correct or beneficial; it establishes only the surviving instruction-level opportunity. No numeric machine-code addresses are available because the frozen baseline command emitted textual `.s`/LLVM IR and did not emit an object; the cited ranges are exact lines in the complete selected assembly files and the original ranges are in the manifest.

## Limits and next action

The baseline proves compile success and identifies residual W8 native / W4 emitted access work. It does not claim runtime speedup, does not qualify a candidate implementation, and does not change the source. The exact capture and selected outputs are ready for the root checkpoint and Astra's bounded decision. After notifying root, this agent pauses; no source edit, candidate build, timing, test, or additional disassembly is authorized in this baseline tranche.
