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

Native AArch64 (`aarch64-neon`) is unsupported; no claim (owner ruling 2026-09-04, #378): the
three `aarch64-neon` rows and their probes are retired from the registry. See the deferred-defect
register in `docs/TARGET_MATRIX.md` and #378, including the Darwin `svf_block` `flush()`
`bl _memset_pattern16` finding this registry used to carry as a known-red row -- a future revival
of native AArch64 must reopen that entry before restoring the row.

The subject artifact is intentionally reported as
`release_probe_instantiations_of_production_kernels`. It is the release
`audit` executable containing the same inline-always generic production kernel bodies,
not a claim that the audit executable is a shipped host. The current first-stage CI report runs
and uploads x86 evidence non-blockingly.

Each JSON report hashes the complete subject artifact, raw disassembly bytes, and explicit
allowlist. Run it through `scripts/run-native-vectorization-report.sh`; prove its checks red with
`scripts/test-native-vectorization-report.sh` and see `VECTORIZATION_MUTATIONS.md`.
