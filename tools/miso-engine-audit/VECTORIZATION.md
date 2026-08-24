# Native vectorization report

`miso_engine_audit vectorization` certifies fixed, full-bank release-profile instantiations of
three production lane-kernel families: feed-forward gain, feed-forward sum, and recursive SVF.
The explicit registry is `vectorization-allowlist.tsv`:

- x86-64-v3 W8 requires AVX/AVX2 packed-single families and YMM operands;
- AArch64 W4 requires NEON `.4s` families; and
- both reject scalar floating-point arithmetic mnemonics inside the named probe bodies.

The subject artifact is intentionally reported as
`release_probe_instantiations_of_production_kernels`. It is the release
`miso_engine_audit` executable containing the same inline-always generic production kernel bodies,
not a claim that the audit executable is a shipped host. The current first-stage CI report runs
and uploads x86 evidence non-blockingly. The AArch64 registry and probe bodies compile under the
pinned release target, but promotion requires a native AArch64 disassembly report before this tool
can claim executed coverage of that backend.

Each JSON report hashes the complete subject artifact, raw disassembly bytes, and explicit
allowlist. Run it through `scripts/run-native-vectorization-report.sh`; prove its checks red with
`scripts/test-native-vectorization-report.sh` and see `VECTORIZATION_MUTATIONS.md`.
