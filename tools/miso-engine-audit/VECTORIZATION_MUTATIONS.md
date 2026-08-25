# Native vectorization certification red mutations

Issue #144 item 3 requires each structural assertion to discriminate. Two layers do that:
`scripts/test-native-vectorization-report.sh` mutates real registries, real disassembly and real
LLVM IR and requires the subject to go red **for the reason the mutation names**; the unit tests in
`src/vectorization.rs` prove the parsers on synthetic negative fixtures.

Neither layer edits production code, so a red result proves the checker read and enforced a claim
rather than observing that a build completed.

## Live mutations (`scripts/test-native-vectorization-report.sh`)

The suite rebuilds `miso_engine_audit` and asserts the rebuilt binary carries the `vectorization`
subject *before* it runs anything, then takes a green baseline and mutates from it.

| # | mutation | expected rejection | rule it proves |
|---|---|---|---|
| 1 | remove `kernels::svf_block` from the families registry | `is public in the lane crate and is not registered` | a new kernel family is uncertified until registered |
| 2 | register a kernel the lane crate does not expose | `which the lane crate no longer exposes` | a stale rule is a failure, not a free pass |
| 3 | remove the AArch64 row for a certified family | `certified family 'recursive-svf' has no rule` | a family must be covered at *every* backend |
| 4 | rename the probe in the allowlist | `expected exactly one defined symbol` | rename and inline evasions both leave no body to certify |
| 5 | declare a real arithmetic family `no-float` | `in a family declared free of them` | the structural class is checked, not assumed |
| 6 | omit a backend entirely (no `--backend`, no `--skip-backend`) | `was neither certified nor explicitly skipped` | dropping a backend is not a pass — the exact hole in the first slice |
| 7 | certify one product when the registry names two | `which this run did not certify` | a shipped product cannot be quietly skipped |
| 8 | raise the shipped kernel-host floors past what the artifact does | `below the registered floor` | a bank that stopped vectorizing is caught in the shipping bytes |
| 9 | point the render-entry rule at a symbol the artifact lacks | `expected exactly one definition` | the exported entry is read from the artifact |
| 10 | inject a second symbol header carrying the certified name | `disassembled bodies are named` | the lookalike/duplicate-header evasion is refused, not read |
| 11 | inject `vfmadd213ss` into a certified body | `scalar floating-point arithmetic instructions` | the scalar fused multiply-add an opcode-prefix scan misses |
| 12 | inject `fmul reassoc <8 x float>` into the certified IR | `fast-math flags` | no operation may carry a reassociation licence |
| 13 | inject `call float @expf` into the certified IR | `forbidden intrinsic or math-library symbols` | the arithmetic may not leave the lane domain |
| 14 | inject `fmul <4 x float>` into an eight-lane certified body | `narrower vector type` | half-width vectorization is not vectorization |
| 15 | misspell a skipped backend name | `unknown skipped backend` | the AArch64 guard cannot be spelled into silence |

## Parser negative fixtures (`src/vectorization.rs` tests)

The roster scan, the symbol-table parse, the disassembly parse and the IR scan are the places where
"structured parsing over greps" has to earn its name. Each has a fixture built from the exact shape
that defeats the naive version:

| fixture | what the naive parse does wrong |
|---|---|
| `pub fn` in a doc comment, line comment, block comment and string literal | a token grep registers four kernels that do not exist |
| a `{` inside a string literal | a brace-depth counter never returns to zero and hides every later kernel |
| `pub fn` inside a function body and inside a nested `mod` | an item that escaped the top level is still counted as a certified kernel |
| an `nm` row whose demangled name contains spaces (`<A as B>::c`) | splitting from the left reads the name as `<A` |
| two symbol headers with the same demangled name at the same address | the first body wins, and an injected one is indistinguishable from the real one |
| `jmp 0x40 <vmulps_lookalike>` | a substring rule counts a branch target as a required opcode |
| `@llvm.experimental.noalias.scope.decl` | a substring scan for `@llvm.exp` reports a transcendental in every body that touches two slices — this one actually happened during development and is why the callee scan extracts whole names |
| `@llvm.fma.v8f32` next to `@llvm.fmuladd.v8f32` | a prefix rule forbids the permitted fusion or permits the forbidden one |
| a length-prefixed needle `15probe_svf_block` against `probe_svf_block_ramped` | an unprefixed needle matches the longer name too |
| `fmla s20, s18, s6` and `fmla v20.2s, ...` on AArch64 | the mnemonic is identical to the wide form; only the operands say the width |
| a register-indirect `call *%rax` | a call closure that silently skips it overstates its own reach |
