# Sol implementation brief — issue 113 C ABI control and plan replacement

## Terminal verdict

**TERMINAL ARCHITECTURE STOP / NO IMPLEMENTATION / NO OVERALL PASS.** Sol XHigh's stateless audit
confirmed Sol High's blocker before pass 1. The accepted protocol controller has no prospective
prepare/commit/cancel token: structural dispatch commits `SessionStore`, reliable event and replay
state inside its one-shot command path. The accepted plan exchange has no control-side publication
reservation/cancel token, and retirement capacity is admitted only by the render owner at a block
boundary.

Protocol-first ordering can leave a committed session without its plan; plan-first ordering can
leave a published plan without its protocol/session commit. Copying accepted controller semantics
into CAPI is forbidden. The required atomic transaction is therefore impossible within Issue 113's
frozen CAPI plus one core-seam allowance.

The clean audit baseline was `b5be8148b7651024307eca17b664b09a07a13122`, tree
`cef7922aff699afb292e22fa13953356aa875753`. Sol High made no implementation edit. All benchmark,
timing and real-workload counters remain zero; no build or execution evidence is claimed.

## Successor route

Issue 117, **Complete C ABI transactions with two-phase protocol and plan reservations**, consumes
the exact accepted Issues 005, 003 and 022 plus this stopped readiness record. It alone may add the
narrow protocol transaction token and core publication/retirement reservation needed before CAPI
integration. Issue 113 supplies technical reasoning only, not an accepted product checkpoint.

Accepted Issue 117 gates **Optional binary WebSocket sidecar** (025). **Qualify native C ABI and
reference runner target matrix** (114) waits on accepted Issues 116 and 117. Do not resume Issue
113, weaken atomicity, duplicate protocol behavior or claim PASS.
