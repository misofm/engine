# Sol implementation brief — issue 115 portable runner publication

## Decision

**TERMINAL SECOND-HOLD STOP / NO OVERALL PASS.** Preserve stopped Issue 073's Linux runner and
Issue 115 checkpoint `99f081f` only as technical input. Sol XHigh found that cleanup still checks
path identity and then separately unlinks the pathname. The bounded correction established that the
frozen visible named partial cannot be conditionally unlinked by held identity on Linux or Apple;
quarantine only moves the race. The two-pass budget is exhausted and no full runner seal is
authorized. All workload/benchmark/timing/playback/listening counts are zero.

Accepted dependencies are **Stable C ABI and host-fed planar PCM render** (022) and **Close native-
source seek submission qualification and seal backpressure fix** (112). Stopped **Native WAV/RF64
reference runner over frozen C ABI V1** (073) contributes technical bytes only. Issue 115 is not an
accepted dependency. Accepted stateless successor 116 plus **Close C ABI control/event transport
and transactional plan replacement** (113) later gate Issue 114.

## Sole correction

Keep Linux held-FD `linkat(AT_EMPTY_PATH)`. Delete the generic ownership-check/pathname-hard-link
fallback. Provide explicit Linux/Android, Apple Unix and Windows adapters. Publication must be
atomic no-replace and accepted bytes must bind to the held create-new partial identity; an adapter
without a proved primitive fails before partial creation. Cleanup removes only a proven owned entry
or the exact entry atomically created by the publication operation. Failure always leaves final
absent and preserves substituted sentinels.

Remove unconditional Unix imports. Compile the library once for `x86_64-unknown-linux-gnu`,
`x86_64-pc-windows-gnu` and `aarch64-apple-darwin`.
Freeze adversarial swaps before and between every path operation, all final collision kinds,
wrong-identity publication, unlink/final-check failures, write/sync/digest failures and exact
success. Static mutations reject the former check/use fallback and unowned cleanup.

## Frozen input and fence

The Issue-073 runner library SHA-256 is
`4ed33714d232ac98c019e2af05662d0ffa03f472008333109298f63b67769444`; manifest SHA-256 is
`8d251ad6b1eca8c95e24b8b4e2959e397d8ec954502307351f1c7fb3c01a9634`. Its CLI, decode/feed/render,
fixtures/output hashes, C ABI/source/header/product bytes and diagnostics are frozen.

Edit only runner publication/test code, minimal target binding manifest/lock rows, runner docs,
exact checker/mutation rows and issue routing/evidence. No product, fixture or main change.

## Evidence sequence

Run the complete original 15 tests plus successor publication rows, independent manifest, C ABI,
strict check/Clippy/rustdoc/format, policies/mutations, static scans and exact Windows/Apple/Linux
compile checks. Sol High stops. Sol XHigh focused PASS permits a clean checkpoint and exactly one
fresh full runner nonbenchmark seal. First failure stops; no benchmark, timing or real workload.
Only the clean seal could have yielded overall PASS. It was not authorized or run. Stateless Issue
116 must replace the impossible same-privilege mutation guarantee with an explicit exclusive-
output-directory invocation precondition, preserve the technical bytes, and run a fresh seal before
Issue 114 can consume the runner.

## Terminal review record

Checkpoint/tree: `99f081f327bd250343a80f928aa099c994ca8e59` /
`8f883e6dbbb31a4dd7350239c81a40466560e7e2`. Exact changed hashes are runner library
`9df99e837c23c81ee7df2ddf983941af6e9f5830b2333a95b12e613554159894`, documentation
`2d5a69e6f3f47c3c6bb5e3e9582056bd44d92087581a6ba0ada01339056b003a`, checker
`ceac1ed25f6ff816fb0a0743408daa0be20f5d40a1651073fcead3fdbb7c8407`, and mutations
`26ddcf8b4db0ed02a45c2a245acbc5475f724a0e21a61d774c3b0b04e5ca6008`.

The reported 16/16 runner tests, Linux/Windows/Apple compile rows and strict focused gates are useful
technical evidence, not PASS. No implementation edit followed the HOLD. Issue 115 is **STOPPED**;
there is no retry and no claim against concurrent same-privilege directory-entry mutation.
