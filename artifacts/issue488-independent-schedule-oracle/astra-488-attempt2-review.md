# Astra #488 Sol attempt 2 — PASS

Exact clean head c86c43478fbe493be7aceabb9daf0a3225ec49a3, engine-488-schedule-oracle. Read full frozen numbered scope, prior FAIL, source delta and retained actual command/log/status records. No tests/builds/timing or repository/Git/GitHub mutations.

Both finite gaps are closed. The effect-only case now constructs an actual runtime BankMembership containing only Effect entries, calls units_of and projects away membership tags only, then compares directly to the existing handwritten expected_effect_only. The model independently compares to that same literal. Neither output derives the other's expectation. Accepted mixed/empty/unbanked/nonmonotonic-ID/first-emission/lane-order/singleton cases remain intact; no runtime algorithm or new oracle framework was added.

The retained mutation-diff.log is the actual git diff of precisely the sole members.sort_unstable() removal in runtime::units_of. The corresponding exact test returns101 at the unchanged literal mismatch: actual Effect[1,4]/Builtin[2,5], expected[4,1]/[5,2]. Restored exact debug and release each pass one test/status0. Current source visibly restores the sort, and runtime.rs has no cumulative delta from base. This is the requested actual implementation discriminator, not a reconstructed diff or model-only mutation.

Independent fmt, diff-check and affected graph policy now have separate numeric0 records. Both full graph library profiles report56 passing tests,0 failures; focused restored profiles report1 each. Prior missing first-log/recapture and bundle-status limitations remain historical and are not relabelled. Source/test scope stays within the numbered program.rs cfg(test) change and spec/evidence.

Source PASS permits root packaging and delivered-main integration with source identity preserved, then actual-head PR review and requiredCI/verified remote closure. No benchmark, artifact regeneration, second scheduler or additional property matrix is justified by this test-only result. CP-15 closure is bounded independent unit-order checking; it does not claim all other interpreter models are independent.
