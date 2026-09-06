# Final combined metering review — issues 516, 519 and 520

**Verdict: PASS for the combined engine capabilities and the exact qualified Linux artifact.**

Evidence candidate: `a1aeefb830b2ac192274153539847765385ed7f1`.
Executable source was frozen at `11cb3c2e`; subsequent committed changes are artifact identity and issue/failure evidence only.
Shipped Wasm SHA-256: `671d615de9a74232c2af623592fc85eca68b8cd571fb5b7542ba648e621af4e3`.

Reviewer independently hashed `/private/tmp/engine-519-linux-final-output/miso-engine-v1-audio-worklet.simd128.wasm`. The result matches the committed pin and the actual generated `qualification/results.json`, whose candidate is `a1aeefb8`. This is the canonical Linux/amd64 Rust 1.97.1 build; earlier Mac/candidate hashes and their qualification outcomes are not substituted for this artifact.

## Accepted capabilities

- #516: exact per-window track/final-master peak publication, stable empty/invalid poll behavior, bounded stale/lost interval recovery and lease epochs, and explicit independent GR age semantics. Strict main-realm transport validation accepts and validates the emitted metadata.
- #519: engine-owned source/tap/metric selection usable through shared host-core preparation, explicit snapshot presence, preserved default full numerical path, sanitization in every selected numeric pass, and actual removal of unrequested energy/count/held/sqrt work. Fixed-layout queue, seal and host pending storage remains resource-accounted.
- #520: O(1) return before track/effect scans when no complete master interval exists, continuous observation of required samples, and retained bounded recovery at ready boundaries. No publication cadence or UI/subscription API is added.

These findings incorporate the independently executed focused reviews recorded in `/private/tmp/engine-516-attempt2-review.md`, `/private/tmp/engine-519-review.md`, and `/private/tmp/engine-520-review.md`. No new broad reruns were performed for this final evidence review.

## Final inspected qualification

- `/private/tmp/engine-519-520-static.log`: unchanged static/object/callback, kernel roster, ABI/metadata, vocabulary and boot-budget high-water checks PASS; zero budget mismatches.
- `/private/tmp/engine-519-520-browser.log` and actual generated matrix/results: exact six-file shipped set and Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 qualification PASS, including mutation proofs.
- `/private/tmp/engine-519-520-hermetic.log`: actual worklet/host execution and malformed-message, allocator, opcode/callgraph and protocol mutation proofs PASS. Intentional negative mutation diagnostics are not candidate failures.
- Prior source-checkpoint focused/full test results remain applicable; the final evidence/pin commits do not modify executable source.

No gate or callgraph allowlist was weakened. The preexisting platform tan-ULP test failure was independently reproduced on untouched pre-metering and issue-parent sources and is not silently claimed fixed.

## Performance and delivery limits

The sole descriptive timing invocation failed during warmup because the timing fixture omitted its final end-of-region marker. No measured rounds or speedup figures are available. Failed JSONL/log evidence is committed under `docs/evidence/metering-519-520/`; the bounded repair and new successor-owned invocation are explicitly assigned to issue #522. This is a transparent tooling transfer under the repository ceremony boundary, not a retry or a relaxation of engine correctness/work-avoidance evidence. No microsecond, percentage, cycle or end-to-end performance gain is claimed by this PASS.

There are no remaining technical blockers for #516/#519/#520 in the reviewed scope. At final review, the newly generated browser matrix/results were still root-owned uncommitted changes. Root must commit/push final evidence, synchronize issue/PR state and verify CI before claiming remote delivery. This verdict does not claim those delivery steps are already complete and does not authorize or perform a merge.
