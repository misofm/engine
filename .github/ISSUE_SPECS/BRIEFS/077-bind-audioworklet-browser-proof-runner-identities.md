# Sol implementation brief — issue 077 bind AudioWorklet browser proof runner identities

## Decision

**DEFERRED / NOT STARTED.** Consume stopped Issue-076 checkpoint `1875c97` only as technical input.
Because Issues 024, 075 and 076 stopped consecutively, move orchestration to another dependency-ready
feature before returning. On resumption, allow one Terra attempt plus one bounded Sol correction.

## Literal closure

- Add one `runnerSha256` map containing exactly the browser-correctness Python runner, hermetic test
  script, seal wrapper and run wrapper.
- Recompute and compare the full map during seal creation and before and after browser execution.
- Hermetically mutate each file independently; every mutation, missing/extra entry and symlink must
  reject before browser launch. The exact clean map accepts.
- Freeze every Issue-076 product, artifact, fixture, independent-oracle and browser-gate byte.

After nonexecuting gates pass on a clean commit, request one no-retry seal authorization. Only an
independently verified fresh seal can receive one later no-retry browser authorization. The old seals
remain immutable evidence. No broad Issue-074 work, product change, benchmark or timing is allowed.
