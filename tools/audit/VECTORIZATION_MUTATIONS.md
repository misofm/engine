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
| rename the exact gain-probe header to a near-name through an objdump test wrapper | probe symbol is absent |
| replace the SVF body with two exact headers whose required families are split across bodies | ambiguous disassembly bodies |
| remove the x86 sum probe row | active registry and allowlist differ |

The missing-family and incomplete-registry mutations alter temporary allowlists. The scalar-
fallback, fused-multiply-add, call, near-name, and split-body mutations wrap the disassembler and
inject or replace only the captured probe body. The fused-multiply-add, call, near-name, and
split-body mutations additionally assert their failure *class*, so a red for the wrong reason does
not count as proof. The wrapper's unmodified output is a green control. None recompiles or edits
production code, so a red result proves the disassembly checker read and enforced the claim rather
than merely observing that a build completed. The fused-multiply-add mutation is the codegen leg
of the unfused contract that `scripts/check-unfused-seal.sh` guards at source level (issue #372,
row LANE-9).
