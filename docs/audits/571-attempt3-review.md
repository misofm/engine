# Issue 571 attempt 3 review and hard stop

Reviewed head: `d2e16082852daccc8181c87d18f6126400c6d66f`

Reviewer: Astra LOW

Verdict: **FAIL — three-attempt hard stop reached**

## Blocking finding

The new threaded generic schedule is not deterministic. After the control side releases the `request_visible` barrier, render may complete `cancel_boundary` and publish its acknowledgement before control asserts that `poll_cancel_boundary` returns `None`. That legal schedule makes the assertion fail. Because render then waits at a later `acknowledged` barrier, an assertion failure on control can also strand the scoped thread and hang the test.

This is a test-ordering and failure-safe rendezvous defect. Review found no remaining production cancellation defect. Repeating the passing test does not eliminate the permitted failing schedule.

## Accepted evidence

- Direct unpublished-state inspection discriminates the former unsupported-automation publication defect.
- Serial/order overflow preflights, generic resource measurements, repeated cancellation allocation checks, stale/reused identities, exact two-ticket accounting, partial-collection publication refusal, and the three recorded production mutations are adequate.
- The physical-credit mutation fails at actual terminal collection rather than fixture setup.
- The public scope, single ticket authority, independent cancel capacity, bounded realtime work, and automation accounting remain acceptable.

Independent gates passed 163 protocol tests, strict Clippy with only existing unrelated allowlist warnings, formatting, whitespace, workspace policy, and CI routing checks.

## Required successor boundary

Do not revise #571 a fourth time. Preserve this source checkpoint and create one new bounded successor that owns only deterministic cancellation-test ordering and failure-safe thread rendezvous. It must perform the pre-ack assertion before render is released to acknowledge, retain the existing zero/partial/full and ownership assertions, and rerun unchanged product gates. Runtime source and public API are frozen unless the successor's Astra LOW brief identifies a newly reproduced production defect.
