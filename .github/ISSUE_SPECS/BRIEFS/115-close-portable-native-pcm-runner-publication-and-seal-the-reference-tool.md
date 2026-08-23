# Sol implementation brief — issue 115 portable runner publication

## Decision

**READY AFTER THE ISSUE-073 STOP CHECKPOINT.** Preserve stopped Issue 073's Linux runner as
technical input, replace only its non-Linux publication boundary, prove Windows/Unix compilation
and run one fresh full runner seal. Sol High implements; Sol XHigh verifies. One pass plus one
bounded HOLD correction; second HOLD stops. All workload/benchmark/timing/playback/listening counts
remain zero.

Accepted dependencies are **Stable C ABI and host-fed planar PCM render** (022) and **Close native-
source seek submission qualification and seal backpressure fix** (112). Stopped **Native WAV/RF64
reference runner over frozen C ABI V1** (073) contributes technical bytes only. Accepted 115 plus
**Close C ABI control/event transport and transactional plan replacement** (113) later gate Issue
114.

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
Only the clean seal can yield overall PASS and unblock Issue 114.
