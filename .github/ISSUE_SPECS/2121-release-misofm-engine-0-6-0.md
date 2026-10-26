# Release @misofm/engine 0.6.0

Filed by root for the console sprint (2026-10-26). The owner approved publishing SDK releases when their release PR is green, through the manual qualify-then-publish workflow.

## Product outcome
`@misofm/engine` 0.6.0 on npm, adding to 0.5.0:
- #2111: the WebAssembly SIMD BLAKE3 (`prepareBlake3`), about 1.5 GB/s per worker, and the allocation-free JS fallback;
- #2116: `planLatency()` on both engines;
- #2118: concurrent PCM pump window reads with read-ahead, and the stale-window seek fix.

## Smallest slice (docs/RELEASE.md, "The release PR")
Re-pin the worklet module, record a fresh two-browser qualification (Chromium and WebKit), set the 0.6.0 release identity (npm-publish.yml, test-npm-publish-modes.py, sdk/package.json and lock, the README install line and release record #2121), and add a short "Upgrading from 0.5.0" note (additive API; behaviour changes). No Rust source change. After the merge: qualify, publish, verify.
