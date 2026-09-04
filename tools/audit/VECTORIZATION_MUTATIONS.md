# Native vectorization certification red mutations

Issue #144 item 3 requires each structural assertion to discriminate. The unit tests in
`src/vectorization.rs` use synthetic disassembly to prove the rule families independently,
and `scripts/test-native-vectorization-report.sh` repeats them against the real release artifact:

| mutation | expected rejection |
|---|---|
| replace required `vmulps` with a nonexistent instruction family | missing vector family |
| inject `vaddss` into the real gain-probe disassembly through an objdump test wrapper | forbidden scalar fallback |
| inject `vfmadd213ps` into the real SVF-probe disassembly through an objdump test wrapper | forbidden scalar fallback (the unfused seal) |
| inject `call` into the real SVF-probe disassembly through an objdump test wrapper | forbidden call inside a kernel body |
| remove the x86 sum probe row | active registry and allowlist differ |

The missing-family and incomplete-registry mutations alter temporary allowlists. The scalar-
fallback, fused-multiply-add, and call mutations wrap the disassembler and inject one instruction
into a captured probe body. The fused-multiply-add and call mutations additionally assert the
failure *class* (`forbidden scalar fallback` / `forbidden call`), so a red for the wrong reason
does not count as proof. None recompiles or edits production code, so a red result proves the
disassembly checker read and enforced the claim rather than merely observing that a build
completed. The fused-multiply-add mutation is the codegen leg of the unfused contract that
`scripts/check-unfused-seal.sh` guards at source level (issue #372, row LANE-9).
