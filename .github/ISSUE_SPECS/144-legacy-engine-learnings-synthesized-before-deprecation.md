# 144 Legacy engine learnings: what v1 and engine-v2-old do better, synthesized before deprecation

One-line summary: Preserve and selectively adopt the remaining proven lessons from the two legacy
engines before those repositories are deprecated.

**Authority: GitHub issue #144 and the bounded execution assignment.** The issue body is the
stateless brief. This assignment owns only items 1, 3, 8, and 9 plus the two standing review rules;
all other numbered items remain explicitly deferred and must not be implemented on this branch.

Read, in order: `AGENTS.md`; `gh issue view 144`; then the named v1 and v2-old source material. Do
not weaken a gate, omit corpus rows, redefine shipped artifacts, or cross into a deferred item to
manufacture a passing result.

## Current evidence and decisions

- Item 1's exact native full-corpus experiment genuinely clears then sets MXCSR FTZ and DAZ and
  restores the entry word with an unwind-safe guard. Unmodified `origin/main` diverges in 70 of the
  331 case/width rows. The test therefore remains an explicit ignored reproducer, not an accepted
  gate; the proposed `FLUSH_EPS` mutation cannot discriminate until the baseline is green.
- Item 3's x86 release probe report is useful diagnostic evidence but is not accepted certification:
  it lacks the required fresh LLVM-IR chain, shipped-product binding, and native AArch64 report.
- Items 8 and 9 and the two verbatim standing review rules are complete subject to final review.

Resolving either failed item requires an amended issue decision. Item 1 reaches deferred FP/math/DSP
semantics; item 3 requires a precise shipped-artifact and backend matrix definition. Preserve these
findings rather than narrowing the asserted claim.
