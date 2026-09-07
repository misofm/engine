# Issue 572 attempt 1 review

Reviewed head: `9f0e40b0bff0891c734ee18085dc04f5005502f4`

Reviewer: Astra LOW

Verdict: **FAIL**

The normal ordering is deterministic and all product/accounting assertions remain intact, but the control-side failure path can still deadlock. `release_tx` is declared outside `thread::scope`, so the scope body borrows it. If the pre-ack assertion panics, `thread::scope` waits to join render while the outer sender remains alive; render blocks forever on `release_rx.recv()`.

A standalone reproduction of these exact capture/lifetime relationships printed the expected assertion panic and timed out with exit 124. The recorded release-before-poll mutation runs only after render has exited and does not cover this unwind path.

Runtime/API source is unchanged from accepted #571 head `d2e16082`. The full 163-test protocol suite, strict Clippy, formatting, diff, workspace policy, and CI routing checks pass. Attempt 2 must make the scope own the control-side channel endpoints, or create them inside the scope body, so unwind drops the release sender before automatic join. Add a discriminating pre-release assertion/error-path test. No runtime or API change is authorized.
