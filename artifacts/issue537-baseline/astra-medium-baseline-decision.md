**PASS — bounded baseline decision: authorize Luna xhigh attempt 1’s first compact tranche.**

Verified clean, pushed `ce3fb612` against live main `375a86c`; #537 is OPEN with exact matching title/body. Source SHA matches `1058d604…39879779`. Recorded original LLVM/ASM sizes and hashes match; capture status is 0 under the pinned release configuration. Post-main `34083008320` is **SUCCESS**.

Native W8 instructions confirm surviving per-lane offset loads, wrap/index calculations, bounds checks, and `vmovd`/`vpinsrd` packing with `vinserti128`. No uniform-offset arm bypasses this work. W4 corroborates the mechanism **as emitted-only evidence**; W1 offers no cross-lane packing opportunity.

**Narrowing:** low/high bands already reuse computed indices through compiler CSE. Do not credit eliminating duplicate wrap/address calculations across those bands, or infer temporary-array stack traffic.

Authorize only transient per-channel classification of exactly `L::WIDTH` offsets once per actual segment, safe contiguous row loads for uniform offsets, and original ragged fallback. Preserve arithmetic, state, resources, bounds safety and bypass.

First tranche: compact source change plus independent old-index oracle and actual-callsite witness, including the frozen fallback mutation. Pause immediately when compiling/focused-green for root’s exact-path checkpoint/push. Public transitions, allocation/liveness and candidate lowerings follow under the frozen spec; maximum three attempts.

This is no implementation acceptance or projected speedup claim.
