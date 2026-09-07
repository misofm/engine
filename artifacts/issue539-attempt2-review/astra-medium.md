**PASS — consolidated attempt-2 source/evidence verification for #539.** The sole attempt-1 blocking gap is closed. Attempt 1 remains FAIL and counted; this is attempt 2, with the three-attempt limit unchanged.

Verified clean, pushed HEAD `82a42b5221a0a40d165bbb04b33f113ad5329b01` against live main `e5b86cf315487fcc602db420dc1a6121f1ac4837`. Main’s intervening changes are documentation/cleanup records, integrated through `cbe5f783`; no DSP base change occurred. [#539](https://github.com/misofm/engine/issues/539) is OPEN with exact title/body synchronization.

Verified final SHA-256 identities:

- `lib.rs`: `677a5596a305039ddbed39d634cde80e90bf99d39439896b1d58a4a539d73b55`
- `tests/mono_collapse.rs`: `1f42a58d6f71232a3af6a0a72717ae57bcf1acc0391a9384eb07e21bfbb2dd6d`
- `tests/allocation.rs`: `7825d30723192724a0cab7165953eeb00860e5dad66dc6b7d74333d1ca0ea920`

The [corrected transition fixture](/home/bl/misofm/engine-limiter-stationary-dispatch/crates/true-peak-limiter/tests/mono_collapse.rs:326) snapshots **every track’s complete common, left and right payload** immediately after desymmetrization, before the next render, and again after the first resumed dual block. Both comparisons use exact byte equality against the never-collapsed reference; existing PCM comparisons remain.

The population offsets match the [actual codec](/home/bl/misofm/engine-limiter-stationary-dispatch/crates/true-peak-limiter/src/lib.rs:2534): twelve history words begin at word 15; rings begin at word 27; at the fixture’s fixed 48 kHz, main length is 486 and required/box lengths are 481 each. Guards cover both channels of every reference track and require history/main words distinct from zero and gain-ring words distinct from one. The loud, alternating fixture and gain-ring guards establish meaningful populated state. These guards supplement—not reduce—the complete payload comparison, which includes cursors, recursive state, phase/prefix, box sum and every ramp word.

Both final mono binaries recompiled and passed **2 tests in debug and 2 in release**. Retained native fingerprints specify AVX2/FMA; `Backend::current()` selects Simd8 on x86-64, and `for_backend` returns Eight. Thus the fixture’s optional-backend return did not turn these results into skipped W8 evidence.

The remaining `lib.rs` diff is exactly the three authorized formatter changes: the tuple arm, private oracle call and population assertion. No operation, state update or expectation changed. Final workspace `fmt --check` passed; the final private old-behavior/path selector recompiled and passed **1 test**. Strict affected Clippy passed on the corrected test before those mechanical formatting changes. The original failed formatting check remains retained. **56 attempt-2 files match their raw counterparts.**

No additional pre-PASS qualification is necessary for this mechanical diff. Prior native/Wasm lowering remains attributed to **`615787e9` / `08ea2bfa…`**, not the final source hash; I accept the reviewed semantic connection without asserting identical rebuilt artifact bytes. Attempt-1 source, realtime, oracle and negative-control findings remain intact. Its full suite **42 passed / 1 ignored** and corpus **139 cases / 349 comparisons per leg** remain historically attributed evidence, not newly executed checks.

The substantial emitted-code expansion remains an **unmeasured tradeoff**. No net performance or speedup claim is accepted.

This PASS accepts the bounded source/evidence slice. Ordinary native ABI, published-artifact and browser qualification on final source, exact PR-head/current-base review, required CI, merge, GitHub synchronization and eventual worktree removal remain pending.

Review was read-only: no edits, builds, tests, mutations, benchmarks, captures, Git/GitHub writes, agents or report-file writes.