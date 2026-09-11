# route_fold needs a corpus differential (the three shadowed clauses are schedule-order-dependent)

## Approved current scope — 2026-09-11

Add a narrow test-support observation of the runtime's actual route-fold decisions and compare it with an independent model using the existing seeded graph corpus. Cover sole-readership, intervening master reads, and one-master retention, including bounded constructed valid program shapes where compiler schedule ordering otherwise shadows the clauses. Preserve production routing decisions and PCM. Add the requested comment guard for plain_route_gains/node_kind coupling. Reuse existing corpus infrastructure; no new benchmark framework or exhaustive fixture campaign.

Own crates/graph/src/runtime.rs narrow test-support seam/comments and crates/graph/src/program/tests.rs (or existing directly relevant graph tests). No production route-fold algorithm, bind-error correction, policy, artifact pin or compiler edits. #221 must follow this issue's delivered coverage.

Gates: focused graph corpus and existing route-fold tests; each named safety clause must have a discriminating physical negative control or an explicit evidence-based scope ruling before PASS. Avoid self-comparison: oracle cannot call the same predicate it verifies. Maintain accepted/refused coverage and stable ordering. Separate negative and positive build targets. Strict focused Clippy, relevant graph tests and required PR/main CI; no timing claim. If existing corpus cannot exercise the frozen contract with one bounded extension, report before building a new harness.

Astra XHIGH scoping approved; user authorized execution. Astra LOW implements, Astra XHIGH independently verifies. Five attempts maximum; each coherent pass gets one adversarial verdict. Root checkpoints exact paths and pushes promptly when focused checks pass, before more implementation. At most two active issues: #220 and #162; #221 queued behind delivered #220. Isolated worktrees; no overlapping edits. Root/lane B owns artifact qualification and pinning. Preserve histories and failed evidence; never weaken gates or commit compiler-IR captures.

Record actual argv/environment/source/exits/logs externally; pause green for root checkpoint. Stop on first unexpected failure and report for bounded correction. Required exact-head PR and main qualification plus upstream GitHub synchronization precede closure. Remove clean delivered worktrees after preserving evidence/history. Historical model names below are superseded by user routing.

## Historical issue body

Required follow-up F1 from strip Job 3's adversarial verification. The fold's three GREEN clauses (sole-readership of the last slot, the in-between master scan, one-master retain) are genuinely shadowed in every compilable session today — proven by five adversarially constructed shapes — but the shadowing rests on incidental deterministic-schedule facts (submix-destination edges sorting before output-destination routes; sidechain consumers scheduling before routes) that nothing pins. A scheduler-order change would make the sole-readership clause the only defense, with zero red tests.

Fix: an M1-style `route_folds_over_program` corpus differential driving the runtime's own `route_fold` clause code over the seeded random-graph corpus (house precedent: #208's scatter_redirects_over_program, which this same round demanded and landed). Must land BEFORE the next change in the route-fold/mixdown class. Also fold in a comment-level guard for the `plain_route_gains`/`node_kind` cascade coupling (F7 residual).