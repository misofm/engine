# 092 Audit: miso-engine-transient-shaper

One-line summary: Re-land the transient shaper on the shared envelope followers and block kernels.

**Authority: GitHub issue #92 and its plan comment.** This file is a stateless pointer, not a
second copy of the brief. The issue body carries the findings with `path:line` evidence; the plan
comment on the issue carries the numbered steps, evals, acceptance checklist and hazards; and the
master plan (the first comment on issue #83) decides everything cross-cutting -- the numeric
contract (D1-D12), the `Lane` trait and its per-operation semantics, the block-kernel contract, the
`miso-engine-math` and `miso-engine-effect-runtime` boundaries, the fixture re-pin policy of §8, the
workstream waves of §9 and the evals of §10. Where this file and those comments disagree, they win
and this file is corrected in the same checkpoint.

Read, in order: `AGENTS.md`; issue #125 (standing instructions for the audit workstream); issue #83
body, master-plan comment and execution-plan comment; then `gh issue view 92` and its plan
comment.

Do not re-decide anything the master plan decides, do not loosen a gate, and do not pin a fixture
from production output: fixtures are regenerated only from an independent `f64` oracle or from the
scalar `Lane` instantiation, with the old-to-new deviation and the audit finding cited in the
commit message.
