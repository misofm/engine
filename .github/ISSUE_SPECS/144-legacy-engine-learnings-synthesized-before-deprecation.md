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
- Item 3 is complete. The successor slice on `audit-144-certification` closes every blocker the
  first slice recorded, and `docs/NATIVE_VECTORIZATION_V1.md` is the contract plus the honest limits.
  The shipped-artifact and backend matrix the earlier record asked for is now defined there and
  enforced from three registries:
  - **Fresh LLVM IR and object.** `tools/miso-engine-vectorization-probes` is built per backend with
    `--emit=llvm-ir,obj`; each family's IR and disassembled body are asserted independently for
    backend-width arithmetic, no scalar or narrow residue, no fast-math flag, no `@llvm.fmuladd`,
    and no math-library call. `@llvm.fma` stays permitted as the one D3 fusion.
  - **Shipped-product binding.** `libmiso_engine_capi.so` and the browser artifact's native twin
    (`libmiso_engine_host_web.so`) are read as built. The render entry is required to be a defined
    export, its direct and GOT-indirect call closure is walked and reported, and seven production
    bank functions per product are required to be defined once, above a floor, and strictly vector
    dominated. The limit is stated rather than papered over: kernels are `#[inline(always)]` and
    have no symbol in any artifact, so the binding is proven at the instantiating production
    function, and the closure stops at the `dyn PreparedPlanExecutor` boundary.
  - **AArch64.** Certified in this environment, not skipped: the probe crate is built as an rlib
    with `--emit=llvm-ir,obj`, so no cross linker is required. The guard is an explicit skip with a
    reason when the target standard library is absent, and a backend that is neither certified nor
    explicitly skipped is now a failure -- the exact hole that let an all-AArch64-row deletion stay
    green on x86.
  - **Completeness.** 27 public lane kernels parsed out of the lane sources, 22 certified at both
    backends, 5 exempt with written reasons; the check fails in both directions.
  - **Receipt.** One hash chain over sources, registries, flags, tool versions, IR, objects,
    artifacts and disassembly. Two clean rebuilds produce a byte-identical report, `chain_sha256`
    included.
  - **Discrimination.** 15 live red mutations, each asserted to fail for its own reason, plus parser
    negative fixtures for the rename, inline, duplicate-header and scalar-FMA evasions. The suite
    rebuilds its binary and asserts the rebuilt binary carries the subject.
  It remains a **non-blocking** CI report by decision. The promotion criteria are recorded in
  `tools/miso-engine-audit/VECTORIZATION.md`; the binding one is that the shipped kernel-host roster
  is curated by hand rather than derived, so a gate would claim coverage it does not have.
- Items 8 and 9 and the two verbatim standing review rules are complete subject to final review.

Item 1 remains the one open decision on this assignment: it reaches deferred FP/math/DSP semantics,
and issue #146 escalated item 2 to pin the floating-point environment at every render entry rather
than chase per-kernel inertness through ~70 rows. Preserve that finding rather than narrowing the
asserted claim.
