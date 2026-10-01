## Why

The owner's stance, stated 2026-09-04 while verifying PR #373:

> We don't have to be 100% dependency-free as long as the dependency is battle-tested, popular in the ecosystem, genuinely reduces a lot of error potential in our engine, and is able to compile to WASM. We should be spending most of our effort on audio and sound-related features, not these super low level mechanical issues.

The trigger: PR #373 (LANE-9, #372) went through three consecutive text-format bugs in `tools/audit/src/vectorization.rs`, a hand-rolled scraper of `objdump` output. A substring match fired on `tbl` and on `*_block` symbol names; the rewrite that anchored on the mnemonic then mistook the 8-character AArch64 encoding word for the mnemonic, so a real `bl _memset_pattern16` inside `probe_svf_simd4` on Darwin passed silently. None of these were logic bugs. All of them were "we parsed a text format we do not own".

This issue is the umbrella for replacing that kind of code with crates that meet the four criteria above. Part 1 is concrete and researched. Part 2 is a workspace-wide audit whose results will be appended when the sweep finishes.

## Part 1: replace objdump text scraping in `audit vectorization`

**Recommended stack** (research report with comparison table, API sketch, measurements and sources: written 2026-09-04 by a Sonnet research agent, proof-of-concept compiled and run on x86_64 with rustc 1.97.1):

| role | crate | version | license | downloads (total / 90d) | last release | maintainer |
|---|---|---|---|---|---|---|
| container + symbol table (ELF, Mach-O) | `object` | 0.40.0 | Apache-2.0 OR MIT | 595.8M / 105.9M | 2026-08-01 | gimli-rs |
| match demangled Rust paths (legacy + v0) | `rustc-demangle` | 0.1.28 | MIT/Apache-2.0 | 548.7M / 85.7M | 2026-07-08 | rust-lang |
| x86_64 decoder | `yaxpeax-x86` | 2.2.0 | 0BSD | 615K / 115K | 2026-07-05 | iximeow (solo) |
| AArch64 decoder | `yaxpeax-arm` | 0.4.0 | 0BSD | 332K / 51.6K | 2025-10-20 | iximeow (solo) |

All pure Rust, no C toolchain at build time. Trimmed dependency tree: 13 crates. Clean release build of the whole stack: ~5.7 s. The audit binary is a dev/CI tool, never render-reachable and never shipped, so `docs/REALTIME_DEPENDENCY_POLICY.md` does not apply; wasm32 support is not required for this part.

**What the rewrite gives us**

- Symbol bodies come from `object`'s symbol table as exact byte ranges, not from `<symbol>:` header lines. Fail loudly on zero or multiple matches per probe name.
- Instructions are decoded into typed opcodes and operands. "Is this a call" becomes `Opcode::CALL | Opcode::CALLF` (x86) and `Opcode::BL | Opcode::BLR` (AArch64). "Is this a `.4s` vector op" becomes `Operand::SIMDRegisterElements(SIMDSizeCode::Q, _, SIMDSizeCode::S)` versus scalar `Operand::SIMDRegister(SIMDSizeCode::S, _)`. YMM width is `RegisterClass::width() == 32`. No text, no dialects (GNU vs LLVM vs Apple syntax stop mattering).
- Both decoders are plain functions over byte slices with no `cfg(target_arch)`. The x86 nightly can therefore decode a **cross-compiled aarch64 artifact**, which closes the "AArch64 is compile-checked only" caveat in `VECTORIZATION.md`. Cross-building the release audit binary for `aarch64-unknown-linux-gnu` and `aarch64-apple-ios` from x86_64 with the pinned toolchain was demonstrated during the #373 verification (about one minute each with rust-lld). The Darwin `memset_pattern16` true positive would have been caught by CI instead of by someone running the script on a Mac.
- A byte sequence the decoder does not recognise returns a typed `Err`, never a fabricated opcode. The audit should treat a decode error inside a probe body as a hard failure.
- An AArch64 `b` used as a tail call becomes detectable: a `B` whose `PCOffset` target lands outside the probe's byte range is a probable tail call. Text scraping could not do this.

**Compile both decoders unconditionally, not behind features.** Measured: adding `yaxpeax-arm` next to `yaxpeax-x86` costs ~69 KB of stripped binary and ~0.03 s of build time. A feature split would recreate inside the decoder the exact asymmetry that let the aarch64 path go untested.

**Rejected**

- `capstone` / `capstone-sys` and `bad64` / `bad64-sys`: both compile vendored C at build time (`cc::Build` unconditionally; `bad64-sys` also needs `bindgen`/libclang). Disqualified on that alone.
- `iced-x86`: excellent and far more popular, but x86-only, so it does not reduce the crate count, and its `Code` enum bakes encoding form into variant names (`VEX_Vmulps_ymm_ymm_ymmm256`), a worse fit for the allowlist's mnemonic-family shape.
- `disarm64`: credible pure-Rust AArch64 fallback if `yaxpeax-arm` maintenance stalls; it also decodes SVE/SME, which `yaxpeax-arm` explicitly refuses with `DecodeError::IncompleteDecoder`. Younger, solo maintainer, operand shape baked into variant names.

**Risks and mitigations**

- Solo maintainer for the three `yaxpeax` crates, hosted on a self-hosted git server. Exact-pin versions as the workspace already does for `sha2` and `wide`; vendor or mirror if the host ever becomes unreachable in CI; `disarm64` is the documented AArch64 fallback.
- `yaxpeax-arm` does not decode SVE/SME. Our NEON kernels only need baseline `.4s`; a future SVE kernel would need `disarm64` or an upstream fix.
- `object` needs `.symtab`. The release profile has `debug = 1` and no `strip`, same assumption the current objdump path makes. Add a one-line comment at the profile site.
- Demangled-path suffix matching (v0 mangling adds crate disambiguators): assert exactly one match per probe, as `certify()` already does for missing symbols.
- No explicit `rust-version` on the `yaxpeax` crates; `object` declares 1.85. All build on the pinned 1.97.1. Exact pins protect against implicit MSRV bumps.

**Scope of the Part 1 PR**

1. Owner ruling on the four crates above (this issue is that ruling once accepted).
2. Rewrite `symbol_bodies` / `certify` in `tools/audit/src/vectorization.rs` on `object` + `yaxpeax`; keep the TSV format and the report JSON schema; keep the unit tests' shape but feed them bytes, not text.
3. Add a cross-compile step to the nightly (`--target aarch64-unknown-linux-gnu`, link with rust-lld and stub libs or emit objects) and run the AArch64 rows from that artifact on the x86 runner. Retire the "compile-checked only" sentence in `VECTORIZATION.md`.
4. Extend `scripts/test-native-vectorization-report.sh` mutations to byte-level injection (patch a `call`/`bl` encoding into a copy of the artifact) and assert the failure class.
5. Land after #373 merges; do not fold this into LANE-9.

## Part 2: workspace-wide audit of hand-rolled code

A Fable agent is sweeping the workspace for accidental reinvention of well-known formats and algorithms (hand-written JSON writers, CLI arg loops, error boilerplate, hex/base64, ring buffers and atomics protocols, FFT/resampling, test harness machinery, WASM glue). Each candidate is checked against: crates.io downloads, maintainer, license, MSRV vs 1.97.1, wasm32-unknown-unknown support, whether it would enter the render path (must be alloc-free and lock-free there or it is out), the concrete error potential removed, and effort. Contract-driven hand-rolling (unfused fma, flush-to-zero, bit identity across scalar/W4/W8/wasm, the realtime zero-alloc path) is recorded as KEEP, not flagged.

Results will be appended to this issue as a comment when the sweep finishes, with a ranked table and a suggested first three.

## Definition of done for this issue

- Part 1 landed and the nightly checks all six allowlist rows from one x86 runner.
- Part 2 results triaged into REPLACE / KEEP / DISCUSS with an owner decision on each REPLACE, and one child issue per accepted REPLACE.
