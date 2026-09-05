# Deferred RT-1 descriptive measurement

Candidate a74477c68eaf8650fdc86ec6d0f3ac04a18cb880 contains the accepted RT-1 implementation from merged1fa4424d plus only the dedicated registration/evidence. Preflight passed with zero workloads. Read-only readiness recorded load0.20, binary age113seconds, CPU63 affinity and sibling31 busy0.00%, without changing the0.50/60second/5% gates.

Exactly one controlled runner invocation completed one warmup and two measured rounds, producing46 records. All production per-record and aggregate validators passed; raw and accepted data are byte-identical at79132bytes, SHA-256ef4913f8267a1d5987a913f2bc7d4ceeb82f310d17df3518b947fd4bda1e559f. Original stderr is preserved and matches its disposition digest. The runner binary SHA-256 is748fe3460c6c52d457261f2cd41d4319fd6c888c0f525031cead8354fdb76eaf under opt-level3/LTO=false/codegen-units16, matching the warmed binary. The default-profile preflight binary is separately recorded.

No workload or validator changed, no invocation was repeated, and issue399's refusal remains untouched. Detailed row/digest comparison and Astra evidence/actual-PR review follow this recoverable completed-capture checkpoint. Timings are descriptive and no speedup is claimed by this checkpoint.
