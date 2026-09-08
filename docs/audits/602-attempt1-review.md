# Issue 602 attempt 1 adversarial review

Reviewer: Astra LOW

Reviewed head: `68950adf1a1f2458431f64d9a5fc84766e182d45`

Verdict: **FAIL**. Final preflight and capture are not authorized.

The local head, upstream head, and clean worktree matched; `Cargo.lock` was unchanged. The
timing seam itself is narrow: buffer construction and render bookkeeping surround the injected
observer, while only `PreparedRenderPlan::render` is inside the timed closure. Issue #600's
existing command remains untimed.

Blocking findings:

1. The runner publishes accepted records before checking lifecycle markers. Marker counts do not
   gate PASS, do not require one start marker, and do not prove ordered, distinct round 1 and round
   2 completion. A temporary stub-only reproduction published valid JSON as PASS with zero start
   markers, zero round markers, and zero reported timed calls. No real capture entry or timer ran.
2. The self-test lacks missing and extra record cases, and its purported reorder produces duplicate
   round 2 records rather than rounds 2 then 1. It has no successful lifecycle case. Its copied
   identity fixtures can fail first on unrelated argv identity, and its sentinel is not connected
   to the real launch path.
3. Failed no-clobber publication in preflight or final disposition can fall through to READY/PASS
   because the failing `ln` is the first command of an AND list under `set -e`.
4. Preflight leaves Cargo profile, target, and compiler-wrapper environment inputs uncontrolled
   while asserting fixed effective build settings.
5. Strict validation has no upper bounds for elapsed/nonzero counts, permits duplicate
   `missing_metadata` entries, and omits target pair, smoothing, and owner count from the seal.

Safe checks passed for the existing assertions: the stub lifecycle script, two capture arithmetic
unit tests in debug and release, and all twelve qualification checksum entries. Those checks do not
cover the blocking contract gaps.

Issue #602 already consumed its one bounded runner correction. Its remaining general attempt count
does not authorize another runner correction. Repair and promotion must move to a numbered successor.
