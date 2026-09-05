# Astra #435 Sol attempt 2 review

**FAIL at `5177b9088c94b419f7ac7999059e48ca6850632c`.** The finite safety-documentation correction is mostly complete, but one contradictory foundational SAFETY justification remains. One final prose-only Sol revision can finish it; no runtime/API redesign or new tests are requested.

Accepted work remains valid: I1/I2 are now explicitly builder-proved structure, wave numbering is explicitly not synchronization, E1 supplies the separate foreign write/read happens-before and nonoverlap obligation, and production's exclusive single-lease execution is distinguished from retained multi-lease responsibility. Deleted I3/I4 references and obsolete production worker/coordinator enforcement claims were removed. The concurrent test description correctly identifies disjoint writes followed by joins. Runtime removal, release-mode read-ID bounds guard, builder validation and graph identity/allocation fixtures remain unchanged from accepted attempt1 work (apart from an inconsequential test panic-message wording).

## Remaining contradictory safety statement

`crates/engine/src/realtime/disjoint.rs:224-226`, `write`, still says:

> I1 — buffer is writable by this lease alone ... so no other lease can produce a reference to these words

This is false under retained I2: another lease may produce a SHARED reference to a prior producer's buffer. I1 prevents another writable owner; it does not forbid foreign shared references. The newly corrected E1 is precisely what must prevent those references overlapping this exclusive write. `write_stereo` at240 inherits this incomplete argument via “I1 as in write,” and the combined write/read methods cite write as their mutable-range proof. Consequently the local SAFETY chain remains inconsistent with the new module contract, despite all labels now resolving.

Correct only these comments: distinguish I1's absence of foreign mutable ownership, E1's exclusion of overlapping foreign shared reads, and `&mut self`'s exclusion of references from the same lease. Have write_stereo cite that complete I1/E1 argument plus plane disjointness. No new enforcement, unsafe block, signature, runtime behavior or test change. In docs/REALTIME_DEPENDENCY_POLICY.md:96-98, also clarify the introductory sentence so it says structural I1/I2 are proved at bind while E1 is discharged at execution; the subsequent E1 paragraph already says the correct thing and can remain.

This is completion of the frozen request to state the retained ordering/nonconcurrency proof coherently, not an expansion into general API soundness redesign. Do not change accepted runtime code to resolve a documentation inconsistency.

The retained proportional fmt/diff/realtime check record is green (42 regions/12 files); prior focused debug/release and root allocation evidence remain applicable. No tests/builds/timing or repository/GitHub mutation were performed by this review. Mandatory immutable workspace/targets/artifact/browser qualification is still pending.

Sol attempt2 is consumed. One final attempt3 remains for this exact comment correction, then a further FAIL requires hard stop/rescope rather than a fourth repair.
