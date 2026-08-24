# Native vectorization certification red mutations

Issue #144 item 3 requires each structural assertion to discriminate. The unit tests in
`src/vectorization.rs` use synthetic disassembly to prove the three rule families independently,
and `scripts/test-native-vectorization-report.sh` repeats them against the real release artifact:

| mutation | expected rejection |
|---|---|
| replace required `vmulps` with a nonexistent instruction family | missing vector family |
| inject `vaddss` into the real gain-probe disassembly through an objdump test wrapper | forbidden scalar fallback |
| remove the x86 sum probe row | active registry and allowlist differ |

The mutations alter a temporary allowlist only. They never recompile or edit production code, so a
red result proves the disassembly checker read and enforced the claim rather than merely observing
that a build completed.
