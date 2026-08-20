# Engine V2 issue specifications

These Markdown files are source-of-truth bodies for later GitHub issue creation.  They are not GitHub templates and do not create issues by themselves.  Each body is intentionally stateless: it contains its mission, applicable invariants, interface contract, dependencies, deliverables, non-goals, hazards, gates, target matrix, and evidence requirements.  “Declared tolerance,” “configured budget,” or similar language is valid only when the issue requires the value and its research/measurement rationale to be frozen in the Sol-approved brief before production code starts.

## Use

1. Create the GitHub issue title from the body H1 after removing its three-digit ordering prefix, then copy the complete body without replacing substantive sections.
2. Sol approves the brief and objective gates before implementation.
3. Terra implements attempt 1 and appends evidence to the issue.
4. Sol conducts adversarial review and may make two further implementation/revision attempts.
5. After three failed attempts total, stop and create a rescope/rebrief issue; do not lower acceptance gates.

Files/H1s are ordered by numeric prefix and use lowercase kebab-case filenames.  The prefix is planning metadata, not part of the published GitHub title; dependency entries therefore name the exact published title and remain portable outside this repository.

Issue numbers preserve creation order, not dependency order. The issue-011 rescope moved external
descriptor/package/state bytes into issue 029 without renumbering existing specs: launch work uses
**Native effect runtime contract and conformance**, while issue 027 and future repository work use
**Canonical effect interchange, state migration, and CID package identity**. Therefore the
extensibility sequence is 029 -> 027 -> 028.

## Shared definition

Engine V2 is a greenfield, Rust, agent-first mixing/mastering engine.  It must not inspect/copy V1.  The render thread exclusively owns a prepared plan whose topology/capacities are immutable and whose preallocated DSP state is mutated during rendering.  The render path performs no allocation/free, lock, I/O, network, logging, syscall, structural plan mutation, or data-dependent unbounded work; displaced plans are reclaimed off-thread.  There is no compiled track limit.  Audio is planar `f32`; dual-mono channels remain independent unless an explicit contract links them.  Output is PCM.
