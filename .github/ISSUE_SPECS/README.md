# Engine V2 issue specifications

These Markdown files are source-of-truth bodies for later GitHub issue creation.  They are not GitHub templates and do not create issues by themselves.  Each body is intentionally stateless: it contains its mission, applicable invariants, interface contract, dependencies, deliverables, non-goals, hazards, gates, target matrix, and evidence requirements.  “Declared tolerance,” “configured budget,” or similar language is valid only when the issue requires the value and its research/measurement rationale to be frozen in the Sol-approved brief before production code starts.

## Use

1. Create the GitHub issue title from the body H1 after removing its three-digit ordering prefix, then copy the complete body without replacing substantive sections.
2. Sol approves the brief and objective gates before implementation.
3. Terra implements attempt 1 and appends evidence to the issue.
4. Sol conducts adversarial review and may make two further implementation/revision attempts unless
   the issue's authoritative brief freezes a smaller attempt budget.
5. After the frozen attempt budget fails, stop and create a stateless rescope/rebrief issue; do not
   lower acceptance gates or silently begin another attempt.

Files/H1s are ordered by numeric prefix and use lowercase kebab-case filenames.  The prefix is planning metadata, not part of the published GitHub title; dependency entries therefore name the exact published title and remain portable outside this repository.

Issue numbers preserve creation order, not dependency order. The issue-011 rescope moved external
descriptor/package/state bytes into issue 029 without renumbering existing specs: launch work uses
**Native effect runtime contract and conformance**, while issue 027 and future repository work use
**Canonical effect interchange, state migration, and CID package identity**. Therefore the
extensibility sequence is 029 -> 027 -> 028.

The issue-006 three-attempt workflow remains recorded as failed at runner artifact promotion. A
fresh Sol rescope accepted its launch-critical graph compiler/runtime/PDC outcome from complete
functional evidence and the already measured validator-valid raw descriptive benchmark. Issue 030
owns only benchmark-wrapper operational hardening and promotion of those exact bytes. It is not a
dependency of issues 007–010 and does not authorize another issue-006 benchmark run.

Issue 032 corrects the repository-wide launch sample-rate policy after issue 007 accepted its
launch filter recurrence: 44,100/48,000/88,200/96,000 Hz are the exact launch session/render set;
176,400/192,000/352,800/384,000 Hz remain extended compatibility evidence only. Dependency order
places the accepted 007 slice -> 032 -> 034 -> 008, while 010 follows 032; downstream effects,
hosts and release qualification follow their exact listed dependencies.
Historical “required/all-eight” evidence is preserved as period evidence and does not override
issue 032.

Issue 007 stopped after three failed attempts. Its post-stop Sol rescope accepts only the proven
conditioned scalar TPT/builtin runtime slice as reusable technical foundation; it does not claim
machine qualification and preserves every failure. Issue 034, **Issue-007 launch-critical builtin
contract closure**, owns parameter metadata, sealed-only graph attachment, exact checked resource
accounting and compiler mutations. Issue 035, **Issue-007 builtin qualification tooling, audits,
and benchmark**, depends on 034 and owns the complete fixture/audit/target evidence and the sole
eventual one-invocation/two-round builtin benchmark. Both corrective workflows have at most two
attempts. The timed invocation count is currently zero; issue 034 cannot authorize a run.

Dependency order is 007 accepted slice -> 034 -> both 008 and 035. Issue 008 needs 034's stable
preparation/metadata and sealed graph/resource contract but not 035's scalar qualification
tooling. Issues 022, 023 and 024 wait for 035's corrected machine candidate. Issue 033 runs the
real preregistered human listening only after 035 seals that candidate and its accepted benchmark;
issue 026 waits for both 035 and 033. This ordering forbids synthetic trials, audible-quality
claims and release bypasses while allowing independent SIMD work after its true contract
dependency is complete.

## Shared definition

Engine V2 is a greenfield, Rust, agent-first mixing/mastering engine.  It must not inspect/copy V1.  The render thread exclusively owns a prepared plan whose topology/capacities are immutable and whose preallocated DSP state is mutated during rendering.  The render path performs no allocation/free, lock, I/O, network, logging, syscall, structural plan mutation, or data-dependent unbounded work; displaced plans are reclaimed off-thread.  There is no compiled track limit.  Audio is planar `f32`; dual-mono channels remain independent unless an explicit contract links them.  Output is PCM.
