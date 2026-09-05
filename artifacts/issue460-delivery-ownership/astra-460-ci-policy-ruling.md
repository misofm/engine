# Astra #460 CI ownership-file registration ruling

**Approve one exact test-file registration after amending/synchronizing #460.** PR472's prior PASS does not authorize merge after required CI failed. Read retained `/tmp/engine-472-policy-ci.log`: realtime policy rejects only the already reviewed test allocator's unsafe impl/functions/System calls because its file is absent from the existing ownership/audit exclusion list.

Add precisely the anchored path alternative `^crates/protocol/tests/delivery_ownership[.]rs:` to the existing `unsafe source exclusions` expression in scripts/check-realtime-policy.sh. The escaped literal dot ensures exact filename matching. Do not add a protocol-directory wildcard, a new general allocator exemption, lint allow, alternate scan path or quiet/error suppression. Add a concise adjacent comment explaining that this issue-approved integration test delegates allocation/deallocation to System and records scoped audit counters; production protocol source receives no unsafe permission.

This is policy registration for the accepted test-only audit implementation, not a new runtime feature attempt or authorization to change allocator code. Extend the numbered scope only to this one policy registration/comment plus candid evidence. Preserve the existing complete source scan, status/payload handling, marker population and every other exclusion.

Proportional checks: bash syntax; real realtime policy; existing test-realtime-policy suite; formatting/source diff checks. Confirm its unapproved unsafe fixture still fails, and that an adjacent differently named protocol test file is not covered by this exact alternative (a tiny scratch fixture suffices if not already covered). No new mutation campaign, Cargo/workspace/target/artifact/browser or timing rerun is needed for this shell-only registration. Prior runtime evidence remains applicable with exact delta disclosed.

Retain failed CI job101315772366 and terminal result. Root checkpoints/pushes the registration/evidence, requests actual-new-head Astra review, and waits for required qualification SUCCESS. No merge on93007b36 based on its earlier conditional review.

No reviewer edits, tests, builds, timing or Git/GitHub mutations were performed.
