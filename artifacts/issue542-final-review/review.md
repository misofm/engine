# Issue 542 final integrated adversarial review

Verdict: **PASS** for source and attached qualification evidence.
Reviewer routing: Astra LOW. Historical actual Sol xhigh attempt 1 FAIL and attempt 2 PASS remain attributed to Sol; the misrouted Luna work remains Luna evidence.

Reviewed head: `859b820bf6820c00f235364a827f9d5814e21a70`.
Current base: `b95c9b7b028ed07cfea2f7669467689c05320c37` (verified origin/main).
Working directory: `/home/bl/misofm/engine-protocol-conformance-boundary`.
Review date: 2026-09-07 UTC.

The actual base-to-head product diff stays within the amended issue authority, including only the approved stale bench conversion removal and induced three lockfile edges. Conformance owns the corpus, runner and fixture; protocol has only a dev dependency on conformance. Normal dependency inspection contains no conformance. No production codec/controller behavior or public escape hatch was introduced.

The corpus builder is unchanged apart from ownership/import and provenance text. The all-opcode fixture function is byte-identical to the baseline. The single pin remains `0xbdeb_b0f8_1c38_ec42`; the native 46-frame/typed-decoder/deep-dispatch tests pass independently. The deterministic million-mutation consumer changes only imports and retains its generator/assertions. All extracted bodies were compared with baseline: 44 controller, 17 message-wire and 19 session-wire tests preserve names, conditions and assertions. Differences are paths/imports/formatting and the test-only encoded-fixture bridge needed for the dev-dependency cycle.

The boundary checker exempts only three exact child paths after verifying each exact adjacent cfg(test)/mod tests declaration; no blanket tests.rs exemption remains. Checked discovery/status handling remains in place. Preserved attempt 2 controls reject both removed guards and the forbidden export, restore byte-exactly, and use the same current checker hash. The current positive checker also passes.

The Wasm runner retains typed decoder assertions, count assertion and returned digest verdict. The parity script accepts only the exact zero return with interpreter status zero; silence, other returns, traps and process/build failures remain red. The final self-test capture includes scalar/SIMD builds, green scratch control, inert-invocation refusal and three rebuilt red rows. Both retained actual artifacts independently match the final manifest sizes and SHA256 hashes and return the expected zero; no rebuild or benchmark was run for this review.

Qualification provenance: all 24 metadata/raw pairs decompress; all 833 recorded source-hash entries match current source. Their recorded qualified head is `f9c279075d22fb66640df4d7cb669f7e82c62f47`; the delta to reviewed head contains only the issue record and qualification evidence. Metadata records UTC, cwd, command, head and numeric status. Initial unsuccessful probes remain retained and receive no gate credit. Native debug/release, protocol feature variants, mutation, policy, bench compile, Clippy/rustdoc and format captures agree with the claims. This review relies on those captures for the larger suites and independently reruns only the focused checks listed below.

Evidence nuance (nonblocking): the corrected final boundary probe inserts literal backslash-n characters around the forbidden export, so it is a forbidden-token structural control, not a compilable Rust export. Its checker exit 1 and before/after hash restoration are authentic. The preserved attempt 2 proper export control already establishes the exact requested export discrimination against byte-identical current lib.rs and checker, so this does not leave that gate unproved. No compile success is credited to either deliberately broken export fixture.

Independent commands and numeric exits are retained in `commands.json` and `check-*.log`: head/base identity, full diff whitespace, shell syntax, positive boundary checker, locked normal protocol dependency tree, and focused conformance corpus tests all exit 0 (3 tests pass, zero ignored). `structural-artifact-audit.txt` records exact interpreter commands, exits, artifact identities and extraction/fixture checks. `evidence-audit.txt` records the metadata/hash census. Initial worktree was clean; this review adds only this report directory, left uncommitted for root.

No source blocker found. PR/current-head required qualification, merge, GitHub synchronization and cleanup remain root delivery obligations; this is not a remote-delivery claim.
