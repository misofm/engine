# Close true-peak-limiter FX4 investigation as deferred optimization

GitHub: https://github.com/misofm/engine/issues/683

Parent: #559 (FX4). Coordination: #560. Base:
`8999def5ac8aea06a0082df2b4764878e0b13dc8`.

## Problem

The bounded true-peak-limiter FX4 lowering investigation exhausted all three
authorized capture attempts in #668 without qualifying a constant-materialization
mapping. Attempt 3 produced status-zero compiler payloads and a working decode,
but its required freshness record, complete caller/route/specialization/backedge
attribution, payload manifest, and self-excluding manifest did not pass. Those raw
captures cannot support a residual or implementation decision.

No measured performance problem or owner justification exists for another
limiter FX4 attempt. Continuing the same capture under another name would be a
disguised fourth #668 attempt.

## Smallest closable slice

Record a documentation-only disposition that closes the true-peak-limiter part
of the current FX4 investigation as **deferred optimization**:

- #668 attempt 1 failed its capture-control prerequisite before any compiler
  invocation because the wrapper rejected both controls;
- attempt 2 failed preflight persistence before controls or compiler execution;
- attempt 3 did not persist the required pre-creation freshness proof, and its
  otherwise status-zero compiler captures never gained complete attribution or
  manifest evidence;
- all three attempts receive no inherited qualification or implementation credit;
- limiter FX4 applicability and any lowering residual remain unresolved, not
  eliminated or qualified;
- delivered product source remains unchanged; and
- no timing, cycle, improvement, projected-saving, regression, floor, or
  budget-miss claim is established.

This disposition does not increase delivered-optimization accounting. Any future
limiter constant capture or implementation requires a genuinely new weekly or
performance issue with a measured budget or owner-approved justification. The
`box_sum / window` reciprocal-substitution question remains a separate class-B
owner ruling and receives no decision here.

## Exact ownership

This issue owns only:

- this numbered issue spec;
- concise disposition rows in the #559 and #560 tracker specs and GitHub bodies.

It owns no Rust or test source, capture, decoding completion, attribution repair,
manifest, benchmark, timing run, compiler payload, target, artifact,
AudioWorklet pin, lockfile, policy, workflow, or cleanup. Lane B alone owns any
AudioWorklet qualification and pins.

Preserve the failed #668 worktree
`/home/bl/misofm/engine-limiter-gain-constant-materialization`, branch
`codex/classify-limiter-gain-constants`, all three committed attempt records,
and every `/tmp/issue668-*` evidence and target path. Preserve the failed
#643/#647/#649/#651/#656/#660 soft-clip worktrees, branches, histories, and
retained #649/#656/#660 payloads exactly as #559 requires. Do not remove,
rewrite, repair, reconstruct, recapture, or rerun them.

## Gates and workflow

1. Sol briefs this issue from exact current main after confirming both shared
   issue slots are free and the scope is distinct from #668's failed capture.
2. Astra LOW reviews the exact clean pushed brief, issue/spec parity, ownership,
   no-credit accounting, and preservation rules before disposition authorship.
3. A Luna HIGH or XHIGH documentation executor changes only this spec and the
   concise #559/#560 rows. It verifies an empty product-source diff and makes no
   new lowering or performance inference.
4. Astra LOW adversarially reviews the exact pushed evidence checkpoint. Any
   correction consumes the next of at most three documentation attempts.
5. After PASS, use exact-head/current-main PR review, required qualification,
   guarded merge-parent verification, post-main qualification, GitHub/tracker
   synchronization, and clean removal only of this delivered worktree.

No compiler command, capture, decode, source edit, test, benchmark, timing,
artifact qualification, pin change, failed-state cleanup, or fourth #668 attempt
is authorized.

## Initial scope record

Astra LOW reviewed live main
`8999def5ac8aea06a0082df2b4764878e0b13dc8`, clean synchronized tracker
`38b6814b1418bd11772e214252e4afe6ad9b05f9`, and #668's clean pushed hard-stop
head `0fb54a41cec818a85a768b4ea0405747658e9bb5`. It returned **SCOPE PASS** for
this genuinely different documentation-only slice, subject to a fresh exact-head
review after the numbered brief and matching GitHub issue are pushed. Both shared
issue slots were free before creation. No product or evidence command ran.
