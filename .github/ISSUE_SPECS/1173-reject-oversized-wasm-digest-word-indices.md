# Reject oversized Wasm digest-word indices before offset arithmetic

## Source finding and smallest successor slice

The complete wasm-gate-guest housekeeping review (#1149) found that `miso_gate_digest_word` documents a trap for every out-of-range word, but shipping wasm32 performs `(word as usize) * 4` before checking `offset + 4 <= digest.len()`. With release overflow checks disabled and 32-bit usize, word `0x40000000` wraps its offset to zero and passes the check, selecting word zero instead of trapping. Root and worker B independently read the source, profile and host caller. No runtime reproducer or fix has been executed. Safe slice bounds remain in place; this is a tooling ABI refusal mismatch, not a claimed memory-safety defect or engine C-ABI failure.

The current Wasmtime host requests only words 0..8 exclusive. Its 142-case/250-comparison successful guest run therefore does not qualify hostile argument refusal. All valid digest values and shared-corpus arithmetic remain covered.

## Frozen boundary and authorization

A future bounded brief must reject every out-of-range u32 word before offset arithmetic or references, preserving the existing documented trap contract and all valid words, exports, cache keys/ownership/poison handling, case/width bounds, generators, report schema and DSP behavior. Do not introduce a new ABI, corpus, dependency or generic test framework. Scope the minimum existing guest/Wasmtime test owner required to exercise the actual 32-bit shipping module. No implementation is authorized by this source record.

## Objective gates and test value

- Exercise an actual release simd128 guest with the ordinary boundary word 8 and the confirmed wrap word 0x40000000; both must trap. Existing valid reads remain equal to their pins.
- The focused regression must be red when the fix is reverted. Its unique defect is acceptance of a large invalid word after 32-bit offset wrapping; existing host comparisons never submit that input. Avoid a native-only regression whose 64-bit arithmetic cannot reach this defect.
- Retain the current actual guest parity and SIMD/detector gates, export/import shape, native G5/G6 ownership and proportional lint/format/policy checks. No extra target matrix, timing or fixture expansion.
- Freeze a bounded issue-first attempt limit and checkpoint/review workflow before implementation; preserve raw failure evidence and close only after PASS is upstream.

## Decision record

Recorded 2026-10-01 from the requested GPT-6.1 Sol xhigh worker B and root source review. This is a specific documented-contract follow-up, not an owner API-design question. Housekeeping #1149 remains a separate comment-only cleanup and does not claim this gap fixed.
