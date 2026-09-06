# Live fader/matrix qualification evidence (#430/#459)

Source candidate: `7951736605fa64870bc1d91342d00d5fdb6417c5`.
Final source review and focused debug/release proof are retained alongside parent qualification.
The workspace includes doctests: 275 result blocks, 1,591 passed, zero failed, 24 ignored.
Native CAPI resources/shared/static ABI, scalar18 Wasm, SIMD check, executed protocol parity,
current non-LTO atomics inspection, two identical shipped SIMD builds, static/resource/hermetic
worklet checks and all three browser record/check legs passed.

Preserved failures: initial digest build refused a missing output directory before compilation;
initial resource check detected two stale graph totals (each +8 bytes for one Wasm two-Box owner);
initial browser invocation lacked Playwright. Corrected executions are separately retained.
Only two resource expectations changed; all PCM digests stayed identical. The resource checker
also passed its 26 red controls. Native fixture accounting and Astra's independent derivation
are retained. The earlier focused policy PATH failure and corrected statuses remain candid.

`provenance.json` identifies source, artifact, exact terminal statuses and packaging-only changes.
`manifest.json` binds every retained file by byte size and SHA-256. Raw logs are copied verbatim,
including any trailing blank lines; these are evidence, not source formatting failures.
No benchmark ran here. #431 retains timing; #443/#444 retain scalar/concurrent rollout.
Actual-head PR review and required CI remain separate delivery obligations.
