# Native vectorization report

`audit vectorization` certifies fixed, full-bank release-profile instantiations of
three production lane-kernel families: feed-forward gain, feed-forward sum, and recursive SVF.
The explicit registry is `vectorization-allowlist.tsv`:

- x86-64-v3 W8 requires AVX/AVX2 packed-single families and YMM operands;
- AArch64 W4 requires NEON `.4s` families;
- both reject scalar floating-point arithmetic mnemonics inside the named probe bodies;
- the recursive-SVF rows are the codegen leg of the unfused contract (issue #163 phase 2): the
  required groups are the separate multiply and add (`vmulps` + `vaddps` on x86, `fmul` + `fadd`
  on AArch64) and the fused mnemonics (`vfmadd`/`vfnmadd`, `fmla`/`fmls`) are forbidden, so the
  report fails if `Lane::fma` ever fuses again; and
- every row forbids call instructions (`call` on x86, `bl` on AArch64) inside the named probe
  bodies, so a helper call emitted into a kernel body (the LANE-1 `bl _memset_pattern16` shape)
  fails the report instead of passing silently.

The subject artifact is intentionally reported as
`release_probe_instantiations_of_production_kernels`. It is the release
`audit` executable containing the same inline-always generic production kernel bodies,
not a claim that the audit executable is a shipped host. The current first-stage CI report runs
and uploads x86 evidence non-blockingly. The AArch64 registry and probe bodies compile under the
pinned release target, but promotion requires a native AArch64 disassembly report before this tool
can claim executed coverage of that backend.

Each JSON report hashes the complete subject artifact, raw disassembly bytes, and explicit
allowlist. Run it through `scripts/run-native-vectorization-report.sh`; prove its checks red with
`scripts/test-native-vectorization-report.sh` and see `VECTORIZATION_MUTATIONS.md`.
