# Issue 607 shipped-artifact applicability

Reviewer: lane-B Astra LOW under #560/#608

Reviewed head: `8bb2a2e763103030af258a69eecae5bd2374850b`

Verdict: **PASS — not applicable**. Retain AudioWorklet artifact pin
`39ebe7cd3f71f34ab11260f27fa1eaad281dd61642c50d9ed6210e703d95dd55` and its existing
qualification attribution. No rebuild, artifact qualification, browser matrix, or pin change is
required for issue 607.

The issue-607 delta from stopped #606 is the native capture preflight script plus spec, capture, and
review evidence. Its complete delivery delta from current main additionally contains the native
capture scripts and `tools/bench` input-symmetry sources inherited from the stopped predecessors. It
changes no runtime crate, browser host, SDK, Cargo manifest or lockfile, Cargo configuration,
toolchain, fixture, artifact builder, metadata generator, generated consumer, pin, browser result, or
deployment matrix.

The AudioWorklet builder independently builds `host-web`, copies unchanged JavaScript/type files,
and invokes unchanged `parameter-metadata`; it consumes neither `tools/bench` nor these native capture
scripts. This is a source/dependency applicability proof, not a newly executed byte-identity probe.
Issue 608's later artifact gate remains separate. No artifact build, preflight, capture, timing, edit,
or pin change ran during this review.
