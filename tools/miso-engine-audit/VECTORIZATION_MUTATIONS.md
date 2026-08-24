# Native vectorization certification red mutations

Issue #144 item 3 requires each structural assertion to discriminate. The unit tests in
`src/vectorization.rs` use synthetic disassembly to prove the three rule families independently,
and `scripts/test-native-vectorization-report.sh` repeats them against the real release artifact:

| mutation | expected rejection |
|---|---|
| replace required `vmulps` with a nonexistent instruction family | missing vector family |
| inject `vaddss` into the real gain-probe disassembly through an objdump test wrapper | forbidden scalar fallback |
| remove the x86 sum probe row | active registry and allowlist differ |

The missing-family and incomplete-registry mutations alter temporary allowlists. The scalar-
fallback mutation wraps the disassembler and injects one scalar instruction into the captured gain
body. None recompiles or edits production code, so a red result proves the disassembly checker read
and enforced the claim rather than merely observing that a build completed.
