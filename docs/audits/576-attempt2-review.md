# Issue 576 attempt 2 review

Reviewed head: `e6df0a6c7b3f3c9264a95ed5767c13cb7b07c7f2`

Reviewer: Astra LOW

Verdict: **FAIL**

The named-allocation cap check, private producer ownership and exact-path isolation
improved. The full host-core all-target suite, ten endpoint tests in debug and
release, the private fault unit test, strict Clippy, formatting/diff, workspace and
host policy, and Wasm scalar/simd128 compilation passed.

Cancellation still permits irreversible generic credit release before endpoint
outcome reconciliation. After render passes its cancellation check, control may begin
cancellation and observe an empty outcome queue; render can then publish an Applied
outcome and terminal before generic `collect` releases the ticket, leaving the
endpoint's later outcome take to return Empty. Cancellation start is not proof of a
canceled outcome. `poll_cancel_boundary` also delegates directly to the generic core
without performing the required endpoint reconciliation. Attempt 3 must require an
acknowledged cancellation and matching outcome/terminal join before any release.

The integration test installs no allocator wrapper, so entering the render audit does
not make its zero violation counters evidence. Attempt 3 must install a real counting
allocator, prove positive allocation/free liveness, repeat endpoint cycles, and
independently observe retained storage.

Concurrency, audio and mutations remain insufficiently discriminating: both tickets
precede render; no publication crosses the actual singleton claim; PCM is only
nonzero rather than compared for addressed bank and scalar builtin owners, state and
post-fader observation; paired dispatch is not directly refused; and the five
mutations exercise neighboring behavior instead of the frozen claims. The threaded
fixture also keeps its step sender outside the scope, so a control panic can strand
the render receiver during scoped join. Attempt 3 must move all rendezvous endpoints
inside scope and prove sender-drop exit.

Resource field names and composition remain inaccurate: a builtin-only retention sum
is documented as host-wide, queue bytes mix with inline sizes, the largest allocation
includes inline objects, and render inline bytes exclude started-plan/fault state.
Attempt 3 must separate retained heap, inline ownership and host composition and
validate them with an independent oracle.

Attempt 3 is the final allowed implementation attempt. It remains within the same
#576 paths. A third FAIL triggers the hard stop and a bounded successor; no fourth
revision or weakened gate is authorized.
