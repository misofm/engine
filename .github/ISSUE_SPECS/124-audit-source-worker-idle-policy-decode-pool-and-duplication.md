# 124 Source worker idle policy, decode pool, and duplication (wave 4 of the #101 plan)

One-line summary: Source worker idle policy, decode pool and duplication: wave 4 of the #101 plan.

> Folded verbatim from GitHub issue #124's body on 2026-10-11 (root ruling on the body-sync unmatched set, decision D2); the text exists on no commit of main. It is the issue's original body (created 2026-08-23, last updated 2026-08-23T23:54Z) and holds the scope; this spec is now the authority for that scope.

Stateless successor for the wave-4 part of the #101 audit plan (the wave-0 seek bug was fixed in `dfdefff` and #112 sealed it; #101 is closed).

Scope (from the #101 plan comment, sections 6.2–6.6 and evals E3–E10; apply unchanged):
- idle worker policy: `park`/`park_timeout` + controller `unpark`; no `yield_now` spin anywhere (today an idle source burns a full core, N stems = N cores);
- one decode thread per prepared set with per-job termination, instead of one OS thread per source;
- decoder: buffered multi-quantum reads with a local file cursor (no seek+read pair per quantum), typed per-encoding conversion kernels with a branchless sanitiser (class A, set-equivalent);
- collapse the six duplicated function pairs; remove the render-side double copy via `begin_block`/`copy_channel`/`end_block`;
- accounting/evidence code off the render path.

Evals: seek-storm (1,000 seeks while rendering, latest generation wins, zero render-side allocation), idle CPU ≤ 5 % via `/proc/<tid>/stat`, decode throughput before/after (descriptive, expect ≥ 10×), memory ceiling independent of stem length (existing test), render-side allocation/syscall audit unchanged-clean.

Wave 4 of the #83 workstream; independent of the DSP waves. Authority: #83 master plan (revision 3) and the #101 plan comment.

**Authority: this file.** The scope above is the issue's brief. The plan comment on GitHub issue
#101 (its `## Implementation plan` comment) carries the numbered steps, evals, acceptance checklist
and hazards, and the master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric contract (D1-D12), the
`Lane` trait and its per-operation semantics, the block-kernel contract, the `math` and
`effect-runtime` boundaries, the fixture re-pin policy of §8, the workstream waves of §9 and the
evals of §10. Where this file and the master plan disagree on a cross-cutting point, the master plan
wins and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 124` and the plan
comment on #101 (`gh issue view 101 --comments`).

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.
