# Remove the browser adapter's legacy observation and spectrum path

Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`, decision 8): remove it, provided nothing still uses it. `hosts/host-web` keeps a "legacy" and a "protected" observation and spectrum path (`hosts/host-web/src/ffi.rs:646-690`, `protected_observation_prepared` in `lib.rs`), which doubles the refusal tests. Source: the test-value audit, ruling R8, and its verification. This is browser-adapter code, not core.

## Smallest closable slice

1. **Prove non-use first.** Show that the SDK (`sdk/src/core/boundary.ts` and every caller) and the shipped web entry points (`hosts/host-web/web/`) reach only the protected path: trace every `miso_engine_web_v1_*observ*` / spectrum export the SDK calls, and the mode each boot path prepares. **If anything reaches the legacy path, stop and report instead of deleting.**
2. Remove the legacy path, its dispatch and alias handling, and the tests that exist only for it; keep the protected path's refusal tests.

## Objective gates

- The SDK tests, browser qualification (all three browsers) and the host-web tests pass.
- The shipped module shrinks or is explained; the AudioWorklet callgraph and realtime gates pass.
- Test lists before and after show only legacy-only tests removed.
