**PASS — #536 authorizes only the diagnostic first tranche.** No brief blockers or unnecessary expansion of the retained triage.

Verified clean local HEAD/upstream `ca6229bd6e75c0a072927d4ddff07abf98e4aee5`, based on unchanged PR535 head `cc5f04fb3bd98052fc4d3f8edbb814718259a543`. GitHub #536 is OPEN with matching title and exact local-spec body. The retained failure stdout matches the original log byte-for-byte. Required run34080063161 remains failed.

The brief correctly bounds implementation to `crates/host-core/tests/scalar_point_endpoint.rs` using existing allocation counters, TLS auditor and child helpers:

- Capture exactly four debug comparisons: two existing children × inherited concurrency1/2, **before** changing child arguments.
- Pair global and same-thread diagnostics without changing preparation behavior; keep setup, printing and owner destruction outside measurement windows.
- Use deterministic foreign-thread allocation to demonstrate counter contamination sensitivity, preserving own-thread counter liveness.
- Preserve every original allocation/byte, realtime, resource and admission assertion.

The diagnostic checkpoint must receive the specified bounded review before adding `--test-threads=1` to both launchers. Passing comparisons or the synthetic control alone cannot establish the historical cause. Same-thread excess or insufficient supporting evidence requires stopping and rebriefing.

Finite gates, the three-attempt limit, Luna high/xhigh implementation, Astra medium verification and one completed-package integration into PR535 are appropriate. No production, Cargo, CI, policy or expectation changes are authorized.

This is scope approval only—not implementation acceptance, historical-cause attribution or merge authorization.