# Issue498 Luna attempt1

Final source735dc5d5 removes only the private Vec-producing claim helper and collects both owned sets directly from existing borrowed claims. No tests or other source files changed. Graph debug/release each execute57 tests; real source fanout exact test executes1 each in debug/release. Strict all-targets/all-features graph Clippy, fmt, diff and graph policy exit0.

The first graph-debug log records base1caa4c09 plus the then-uncommitted exact graph diff later checkpointed735dc5d5. Its environment line mistakenly captures inherited PATH before the Cargo-prefix override; Luna disclosed that attribution error afterward and the original log is unchanged. Later logs record effective prefixed PATH and committed source. No source changes occurred between that first run and the checkpoint/final gates.

All raw commands/statuses are preserved. This is source-attempt evidence, not Astra PASS, delivery/artifact qualification or timing. The transformation removes two intermediate vectors when sources exist and one extra key-clone population during bind; it preserves both resulting sets and makes no allocation-free or measured speed claim.

## Qualified delivery

Astra source PASS atfa9d8665. The original ordinary builder at5d8d893c compiled successfully then exited1 for expected87007c1a versus observed8a42eb47. The retained bounded ruling authorized only the observed pin. Frozen candidatea928a42d passes all seven pipeline commands: verified rebuild, static checks, resources plus26 negative controls, hermetic suite, npm setup, current Chromium151.0.7922.34/Firefox153.0/WebKit26.5 with self-tests, and matrix check. Exact argv/cwd/source/raw output and numeric0 exits are retained. Final module identity is independently hashed. Generated records at5ff72a70 change only candidate/hash; numerical expectations remain unchanged. Exact-head PR review and required CI remain pending.
