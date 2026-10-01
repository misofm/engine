# Native vectorization report

`audit vectorization` certifies fixed, full-bank release-profile instantiations of
three production lane-kernel families: feed-forward gain, feed-forward sum, and recursive SVF.
The explicit registry is `vectorization-allowlist.tsv`:

- x86-64-v3 W8 requires AVX/AVX2 packed-single families and YMM operands;
- the report rejects scalar floating-point arithmetic mnemonics inside the named probe bodies;
- the recursive-SVF row is the codegen leg of the unfused contract (issue #163 phase 2): the
  required groups are the separate multiply and add (`vmulps` + `vaddps`) and the fused mnemonics
  (`vfmadd`/`vfnmadd`) are forbidden, so the report fails if `Lane::fma` ever fuses again; and
- every row forbids call instructions (`call`/`callq`) inside the named probe bodies, matched on
  the exact mnemonic token rather than as a substring, so a helper call emitted into a kernel body
  fails the report instead of passing silently.
- probe ownership is based on the complete demangled path (`audit::vectorization::probe_*`) in an
  actual disassembler header: a nonempty hexadecimal address token, whitespace, and the outer
  `<symbol>:` envelope. Prose and instruction-annotation prefixes do not create or terminate
  ownership. Only the Rust disambiguator suffix `::h` plus sixteen lowercase hexadecimal digits
  is accepted as optional decoration. A similarly named or unrelated-path header is absent, and
  repeated exact headers are ambiguous; their bodies are kept separate and never combined.

On the release artifact used for the #758 qualification, LLVM `llvm-objdump` 18.1.3 and GNU
`objdump` 2.42 both emitted the three headers with the exact `audit::vectorization::probe_*`
spelling and no retained suffix. The ordinary report passed with either disassembler against the
same artifact; the focused tests retain coverage for the narrowly supported Rust disambiguator.

Native AArch64 is an official mobile target under the 2026-09-28 owner ruling
(`docs/rulings/engine-footprint-2026-09-28.md`). This report's three `aarch64-neon` allowlist rows
remain retired and unqualified. The deferred-defect register in `docs/TARGET_MATRIX.md` preserves
the Darwin `svf_block` `flush()` `bl _memset_pattern16` finding that was a known-red row; #377 owns
the parser and cross-artifact qualification work. On an AArch64 host this audit is red by design:
`certify()` reports `allowlist registry mismatch for aarch64-neon` until the rows are restored.

The subject artifact is intentionally reported as
`release_probe_instantiations_of_production_kernels`. It is the release
`audit` executable containing the same inline-always generic production kernel bodies,
not a claim that the audit executable is a shipped host. The current first-stage CI report runs
and uploads x86 evidence non-blockingly.

Each JSON report hashes the complete subject artifact, raw disassembly bytes, and explicit
allowlist. Run it through `scripts/run-native-vectorization-report.sh`; prove its checks red with
`scripts/test-native-vectorization-report.sh` and see `VECTORIZATION_MUTATIONS.md`.
