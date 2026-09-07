**FAIL — consolidated attempt-1 verdict. Attempt 2 is authorized within #537.**

Verified clean, pushed `c73e4b56`, live base `375a86c`, and OPEN #537 with exact title/body identity. Restored source matches `dbdc6847…88c1932`.

The frozen negative control failed: forcing actual uniform access to fallback still produced **1 passed, status 0**. [The counter records classification before executing the branch](/home/bl/misofm/engine-multiband-detector-access/crates/multiband-compressor/src/lib.rs:900), so it cannot prove uniform access occurred. Plausible arithmetic does not satisfy this gate.

Additional bounded corrections:

- Global counters can receive calls from concurrently running `split` tests; isolate observation to the witness’s test thread.
- Corrected words distinguish rows/lanes, but the single-ring oracle and count-only render witness do not establish distinct band/channel dataflow. Add distinguishable band/channel words and check actual accessed words against the old-index oracle.

The production diff retains safe loads, original ragged indexing, width-bounded transient classification and unchanged DSP/state layout. This does not establish acceptance.

**Attempt 2 — Luna xhigh:**

1. Correct executed-branch observability first, retaining W1/W4/W8 and mixed-channel coverage. Freeze the corrected candidate; run one fallback-only control that fails the **same mechanism assertion** while old-word equality passes. Restore byte-exactly, verify focused green, then pause for root checkpoint/push.
2. Complete frozen populated PCM/state/report transitions, reset/bypass, allocation/free and own-thread liveness gates; then candidate native/Wasm lowering and final checks.

Preserve the zero-selected capture, periodic-oracle correction, passing mutant and restored pass. The prepared second patch remains **unexecuted**, not evidence. No baseline redo, new framework/corpus, gate weakening or performance claim. Maximum **three total attempts**.